use crate::analysis::morphology::KagomeToken;
use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::Duration;

const KAGOME_PATH: &str = "/home/dylank/go/bin/kagome";
const SERVER_PORT: u16 = 6061;

#[derive(Serialize)]
struct TokenizeRequest {
    sentence: String,
    mode: String,
}

pub struct KagomeServer {
    process: Child,
    client: reqwest::blocking::Client,
    base_url: String,
}

impl KagomeServer {
    pub fn start() -> Result<Self, Error> {
        println!("Starting Kagome server on port {}...", SERVER_PORT);

        let process = Command::new(KAGOME_PATH)
            .args(["server", "-http", &format!(":{}", SERVER_PORT)])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| Error::Other(format!("Failed to start Kagome server: {}", e)))?;

        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| Error::Other(format!("Failed to create HTTP client: {}", e)))?;

        let base_url = format!("http://localhost:{}", SERVER_PORT);

        let mut attempts = 0;
        loop {
            if attempts >= 30 {
                return Err(Error::Other(
                    "Kagome server failed to start within 30 seconds".to_string(),
                ));
            }

            if client.get(format!("{}/", base_url)).send().is_ok() {
                break;
            }

            thread::sleep(Duration::from_secs(1));
            attempts += 1;
        }

        println!("Kagome server started successfully");

        Ok(KagomeServer {
            process,
            client,
            base_url,
        })
    }

    pub fn tokenize(
        &self,
        text: &str,
        transcript_boundaries: &[(u32, u32)],
    ) -> Result<Vec<Vec<KagomeToken>>, Error> {
        let request = TokenizeRequest {
            sentence: text.to_string(),
            mode: "search".to_string(),
        };

        let response = self
            .client
            .put(format!("{}/tokenize", self.base_url))
            .json(&request)
            .send()
            .map_err(|e| Error::Other(format!("Failed to send request to Kagome server: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::Other(format!(
                "Kagome server returned error: {}",
                response.status()
            )));
        }

        #[derive(Deserialize)]
        struct TokenizeResponse {
            tokens: Vec<KagomeToken>,
        }

        let response_data: TokenizeResponse = response
            .json()
            .map_err(|e| Error::Other(format!("Failed to parse Kagome response: {}", e)))?;

        let mut token_arrays: Vec<Vec<KagomeToken>> = transcript_boundaries
            .iter()
            .map(|_| Vec::with_capacity(15))
            .collect();

        // Convert byte boundaries to character boundaries for proper offset adjustment
        // Kagome returns character offsets, but our boundaries are byte positions
        let char_boundaries: Vec<(u32, u32)> = transcript_boundaries
            .iter()
            .map(|(byte_start, byte_end)| {
                let char_start = text[..*byte_start as usize].chars().count() as u32;
                let char_end = text[..*byte_end as usize].chars().count() as u32;
                (char_start, char_end)
            })
            .collect();

        let mut current_transcript = 0;

        for token in response_data.tokens {
            if token.surface == "\n" {
                continue;
            }

            // Find which transcript this token belongs to based on character position
            while current_transcript < char_boundaries.len() - 1 {
                let (_, char_end) = char_boundaries[current_transcript];
                if token.start < char_end {
                    break;
                }
                current_transcript += 1;
            }

            // Adjust token character offsets to be relative to the individual transcript
            // instead of the combined text buffer
            let (char_start, _) = char_boundaries[current_transcript];
            let mut adjusted_token = token;
            adjusted_token.start -= char_start;
            adjusted_token.end -= char_start;

            token_arrays[current_transcript].push(adjusted_token);
        }

        Ok(token_arrays)
    }

    pub fn tokenize_normal_mode(
        &self,
        text: &str,
    ) -> Result<Vec<Vec<KagomeToken>>, Error> {
        let request = TokenizeRequest {
            sentence: text.to_string(),
            mode: "normal".to_string(),
        };

        let response = self
            .client
            .put(format!("{}/tokenize", self.base_url))
            .json(&request)
            .send()
            .map_err(|e| Error::Other(format!("Failed to send request to Kagome server: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::Other(format!(
                "Kagome server returned error: {}",
                response.status()
            )));
        }

        #[derive(Deserialize)]
        struct TokenizeResponse {
            tokens: Vec<KagomeToken>,
        }

        let response_data: TokenizeResponse = response
            .json()
            .map_err(|e| Error::Other(format!("Failed to parse Kagome response: {}", e)))?;

        let mut token_arrays = Vec::new();
        let mut current_tokens = Vec::new();

        for token in response_data.tokens {
            if token.surface == "\n" {
                if !current_tokens.is_empty() {
                    token_arrays.push(current_tokens);
                    current_tokens = Vec::new();
                }
            } else {
                current_tokens.push(token);
            }
        }

        if !current_tokens.is_empty() {
            token_arrays.push(current_tokens);
        }

        Ok(token_arrays)
    }

    pub fn shutdown(mut self) -> Result<(), Error> {
        println!("Shutting down Kagome server...");

        self.process
            .kill()
            .map_err(|e| Error::Other(format!("Failed to kill Kagome server: {}", e)))?;

        self.process
            .wait()
            .map_err(|e| Error::Other(format!("Failed to wait for Kagome server: {}", e)))?;

        println!("Kagome server shut down");
        Ok(())
    }
}

impl Drop for KagomeServer {
    fn drop(&mut self) {
        let _ = self.process.kill();
    }
}

