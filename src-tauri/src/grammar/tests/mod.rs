mod integration;
mod n1_patterns;
mod n2_patterns;
mod n3_patterns;
mod n4_patterns;
mod n5_patterns;

use crate::analysis::kagome_server::KagomeServer;
use crate::analysis::morphology::{process_batch_with_kagome_server, KagomeToken};
use crate::grammar::create_pattern_matcher;
use crate::grammar::pattern_matcher::PatternMatch;
use crate::grammar::types::ConjugationPattern;
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
pub fn detect_patterns(tokens: &[KagomeToken]) -> Vec<PatternMatch<ConjugationPattern>> {
    let matcher = create_pattern_matcher();
    matcher.match_tokens(tokens)
}

/// Check if a specific pattern was detected
pub fn has_pattern(matches: &[PatternMatch<ConjugationPattern>], pattern_name: &str) -> bool {
    matches.iter().any(|m| m.pattern_name == pattern_name)
}

/// Print debug information about tokenization and pattern detection
pub fn print_debug(
    sentence: &str,
    tokens: &[KagomeToken],
    matches: &[PatternMatch<ConjugationPattern>],
) {
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
            "  surface='{}' base='{}' pos={}{}",
            token.surface, token.base_form, pos_str, features
        );
    }
    println!("Patterns detected:");
    if matches.is_empty() {
        println!("  (none)");
    } else {
        for m in matches {
            println!("  {} (confidence: {:.1})", m.pattern_name, m.confidence);
        }
    }
}
