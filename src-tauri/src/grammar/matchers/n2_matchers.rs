use super::TokenMatcherLogic;
use crate::analysis::morphology::KagomeToken;

// ========== Formal Expressions ==========

/// Match および or 及び as conjunction (not verb)
#[derive(Debug)]
pub struct OyobiConjunctionMatcher;

impl TokenMatcherLogic for OyobiConjunctionMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        (token.surface == "および" || token.surface == "及び")
            && token.pos.first().is_some_and(|pos| pos == "接続詞")
    }
}

/// Match dependent noun もの (名詞/非自立) for mono_no pattern
#[derive(Debug)]
pub struct DependentNounMonoMatcher;

impl TokenMatcherLogic for DependentNounMonoMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        token.surface == "もの"
            && token.pos.first().is_some_and(|pos| pos == "名詞")
            && token.pos.get(1).is_some_and(|pos| pos == "非自立")
    }
}
