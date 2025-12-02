use kagome_client::KagomeServer;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::time::Instant;

fn get_jitendex_path() -> String {
    let home = std::env::var("HOME").expect("HOME environment variable not set");
    format!("{}/Downloads/jitendex-yomitan", home)
}

fn get_output_path() -> String {
    let home = std::env::var("HOME").expect("HOME environment variable not set");
    format!(
        "{}/Programming-Projects/japanese-subtitle-parser/grammar-lib/src/compounds.txt",
        home
    )
}
const KAGOME_PORT: u16 = 6062; // Different port to avoid conflicts
const BATCH_SIZE: usize = 500; // Send multiple words per request

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    // Collect all headwords from jitendex
    println!("Reading jitendex term banks...");
    let jitendex_path = get_jitendex_path();
    let output_path = get_output_path();
    let mut headwords: Vec<String> = Vec::new();

    for entry in fs::read_dir(&jitendex_path)? {
        let path = entry?.path();
        let filename = path.file_name().unwrap().to_str().unwrap();

        if !filename.starts_with("term_bank_") || !filename.ends_with(".json") {
            continue;
        }

        let content = fs::read_to_string(&path)?;
        let entries: Vec<serde_json::Value> = serde_json::from_str(&content)?;

        for entry in entries {
            if let Some(arr) = entry.as_array() {
                if let Some(headword) = arr.first().and_then(|v| v.as_str()) {
                    // Skip single characters and empty strings
                    if headword.chars().count() > 1 {
                        headwords.push(headword.to_string());
                    }
                }
            }
        }
    }

    // Deduplicate
    let unique_headwords: HashSet<String> = headwords.into_iter().collect();
    let headwords: Vec<String> = unique_headwords.into_iter().collect();
    println!("Found {} unique headwords (>1 char)", headwords.len());

    // Start Kagome server
    println!("Starting Kagome server on port {}...", KAGOME_PORT);
    let server = KagomeServer::start(KAGOME_PORT)?;

    // Process in batches, using newline as separator
    println!("Tokenizing headwords...");
    let mut compounds: HashSet<String> = HashSet::new();
    let total_batches = headwords.len().div_ceil(BATCH_SIZE);

    for (batch_idx, chunk) in headwords.chunks(BATCH_SIZE).enumerate() {
        if batch_idx % 50 == 0 {
            println!(
                "  Batch {}/{} ({:.1}%)",
                batch_idx + 1,
                total_batches,
                (batch_idx as f64 / total_batches as f64) * 100.0
            );
        }

        // Join with newlines so Kagome tokenizes each separately
        let batch_text = chunk.join("\n");
        let tokens = server.tokenize(&batch_text, "normal")?;

        // Split tokens back by newline boundaries
        let mut current_word_idx = 0;
        let mut token_count_for_current = 0;

        for token in &tokens {
            if token.surface == "\n" {
                // End of current word
                if token_count_for_current > 1 {
                    compounds.insert(chunk[current_word_idx].clone());
                }
                current_word_idx += 1;
                token_count_for_current = 0;
            } else if !token.surface.trim().is_empty() {
                token_count_for_current += 1;
            }
        }

        // Handle last word in batch (no trailing newline)
        if current_word_idx < chunk.len() && token_count_for_current > 1 {
            compounds.insert(chunk[current_word_idx].clone());
        }
    }

    // Write results
    println!("Writing {} compounds to {}", compounds.len(), output_path);
    let file = File::create(&output_path)?;
    let mut writer = BufWriter::new(file);

    let mut sorted_compounds: Vec<_> = compounds.into_iter().collect();
    sorted_compounds.sort();

    for compound in &sorted_compounds {
        writeln!(writer, "{}", compound)?;
    }

    println!("Done in {:.1}s", start.elapsed().as_secs_f64());
    println!("Output: {} compounds", sorted_compounds.len());

    Ok(())
}
