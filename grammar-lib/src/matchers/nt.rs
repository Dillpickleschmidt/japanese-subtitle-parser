use crate::pattern_matcher::TokenMatcher;

// Pattern: ぞ (emphatic sentence-ending particle)
// Structures: Phrase + ぞ
pub fn zo() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    use super::Matcher;

    // Match ぞ (助詞/終助詞)
    #[derive(Debug)]
    struct ZoMatcher;
    impl Matcher for ZoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ぞ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "終助詞")
        }
    }

    vec![
        TokenMatcher::Any,  // Preceding word (verb, adjective, auxiliary verb)
        TokenMatcher::Custom(Arc::new(ZoMatcher)),
    ]
}

// Pattern: ぜ
pub fn ze() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: わ
pub fn wa() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: い
pub fn i() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ん (Slang)
pub fn n_slang() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: つ (Slang)
pub fn tsu_slang() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～やがる
pub fn uff5e_yagaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: がいい
pub fn gaii() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: かろう
pub fn karou() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: やや
pub fn yaya() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: いずれも
pub fn izuremo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: あわよくば
pub fn awayokuba() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: むず
pub fn muzu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}
