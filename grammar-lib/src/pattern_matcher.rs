use crate::matchers::matches as matcher_matches;
pub use crate::matchers::CustomMatcher;
use crate::types::KagomeToken;

#[derive(Debug, Clone)]
pub enum TokenMatcher {
    Verb {
        conjugation_form: Option<&'static str>,
        base_form: Option<&'static str>,
    },
    Surface(&'static str),
    Any,
    Custom(CustomMatcher),
}

/// Category of grammar pattern for filtering and vocabulary extraction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternCategory {
    /// Basic conjugation forms - detected but not stored as grammar patterns
    /// Used for vocabulary consolidation (skip auxiliary tokens)
    Conjugation,
    /// Actual grammatical constructions - stored as grammar patterns
    Construction,
}

#[derive(Debug, Clone)]
pub struct GrammarPattern {
    pub name: &'static str,
    pub tokens: Vec<TokenMatcher>,
    pub priority: u8, // Higher = more specific/important
    pub category: PatternCategory,
}

#[derive(Debug)]
pub struct PatternMatcher<T> {
    patterns: Vec<(GrammarPattern, T)>,
}

#[derive(Debug, Clone)]
pub struct PatternMatch<T> {
    #[allow(dead_code)]
    pub result: T,
    pub confidence: f32,
    pub pattern_name: &'static str,
    pub category: PatternCategory,
    /// 0-indexed character position where pattern starts (NOT a byte offset)
    /// To extract text in Rust, convert to byte position first using char_indices()
    pub start_char: u32,
    /// 0-indexed character position where pattern ends (NOT a byte offset)
    /// To extract text in Rust, convert to byte position first using char_indices()
    pub end_char: u32,
}

impl<T> PatternMatcher<T> {
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
        }
    }

    pub fn add_patterns(&mut self, patterns: Vec<(GrammarPattern, T)>) {
        self.patterns.extend(patterns);
    }
}

impl<T: Clone> PatternMatcher<T> {
    /// Match patterns against tokens, returning all matches sorted by confidence
    /// Also returns a set of token indices that are auxiliary (for vocabulary consolidation)
    pub fn match_tokens(
        &self,
        tokens: &[KagomeToken],
    ) -> (Vec<PatternMatch<T>>, std::collections::HashSet<usize>) {
        use std::collections::HashSet;

        let mut matches = Vec::new();
        let mut auxiliary_indices = HashSet::new();

        for start_pos in 0..tokens.len() {
            for (pattern, result) in &self.patterns {
                if let Some(match_result) =
                    self.match_pattern_at(pattern, tokens, start_pos, result)
                {
                    // Mark auxiliary tokens (all except the first token in the pattern)
                    let pattern_len = pattern.tokens.len();
                    for offset in 1..pattern_len {
                        auxiliary_indices.insert(start_pos + offset);
                    }

                    matches.push(match_result);
                }
            }
        }

        // Sort by confidence (descending), then by character length (descending)
        matches.sort_by(|a, b| {
            b.confidence
                .partial_cmp(&a.confidence)
                .unwrap()
                .then((b.end_char - b.start_char).cmp(&(a.end_char - a.start_char)))
            // Prefer longer matches
        });

        (matches, auxiliary_indices)
    }

    fn match_pattern_at(
        &self,
        pattern: &GrammarPattern,
        tokens: &[KagomeToken],
        start: usize,
        result: &T,
    ) -> Option<PatternMatch<T>> {
        if start + pattern.tokens.len() > tokens.len() {
            return None;
        }

        let mut specificity_score = 0.0;

        for (i, matcher) in pattern.tokens.iter().enumerate() {
            let token = &tokens[start + i];
            let (matches, score) = self.token_matches(matcher, token);

            if !matches {
                return None;
            }

            specificity_score += score;
        }

        // Calculate confidence based on pattern priority and specificity
        let confidence =
            (pattern.priority as f32) + (specificity_score / pattern.tokens.len() as f32);

        // Calculate character ranges from matched tokens
        let start_char = tokens[start].start;
        let end_char = tokens[start + pattern.tokens.len() - 1].end;

        Some(PatternMatch {
            result: result.clone(),
            confidence,
            pattern_name: pattern.name,
            category: pattern.category,
            start_char,
            end_char,
        })
    }

    fn token_matches(&self, matcher: &TokenMatcher, token: &KagomeToken) -> (bool, f32) {
        match matcher {
            TokenMatcher::Verb {
                conjugation_form,
                base_form,
            } => {
                if token.pos.first().is_none_or(|pos| pos != "動詞") {
                    return (false, 0.0);
                }

                let mut score = 1.0; // Base score for matching POS

                if let Some(expected_form) = conjugation_form {
                    if token.features.get(5).is_none_or(|f| f != expected_form) {
                        return (false, 0.0);
                    }
                    score += 2.0; // Higher score for specific conjugation
                }

                if let Some(expected_base) = base_form {
                    if &token.base_form != expected_base {
                        return (false, 0.0);
                    }
                    score += 3.0; // Highest score for specific verb
                }

                (true, score)
            }

            TokenMatcher::Surface(expected) => {
                if token.surface == *expected {
                    (true, 3.0) // High score for exact surface match
                } else {
                    (false, 0.0)
                }
            }

            TokenMatcher::Any => (true, 0.5), // Low score since it matches anything

            TokenMatcher::Custom(custom_matcher) => {
                let matches = matcher_matches(custom_matcher, token);
                if matches {
                    (true, 2.0) // Medium-high score for custom logic
                } else {
                    (false, 0.0)
                }
            }
        }
    }
}

impl<T> Default for PatternMatcher<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl TokenMatcher {
    pub fn verb_with_form(form: &'static str) -> Self {
        TokenMatcher::Verb {
            conjugation_form: Some(form),
            base_form: None,
        }
    }

    pub fn specific_verb(base_form: &'static str) -> Self {
        TokenMatcher::Verb {
            conjugation_form: None,
            base_form: Some(base_form),
        }
    }
}
