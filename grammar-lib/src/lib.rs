// Internal implementation modules
mod matchers;
mod pattern_matcher;
mod pattern_registry;
mod patterns;

// Public API modules
pub mod compounds;
pub mod text_utils;
pub mod token_combiner;
pub mod types;
pub mod vocabulary;

// Re-export types needed by consumers
pub use compounds::{find_compound_spans, CompoundSpan};
pub use pattern_matcher::{PatternCategory, PatternMatch};
pub use text_utils::{char_pos_to_byte_pos, pattern_text};
pub use pattern_registry::get_jlpt_level;
pub use token_combiner::{combine_conjugation_tokens, select_best_patterns};
pub use types::{AnalysisResult, KagomeToken};
pub use vocabulary::{extract_vocabulary, VocabWord};

// Internal helpers
use patterns::create_pattern_matcher;

/// Unified analysis function that combines tokens and detects compounds.
/// Returns combined tokens, grammar matches, and compound spans.
pub fn analyze(text: &str, tokens: &[KagomeToken]) -> AnalysisResult {
    let matcher = create_pattern_matcher();
    let (matches, _auxiliary_indices) = matcher.match_tokens(tokens);

    // Step 1: Combine tokens using conjugation patterns
    let combined_tokens = combine_conjugation_tokens(text, tokens, &matches);

    // Step 2: Find compound spans on combined tokens
    let compound_spans = find_compound_spans(&combined_tokens);

    AnalysisResult {
        tokens: combined_tokens,
        grammar_matches: matches,
        compound_spans,
    }
}

/// Strip parentheses from text before tokenization.
/// Returns (stripped_text, char_map) where char_map[stripped_idx] = original_idx.
/// Returns None for char_map if no parentheses found (no remapping needed).
/// Handles: ( ) （ ） 〔 〕 ｛ ｝ (not quote brackets 「」)
pub fn strip_parentheses(text: &str) -> (String, Option<Vec<u32>>) {
    let paren_regions = find_paren_regions(text);

    if paren_regions.is_empty() {
        return (text.to_string(), None);
    }

    let mut stripped = String::with_capacity(text.len());
    let mut char_map: Vec<u32> = Vec::with_capacity(text.chars().count());

    for (char_idx, (byte_pos, ch)) in text.char_indices().enumerate() {
        if !is_in_paren_region(byte_pos as u32, &paren_regions) {
            stripped.push(ch);
            char_map.push(char_idx as u32);
        }
    }

    (stripped, Some(char_map))
}

/// Analyze stripped text and remap pattern positions back to original.
/// Use after strip_parentheses() and tokenization.
/// Note: Token positions are NOT remapped (they remain relative to stripped text).
pub fn analyze_and_remap(
    stripped: &str,
    tokens: &[KagomeToken],
    char_map: &[u32],
) -> AnalysisResult {
    let mut result = analyze(stripped, tokens);

    // Remap pattern positions (character indices) to original text
    for m in &mut result.grammar_matches {
        m.start_char = remap_start(m.start_char, char_map);
        m.end_char = remap_end(m.end_char, char_map);
    }

    result
}

/// Find byte ranges of parentheses: ( ) （ ） 〔 〕 ｛ ｝
fn find_paren_regions(text: &str) -> Vec<(u32, u32)> {
    let mut regions = Vec::new();
    let mut stack: Vec<(u8, u32)> = Vec::new();

    for (byte_pos, ch) in text.char_indices() {
        let byte_pos = byte_pos as u32;
        let bracket = match ch {
            '(' => Some((0u8, true)),
            '（' => Some((1, true)),
            '〔' => Some((2, true)),
            '｛' => Some((3, true)),
            ')' => Some((0, false)),
            '）' => Some((1, false)),
            '〕' => Some((2, false)),
            '｝' => Some((3, false)),
            _ => None,
        };

        if let Some((btype, is_open)) = bracket {
            if is_open {
                stack.push((btype, byte_pos));
            } else if let Some((open_type, start)) = stack.pop() {
                if open_type == btype {
                    regions.push((start, byte_pos + ch.len_utf8() as u32));
                }
            }
        }
    }

    regions
}

#[inline]
fn is_in_paren_region(pos: u32, regions: &[(u32, u32)]) -> bool {
    regions.iter().any(|(start, end)| pos >= *start && pos < *end)
}

/// Remap a start position (inclusive) from stripped to original
#[inline]
fn remap_start(stripped_pos: u32, char_map: &[u32]) -> u32 {
    if (stripped_pos as usize) < char_map.len() {
        char_map[stripped_pos as usize]
    } else if !char_map.is_empty() {
        // Past end - extrapolate
        let last_stripped = (char_map.len() - 1) as u32;
        let last_original = char_map[char_map.len() - 1];
        last_original + (stripped_pos - last_stripped)
    } else {
        stripped_pos
    }
}

/// Remap an end position (exclusive) from stripped to original.
/// Maps the last included char and adds 1.
#[inline]
fn remap_end(stripped_end: u32, char_map: &[u32]) -> u32 {
    if stripped_end == 0 {
        return 0;
    }
    let last_char = stripped_end - 1;
    if (last_char as usize) < char_map.len() {
        char_map[last_char as usize] + 1
    } else if !char_map.is_empty() {
        // Past end - extrapolate
        let last_stripped = (char_map.len() - 1) as u32;
        let last_original = char_map[char_map.len() - 1];
        last_original + 1 + (last_char - last_stripped)
    } else {
        stripped_end
    }
}

/// Pre-initialize heavy statics (compounds dictionary).
/// Call this before other slow initializations to avoid resource contention.
pub fn initialize() {
    let _ = compounds::COMPOUNDS.len();
}

#[cfg(test)]
mod tests;
