use crate::pattern_matcher::{MatchContext, TokenMatcher};
use super::any;

// Pattern: ぞ (emphatic sentence-ending particle)
// Structures: Phrase + ぞ
pub fn zo() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    use super::Matcher;

    // Match ぞ (助詞/終助詞)
    #[derive(Debug)]
    struct ZoMatcher;
    impl Matcher for ZoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ぞ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "終助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),  // Preceding word (verb, adjective, auxiliary verb)
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
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ぜ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "終助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),  // Preceding word (verb, adjective, auxiliary verb)
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
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "わ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "終助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),  // Preceding word (verb, adjective, auxiliary verb)
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
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "わい"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "終助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match だい (名詞/一般)
    #[derive(Debug)]
    struct DaiMatcher;
    impl Matcher for DaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "だい"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Combined matcher for わい or だい
    #[derive(Debug)]
    struct IMatcher;
    impl Matcher for IMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            let (wai_matches, wai_n) = WaiMatcher.matches(ctx);
            if wai_matches {
                return (true, wai_n);
            }
            DaiMatcher.matches(ctx)
        }
    }

    vec![
        any(),  // Preceding word (verb, adjective, auxiliary verb)
        TokenMatcher::Custom(Arc::new(IMatcher)),
    ]
}

// Pattern: ん (Slang) - abbreviation for らない or ている
// Structures: Verb[未然特殊] + ない (らない → んない)
//             Verb[連用タ接続] + て + ん (ている → てん)
pub fn n_slang() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    use super::{Matcher, check_token};

    // Match verb in 未然特殊 conjugation (わかん, なん) OR verb + て + ん
    #[derive(Debug)]
    struct NSlangVerbMatcher;
    impl Matcher for NSlangVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
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
            })
        }
    }

    // Match either: ない/ねえ auxiliary OR て particle
    #[derive(Debug)]
    struct NSlangFollowerMatcher;
    impl Matcher for NSlangFollowerMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
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
            })
        }
    }

    // For variant 2 only: match ん (名詞/非自立) after て
    #[derive(Debug)]
    struct NNounMatcher;
    impl Matcher for NNounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ん"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
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
// Pattern: つ (Slang) - という contraction (called/that was said)
// Structures: という → つ/っつ/つう + variants
pub fn tsu_slang() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    use super::{Matcher, check_token};

    // Match all variants of つ slang:
    // - つ (助動詞, base=つ) - basic form
    // - つう (名詞, base=つう) - variant form
    // - つる verb base (conjugated forms like つった, つって)
    #[derive(Debug)]
    struct TsuSlangMatcher;
    impl Matcher for TsuSlangMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // つ as auxiliary verb (基本形)
            if token.surface == "つ"
                && token.base_form == "つ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
            {
                return true;
            }

            // つう as noun
            if token.surface == "つう"
                && token.base_form == "つう"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
            {
                return true;
            }

            // つる verb (conjugated forms: つった, つって, etc.)
            if token.base_form == "つる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.surface.starts_with("つ")
            {
                return true;
            }

            false
            })
        }
    }

    // Just match the つ form itself - can appear anywhere という would
    vec![TokenMatcher::Custom(Arc::new(TsuSlangMatcher))]
}

// Pattern: ～やがる (have the nerve to / have the gall to)
// Structures: Verb[stem] + やがる, Verb[て] + やがる
// Tokenization: や (助詞) + がる (動詞/接尾)
pub fn uff5e_yagaru() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    use super::Matcher;

    // Matcher for や particle (can be 並立助詞 or 係助詞)
    #[derive(Debug)]
    struct YaMatcher;
    impl Matcher for YaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "や"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "並立助詞")
                    || token.pos.get(1).is_some_and(|pos| pos == "係助詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matcher for がる suffix verb
    #[derive(Debug)]
    struct GaruMatcher;
    impl Matcher for GaruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "がる"
                && token.base_form == "がる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Preceding verb (stem or te-form) or て particle
        TokenMatcher::Custom(Arc::new(YaMatcher)),
        TokenMatcher::Custom(Arc::new(GaruMatcher)),
    ]
}

// Pattern: がいい
pub fn gaii() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: かろう (old-fashioned auxiliary verb - volition/agreement)
// Structures: い-Adjective[未然ウ接続] + う
pub fn karou() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    use super::{Matcher, check_token};

    // Matcher for adjective in 未然ウ接続 form (e.g., よかろ, 寒かろ, なかろ)
    #[derive(Debug)]
    struct AdjectiveKaroMatcher;
    impl Matcher for AdjectiveKaroMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Can be 形容詞/自立 or 助動詞 (for ない)
            let is_adjective = token.pos.first().is_some_and(|pos| pos == "形容詞");
            let is_auxiliary = token.pos.first().is_some_and(|pos| pos == "助動詞");

            // Must be in 未然ウ接続 conjugation form
            let has_correct_form = token.features.get(5).is_some_and(|f| f == "未然ウ接続");

            (is_adjective || is_auxiliary) && has_correct_form
            })
        }
    }

    // Matcher for う auxiliary verb
    #[derive(Debug)]
    struct UAuxiliaryMatcher;
    impl Matcher for UAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "う"
                && token.base_form == "う"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(AdjectiveKaroMatcher)),
        TokenMatcher::Custom(Arc::new(UAuxiliaryMatcher)),
    ]
}

// Pattern: やや (adverb meaning "a little bit" / "slightly")
// Structures: やや + Phrase
pub fn yaya() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    use super::Matcher;

    #[derive(Debug)]
    struct YayaMatcher;
    impl Matcher for YayaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "やや"
                && token.base_form == "やや"
                && token.pos.first().is_some_and(|pos| pos == "副詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![TokenMatcher::Custom(Arc::new(YayaMatcher))]
}

// Pattern: いずれも (all / any / both)
// Structures: いずれも + Phrase
pub fn izuremo() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    use super::Matcher;

    // Match いずれ (名詞/代名詞/一般)
    #[derive(Debug)]
    struct IzureMatcher;
    impl Matcher for IzureMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "いずれ"
                && token.base_form == "いずれ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "代名詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match も (助詞/係助詞)
    #[derive(Debug)]
    struct MoMatcher;
    impl Matcher for MoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(IzureMatcher)),
        TokenMatcher::Custom(Arc::new(MoMatcher)),
    ]
}

// Pattern: あわよくば (if possible / if luck is on my side)
// Structures: あわよくば + Phrase
pub fn awayokuba() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    use super::Matcher;

    #[derive(Debug)]
    struct AwayokubaMatcher;
    impl Matcher for AwayokubaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "あわよくば"
                && token.base_form == "あわよくば"
                && token.pos.first().is_some_and(|pos| pos == "副詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    vec![TokenMatcher::Custom(Arc::new(AwayokubaMatcher))]
}

// Pattern: むず
/// Match むず (classical auxiliary verb - conjecture/strong will)
/// Handles all forms: むず, んず, むずる, むずれ, なんず
pub fn muzu() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    use super::Matcher;

    // Matcher for むず, むずる, むずれ (all tokenize as 名詞/一般 with empty base)
    #[derive(Debug)]
    struct MuzuNounMatcher;
    impl Matcher for MuzuNounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "むず" || token.surface == "むずる" || token.surface == "むずれ")
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
                && token.base_form.is_empty() => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matcher for なんず (tokenizes as verb with base なんずる)
    #[derive(Debug)]
    struct NanzuMatcher;
    impl Matcher for NanzuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "なんず"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.base_form == "なんずる" => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Combined matcher for all single-token forms
    #[derive(Debug)]
    struct MuzuCombinedMatcher;
    impl Matcher for MuzuCombinedMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            let (muzu_matches, muzu_n) = MuzuNounMatcher.matches(ctx);
            if muzu_matches {
                return (true, muzu_n);
            }
            NanzuMatcher.matches(ctx)
        }
    }

    vec![
        any(), // Preceding verb (negative stem form)
        TokenMatcher::Custom(Arc::new(MuzuCombinedMatcher)),
    ]
}

// Pattern: がいい (you shall/should do)
/// Match Verb[基本形] + が + いい/よい
/// Used by those in high positions to give commands
pub fn ga_ii() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    use super::{Matcher, verb_form};

    // Matcher for が particle (接続助詞)
    #[derive(Debug)]
    struct GaMatcher;
    impl Matcher for GaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "が"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matcher for いい or よい adjective (can be 自立 or 非自立)
    #[derive(Debug)]
    struct IiYoiMatcher;
    impl Matcher for IiYoiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.base_form == "いい" || token.base_form == "よい")
                && token.pos.first().is_some_and(|pos| pos == "形容詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        verb_form("基本形"), // Verb in dictionary form
        TokenMatcher::Custom(Arc::new(GaMatcher)),
        TokenMatcher::Custom(Arc::new(IiYoiMatcher)),
    ]
}
