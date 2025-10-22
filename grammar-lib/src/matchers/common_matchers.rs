use super::TokenMatcherLogic;
use crate::types::KagomeToken;

/// Match verb in 連用形 or 連用タ接続 (for flexible patterns)
#[derive(Debug)]
pub struct FlexibleVerbFormMatcher;

impl TokenMatcherLogic for FlexibleVerbFormMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        if token.pos.first().is_none_or(|pos| pos != "動詞") {
            false
        } else {
            let form = token.features.get(5);
            form.is_some_and(|f| f == "連用形" || f == "連用タ接続")
        }
    }
}

/// Match any particle (助詞)
/// Matches: が、を、に、へ、から、まで、より、で、も、など
#[derive(Debug)]
pub struct ParticleMatcher;

impl TokenMatcherLogic for ParticleMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        token.pos.first().is_some_and(|pos| pos == "助詞")
    }
}
