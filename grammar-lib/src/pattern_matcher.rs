use std::sync::Arc;

use crate::types::KagomeToken;

// ============================================================================
// PUBLIC TYPES
// ============================================================================

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

#[derive(Debug, Clone)]
pub struct GrammarPattern {
    pub name: &'static str,
    pub tokens: Vec<TokenMatcher>,
    pub priority: u8, // Higher = more specific/important
    pub category: PatternCategory,
    pub jlpt_level: &'static str,
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
pub enum TokenMatcher {
    Verb {
        conjugation_form: Option<&'static str>,
        base_form: Option<&'static str>,
    },
    Adjective {
        base_form: Option<&'static str>,
    },
    Surface(&'static str),
    Any,
    Custom(Arc<dyn crate::matchers::Matcher>),
    /// Wildcard matcher - skips min to max tokens with optional stop conditions.
    /// NOTE: Only one wildcard per pattern is currently supported.
    /// When wildcard is encountered, remaining pattern is matched and result is returned immediately.
    /// stop_conditions: matchers to check - if any match, stop advancing the wildcard
    Wildcard {
        min: usize,
        max: usize,
        stop_conditions: Vec<TokenMatcher>,
    },
    /// Optional matcher - inner matcher is tried, but pattern continues if it doesn't match.
    /// If inner matcher succeeds, position advances. If it fails, position stays same (pattern skips it).
    Optional(Box<TokenMatcher>),
}

// ============================================================================
// PUBLIC API - Pattern matching and filtering
// ============================================================================

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

        // Extend construction patterns to include adjacent auxiliary verbs
        Self::extend_with_auxiliary_verbs(&mut matches, tokens);

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

    /// Select non-redundant patterns using token-set containment
    /// Filters out patterns whose token-set is completely contained in a higher-confidence pattern
    /// This matches the behavior of selectAndLayerGrammarPatterns in the TypeScript extension
    pub fn select_non_redundant_patterns(
        matches: &[PatternMatch<T>],
        tokens: &[KagomeToken],
    ) -> Vec<PatternMatch<T>> {
        use std::collections::HashSet;

        if matches.is_empty() || tokens.is_empty() {
            return Vec::new();
        }

        // Build token-set for each pattern
        let pattern_token_sets: Vec<HashSet<usize>> = matches
            .iter()
            .map(|pattern| {
                let mut token_set = HashSet::new();
                for (token_idx, token) in tokens.iter().enumerate() {
                    // Check if token overlaps with pattern character range
                    if pattern.start_char < token.end && pattern.end_char > token.start {
                        token_set.insert(token_idx);
                    }
                }
                token_set
            })
            .collect();

        // Sort matches by confidence (descending), then by length (descending)
        let mut indexed_matches: Vec<(usize, &PatternMatch<T>)> =
            matches.iter().enumerate().collect();
        indexed_matches.sort_by(|a, b| {
            b.1.confidence
                .partial_cmp(&a.1.confidence)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then((b.1.end_char - b.1.start_char).cmp(&(a.1.end_char - a.1.start_char)))
        });

        // Filter patterns: keep those not contained in higher-confidence patterns
        let mut selected_indices = Vec::new();

        for (orig_idx, _pattern) in &indexed_matches {
            let pattern_tokens = &pattern_token_sets[*orig_idx];

            // Check if this pattern's token-set is contained in any already-selected pattern
            let is_redundant = selected_indices.iter().any(|selected_idx: &usize| {
                let selected_tokens = &pattern_token_sets[*selected_idx];

                // Check if pattern_tokens ⊆ selected_tokens
                if pattern_tokens.len() > selected_tokens.len() {
                    return false;
                }

                pattern_tokens
                    .iter()
                    .all(|token_idx| selected_tokens.contains(token_idx))
            });

            if !is_redundant {
                selected_indices.push(*orig_idx);
            }
        }

        // Return selected patterns in original order
        selected_indices.sort();
        selected_indices
            .iter()
            .map(|idx| matches[*idx].clone())
            .collect()
    }

    // ========================================================================
    // PRIVATE HELPER METHODS
    // ========================================================================

    /// Extends all patterns ending with verbs or adjectives to include following auxiliary verbs
    /// e.g., te_iru (1,6) followed by ます (6,8) becomes (1,8)
    /// e.g., adjective (4,7) followed by です (7,9) becomes (4,9)
    fn extend_with_auxiliary_verbs(matches: &mut [PatternMatch<T>], tokens: &[KagomeToken]) {
        for pattern in matches {
            // Skip if pattern doesn't contain any verb or adjective
            let has_verb_or_adjective = tokens.iter().any(|token| {
                token.start >= pattern.start_char
                    && token.end <= pattern.end_char
                    && (token.pos.first().is_some_and(|pos| pos == "動詞")
                        || token.pos.first().is_some_and(|pos| pos == "形容詞")
                        || (token.pos.first().is_some_and(|pos| pos == "名詞")
                            && token.pos.get(1).is_some_and(|sub| sub == "形容動詞語幹")))
            });

            if !has_verb_or_adjective {
                continue;
            }

            // Find the next token (potential auxiliary verb)
            let next_token_idx = tokens.iter().position(|t| t.start == pattern.end_char);

            if let Some(idx) = next_token_idx {
                // Extend through consecutive auxiliary verbs
                let mut extend_to = pattern.end_char;
                let mut current_idx = idx;

                while current_idx < tokens.len()
                    && tokens[current_idx]
                        .pos
                        .first()
                        .is_some_and(|pos| pos == "助動詞")
                {
                    extend_to = tokens[current_idx].end;
                    current_idx += 1;
                }

                if extend_to > pattern.end_char {
                    pattern.end_char = extend_to;
                }
            }
        }
    }

    /// Try to match a pattern at a specific position in the token stream
    fn match_pattern_at(
        &self,
        pattern: &GrammarPattern,
        tokens: &[KagomeToken],
        start: usize,
        result: &T,
    ) -> Option<PatternMatch<T>> {
        if start >= tokens.len() {
            return None;
        }

        let mut specificity_score = 0.0;
        let mut current_pos = start;

        for (i, matcher) in pattern.tokens.iter().enumerate() {
            if current_pos >= tokens.len() {
                return None;
            }

            match matcher {
                TokenMatcher::Wildcard {
                    min,
                    max,
                    stop_conditions,
                } => {
                    // Delegate to wildcard-specific matching logic
                    return self.match_with_wildcard(
                        pattern,
                        tokens,
                        i,
                        current_pos,
                        specificity_score,
                        *min,
                        *max,
                        stop_conditions.clone(),
                        start,
                        result,
                    );
                }

                TokenMatcher::Optional(inner) => {
                    // Try to match optional, but don't fail if it doesn't match
                    let (matches, score) = self.token_matches(inner, &tokens[current_pos]);
                    if matches {
                        specificity_score += score;
                        current_pos += 1;
                    }
                    // If doesn't match, continue without advancing (pattern skips it)
                }

                _ => {
                    // Regular token matching
                    let token = &tokens[current_pos];
                    let (matches, score) = self.token_matches(matcher, token);

                    if !matches {
                        return None;
                    }

                    specificity_score += score;
                    current_pos += 1;
                }
            }
        }

        self.finalize_match(
            pattern,
            result,
            tokens,
            start,
            current_pos,
            specificity_score,
        )
    }

    /// Helper to finalize a successful match
    fn finalize_match(
        &self,
        pattern: &GrammarPattern,
        result: &T,
        tokens: &[KagomeToken],
        start: usize,
        end_pos: usize,
        specificity_score: f32,
    ) -> Option<PatternMatch<T>> {
        if end_pos == 0 || start >= tokens.len() {
            return None;
        }

        let confidence =
            (pattern.priority as f32) + (specificity_score / pattern.tokens.len() as f32);

        let mut start_char = tokens[start].start;
        let end_char = tokens[end_pos - 1].end;

        start_char = extend_for_preceding_suru_noun(tokens, start, start_char);

        Some(PatternMatch {
            result: result.clone(),
            confidence,
            pattern_name: pattern.name,
            category: pattern.category,
            start_char,
            end_char,
        })
    }

    /// Handle wildcard matching - tries skipping min to max tokens with stop conditions
    /// Returns the finalized match if successful, or None if wildcard didn't match
    fn match_with_wildcard(
        &self,
        pattern: &GrammarPattern,
        tokens: &[KagomeToken],
        wildcard_index: usize,
        current_pos: usize,
        specificity_score: f32,
        min: usize,
        max: usize,
        stop_conditions: Vec<TokenMatcher>,
        start: usize,
        result: &T,
    ) -> Option<PatternMatch<T>> {
        let mut matched = false;
        let mut final_pos = current_pos;
        let mut updated_score = specificity_score;

        for skip_count in min..=max {
            let check_pos = current_pos + skip_count;

            if check_pos >= tokens.len() {
                break;
            }

            // Check tokens in wildcard range for stop conditions
            let mut should_stop = false;
            for offset in 0..skip_count {
                let wildcard_token = &tokens[current_pos + offset];

                // Always stop at punctuation (sentence/clause boundary)
                if wildcard_token.pos.first().is_some_and(|pos| pos == "記号") {
                    should_stop = true;
                    break;
                }

                // Check if any stop condition matches this token
                for stop_condition in &stop_conditions {
                    let (matches, _score) = self.token_matches(stop_condition, wildcard_token);
                    if matches {
                        should_stop = true;
                        break;
                    }
                }

                if should_stop {
                    break;
                }
            }

            if should_stop {
                break;
            }

            // Collect remaining matchers after wildcard
            let remaining_matchers: Vec<_> =
                pattern.tokens.iter().skip(wildcard_index + 1).collect();

            // Try matching rest of pattern from check_pos
            if let Some(end_pos) =
                self.match_remaining_pattern_with_pos(&remaining_matchers, tokens, check_pos)
            {
                final_pos = end_pos;
                updated_score += 0.5 * skip_count as f32; // Lower score for more skips
                matched = true;
                break;
            }
        }

        if !matched {
            return None;
        }

        self.finalize_match(pattern, result, tokens, start, final_pos, updated_score)
    }

    /// Helper to match remaining tokens after wildcard and return the end position
    fn match_remaining_pattern_with_pos(
        &self,
        remaining: &[&TokenMatcher],
        tokens: &[KagomeToken],
        start_pos: usize,
    ) -> Option<usize> {
        let mut pos = start_pos;

        for matcher in remaining {
            if pos >= tokens.len() {
                return if remaining.is_empty() {
                    Some(pos)
                } else {
                    None
                };
            }

            // Don't support nested wildcards
            if matches!(matcher, TokenMatcher::Wildcard { .. }) {
                return None;
            }

            let (matches, _) = self.token_matches(matcher, &tokens[pos]);
            if !matches {
                return None;
            }

            pos += 1;
        }

        Some(pos)
    }

    /// Check if a token matcher matches a given token, returning match status and specificity score
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

            TokenMatcher::Adjective { base_form } => {
                // Match both i-adjectives (形容詞) and na-adjectives (名詞/形容動詞語幹)
                let is_i_adjective = token.pos.first().is_some_and(|pos| pos == "形容詞");
                let is_na_adjective = token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|sub| sub == "形容動詞語幹");

                if !is_i_adjective && !is_na_adjective {
                    return (false, 0.0);
                }

                let mut score = 1.0; // Base score for matching POS

                if let Some(expected_base) = base_form {
                    if &token.base_form != expected_base {
                        return (false, 0.0);
                    }
                    score += 3.0; // High score for specific adjective
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

            TokenMatcher::Custom(matcher) => {
                let matches = matcher.matches(token);
                if matches {
                    (true, 2.0) // Medium-high score for custom logic
                } else {
                    (false, 0.0)
                }
            }

            TokenMatcher::Wildcard { .. } => {
                // Wildcards don't match individual tokens, handled in match_pattern_at
                (false, 0.0)
            }

            TokenMatcher::Optional(inner) => {
                // Optional delegates to the inner matcher
                self.token_matches(inner, token)
            }
        }
    }
}

// ============================================================================
// UTILITY IMPLEMENTATIONS
// ============================================================================

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

    pub fn specific_verb_with_form(base_form: &'static str, form: &'static str) -> Self {
        TokenMatcher::Verb {
            conjugation_form: Some(form),
            base_form: Some(base_form),
        }
    }

    pub fn specific_adjective(base_form: &'static str) -> Self {
        TokenMatcher::Adjective {
            base_form: Some(base_form),
        }
    }
}

// ============================================================================
// PRIVATE HELPERS - utility functions
// ============================================================================

/// Extends start_char to include preceding サ変接続 noun if pattern starts with する verb
fn extend_for_preceding_suru_noun(
    tokens: &[KagomeToken],
    start_idx: usize,
    mut start_char: u32,
) -> u32 {
    if tokens[start_idx].base_form == "する" && start_idx > 0 {
        let prev_token = &tokens[start_idx - 1];
        // Check if previous token is サ変接続 noun
        if prev_token.pos.get(1).is_some_and(|p| p == "サ変接続") {
            start_char = prev_token.start;
        }
    }
    start_char
}
