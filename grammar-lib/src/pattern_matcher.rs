use crate::matchers::{
    CausativeFormMatcher, DependentNounMonoMatcher, DeshouFormMatcher, FlexibleVerbFormMatcher,
    GuraiFormMatcher, HajimeteAdverbMatcher, IiFormMatcher, IkenaiFormMatcher,
    ImperativeFormMatcher, MaiFormMatcher, MasenFormMatcher, MashouFormMatcher, MustPatternMatcher,
    NDesuFormMatcher, NakattaFormMatcher, NakereFormMatcher, NakuFormMatcher, NiKansuruFormMatcher,
    NonNaruMizenMatcher, NonPotentialMizenMatcher, NounMatcher, OiteFormMatcher,
    OyobiConjunctionMatcher, PastAuxiliaryMatcher, PpoiFormMatcher, RareruFormMatcher,
    SaseFormMatcher, ShiParticleMatcher, SouAppearanceStemMatcher, SouHearsayStemMatcher,
    SugiruStemMatcher, TagaruFormMatcher, TaiFormMatcher, TakattaFormMatcher, TakuFormMatcher,
    TaraFormMatcher, TariParticleMatcher, TateSuffixMatcher, TeDeFormMatcher, TeParticleMatcher,
    TekiSuffixMatcher, ToIiFormMatcher, TokenMatcherLogic, YokattaFormMatcher,
};
use crate::types::KagomeToken;

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

/// Category of grammar pattern for filtering and vocabulary extraction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternCategory {
    /// Basic conjugation forms - detected but not stored as grammar patterns
    /// Used for vocabulary consolidation (skip auxiliary tokens)
    Conjugation,
    /// Actual grammatical constructions - stored as grammar patterns
    Construction,
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
    /// Match 未然形 verbs that are NOT potential forms (excludes れる/られる)
    NonPotentialMizen,
    /// Match 未然形 verbs excluding なる (for shika_nai pattern)
    NonNaruMizen,
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
    /// Match および or 及び as conjunction (not verb)
    OyobiConjunction,
    /// Match noun (名詞)
    Noun,
    /// Match dependent noun もの (名詞/非自立) for mono_no pattern
    DependentNounMono,
}

/// Represents a complete grammar pattern
#[derive(Debug, Clone)]
pub struct GrammarPattern {
    pub name: &'static str,
    pub tokens: Vec<TokenMatcher>,
    pub priority: u8, // Higher = more specific/important
    pub category: PatternCategory,
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
                    // Trait-based matchers (migrated)
                    CustomMatcher::TaiForm => TaiFormMatcher.matches(token),
                    CustomMatcher::TakuForm => TakuFormMatcher.matches(token),
                    CustomMatcher::TakattaForm => TakattaFormMatcher.matches(token),
                    CustomMatcher::NakattaForm => NakattaFormMatcher.matches(token),
                    CustomMatcher::NakereForm => NakereFormMatcher.matches(token),
                    CustomMatcher::NakuForm => NakuFormMatcher.matches(token),
                    CustomMatcher::SaseForm => SaseFormMatcher.matches(token),
                    CustomMatcher::RareruForm => RareruFormMatcher.matches(token),
                    CustomMatcher::CausativeForm => CausativeFormMatcher.matches(token),
                    CustomMatcher::TaraForm => TaraFormMatcher.matches(token),
                    CustomMatcher::PastAuxiliary => PastAuxiliaryMatcher.matches(token),
                    CustomMatcher::TeParticle => TeParticleMatcher.matches(token),
                    CustomMatcher::TeDeForm => TeDeFormMatcher.matches(token),
                    CustomMatcher::FlexibleVerbForm => FlexibleVerbFormMatcher.matches(token),
                    CustomMatcher::MustPattern => MustPatternMatcher.matches(token),
                    CustomMatcher::NonPotentialMizen => NonPotentialMizenMatcher.matches(token),
                    CustomMatcher::NonNaruMizen => NonNaruMizenMatcher.matches(token),
                    CustomMatcher::IiForm => IiFormMatcher.matches(token),
                    CustomMatcher::IkenaiForm => IkenaiFormMatcher.matches(token),
                    CustomMatcher::MashouForm => MashouFormMatcher.matches(token),
                    CustomMatcher::MasenForm => MasenFormMatcher.matches(token),
                    CustomMatcher::ImperativeForm => ImperativeFormMatcher.matches(token),
                    CustomMatcher::TariParticle => TariParticleMatcher.matches(token),
                    CustomMatcher::DeshouForm => DeshouFormMatcher.matches(token),
                    CustomMatcher::NDesuForm => NDesuFormMatcher.matches(token),
                    CustomMatcher::YokattaForm => YokattaFormMatcher.matches(token),
                    CustomMatcher::TagaruForm => TagaruFormMatcher.matches(token),
                    CustomMatcher::ToIiForm => ToIiFormMatcher.matches(token),
                    CustomMatcher::ShiParticle => ShiParticleMatcher.matches(token),
                    CustomMatcher::SugiruStem => SugiruStemMatcher.matches(token),
                    CustomMatcher::SouAppearanceStem => SouAppearanceStemMatcher.matches(token),
                    CustomMatcher::SouHearsayStem => SouHearsayStemMatcher.matches(token),
                    CustomMatcher::MaiForm => MaiFormMatcher.matches(token),
                    CustomMatcher::PpoiForm => PpoiFormMatcher.matches(token),
                    CustomMatcher::TekiSuffix => TekiSuffixMatcher.matches(token),
                    CustomMatcher::TateSuffix => TateSuffixMatcher.matches(token),
                    CustomMatcher::GuraiForm => GuraiFormMatcher.matches(token),
                    CustomMatcher::OiteForm => OiteFormMatcher.matches(token),
                    CustomMatcher::NiKansuruForm => NiKansuruFormMatcher.matches(token),
                    CustomMatcher::HajimeteAdverb => HajimeteAdverbMatcher.matches(token),
                    CustomMatcher::OyobiConjunction => OyobiConjunctionMatcher.matches(token),
                    CustomMatcher::Noun => NounMatcher.matches(token),
                    CustomMatcher::DependentNounMono => DependentNounMonoMatcher.matches(token),
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
