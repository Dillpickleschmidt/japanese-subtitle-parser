use super::TokenMatcherLogic;
use crate::types::KagomeToken;

// ========== Suffixes ==========

/// Match っぽい suffix (can be split, together, or compound)
#[derive(Debug)]
pub struct PpoiFormMatcher;

impl TokenMatcherLogic for PpoiFormMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        (token.surface == "っぽい" || token.surface == "ぽい" || token.surface.ends_with("っぽい"))
            && token
                .pos
                .first()
                .is_some_and(|pos| pos == "接尾辞" || pos == "形容詞")
    }
}

/// Match 的 suffix (ish/like)
#[derive(Debug)]
pub struct TekiSuffixMatcher;

impl TokenMatcherLogic for TekiSuffixMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        token.surface == "的"
            && token.pos.first().is_some_and(|pos| pos == "名詞")
            && token.pos.get(1).is_some_and(|pos| pos == "接尾")
    }
}

/// Match たて suffix (freshly/just done)
#[derive(Debug)]
pub struct TateSuffixMatcher;

impl TokenMatcherLogic for TateSuffixMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        token.surface == "たて"
            && token.pos.first().is_some_and(|pos| pos == "名詞")
            && token.pos.get(1).is_some_and(|pos| pos == "接尾")
    }
}

// ========== Particles and Expressions ==========

/// Match ぐらい or くらい (about/approximately)
#[derive(Debug)]
pub struct GuraiFormMatcher;

impl TokenMatcherLogic for GuraiFormMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        token.surface == "ぐらい" || token.surface == "くらい"
    }
}

/// Match おいて or において (at/in)
#[derive(Debug)]
pub struct OiteFormMatcher;

impl TokenMatcherLogic for OiteFormMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        token.surface == "において" || (token.surface == "おい" && token.base_form == "おく")
    }
}

/// Match に関する or に関して (regarding/about)
#[derive(Debug)]
pub struct NiKansuruFormMatcher;

impl TokenMatcherLogic for NiKansuruFormMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        token.surface == "に関する" || token.surface == "に関して"
    }
}

/// Match 初めて as adverb (for て初めて pattern)
#[derive(Debug)]
pub struct HajimeteAdverbMatcher;

impl TokenMatcherLogic for HajimeteAdverbMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        token.surface == "初めて" && token.pos.first().is_some_and(|pos| pos == "副詞")
    }
}

// ========== Verb Forms ==========

/// Match まい (negative volition)
#[derive(Debug)]
pub struct MaiFormMatcher;

impl TokenMatcherLogic for MaiFormMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        token.surface == "まい" && token.pos.first().is_some_and(|pos| pos == "助動詞")
    }
}

// ========== Parts of Speech ==========

/// Match noun (名詞)
#[derive(Debug)]
pub struct NounMatcher;

impl TokenMatcherLogic for NounMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        token.pos.first().is_some_and(|pos| pos == "名詞")
    }
}
