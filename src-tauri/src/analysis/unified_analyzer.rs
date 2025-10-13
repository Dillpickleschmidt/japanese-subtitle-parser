use crate::analysis::kagome_server::KagomeServer;
use crate::analysis::morphology::process_batch_with_kagome_server;
use crate::analysis::vocabulary_extractor::{
    KagomeVocabularyExtractor, VocabularyExtractor, WordKey,
};
use crate::db::grammar_pattern::GrammarPatternCollector;
use crate::error::Error;
use grammar_lib::{
    create_pattern_matcher,
    pattern_matcher::{PatternCategory, PatternMatcher},
    types::{ConjugationPattern, KagomeToken},
};
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

static PATTERN_MATCHER: LazyLock<PatternMatcher<ConjugationPattern>> =
    LazyLock::new(|| create_pattern_matcher());

pub struct UnifiedAnalyzer {
    extractor: KagomeVocabularyExtractor,
}

#[derive(Debug)]
pub struct UnifiedAnalysisResult {
    pub words: HashMap<WordKey, HashSet<i64>>, // word key -> transcript_ids
    pub grammar_patterns: HashMap<i32, GrammarPatternCollector>, // episode_id -> collector
}

impl UnifiedAnalyzer {
    pub fn new() -> Self {
        UnifiedAnalyzer {
            extractor: KagomeVocabularyExtractor::new(),
        }
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
                    // Analyze grammar patterns first to get auxiliary token indices
                    let collector = grammar_collectors
                        .entry(episode_id)
                        .or_insert_with(GrammarPatternCollector::new);

                    let auxiliary_indices =
                        self.analyze_grammar_patterns(tokens, collector, transcript_id);

                    // Extract vocabulary, skipping auxiliary tokens
                    self.extractor
                        .extract(tokens, &auxiliary_indices, transcript_id, &mut words);
                }
            }
        }

        Ok(UnifiedAnalysisResult {
            words,
            grammar_patterns: grammar_collectors,
        })
    }

    fn analyze_grammar_patterns(
        &self,
        tokens: &[KagomeToken],
        collector: &mut GrammarPatternCollector,
        transcript_id: i64,
    ) -> HashSet<usize> {
        let (pattern_matches, auxiliary_indices) = PATTERN_MATCHER.match_tokens(tokens);

        // Only store Construction patterns (skip basic Conjugation patterns)
        for pattern_match in pattern_matches {
            if pattern_match.category == PatternCategory::Construction {
                collector.add_pattern(
                    pattern_match.pattern_name.to_string(),
                    transcript_id,
                    pattern_match.confidence.into(),
                    pattern_match.start_char,
                    pattern_match.end_char,
                );
            }
        }

        auxiliary_indices
    }
}

impl Default for UnifiedAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
