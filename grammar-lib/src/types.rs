use serde::{Deserialize, Serialize};

use crate::compounds::CompoundSpan;
use crate::pattern_matcher::PatternMatch;

/// Kagome token structure from morphological analysis
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KagomeToken {
    #[serde(default)]
    pub id: u32,
    #[serde(default)]
    pub start: u32,
    #[serde(default)]
    pub end: u32,
    #[serde(default)]
    pub surface: String,
    #[serde(default)]
    pub class: String,
    pub pos: Vec<String>,
    pub base_form: String,
    pub reading: String,
    #[serde(default)]
    pub pronunciation: String,
    #[serde(default)]
    pub features: Vec<String>,
}

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
