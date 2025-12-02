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
pub use types::AnalysisResult;
pub use vocabulary::{extract_vocabulary, VocabWord};

// Internal helpers
use kagome_client::KagomeToken;
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

/// Pre-initialize heavy statics (compounds dictionary).
/// Call this before other slow initializations to avoid resource contention.
pub fn initialize() {
    let _ = compounds::COMPOUNDS.len();
}

#[cfg(test)]
mod tests;
