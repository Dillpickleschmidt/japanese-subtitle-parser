use crate::analysis::morphology::KagomeToken;
use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::Duration;

const KAGOME_PATH: &str = "/home/dylank/go/bin/kagome";
const SERVER_PORT: u16 = 6061; // Use non-default port to avoid conflicts

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

        // Start the Kagome server
        let process = Command::new(KAGOME_PATH)
            .args(["server", "-http", &format!(":{}", SERVER_PORT)])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| Error::Other(format!("Failed to start Kagome server: {}", e)))?;

        // Create HTTP client
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| Error::Other(format!("Failed to create HTTP client: {}", e)))?;

        let base_url = format!("http://localhost:{}", SERVER_PORT);

        // Wait for server to be ready
        let mut attempts = 0;
        loop {
            if attempts >= 30 {
                return Err(Error::Other(
                    "Kagome server failed to start within 30 seconds".to_string(),
                ));
            }

            // Try to connect
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

        // Map tokens back to their original transcripts using position boundaries
        // Pre-allocate with estimated capacity (avg ~15 tokens per transcript)
        let mut token_arrays: Vec<Vec<KagomeToken>> = transcript_boundaries
            .iter()
            .map(|_| Vec::with_capacity(15))
            .collect();

        // Sequential scan optimization: since tokens are in order, track current transcript
        let mut current_transcript = 0;

        for token in response_data.tokens {
            // Skip newline tokens (they're just separators)
            if token.surface == "\n" {
                continue;
            }

            // Move to the next transcript if we've passed its boundary
            // Tokens are in order, so we only need to scan forward
            while current_transcript < transcript_boundaries.len() - 1 {
                let (_, end) = transcript_boundaries[current_transcript];
                if token.start < end {
                    break; // Token belongs to current transcript
                }
                current_transcript += 1;
            }

            // Add token to the current transcript
            token_arrays[current_transcript].push(token);
        }

        Ok(token_arrays)
    }

    /// Tokenize text using normal mode for dictionary lookups
    /// This is used for reading corrections where we want dictionary forms, not search segmentation
    pub fn tokenize_normal_mode(
        &self,
        text: &str,
    ) -> Result<Vec<Vec<KagomeToken>>, Error> {
        let request = TokenizeRequest {
            sentence: text.to_string(),
            mode: "normal".to_string(), // Use normal mode for dictionary readings
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

        // For reading corrections, we split by newlines and group tokens accordingly
        // Each word is on its own line, so we group tokens until we hit a newline token
        let mut token_arrays = Vec::new();
        let mut current_tokens = Vec::new();

        for token in response_data.tokens {
            if token.surface == "\n" {
                // Newline marks end of current word
                if !current_tokens.is_empty() {
                    token_arrays.push(current_tokens);
                    current_tokens = Vec::new();
                }
            } else {
                current_tokens.push(token);
            }
        }

        // Don't forget the last word if it doesn't end with newline
        if !current_tokens.is_empty() {
            token_arrays.push(current_tokens);
        }

        Ok(token_arrays)
    }

    pub fn shutdown(mut self) -> Result<(), Error> {
        println!("Shutting down Kagome server...");

        // Try to kill the process gracefully
        self.process
            .kill()
            .map_err(|e| Error::Other(format!("Failed to kill Kagome server: {}", e)))?;

        // Wait for it to exit
        self.process
            .wait()
            .map_err(|e| Error::Other(format!("Failed to wait for Kagome server: {}", e)))?;

        println!("Kagome server shut down");
        Ok(())
    }
}

impl Drop for KagomeServer {
    fn drop(&mut self) {
        // Best effort to kill the server if it's still running
        let _ = self.process.kill();
    }
}

