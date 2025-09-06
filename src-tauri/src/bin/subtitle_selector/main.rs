use clap::Parser;
use reqwest::Client;
use std::path::PathBuf;
use std::rc::Rc;
use tokio;

mod candidate_parsing;
mod download_manager;
mod jimaku_api;
mod persistence;
mod selection_engine;
mod types;

use candidate_parsing::parse_candidates;
use download_manager::download_and_process_files;
use jimaku_api::fetch_jimaku_files;
use persistence::{read_existing_mapping, should_download, write_mapping_csv};
use selection_engine::select_best_subtitles;
use types::{format_source_type, SelectionMap};

#[derive(Parser)]
#[command(name = "subtitle_selector")]
#[command(about = "Select and download optimal subtitle files from Jimaku.cc")]
struct Args {
    #[arg(long, help = "Entry ID from Jimaku.cc")]
    entry_id: u32,

    #[arg(long, help = "Output directory for subtitle files")]
    output_dir: PathBuf,

    #[arg(long, help = "Authorization token for Jimaku.cc API")]
    auth_token: String,

    #[arg(long, help = "Show selection process without downloading")]
    dry_run: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("Fetching subtitle files for entry ID: {}", args.entry_id);

    let client = Client::new();
    let files = fetch_jimaku_files(&client, args.entry_id, &args.auth_token).await?;

    println!("Found {} subtitle files", files.len());

    let candidates = parse_candidates(files)?;
    let all_selections = select_best_subtitles(candidates);

    // Read existing mapping to determine what needs downloading
    let csv_path = args.output_dir.join("source_mapping.csv");
    let current_choices = read_existing_mapping(&csv_path)?;

    // Filter selections to only include better choices
    let mut filtered_selections = SelectionMap::new();
    let mut skipped_count = 0;

    for (episode, candidate) in all_selections {
        if should_download(episode, current_choices.get(&episode), &candidate) {
            filtered_selections.insert(episode, candidate);
        } else {
            skipped_count += 1;
        }
    }

    if skipped_count > 0 {
        println!(
            "Skipping {} episodes (already have better or equal quality)",
            skipped_count
        );
    }

    if filtered_selections.is_empty() {
        println!("No new downloads needed - all current files are optimal!");
        return Ok(());
    }

    println!("Selected {} optimal files:", filtered_selections.len());
    for (episode, candidate) in &filtered_selections {
        println!(
            "  Episode {}: {} ({})",
            episode,
            candidate.file_info.name,
            format_source_type(&candidate.source_type)
        );
    }

    if !args.dry_run {
        download_and_process_files(filtered_selections.clone(), &args.output_dir, &client).await?;

        // Merge new selections with existing choices for CSV update
        let mut all_final_selections = SelectionMap::new();

        // Add existing choices that weren't replaced
        for (episode, current_choice) in current_choices {
            if !filtered_selections.contains_key(&episode) {
                // Convert CurrentChoice back to SubtitleCandidate for CSV writing
                let is_zip = current_choice.format == "zip";
                let dummy_candidate = types::SubtitleCandidate {
                    file_info: Rc::new(types::JimakuFile {
                        name: current_choice.original_source.clone(),
                        url: "".to_string(),
                        size: 0,
                        last_modified: "".to_string(),
                    }),
                    episode_numbers: vec![episode],
                    source_type: types::SourceType::OtherWeb, // Default since we don't track this anymore
                    is_cc: false,                             // We don't track this
                    format: if is_zip {
                        "unknown".to_string() // ZIP files don't have a specific subtitle format
                    } else {
                        current_choice.format.clone()
                    },
                    is_zip,
                };
                all_final_selections.insert(episode, dummy_candidate);
            }
        }

        // Add new selections
        all_final_selections.extend(filtered_selections);

        write_mapping_csv(&csv_path, &all_final_selections)?;
        println!("Updated source mapping CSV");
        println!("Processing completed successfully!");
    } else {
        println!("Dry run completed - no files downloaded");
    }

    Ok(())
}

