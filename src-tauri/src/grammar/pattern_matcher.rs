use crate::analysis::morphology::KagomeToken;

// Pre-defined string constants to avoid repeated allocations
const VERB_POS: &str = "動詞";
const ADJECTIVE_POS: &str = "形容詞";
const AUXILIARY_VERB_POS: &str = "助動詞";
const RENYOU_FORM: &str = "連用形";
const RENYOU_TA_FORM: &str = "連用タ接続";

/// Represents a single token matching criterion
#[derive(Debug, Clone)]
pub enum TokenMatcher {
    /// Match verbs with optional constraints
    Verb {
        conjugation_form: Option<&'static str>,
        base_form: Option<&'static str>,
    },
    // Removed unused variants: Adjective, Adverb
    /// Match exact surface form
    Surface(&'static str),
    // Removed unused Pos variant
    /// Match anything (wildcard)
    Any,
    /// Custom matching with predefined types
    Custom(CustomMatcher),
}

/// Predefined custom matchers to avoid function pointer comparison issues
#[derive(Debug, Clone, PartialEq)]
pub enum CustomMatcher {
    /// Match たい as auxiliary verb or adjective
    TaiForm,
    /// Match たく from たい
    TakuForm,
    /// Match たかっ from たい  
    TakattaForm,
    /// Match なかっ from ない
    NakattaForm,
    /// Match なけれ from ない
    NakereForm,
    /// Match なく from ない
    NakuForm,
    /// Match させ from させる
    SaseForm,
    /// Match られる or れる (potential/passive)
    RareruForm,
    /// Match させる or せる (causative)
    CausativeForm,
    /// Match たら (conditional)
    TaraForm,
    /// Match だ or た (past auxiliary)
    PastAuxiliary,
    /// Match て or で (te-form particle)
    TeParticle,
    /// Match verb in 連用形 or 連用タ接続 (for flexible patterns)
    FlexibleVerbForm,
    /// Match specific verb forms for must patterns
    MustPattern,
}

/// Represents a complete grammar pattern
#[derive(Debug, Clone)]
pub struct GrammarPattern {
    pub name: &'static str,
    pub tokens: Vec<TokenMatcher>,
    pub priority: u8, // Higher = more specific/important
                      // Removed unused description field
}

/// Generic pattern matching engine
#[derive(Debug)]
pub struct PatternMatcher<T> {
    patterns: Vec<(GrammarPattern, T)>,
}

/// Result of pattern matching with confidence score
#[derive(Debug, Clone)]
pub struct PatternMatch<T> {
    #[allow(dead_code)]
    pub result: T,
    pub confidence: f32,
    pub pattern_name: &'static str,
    // Removed unused fields: matched_tokens, start_position
    pub end_position: usize, // Used for sorting by length preference
}

impl<T> PatternMatcher<T> {
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
        }
    }

    // Removed unused add_pattern method - patterns are added via add_patterns() during initialization

    pub fn add_patterns(&mut self, patterns: Vec<(GrammarPattern, T)>) {
        self.patterns.extend(patterns);
    }
}

impl<T: Clone> PatternMatcher<T> {
    /// Match patterns against tokens, returning all matches sorted by confidence
    pub fn match_tokens(&self, tokens: &[KagomeToken]) -> Vec<PatternMatch<T>> {
        let mut matches = Vec::new();

        // Try to match each pattern at each position
        for start_pos in 0..tokens.len() {
            for (pattern, result) in &self.patterns {
                if let Some(match_result) =
                    self.match_pattern_at(pattern, tokens, start_pos, result)
                {
                    matches.push(match_result);
                }
            }
        }

        // Sort by confidence (descending), then by priority (descending)
        matches.sort_by(|a, b| {
            b.confidence
                .partial_cmp(&a.confidence)
                .unwrap()
                .then(b.end_position.cmp(&a.end_position)) // Prefer longer matches
        });

        matches
    }

    // Unused methods removed: best_match(), matches_for_result()

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

        Some(PatternMatch {
            result: result.clone(),
            confidence,
            pattern_name: pattern.name,
            end_position: start + pattern.tokens.len() - 1,
        })
    }

    fn token_matches(&self, matcher: &TokenMatcher, token: &KagomeToken) -> (bool, f32) {
        match matcher {
            TokenMatcher::Verb {
                conjugation_form,
                base_form,
            } => {
                // Use constant reference to avoid string allocation
                if !token.pos.first().map_or(false, |pos| pos == VERB_POS) {
                    return (false, 0.0);
                }

                let mut score = 1.0; // Base score for matching POS

                if let Some(expected_form) = conjugation_form {
                    // Compare with string slice instead of creating String
                    if !token.features.get(5).map_or(false, |f| f == expected_form) {
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
                let matches = match custom_matcher {
                    CustomMatcher::TaiForm => {
                        token.surface == "たい"
                            && (token.pos.first().map_or(false, |pos| pos == ADJECTIVE_POS)
                                || token
                                    .pos
                                    .first()
                                    .map_or(false, |pos| pos == AUXILIARY_VERB_POS))
                    }
                    CustomMatcher::TakuForm => token.surface == "たく" && token.base_form == "たい",
                    CustomMatcher::TakattaForm => {
                        token.surface == "たかっ" && token.base_form == "たい"
                    }
                    CustomMatcher::NakattaForm => {
                        token.surface == "なかっ" && token.base_form == "ない"
                    }
                    CustomMatcher::NakereForm => {
                        token.surface == "なけれ" && token.base_form == "ない"
                    }
                    CustomMatcher::NakuForm => token.surface == "なく" && token.base_form == "ない",
                    CustomMatcher::SaseForm => {
                        token.surface == "させ" && token.base_form == "させる"
                    }
                    CustomMatcher::RareruForm => {
                        (token.surface == "られる" || token.surface == "れる")
                            && (token.base_form == "られる" || token.base_form == "れる")
                    }
                    CustomMatcher::CausativeForm => {
                        (token.surface == "させる" || token.surface == "せる")
                            && (token.base_form == "させる" || token.base_form == "せる")
                    }
                    CustomMatcher::TaraForm => {
                        token.surface == "たら"
                            && (token
                                .pos
                                .first()
                                .map_or(false, |pos| pos == AUXILIARY_VERB_POS)
                                || token.base_form == "た")
                    }
                    CustomMatcher::PastAuxiliary => {
                        (token.surface == "た" || token.surface == "だ")
                            && (token
                                .pos
                                .first()
                                .map_or(false, |pos| pos == AUXILIARY_VERB_POS)
                                || token.base_form == "た"
                                || token.base_form == "だ")
                    }
                    CustomMatcher::TeParticle => token.surface == "て" || token.surface == "で",
                    CustomMatcher::FlexibleVerbForm => {
                        if !token.pos.first().map_or(false, |pos| pos == VERB_POS) {
                            false
                        } else {
                            let form = token.features.get(5);
                            form.map_or(false, |f| f == RENYOU_FORM || f == RENYOU_TA_FORM)
                        }
                    }
                    CustomMatcher::MustPattern => {
                        token.surface == "なら"
                            || token.surface == "いけ"
                            || token.surface == "だめ"
                    }
                };

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

// Helper functions for common token matchers
impl TokenMatcher {
    // Removed unused verb() method

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
