// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod error;
mod srt_parser;

use db::DbHandler;
use error::Error;
use srt_parser::process_srt_directory as _process_srt_directory;
use std::path::Path;
use std::sync::Mutex;
// use std::time::Instant;
use tauri::State;

struct TranscriptDatabase(Mutex<DbHandler>);
struct JmdictDatabase(Mutex<DbHandler>);

fn main() -> Result<(), Error> {
    // let start_time = Instant::now();

    // Initialize databases
    let transcript_db = TranscriptDatabase(Mutex::new(DbHandler::new("transcripts.db")?));
    let jmdict_db = JmdictDatabase(Mutex::new(DbHandler::new("jmdict.sqlite")?));

    // Create tables in the transcript database
    transcript_db.0.lock().unwrap().create_tables()?;

    println!("Databases initialized successfully.");

    tauri::Builder::default()
        .manage(transcript_db)
        .manage(jmdict_db)
        .invoke_handler(tauri::generate_handler![
            process_srt_directory,
            create_reverse_index,
            get_all_shows,
            search_word_with_context
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // let duration = start_time.elapsed();
    // println!("Total execution time: {:?}", duration);
    Ok(())
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn process_srt_directory(root_dir: String, database: State<TranscriptDatabase>) {
    println!("Processing SRT files in {}...", root_dir);
    let mut db = database.0.lock().unwrap();
    let root_dir = Path::new(&root_dir);
    let show_entries = _process_srt_directory(root_dir);
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
