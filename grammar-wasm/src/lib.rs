use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use grammar_lib::PatternCategory;
use grammar_lib::PatternMatch;
use grammar_lib::VocabWord;
use grammar_lib::create_pattern_matcher;
use grammar_lib::extract_vocabulary;
use grammar_lib::types::KagomeToken;

/// Grammar match result for JavaScript
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarMatch {
    pub pattern_name: String,
    pub confidence: f32,
    pub start_char: u32,
    pub end_char: u32,
    pub category: String,
}

/// Combined analysis result with both grammar patterns and vocabulary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisWithVocabulary {
    pub grammar_patterns: Vec<GrammarMatch>,
    pub vocabulary: Vec<VocabWord>,
}

/// Convert a PatternMatch from grammar-lib to a GrammarMatch for JavaScript
fn convert_to_grammar_match(m: PatternMatch) -> GrammarMatch {
    GrammarMatch {
        pattern_name: m.pattern_name.to_string(),
        confidence: m.confidence,
        start_char: m.start_char,
        end_char: m.end_char,
        category: match m.category {
            PatternCategory::Construction => "Construction".to_string(),
            PatternCategory::Conjugation => "Conjugation".to_string(),
        },
    }
}

#[wasm_bindgen]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Analyze batch of subtitles, returns array of match arrays
#[wasm_bindgen]
pub fn analyze_batch(token_arrays_js: JsValue) -> Result<JsValue, JsValue> {
    let token_arrays: Vec<Vec<KagomeToken>> = serde_wasm_bindgen::from_value(token_arrays_js)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse tokens: {}", e)))?;

    let matcher = create_pattern_matcher();
    let mut all_matches = Vec::new();

    for tokens in token_arrays {
        let (pattern_matches, _auxiliary_indices) = matcher.match_tokens(&tokens);

        let js_matches: Vec<GrammarMatch> = pattern_matches
            .into_iter()
            .map(convert_to_grammar_match)
            .collect();

        all_matches.push(js_matches);
    }

    serde_wasm_bindgen::to_value(&all_matches)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize matches: {}", e)))
}

/// Analyze single subtitle's tokens
#[wasm_bindgen]
pub fn analyze_single(tokens_js: JsValue) -> Result<JsValue, JsValue> {
    let tokens: Vec<KagomeToken> = serde_wasm_bindgen::from_value(tokens_js)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse tokens: {}", e)))?;

    let matcher = create_pattern_matcher();
    let (pattern_matches, _auxiliary_indices) = matcher.match_tokens(&tokens);

    let js_matches: Vec<GrammarMatch> = pattern_matches
        .into_iter()
        .map(convert_to_grammar_match)
        .collect();

    serde_wasm_bindgen::to_value(&js_matches)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize matches: {}", e)))
}

/// Analyze single subtitle's tokens with vocabulary extraction
#[wasm_bindgen]
pub fn analyze_single_all(tokens_js: JsValue) -> Result<JsValue, JsValue> {
    let tokens: Vec<KagomeToken> = serde_wasm_bindgen::from_value(tokens_js)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse tokens: {}", e)))?;

    let matcher = create_pattern_matcher();
    let (pattern_matches, auxiliary_indices) = matcher.match_tokens(&tokens);

    let js_matches: Vec<GrammarMatch> = pattern_matches
        .into_iter()
        .map(convert_to_grammar_match)
        .collect();

    let vocabulary = extract_vocabulary(&tokens, &auxiliary_indices);

    let result = AnalysisWithVocabulary {
        grammar_patterns: js_matches,
        vocabulary,
    };

    serde_wasm_bindgen::to_value(&result)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize result: {}", e)))
}

/// Analyze batch of subtitles, returns array of match arrays
#[wasm_bindgen]
pub fn analyze_batch_all(token_arrays_js: JsValue) -> Result<JsValue, JsValue> {
    let token_arrays: Vec<Vec<KagomeToken>> = serde_wasm_bindgen::from_value(token_arrays_js)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse tokens: {}", e)))?;

    let matcher = create_pattern_matcher();
    let mut all_results = Vec::new();

    for tokens in token_arrays {
        let (pattern_matches, auxiliary_indices) = matcher.match_tokens(&tokens);

        let js_matches: Vec<GrammarMatch> = pattern_matches
            .into_iter()
            .map(convert_to_grammar_match)
            .collect();

        let vocabulary = extract_vocabulary(&tokens, &auxiliary_indices);

        let result = AnalysisWithVocabulary {
            grammar_patterns: js_matches,
            vocabulary,
        };

        all_results.push(result);
    }

    serde_wasm_bindgen::to_value(&all_results)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize results: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grammar_module_loads() {
        let _matcher = create_pattern_matcher();
    }
}
