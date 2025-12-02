use crate::error::Error;
use kagome_client::KagomeToken;

pub use kagome_client::KagomeServer;

const SERVER_PORT: u16 = 6061;

/// Extension trait to add boundary-aware tokenization to KagomeServer
pub trait KagomeServerExt {
    fn start_default() -> Result<KagomeServer, Error>;
    fn tokenize_with_boundaries(
        &self,
        text: &str,
        transcript_boundaries: &[(u32, u32)],
    ) -> Result<Vec<Vec<KagomeToken>>, Error>;
    fn tokenize_by_newlines(&self, text: &str) -> Result<Vec<Vec<KagomeToken>>, Error>;
}

impl KagomeServerExt for KagomeServer {
    /// Start a Kagome server on the default port (6061).
    fn start_default() -> Result<KagomeServer, Error> {
        KagomeServer::start(SERVER_PORT).map_err(|e| Error::Other(e.to_string()))
    }

    /// Tokenize text and split tokens by transcript boundaries.
    /// Each boundary is a (byte_start, byte_end) pair.
    fn tokenize_with_boundaries(
        &self,
        text: &str,
        transcript_boundaries: &[(u32, u32)],
    ) -> Result<Vec<Vec<KagomeToken>>, Error> {
        let tokens = self
            .tokenize(text, "search")
            .map_err(|e| Error::Other(e.to_string()))?;

        let mut token_arrays: Vec<Vec<KagomeToken>> = transcript_boundaries
            .iter()
            .map(|_| Vec::with_capacity(15))
            .collect();

        // Convert byte boundaries to character boundaries
        let char_boundaries: Vec<(u32, u32)> = transcript_boundaries
            .iter()
            .map(|(byte_start, byte_end)| {
                let char_start = text[..*byte_start as usize].chars().count() as u32;
                let char_end = text[..*byte_end as usize].chars().count() as u32;
                (char_start, char_end)
            })
            .collect();

        for token in tokens {
            if token.surface.trim().is_empty() {
                continue;
            }

            // Find which transcript this token belongs to
            let transcript_idx = char_boundaries
                .iter()
                .position(|(start, end)| token.start >= *start && token.start < *end);

            let current_transcript = match transcript_idx {
                Some(idx) => idx,
                None => {
                    eprintln!(
                        "WARNING: Token '{}' at position {}-{} doesn't fit in any transcript boundary",
                        token.surface, token.start, token.end
                    );
                    continue;
                }
            };

            let (char_start, _) = char_boundaries[current_transcript];

            // Adjust token offsets to be relative to individual transcript
            let mut adjusted_token = token;
            adjusted_token.start =
                adjusted_token
                    .start
                    .checked_sub(char_start)
                    .unwrap_or_else(|| {
                        panic!(
                            "Overflow subtracting start: token.start={} < char_start={}",
                            adjusted_token.start, char_start
                        )
                    });
            adjusted_token.end = adjusted_token
                .end
                .checked_sub(char_start)
                .unwrap_or_else(|| {
                    panic!(
                        "Overflow subtracting end: token.end={} < char_start={}",
                        adjusted_token.end, char_start
                    )
                });

            token_arrays[current_transcript].push(adjusted_token);
        }

        Ok(token_arrays)
    }

    /// Tokenize text in normal mode and split by newlines.
    fn tokenize_by_newlines(&self, text: &str) -> Result<Vec<Vec<KagomeToken>>, Error> {
        let tokens = self
            .tokenize(text, "normal")
            .map_err(|e| Error::Other(e.to_string()))?;

        let mut token_arrays = Vec::new();
        let mut current_tokens = Vec::new();

        for token in tokens {
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
}
