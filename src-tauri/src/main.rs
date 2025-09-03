// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod error;
mod srt_parser;

use db::DbHandler;
use error::Error;
use srt_parser::process_srt_directory as _process_srt_directory;
use std::fs;
use std::path::Path;
use std::sync::Mutex;
use tauri::Manager;
use tauri::State;

struct TranscriptDatabase(Mutex<DbHandler>);
struct JmdictDatabase(Mutex<DbHandler>);
struct TranscriptsRawPath(std::path::PathBuf);

fn main() -> Result<(), Error> {
    tauri::Builder::default()
        .setup(|app| {
            let path_resolver = app.path_resolver();

            // Resolve paths for resources
            let transcripts_raw_path = path_resolver
                .resolve_resource("data/transcripts_raw")
                .expect("failed to resolve transcripts_raw directory");

            let jmdict_path = path_resolver
                .resolve_resource("jmdict.sqlite")
                .expect("failed to resolve jmdict.sqlite");

            let transcripts_db_path = path_resolver
                .resolve_resource("transcripts.db")
                .expect("failed to resolve transcripts.db");

            // Check if transcripts.db exists in resources
            let transcripts_db_exists = transcripts_db_path.exists();

            // Initialize databases with resolved paths
            let transcript_db = if transcripts_db_exists {
                TranscriptDatabase(Mutex::new(DbHandler::new(
                    transcripts_db_path.to_str().unwrap(),
                )?))
            } else {
                // If transcripts.db doesn't exist in resources, create a new one in the app's data directory
                let app_data_dir = app
                    .path_resolver()
                    .app_data_dir()
                    .expect("failed to get app data dir");
                fs::create_dir_all(&app_data_dir)?;
                let new_transcripts_db_path = app_data_dir.join("transcripts.db");
                TranscriptDatabase(Mutex::new(DbHandler::new(
                    new_transcripts_db_path.to_str().unwrap(),
                )?))
            };

            let jmdict_db =
                JmdictDatabase(Mutex::new(DbHandler::new(jmdict_path.to_str().unwrap())?));

            // Create tables in the transcript database if it's newly created
            if !transcripts_db_exists {
                transcript_db.0.lock().unwrap().create_tables()?;
            }

            println!("Databases initialized successfully.");

            // Store the paths in the app's managed state for later use
            app.manage(TranscriptsRawPath(transcripts_raw_path));
            app.manage(transcript_db);
            app.manage(jmdict_db);

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
fn process_srt_directory(app_handle: tauri::AppHandle, database: State<TranscriptDatabase>) {
    let transcripts_raw_path = app_handle.state::<TranscriptsRawPath>();
    let root_dir = transcripts_raw_path.0.to_str().unwrap();

    println!("Processing SRT files in {}...", root_dir);
    let mut db = database.0.lock().unwrap();
    let show_entries = _process_srt_directory(Path::new(root_dir));
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
                episode.episode_number as i32,
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
    db.insert_shows(&shows).unwrap();
    println!("Shows inserted successfully.");

    db.insert_episodes(&episodes).unwrap();
    println!("Episodes inserted successfully.");

    db.insert_transcripts(&transcripts).unwrap();
    println!("Transcripts inserted successfully.");
}

#[tauri::command]
fn create_reverse_index(
    transcript_db: State<TranscriptDatabase>,
    jmdict_db: State<JmdictDatabase>,
) {
    let mut db = transcript_db.0.lock().unwrap();
    let mut jmdict = jmdict_db.0.lock().unwrap();
    db.create_reverse_index("parsed_transcripts.csv", &mut jmdict)
        .unwrap();
    println!("Reverse index created successfully.");
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
