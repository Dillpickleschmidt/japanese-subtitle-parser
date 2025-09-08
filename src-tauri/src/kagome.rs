use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::process::{Command, Stdio};
use std::time::Instant;

// Configuration constants
const KAGOME_PATH: &str = "/home/dylank/go/bin/kagome";
const BATCH_SIZE: usize = 1000;

#[derive(Debug, Deserialize, Serialize)]
pub struct KagomeToken {
    // Only keep essential fields for production use
    #[serde(default)]
    pub id: u32,
    #[serde(default)]
    pub start: u32,
    #[serde(default)]
    pub end: u32,
    #[serde(default)]
    pub surface: String,
    #[serde(default)]
    pub class: String,
    pub pos: Vec<String>,
    pub base_form: String,
    pub reading: String,
    #[serde(default)]
    pub pronunciation: String,
    #[serde(default)]
    pub features: Vec<String>,
}

pub fn process_text_with_kagome(text: &str) -> Result<Vec<Vec<KagomeToken>>, Error> {
    let mut child = Command::new(KAGOME_PATH)
        .args(["-json", "-mode", "search"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| Error::Other(format!("Failed to spawn kagome: {}", e)))?;

    if let Some(stdin) = child.stdin.take() {
        let mut stdin = stdin;
        stdin
            .write_all(text.as_bytes())
            .map_err(|e| Error::Other(format!("Failed to write to kagome: {}", e)))?;
        drop(stdin);
    }

    let output = child
        .wait_with_output()
        .map_err(|e| Error::Other(format!("Kagome execution failed: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::Other(format!("Kagome failed: {}", stderr)));
    }

    let json_output = String::from_utf8(output.stdout)
        .map_err(|e| Error::Other(format!("Invalid kagome output: {}", e)))?;

    if json_output.trim().is_empty() {
        return Ok(Vec::new());
    }

    // Parse multiple JSON arrays using streaming deserializer
    use std::io::Cursor;
    let cursor = Cursor::new(json_output.as_bytes());
    let stream = serde_json::Deserializer::from_reader(cursor).into_iter::<Vec<KagomeToken>>();

    let mut all_arrays = Vec::new();

    for result in stream {
        match result {
            Ok(tokens) => all_arrays.push(tokens),
            Err(e) => return Err(Error::Other(format!("JSON parse error: {}", e))),
        }
    }

    Ok(all_arrays)
}

pub fn extract_words_from_transcripts(
    transcript_texts: &[(i64, String)],
) -> Result<HashMap<(String, String, Vec<String>), HashSet<i64>>, Error> {
    println!(
        "Starting word extraction from {} transcripts",
        transcript_texts.len()
    );
    let mut word_map: HashMap<(String, String, Vec<String>), HashSet<i64>> = HashMap::new();

    for (i, chunk) in transcript_texts.chunks(BATCH_SIZE).enumerate() {
        println!(
            "Processing batch {} of {} (transcripts {}-{})",
            i + 1,
            (transcript_texts.len() + BATCH_SIZE - 1) / BATCH_SIZE,
            i * BATCH_SIZE + 1,
            std::cmp::min((i + 1) * BATCH_SIZE, transcript_texts.len())
        );

        let chunk_result = process_transcript_batch(chunk)?;

        for ((word, reading, pos), transcript_ids) in chunk_result {
            word_map
                .entry((word, reading, pos))
                .or_default()
                .extend(transcript_ids);
        }
    }

    println!(
        "Word extraction completed. Found {} unique words",
        word_map.len()
    );

    // Fix base form readings
    println!("Correcting base form readings...");
    let corrected_word_map = fix_base_form_readings(word_map)?;
    println!("Base form reading correction completed");

    Ok(corrected_word_map)
}

fn fix_base_form_readings(
    word_map: HashMap<(String, String, Vec<String>), HashSet<i64>>,
) -> Result<HashMap<(String, String, Vec<String>), HashSet<i64>>, Error> {
    // Extract unique base forms as Vec to maintain order
    let unique_base_forms: Vec<String> = word_map
        .keys()
        .map(|(base_form, _, _)| base_form.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    if unique_base_forms.is_empty() {
        return Ok(word_map);
    }

    println!("Correcting {} base form readings", unique_base_forms.len());

    // Create mapping from base form to correct reading
    let mut base_form_to_reading: HashMap<&str, String> =
        HashMap::with_capacity(unique_base_forms.len());

    // Process base forms in batches to avoid memory issues and timeouts
    for (batch_idx, batch) in unique_base_forms.chunks(BATCH_SIZE).enumerate() {
        println!(
            "  Processing reading batch {} of {} ({} words)",
            batch_idx + 1,
            (unique_base_forms.len() + BATCH_SIZE - 1) / BATCH_SIZE,
            batch.len()
        );

        // Combine batch into single text blob
        let combined_batch = batch.join("\n");

        // Process through kagome
        let batch_tokens = process_text_with_kagome(&combined_batch)?;

        // Map results back to base forms
        for (line_idx, base_form) in batch.iter().enumerate() {
            if let Some(tokens) = batch_tokens.get(line_idx) {
                // Find the first content word in the line (should be our base form)
                let reading = tokens
                    .iter()
                    .find(|token| is_content_word(token) && token.base_form == *base_form)
                    .map(|token| &token.reading)
                    .or_else(|| tokens.first().map(|token| &token.reading));

                if let Some(reading) = reading {
                    base_form_to_reading.insert(base_form, reading.clone());
                }
            }
        }
    }

    // Update word map in place with corrected readings
    let mut corrected_word_map: HashMap<(String, String, Vec<String>), HashSet<i64>> =
        HashMap::with_capacity(word_map.len());

    for ((base_form, old_reading, pos), transcript_ids) in word_map {
        let correct_reading = base_form_to_reading
            .get(base_form.as_str())
            .cloned()
            .unwrap_or(old_reading);

        corrected_word_map
            .entry((base_form, correct_reading, pos))
            .or_default()
            .extend(transcript_ids);
    }

    println!("Corrected {} readings", base_form_to_reading.len());
    Ok(corrected_word_map)
}

fn process_transcript_batch(
    batch: &[(i64, String)],
) -> Result<HashMap<(String, String, Vec<String>), HashSet<i64>>, Error> {
    // Combine all transcript texts with newlines for kagome processing
    let combined_text = batch
        .iter()
        .map(|(_, text)| text.as_str())
        .collect::<Vec<_>>()
        .join("\n");

    let start = Instant::now();
    let lines_of_tokens = process_text_with_kagome(&combined_text)?;
    println!(
        "  Kagome processed {} transcripts in {:.2?}",
        batch.len(),
        start.elapsed()
    );

    let mut word_map: HashMap<(String, String, Vec<String>), HashSet<i64>> = HashMap::new();

    // Map each line of tokens back to its corresponding transcript
    for (line_idx, &(transcript_id, _)) in batch.iter().enumerate() {
        if let Some(tokens) = lines_of_tokens.get(line_idx) {
            for token in tokens {
                if is_content_word(token) {
                    word_map
                        .entry((
                            token.base_form.clone(),
                            token.reading.clone(),
                            token.pos.clone(),
                        ))
                        .or_default()
                        .insert(transcript_id);
                }
            }
        }
    }

    Ok(word_map)
}

fn is_content_word(token: &KagomeToken) -> bool {
    if token.pos.is_empty() {
        return false;
    }

    let pos = &token.pos[0];

    // Filter out symbols/punctuation completely - not words
    if pos == "記号" {
        return false;
    }

    matches!(pos.as_str(), "名詞" | "動詞" | "形容詞" | "副詞" | "感動詞")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_text_with_kagome() {
        let text = "私は猫が好きです。";
        let result = process_text_with_kagome(text);

        match result {
            Ok(tokens) => {
                assert!(!tokens.is_empty());
                for token in tokens {
                    println!("{}: {} ({})", token.surface, token.base_form, token.reading);
                }
            }
            Err(e) => println!("Test failed (kagome may not be installed): {}", e),
        }
    }

    #[test]
    fn test_is_content_word() {
        let noun_token = KagomeToken {
            id: 1,
            start: 0,
            end: 1,
            surface: "猫".to_string(),
            class: "KNOWN".to_string(),
            pos: vec!["名詞".to_string(), "一般".to_string()],
            base_form: "猫".to_string(),
            reading: "ネコ".to_string(),
            pronunciation: "ネコ".to_string(),
            features: vec![],
        };

        let particle_token = KagomeToken {
            id: 2,
            start: 1,
            end: 2,
            surface: "は".to_string(),
            class: "KNOWN".to_string(),
            pos: vec!["助詞".to_string(), "係助詞".to_string()],
            base_form: "は".to_string(),
            reading: "ハ".to_string(),
            pronunciation: "ワ".to_string(),
            features: vec![],
        };

        assert!(is_content_word(&noun_token));
        assert!(!is_content_word(&particle_token));
    }
}
