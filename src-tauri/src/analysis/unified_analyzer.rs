use crate::analysis::kagome_server::KagomeServer;
use crate::analysis::morphology::process_batch_with_kagome_server;
use crate::db::grammar_pattern::GrammarPatternCollector;
use crate::error::Error;
use grammar_lib::{extract_vocabulary, strip_parentheses, KagomeToken, PatternCategory, VocabWord};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct UnifiedAnalysisResult {
    pub words: HashMap<VocabWord, HashSet<i64>>, // vocabulary word -> transcript_ids
    pub grammar_patterns: HashMap<i32, GrammarPatternCollector>, // episode_id -> collector
}

/// Analyze a batch of transcript lines, extracting both grammar patterns and vocabulary.
/// Strips parentheses before tokenization to avoid false positives inside furigana.
pub fn analyze_batch(
    batch: &[(i64, i32, String)],
    server: &KagomeServer,
) -> Result<UnifiedAnalysisResult, Error> {
    // Strip parentheses and build batch for tokenization in one pass
    let (stripped_data, stripped_batch): (Vec<_>, Vec<_>) = batch
        .iter()
        .map(|(id, ep, text)| {
            let (stripped, char_map) = strip_parentheses(text);
            ((stripped.clone(), char_map), (*id, *ep, stripped))
        })
        .unzip();

    let token_arrays = process_batch_with_kagome_server(&stripped_batch, server)?;

    let estimated_word_capacity = (batch.len() * 18) * 10 / 7;
    let mut words: HashMap<VocabWord, HashSet<i64>> =
        HashMap::with_capacity(estimated_word_capacity);
    let estimated_episodes = (batch.len() / 20).max(1);
    let mut grammar_collectors = HashMap::with_capacity(estimated_episodes);

    for (line_idx, &(transcript_id, episode_id, ref _text)) in batch.iter().enumerate() {
        if let Some(tokens) = token_arrays.get(line_idx) {
            if !tokens.is_empty() {
                let (stripped, char_map) = &stripped_data[line_idx];

                // Convert kagome_client::KagomeToken -> grammar_lib::KagomeToken via serde
                let tokens: Vec<KagomeToken> =
                    serde_json::from_value(serde_json::to_value(tokens).unwrap()).unwrap();

                // Analyze, remapping if parentheses were stripped
                let result = match char_map {
                    Some(map) => grammar_lib::analyze_and_remap(stripped, &tokens, map),
                    None => grammar_lib::analyze(stripped, &tokens),
                };

                // Collect Construction patterns
                let collector = grammar_collectors
                    .entry(episode_id)
                    .or_insert_with(GrammarPatternCollector::new);

                for pattern_match in &result.grammar_matches {
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

                // Extract vocabulary from combined tokens
                let vocab_words = extract_vocabulary(&result.tokens);
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
