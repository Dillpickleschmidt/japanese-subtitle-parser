use crate::types::KagomeToken;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// A vocabulary word extracted from tokens
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VocabWord {
    pub base_form: String,
    pub reading: String,
    pub pos: Vec<String>,
}

impl VocabWord {
    /// Create a VocabWord from a token
    pub fn from_token(token: &KagomeToken) -> Self {
        Self {
            base_form: token.base_form.clone(),
            reading: token.reading.clone(),
            pos: token.pos.clone(),
        }
    }

    /// Check if this is a content word (not punctuation/particles)
    fn is_content_word(&self) -> bool {
        if self.pos.is_empty() {
            return false;
        }

        let pos = &self.pos[0];

        // Filter out symbols/punctuation completely
        if pos == "記号" {
            return false;
        }

        matches!(pos.as_str(), "名詞" | "動詞" | "形容詞" | "副詞" | "感動詞")
    }
}

/// Extract vocabulary from tokens, excluding auxiliary tokens that are part of grammar patterns
///
/// # Arguments
/// - `tokens`: The morphologically analyzed tokens
/// - `auxiliary_indices`: Token indices that are auxiliary (parts of grammar constructions)
///
/// # Returns
/// A vector of unique vocabulary words (deduplicated by position)
pub fn extract_vocabulary(
    tokens: &[KagomeToken],
    auxiliary_indices: &HashSet<usize>,
) -> Vec<VocabWord> {
    let mut words = Vec::new();
    let mut seen = HashSet::new();

    for (i, token) in tokens.iter().enumerate() {
        // Skip auxiliary tokens (parts of conjugations/constructions)
        if auxiliary_indices.contains(&i) {
            continue;
        }

        let word = VocabWord::from_token(token);

        // Skip if not a content word
        if !word.is_content_word() {
            continue;
        }

        // Deduplicate
        if !seen.contains(&word) {
            seen.insert(word.clone());
            words.push(word);
        }
    }

    words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vocab_word_is_content_word() {
        let noun = VocabWord {
            base_form: "猫".to_string(),
            reading: "ねこ".to_string(),
            pos: vec!["名詞".to_string(), "一般".to_string()],
        };
        assert!(noun.is_content_word());

        let particle = VocabWord {
            base_form: "を".to_string(),
            reading: "を".to_string(),
            pos: vec!["助詞".to_string()],
        };
        assert!(!particle.is_content_word());

        let punctuation = VocabWord {
            base_form: "。".to_string(),
            reading: "。".to_string(),
            pos: vec!["記号".to_string()],
        };
        assert!(!punctuation.is_content_word());
    }

    #[test]
    fn test_extract_vocabulary_skips_auxiliary() {
        let tokens = vec![
            KagomeToken {
                id: 0,
                start: 0,
                end: 1,
                surface: "見".to_string(),
                class: String::new(),
                pos: vec!["動詞".to_string()],
                base_form: "見る".to_string(),
                reading: "みる".to_string(),
                pronunciation: String::new(),
                features: vec![],
            },
            KagomeToken {
                id: 1,
                start: 1,
                end: 2,
                surface: "て".to_string(),
                class: String::new(),
                pos: vec!["助詞".to_string()],
                base_form: "て".to_string(),
                reading: "て".to_string(),
                pronunciation: String::new(),
                features: vec![],
            },
        ];

        let mut auxiliary = HashSet::new();
        auxiliary.insert(1); // Mark second token as auxiliary

        let words = extract_vocabulary(&tokens, &auxiliary);

        // Should only get the verb, not the particle
        assert_eq!(words.len(), 1);
        assert_eq!(words[0].base_form, "見る");
    }
}
