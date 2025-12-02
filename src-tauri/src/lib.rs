mod analysis;
mod db;
mod error;
mod subtitle_importer;

#[cfg(test)]
mod test_utils;

pub use error::Error;

use db::DbHandler;
use std::path::Path;
use std::sync::Mutex;
use subtitle_importer::process_srt_directory as parse_subtitles_from_directory;
use tauri::Manager;
use tauri::State;

struct SubtitleDatabase(Mutex<DbHandler>);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), Error> {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Create transcripts.db in the src-tauri directory
            let current_dir = std::env::current_dir().expect("failed to get current directory");
            let transcripts_db_path = current_dir.join("transcripts.db");
            let subtitle_db = SubtitleDatabase(Mutex::new(DbHandler::new(
                transcripts_db_path.to_str().unwrap(),
            )?));

            // Create tables in the subtitle database
            subtitle_db.0.lock().unwrap().create_tables()?;

            println!("Databases initialized successfully.");

            // Store the database in the app's managed state for later use
            app.manage(subtitle_db);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            import_subtitles_from_directory,
            analyze_japanese_transcripts,
            get_all_shows,
            search_word_with_context,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

#[tauri::command]
fn import_subtitles_from_directory(
    root_dir: String,
    database: State<SubtitleDatabase>,
) -> Result<String, String> {
    println!("Importing subtitle files from {}...", root_dir);
    let mut db = database
        .0
        .lock()
        .map_err(|e| format!("Database lock error: {}", e))?;

    let show_entries = parse_subtitles_from_directory(Path::new(&root_dir));
    println!(
        "Processed {} entries.",
        show_entries
            .iter()
            .map(|show| show.episodes.len())
            .sum::<usize>()
    );

    // Prepare show data for batch insertion
    let mut shows = Vec::with_capacity(show_entries.len());
    let anime_type = "Anime".to_string();

    for show in &show_entries {
        shows.push((show.name.clone(), anime_type.clone()));
    }

    // Prepare all data for batch processing
    let total_episodes = show_entries.iter().map(|s| s.episodes.len()).sum();
    let mut all_episodes = Vec::with_capacity(total_episodes);
    let mut all_transcripts = Vec::new();

    for (show_index, show) in show_entries.iter().enumerate() {
        let show_id = (show_index + 1) as i32;

        for episode in &show.episodes {
            let episode_index = all_episodes.len();

            // Add episode data
            all_episodes.push((
                show_id,
                episode.episode_name.clone(),
                episode.episode_number,
            ));

            // Add transcript data with episode index for later mapping
            for subtitle in &episode.content.0 {
                all_transcripts.push((
                    episode_index, // We'll map this to episode_id later
                    subtitle.number as i32,
                    subtitle.start_time.to_milliseconds(),
                    subtitle.end_time.to_milliseconds(),
                    subtitle.text.clone(),
                ));
            }
        }
    }

    // Insert all shows and episodes
    db.insert_shows(&shows)
        .map_err(|e| format!("Failed to insert shows: {}", e))?;
    println!("Shows inserted successfully.");

    let episode_ids = db
        .insert_episodes(&all_episodes)
        .map_err(|e| format!("Failed to insert episodes: {}", e))?;
    println!("Episodes inserted successfully.");

    // Convert transcripts to final format with actual episode IDs
    let final_transcripts: Vec<_> = all_transcripts
        .into_iter()
        .map(|(episode_index, line_id, time_start, time_end, text)| {
            (
                episode_ids[episode_index],
                line_id,
                time_start,
                time_end,
                text,
            )
        })
        .collect();

    // Process transcripts in batches for progress logging
    const BATCH_SIZE: usize = 50000;
    let total_transcripts = final_transcripts.len();

    for (batch_index, chunk) in final_transcripts.chunks(BATCH_SIZE).enumerate() {
        let start = batch_index * BATCH_SIZE;
        let end = std::cmp::min(start + BATCH_SIZE, total_transcripts);

        println!(
            "Processing transcripts {}-{} of {}",
            start + 1,
            end,
            total_transcripts
        );

        db.insert_transcripts(chunk)
            .map_err(|e| format!("Failed to insert transcript batch: {}", e))?;
    }

    println!("All shows and episodes processed successfully.");

    Ok("Processing completed successfully!".to_string())
}

#[tauri::command]
fn analyze_japanese_transcripts(subtitle_db: State<SubtitleDatabase>) -> Result<String, String> {
    let mut db = subtitle_db
        .0
        .lock()
        .map_err(|e| format!("Database lock error: {}", e))?;
    db.create_reverse_index()
        .map_err(|e| format!("Failed to analyze Japanese transcripts: {}", e))?;
    Ok("Japanese transcript analysis completed successfully!".to_string())
}

#[tauri::command]
fn get_all_shows(database: State<SubtitleDatabase>) -> Result<Vec<(i32, String)>, String> {
    let mut db = database.0.lock().unwrap();
    let shows_ids = db.get_show_id_name_pairs().map_err(|err| err.to_string())?;
    Ok(shows_ids)
}

#[tauri::command]
fn search_word_with_context(
    word: String,
    enabled_show_ids: Vec<i32>,
    database: State<SubtitleDatabase>,
) -> Result<String, String> {
    let db = database.0.lock().unwrap();
    db.search_word_with_context(&word, &enabled_show_ids)
        .map(|results| results.to_string())
        .map_err(|err| err.to_string())
}
