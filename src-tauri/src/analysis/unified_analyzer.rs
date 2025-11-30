use crate::analysis::kagome_server::KagomeServer;
use crate::analysis::morphology::process_batch_with_kagome_server;
use crate::db::grammar_pattern::GrammarPatternCollector;
use crate::error::Error;
use grammar_lib::{
    create_pattern_matcher, extract_vocabulary, types::KagomeToken, PatternCategory,
    PatternMatcher, VocabWord,
};
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

static PATTERN_MATCHER: LazyLock<PatternMatcher> = LazyLock::new(|| create_pattern_matcher());

#[derive(Debug)]
pub struct UnifiedAnalysisResult {
    pub words: HashMap<VocabWord, HashSet<i64>>, // vocabulary word -> transcript_ids
    pub grammar_patterns: HashMap<i32, GrammarPatternCollector>, // episode_id -> collector
}

/// Analyze a batch of transcript lines, extracting both grammar patterns and vocabulary
pub fn analyze_batch(
    batch: &[(i64, i32, String)],
    server: &KagomeServer,
) -> Result<UnifiedAnalysisResult, Error> {
    let token_arrays = process_batch_with_kagome_server(batch, server)?;

    let estimated_word_capacity = (batch.len() * 18) * 10 / 7;
    let mut words: HashMap<VocabWord, HashSet<i64>> =
        HashMap::with_capacity(estimated_word_capacity);
    let estimated_episodes = (batch.len() / 20).max(1);
    let mut grammar_collectors = HashMap::with_capacity(estimated_episodes);

    for (line_idx, &(transcript_id, episode_id, _)) in batch.iter().enumerate() {
        if let Some(tokens) = token_arrays.get(line_idx) {
            if !tokens.is_empty() {
                // Analyze grammar patterns first to get auxiliary token indices
                let collector = grammar_collectors
                    .entry(episode_id)
                    .or_insert_with(GrammarPatternCollector::new);

                let auxiliary_indices = analyze_grammar_patterns(tokens, collector, transcript_id);

                // Extract vocabulary, skipping auxiliary tokens
                let vocab_words = extract_vocabulary(tokens, &auxiliary_indices);
                for word in vocab_words {
                    words.entry(word).or_default().insert(transcript_id);
                }
            }
        }
    }

    Ok(UnifiedAnalysisResult {
        words,
        grammar_patterns: grammar_collectors,
    })
}

/// Analyze grammar patterns in tokens and collect construction patterns
fn analyze_grammar_patterns(
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
