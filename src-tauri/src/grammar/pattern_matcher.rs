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
    /// Match exact surface form
    Surface(&'static str),
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
    /// Match て or で as either surface or particle
    TeDeForm,
    /// Match verb in 連用形 or 連用タ接続 (for flexible patterns)
    FlexibleVerbForm,
    /// Match specific verb forms for must patterns
    MustPattern,
    /// Match いい or 良い (good/okay)
    IiForm,
    /// Match いけない or いけません (must not)
    IkenaiForm,
    /// Match ましょ + う (let's/shall we)
    MashouForm,
    /// Match ませ + ん (polite negative)
    MasenForm,
    /// Match imperative forms (命令形, 命令ｒｏ, 命令ｉ)
    ImperativeForm,
    /// Match たり particle (並立助詞)
    TariParticle,
    /// Match でしょう (probably)
    DeshouForm,
    /// Match ん or の before です (explanatory)
    NDesuForm,
    /// Match よかった (was good/glad)
    YokattaForm,
    /// Match た from たい + がる
    TagaruForm,
    /// Match いい in といい context (can be verb いう or adj いい)
    ToIiForm,
    /// Match し particle (接続助詞)
    ShiParticle,
    /// Match verb 連用形 or adjective stem (for すぎる pattern)
    SugiruStem,
    /// Match verb 連用形 or adjective stem (for そう appearance)
    SouAppearanceStem,
    /// Match plain form verb or adjective (for そうだ hearsay)
    SouHearsayStem,
    /// Match まい (negative volition)
    MaiForm,
    /// Match っぽい suffix
    PpoiForm,
    /// Match 的 suffix
    TekiSuffix,
    /// Match たて suffix
    TateSuffix,
    /// Match ぐらい or くらい (about/approximately)
    GuraiForm,
    /// Match おいて or において (at/in)
    OiteForm,
    /// Match に関する or に関して (regarding/about)
    NiKansuruForm,
    /// Match 初めて as adverb (for て初めて pattern)
    HajimeteAdverb,
}

/// Represents a complete grammar pattern
#[derive(Debug, Clone)]
pub struct GrammarPattern {
    pub name: &'static str,
    pub tokens: Vec<TokenMatcher>,
    pub priority: u8, // Higher = more specific/important
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
    pub start_char: u32, // Character start position in original text
    pub end_char: u32,   // Character end position in original text
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

        // Sort by confidence (descending), then by character length (descending)
        matches.sort_by(|a, b| {
            b.confidence
                .partial_cmp(&a.confidence)
                .unwrap()
                .then((b.end_char - b.start_char).cmp(&(a.end_char - a.start_char)))
            // Prefer longer matches
        });

        matches
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
                // Use constant reference to avoid string allocation
                if token.pos.first().is_none_or(|pos| pos != VERB_POS) {
                    return (false, 0.0);
                }

                let mut score = 1.0; // Base score for matching POS

                if let Some(expected_form) = conjugation_form {
                    // Compare with string slice instead of creating String
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
                let matches = match custom_matcher {
                    CustomMatcher::TaiForm => {
                        token.surface == "たい"
                            && (token.pos.first().is_some_and(|pos| pos == ADJECTIVE_POS)
                                || token
                                    .pos
                                    .first()
                                    .is_some_and(|pos| pos == AUXILIARY_VERB_POS))
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
                        (token.surface == "たら" || token.surface == "だら")
                            && (token
                                .pos
                                .first()
                                .is_some_and(|pos| pos == AUXILIARY_VERB_POS)
                                || token.base_form == "た"
                                || token.base_form == "だ")
                    }
                    CustomMatcher::PastAuxiliary => {
                        (token.surface == "た" || token.surface == "だ")
                            && (token
                                .pos
                                .first()
                                .is_some_and(|pos| pos == AUXILIARY_VERB_POS)
                                || token.base_form == "た"
                                || token.base_form == "だ")
                    }
                    CustomMatcher::TeParticle => token.surface == "て" || token.surface == "で",
                    CustomMatcher::TeDeForm => {
                        // Match both Surface("て"/"で") and Particle("で")
                        token.surface == "て" || token.surface == "で"
                    }
                    CustomMatcher::FlexibleVerbForm => {
                        if token.pos.first().is_none_or(|pos| pos != VERB_POS) {
                            false
                        } else {
                            let form = token.features.get(5);
                            form.is_some_and(|f| f == RENYOU_FORM || f == RENYOU_TA_FORM)
                        }
                    }
                    CustomMatcher::MustPattern => {
                        token.surface == "なら"
                            || token.surface == "いけ"
                            || token.surface == "だめ"
                    }
                    CustomMatcher::IiForm => {
                        (token.surface == "いい" || token.surface == "良い")
                            && (token.base_form == "いい" || token.base_form == "良い")
                    }
                    CustomMatcher::IkenaiForm => {
                        token.surface == "いけ"
                            || token.surface == "いけない"
                            || token.surface == "いけません"
                    }
                    CustomMatcher::MashouForm => {
                        token.surface == "ましょう"
                            || (token.surface == "ましょ" && token.base_form == "ます")
                    }
                    CustomMatcher::MasenForm => {
                        (token.surface == "ませ" && token.base_form == "ます")
                            || token.surface == "ません"
                    }
                    CustomMatcher::ImperativeForm => {
                        if token.pos.first().is_none_or(|pos| pos != VERB_POS) {
                            false
                        } else {
                            let form = token.features.get(5);
                            form.is_some_and(|f| {
                                f == "命令形" || f == "命令ｒｏ" || f == "命令ｉ" || f == "命令ｅ"
                            })
                        }
                    }
                    CustomMatcher::TariParticle => {
                        (token.surface == "たり" || token.surface == "だり")
                            && token.pos.first().is_some_and(|pos| pos == "助詞")
                            && token.pos.get(1).is_some_and(|pos| pos == "並立助詞")
                    }
                    CustomMatcher::DeshouForm => {
                        (token.surface == "でしょう" || token.surface == "だろう")
                            || (token.surface == "でしょ" && token.base_form == "です")
                            || (token.surface == "だろ" && token.base_form == "だ")
                    }
                    CustomMatcher::NDesuForm => {
                        (token.surface == "ん" || token.surface == "の")
                            && token.pos.first().is_some_and(|pos| pos == "名詞")
                    }
                    CustomMatcher::YokattaForm => {
                        (token.surface == "よかっ" || token.surface == "良かっ")
                            && (token.base_form == "よい"
                                || token.base_form == "良い"
                                || token.base_form == "いい")
                    }
                    CustomMatcher::TagaruForm => {
                        token.surface == "た"
                            && token.base_form == "たい"
                            && token
                                .pos
                                .first()
                                .is_some_and(|pos| pos == AUXILIARY_VERB_POS)
                    }
                    CustomMatcher::ToIiForm => {
                        // Can be either the verb いう (to say) or adjective いい (good)
                        token.surface == "いい"
                            && (token.base_form == "いう" || token.base_form == "いい")
                            || (token.surface == "良い" && token.base_form == "良い")
                    }
                    CustomMatcher::ShiParticle => {
                        token.surface == "し"
                            && token.pos.first().is_some_and(|pos| pos == "助詞")
                            && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
                    }
                    CustomMatcher::SugiruStem => {
                        // Match verb 連用形 OR adjective stem for すぎる pattern
                        if token.pos.first().is_some_and(|pos| pos == VERB_POS) {
                            // Verb: match 連用形
                            let form = token.features.get(5);
                            form.is_some_and(|f| f == RENYOU_FORM)
                        } else if token.pos.first().is_some_and(|pos| pos == ADJECTIVE_POS) {
                            // i-adjective: match ガル接続 form (connects to がる/すぎる)
                            let form = token.features.get(5);
                            form.is_some_and(|f| f == "ガル接続")
                        } else if token.pos.first().is_some_and(|pos| pos == "名詞") {
                            // na-adjective: often tagged as 名詞/形容動詞語幹
                            token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹")
                        } else {
                            false
                        }
                    }
                    CustomMatcher::SouAppearanceStem => {
                        // Match verb 連用形 OR adjective stem for そう (appearance) pattern
                        // Same logic as SugiruStem - both use the same forms
                        if token.pos.first().is_some_and(|pos| pos == VERB_POS) {
                            // Verb: match 連用形
                            let form = token.features.get(5);
                            form.is_some_and(|f| f == RENYOU_FORM)
                        } else if token.pos.first().is_some_and(|pos| pos == ADJECTIVE_POS) {
                            // i-adjective: match ガル接続 form
                            let form = token.features.get(5);
                            form.is_some_and(|f| f == "ガル接続")
                        } else if token.pos.first().is_some_and(|pos| pos == "名詞") {
                            // na-adjective: often tagged as 名詞/形容動詞語幹
                            token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹")
                        } else {
                            false
                        }
                    }
                    CustomMatcher::SouHearsayStem => {
                        // Match plain/dictionary form verb or adjective for そうだ (hearsay)
                        if token.pos.first().is_some_and(|pos| pos == VERB_POS) {
                            // Verb: match 基本形
                            let form = token.features.get(5);
                            form.is_some_and(|f| f == "基本形")
                        } else if token.pos.first().is_some_and(|pos| pos == ADJECTIVE_POS) {
                            // i-adjective: match 基本形 (e.g., 高い)
                            let form = token.features.get(5);
                            form.is_some_and(|f| f == "基本形")
                        } else if token.pos.first().is_some_and(|pos| pos == "形容動詞") {
                            // na-adjective: match 基本形 or 語幹
                            true
                        } else if token.pos.first().is_some_and(|pos| pos == "名詞") {
                            // na-adjective stem: tagged as 名詞/形容動詞語幹
                            token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹")
                        } else {
                            false
                        }
                    }
                    CustomMatcher::MaiForm => {
                        // Match まい (negative volition) - can attach to verb dict form or 未然形
                        token.surface == "まい"
                            && token
                                .pos
                                .first()
                                .is_some_and(|pos| pos == AUXILIARY_VERB_POS)
                    }
                    CustomMatcher::PpoiForm => {
                        // Match っぽい suffix - can be split, together, or as part of compound
                        (token.surface == "っぽい"
                            || token.surface == "ぽい"
                            || token.surface.ends_with("っぽい"))
                            && token
                                .pos
                                .first()
                                .is_some_and(|pos| pos == "接尾辞" || pos == "形容詞")
                    }
                    CustomMatcher::TekiSuffix => {
                        // Match 的 suffix (ish/like) - tagged as 名詞/接尾/形容動詞語幹
                        token.surface == "的"
                            && token.pos.first().is_some_and(|pos| pos == "名詞")
                            && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                    }
                    CustomMatcher::TateSuffix => {
                        // Match たて suffix (freshly/just done) - tagged as 名詞/接尾
                        token.surface == "たて"
                            && token.pos.first().is_some_and(|pos| pos == "名詞")
                            && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                    }
                    CustomMatcher::GuraiForm => {
                        // Match ぐらい or くらい (about/approximately)
                        token.surface == "ぐらい" || token.surface == "くらい"
                    }
                    CustomMatcher::OiteForm => {
                        // Match において (single compound token) or おい (from おく verb in て-form)
                        token.surface == "において"
                            || (token.surface == "おい" && token.base_form == "おく")
                    }
                    CustomMatcher::NiKansuruForm => {
                        // Match に関する or に関して (regarding/about)
                        token.surface == "に関する" || token.surface == "に関して"
                    }
                    CustomMatcher::HajimeteAdverb => {
                        // Match 初めて as adverb (for て初めて pattern)
                        token.surface == "初めて"
                            && token.pos.first().is_some_and(|pos| pos == "副詞")
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
