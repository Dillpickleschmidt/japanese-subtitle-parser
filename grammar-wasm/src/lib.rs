use grammar_lib::KagomeToken;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Analyze a single subtitle: combines tokens, detects grammar patterns, finds compounds
#[wasm_bindgen]
pub fn analyze(text: &str, tokens_js: JsValue) -> Result<JsValue, JsValue> {
    let tokens: Vec<KagomeToken> = serde_wasm_bindgen::from_value(tokens_js)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse tokens: {}", e)))?;

    let result = grammar_lib::analyze(text, &tokens);

    serde_wasm_bindgen::to_value(&result)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize result: {}", e)))
}

/// Analyze batch of subtitles: combines tokens, detects grammar patterns, finds compounds
#[wasm_bindgen]
pub fn analyze_batch(texts_js: JsValue, token_arrays_js: JsValue) -> Result<JsValue, JsValue> {
    let texts: Vec<String> = serde_wasm_bindgen::from_value(texts_js)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse texts: {}", e)))?;
    let token_arrays: Vec<Vec<KagomeToken>> = serde_wasm_bindgen::from_value(token_arrays_js)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse tokens: {}", e)))?;

    if texts.len() != token_arrays.len() {
        return Err(JsValue::from_str(
            "texts and token_arrays must have same length",
        ));
    }

    let results: Vec<_> = texts
        .iter()
        .zip(token_arrays.iter())
        .map(|(text, tokens)| grammar_lib::analyze(text, tokens))
        .collect();

    serde_wasm_bindgen::to_value(&results)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize results: {}", e)))
}
