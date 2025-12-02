use grammar_lib::{extract_vocabulary, KagomeToken, PatternCategory, VocabWord};
use serde::{Deserialize, Serialize};
use std::io::{self, Read};

#[derive(Debug, Serialize, Deserialize)]
struct PatternMatch {
    pattern_name: String,
    confidence: f32,
    start_char: u32,
    end_char: u32,
    category: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CompoundSpan {
    start: usize,
    end: usize,
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AnalysisOutput {
    tokens: Vec<KagomeToken>,
    grammar_matches: Vec<PatternMatch>,
    compound_spans: Vec<CompoundSpan>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AnalysisWithVocabulary {
    tokens: Vec<KagomeToken>,
    grammar_matches: Vec<PatternMatch>,
    compound_spans: Vec<CompoundSpan>,
    vocabulary: Vec<VocabWord>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let with_vocabulary = std::env::args().any(|arg| arg == "--with-vocabulary");

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let tokens: Vec<KagomeToken> = serde_json::from_str(&input)?;
    let text: String = tokens.iter().map(|t| t.surface.as_str()).collect();

    let result = grammar_lib::analyze(&text, &tokens);

    let grammar_output: Vec<PatternMatch> = result
        .grammar_matches
        .iter()
        .map(|m| PatternMatch {
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

    let compound_output: Vec<CompoundSpan> = result
        .compound_spans
        .iter()
        .map(|c| CompoundSpan {
            start: c.start,
            end: c.end,
            text: c.text.clone(),
        })
        .collect();

    if with_vocabulary {
        let vocabulary = extract_vocabulary(&result.tokens);
        let output = AnalysisWithVocabulary {
            tokens: result.tokens,
            grammar_matches: grammar_output,
            compound_spans: compound_output,
            vocabulary,
        };
        println!("{}", serde_json::to_string(&output)?);
    } else {
        let output = AnalysisOutput {
            tokens: result.tokens,
            grammar_matches: grammar_output,
            compound_spans: compound_output,
        };
        println!("{}", serde_json::to_string(&output)?);
    }

    Ok(())
}
