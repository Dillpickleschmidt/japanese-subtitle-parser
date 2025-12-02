use grammar_lib::KagomeToken;
use serde::{Deserialize, Serialize};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KagomeError {
    #[error("Failed to start Kagome server: {0}")]
    StartFailed(String),
    #[error("Failed to create HTTP client: {0}")]
    ClientCreation(String),
    #[error("Kagome server failed to start within {0} seconds")]
    StartTimeout(u32),
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Kagome server returned error status: {0}")]
    ServerError(String),
}

#[derive(Serialize)]
struct TokenizeRequest {
    sentence: String,
    mode: String,
}

#[derive(Deserialize)]
struct TokenizeResponse {
    tokens: Vec<KagomeToken>,
}

pub struct KagomeServer {
    process: Child,
    client: reqwest::blocking::Client,
    base_url: String,
}

impl KagomeServer {
    /// Start a Kagome server on the specified port.
    pub fn start(port: u16) -> Result<Self, KagomeError> {
        println!("Starting Kagome server on port {}...", port);

        let process = Command::new("kagome")
            .args(["server", "-http", &format!(":{}", port)])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| KagomeError::StartFailed(e.to_string()))?;

        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| KagomeError::ClientCreation(e.to_string()))?;

        let base_url = format!("http://localhost:{}", port);

        // Wait for server to be ready
        let timeout_secs = 30;
        for _ in 0..timeout_secs {
            if client.get(format!("{}/", base_url)).send().is_ok() {
                println!("Kagome server started successfully");
                return Ok(KagomeServer {
                    process,
                    client,
                    base_url,
                });
            }
            thread::sleep(Duration::from_secs(1));
        }

        Err(KagomeError::StartTimeout(timeout_secs))
    }

    /// Tokenize text using Kagome.
    /// Returns a flat list of tokens with their positions.
    pub fn tokenize(&self, text: &str, mode: &str) -> Result<Vec<KagomeToken>, KagomeError> {
        let request = TokenizeRequest {
            sentence: text.to_string(),
            mode: mode.to_string(),
        };

        let response = self
            .client
            .put(format!("{}/tokenize", self.base_url))
            .json(&request)
            .send()?;

        if !response.status().is_success() {
            return Err(KagomeError::ServerError(response.status().to_string()));
        }

        let response_data: TokenizeResponse = response.json()?;
        Ok(response_data.tokens)
    }

    /// Shutdown the Kagome server gracefully.
    pub fn shutdown(mut self) -> Result<(), KagomeError> {
        println!("Shutting down Kagome server...");
        let _ = self.process.kill();
        let _ = self.process.wait();
        println!("Kagome server shut down");
        Ok(())
    }
}

impl Drop for KagomeServer {
    fn drop(&mut self) {
        let _ = self.process.kill();
        let _ = self.process.wait();
    }
}
