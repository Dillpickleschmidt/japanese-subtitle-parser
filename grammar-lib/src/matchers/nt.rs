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

// Pattern: ぜ (friendly emphatic sentence-ending particle)
// Structures: Phrase + ぜ
pub fn ze() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    use super::Matcher;

    // Match ぜ (助詞/終助詞)
    #[derive(Debug)]
    struct ZeMatcher;
    impl Matcher for ZeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ぜ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "終助詞")
        }
    }

    vec![
        TokenMatcher::Any,  // Preceding word (verb, adjective, auxiliary verb)
        TokenMatcher::Custom(Arc::new(ZeMatcher)),
    ]
}

// Pattern: わ (sentence-ending particle for emphasis/conviction)
// Structures: Phrase + わ
pub fn wa() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    use super::Matcher;

    // Match わ (助詞/終助詞)
    #[derive(Debug)]
    struct WaMatcher;
    impl Matcher for WaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "わ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "終助詞")
        }
    }

    vec![
        TokenMatcher::Any,  // Preceding word (verb, adjective, auxiliary verb)
        TokenMatcher::Custom(Arc::new(WaMatcher)),
    ]
}

// Pattern: い (sentence-ending particle for friendliness/familiarity)
// Structures: Phrase + わい/だい
// Note: かい is handled by a separate N4 pattern
pub fn i() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    use super::Matcher;

    // Match わい (助詞/終助詞)
    #[derive(Debug)]
    struct WaiMatcher;
    impl Matcher for WaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "わい"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "終助詞")
        }
    }

    // Match だい (名詞/一般)
    #[derive(Debug)]
    struct DaiMatcher;
    impl Matcher for DaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "だい"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
        }
    }

    // Combined matcher for わい or だい
    #[derive(Debug)]
    struct IMatcher;
    impl Matcher for IMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            WaiMatcher.matches(token) || DaiMatcher.matches(token)
        }
    }

    vec![
        TokenMatcher::Any,  // Preceding word (verb, adjective, auxiliary verb)
        TokenMatcher::Custom(Arc::new(IMatcher)),
    ]
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
