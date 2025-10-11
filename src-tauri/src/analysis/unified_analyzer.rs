use crate::analysis::kagome_server::KagomeServer;
use crate::analysis::morphology::{process_batch_with_kagome_server, KagomeToken};
use crate::db::grammar_pattern::GrammarPatternCollector;
use crate::error::Error;
use crate::grammar::{
    create_pattern_matcher, pattern_matcher::PatternMatcher, types::ConjugationPattern,
};
use std::collections::{HashMap, HashSet};
use std::sync::{LazyLock, Mutex};

static PATTERN_MATCHER: LazyLock<PatternMatcher<ConjugationPattern>> =
    LazyLock::new(|| create_pattern_matcher());

static POS_CACHE: LazyLock<Mutex<HashMap<Vec<String>, Vec<String>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static BASE_FORM_CACHE: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static READING_CACHE: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

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

fn intern_string(s: &str, cache: &LazyLock<Mutex<HashMap<String, String>>>) -> String {
    let mut intern_cache = cache.lock().unwrap();

    if let Some(cached) = intern_cache.get(s) {
        return cached.clone();
    }

    let owned = s.to_string();
    intern_cache.insert(owned.clone(), owned.clone());
    owned
}

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

pub struct UnifiedAnalyzer;

#[derive(Debug)]
pub struct UnifiedAnalysisResult {
    pub words: HashMap<WordKey, HashSet<i64>>, // word key -> transcript_ids
    pub grammar_patterns: HashMap<i32, GrammarPatternCollector>, // episode_id -> collector
}

impl UnifiedAnalyzer {
    pub fn new() -> Self {
        UnifiedAnalyzer
    }

    pub fn analyze_batch(
        &self,
        batch: &[(i64, i32, String)],
        server: &KagomeServer,
    ) -> Result<UnifiedAnalysisResult, Error> {
        let token_arrays = process_batch_with_kagome_server(batch, server)?;

        let estimated_word_capacity = (batch.len() * 18) * 10 / 7;
        let mut words = HashMap::with_capacity(estimated_word_capacity);
        let estimated_episodes = (batch.len() / 20).max(1);
        let mut grammar_collectors = HashMap::with_capacity(estimated_episodes);

        for (line_idx, &(transcript_id, episode_id, _)) in batch.iter().enumerate() {
            if let Some(tokens) = token_arrays.get(line_idx) {
                if !tokens.is_empty() {
                    self.extract_words_from_tokens(tokens, transcript_id, &mut words);

                    let collector = grammar_collectors
                        .entry(episode_id)
                        .or_insert_with(GrammarPatternCollector::new);

                    self.analyze_grammar_patterns(tokens, collector, transcript_id);
                }
            }
        }

        Ok(UnifiedAnalysisResult {
            words,
            grammar_patterns: grammar_collectors,
        })
    }

    fn extract_words_from_tokens(
        &self,
        tokens: &[KagomeToken],
        transcript_id: i64,
        word_map: &mut HashMap<WordKey, HashSet<i64>>,
    ) {
        for token in tokens {
            if self.is_content_word(token) {
                let key = WordKey::from_token(token);

                word_map
                    .entry(key)
                    .or_insert_with(HashSet::new)
                    .insert(transcript_id);
            }
        }
    }

    fn analyze_grammar_patterns(
        &self,
        tokens: &[KagomeToken],
        collector: &mut GrammarPatternCollector,
        transcript_id: i64,
    ) {
        let pattern_matches = PATTERN_MATCHER.match_tokens(tokens);

        for pattern_match in pattern_matches {
            collector.add_pattern(
                pattern_match.pattern_name.to_string(),
                transcript_id,
                pattern_match.confidence.into(),
            );
        }
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

impl Default for UnifiedAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
