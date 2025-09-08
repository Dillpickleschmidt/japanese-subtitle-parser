// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod error;
mod kagome;
mod srt_parser;

use db::DbHandler;
use error::Error;
use srt_parser::process_srt_directory as _process_srt_directory;
use std::path::Path;
use std::sync::Mutex;
use tauri::Manager;
use tauri::State;

struct TranscriptDatabase(Mutex<DbHandler>);

fn main() -> Result<(), Error> {
    tauri::Builder::default()
        .setup(|app| {
            // Create transcripts.db in the src-tauri directory
            let current_dir = std::env::current_dir().expect("failed to get current directory");
            let transcripts_db_path = current_dir.join("transcripts.db");
            let transcript_db = TranscriptDatabase(Mutex::new(DbHandler::new(
                transcripts_db_path.to_str().unwrap(),
            )?));

            // Create tables in the transcript database
            transcript_db.0.lock().unwrap().create_tables()?;

            println!("Databases initialized successfully.");

            // Store the database in the app's managed state for later use
            app.manage(transcript_db);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            process_srt_directory,
            create_reverse_index,
            get_all_shows,
            search_word_with_context,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn process_srt_directory(
    root_dir: String,
    database: State<TranscriptDatabase>,
) -> Result<String, String> {
    println!("Processing SRT files in {}...", root_dir);
    let mut db = database
        .0
        .lock()
        .map_err(|e| format!("Database lock error: {}", e))?;
    let show_entries = _process_srt_directory(Path::new(&root_dir));
    println!(
        "Processed {} entries.",
        show_entries
            .iter()
            .map(|show| show.episodes.len())
            .sum::<usize>()
    );

    // Prepare data for batch insertion
    let mut shows = Vec::new();
    let mut episodes = Vec::new();
    let mut transcripts = Vec::new();

    for (show_id, show) in show_entries.iter().enumerate() {
        let show_id = (show_id + 1) as i32;
        shows.push((show.name.clone(), "Anime".to_string()));

        for episode in &show.episodes {
            let episode_id = (episodes.len() + 1) as i32;
            episodes.push((
                show_id,
                episode.episode_name.clone(),
                episode.season,
                episode.episode_number,
            ));

            for subtitle in episode.content.0.iter() {
                transcripts.push((
                    episode_id,
                    subtitle.number as i32,
                    subtitle.start_time.to_string(),
                    subtitle.end_time.to_string(),
                    subtitle.text.clone(),
                ));
            }
        }
    }

    // Perform batch insertions
    db.insert_shows(&shows)
        .map_err(|e| format!("Failed to insert shows: {}", e))?;
    println!("Shows inserted successfully.");

    db.insert_episodes(&episodes)
        .map_err(|e| format!("Failed to insert episodes: {}", e))?;
    println!("Episodes inserted successfully.");

    db.insert_transcripts(&transcripts)
        .map_err(|e| format!("Failed to insert transcripts: {}", e))?;
    println!("Transcripts inserted successfully.");

    Ok("Processing completed successfully!".to_string())
}

#[tauri::command]
fn create_reverse_index(transcript_db: State<TranscriptDatabase>) -> Result<String, String> {
    let mut db = transcript_db
        .0
        .lock()
        .map_err(|e| format!("Database lock error: {}", e))?;
    db.create_reverse_index()
        .map_err(|e| format!("Failed to create reverse index: {}", e))?;
    Ok("Reverse index created successfully!".to_string())
}

#[tauri::command]
fn get_all_shows(database: State<TranscriptDatabase>) -> Result<Vec<(i32, String)>, String> {
    let mut db = database.0.lock().unwrap();
    // force error to string for now for frontend to handle
    let shows_ids = db.get_show_id_name_pairs().map_err(|err| err.to_string())?;
    Ok(shows_ids)
}

#[tauri::command]
fn search_word_with_context(
    word: String,
    enabled_show_ids: Vec<i32>,
    database: State<TranscriptDatabase>,
) -> Result<String, String> {
    let db = database.0.lock().unwrap();
    db.search_word_with_context(&word, &enabled_show_ids)
        .map(|results| results.to_string())
        .map_err(|err| err.to_string())
}
