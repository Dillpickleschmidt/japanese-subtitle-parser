use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::process::{Command, Stdio};

// Configuration constants
const KAGOME_PATH: &str = "/home/dylank/go/bin/kagome";

#[derive(Debug, Deserialize, Serialize, Clone)]
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
        .args(["-mode", "search"]) // Removed -json for TSV output
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

    let tsv_output = String::from_utf8(output.stdout)
        .map_err(|e| Error::Other(format!("Invalid kagome output: {}", e)))?;

    if tsv_output.trim().is_empty() {
        return Ok(Vec::new());
    }

    // Parse TSV output - much faster than JSON
    let mut all_arrays = Vec::new();
    let mut current_tokens = Vec::new();

    for line in tsv_output.lines() {
        let line = line.trim();

        // EOS marks end of a text segment
        if line == "EOS" {
            if !current_tokens.is_empty() {
                all_arrays.push(current_tokens);
                current_tokens = Vec::new();
            }
            continue;
        }

        // Skip empty lines
        if line.is_empty() {
            continue;
        }

        // Parse TSV line: surface\tfeatures
        let parts: Vec<&str> = line.splitn(2, '\t').collect();
        if parts.len() != 2 {
            continue; // Skip malformed lines
        }

        let surface = parts[0].to_string();
        let features: Vec<String> = parts[1].split(',').map(|s| s.to_string()).collect();

        // Extract common fields from features
        // Format: pos1,pos2,pos3,pos4,pos5,pos6,base_form,reading,pronunciation
        let pos = features
            .iter()
            .take(6)
            .filter(|s| !s.is_empty() && *s != "*")
            .cloned()
            .collect();

        let base_form = features
            .get(6)
            .map(|s| if s == "*" { surface.clone() } else { s.clone() })
            .unwrap_or_else(|| surface.clone());

        let reading = features
            .get(7)
            .map(|s| if s == "*" { String::new() } else { s.clone() })
            .unwrap_or_default();

        let pronunciation = features
            .get(8)
            .map(|s| if s == "*" { String::new() } else { s.clone() })
            .unwrap_or_default();

        let token = KagomeToken {
            id: 0,    // Not available in TSV format
            start: 0, // Not available in TSV format
            end: 0,   // Not available in TSV format
            surface,
            class: String::new(), // Not available in TSV format
            pos,
            base_form,
            reading,
            pronunciation,
            features,
        };

        current_tokens.push(token);
    }

    // Don't forget the last segment if it doesn't end with EOS
    if !current_tokens.is_empty() {
        all_arrays.push(current_tokens);
    }

    Ok(all_arrays)
}

/// Get correct base form readings for a list of words
/// This function batches words into chunks for Kagome processing
pub fn get_base_form_readings(
    base_forms: &[&str],
) -> Result<std::collections::HashMap<String, String>, Error> {
    use std::collections::HashMap;

    if base_forms.is_empty() {
        return Ok(HashMap::new());
    }

    let mut reading_map = HashMap::new();

    // Process in chunks of 5,000 words to avoid overwhelming Kagome
    for chunk in base_forms.chunks(5000) {
        let combined_text = chunk.join("\n");
        let token_arrays = process_text_with_kagome(&combined_text)?;

        // Map results back to original base forms for this chunk
        for (i, &base_form) in chunk.iter().enumerate() {
            if let Some(tokens) = token_arrays.get(i) {
                if let Some(first_token) = tokens.first() {
                    // Use the reading from the dictionary form analysis
                    reading_map.insert(base_form.to_string(), first_token.reading.clone());
                }
            }
        }
    }

    Ok(reading_map)
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
                for token_array in tokens {
                    for token in token_array {
                        println!("{}: {} ({})", token.surface, token.base_form, token.reading);
                    }
                }
            }
            Err(e) => println!("Test failed (kagome may not be installed): {}", e),
        }
    }
}
