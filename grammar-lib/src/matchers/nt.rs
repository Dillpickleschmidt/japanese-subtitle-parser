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

// Pattern: ん (Slang) - abbreviation for らない or ている
// Structures: Verb[未然特殊] + ない (らない → んない)
//             Verb[連用タ接続] + て + ん (ている → てん)
pub fn n_slang() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    use super::Matcher;

    // Match verb in 未然特殊 conjugation (わかん, なん) OR verb + て + ん
    #[derive(Debug)]
    struct NSlangVerbMatcher;
    impl Matcher for NSlangVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Variant 1: Verb in 未然特殊 (らない → んない)
            if token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然特殊")
            {
                return true;
            }
            // Variant 2: Verb in 連用タ接続 (for ている → てん)
            if token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用タ接続")
            {
                return true;
            }
            false
        }
    }

    // Match either: ない/ねえ auxiliary OR て particle
    #[derive(Debug)]
    struct NSlangFollowerMatcher;
    impl Matcher for NSlangFollowerMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Variant 1: ない/ねえ auxiliary
            if (token.surface == "ない" || token.surface == "ねえ")
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
            {
                return true;
            }
            // Variant 2: て particle
            if token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
            {
                return true;
            }
            false
        }
    }

    // For variant 2 only: match ん (名詞/非自立) after て
    #[derive(Debug)]
    struct NNounMatcher;
    impl Matcher for NNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ん"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Use Optional to make the third token optional (needed for variant 2, not for variant 1)
    vec![
        TokenMatcher::Custom(Arc::new(NSlangVerbMatcher)),
        TokenMatcher::Custom(Arc::new(NSlangFollowerMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NNounMatcher)))),
    ]
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
