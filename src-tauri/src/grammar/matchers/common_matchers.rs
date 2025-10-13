use super::TokenMatcherLogic;
use crate::grammar::types::KagomeToken;

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
