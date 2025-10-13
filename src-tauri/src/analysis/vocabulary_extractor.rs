use crate::analysis::morphology::KagomeToken;
use std::collections::{HashMap, HashSet};
use std::sync::{LazyLock, Mutex};

// ========== String Interning (Internal Implementation) ==========

/// Global cache for POS tag interning
static POS_CACHE: LazyLock<Mutex<HashMap<Vec<String>, Vec<String>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Global cache for base form interning
static BASE_FORM_CACHE: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Global cache for reading interning
static READING_CACHE: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Intern POS tags to reduce memory usage
fn intern_pos_tags(pos: &[String]) -> Vec<String> {
    let mut cache = POS_CACHE.lock().unwrap();

    if let Some(cached) = cache.get(pos) {
        return cached.clone();
    }

    let owned_pos = pos.to_vec();
    let cached_pos = owned_pos.clone();
    cache.insert(owned_pos, cached_pos.clone());
    cached_pos
}

/// Intern string to reduce memory usage
fn intern_string(s: &str, cache: &LazyLock<Mutex<HashMap<String, String>>>) -> String {
    let mut intern_cache = cache.lock().unwrap();

    if let Some(cached) = intern_cache.get(s) {
        return cached.clone();
    }

    let owned = s.to_string();
    intern_cache.insert(owned.clone(), owned.clone());
    owned
}

// ========== Public API ==========

/// Efficient hash-friendly key for word deduplication with string interning
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WordKey {
    pub base_form: String,
    pub reading: String,
    pub pos: Vec<String>,
}

impl WordKey {
    pub fn from_token(token: &KagomeToken) -> Self {
        Self {
            base_form: intern_string(&token.base_form, &BASE_FORM_CACHE),
            reading: intern_string(&token.reading, &READING_CACHE),
            pos: intern_pos_tags(&token.pos),
        }
    }
}

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

/// Vocabulary extractor using Kagome morphological analysis
pub struct KagomeVocabularyExtractor;

impl KagomeVocabularyExtractor {
    pub fn new() -> Self {
        Self
    }

    fn is_content_word(&self, token: &KagomeToken) -> bool {
        if token.pos.is_empty() {
            return false;
        }

        let pos = &token.pos[0];

        // Filter out symbols/punctuation completely
        if pos == "記号" {
            return false;
        }

        matches!(pos.as_str(), "名詞" | "動詞" | "形容詞" | "副詞" | "感動詞")
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
        for (i, token) in tokens.iter().enumerate() {
            // Skip auxiliary tokens (parts of conjugations/constructions)
            if auxiliary_indices.contains(&i) {
                continue;
            }

            if self.is_content_word(token) {
                let key = WordKey::from_token(token);

                word_map.entry(key).or_default().insert(transcript_id);
            }
        }
    }
}

impl Default for KagomeVocabularyExtractor {
    fn default() -> Self {
        Self::new()
    }
}
