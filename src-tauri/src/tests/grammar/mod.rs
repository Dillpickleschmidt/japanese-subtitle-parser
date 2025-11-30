mod integration;
mod n1_patterns;
mod n2_patterns;
mod n3_patterns;
mod n4_patterns;
mod n5_patterns;

use crate::analysis::kagome_server::KagomeServer;
use crate::analysis::morphology::process_batch_with_kagome_server;
use grammar_lib::create_pattern_matcher;
use grammar_lib::types::KagomeToken;
use grammar_lib::{PatternMatch, PatternMatcher};
use std::sync::{LazyLock, Mutex};

/// Shared Kagome server for all tests (avoids port conflicts)
static KAGOME_SERVER: LazyLock<Mutex<KagomeServer>> = LazyLock::new(|| {
    Mutex::new(KagomeServer::start().expect("Failed to start Kagome server for tests"))
});

/// Tokenize a Japanese sentence using Kagome
pub fn tokenize_sentence(text: &str) -> Vec<KagomeToken> {
    let server = KAGOME_SERVER.lock().unwrap();
    let batch = vec![(1i64, 1i32, text.to_string())];
    let result = process_batch_with_kagome_server(&batch, &server).unwrap();
    result[0].clone()
}

/// Detect grammar patterns in a token sequence
pub fn detect_patterns(tokens: &[KagomeToken]) -> Vec<PatternMatch> {
    let matcher = create_pattern_matcher();
    let (matches, _auxiliary_indices) = matcher.match_tokens(tokens);
    matches
}

/// Check if a specific pattern was detected
pub fn has_pattern(matches: &[PatternMatch], pattern_name: &str) -> bool {
    matches.iter().any(|m| m.pattern_name == pattern_name)
}

/// Helper: Convert character position to byte position in a string
pub fn char_pos_to_byte_pos(s: &str, char_pos: usize) -> usize {
    s.char_indices()
        .nth(char_pos)
        .map(|(byte_pos, _)| byte_pos)
        .unwrap_or(s.len())
}

/// Get the text span covered by a pattern in the sentence
pub fn pattern_text(sentence: &str, pattern: &PatternMatch) -> String {
    let start_byte = char_pos_to_byte_pos(sentence, pattern.start_char as usize);
    let end_byte = char_pos_to_byte_pos(sentence, pattern.end_char as usize);
    sentence[start_byte..end_byte].to_string()
}

/// Assert that a pattern was detected
pub fn assert_has_pattern(matches: &[PatternMatch], pattern_name: &str) {
    assert!(
        has_pattern(matches, pattern_name),
        "Pattern '{}' not found",
        pattern_name
    );
}

/// Assert that a pattern would be selected (not filtered as redundant)
/// Uses the same logic as selectAndLayerGrammarPatterns in the TypeScript extension
pub fn assert_pattern_selected(
    matches: &[PatternMatch],
    tokens: &[KagomeToken],
    pattern_name: &str,
) {
    let selected = PatternMatcher::select_non_redundant_patterns(matches, tokens);
    assert!(
        selected.iter().any(|p| p.pattern_name == pattern_name),
        "Pattern '{}' exists but was filtered as redundant",
        pattern_name
    );
}

/// Assert that a pattern is at the expected character range
pub fn assert_pattern_range(
    matches: &[PatternMatch],
    pattern_name: &str,
    expected_start: u32,
    expected_end: u32,
) {
    let pattern = matches
        .iter()
        .find(|p| p.pattern_name == pattern_name)
        .unwrap_or_else(|| panic!("Pattern '{}' not found", pattern_name));

    assert_eq!(
        pattern.start_char, expected_start,
        "Pattern '{}': expected start {}, got {}",
        pattern_name, expected_start, pattern.start_char
    );
    assert_eq!(
        pattern.end_char, expected_end,
        "Pattern '{}': expected end {}, got {}",
        pattern_name, expected_end, pattern.end_char
    );
}

/// Print debug information about tokenization and pattern detection
pub fn print_debug(sentence: &str, tokens: &[KagomeToken], matches: &[PatternMatch]) {
    println!("\n=== Sentence: {} ===", sentence);
    println!("Tokens:");
    for token in tokens {
        let pos_str = token.pos.join("/");
        let features = if !token.features.is_empty() {
            format!(" [features: {}]", token.features.join(", "))
        } else {
            String::new()
        };
        println!(
            "  surface='{}' base='{}' pos={} start={} end={}{}",
            token.surface, token.base_form, pos_str, token.start, token.end, features
        );
    }
    println!("Patterns detected:");
    if matches.is_empty() {
        println!("  (none)");
    } else {
        for m in matches {
            let text = pattern_text(sentence, m);
            println!(
                "  {} (confidence: {:.1}) [chars {}-{}] = '{}'",
                m.pattern_name, m.confidence, m.start_char, m.end_char, text
            );
        }
    }
}
