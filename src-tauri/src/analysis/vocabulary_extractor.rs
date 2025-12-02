use kagome_client::KagomeToken;
use grammar_lib::VocabWord;
use std::collections::{HashMap, HashSet};

// ========== Type Alias for Database Context ==========

/// Type alias using grammar-lib's VocabWord for consistency
pub type WordKey = VocabWord;

// ========== Public API ==========

/// Trait for extracting vocabulary from tokenized text
pub trait VocabularyExtractor {
    fn extract(
        &self,
        tokens: &[KagomeToken],
        auxiliary_indices: &HashSet<usize>,
        transcript_id: i64,
        word_map: &mut HashMap<WordKey, HashSet<i64>>,
    );
}

/// Vocabulary extractor using grammar-lib's vocabulary extraction
pub struct KagomeVocabularyExtractor;

impl KagomeVocabularyExtractor {
    pub fn new() -> Self {
        Self
    }
}

impl VocabularyExtractor for KagomeVocabularyExtractor {
    fn extract(
        &self,
        tokens: &[KagomeToken],
        auxiliary_indices: &HashSet<usize>,
        transcript_id: i64,
        word_map: &mut HashMap<WordKey, HashSet<i64>>,
    ) {
        // Use grammar-lib's vocabulary extraction
        let vocab_words = grammar_lib::extract_vocabulary(tokens, auxiliary_indices);

        // Convert to database format (map with transcript_id tracking)
        for word in vocab_words {
            word_map.entry(word).or_default().insert(transcript_id);
        }
    }
}

impl Default for KagomeVocabularyExtractor {
    fn default() -> Self {
        Self::new()
    }
}
