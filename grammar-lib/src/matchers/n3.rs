use crate::pattern_matcher::{MatchContext, TokenMatcher};
use std::sync::Arc;
use super::{Matcher, check_token, verb, verb_form, verb_base, surface, any, optional, wildcard};

// って: Casual topic marker (replacing は)
// Structures: Sentence topic + って
pub fn tte() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for って as topic marker particle
    #[derive(Debug)]
    struct TteParticleMatcher;
    impl Matcher for TteParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            token.surface == "って"
                && token.base_form == "って"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "連語") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Sentence topic (usually a noun)
        TokenMatcher::Custom(Arc::new(TteParticleMatcher)),
    ]
}

// Pattern: ばいい (it would be good if)
// Structures: Verb［ば］+ いい
pub fn baii() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Custom(Arc::new(VerbOrIAdjKateiMatcher)),
        TokenMatcher::Custom(Arc::new(BaConditionalMatcher)),
        super::ii_form(),
    ]
}

// たらいい・といい: Conditional + いい (it would be good if)
// This pattern has multiple structural variants that will be registered separately

// Helper matchers for conditional forms
#[derive(Debug)]
struct TaraConditionalMatcher;
impl Matcher for TaraConditionalMatcher {
    fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "たら"
            && token.base_form == "た"
            && token.pos.first().is_some_and(|pos| pos == "助動詞")
            && token.features.get(5).is_some_and(|f| f == "仮定形") => (true, 1),
                _ => (false, 0),
            }
        }
}

#[derive(Debug)]
struct BaConditionalMatcher;
impl Matcher for BaConditionalMatcher {
    fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ば"
            && token.base_form == "ば"
            && token.pos.first().is_some_and(|pos| pos == "助詞")
            && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
}

#[derive(Debug)]
struct ToConditionalMatcher;
impl Matcher for ToConditionalMatcher {
    fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "と"
            && token.base_form == "と"
            && token.pos.first().is_some_and(|pos| pos == "助詞")
            && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
}

#[derive(Debug)]
struct VerbOrIAdjKateiMatcher;
impl Matcher for VerbOrIAdjKateiMatcher {
    fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.features.get(5).is_some_and(|f| f == "仮定形")
            && (token.pos.first().is_some_and(|pos| pos == "動詞")
                || token.pos.first().is_some_and(|pos| pos == "形容詞")) => (true, 1),
                _ => (false, 0),
            }
        }
}

#[derive(Debug)]
struct VerbOrIAdjRenyouTaMatcher;
impl Matcher for VerbOrIAdjRenyouTaMatcher {
    fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.features.get(5).is_some_and(|f| f == "連用タ接続")
            && (token.pos.first().is_some_and(|pos| pos == "動詞")
                || token.pos.first().is_some_and(|pos| pos == "形容詞")) => (true, 1),
                _ => (false, 0),
            }
        }
}

#[derive(Debug)]
struct VerbOrIAdjRenyouMatcher;
impl Matcher for VerbOrIAdjRenyouMatcher {
    fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.features.get(5).is_some_and(|f| f == "連用形")
            && (token.pos.first().is_some_and(|pos| pos == "動詞")
                || token.pos.first().is_some_and(|pos| pos == "形容詞")) => (true, 1),
                _ => (false, 0),
            }
        }
}

#[derive(Debug)]
struct VerbOrIAdjKihonMatcher;
impl Matcher for VerbOrIAdjKihonMatcher {
    fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.features.get(5).is_some_and(|f| f == "基本形")
            && (token.pos.first().is_some_and(|pos| pos == "動詞")
                || token.pos.first().is_some_and(|pos| pos == "形容詞")) => (true, 1),
                _ => (false, 0),
            }
        }
}

#[derive(Debug)]
struct NaAdjStemMatcher;
impl Matcher for NaAdjStemMatcher {
    fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "名詞")
            && token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹") => (true, 1),
                _ => (false, 0),
            }
        }
}

#[derive(Debug)]
struct DaRenyouTaMatcher;
impl Matcher for DaRenyouTaMatcher {
    fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "だっ"
            && token.base_form == "だ"
            && token.pos.first().is_some_and(|pos| pos == "助動詞")
            && token.features.get(5).is_some_and(|f| f == "連用タ接続") => (true, 1),
                _ => (false, 0),
            }
        }
}

#[derive(Debug)]
struct DeRenyouMatcher;
impl Matcher for DeRenyouMatcher {
    fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "で"
            && token.base_form == "だ"
            && token.pos.first().is_some_and(|pos| pos == "助動詞")
            && token.features.get(5).is_some_and(|f| f == "連用形") => (true, 1),
                _ => (false, 0),
            }
        }
}

#[derive(Debug)]
struct AreKateiMatcher;
impl Matcher for AreKateiMatcher {
    fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "あれ"
            && token.base_form == "ある"
            && token.pos.first().is_some_and(|pos| pos == "助動詞")
            && token.features.get(5).is_some_and(|f| f == "仮定形") => (true, 1),
                _ => (false, 0),
            }
        }
}

#[derive(Debug)]
struct DaKihonMatcher;
impl Matcher for DaKihonMatcher {
    fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "だ"
            && token.base_form == "だ"
            && token.pos.first().is_some_and(|pos| pos == "助動詞")
            && token.features.get(5).is_some_and(|f| f == "基本形") => (true, 1),
                _ => (false, 0),
            }
        }
}

// Variant 1: Verb/い-Adj(仮定形) + ば + いい
pub fn taraii_u30fb_toii_ba() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Custom(Arc::new(VerbOrIAdjKateiMatcher)),
        TokenMatcher::Custom(Arc::new(BaConditionalMatcher)),
        super::ii_form(),
    ]
}

// Variant 2: Verb/い-Adj(連用タ接続) + たら + いい
pub fn taraii_u30fb_toii_tara_ta() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Custom(Arc::new(VerbOrIAdjRenyouTaMatcher)),
        TokenMatcher::Custom(Arc::new(TaraConditionalMatcher)),
        super::ii_form(),
    ]
}

// Variant 3: Verb/い-Adj(連用形) + たら + いい
pub fn taraii_u30fb_toii_tara_ren() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Custom(Arc::new(VerbOrIAdjRenyouMatcher)),
        TokenMatcher::Custom(Arc::new(TaraConditionalMatcher)),
        super::ii_form(),
    ]
}

// Variant 4: Verb/い-Adj(基本形) + と + いい
pub fn taraii_u30fb_toii_to() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Custom(Arc::new(VerbOrIAdjKihonMatcher)),
        TokenMatcher::Custom(Arc::new(ToConditionalMatcher)),
        super::ii_form(),
    ]
}

// Variant 5: な-Adj + だっ + たら + いい
pub fn taraii_u30fb_toii_na_dattara() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Custom(Arc::new(NaAdjStemMatcher)),
        TokenMatcher::Custom(Arc::new(DaRenyouTaMatcher)),
        TokenMatcher::Custom(Arc::new(TaraConditionalMatcher)),
        super::ii_form(),
    ]
}

// Variant 6: な-Adj + で + あれ + ば + いい
pub fn taraii_u30fb_toii_na_deareba() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Custom(Arc::new(NaAdjStemMatcher)),
        TokenMatcher::Custom(Arc::new(DeRenyouMatcher)),
        TokenMatcher::Custom(Arc::new(AreKateiMatcher)),
        TokenMatcher::Custom(Arc::new(BaConditionalMatcher)),
        super::ii_form(),
    ]
}

// Variant 7: な-Adj + だ + と + いい
pub fn taraii_u30fb_toii_na_dato() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Custom(Arc::new(NaAdjStemMatcher)),
        TokenMatcher::Custom(Arc::new(DaKihonMatcher)),
        TokenMatcher::Custom(Arc::new(ToConditionalMatcher)),
        super::ii_form(),
    ]
}

// Legacy function for backward compatibility (if patterns.rs uses it)
pub fn taraii_u30fb_toii() -> Vec<TokenMatcher> {
    // Return the most common variant (たら form)
    taraii_u30fb_toii_tara_ta()
}

// Pattern: 中
pub fn naka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: の間に (during/while/between)
// Structures: Verb + 間に / い-Adj + 間に / な-Adj + な + 間に / Noun + の + 間に
pub fn nomani() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct MaMatcher;
    impl Matcher for MaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "間"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(MaMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
    ]
}

// Pattern: うちに (while/during - temporal expression)
// Structures: Verb[る] + うちに / い-Adj + うちに / な-Adj + な + うちに / Noun + の + うちに
pub fn uchini() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct AttributivePrecedingMatcher;
    impl Matcher for AttributivePrecedingMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Matches: Verb (基本形), い-Adjective (基本形), な (助動詞 体言接続), の (助詞 連体化)
            if token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "基本形") {
                return true;
            }
            if token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "基本形") {
                return true;
            }
            if token.surface == "な"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "だ"
                && token.features.get(5).is_some_and(|f| f == "体言接続") {
                return true;
            }
            if token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化") {
                return true;
            }
            false
            })
        }
    }

    #[derive(Debug)]
    struct UchiMatcher;
    impl Matcher for UchiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "うち"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NiCaseParticleMatcher;
    impl Matcher for NiCaseParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(AttributivePrecedingMatcher)),
        TokenMatcher::Custom(Arc::new(UchiMatcher)),
        TokenMatcher::Custom(Arc::new(NiCaseParticleMatcher)),
    ]
}

// Pattern: ないうちに (before/without happening)
// Structures: Verb[ない] + うちに
pub fn naiuchini() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ない auxiliary (negative form)
    #[derive(Debug)]
    struct NaiAuxiliaryMatcher;
    impl Matcher for NaiAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match うち (non-independent noun)
    #[derive(Debug)]
    struct UchiMatcher;
    impl Matcher for UchiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "うち"
                && token.base_form == "うち"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match に particle
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NaiAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(UchiMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
    ]
}

// Pattern: べき (ought to/should - moral obligation)
// Structures: Verb + べき + だ, Verb + べき + Noun
pub fn beki() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match べき auxiliary verb
    #[derive(Debug)]
    struct BekiMatcher;
    impl super::Matcher for BekiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "べき"
                && token.base_form == "べし"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Verb (基本形 or 文語基本形)
        TokenMatcher::Custom(Arc::new(BekiMatcher)),
    ]
}

// Pattern: べきではない (ought not to / should not)
// Structures: Verb + べき + ではない/じゃない/でない/ではありません/じゃありません
pub fn bekidehanai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match べき auxiliary verb
    #[derive(Debug)]
    struct BekiMatcher;
    impl super::Matcher for BekiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "べき"
                && token.base_form == "べし"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match で (助動詞, base='だ') or じゃ (助詞/副助詞)
    #[derive(Debug)]
    struct DeJaMatcher;
    impl super::Matcher for DeJaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "だ")
                || (token.surface == "じゃ" && token.pos.first().is_some_and(|pos| pos == "助詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match は (助詞/係助詞)
    #[derive(Debug)]
    struct WaParticleMatcher;
    impl super::Matcher for WaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "は" && token.pos.first().is_some_and(|pos| pos == "助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Pattern structure - match ending explicitly
    // Matches: べきではない, べきでない, べきじゃない, べきではありません, べきじゃありません
    vec![
        any(), // Verb (基本形)
        TokenMatcher::Custom(Arc::new(BekiMatcher)), // べき
        TokenMatcher::Custom(Arc::new(DeJaMatcher)), // で or じゃ
        optional(TokenMatcher::Custom(Arc::new(WaParticleMatcher))), // optional は
        any(), // ない, あり, etc. - at least one more token to complete the pattern
    ]
}

// Pattern: なかなか (quite/considerably/very)
// Structures: なかなか + Adjective, なかなか + の + Noun
pub fn nakanaka() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct NakanakaMatcher;
    impl Matcher for NakanakaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "なかなか"
                && token.base_form == "なかなか"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "助詞類接続") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![TokenMatcher::Custom(Arc::new(NakanakaMatcher))]
}

// Pattern: あまり (so much that / excessive - leading to negative result)
// Structures:
//   1. Verb (基本形) + あまり
//   2. Noun + の + あまり (includes さ/み derived nouns)
//   3. な-Adjective + な + あまり
pub fn amari() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct AmariPrecedingMatcher;
    impl Matcher for AmariPrecedingMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match the token directly before あまり:
            // 1. Verb (基本形)
            // 2. の (助詞/連体化)
            // 3. な (助動詞/体言接続)

            // Verb in 基本形
            if token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "基本形") {
                return true;
            }

            // の (助詞/連体化)
            if token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化") {
                return true;
            }

            // な (助動詞/体言接続)
            if token.surface == "な"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "だ"
                && token.features.get(5).is_some_and(|f| f == "体言接続") {
                return true;
            }

            false
            })
        }
    }

    #[derive(Debug)]
    struct AmariMatcher;
    impl Matcher for AmariMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            token.surface == "あまり"
                && token.base_form == "あまり"
                && token.pos.first().is_some_and(|pos| pos == "名詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(AmariPrecedingMatcher)),
        TokenMatcher::Custom(Arc::new(AmariMatcher)),
    ]
}

// Pattern: なかなか～ない (hardly/not easily/far from)
// Structures: なかなか + Phrase + Verb[ない]
pub fn nakanaka_uff5e_nai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match なかなか as adverb
    #[derive(Debug)]
    struct NakanakaMatcher;
    impl super::Matcher for NakanakaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "なかなか"
                && token.pos.first().is_some_and(|pos| pos == "副詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match negative auxiliary ない or ません/ん
    #[derive(Debug)]
    struct NegativeMatcher;
    impl super::Matcher for NegativeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            (token.surface == "ない" && token.pos.first().is_some_and(|pos| pos == "助動詞"))

                || (token.surface == "ませ" && token.base_form == "ます" && token.pos.first().is_some_and(|pos| pos == "助動詞"))

                || (token.surface == "ん" && token.pos.first().is_some_and(|pos| pos == "助動詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NakanakaMatcher)),
        wildcard(0, 10, vec![]),
        TokenMatcher::Custom(Arc::new(NegativeMatcher)),
    ]
}

// Pattern: によると・によれば (according to)
// Structures: Noun + によると / Verb + ところ + によると / (1) によれば / によりますと
pub fn niyoruto_u30fb_niyoreba() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Custom matcher for に particle (case-marking)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Custom matcher for よる/より/よれ (verb forms)
    #[derive(Debug)]
    struct YoruVerbMatcher;
    impl Matcher for YoruVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "よる" || token.surface == "より" || token.surface == "よれ")
                && token.base_form == "よる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Custom matcher for ます (polite auxiliary)
    #[derive(Debug)]
    struct MasuMatcher;
    impl Matcher for MasuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Custom matcher for と or ば (conjunctive particles)
    #[derive(Debug)]
    struct ToOrBaMatcher;
    impl Matcher for ToOrBaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "と" || token.surface == "ば")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(YoruVerbMatcher)),
        optional(TokenMatcher::Custom(Arc::new(MasuMatcher))),
        TokenMatcher::Custom(Arc::new(ToOrBaMatcher)),
    ]
}

// Pattern: によって・による (depending on, according to, by means of)
// Structures: Noun + によって / Noun + により / Noun + による
pub fn niyotte_u30fb_niyoru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Custom matcher for によって/により/による
    #[derive(Debug)]
    struct NiYotteMatcher;
    impl Matcher for NiYotteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "によって" || token.surface == "により" || token.surface == "による")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "連語") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(NiYotteMatcher)),
    ]
}

// Pattern: 全く～ない (not at all / completely not)
// Structures: まったく/全く + Phrase[ない]
pub fn mattaku_uff5e_nai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match まったく or 全く as adverb
    #[derive(Debug)]
    struct MattakuMatcher;
    impl Matcher for MattakuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "まったく" || token.surface == "全く")
                && (token.base_form == "まったく" || token.base_form == "全く")
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "助詞類接続") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match ない as auxiliary verb or adjective
    #[derive(Debug)]
    struct NaiMatcher;
    impl Matcher for NaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ない"
                && token.base_form == "ない"
                && (token.pos.first().is_some_and(|pos| pos == "助動詞")
                    || token.pos.first().is_some_and(|pos| pos == "形容詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(MattakuMatcher)),
        wildcard(0, 10, vec![]),
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
    ]
}

// Pattern: ことだ (should/ought to - advice/weak command)
// Structures: Verb[る/ない] + こと + だ/です
pub fn kotoda() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct KotoMatcher;
    impl Matcher for KotoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct DaDesuMatcher;
    impl Matcher for DaDesuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "だ" || token.surface == "です")
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && (token.features.get(4).is_some_and(|f| f == "特殊・ダ")
                    || token.features.get(4).is_some_and(|f| f == "特殊・デス")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
        TokenMatcher::Custom(Arc::new(DaDesuMatcher)),
    ]
}

// Pattern: そうだ (hearsay - I heard that)
// Structures:
//   - Verb + そうだ/そうです
//   - い-Adjective + そうだ/そうです
//   - Noun + だそうだ/だそうです
//   - な-Adjective + だそうだ/だそうです
pub fn souda() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match だ (助動詞, 基本形) - optional for Verb/i-Adj, required for Noun/na-Adj
    #[derive(Debug)]
    struct DaCopulaMatcher;
    impl Matcher for DaCopulaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "だ"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match そう (名詞/特殊/助動詞語幹 OR 名詞/接尾/助動詞語幹)
    #[derive(Debug)]
    struct SouAuxiliaryMatcher;
    impl Matcher for SouAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "そう"
                && token.base_form == "そう"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "特殊" || pos == "接尾")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match だ or です (助動詞, 基本形)
    #[derive(Debug)]
    struct DaDesuMatcher;
    impl Matcher for DaDesuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "だ" || token.surface == "です")
                && (token.base_form == "だ" || token.base_form == "です")
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Content word (Verb, i-Adj, Noun, na-Adj)
        optional(TokenMatcher::Custom(Arc::new(DaCopulaMatcher))), // Optional だ (for Noun/na-Adj)
        TokenMatcher::Custom(Arc::new(SouAuxiliaryMatcher)), // そう
        TokenMatcher::Custom(Arc::new(DaDesuMatcher)), // だ or です
    ]
}

// Pattern: すると (then/upon that/in that case)
// Structures: する + と
pub fn suruto() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct SuruVerbMatcher;
    impl Matcher for SuruVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "する"
                && token.base_form == "する"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立")
                && token.features.get(5).is_some_and(|f| f == "基本形") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct ToConjunctionMatcher;
    impl Matcher for ToConjunctionMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(SuruVerbMatcher)),
        TokenMatcher::Custom(Arc::new(ToConjunctionMatcher)),
    ]
}

// Pattern: そうすると (then/if you do that/in that case)
// Structures: そう + する + と
pub fn sousuruto() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct SouAdverbMatcher;
    impl Matcher for SouAdverbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "そう"
                && token.base_form == "そう"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "助詞類接続") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct SuruVerbMatcher;
    impl Matcher for SuruVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "する"
                && token.base_form == "する"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立")
                && token.features.get(5).is_some_and(|f| f == "基本形") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct ToConjunctionMatcher;
    impl Matcher for ToConjunctionMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(SouAdverbMatcher)),
        TokenMatcher::Custom(Arc::new(SuruVerbMatcher)),
        TokenMatcher::Custom(Arc::new(ToConjunctionMatcher)),
    ]
}

// Pattern: のはXの方だ (the one that A is B)
// Structures: Phrase + のは + Noun + の方だ/です
pub fn nohaxnohouda() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    use super::noun_matcher;

    // Match の as nominalizer (名詞/非自立/一般)
    #[derive(Debug)]
    struct NoNominalizerMatcher;
    impl Matcher for NoNominalizerMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match は as topic particle (助詞/係助詞)
    #[derive(Debug)]
    struct HaParticleMatcher;
    impl Matcher for HaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match の as possessive/connective particle (助詞/連体化)
    #[derive(Debug)]
    struct NoPossessiveMatcher;
    impl Matcher for NoPossessiveMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match 方 as noun (名詞/非自立/一般)
    #[derive(Debug)]
    struct HouNounMatcher;
    impl Matcher for HouNounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "方"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match だ or です as copula (助動詞)
    #[derive(Debug)]
    struct DaDesuCopulaMatcher;
    impl Matcher for DaDesuCopulaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "だ" || token.surface == "です")
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NoNominalizerMatcher)),
        TokenMatcher::Custom(Arc::new(HaParticleMatcher)),
        noun_matcher(), // Any noun (俺, タケル, 私, etc.)
        TokenMatcher::Custom(Arc::new(NoPossessiveMatcher)),
        TokenMatcher::Custom(Arc::new(HouNounMatcher)),
        TokenMatcher::Custom(Arc::new(DaDesuCopulaMatcher)),
    ]
}

// Pattern: Noun＋型 (split form: Noun/Adjective + がた/かた)
// Structures: Noun + がた, い-Adj + かた, Noun + の + かた
pub fn nountasukata() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct NounAdjOrNoMatcher;
    impl Matcher for NounAdjOrNoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match: Noun, Adjective, or の particle (not determiners)
            if token.pos.first().is_some_and(|pos| pos == "名詞") {
                return true;
            }
            if token.pos.first().is_some_and(|pos| pos == "形容詞") {
                return true;
            }
            if token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
            {
                return true;
            }
            false
            })
        }
    }

    #[derive(Debug)]
    struct KataGataMatcher;
    impl Matcher for KataGataMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "がた" || token.surface == "かた")
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && (token.base_form == "がた" || token.base_form == "かた") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NounAdjOrNoMatcher)),
        TokenMatcher::Custom(Arc::new(KataGataMatcher)),
    ]
}

// Pattern: Noun＋型 (compound form: ～型 or ～形 as single token)
// Structures: 文型, etc. (compound nouns ending with 型 or 形)
pub fn nountasukata_compound() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KataKeiCompoundMatcher;
    impl Matcher for KataKeiCompoundMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "名詞")
                && (token.base_form.ends_with("型") || token.base_form.ends_with("形"))
                && token.base_form.len() > 3 => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![TokenMatcher::Custom(Arc::new(KataKeiCompoundMatcher))]
}

// てごらん: Please try to (honorific suggestion)
// Structures: Verb[て] + ごらん, Verb[て] + ごらんなさい
pub fn tegoran() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for て/で conjunction particle
    #[derive(Debug)]
    struct TeDeConjunctionMatcher;
    impl Matcher for TeDeConjunctionMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matcher for ごらん as bound verb noun
    #[derive(Debug)]
    struct GoranMatcher;
    impl Matcher for GoranMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ごらん"
                && token.base_form == "ごらん"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "動詞非自立的") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matcher for optional なさい (imperative form)
    #[derive(Debug)]
    struct NasaiMatcher;
    impl Matcher for NasaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "なさい"
                && token.base_form == "なさる"
                && token.pos.first().is_some_and(|pos| pos == "動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        super::flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(TeDeConjunctionMatcher)),
        TokenMatcher::Custom(Arc::new(GoranMatcher)),
        optional(TokenMatcher::Custom(Arc::new(NasaiMatcher))),
    ]
}

// Pattern: Particle + の (nominalization with particles)
// Structures: Noun + Particle(から/と/へ/で/まで) + の
// Meaning: Forms a link between two nouns where noun B has qualities described by noun A + particle
pub fn particle_no() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match particle (から/と/へ/で/まで) that can precede の
    #[derive(Debug)]
    struct LinkingParticleMatcher;
    impl Matcher for LinkingParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match から/と/へ/で as 格助詞 or まで as 副助詞
            if !["から", "と", "へ", "で", "まで"].contains(&token.surface.as_str()) {
                return false;
            }

            // から, と, へ, で are 格助詞
            if (token.surface == "から" || token.surface == "と"
                || token.surface == "へ" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") {
                return true;
            }

            // まで is 副助詞
            if token.surface == "まで"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞") {
                return true;
            }

            false
            })
        }
    }

    // Match の as nominalizing particle (連体化)
    #[derive(Debug)]
    struct NominalizingNoMatcher;
    impl Matcher for NominalizingNoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        super::noun_matcher(), // Preceding noun
        TokenMatcher::Custom(Arc::new(LinkingParticleMatcher)),
        TokenMatcher::Custom(Arc::new(NominalizingNoMatcher)),
    ]
}

// Pattern: である
// Pattern: である (formal copula - formal equivalent of だ)
// Structures: Noun/な-Adjective + である / Noun/な-Adjective + であります
pub fn dearu() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct DeAuxiliaryMatcher;
    impl Matcher for DeAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "で"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct AruAuxiliaryMatcher;
    impl Matcher for AruAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "ある" || token.surface == "あり")
                && token.base_form == "ある"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct MasuMatcher;
    impl Matcher for MasuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ます"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Noun or na-Adjective
        TokenMatcher::Custom(Arc::new(DeAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(AruAuxiliaryMatcher)),
        optional(TokenMatcher::Custom(Arc::new(MasuMatcher))),
    ]
}

// Pattern: ところが
// Pattern: ところが (however, but unexpectedly)
// Structures: ところが (conjunction showing unexpected result)
pub fn tokoroga() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct TokorogaMatcher;
    impl super::Matcher for TokorogaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ところが"
                && token.base_form == "ところが"
                && token.pos.first().is_some_and(|pos| pos == "接続詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    vec![TokenMatcher::Custom(Arc::new(TokorogaMatcher))]
}

// Pattern: ところで (by the way, incidentally)
// Structures: ところで (conjunction for introducing new topics)
pub fn tokorode() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct TokorodeMatcher;
    impl super::Matcher for TokorodeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ところで"
                && token.base_form == "ところで"
                && token.pos.first().is_some_and(|pos| pos == "接続詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    vec![TokenMatcher::Custom(Arc::new(TokorodeMatcher))]
}

// Pattern: ほど (to the extent that / so much that / about)
// Structures: Verb + ほど, Adjective + ほど, な-Adj + な + ほど, Noun + ほど
pub fn hodo() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    use super::Matcher;

    // Match ほど as particle
    #[derive(Debug)]
    struct HodoParticleMatcher;
    impl Matcher for HodoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ほど"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match: Any (word in attributive form) + ほど
    vec![
        any(),
        TokenMatcher::Custom(Arc::new(HodoParticleMatcher)),
    ]
}

// Pattern: ば〜ほど (the more...the more)
// Structures: Verb[ば] + Verb[る] + ほど, い-Adj[ば] + い-Adj + ほど, Noun + なら(ば) + Noun + ほど, etc.
pub fn ba_u301c_hodo() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    use super::Matcher;

    // Match ば as connective particle
    #[derive(Debug)]
    struct BaParticleMatcher;
    impl Matcher for BaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ば"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match ほど as particle
    #[derive(Debug)]
    struct HodoParticleMatcher;
    impl Matcher for HodoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ほど"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match: [word/aux] + ば + [1-10 tokens] + ほど
    // Note: We match one token before ば. For noun variants like プロならばプロほど,
    // this will match なら+ば, not capturing the noun before. This is a limitation
    // but acceptable for pattern detection purposes.
    vec![
        any(), // Match the token immediately before ば (verb, adj, or auxiliary)
        TokenMatcher::Custom(Arc::new(BaParticleMatcher)),
        wildcard(1, 10, vec![]),
        TokenMatcher::Custom(Arc::new(HodoParticleMatcher)),
    ]
}

// Pattern: ほど～ない (not as...as / not to the extent of)
// Structures: Verb + ほど + Verb[ない], Noun + ほど + Adjective[ない], etc.
pub fn hodo_uff5e_nai() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    use super::Matcher;

    // Match ほど as particle (reuse from hodo())
    #[derive(Debug)]
    struct HodoParticleMatcher;
    impl Matcher for HodoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ほど"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match ない in various forms (助動詞 or 形容詞)
    #[derive(Debug)]
    struct NaiMatcher;
    impl Matcher for NaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.base_form == "ない"
                && (token.pos.first().is_some_and(|pos| pos == "助動詞")
                    || token.pos.first().is_some_and(|pos| pos == "形容詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match: Any + ほど + Wildcard{1-15} + ない
    vec![
        any(),
        TokenMatcher::Custom(Arc::new(HodoParticleMatcher)),
        wildcard(1, 15, vec![]),
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
    ]
}

// Pattern: では・それでは・じゃあ (conjunction/transition)
// Structures: それでは/では/じゃあ/じゃ + Phrase
pub fn deha_u30fb_soredeha_u30fb_jaa() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match それでは, では, じゃあ, or じゃ as conjunction
    #[derive(Debug)]
    struct DehaJaaMatcher;
    impl super::Matcher for DehaJaaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            token.pos.first().is_some_and(|pos| pos == "接続詞")
                && (token.surface == "それでは"
                    || token.surface == "では"
                    || token.surface == "じゃあ"
                    || token.surface == "じゃ") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![TokenMatcher::Custom(Arc::new(DehaJaaMatcher))]
}

// Pattern: のに (in order to / for - purpose/goal)
// Structures: Verb[る] + の + に
//
// NOTE: This is distinct from N4 "のに " (despite).
// N3: Verb + の (nominalizer) + に (goal marker) - two tokens
// N4: Any + のに (conjunction particle) - single token
// The tokenization difference allows us to distinguish them.
pub fn noni_2() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match の as 名詞/非自立 (nominalizer)
    #[derive(Debug)]
    struct NoNominalizerMatcher;
    impl Matcher for NoNominalizerMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match に as 助詞/格助詞 (case-marking particle)
    #[derive(Debug)]
    struct NiCaseParticleMatcher;
    impl Matcher for NiCaseParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        verb(),
        TokenMatcher::Custom(Arc::new(NoNominalizerMatcher)),
        TokenMatcher::Custom(Arc::new(NiCaseParticleMatcher)),
    ]
}

// Pattern: ため(に) (for the sake of / in order to - purpose)
// Structures: Verb[る] + ため(に) / Noun + の + ため(に)
pub fn tame_ni() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct TameMatcher;
    impl Matcher for TameMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            token.surface == "ため"
                && token.base_form == "ため"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NoRentaikaMatcher;
    impl Matcher for NoRentaikaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Verb or Noun
        optional(TokenMatcher::Custom(Arc::new(
            NoRentaikaMatcher,
        ))), // Optional の for nouns
        TokenMatcher::Custom(Arc::new(TameMatcher)),
        optional(TokenMatcher::Custom(Arc::new(
            NiParticleMatcher,
        ))), // Optional に
    ]
}

// Pattern: ために (due to, because of, for the sake of)
// Structures: Verb + ため(に) / い-Adjective + ため(に) / な-Adjective + な + ため(に) / Noun + の + ため(に)
pub fn tameni() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct TameMatcher;
    impl Matcher for TameMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            token.surface == "ため"
                && token.base_form == "ため"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // We need to handle 4 cases:
    // 1. Verb + ため(に)
    // 2. い-Adjective + ため(に)
    // 3. な-Adjective + な + ため(に)
    // 4. Noun + の + ため(に)
    //
    // The challenge is that な-adjective needs TWO tokens before ため (stem + な),
    // while the others need only ONE token.
    // We'll use a custom matcher that handles all cases:

    #[derive(Debug)]
    struct PreTameMatcherAny;
    impl Matcher for PreTameMatcherAny {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match Verb, い-Adjective, な-Adjective stem, Noun, or particles の/な
            let is_verb = token.pos.first().is_some_and(|pos| pos == "動詞");
            let is_i_adj = token.pos.first().is_some_and(|pos| pos == "形容詞");
            let is_na_adj_stem = token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹");
            let is_noun = token.pos.first().is_some_and(|pos| pos == "名詞");
            let is_na = token.surface == "な" && token.base_form == "だ";
            let is_no = token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化");

            is_verb || is_i_adj || is_na_adj_stem || is_noun || is_na || is_no
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(PreTameMatcherAny)), // Verb/Adj/Noun or particle
        optional(TokenMatcher::Custom(Arc::new(
            PreTameMatcherAny,
        ))), // Optional second token (for な-Adj or Noun+の)
        TokenMatcher::Custom(Arc::new(TameMatcher)),
        optional(TokenMatcher::Custom(Arc::new(
            NiParticleMatcher,
        ))), // Optional に
    ]
}

// Pattern: ということだ (it is said that / it means that - hearsay/conclusion with certainty)
// Structures: Phrase + ということ + だ / Phrase + ということ + です
pub fn toiukotoda() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct ToiuMatcher;
    impl Matcher for ToiuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "という"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "連語") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct KotoMatcher;
    impl Matcher for KotoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct DaDesuMatcher;
    impl Matcher for DaDesuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "だ" || token.surface == "です")
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && (token.features.get(4).is_some_and(|f| f == "特殊・ダ")
                    || token.features.get(4).is_some_and(|f| f == "特殊・デス")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(ToiuMatcher)),
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
        TokenMatcher::Custom(Arc::new(DaDesuMatcher)),
    ]
}

// Pattern: というのは (the thing known as, what I mean is)
// Structures: というのは / とは
// Note: って variant is handled by separate って pattern
pub fn toiunoha() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match という (compound particle)
    #[derive(Debug)]
    struct ToiuMatcher;
    impl Matcher for ToiuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "という"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "連語") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match の as noun (nominalizer)
    #[derive(Debug)]
    struct NoNounMatcher;
    impl Matcher for NoNounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "名詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match は (topic particle)
    #[derive(Debug)]
    struct HaTopicMatcher;
    impl Matcher for HaTopicMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Full form: という + の + は
    vec![
        TokenMatcher::Custom(Arc::new(ToiuMatcher)),
        TokenMatcher::Custom(Arc::new(NoNounMatcher)),
        TokenMatcher::Custom(Arc::new(HaTopicMatcher)),
    ]
}

// Pattern: とは (abbreviated form of というのは)
pub fn toha() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match と as case particle
    #[derive(Debug)]
    struct TohaToMatcher;
    impl Matcher for TohaToMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match は (topic particle)
    #[derive(Debug)]
    struct TohaHaMatcher;
    impl Matcher for TohaHaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Abbreviated form: と + は
    vec![
        TokenMatcher::Custom(Arc::new(TohaToMatcher)),
        TokenMatcher::Custom(Arc::new(TohaHaMatcher)),
    ]
}

// Pattern: 的 (like / -ish / -ly)
// Structures: Noun + 的 + に / Noun + 的 + な + Noun
pub fn teki() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match 的 as suffix (名詞/接尾/形容動詞語幹)
    #[derive(Debug)]
    struct TekiSuffixMatcher;
    impl super::Matcher for TekiSuffixMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "的"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match に as adverbializing particle OR な as copula
    #[derive(Debug)]
    struct NiOrNaMatcher;
    impl super::Matcher for NiOrNaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            (token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副詞化"))

                || (token.surface == "な"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.features.get(5).is_some_and(|f| f == "体言接続")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(TekiSuffixMatcher)),
        TokenMatcher::Custom(Arc::new(NiOrNaMatcher)),
    ]
}

// Pattern: もの・もの (because / 'cause - sentence-ending particle)
// Structures: Verb/Adj + もの, Verb/Adj + もん, Noun/な-Adj + だ + もの/もん, + ん + だ + もの/もん
pub fn mono_u30fb_mon() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ん as dependent noun (explanatory particle)
    #[derive(Debug)]
    struct NParticleMatcher;
    impl super::Matcher for NParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ん"
                && token.base_form == "ん"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match だ as auxiliary (助動詞)
    #[derive(Debug)]
    struct DaAuxMatcher;
    impl super::Matcher for DaAuxMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "だ" || token.surface == "な")
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match もの or もん (both as noun OR もん as particle)
    #[derive(Debug)]
    struct MonoOrMonMatcher;
    impl super::Matcher for MonoOrMonMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            (token.surface == "もの"
                && token.base_form == "もの"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立"))

                || (token.surface == "もん"
                    && token.base_form == "もん"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "終助詞"))

                || (token.surface == "もん"
                    && token.base_form == "もん"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "非自立")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match content words that can precede もの/もん
    // This includes verbs, adjectives, nouns, and certain auxiliary verbs (like ない)
    #[derive(Debug)]
    struct ContentWordMatcher;
    impl super::Matcher for ContentWordMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match verbs (動詞)
            token.pos.first().is_some_and(|pos| pos == "動詞")
                // OR adjectives (形容詞)
                || token.pos.first().is_some_and(|pos| pos == "形容詞")
                // OR nouns (名詞) - but only non-auxiliary nouns
                || (token.pos.first().is_some_and(|pos| pos == "名詞")
                    && !token.pos.get(1).is_some_and(|pos| pos == "非自立"))
                // OR auxiliary verbs (助動詞) - for cases like ないんだもの
                || token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    // Pattern: ContentWord + [optional: ん] + [optional: だ] + (もの|もん)
    // This matches all variants:
    // - Verb/Adj + もの
    // - Verb/Adj + もん
    // - Noun/な-Adj + だ + もの
    // - Noun/な-Adj + だ + もん
    // - Verb/Adj + ん + だ + もの
    // - Verb/Adj + ん + だ + もん
    // - Noun/な-Adj + な + ん + だ + もの
    vec![
        TokenMatcher::Custom(Arc::new(ContentWordMatcher)),
        optional(TokenMatcher::Custom(Arc::new(
            NParticleMatcher,
        ))),
        optional(TokenMatcher::Custom(Arc::new(
            DaAuxMatcher,
        ))),
        TokenMatcher::Custom(Arc::new(MonoOrMonMatcher)), // Matches もの or もん
    ]
}

// Pattern: ものだ (should / naturally is / common sense)
// Structures: Any + もの/もん + だ/ではない/じゃない
pub fn monoda() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match もの or もん (dependent noun)
    #[derive(Debug)]
    struct MonoMatcher;
    impl super::Matcher for MonoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "もの" || token.surface == "もん")
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match だ (助動詞, 基本形)
    #[derive(Debug)]
    struct DaMatcher;
    impl super::Matcher for DaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "だ" => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(MonoMatcher)),
        TokenMatcher::Custom(Arc::new(DaMatcher)),
    ]
}

// Pattern: ものではない (negative form)
pub fn monoda_dewanai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match もの or もん (dependent noun)
    #[derive(Debug)]
    struct MonoMatcher;
    impl super::Matcher for MonoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "もの" || token.surface == "もん")
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match で (助動詞, base='だ')
    #[derive(Debug)]
    struct DeMatcher;
    impl super::Matcher for DeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "だ" => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match ない (形容詞)
    #[derive(Debug)]
    struct NaiMatcher;
    impl super::Matcher for NaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ない"
                && token.pos.first().is_some_and(|pos| pos == "形容詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(MonoMatcher)),
        TokenMatcher::Custom(Arc::new(DeMatcher)),
        surface("は"),
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
    ]
}

// Pattern: ものじゃない (casual negative)
pub fn monoda_janai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match もの or もん (dependent noun)
    #[derive(Debug)]
    struct MonoMatcher;
    impl super::Matcher for MonoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "もの" || token.surface == "もん")
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match じゃ (助詞/副助詞)
    #[derive(Debug)]
    struct JaMatcher;
    impl super::Matcher for JaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "じゃ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match ない (助動詞)
    #[derive(Debug)]
    struct NaiMatcher;
    impl super::Matcher for NaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(MonoMatcher)),
        TokenMatcher::Custom(Arc::new(JaMatcher)),
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
    ]
}

// Pattern: 最中に (right in the middle of / in the midst of)
// Structures: Verb[ている] + 最中に/最中だ/最中です OR Noun + の + 最中に/最中だ/最中です
pub fn saichuuni() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match 最中 (saichuu) as noun/adverbial
    #[derive(Debug)]
    struct SaichuuMatcher;
    impl Matcher for SaichuuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "最中"
                && token.base_form == "最中"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副詞可能") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match に (case particle), だ (copula), or です (polite copula) after 最中
    #[derive(Debug)]
    struct NiDaDesuMatcher;
    impl Matcher for NiDaDesuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // に as case particle
            if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            {
                return true;
            }

            // だ as auxiliary verb
            if token.surface == "だ"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
            {
                return true;
            }

            // です as auxiliary verb
            if token.surface == "です"
                && token.base_form == "です"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
            {
                return true;
            }

            false
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(SaichuuMatcher)),
        TokenMatcher::Custom(Arc::new(NiDaDesuMatcher)),
    ]
}

// Pattern: 上で
// Pattern: 上で (upon / after - formal progression)
// Structures: Verb[た] + 上で / Noun + の + 上で
pub fn uede() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for 上 (dependent noun meaning "above/upon")
    #[derive(Debug)]
    struct UeMatcher;
    impl Matcher for UeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "上"
                && token.base_form == "上"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matcher for で particle
    #[derive(Debug)]
    struct DeParticleMatcher;
    impl Matcher for DeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(UeMatcher)),
        TokenMatcher::Custom(Arc::new(DeParticleMatcher)),
    ]
}

// Pattern: おかげで (thanks to / because of)
// Structures:
//   - Verb (attributive) + おかげで
//   - い-Adjective + おかげで
//   - な-Adjective + な + おかげで
//   - Noun + の + おかげで
pub fn okagede() -> Vec<TokenMatcher> {
    use crate::pattern_matcher::{MatchContext, TokenMatcher};

    // Match おかげ as noun
    #[derive(Debug)]
    struct OkageMatcher;
    impl Matcher for OkageMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "おかげ"
                && token.pos.first().is_some_and(|pos| pos == "名詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match で as case particle
    #[derive(Debug)]
    struct DeCaseParticleMatcher;
    impl Matcher for DeCaseParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match any verb, adjective, or noun that can precede おかげで
    // This includes:
    // - Verbs in dictionary/attributive form
    // - Past tense auxiliary た (after verbs)
    // - い-adjectives in basic form
    // - な-adjectives with な
    // - Nouns with の
    #[derive(Debug)]
    struct AttributivePrecedingMatcher;
    impl Matcher for AttributivePrecedingMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Verb in dictionary/attributive form
            let is_verb = token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "基本形" || f == "連体形");

            // Past tense auxiliary た (基本形)
            let is_past_aux = token.surface == "た"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "た"
                && token.features.get(5).is_some_and(|f| f == "基本形");

            // い-adjective in basic form
            let is_i_adj = token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "基本形");

            // な (copula in attributive form after na-adjective)
            let is_na_copula = token.surface == "な"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "だ"
                && token.features.get(5).is_some_and(|f| f == "体言接続");

            // の particle (after noun)
            let is_no_particle = token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "連体化")
                    || token.pos.get(1).is_some_and(|pos| pos == "格助詞"));

            is_verb || is_past_aux || is_i_adj || is_na_copula || is_no_particle
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(AttributivePrecedingMatcher)),
        TokenMatcher::Custom(Arc::new(OkageMatcher)),
        TokenMatcher::Custom(Arc::new(DeCaseParticleMatcher)),
    ]
}

// Pattern: にもとづいて (based on)
// Structures: Noun + に基づいて / Noun + に基づいた + Noun
pub fn nimotozuite() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.base_form == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct MotozukuMatcher;
    impl Matcher for MotozukuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立")
                && (token.base_form == "もとづく" || token.base_form == "基づく") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct TeOrTaMatcher;
    impl Matcher for TeOrTaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            (token.surface == "て"
                && token.base_form == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞"))

                || (token.surface == "た"
                    && token.base_form == "た"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(MotozukuMatcher)),
        TokenMatcher::Custom(Arc::new(TeOrTaMatcher)),
    ]
}

// Pattern: 点 (point / aspect / respect)
// Structures: Verb/Adj/Noun + 点（で/が/は/etc）
pub fn ten() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct TenMatcher;
    impl Matcher for TenMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "点"
                && token.base_form == "点"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![TokenMatcher::Custom(Arc::new(TenMatcher))]
}

// Pattern: なぜなら〜から (because / the reason is)
// Structures: なぜなら(ば) + Reason Phrase + から + だ/です
pub fn nazenara_u301c_kara() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matches なぜなら OR なぜ (starting either variant)
    #[derive(Debug)]
    struct NazenaraOrNazeMatcher;
    impl Matcher for NazenaraOrNazeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            (token.surface == "なぜなら"
                && token.base_form == "なぜなら"
                && token.pos.first().is_some_and(|pos| pos == "接続詞"))

            || (token.surface == "なぜ"
                && token.base_form == "なぜ"
                && token.pos.first().is_some_and(|pos| pos == "副詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matches なら (auxiliary verb - hypothetical form of だ)
    // Only present in three-token variant (なぜならば)
    #[derive(Debug)]
    struct NaraMatcher;
    impl Matcher for NaraMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "なら"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matches ば (connective particle)
    // Only present in three-token variant (なぜならば)
    #[derive(Debug)]
    struct BaParticleMatcher;
    impl Matcher for BaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ば"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matches から (connective particle meaning "because")
    #[derive(Debug)]
    struct KaraConnectiveMatcher;
    impl Matcher for KaraConnectiveMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "から"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matches だ or です (copula auxiliary verb)
    #[derive(Debug)]
    struct DaDesuMatcher;
    impl Matcher for DaDesuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "だ" || token.surface == "です")
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matches comma (optional)
    #[derive(Debug)]
    struct CommaMatcher;
    impl Matcher for CommaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "、"
                && token.pos.first().is_some_and(|pos| pos == "記号")
                && token.pos.get(1).is_some_and(|pos| pos == "読点") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        // Match なぜなら (1 token) OR なぜ (first of 3 tokens)
        TokenMatcher::Custom(Arc::new(NazenaraOrNazeMatcher)),
        // Optional なら + ば (only for three-token variant)
        optional(TokenMatcher::Custom(Arc::new(NaraMatcher))),
        optional(TokenMatcher::Custom(Arc::new(BaParticleMatcher))),
        // Optional comma
        optional(TokenMatcher::Custom(Arc::new(CommaMatcher))),
        // Wildcard for the reason phrase (1-20 tokens)
        wildcard(1, 20, vec![]),
        // から (connective particle)
        TokenMatcher::Custom(Arc::new(KaraConnectiveMatcher)),
        // だ or です (copula)
        TokenMatcher::Custom(Arc::new(DaDesuMatcher)),
    ]
}

// Pattern: こそ (emphasis particle)
// Structures: Noun + こそ
pub fn koso() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KosoMatcher;
    impl Matcher for KosoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "こそ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(KosoMatcher)),
    ]
}

// Pattern: からこそ (precisely because)
// Structures: Verb + た + から + こそ, Noun + だ + から + こそ
pub fn karakoso() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matches た (past auxiliary) or だ (copula) before から
    #[derive(Debug)]
    struct TaOrDaMatcher;
    impl super::Matcher for TaOrDaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "助動詞")
                && (token.base_form == "た" || token.base_form == "だ") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matches から (conjunction particle)
    #[derive(Debug)]
    struct KaraConjunctionMatcher;
    impl super::Matcher for KaraConjunctionMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "から"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matches こそ (emphatic particle)
    #[derive(Debug)]
    struct KosoMatcher;
    impl super::Matcher for KosoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "こそ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(TaOrDaMatcher)),
        TokenMatcher::Custom(Arc::new(KaraConjunctionMatcher)),
        TokenMatcher::Custom(Arc::new(KosoMatcher)),
    ]
}

// Pattern: ばかり
// Pattern: ばかり (nothing but / only)
// Structures: Verb[て] + ばかり, Noun + ばかり
pub fn bakari() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ばかり as adverbial particle (助詞/副助詞)
    #[derive(Debug)]
    struct BakariParticleMatcher;
    impl Matcher for BakariParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ばかり"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(BakariParticleMatcher)),
    ]
}

// Pattern: ばかりだ (keeps on / only ~ is occurring)
// Structures: Verb[る] + ばかり + だ/です/で
pub fn bakarida() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ばかり as adverbial particle (助詞/副助詞)
    #[derive(Debug)]
    struct BakariParticleMatcher;
    impl Matcher for BakariParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ばかり"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match だ, です, or で as auxiliary verb (助動詞)
    #[derive(Debug)]
    struct DaDesuDeMatcher;
    impl Matcher for DaDesuDeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "だ" || token.surface == "です" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(BakariParticleMatcher)),
        TokenMatcher::Custom(Arc::new(DaDesuDeMatcher)),
    ]
}

// Pattern: ばかりに (simply because / just because)
// Structures: Verb[た] + ばかりに, Adj + ばかりに, な-Adj + な + ばかりに, Noun + である + ばかりに
pub fn bakarini() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ばかり as adverbial particle (助詞/副助詞)
    #[derive(Debug)]
    struct BakariParticleMatcher;
    impl Matcher for BakariParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ばかり"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match に as case particle (助詞/格助詞)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(BakariParticleMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
    ]
}

// Pattern: ことがある (sometimes happens / there are times when)
// Structures: Verb/Adj + こと + が/も + ある
pub fn kotogaaru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct KotoMatcher;
    impl Matcher for KotoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct GaMoParticleMatcher;
    impl Matcher for GaMoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "が" || token.surface == "も")
                && token.pos.first().is_some_and(|pos| pos == "助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
        TokenMatcher::Custom(Arc::new(GaMoParticleMatcher)),
        verb_base("ある"),
    ]
}

// Pattern: ことにする
// Pattern: ことにする (decide to / make it that)
// Structures: Verb[る/ない] + ことにする/します
// Expresses conscious decision-making
pub fn kotonisuru() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KotoMatcher;
    impl Matcher for KotoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        verb_base("する"),
    ]
}

// Pattern: ことなの (explanatory "it is that")
// Structures: こと + な + の/ん
pub fn kotonano() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match こと (名詞/非自立)
    #[derive(Debug)]
    struct KotoMatcher;
    impl Matcher for KotoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "こと"
                && token.base_form == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match な (助動詞, base=だ)
    #[derive(Debug)]
    struct NaMatcher;
    impl Matcher for NaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "な"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match の (名詞/非自立 or 助詞/終助詞) or ん (名詞/非自立)
    #[derive(Debug)]
    struct NoOrNMatcher;
    impl Matcher for NoOrNMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // の as 名詞/非自立 or 助詞/終助詞
            if token.surface == "の" {
                (token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "非自立"))
                    || (token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "終助詞"))
            } else if token.surface == "ん" {
                // ん as 名詞/非自立
                token.base_form == "ん"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "非自立")
            } else {
                false
            }
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
        TokenMatcher::Custom(Arc::new(NaMatcher)),
        TokenMatcher::Custom(Arc::new(NoOrNMatcher)),
    ]
}

// Pattern: ことになる (it has been decided / will end up)
// Structures: Verb/Adjective + ことになる
pub fn kotoninaru() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KotoMatcher;
    impl Matcher for KotoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        verb_base("なる"),
    ]
}

// Pattern: ～は～で有名 (famous for)
// Structures:
//   Noun + は + Verb/Adj + こと/の + で + 有名
//   Noun + は + Noun + で + 有名
pub fn uff5e_ha_uff5e_deyuumei() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matches で (格助詞) or ので (接続助詞)
    #[derive(Debug)]
    struct DeOrNodeMatcher;
    impl Matcher for DeOrNodeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞"))
                || (token.surface == "ので"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matches ゆう (verb, hiragana) OR 有名 (kanji, na-adjective stem)
    #[derive(Debug)]
    struct YuuOrYuumeiMatcher;
    impl Matcher for YuuOrYuumeiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            (token.surface == "ゆう"
                && token.base_form == "ゆう"
                && token.pos.first().is_some_and(|pos| pos == "動詞"))

                || (token.surface == "有名"
                    && token.base_form == "有名"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matches めい (noun) after ゆう verb - only for hiragana form
    #[derive(Debug)]
    struct MeiNounMatcher;
    impl Matcher for MeiNounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "めい"
                && token.base_form == "めい"
                && token.pos.first().is_some_and(|pos| pos == "名詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(DeOrNodeMatcher)),
        TokenMatcher::Custom(Arc::new(YuuOrYuumeiMatcher)),
        optional(TokenMatcher::Custom(Arc::new(MeiNounMatcher))),
    ]
}

// Pattern: ことはない (no need to / never happens)
// Structures: Verb + ことはない/ありません
pub fn kotohanai() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KotoMatcher;
    impl Matcher for KotoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct HaParticleMatcher;
    impl Matcher for HaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NaiAruMatcher;
    impl Matcher for NaiAruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match ない (i-adjective) or ある (verb for polite ありません)
            (token.surface == "ない" || token.base_form == "ない")
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
                || (token.base_form == "ある"
                    && token.pos.first().is_some_and(|pos| pos == "動詞"))
            })
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
        TokenMatcher::Custom(Arc::new(HaParticleMatcher)),
        TokenMatcher::Custom(Arc::new(NaiAruMatcher)),
    ]
}

// Pattern:  ～と言っても
pub fn uff5e_toittemo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: といえば (speaking of, when it comes to)
// Structures: Noun + といえば/というと/といったら
pub fn toieba() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for と as quotation particle
    #[derive(Debug)]
    struct ToQuotationMatcher;
    impl Matcher for ToQuotationMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "引用") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matcher for いう verb in various forms
    #[derive(Debug)]
    struct IuVerbMatcher;
    impl Matcher for IuVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.base_form == "いう"
                && token.pos.first().is_some_and(|pos| pos == "動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // This pattern needs to match three variants:
    // 1. Noun + と + いう(仮定形) + ば = といえば
    // 2. Noun + と + いう(基本形) + と = というと
    // 3. Noun + と + いう(連用タ接続) + たら = といったら
    //
    // We'll match: Noun + と(引用) + いう + (ば or と or たら)
    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(ToQuotationMatcher)),
        TokenMatcher::Custom(Arc::new(IuVerbMatcher)),
        TokenMatcher::Custom(Arc::new(EndingMatcher)),
    ]
}

// Helper matcher for the ending (ば, と, or たら)
#[derive(Debug)]
struct EndingMatcher;
impl Matcher for EndingMatcher {
    fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
        (token.surface == "ば"
            && token.pos.first().is_some_and(|pos| pos == "助詞")
            && token.pos.get(1).is_some_and(|pos| pos == "接続助詞"))

        || (token.surface == "と"
            && token.pos.first().is_some_and(|pos| pos == "助詞")
            && token.pos.get(1).is_some_and(|pos| pos == "接続助詞"))

        || (token.surface == "たら"
            && token.base_form == "た"
            && token.pos.first().is_some_and(|pos| pos == "助動詞")) => (true, 1),
                _ => (false, 0),
            }
        }
}

// Pattern: 合う (to do mutually/reciprocally with another)
// Structures: Verb[stem] + 合う
pub fn au() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for verb in 連用形 (stem form)
    #[derive(Debug)]
    struct VerbStemMatcher;
    impl Matcher for VerbStemMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matcher for 'あう' as dependent verb (non-self-standing)
    // Note: Kagome may tokenize this as either "あう" or "ある" in the base form
    // Both forms appear when あう is used as a suffix meaning "to do together"
    #[derive(Debug)]
    struct AuDependentVerbMatcher;
    impl Matcher for AuDependentVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.base_form == "あう" || token.base_form == "ある")
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbStemMatcher)),
        TokenMatcher::Custom(Arc::new(AuDependentVerbMatcher)),
    ]
}

// Pattern: に合わせて・に合った (in accordance with / matching)
// Structures: Noun + に合わせて / Noun + に合った + Noun
pub fn niawasete_u30fb_niatta() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    use super::noun_matcher;

    // Matcher for に particle
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matcher for the verb: either 合わせ/あわせ (連用形) OR あっ/合っ (連用タ接続)
    #[derive(Debug)]
    struct AwaseVerbMatcher;
    impl Matcher for AwaseVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                if !token.pos.first().is_some_and(|pos| pos == "動詞") {
                return false;
            }

            // Match 合わせ/あわせ in 連用形 (for に合わせて)
            if (token.base_form == "合わせる" || token.base_form == "あわせる")
                && token.features.get(5).is_some_and(|f| f == "連用形")
            {
                return true;
            }

            // Match あっ/合っ in 連用タ接続 (for に合った)
            if (token.base_form == "ある" || token.base_form == "合う")
                && token.features.get(5).is_some_and(|f| f == "連用タ接続")
            {
                return true;
            }

            false
            })
        }
    }

    // Matcher for the final particle: either て (接続助詞) OR た (助動詞)
    #[derive(Debug)]
    struct TeOrTaMatcher;
    impl Matcher for TeOrTaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match て as 助詞/接続助詞 (for に合わせて)
            if token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
            {
                return true;
            }

            // Match た as 助動詞 (for に合った)
            if token.surface == "た"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "た"
            {
                return true;
            }

            false
            })
        }
    }

    // This pattern matches both variants:
    // 1. Noun + に + 合わせ/あわせ + て (に合わせて)
    // 2. Noun + に + あっ/合っ + た (に合った)
    vec![
        noun_matcher(),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(AwaseVerbMatcher)),
        TokenMatcher::Custom(Arc::new(TeOrTaMatcher)),
    ]
}

// Pattern: について (about, concerning)
// Structures: Noun + について / Noun + について + の + Noun
pub fn nitsuite() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct NitsuiteMatcher;
    impl Matcher for NitsuiteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "について"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "連語") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(NitsuiteMatcher)),
    ]
}

// Pattern: ～(の)姿 (figure / appearance / state)
// Structures: Verb + 姿, Noun + (の) + 姿
pub fn uff5e_no_sugata() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for 姿 as a noun (名詞/一般)
    #[derive(Debug)]
    struct SugataMatcher;
    impl super::Matcher for SugataMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "姿"
                && token.base_form == "姿"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![TokenMatcher::Custom(Arc::new(SugataMatcher))]
}

// Pattern: と言える (can say that / it is fair to say)
// Structures: Phrase + と + (も) + いえる/いえよう
pub fn toieru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for と particle (quotation/citation)
    #[derive(Debug)]
    struct ToParticleMatcher;
    impl Matcher for ToParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "引用") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matcher for optional も particle
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matcher for いえる verb (potential form of 言う)
    // Accepts: いえる (基本形), いえよ (未然ウ接続), いえ (連用形)
    #[derive(Debug)]
    struct IeruMatcher;
    impl Matcher for IeruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.base_form == "いえる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(ToParticleMatcher)),
        optional(TokenMatcher::Custom(Arc::new(MoParticleMatcher))),
        TokenMatcher::Custom(Arc::new(IeruMatcher)),
    ]
}

// ちゃんと・きちんと: Properly/neatly (adverbs)
// Structures: ちゃんと/きちんと + Phrase
pub fn chanto_u30fb_kichinto() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for ちゃんと or きちんと as adverbs
    #[derive(Debug)]
    struct ChantoKichintoMatcher;
    impl Matcher for ChantoKichintoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "ちゃんと" || token.surface == "きちんと")
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![TokenMatcher::Custom(Arc::new(ChantoKichintoMatcher))]
}

// Pattern: そのため(に) (for that reason/to that end)
// Structures: そのため + (に) + Phrase
pub fn sonotame_ni() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct SonoRentaishiMatcher;
    impl super::Matcher for SonoRentaishiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            token.surface == "その"
                && token.base_form == "その"
                && token.pos.first().is_some_and(|pos| pos == "連体詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct TameNounMatcher;
    impl super::Matcher for TameNounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            token.surface == "ため"
                && token.base_form == "ため"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(SonoRentaishiMatcher)),
        TokenMatcher::Custom(Arc::new(TameNounMatcher)),
        optional(TokenMatcher::Custom(Arc::new(
            NiParticleMatcher,
        ))),
    ]
}

// Pattern: その結果 (as a result)
// Structure: その + 結果
pub fn sonokekka() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matches その as 連体詞
    #[derive(Debug)]
    struct SonoRentaishiMatcher;
    impl super::Matcher for SonoRentaishiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "その" && token.pos.first().is_some_and(|pos| pos == "連体詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matches 結果 as 名詞/副詞可能
    #[derive(Debug)]
    struct KekkaFukushiKanouMatcher;
    impl super::Matcher for KekkaFukushiKanouMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "結果"
                && token.base_form == "結果"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副詞可能") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(SonoRentaishiMatcher)),
        TokenMatcher::Custom(Arc::new(KekkaFukushiKanouMatcher)),
    ]
}

// Pattern: に比べて (compared to)
// Structures: Noun + に + 比べて/比べたら/比べれば/比べると
pub fn nikurabete() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match に particle (格助詞)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match 比べる/くらべる verb in various forms
    #[derive(Debug)]
    struct KuraberuVerbMatcher;
    impl super::Matcher for KuraberuVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "動詞")
                && (token.base_form == "比べる" || token.base_form == "くらべる") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match て/ば/と/たら particles (conjunctive/conditional)
    #[derive(Debug)]
    struct ConditionalParticleMatcher;
    impl super::Matcher for ConditionalParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // て (接続助詞) - for に比べて
            (token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞"))
            // ば (接続助詞) - for に比べれば
            || (token.surface == "ば"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞"))
            // と (接続助詞) - for に比べると
            || (token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞"))
            // たら (助動詞, 仮定形) - for に比べたら
            || (token.surface == "たら"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "た")
            })
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(KuraberuVerbMatcher)),
        TokenMatcher::Custom(Arc::new(ConditionalParticleMatcher)),
    ]
}

// Pattern: どんなに〜ても (no matter how)
// Structures: どんなに + Verb/Adj[ても] OR どんなに + Noun/な-Adj + でも
pub fn donnani_u301c_temo() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match どんなに (adverb) OR どんな (adnominal without に)
    #[derive(Debug)]
    struct DonnaniMatcher;
    impl Matcher for DonnaniMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "どんなに" && token.base_form == "どんなに"
                && token.pos.first().is_some_and(|p| p == "副詞"))
            || (token.surface == "どんな" && token.base_form == "どんな"
                && token.pos.first().is_some_and(|p| p == "連体詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match Verb/Adjective in conjunctive form, OR Noun/な-Adjective
    #[derive(Debug)]
    struct PreTemoMatcher;
    impl Matcher for PreTemoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Verb in 連用タ接続 or 連用形
            if token.pos.first().is_some_and(|p| p == "動詞") {
                return token.features.get(5).is_some_and(|f|
                    f == "連用タ接続" || f == "連用形");
            }
            // い-Adjective in 連用テ接続
            if token.pos.first().is_some_and(|p| p == "形容詞") {
                return token.features.get(5).is_some_and(|f| f == "連用テ接続");
            }
            // Noun or な-Adjective (形容動詞語幹)
            if token.pos.first().is_some_and(|p| p == "名詞") {
                return true;
            }
            // 助動詞 like たい in 連用テ接続 (for たくても)
            if token.pos.first().is_some_and(|p| p == "助動詞") {
                return token.features.get(5).is_some_and(|f| f == "連用テ接続");
            }
            false
            })
        }
    }

    // Match ても (て + も) OR でも (as single token OR で + も)
    #[derive(Debug)]
    struct TemoOrDemoMatcher;
    impl Matcher for TemoOrDemoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            (token.surface == "て" && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞"))

            || (token.surface == "で" && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞"))

            || (token.surface == "でも" && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "副助詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match も particle (only if not already matched as でも)
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "も"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(DonnaniMatcher)),
        wildcard(0, 3, vec![]),
        TokenMatcher::Custom(Arc::new(PreTemoMatcher)),
        TokenMatcher::Custom(Arc::new(TemoOrDemoMatcher)),
        optional(TokenMatcher::Custom(Arc::new(MoParticleMatcher))),
    ]
}

// Pattern: いくら〜でも
// いくら〜でも: No matter how much (いくら + phrase + ても/でも)
// Structures:
//   - いくら + Verb[ても] (いくら...て + も)
//   - いくら + い-Adjective[ても] (いくら...て + も)
//   - いくら + Noun + でも (いくら...でも as single token)
//   - いくら + な-Adjective + でも (いくら...でも as single token)
pub fn ikura_u301c_demo() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match いくら as adverb or noun
    #[derive(Debug)]
    struct IkuraMatcher;
    impl Matcher for IkuraMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "いくら"
                && (token.pos.first().is_some_and(|pos| pos == "副詞")
                    || token.pos.first().is_some_and(|pos| pos == "名詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match ても (も after て) or でも (single particle)
    #[derive(Debug)]
    struct TemoOrDemoMatcher;
    impl Matcher for TemoOrDemoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match も (係助詞) - for ても pattern
            (token.surface == "も" && token.pos.get(1).is_some_and(|pos| pos == "係助詞"))
                // Match でも (副助詞) - for noun/na-adj + でも pattern
                || (token.surface == "でも" && token.pos.get(1).is_some_and(|pos| pos == "副助詞"))
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(IkuraMatcher)),
        wildcard(1, 5, vec![]),
        TokenMatcher::Custom(Arc::new(TemoOrDemoMatcher)),
    ]
}

// Pattern: 〜かは〜によって違う
/// Match 〜かは〜によって違う (whether A depends on B / differs depending on)
/// Structures:
/// 1. Phrase + かどうか + は + Noun + によって違う
/// 2. Phrase + か + Phrase + かは + Noun + によって違う
/// 3. Phrase + かは + Noun + によって違う
/// 4. Variants: による (without 違う), によって違います (polite)
pub fn u301c_kaha_u301c_niyottechigau() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match か (助詞/副助詞／並立助詞／終助詞)
    #[derive(Debug)]
    struct KaParticleMatcher;
    impl Matcher for KaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "か"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞／並立助詞／終助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match は (助詞/係助詞)
    #[derive(Debug)]
    struct WaParticleMatcher;
    impl Matcher for WaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match によって or による (助詞/格助詞/連語)
    #[derive(Debug)]
    struct NiyotteOrNiyoruMatcher;
    impl Matcher for NiyotteOrNiyoruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "によって" || token.surface == "による")
                && (token.base_form == "によって" || token.base_form == "による")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match 違う verb (base form or polite form)
    #[derive(Debug)]
    struct ChigauMatcher;
    impl Matcher for ChigauMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.base_form == "違う"
                && token.pos.first().is_some_and(|pos| pos == "動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        // Pattern: か + は + (1-10 tokens) + によって/による + (optional 違う)
        // This matches the "かは" part after either "かどうか" or "A か B か" constructions
        // The full grammatical pattern is detected, even though we start matching from the second か
        TokenMatcher::Custom(Arc::new(KaParticleMatcher)),  // Second か (after どう or phrase)
        TokenMatcher::Custom(Arc::new(WaParticleMatcher)),  // は particle
        // Wildcard to match Noun (and possible modifiers like "の進み具合")
        wildcard(1, 10, vec![]),
        // Match either によって or による
        TokenMatcher::Custom(Arc::new(NiyotteOrNiyoruMatcher)),
        // Optional 違う (may not be present in による variant)
        optional(TokenMatcher::Custom(Arc::new(ChigauMatcher))),
    ]
}

// かなり: Considerably/quite (adverb form)
// Structures: かなり + Phrase
pub fn kanari() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    #[derive(Debug)]
    struct KanariMatcher;
    impl Matcher for KanariMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "かなり"
                && token.pos.first().is_some_and(|pos| pos == "副詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![TokenMatcher::Custom(Arc::new(KanariMatcher))]
}

// Pattern: あまりに (excessively/so much)
// Structures: あまりに / あまり + の / あんまり / あまりにも
pub fn amarini() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct AmariniMatcher;
    impl Matcher for AmariniMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "あまりに" || token.surface == "あんまり")
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "助詞類接続") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    vec![TokenMatcher::Custom(Arc::new(AmariniMatcher))]
}

// Pattern: あまりの (あまり + の + Noun)
pub fn amarino_noun() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct AmariMatcher;
    impl Matcher for AmariMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "あまり"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "助詞類接続") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NoParticleMatcher;
    impl Matcher for NoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(AmariMatcher)),
        TokenMatcher::Custom(Arc::new(NoParticleMatcher)),
        super::noun_matcher(),
    ]
}

// Pattern: あまりにも (あまりに + も for emphasis)
pub fn amarinimo() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct AmariniMatcher;
    impl Matcher for AmariniMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "あまりに"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "助詞類接続") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
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
        TokenMatcher::Custom(Arc::new(AmariniMatcher)),
        TokenMatcher::Custom(Arc::new(MoParticleMatcher)),
    ]
}

// Pattern: わけだ
pub fn wakeda() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match という particle (optional)
    #[derive(Debug)]
    struct ToiuMatcher;
    impl Matcher for ToiuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "という"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match わけ (dependent noun)
    #[derive(Debug)]
    struct WakeMatcher;
    impl Matcher for WakeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "わけ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match だ or です (auxiliary verb)
    #[derive(Debug)]
    struct DaDesuMatcher;
    impl Matcher for DaDesuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.base_form == "だ" || token.base_form == "です")
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        optional(TokenMatcher::Custom(Arc::new(ToiuMatcher))),
        TokenMatcher::Custom(Arc::new(WakeMatcher)),
        TokenMatcher::Custom(Arc::new(DaDesuMatcher)),
    ]
}

// Pattern: わけではない
pub fn wakedehanai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match わけ (dependent noun)
    #[derive(Debug)]
    struct WakeMatcher;
    impl Matcher for WakeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "わけ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match で or じゃ
    #[derive(Debug)]
    struct DeJyaMatcher;
    impl Matcher for DeJyaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            (token.surface == "で"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞"))

            || (token.surface == "じゃ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match は particle (optional for じゃない case)
    #[derive(Debug)]
    struct WaMatcher;
    impl Matcher for WaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match ない or あり (start of negative)
    #[derive(Debug)]
    struct NaiAriMatcher;
    impl Matcher for NaiAriMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            (token.surface == "ない"
                && (token.pos.first().is_some_and(|pos| pos == "助動詞")
                    || token.pos.first().is_some_and(|pos| pos == "形容詞")))

            || (token.surface == "あり"
                && token.base_form == "ある"
                && token.pos.first().is_some_and(|pos| pos == "動詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(WakeMatcher)),
        TokenMatcher::Custom(Arc::new(DeJyaMatcher)),
        optional(TokenMatcher::Custom(Arc::new(WaMatcher))),
        TokenMatcher::Custom(Arc::new(NaiAriMatcher)),
    ]
}

// Pattern: と同時に (at the same time as)
// Structures: Verb/Adj/Noun + と + 同時に
pub fn todoujini() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match と as connective or case particle
    #[derive(Debug)]
    struct ToMatcher;
    impl super::Matcher for ToMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
                    || token.pos.get(1).is_some_and(|pos| pos == "格助詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match 同時に as adverb
    #[derive(Debug)]
    struct DoujiniMatcher;
    impl super::Matcher for DoujiniMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "同時に"
                && token.base_form == "同時に"
                && token.pos.first().is_some_and(|pos| pos == "副詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(ToMatcher)),
        TokenMatcher::Custom(Arc::new(DoujiniMatcher)),
    ]
}

// Pattern: ところだった ① (was about to / almost happened)
// Structures: Verb[る/ない] + ところ + だった/でした
pub fn tokorodatta_u2460() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for ところ as 名詞/非自立
    #[derive(Debug)]
    struct TokoroMatcher;
    impl Matcher for TokoroMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ところ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matcher for だった (だ + た) or でした (です + た)
    // This matches the だっ/でし part
    #[derive(Debug)]
    struct DaDattaMatcher;
    impl Matcher for DaDattaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if ((token.surface == "だっ" && token.base_form == "だ")
                || (token.surface == "でし" && token.base_form == "です"))
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Pattern: Verb/Negation + ところ + だった/でした
    // Using any() to catch both Verb[基本形] and ない[基本形]
    vec![
        any(),  // Catches verb in 基本形 or ない auxiliary
        TokenMatcher::Custom(Arc::new(TokoroMatcher)),
        TokenMatcher::Custom(Arc::new(DaDattaMatcher)),
        super::past_auxiliary(),  // た
    ]
}

// Pattern: だって (because/but/even)
// Structures: Noun + だって (particle "even") OR だって + Phrase (conjunction "because/but")
pub fn datte() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct DatteMatcher;
    impl Matcher for DatteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 


            token.surface == "だって"
                && (token.pos.first().is_some_and(|pos| pos == "接続詞")
                    || (token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "副助詞"))) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // For particle form (Noun + だって), optionally match preceding noun to include it in range
    // For conjunction form (だって + Phrase), just match だって
    vec![
        optional(super::noun_matcher()),
        TokenMatcher::Custom(Arc::new(DatteMatcher)),
    ]
}

// Pattern: んだって
// Pattern: んだって (I heard that / it's thought that)
// Structures: Verb/Adj/Noun + ん + だって (or だ + って)
pub fn ndatte() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ん (explanatory particle)
    // After verbs: ん (助動詞, 不変化型)
    // After adjectives/nouns: ん (名詞/非自立/一般)
    #[derive(Debug)]
    struct NParticleMatcher;
    impl Matcher for NParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ん"
                && (token.pos.first().is_some_and(|pos| pos == "助動詞")
                    || (token.pos.first().is_some_and(|pos| pos == "名詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "非自立"))) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match だ (auxiliary verb)
    #[derive(Debug)]
    struct DaAuxMatcher;
    impl Matcher for DaAuxMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.base_form == "だ" && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match だって or って
    // だって (助詞/終助詞) after verbs
    // って (助詞/格助詞/連語) after adjectives/nouns
    #[derive(Debug)]
    struct DatteOrTteMatcher;
    impl Matcher for DatteOrTteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "だって"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "終助詞"))
                || (token.surface == "って"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Try two patterns:
    // Pattern 1 (after verbs): ん + だって (2 tokens)
    // Pattern 2 (after adjectives/nouns): ん + だ + って (3 tokens)
    // We use the longer pattern (3 tokens) with Optional(だ)
    vec![
        TokenMatcher::Custom(Arc::new(NParticleMatcher)),
        optional(TokenMatcher::Custom(Arc::new(DaAuxMatcher))),
        TokenMatcher::Custom(Arc::new(DatteOrTteMatcher)),
    ]
}

// Pattern: 関係がある (to be related to / to have a connection with)
// Structures: Noun + に/と(+の) + 関係 + が + ある/ない
pub fn kankeigaaru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match 関係 (noun)
    #[derive(Debug)]
    struct KankeiMatcher;
    impl Matcher for KankeiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "関係"
                && token.base_form == "関係"
                && token.pos.first().is_some_and(|pos| pos == "名詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match が particle
    #[derive(Debug)]
    struct GaParticleMatcher;
    impl Matcher for GaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "が"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match ある (verb) or ない (adjective)
    #[derive(Debug)]
    struct AruNaiMatcher;
    impl Matcher for AruNaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "ある"
                && token.base_form == "ある"
                && token.pos.first().is_some_and(|pos| pos == "動詞"))
                || (token.surface == "ない"
                    && token.base_form == "ない"
                    && token.pos.first().is_some_and(|pos| pos == "形容詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(KankeiMatcher)),
        TokenMatcher::Custom(Arc::new(GaParticleMatcher)),
        TokenMatcher::Custom(Arc::new(AruNaiMatcher)),
    ]
}

// Pattern: に関する・に関して
// Pattern: に関する・に関して (about / related to / regarding)
// Structures: Noun + に関（かん）して / Noun + に関（かん）する + Noun
//
// Tokenization patterns observed:
// 1. Kanji form に関する: single particle (助詞/格助詞/連語)
// 2. Hiragana にかんする: に (助詞) + かんする (動詞, base='かんする')
// 3. Hiragana にかんして: に (助詞) + かんし (名詞/サ変接続) + て (助詞/格助詞/連語)
// 4. Hiragana にかんして: に (助詞) + かん (名詞) + し (動詞 base='する') + て (助詞/接続助詞)
//
// Due to multiple tokenization patterns, we'll create separate matchers for each
pub fn nikansuru_u30fb_nikanshite() -> Vec<TokenMatcher> {
    vec![]  // Placeholder - actual matching done by specific variants below
}

// Variant 1: Single particle form (に関する / にかんする as one token)
pub fn nikansuru_particle() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct NikansuruParticleMatcher;
    impl Matcher for NikansuruParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "に関する" || token.surface == "にかんする")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "連語") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),  // Noun or other token before pattern
        TokenMatcher::Custom(Arc::new(NikansuruParticleMatcher)),
    ]
}

// Variant 2: Split form with verb (に + かんする)
pub fn nikansuru_verb() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct KansuruVerbMatcher;
    impl Matcher for KansuruVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.base_form == "かんする"
                && token.pos.first().is_some_and(|pos| pos == "動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),  // Noun before に
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(KansuruVerbMatcher)),
    ]
}

// Variant 3: Split form with noun+て (に + かんし + て)
pub fn nikanshite_noun() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct KanshiNounMatcher;
    impl Matcher for KanshiNounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "かんし"
                && token.pos.first().is_some_and(|pos| pos == "名詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct TeParticleMatcher;
    impl Matcher for TeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "連語") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),  // Noun before に
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(KanshiNounMatcher)),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
    ]
}

// Variant 4: Split form with noun+verb+て (に + かん + し + て)
pub fn nikanshite_noun_verb() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct KanNounMatcher;
    impl Matcher for KanNounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "かん"
                && token.pos.first().is_some_and(|pos| pos == "名詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct SuruVerbMatcher;
    impl Matcher for SuruVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "し"
                && token.base_form == "する"
                && token.pos.first().is_some_and(|pos| pos == "動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct TeParticleMatcher;
    impl Matcher for TeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),  // Noun before に
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(KanNounMatcher)),
        TokenMatcher::Custom(Arc::new(SuruVerbMatcher)),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
    ]
}

// Pattern: に対して (toward / in regard to / in contrast to)
// Structures: Noun + に対して / Noun + に対する + Noun / Verb/Adj/Noun + の + に対して
pub fn nitaishite() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for にたいする or にたいして (tokenized as single particle)
    #[derive(Debug)]
    struct NitaishiteMatcher;
    impl Matcher for NitaishiteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "にたいする" || token.surface == "にたいして")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "連語") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // This pattern has multiple forms:
    // 1. Noun + にたいする (に対する)
    // 2. Noun + にたいして (に対して)
    // 3. の + にたいして (contrast pattern)
    // Since Kagome tokenizes these as single particles, we need to be flexible
    // We'll match: (any token) + にたいする/にたいして
    vec![
        any(),  // Can be noun, の, etc.
        TokenMatcher::Custom(Arc::new(NitaishiteMatcher)),
    ]
}

// Pattern: くらい ② (degree/extent - so...that)
// Structures: Verb/Adjective/Noun + くらい/ぐらい
pub fn kurai_u2461() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KuraiMatcher;
    impl Matcher for KuraiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            (token.surface == "くらい" || token.surface == "ぐらい")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Matches verb/adjective/noun before くらい
        TokenMatcher::Custom(Arc::new(KuraiMatcher)),
    ]
}

// Pattern: は～くらいです (about the extent of / the only)
// Structures: は + (phrase) + くらい/ぐらい + (の/な)もの? + です/だ
//
// Tokenization:
// - くらい/ぐらい (助詞/副助詞)
// - Optional: の (助詞/連体化) + もの (名詞/非自立/一般) for emphasis
// - Optional: な (助動詞, base='だ', 体言接続) + もの for emphasis with verb/adjective
// - です/だ (助動詞)
//
// Note: This pattern detects くらい/ぐらいです (with optional のもの/なもの).
// The は particle is part of the sentence structure but not included in the match range,
// as it marks the topic that comes before the extent expression.
pub fn ha_uff5e_kuraidesu() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    use super::{Matcher, check_token};

    #[derive(Debug)]
    struct KuraiParticleMatcher;
    impl Matcher for KuraiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "くらい" || token.surface == "ぐらい")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct MonoNounMatcher;
    impl Matcher for MonoNounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "もの"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct DesuDaCopulaMatcher;
    impl Matcher for DesuDaCopulaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "です" || token.surface == "だ")
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NoOrNaMatcher;
    impl Matcher for NoOrNaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                if token.surface == "の" {
                token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "連体化")
            } else if token.surface == "な" {
                token.base_form == "だ" && token.pos.first().is_some_and(|pos| pos == "助動詞")
            } else {
                false
            }
            })
        }
    }

    vec![
        any(), // Preceding token (verb, noun, adjective, etc.)
        TokenMatcher::Custom(Arc::new(KuraiParticleMatcher)),
        optional(TokenMatcher::Custom(Arc::new(
            NoOrNaMatcher,
        ))),
        optional(TokenMatcher::Custom(Arc::new(
            MonoNounMatcher,
        ))),
        TokenMatcher::Custom(Arc::new(DesuDaCopulaMatcher)),
    ]
}

// Pattern: さ - Interjection (drawing attention, inviting action)
// Structures: さあ (as 感動詞 interjection)
//
// Note: This pattern and "さ - Filler" both match さあ as 感動詞.
// The distinction is semantic/contextual rather than structural:
// - Interjection: Typically at sentence start, drawing attention ("ok then", "well")
// - Filler: Typically mid-sentence, expressing hesitation ("um", "uh")
//
// Since they're structurally identical and "often used interchangeably"
// (per grammar data), both patterns will match. Users can determine
// meaning from context.
//
// UNDETECTABLE: さー (with prolonged sound mark) tokenizes as two separate tokens
// (さ + ー), making it impossible to detect as a single interjection pattern.
pub fn sa_interjection() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for さあ as interjection (drawing attention)
    #[derive(Debug)]
    struct SaInterjectionMatcher;
    impl Matcher for SaInterjectionMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 

            token.surface == "さあ"
                && token.base_form == "さあ"
                && token.pos.first().is_some_and(|pos| pos == "感動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![TokenMatcher::Custom(Arc::new(SaInterjectionMatcher))]
}

// Pattern: さ - Filler (hesitation/thinking filler word)
// Structures: さあ/さー (as 感動詞 interjection)
pub fn sa_filler() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for さあ/さー as interjection (filler word)
    #[derive(Debug)]
    struct SaFillerMatcher;
    impl Matcher for SaFillerMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            (token.surface == "さあ" || token.surface == "さー")
                && token.base_form == "さあ"
                && token.pos.first().is_some_and(|pos| pos == "感動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![TokenMatcher::Custom(Arc::new(SaFillerMatcher))]
}

// さ - Casual よ: Sentence-ending particle (drawing attention with confidence)
// Structures: Phrase + さ
pub fn sa_casual_yo() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for さ as sentence-ending particle
    #[derive(Debug)]
    struct SaCasualYoMatcher;
    impl Matcher for SaCasualYoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            token.surface == "さ"
                && token.base_form == "さ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "終助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Phrase/content word before さ
        TokenMatcher::Custom(Arc::new(SaCasualYoMatcher)),
    ]
}

// Pattern: それぞれ (each/respectively)
// Structures: それぞれ + Phrase, それぞれ + の + Noun
pub fn sorezore() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matches それぞれ as 名詞/副詞可能
    #[derive(Debug)]
    struct SoreZoreMatcher;
    impl super::Matcher for SoreZoreMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "それぞれ"
                && token.base_form == "それぞれ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副詞可能") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matches の as 助詞/連体化 (nominalizing particle)
    #[derive(Debug)]
    struct NoRentaikaMatcher;
    impl super::Matcher for NoRentaikaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(SoreZoreMatcher)),
        optional(TokenMatcher::Custom(Arc::new(
            NoRentaikaMatcher,
        ))),
    ]
}

// Pattern: そこで (accordingly/as such)
// Structures: (Situation) Phrase。そこで + (Solution) Phrase
pub fn sokode() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct SokodeMatcher;
    impl super::Matcher for SokodeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            token.surface == "そこで"
                && token.base_form == "そこで"
                && token.pos.first().is_some_and(|pos| pos == "接続詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    vec![TokenMatcher::Custom(Arc::new(SokodeMatcher))]
}

// Pattern: しかない (no choice but to / there is only)
// Structures: Verb + しかない
pub fn shikanai() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct ShikaParticleMatcher;
    impl super::Matcher for ShikaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            token.surface == "しか"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NaiAdjMatcher;
    impl super::Matcher for NaiAdjMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "形容詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        // Verb (基本形)
        verb_form("基本形"),
        // しか (係助詞)
        TokenMatcher::Custom(Arc::new(ShikaParticleMatcher)),
        // ない (形容詞)
        TokenMatcher::Custom(Arc::new(NaiAdjMatcher)),
    ]
}

// Pattern: てもかまわない
// てもかまわない: Doesn't matter / don't mind
// Structures: Verb[ても] + かまわない, Adj[ても] + かまわない, Noun/な-Adj + でも + かまわない
// Also polite forms: かまいません
pub fn temokamawanai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for ても or でも particle sequences
    // Can be either:
    //   - て (助詞/接続助詞) + も (助詞/係助詞) for verbs/i-adjectives
    //   - で (助詞/接続助詞) + も (助詞/係助詞) for verbs in negative て-form
    //   - で (助詞/格助詞) + も (助詞/係助詞) for nouns (when tokenized separately)
    //   - でも (助詞/副助詞) as single token for nouns/na-adjectives
    #[derive(Debug)]
    struct TemoOrDemoMatcher;
    impl super::Matcher for TemoOrDemoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match て/で as conjunction/case particle OR でも as single adverbial particle
            if token.surface == "でも"
                && token.base_form == "でも"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞") {
                // でも as single token (副助詞) - for nouns/na-adjectives
                return true;
            }

            // て or で as conjunction/case particle followed by も
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
                    || token.pos.get(1).is_some_and(|pos| pos == "格助詞"))
            })
        }
    }

    // Matcher for も particle (only needed when て/で are separate tokens)
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl super::Matcher for MoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matcher for かまう verb (かまわ, かまい forms)
    #[derive(Debug)]
    struct KamauVerbMatcher;
    impl super::Matcher for KamauVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.base_form == "かまう"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && (token.surface == "かまわ" || token.surface == "かまい") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matcher for negative forms: ない, ませ, ん
    #[derive(Debug)]
    struct NegativeFormMatcher;
    impl super::Matcher for NegativeFormMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "助動詞")
                && (token.base_form == "ない" || token.base_form == "ます" || token.base_form == "ん") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Content word before ても/でも
        TokenMatcher::Custom(Arc::new(TemoOrDemoMatcher)), // て/で or でも
        optional(TokenMatcher::Custom(Arc::new(MoParticleMatcher))), // も (optional - not present for でも single token)
        TokenMatcher::Custom(Arc::new(KamauVerbMatcher)), // かまう verb
        TokenMatcher::Custom(Arc::new(NegativeFormMatcher)), // ない or ます
    ]
}

// Pattern: ～ても～なくても
pub fn uff5e_temo_uff5e_nakutemo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: んじゃない (prohibition - don't do)
// Structures: Verb + ん + じゃない/ありません/なかった
//             Verb + て + ん + じゃない (てん contraction)
pub fn njanai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match verb (dictionary form or て-form)
    #[derive(Debug)]
    struct VerbOrTeMatcher;
    impl Matcher for VerbOrTeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            (token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "基本形"))
                || (token.surface == "て" && token.pos.first().is_some_and(|pos| pos == "助詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match ん (explanatory particle)
    #[derive(Debug)]
    struct NParticleMatcher;
    impl Matcher for NParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ん"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match じゃ (particle)
    #[derive(Debug)]
    struct JyaMatcher;
    impl Matcher for JyaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "じゃ" && token.pos.first().is_some_and(|pos| pos == "助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match ない/あり/なかっ (negative forms)
    #[derive(Debug)]
    struct NegativeFormMatcher;
    impl Matcher for NegativeFormMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            (token.surface == "ない" && token.pos.first().is_some_and(|pos| pos == "助動詞"))

                || (token.surface == "あり"
                    && token.base_form == "ある"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞"))

                || (token.surface == "なかっ"
                    && token.base_form == "ない"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbOrTeMatcher)),
        TokenMatcher::Custom(Arc::new(NParticleMatcher)),
        TokenMatcher::Custom(Arc::new(JyaMatcher)),
        TokenMatcher::Custom(Arc::new(NegativeFormMatcher)),
    ]
}

// Pattern: わけがない (there's no way that / it's impossible that)
// Structures: わけ + が + ない/ありません
pub fn wakeganai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match わけ (dependent noun)
    #[derive(Debug)]
    struct WakeMatcher;
    impl Matcher for WakeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "わけ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match が particle
    #[derive(Debug)]
    struct GaMatcher;
    impl Matcher for GaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "が"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match ない or あり (start of negative)
    #[derive(Debug)]
    struct NaiAriMatcher;
    impl Matcher for NaiAriMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            (token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "形容詞"))

            || (token.surface == "あり"
                && token.base_form == "ある"
                && token.pos.first().is_some_and(|pos| pos == "動詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(WakeMatcher)),
        TokenMatcher::Custom(Arc::new(GaMatcher)),
        TokenMatcher::Custom(Arc::new(NaiAriMatcher)),
    ]
}

// Pattern: としたら・とすれば・とすると (assuming that / if it were the case that)
// Structures: と + する(various forms) + conditional ending
// Matches all three variants: としたら, とすれば, とすると
pub fn toshitara_u30fb_tosureba_u30fb_tosuruto() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match と particle (quotation/citation)
    #[derive(Debug)]
    struct ToParticleMatcher;
    impl Matcher for ToParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match する in any form (連用形 for し, 仮定形 for すれ, 基本形 for する)
    #[derive(Debug)]
    struct SuruMatcher;
    impl Matcher for SuruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.base_form == "する"
                && token.pos.first().is_some_and(|pos| pos == "動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match conditional ending: たら OR ば OR と(接続助詞)
    #[derive(Debug)]
    struct ConditionalEndingMatcher;
    impl Matcher for ConditionalEndingMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // たら (for としたら)
            if token.surface == "たら"
                && token.base_form == "た"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
            {
                return true;
            }

            // ば (for とすれば)
            if token.surface == "ば"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
            {
                return true;
            }

            // と as 接続助詞 (for とすると)
            if token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
            {
                return true;
            }

            false
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(ToParticleMatcher)),
        TokenMatcher::Custom(Arc::new(SuruMatcher)),
        TokenMatcher::Custom(Arc::new(ConditionalEndingMatcher)),
    ]
}

// Pattern: として (as / in the capacity of)
// Structures: Noun + として
pub fn toshite() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct ToshiteMatcher;
    impl Matcher for ToshiteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "として"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    vec![TokenMatcher::Custom(Arc::new(ToshiteMatcher))]
}

// Pattern: にしては (considering / for)
// Structures: Verb/Noun + にしては
pub fn nishiteha() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct ShiVerbMatcher;
    impl Matcher for ShiVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "し"
                && token.base_form == "する"
                && token.pos.first().is_some_and(|pos| pos == "動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct TeParticleMatcher;
    impl Matcher for TeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct HaParticleMatcher;
    impl Matcher for HaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(ShiVerbMatcher)),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(HaParticleMatcher)),
    ]
}

// Pattern: にしても
// Pattern: にしても (even if / even though / even considering)
// Structures: Verb/Adjective/Noun + にしても
pub fn nishitemo() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match に particle (格助詞)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match し from する verb (連用形)
    #[derive(Debug)]
    struct ShiVerbMatcher;
    impl Matcher for ShiVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "し"
                && token.base_form == "する"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match て particle (接続助詞)
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl Matcher for TeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match も particle (係助詞)
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
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
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(ShiVerbMatcher)),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(MoParticleMatcher)),
    ]
}

// Pattern: ～というのは事実だ
// Pattern: ～というのは事実だ (it is a fact that)
// Structures: Phrase + (という) + のは事実だ/です
// Meaning: "it is a fact that / it is true that" - strongly expresses something is true/factual
// Usage: Declares a statement as undeniable truth
// Examples:
//   - 殺したというのは事実だ (It is a fact that [someone] killed)
//   - 無視したのは事実だ (It is true that [I] ignored)
//   - 仲直りしたのは事実です (It is a fact that [we] made up - polite)
// Note: という is optional, making it sound slightly weaker when omitted
pub fn uff5e_toiunohajijitsuda() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct ToiuMatcher;
    impl Matcher for ToiuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "という"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "連語") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NoMatcher;
    impl Matcher for NoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct WaParticleMatcher;
    impl Matcher for WaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct JijitsuMatcher;
    impl Matcher for JijitsuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "事実"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副詞可能") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct DaDesuMatcher;
    impl Matcher for DaDesuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "だ" || token.surface == "です")
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        optional(TokenMatcher::Custom(Arc::new(ToiuMatcher))),
        TokenMatcher::Custom(Arc::new(NoMatcher)),
        TokenMatcher::Custom(Arc::new(WaParticleMatcher)),
        TokenMatcher::Custom(Arc::new(JijitsuMatcher)),
        TokenMatcher::Custom(Arc::new(DaDesuMatcher)),
    ]
}

// Pattern: から言うと (speaking from, from the viewpoint of)
// Structures: Noun + から + 言う + と/ば/て
pub fn karaiuto() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KaraParticleMatcher;
    impl Matcher for KaraParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "から"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct IuVerbMatcher;
    impl Matcher for IuVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.base_form == "言う"
                && token.pos.first().is_some_and(|pos| pos == "動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct ConditionalParticleMatcher;
    impl Matcher for ConditionalParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "と" || token.surface == "ば" || token.surface == "て")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Noun
        TokenMatcher::Custom(Arc::new(KaraParticleMatcher)),
        TokenMatcher::Custom(Arc::new(IuVerbMatcher)),
        TokenMatcher::Custom(Arc::new(ConditionalParticleMatcher)),
    ]
}

// Pattern: に取って (for / to / concerning)
// Structures: Noun + に + 取って
pub fn nitotte() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match に particle (case-marking particle)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match 取る verb (to take)
    #[derive(Debug)]
    struct ToruVerbMatcher;
    impl Matcher for ToruVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.base_form == "取る"
                && token.pos.first().is_some_and(|pos| pos == "動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match て particle (conjunctive particle)
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl Matcher for TeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(ToruVerbMatcher)),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
    ]
}

// Pattern: ことから (from the fact that)
// Structures: Verb/Adjective/Noun + ことから
// Used to draw logical conclusions from facts
pub fn kotokara() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KotoMatcher;
    impl Matcher for KotoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct KaraParticleMatcher;
    impl Matcher for KaraParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "から"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
        TokenMatcher::Custom(Arc::new(KaraParticleMatcher)),
    ]
}

// Pattern: というより (rather than saying, more like)
// Structures: Verb/Adjective/Noun + (だ) + というより
pub fn toiuyori() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct ToQuotationMatcher;
    impl Matcher for ToQuotationMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "引用") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct IuMatcher;
    impl Matcher for IuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.base_form == "いう"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "基本形") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct YoriMatcher;
    impl Matcher for YoriMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "より"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct DaCopulaMatcher;
    impl Matcher for DaCopulaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "基本形") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),
        optional(TokenMatcher::Custom(Arc::new(DaCopulaMatcher))),
        TokenMatcher::Custom(Arc::new(ToQuotationMatcher)),
        TokenMatcher::Custom(Arc::new(IuMatcher)),
        TokenMatcher::Custom(Arc::new(YoriMatcher)),
    ]
}

// Pattern: はもちろん (not only...but also)
// Structures: Noun (A) + はもちろん + Noun (B) + も/さえ
pub fn hamochiron() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct HaParticleMatcher;
    impl Matcher for HaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct MochironAdverbMatcher;
    impl Matcher for MochironAdverbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "もちろん"
                && token.base_form == "もちろん"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(HaParticleMatcher)),
        TokenMatcher::Custom(Arc::new(MochironAdverbMatcher)),
    ]
}

// Pattern: をはじめ (not only / starting with)
// Structures: Noun + をはじめ(として) / Noun + をはじめとする + Noun
pub fn wohajime() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match はじめ as noun
    #[derive(Debug)]
    struct HajimeMatcher;
    impl Matcher for HajimeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "はじめ"
                && token.pos.first().is_some_and(|pos| pos == "名詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match として particle
    #[derive(Debug)]
    struct ToshiteMatcher;
    impl Matcher for ToshiteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "として"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match とする sequence
    #[derive(Debug)]
    struct TosuruMatcher;
    impl Matcher for TosuruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        super::noun_matcher(),
        surface("を"),
        TokenMatcher::Custom(Arc::new(HajimeMatcher)),
        optional(TokenMatcher::Custom(Arc::new(ToshiteMatcher))),
        optional(TokenMatcher::Custom(Arc::new(TosuruMatcher))),
        optional(verb_base("する")),
    ]
}

// Pattern: て初めて
pub fn tehajimete() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// さえ: even
// Structures:
//   - Noun + (Particle) + さえ
//   - Verb[stem] + さえ
//   - Verb[て] + さえ
//   - Verb + こと/の + さえ
pub fn sae() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match さえ as 助詞/係助詞
    #[derive(Debug)]
    struct SaeParticleMatcher;
    impl super::Matcher for SaeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "さえ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match any case particle (で, に, を, etc.)
    #[derive(Debug)]
    struct CaseParticleMatcher;
    impl super::Matcher for CaseParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Content word (noun, verb, adjective, etc.)
        optional(TokenMatcher::Custom(Arc::new(
            CaseParticleMatcher,
        ))),
        TokenMatcher::Custom(Arc::new(SaeParticleMatcher)),
    ]
}

// Pattern: さえ〜ば (if only / as long as)
// Structures:
//   Verb[stem] + さえ + すれば
//   Verb[て] + さえ + いれば
//   Noun + さえ + Verb[ば]
//   い-Adjective[く] + さえ + あれば
//   Noun + さえ + い-Adjective[ば]
//   な-Adjective + (で) + さえ + あれば
pub fn sae_u301c_ba() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match さえ as 助詞/係助詞 (reuse from sae() pattern)
    #[derive(Debug)]
    struct SaeParticleMatcher;
    impl super::Matcher for SaeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "さえ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match ば as 助詞/接続助詞
    #[derive(Debug)]
    struct BaParticleMatcher;
    impl super::Matcher for BaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ば"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Content word before さえ (verb, noun, adjective, etc.)
        TokenMatcher::Custom(Arc::new(SaeParticleMatcher)),
        wildcard(0, 6, vec![]), // 0-6 tokens between さえ and ば (e.g., すれ, いれ, あれ, verb in 仮定形, etc.)
        TokenMatcher::Custom(Arc::new(BaParticleMatcher)),
    ]
}

// Pattern: たものだ
// Pattern: たものだ (used to / would often)
// Structures: Verb[た] + ものだ, Verb[た] + ものです
pub fn tamonoda() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct MonoBoundNounMatcher;
    impl Matcher for MonoBoundNounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "もの"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct DaDesuMatcher;
    impl Matcher for DaDesuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "だ" || token.surface == "です")
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        super::flexible_verb_form(),
        super::past_auxiliary(),
        TokenMatcher::Custom(Arc::new(MonoBoundNounMatcher)),
        TokenMatcher::Custom(Arc::new(DaDesuMatcher)),
    ]
}

// Pattern: さて
// Pattern: さて (well then / now)
// Structures: さて + (New Topic) Phrase
// Meaning: "well" or "well then" - topic change conjunction
// Usage: Used at sentence beginning to change topic or move to new point
// Examples:
//   - さて、そろそろ出ますか (Well then, shall we head off?)
//   - さて、とりあえず乾杯しましょう (Well, let's toast first)
//   - さて、この問題の答えが分かる人はいますか (Now, does anyone know the answer?)
pub fn sate() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct SateMatcher;
    impl Matcher for SateMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "さて"
                && token.pos.first().is_some_and(|pos| pos == "接続詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![TokenMatcher::Custom(Arc::new(SateMatcher))]
}

// Pattern: むしろ (rather/instead)
// Structures: むしろ + (Preferred Choice) Phrase
pub fn mushiro() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct MushiroMatcher;
    impl Matcher for MushiroMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "むしろ"
                && token.base_form == "むしろ"
                && token.pos.first().is_some_and(|pos| pos == "副詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    vec![TokenMatcher::Custom(Arc::new(MushiroMatcher))]
}

// Pattern: つまり (in other words/in short)
// Structures: Phrase (A)。つまり + (Summary) Phrase (B)
pub fn tsumari() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct TsumariMatcher;
    impl Matcher for TsumariMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            token.surface == "つまり"
                && token.base_form == "つまり"
                && (token.pos.first().is_some_and(|pos| pos == "名詞")
                    || token.pos.first().is_some_and(|pos| pos == "接続詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![TokenMatcher::Custom(Arc::new(TsumariMatcher))]
}

// Pattern: 即ち
// Pattern: In other words (すなわち、義理の母です)
// Structure: すなわち
pub fn sunawachi() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct SunawachiMatcher;
    impl Matcher for SunawachiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "すなわち"
                && token.pos.first().is_some_and(|pos| pos == "接続詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    vec![TokenMatcher::Custom(Arc::new(SunawachiMatcher))]
}

// Pattern: Rather, on the contrary (かえって邪魔になってる)
// Structure: かえって
pub fn kaette() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KaetteMatcher;
    impl Matcher for KaetteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "かえって"
                && token.pos.first().is_some_and(|pos| pos == "副詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    vec![TokenMatcher::Custom(Arc::new(KaetteMatcher))]
}

// Pattern: まるで…ようだ (it is as if / it is as though)
// Structures: まるで + description + ようだ/みたいだ
pub fn marude_u2026_youda() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match まるで (adverb)
    #[derive(Debug)]
    struct MarudeMatcher;
    impl Matcher for MarudeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "まるで"
                && token.pos.first().is_some_and(|pos| pos == "副詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match よう or みたい (conjecture/similarity markers)
    #[derive(Debug)]
    struct YouMitaiMatcher;
    impl Matcher for YouMitaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "よう"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立"))
            || (token.surface == "みたい"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match だ or です (copula)
    #[derive(Debug)]
    struct DaDesuMatcher;
    impl Matcher for DaDesuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "だ" || token.surface == "です")
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(MarudeMatcher)),
        wildcard(1, 20, vec![]),
        TokenMatcher::Custom(Arc::new(YouMitaiMatcher)),
        TokenMatcher::Custom(Arc::new(DaDesuMatcher)),
    ]
}

// Pattern: ような気がする (have a feeling that / kinda feel like)
// Structures: (Verb/Adj/Noun) + (ような) + 気がする/気がします
pub fn younakigasuru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match よう (auxiliary verb stem)
    #[derive(Debug)]
    struct YouMatcher;
    impl Matcher for YouMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "よう"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                && token.pos.get(2).is_some_and(|pos| pos == "助動詞語幹") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match な (auxiliary verb, base=だ)
    #[derive(Debug)]
    struct NaMatcher;
    impl Matcher for NaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "な"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match 気 (dependent noun)
    #[derive(Debug)]
    struct KiMatcher;
    impl Matcher for KiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "気"
                && token.base_form == "気"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match が (particle)
    #[derive(Debug)]
    struct GaMatcher;
    impl Matcher for GaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "が"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Pattern: [content] + よう + な + 気 + が + する
    // The wildcard should stop before よう or 気
    // Note: ような is required - "気がする" alone is matched by the separate "がする" pattern
    vec![
        wildcard(1, 20, vec![]),
        TokenMatcher::Custom(Arc::new(YouMatcher)),
        TokenMatcher::Custom(Arc::new(NaMatcher)),
        TokenMatcher::Custom(Arc::new(KiMatcher)),
        TokenMatcher::Custom(Arc::new(GaMatcher)),
        verb_base("する"),
    ]
}

// Pattern: とても～ない (not at all)
// Structures: とても + ... + ない (with at least one verb somewhere)
pub fn totemo_uff5e_nai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match とても (adverb)
    #[derive(Debug)]
    struct TotemoMatcher;
    impl Matcher for TotemoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "とても"
                && token.pos.first().is_some_and(|pos| pos == "副詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match ない (negative auxiliary)
    #[derive(Debug)]
    struct NaiAuxiliaryMatcher;
    impl Matcher for NaiAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "ない" => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(TotemoMatcher)),
        wildcard(1, 8, vec![]),
        TokenMatcher::Custom(Arc::new(NaiAuxiliaryMatcher)),
    ]
}

// Pattern: Not particularly (べつに構わない)
// Structures: 別に + ... + ない/ではない/じゃない
pub fn betsuni_u301c_nai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct BetsuniMatcher;
    impl Matcher for BetsuniMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "べつに"
                && token.pos.first().is_some_and(|pos| pos == "副詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NaiFormMatcher;
    impl Matcher for NaiFormMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            (token.surface == "ない" || token.base_form == "ない")
                && (token.pos.first().is_some_and(|pos| pos == "助動詞")
                    || token.pos.first().is_some_and(|pos| pos == "形容詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(BetsuniMatcher)),
        wildcard(0, 10, vec![]),
        TokenMatcher::Custom(Arc::new(NaiFormMatcher)),
    ]
}

// Pattern: ばかりでなく (not only...but also)
// Structures: Any + ばかり + (で/じゃ + は? + なく + て? | か)
pub fn bakaridenaku() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ばかり (助詞/副助詞)
    #[derive(Debug)]
    struct BakariParticleMatcher;
    impl super::Matcher for BakariParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ばかり"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match で (助動詞, base: だ) or じゃ (助詞/副助詞)
    #[derive(Debug)]
    struct DeJaMatcher;
    impl super::Matcher for DeJaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                if token.surface == "で" {
                // で as 助動詞 (base: だ)
                token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.base_form == "だ"
            } else if token.surface == "じゃ" {
                // じゃ as 助詞/副助詞
                token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "副助詞")
            } else {
                false
            }
            })
        }
    }

    // Match は as 助詞/係助詞 (optional)
    #[derive(Debug)]
    struct WaParticleMatcher;
    impl super::Matcher for WaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match なく (助動詞, base: ない)
    #[derive(Debug)]
    struct NakuMatcher;
    impl super::Matcher for NakuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "なく"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match て as 助詞/接続助詞 (optional)
    #[derive(Debug)]
    struct TeConjunctionMatcher;
    impl super::Matcher for TeConjunctionMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match the full form: Any + ばかり + で/じゃ + は? + なく + て?
    vec![
        any(),
        TokenMatcher::Custom(Arc::new(BakariParticleMatcher)),
        TokenMatcher::Custom(Arc::new(DeJaMatcher)),
        optional(TokenMatcher::Custom(Arc::new(
            WaParticleMatcher,
        ))),
        TokenMatcher::Custom(Arc::new(NakuMatcher)),
        optional(TokenMatcher::Custom(Arc::new(
            TeConjunctionMatcher,
        ))),
    ]
}

// Pattern: ばかりか (alternative concise form of ばかりでなく)
// Structure: Any + ばかり + か
pub fn bakarika() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ばかり (助詞/副助詞)
    #[derive(Debug)]
    struct BakariParticleMatcher;
    impl super::Matcher for BakariParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ばかり"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match か as 助詞
    #[derive(Debug)]
    struct KaParticleMatcher;
    impl super::Matcher for KaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "か" && token.pos.first().is_some_and(|pos| pos == "助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(BakariParticleMatcher)),
        TokenMatcher::Custom(Arc::new(KaParticleMatcher)),
    ]
}

// Pattern: ではなくて・じゃなくて
// Pattern: ではなくて・じゃなくて (negative copula te-form)
// Structures: (の) + で/じゃ + (は) + なく + (て)
pub fn dehanakute_u30fb_janakute() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match の as nominalizer (optional)
    #[derive(Debug)]
    struct NoNominalizerMatcher;
    impl super::Matcher for NoNominalizerMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match で (助動詞, base: だ) or で (助詞/格助詞) or じゃ (助詞/副助詞)
    #[derive(Debug)]
    struct DeJaMatcher;
    impl super::Matcher for DeJaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                if token.surface == "で" {
                // Can be 助動詞 (base: だ) or 助詞/格助詞
                (token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.base_form == "だ")
                    || (token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞"))
            } else if token.surface == "じゃ" {
                // じゃ as 助詞/副助詞
                token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "副助詞")
            } else {
                false
            }
            })
        }
    }

    // Match は as 助詞/係助詞 (optional)
    #[derive(Debug)]
    struct WaParticleMatcher;
    impl super::Matcher for WaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match なく (形容詞 or 助動詞, base: ない)
    #[derive(Debug)]
    struct NakuMatcher;
    impl super::Matcher for NakuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "なく"
                && token.base_form == "ない"
                && (token.pos.first().is_some_and(|pos| pos == "形容詞")
                    || token.pos.first().is_some_and(|pos| pos == "助動詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match て as 助詞/接続助詞 (optional)
    #[derive(Debug)]
    struct TeConjunctionMatcher;
    impl super::Matcher for TeConjunctionMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        optional(TokenMatcher::Custom(Arc::new(
            NoNominalizerMatcher,
        ))),
        TokenMatcher::Custom(Arc::new(DeJaMatcher)),
        optional(TokenMatcher::Custom(Arc::new(
            WaParticleMatcher,
        ))),
        TokenMatcher::Custom(Arc::new(NakuMatcher)),
        optional(TokenMatcher::Custom(Arc::new(
            TeConjunctionMatcher,
        ))),
    ]
}

// Pattern: だけでなく(て)～も
// Pattern: だけでなく(て)～も (not only... but also with も emphasis)
// Structures: Noun + だけでなく(て) + ... + Noun + も
pub fn dakedenaku_te_uff5e_mo() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Reuse N4 だけでなく matcher components
    #[derive(Debug)]
    struct DakeMatcher;
    impl super::Matcher for DakeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "だけ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct DeJaMatcher;
    impl super::Matcher for DeJaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "で"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞"))
            || (token.surface == "じゃ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct WaMatcher;
    impl super::Matcher for WaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NakuMatcher;
    impl super::Matcher for NakuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "なく"
                && token.base_form == "ない"
                && (token.pos.first().is_some_and(|pos| pos == "助動詞")
                    || token.pos.first().is_some_and(|pos| pos == "形容詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct TeMatcher;
    impl super::Matcher for TeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matcher for も particle (係助詞)
    #[derive(Debug)]
    struct MoMatcher;
    impl super::Matcher for MoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Pattern: Noun + だけ + でなく(て)/じゃなく(て) + (punctuation) + ... + も
    // Note: Wildcard stops at punctuation automatically, so we need to handle it explicitly

    // Matcher for punctuation (optional comma/period after だけでなく)
    #[derive(Debug)]
    struct PunctuationMatcher;
    impl super::Matcher for PunctuationMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "記号") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        // Noun (subject A)
        super::noun_matcher(),
        // だけ
        TokenMatcher::Custom(Arc::new(DakeMatcher)),
        // で or じゃ
        TokenMatcher::Custom(Arc::new(DeJaMatcher)),
        // は (optional)
        optional(TokenMatcher::Custom(Arc::new(WaMatcher))),
        // なく
        TokenMatcher::Custom(Arc::new(NakuMatcher)),
        // て (optional)
        optional(TokenMatcher::Custom(Arc::new(TeMatcher))),
        // Optional punctuation (、)
        optional(TokenMatcher::Custom(Arc::new(PunctuationMatcher))),
        // Wildcard for intermediate content until も (0-10 tokens)
        // This allows for nouns, particles between punctuation and も
        // Note: Wildcard with min=0 allows for cases like "だけでなく台風も" (no intermediate tokens)
        wildcard(0, 10, vec![]),
        // も (終点)
        TokenMatcher::Custom(Arc::new(MoMatcher)),
    ]
}

// Pattern: だけしか (only/nothing but)
// Structures: Noun + だけ + しか + ない
pub fn dakeshika() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match だけ as adverbial particle
    #[derive(Debug)]
    struct DakeParticleMatcher;
    impl super::Matcher for DakeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "だけ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match しか as bound particle
    #[derive(Debug)]
    struct ShikaParticleMatcher;
    impl super::Matcher for ShikaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "しか"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match negative forms: ない (助動詞 or 形容詞), ません, ん
    #[derive(Debug)]
    struct NegativeFormMatcher;
    impl super::Matcher for NegativeFormMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // ない can be 助動詞 (auxiliary) or 形容詞 (i-adjective)
            if token.base_form == "ない" {
                token.pos.first().is_some_and(|pos| pos == "助動詞" || pos == "形容詞")
            } else {
                // ます or ん are always 助動詞
                token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && (token.base_form == "ます" || token.base_form == "ん")
            }
            })
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(DakeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(ShikaParticleMatcher)),
        wildcard(0, 10, vec![]),
        TokenMatcher::Custom(Arc::new(NegativeFormMatcher)),
    ]
}

// Pattern: は言うまでもない ① (it goes without saying)
// Handles BOTH tokenization forms:
// 1. Single-token kanji form: 言うまでもない (形容詞/自立)
// 2. Split form: いう/言う (verb) + まで + も + ない/ある + polite endings
//
// This pattern expresses "it goes without saying that (A)" or "needless to say (A)".
// Structures tested:
// - Any + は/も + 言うまでもない (kanji single token)
// - Any + は/も + いうまでもない (hiragana split: いう + まで + も + ない)
// - Any + は/も + 言うまでもありません (polite: 言う + まで + も + あり + ませ + ん)

// Matcher for kanji single-token form: 言うまでもない
pub fn haiumademonai_u2460_single() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct HaMoParticleMatcher;
    impl Matcher for HaMoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "は" || token.surface == "も")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct IumademoinaiAdjectiveMatcher;
    impl Matcher for IumademoinaiAdjectiveMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "言うまでもない"
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(HaMoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(IumademoinaiAdjectiveMatcher)),
    ]
}

// Matcher for split form: いう/言う + まで + も + ない
pub fn haiumademonai_u2460_split() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct HaMoParticleMatcher;
    impl Matcher for HaMoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "は" || token.surface == "も")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct IuVerbMatcher;
    impl Matcher for IuVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "いう" || token.surface == "言う")
                && (token.base_form == "いう" || token.base_form == "言う")
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct MadeParticleMatcher;
    impl Matcher for MadeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "まで"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NaiAdjectiveMatcher;
    impl Matcher for NaiAdjectiveMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ない"
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(HaMoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(IuVerbMatcher)),
        TokenMatcher::Custom(Arc::new(MadeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(MoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(NaiAdjectiveMatcher)),
    ]
}

// Matcher for polite form: 言う/いう + まで + も + あり + ませ + ん
pub fn haiumademonai_u2460_polite() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct HaMoParticleMatcher;
    impl Matcher for HaMoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "は" || token.surface == "も")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct IuVerbMatcher;
    impl Matcher for IuVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "いう" || token.surface == "言う")
                && (token.base_form == "いう" || token.base_form == "言う")
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct MadeParticleMatcher;
    impl Matcher for MadeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "まで"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct AriVerbMatcher;
    impl Matcher for AriVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "あり"
                && token.base_form == "ある"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct MaseAuxiliaryMatcher;
    impl Matcher for MaseAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ませ"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NNegativeAuxiliaryMatcher;
    impl Matcher for NNegativeAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ん"
                && token.base_form == "ん"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(HaMoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(IuVerbMatcher)),
        TokenMatcher::Custom(Arc::new(MadeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(MoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(AriVerbMatcher)),
        TokenMatcher::Custom(Arc::new(MaseAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(NNegativeAuxiliaryMatcher)),
    ]
}

// Main pattern function (for backwards compatibility)
pub fn haiumademonai_u2460() -> Vec<TokenMatcher> {
    // Default to split form (most common)
    haiumademonai_u2460_split()
}

// Pattern: 決して〜ない (never / under no circumstances / by no means)
// Structures: 決して + Wildcard{0-20} + ない
pub fn kesshite_u301c_nai() -> Vec<TokenMatcher> {
    use super::Matcher;
    use std::sync::Arc;

    #[derive(Debug)]
    struct KesshiteMatcher;
    impl Matcher for KesshiteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "決して"
                && token.pos.first().is_some_and(|p| p == "副詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NaiMatcher;
    impl Matcher for NaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.base_form == "ない"
                && (token.pos.first().is_some_and(|p| p == "助動詞")
                    || token.pos.first().is_some_and(|p| p == "形容詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(KesshiteMatcher)),
        wildcard(0, 20, vec![]),
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
    ]
}

// Pattern: わけにはいかない (cannot afford to / impossible to / it cannot be so that)
// Structures: わけ + に + は + いか + ない/ません
pub fn wakenihaikanai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match わけ (dependent noun)
    #[derive(Debug)]
    struct WakeMatcher;
    impl Matcher for WakeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "わけ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match に particle (case marker)
    #[derive(Debug)]
    struct NiMatcher;
    impl Matcher for NiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match は particle (topic/contrast)
    #[derive(Debug)]
    struct WaMatcher;
    impl Matcher for WaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match いか or いき (from いく verb)
    #[derive(Debug)]
    struct IkaMatcher;
    impl Matcher for IkaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "いか" || token.surface == "いき")
                && token.base_form == "いく"
                && token.pos.first().is_some_and(|pos| pos == "動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match ない or ませ (negative ending)
    #[derive(Debug)]
    struct NaiMaseMatcher;
    impl Matcher for NaiMaseMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            (token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞"))

            || (token.surface == "ませ"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(WakeMatcher)),
        TokenMatcher::Custom(Arc::new(NiMatcher)),
        TokenMatcher::Custom(Arc::new(WaMatcher)),
        TokenMatcher::Custom(Arc::new(IkaMatcher)),
        TokenMatcher::Custom(Arc::new(NaiMaseMatcher)),
    ]
}

// Pattern: 〜ようとしない
pub fn u301c_youtoshinai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: もしかしたら (maybe/perhaps/possibly)
// Structures: もしかしたら, もしかして, もしかすると
//
// Kagome tokenizes these in three different ways:
// 1. もしかして (副詞/一般) - single adverb token
// 2. もしか (副詞/助詞類接続) + し (動詞, する) + たら (助動詞, た)
// 3. もしか (副詞/助詞類接続) + する (動詞) + と (助詞/接続助詞)
pub fn moshikashitara() -> Vec<TokenMatcher> {
    // Matcher for し (連用形 of する)
    #[derive(Debug)]
    struct ShiVerbMatcher;
    impl Matcher for ShiVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "し"
                && token.base_form == "する"
                && token.pos.first().is_some_and(|pos| pos == "動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matcher for たら (conditional form of た)
    #[derive(Debug)]
    struct TaraAuxiliaryMatcher;
    impl Matcher for TaraAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "たら"
                && token.base_form == "た"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matcher for する (dictionary form)
    #[derive(Debug)]
    struct SuruVerbMatcher;
    impl Matcher for SuruVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "する"
                && token.base_form == "する"
                && token.pos.first().is_some_and(|pos| pos == "動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matcher for と (conditional particle)
    #[derive(Debug)]
    struct ToConditionalMatcher;
    impl Matcher for ToConditionalMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // We need a custom matcher that can handle all three forms
    // Since we can't use alternation, we'll create a single comprehensive matcher
    #[derive(Debug)]
    struct MoshikashitaraComprehensiveMatcher;
    impl Matcher for MoshikashitaraComprehensiveMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            (token.surface == "もしかして"
                && token.base_form == "もしかして"
                && token.pos.first().is_some_and(|pos| pos == "副詞"))
                || (token.surface == "もしか"
                    && token.base_form == "もしか"
                    && token.pos.first().is_some_and(|pos| pos == "副詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // We'll match the comprehensive start, then optionally match the rest
    vec![
        TokenMatcher::Custom(Arc::new(MoshikashitaraComprehensiveMatcher)),
        // Optional: し or する
        optional(TokenMatcher::Custom(Arc::new(
            ShiVerbMatcher,
        ))),
        optional(TokenMatcher::Custom(Arc::new(
            SuruVerbMatcher,
        ))),
        // Optional: たら or と
        optional(TokenMatcher::Custom(Arc::new(
            TaraAuxiliaryMatcher,
        ))),
        optional(TokenMatcher::Custom(Arc::new(
            ToConditionalMatcher,
        ))),
    ]
}

// Pattern: たとえ〜ても
pub fn tatoe_u301c_temo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ことに
// Pattern: ことに (particularly/especially/to my...)
// Structures: Verb/Adjective/Noun + ことに
// Expresses emotional emphasis or notable circumstance
pub fn kotoni() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KotoMatcher;
    impl Matcher for KotoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
    ]
}

// Pattern: ことか (how / god knows / what)
// Structures: Verb/Adjective/Noun + ことか
// Expresses rhetorical emphasis on extent/magnitude
pub fn kotoka() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KotoMatcher;
    impl Matcher for KotoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct KaParticleMatcher;
    impl Matcher for KaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "か"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞／並立助詞／終助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
        TokenMatcher::Custom(Arc::new(KaParticleMatcher)),
    ]
}

// Pattern: ～かというと ① (the reason why / if asked why)
// Structures: か + と + いう/言う + と/ば/たら
pub fn uff5e_katoiuto_u2460() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matches か as question particle
    #[derive(Debug)]
    struct KaParticleMatcher;
    impl Matcher for KaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "か"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token
                    .pos
                    .get(1)
                    .is_some_and(|pos| pos == "副助詞／並立助詞／終助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matches と as quotation particle
    #[derive(Debug)]
    struct ToQuoteMatcher;
    impl Matcher for ToQuoteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "引用") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matches いう/言う verb (any conjugation form)
    #[derive(Debug)]
    struct IuVerbMatcher;
    impl Matcher for IuVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.base_form == "いう" || token.base_form == "言う")
                && token.pos.first().is_some_and(|pos| pos == "動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matches と/ば as conditional/connective particles OR たら as auxiliary
    #[derive(Debug)]
    struct ConditionalEndMatcher;
    impl Matcher for ConditionalEndMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Matches と or ば as 接続助詞
            if (token.surface == "と" || token.surface == "ば")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
            {
                return true;
            }
            // Matches たら as auxiliary in 仮定形
            if token.surface == "たら"
                && token.base_form == "た"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
            {
                return true;
            }
            false
            })
        }
    }

    vec![
        any(), // Preceding phrase
        TokenMatcher::Custom(Arc::new(KaParticleMatcher)),
        TokenMatcher::Custom(Arc::new(ToQuoteMatcher)),
        TokenMatcher::Custom(Arc::new(IuVerbMatcher)),
        TokenMatcher::Custom(Arc::new(ConditionalEndMatcher)),
    ]
}

// Pattern: ～かというと ② (if I were to say [question word])
// Structures: Question Word + Phrase + (の/ん) + か + と + いう + と/ば/たら
// Question words: 何, 誰, どこ, どの, いつ, どれ, どちら, 何で
pub fn uff5e_katoiuto_u2461() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Question words to match (who, what, where, which, when, how)
    const QUESTION_WORDS: &[&str] = &[
        "何", "誰", "どこ", "どの", "いつ", "どれ", "どちら", "何で", "なに", "だれ", "いくつ",
    ];

    // Matches question words
    #[derive(Debug)]
    struct QuestionWordMatcher;
    impl Matcher for QuestionWordMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if QUESTION_WORDS.contains(&token.surface.as_str())
                || QUESTION_WORDS.contains(&token.base_form.as_str()) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matches か as question particle
    #[derive(Debug)]
    struct KaParticleMatcher;
    impl Matcher for KaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "か"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token
                    .pos
                    .get(1)
                    .is_some_and(|pos| pos == "副助詞／並立助詞／終助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matches と as quotation particle
    #[derive(Debug)]
    struct ToQuoteMatcher;
    impl Matcher for ToQuoteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "引用") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matches いう/言う verb (any conjugation form)
    #[derive(Debug)]
    struct IuVerbMatcher;
    impl Matcher for IuVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.base_form == "いう" || token.base_form == "言う")
                && token.pos.first().is_some_and(|pos| pos == "動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matches と/ば as conditional/connective particles OR たら as auxiliary
    #[derive(Debug)]
    struct ConditionalEndMatcher;
    impl Matcher for ConditionalEndMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Matches と or ば as 接続助詞
            if (token.surface == "と" || token.surface == "ば")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
            {
                return true;
            }
            // Matches たら as auxiliary in 仮定形
            if token.surface == "たら"
                && token.base_form == "た"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
            {
                return true;
            }
            false
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(QuestionWordMatcher)), // Question word
        wildcard(0, 20, vec![]), // Phrase between question word and か
        TokenMatcher::Custom(Arc::new(KaParticleMatcher)),
        TokenMatcher::Custom(Arc::new(ToQuoteMatcher)),
        TokenMatcher::Custom(Arc::new(IuVerbMatcher)),
        TokenMatcher::Custom(Arc::new(ConditionalEndMatcher)),
    ]
}

// Pattern: で言うと (if said with, speaking of)
// Structures: Noun + で言うと
pub fn deiuto() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct DeMatcher;
    impl Matcher for DeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct IuMatcher;
    impl Matcher for IuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.base_form == "言う"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "基本形") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct ToMatcher;
    impl Matcher for ToMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(DeMatcher)),
        TokenMatcher::Custom(Arc::new(IuMatcher)),
        TokenMatcher::Custom(Arc::new(ToMatcher)),
    ]
}

// Pattern: ～ずつ
pub fn uff5e_zutsu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ずっと ② (by far/much more - comparative)
// NOTE: Tokenization is identical to ずっと ① (continuously).
// Both patterns use 副詞/一般. The difference is semantic context:
// - ずっと ① = temporal continuity ("continuously", "the whole time")
// - ずっと ② = comparative degree ("by far", "much more")
//
// According to the Fun Fact in grammar_points_data.json, both meanings
// derive from the same core concept of "unwavering/unfaltering" and
// "far more (A)" / "to the maximum amount possible".
//
// Since we cannot reliably distinguish these structurally, both patterns
// will be detected when ずっと appears. The user should determine the
// meaning from context.
pub fn zutto_u2461() -> Vec<TokenMatcher> {
    use super::Matcher;

    // Match ずっと adverb (副詞/一般) - identical to ずっと ①
    #[derive(Debug)]
    struct ZuttoMatcher;
    impl Matcher for ZuttoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ずっと"
                && token.base_form == "ずっと"
                && token.pos.first().is_some_and(|pos| pos == "副詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![TokenMatcher::Custom(Arc::new(ZuttoMatcher))]
}

// Pattern: だらけ (covered with/full of - scattered state)
// Structures: Noun + だらけ, Noun + だらけ + の + Noun
pub fn darake() -> Vec<TokenMatcher> {
    use super::Matcher;

    // Match だらけ suffix (名詞/接尾/一般)
    #[derive(Debug)]
    struct DarakeSuffixMatcher;
    impl Matcher for DarakeSuffixMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "だらけ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match optional の particle (連体化)
    #[derive(Debug)]
    struct NoRentaikaMatcher;
    impl Matcher for NoRentaikaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(DarakeSuffixMatcher)),
        optional(TokenMatcher::Custom(Arc::new(NoRentaikaMatcher))),
    ]
}

// Pattern: もっとも (although/however/with that said)
// Structures: もっとも + Phrase
pub fn mottomo() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct MottomoMatcher;
    impl Matcher for MottomoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "もっとも"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    vec![TokenMatcher::Custom(Arc::new(MottomoMatcher))]
}

// Pattern: 再び (again/once more/a second time)
// Structures: ふたたび + Phrase
pub fn futatabi() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct FutatabiMatcher;
    impl Matcher for FutatabiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "ふたたび" || token.surface == "再び")
                && token.pos.first().is_some_and(|pos| pos == "副詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    vec![TokenMatcher::Custom(Arc::new(FutatabiMatcher))]
}

// Pattern: み (adjective stem + み → noun suffix for "-ness")
// Structures: い-Adjective stem + み, な-Adjective stem + み
//
// Tokenization:
// - い-Adjective + み: Often a single token (e.g., 楽しみ, 甘み, 赤み, 温かみ) as 名詞/一般
// - な-Adjective + み: Two tokens - adjective stem (名詞/形容動詞語幹) + み (動詞/自立, base='みる')
//
// This matcher handles the two-token case (な-adjective stem + み).
// The single-token case (nouns ending in み) is harder to distinguish from regular nouns,
// so we focus on the detectable pattern: な-adjective stem followed by み suffix.
pub fn mi() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match な-adjective stems (名詞/形容動詞語幹)
    #[derive(Debug)]
    struct NaAdjectiveStemMatcher;
    impl super::Matcher for NaAdjectiveStemMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match み when misclassified as みる verb (appears after な-adjective stems)
    #[derive(Debug)]
    struct MiSuffixMatcher;
    impl super::Matcher for MiSuffixMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "み"
                && token.base_form == "みる"
                && token.pos.first().is_some_and(|pos| pos == "動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NaAdjectiveStemMatcher)),
        TokenMatcher::Custom(Arc::new(MiSuffixMatcher)),
    ]
}

// Pattern: と同じくらい (about the same as)
// Structures: Noun + と + 同じ + くらい/ぐらい
pub fn toonajikurai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match くらい or ぐらい (助詞/副助詞)
    #[derive(Debug)]
    struct KuraiGuraiMatcher;
    impl super::Matcher for KuraiGuraiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "くらい" || token.surface == "ぐらい")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match 同じ (連体詞)
    #[derive(Debug)]
    struct OnajiMatcher;
    impl super::Matcher for OnajiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "同じ"
                && token.pos.first().is_some_and(|pos| pos == "連体詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        super::noun_matcher(),
        surface("と"),
        TokenMatcher::Custom(Arc::new(OnajiMatcher)),
        TokenMatcher::Custom(Arc::new(KuraiGuraiMatcher)),
    ]
}

// Pattern: と同じで・と違って (same as / different from)
// Structures: Noun + と + 同じで OR Noun + と + 違って
pub fn toonajide_u30fb_tochigatte() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for と particle (case-marking particle)
    #[derive(Debug)]
    struct ToParticleMatcher;
    impl Matcher for ToParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "と"
                && token.base_form == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matcher for 同じ followed by で OR 違っ followed by て
    // This matches the second token (同じ or 違っ)
    #[derive(Debug)]
    struct OnajideOrChigatteMatcher;
    impl Matcher for OnajideOrChigatteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            (token.surface == "同じ"
                && token.base_form == "同じ"
                && token.pos.first().is_some_and(|pos| pos == "連体詞"))
            ||

            (token.base_form == "違う"
                && token.pos.first().is_some_and(|pos| pos == "動詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matcher for で (auxiliary) OR て (particle)
    // This matches the third token (で or て)
    #[derive(Debug)]
    struct DeOrTeMatcher;
    impl Matcher for DeOrTeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            (token.surface == "で"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞"))
            ||

            (token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(ToParticleMatcher)),
        TokenMatcher::Custom(Arc::new(OnajideOrChigatteMatcher)),
        TokenMatcher::Custom(Arc::new(DeOrTeMatcher)),
    ]
}

// Pattern: と並んで (alongside, comparable to)
// Structures: Noun + と + 並ぶ + (んで/ぶほど)
pub fn tonarande() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // と as case marking particle
    #[derive(Debug)]
    struct ToMatcher;
    impl Matcher for ToMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // 並ぶ verb in any conjugation form
    #[derive(Debug)]
    struct NarabuMatcher;
    impl Matcher for NarabuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.base_form == "並ぶ"
                && token.pos.first().is_some_and(|pos| pos == "動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // で (接続助詞) or ほど (副助詞)
    #[derive(Debug)]
    struct EndingMatcher;
    impl Matcher for EndingMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "で" && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞"))
            || (token.surface == "ほど" && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(ToMatcher)),
        TokenMatcher::Custom(Arc::new(NarabuMatcher)),
        TokenMatcher::Custom(Arc::new(EndingMatcher)),
    ]
}

// Pattern: に違いない (must be / there is no doubt that)
// Structures: Verb/Adjective/Noun + に違いない, Verb/Adjective/Noun + に違いありません
pub fn nichigainai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for に particle
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matcher for ちがい/違い (as noun/ナイ形容詞語幹)
    #[derive(Debug)]
    struct ChigaiMatcher;
    impl Matcher for ChigaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "ちがい" || token.surface == "違い")
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "ナイ形容詞語幹") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matcher for ない as auxiliary
    #[derive(Debug)]
    struct NaiAuxiliaryMatcher;
    impl Matcher for NaiAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Verb, Adjective, or Noun
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(ChigaiMatcher)),
        TokenMatcher::Custom(Arc::new(NaiAuxiliaryMatcher)),
    ]
}

// Pattern: 当たり (per / each)
// Structures: Number + Counter + 当（あ）たり
pub fn atari() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match あたり or 当たり as suffix noun
    #[derive(Debug)]
    struct AtariMatcher;
    impl Matcher for AtariMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "あたり" || token.surface == "当たり")
                && (token.base_form == "あたり" || token.base_form == "当たり")
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "接尾")
                    || token.pos.get(1).is_some_and(|pos| pos == "一般")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![TokenMatcher::Custom(Arc::new(AtariMatcher))]
}

// Pattern: に当たる (corresponds to / amounts to / is in regard to)
// Structures: Noun + にあたる, Noun + にあたる + Noun, Noun + にあたります
// Note: Kagome tokenizes this in two ways:
// 1. "にあたる" as single particle (助詞/格助詞/連語) when followed by noun or at end
// 2. "に" + "あたる" verb when at end or with ます
pub fn niataru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for に particle
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matcher for あたる verb
    #[derive(Debug)]
    struct AtaruVerbMatcher;
    impl Matcher for AtaruVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.base_form == "あたる"
                && token.pos.first().is_some_and(|pos| pos == "動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matcher for ます auxiliary (optional)
    #[derive(Debug)]
    struct MasuMatcher;
    impl Matcher for MasuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match: Any token + に + あたる + optional ます
    vec![
        any(),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(AtaruVerbMatcher)),
        optional(TokenMatcher::Custom(Arc::new(MasuMatcher))),
    ]
}

// Pattern: に当たる (compound particle tokenization)
// Structures: Any token + にあたる (as single particle token)
pub fn niataru_particle() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for にあたる as a single particle (助詞/格助詞/連語)
    #[derive(Debug)]
    struct NiataruParticleMatcher;
    impl Matcher for NiataruParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "にあたる"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "連語") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match: Any token + にあたる (as single particle)
    vec![
        any(),
        TokenMatcher::Custom(Arc::new(NiataruParticleMatcher)),
    ]
}

// Pattern: に限る (nothing better than / limited to)
// Structures: Verb/Noun + に + 限る/限ります
pub fn nikagiru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct KagiruVerbMatcher;
    impl Matcher for KagiruVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "かぎる" || token.surface == "かぎり" || token.surface == "限る" || token.surface == "限り")
                && token.base_form == "かぎる"
                && token.pos.first().is_some_and(|pos| pos == "動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct MasuAuxiliaryMatcher;
    impl Matcher for MasuAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ます" && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(KagiruVerbMatcher)),
        optional(TokenMatcher::Custom(Arc::new(MasuAuxiliaryMatcher))),
    ]
}

// Pattern: とは限らない (not necessarily, not always)
// Structures: Verb/い-Adj/な-Adj/Noun + (だ) + とは限らない/とは限りません
pub fn tohakagiranai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match と particle (quotation)
    #[derive(Debug)]
    struct ToQuotationMatcher;
    impl Matcher for ToQuotationMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "と"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match は topic particle
    #[derive(Debug)]
    struct WaTopicMatcher;
    impl Matcher for WaTopicMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "は"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match 限る verb
    #[derive(Debug)]
    struct KagiruMatcher;
    impl Matcher for KagiruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.base_form == "限る"
                && token.pos.first().is_some_and(|p| p == "動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),  // Verb, i-Adj, na-Adj, or Noun
        optional(surface("だ")),  // Optional だ for na-adj/noun
        TokenMatcher::Custom(Arc::new(ToQuotationMatcher)),
        TokenMatcher::Custom(Arc::new(WaTopicMatcher)),
        TokenMatcher::Custom(Arc::new(KagiruMatcher)),
        any(),  // Match ない or ませ
        optional(any()),  // Optionally match ん for polite form
    ]
}

// Pattern: めったに〜ない (rarely/seldom/hardly)
// Structures: めったに + Wildcard{0-20} + ない
//             OR
//             めった + に + Wildcard{0-20} + ない
//
// Kagome tokenizes this in two ways:
// 1. Short form: めったに (副詞/一般) - single adverb token
// 2. Long form: めった (名詞/形容動詞語幹) + に (助詞/副詞化) - two tokens
//
// Both forms are followed by a phrase ending in ない (negative)
pub fn mettani_u301c_nai() -> Vec<TokenMatcher> {
    // Matcher for に particle (adverbializer)
    #[derive(Debug)]
    struct NiAdverbializerMatcher;
    impl Matcher for NiAdverbializerMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副詞化") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matcher for ない (negative marker)
    #[derive(Debug)]
    struct NaiMatcher;
    impl Matcher for NaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.base_form == "ない"
                && (token.pos.first().is_some_and(|pos| pos == "助動詞")
                    || token.pos.first().is_some_and(|pos| pos == "形容詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // We need to match EITHER:
    // 1. めったに (adverb) + wildcard + ない
    // 2. めった (stem) + に + wildcard + ない
    //
    // Since we can't use alternation in the matcher pattern, we'll use a custom matcher
    // that checks if we start with either form, then use wildcard + ない

    #[derive(Debug)]
    struct MettaniOrMettaNiMatcher;
    impl Matcher for MettaniOrMettaNiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            (token.surface == "めったに" && token.pos.first().is_some_and(|pos| pos == "副詞"))
                || (token.surface == "めった"
                    && token.base_form == "めった"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(MettaniOrMettaNiMatcher)),
        optional(TokenMatcher::Custom(Arc::new(
            NiAdverbializerMatcher,
        ))),
        wildcard(0, 20, vec![]),
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
    ]
}

// Pattern: 割に (although/despite/comparatively)
// Structures: Verb/Adjective/Noun + わりに
pub fn warini() -> Vec<TokenMatcher> {
    // Match わり (noun)
    #[derive(Debug)]
    struct WariniNounMatcher;
    impl Matcher for WariniNounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "わり"
                && token.base_form == "わり"
                && token.pos.first().is_some_and(|pos| pos == "名詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match に particle (格助詞)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Matches the predicate (verb, adjective, noun+particle, etc.)
        TokenMatcher::Custom(Arc::new(WariniNounMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
    ]
}

// Pattern: Verb[volitional]とする (try to / be about to)
// Structures: Verb[未然ウ接続] + う + と + する/します/した/etc.
pub fn verb_volitional_tosuru() -> Vec<TokenMatcher> {
    // Match verb in volitional form (未然ウ接続)
    #[derive(Debug)]
    struct VolitionalVerbMatcher;
    impl Matcher for VolitionalVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然ウ接続") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match う as auxiliary verb (助動詞/不変化型/基本形)
    #[derive(Debug)]
    struct VolitionalAuxiliaryMatcher;
    impl Matcher for VolitionalAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "う"
                && token.base_form == "う"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match と as quotation particle
    #[derive(Debug)]
    struct ToQuotationMatcher;
    impl Matcher for ToQuotationMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VolitionalVerbMatcher)),
        TokenMatcher::Custom(Arc::new(VolitionalAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(ToQuotationMatcher)),
        verb_base("する"),
    ]
}

// Pattern: ～ようとしない (shall not / doesn't try to)
// Structures: Verb[おう] + としない / としません
pub fn youtoshinai() -> Vec<TokenMatcher> {
    // Match verb in volitional form (未然ウ接続)
    #[derive(Debug)]
    struct VolitionalVerbMatcher;
    impl Matcher for VolitionalVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然ウ接続") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match う as auxiliary verb (助動詞/不変化型/基本形)
    #[derive(Debug)]
    struct VolitionalAuxiliaryMatcher;
    impl Matcher for VolitionalAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "う"
                && token.base_form == "う"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match と as quotation particle
    #[derive(Debug)]
    struct ToQuotationMatcher;
    impl Matcher for ToQuotationMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match し from する in 未然形 or 連用形
    #[derive(Debug)]
    struct ShiMatcher;
    impl Matcher for ShiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "し"
                && token.base_form == "する"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && (token.features.get(5).is_some_and(|f| f == "未然形")
                    || token.features.get(5).is_some_and(|f| f == "連用形")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match ない (negative auxiliary) or ませ (polite negative stem)
    #[derive(Debug)]
    struct NaiMaseMatcher;
    impl Matcher for NaiMaseMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞"))
            || (token.surface == "ませ"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // For polite form, optionally match ん after ませ
    #[derive(Debug)]
    struct NMatcher;
    impl Matcher for NMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ん"
                && token.base_form == "ん"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VolitionalVerbMatcher)),
        TokenMatcher::Custom(Arc::new(VolitionalAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(ToQuotationMatcher)),
        TokenMatcher::Custom(Arc::new(ShiMatcher)),
        TokenMatcher::Custom(Arc::new(NaiMaseMatcher)),
        optional(TokenMatcher::Custom(Arc::new(NMatcher))),
    ]
}

// Pattern: ～と言っても (even though / although I say)
// Structures: Verb/Adj/Noun + (だ) + といっても
pub fn toittemo() -> Vec<TokenMatcher> {
    // Match だ as auxiliary (optional for verb/adj, required for na-adj/noun)
    #[derive(Debug)]
    struct DaAuxiliaryMatcher;
    impl Matcher for DaAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "だ"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match と as quotation particle
    #[derive(Debug)]
    struct ToQuotationMatcher;
    impl Matcher for ToQuotationMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "引用") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match いう in て-form (いっ)
    #[derive(Debug)]
    struct IuVerbTeMatcher;
    impl Matcher for IuVerbTeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "いっ"
                && token.base_form == "いう"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用タ接続") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match て as conjunction particle
    #[derive(Debug)]
    struct TeConjunctionMatcher;
    impl Matcher for TeConjunctionMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match も as binding particle
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
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
        any(), // Verb, Adjective, or Noun
        optional(TokenMatcher::Custom(Arc::new(DaAuxiliaryMatcher))),
        TokenMatcher::Custom(Arc::new(ToQuotationMatcher)),
        TokenMatcher::Custom(Arc::new(IuVerbTeMatcher)),
        TokenMatcher::Custom(Arc::new(TeConjunctionMatcher)),
        TokenMatcher::Custom(Arc::new(MoParticleMatcher)),
    ]
}

// Pattern: Verb[volitional] + としたが (was about to do X, but Y)
// Structures:
//   - Verb[おう] + としたが
//   - Verb[おう] + としたら
//   - Verb[おう] + としたけど/けれど/けれども
pub fn verb_volitional_toshitaga() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match verb in volitional form (未然ウ接続)
    #[derive(Debug)]
    struct VolitionalVerbMatcher;
    impl Matcher for VolitionalVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然ウ接続") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match う as auxiliary verb (助動詞/不変化型/基本形)
    #[derive(Debug)]
    struct VolitionalAuxiliaryMatcher;
    impl Matcher for VolitionalAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "う"
                && token.base_form == "う"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match と as quotation particle
    #[derive(Debug)]
    struct ToQuotationMatcher;
    impl Matcher for ToQuotationMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match し from する in 連用形
    #[derive(Debug)]
    struct ShiVerbMatcher;
    impl Matcher for ShiVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "し"
                && token.base_form == "する"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match た or たら (past or conditional form)
    // - た: surface='た' base='た' pos=助動詞 features[5]=基本形
    // - たら: surface='たら' base='た' pos=助動詞 features[5]=仮定形
    #[derive(Debug)]
    struct TaTaraMatcher;
    impl Matcher for TaTaraMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "た" || token.surface == "たら")
                && token.base_form == "た"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match が/けど/けれど/けれども as conjunction particle
    #[derive(Debug)]
    struct GaKedoMatcher;
    impl Matcher for GaKedoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "が" || token.surface == "けど" ||
             token.surface == "けれど" || token.surface == "けれども")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VolitionalVerbMatcher)),
        TokenMatcher::Custom(Arc::new(VolitionalAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(ToQuotationMatcher)),
        TokenMatcher::Custom(Arc::new(ShiVerbMatcher)),
        TokenMatcher::Custom(Arc::new(TaTaraMatcher)),
        optional(TokenMatcher::Custom(Arc::new(GaKedoMatcher))),
    ]
}

// Pattern: 言うまでもない ② (sentence-initial "it goes without saying")
// Structures:
//   - いう + まで + も + ない + (optional: こと + だ) + (optional: が/けど/けれども)
//   - 言うまでもなく (single token adjective with base form 言うまでもない)
// Note: This differs from は言うまでもない which comes after phrases
pub fn iumademonai_u2461() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Custom matcher that handles BOTH split and single-token forms
    #[derive(Debug)]
    struct IumademonaiSentenceInitialMatcher;
    impl super::Matcher for IumademonaiSentenceInitialMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Form 1: Single token (言うまでもなく with base 言うまでもない)
            if token.base_form == "言うまでもない"
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
            {
                return true;
            }

            // Form 2: Split tokenization - match いう (verb)
            if token.base_form == "いう"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
            {
                return true;
            }

            false
            })
        }
    }

    // After いう, expect まで
    #[derive(Debug)]
    struct MadeParticleMatcher;
    impl super::Matcher for MadeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "まで"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // After まで, expect も
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl super::Matcher for MoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // After も, expect ない (形容詞)
    #[derive(Debug)]
    struct NaiAdjectiveMatcher;
    impl super::Matcher for NaiAdjectiveMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ない"
                && token.pos.first().is_some_and(|pos| pos == "形容詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Optional: こと
    #[derive(Debug)]
    struct KotoMatcher;
    impl super::Matcher for KotoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Optional: だ
    #[derive(Debug)]
    struct DaAuxiliaryMatcher;
    impl super::Matcher for DaAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Optional: が/けど/けれども
    #[derive(Debug)]
    struct GaKedoMatcher;
    impl super::Matcher for GaKedoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "が" || token.surface == "けど" || token.surface == "けれども" || token.surface == "けれど" || token.surface == "けども")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Pattern structure:
    // - First token: either 言うまでもなく (single) OR いう (verb, start of split form)
    // - If split form (いう): followed by まで + も + ない
    // - Optionally followed by こと, だ, が/けど/etc
    vec![
        TokenMatcher::Custom(Arc::new(IumademonaiSentenceInitialMatcher)),
        optional(TokenMatcher::Custom(Arc::new(MadeParticleMatcher))),
        optional(TokenMatcher::Custom(Arc::new(MoParticleMatcher))),
        optional(TokenMatcher::Custom(Arc::new(NaiAdjectiveMatcher))),
        optional(TokenMatcher::Custom(Arc::new(KotoMatcher))),
        optional(TokenMatcher::Custom(Arc::new(DaAuxiliaryMatcher))),
        optional(TokenMatcher::Custom(Arc::new(GaKedoMatcher))),
    ]
}

// そうもない: very unlikely / doesn't even appear likely
// Structures:
//   - Verb[stem] + そう + も + ない
//   - Verb[stem] + そう + も + ありません
pub fn soumonai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match そう as 名詞/接尾/助動詞語幹
    #[derive(Debug)]
    struct SouAuxiliaryMatcher;
    impl super::Matcher for SouAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "そう"
                && token.base_form == "そう"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                && token.pos.get(2).is_some_and(|pos| pos == "助動詞語幹") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match も as 助詞/係助詞
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl super::Matcher for MoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "も"
                && token.base_form == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match ない (i-adjective form), ある, ます, or ん (for polite negative)
    #[derive(Debug)]
    struct NegativeOrAruMatcher;
    impl super::Matcher for NegativeOrAruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // ない as i-adjective
            if token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
            {
                return true;
            }
            // ある or あり (for polite form)
            if (token.surface == "ある" || token.surface == "あり")
                && token.base_form == "ある"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
            {
                return true;
            }
            // ます (for polite negative)
            if token.surface == "ませ"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
            {
                return true;
            }
            // ん (for polite negative ending)
            if token.surface == "ん"
                && token.base_form == "ん"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
            {
                return true;
            }
            false
            })
        }
    }

    vec![
        super::flexible_verb_form(), // Verb in 連用形 or 連用タ接続
        TokenMatcher::Custom(Arc::new(SouAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(MoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(NegativeOrAruMatcher)),
        optional(TokenMatcher::Custom(Arc::new(
            NegativeOrAruMatcher,
        ))),
        optional(TokenMatcher::Custom(Arc::new(
            NegativeOrAruMatcher,
        ))),
    ]
}

// Pattern: ないことはない
// Pattern: ないことはない (it's not impossible / it's not that...not)
// Structures: Verb[ない] + ことはない / い-Adj[ない] + ことはない / な-Adj + ではない + ことはない
// Double negative expressing possibility, often with half-hearted nuance
pub fn naikotohanai() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct NaiMatcher;
    impl Matcher for NaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ない"
                && (token.pos.first().is_some_and(|pos| pos == "助動詞")
                    || token.pos.first().is_some_and(|pos| pos == "形容詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct KotoMatcher;
    impl Matcher for KotoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct HaMoParticleMatcher;
    impl Matcher for HaMoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "は" || token.surface == "も")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NaiAruMatcher;
    impl Matcher for NaiAruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match ない (adjective or auxiliary) or ある (verb for polite ありません)
            (token.surface == "ない" || token.base_form == "ない")
                && (token.pos.first().is_some_and(|pos| pos == "形容詞")
                    || token.pos.first().is_some_and(|pos| pos == "助動詞"))
                || (token.base_form == "ある"
                    && token.pos.first().is_some_and(|pos| pos == "動詞"))
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
        TokenMatcher::Custom(Arc::new(HaMoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(NaiAruMatcher)),
    ]
}

// Pattern: なんか・なんて (such as, things like)
// Structures: Verb/Adjective/Noun + なんて OR Noun + なんか
pub fn nanka_u30fb_nante() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matches なんて or なんか (副助詞 - adverbial particle)
    #[derive(Debug)]
    struct NanteNankaMatcher;
    impl Matcher for NanteNankaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "なんて" || token.surface == "なんか")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Any word (verb, adjective, or noun)
        TokenMatcher::Custom(Arc::new(NanteNankaMatcher)),
    ]
}

// Pattern: 又〜も (moreover/additionally)
// Structures: また + (comma) + ... + も/でも/ても
pub fn mata_u301c_mo() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match また as conjunction
    #[derive(Debug)]
    struct MataMatcher;
    impl Matcher for MataMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "また" && token.pos.first().is_some_and(|pos| pos == "接続詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match comma (optional)
    #[derive(Debug)]
    struct CommaMatcher;
    impl Matcher for CommaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "、" && token.pos.first().is_some_and(|pos| pos == "記号") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match も as 係助詞
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "も" && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(MataMatcher)),
        optional(TokenMatcher::Custom(Arc::new(CommaMatcher))),
        wildcard(0, 10, vec![]),
        TokenMatcher::Custom(Arc::new(MoParticleMatcher)),
    ]
}

// ついでに: While you're at it / on the occasion of
// Structures: Verb + ついでに, Noun + の + ついでに, Phrase。ついでに + Phrase
pub fn tsuideni() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct TsuideMatcher;
    impl Matcher for TsuideMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.base_form == "ついで"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && (token.pos.get(1).is_some_and(|p| p == "一般")
                    || token.pos.get(1).is_some_and(|p| p == "非自立")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NoRentaikaMatcher;
    impl Matcher for NoRentaikaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),                                  // Verb or Noun
        optional(TokenMatcher::Custom(Arc::new(
            NoRentaikaMatcher,
        ))), // Optional の (for nouns)
        TokenMatcher::Custom(Arc::new(TsuideMatcher)),      // ついで (名詞)
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),  // に (格助詞)
    ]
}

// Pattern: と共に (together with, at the same time as)
// Structures: Verb/Adj/Noun + と共に (single compound particle)
pub fn totomoni() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // と共に as compound particle
    #[derive(Debug)]
    struct TotomoniMatcher;
    impl Matcher for TotomoniMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "と共に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "連語") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(TotomoniMatcher)),
    ]
}

// Pattern: につれて (as, in proportion to)
// Structures: Verb + につれて / Noun + につれて
pub fn nitsurete() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct NitsureteMatcher;
    impl Matcher for NitsureteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "につれて"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "連語") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),  // Can be verb or noun
        TokenMatcher::Custom(Arc::new(NitsureteMatcher)),
    ]
}

// Pattern: 直ちに (immediately/at once - formal/purposeful)
// Structures: ただちに + Phrase
pub fn tadachini() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct TadachiniMatcher;
    impl Matcher for TadachiniMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "ただちに" || token.surface == "直ちに")
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    vec![TokenMatcher::Custom(Arc::new(TadachiniMatcher))]
}

// Pattern: たとたんに (the instant/the moment)
// Structures: Verb［た］+ とたん(に)
pub fn tatotanni() -> Vec<TokenMatcher> {
    use super::{flexible_verb_form, past_auxiliary};

    #[derive(Debug)]
    struct TotanNounMatcher;
    impl Matcher for TotanNounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "とたん" || token.surface == "途端")
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                && token.pos.get(2).is_some_and(|pos| pos == "副詞可能") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        flexible_verb_form(),
        past_auxiliary(),
        TokenMatcher::Custom(Arc::new(TotanNounMatcher)),
        optional(TokenMatcher::Custom(Arc::new(NiParticleMatcher))),
    ]
}

// Pattern: おきに (at intervals of / every X)
// Structures: Number + Counter + おきに
pub fn okini() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct NumberMatcher;
    impl Matcher for NumberMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "数") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct CounterMatcher;
    impl Matcher for CounterMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                && token.pos.get(2).is_some_and(|pos| pos == "助数詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct OkiMatcher;
    impl Matcher for OkiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "おき"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NiCaseParticleMatcher;
    impl Matcher for NiCaseParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NumberMatcher)),
        TokenMatcher::Custom(Arc::new(CounterMatcher)),
        TokenMatcher::Custom(Arc::new(OkiMatcher)),
        TokenMatcher::Custom(Arc::new(NiCaseParticleMatcher)),
    ]
}

// Pattern: たびに
// たびに: Every time / Whenever
// Structures: Verb［る］+ たびに, Noun + の + たびに
pub fn tabini() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct TabiNounMatcher;
    impl super::Matcher for TabiNounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            token.surface == "たび"
                && token.base_form == "たび"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                && token.pos.get(2).is_some_and(|pos| pos == "副詞可能") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            token.surface == "に"
                && token.base_form == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NoRentaikaMatcher;
    impl super::Matcher for NoRentaikaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            token.surface == "の"
                && token.base_form == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        // Any token (verb in 基本形 or noun)
        any(),
        // Optional の (連体化 particle for nouns)
        optional(TokenMatcher::Custom(Arc::new(
            NoRentaikaMatcher,
        ))),
        // たび (名詞/非自立/副詞可能)
        TokenMatcher::Custom(Arc::new(TabiNounMatcher)),
        // に (助詞/格助詞)
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
    ]
}

// Pattern: あるいは
pub fn aruiha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ながらも (although, even while)
// Structures: Verb[stem] + ながら(も) / な-Adj + ながら(も) / Noun + ながら(も)
pub fn nagaramo() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ながら as 助詞/接続助詞
    #[derive(Debug)]
    struct NagaraMatcher;
    impl Matcher for NagaraMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ながら"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match も as 助詞/係助詞
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

    // Match verb in 連用形 (stem form), な-Adjective, or noun
    #[derive(Debug)]
    struct VerbOrNaAdjectiveOrNounMatcher;
    impl Matcher for VerbOrNaAdjectiveOrNounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Verb in 連用形
            if token.pos.first().is_some_and(|pos| pos == "動詞") {
                return token
                    .features
                    .get(5)
                    .is_some_and(|form| form == "連用形" || form == "連用タ接続");
            }
            // な-Adjective (名詞/形容動詞語幹) or general noun
            if token.pos.first().is_some_and(|pos| pos == "名詞") {
                return true;
            }
            false
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbOrNaAdjectiveOrNounMatcher)),
        TokenMatcher::Custom(Arc::new(NagaraMatcher)),
        optional(TokenMatcher::Custom(Arc::new(MoMatcher))),
    ]
}

// Pattern: において・における (at, in, on, regarding)
// Structures: Noun + において / Noun + における + Noun / Noun + においての + Noun
pub fn nioite_u30fb_niokeru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct NioiteMatcher;
    impl Matcher for NioiteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "において" || token.surface == "における")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "連語") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(NioiteMatcher)),
    ]
}

// Pattern: 第一 (first of all/foremost/most important)
// Structures: だいいち(に), 第一(に), だいいち + の, Phrase + だいいち + だ/です
pub fn daiichi() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match だいいち as adverb OR 第 as prefix
    #[derive(Debug)]
    struct DaiichiMatcher;
    impl Matcher for DaiichiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            (token.surface == "だいいち" && token.pos.first().is_some_and(|pos| pos == "副詞"))

            || (token.surface == "第" && token.pos.first().is_some_and(|pos| pos == "接頭詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match 一 (kanji number "one") - only needed for kanji form
    #[derive(Debug)]
    struct IchiMatcher;
    impl Matcher for IchiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "一" && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "数") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(DaiichiMatcher)),
        // 一 is optional because it's only present in the kanji form
        optional(TokenMatcher::Custom(Arc::new(IchiMatcher))),
    ]
}

// Pattern: ますます (more and more/increasingly)
// Structures: ますます
pub fn masumasu() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    #[derive(Debug)]
    struct MasumasuMatcher;
    impl super::Matcher for MasumasuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ますます"
                && token.pos.first().is_some_and(|pos| pos == "副詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    vec![TokenMatcher::Custom(Arc::new(MasumasuMatcher))]
}

// Pattern: 一方だ
// Pattern: 一方だ (more and more / continuing to / getting X-er and X-er)
// Structures: Verb + 一方 + だ/です
pub fn ippouda() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match 一方 (dependent noun meaning "single direction")
    // 一方 (名詞/非自立/副詞可能)
    #[derive(Debug)]
    struct IppouMatcher;
    impl Matcher for IppouMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "一方"
                && token.base_form == "一方"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match だ or です (copula auxiliary)
    // だ (助動詞, base=だ, 基本形)
    // です (助動詞, base=です, 基本形)
    #[derive(Debug)]
    struct DaDesuMatcher;
    impl Matcher for DaDesuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.base_form == "だ" || token.base_form == "です")
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(IppouMatcher)),
        TokenMatcher::Custom(Arc::new(DaDesuMatcher)),
    ]
}

// Pattern: 一方で
// Pattern: 一方で (on the other hand / while / at the same time)
// Structures: Verb/Adjective + 一方（で） / Phrase。一方（で）
pub fn ippoude() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for 一方 - can be noun or conjunction
    #[derive(Debug)]
    struct IppouMatcher;
    impl Matcher for IppouMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "一方"
                && token.base_form == "一方"
                && (token.pos.first().is_some_and(|pos| pos == "名詞")
                    || token.pos.first().is_some_and(|pos| pos == "接続詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matcher for で particle (not auxiliary verb で from だ)
    #[derive(Debug)]
    struct DeParticleMatcher;
    impl Matcher for DeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(IppouMatcher)),
        optional(TokenMatcher::Custom(Arc::new(DeParticleMatcher))),
    ]
}

// Pattern: 遂に (finally/at last - formal)
// Structures: ついに + Phrase
// Meaning: "finally" or "at last" - emphasizes completion after significant time/effort
// Usage: Indicates something has finally happened after a long period of time
// Examples:
//   - ついに日本上陸 (finally arrived in Japan)
//   - ついにドラゴンズが勝った (The Dragons have finally won)
//   - 遂に結婚した (finally married)
// Note: Both ついに (hiragana) and 遂に (with kanji) are common. Slightly more formal than とうとう.
pub fn tsuini() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct TsuiniMatcher;
    impl Matcher for TsuiniMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "ついに" || token.surface == "遂に")
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    vec![TokenMatcher::Custom(Arc::new(TsuiniMatcher))]
}

// Pattern: すでに
// Pattern: すでに (already - formal)
// Structures: すでに + Phrase
// Meaning: "already" - formal alternative to もう
// Usage: Indicates something is already in a completed/unchanging state
// Examples:
//   - すでに沸いている (already boiled)
//   - すでに決まった (already decided)
//   - すでに遅すぎる (already too late)
pub fn sudeni() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct SudeniMatcher;
    impl Matcher for SudeniMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "すでに"
                && token.pos.first().is_some_and(|pos| pos == "副詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![TokenMatcher::Custom(Arc::new(SudeniMatcher))]
}

// Pattern: ずに (without doing)
// Structures: Verb[未然形] + ず(に)
// Exception: する → せず(に)
pub fn zuni() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match verb in 未然形 (imperfective/negative stem form)
    // This includes both regular verbs and する verbs (which become せ)
    #[derive(Debug)]
    struct MizenFormVerbMatcher;
    impl Matcher for MizenFormVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "動詞")
                && token
                    .features
                    .get(5)
                    .is_some_and(|f| f == "未然形" || f == "未然ヌ接続") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match ず (助動詞, base=ぬ, 特殊・ヌ, 連用ニ接続)
    #[derive(Debug)]
    struct ZuAuxiliaryMatcher;
    impl Matcher for ZuAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ず"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "ぬ" => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match に particle (格助詞/一般) - optional
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(MizenFormVerbMatcher)),
        TokenMatcher::Custom(Arc::new(ZuAuxiliaryMatcher)),
        optional(TokenMatcher::Custom(Arc::new(
            NiParticleMatcher,
        ))),
    ]
}

// Pattern: ずにはいられない (can't help but do / cannot resist doing)
// Structures: Verb[未然形] + ずにはいられない / ずにはいられません
// Exception: する → せずにはいられない
pub fn zunihairarenai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Reuse MizenFormVerbMatcher from zuni
    #[derive(Debug)]
    struct MizenFormVerbMatcher;
    impl Matcher for MizenFormVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "動詞")
                && token
                    .features
                    .get(5)
                    .is_some_and(|f| f == "未然形" || f == "未然ヌ接続") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match ず (助動詞, base=ぬ)
    #[derive(Debug)]
    struct ZuAuxiliaryMatcher;
    impl Matcher for ZuAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ず"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "ぬ" => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match に particle (格助詞/一般)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match は particle (係助詞)
    #[derive(Debug)]
    struct WaParticleMatcher;
    impl Matcher for WaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match い or はいら from いる/はいる verb (未然形)
    // Kagome may tokenize "いられ" differently depending on context
    #[derive(Debug)]
    struct IruMizenMatcher;
    impl Matcher for IruMizenMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然形")
                && ((token.surface == "い" && token.base_form == "いる")
                    || (token.surface == "はいら" && token.base_form == "はいる")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match られ or れ from られる/れる auxiliary (potential)
    // In standard form: られ (未然形)
    // In polite form: れ (連用形)
    #[derive(Debug)]
    struct RareruAuxiliaryMatcher;
    impl Matcher for RareruAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if ((token.surface == "られ" && token.base_form == "られる")
                || (token.surface == "れ" && token.base_form == "れる"))
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match ない (basic form) or ません (polite negative)
    // For standard: ない (助動詞, 基本形)
    // For polite: ませ + ん
    #[derive(Debug)]
    struct NaiMasenMatcher;
    impl Matcher for NaiMasenMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            (token.surface == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "ない")

                || (token.surface == "ませ"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.base_form == "ます") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Optional ん for polite form (ません)
    #[derive(Debug)]
    struct NMatcher;
    impl Matcher for NMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ん"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "ん" => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(MizenFormVerbMatcher)),
        TokenMatcher::Custom(Arc::new(ZuAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        // は particle is optional - Kagome may tokenize "はいら" as verb "はいる" without separate は
        optional(TokenMatcher::Custom(Arc::new(WaParticleMatcher))),
        TokenMatcher::Custom(Arc::new(IruMizenMatcher)),
        TokenMatcher::Custom(Arc::new(RareruAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(NaiMasenMatcher)),
        optional(TokenMatcher::Custom(Arc::new(NMatcher))),
    ]
}

// Pattern: なし (without)
// Structures: Noun + なし + (で/だ/です/の/に)
// Note: Can appear as compound (許可なし as one token) or separate (肉 + なし)
pub fn nashi() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matches なし as 助動詞 or 形容詞 with base_form=ない (standalone)
    // OR compound forms ending in なし (e.g., 許可なし)
    #[derive(Debug)]
    struct NashiOrCompoundMatcher;
    impl Matcher for NashiOrCompoundMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            (token.surface == "なし"
                && token.base_form == "ない"
                && (token.pos.first().is_some_and(|pos| pos == "助動詞")
                    || token.pos.first().is_some_and(|pos| pos == "形容詞")))

            || (token.surface.ends_with("なし")
                && token.base_form.ends_with("ない")
                && token.pos.first().is_some_and(|pos| pos == "形容詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matches で (from だ), だ, です, の, or に particles
    #[derive(Debug)]
    struct NashiFollowingMatcher;
    impl Matcher for NashiFollowingMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            (token.surface == "で" && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞"))

            || (token.surface == "だ" && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞"))

            || (token.surface == "です" && token.base_form == "です"
                && token.pos.first().is_some_and(|pos| pos == "助動詞"))

            || (token.surface == "の" && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化"))

            || (token.surface == "に" && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NashiOrCompoundMatcher)),
        optional(TokenMatcher::Custom(Arc::new(NashiFollowingMatcher))),
    ]
}

// Pattern: あり
// Pattern: あり (with/possible/exists - literary form of ある)
// Structures: Phrase + あり
// Meaning: "with (A), amongst other things" / "that's possible/acceptable"
// Usage: Indicates something as one possibility among many
// Examples:
//   - ラーメンもあり (Ramen is possible/acceptable)
//   - 駐車場ありのホテル (hotel with parking lot)
//   - 字幕ありで見たい (want to watch with subtitles)
//   - めっちゃありです (that's totally possible - casual response)
pub fn ari() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct AriMatcher;
    impl Matcher for AriMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            token.surface == "あり"
                && token.base_form == "ある"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立")
                && token.features.get(5).is_some_and(|f| f == "連用形") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),  // Preceding word (noun, adjective, adverb, etc.)
        TokenMatcher::Custom(Arc::new(AriMatcher)),
    ]
}

// Pattern: 考えられない (unthinkable/unimaginable)
// Structures: 考えられない/ません/なかった/ませんでした
pub fn kangaerarenai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match 考え (未然形 of 考える)
    #[derive(Debug)]
    struct KangaeMatcher;
    impl Matcher for KangaeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.base_form == "考える"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然形") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match られ (potential form suffix)
    #[derive(Debug)]
    struct RareMatcher;
    impl Matcher for RareMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.base_form == "られる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match negative auxiliary (ない or ます)
    #[derive(Debug)]
    struct NegAuxMatcher;
    impl Matcher for NegAuxMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "助動詞")
                && (token.base_form == "ない" || token.base_form == "ます") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match past auxiliary (た or です) - only appears after ない/ます
    #[derive(Debug)]
    struct PastAuxMatcher;
    impl Matcher for PastAuxMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "助動詞")
                && (token.base_form == "た" || token.base_form == "です") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(KangaeMatcher)),
        TokenMatcher::Custom(Arc::new(RareMatcher)),
        TokenMatcher::Custom(Arc::new(NegAuxMatcher)), // ない, なかっ, ません
        optional(TokenMatcher::Custom(Arc::new(PastAuxMatcher))), // た, でした
    ]
}

// Pattern: 必ずしも (not necessarily/not always)
// Structures: 必ずしも (single adverb token)
pub fn kanarazushimo() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KanarazushimoMatcher;
    impl Matcher for KanarazushimoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "必ずしも"
                && token.base_form == "必ずしも"
                && token.pos.first().is_some_and(|pos| pos == "副詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    vec![TokenMatcher::Custom(Arc::new(KanarazushimoMatcher))]
}

// Pattern: 連用形 (Conjunctive Form - Formal Clause Connector)
// Structures: Verb[stem] + 、+ Phrase | い-Adjective[く] + 、+ Phrase
pub fn renyoukei() -> Vec<TokenMatcher> {
    // Matcher for verb in 連用形 OR い-Adjective in 連用テ接続
    #[derive(Debug)]
    struct RenyoukeiMatcher;
    impl Matcher for RenyoukeiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match verb in 連用形 (stem form)
            let is_verb_renyoukei = token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形");

            // Match い-Adjective in 連用テ接続 (く-form)
            let is_iadj_ku = token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "連用テ接続");

            is_verb_renyoukei || is_iadj_ku
            })
        }
    }

    // Matcher for comma (、)
    #[derive(Debug)]
    struct CommaMatcher;
    impl Matcher for CommaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "、"
                && token.pos.first().is_some_and(|pos| pos == "記号")
                && token.pos.get(1).is_some_and(|pos| pos == "読点") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(RenyoukeiMatcher)),
        TokenMatcher::Custom(Arc::new(CommaMatcher)),
    ]
}

// Pattern: 向き (suitable for / facing toward)
// Structures: Noun + 向き
pub fn muki() -> Vec<TokenMatcher> {
    // Matcher for 向き as noun suffix
    #[derive(Debug)]
    struct MukiSuffixMatcher;
    impl Matcher for MukiSuffixMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.base_form == "向き"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Noun before 向き
        TokenMatcher::Custom(Arc::new(MukiSuffixMatcher)),
    ]
}

// Pattern: 向け (intended for/aimed at)
// Structures: Noun + 向け
pub fn muke() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct MukeMatcher;
    impl Matcher for MukeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "向け"
                && token.base_form == "向け"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(MukeMatcher)),
    ]
}

// Pattern: 上がる・上げる (finish up / complete)
// Structures: Verb[stem] + 上がる/上げる
pub fn agaru_u30fb_ageru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for compound verbs ending in あがる or あげる
    #[derive(Debug)]
    struct AgaruAgeruMatcher;
    impl Matcher for AgaruAgeruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "動詞")
                && (token.base_form.ends_with("あがる") || token.base_form.ends_with("あげる"))
                && token.base_form != "あがる"
                && token.base_form != "あげる"
                && token.base_form != "上がる"
                && token.base_form != "上げる" => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![TokenMatcher::Custom(Arc::new(AgaruAgeruMatcher))]
}

// Pattern: 切る (do completely / to exhaustion)
// Structures: Verb[stem] + 切る
pub fn kiru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for きる as auxiliary verb
    // Can be 自立 or 非自立, but must be 五段・ラ行 (godan-ra) conjugation
    #[derive(Debug)]
    struct KiruAuxiliaryMatcher;
    impl Matcher for KiruAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.base_form == "きる"
                && token.features.get(4).is_some_and(|f| f.starts_with("五段・ラ行")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        super::flexible_verb_form(), // Verb in 連用形 or 連用タ接続
        TokenMatcher::Custom(Arc::new(KiruAuxiliaryMatcher)),
    ]
}

// Pattern: 切れない (unable to finish completely)
// Structures: Verb[stem] + 切れない/切れません
pub fn kirenai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for きれる as auxiliary verb (negative potential form)
    // Must be 一段 conjugation (ichidan), pos=動詞/非自立
    #[derive(Debug)]
    struct KirenaiAuxiliaryMatcher;
    impl Matcher for KirenaiAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                && token.base_form == "きれる"
                && token.features.get(4).is_some_and(|f| f == "一段") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        super::flexible_verb_form(), // Verb in 連用形 or 連用タ接続
        TokenMatcher::Custom(Arc::new(KirenaiAuxiliaryMatcher)),
    ]
}

// Pattern: きり (only/just/since)
// Structures: Verb[た] + きり, Noun/Counter + きり/っきり
pub fn kiri() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KiriMatcher;
    impl Matcher for KiriMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            (token.surface == "きり" || token.surface == "っきり")
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "非自立")
                    || token.pos.get(1).is_some_and(|pos| pos == "接尾")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Matches verb/noun/counter before きり
        TokenMatcher::Custom(Arc::new(KiriMatcher)),
    ]
}

// Pattern: かけ
// かけ: Half-finished/about to (Verb[stem] + かけ)
// Structures:
//   - Verb[stem] + かけだ (split: verb + かけ)
//   - Verb[stem] + かける (split: verb + かける)
//   - Verb[stem] + かけの + Noun (can be split or compound)
// Tokenization:
//   - Split form: 食べ (verb) + かけ (動詞/非自立) - match as 2 tokens
//   - Compound form: 死にかけ (single verb with base="死にかける") - match as 1 token
// Note: We need two separate patterns for different tokenizations
pub fn kake() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match かけ/かける as non-independent verb (split form only)
    #[derive(Debug)]
    struct KakeSplitMatcher;
    impl Matcher for KakeSplitMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            (token.surface == "かけ" || token.surface == "かける")
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                && token.base_form == "かける" => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match preceding verb stem
    #[derive(Debug)]
    struct VerbStemMatcher;
    impl Matcher for VerbStemMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbStemMatcher)),
        TokenMatcher::Custom(Arc::new(KakeSplitMatcher)),
    ]
}

// かけ (compound): Compound verbs ending in かける (like 死にかける)
pub fn kake_compound() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct KakeCompoundMatcher;
    impl Matcher for KakeCompoundMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.surface.ends_with("かけ")
                && token.base_form.ends_with("かける")
                && token.base_form != "かける" => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![TokenMatcher::Custom(Arc::new(KakeCompoundMatcher))]
}

// Pattern: にかけて (from A to B, throughout A)
// Structures: Noun + にかけて / Noun + にかけては
pub fn nikakete() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct NikaketeMatcher;
    impl Matcher for NikaketeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "にかけて"
                && token.base_form == "にかけて"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct WaParticleMatcher;
    impl Matcher for WaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "は"
                && token.base_form == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NikaketeMatcher)),
        optional(TokenMatcher::Custom(Arc::new(
            WaParticleMatcher,
        ))),
    ]
}

// Pattern: たて (freshly/just finished)
// Structures: Verb[stem] + たて / Verb[stem] + たて + の + Noun
pub fn tate() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct TateSuffixMatcher;
    impl Matcher for TateSuffixMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            token.surface == "たて"
                && token.base_form == "たて"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NoRentaikaMatcher;
    impl Matcher for NoRentaikaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Verb (連用形) or Noun
        TokenMatcher::Custom(Arc::new(TateSuffixMatcher)),
        optional(TokenMatcher::Custom(Arc::new(
            NoRentaikaMatcher,
        ))), // Optional の for noun modification
    ]
}

// Pattern: たとえ〜ても (even if)
// Structures: たとえ + Verb[ても] / たとえ + い-Adj[ても] / たとえ + な-Adj + でも / たとえ + Noun + でも
pub fn tatoetemo() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct TatoeAdverbMatcher;
    impl Matcher for TatoeAdverbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            token.surface == "たとえ"
                && token.base_form == "たとえ"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct TeMoOrDemoStartMatcher;
    impl Matcher for TeMoOrDemoStartMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match て (connecting particle), で (case particle), OR でも (single token for na-adj)
            (token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞"))
                || (token.surface == "で"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞"))
                || (token.surface == "でも"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "副助詞"))
            })
        }
    }

    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match the pattern: たとえ + <content> + (て|で)+も OR でも
    // Note: でも can be either 1 token (副助詞 for na-adj) or 2 tokens (格助詞+係助詞 for nouns)
    vec![
        TokenMatcher::Custom(Arc::new(TatoeAdverbMatcher)),
        wildcard(1, 5, vec![]),
        TokenMatcher::Custom(Arc::new(TeMoOrDemoStartMatcher)), // Match て, で, or でも
        optional(TokenMatcher::Custom(Arc::new(
            MoParticleMatcher,
        ))), // Optional も (present after て/で, absent after でも as single token)
    ]
}

// Pattern: 込む ① (to put into/go into)
// Structures: Verb[stem] + 込む
pub fn komu_u2460() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for compound verbs ending in 込む (kanji)
    // Note: Both 込む① and 込む② tokenize identically
    // Show both explanations to user when detected
    #[derive(Debug)]
    struct KomuMatcher;
    impl Matcher for KomuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.base_form.ends_with("込む")
                && token.base_form != "込む" => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![TokenMatcher::Custom(Arc::new(KomuMatcher))]
}

// Pattern: 込む ② (to remain in/do deeply)
// Structures: Verb[stem] + 込む
pub fn komu_u2461() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for compound verbs ending in 込む (kanji)
    // Note: Both 込む① and 込む² tokenize identically
    // Show both explanations to user when detected
    #[derive(Debug)]
    struct KomuMatcher;
    impl Matcher for KomuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.base_form.ends_with("込む")
                && token.base_form != "込む" => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![TokenMatcher::Custom(Arc::new(KomuMatcher))]
}

// Pattern: ふりをする (pretend to be/do)
// Structures: Verb/Adjective/Noun + ふり + を + する
pub fn furiwosuru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ふり as 名詞/非自立/一般
    #[derive(Debug)]
    struct FuriMatcher;
    impl Matcher for FuriMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ふり"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match を particle
    #[derive(Debug)]
    struct WoParticleMatcher;
    impl Matcher for WoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "を"
                && token.pos.first().is_some_and(|pos| pos == "助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),                           // Preceding word (verb/adj/noun)
        TokenMatcher::Custom(Arc::new(FuriMatcher)), // ふり
        TokenMatcher::Custom(Arc::new(WoParticleMatcher)), // を
        verb_base("する"),          // する (any form)
    ]
}

// Pattern: できれば・できたら
pub fn dekireba_u30fb_dekitara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: でよければ (if...is okay)
// Structures: Noun + で + よければ
// Tokenization: で (助詞/格助詞) + よけれ (形容詞/自立, base: よい, 仮定形) + ば (助詞/接続助詞)
pub fn deyokereba() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match で as case particle
    #[derive(Debug)]
    struct DeParticleMatcher;
    impl super::Matcher for DeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match よけれ (仮定形 of よい)
    #[derive(Debug)]
    struct YokereMatcher;
    impl super::Matcher for YokereMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "よけれ"
                && token.base_form == "よい"
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token
                    .features
                    .get(5)
                    .is_some_and(|form| form == "仮定形") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match ば as conjunction particle
    #[derive(Debug)]
    struct BaParticleMatcher;
    impl super::Matcher for BaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ば"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(DeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(YokereMatcher)),
        TokenMatcher::Custom(Arc::new(BaParticleMatcher)),
    ]
}

// 次第: as soon as (終わり次第 - as soon as it ends)
// Structures: Verb[stem] + 次第 / [する]Verb + 次第
pub fn shidai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match 次第 as a noun with 副詞可能 feature
    #[derive(Debug)]
    struct ShidaiMatcher;
    impl Matcher for ShidaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "次第"
                && token.base_form == "次第"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副詞可能") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        super::flexible_verb_form(), // Match 連用形 or 連用タ接続
        TokenMatcher::Custom(Arc::new(ShidaiMatcher)),
    ]
}

// とおり: in that way / just like
// Structures:
//   - Verb + とおり
//   - Noun + どおり
//   - Noun + の + とおり
pub fn toori() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match とおり or どおり as noun suffix or bound noun
    #[derive(Debug)]
    struct TooriDooriMatcher;
    impl super::Matcher for TooriDooriMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "とおり" || token.surface == "どおり")
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "非自立")
                    || token.pos.get(1).is_some_and(|pos| pos == "接尾")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match の as 助詞/連体化
    #[derive(Debug)]
    struct NoRentaiMatcher;
    impl super::Matcher for NoRentaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Content word (verb or noun)
        optional(TokenMatcher::Custom(Arc::new(NoRentaiMatcher))),
        TokenMatcher::Custom(Arc::new(TooriDooriMatcher)),
    ]
}

// Pattern: でもある
// Pattern: でもある (is also)
// Structures:
//   - Noun + でもある/でもあります (で can be separate or part of でも)
//   - い-Adjective[く] + もある/もあります
//   - な-Adjective + でもある/でもあります (で can be separate or part of でも)
pub fn demoaru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matches で (格助詞), も (係助詞), or でも (副助詞 single token)
    #[derive(Debug)]
    struct DemoOrMoMatcher;
    impl super::Matcher for DemoOrMoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            (token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞"))

                || (token.surface == "も"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "係助詞"))

                || (token.surface == "でも"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "副助詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matches ある verb (基本形 or 連用形)
    #[derive(Debug)]
    struct AruVerbMatcher;
    impl super::Matcher for AruVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.base_form == "ある"
                && (token.surface == "ある" || token.surface == "あり") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matches ます
    #[derive(Debug)]
    struct MasuMatcher;
    impl super::Matcher for MasuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "ます" => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Pattern: Any word + で/も/でも + optional も + ある + optional ます
    // This handles:
    //   - Noun + で + も + ある (4 tokens)
    //   - Noun + でも + ある (3 tokens, でも as single token)
    //   - い-Adj[く] + も + ある (3 tokens)
    //   - な-Adj + でも + ある (3 tokens, でも as single token)
    vec![
        any(),
        TokenMatcher::Custom(Arc::new(DemoOrMoMatcher)),
        optional(TokenMatcher::Custom(Arc::new(
            DemoOrMoMatcher,
        ))),
        TokenMatcher::Custom(Arc::new(AruVerbMatcher)),
        optional(TokenMatcher::Custom(Arc::new(MasuMatcher))),
    ]
}

// Pattern: どうしても
// Pattern: どうしても (no matter what, by all means, in any case)
// Structures: どうしても as adverb
pub fn doushitemo() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct DoushitemoMatcher;
    impl Matcher for DoushitemoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "どうしても"
                && token.base_form == "どうしても"
                && token.pos.first().is_some_and(|pos| pos == "副詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    vec![TokenMatcher::Custom(Arc::new(DoushitemoMatcher))]
}

// Pattern: もしも～なら・もしも～でも (supposing that / assuming that)
// Structures: もしも + Phrase + (なら/ならば/ば/と/ても/でも)
pub fn moshimo_uff5e_nara_u30fb_moshimo_uff5e_demo() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match もしも adverb
    #[derive(Debug)]
    struct MoshimoMatcher;
    impl Matcher for MoshimoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "もしも"
                && token.base_form == "もしも"
                && token.pos.first().is_some_and(|pos| pos == "副詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match conditional endings: なら, ば, と, ても, でも
    // This includes:
    // - なら (助動詞, base=だ, 仮定形)
    // - ば (助詞/接続助詞)
    // - と (助詞/接続助詞)
    // - ても/でも require て/で + も which are handled by existing patterns
    #[derive(Debug)]
    struct ConditionalMatcher;
    impl Matcher for ConditionalMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match なら (助動詞, base=だ, 仮定形)
            if token.surface == "なら"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
            {
                return true;
            }

            // Match ば (助詞/接続助詞)
            if token.surface == "ば"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
            {
                return true;
            }

            // Match と (助詞/接続助詞)
            if token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
            {
                return true;
            }

            // Match も (助詞/係助詞) for ても/でも constructions
            if token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
            {
                return true;
            }

            false
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(MoshimoMatcher)),
        wildcard(0, 20, vec![]),
        TokenMatcher::Custom(Arc::new(ConditionalMatcher)),
    ]
}

// Pattern: 同士
// Pattern: 同士 (fellow/mutually/together)
// Structures: Noun + 同士
pub fn doushi() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct DoushiMatcher;
    impl Matcher for DoushiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "同士"
                && token.base_form == "同士"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(DoushiMatcher)),
    ]
}

// Pattern: がたい (difficult to do)
// Structures: Verb[stem/連用形] + がたい (+ です optional)
pub fn gatai() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct GataiMatcher;
    impl Matcher for GataiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "がたい"
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        super::flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(GataiMatcher)),
    ]
}

// Pattern: まさか (no way/don't tell me)
// Structures: まさか
pub fn masaka() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    #[derive(Debug)]
    struct MasakaMatcher;
    impl super::Matcher for MasakaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "まさか"
                && token.pos.first().is_some_and(|pos| pos == "副詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    vec![TokenMatcher::Custom(Arc::new(MasakaMatcher))]
}

// Pattern: 前者は・後者は (The Former / The Latter)
// Structures: 前者は + (Comment) | 後者は + (Comment)
pub fn zenshaha_u30fb_koushaha() -> Vec<TokenMatcher> {
    // Matcher for 前者 OR 後者 as nouns
    #[derive(Debug)]
    struct ZenshaKoushaMatcher;
    impl Matcher for ZenshaKoushaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "名詞")
                && (token.base_form == "前者" || token.base_form == "後者") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matcher for は particle (topic marker)
    #[derive(Debug)]
    struct WaParticleMatcher;
    impl Matcher for WaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(ZenshaKoushaMatcher)),
        TokenMatcher::Custom(Arc::new(WaParticleMatcher)),
    ]
}

// Pattern: つい (accidentally/unconsciously/against one's better judgment)
// Structures: つい + Phrase
pub fn tsui() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct TsuiAdverbMatcher;
    impl Matcher for TsuiAdverbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            token.surface == "つい"
                && token.base_form == "つい"
                && token.pos.first().is_some_and(|pos| pos == "副詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![TokenMatcher::Custom(Arc::new(TsuiAdverbMatcher))]
}

// Pattern: せいで (because of / due to - negative result)
// Structures: Verb/Adjective/Noun + (な/の) + せい + で
pub fn seide() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct SeiMatcher;
    impl Matcher for SeiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "せい"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct DeParticleMatcher;
    impl Matcher for DeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NoNaConnectorMatcher;
    impl Matcher for NoNaConnectorMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            (token.surface == "の" && token.pos.first().is_some_and(|pos| pos == "助詞"))
                || (token.surface == "な" && token.pos.first().is_some_and(|pos| pos == "助動詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),  // Verb/Adjective/Noun before せい
        optional(TokenMatcher::Custom(Arc::new(NoNaConnectorMatcher))),
        TokenMatcher::Custom(Arc::new(SeiMatcher)),
        TokenMatcher::Custom(Arc::new(DeParticleMatcher)),
    ]
}

// Pattern: くせに (despite/even though)
// Structures: Verb/Adjective/Noun + (な/の) + くせ + に
pub fn kuseni() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KuseMatcher;
    impl Matcher for KuseMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "くせ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(KuseMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
    ]
}

// Pattern: がち (tend to/prone to)
// Structures: Verb[stem] + がち / Noun + がち
pub fn gachi() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct GachiPrecedingMatcher;
    impl Matcher for GachiPrecedingMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Matches: Verb in 連用形 (stem) OR Noun
            if token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形") {
                return true;
            }
            // Match nouns (especially サ変接続 and 一般)
            if token.pos.first().is_some_and(|pos| pos == "名詞") {
                return true;
            }
            false
            })
        }
    }

    #[derive(Debug)]
    struct GachiMatcher;
    impl Matcher for GachiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "がち"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                && token.pos.get(2).is_some_and(|pos| pos == "形容動詞語幹") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(GachiPrecedingMatcher)),
        TokenMatcher::Custom(Arc::new(GachiMatcher)),
    ]
}

// Pattern: ぎみ (sensation of / tendency)
// Structures: Verb[stem/連用形] + ぎみ / Noun + ぎみ
pub fn gimi() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct GimiPrecedingMatcher;
    impl Matcher for GimiPrecedingMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Matches: Verb in 連用形 (stem) OR Noun
            if token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形") {
                return true;
            }
            // Match nouns
            if token.pos.first().is_some_and(|pos| pos == "名詞") {
                return true;
            }
            false
            })
        }
    }

    #[derive(Debug)]
    struct GimiMatcher;
    impl Matcher for GimiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ぎみ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                && token.pos.get(2).is_some_and(|pos| pos == "一般") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(GimiPrecedingMatcher)),
        TokenMatcher::Custom(Arc::new(GimiMatcher)),
    ]
}

// Pattern: っぽい (ish/like/tendency to)
// Structures: Verb[stem]/Adjective/Noun + っぽい
//
// Tokenization:
//   Split form: Content word + っぽい (形容詞/接尾) - detectable
//   Compound form: 白っぽい, 安っぽい (形容詞/自立) - not detectable (lexicalized)
//
// Note: っぽい creates い-Adjectives, so it conjugates (っぽい, っぽく, っぽかった, etc.)
pub fn ppoi() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct PpoiSuffixMatcher;
    impl Matcher for PpoiSuffixMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.base_form == "っぽい"
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),  // Verb, Noun, or な-Adjective
        TokenMatcher::Custom(Arc::new(PpoiSuffixMatcher)),
    ]
}

// Pattern: っぱなし (left in a state / left unchecked)
// Structures: Verb[stem] + っぱなし
//
// Tokenization:
//   Split form: Verb/Noun (any form) + っぱなし (名詞/接尾/一般) - detectable
//   Compound form: 開けっぱなし (名詞/一般) - not detectable as pattern
//
// Note: Sometimes verb stems are tokenized as nouns (勝ち, つけ), so we match both
pub fn ppanashi() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct PpanashiSuffixMatcher;
    impl Matcher for PpanashiSuffixMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "っぱなし"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),  // Match verbs or nouns (verb stems can be nouns)
        TokenMatcher::Custom(Arc::new(PpanashiSuffixMatcher)),
    ]
}

// Pattern: わざわざ (to go out of one's way)
// Structures: わざわざ + Phrase
pub fn wazawaza() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct WazawazaMatcher;
    impl Matcher for WazawazaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "わざわざ"
                && token.base_form == "わざわざ"
                && token.pos.first().is_some_and(|pos| pos == "副詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    vec![TokenMatcher::Custom(Arc::new(WazawazaMatcher))]
}

// Pattern: 一体 (on earth/in the world)
// Structures: いったい + Question Word + Phrase
pub fn ittai() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct IttaiMatcher;
    impl Matcher for IttaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "いったい"
                && token.base_form == "いったい"
                && token.pos.first().is_some_and(|pos| pos == "副詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    vec![TokenMatcher::Custom(Arc::new(IttaiMatcher))]
}

// Pattern: 折角 (with effort/specially/long-awaited)
// Structures: せっかく + Phrase OR せっかく + の + Noun
pub fn sekkaku() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct SekkakuMatcher;
    impl Matcher for SekkakuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "せっかく" || token.surface == "折角")
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NoRentaika;
    impl Matcher for NoRentaika {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(SekkakuMatcher)),
        optional(TokenMatcher::Custom(Arc::new(NoRentaika))),
    ]
}

// っけ: Recall/confirmation particle (trying to remember or confirm information)
// Structures:
//   - Verb[た] + っけ
//   - Verb[る] + んだ + っけ
//   - い-Adjective[た] + っけ
//   - な-Adjective/Noun + だった + っけ
pub fn kke() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for っけ as sentence-ending particle
    #[derive(Debug)]
    struct KkeParticleMatcher;
    impl Matcher for KkeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            token.surface == "っけ"
                && token.base_form == "っけ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "終助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matcher for た or だ in base form (基本形) before っけ
    #[derive(Debug)]
    struct TaDaAuxiliaryMatcher;
    impl Matcher for TaDaAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            (token.surface == "た" || token.surface == "だ")
                && (token.base_form == "た" || token.base_form == "だ")
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "基本形") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Content word before auxiliary (verb, adjective, noun, etc.)
        TokenMatcher::Custom(Arc::new(TaDaAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(KkeParticleMatcher)),
    ]
}

// Pattern: 代わりに
// 代わりに: in exchange for / instead of (電話する代わりに - in exchange for calling)
// Structures: Verb + 代わりに / [い]Adj + 代わりに / [な]Adj + な + 代わりに / Noun + の + 代わりに
pub fn kawarini() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match 代わり as a noun
    #[derive(Debug)]
    struct KawariMatcher;
    impl Matcher for KawariMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "代わり" || token.surface == "替わり")
                && (token.base_form == "代わり" || token.base_form == "替わり")
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match に as case particle
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        optional(any()), // Can be preceded by verb, adjective, noun + の, or nothing
        TokenMatcher::Custom(Arc::new(KawariMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
    ]
}

// Pattern: に代わって (in place of / on behalf of)
// Structures: Noun + に + 代わって / 代わり
pub fn nikawatte() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match に particle (case-marking particle)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match 代わる verb in any conjugation form
    #[derive(Debug)]
    struct KawaruVerbMatcher;
    impl Matcher for KawaruVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.base_form == "代わる"
                && token.pos.first().is_some_and(|pos| pos == "動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match て particle (conjunctive particle)
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl Matcher for TeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(KawaruVerbMatcher)),
        optional(TokenMatcher::Custom(Arc::new(TeParticleMatcher))),
    ]
}

// Pattern: どころか (far from, let alone, anything but)
// Structures: Verb/Adjective/Noun/な + どころか
pub fn dokoroka() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct DokorokaMatcher;
    impl Matcher for DokorokaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "どころか"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(DokorokaMatcher)),
    ]
}

// Pattern: という理由で (for that reason, being that)
// Structures: という理由で / そういう理由で
pub fn toiuriyuude() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for either という or そういう
    #[derive(Debug)]
    struct ToiuOrSoiuMatcher;
    impl Matcher for ToiuOrSoiuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "という"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "連語"))
            || (token.surface == "そういう"
                && token.pos.first().is_some_and(|pos| pos == "連体詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matcher for 理由 as noun
    #[derive(Debug)]
    struct RiyuuMatcher;
    impl Matcher for RiyuuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "理由"
                && token.base_form == "理由"
                && token.pos.first().is_some_and(|pos| pos == "名詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matcher for で as case marking particle
    #[derive(Debug)]
    struct DeMatcher;
    impl Matcher for DeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Pattern: (という or そういう) + 理由 + で
    vec![
        TokenMatcher::Custom(Arc::new(ToiuOrSoiuMatcher)),
        TokenMatcher::Custom(Arc::new(RiyuuMatcher)),
        TokenMatcher::Custom(Arc::new(DeMatcher)),
    ]
}

// Pattern: ～は～となっている (A is B / has become B)
// Structures: [な-Adjective/Noun] + となっている/となっています
pub fn uff5e_ha_uff5e_tonatteiru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match と particle (格助詞)
    #[derive(Debug)]
    struct ToParticleMatcher;
    impl Matcher for ToParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "と"
                && token.base_form == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match なっ from なる (連用タ接続)
    #[derive(Debug)]
    struct NatMatcher;
    impl Matcher for NatMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "なっ"
                && token.base_form == "なる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token
                    .features
                    .get(5)
                    .is_some_and(|f| f == "連用タ接続" || f == "連用形") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match て particle (接続助詞)
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl Matcher for TeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match いる/い from いる (auxiliary verb)
    #[derive(Debug)]
    struct IruMatcher;
    impl Matcher for IruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "いる" || token.surface == "い")
                && token.base_form == "いる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match ます (optional polite form)
    #[derive(Debug)]
    struct MasuMatcher;
    impl Matcher for MasuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ます"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(ToParticleMatcher)),
        TokenMatcher::Custom(Arc::new(NatMatcher)),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(IruMatcher)),
        optional(TokenMatcher::Custom(Arc::new(MasuMatcher))),
    ]
}

// Pattern: 左右する
// Pattern: 左右する (influence/dictate/control)
// Structures: 左右 + する (all conjugations including passive される)
pub fn sayuusuru() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct SayuuMatcher;
    impl Matcher for SayuuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "左右"
                && token.base_form == "左右"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "サ変接続") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(SayuuMatcher)),
        verb_base("する"),
    ]
}

// Pattern: あるいは (or/alternatively)
// Structures: (Optional か) + あるいは
pub fn aruiwa() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KaParticleMatcher;
    impl Matcher for KaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "か"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞／並立助詞／終助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct AruiwaMatcher;
    impl Matcher for AruiwaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "あるいは"
                && token.pos.first().is_some_and(|pos| pos == "接続詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        optional(TokenMatcher::Custom(Arc::new(KaParticleMatcher))),
        TokenMatcher::Custom(Arc::new(AruiwaMatcher)),
    ]
}

// Pattern: ～ずつ (each/per/at a time)
// Structures: Number + Counter + ずつ / 少し + ずつ / いくらか + ずつ
// Note: Matches the immediate token before ずつ (counter, noun, adverb, or particle)
pub fn zutsu() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct ZutsuPrecedingMatcher;
    impl Matcher for ZutsuPrecedingMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match the immediate token before ずつ:
            // 1. Numbers (名詞/数) - for patterns like 一人ずつ (where 一 might be separate)
            if token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "数") {
                return true;
            }
            // 2. Counters (名詞/接尾/助数詞) - for 人, 日, etc.
            if token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾") {
                return true;
            }
            // 3. General nouns (名詞/一般) - for 一つ, いくら, etc.
            if token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般") {
                return true;
            }
            // 4. 少し (副詞/助詞類接続)
            if token.surface == "少し"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "助詞類接続") {
                return true;
            }
            // 5. か particle (for いくらか pattern)
            if token.surface == "か"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞／並立助詞／終助詞") {
                return true;
            }
            false
            })
        }
    }

    #[derive(Debug)]
    struct ZutsuMatcher;
    impl Matcher for ZutsuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ずつ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(ZutsuPrecedingMatcher)),
        TokenMatcher::Custom(Arc::new(ZutsuMatcher)),
    ]
}

// かなり + の + Noun: Considerable amount of
// Structures: かなり + の + Noun
pub fn kanari_no_noun() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    #[derive(Debug)]
    struct KanariNounMatcher;
    impl Matcher for KanariNounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "かなり"
                && token.pos.first().is_some_and(|pos| pos == "名詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NoParticleMatcher;
    impl Matcher for NoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(KanariNounMatcher)),
        TokenMatcher::Custom(Arc::new(NoParticleMatcher)),
        super::noun_matcher(),
    ]
}

// Pattern: ～ても～なくても (whether or not)
// Structures: Verb［ても］(A) + Verb［なくても］(A)
//
// Note: This pattern requires the same verb to appear twice with different conjugations.
// We use a simpler approach: match Verb + て + も + Verb + なく + て + も
pub fn temo_nakutemo() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct TemoNakutemoTeDeParticleMatcher;
    impl super::Matcher for TemoNakutemoTeDeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct TemoNakutemoMoParticleMatcher;
    impl super::Matcher for TemoNakutemoMoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct MizenVerbMatcher;
    impl super::Matcher for MizenVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match verb in 未然形 (or 未然ウ接続 for volitional)
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然形" || f == "未然ウ接続")
            })
        }
    }

    #[derive(Debug)]
    struct NakuNaiMatcher;
    impl super::Matcher for NakuNaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            (token.surface == "なく" || token.surface == "なくっ")
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        // First verb (連用形 or 連用タ接続)
        super::flexible_verb_form(),
        // て/で
        TokenMatcher::Custom(Arc::new(TemoNakutemoTeDeParticleMatcher)),
        // も
        TokenMatcher::Custom(Arc::new(TemoNakutemoMoParticleMatcher)),
        // Second verb (未然形) - ideally same base as first, but we can't validate that easily
        TokenMatcher::Custom(Arc::new(MizenVerbMatcher)),
        // なく (ない)
        TokenMatcher::Custom(Arc::new(NakuNaiMatcher)),
        // て
        TokenMatcher::Custom(Arc::new(TemoNakutemoTeDeParticleMatcher)),
        // も
        TokenMatcher::Custom(Arc::new(TemoNakutemoMoParticleMatcher)),
    ]
}

// Pattern: しかない (polite form - しかありません)
// Structures: Verb + しかありません
pub fn shikanai_polite() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct ShikaParticleMatcher;
    impl super::Matcher for ShikaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            token.surface == "しか"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct AriVerbMatcher;
    impl super::Matcher for AriVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            token.surface == "あり"
                && token.base_form == "ある"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct MaseMatcher;
    impl super::Matcher for MaseMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            token.surface == "ませ"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "未然形") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NMatcher;
    impl super::Matcher for NMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            token.surface == "ん"
                && token.base_form == "ん"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        // Verb (基本形)
        verb_form("基本形"),
        // しか (係助詞)
        TokenMatcher::Custom(Arc::new(ShikaParticleMatcher)),
        // あり (ある verb, 連用形)
        TokenMatcher::Custom(Arc::new(AriVerbMatcher)),
        // ませ (ます auxiliary, 未然形)
        TokenMatcher::Custom(Arc::new(MaseMatcher)),
        // ん (auxiliary)
        TokenMatcher::Custom(Arc::new(NMatcher)),
    ]
}

// て初めて: Only after/not until
// Structures: Verb[て] + 初めて
pub fn te_hajimete() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for て/で as conjunction particle
    #[derive(Debug)]
    struct TeDeConjunctionMatcher;
    impl Matcher for TeDeConjunctionMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matcher for 初めて as adverb
    #[derive(Debug)]
    struct HajimeteAdverbMatcher;
    impl Matcher for HajimeteAdverbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "初めて" || token.surface == "はじめて")
                && token.base_form == "初めて"
                && token.pos.first().is_some_and(|pos| pos == "副詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        super::flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(TeDeConjunctionMatcher)),
        TokenMatcher::Custom(Arc::new(HajimeteAdverbMatcher)),
    ]
}

// 中: During/throughout/in the middle of
// Structures: Noun + 中（ちゅう/じゅう）（に）
pub fn chuu() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for 中 or じゅう as suffix
    #[derive(Debug)]
    struct ChuuJuuSuffixMatcher;
    impl Matcher for ChuuJuuSuffixMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 


            ((token.surface == "中" && token.base_form == "中")
                || (token.surface == "じゅう" && token.base_form == "じゅう"))
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matcher for optional に particle
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Noun before 中/じゅう
        TokenMatcher::Custom(Arc::new(ChuuJuuSuffixMatcher)),
        optional(TokenMatcher::Custom(Arc::new(NiParticleMatcher))),
    ]
}

// Pattern: できれば・できたら (if possible)
// Structures: できれば/できたら + Phrase
pub fn dekireba_dekitara() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // できれ (仮定形 of できる) OR でき (連用形 of できる)
    #[derive(Debug)]
    struct DekirebaDekiraraMatcher;
    impl Matcher for DekirebaDekiraraMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.base_form == "できる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && (token.features.get(5).is_some_and(|form| form == "仮定形")
                    || token.features.get(5).is_some_and(|form| form == "連用形")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // ば (接続助詞) OR たら (た in 仮定形)
    #[derive(Debug)]
    struct BaTaraMatcher;
    impl Matcher for BaTaraMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if 
            (token.surface == "ば"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞"))

            || (token.surface == "たら"
                && token.base_form == "た"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|form| form == "仮定形")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(DekirebaDekiraraMatcher)),
        TokenMatcher::Custom(Arc::new(BaTaraMatcher)),
    ]
}
