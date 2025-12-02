use grammar_lib::{extract_vocabulary, PatternCategory, VocabWord};
use kagome_client::KagomeToken;
use serde::{Deserialize, Serialize};
use std::io::{self, Read};

/// Output format for grammar patterns
#[derive(Debug, Serialize, Deserialize)]
struct GrammarMatch {
    pattern_name: String,
    confidence: f32,
    start_char: u32,
    end_char: u32,
    category: String, // "Construction" or "Conjugation"
}

/// Combined output format when --with-vocabulary flag is used
#[derive(Debug, Serialize, Deserialize)]
struct AnalysisWithVocabulary {
    grammar_patterns: Vec<GrammarMatch>,
    vocabulary: Vec<VocabWord>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check for --with-vocabulary flag
    let with_vocabulary = std::env::args().any(|arg| arg == "--with-vocabulary");

    // Read JSON tokens from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let tokens: Vec<KagomeToken> = serde_json::from_str(&input)?;

    // Reconstruct text from token surface forms (needed for analyze())
    let text: String = tokens.iter().map(|t| t.surface.as_str()).collect();

    // Use unified analyze() function
    let result = grammar_lib::analyze(&text, &tokens);

    // Convert grammar patterns to output format
    let grammar_output: Vec<GrammarMatch> = result
        .grammar_matches
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
        })
        .collect();

    // Output JSON to stdout
    if with_vocabulary {
        // Extract vocabulary from combined tokens
        let vocabulary = extract_vocabulary(&result.tokens);
        let combined_output = AnalysisWithVocabulary {
            grammar_patterns: grammar_output,
            vocabulary,
        };
        println!("{}", serde_json::to_string(&combined_output)?);
    } else {
        println!("{}", serde_json::to_string(&grammar_output)?);
    }

    Ok(())
}
