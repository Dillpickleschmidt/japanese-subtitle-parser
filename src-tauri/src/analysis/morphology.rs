use crate::analysis::kagome_server::KagomeServer;
use crate::error::Error;

// Re-export KagomeToken from grammar module for backwards compatibility
pub use crate::grammar::types::KagomeToken;

pub fn process_batch_with_kagome_server(
    batch: &[(i64, i32, String)],
    server: &KagomeServer,
) -> Result<Vec<Vec<KagomeToken>>, Error> {
    let mut combined_text = String::new();
    let mut boundaries = Vec::new();

    for (_, _, text) in batch {
        let start = combined_text.len() as u32;
        combined_text.push_str(text);
        let end = combined_text.len() as u32;
        boundaries.push((start, end));

        combined_text.push('\n');
    }

    if combined_text.ends_with('\n') {
        combined_text.pop();
    }

    server.tokenize(&combined_text, &boundaries)
}

pub fn get_base_form_readings(
    base_forms: &[&str],
    server: &KagomeServer,
) -> Result<std::collections::HashMap<String, String>, Error> {
    use std::collections::HashMap;

    if base_forms.is_empty() {
        return Ok(HashMap::new());
    }

    let mut reading_map = HashMap::new();

    const CHUNK_SIZE: usize = 5000;
    let total_chunks = (base_forms.len() + CHUNK_SIZE - 1) / CHUNK_SIZE;

    for (chunk_idx, chunk) in base_forms.chunks(CHUNK_SIZE).enumerate() {
        println!(
            "Processing reading corrections batch {}/{} ({} words)",
            chunk_idx + 1,
            total_chunks,
            chunk.len()
        );

        let combined_text = chunk.join("\n");

        let token_arrays = server.tokenize_normal_mode(&combined_text)?;

        for (i, &base_form) in chunk.iter().enumerate() {
            if let Some(tokens) = token_arrays.get(i) {
                if let Some(first_token) = tokens.first() {
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
    use crate::analysis::kagome_server::KagomeServer;

    #[test]
    fn test_process_text_with_kagome() {
        let text = "私は猫が好きです。";

        let server = KagomeServer::start().unwrap();
        let batch = vec![(1i64, 1i32, text.to_string())];
        let token_arrays = process_batch_with_kagome_server(&batch, &server).unwrap();

        assert_eq!(token_arrays.len(), 1);
        assert!(!token_arrays[0].is_empty());

        for token in &token_arrays[0] {
            assert!(!token.surface.is_empty());
            assert!(!token.base_form.is_empty());
        }

        server.shutdown().unwrap();
    }
}
