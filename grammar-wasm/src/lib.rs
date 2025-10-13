use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use grammar_lib::create_pattern_matcher;
use grammar_lib::pattern_matcher::PatternCategory;
use grammar_lib::types::KagomeToken;

/// Grammar match result for JavaScript
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarMatch {
    pub pattern_name: String,
    pub confidence: f32,
    pub start_char: u32,
    pub end_char: u32,
    pub category: String,
    pub conjugation_pattern: String,
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
            .map(|m| GrammarMatch {
                pattern_name: m.pattern_name.to_string(),
                confidence: m.confidence,
                start_char: m.start_char,
                end_char: m.end_char,
                category: match m.category {
                    PatternCategory::Construction => "Construction".to_string(),
                    PatternCategory::Conjugation => "Conjugation".to_string(),
                },
                conjugation_pattern: format!("{:?}", m.result),
            })
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
        .map(|m| GrammarMatch {
            pattern_name: m.pattern_name.to_string(),
            confidence: m.confidence,
            start_char: m.start_char,
            end_char: m.end_char,
            category: match m.category {
                PatternCategory::Construction => "Construction".to_string(),
                PatternCategory::Conjugation => "Conjugation".to_string(),
            },
            conjugation_pattern: format!("{:?}", m.result),
        })
        .collect();

    serde_wasm_bindgen::to_value(&js_matches)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize matches: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grammar_module_loads() {
        let _matcher = create_pattern_matcher();
    }
}
