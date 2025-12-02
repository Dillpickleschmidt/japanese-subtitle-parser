mod integration;
mod n1_patterns;
mod n2_patterns;
mod n3_patterns;
mod n4_patterns;
mod n5_patterns;

use crate::analysis::kagome_server::{KagomeServer, KagomeServerExt};
use crate::analysis::morphology::process_batch_with_kagome_server;
use grammar_lib::{pattern_text, select_best_patterns, KagomeToken, PatternMatch};
use std::sync::{LazyLock, Mutex};

/// Shared Kagome server for all tests (avoids port conflicts)
static KAGOME_SERVER: LazyLock<Mutex<KagomeServer>> = LazyLock::new(|| {
    // Pre-load heavy statics before starting server to avoid CPU contention during health checks
    grammar_lib::initialize();
    Mutex::new(KagomeServer::start_default().expect("Failed to start Kagome server for tests"))
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
    let text: String = tokens.iter().map(|t| t.surface.as_str()).collect();
    let result = grammar_lib::analyze(&text, tokens);
    result.grammar_matches
}

/// Check if a specific pattern was detected
pub fn has_pattern(matches: &[PatternMatch], pattern_name: &str) -> bool {
    matches.iter().any(|m| m.pattern_name == pattern_name)
}

/// Assert that a pattern was detected
pub fn assert_has_pattern(matches: &[PatternMatch], pattern_name: &str) {
    assert!(
        has_pattern(matches, pattern_name),
        "Pattern '{}' not found",
        pattern_name
    );
}

/// Assert that a pattern would be selected as best match
pub fn assert_pattern_selected(matches: &[PatternMatch], pattern_name: &str) {
    let match_refs: Vec<&PatternMatch> = matches.iter().collect();
    let selected = select_best_patterns(&match_refs);
    assert!(
        selected.iter().any(|p| p.pattern_name == pattern_name),
        "Pattern '{}' exists but was not selected as best match",
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
