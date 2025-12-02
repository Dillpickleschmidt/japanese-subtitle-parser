use std::collections::HashSet;
use std::sync::LazyLock;

use serde::{Deserialize, Serialize};

use crate::KagomeToken;

/// Compound expression span (indices into combined tokens)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompoundSpan {
    /// Start token index (inclusive)
    pub start: usize,
    /// End token index (inclusive)
    pub end: usize,
    /// The dictionary form of the compound (e.g., "目が覚める")
    pub text: String,
}

/// Embedded HashSet of ~298k compound expressions from JitEndex
pub(crate) static COMPOUNDS: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| include_str!("compounds.txt").lines().collect());

/// Find all compound expression spans in the given tokens.
/// Returns ALL matching compounds (overlapping allowed), sorted by length descending.
/// Consumer can decide which to display (e.g., longest only, or all).
pub fn find_compound_spans(tokens: &[KagomeToken]) -> Vec<CompoundSpan> {
    let mut spans = Vec::new();
    let max_span = 5;

    for start in 0..tokens.len() {
        for end in start..tokens.len().min(start + max_span) {
            // Skip single-token "compounds" - those are just regular words
            if end == start {
                continue;
            }

            let text = build_compound_text(&tokens[start..=end]);
            if COMPOUNDS.contains(text.as_str()) {
                spans.push(CompoundSpan { start, end, text });
            }
        }
    }

    // Sort by length descending (longest first), then by start position
    spans.sort_by(|a, b| {
        let len_a = a.end - a.start;
        let len_b = b.end - b.start;
        len_b.cmp(&len_a).then_with(|| a.start.cmp(&b.start))
    });

    spans
}

/// Build the compound lookup text from a slice of tokens.
/// Uses surface form for all tokens except the last one, which uses base_form.
/// This handles conjugated compounds like "知り尽くしていた" → "知り尽くす"
fn build_compound_text(tokens: &[KagomeToken]) -> String {
    let mut result = String::new();
    for (i, token) in tokens.iter().enumerate() {
        if i == tokens.len() - 1 {
            result.push_str(&token.base_form);
        } else {
            result.push_str(&token.surface);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_token(surface: &str, base_form: &str) -> KagomeToken {
        KagomeToken {
            id: 0,
            start: 0,
            end: 0,
            surface: surface.to_string(),
            class: String::new(),
            pos: vec!["名詞".to_string()],
            base_form: base_form.to_string(),
            reading: String::new(),
            pronunciation: String::new(),
            features: vec![],
        }
    }

    #[test]
    fn test_build_compound_text() {
        let tokens = vec![
            make_token("知り", "知る"),
            make_token("尽くしていた", "尽くす"),
        ];
        assert_eq!(build_compound_text(&tokens), "知り尽くす");
    }

    #[test]
    fn test_find_compound_spans_basic() {
        // This test requires the actual compounds.txt to be loaded
        // "目が覚める" should be in the compounds list
        let tokens = vec![
            make_token("目", "目"),
            make_token("が", "が"),
            make_token("覚めていた", "覚める"),
        ];
        let spans = find_compound_spans(&tokens);

        // Check if we found the compound
        let found = spans.iter().any(|s| s.text == "目が覚める");
        assert!(found, "Should find '目が覚める' compound");
    }
}
