use grammar_lib::types::{ConjugationPattern, KagomeToken};
use grammar_lib::{create_pattern_matcher, PatternCategory};
use serde::{Deserialize, Serialize};
use std::io::{self, Read};

/// Output format matching the WASM GrammarMatch interface
#[derive(Debug, Serialize, Deserialize)]
struct GrammarMatch {
    pattern_name: String,
    confidence: f32,
    start_char: u32,
    end_char: u32,
    category: String, // "Construction" or "Conjugation"
    conjugation_pattern: String,
}

/// Convert ConjugationPattern enum to string representation
fn conjugation_to_string(pattern: &ConjugationPattern) -> String {
    format!("{:?}", pattern)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read JSON tokens from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let tokens: Vec<KagomeToken> = serde_json::from_str(&input)?;

    // Create pattern matcher with all JLPT level patterns
    let matcher = create_pattern_matcher();

    // Match patterns against tokens
    let (pattern_matches, _auxiliary_indices) = matcher.match_tokens(&tokens);

    // Convert to output format
    let output: Vec<GrammarMatch> = pattern_matches
        .into_iter()
        .map(|m| GrammarMatch {
            pattern_name: m.pattern_name.to_string(),
            confidence: m.confidence,
            start_char: m.start_char,
            end_char: m.end_char,
            category: match m.category {
                PatternCategory::Construction => "Construction".to_string(),
                PatternCategory::Conjugation => "Conjugation".to_string(),
            },
            conjugation_pattern: conjugation_to_string(&m.result),
        })
        .collect();

    // Output JSON to stdout
    println!("{}", serde_json::to_string(&output)?);

    Ok(())
}
