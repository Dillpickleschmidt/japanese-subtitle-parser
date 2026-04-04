use std::sync::Arc;

use crate::KagomeToken;
use serde::{Deserialize, Serialize};

// ============================================================================
// PUBLIC TYPES
// ============================================================================

/// Context for pattern matching - provides access to token stream for lookahead.
///
/// `lookahead(n)` and `lookbehind(n)` are sentence-break-aware for n > 1:
/// they return None if any intermediate token (between current and target) is
/// sentence-ending punctuation. For n == 1, they always return the adjacent
/// token directly.
pub struct MatchContext<'a> {
    pub tokens: &'a [KagomeToken],
    pub position: usize,
}

impl<'a> MatchContext<'a> {
    pub fn current(&self) -> Option<&'a KagomeToken> {
        self.tokens.get(self.position)
    }

    #[allow(dead_code)]
    pub fn lookahead(&self, n: usize) -> Option<&'a KagomeToken> {
        // For n > 1, check that no intermediate token is a sentence break
        for i in 1..n {
            if let Some(t) = self.tokens.get(self.position + i) {
                if is_sentence_break(t) {
                    return None;
                }
            }
        }
        self.tokens.get(self.position + n)
    }

    #[allow(dead_code)]
    pub fn lookbehind(&self, n: usize) -> Option<&'a KagomeToken> {
        if self.position < n {
            return None;
        }
        // For n > 1, check that no intermediate token is a sentence break
        for i in 1..n {
            if let Some(t) = self.tokens.get(self.position - i) {
                if is_sentence_break(t) {
                    return None;
                }
            }
        }
        self.tokens.get(self.position - n)
    }
}

/// Check if a token is sentence-ending punctuation
fn is_sentence_break(token: &KagomeToken) -> bool {
    matches!(token.surface.as_str(), "。" | "！" | "？" | "!" | "?")
        || (token.pos.first().is_some_and(|p| p == "記号")
            && token.pos.get(1).is_some_and(|p| p == "句点"))
}

#[derive(Debug)]
pub struct PatternMatcher {
    patterns: Vec<GrammarPattern>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PatternMatch {
    pub confidence: f32,
    pub pattern_name: &'static str,
    pub category: PatternCategory,
    /// 0-indexed character position where pattern starts (NOT a byte offset)
    pub start_char: u32,
    /// 0-indexed character position where pattern ends (NOT a byte offset)
    pub end_char: u32,
}

#[derive(Debug, Clone)]
pub struct GrammarPattern {
    pub name: &'static str,
    pub tokens: Vec<TokenMatcher>,
    pub priority: u8,
    pub category: PatternCategory,
    pub jlpt_level: &'static str,
}

/// Category of grammar pattern for filtering and vocabulary extraction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatternCategory {
    /// Basic conjugation forms - detected but not stored as grammar patterns
    Conjugation,
    /// Actual grammatical constructions - stored as grammar patterns
    Construction,
}

#[derive(Debug, Clone)]
pub enum TokenMatcher {
    /// Match exact surface form
    Surface(&'static str),
    /// Match any token
    Any,
    /// Custom matcher with full context access
    Custom(Arc<dyn crate::matchers::Matcher>),
    /// Skip min to max tokens (only one per pattern supported)
    /// stop_conditions: matchers that when matched, stop the wildcard
    /// allow_commas: if true, commas (読点) don't stop the wildcard
    Wildcard {
        min: usize,
        max: usize,
        stop_conditions: Vec<TokenMatcher>,
        allow_commas: bool,
    },
    /// Optional - try to match, continue either way
    Optional(Box<TokenMatcher>),
    /// Match first successful alternative
    Or(Vec<TokenMatcher>),
}

// ============================================================================
// PUBLIC API - Pattern matching and filtering
// ============================================================================

impl PatternMatcher {
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
        }
    }

    pub fn add_patterns(&mut self, patterns: Vec<GrammarPattern>) {
        self.patterns.extend(patterns);
    }
}

impl PatternMatcher {
    /// Match patterns against tokens, returning all matches sorted by confidence
    /// Also returns a set of token indices that are auxiliary (for vocabulary consolidation)
    pub fn match_tokens(
        &self,
        tokens: &[KagomeToken],
    ) -> (Vec<PatternMatch>, std::collections::HashSet<usize>) {
        use std::collections::HashSet;

        let mut matches = Vec::new();
        let mut auxiliary_indices = HashSet::new();

        for start_pos in 0..tokens.len() {
            for pattern in &self.patterns {
                if let Some(match_result) = self.match_pattern_at(pattern, tokens, start_pos) {
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
                .unwrap_or(std::cmp::Ordering::Equal) // Handle NaN by treating as equal
                .then((b.end_char - b.start_char).cmp(&(a.end_char - a.start_char)))
            // Prefer longer matches
        });

        (matches, auxiliary_indices)
    }

    // ========================================================================
    // PRIVATE HELPER METHODS
    // ========================================================================

    /// Extends all patterns ending with verbs or adjectives to include following auxiliary verbs
    /// e.g., te_iru (1,6) followed by ます (6,8) becomes (1,8)
    /// e.g., adjective (4,7) followed by です (7,9) becomes (4,9)
    fn extend_with_auxiliary_verbs(matches: &mut [PatternMatch], tokens: &[KagomeToken]) {
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
                    let aux_token = &tokens[current_idx];

                    // Don't extend Conjugation patterns through だろう/でしょう
                    // These are standalone patterns that apply to the whole verb, not part of the conjugation
                    // BUT Construction patterns can include だろう as part of the expression (e.g., といえるだろう)
                    if pattern.category == PatternCategory::Conjugation {
                        if aux_token.base_form == "だ" && aux_token.surface == "だろ" {
                            break;
                        }
                        if aux_token.base_form == "です" && aux_token.surface == "でしょ" {
                            break;
                        }
                    }

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
    ) -> Option<PatternMatch> {
        if pattern.tokens.is_empty() || start >= tokens.len() {
            return None;
        }

        let mut specificity_score = 0.0;
        let mut current_pos = start;

        for (i, matcher) in pattern.tokens.iter().enumerate() {
            match matcher {
                TokenMatcher::Wildcard { min, max, stop_conditions, allow_commas } => {
                    return self.match_with_wildcard(
                        pattern,
                        tokens,
                        i,
                        current_pos,
                        specificity_score,
                        *min,
                        *max,
                        stop_conditions,
                        *allow_commas,
                        start,
                    );
                }

                TokenMatcher::Optional(inner) => {
                    if current_pos < tokens.len() {
                        let ctx = MatchContext { tokens, position: current_pos };
                        let (matches, score, consumed) = Self::token_matches_ctx(inner, &ctx);
                        if matches {
                            specificity_score += score;
                            current_pos += consumed;
                        }
                    }
                }

                _ => {
                    if current_pos >= tokens.len() {
                        return None;
                    }

                    let ctx = MatchContext { tokens, position: current_pos };
                    let (matches, score, consumed) = Self::token_matches_ctx(matcher, &ctx);

                    if !matches {
                        return None;
                    }

                    specificity_score += score;
                    current_pos += consumed;
                }
            }
        }

        self.finalize_match(pattern, tokens, start, current_pos, specificity_score)
    }

    /// Helper to finalize a successful match
    fn finalize_match(
        &self,
        pattern: &GrammarPattern,
        tokens: &[KagomeToken],
        start: usize,
        end_pos: usize,
        specificity_score: f32,
    ) -> Option<PatternMatch> {
        if end_pos == 0 || start >= tokens.len() {
            return None;
        }

        let confidence =
            (pattern.priority as f32) + (specificity_score / pattern.tokens.len() as f32);

        let mut start_char = tokens[start].start;
        let end_char = tokens[end_pos - 1].end;

        start_char = extend_for_preceding_suru_noun(tokens, start, start_char);

        Some(PatternMatch {
            confidence,
            pattern_name: pattern.name,
            category: pattern.category,
            start_char,
            end_char,
        })
    }

    /// Handle wildcard matching - tries skipping min to max tokens
    /// Returns the finalized match if successful, or None if wildcard didn't match
    #[allow(clippy::too_many_arguments)]
    fn match_with_wildcard(
        &self,
        pattern: &GrammarPattern,
        tokens: &[KagomeToken],
        wildcard_index: usize,
        current_pos: usize,
        specificity_score: f32,
        min: usize,
        max: usize,
        stop_conditions: &[TokenMatcher],
        allow_commas: bool,
        start: usize,
    ) -> Option<PatternMatch> {
        let remaining_matchers: Vec<_> = pattern.tokens.iter().skip(wildcard_index + 1).collect();
        let mut partial_match: Option<PatternMatch> = None;

        for skip_count in min..=max {
            let check_pos = current_pos + skip_count;

            if check_pos >= tokens.len() {
                break;
            }

            // Check for stop conditions and punctuation
            let mut should_stop = false;
            for offset in 0..skip_count {
                let pos = current_pos + offset;
                // Stop at punctuation (allow commas if flag is set)
                if tokens[pos].pos.first().is_some_and(|p| p == "記号") {
                    if !(allow_commas && tokens[pos].pos.get(1).is_some_and(|p| p == "読点")) {
                        should_stop = true;
                        break;
                    }
                }
                // Stop if any stop_condition matches
                let ctx = MatchContext { tokens, position: pos };
                for stop_cond in stop_conditions {
                    let (matches, _, _) = Self::token_matches_ctx(stop_cond, &ctx);
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

            if let Some((end_pos, all_optionals_matched)) = self.match_remaining_pattern(tokens, &remaining_matchers, check_pos) {
                let updated_score = specificity_score + 0.5 * skip_count as f32;
                if all_optionals_matched {
                    // All Optionals matched - this is a complete match, return immediately
                    return self.finalize_match(pattern, tokens, start, end_pos, updated_score);
                }
                // Some Optionals didn't match - save as fallback, keep looking
                // for a match where they do
                if partial_match.is_none() {
                    partial_match = self.finalize_match(pattern, tokens, start, end_pos, updated_score);
                }
            }
        }

        partial_match
    }

    /// Helper to match remaining tokens after wildcard and return the end position
    /// Returns (end_pos, all_optionals_matched) - the bool indicates if all Optional
    /// matchers actually matched their inner content
    fn match_remaining_pattern(
        &self,
        tokens: &[KagomeToken],
        remaining: &[&TokenMatcher],
        start_pos: usize,
    ) -> Option<(usize, bool)> {
        let mut pos = start_pos;
        let mut all_optionals_matched = true;

        for matcher in remaining {
            if pos >= tokens.len() {
                // Allow trailing Optional matchers at end of tokens
                if matches!(matcher, TokenMatcher::Optional(_)) {
                    all_optionals_matched = false;
                    continue;
                }
                return None;
            }

            // Don't support nested wildcards
            if matches!(matcher, TokenMatcher::Wildcard { .. }) {
                return None;
            }

            // Handle Optional matchers - don't fail if inner doesn't match
            if let TokenMatcher::Optional(inner) = matcher {
                let ctx = MatchContext { tokens, position: pos };
                let (matches, _, consumed) = Self::token_matches_ctx(inner, &ctx);
                if matches {
                    pos += consumed;
                } else {
                    all_optionals_matched = false;
                }
                continue;
            }

            let ctx = MatchContext { tokens, position: pos };
            let (matches, _, consumed) = Self::token_matches_ctx(matcher, &ctx);
            if !matches {
                return None;
            }

            pos += consumed;
        }

        Some((pos, all_optionals_matched))
    }

    /// Check if a token matcher matches at current context position
    /// Returns (matches, score, tokens_consumed)
    fn token_matches_ctx(matcher: &TokenMatcher, ctx: &MatchContext) -> (bool, f32, usize) {
        match matcher {
            TokenMatcher::Surface(expected) => {
                if let Some(token) = ctx.current() {
                    if token.surface == *expected {
                        return (true, 3.0, 1);
                    }
                }
                (false, 0.0, 0)
            }

            TokenMatcher::Any => {
                if ctx.current().is_some() {
                    (true, 0.5, 1)
                } else {
                    (false, 0.0, 0)
                }
            }

            TokenMatcher::Custom(matcher) => {
                let (matches, consumed) = matcher.matches(ctx);
                if matches {
                    (true, 2.0, consumed)
                } else {
                    (false, 0.0, 0)
                }
            }

            TokenMatcher::Wildcard { .. } => {
                // Wildcards handled separately in match_pattern_at
                (false, 0.0, 0)
            }

            TokenMatcher::Optional(inner) => {
                Self::token_matches_ctx(inner, ctx)
            }

            TokenMatcher::Or(alternatives) => {
                for alt in alternatives {
                    let (matches, score, consumed) = Self::token_matches_ctx(alt, ctx);
                    if matches {
                        return (matches, score, consumed);
                    }
                }
                (false, 0.0, 0)
            }
        }
    }
}

// ============================================================================
// UTILITY IMPLEMENTATIONS
// ============================================================================

impl Default for PatternMatcher {
    fn default() -> Self {
        Self::new()
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
