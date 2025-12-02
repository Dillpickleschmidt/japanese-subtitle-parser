use serde::Serialize;

use crate::compounds::CompoundSpan;
use crate::pattern_matcher::PatternMatch;
use kagome_client::KagomeToken;

/// Result of unified text analysis
#[derive(Debug, Clone, Serialize)]
pub struct AnalysisResult {
    /// Combined tokens (conjugations merged)
    pub tokens: Vec<KagomeToken>,
    /// All grammar pattern matches (both Construction and Conjugation)
    pub grammar_matches: Vec<PatternMatch>,
    /// Compound expression spans (indices into combined tokens)
    pub compound_spans: Vec<CompoundSpan>,
}
