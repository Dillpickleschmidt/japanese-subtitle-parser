use crate::pattern_matcher::{MatchContext, TokenMatcher};
use std::sync::Arc;

use super::{
    adjective, any, check_token, flexible_verb_form, noun, optional, or, past_auxiliary,
    rareru_suffix, reru_suffix, surface, surface_particle, verb, verb_base, verb_form, wildcard,
    Matcher,
};

// Pattern: という (called/named)
// Structures: Noun (A) + という + Noun (B)
pub fn toiu() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct ToiuMatcher;
    impl super::Matcher for ToiuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "という"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        super::noun(),
        TokenMatcher::Custom(Arc::new(ToiuMatcher)),
        super::noun(),
    ]
}

// Pattern: という1 (every single A)
// Structures: Noun (A) + という + Noun (A) - same noun repeated
// Meaning: "every single (A)" / "(A) of all (A)s"
// という1 is UNDETECTABLE - requires cross-token validation not supported by current architecture
// See tests/n1_patterns.rs toiu1 comment for full explanation
// Users should rely on basic という pattern and manually check if nouns are identical
pub fn toiu1() -> Vec<TokenMatcher> {
    vec![] // Intentionally empty - pattern cannot be detected
}

// Match まま (unchanged state noun)
fn mama_matcher() -> TokenMatcher {
    #[derive(Debug)]
    struct MamaMatcher;
    impl super::Matcher for MamaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "まま"
                        && token.pos.first().is_some_and(|pos| pos == "名詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "非自立") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(MamaMatcher))
}

// Match に particle (optional after まま)
fn ni_particle_matcher() -> TokenMatcher {
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "に"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(NiParticleMatcher))
}

// Match any verb form that can precede た
fn verb_before_ta_matcher() -> TokenMatcher {
    #[derive(Debug)]
    struct VerbBeforeTaMatcher;
    impl super::Matcher for VerbBeforeTaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(VerbBeforeTaMatcher))
}

// Match た (past auxiliary) - strict surface match only
fn ta_auxiliary_matcher() -> TokenMatcher {
    #[derive(Debug)]
    struct TaAuxiliaryMatcher;
    impl super::Matcher for TaAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "た"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(TaAuxiliaryMatcher))
}

// Pattern: まま(に) - Verb[た] + まま
// Structure: Verb stem + た + まま (WITHOUT に)
pub fn mama_ni() -> Vec<TokenMatcher> {
    vec![verb(), past_auxiliary(), mama_matcher()]
}

// Pattern: まま(に) - Verb[た] + まま + に
// Structure: Verb stem + た + まま + に (WITH required に)
pub fn mama_ni_with_ni() -> Vec<TokenMatcher> {
    vec![
        verb(),
        past_auxiliary(),
        mama_matcher(),
        ni_particle_matcher(),
    ]
}

// Pattern: まま(に) - Verb[ない] + まま (+ に)
// Structure: Verb negative + ない + まま (+ に)
pub fn mama_ni_nai() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct NaiAuxiliaryMatcher;
    impl super::Matcher for NaiAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ない"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        verb_form("未然形"),
        TokenMatcher::Custom(Arc::new(NaiAuxiliaryMatcher)),
        mama_matcher(),
        optional(ni_particle_matcher()),
    ]
}

// Pattern: まま(に) - い-Adjective + まま (+ に)
// Structure: い-Adjective + まま (+ に)
pub fn mama_ni_i_adj() -> Vec<TokenMatcher> {
    vec![adjective(), mama_matcher(), optional(ni_particle_matcher())]
}

// Pattern: まま(に) - な-Adjective + な + まま (+ に)
// Structure: な-Adjective stem + な (だ auxiliary) + まま (+ に)
pub fn mama_ni_na_adj() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct NaAdjStemMatcher;
    impl super::Matcher for NaAdjStemMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.pos.first().is_some_and(|pos| pos == "名詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NaCopulaMatcher;
    impl super::Matcher for NaCopulaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "な"
                        && token.base_form == "だ"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NaAdjStemMatcher)),
        TokenMatcher::Custom(Arc::new(NaCopulaMatcher)),
        mama_matcher(),
        optional(ni_particle_matcher()),
    ]
}

// Pattern: まま(に) - Noun + の + まま (+ に)
// Structure: Noun + の (connective particle) + まま (+ に)
pub fn mama_ni_noun() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct NoParticleMatcher;
    impl super::Matcher for NoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "の"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "連体化")
            })
        }
    }

    vec![
        noun(),
        TokenMatcher::Custom(Arc::new(NoParticleMatcher)),
        mama_matcher(),
        optional(ni_particle_matcher()),
    ]
}

// Pattern: まま(に)1 (as one wishes, on a whim)
// Structure: Verb[dictionary form] + (が) + まま(に)
// Meaning: "to do as one wishes/desires", "on a whim"
// Different from basic まま(に) which uses past/negative forms and means "left unchanged"
pub fn mama_ni_1() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct DictionaryFormVerbMatcher;
    impl super::Matcher for DictionaryFormVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token.features.get(5).is_some_and(|f| f == "基本形")
            })
        }
    }

    #[derive(Debug)]
    struct GaParticleMatcher;
    impl super::Matcher for GaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "が"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(DictionaryFormVerbMatcher)),
        optional(TokenMatcher::Custom(Arc::new(GaParticleMatcher))),
        mama_matcher(),
        optional(ni_particle_matcher()),
    ]
}

// Pattern: に至るまで (everything from A to B, up to and including)
// Structures: Noun B + に + 至る + まで (+ の + Noun C)
// Note: Often preceded by "Noun A + から/より" but we match the core pattern only
pub fn niitarumade() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match に as case particle
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "に"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            })
        }
    }

    // Match 至る verb in 基本形
    #[derive(Debug)]
    struct ItaruMatcher;
    impl super::Matcher for ItaruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.base_form == "至る"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token.features.get(5).is_some_and(|f| f == "基本形")
            })
        }
    }

    // Match まで as adverbial particle
    #[derive(Debug)]
    struct MadeParticleMatcher;
    impl super::Matcher for MadeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "まで"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "副助詞")
            })
        }
    }

    // Match の as relativizing particle (連体化)
    #[derive(Debug)]
    struct NoRentaikaMatcher;
    impl super::Matcher for NoRentaikaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "の"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "連体化")
            })
        }
    }

    vec![
        // Required: Noun B + に
        super::noun(),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        // Required: 至る + まで
        TokenMatcher::Custom(Arc::new(ItaruMatcher)),
        TokenMatcher::Custom(Arc::new(MadeParticleMatcher)),
        // Optional: の + Noun C
        optional(TokenMatcher::Custom(Arc::new(NoRentaikaMatcher))),
        optional(super::noun()),
    ]
}

// Pattern: たところで (even if, even though)
// Matches: Verb (連用形/連用タ接続) + た/だ (past auxiliary) + ところ (名詞/非自立) + で (格助詞)
pub fn tatokorode() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct TokoroMatcher;
    impl super::Matcher for TokoroMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ところ"
                    && token.base_form == "ところ"
                    && token.pos.first().is_some_and(|p| p == "名詞")
                    && token.pos.get(1).is_some_and(|p| p == "非自立")
            })
        }
    }

    #[derive(Debug)]
    struct DeParticleMatcher;
    impl super::Matcher for DeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "で"
                    && token.base_form == "で"
                    && token.pos.first().is_some_and(|p| p == "助詞")
                    && token.pos.get(1).is_some_and(|p| p == "格助詞")
            })
        }
    }

    vec![
        super::flexible_verb_form(),
        super::past_auxiliary(),
        TokenMatcher::Custom(Arc::new(TokoroMatcher)),
        TokenMatcher::Custom(Arc::new(DeParticleMatcher)),
    ]
}

// Pattern: 如く・如き・如し (like, as if, similar to)
// Structures:
//   - (Noun/Verb/Adj) + の + ごとし/ごとく/ごとき
//   - Noun + ごとき (direct, without の)
//   - (Verb/Auxiliary) + が + ごとし/ごとく/ごとき (classical)
pub fn gotoku_u30fb_shiki_u30fb_gotoshi() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for の (連体化 particle)
    #[derive(Debug)]
    struct NoRentaikaMatcher;
    impl super::Matcher for NoRentaikaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "の"
                    && token.pos.first().is_some_and(|p| p == "助詞")
                    && token.pos.get(1).is_some_and(|p| p == "連体化")
            })
        }
    }

    // Matcher for が (接続助詞 - conjunctive particle for classical usage)
    #[derive(Debug)]
    struct GaSetsuzokuMatcher;
    impl super::Matcher for GaSetsuzokuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "が"
                    && token.pos.first().is_some_and(|p| p == "助詞")
                    && token.pos.get(1).is_some_and(|p| p == "接続助詞")
            })
        }
    }

    // Matcher for ごとし/ごとく/ごとき (all forms of classical auxiliary)
    #[derive(Debug)]
    struct GotoshiMatcher;
    impl super::Matcher for GotoshiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.base_form == "ごとし"
                    && token.pos.first().is_some_and(|p| p == "助動詞")
                    && (token.surface == "ごとし"
                        || token.surface == "ごとく"
                        || token.surface == "ごとき")
            })
        }
    }

    // Pattern: (Any) + (の or が or nothing) + ごとし/ごとく/ごとき
    // - の(連体化) for most cases
    // - が(接続助詞) for classical verb/auxiliary usage
    // - nothing (direct) for ごとき after nouns (体言接続)
    vec![
        any(),
        optional(TokenMatcher::Custom(Arc::new(NoRentaikaMatcher))),
        optional(TokenMatcher::Custom(Arc::new(GaSetsuzokuMatcher))),
        TokenMatcher::Custom(Arc::new(GotoshiMatcher)),
    ]
}

// Pattern: に足る (worthy of, enough for)
// Structures: (Verb[る] or Noun) + に + 足る + Noun
pub fn nitaru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match に as case particle
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "に"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match 足る verb in 基本形
    #[derive(Debug)]
    struct TaruMatcher;
    impl super::Matcher for TaruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.base_form == "足る"
                        && token.pos.first().is_some_and(|pos| pos == "動詞")
                        && token.features.get(5).is_some_and(|f| f == "基本形") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match verb in 基本形 or noun
    #[derive(Debug)]
    struct VerbOrNounMatcher;
    impl super::Matcher for VerbOrNounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                let is_verb = token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token.features.get(5).is_some_and(|f| f == "基本形");
                let is_noun = token.pos.first().is_some_and(|pos| pos == "名詞");
                is_verb || is_noun
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbOrNounMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(TaruMatcher)),
        noun(),
    ]
}

// Pattern: 極まりない・極まる (extremely)
// Structures:
// - な-Adj + (な) + (こと) + 極まりない
// - な-Adj + 極まる
// - い-Adj + こと + 極まりない
pub fn kiwamarinai_u30fb_kiwamaru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match な particle from だ (助動詞, 特殊・ダ, 体言接続)
    #[derive(Debug)]
    struct NaParticleMatcher;
    impl super::Matcher for NaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "な"
                        && token.pos.get(0).is_some_and(|p| p == "助動詞")
                        && token.features.get(4).is_some_and(|f| f == "特殊・ダ")
                        && token.features.get(5).is_some_and(|f| f == "体言接続") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match こと (名詞/非自立/一般)
    #[derive(Debug)]
    struct KotoMatcher;
    impl super::Matcher for KotoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "こと"
                        && token.pos.get(0).is_some_and(|p| p == "名詞")
                        && token.pos.get(1).is_some_and(|p| p == "非自立") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match 極まりない or 極まる
    let kiwama_matcher = or(vec![
        TokenMatcher::Custom(Arc::new({
            #[derive(Debug)]
            struct KiwamarinaiMatcher;
            impl super::Matcher for KiwamarinaiMatcher {
                fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
                    check_token(ctx, |token| {
                        token.base_form == "極まりない"
                            && token.pos.get(0).is_some_and(|p| p == "形容詞")
                    })
                }
            }
            KiwamarinaiMatcher
        })),
        TokenMatcher::Custom(Arc::new({
            #[derive(Debug)]
            struct KiwamaruMatcher;
            impl super::Matcher for KiwamaruMatcher {
                fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
                    check_token(ctx, |token| {
                        token.base_form == "極まる" && token.pos.get(0).is_some_and(|p| p == "動詞")
                    })
                }
            }
            KiwamaruMatcher
        })),
    ]);

    vec![
        adjective(),
        optional(TokenMatcher::Custom(Arc::new(NaParticleMatcher))),
        optional(TokenMatcher::Custom(Arc::new(KotoMatcher))),
        kiwama_matcher,
    ]
}

// Pattern: といえども (even if, although)
// Structures: Verb/Noun/Adj + と + いえ + ども
pub fn toiedomo() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match と (助詞/格助詞/引用)
    #[derive(Debug)]
    struct ToQuoteMatcher;
    impl super::Matcher for ToQuoteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "と"
                        && token.pos.get(0).is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "格助詞")
                        && token.pos.get(2).is_some_and(|p| p == "引用") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match いえ (動詞, 仮定形, base=いう)
    #[derive(Debug)]
    struct IeMatcher;
    impl super::Matcher for IeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "いえ"
                        && token.base_form == "いう"
                        && token.pos.get(0).is_some_and(|p| p == "動詞")
                        && token.features.get(5).is_some_and(|f| f == "仮定形") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match ども (助詞/接続助詞)
    #[derive(Debug)]
    struct DomoMatcher;
    impl super::Matcher for DomoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ども"
                        && token.pos.get(0).is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "接続助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(ToQuoteMatcher)),
        TokenMatcher::Custom(Arc::new(IeMatcher)),
        TokenMatcher::Custom(Arc::new(DomoMatcher)),
    ]
}

// Pattern: を以て (by means of, with) - compound particle
// Matches: をもって as a single compound particle (助詞/格助詞/連語)
pub fn womotte() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct WomotteMatcher;
    impl super::Matcher for WomotteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "をもって"
                        && token.base_form == "をもって"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "格助詞")
                        && token.pos.get(2).is_some_and(|p| p == "連語") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }
    vec![
        super::noun(),
        TokenMatcher::Custom(Arc::new(WomotteMatcher)),
    ]
}

// Pattern: を以て (by means of, with) - split tokenization
// Matches: Noun + を + もつ (verb) + て
pub fn womotte_split() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct WoParticleMatcher;
    impl super::Matcher for WoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "を"
                        && token.base_form == "を"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct TeParticleMatcher;
    impl super::Matcher for TeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "て"
                        && token.base_form == "て"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "接続助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        super::noun(),
        TokenMatcher::Custom(Arc::new(WoParticleMatcher)),
        verb_base("もつ"),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
    ]
}

// Pattern: きらいがある (tends to, has a tendency to)
// Structures: Verb/Noun + きらい + が + ある/あります
pub fn kiraigaaru() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KiraiMatcher;
    impl super::Matcher for KiraiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "きらい"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "非自立")
            })
        }
    }

    #[derive(Debug)]
    struct GaKakuMatcher;
    impl super::Matcher for GaKakuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "が"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            })
        }
    }

    #[derive(Debug)]
    struct AruAriMatcher;
    impl super::Matcher for AruAriMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.base_form == "ある"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                    && (token.surface == "ある" || token.surface == "あり")
            })
        }
    }

    #[derive(Debug)]
    struct MasuMatcher;
    impl super::Matcher for MasuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ます" && token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(KiraiMatcher)),
        TokenMatcher::Custom(Arc::new(GaKakuMatcher)),
        TokenMatcher::Custom(Arc::new(AruAriMatcher)),
        optional(TokenMatcher::Custom(Arc::new(MasuMatcher))),
    ]
}

// Pattern: ならまだしも (if A, that's fine, but B)
// Matches: なら (助動詞/仮定形) + まだしも (副詞)
// Note: Pattern range will include preceding token automatically
pub fn naramadashimo() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct NaraMatcher;
    impl super::Matcher for NaraMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "なら"
                    && token.base_form == "だ"
                    && token.pos.first().is_some_and(|p| p == "助動詞")
                    && token.features.get(5).is_some_and(|f| f == "仮定形")
            })
        }
    }

    #[derive(Debug)]
    struct MadashimoMatcher;
    impl super::Matcher for MadashimoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "まだしも"
                    && token.base_form == "まだしも"
                    && token.pos.first().is_some_and(|p| p == "副詞")
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NaraMatcher)),
        TokenMatcher::Custom(Arc::new(MadashimoMatcher)),
    ]
}

// Pattern: までもない (no need to, not necessary)
// Structures: Verb[る] + まで + も + ない/なく/なくて/ありません
pub fn mademonai() -> Vec<TokenMatcher> {
    // Match まで as 助詞/副助詞
    #[derive(Debug)]
    struct MadeMatcher;
    impl super::Matcher for MadeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "まで"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "副助詞")
            })
        }
    }

    // Match も as 助詞/係助詞
    #[derive(Debug)]
    struct MoKakariMatcher;
    impl super::Matcher for MoKakariMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "も"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
            })
        }
    }

    // Match ない (形容詞, 基本形), なく (形容詞, 連用テ接続), OR あり (動詞)
    #[derive(Debug)]
    struct NaiNakuAriMatcher;
    impl super::Matcher for NaiNakuAriMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match ない or なく
                if token.base_form == "ない"
                    && token.pos.first().is_some_and(|pos| pos == "形容詞")
                    && (token.surface == "ない" || token.surface == "なく")
                {
                    return true;
                }
                // Match あり (for polite form)
                if token.surface == "あり"
                    && token.base_form == "ある"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                {
                    return true;
                }
                false
            })
        }
    }

    // Match て (for なくて form)
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl super::Matcher for TeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "て" && token.pos.first().is_some_and(|pos| pos == "助詞")
            })
        }
    }

    // Match ませ (for polite form)
    #[derive(Debug)]
    struct MaseMatcher;
    impl super::Matcher for MaseMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ませ"
                    && token.base_form == "ます"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    // Match ん (for polite form)
    #[derive(Debug)]
    struct NMatcher;
    impl super::Matcher for NMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ん" && token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    vec![
        verb_form("基本形"),
        TokenMatcher::Custom(Arc::new(MadeMatcher)),
        TokenMatcher::Custom(Arc::new(MoKakariMatcher)),
        TokenMatcher::Custom(Arc::new(NaiNakuAriMatcher)),
        // Optional て, ませ, ん to handle various endings
        optional(TokenMatcher::Custom(Arc::new(TeParticleMatcher))),
        optional(TokenMatcher::Custom(Arc::new(MaseMatcher))),
        optional(TokenMatcher::Custom(Arc::new(NMatcher))),
    ]
}

// Pattern: ともなると・にもなると (when it comes to, once)
// Structures: Noun/Verb + と/に + (も) + なる(と/ば)
pub fn tomonaruto_u30fb_nimonaruto() -> Vec<TokenMatcher> {
    // Match と or に as quotation/general case particle
    #[derive(Debug)]
    struct ToNiMatcher;
    impl super::Matcher for ToNiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.surface == "と" || token.surface == "に")
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            })
        }
    }

    // Match optional も (係助詞)
    #[derive(Debug)]
    struct MoKakariMatcher;
    impl super::Matcher for MoKakariMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "も"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
            })
        }
    }

    // Match なる (基本形) or なれ (仮定形)
    #[derive(Debug)]
    struct NaruNareMatcher;
    impl super::Matcher for NaruNareMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.base_form == "なる"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                    && ((token.surface == "なる"
                        && token.features.get(5).is_some_and(|f| f == "基本形"))
                        || (token.surface == "なれ"
                            && token.features.get(5).is_some_and(|f| f == "仮定形")))
            })
        }
    }

    // Match と (接続助詞) or ば (接続助詞)
    #[derive(Debug)]
    struct ToBaMatcher;
    impl super::Matcher for ToBaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.surface == "と" || token.surface == "ば")
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
            })
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(ToNiMatcher)),
        optional(TokenMatcher::Custom(Arc::new(MoKakariMatcher))),
        TokenMatcher::Custom(Arc::new(NaruNareMatcher)),
        TokenMatcher::Custom(Arc::new(ToBaMatcher)),
    ]
}

// Pattern: をいいことに (take advantage of)
// Structures: (の/なの) + を + いい + こと + に + (して)
pub fn woiikotoni() -> Vec<TokenMatcher> {
    // Match の as nominalizer (名詞/非自立/一般)
    #[derive(Debug)]
    struct NoNominalizerMatcher;
    impl super::Matcher for NoNominalizerMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "の"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "非自立")
            })
        }
    }

    // Match な from だ (助動詞, 特殊・ダ, 体言接続)
    #[derive(Debug)]
    struct NaFromDaMatcher;
    impl super::Matcher for NaFromDaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "な"
                    && token.base_form == "だ"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    // Match を (格助詞)
    #[derive(Debug)]
    struct WoMatcher;
    impl super::Matcher for WoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "を"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            })
        }
    }

    // Match いい (形容詞, 基本形)
    #[derive(Debug)]
    struct IiMatcher;
    impl super::Matcher for IiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "いい"
                    && token.base_form == "いい"
                    && token.pos.first().is_some_and(|pos| pos == "形容詞")
                    && token.features.get(5).is_some_and(|f| f == "基本形")
            })
        }
    }

    // Match こと (名詞/非自立/一般)
    #[derive(Debug)]
    struct KotoMatcher;
    impl super::Matcher for KotoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "こと"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "非自立")
            })
        }
    }

    // Match に (格助詞)
    #[derive(Debug)]
    struct NiMatcher;
    impl super::Matcher for NiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "に"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            })
        }
    }

    // Match し from する (動詞, 連用形)
    #[derive(Debug)]
    struct ShiMatcher;
    impl super::Matcher for ShiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "し"
                    && token.base_form == "する"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token.features.get(5).is_some_and(|f| f == "連用形")
            })
        }
    }

    // Match て (接続助詞)
    #[derive(Debug)]
    struct TeMatcher;
    impl super::Matcher for TeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "て"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
            })
        }
    }

    vec![
        any(), // Preceding verb/noun/adjective
        optional(TokenMatcher::Custom(Arc::new(NaFromDaMatcher))),
        TokenMatcher::Custom(Arc::new(NoNominalizerMatcher)),
        TokenMatcher::Custom(Arc::new(WoMatcher)),
        TokenMatcher::Custom(Arc::new(IiMatcher)),
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
        TokenMatcher::Custom(Arc::new(NiMatcher)),
        optional(TokenMatcher::Custom(Arc::new(ShiMatcher))),
        optional(TokenMatcher::Custom(Arc::new(TeMatcher))),
    ]
}

// Pattern: 如何 (いかん - depending on)
// Structures: Noun + (の) + いかん + で/だ/によって/である
pub fn ika_2() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match の particle (助詞/連体化)
    #[derive(Debug)]
    struct NoParticleMatcher;
    impl super::Matcher for NoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "の"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "連体化") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match いかん (名詞/一般 or 名詞/接尾)
    #[derive(Debug)]
    struct IkanMatcher;
    impl super::Matcher for IkanMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "いかん"
                        && token.pos.first().is_some_and(|pos| pos == "名詞")
                        && (token.pos.get(1).is_some_and(|pos| pos == "一般")
                            || token.pos.get(1).is_some_and(|pos| pos == "接尾")) =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match で (助詞/格助詞/一般 OR 助動詞/特殊・ダ/連用形)
    #[derive(Debug)]
    struct DeMatcher;
    impl super::Matcher for DeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "で"
                        && ((token.pos.first().is_some_and(|pos| pos == "助詞")
                            && token.pos.get(1).is_some_and(|pos| pos == "格助詞"))
                            || (token.pos.first().is_some_and(|pos| pos == "助動詞")
                                && token.base_form == "だ")) =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match だ (助動詞/特殊・ダ/基本形)
    #[derive(Debug)]
    struct DaMatcher;
    impl super::Matcher for DaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "だ"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.base_form == "だ"
                        && token.features.get(5).is_some_and(|f| f == "基本形") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match によって (助詞/格助詞/連語)
    #[derive(Debug)]
    struct NiyotteMatcher;
    impl super::Matcher for NiyotteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "によって"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                        && token.pos.get(2).is_some_and(|pos| pos == "連語") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match ある auxiliary (助動詞/五段・ラ行アル/基本形)
    #[derive(Debug)]
    struct AruAuxMatcher;
    impl super::Matcher for AruAuxMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ある"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.base_form == "ある" =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match は particle (助詞/係助詞) - optional after で or によって
    #[derive(Debug)]
    struct HaParticleMatcher;
    impl super::Matcher for HaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "は"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "係助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        optional(TokenMatcher::Custom(Arc::new(NoParticleMatcher))),
        TokenMatcher::Custom(Arc::new(IkanMatcher)),
        optional(TokenMatcher::Custom(Arc::new(DeMatcher))),
        optional(TokenMatcher::Custom(Arc::new(DaMatcher))),
        optional(TokenMatcher::Custom(Arc::new(NiyotteMatcher))),
        optional(TokenMatcher::Custom(Arc::new(AruAuxMatcher))),
        optional(TokenMatcher::Custom(Arc::new(HaParticleMatcher))),
    ]
}

// Pattern: ～るまでだ (merely, simply, one can only but)
// Structures: Verb[る] + まで + (の + こと)? + だ/です
pub fn uff5e_rumadeda() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match まで (助詞/副助詞)
    #[derive(Debug)]
    struct MadeParticleMatcher;
    impl super::Matcher for MadeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "まで"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "副助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match の (助詞/連体化)
    #[derive(Debug)]
    struct NoRentaikaMatcher;
    impl super::Matcher for NoRentaikaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "の"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "連体化") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match こと (名詞/非自立/一般)
    #[derive(Debug)]
    struct KotoNounMatcher;
    impl super::Matcher for KotoNounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "こと"
                        && token.pos.first().is_some_and(|pos| pos == "名詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "非自立") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match だ (助動詞, 特殊・ダ, 基本形) or です (助動詞, 特殊・デス, 基本形)
    #[derive(Debug)]
    struct DaDesuAuxMatcher;
    impl super::Matcher for DaDesuAuxMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.features.get(5).is_some_and(|form| form == "基本形")
                        && (token.base_form == "だ" || token.base_form == "です") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        verb_form("基本形"),
        TokenMatcher::Custom(Arc::new(MadeParticleMatcher)),
        // Optional: の + こと sequence
        optional(TokenMatcher::Custom(Arc::new(NoRentaikaMatcher))),
        optional(TokenMatcher::Custom(Arc::new(KotoNounMatcher))),
        TokenMatcher::Custom(Arc::new(DaDesuAuxMatcher)),
    ]
}

// Pattern: にあって (in, at, under the conditions of)
// Structures: Noun + に + あって + (も)?
pub fn niatte() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match に particle (助詞/格助詞/一般)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "に"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match あっ (動詞, base=ある, 連用タ接続)
    // Important: This is ある (to be), NOT 遭う (to meet/encounter)
    #[derive(Debug)]
    struct AtteMatcher;
    impl super::Matcher for AtteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "あっ"
                        && token.pos.first().is_some_and(|pos| pos == "動詞")
                        && token.base_form == "ある"
                        && token
                            .features
                            .get(5)
                            .is_some_and(|form| form == "連用タ接続") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match て particle (助詞/接続助詞)
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl super::Matcher for TeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "て"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match も particle (助詞/係助詞)
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl super::Matcher for MoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "も"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "係助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        super::noun(),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(AtteMatcher)),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
        optional(TokenMatcher::Custom(Arc::new(MoParticleMatcher))),
    ]
}

// Pattern: を余儀なくされる (to be forced to)
// Structure: Noun + を + よぎなく + さ + れ + た/ます
pub fn woyoginakusareru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match よぎなく (形容詞/自立, base=よぎない, 連用テ接続)
    #[derive(Debug)]
    struct YoginakuMatcher;
    impl super::Matcher for YoginakuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "よぎなく"
                        && token.base_form == "よぎない"
                        && token.pos.first().is_some_and(|pos| pos == "形容詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match さ from する (動詞/自立, base=する, サ変・スル/未然レル接続)
    #[derive(Debug)]
    struct SasuruMatcher;
    impl super::Matcher for SasuruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "さ"
                        && token.base_form == "する"
                        && token.pos.first().is_some_and(|pos| pos == "動詞")
                        && token.features.get(4).is_some_and(|f| f.contains("サ変"))
                        && token.features.get(5).is_some_and(|f| f == "未然レル接続") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match た or ます (助動詞)
    #[derive(Debug)]
    struct TaMasuMatcher;
    impl super::Matcher for TaMasuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if (token.surface == "た" || token.surface == "ます")
                        && token.pos.first().is_some_and(|pos| pos == "助動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match を (格助詞)
    #[derive(Debug)]
    struct WoParticleMatcher;
    impl super::Matcher for WoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "を"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        super::noun(),
        TokenMatcher::Custom(Arc::new(WoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(YoginakuMatcher)),
        TokenMatcher::Custom(Arc::new(SasuruMatcher)),
        reru_suffix(),
        TokenMatcher::Custom(Arc::new(TaMasuMatcher)),
    ]
}

// Pattern: とは (emphatic exclamation expressing shock/surprise)
// Structures: Verb/Noun/Adj + (など) + とは
// Note: Structurally identical to というのは_abbreviated (N3), but used for emphatic exclamation
// rather than definition/explanation. Both patterns will be detected when と+は appears.
pub fn toha() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match と as quotation particle
    #[derive(Debug)]
    struct ToQuotationMatcher;
    impl super::Matcher for ToQuotationMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "と"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                        && token.pos.get(2).is_some_and(|pos| pos == "引用") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match は (topic particle)
    #[derive(Debug)]
    struct HaTopicMatcher;
    impl super::Matcher for HaTopicMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "は"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "係助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match any preceding token (verb/noun/adjective)
    // Optional など can appear before とは but we'll keep the matcher simple
    vec![
        any(),
        TokenMatcher::Custom(Arc::new(ToQuotationMatcher)),
        TokenMatcher::Custom(Arc::new(HaTopicMatcher)),
    ]
}

// Pattern: じゃあるまいし (it's not like, you're not)
// Structures: Noun/ん/わけ + じゃ/では + ある + まい + し
pub fn jaarumaishi() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match じゃ (助詞/副助詞) OR で (助詞/格助詞 OR 助動詞)
    #[derive(Debug)]
    struct JaOrDeMatcher;
    impl super::Matcher for JaOrDeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                if token.surface == "じゃ" {
                    token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "副助詞")
                } else if token.surface == "で" {
                    (token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "格助詞"))
                        || (token.pos.first().is_some_and(|p| p == "助動詞")
                            && token.base_form == "だ")
                } else {
                    false
                }
            })
        }
    }

    // Match は (助詞/係助詞) - optional, only after で
    #[derive(Debug)]
    struct HaTopicParticleMatcher;
    impl super::Matcher for HaTopicParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "は"
                    && token.pos.first().is_some_and(|p| p == "助詞")
                    && token.pos.get(1).is_some_and(|p| p == "係助詞")
            })
        }
    }

    // Match ある (助動詞, 五段・ラ行アル, 基本形)
    #[derive(Debug)]
    struct AruAuxiliaryMatcher;
    impl super::Matcher for AruAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ある"
                    && token.pos.first().is_some_and(|p| p == "助動詞")
                    && token.base_form == "ある"
            })
        }
    }

    // Match まい (助動詞, 不変化型, 基本形)
    #[derive(Debug)]
    struct MaiAuxiliaryMatcher;
    impl super::Matcher for MaiAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "まい"
                    && token.pos.first().is_some_and(|p| p == "助動詞")
                    && token.base_form == "まい"
            })
        }
    }

    // Match し (助詞/接続助詞)
    #[derive(Debug)]
    struct ShiConjunctionMatcher;
    impl super::Matcher for ShiConjunctionMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "し"
                    && token.pos.first().is_some_and(|p| p == "助詞")
                    && token.pos.get(1).is_some_and(|p| p == "接続助詞")
            })
        }
    }

    // Pattern: (じゃ OR で) + optional は + ある + まい + し
    vec![
        any(), // Noun/ん/わけ - we match any preceding token
        TokenMatcher::Custom(Arc::new(JaOrDeMatcher)),
        optional(TokenMatcher::Custom(Arc::new(HaTopicParticleMatcher))),
        TokenMatcher::Custom(Arc::new(AruAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(MaiAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(ShiConjunctionMatcher)),
    ]
}

// Pattern: てからというもの (ever since)
// Structures: Verb[て] + から + というもの OR それから + というもの
pub fn tekaratoiumono() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match て (助詞/接続助詞) OR それから (接続詞)
    #[derive(Debug)]
    struct TeOrSorekaraMatcher;
    impl super::Matcher for TeOrSorekaraMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                if token.surface == "て" {
                    token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "接続助詞")
                } else if token.surface == "それから" {
                    token.pos.first().is_some_and(|p| p == "接続詞")
                } else {
                    false
                }
            })
        }
    }

    // Match から (助詞/格助詞/一般) - only after て, not after それから
    #[derive(Debug)]
    struct KaraParticleMatcher;
    impl super::Matcher for KaraParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "から"
                    && token.pos.first().is_some_and(|p| p == "助詞")
                    && token.pos.get(1).is_some_and(|p| p == "格助詞")
            })
        }
    }

    // Match という (助詞/格助詞/連語)
    #[derive(Debug)]
    struct ToiuMatcher;
    impl super::Matcher for ToiuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "という"
                    && token.pos.first().is_some_and(|p| p == "助詞")
                    && token.pos.get(1).is_some_and(|p| p == "格助詞")
                    && token.pos.get(2).is_some_and(|p| p == "連語")
            })
        }
    }

    // Match もの (名詞/非自立/一般)
    #[derive(Debug)]
    struct MonoMatcher;
    impl super::Matcher for MonoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "もの"
                    && token.pos.first().is_some_and(|p| p == "名詞")
                    && token.pos.get(1).is_some_and(|p| p == "非自立")
            })
        }
    }

    // Pattern: (て OR それから) + optional から + という + もの
    // When it's て, から is required; when it's それから, から is already included
    vec![
        TokenMatcher::Custom(Arc::new(TeOrSorekaraMatcher)),
        optional(TokenMatcher::Custom(Arc::new(KaraParticleMatcher))),
        TokenMatcher::Custom(Arc::new(ToiuMatcher)),
        TokenMatcher::Custom(Arc::new(MonoMatcher)),
    ]
}

// Pattern: かたわら (besides, in addition to, while)
// Structures: Verb[る] + かたわら / Noun + の + かたわら
pub fn katawara() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match かたわら (名詞/副詞可能)
    #[derive(Debug)]
    struct KatawaraMatcher;
    impl super::Matcher for KatawaraMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "かたわら"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "副詞可能")
            })
        }
    }

    // Match verb in dictionary form (基本形) OR noun
    #[derive(Debug)]
    struct VerbOrNounMatcher;
    impl super::Matcher for VerbOrNounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match verb in dictionary form
                if token.pos.first().is_some_and(|pos| pos == "動詞") {
                    return token.features.get(5).is_some_and(|form| form == "基本形");
                }
                // Or match any noun
                token.pos.first().is_some_and(|pos| pos == "名詞")
            })
        }
    }

    // Match の particle (助詞/連体化)
    #[derive(Debug)]
    struct NoParticleMatcher;
    impl super::Matcher for NoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "の"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "連体化")
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbOrNounMatcher)),
        optional(TokenMatcher::Custom(Arc::new(NoParticleMatcher))),
        TokenMatcher::Custom(Arc::new(KatawaraMatcher)),
    ]
}

// Pattern: を皮切りに (starting with, beginning with)
// Structures: Noun/の + を + 皮切り + に/として/にして
pub fn wokawakirini() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match を particle (助詞/格助詞/一般)
    #[derive(Debug)]
    struct WoParticleMatcher;
    impl super::Matcher for WoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "を"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            })
        }
    }

    // Match 皮切り (名詞/一般)
    #[derive(Debug)]
    struct KawakiriMatcher;
    impl super::Matcher for KawakiriMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "皮切り"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "一般")
            })
        }
    }

    // Match に particle OR として (助詞/格助詞)
    #[derive(Debug)]
    struct NiOrToshiteMatcher;
    impl super::Matcher for NiOrToshiteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                if token.surface == "に" {
                    return token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞");
                }
                if token.surface == "として" {
                    return token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                        && token.pos.get(2).is_some_and(|pos| pos == "連語");
                }
                false
            })
        }
    }

    // Match optional にして (に + し + て sequence)
    // This is handled by making the following tokens optional:
    // し (連用形 of する) + て (接続助詞)

    vec![
        TokenMatcher::Custom(Arc::new(WoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(KawakiriMatcher)),
        TokenMatcher::Custom(Arc::new(NiOrToshiteMatcher)),
    ]
}

// Pattern: に至っては (when it comes to, as for)
// Structure: Noun + に + いたっ + て + は
pub fn niitatteha() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match に (格助詞)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "に"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            })
        }
    }

    // Match いたっ (動詞/自立, base=いたる, 連用タ接続)
    #[derive(Debug)]
    struct ItattaMatcher;
    impl super::Matcher for ItattaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "いたっ"
                    && token.base_form == "いたる"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
            })
        }
    }

    // Match て (接続助詞)
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl super::Matcher for TeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "て"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
            })
        }
    }

    // Match は (係助詞)
    #[derive(Debug)]
    struct WaParticleMatcher;
    impl super::Matcher for WaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "は"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
            })
        }
    }

    vec![
        super::noun(),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(ItattaMatcher)),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(WaParticleMatcher)),
    ]
}

// Pattern: なり (as soon as, the moment)
// Structures: Verb[る] + なり
pub fn nari() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match なり as conjunction particle (助詞/接続助詞)
    #[derive(Debug)]
    struct NariParticleMatcher;
    impl super::Matcher for NariParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "なり"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        verb_form("基本形"), // Dictionary form verb
        TokenMatcher::Custom(Arc::new(NariParticleMatcher)),
    ]
}

// Pattern: ともなく・ともなしに (absentmindedly, without paying attention)
// Structures: Verb[る] + ともなく, Verb[る] + ともなしに
pub fn tomonaku_u30fb_tomonashini() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match とも (助詞/接続助詞) OR と (助詞/格助詞/引用)
    #[derive(Debug)]
    struct ToOrTomoMatcher;
    impl super::Matcher for ToOrTomoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if (token.surface == "とも"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "接続助詞"))
                        || (token.surface == "と"
                            && token.pos.first().is_some_and(|pos| pos == "助詞")
                            && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                            && token.pos.get(2).is_some_and(|pos| pos == "引用")) =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match も (助詞/係助詞) - optional, only for ともなしに variant
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl super::Matcher for MoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "も"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "係助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match なく (形容詞/自立, 連用テ接続, base=ない) OR なし (形容詞/自立, 文語基本形, base=ない)
    #[derive(Debug)]
    struct NakuOrNashiMatcher;
    impl super::Matcher for NakuOrNashiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if (token.surface == "なく" || token.surface == "なし")
                        && token.base_form == "ない"
                        && token.pos.first().is_some_and(|pos| pos == "形容詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "自立") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match に particle (助詞/格助詞/一般) - optional, only for ともなしに
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "に"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Pattern 1: Verb + とも + なく (3 tokens)
    // Pattern 2: Verb + と + も + なし + に (5 tokens)
    // Use optional matchers for も and に to handle both variants
    vec![
        verb_form("基本形"), // Dictionary form verb
        TokenMatcher::Custom(Arc::new(ToOrTomoMatcher)),
        optional(TokenMatcher::Custom(Arc::new(MoParticleMatcher))),
        TokenMatcher::Custom(Arc::new(NakuOrNashiMatcher)),
        optional(TokenMatcher::Custom(Arc::new(NiParticleMatcher))),
    ]
}

// Pattern: 塗れ (まみれ - completely covered in, smeared all over with)
// Structures: Noun + まみれ
//
// Kagome tokenization varies:
// 1. Compound forms: 泥まみれ (名詞/一般), 血まみれ (名詞/形容動詞語幹) - single token
// 2. Split form: 借金 (名詞) + まみれ (名詞/接尾/一般) - two tokens
//
// We need TWO patterns to handle both cases:
// - nure_compound: Nouns with base_form ending in まみれ (compound words)
// - nure: Noun + まみれ suffix (split form)

// Match compound まみれ words (泥まみれ, 血まみれ, etc.)
pub fn nure_compound() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct MamireCompoundMatcher;
    impl super::Matcher for MamireCompoundMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.pos.first().is_some_and(|pos| pos == "名詞")
                        && token.base_form.ends_with("まみれ")
                        && token.base_form != "まみれ" =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![TokenMatcher::Custom(Arc::new(MamireCompoundMatcher))]
}

// Match split まみれ forms (Noun + まみれ suffix)
pub fn nure() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match まみれ as a noun suffix (名詞/接尾/一般)
    #[derive(Debug)]
    struct MamireSuffixMatcher;
    impl super::Matcher for MamireSuffixMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "まみれ"
                        && token.pos.first().is_some_and(|pos| pos == "名詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "接尾") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        super::noun(),
        TokenMatcher::Custom(Arc::new(MamireSuffixMatcher)),
    ]
}

// Pattern: ようが～まいが (whether or not)
// Structures: Verb[volitional] + が + Verb[まい] + が
//
// Main pattern: Verb + う/よう + が + (same verb) + まい + が
// The pattern can also use antonyms with adjectives/nouns:
// - Adj[かろう] + が + Adj[かろう] + が
// - Noun + だろう + が + Noun + だろう + が
//
// For now, we implement the main verb pattern with Verb + う/よう + が + Verb + まい + が
pub fn youga_uff5e_maiga() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match volitional auxiliary う or よう
    #[derive(Debug)]
    struct VolitionalAuxMatcher;
    impl super::Matcher for VolitionalAuxMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if (token.surface == "う" || token.surface == "よう")
                        && token.pos.first().is_some_and(|pos| pos == "助動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match が as conjunction particle
    #[derive(Debug)]
    struct GaConjunctionMatcher;
    impl super::Matcher for GaConjunctionMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "が"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match まい auxiliary verb
    #[derive(Debug)]
    struct MaiAuxiliaryMatcher;
    impl super::Matcher for MaiAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "まい"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        // First part: Verb(未然ウ接続) + う/よう + が
        any(), // Verb in 未然ウ接続 form
        TokenMatcher::Custom(Arc::new(VolitionalAuxMatcher)),
        TokenMatcher::Custom(Arc::new(GaConjunctionMatcher)),
        // Wildcard to allow different verb or same verb
        wildcard(0, 3, vec![]),
        // Second part: Verb(基本形) + まい + が
        any(), // Verb in dictionary form
        TokenMatcher::Custom(Arc::new(MaiAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(GaConjunctionMatcher)),
    ]
}

// Pattern: からする (about X, X or more, starting at X)
// Structures: Number + Counter + からする/からします
pub fn karasuru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match number tokens (including counters)
    #[derive(Debug)]
    struct NumberOrCounterMatcher;
    impl super::Matcher for NumberOrCounterMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match 名詞/数 or 名詞/接尾/助数詞
                if let Some(pos1) = token.pos.first() {
                    if pos1 == "名詞" {
                        if let Some(pos2) = token.pos.get(1) {
                            return pos2 == "数"
                                || (pos2 == "接尾"
                                    && token.pos.get(2).is_some_and(|p| p == "助数詞"));
                        }
                    }
                }
                false
            })
        }
    }

    // Match から particle (格助詞)
    #[derive(Debug)]
    struct KaraParticleMatcher;
    impl super::Matcher for KaraParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "から"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        // Match 1+ number/counter tokens
        TokenMatcher::Custom(Arc::new(NumberOrCounterMatcher)),
        wildcard(0, 5, vec![]),
        // Match から
        TokenMatcher::Custom(Arc::new(KaraParticleMatcher)),
        // Match する verb
        verb_base("する"),
    ]
}

// Pattern: からの (about X, X or more, starting at X) - の variant
// Structures: Number + Counter + からの + Noun
pub fn karano() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match number tokens (including counters)
    #[derive(Debug)]
    struct NumberOrCounterMatcher;
    impl super::Matcher for NumberOrCounterMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match 名詞/数 or 名詞/接尾/助数詞
                if let Some(pos1) = token.pos.first() {
                    if pos1 == "名詞" {
                        if let Some(pos2) = token.pos.get(1) {
                            return pos2 == "数"
                                || (pos2 == "接尾"
                                    && token.pos.get(2).is_some_and(|p| p == "助数詞"));
                        }
                    }
                }
                false
            })
        }
    }

    // Match から particle (格助詞)
    #[derive(Debug)]
    struct KaraParticleMatcher;
    impl super::Matcher for KaraParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "から"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match の particle (for からの)
    #[derive(Debug)]
    struct NoParticleMatcher;
    impl super::Matcher for NoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "の"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "連体化") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        // Match 1+ number/counter tokens
        TokenMatcher::Custom(Arc::new(NumberOrCounterMatcher)),
        wildcard(0, 5, vec![]),
        // Match から
        TokenMatcher::Custom(Arc::new(KaraParticleMatcher)),
        // Match の particle
        TokenMatcher::Custom(Arc::new(NoParticleMatcher)),
    ]
}

// Pattern: にして① (at (A), over (A), only when (A))
// Structures: Number + Counter + にして, Noun + にして
pub fn nishite_u2460() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match に particle (格助詞)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "に"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match し (する in 連用形)
    #[derive(Debug)]
    struct ShiVerbMatcher;
    impl super::Matcher for ShiVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "し"
                        && token.base_form == "する"
                        && token.pos.first().is_some_and(|p| p == "動詞")
                        && token.features.get(5).is_some_and(|f| f == "連用形") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match て particle (接続助詞)
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl super::Matcher for TeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "て"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "接続助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        // Match number/counter or noun - use specific noun types to avoid matching across particles
        noun(),
        // Optional: allow one more noun token (for multi-token numbers like ３０)
        optional(noun()),
        // Optional: allow one more noun token (for counters like 歳, or suffixes like 目)
        optional(noun()),
        // Match に
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        // Match し
        TokenMatcher::Custom(Arc::new(ShiVerbMatcher)),
        // Match て
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
    ]
}

// Pattern: ものを (if only... but)
// Structures: Verb/Adjective + ものを
pub fn monowo() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match もの as a non-independent noun
    #[derive(Debug)]
    struct MonoMatcher;
    impl super::Matcher for MonoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "もの"
                        && token.pos.first().is_some_and(|pos| pos == "名詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "非自立") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match を particle
    #[derive(Debug)]
    struct WoParticleMatcher;
    impl super::Matcher for WoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "を"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Could be verb, adjective, or auxiliary (な)
        TokenMatcher::Custom(Arc::new(MonoMatcher)),
        TokenMatcher::Custom(Arc::new(WoParticleMatcher)),
    ]
}

// Pattern: であれ (even if)
// Structures: Noun/な-Adjective/WH-Word + であれ
pub fn deare() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match で (auxiliary verb だ in 連用形)
    #[derive(Debug)]
    struct DeAuxiliaryMatcher;
    impl super::Matcher for DeAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "で"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.base_form == "だ" =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match あれ (auxiliary verb ある in 命令ｅ form)
    #[derive(Debug)]
    struct AreImperativeMatcher;
    impl super::Matcher for AreImperativeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "あれ"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.base_form == "ある"
                        && token.features.get(5).is_some_and(|f| f.contains("命令")) =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Noun, な-Adjective stem, or question word
        TokenMatcher::Custom(Arc::new(DeAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(AreImperativeMatcher)),
    ]
}

// Pattern: をおいてほかに〜ない (none other than, nothing else but)
// Structures: Noun + をおいて + ほか + に(は) + ... + ない
pub fn wooitehokani_u301c_nai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match をおく verb in 連用タ接続 form
    #[derive(Debug)]
    struct OkuVerbMatcher;
    impl super::Matcher for OkuVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.base_form == "おく"
                        && token.pos.first().is_some_and(|pos| pos == "動詞")
                        && token
                            .features
                            .get(5)
                            .is_some_and(|form| form == "連用タ接続") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match て as conjunctive particle
    #[derive(Debug)]
    struct TeConjunctiveMatcher;
    impl super::Matcher for TeConjunctiveMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "て"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match ない (either 助動詞 or 形容詞)
    #[derive(Debug)]
    struct NaiNegativeMatcher;
    impl super::Matcher for NaiNegativeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ない"
                        && (token.pos.first().is_some_and(|pos| pos == "助動詞")
                            || token.pos.first().is_some_and(|pos| pos == "形容詞")) =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match ほか as noun
    #[derive(Debug)]
    struct HokaMatcher;
    impl super::Matcher for HokaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ほか"
                        && token.pos.first().is_some_and(|pos| pos == "名詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match は particle (optional)
    #[derive(Debug)]
    struct WaParticleMatcher;
    impl super::Matcher for WaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "は"
                        && token.pos.first().is_some_and(|pos| pos == "助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        noun(),
        surface("を"),
        TokenMatcher::Custom(Arc::new(OkuVerbMatcher)),
        TokenMatcher::Custom(Arc::new(TeConjunctiveMatcher)),
        TokenMatcher::Custom(Arc::new(HokaMatcher)),
        surface("に"),
        optional(TokenMatcher::Custom(Arc::new(WaParticleMatcher))),
        wildcard(0, 10, vec![]),
        TokenMatcher::Custom(Arc::new(NaiNegativeMatcher)),
    ]
}

// Pattern: をもって (as of, effective from - time expressions)
// Structures: Time expression + をもって/をもちまして
// Note: Same structure as を以て but different semantic usage (time vs means)
pub fn womotte_2() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match をもって or をもちまして as a compound particle
    #[derive(Debug)]
    struct WomotteTimeMatcher;
    impl super::Matcher for WomotteTimeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if (token.surface == "をもって" || token.surface == "をもちまして")
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "格助詞")
                        && token.pos.get(2).is_some_and(|p| p == "連語") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![noun(), TokenMatcher::Custom(Arc::new(WomotteTimeMatcher))]
}

// Pattern: とはいえ (although, be that as it may)
// Structures: Verb/Adj/Noun + と + は + いえ
pub fn tohaie() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match と as quotation particle (格助詞/引用)
    #[derive(Debug)]
    struct ToQuoteMatcher;
    impl super::Matcher for ToQuoteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "と"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match は as topic particle (係助詞)
    #[derive(Debug)]
    struct HaTopicMatcher;
    impl super::Matcher for HaTopicMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "は"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "係助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match いえ as verb いう in imperative form (命令ｅ)
    #[derive(Debug)]
    struct IeMatcher;
    impl super::Matcher for IeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "いえ"
                        && token.base_form == "いう"
                        && token.pos.first().is_some_and(|p| p == "動詞")
                        && token.features.get(5).is_some_and(|f| f.contains("命令")) =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(ToQuoteMatcher)),
        TokenMatcher::Custom(Arc::new(HaTopicMatcher)),
        TokenMatcher::Custom(Arc::new(IeMatcher)),
    ]
}

// Pattern: ならでは (unique to, impossible if not)
// Structures: Noun + ならでは + (の/だ/です/Verb)
pub fn naradeha() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ならでは as noun suffix
    #[derive(Debug)]
    struct NaradehaMatcher;
    impl super::Matcher for NaradehaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ならでは"
                        && token.base_form == "ならでは"
                        && token.pos.first().is_some_and(|p| p == "名詞")
                        && token.pos.get(1).is_some_and(|p| p == "接尾") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![noun(), TokenMatcher::Custom(Arc::new(NaradehaMatcher))]
}

// Pattern: すら
// Pattern: すら (even - extreme example)
// Structures: Noun + (Particle) + すら(も)
pub fn sura() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match すら as 係助詞
    #[derive(Debug)]
    struct SuraParticleMatcher;
    impl super::Matcher for SuraParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "すら"
                        && token.base_form == "すら"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "係助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match optional も after すら
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl super::Matcher for MoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "も"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "係助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match optional particle (に, で from だ, etc.)
    #[derive(Debug)]
    struct OptionalParticleMatcher;
    impl super::Matcher for OptionalParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match に as 格助詞
                if token.surface == "に"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                {
                    return true;
                }
                // Match で from だ as 助動詞
                if token.surface == "で"
                    && token.base_form == "だ"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                {
                    return true;
                }
                false
            })
        }
    }

    vec![
        noun(),                                                            // Noun
        optional(TokenMatcher::Custom(Arc::new(OptionalParticleMatcher))), // Optional particle
        TokenMatcher::Custom(Arc::new(SuraParticleMatcher)),
        optional(TokenMatcher::Custom(Arc::new(MoParticleMatcher))), // Optional も
    ]
}

// Pattern: あっての (B exists only because of A)
// Structures: Noun (A) + (が) + あっての + Noun (B)
pub fn atteno() -> Vec<TokenMatcher> {
    use super::Matcher;
    use std::sync::Arc;

    // Match が particle (optional)
    #[derive(Debug)]
    struct GaParticleMatcher;
    impl super::Matcher for GaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "が"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match ある verb in 連用タ接続 form (あっ)
    #[derive(Debug)]
    struct AruVerbMatcher;
    impl super::Matcher for AruVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.base_form == "ある"
                        && token.pos.first().is_some_and(|pos| pos == "動詞")
                        && token.features.get(5).is_some_and(|f| f == "連用タ接続") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match て as conjunctive particle
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl super::Matcher for TeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "て"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match の as nominalizing particle
    #[derive(Debug)]
    struct NoParticleMatcher;
    impl super::Matcher for NoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "の"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "連体化") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        super::noun(),
        optional(TokenMatcher::Custom(Arc::new(GaParticleMatcher))),
        TokenMatcher::Custom(Arc::new(AruVerbMatcher)),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(NoParticleMatcher)),
        super::noun(),
    ]
}

// Pattern: ～たまでだ (I simply/only did A)
// Structures: Verb[た] + まで + だ/です, Verb[た] + までのこと + だ/です
pub fn uff5e_tamadeda() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Helper: Match まで as 副助詞
    #[derive(Debug)]
    struct MadeParticleMatcher;
    impl super::Matcher for MadeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "まで"
                        && token.base_form == "まで"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "副助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Helper: Match の as 連体化 particle
    #[derive(Debug)]
    struct NoParticleMatcher;
    impl super::Matcher for NoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "の"
                        && token.base_form == "の"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "連体化") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Helper: Match こと as 非自立 noun
    #[derive(Debug)]
    struct KotoNounMatcher;
    impl super::Matcher for KotoNounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "こと"
                        && token.base_form == "こと"
                        && token.pos.first().is_some_and(|p| p == "名詞")
                        && token.pos.get(1).is_some_and(|p| p == "非自立") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Helper: Match だ or です as auxiliary verb
    #[derive(Debug)]
    struct DaDesuMatcher;
    impl super::Matcher for DaDesuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if (token.surface == "だ" || token.surface == "です")
                        && token.pos.first().is_some_and(|p| p == "助動詞")
                        && (token.base_form == "だ" || token.base_form == "です") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Pattern: Verb[連用形/連用タ接続] + た + まで + Optional(の + こと) + だ/です
    super::concat(vec![
        vec![super::flexible_verb_form()],
        vec![super::past_auxiliary()],
        vec![TokenMatcher::Custom(Arc::new(MadeParticleMatcher))],
        vec![
            optional(TokenMatcher::Custom(Arc::new(NoParticleMatcher))),
            optional(TokenMatcher::Custom(Arc::new(KotoNounMatcher))),
        ],
        vec![TokenMatcher::Custom(Arc::new(DaDesuMatcher))],
    ])
}

// Pattern: を経て (through, via, after undergoing)
// Structures: Noun + を + 経（へ）て
pub fn wohete() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match を particle (格助詞)
    #[derive(Debug)]
    struct WoParticleMatcher;
    impl super::Matcher for WoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "を"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match 経る verb in 連用形
    #[derive(Debug)]
    struct HeruVerbMatcher;
    impl super::Matcher for HeruVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.base_form == "経る"
                        && token.pos.first().is_some_and(|pos| pos == "動詞")
                        && token.features.get(5).is_some_and(|f| f == "連用形") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match て particle (接続助詞)
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl super::Matcher for TeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "て"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        noun(),
        TokenMatcher::Custom(Arc::new(WoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(HeruVerbMatcher)),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
    ]
}

// Pattern: ながらに (while being, as)
// Structures:
//   - Noun/Verb[stem] + ながら + に(して)
//   - Compound adverbs: 昔ながら, 生まれながら, いつもながら + Optional(に/の)
pub fn nagarani() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for ながら: either as 助詞 or compound adverb
    #[derive(Debug)]
    struct NagaraMatcher;
    impl super::Matcher for NagaraMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match ながら as 助詞/接続助詞
                if token.surface == "ながら"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
                {
                    if let Some(next) = ctx.lookahead(1) {
                        if (next.surface == "に"
                            && next.pos.first().is_some_and(|pos| pos == "助詞")
                            && (next.pos.get(1).is_some_and(|pos| pos == "副詞化")
                                || next.pos.get(1).is_some_and(|pos| pos == "格助詞")))
                            || (next.surface == "の"
                                && next.pos.first().is_some_and(|pos| pos == "助詞")
                                && next.pos.get(1).is_some_and(|pos| pos == "連体化"))
                        {
                            return true;
                        }
                    }

                    if let Some(prev) = ctx.lookbehind(1) {
                        return matches!(
                            prev.surface.as_str(),
                            "いつも" | "毎回" | "毎度" | "溜め息" | "溜息"
                        );
                    }

                    return false;
                }
                // Match compound adverbs ending in ながら
                if token.pos.first().is_some_and(|pos| pos == "副詞")
                    && token.surface.ends_with("ながら")
                {
                    return true;
                }
                false
            })
        }
    }

    // Matcher for に or の particle after ながら
    #[derive(Debug)]
    struct NiNoParticleMatcher;
    impl super::Matcher for NiNoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                if !token.pos.first().is_some_and(|pos| pos == "助詞") {
                    return false;
                }
                // に as 副詞化 or 格助詞
                if token.surface == "に"
                    && (token.pos.get(1).is_some_and(|pos| pos == "副詞化")
                        || token.pos.get(1).is_some_and(|pos| pos == "格助詞"))
                {
                    return true;
                }
                // の as 連体化
                if token.surface == "の" && token.pos.get(1).is_some_and(|pos| pos == "連体化")
                {
                    return true;
                }
                false
            })
        }
    }

    // Matcher for して (する in て-form)
    #[derive(Debug)]
    struct ShiteMatcher;
    impl super::Matcher for ShiteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "し"
                        && token.base_form == "する"
                        && token.pos.first().is_some_and(|pos| pos == "動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matcher for て particle
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl super::Matcher for TeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "て"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Pattern: (Noun/Verb/Adverb) + (ながら or compound) + Optional(に/の) + Optional(して)
    vec![
        any(),
        TokenMatcher::Custom(Arc::new(NagaraMatcher)),
        optional(TokenMatcher::Custom(Arc::new(NiNoParticleMatcher))),
        optional(TokenMatcher::Custom(Arc::new(ShiteMatcher))),
        optional(TokenMatcher::Custom(Arc::new(TeParticleMatcher))),
    ]
}

// Pattern: たなり・なり (remain as is, stay in that state)
// Structures: Verb[た] + なり + Optional(で)
pub fn tanari_u30fb_nari() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct NariParticleMatcher;
    impl super::Matcher for NariParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "なり"
                        && token.base_form == "なり"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && (token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
                            || token.pos.get(1).is_some_and(|pos| pos == "副助詞")) =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct DeParticleMatcher;
    impl super::Matcher for DeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "で"
                        && token.base_form == "だ"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        super::flexible_verb_form(),
        super::past_auxiliary(),
        TokenMatcher::Custom(Arc::new(NariParticleMatcher)),
        optional(TokenMatcher::Custom(Arc::new(DeParticleMatcher))),
    ]
}

// Pattern: の極み (the ultimate/extreme/epitome of)
// Structures: Noun + の + 極（きわ）み
pub fn nokiwami() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match の as rentaika particle (連体化)
    #[derive(Debug)]
    struct NoParticleMatcher;
    impl super::Matcher for NoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "の"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "連体化")
            })
        }
    }

    // Match 極み as noun
    #[derive(Debug)]
    struct KiwamiMatcher;
    impl super::Matcher for KiwamiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "極み"
                    && token.base_form == "極み"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "一般")
            })
        }
    }

    vec![
        super::noun(),
        TokenMatcher::Custom(Arc::new(NoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(KiwamiMatcher)),
    ]
}

// Pattern: にしてみれば (from the point of view of)
// Structures: Noun + にしてみれば, Noun + にしてみたら
pub fn nishitemireba() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for に particle (格助詞)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "に"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            })
        }
    }

    // Matcher for し (する in 連用形)
    #[derive(Debug)]
    struct ShiMatcher;
    impl super::Matcher for ShiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "し"
                    && token.base_form == "する"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token.features.get(5).is_some_and(|f| f == "連用形")
            })
        }
    }

    // Matcher for て particle (接続助詞)
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl super::Matcher for TeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "て"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
            })
        }
    }

    // Matcher for み or みれ (みる in 連用形 or 仮定形)
    #[derive(Debug)]
    struct MiMatcher;
    impl super::Matcher for MiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.surface == "み" || token.surface == "みれ")
                    && token.base_form == "みる"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "非自立")
            })
        }
    }

    // Matcher for れば or たら (conditional endings)
    #[derive(Debug)]
    struct ConditionalEndingMatcher;
    impl super::Matcher for ConditionalEndingMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // ば particle (接続助詞) for みれば
                if token.surface == "ば"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
                {
                    return true;
                }

                // たら (た in 仮定形, 助動詞) for みたら
                if token.surface == "たら"
                    && token.base_form == "た"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.features.get(5).is_some_and(|f| f == "仮定形")
                {
                    return true;
                }

                false
            })
        }
    }

    vec![
        super::noun(),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(ShiMatcher)),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(MiMatcher)),
        TokenMatcher::Custom(Arc::new(ConditionalEndingMatcher)),
    ]
}

// Pattern: だの (things like, and whatnot)
// Structures: A + だの + B + だの
// Note: だの tokenizes as single token (助詞/並立助詞) in most cases
// After い-adjectives and some contexts, it may tokenize as だ + の
pub fn dano() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for だの as a single particle token
    #[derive(Debug)]
    struct DanoParticleMatcher;
    impl super::Matcher for DanoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "だの"
                    && token.base_form == "だの"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "並立助詞")
            })
        }
    }

    // Match just the だの particle itself
    // This handles the common case where だの is a single token
    vec![TokenMatcher::Custom(Arc::new(DanoParticleMatcher))]
}

// Pattern: だの (split tokenization variant)
// Handles cases where だの is tokenized as だ (助動詞) + の (名詞/非自立)
// This occurs after い-adjectives and some other contexts
pub fn dano_split() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for だ (助動詞)
    #[derive(Debug)]
    struct DaAuxiliaryMatcher;
    impl super::Matcher for DaAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "だ"
                    && token.base_form == "だ"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    // Matcher for の (名詞/非自立/一般)
    #[derive(Debug)]
    struct NoNounMatcher;
    impl super::Matcher for NoNounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "の"
                    && token.base_form == "の"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "非自立")
            })
        }
    }

    // Match: だ + の sequence
    vec![
        TokenMatcher::Custom(Arc::new(DaAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(NoNounMatcher)),
    ]
}

// Pattern: あくまでも
/// Pattern: あくまでも (to the end, persistently, stubbornly)
/// Structures: あくまで（も） + Phrase
pub fn akumademo() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Custom matcher for あくまでも or あくまで (adverb)
    #[derive(Debug)]
    struct AkumademoMatcher;
    impl super::Matcher for AkumademoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.surface == "あくまでも" || token.surface == "あくまで")
                    && token.base_form == token.surface
                    && token.pos.first().is_some_and(|pos| pos == "副詞")
            })
        }
    }

    vec![TokenMatcher::Custom(Arc::new(AkumademoMatcher))]
}

// Pattern: べく (in order to, for the purpose of)
// Structures: Verb[基本形/文語基本形] + べく
pub fn beku() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match べく as auxiliary verb with base form べし
    #[derive(Debug)]
    struct BekuMatcher;
    impl super::Matcher for BekuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "べく"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.base_form == "べし" =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match verb in dictionary form (基本形 or 文語基本形)
    #[derive(Debug)]
    struct DictionaryFormVerbMatcher;
    impl super::Matcher for DictionaryFormVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.pos.first().is_some_and(|pos| pos == "動詞")
                        && token
                            .features
                            .get(5)
                            .is_some_and(|form| form == "基本形" || form == "文語基本形") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(DictionaryFormVerbMatcher)),
        TokenMatcher::Custom(Arc::new(BekuMatcher)),
    ]
}

// Pattern: ところを (at a time when, in spite of)
// Structures: Verb/Adjective/Noun + ところを
pub fn tokorowo() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ところ as 名詞
    #[derive(Debug)]
    struct TokoroMatcher;
    impl super::Matcher for TokoroMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ところ"
                        && token.base_form == "ところ"
                        && token.pos.first().is_some_and(|pos| pos == "名詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match を as 格助詞
    #[derive(Debug)]
    struct WoParticleMatcher;
    impl super::Matcher for WoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "を"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Verb, Adjective, Noun, or particle (な, の)
        TokenMatcher::Custom(Arc::new(TokoroMatcher)),
        TokenMatcher::Custom(Arc::new(WoParticleMatcher)),
    ]
}

// Pattern: からある (as much as, as many as)
// Structures: Number + Counter + から + ある/いる
pub fn karaaru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match counter suffix (助数詞)
    #[derive(Debug)]
    struct CounterMatcher;
    impl super::Matcher for CounterMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.pos.first().is_some_and(|pos| pos == "名詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                        && token.pos.get(2).is_some_and(|pos| pos == "助数詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match から particle
    #[derive(Debug)]
    struct KaraParticleMatcher;
    impl super::Matcher for KaraParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "から"
                        && token.base_form == "から"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match ある or いる verb
    #[derive(Debug)]
    struct AruIruMatcher;
    impl super::Matcher for AruIruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if (token.base_form == "ある" || token.base_form == "いる")
                        && token.pos.first().is_some_and(|pos| pos == "動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(CounterMatcher)),
        TokenMatcher::Custom(Arc::new(KaraParticleMatcher)),
        TokenMatcher::Custom(Arc::new(AruIruMatcher)),
    ]
}

// Pattern: にして②
// Pattern: にして② (both (A) and (B))
// Structures: Noun + にして, な-Adjective + にして
pub fn nishite_u2461() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match regular noun or な-adjective (excluding number-like nouns)
    // This pattern is for "both A and B" meaning, typically with:
    // - Regular nouns (教授, 経営者, etc.)
    // - な-adjectives (安全, 丁寧, etc.)
    // - Set expressions (幸い, 不幸, etc.)
    #[derive(Debug)]
    struct NounOrNaAdjectiveMatcher;
    impl super::Matcher for NounOrNaAdjectiveMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.pos.first().is_some_and(|p| p == "名詞")
                        && !is_number_or_counter(token) =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Helper to exclude numbers/counters (which are for にして①)
    fn is_number_or_counter(token: &crate::KagomeToken) -> bool {
        // Exclude 数 (numbers) and typical counter-like nouns
        if let Some(subtype) = token.pos.get(1) {
            if subtype == "数" {
                return true;
            }
        }
        // Also exclude words that end with typical counters
        // But this is tricky - for now just rely on POS tag
        false
    }

    // Match に particle (格助詞)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "に"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match し (する in 連用形)
    #[derive(Debug)]
    struct ShiVerbMatcher;
    impl super::Matcher for ShiVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "し"
                        && token.base_form == "する"
                        && token.pos.first().is_some_and(|p| p == "動詞")
                        && token.features.get(5).is_some_and(|f| f == "連用形") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match て particle (接続助詞)
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl super::Matcher for TeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "て"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "接続助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NounOrNaAdjectiveMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(ShiVerbMatcher)),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
    ]
}

// Pattern: つ〜つ (doing A and B repeatedly/alternately)
// Structures: Verb[stem] + つ + Verb[stem] + つ
pub fn tsu_u301c_tsu() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match つ as auxiliary verb (助動詞, 下二・タ行, 基本形)
    #[derive(Debug)]
    struct TsuAuxiliaryMatcher;
    impl super::Matcher for TsuAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "つ"
                        && token.base_form == "つ"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        // First verb in 連用形 (conjunctive form)
        verb_form("連用形"),
        // First つ
        TokenMatcher::Custom(Arc::new(TsuAuxiliaryMatcher)),
        // Second verb (can be any form - 連用形 for regular, 未然形+れ連用形 for passive)
        any(),
        // Allow optional れる/られる auxiliary (for passive forms)
        optional(reru_suffix()),
        // Second つ
        TokenMatcher::Custom(Arc::new(TsuAuxiliaryMatcher)),
    ]
}

// Pattern: 飽くまで(も)
pub fn akumade_mo() -> Vec<TokenMatcher> {
    vec![] // TODO: Implement
}

// Pattern: であれ〜であれ (whether X or Y, no matter if X or Y)
// Structures: Noun/な-Adj + であれ + Noun/な-Adj + であれ
// Note: Handles compound nouns like 日本製 (multiple consecutive noun tokens)
pub fn deare_u301c_deare() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match で (auxiliary verb だ in 連用形)
    #[derive(Debug)]
    struct DeAuxiliaryMatcher;
    impl super::Matcher for DeAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "で"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.base_form == "だ" =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match あれ (auxiliary verb ある in 命令ｅ form)
    #[derive(Debug)]
    struct AreImperativeMatcher;
    impl super::Matcher for AreImperativeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "あれ"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.base_form == "ある"
                        && token.features.get(5).is_some_and(|f| f.contains("命令")) =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        // First noun/な-adjective + であれ
        noun(),
        TokenMatcher::Custom(Arc::new(DeAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(AreImperativeMatcher)),
        // Second noun/な-adjective + であれ
        noun(),
        TokenMatcher::Custom(Arc::new(DeAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(AreImperativeMatcher)),
    ]
}

// Pattern: たら最後 (once X happens, Y inevitably follows)
// Structures: Verb[た] + が + 最後 / Verb[たら] + 最後
pub fn tarasaigo() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match た (助動詞/基本形) or たら (助動詞/仮定形)
    #[derive(Debug)]
    struct TaOrTaraMatcher;
    impl super::Matcher for TaOrTaraMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.base_form == "た"
                        && (token.features.get(5).is_some_and(|f| f == "基本形")
                            || token.features.get(5).is_some_and(|f| f == "仮定形")) =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match が as 接続助詞 (used in た + が + 最後)
    #[derive(Debug)]
    struct GaConjunctionMatcher;
    impl super::Matcher for GaConjunctionMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "が"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        super::flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(TaOrTaraMatcher)),
        optional(TokenMatcher::Custom(Arc::new(GaConjunctionMatcher))),
        surface("最後"),
    ]
}

// Pattern: いかなる (no matter what, any kind of)
// Structures: いかなる + Noun
pub fn ikanaru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct IkanaruMatcher;
    impl super::Matcher for IkanaruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "いかなる"
                        && token.base_form == "いかなる"
                        && token.pos.first().is_some_and(|pos| pos == "連体詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(IkanaruMatcher)),
        noun(), // Noun that follows
    ]
}

// Pattern: なりに (in one's own way, for what it is)
// Structures: Noun/Adjective/Verb + なり + に/の
pub fn narini() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match なり as 副助詞 (adverbial particle)
    #[derive(Debug)]
    struct NariParticleMatcher;
    impl super::Matcher for NariParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "なり"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "副助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Pattern: (Noun/Adj/Verb/AuxVerb) + なり(副助詞) + (に OR の)
    // Note: そ れなり is a special case that's tokenized as a single noun "それなり",
    // so it won't be caught by this pattern. We handle it separately below.
    vec![
        or(vec![noun(), adjective(), verb(), past_auxiliary()]),
        TokenMatcher::Custom(Arc::new(NariParticleMatcher)),
        or(vec![surface("に"), surface("の")]),
    ]
}

// Pattern: それなり + に/の (variant of なりに for fixed expression)
// When "それなり" is tokenized as a single noun
pub fn narini_sorenari() -> Vec<TokenMatcher> {
    vec![surface("それなり"), or(vec![surface("に"), surface("の")])]
}

// Pattern: れる・られる + ままに (as one is told/ordered)
// Structures: Verb[られる] + (が) + まま + (に)
pub fn reru_u30fb_rareru_mamani() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match が as conjunction particle (助詞/接続助詞)
    #[derive(Debug)]
    struct GaConjunctionMatcher;
    impl super::Matcher for GaConjunctionMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "が"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "接続助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match に case particle (助詞/格助詞)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "に"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        // Verb[未然形] is already matched, we look for れる/られる
        any(), // The verb in 未然形 before れる/られる
        or(vec![reru_suffix(), rareru_suffix()]),
        optional(TokenMatcher::Custom(Arc::new(GaConjunctionMatcher))),
        surface("まま"),
        optional(TokenMatcher::Custom(Arc::new(NiParticleMatcher))),
    ]
}

// Pattern: にまつわる (related to, connected to, surrounding)
// Structures: Noun (A) + にまつわる + (Adjective) + Noun (B)
pub fn nimatsuwaru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match にまつわる as a compound particle
    #[derive(Debug)]
    struct NimatsuwaruMatcher;
    impl super::Matcher for NimatsuwaruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "にまつわる"
                        && token.base_form == "にまつわる"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                        && token.pos.get(2).is_some_and(|pos| pos == "連語") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        super::noun(),
        TokenMatcher::Custom(Arc::new(NimatsuwaruMatcher)),
        // Allow 0-2 tokens between にまつわる and the noun (for adjectives)
        wildcard(0, 2, vec![]),
        super::noun(),
    ]
}

// Pattern: たる (classical copula - position/role)
// Structures: Noun + たるに, Noun + たる + Noun
pub fn taru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // たる can be tokenized in two ways:
    // 1. As 名詞/一般 (in "たるに")
    // 2. As 助動詞 with base たり (in "たるもの")
    #[derive(Debug)]
    struct TaruMatcher;
    impl super::Matcher for TaruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "たる"
                        && ((token.pos.first().is_some_and(|pos| pos == "名詞")
                            && token.base_form == "たる")
                            || (token.pos.first().is_some_and(|pos| pos == "助動詞")
                                && token.base_form == "たり")) =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Optional に particle (for たるに) or もの noun (for たるもの)
    #[derive(Debug)]
    struct TaruFollowMatcher;
    impl super::Matcher for TaruFollowMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match に (格助詞) for たるに
                (token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞"))
                // Or match もの (名詞) for たるもの
                || (token.surface == "もの"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.base_form == "もの")
            })
        }
    }

    vec![
        super::noun(),
        TokenMatcher::Custom(Arc::new(TaruMatcher)),
        optional(TokenMatcher::Custom(Arc::new(TaruFollowMatcher))),
    ]
}

// Pattern: なら〜で (if X, should/must do X properly)
// Structures: Word + なら + Same Word + で
// Works with verbs, い-adjectives, な-adjectives, nouns
pub fn nara_u301c_de() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matches なら as 助動詞 with base だ in 仮定形
    #[derive(Debug)]
    struct NaraMatcher;
    impl super::Matcher for NaraMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "なら"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.base_form == "だ" =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matches で as 助詞/接続助詞 or 助詞/格助詞
    #[derive(Debug)]
    struct DeParticleMatcher;
    impl super::Matcher for DeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "で"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && (token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
                            || token.pos.get(1).is_some_and(|pos| pos == "格助詞")) =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matches verbs (動詞), い-adjectives (形容詞), な-adjectives (名詞/形容動詞語幹)
    #[derive(Debug)]
    struct FirstWordMatcher;
    impl super::Matcher for FirstWordMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.pos.first().is_some_and(|pos| pos == "動詞")
                        || token.pos.first().is_some_and(|pos| pos == "形容詞")
                        || (token.pos.first().is_some_and(|pos| pos == "名詞")
                            && token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹")) =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    let first_word = TokenMatcher::Custom(Arc::new(FirstWordMatcher));

    vec![
        first_word.clone(),
        TokenMatcher::Custom(Arc::new(NaraMatcher)),
        first_word,
        TokenMatcher::Custom(Arc::new(DeParticleMatcher)),
    ]
}

// Pattern: をものともせず (undaunted by, in defiance of)
// Structures: Verb/Adjective + の + をものともせず or Noun + をものともせず
pub fn womonotomosezu() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matches の as nominalizer (名詞/非自立/一般)
    #[derive(Debug)]
    struct NoNominalizerMatcher;
    impl super::Matcher for NoNominalizerMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "の"
                        && token.base_form == "の"
                        && token.pos.first().is_some_and(|pos| pos == "名詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "非自立") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matches を case particle (助詞/格助詞/一般)
    #[derive(Debug)]
    struct WoParticleMatcher;
    impl super::Matcher for WoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "を"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matches もの (名詞/非自立/一般)
    #[derive(Debug)]
    struct MonoNounMatcher;
    impl super::Matcher for MonoNounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "もの"
                        && token.base_form == "もの"
                        && token.pos.first().is_some_and(|pos| pos == "名詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "非自立") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matches と as quotation particle (助詞/格助詞/引用)
    #[derive(Debug)]
    struct ToQuotationMatcher;
    impl super::Matcher for ToQuotationMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "と"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                        && token.pos.get(2).is_some_and(|pos| pos == "引用") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matches も as binding particle (助詞/係助詞)
    #[derive(Debug)]
    struct MoBindingMatcher;
    impl super::Matcher for MoBindingMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "も"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "係助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matches せ (する in 未然ヌ接続)
    #[derive(Debug)]
    struct SeVerbMatcher;
    impl super::Matcher for SeVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "せ"
                        && token.base_form == "する"
                        && token.pos.first().is_some_and(|pos| pos == "動詞")
                        && token.features.get(5).is_some_and(|f| f == "未然ヌ接続") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matches ず (助動詞, base=ぬ)
    #[derive(Debug)]
    struct ZuAuxiliaryMatcher;
    impl super::Matcher for ZuAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ず"
                        && token.base_form == "ぬ"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matches optional に particle
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "に"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        // Optional の (for Verb/Adjective + の patterns)
        optional(TokenMatcher::Custom(Arc::new(NoNominalizerMatcher))),
        TokenMatcher::Custom(Arc::new(WoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(MonoNounMatcher)),
        TokenMatcher::Custom(Arc::new(ToQuotationMatcher)),
        TokenMatcher::Custom(Arc::new(MoBindingMatcher)),
        TokenMatcher::Custom(Arc::new(SeVerbMatcher)),
        TokenMatcher::Custom(Arc::new(ZuAuxiliaryMatcher)),
        // Optional に at the end
        optional(TokenMatcher::Custom(Arc::new(NiParticleMatcher))),
    ]
}

// Pattern: には当たらない (not worth doing, no need to)
// Structures: Verb + に(は) + あたらない/あたりません
pub fn nihaataranai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for に particle
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "に"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matcher for は particle
    #[derive(Debug)]
    struct WaParticleMatcher;
    impl super::Matcher for WaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "は"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "係助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matcher for あたる verb in 未然形 (for ない)
    #[derive(Debug)]
    struct AtaraMizenMatcher;
    impl super::Matcher for AtaraMizenMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.base_form == "あたる"
                        && token.pos.first().is_some_and(|pos| pos == "動詞")
                        && token.features.get(5).is_some_and(|f| f == "未然形") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matcher for ない auxiliary (must follow あたら)
    #[derive(Debug)]
    struct NaiAuxiliaryMatcher;
    impl super::Matcher for NaiAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ない"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match: Verb (dictionary form) + に + (は optional) + あたら + ない
    vec![
        verb(),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        optional(TokenMatcher::Custom(Arc::new(WaParticleMatcher))),
        TokenMatcher::Custom(Arc::new(AtaraMizenMatcher)),
        TokenMatcher::Custom(Arc::new(NaiAuxiliaryMatcher)),
    ]
}

// Pattern: ものと思う (believe that, have confidence that)
// Structures: もの + と + おもう
pub fn monotoomou() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match もの as 名詞/非自立/一般
    #[derive(Debug)]
    struct MonoMatcher;
    impl super::Matcher for MonoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "もの"
                    && token.base_form == "もの"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "非自立")
            })
        }
    }

    // Match と as quotation particle (助詞/格助詞/引用)
    #[derive(Debug)]
    struct ToQuotationMatcher;
    impl super::Matcher for ToQuotationMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "と"
                    && token.base_form == "と"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                    && token.pos.get(2).is_some_and(|pos| pos == "引用")
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(MonoMatcher)),
        TokenMatcher::Custom(Arc::new(ToQuotationMatcher)),
        verb_base("おもう"),
    ]
}

// Pattern: を踏まえて (considering, based on)
// Structures: Noun + を + ふまえ(る) + て/た/た上で/ての
pub fn wofumaete() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match を particle (助詞/格助詞)
    #[derive(Debug)]
    struct WoParticleMatcher;
    impl super::Matcher for WoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "を"
                    && token.pos.first().is_some_and(|p| p == "助詞")
                    && token.pos.get(1).is_some_and(|p| p == "格助詞")
            })
        }
    }

    // Match て or た (after ふまえ)
    #[derive(Debug)]
    struct TeTaMatcher;
    impl super::Matcher for TeTaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.surface == "て" && token.pos.first().is_some_and(|p| p == "助詞"))
                    || (token.surface == "た" && token.pos.first().is_some_and(|p| p == "助動詞"))
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(WoParticleMatcher)),
        verb_base("ふまえる"),
        TokenMatcher::Custom(Arc::new(TeTaMatcher)),
    ]
}

// Pattern: ゆえに (because of, due to, consequently)
// Structures:
//   - Noun/Adj/Verb + ゆえ + に
//   - Noun/Adj/Verb + ゆえ + の (modifying noun)
//   - が + ゆえに (single token, 接続詞)
pub fn yueni() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ゆえ as 名詞 (接尾 or 非自立/副詞可能)
    #[derive(Debug)]
    struct YueMatcher;
    impl super::Matcher for YueMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ゆえ"
                    && token.base_form == "ゆえ"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && (token
                        .pos
                        .get(1)
                        .is_some_and(|pos| pos == "接尾" || pos == "非自立"))
            })
        }
    }

    // Match に as 助詞/格助詞
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "に"
                    && token.base_form == "に"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(YueMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
    ]
}

// Pattern: ゆえに (conjunction form - single token)
// Used after が or at sentence start
pub fn yueni_conjunction() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct YueniConjunctionMatcher;
    impl super::Matcher for YueniConjunctionMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ゆえに"
                        && token.base_form == "ゆえに"
                        && token.pos.first().is_some_and(|pos| pos == "接続詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![TokenMatcher::Custom(Arc::new(YueniConjunctionMatcher))]
}

// Pattern: ゆえの (modifying noun)
pub fn yueno() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ゆえ as 名詞
    #[derive(Debug)]
    struct YueMatcher;
    impl super::Matcher for YueMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ゆえ"
                        && token.base_form == "ゆえ"
                        && token.pos.first().is_some_and(|pos| pos == "名詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match の as 助詞/連体化
    #[derive(Debug)]
    struct NoParticleMatcher;
    impl super::Matcher for NoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "の"
                        && token.base_form == "の"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "連体化") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(YueMatcher)),
        TokenMatcher::Custom(Arc::new(NoParticleMatcher)),
    ]
}

// Pattern: にとどまらず
// Pattern: にとどまらず (not limited to, not stopping at)
// Structures: Verb/Noun/な-Adj + に + とどまら(ず) + ず
// Verb/Noun can be preceded by optional である for formal register
pub fn nitodomarazu() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match に particle (格助詞)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "に"
                    && token.pos.first().is_some_and(|p| p == "助詞")
                    && token.pos.get(1).is_some_and(|p| p == "格助詞")
            })
        }
    }

    // Match とどまる verb in 未然形 (とどまら)
    #[derive(Debug)]
    struct TodomaruVerbMatcher;
    impl super::Matcher for TodomaruVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.base_form == "とどまる"
                    && token.pos.first().is_some_and(|p| p == "動詞")
                    && token.features.get(5).is_some_and(|f| f == "未然形")
            })
        }
    }

    // Match ず auxiliary (base=ぬ)
    #[derive(Debug)]
    struct ZuAuxiliaryMatcher;
    impl super::Matcher for ZuAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ず"
                    && token.base_form == "ぬ"
                    && token.pos.first().is_some_and(|p| p == "助動詞")
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(TodomaruVerbMatcher)),
        TokenMatcher::Custom(Arc::new(ZuAuxiliaryMatcher)),
    ]
}

// Pattern: と思いきや (despite having thought, when I thought)
// Structures: Verb/Adj/Noun + (か) + と + 思い + きや
pub fn toomoikiya() -> Vec<TokenMatcher> {
    use super::concat;
    use std::sync::Arc;

    // Match か particle (optional)
    #[derive(Debug)]
    struct KaParticleMatcher;
    impl super::Matcher for KaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "か" && token.pos.first().is_some_and(|pos| pos == "助詞")
            })
        }
    }

    // Match と particle (格助詞/引用)
    #[derive(Debug)]
    struct ToQuotativeMatcher;
    impl super::Matcher for ToQuotativeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "と"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            })
        }
    }

    // Match 思う/おもう verb in 連用形
    #[derive(Debug)]
    struct OmouRenyoukeiMatcher;
    impl super::Matcher for OmouRenyoukeiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.base_form == "思う" || token.base_form == "おもう")
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token.features.get(5).is_some_and(|f| f == "連用形")
            })
        }
    }

    // Match き auxiliary verb (classical past tense)
    #[derive(Debug)]
    struct KiAuxiliaryMatcher;
    impl super::Matcher for KiAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "き"
                    && token.base_form == "き"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    // Match や particle (接続助詞)
    #[derive(Debug)]
    struct YaConjunctiveMatcher;
    impl super::Matcher for YaConjunctiveMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "や"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
            })
        }
    }

    // Matches any word (verb, noun, adjective) or だ auxiliary
    #[derive(Debug)]
    struct PredicateMatcher;
    impl super::Matcher for PredicateMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                let is_verb = token.pos.first().is_some_and(|pos| pos == "動詞");
                let is_noun = token.pos.first().is_some_and(|pos| pos == "名詞");
                let is_adj = token.pos.first().is_some_and(|pos| pos == "形容詞");
                let is_da =
                    token.surface == "だ" && token.pos.first().is_some_and(|pos| pos == "助動詞");

                is_verb || is_noun || is_adj || is_da
            })
        }
    }

    // Pattern: [Verb/Noun/Adj/だ] + (optional か) + と + 思い + き + や
    concat(vec![
        vec![TokenMatcher::Custom(Arc::new(PredicateMatcher))],
        vec![optional(TokenMatcher::Custom(Arc::new(KaParticleMatcher)))],
        vec![TokenMatcher::Custom(Arc::new(ToQuotativeMatcher))],
        vec![TokenMatcher::Custom(Arc::new(OmouRenyoukeiMatcher))],
        vec![TokenMatcher::Custom(Arc::new(KiAuxiliaryMatcher))],
        vec![TokenMatcher::Custom(Arc::new(YaConjunctiveMatcher))],
    ])
}

// Pattern: どうにも (even however/no way)
// Structures: どうにも（こうにも） + (Negative expressions)
pub fn dounimo() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match どうにも as single token (副詞/一般)
    #[derive(Debug)]
    struct DounimoAdverbMatcher;
    impl super::Matcher for DounimoAdverbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "どうにも"
                        && token.pos.first().is_some_and(|pos| pos == "副詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Main pattern: Match どうにも as single token (most common case)
    // The split tokenization (どう + に + も) will be handled by a separate pattern
    vec![TokenMatcher::Custom(Arc::new(DounimoAdverbMatcher))]
}

// Pattern: どうにも (split tokenization variant)
// Structures: どう + に + も (when tokenized as three separate tokens)
pub fn dounimo_split() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match どう (副詞/助詞類接続)
    #[derive(Debug)]
    struct DouAdverbMatcher;
    impl super::Matcher for DouAdverbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "どう"
                        && token.pos.first().is_some_and(|pos| pos == "副詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match に (助詞/格助詞)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "に"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match も (助詞/係助詞)
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl super::Matcher for MoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "も"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "係助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(DouAdverbMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(MoParticleMatcher)),
    ]
}

// Pattern: ことだし (since/because) - Simple form
// Structures: Verb + ことだし, い-Adjective + ことだし
pub fn kotodashi() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for こと (noun, 非自立)
    #[derive(Debug)]
    struct KotoMatcher;
    impl super::Matcher for KotoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "こと"
                        && token.pos.first().is_some_and(|p| p == "名詞")
                        && token.pos.get(1).is_some_and(|p| p == "非自立") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matcher for だ (auxiliary verb)
    #[derive(Debug)]
    struct DaMatcher;
    impl super::Matcher for DaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.base_form == "だ"
                        && token.pos.first().is_some_and(|p| p == "助動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matcher for し (conjunction particle)
    #[derive(Debug)]
    struct ShiMatcher;
    impl super::Matcher for ShiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "し"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "接続助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Verb or い-Adjective (immediately before こと)
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
        TokenMatcher::Custom(Arc::new(DaMatcher)),
        TokenMatcher::Custom(Arc::new(ShiMatcher)),
    ]
}

// Pattern: ことだし (since/because) - Compound form
// Structures: Noun + の + ことだし, な-Adjective + な + ことだし
pub fn kotodashi_compound() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for の or な (particles that connect nouns/na-adj to こと)
    #[derive(Debug)]
    struct NoNaMatcher;
    impl super::Matcher for NoNaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if (token.surface == "の"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "連体化"))
                        || (token.surface == "な"
                            && token.base_form == "だ"
                            && token.pos.first().is_some_and(|p| p == "助動詞")) =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Reuse matchers from kotodashi()
    #[derive(Debug)]
    struct KotoMatcher;
    impl super::Matcher for KotoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "こと"
                        && token.pos.first().is_some_and(|p| p == "名詞")
                        && token.pos.get(1).is_some_and(|p| p == "非自立") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct DaMatcher;
    impl super::Matcher for DaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.base_form == "だ"
                        && token.pos.first().is_some_and(|p| p == "助動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct ShiMatcher;
    impl super::Matcher for ShiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "し"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "接続助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),                                       // Noun or な-Adjective stem
        TokenMatcher::Custom(Arc::new(NoNaMatcher)), // の or な
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
        TokenMatcher::Custom(Arc::new(DaMatcher)),
        TokenMatcher::Custom(Arc::new(ShiMatcher)),
    ]
}

// Pattern: がん～ (colloquial intensifier - "majorly/completely/furiously")
// Structures: ガン + Verb[stem], ガン + Noun (suru-verb)
pub fn gan_uff5e() -> Vec<TokenMatcher> {
    vec![
        surface("ガン"),          // Match "ガン" (katakana) as surface text
        or(vec![verb(), noun()]), // Match verb or noun following ガン
    ]
}

// Pattern: か否か (whether or not) - Variant 1
// Structures: Verb/Adj/Noun + かいな + か
pub fn kainaka() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for かいな (noun)
    #[derive(Debug)]
    struct KainaMatcher;
    impl super::Matcher for KainaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "かいな"
                        && token.pos.first().is_some_and(|pos| pos == "名詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matcher for か particle
    #[derive(Debug)]
    struct KaParticleMatcher;
    impl super::Matcher for KaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "か"
                        && token.pos.first().is_some_and(|pos| pos == "助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matches: Any token + かいな(noun) + か(particle)
    vec![
        any(),
        TokenMatcher::Custom(Arc::new(KainaMatcher)),
        TokenMatcher::Custom(Arc::new(KaParticleMatcher)),
    ]
}

// Pattern: か否か (whether or not) - Variant 2
// Structures: Verb/Adj/Noun + である + か + いなか
// This variant appears after である or in certain contexts
pub fn kainaka_ka_inaka() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for か particle
    #[derive(Debug)]
    struct KaParticleMatcher;
    impl super::Matcher for KaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "か"
                        && token.pos.first().is_some_and(|pos| pos == "助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matcher for いなか (noun)
    #[derive(Debug)]
    struct InakaMatcher;
    impl super::Matcher for InakaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "いなか"
                        && token.pos.first().is_some_and(|pos| pos == "名詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matches: Any token + か(particle) + いなか(noun)
    vec![
        any(),
        TokenMatcher::Custom(Arc::new(KaParticleMatcher)),
        TokenMatcher::Custom(Arc::new(InakaMatcher)),
    ]
}

// Pattern: たら〜で (even if, if...then with repeating words)
// Structures: Word[conditional/たら/ば/なら] + Same_Word[plain/た] + で
//
// NOTE: This pattern ideally requires matching the same word twice (different conjugations).
// Due to the constraint that matchers only see one token at a time, we match the structural
// pattern without verifying word repetition. This may produce false positives but captures
// the most common usage patterns.
//
// Structure variants:
// 1. Verb[連用タ接続] + たら + Verb[連用タ接続] + た + で
// 2. Verb[仮定形] + ば + Verb[連用タ接続] + た + で
// 3. い-Adj[連用タ接続] + たら + い-Adj[基本形] + で
// 4. い-Adj[仮定形] + ば + い-Adj[基本形] + で
// 5. な-Adj + なら + な-Adj + で
pub fn tara_u301c_de() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match たら (conditional auxiliary)
    #[derive(Debug)]
    struct TaraConditionalMatcher;
    impl super::Matcher for TaraConditionalMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "たら"
                        && token.base_form == "た"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.features.get(5).is_some_and(|f| f == "仮定形") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match で (conjunction particle after た)
    #[derive(Debug)]
    struct DeConjunctionMatcher;
    impl super::Matcher for DeConjunctionMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "で"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Verb[連用タ接続] + たら + Verb[連用タ接続] + た + で
    vec![
        super::flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(TaraConditionalMatcher)),
        super::flexible_verb_form(),
        past_auxiliary(),
        TokenMatcher::Custom(Arc::new(DeConjunctionMatcher)),
    ]
}

// Pattern: たら〜で - ば variant (verb)
// Structures: Verb[仮定形] + ば + Verb[連用タ接続] + た + で
pub fn tara_u301c_de_ba_verb() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ば (conditional particle)
    #[derive(Debug)]
    struct BaMatcher;
    impl super::Matcher for BaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ば"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match で (conjunction particle after た)
    #[derive(Debug)]
    struct DeConjunctionMatcher;
    impl super::Matcher for DeConjunctionMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "で"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        verb_form("仮定形"),
        TokenMatcher::Custom(Arc::new(BaMatcher)),
        super::flexible_verb_form(),
        past_auxiliary(),
        TokenMatcher::Custom(Arc::new(DeConjunctionMatcher)),
    ]
}

// Pattern: たら〜で - たら variant (い-adjective)
// Structures: い-Adj[連用タ接続] + たら + い-Adj[基本形] + で
pub fn tara_u301c_de_i_adj() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match い-adjective in 連用タ接続
    #[derive(Debug)]
    struct IAdjRenyouMatcher;
    impl super::Matcher for IAdjRenyouMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.pos.first().is_some_and(|pos| pos == "形容詞")
                        && token.features.get(5).is_some_and(|f| f == "連用タ接続") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match たら (conditional auxiliary)
    #[derive(Debug)]
    struct TaraConditionalMatcher;
    impl super::Matcher for TaraConditionalMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "たら"
                        && token.base_form == "た"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.features.get(5).is_some_and(|f| f == "仮定形") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match で (conjunction particle or だ auxiliary in 連用形)
    #[derive(Debug)]
    struct DeAfterAdjMatcher;
    impl super::Matcher for DeAfterAdjMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "で"
                        && ((token.pos.first().is_some_and(|pos| pos == "助詞")
                            && token.pos.get(1).is_some_and(|pos| pos == "接続助詞"))
                            || (token.pos.first().is_some_and(|pos| pos == "助動詞")
                                && token.base_form == "だ")) =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(IAdjRenyouMatcher)),
        TokenMatcher::Custom(Arc::new(TaraConditionalMatcher)),
        adjective(),
        TokenMatcher::Custom(Arc::new(DeAfterAdjMatcher)),
    ]
}

// Pattern: たら〜で - ば variant (い-adjective)
// Structures: い-Adj[仮定形] + ば + い-Adj[基本形] + で
pub fn tara_u301c_de_ba_i_adj() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match い-adjective in 仮定形
    #[derive(Debug)]
    struct IAdjKateiMatcher;
    impl super::Matcher for IAdjKateiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.pos.first().is_some_and(|pos| pos == "形容詞")
                        && token.features.get(5).is_some_and(|f| f == "仮定形") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match ば (conditional particle)
    #[derive(Debug)]
    struct BaMatcher;
    impl super::Matcher for BaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ば"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match で (auxiliary だ in 連用形 after adjective)
    #[derive(Debug)]
    struct DeAuxiliaryMatcher;
    impl super::Matcher for DeAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "で"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.base_form == "だ"
                        && token.features.get(5).is_some_and(|f| f == "連用形") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(IAdjKateiMatcher)),
        TokenMatcher::Custom(Arc::new(BaMatcher)),
        adjective(),
        TokenMatcher::Custom(Arc::new(DeAuxiliaryMatcher)),
    ]
}

// Pattern: たら〜で - なら variant (な-adjective)
// Structures: な-Adj[名詞/形容動詞語幹] + なら + な-Adj[名詞/形容動詞語幹] + で
pub fn tara_u301c_de_nara_na_adj() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match な-adjective (名詞/形容動詞語幹)
    #[derive(Debug)]
    struct NaAdjMatcher;
    impl super::Matcher for NaAdjMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.pos.first().is_some_and(|pos| pos == "名詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match なら (conditional auxiliary, 仮定形 of だ)
    #[derive(Debug)]
    struct NaraConditionalMatcher;
    impl super::Matcher for NaraConditionalMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "なら"
                        && token.base_form == "だ"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.features.get(5).is_some_and(|f| f == "仮定形") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match で (auxiliary だ in 連用形)
    #[derive(Debug)]
    struct DeAuxiliaryMatcher;
    impl super::Matcher for DeAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "で"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.base_form == "だ"
                        && token.features.get(5).is_some_and(|f| f == "連用形") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NaAdjMatcher)),
        TokenMatcher::Custom(Arc::new(NaraConditionalMatcher)),
        TokenMatcher::Custom(Arc::new(NaAdjMatcher)),
        TokenMatcher::Custom(Arc::new(DeAuxiliaryMatcher)),
    ]
}

// Pattern: べくして (as expected, destined to)
// Structures: Verb + べく + し + て
pub fn bekushite() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match べく (classical auxiliary verb べし in 連用形)
    #[derive(Debug)]
    struct BekuMatcher;
    impl super::Matcher for BekuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "べく"
                        && token.base_form == "べし"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match し from する (連用形)
    #[derive(Debug)]
    struct ShiMatcher;
    impl super::Matcher for ShiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "し"
                        && token.base_form == "する"
                        && token.pos.first().is_some_and(|pos| pos == "動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match て (conjunctive particle)
    #[derive(Debug)]
    struct TeMatcher;
    impl super::Matcher for TeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "て"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Verb before べくして
        TokenMatcher::Custom(Arc::new(BekuMatcher)),
        TokenMatcher::Custom(Arc::new(ShiMatcher)),
        TokenMatcher::Custom(Arc::new(TeMatcher)),
    ]
}

// Pattern: かれ〜かれ (whether A or B)
// Structures: い-Adjective[かれ] + い-Adjective[かれ]
// Fixed expressions: 遅かれ早かれ, 多かれ少なかれ, 良かれ悪しかれ, etc.
pub fn kare_u301c_kare() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for かれ form (imperative e-form of い-adjective)
    #[derive(Debug)]
    struct KareFormMatcher;
    impl super::Matcher for KareFormMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface.ends_with("かれ")
                        && token.pos.first().is_some_and(|pos| pos == "形容詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "自立")
                        && token.features.get(5).is_some_and(|f| f == "命令ｅ") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(KareFormMatcher)),
        TokenMatcher::Custom(Arc::new(KareFormMatcher)),
    ]
}

// Pattern: 〜に〜ない
// Pattern: 〜に〜ない (cannot X even if one wants to)
// Structures: Verb[る] + に + Verb[potential negative]
//            Verb[よう] + にも + Verb[potential negative]
//            する + に + できない
pub fn u301c_ni_u301c_nai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for に particle
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "に"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matcher for optional volitional う/よう
    // Note: For godan verbs, it's う auxiliary (助動詞)
    //       For ichidan verbs, it's よう noun suffix (名詞/接尾)
    #[derive(Debug)]
    struct VolitionalMatcher;
    impl super::Matcher for VolitionalMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Godan verbs: う auxiliary
                if token.surface == "う"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.base_form == "う"
                {
                    return true;
                }
                // Ichidan verbs: よう noun suffix
                if token.surface == "よう"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                    && token.base_form == "よう"
                {
                    return true;
                }
                false
            })
        }
    }

    // Matcher for optional も particle
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl super::Matcher for MoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "も"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "係助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matcher for optional verb before potential auxiliary (for ichidan verbs)
    // Matches: 辞め (base=辞める, 未然形) that comes before られる
    #[derive(Debug)]
    struct OptionalVerbBeforePotentialMatcher;
    impl super::Matcher for OptionalVerbBeforePotentialMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Only match verbs in 未然形 (ichidan verb stems before られる)
                token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立")
                && token.features.get(5).is_some_and(|f| f == "未然形")
                // Should NOT be a potential form itself (not ending in える/れる/できる/られる)
                && !token.base_form.ends_with("える")
                && !token.base_form.ends_with("れる")
                && token.base_form != "できる"
                && token.base_form != "られる"
            })
        }
    }

    // Matcher for potential form verb in 未然形 + ない/なかった
    // This matches:
    // - Godan verbs: 笑え (base=笑える), 断れ (base=断れる)
    // - Ichidan potential auxiliary: られ (base=られる)
    // - Special: でき (base=できる) for する verbs
    #[derive(Debug)]
    struct PotentialNegativeMatcher;
    impl super::Matcher for PotentialNegativeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.pos.first().is_some_and(|pos| pos == "動詞")
                        && (token.pos.get(1).is_some_and(|pos| pos == "自立")
                            || token.pos.get(1).is_some_and(|pos| pos == "接尾"))
                        && token.features.get(5).is_some_and(|f| f == "未然形")
                        && (token.base_form.ends_with("える")
                            || token.base_form.ends_with("れる")
                            || token.base_form == "できる"
                            || token.base_form == "られる") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matcher for ない auxiliary (present or past)
    #[derive(Debug)]
    struct NaiMatcher;
    impl super::Matcher for NaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.base_form == "ない" =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        verb(), // First verb (dictionary or volitional form)
        optional(TokenMatcher::Custom(Arc::new(VolitionalMatcher))),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        optional(TokenMatcher::Custom(Arc::new(MoParticleMatcher))),
        optional(TokenMatcher::Custom(Arc::new(
            OptionalVerbBeforePotentialMatcher,
        ))), // Optional verb stem (for ichidan: 辞め before られ)
        TokenMatcher::Custom(Arc::new(PotentialNegativeMatcher)),
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
        optional(past_auxiliary()), // Optional た auxiliary (for past tense)
    ]
}

// Pattern: なくして(は)
// Pattern: なくして(は) (without)
// Structures: Noun + なくして(は) OR Verb + ことなくして(は)
pub fn nakushite_ha() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for なくし (handles both tokenizations):
    //   - なく(形容詞/ない) + し(動詞/する) → 2 tokens
    //   - なくし(動詞/なくす) as single token → 1 token (Kagome ambiguity)
    #[derive(Debug)]
    struct NakushiMatcher;
    impl super::Matcher for NakushiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                // Split: なく(adj ない) + し(verb する)
                Some(token)
                    if token.surface == "なく"
                        && token.base_form == "ない"
                        && token.pos.first().is_some_and(|p| p == "形容詞") =>
                {
                    if let Some(next) = ctx.lookahead(1) {
                        if next.surface == "し"
                            && next.base_form == "する"
                            && next.pos.first().is_some_and(|p| p == "動詞")
                        {
                            return (true, 2);
                        }
                    }
                    (false, 0)
                }
                // Single token: なくし(verb なくす) — Kagome sometimes merges these
                Some(token)
                    if token.surface == "なくし"
                        && token.base_form == "なくす"
                        && token.pos.first().is_some_and(|p| p == "動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matcher for て particle
    #[derive(Debug)]
    struct TeMatcher;
    impl super::Matcher for TeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "て"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "接続助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matcher for は particle
    #[derive(Debug)]
    struct HaMatcher;
    impl super::Matcher for HaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "は"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "係助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Noun or こと
        TokenMatcher::Custom(Arc::new(NakushiMatcher)),
        TokenMatcher::Custom(Arc::new(TeMatcher)),
        TokenMatcher::Custom(Arc::new(HaMatcher)), // は required to distinguish from verb なくす+て
    ]
}

// Pattern: のなんのって (extremely, so much that)
// Structures: Verb + のなんのって, い-Adj + のなんのって, な-Adj + な + のなんのって
pub fn nonannotte() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match な auxiliary (da copula in rentaikei form)
    #[derive(Debug)]
    struct NaAuxiliaryMatcher;
    impl super::Matcher for NaAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "な"
                        && token.base_form == "だ"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match ん as dependent noun (only for adjective patterns)
    #[derive(Debug)]
    struct NNounMatcher;
    impl super::Matcher for NNounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ん"
                        && token.base_form == "ん"
                        && token.pos.first().is_some_and(|pos| pos == "名詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "非自立") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match って particle
    #[derive(Debug)]
    struct TteParticleMatcher;
    impl super::Matcher for TteParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "って"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // This pattern has multiple tokenization patterns:
    // 1. Verb[た] + の(連体化) + なんの(固有名詞) + って
    // 2. い-Adj + の(名詞/非自立) + な(助動詞) + ん(名詞/非自立) + の(連体化) + って
    // 3. な-Adj + な(助動詞) + の(名詞/非自立) + な(助動詞) + ん(名詞/非自立) + の(連体化) + って
    //
    // Strategy: Use wildcard to capture preceding context (1-3 tokens) + の + (な or なんの) + [optional ん + の] + って
    // This allows us to match from the verb/adjective rather than just from auxiliary

    // Adjective pattern: Wildcard + の(非自立) + な(助動詞) + ん(名詞) + の(連体化) + って

    vec![
        wildcard(1, 1, vec![]),
        surface("の"),                                      // の (any form)
        TokenMatcher::Custom(Arc::new(NaAuxiliaryMatcher)), // な (auxiliary only)
        TokenMatcher::Custom(Arc::new(NNounMatcher)),       // ん
        surface("の"),                                      // の (again)
        TokenMatcher::Custom(Arc::new(TteParticleMatcher)), // って
    ]
}

// Pattern: のなんのって (extremely) - Verb variant
// Structures: Verb[た] + のなんのって
pub fn nonannotte_verb() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Custom matchers for specific POS validation
    #[derive(Debug)]
    struct NoParticleMatcher;
    impl super::Matcher for NoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "の"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "連体化") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NannoMatcher;
    impl super::Matcher for NannoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "なんの"
                        && token.pos.first().is_some_and(|pos| pos == "名詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "固有名詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct TteMatcher;
    impl super::Matcher for TteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "って"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Verb pattern: Wildcard + の(連体化) + なんの(固有名詞) + って
    vec![
        wildcard(1, 1, vec![]),
        TokenMatcher::Custom(Arc::new(NoParticleMatcher)), // の (particle, not noun)
        TokenMatcher::Custom(Arc::new(NannoMatcher)),      // なんの (proper noun)
        TokenMatcher::Custom(Arc::new(TteMatcher)),        // って
    ]
}

// Pattern: にかかっている (depends on)
// Structures: Noun/Phrase + に + かかっている, Noun/Phrase + に + かかっています
pub fn nikakatteiru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match particle に (case particle)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "に"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match verb かかる in te-form (連用タ接続)
    #[derive(Debug)]
    struct KakaruVerbMatcher;
    impl super::Matcher for KakaruVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.base_form == "かかる"
                        && token.pos.first().is_some_and(|p| p == "動詞")
                        && token.features.get(5).is_some_and(|f| f == "連用タ接続") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match て particle
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl super::Matcher for TeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "て"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "接続助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(KakaruVerbMatcher)),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
        verb_base("いる"),
    ]
}

// Pattern: てやまない (never cease to, earnestly)
// Structures: Verb[て] + やまない, Verb[て] + やみません
pub fn teyamanai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match verb やむ in either 未然形 (for やまない) or 連用形 (for やみません)
    #[derive(Debug)]
    struct YamuMatcher;
    impl super::Matcher for YamuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.base_form == "やむ"
                        && token.pos.first().is_some_and(|pos| pos == "動詞")
                        && (token.features.get(5).map(|f| f.as_str()) == Some("未然形")
                            || token.features.get(5).map(|f| f.as_str()) == Some("連用形")) =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match ない (auxiliary) or ません
    #[derive(Debug)]
    struct NaiMasenMatcher;
    impl super::Matcher for NaiMasenMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) => {
                    // Match ない (助動詞)
                    if token.surface == "ない"
                        && token.base_form == "ない"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    {
                        return (true, 1);
                    }
                    // Match ませ (ます, 未然形)
                    if token.surface == "ませ"
                        && token.base_form == "ます"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.features.get(5).map(|f| f.as_str()) == Some("未然形")
                    {
                        return (true, 1);
                    }
                    (false, 0)
                }
                _ => (false, 0),
            }
        }
    }

    // Match ん (for ません)
    #[derive(Debug)]
    struct NMatcher;
    impl super::Matcher for NMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ん"
                        && token.base_form == "ん"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match て or で particle
    #[derive(Debug)]
    struct TeFormMatcher;
    impl super::Matcher for TeFormMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if (token.surface == "て" || token.surface == "で")
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        super::flexible_verb_form(), // Verb in 連用形 or 連用タ接続
        TokenMatcher::Custom(Arc::new(TeFormMatcher)), // て or で
        TokenMatcher::Custom(Arc::new(YamuMatcher)),
        TokenMatcher::Custom(Arc::new(NaiMasenMatcher)),
        optional(TokenMatcher::Custom(Arc::new(NMatcher))), // Optional ん for polite
    ]
}

// Pattern: ぐらいなら (would rather B than A, better off B than A)
// Structures: Verb[dictionary] + ぐらいなら/くらいなら
pub fn gurainara() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ぐらい or くらい as 助詞/副助詞
    #[derive(Debug)]
    struct GuraiKuraiMatcher;
    impl super::Matcher for GuraiKuraiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if (token.surface == "ぐらい" || token.surface == "くらい")
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "副助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match なら (助動詞 with base だ in 仮定形)
    #[derive(Debug)]
    struct NaraMatcher;
    impl super::Matcher for NaraMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "なら"
                        && token.base_form == "だ"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.features.get(5).is_some_and(|f| f == "仮定形") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        verb_form("基本形"), // Dictionary form verb
        TokenMatcher::Custom(Arc::new(GuraiKuraiMatcher)),
        TokenMatcher::Custom(Arc::new(NaraMatcher)),
    ]
}

// Pattern: ってば・ったら (insisting viewpoint / addressing person with frustration)
// Structures: Verb/Adj/Noun + ってば OR Noun/な-Adj + だ + ってば (same for ったら)
//
// Tokenization patterns:
// 1. [Any] + って (助詞/格助詞/連語) + ば (助詞/接続助詞)
//    - Verb: 分かったってば
//    - Noun (person): 金太郎ってば
// 2. [Any] + だって (助詞/副助詞) + ば (助詞/接続助詞)
//    - な-Adj/Noun: 嫌いだってば
// 3. [Noun/な-Adj] + だっ (助動詞 だ, 連用タ接続) + たら (助動詞 た, 仮定形)
//    - Noun + だったら: 大人だったら
//
// Note: い-Adj + ったら tokenizes as いう (verb) + たら - undetectable structurally
//
// This function handles Pattern 1: って + ば
pub fn tteba_u30fb_ttara() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for って (助詞/格助詞/連語)
    #[derive(Debug)]
    struct TteMatcher;
    impl super::Matcher for TteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "って"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "格助詞")
                        && token.pos.get(2).is_some_and(|p| p == "連語") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matcher for ば (助詞/接続助詞)
    #[derive(Debug)]
    struct BaMatcher;
    impl super::Matcher for BaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ば"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "接続助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Pattern: [Any] + って + ば
    vec![
        any(),
        TokenMatcher::Custom(Arc::new(TteMatcher)),
        TokenMatcher::Custom(Arc::new(BaMatcher)),
    ]
}

// Pattern: ってば・ったら - だってば variant
// Handles Pattern 2: だって + ば (e.g., 嫌いだってば)
pub fn tteba_u30fb_ttara_datte() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for だって (助詞/副助詞)
    #[derive(Debug)]
    struct DatteMatcher;
    impl super::Matcher for DatteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "だって"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "副助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matcher for ば (助詞/接続助詞)
    #[derive(Debug)]
    struct BaMatcher;
    impl super::Matcher for BaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ば"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "接続助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Pattern: [Any] + だって + ば
    vec![
        any(),
        TokenMatcher::Custom(Arc::new(DatteMatcher)),
        TokenMatcher::Custom(Arc::new(BaMatcher)),
    ]
}

// Pattern: ってば・ったら - だったら variant
// Handles Pattern 3: だっ + たら (e.g., 大人だったら)
pub fn tteba_u30fb_ttara_dattara() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for nouns/な-adjectives that can take だ copula
    // This pattern is for insisting/frustration, typically after nouns (especially people) or な-adjectives
    // NOT for generic conditionals like "ままだったら" (if it stayed that way)
    #[derive(Debug)]
    struct NounOrNaAdjMatcher;
    impl super::Matcher for NounOrNaAdjMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) => {
                    // Accept nouns, but exclude non-independent nouns like まま, こと, の, etc.
                    // These typically form conditionals, not the insistent ったら pattern
                    if token.pos.first().is_some_and(|p| p == "名詞") {
                        // Exclude 非自立 (non-independent) nouns - these are grammatical, not standalone
                        if token.pos.get(1).is_some_and(|p| p == "非自立") {
                            return (false, 0);
                        }
                        // Exclude 接尾 (suffix) nouns
                        if token.pos.get(1).is_some_and(|p| p == "接尾") {
                            return (false, 0);
                        }
                        // Exclude formal/literary nouns that form conditionals
                        // まま (as is), こと (thing), もの (thing), ところ (place/point), はず (expected)
                        if ["まま", "こと", "もの", "ところ", "はず", "わけ", "つもり"]
                            .contains(&token.base_form.as_str())
                        {
                            return (false, 0);
                        }
                        return (true, 1);
                    }
                    // Accept な-adjectives (形容動詞)
                    if token.pos.first().is_some_and(|p| p == "形容動詞") {
                        return (true, 1);
                    }
                    (false, 0)
                }
                _ => (false, 0),
            }
        }
    }

    // Matcher for だっ (助動詞 だ in 連用タ接続)
    #[derive(Debug)]
    struct DatMatcher;
    impl super::Matcher for DatMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "だっ"
                        && token.base_form == "だ"
                        && token.pos.first().is_some_and(|p| p == "助動詞")
                        && token.features.get(5).is_some_and(|f| f == "連用タ接続") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matcher for たら (助動詞 た in 仮定形), must be sentence-final or followed by ！
    // This distinguishes exasperation だったら！ from conditional だったら、〜
    #[derive(Debug)]
    struct TaraSentenceFinalMatcher;
    impl super::Matcher for TaraSentenceFinalMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "たら"
                        && token.base_form == "た"
                        && token.pos.first().is_some_and(|p| p == "助動詞")
                        && token.features.get(5).is_some_and(|f| f == "仮定形") =>
                {
                    // Check what follows: must be end-of-input, or punctuation (！。)
                    match ctx.lookahead(1) {
                        None => (true, 1), // end of sentence
                        Some(next) => {
                            let is_exclamation = next.surface == "！" || next.surface == "!";
                            let is_period = next.surface == "。";
                            if is_exclamation || is_period {
                                (true, 1)
                            } else {
                                (false, 0)
                            }
                        }
                    }
                }
                _ => (false, 0),
            }
        }
    }

    // Pattern: [Noun/な-Adj] + だっ + たら (sentence-final only)
    vec![
        TokenMatcher::Custom(Arc::new(NounOrNaAdjMatcher)),
        TokenMatcher::Custom(Arc::new(DatMatcher)),
        TokenMatcher::Custom(Arc::new(TaraSentenceFinalMatcher)),
    ]
}

// Pattern: ずとも (even if not / don't have to)
// Structures: Verb[ない] + ず + とも
// Note: とも may tokenize as single token (助詞/接続助詞) or as と + も (two tokens)
pub fn zutomo() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ず (classical negative auxiliary)
    #[derive(Debug)]
    struct ZuMatcher;
    impl super::Matcher for ZuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ず"
                        && token.pos.first().is_some_and(|p| p == "助動詞")
                        && token.base_form == "ぬ" =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match とも as single token (助詞/接続助詞)
    #[derive(Debug)]
    struct TomoMatcher;
    impl super::Matcher for TomoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "とも"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "接続助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match verb in 未然形 or 未然ヌ接続 (nai-stem for classical ず)
    #[derive(Debug)]
    struct NaiStemVerbMatcher;
    impl super::Matcher for NaiStemVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.pos.first().is_some_and(|p| p == "動詞")
                        && token
                            .features
                            .get(5)
                            .is_some_and(|f| f == "未然形" || f == "未然ヌ接続") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Pattern: verb[未然形/未然ヌ接続] + ず + とも (when とも is single token)
    vec![
        TokenMatcher::Custom(Arc::new(NaiStemVerbMatcher)),
        TokenMatcher::Custom(Arc::new(ZuMatcher)),
        TokenMatcher::Custom(Arc::new(TomoMatcher)),
    ]
}

// Pattern: ずとも (split tokenization) - handles when とも splits into と + も
// Structures: Verb[ない] + ず + と + も
pub fn zutomo_split() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ず (classical negative auxiliary)
    #[derive(Debug)]
    struct ZuMatcher;
    impl super::Matcher for ZuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ず"
                        && token.pos.first().is_some_and(|p| p == "助動詞")
                        && token.base_form == "ぬ" =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match と particle
    #[derive(Debug)]
    struct ToParticleMatcher;
    impl super::Matcher for ToParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "と" && token.pos.first().is_some_and(|p| p == "助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match も particle
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl super::Matcher for MoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "も" && token.pos.first().is_some_and(|p| p == "助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match verb in 未然形 or 未然ヌ接続
    #[derive(Debug)]
    struct NaiStemVerbMatcher;
    impl super::Matcher for NaiStemVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.pos.first().is_some_and(|p| p == "動詞")
                        && token
                            .features
                            .get(5)
                            .is_some_and(|f| f == "未然形" || f == "未然ヌ接続") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Pattern: verb[未然形/未然ヌ接続] + ず + と + も (when とも splits)
    vec![
        TokenMatcher::Custom(Arc::new(NaiStemVerbMatcher)),
        TokenMatcher::Custom(Arc::new(ZuMatcher)),
        TokenMatcher::Custom(Arc::new(ToParticleMatcher)),
        TokenMatcher::Custom(Arc::new(MoParticleMatcher)),
    ]
}

// Pattern: とあって (since/because of)
// Structures: Verb/Adjective/Noun + (だ) + とあって
pub fn toatte() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match と particle (either quotative or general case particle)
    #[derive(Debug)]
    struct ToParticleMatcher;
    impl super::Matcher for ToParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "と"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match あっ (verb ある in 連用タ接続 form)
    #[derive(Debug)]
    struct AtteMatcher;
    impl super::Matcher for AtteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.base_form == "ある"
                        && token.pos.first().is_some_and(|pos| pos == "動詞")
                        && token
                            .features
                            .get(5)
                            .is_some_and(|form| form == "連用タ接続") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match て particle
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl super::Matcher for TeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "て"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(ToParticleMatcher)),
        TokenMatcher::Custom(Arc::new(AtteMatcher)),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
    ]
}

// Pattern: でもなんでもない (not at all / definitely not)
// Structures: Noun + でもなんでもない / な-Adj + でもなんでもない / い-Adj + くもなんでもない
pub fn demonandemonai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match で (助詞/格助詞/一般) case particle
    #[derive(Debug)]
    struct DeParticleMatcher;
    impl super::Matcher for DeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "で"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match も (助詞/係助詞)
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl super::Matcher for MoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "も"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "係助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match なん (名詞/代名詞/一般)
    #[derive(Debug)]
    struct NanNounMatcher;
    impl super::Matcher for NanNounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "なん"
                        && token.base_form == "なん"
                        && token.pos.first().is_some_and(|pos| pos == "名詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match ない (形容詞/自立)
    #[derive(Debug)]
    struct NaiAdjectiveMatcher;
    impl super::Matcher for NaiAdjectiveMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ない"
                        && token.base_form == "ない"
                        && token.pos.first().is_some_and(|pos| pos == "形容詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Pattern: [Any] + で + も + なん + で + も + ない
    // This handles nouns: ファン + で + も + なん + で + も + ない
    vec![
        any(),
        TokenMatcher::Custom(Arc::new(DeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(MoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(NanNounMatcher)),
        TokenMatcher::Custom(Arc::new(DeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(MoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(NaiAdjectiveMatcher)),
    ]
}

// Pattern: でもなんでもない (combined でも variant)
// Handles な-adjectives: 迷惑 + でも + なん + で + も + ない
pub fn demonandemonai_demo() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct DemoParticleMatcher;
    impl super::Matcher for DemoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "でも"
                        && token.base_form == "でも"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "副助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NanNounMatcher;
    impl super::Matcher for NanNounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "なん"
                        && token.base_form == "なん"
                        && token.pos.first().is_some_and(|pos| pos == "名詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct DeParticleMatcher;
    impl super::Matcher for DeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "で"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct MoParticleMatcher;
    impl super::Matcher for MoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "も"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "係助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NaiAdjectiveMatcher;
    impl super::Matcher for NaiAdjectiveMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ない"
                        && token.base_form == "ない"
                        && token.pos.first().is_some_and(|pos| pos == "形容詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Pattern: [Any] + でも + なん + で + も + ない
    vec![
        any(),
        TokenMatcher::Custom(Arc::new(DemoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(NanNounMatcher)),
        TokenMatcher::Custom(Arc::new(DeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(MoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(NaiAdjectiveMatcher)),
    ]
}

// Pattern: でもなんでもない (くも variant for い-adjectives)
// Handles い-adjectives: 重く + も + なん + で + も + ない
pub fn demonandemonai_kumo() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match も (助詞/係助詞)
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl super::Matcher for MoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "も"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "係助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match で (助詞/格助詞/一般) case particle
    #[derive(Debug)]
    struct DeParticleMatcher;
    impl super::Matcher for DeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "で"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match なん (名詞/代名詞/一般)
    #[derive(Debug)]
    struct NanNounMatcher;
    impl super::Matcher for NanNounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "なん"
                        && token.base_form == "なん"
                        && token.pos.first().is_some_and(|pos| pos == "名詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match ない (形容詞/自立)
    #[derive(Debug)]
    struct NaiAdjectiveMatcher;
    impl super::Matcher for NaiAdjectiveMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ない"
                        && token.base_form == "ない"
                        && token.pos.first().is_some_and(|pos| pos == "形容詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Pattern: [Any] + も + なん + で + も + ない
    // This handles い-adjectives in ku-form: 重く + も + なん + で + も + ない
    vec![
        any(),
        TokenMatcher::Custom(Arc::new(MoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(NanNounMatcher)),
        TokenMatcher::Custom(Arc::new(DeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(MoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(NaiAdjectiveMatcher)),
    ]
}

// Pattern: ぐるみで (including / all over)
// Structures: Noun + ぐるみで / Noun + ぐるみの
pub fn gurumide() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ぐるみ as noun suffix
    #[derive(Debug)]
    struct GurumiMatcher;
    impl super::Matcher for GurumiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ぐるみ"
                        && token.pos.first().is_some_and(|pos| pos == "名詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "接尾") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match で (case particle) or の (nominalizer)
    #[derive(Debug)]
    struct DeNoMatcher;
    impl super::Matcher for DeNoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if (token.surface == "で"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞"))
                        || (token.surface == "の"
                            && token.pos.first().is_some_and(|pos| pos == "助詞")
                            && token.pos.get(1).is_some_and(|pos| pos == "連体化")) =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Preceding noun
        TokenMatcher::Custom(Arc::new(GurumiMatcher)),
        TokenMatcher::Custom(Arc::new(DeNoMatcher)),
    ]
}

// Pattern: そばから (as soon as / right after)
// Structures: Verb[る] + そばから, Verb[た] + そばから
pub fn sobakara() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match そば (noun "side")
    #[derive(Debug)]
    struct SobaMatcher;
    impl super::Matcher for SobaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "そば"
                        && token.base_form == "そば"
                        && token.pos.first().is_some_and(|pos| pos == "名詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match から as case particle (格助詞)
    #[derive(Debug)]
    struct KaraParticleMatcher;
    impl super::Matcher for KaraParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "から"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match verb in dictionary form or continuative form (連用形/連用タ接続)
    #[derive(Debug)]
    struct VerbBeforeSobaMatcher;
    impl super::Matcher for VerbBeforeSobaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                if !token.pos.first().is_some_and(|pos| pos == "動詞") {
                    return false;
                }
                // Dictionary form (基本形) or continuative forms
                if let Some(form) = token.features.get(5) {
                    form == "基本形" || form == "連用形" || form == "連用タ接続"
                } else {
                    false
                }
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbBeforeSobaMatcher)),
        optional(super::past_auxiliary()), // た/だ for past tense
        TokenMatcher::Custom(Arc::new(SobaMatcher)),
        TokenMatcher::Custom(Arc::new(KaraParticleMatcher)),
    ]
}

// Pattern: 訳あり(訳あって) (for a reason, defective)
// Structures: わけ/訳 + あり + [な/の/で/て/Noun]
pub fn wakeari_yakuatte() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match わけ (hiragana) or 訳 (kanji)
    #[derive(Debug)]
    struct WakeMatcher;
    impl super::Matcher for WakeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if (token.surface == "わけ" || token.surface == "訳")
                        && (token.pos.first().is_some_and(|pos| pos == "名詞")
                            || (token.pos.first().is_some_and(|pos| pos == "動詞")
                                && token.features.get(5).is_some_and(|f| f == "連用形"))) =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match あっ (ある verb in 連用タ接続 for あって form)
    #[derive(Debug)]
    struct AtteMatcher;
    impl super::Matcher for AtteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if (token.surface == "あっ" || token.surface == "あり")
                        && token.pos.first().is_some_and(|pos| pos == "動詞")
                        && token.features.get(5).is_some_and(|f| {
                            f == "連用タ接続" || f == "連用形" || f == "基本形"
                        }) =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match particles/auxiliaries that can follow: な, の, で, て
    #[derive(Debug)]
    struct FollowingParticleMatcher;
    impl super::Matcher for FollowingParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if (token.surface == "な"
                        && token.pos.first().is_some_and(|pos| pos == "助詞"))
                        || (token.surface == "の"
                            && token.pos.first().is_some_and(|pos| pos == "助詞"))
                        || (token.surface == "で"
                            && (token.pos.first().is_some_and(|pos| pos == "助動詞")
                                || token.pos.first().is_some_and(|pos| pos == "助詞")))
                        || (token.surface == "て"
                            && token.pos.first().is_some_and(|pos| pos == "助詞")) =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(WakeMatcher)),
        TokenMatcher::Custom(Arc::new(AtteMatcher)),
        optional(TokenMatcher::Custom(Arc::new(FollowingParticleMatcher))),
    ]
}

// Pattern: に至って・に至り (only when/going as far as)
// Structures: Verb + に至って, Noun + に至って, に至り
pub fn niitatte_u30fb_niitari() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match 至って (te-form) or 至り (conjunctive form)
    #[derive(Debug)]
    struct ItatteMatcher;
    impl super::Matcher for ItatteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.base_form == "至る"
                        && token.pos.first().is_some_and(|pos| pos == "動詞")
                        && (token.surface == "至っ" || token.surface == "至り") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Verb or Noun
        surface("に"),
        TokenMatcher::Custom(Arc::new(ItatteMatcher)),
        // Match optional て particle following 至っ
        optional(surface("て")),
    ]
}

// Pattern: だに + しない
// Pattern: だに + しない (not even, cannot even)
// Structures: Noun + だ + に + し + ない, Noun + だに + Verb + ない
//
// Two tokenization patterns:
// 1. Noun(サ変接続) + だ(助動詞) + に(助詞/格助詞) + し(動詞) + ない
//    e.g., 予想だにしない, 想像だにしない
// 2. Noun + だに(助詞/副助詞) + Verb + ない
//    e.g., 思いだにしない, 夢にだに思わない
pub fn dani_shinai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matches だ auxiliary (特殊・ダ/基本形) - keep custom for feature check
    #[derive(Debug)]
    struct DaAuxiliaryMatcher;
    impl super::Matcher for DaAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "だ"
                        && token.base_form == "だ"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token
                            .features
                            .get(4)
                            .is_some_and(|f| f.as_str() == "特殊・ダ") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matches ない auxiliary - keep custom for POS check
    #[derive(Debug)]
    struct NaiAuxiliaryMatcher;
    impl super::Matcher for NaiAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.base_form == "ない"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Pattern for サ変接続 nouns: Noun + だ + に + し + ない
    vec![
        any(), // Noun (usually サ変接続 like 予想, 想像)
        TokenMatcher::Custom(Arc::new(DaAuxiliaryMatcher)),
        surface("に"),     // に particle
        verb_base("する"), // し from する
        TokenMatcher::Custom(Arc::new(NaiAuxiliaryMatcher)),
    ]
}

// Pattern: Noun + だに (as single particle) + Verb + ない
// This handles cases where だに tokenizes as 助詞/副助詞
pub fn dani_shinai_particle() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matches だに as a single particle (助詞/副助詞) - keep custom for specific POS validation
    #[derive(Debug)]
    struct DaniParticleMatcher;
    impl super::Matcher for DaniParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "だに"
                        && token.base_form == "だに"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "副助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matches ない auxiliary - keep custom for important POS validation
    #[derive(Debug)]
    struct NaiAuxiliaryMatcher;
    impl super::Matcher for NaiAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.base_form == "ない"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Noun (like 思い, 夢)
        TokenMatcher::Custom(Arc::new(DaniParticleMatcher)),
        wildcard(1, 3, vec![]), // Verb phrase (may include て, いる, etc.)
        TokenMatcher::Custom(Arc::new(NaiAuxiliaryMatcher)),
    ]
}

// Pattern: がてら (while doing, on the occasion of)
// Structures: Verb[stem] + がてら, Noun + がてら
pub fn gatera() -> Vec<TokenMatcher> {
    vec![
        any(),           // Verb or Noun
        surface("が"),   // が particle - simplified since がてら context is unambiguous
        surface("てら"), // てら - simplified since がてら is a specific pattern
    ]
}

// Pattern: んがため(に) (for the purpose of, in order to)
// Structures: Verb[未然形] + ん + が + ため + に/の
pub fn ngatame_ni() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match verb in 未然形 or 連用形 - keep custom for complex feature check
    #[derive(Debug)]
    struct VerbBeforeNMatcher;
    impl super::Matcher for VerbBeforeNMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.pos.first().is_some_and(|pos| pos == "動詞")
                        && token
                            .features
                            .get(5)
                            .is_some_and(|f| f == "未然形" || f == "連用形") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbBeforeNMatcher)),
        surface("ん"),   // ん - simplified since context makes it unambiguous
        surface("が"),   // が - simplified since んが context is specific
        surface("ため"), // ため - simplified since んがため is specific
        optional(or(vec![surface("に"), surface("の")])), // に or の particles
    ]
}

// Pattern: いかん〜ず (regardless of, irrespective of)
// Structures: Noun + の + いかん + に/を + かかわら/よら/とわ + ず
pub fn ikan_u301c_zu() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match いかん as noun
    #[derive(Debug)]
    struct IkanNounMatcher;
    impl super::Matcher for IkanNounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "いかん"
                        && token.base_form == "いかん"
                        && token.pos.first().is_some_and(|pos| pos == "名詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match に or を particle
    #[derive(Debug)]
    struct NiOrWoParticleMatcher;
    impl super::Matcher for NiOrWoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if (token.surface == "に" || token.surface == "を")
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match かかわら/よら/とわ verbs in 未然形
    #[derive(Debug)]
    struct IkanVerbMatcher;
    impl super::Matcher for IkanVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.pos.first().is_some_and(|pos| pos == "動詞")
                        && (token.base_form == "かかわる"
                            || token.base_form == "よる"
                            || token.base_form == "とう")
                        && token.features.get(5).is_some_and(|f| f == "未然形") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match ず (classical negative auxiliary)
    #[derive(Debug)]
    struct ZuMatcher;
    impl super::Matcher for ZuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ず"
                        && token.base_form == "ぬ"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(IkanNounMatcher)),
        TokenMatcher::Custom(Arc::new(NiOrWoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(IkanVerbMatcher)),
        TokenMatcher::Custom(Arc::new(ZuMatcher)),
    ]
}

// Pattern: にも～ない (can't do even if wanted to)
// Structures: Verb[volitional] + にも + Verb[potential negative]
// This is a more specific variant of 〜に〜ない requiring volitional + にも
pub fn nimo_uff5e_nai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for volitional う/よう
    #[derive(Debug)]
    struct VolitionalMatcher;
    impl super::Matcher for VolitionalMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Godan verbs: う auxiliary
                if token.surface == "う"
                    && token.base_form == "う"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                {
                    return true;
                }
                // Ichidan verbs: よう noun suffix
                if token.surface == "よう"
                    && token.base_form == "よう"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                {
                    return true;
                }
                false
            })
        }
    }

    // Matcher for に particle
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "に"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matcher for も particle (REQUIRED for this pattern)
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl super::Matcher for MoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "も"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "係助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matcher for ない auxiliary
    #[derive(Debug)]
    struct NaiMatcher;
    impl super::Matcher for NaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.base_form == "ない"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        // Start with any verb (will be before volitional)
        any(),
        // Volitional form (う or よう)
        TokenMatcher::Custom(Arc::new(VolitionalMatcher)),
        // に particle
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        // も particle (required)
        TokenMatcher::Custom(Arc::new(MoParticleMatcher)),
        // Match tokens until ない (potential verb forms, auxiliary verbs, etc.)
        wildcard(1, 5, vec![]),
        // End with ない
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
    ]
}

// Pattern: い-Adj[く] + もなんともない (not A at all, definitely not A)
// Structures: い-Adjective[く] + もなんともない, Verb[stem] + たく + もなんともない
pub fn i_adj_ku_monantomonai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match い-adjectives in く form (連用テ接続/連用形)
    // OR たい auxiliary in たく form (連用テ接続)
    #[derive(Debug)]
    struct IAdjKuOrTaiKuMatcher;
    impl super::Matcher for IAdjKuOrTaiKuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match い-adjectives ending in く (連用テ接続 or 連用形)
                let is_i_adj_ku = token.pos.first().is_some_and(|pos| pos == "形容詞")
                    && token.surface.ends_with("く")
                    && token
                        .features
                        .get(5)
                        .is_some_and(|f| f == "連用テ接続" || f == "連用形");

                // Match たい auxiliary in たく form (連用テ接続)
                let is_tai_ku = token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.base_form == "たい"
                    && token.surface == "たく"
                    && token.features.get(5).is_some_and(|f| f == "連用テ接続");

                is_i_adj_ku || is_tai_ku
            })
        }
    }

    // Match も particle (係助詞)
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl super::Matcher for MoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "も"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "係助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match なんとも adverb
    #[derive(Debug)]
    struct NantomoMatcher;
    impl super::Matcher for NantomoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "なんとも"
                        && token.pos.first().is_some_and(|pos| pos == "副詞")
                        && token.base_form == "なんとも" =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match ない adjective (基本形)
    #[derive(Debug)]
    struct NaiMatcher;
    impl super::Matcher for NaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ない"
                        && token.pos.first().is_some_and(|pos| pos == "形容詞")
                        && token.base_form == "ない" =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(IAdjKuOrTaiKuMatcher)),
        TokenMatcher::Custom(Arc::new(MoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(NantomoMatcher)),
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
    ]
}

// Pattern: Verb + だに (just, merely, even)
// Structures: Verb[dictionary form] + だに
pub fn verb_dani() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match だに particle (助詞/副助詞)
    #[derive(Debug)]
    struct DaniParticleMatcher;
    impl super::Matcher for DaniParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "だに"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "副助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        // Match verbs in dictionary form (基本形)
        verb_form("基本形"),
        TokenMatcher::Custom(Arc::new(DaniParticleMatcher)),
    ]
}

// Pattern: ～なり～なり (either...or...)
// Structures: Item1 + なり + Item2 + なり
// Item can be: Verb(dictionary), Noun, or Noun + Particle
// Note: なり can be 助詞/接続助詞, 助詞/並立助詞, or 助詞/副助詞
pub fn uff5e_nari_uff5e_nari() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Custom matcher for なり as adverbial particle (not verb なる)
    #[derive(Debug)]
    struct NariParticleMatcher;
    impl super::Matcher for NariParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                if token.surface != "なり" {
                    return false;
                }
                if !token.pos.first().is_some_and(|pos| pos == "助詞") {
                    return false;
                }
                // Check if second POS element is one of the valid particle types
                token
                    .pos
                    .get(1)
                    .is_some_and(|p| p == "接続助詞" || p == "並立助詞" || p == "副助詞")
            })
        }
    }

    // Pattern: (Verb|Noun) + なり + (1-8 tokens) + なり
    // Simple pattern without wildcards between item and なり for now
    vec![
        // Match verb or noun
        or(vec![verb(), noun()]),
        TokenMatcher::Custom(Arc::new(NariParticleMatcher)),
        wildcard(1, 8, vec![]),
        TokenMatcher::Custom(Arc::new(NariParticleMatcher)),
    ]
}

// Pattern: ないでもない (kind of / might / not not)
// Meaning: "Kind of (A)" / "Might (A)" - neither confirming nor denying
// Structures: Verb[ない] + (もの) + で + も + ない
pub fn naidemonai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ない as auxiliary (negative form after verb)
    #[derive(Debug)]
    struct NaiAuxiliaryMatcher;
    impl super::Matcher for NaiAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ない"
                        && (token.pos.first().is_some_and(|pos| pos == "助動詞")
                            || token.pos.first().is_some_and(|pos| pos == "形容詞")) =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match で as auxiliary copula or conjunctive particle
    #[derive(Debug)]
    struct DeCopulaMatcher;
    impl super::Matcher for DeCopulaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "で"
                        && (token.base_form == "だ" || token.base_form == "で") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match も as binding particle
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl super::Matcher for MoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "も"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "係助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match final ない (adjective or auxiliary)
    #[derive(Debug)]
    struct NaiFinalMatcher;
    impl super::Matcher for NaiFinalMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ない"
                        && (token.pos.first().is_some_and(|pos| pos == "形容詞")
                            || token.pos.first().is_some_and(|pos| pos == "助動詞")) =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match もの as non-autonomous noun (optional)
    #[derive(Debug)]
    struct MonoNounMatcher;
    impl super::Matcher for MonoNounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "もの"
                        && token.pos.first().is_some_and(|pos| pos == "名詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "非自立") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NaiAuxiliaryMatcher)),
        optional(TokenMatcher::Custom(Arc::new(MonoNounMatcher))),
        TokenMatcher::Custom(Arc::new(DeCopulaMatcher)),
        TokenMatcher::Custom(Arc::new(MoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(NaiFinalMatcher)),
    ]
}

// Pattern: ないではない (kind of / might / not not) - では variant
// Meaning: "Kind of (A)" / "Might (A)" - neither confirming nor denying
// Structures: ない + で + は + ない
pub fn naidewanai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ない as auxiliary or adjective
    #[derive(Debug)]
    struct NaiMatcher;
    impl super::Matcher for NaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ない"
                        && (token.pos.first().is_some_and(|pos| pos == "形容詞")
                            || token.pos.first().is_some_and(|pos| pos == "助動詞")) =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match で as auxiliary copula
    #[derive(Debug)]
    struct DeCopulaMatcher;
    impl super::Matcher for DeCopulaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "で" && token.base_form == "だ" => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match は as binding particle
    #[derive(Debug)]
    struct WaParticleMatcher;
    impl super::Matcher for WaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "は"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "係助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
        TokenMatcher::Custom(Arc::new(DeCopulaMatcher)),
        TokenMatcher::Custom(Arc::new(WaParticleMatcher)),
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
    ]
}

// Pattern: もさることながら (not only A but also B)
// Structures: Noun + もさることながら
pub fn mosarukotonagara() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match も particle (係助詞)
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl super::Matcher for MoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "も"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "係助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match さる (連体詞)
    #[derive(Debug)]
    struct SaruRentaishiMatcher;
    impl super::Matcher for SaruRentaishiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "さる"
                        && token.base_form == "さる"
                        && token.pos.first().is_some_and(|p| p == "連体詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match こと noun
    #[derive(Debug)]
    struct KotoNounMatcher;
    impl super::Matcher for KotoNounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "こと"
                        && token.base_form == "こと"
                        && token.pos.first().is_some_and(|p| p == "名詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match ながら particle (接続助詞)
    #[derive(Debug)]
    struct NagaraParticleMatcher;
    impl super::Matcher for NagaraParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ながら"
                        && token.base_form == "ながら"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "接続助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        super::noun(),
        TokenMatcher::Custom(Arc::new(MoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(SaruRentaishiMatcher)),
        TokenMatcher::Custom(Arc::new(KotoNounMatcher)),
        TokenMatcher::Custom(Arc::new(NagaraParticleMatcher)),
    ]
}

// Pattern: ものと思っていた (was under the impression that)
// Structures: [Attributive form] + ものと思（おも）っていた/いました
pub fn monotoomotteita() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match もの (dependent noun)
    #[derive(Debug)]
    struct MonoNounMatcher;
    impl super::Matcher for MonoNounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "もの"
                        && token.base_form == "もの"
                        && token.pos.first().is_some_and(|p| p == "名詞")
                        && token.pos.get(1).is_some_and(|p| p == "非自立") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match と (quotation particle)
    #[derive(Debug)]
    struct ToQuotationMatcher;
    impl super::Matcher for ToQuotationMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "と"
                        && token.base_form == "と"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "格助詞")
                        && token.pos.get(2).is_some_and(|p| p == "引用") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match 思っ (verb "思う" in 連用タ接続)
    #[derive(Debug)]
    struct OmotMatcher;
    impl super::Matcher for OmotMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "思っ"
                        && token.base_form == "思う"
                        && token.pos.first().is_some_and(|p| p == "動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match て particle
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl super::Matcher for TeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "て"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "接続助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match い (from いる verb in 連用形)
    #[derive(Debug)]
    struct IruRenyouMatcher;
    impl super::Matcher for IruRenyouMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "い"
                        && token.base_form == "いる"
                        && token.pos.first().is_some_and(|p| p == "動詞")
                        && token.pos.get(1).is_some_and(|p| p == "非自立") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match た (past auxiliary) or まし+た (polite past)
    // We use any() for flexibility to match both patterns

    vec![
        any(), // Previous word in attributive form (verb/adj/noun+の)
        TokenMatcher::Custom(Arc::new(MonoNounMatcher)),
        TokenMatcher::Custom(Arc::new(ToQuotationMatcher)),
        TokenMatcher::Custom(Arc::new(OmotMatcher)),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(IruRenyouMatcher)),
        wildcard(1, 2, vec![]), // Matches た (1 token) or まし+た (2 tokens)
    ]
}

// Pattern: でなくてなんだろう (if not A, then what is it?)
// Structures: Noun + でなくてなん + だろう/であろう + (か)
pub fn denakutenandarou() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match で (助動詞, 特殊・ダ, 連用形)
    #[derive(Debug)]
    struct DeMatcher;
    impl super::Matcher for DeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "で"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.base_form == "だ"
                        && token.features.get(5).is_some_and(|f| f == "連用形") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match なく (助動詞, 特殊・ナイ, 連用テ接続)
    #[derive(Debug)]
    struct NakuMatcher;
    impl super::Matcher for NakuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "なく"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.base_form == "ない"
                        && token.features.get(5).is_some_and(|f| f == "連用テ接続") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match なん (名詞, 代名詞)
    #[derive(Debug)]
    struct NanMatcher;
    impl super::Matcher for NanMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "なん"
                        && token.pos.first().is_some_and(|pos| pos == "名詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "代名詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match だろ/で (for だろう or であろう)
    #[derive(Debug)]
    struct DaroOrDeMatcher;
    impl super::Matcher for DaroOrDeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && ((token.surface == "だろ"
                            && token.base_form == "だ"
                            && token.features.get(5).is_some_and(|f| f == "未然形"))
                            || (token.surface == "で"
                                && token.base_form == "だ"
                                && token.features.get(5).is_some_and(|f| f == "連用形"))) =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match あろ (optional, for であろう)
    #[derive(Debug)]
    struct AroMatcher;
    impl super::Matcher for AroMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "あろ"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.base_form == "ある"
                        && token.features.get(5).is_some_and(|f| f == "未然ウ接続") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match う (助動詞, 不変化型)
    #[derive(Debug)]
    struct UMatcher;
    impl super::Matcher for UMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "う"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.features.get(4).is_some_and(|f| f == "不変化型") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match か (optional)
    #[derive(Debug)]
    struct KaMatcher;
    impl super::Matcher for KaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "か"
                        && token.pos.first().is_some_and(|pos| pos == "助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        super::noun(),
        TokenMatcher::Custom(Arc::new(DeMatcher)),
        TokenMatcher::Custom(Arc::new(NakuMatcher)),
        surface("て"),
        TokenMatcher::Custom(Arc::new(NanMatcher)),
        TokenMatcher::Custom(Arc::new(DaroOrDeMatcher)),
        optional(TokenMatcher::Custom(Arc::new(AroMatcher))),
        TokenMatcher::Custom(Arc::new(UMatcher)),
        optional(TokenMatcher::Custom(Arc::new(KaMatcher))),
    ]
}

// Pattern: はさておき・はさておいて (leaving aside, apart from)
// Structures: Noun/Phrase + は + さておき/さておいて
pub fn hasateoki_u30fb_hasateoite() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for は particle (係助詞)
    #[derive(Debug)]
    struct HaKakariMatcher;
    impl super::Matcher for HaKakariMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "は"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "係助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matcher for さておき (連用形) or さておい (連用タ接続)
    #[derive(Debug)]
    struct SateokiMatcher;
    impl super::Matcher for SateokiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.base_form == "さておく"
                        && token.pos.first().is_some_and(|pos| pos == "動詞")
                        && (token.surface == "さておき" || token.surface == "さておい")
                        && token
                            .features
                            .get(5)
                            .is_some_and(|f| f == "連用形" || f == "連用タ接続") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matcher for optional て particle
    #[derive(Debug)]
    struct TeMatcher;
    impl super::Matcher for TeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "て"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(HaKakariMatcher)),
        TokenMatcher::Custom(Arc::new(SateokiMatcher)),
        optional(TokenMatcher::Custom(Arc::new(TeMatcher))),
    ]
}

// Pattern: 折には (on occasions when, when the chance comes up)
// Structures: Verb/Adj/Noun + おり/折 + に + (は)
pub fn oriniha() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for おり or 折 as noun
    #[derive(Debug)]
    struct OriMatcher;
    impl super::Matcher for OriMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.pos.first().is_some_and(|pos| pos == "名詞")
                        && (token.surface == "おり" || token.surface == "折")
                        && (token.base_form == "おり" || token.base_form == "折") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matcher for に particle
    #[derive(Debug)]
    struct NiMatcher;
    impl super::Matcher for NiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "に"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matcher for optional は particle
    #[derive(Debug)]
    struct HaMatcher;
    impl super::Matcher for HaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "は"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "係助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(OriMatcher)),
        TokenMatcher::Custom(Arc::new(NiMatcher)),
        optional(TokenMatcher::Custom(Arc::new(HaMatcher))),
    ]
}

// Pattern: とばかり（に） (as if to say / seeming that)
// Structures: Quote/Noun + と + ばかり + (に)
pub fn tobakari_uff08_ni_uff09() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match と (quotation particle)
    #[derive(Debug)]
    struct ToQuotationMatcher;
    impl super::Matcher for ToQuotationMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "と"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                        && token.pos.get(2).is_some_and(|pos| pos == "引用") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match ばかり (副助詞)
    #[derive(Debug)]
    struct BakariMatcher;
    impl super::Matcher for BakariMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ばかり"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "副助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match に (case particle) - optional
    #[derive(Debug)]
    struct NiCaseMatcher;
    impl super::Matcher for NiCaseMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "に"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                        && token.pos.get(2).is_some_and(|pos| pos == "一般") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(ToQuotationMatcher)),
        TokenMatcher::Custom(Arc::new(BakariMatcher)),
        optional(TokenMatcher::Custom(Arc::new(NiCaseMatcher))),
    ]
}

// Pattern: わ〜わ (more and more / keeps happening / so much)
// Structures: Verb(dictionary) + わ + Verb(dictionary) + わ
// Note: Cannot verify that the same verb is repeated (requires cross-token validation).
// This will match any Verb + わ + Verb + わ pattern, which may include false positives.
pub fn wa_u301c_wa() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match わ particle (助詞/終助詞)
    #[derive(Debug)]
    struct WaParticleMatcher;
    impl super::Matcher for WaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "わ"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "終助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        verb(),
        TokenMatcher::Custom(Arc::new(WaParticleMatcher)),
        verb(),
        TokenMatcher::Custom(Arc::new(WaParticleMatcher)),
    ]
}

// Pattern: なりとも (at least, even)
// Structures: Noun + なりとも
//
// Note: This pattern has two tokenization variants:
// 1. Split: Noun + なり(助動詞) + と(助詞) + も(助詞) - e.g., 一度なりとも, 一時なりとも, 一目なりとも
// 2. Compound adverb: 多少なりとも, わずかなりとも (副詞/一般 as single token)
//
// The compound adverb case is undetectable with the current 4-token pattern matcher.
// TODO: Consider adding a separate pattern for compound adverbs ending in なりとも.
pub fn naritomo() -> Vec<TokenMatcher> {
    use super::Matcher;
    use std::sync::Arc;

    // Matcher for なり (助動詞/文語・ナリ)
    #[derive(Debug)]
    struct NariAuxiliaryMatcher;
    impl super::Matcher for NariAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "なり"
                        && token.pos.first().is_some_and(|p| p == "助動詞")
                        && token.base_form == "なり" =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matcher for と particle (格助詞/引用)
    #[derive(Debug)]
    struct ToParticleMatcher;
    impl super::Matcher for ToParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "と"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matcher for も particle (係助詞)
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl super::Matcher for MoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "も"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "係助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matches split pattern: Noun + なり + と + も
    vec![
        super::noun(),
        TokenMatcher::Custom(Arc::new(NariAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(ToParticleMatcher)),
        TokenMatcher::Custom(Arc::new(MoParticleMatcher)),
    ]
}

// Pattern: に至っても (even when it reaches/even if it comes to)
// Structures: Verb + に至（いた）っても / Noun + に至（いた）っても
pub fn niitattemo() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match 至っ (te-form only for this pattern)
    #[derive(Debug)]
    struct ItatteVerbMatcher;
    impl super::Matcher for ItatteVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.base_form == "至る"
                        && token.pos.first().is_some_and(|pos| pos == "動詞")
                        && token.surface == "至っ" =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match て particle
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl super::Matcher for TeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "て"
                        && token.pos.first().is_some_and(|pos| pos == "助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match も particle
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl super::Matcher for MoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "も"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "係助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Verb or Noun
        surface("に"),
        TokenMatcher::Custom(Arc::new(ItatteVerbMatcher)),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(MoParticleMatcher)),
    ]
}

// Pattern: を兼ねて (also partly for the purpose of / to double as)
// Structures: Noun + を/も + 兼（か）ねて
pub fn wokanete() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match を or も particle
    #[derive(Debug)]
    struct WoOrMoParticleMatcher;
    impl super::Matcher for WoOrMoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if (token.surface == "を" || token.surface == "も")
                        && token.pos.first().is_some_and(|pos| pos == "助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        noun(),                                                // Noun
        TokenMatcher::Custom(Arc::new(WoOrMoParticleMatcher)), // を or も
        verb_base("かねる"),                                   // かね
        surface("て"),                                         // て
    ]
}

// Pattern: Verb[ない]もの(だろう)か (if only, isn't there a way to)
// Structures: Verb[ない] + もの + (だろう/でしょう) + か
pub fn verb_nai_mono_darou_ka() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ない (助動詞 or 形容詞 for existence)
    #[derive(Debug)]
    struct NaiMatcher;
    impl super::Matcher for NaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ない"
                        && token.base_form == "ない"
                        && (token.pos.first().is_some_and(|p| p == "助動詞")
                            || token.pos.first().is_some_and(|p| p == "形容詞")) =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match だろ or でしょ (助動詞, 未然形 of だ or です)
    #[derive(Debug)]
    struct DarouDeshouMatcher;
    impl super::Matcher for DarouDeshouMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if (token.surface == "だろ" || token.surface == "でしょ")
                        && (token.base_form == "だ" || token.base_form == "です")
                        && token.pos.first().is_some_and(|p| p == "助動詞")
                        && token.features.get(5).is_some_and(|f| f.contains("未然")) =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Pattern: ない + もの + Optional(だろう/でしょう) + か
    vec![
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
        surface("もの"), // もの
        optional(TokenMatcher::Custom(Arc::new(DarouDeshouMatcher))),
        optional(surface("う")), // う
        surface("か"),           // か
    ]
}

// Pattern: Verb[て] + みせる (I will definitely do, I swear I will do)
// Structures: Verb[て] + みせる/みせます
pub fn verb_te_miseru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match て or で particle (接続助詞)
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl super::Matcher for TeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if (token.surface == "て" || token.surface == "で")
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match みせる as auxiliary verb (動詞/非自立)
    #[derive(Debug)]
    struct MiseruAuxiliaryMatcher;
    impl super::Matcher for MiseruAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.base_form == "みせる"
                        && token.pos.first().is_some_and(|pos| pos == "動詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "非自立") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(MiseruAuxiliaryMatcher)),
    ]
}

// Pattern: 相まって (combined with, coupled with)
// Structures: Noun + が/と + 相まって
pub fn aimatte() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match が or と particle
    #[derive(Debug)]
    struct GaToParticleMatcher;
    impl super::Matcher for GaToParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if (token.surface == "が" || token.surface == "と")
                        && token.pos.first().is_some_and(|pos| pos == "助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        noun(),
        TokenMatcher::Custom(Arc::new(GaToParticleMatcher)),
        surface("相まって"), // 相まって
    ]
}

// Pattern: に足りない (not worth / not sufficient)
// Structures: Verb[基本形] + に + 足りない
pub fn nitarinai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match に particle
    #[derive(Debug)]
    struct NiMatcher;
    impl super::Matcher for NiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "に"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match ない auxiliary
    #[derive(Debug)]
    struct NaiMatcher;
    impl super::Matcher for NaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ない"
                        && token.base_form == "ない"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        verb_form("基本形"),
        TokenMatcher::Custom(Arc::new(NiMatcher)),
        verb_base("たりる"), // たり
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
    ]
}

// Pattern: べからず (must not, ought not to)
// Structures: Verb[基本形] + べから + ず/ざる (+ Noun)
pub fn bekarazu() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match べから (助動詞, base=べし, 未然形)
    #[derive(Debug)]
    struct BekaraMatcher;
    impl super::Matcher for BekaraMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "べから"
                        && token.base_form == "べし"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.features.get(5).is_some_and(|f| f == "未然形") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match ず (助動詞, base=ぬ, 連用ニ接続)
    #[derive(Debug)]
    struct ZuMatcher;
    impl super::Matcher for ZuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ず"
                        && token.base_form == "ぬ"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.features.get(5).is_some_and(|f| f == "連用ニ接続") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match ざる (助動詞, base=ぬ, 体言接続)
    #[derive(Debug)]
    struct ZaruMatcher;
    impl super::Matcher for ZaruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ざる"
                        && token.base_form == "ぬ"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.features.get(5).is_some_and(|f| f == "体言接続") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        verb_form("基本形"),
        TokenMatcher::Custom(Arc::new(BekaraMatcher)),
        or(vec![
            TokenMatcher::Custom(Arc::new(ZuMatcher)),
            TokenMatcher::Custom(Arc::new(ZaruMatcher)),
        ]),
    ]
}

// Pattern: んばかりに (as if about to, seeming that it will)
// Structures: Verb[未然形] + ん + ばかり + に/の
//            Verb[体言接続特殊] + ばかり + に/の
pub fn nbakarini() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for verb in 未然形 or 体言接続特殊 form
    #[derive(Debug)]
    struct VerbMizenOrTaigenMatcher;
    impl super::Matcher for VerbMizenOrTaigenMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                if !token.pos.first().is_some_and(|pos| pos == "動詞") {
                    return false;
                }

                // Match 未然形 (before ん auxiliary) or 体言接続特殊 (verb ending with ん)
                token
                    .features
                    .get(5)
                    .is_some_and(|f| f == "未然形" || f == "体言接続特殊")
            })
        }
    }

    // Matcher for ん auxiliary verb
    #[derive(Debug)]
    struct NAuxMatcher;
    impl super::Matcher for NAuxMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ん"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.base_form == "ん"
            })
        }
    }

    // Matcher for ばかり particle
    #[derive(Debug)]
    struct BakariMatcher;
    impl super::Matcher for BakariMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ばかり"
                    && token.base_form == "ばかり"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "副助詞")
            })
        }
    }

    // Matcher for に or の particle
    #[derive(Debug)]
    struct NiOrNoParticleMatcher;
    impl super::Matcher for NiOrNoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.surface == "に" || token.surface == "の")
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
            })
        }
    }

    // Two patterns:
    // 1. Verb[体言接続特殊] (ends with ん) + ばかり + (に/の)?
    // 2. Verb[未然形] + ん(助動詞) + ばかり + (に/の)?

    vec![
        TokenMatcher::Custom(Arc::new(VerbMizenOrTaigenMatcher)),
        optional(TokenMatcher::Custom(Arc::new(NAuxMatcher))),
        TokenMatcher::Custom(Arc::new(BakariMatcher)),
        optional(TokenMatcher::Custom(Arc::new(NiOrNoParticleMatcher))),
    ]
}

// Pattern: に則って・に則り (in accordance with, based on)
// Structures: Noun + に則（のっと）って/に則（のっと）った/に則（のっと）り/に則（のっと）っての
//
// Matches: Noun + に + 則る/のっとる (in any conjugation)
// The verb 則る/のっとる means "to follow" or "to conform to"
pub fn ninottotte_u30fb_ninottori() -> Vec<TokenMatcher> {
    vec![
        noun(),
        surface("に"),
        or(vec![verb_base("則る"), verb_base("のっとる")]),
    ]
}

// Pattern: Adj限りだ (extremely, as ~ as can be)
// Structures: Adjective + (な) + 限り + だ/です
//
// Examples:
// - 羨ましい限りだ (very enviable)
// - 残念な限りだ (very disappointing)
// - 嬉しい限りです (very happy - polite)
//
// Meaning: "extremely (A)", "as (A) as can be" - the limit of (A)
// Formal expression highlighting intensity of traits/emotions
//
// Tokenization:
// - い-Adj (形容詞/自立, 基本形) + 限り + だ/です
// - な-Adj (名詞/形容動詞語幹) + な (助動詞/体言接続) + 限り + だ/です
pub fn adjkagirida() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match な copula (for な-adjectives)
    #[derive(Debug)]
    struct NaCopulaMatcher;
    impl super::Matcher for NaCopulaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "な"
                    && token.base_form == "だ"
                    && token.pos.first().is_some_and(|p| p == "助動詞")
                    && token.features.get(5).is_some_and(|f| f == "体言接続")
            })
        }
    }

    // Match 限り noun
    #[derive(Debug)]
    struct KagiriMatcher;
    impl super::Matcher for KagiriMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "限り"
                    && token.base_form == "限り"
                    && token.pos.first().is_some_and(|p| p == "名詞")
                    && token.pos.get(1).is_some_and(|p| p == "非自立")
            })
        }
    }

    // Match だ or です auxiliary
    #[derive(Debug)]
    struct DaDesuMatcher;
    impl super::Matcher for DaDesuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.pos.first().is_some_and(|p| p == "助動詞")
                    && token.features.get(5).is_some_and(|f| f == "基本形")
                    && ((token.surface == "だ" && token.base_form == "だ")
                        || (token.surface == "です" && token.base_form == "です"))
            })
        }
    }

    vec![
        adjective(),
        optional(TokenMatcher::Custom(Arc::new(NaCopulaMatcher))),
        TokenMatcher::Custom(Arc::new(KagiriMatcher)),
        TokenMatcher::Custom(Arc::new(DaDesuMatcher)),
    ]
}

// Pattern: はおろか (let alone, not to mention)
// Structures: Noun + は + おろか (+ comma + Noun + さえ/も/すら/まで)
pub fn haoroka() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match "おろか" as a noun
    #[derive(Debug)]
    struct OrokaMatcher;
    impl super::Matcher for OrokaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "おろか"
                    && token.base_form == "おろか"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
            })
        }
    }

    vec![
        noun(),
        surface("は"),
        TokenMatcher::Custom(Arc::new(OrokaMatcher)),
    ]
}

// Pattern: めく・めいた (shows signs of, has appearance of)
// Structures: Noun + めく/めいて/めいている/めいた
//
// Two tokenization patterns:
// 1. Split: Noun + めく(動詞/非自立, base=めく) - e.g., 謎めく, 冗談めく, 皮肉めく
// 2. Compound: Single token verb (動詞/自立, base=春めく/夏めく/冬めく) - e.g., 春めく, 夏めく, 冬めく
//
// Pattern includes full construction:
// - Split form: Noun + めく/めい (e.g., 謎めいた, 冗談めいて)
// - Compound form: Just verb token (e.g., 春めい, 夏めいて)
pub fn meku_u30fb_meita() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for めく verb in any form
    // Matches both split (動詞/非自立, base=めく) and compound (動詞/自立, base ends with めく)
    #[derive(Debug)]
    struct MekuVerbMatcher;
    impl super::Matcher for MekuVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                if !token.pos.first().is_some_and(|pos| pos == "動詞") {
                    return false;
                }

                // Split form: めく as 動詞/非自立
                if token.base_form == "めく" && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                {
                    return true;
                }

                // Compound form: verb with base ending in めく as 動詞/自立
                if token.base_form.ends_with("めく")
                && token.base_form != "めく"  // Exclude bare めく
                && token.pos.get(1).is_some_and(|pos| pos == "自立")
                {
                    return true;
                }

                false
            })
        }
    }

    vec![
        optional(super::noun()),
        TokenMatcher::Custom(Arc::new(MekuVerbMatcher)),
    ]
}

// Pattern: といわず (not just...but also everything)
// Structures: Noun (A) + といわず + Noun (B) + といわず
pub fn toiwazu() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match と as 助詞/格助詞/引用
    #[derive(Debug)]
    struct ToParticleMatcher;
    impl super::Matcher for ToParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "と"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                        && token.pos.get(2).is_some_and(|pos| pos == "引用") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match いわ (未然形 of いう)
    #[derive(Debug)]
    struct IwaMatcher;
    impl super::Matcher for IwaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "いわ"
                        && token.base_form == "いう"
                        && token.pos.first().is_some_and(|pos| pos == "動詞")
                        && token.features.get(5).is_some_and(|form| form == "未然形") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match ず (助動詞/特殊・ヌ/連用ニ接続, base=ぬ)
    #[derive(Debug)]
    struct ZuMatcher;
    impl super::Matcher for ZuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ず"
                        && token.base_form == "ぬ"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Pattern: Noun(s) + と + いわ + ず (appears twice)
    // We'll match: Noun+ + と + いわ + ず + (optional comma) + Noun+ + と + いわ + ず
    // Note: Nouns can be compound (multiple noun tokens like 仕事中 = 仕事 + 中)
    vec![
        // First noun phrase (one or more consecutive nouns)
        noun(),
        optional(noun()),
        optional(noun()),
        TokenMatcher::Custom(Arc::new(ToParticleMatcher)),
        TokenMatcher::Custom(Arc::new(IwaMatcher)),
        TokenMatcher::Custom(Arc::new(ZuMatcher)),
        // Optional comma between the two occurrences
        optional(surface("、")),
        // Second noun phrase (one or more consecutive nouns)
        noun(),
        optional(noun()),
        optional(noun()),
        TokenMatcher::Custom(Arc::new(ToParticleMatcher)),
        TokenMatcher::Custom(Arc::new(IwaMatcher)),
        TokenMatcher::Custom(Arc::new(ZuMatcher)),
    ]
}

// Pattern: にもほどがある (there is a limit to)
// Structures: Word + にも + ほどがある
pub fn nimohodogaaru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match に as 助詞/格助詞
    #[derive(Debug)]
    struct NiMatcher;
    impl super::Matcher for NiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "に"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match も as 助詞/係助詞
    #[derive(Debug)]
    struct MoMatcher;
    impl super::Matcher for MoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "も"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "係助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match ほど as 助詞/副助詞
    #[derive(Debug)]
    struct HodoMatcher;
    impl super::Matcher for HodoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ほど"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "副助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match が as 助詞/格助詞
    #[derive(Debug)]
    struct GaMatcher;
    impl super::Matcher for GaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "が"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match ある verb (基本形 or 連用形 for あり+ます)
    #[derive(Debug)]
    struct AruMatcher;
    impl super::Matcher for AruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if (token.surface == "ある" || token.surface == "あり")
                        && token.base_form == "ある"
                        && token.pos.first().is_some_and(|pos| pos == "動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match ます auxiliary
    #[derive(Debug)]
    struct MasuMatcher;
    impl super::Matcher for MasuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ます" && token.base_form == "ます" => {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Preceding word (verb, adjective, or noun)
        TokenMatcher::Custom(Arc::new(NiMatcher)),
        TokenMatcher::Custom(Arc::new(MoMatcher)),
        TokenMatcher::Custom(Arc::new(HodoMatcher)),
        TokenMatcher::Custom(Arc::new(GaMatcher)),
        TokenMatcher::Custom(Arc::new(AruMatcher)),
        optional(TokenMatcher::Custom(Arc::new(MasuMatcher))), // Optional ます for polite form
    ]
}

// Pattern: にもまして (even more than, more than ever)
// Structures: Noun + にもまして
pub fn nimomashite() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match まして as 副詞/一般
    #[derive(Debug)]
    struct MashiteMatcher;
    impl super::Matcher for MashiteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "まして"
                        && token.pos.first().is_some_and(|pos| pos == "副詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "一般") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        super::noun(),
        surface("に"),
        surface("も"),
        TokenMatcher::Custom(Arc::new(MashiteMatcher)),
    ]
}

// Pattern: まくる (split tokenization - do repeatedly/excessively)
// Structures: Verb[stem] + まくる
pub fn makuru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match まくる as 動詞 (自立 or 非自立)
    #[derive(Debug)]
    struct MakuruMatcher;
    impl super::Matcher for MakuruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.base_form == "まくる"
                        && token.pos.first().is_some_and(|pos| pos == "動詞")
                        && (token.pos.get(1).is_some_and(|pos| pos == "自立")
                            || token.pos.get(1).is_some_and(|pos| pos == "非自立")) =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        super::flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(MakuruMatcher)),
    ]
}

// Pattern: まくる (compound tokenization - single token verbs ending in まくる)
// Structures: Compound verb (e.g., 歌いまくる, 飲みまくる) as single token
pub fn makuru_compound() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match compound verbs where base_form ends with まくる (but not まくる itself)
    #[derive(Debug)]
    struct MakuruCompoundMatcher;
    impl super::Matcher for MakuruCompoundMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.pos.first().is_some_and(|pos| pos == "動詞")
                        && token.base_form.ends_with("まくる")
                        && token.base_form != "まくる" =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![TokenMatcher::Custom(Arc::new(MakuruCompoundMatcher))]
}

// Pattern: わ〜わ（で） (with A and B, C)
// Structures: (A)わ + (B)わ + (で)
pub fn wa_u301c_wa_uff08_de_uff09() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match わ particle (助詞/終助詞)
    #[derive(Debug)]
    struct WaParticleMatcher;
    impl super::Matcher for WaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "わ"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "終助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match second わ (can be particle OR noun)
    #[derive(Debug)]
    struct WaSecondMatcher;
    impl super::Matcher for WaSecondMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "わ"
                        && ((token.pos.first().is_some_and(|p| p == "助詞")
                            && token.pos.get(1).is_some_and(|p| p == "終助詞"))
                            || token.pos.first().is_some_and(|p| p == "名詞")) =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match で after わ (can be auxiliary verb OR particle)
    #[derive(Debug)]
    struct DeMatcher;
    impl super::Matcher for DeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "で"
                        && ((token.base_form == "だ"
                            && token.pos.first().is_some_and(|p| p == "助動詞"))
                            || (token.pos.first().is_some_and(|p| p == "助詞")
                                && token.pos.get(1).is_some_and(|p| p == "格助詞"))) =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Note: This pattern may not detect cases where punctuation (commas, etc.)
    // appears between the two わ, because Wildcards stop at punctuation by default.
    // This is a limitation of the current TokenMatcher architecture.
    vec![
        TokenMatcher::Custom(Arc::new(WaParticleMatcher)),
        wildcard(0, 30, vec![]),
        TokenMatcher::Custom(Arc::new(WaSecondMatcher)),
        optional(TokenMatcher::Custom(Arc::new(DeMatcher))),
    ]
}

// Pattern: どうにか
// Pattern: どうにか (somehow, one way or another, barely)
// Structure: どうにか + Phrase
pub fn dounika() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match どうにか as 副詞/一般
    #[derive(Debug)]
    struct DounikaMatcher;
    impl super::Matcher for DounikaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "どうにか"
                        && token.pos.first().is_some_and(|pos| pos == "副詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "一般") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![TokenMatcher::Custom(Arc::new(DounikaMatcher))]
}

// Pattern: や否や (as soon as)
// Structures: Verb[る] + や否や
pub fn yainaya() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match やいなや as connective particle
    #[derive(Debug)]
    struct YainayaMatcher;
    impl super::Matcher for YainayaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "やいなや"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        verb_form("基本形"),
        TokenMatcher::Custom(Arc::new(YainayaMatcher)),
    ]
}

// Pattern: 次第です (because, the reason is)
// Structures: Verb + 次第 + です
pub fn shidaidesu() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match 次第 as noun
    #[derive(Debug)]
    struct ShidaiMatcher;
    impl super::Matcher for ShidaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "次第"
                        && token.pos.first().is_some_and(|pos| pos == "名詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match です auxiliary
    #[derive(Debug)]
    struct DesuMatcher;
    impl super::Matcher for DesuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "です"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(ShidaiMatcher)),
        TokenMatcher::Custom(Arc::new(DesuMatcher)),
    ]
}

// Pattern: というところ (I would say about / approximately)
// Structures: Phrase + という/といった + ところ + だ/です
//
// Examples:
// - 二週間で終わるというところだ (I would say it will finish in two weeks)
// - 15分というところです (I would say about 15 minutes)
// - 5000万といったところです (I would say about 50 million)
pub fn toiutokoro() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match という or といった as 助詞/格助詞/連語
    #[derive(Debug)]
    struct ToiuVariantMatcher;
    impl super::Matcher for ToiuVariantMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.surface == "という" || token.surface == "といった")
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                    && token.pos.get(2).is_some_and(|pos| pos == "連語")
            })
        }
    }

    // Match ところ as 名詞/非自立/副詞可能
    #[derive(Debug)]
    struct TokoroMatcher;
    impl super::Matcher for TokoroMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ところ"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                    && token.pos.get(2).is_some_and(|pos| pos == "副詞可能")
            })
        }
    }

    // Match だ or です as auxiliary verb (optional ending)
    #[derive(Debug)]
    struct DaDeSuMatcher;
    impl super::Matcher for DaDeSuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.surface == "だ" || token.surface == "です")
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    vec![
        any(), // Preceding phrase/number/expression
        TokenMatcher::Custom(Arc::new(ToiuVariantMatcher)),
        TokenMatcher::Custom(Arc::new(TokoroMatcher)),
        optional(TokenMatcher::Custom(Arc::new(DaDeSuMatcher))),
    ]
}

// Pattern: １～たりとも～ない (not even one, not a single)
// Structures: Counter/Quantifier + たりとも + Negative phrase
//
// Examples:
// - 一秒たりとも気を抜くことができない (can't lose focus even for a second)
// - 何人たりとも立ち入ることが許されない (no one is allowed to enter)
// - 少したりとも油断をすると (if you let your guard down even a little)
//
// Tokenization:
// - Any word (counter, quantifier, noun)
// - たり: 助動詞 (文語・ナリ, 基本形) OR 助詞/並立助詞
// - と: 助詞/格助詞/引用
// - も: 助詞/係助詞
pub fn ichi_uff5e_taritomo_uff5e_nai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match たり particle/auxiliary
    // Can be 助動詞 (文語・ナリ, 基本形) or 助詞/並立助詞
    #[derive(Debug)]
    struct TariMatcher;
    impl super::Matcher for TariMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "たり"
                    && token.base_form == "たり"
                    && ((token.pos.first().is_some_and(|p| p == "助動詞")
                        && token.features.get(4).is_some_and(|f| f == "文語・ナリ")
                        && token.features.get(5).is_some_and(|f| f == "基本形"))
                        || (token.pos.first().is_some_and(|p| p == "助詞")
                            && token.pos.get(1).is_some_and(|p| p == "並立助詞")))
            })
        }
    }

    // Match と particle (quotation/comparison)
    #[derive(Debug)]
    struct ToQuoteMatcher;
    impl super::Matcher for ToQuoteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "と"
                    && token.pos.first().is_some_and(|p| p == "助詞")
                    && token.pos.get(1).is_some_and(|p| p == "格助詞")
                    && token.pos.get(2).is_some_and(|p| p == "引用")
            })
        }
    }

    // Match も particle (binding particle)
    #[derive(Debug)]
    struct MoBindingMatcher;
    impl super::Matcher for MoBindingMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "も"
                    && token.pos.first().is_some_and(|p| p == "助詞")
                    && token.pos.get(1).is_some_and(|p| p == "係助詞")
            })
        }
    }

    vec![
        any(), // Any word (counter, quantifier, noun, etc.)
        TokenMatcher::Custom(Arc::new(TariMatcher)),
        TokenMatcher::Custom(Arc::new(ToQuoteMatcher)),
        TokenMatcher::Custom(Arc::new(MoBindingMatcher)),
    ]
}

// Pattern: ったらない・といったらない (too X for words / indescribably X)
// Structures: Noun/Adj/Verb + (と)いったら + ない/ありゃしない/ありません
pub fn ttaranai_u30fb_toittaranai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match と as quotation particle
    #[derive(Debug)]
    struct ToParticleMatcher;
    impl super::Matcher for ToParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "と"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            })
        }
    }

    // Match いっ (連用タ接続 of いう)
    #[derive(Debug)]
    struct IttaMatcher;
    impl super::Matcher for IttaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "いっ"
                    && token.base_form == "いう"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
            })
        }
    }

    // Match たら (仮定形 of た auxiliary)
    #[derive(Debug)]
    struct TaraMatcher;
    impl super::Matcher for TaraMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "たら"
                    && token.base_form == "た"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    // Match ない (adjective or auxiliary) - final ending
    #[derive(Debug)]
    struct NaiEndingMatcher;
    impl super::Matcher for NaiEndingMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ない"
                    && (token.pos.first().is_some_and(|pos| pos == "形容詞")
                        || token.pos.first().is_some_and(|pos| pos == "助動詞"))
            })
        }
    }

    // Pattern: Noun/Adj/Verb + (optional と) + いっ + たら + ない
    // This matches the most common forms: といったらない and ったらない
    // The longer forms (ありゃしない, ありません) will be matched by separate patterns if needed
    vec![
        any(),
        optional(TokenMatcher::Custom(Arc::new(ToParticleMatcher))),
        TokenMatcher::Custom(Arc::new(IttaMatcher)),
        TokenMatcher::Custom(Arc::new(TaraMatcher)),
        TokenMatcher::Custom(Arc::new(NaiEndingMatcher)),
    ]
}

// Pattern: に照らして・に照らすと (in light of / in accordance with)
// Structures: Noun + に + 照らして/照らした/照らすと
//
// Examples:
// - 経験に照らして (in light of experience)
// - 法律に照らして (in accordance with the law)
// - 経済統計に照らした犯罪統計 (crime statistics in light of economic statistics)
// - 法律に照らすと (in light of the law, if we apply...)
pub fn niterashite_u30fb_niterasuto() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match に as 助詞/格助詞/一般
    #[derive(Debug)]
    struct NiMatcher;
    impl super::Matcher for NiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "に"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            })
        }
    }

    // Match 照らす as 動詞/自立 (any conjugation form)
    #[derive(Debug)]
    struct TerasuVerbMatcher;
    impl super::Matcher for TerasuVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.base_form == "照らす"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "自立")
            })
        }
    }

    // Match any of the three endings: て, た, or と
    #[derive(Debug)]
    struct TerasuEndingMatcher;
    impl super::Matcher for TerasuEndingMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // て (助詞/接続助詞)
                if token.surface == "て"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
                {
                    return true;
                }
                // た (助動詞)
                if token.surface == "た"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.base_form == "た"
                {
                    return true;
                }
                // と (助詞/接続助詞)
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
        super::noun(), // Preceding noun (経験, 法律, etc.)
        TokenMatcher::Custom(Arc::new(NiMatcher)),
        TokenMatcher::Custom(Arc::new(TerasuVerbMatcher)),
        TokenMatcher::Custom(Arc::new(TerasuEndingMatcher)),
    ]
}

// Pattern: とあれば (if/when it comes to)
// Structures: Noun + とあれば, Verb/Adj + とあれば (quotation)
pub fn toareba() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matches と as 助詞/格助詞 (can be 一般 or 引用)
    #[derive(Debug)]
    struct ToMatcher;
    impl super::Matcher for ToMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "と"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            })
        }
    }

    // Matches あれ (仮定形 of ある)
    #[derive(Debug)]
    struct AreMatcher;
    impl super::Matcher for AreMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "あれ"
                    && token.base_form == "ある"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token.features.get(5).is_some_and(|f| f == "仮定形")
            })
        }
    }

    // Matches ば as 助詞/接続助詞
    #[derive(Debug)]
    struct BaMatcher;
    impl super::Matcher for BaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ば"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
            })
        }
    }

    vec![
        any(), // Preceding word (noun/verb/adjective)
        TokenMatcher::Custom(Arc::new(ToMatcher)),
        TokenMatcher::Custom(Arc::new(AreMatcher)),
        TokenMatcher::Custom(Arc::new(BaMatcher)),
    ]
}

// Pattern: さぞ (you must be very, I dare say that)
// Structures: さぞ・さぞや・さぞかし + Phrase + だろう/でしょう
pub fn sazo() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    #[derive(Debug)]
    struct SazoMatcher;
    impl super::Matcher for SazoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.pos.first().is_some_and(|pos| pos == "副詞")
                        && (token.base_form == "さぞ"
                            || token.base_form == "さぞや"
                            || token.base_form == "さぞかし") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }
    vec![TokenMatcher::Custom(Arc::new(SazoMatcher))]
}

// Pattern: ときたら (when it comes to / that darn)
// Structures: Noun + ときたら
// Note: Written in kana (not と来たら). Kagome tokenizes as とく + たら
pub fn tokitara() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matches とき (tokenized as とく verb in 連用形)
    #[derive(Debug)]
    struct TokiMatcher;
    impl super::Matcher for TokiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "とき"
                        && token.base_form == "とく"
                        && token.pos.first().is_some_and(|pos| pos == "動詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                        && token.features.get(5).is_some_and(|f| f == "連用形") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matches たら (仮定形 of た auxiliary)
    #[derive(Debug)]
    struct TaraMatcher;
    impl super::Matcher for TaraMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "たら"
                        && token.base_form == "た"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.features.get(5).is_some_and(|f| f == "仮定形") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        super::noun(), // Preceding noun
        TokenMatcher::Custom(Arc::new(TokiMatcher)),
        TokenMatcher::Custom(Arc::new(TaraMatcher)),
    ]
}

// Pattern: びる (seeming/looking like)
// Structures: Noun/Adjective stem + びる (e.g., 大人びる, 古びる, 田舎びる, ひなびる)
//
// Tokenization: Single compound verb token (動詞/自立, base ends with びる)
// Examples: 大人びて (base=大人びる), 古びた (base=古びる), 田舎びている (base=田舎びる)
//
// Note: Unlike めく which can be split (Noun + めく), びる appears only as compound verbs
// where Kagome recognizes the full word as a single verb token with base ending in びる.
// We must exclude standalone verbs that happen to end in びる (like 延びる, 飛びる, etc.)
pub fn biru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // びる is a suffix used with a limited set of expressions.
    const ALLOWED_BIRU_VERBS: &[&str] = &[
        "大人びる",
        "古びる",
        "ひなびる",
        "田舎びる",
        "物寂びる",
        "幼びる",
        "おさなびる",
        "神さびる",
        "ひねこびる",
    ];

    // Matcher for the limited set of lexicalized びる verbs.
    #[derive(Debug)]
    struct BiruVerbMatcher;
    impl super::Matcher for BiruVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.pos.first().is_some_and(|pos| pos == "動詞")
                        && ALLOWED_BIRU_VERBS.contains(&token.base_form.as_str())
                        && token.pos.get(1).is_some_and(|pos| pos == "自立") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![TokenMatcher::Custom(Arc::new(BiruVerbMatcher))]
}

// Pattern: にしたところで (even if / even though)
// Structures: Noun + にしたところで / としたところで
//            Noun + にしたって / としたって (casual)
pub fn nishitatokorode() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match に or と (case-marking particle)
    #[derive(Debug)]
    struct NiToMatcher;
    impl super::Matcher for NiToMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if (token.surface == "に" || token.surface == "と")
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match し (from する, 連用形)
    #[derive(Debug)]
    struct ShiMatcher;
    impl super::Matcher for ShiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "し"
                        && token.base_form == "する"
                        && token.pos.first().is_some_and(|p| p == "動詞")
                        && token.features.get(5).is_some_and(|f| f == "連用形") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match ところで or たって (particle)
    #[derive(Debug)]
    struct TokorodeTatteMatcher;
    impl super::Matcher for TokorodeTatteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if (token.surface == "って"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "格助詞"))
                        || (token.surface == "ところ"
                            && token.pos.first().is_some_and(|p| p == "名詞")
                            && token.pos.get(1).is_some_and(|p| p == "非自立")) =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match で (particle, for ところで)
    #[derive(Debug)]
    struct DeMatcher;
    impl super::Matcher for DeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "で"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Preceding noun/verb/adjective
        TokenMatcher::Custom(Arc::new(NiToMatcher)),
        TokenMatcher::Custom(Arc::new(ShiMatcher)),
        past_auxiliary(),
        TokenMatcher::Custom(Arc::new(TokorodeTatteMatcher)),
        optional(TokenMatcher::Custom(Arc::new(DeMatcher))), // で is only needed for ところで
    ]
}

// Pattern: ～ばこそ (precisely because)
// Structures: Verb[ば] + こそ
//            い-Adjective[ば] + こそ
//            Noun/な-Adjective + であれば + こそ
pub fn uff5e_bakoso() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matches ば (助詞/接続助詞)
    #[derive(Debug)]
    struct BaMatcher;
    impl super::Matcher for BaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ば"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matches こそ (助詞/係助詞)
    #[derive(Debug)]
    struct KosoMatcher;
    impl super::Matcher for KosoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "こそ"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "係助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Matches noun or な-adjective (for であればこそ pattern only)
    // This should NOT match particles, verbs, or i-adjectives
    #[derive(Debug)]
    struct NounOrNaAdjMatcher;
    impl super::Matcher for NounOrNaAdjMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "名詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matches Verb or い-Adjective in 仮定形 (hypothetical form)
    // OR で (助動詞) for the であれば pattern
    #[derive(Debug)]
    struct BakosoFirstTokenMatcher;
    impl super::Matcher for BakosoFirstTokenMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) => {
                    // Match Verb or い-Adj in 仮定形
                    let is_verb = token.pos.first().is_some_and(|pos| pos == "動詞");
                    let is_i_adj = token.pos.first().is_some_and(|pos| pos == "形容詞");
                    let is_katei = token.features.get(5).is_some_and(|f| f == "仮定形");

                    if (is_verb || is_i_adj) && is_katei {
                        return (true, 1);
                    }

                    // Match で (助動詞, base=だ) for であれば pattern
                    if token.surface == "で"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.base_form == "だ"
                    {
                        return (true, 1);
                    }

                    (false, 0)
                }
                _ => (false, 0),
            }
        }
    }

    // Matches あれ (助動詞, base=ある, 仮定形) - optional for であれば pattern
    #[derive(Debug)]
    struct AreMatcher;
    impl super::Matcher for AreMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "あれ"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.base_form == "ある"
                        && token.features.get(5).is_some_and(|f| f == "仮定形") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Pattern matcher:
    // Optional: Noun/な-Adj (only for であればこそ pattern, not for verb/i-adj)
    // Main token: Verb/i-Adj in 仮定形 OR で
    // Optional: あれ (only present in であればこそ pattern)
    // Then: ば + こそ
    vec![
        optional(TokenMatcher::Custom(Arc::new(NounOrNaAdjMatcher))),
        TokenMatcher::Custom(Arc::new(BakosoFirstTokenMatcher)),
        optional(TokenMatcher::Custom(Arc::new(AreMatcher))),
        TokenMatcher::Custom(Arc::new(BaMatcher)),
        TokenMatcher::Custom(Arc::new(KosoMatcher)),
    ]
}

// Pattern: ても差し支えない (it's not a hindrance if, may I)
// Structures: Verb[ても]/Adj[ても] + 差し支え + ありません
//            Noun[でも]/な-Adj[でも] + 差し支え + ありません
pub fn temosashitsukaenai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ても or でも (can be two tokens: て/で + も, or single token でも)
    #[derive(Debug)]
    struct TemoDemoMatcher;
    impl super::Matcher for TemoDemoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) => {
                    // Single token でも (助詞/副助詞)
                    if token.surface == "でも"
                        && token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "副助詞")
                    {
                        return (true, 1);
                    }
                    // Or て/で particle (when followed by も)
                    if (token.surface == "て" || token.surface == "で")
                        && token.pos.first().is_some_and(|p| p == "助詞")
                    {
                        return (true, 1);
                    }
                    (false, 0)
                }
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct MoParticleMatcher;
    impl super::Matcher for MoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "も" && token.pos.first().is_some_and(|p| p == "助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match さしつかえ (verb さしつかえる in 連用形)
    #[derive(Debug)]
    struct SashitsukaeMatcher;
    impl super::Matcher for SashitsukaeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.base_form == "さしつかえる"
                        && token.pos.first().is_some_and(|p| p == "動詞")
                        && token.features.get(5).is_some_and(|f| f == "連用形") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match あり (ある in 連用形)
    #[derive(Debug)]
    struct AriMatcher;
    impl super::Matcher for AriMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.base_form == "ある"
                        && token.pos.first().is_some_and(|p| p == "動詞")
                        && token.features.get(5).is_some_and(|f| f == "連用形") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match ません or ん (negative)
    #[derive(Debug)]
    struct MasenNMatcher;
    impl super::Matcher for MasenNMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if (token.base_form == "ます" || token.base_form == "ん")
                        && token.pos.first().is_some_and(|p| p == "助動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(TemoDemoMatcher)),
        optional(TokenMatcher::Custom(Arc::new(MoParticleMatcher))),
        TokenMatcher::Custom(Arc::new(SashitsukaeMatcher)),
        TokenMatcher::Custom(Arc::new(AriMatcher)),
        TokenMatcher::Custom(Arc::new(MasenNMatcher)),
    ]
}

// Pattern: には及ばない①
pub fn nihaoyobanai_u2460() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match に as 助詞/格助詞/一般
    #[derive(Debug)]
    struct NiMatcher;
    impl super::Matcher for NiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "に"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                    && token.pos.get(2).is_some_and(|pos| pos == "一般")
            })
        }
    }

    // Match は as 助詞/係助詞
    #[derive(Debug)]
    struct WaMatcher;
    impl super::Matcher for WaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "は"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
            })
        }
    }

    // Match 及ぶ verb (及び in 連用形 or 及ば in 未然形)
    #[derive(Debug)]
    struct OyobuMatcher;
    impl super::Matcher for OyobuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.base_form == "及ぶ"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "自立")
                    && (token.surface == "及び" || token.surface == "及ば")
            })
        }
    }

    // Match ない (助動詞) or ませ (助動詞, 未然形 of ます)
    #[derive(Debug)]
    struct NaiMaseMatcher;
    impl super::Matcher for NaiMaseMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match ない (casual negative)
                if token.surface == "ない"
                    && token.base_form == "ない"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.features.get(4).is_some_and(|f| f == "特殊・ナイ")
                {
                    return true;
                }

                // Match ませ (polite negative prep)
                if token.surface == "ませ"
                    && token.base_form == "ます"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token
                        .features
                        .get(4)
                        .is_some_and(|f| f.starts_with("特殊・マス"))
                {
                    return true;
                }

                false
            })
        }
    }

    // Optional ん for polite negative (ません)
    #[derive(Debug)]
    struct NMatcher;
    impl super::Matcher for NMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ん"
                    && token.base_form == "ん"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.features.get(4).is_some_and(|f| f == "不変化型")
            })
        }
    }

    vec![
        any(), // Noun or Verb
        TokenMatcher::Custom(Arc::new(NiMatcher)),
        TokenMatcher::Custom(Arc::new(WaMatcher)),
        TokenMatcher::Custom(Arc::new(OyobuMatcher)),
        TokenMatcher::Custom(Arc::new(NaiMaseMatcher)),
        optional(TokenMatcher::Custom(Arc::new(NMatcher))), // Optional ん for polite
    ]
}

// Pattern: に即して (in accordance with / based on)
// Structures: Noun + に即（そく）して, Noun + に即（そく）した + Noun
// Alternative: に則（そく）して/に則（そく）した (same pronunciation, similar meaning)
pub fn nisokushite() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match に as 助詞/格助詞/一般
    #[derive(Debug)]
    struct NiMatcher;
    impl super::Matcher for NiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "に"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            })
        }
    }

    // Match 即し or 則し (both base forms: 即す or 則す)
    // Both verbs: 動詞/自立, 五段・サ行, 連用形
    #[derive(Debug)]
    struct SokushiMatcher;
    impl super::Matcher for SokushiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.surface == "即し" || token.surface == "則し")
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "自立")
                    && (token.base_form == "即す" || token.base_form == "則す")
            })
        }
    }

    // Match て (助詞/接続助詞) or た (助動詞)
    // て-form for に即して, た-form for に即した
    #[derive(Debug)]
    struct TeOrTaMatcher;
    impl super::Matcher for TeOrTaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.surface == "て"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "接続助詞"))
                    || (token.surface == "た"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.base_form == "た")
            })
        }
    }

    vec![
        super::noun(),
        TokenMatcher::Custom(Arc::new(NiMatcher)),
        TokenMatcher::Custom(Arc::new(SokushiMatcher)),
        TokenMatcher::Custom(Arc::new(TeOrTaMatcher)),
    ]
}

// Pattern: ないまでも (even if not / may not be... but)
// Structures:
// 1. Verb[ない] + までも (e.g., 言えないまでも)
// 2. Noun + ではない + までも (e.g., 毎週ではないまでも)
// 3. Noun + じゃない + までも (e.g., 幸せじゃないまでも)
pub fn naimademo() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match まで particle
    #[derive(Debug)]
    struct MadeMatcher;
    impl super::Matcher for MadeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "まで"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "副助詞")
            })
        }
    }

    // Match も particle
    #[derive(Debug)]
    struct MoMatcher;
    impl super::Matcher for MoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "も"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
            })
        }
    }

    // Custom matcher that handles all three variants
    #[derive(Debug)]
    struct NaimademoCompound;
    impl super::Matcher for NaimademoCompound {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ない"
                    && (token.pos.first().is_some_and(|pos| pos == "助動詞")
                        || token.pos.first().is_some_and(|pos| pos == "形容詞"))
            })
        }
    }

    // Match verb OR noun OR adjective (broader match for start)
    #[derive(Debug)]
    struct VerbNounOrAdjMatcher;
    impl super::Matcher for VerbNounOrAdjMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token
                    .pos
                    .first()
                    .is_some_and(|pos| pos == "動詞" || pos == "名詞" || pos == "形容詞")
            })
        }
    }

    // Single pattern that uses wildcard to match the varying middle part
    vec![
        TokenMatcher::Custom(Arc::new(VerbNounOrAdjMatcher)), // Verb, Noun, or Adjective
        wildcard(0, 2, vec![]), // Optional particles before ない (で+は or じゃ)
        TokenMatcher::Custom(Arc::new(NaimademoCompound)), // ない
        TokenMatcher::Custom(Arc::new(MadeMatcher)), // まで
        TokenMatcher::Custom(Arc::new(MoMatcher)), // も
    ]
}

// Pattern: をよそに (ignoring/disregarding)
// Structures: Noun + をよそに, Phrase + の + をよそに
pub fn woyosoni() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match を particle (格助詞/一般)
    #[derive(Debug)]
    struct WoMatcher;
    impl super::Matcher for WoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "を"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            })
        }
    }

    // Match よそ noun (名詞/代名詞/一般)
    #[derive(Debug)]
    struct YosoMatcher;
    impl super::Matcher for YosoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "よそ" && token.pos.first().is_some_and(|pos| pos == "名詞")
            })
        }
    }

    // Match に particle (格助詞/一般)
    #[derive(Debug)]
    struct NiMatcher;
    impl super::Matcher for NiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "に"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            })
        }
    }

    vec![
        any(), // Noun, の, or other word before を
        TokenMatcher::Custom(Arc::new(WoMatcher)),
        TokenMatcher::Custom(Arc::new(YosoMatcher)),
        TokenMatcher::Custom(Arc::new(NiMatcher)),
    ]
}

// Pattern: に限ったことではない (not limited to / not only)
// Structures: Noun + に + かぎっ + た + こと + で + は + ない/ありません
pub fn nikagittakotodehanai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match に (助詞/格助詞/一般)
    #[derive(Debug)]
    struct NiMatcher;
    impl super::Matcher for NiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "に"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            })
        }
    }

    // Match かぎっ (動詞/自立, base=かぎる, 連用タ接続)
    #[derive(Debug)]
    struct KagitMatcher;
    impl super::Matcher for KagitMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "かぎっ"
                    && token.base_form == "かぎる"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
            })
        }
    }

    // Match た (助動詞, 特殊・タ)
    #[derive(Debug)]
    struct TaMatcher;
    impl super::Matcher for TaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "た"
                    && token.base_form == "た"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    // Match こと (名詞/非自立/一般)
    #[derive(Debug)]
    struct KotoMatcher;
    impl super::Matcher for KotoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "こと" && token.pos.first().is_some_and(|pos| pos == "名詞")
            })
        }
    }

    // Match で (助動詞, 特殊・ダ, 連用形, base=だ)
    #[derive(Debug)]
    struct DeMatcher;
    impl super::Matcher for DeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "で"
                    && token.base_form == "だ"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    // Match は (助詞/係助詞)
    #[derive(Debug)]
    struct WaMatcher;
    impl super::Matcher for WaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "は"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
            })
        }
    }

    // Match ない (助動詞 or 形容詞)
    // OR match あり+ませ+ん sequence for polite form
    #[derive(Debug)]
    struct NaiOrArimasenMatcher;
    impl super::Matcher for NaiOrArimasenMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match ない (助動詞, 特殊・ナイ) or ない (形容詞)
                if token.surface == "ない" && token.base_form == "ない" {
                    return token
                        .pos
                        .first()
                        .is_some_and(|pos| pos == "助動詞" || pos == "形容詞");
                }
                // Match あり for polite form (動詞, base=ある)
                if token.surface == "あり" && token.base_form == "ある" {
                    return token.pos.first().is_some_and(|pos| pos == "動詞");
                }
                false
            })
        }
    }

    // Match ませ (助動詞, 特殊・マス, 未然形) - optional for polite
    #[derive(Debug)]
    struct MaseMatcher;
    impl super::Matcher for MaseMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ませ"
                    && token.base_form == "ます"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    // Match ん (助動詞, 不変化型) - optional for polite
    #[derive(Debug)]
    struct NMatcher;
    impl super::Matcher for NMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ん"
                    && token.base_form == "ん"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    vec![
        super::noun(),
        TokenMatcher::Custom(Arc::new(NiMatcher)),
        TokenMatcher::Custom(Arc::new(KagitMatcher)),
        TokenMatcher::Custom(Arc::new(TaMatcher)),
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
        TokenMatcher::Custom(Arc::new(DeMatcher)),
        TokenMatcher::Custom(Arc::new(WaMatcher)),
        TokenMatcher::Custom(Arc::new(NaiOrArimasenMatcher)),
        optional(TokenMatcher::Custom(Arc::new(MaseMatcher))),
        optional(TokenMatcher::Custom(Arc::new(NMatcher))),
    ]
}

// Pattern: とは比べものにならない (cannot be compared to)
// Structures: Noun + とは比べものにならない/になりません
pub fn tohakurabemononinaranai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // と matcher (助詞/格助詞/一般)
    #[derive(Debug)]
    struct ToMatcher;
    impl super::Matcher for ToMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "と"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            })
        }
    }

    // は matcher (助詞/係助詞)
    #[derive(Debug)]
    struct WaMatcher;
    impl super::Matcher for WaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "は"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
            })
        }
    }

    // くらべ matcher (動詞/自立, base=くらべる, 連用形)
    #[derive(Debug)]
    struct KurabeMatcher;
    impl super::Matcher for KurabeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "くらべ"
                    && token.base_form == "くらべる"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
            })
        }
    }

    // もの matcher (名詞/接尾/一般)
    #[derive(Debug)]
    struct MonoMatcher;
    impl super::Matcher for MonoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "もの" && token.pos.first().is_some_and(|pos| pos == "名詞")
            })
        }
    }

    // に matcher (助詞/格助詞/一般)
    #[derive(Debug)]
    struct NiMatcher;
    impl super::Matcher for NiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "に"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            })
        }
    }

    // なる matcher - matches both なら (未然形) and なり (連用形)
    #[derive(Debug)]
    struct NaruMatcher;
    impl super::Matcher for NaruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.surface == "なら" || token.surface == "なり")
                    && token.base_form == "なる"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
            })
        }
    }

    // ない or ませ matcher (for casual and polite forms)
    #[derive(Debug)]
    struct NaiOrMaseMatcher;
    impl super::Matcher for NaiOrMaseMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                if token.surface == "ない" && token.pos.first().is_some_and(|pos| pos == "助動詞")
                {
                    return true;
                }
                if token.surface == "ませ"
                    && token.base_form == "ます"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                {
                    return true;
                }
                false
            })
        }
    }

    // ん matcher (助動詞, 不変化型) - for polite negative ending
    #[derive(Debug)]
    struct NMatcher;
    impl super::Matcher for NMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ん" && token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    vec![
        noun(),
        TokenMatcher::Custom(Arc::new(ToMatcher)),
        TokenMatcher::Custom(Arc::new(WaMatcher)),
        TokenMatcher::Custom(Arc::new(KurabeMatcher)),
        TokenMatcher::Custom(Arc::new(MonoMatcher)),
        TokenMatcher::Custom(Arc::new(NiMatcher)),
        optional(TokenMatcher::Custom(Arc::new(WaMatcher))), // Optional は before なる
        TokenMatcher::Custom(Arc::new(NaruMatcher)),
        TokenMatcher::Custom(Arc::new(NaiOrMaseMatcher)),
        optional(TokenMatcher::Custom(Arc::new(NMatcher))), // Optional ん for polite
    ]
}

// Pattern: まじき (must not / unbecoming of)
// Structures:
//   1. Noun + にある/としてある + まじき + Noun
//   2. Verb (許す) + まじき + Noun
pub fn majiki() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match に (助詞/格助詞/一般)
    #[derive(Debug)]
    struct NiMatcher;
    impl super::Matcher for NiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "に"
                    && token.pos.first().is_some_and(|p| p == "助詞")
                    && token.pos.get(1).is_some_and(|p| p == "格助詞")
            })
        }
    }

    // Match として (助詞/格助詞/連語)
    #[derive(Debug)]
    struct ToshiteMatcher;
    impl super::Matcher for ToshiteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "として"
                    && token.pos.first().is_some_and(|p| p == "助詞")
                    && token.pos.get(1).is_some_and(|p| p == "格助詞")
                    && token.pos.get(2).is_some_and(|p| p == "連語")
            })
        }
    }

    // Match ある (動詞, base=ある)
    #[derive(Debug)]
    struct AruMatcher;
    impl super::Matcher for AruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ある"
                    && token.base_form == "ある"
                    && token.pos.first().is_some_and(|p| p == "動詞")
            })
        }
    }

    // Match まじき (助動詞, base=まじ, 体言接続)
    #[derive(Debug)]
    struct MajikiMatcher;
    impl super::Matcher for MajikiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "まじき"
                    && token.base_form == "まじ"
                    && token.pos.first().is_some_and(|p| p == "助動詞")
                    && token.features.get(5).is_some_and(|f| f == "体言接続")
            })
        }
    }

    // Match either:
    // 1. Noun + にある/としてある + まじき + Noun
    // 2. ゆるす + まじき + Noun
    // We need to capture both patterns, so we use a compound matcher
    #[derive(Debug)]
    struct MajikiCompoundMatcher;
    impl super::Matcher for MajikiCompoundMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token
                    .pos
                    .first()
                    .is_some_and(|p| p == "名詞" || p == "動詞")
            })
        }
    }

    // Pattern sequence:
    // Word (Noun or ゆるす)
    // + Optional(に/として + ある)
    // + まじき
    // + Noun
    vec![
        TokenMatcher::Custom(Arc::new(MajikiCompoundMatcher)),
        optional(or(vec![
            TokenMatcher::Custom(Arc::new(NiMatcher)),
            TokenMatcher::Custom(Arc::new(ToshiteMatcher)),
        ])),
        optional(TokenMatcher::Custom(Arc::new(AruMatcher))),
        TokenMatcher::Custom(Arc::new(MajikiMatcher)),
        noun(),
    ]
}

// Pattern: の至り (the utmost / extreme of)
// Structures: Noun + の + 至り + だ/です
pub fn noitari() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match の (助詞/連体化)
    #[derive(Debug)]
    struct NoMatcher;
    impl super::Matcher for NoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "の"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "連体化")
            })
        }
    }

    // Match 至り (名詞/一般, base=至り)
    #[derive(Debug)]
    struct ItariMatcher;
    impl super::Matcher for ItariMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "至り"
                    && token.base_form == "至り"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
            })
        }
    }

    vec![
        noun(),
        TokenMatcher::Custom(Arc::new(NoMatcher)),
        TokenMatcher::Custom(Arc::new(ItariMatcher)),
    ]
}

// Pattern: に恥じない (lives up to / not ashamed of)
// Structures: Noun + に + 恥じない/恥じません
pub fn nihajinai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matches に as 助詞/格助詞/一般
    #[derive(Debug)]
    struct NiMatcher;
    impl super::Matcher for NiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "に"
                    && token.pos.first().is_some_and(|p| p == "助詞")
                    && token.pos.get(1).is_some_and(|p| p == "格助詞")
                    && token.pos.get(2).is_some_and(|p| p == "一般")
            })
        }
    }

    // Matches はじ (base=はじる, 動詞/自立, either 未然形 or 連用形)
    #[derive(Debug)]
    struct HajiMatcher;
    impl super::Matcher for HajiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "はじ"
                    && token.base_form == "はじる"
                    && token.pos.first().is_some_and(|p| p == "動詞")
                    && token.pos.get(1).is_some_and(|p| p == "自立")
                    && (token.features.get(5).is_some_and(|f| f == "未然形")
                        || token.features.get(5).is_some_and(|f| f == "連用形"))
            })
        }
    }

    // Matches ない (助動詞, 特殊・ナイ) OR ませ (助動詞, 特殊・マス)
    #[derive(Debug)]
    struct NaiOrMaseMatcher;
    impl super::Matcher for NaiOrMaseMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.pos.first().is_some_and(|p| p == "助動詞")
                    && ((token.surface == "ない" && token.base_form == "ない")
                        || (token.surface == "ませ" && token.base_form == "ます"))
            })
        }
    }

    // Matches ん (助動詞, 不変化型) for polite negative
    #[derive(Debug)]
    struct NMatcher;
    impl super::Matcher for NMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ん"
                    && token.base_form == "ん"
                    && token.pos.first().is_some_and(|p| p == "助動詞")
                    && token.features.get(4).is_some_and(|f| f == "不変化型")
            })
        }
    }

    vec![
        noun(),
        TokenMatcher::Custom(Arc::new(NiMatcher)),
        TokenMatcher::Custom(Arc::new(HajiMatcher)),
        TokenMatcher::Custom(Arc::new(NaiOrMaseMatcher)),
        optional(TokenMatcher::Custom(Arc::new(NMatcher))),
    ]
}

// Pattern: ずじまい (end up not doing)
// Structures: Verb［ない］+ ず + じまい
pub fn zujimai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct ZuAuxiliaryMatcher;
    impl super::Matcher for ZuAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ず"
                    && token.base_form == "ぬ"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    #[derive(Debug)]
    struct JimaiMatcher;
    impl super::Matcher for JimaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "じまい"
                    && token.base_form == "じまい"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "接尾")
            })
        }
    }

    // Match verb in 未然形 or 未然ヌ接続 (for する verbs)
    #[derive(Debug)]
    struct MizenVerbMatcher;
    impl super::Matcher for MizenVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token
                        .features
                        .get(5)
                        .is_some_and(|form| form == "未然形" || form == "未然ヌ接続")
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(MizenVerbMatcher)),
        TokenMatcher::Custom(Arc::new(ZuAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(JimaiMatcher)),
    ]
}

// Pattern: に言わせれば・に言わせると・に言わせたら (if you ask / according to)
// Structures: Noun + に/から + 言わせれば/言わせると/言わせたら
pub fn niiwasereba_u30fb_niiwaseruto_u30fb_niiwasetara() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match に or から (助詞/格助詞/一般)
    #[derive(Debug)]
    struct NiOrKaraMatcher;
    impl super::Matcher for NiOrKaraMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.surface == "に" || token.surface == "から")
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                    && token.pos.get(2).is_some_and(|pos| pos == "一般")
            })
        }
    }

    // Match いわ (動詞/自立, base=いう, 未然形)
    #[derive(Debug)]
    struct IwaMatcher;
    impl super::Matcher for IwaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "いわ"
                    && token.base_form == "いう"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "自立")
                    && token.features.get(5).is_some_and(|f| f == "未然形")
            })
        }
    }

    // Match せれ (仮定形 for ば), せる (基本形 for と), or せ (連用形 for たら)
    // All are 動詞/接尾, base=せる
    #[derive(Debug)]
    struct SeMatcher;
    impl super::Matcher for SeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.surface == "せれ" || token.surface == "せる" || token.surface == "せ")
                    && token.base_form == "せる"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "接尾")
            })
        }
    }

    // Match ば (助詞/接続助詞), と (助詞/接続助詞), or たら (助動詞, base=た, 仮定形)
    #[derive(Debug)]
    struct BaOrToOrTaraMatcher;
    impl super::Matcher for BaOrToOrTaraMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match ば
                if token.surface == "ば"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
                {
                    return true;
                }
                // Match と
                if token.surface == "と"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
                {
                    return true;
                }
                // Match たら
                if token.surface == "たら"
                    && token.base_form == "た"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.features.get(5).is_some_and(|f| f == "仮定形")
                {
                    return true;
                }
                false
            })
        }
    }

    vec![
        super::noun(),
        TokenMatcher::Custom(Arc::new(NiOrKaraMatcher)),
        TokenMatcher::Custom(Arc::new(IwaMatcher)),
        TokenMatcher::Custom(Arc::new(SeMatcher)),
        TokenMatcher::Custom(Arc::new(BaOrToOrTaraMatcher)),
    ]
}

// Pattern: ったら・といったら (emphasizing extreme degree)
// Structures: Noun/Adj + (と)いったら + ありゃしない/ありはしない
pub fn ttara_u30fb_toittara() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match と as quotation particle (optional)
    #[derive(Debug)]
    struct ToParticleMatcher;
    impl super::Matcher for ToParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "と"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            })
        }
    }

    // Match いっ or っ (連用タ接続 of いう or く verb)
    #[derive(Debug)]
    struct IttaTsuMatcher;
    impl super::Matcher for IttaTsuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.surface == "いっ"
                    && token.base_form == "いう"
                    && token.pos.first().is_some_and(|pos| pos == "動詞"))
                    || (token.surface == "っ"
                        && token.base_form == "く"
                        && token.pos.first().is_some_and(|pos| pos == "動詞"))
            })
        }
    }

    // Match たら (仮定形 of た auxiliary)
    #[derive(Debug)]
    struct TaraMatcher;
    impl super::Matcher for TaraMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "たら"
                    && token.base_form == "た"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    // Match ありゃ (仮定縮約１ of ある) or あり (連用形 of ある)
    #[derive(Debug)]
    struct AriMatcher;
    impl super::Matcher for AriMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.base_form == "ある"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                    && (token.surface == "ありゃ" || token.surface == "あり")
            })
        }
    }

    // Match は (係助詞) - optional, only appears in ありはしない
    #[derive(Debug)]
    struct WaParticleMatcher;
    impl super::Matcher for WaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "は"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
            })
        }
    }

    // Match し (未然形 of する)
    #[derive(Debug)]
    struct ShiMatcher;
    impl super::Matcher for ShiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "し"
                    && token.base_form == "する"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
            })
        }
    }

    // Match ない (助動詞)
    #[derive(Debug)]
    struct NaiMatcher;
    impl super::Matcher for NaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ない"
                    && token.base_form == "ない"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    // Pattern: [Word] + (と) + いっ/っ + たら + ありゃ/あり + (は) + し + ない
    super::concat(vec![
        vec![any()], // Preceding word (noun, adjective, etc.)
        vec![optional(TokenMatcher::Custom(Arc::new(ToParticleMatcher)))],
        vec![TokenMatcher::Custom(Arc::new(IttaTsuMatcher))], // いっ or っ
        vec![TokenMatcher::Custom(Arc::new(TaraMatcher))],    // たら
        vec![TokenMatcher::Custom(Arc::new(AriMatcher))],     // ありゃ or あり
        vec![optional(TokenMatcher::Custom(Arc::new(WaParticleMatcher)))], // は (optional)
        vec![TokenMatcher::Custom(Arc::new(ShiMatcher))],     // し
        vec![TokenMatcher::Custom(Arc::new(NaiMatcher))],     // ない
    ])
}

// Pattern: こととて (due to / because of)
// Structures: Verb + こととて, Verb[ぬ] + こととて, Noun + の + こととて
pub fn kototote() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match こと (名詞/非自立/一般)
    #[derive(Debug)]
    struct KotoMatcher;
    impl super::Matcher for KotoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "こと"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "非自立")
            })
        }
    }

    // Match と (助詞/格助詞/引用)
    #[derive(Debug)]
    struct ToParticleMatcher;
    impl super::Matcher for ToParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "と"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            })
        }
    }

    // Match て (助詞/接続助詞)
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl super::Matcher for TeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "て"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
            })
        }
    }

    vec![
        wildcard(1, 2, vec![]),
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
        TokenMatcher::Custom(Arc::new(ToParticleMatcher)),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
    ]
}

// Pattern: ずくめ (nothing but / all in)
// Structures: Noun + ずくめ, Noun + ずくめ + の
pub fn zukume() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct ZukumeMatcher;
    impl super::Matcher for ZukumeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ずくめ"
                    && token.base_form == "ずくめ"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "接尾")
            })
        }
    }

    vec![super::noun(), TokenMatcher::Custom(Arc::new(ZukumeMatcher))]
}

// Pattern: には及ばない②
// Pattern: には及ばない② (not as good as / no match for)
// Structures: Noun + には及ばない, Noun + の足元にも及ばない
pub fn nihaoyobanai_u2461() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match に as 助詞/格助詞/一般
    #[derive(Debug)]
    struct NiMatcher;
    impl super::Matcher for NiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "に"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                    && token.pos.get(2).is_some_and(|pos| pos == "一般")
            })
        }
    }

    // Match およぶ verb (およば in 未然形 or および in 連用形)
    #[derive(Debug)]
    struct OyobuMatcher;
    impl super::Matcher for OyobuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.base_form == "およぶ"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "自立")
                    && (token.surface == "およば" || token.surface == "および")
            })
        }
    }

    // Match ない (助動詞) or ませ (助動詞, 未然形 of ます)
    #[derive(Debug)]
    struct NaiMaseMatcher;
    impl super::Matcher for NaiMaseMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match ない (casual negative)
                if token.surface == "ない"
                    && token.base_form == "ない"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.features.get(4).is_some_and(|f| f == "特殊・ナイ")
                {
                    return true;
                }

                // Match ませ (polite negative prep)
                if token.surface == "ませ"
                    && token.base_form == "ます"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token
                        .features
                        .get(4)
                        .is_some_and(|f| f.starts_with("特殊・マス"))
                {
                    return true;
                }

                false
            })
        }
    }

    // Optional ん for polite negative (ません)
    #[derive(Debug)]
    struct NMatcher;
    impl super::Matcher for NMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ん"
                    && token.base_form == "ん"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.features.get(4).is_some_and(|f| f == "不変化型")
            })
        }
    }

    // Create matcher that handles both には and にも variants
    #[derive(Debug)]
    struct WaOrMoMatcher;
    impl super::Matcher for WaOrMoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.surface == "は" || token.surface == "も")
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
            })
        }
    }

    vec![
        super::noun(), // Preceding noun
        TokenMatcher::Custom(Arc::new(NiMatcher)),
        TokenMatcher::Custom(Arc::new(WaOrMoMatcher)), // は or も
        TokenMatcher::Custom(Arc::new(OyobuMatcher)),
        TokenMatcher::Custom(Arc::new(NaiMaseMatcher)),
        optional(TokenMatcher::Custom(Arc::new(NMatcher))),
    ]
}

// Pattern: とは言うものの
// Pattern: とは言うものの (although it is said that)
// Structures: Verb/Adj/Noun + (だ) + と + (は) + いう + ものの
// Note: は is optional
pub fn tohaiumonono() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct ToMatcher;
    impl super::Matcher for ToMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "と"
                    && token.pos.first().is_some_and(|p| p == "助詞")
                    && token.pos.get(1).is_some_and(|p| p == "格助詞")
                    && token.pos.get(2).is_some_and(|p| p == "引用")
            })
        }
    }

    #[derive(Debug)]
    struct WaMatcher;
    impl super::Matcher for WaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "は"
                    && token.pos.first().is_some_and(|p| p == "助詞")
                    && token.pos.get(1).is_some_and(|p| p == "係助詞")
            })
        }
    }

    #[derive(Debug)]
    struct IuMatcher;
    impl super::Matcher for IuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "いう"
                    && token.base_form == "いう"
                    && token.pos.first().is_some_and(|p| p == "動詞")
                    && token.pos.get(1).is_some_and(|p| p == "自立")
            })
        }
    }

    #[derive(Debug)]
    struct MononoMatcher;
    impl super::Matcher for MononoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ものの"
                    && token.base_form == "ものの"
                    && token.pos.first().is_some_and(|p| p == "助詞")
                    && token.pos.get(1).is_some_and(|p| p == "接続助詞")
            })
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(ToMatcher)),
        optional(TokenMatcher::Custom(Arc::new(WaMatcher))),
        TokenMatcher::Custom(Arc::new(IuMatcher)),
        TokenMatcher::Custom(Arc::new(MononoMatcher)),
    ]
}

// Pattern: が早いか (as soon as, no sooner than)
// Structures: Verb[dictionary] + が + 早い + か
//            Verb[連用タ接続] + た + が + 早い + か
pub fn gahayaika() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match が as 助詞/接続助詞 (conjunctive particle)
    #[derive(Debug)]
    struct GaConjunctionMatcher;
    impl Matcher for GaConjunctionMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "が"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
            })
        }
    }

    // Match 早い as adjective in basic form
    #[derive(Debug)]
    struct HayaiMatcher;
    impl Matcher for HayaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "早い"
                    && token.base_form == "早い"
                    && token.pos.first().is_some_and(|pos| pos == "形容詞")
                    && token.features.get(5).is_some_and(|f| f == "基本形")
            })
        }
    }

    // Match か as adverbial particle
    #[derive(Debug)]
    struct KaAdverbialMatcher;
    impl Matcher for KaAdverbialMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "か"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token
                        .pos
                        .get(1)
                        .is_some_and(|pos| pos == "副助詞／並立助詞／終助詞")
            })
        }
    }

    vec![
        // Match verbs in dictionary form OR past auxiliary た/だ
        or(vec![verb_form("基本形"), past_auxiliary()]),
        TokenMatcher::Custom(Arc::new(GaConjunctionMatcher)),
        TokenMatcher::Custom(Arc::new(HayaiMatcher)),
        TokenMatcher::Custom(Arc::new(KaAdverbialMatcher)),
    ]
}

// Pattern: に難くない
// Pattern: に難くない (not difficult to / not hard to)
// Structures: Noun[suru-verb] + (する) + に + 難く + ない/ありません
pub fn nikatakunai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match に as 助詞/格助詞/一般
    #[derive(Debug)]
    struct NiMatcher;
    impl Matcher for NiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "に"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                    && token.pos.get(2).is_some_and(|pos| pos == "一般")
            })
        }
    }

    // Match かたく (形容詞/自立, base=かたい, 連用テ接続)
    #[derive(Debug)]
    struct KatakuMatcher;
    impl Matcher for KatakuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "かたく"
                    && token.base_form == "かたい"
                    && token.pos.first().is_some_and(|pos| pos == "形容詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "自立")
            })
        }
    }

    // Match ない (助動詞) or あり (start of ありません polite form)
    #[derive(Debug)]
    struct NaiOrAriMatcher;
    impl Matcher for NaiOrAriMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match ない (casual negative)
                if token.surface == "ない"
                    && token.base_form == "ない"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.features.get(4).is_some_and(|f| f == "特殊・ナイ")
                {
                    return true;
                }

                // Match あり (polite negative prep)
                if token.surface == "あり"
                    && token.base_form == "ある"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "自立")
                {
                    return true;
                }

                false
            })
        }
    }

    // Optional ませ for polite negative
    #[derive(Debug)]
    struct MaseMatcher;
    impl Matcher for MaseMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ませ"
                    && token.base_form == "ます"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token
                        .features
                        .get(4)
                        .is_some_and(|f| f.starts_with("特殊・マス"))
            })
        }
    }

    // Optional ん for polite negative
    #[derive(Debug)]
    struct NMatcher;
    impl Matcher for NMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ん"
                    && token.base_form == "ん"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.features.get(4).is_some_and(|f| f == "不変化型")
            })
        }
    }

    vec![
        noun(),                      // Preceding noun (想像, 予想, 理解, etc.)
        optional(verb_base("する")), // Optional する for suru-verbs
        TokenMatcher::Custom(Arc::new(NiMatcher)),
        TokenMatcher::Custom(Arc::new(KatakuMatcher)),
        TokenMatcher::Custom(Arc::new(NaiOrAriMatcher)),
        optional(TokenMatcher::Custom(Arc::new(MaseMatcher))),
        optional(TokenMatcher::Custom(Arc::new(NMatcher))),
    ]
}

// Pattern: ならいざ知らず (I don't know about A, but B / maybe A, but B)
// Structures: Any + なら/は + いざ + 知らず
pub fn naraizashirazu() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match なら (助動詞, base=だ, 仮定形)
    #[derive(Debug)]
    struct NaraMatcher;
    impl Matcher for NaraMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "なら"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.base_form == "だ"
                    && token.features.get(5).is_some_and(|f| f == "仮定形")
            })
        }
    }

    // Match は (助詞/係助詞)
    #[derive(Debug)]
    struct WaMatcher;
    impl Matcher for WaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "は"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
            })
        }
    }

    // Match いざ (副詞/一般)
    #[derive(Debug)]
    struct IzaMatcher;
    impl Matcher for IzaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "いざ"
                    && token.pos.first().is_some_and(|pos| pos == "副詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "一般")
            })
        }
    }

    // Match しら (動詞/自立, base=しる, 未然形)
    #[derive(Debug)]
    struct ShiraMatcher;
    impl Matcher for ShiraMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "しら"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "自立")
                    && token.base_form == "しる"
                    && token.features.get(5).is_some_and(|f| f == "未然形")
            })
        }
    }

    // Match ず (助動詞, base=ぬ, 連用ニ接続)
    #[derive(Debug)]
    struct ZuMatcher;
    impl Matcher for ZuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ず"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.base_form == "ぬ"
                    && token.features.get(5).is_some_and(|f| f == "連用ニ接続")
            })
        }
    }

    vec![
        any(), // Preceding word: Noun, Verb, Adjective, or の
        // Match なら (助動詞, base=だ, 仮定形) OR は (助詞/係助詞)
        or(vec![
            TokenMatcher::Custom(Arc::new(NaraMatcher)),
            TokenMatcher::Custom(Arc::new(WaMatcher)),
        ]),
        TokenMatcher::Custom(Arc::new(IzaMatcher)),
        TokenMatcher::Custom(Arc::new(ShiraMatcher)),
        TokenMatcher::Custom(Arc::new(ZuMatcher)),
    ]
}

// Pattern: を禁じ得ない (cannot help feeling / cannot hold back from)
// Structures: Noun/の + を + 禁じ得ない/禁じ得ません
pub fn wokinjienai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match きんじ (base=きんじる, 動詞/自立, 連用形)
    #[derive(Debug)]
    struct KinjiMatcher;
    impl Matcher for KinjiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "きんじ"
                    && token.base_form == "きんじる"
                    && token.pos.first().is_some_and(|p| p == "動詞")
                    && token.pos.get(1).is_some_and(|p| p == "自立")
                    && token.features.get(5).is_some_and(|f| f == "連用形")
            })
        }
    }

    // Match え (base=える, 動詞/非自立, 未然形 or 連用形)
    #[derive(Debug)]
    struct EMatcher;
    impl Matcher for EMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "え"
                    && token.base_form == "える"
                    && token.pos.first().is_some_and(|p| p == "動詞")
                    && token.pos.get(1).is_some_and(|p| p == "非自立")
                    && token
                        .features
                        .get(5)
                        .is_some_and(|f| f == "未然形" || f == "連用形")
            })
        }
    }

    // Match ない/なかっ (助動詞, 特殊・ナイ) OR ませ (助動詞, 特殊・マス)
    #[derive(Debug)]
    struct NaiOrMaseMatcher;
    impl Matcher for NaiOrMaseMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.pos.first().is_some_and(|p| p == "助動詞")
                    && (((token.surface == "ない" || token.surface == "なかっ")
                        && token.features.get(4).is_some_and(|f| f == "特殊・ナイ"))
                        || (token.surface == "ませ"
                            && token.features.get(4).is_some_and(|f| f == "特殊・マス")))
            })
        }
    }

    // Match ん (助動詞, 不変化型) - for polite negative
    #[derive(Debug)]
    struct NMatcher;
    impl Matcher for NMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ん"
                    && token.pos.first().is_some_and(|p| p == "助動詞")
                    && token.features.get(4).is_some_and(|f| f == "不変化型")
            })
        }
    }

    vec![
        any(), // Preceding noun or の
        surface_particle("を", "格助詞"),
        TokenMatcher::Custom(Arc::new(KinjiMatcher)),
        TokenMatcher::Custom(Arc::new(EMatcher)),
        TokenMatcher::Custom(Arc::new(NaiOrMaseMatcher)),
        optional(TokenMatcher::Custom(Arc::new(NMatcher))),
    ]
}

// Pattern: にかこつけて (under the pretense of / using as an excuse)
// Structures: Noun + にかこつけて, Verb + の + にかこつけて
pub fn nikakotsukete() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match かこつけ (動詞/自立, base=かこつける, 連用形)
    #[derive(Debug)]
    struct KakotsukeMatcher;
    impl Matcher for KakotsukeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "かこつけ"
                    && token.base_form == "かこつける"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token.features.get(5).is_some_and(|f| f == "連用形")
            })
        }
    }

    vec![
        any(), // Noun or の
        surface_particle("に", "格助詞"),
        TokenMatcher::Custom(Arc::new(KakotsukeMatcher)),
        surface_particle("て", "接続助詞"),
    ]
}

// Pattern: ようによっては (depending on the way that)
// Structures: Verb[stem] + ようによっては
// Meaning: "depending on the way that (A), (B)" / "depending on how (A)"
// Handles three tokenization patterns:
// 1. Verb(未然ウ接続) + う(助動詞) + によって + は (volitional form like 見よう)
// 2. Verb(連用形) + よう(名詞/接尾) + によって + は (よう as suffix like 混みよう)
// 3. Noun + よう(名詞/接尾) + によって + は (noun from verb stem like 使いよう)
pub fn youniyotteha() -> Vec<TokenMatcher> {
    // Match よう as noun suffix (名詞/接尾/一般) OR う as volitional (助動詞)
    #[derive(Debug)]
    struct YouOrUMatcher;
    impl super::Matcher for YouOrUMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match よう as noun suffix
                (token.surface == "よう"
                && token.base_form == "よう"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾"))
            // OR match う as volitional auxiliary
            || (token.surface == "う"
                && token.base_form == "う"
                && token.pos.first().is_some_and(|pos| pos == "助動詞"))
            })
        }
    }

    // Match によって as particle (助詞/格助詞/連語)
    #[derive(Debug)]
    struct NiyotteMatcher;
    impl super::Matcher for NiyotteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "によって"
                    && token.base_form == "によって"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            })
        }
    }

    // Match は as adverbial particle (助詞/係助詞)
    #[derive(Debug)]
    struct WaParticleMatcher;
    impl super::Matcher for WaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "は"
                    && token.base_form == "は"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
            })
        }
    }

    vec![
        any(),                                             // Verb (未然ウ接続 or 連用形) or Noun
        TokenMatcher::Custom(Arc::new(YouOrUMatcher)),     // よう or う
        TokenMatcher::Custom(Arc::new(NiyotteMatcher)),    // によって
        TokenMatcher::Custom(Arc::new(WaParticleMatcher)), // は
    ]
}

// Pattern: べくもない (impossible to / no way to)
// Structures: Verb + べく + も + ない/なかった/ありません
pub fn bekumonai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match べく (助動詞, base=べし, 文語・ベシ, 連用形)
    #[derive(Debug)]
    struct BekuMatcher;
    impl super::Matcher for BekuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "べく"
                    && token.base_form == "べし"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    // Match も (助詞/係助詞)
    #[derive(Debug)]
    struct MoMatcher;
    impl super::Matcher for MoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "も" && token.pos.first().is_some_and(|pos| pos == "助詞")
            })
        }
    }

    // Match ない/なかっ (形容詞/自立, base=ない) OR あり (動詞/自立, base=ある)
    #[derive(Debug)]
    struct NaiOrAriMatcher;
    impl super::Matcher for NaiOrAriMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match ない or なかっ (adjective)
                if token.base_form == "ない" && token.pos.first().is_some_and(|pos| pos == "形容詞")
                {
                    return true;
                }
                // Match あり (polite form: ありません)
                if token.surface == "あり"
                    && token.base_form == "ある"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                {
                    return true;
                }
                false
            })
        }
    }

    // Optional: Match ませ (助動詞, base=ます) for polite form
    #[derive(Debug)]
    struct MaseMatcher;
    impl super::Matcher for MaseMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ませ"
                    && token.base_form == "ます"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    // Optional: Match ん (助動詞, 不変化型) for polite negative
    #[derive(Debug)]
    struct NMatcher;
    impl super::Matcher for NMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ん" && token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    vec![
        any(), // Verb (including する-verbs with す stem)
        TokenMatcher::Custom(Arc::new(BekuMatcher)),
        TokenMatcher::Custom(Arc::new(MoMatcher)),
        TokenMatcher::Custom(Arc::new(NaiOrAriMatcher)),
        optional(TokenMatcher::Custom(Arc::new(MaseMatcher))),
        optional(TokenMatcher::Custom(Arc::new(NMatcher))),
        optional(past_auxiliary()), // Match た (助動詞, 特殊・タ) for past tense
    ]
}

// Pattern: と来たら (when it comes to / concerning)
// Structures: Phrase + と + 来 + たら
// Note: Written in kanji (と来たら), unlike ときたら (kana form)
// Kagome tokenizes as: と + 来 (来る verb) + たら
pub fn tokitara_2() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matches と (case-marking particle)
    #[derive(Debug)]
    struct ToMatcher;
    impl super::Matcher for ToMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "と"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            })
        }
    }

    // Matches 来 (来る in 連用形)
    #[derive(Debug)]
    struct KiMatcher;
    impl super::Matcher for KiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "来"
                    && token.base_form == "来る"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "自立")
                    && token.features.get(5).is_some_and(|f| f == "連用形")
            })
        }
    }

    // Matches たら (仮定形 of た auxiliary)
    #[derive(Debug)]
    struct TaraMatcher;
    impl super::Matcher for TaraMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "たら"
                    && token.base_form == "た"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.features.get(5).is_some_and(|f| f == "仮定形")
            })
        }
    }

    vec![
        any(), // Preceding phrase (noun, adjective + noun, etc.)
        TokenMatcher::Custom(Arc::new(ToMatcher)),
        TokenMatcher::Custom(Arc::new(KiMatcher)),
        TokenMatcher::Custom(Arc::new(TaraMatcher)),
    ]
}

// Pattern: ものとして (supposing that / on the assumption that)
// Structures: Verb/Adjective/Noun + もの + として
pub fn monotoshite() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct MonoMatcher;
    impl super::Matcher for MonoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "もの"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "非自立")
            })
        }
    }

    #[derive(Debug)]
    struct ToshiteMatcher;
    impl super::Matcher for ToshiteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "として"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            })
        }
    }

    vec![
        // Match the token immediately before もの
        // This will be: verb, adjective, な (for な-adj), or ある (for である)
        any(),
        TokenMatcher::Custom(Arc::new(MonoMatcher)),
        TokenMatcher::Custom(Arc::new(ToshiteMatcher)),
    ]
}

// Pattern: を前提に (on the premise of / on the assumption that)
// Structures: Noun/こと + を + 前提 + に/にして/として
pub fn wozenteini() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match を (助詞/格助詞/一般)
    #[derive(Debug)]
    struct WoMatcher;
    impl super::Matcher for WoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "を"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            })
        }
    }

    // Match ぜん (名詞/一般, base=ぜん)
    #[derive(Debug)]
    struct ZenMatcher;
    impl super::Matcher for ZenMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ぜん"
                    && token.base_form == "ぜん"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
            })
        }
    }

    // Match てい (名詞/一般, base=てい)
    #[derive(Debug)]
    struct TeiMatcher;
    impl super::Matcher for TeiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "てい"
                    && token.base_form == "てい"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
            })
        }
    }

    // Match に (助詞/格助詞) OR として (助詞/格助詞/連語)
    #[derive(Debug)]
    struct NiOrToshiteMatcher;
    impl super::Matcher for NiOrToshiteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                if token.surface == "に"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                {
                    return true;
                }
                if token.surface == "として"
                    && token.base_form == "として"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                {
                    return true;
                }
                false
            })
        }
    }

    // Optional: Match し (動詞/自立, base=する, 連用形) for にして variant
    #[derive(Debug)]
    struct ShiMatcher;
    impl super::Matcher for ShiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "し"
                    && token.base_form == "する"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token.features.get(5).is_some_and(|f| f == "連用形")
            })
        }
    }

    // Optional: Match て (助詞/接続助詞) for にして variant
    #[derive(Debug)]
    struct TeMatcher;
    impl super::Matcher for TeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "て"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
            })
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(WoMatcher)),
        TokenMatcher::Custom(Arc::new(ZenMatcher)),
        TokenMatcher::Custom(Arc::new(TeiMatcher)),
        TokenMatcher::Custom(Arc::new(NiOrToshiteMatcher)),
        optional(TokenMatcher::Custom(Arc::new(ShiMatcher))),
        optional(TokenMatcher::Custom(Arc::new(TeMatcher))),
    ]
}

// Pattern: ずにはすまない (won't get away without doing / have no choice but to do)
// Structures: Verb[ない] + ず + には + すまない
pub fn zunihasumanai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match verb in mizenkei (negative stem) form for ず
    #[derive(Debug)]
    struct MizenkeiVerbMatcher;
    impl super::Matcher for MizenkeiVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.pos.first().is_some_and(|pos| pos == "動詞") &&
                // Match verbs in 未然形, 未然ヌ接続, or 未然レル接続
                token.features.get(5).is_some_and(|f| {
                    f == "未然形" || f == "未然ヌ接続" || f == "未然レル接続"
                })
            })
        }
    }

    // Match ず (classical negative auxiliary)
    #[derive(Debug)]
    struct ZuMatcher;
    impl super::Matcher for ZuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ず"
                    && token.base_form == "ぬ"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.features.get(4).is_some_and(|f| f == "特殊・ヌ")
            })
        }
    }

    // Match ない (negative auxiliary)
    #[derive(Debug)]
    struct NaiMatcher;
    impl super::Matcher for NaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ない"
                    && token.base_form == "ない"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.features.get(4).is_some_and(|f| f == "特殊・ナイ")
            })
        }
    }

    // Match すま (from すむ)
    #[derive(Debug)]
    struct SumaMatcher;
    impl super::Matcher for SumaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "すま"
                    && token.base_form == "すむ"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token.features.get(5).is_some_and(|f| f == "未然形")
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(MizenkeiVerbMatcher)),
        TokenMatcher::Custom(Arc::new(ZuMatcher)),
        surface("に"),
        surface("は"),
        TokenMatcher::Custom(Arc::new(SumaMatcher)),
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
    ]
}

// Pattern: ないではすまない (won't get away without doing / have no choice but to do - ないでは form)
// Structure: Verb[ない] + では + すまない
pub fn naidewasumanai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match verb in mizenkei (negative stem) form
    #[derive(Debug)]
    struct MizenkeiVerbMatcher;
    impl super::Matcher for MizenkeiVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.pos.first().is_some_and(|pos| pos == "動詞") &&
                // Match verbs in 未然形
                token.features.get(5).is_some_and(|f| f == "未然形")
            })
        }
    }

    // Match ない (negative auxiliary)
    #[derive(Debug)]
    struct NaiMatcher;
    impl super::Matcher for NaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ない"
                    && token.base_form == "ない"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.features.get(4).is_some_and(|f| f == "特殊・ナイ")
            })
        }
    }

    // Match で (from だ or as conjunctive particle)
    // Handles two tokenization variants:
    // 1. で (助動詞, base=だ) - copula form
    // 2. で (助詞/接続助詞, base=で) - conjunctive particle
    #[derive(Debug)]
    struct DeMatcher;
    impl super::Matcher for DeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "で"
                    && (
                        // Case 1: で as copula auxiliary (助動詞, base=だ)
                        (token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.base_form == "だ"
                        && token.features.get(4).is_some_and(|f| f == "特殊・ダ")) ||
                    // Case 2: で as conjunctive particle (助詞/接続助詞, base=で)
                    (token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
                        && token.base_form == "で")
                    )
            })
        }
    }

    // Match すま (from すむ)
    #[derive(Debug)]
    struct SumaMatcher;
    impl super::Matcher for SumaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "すま"
                    && token.base_form == "すむ"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token.features.get(5).is_some_and(|f| f == "未然形")
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(MizenkeiVerbMatcher)),
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
        TokenMatcher::Custom(Arc::new(DeMatcher)),
        surface("は"),
        TokenMatcher::Custom(Arc::new(SumaMatcher)),
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
    ]
}

// Pattern: に堪えない
// Pattern: に堪えない (cannot bear to / cannot tolerate)
// Structures: Verb + に堪えない, Adverb + に堪えない, Noun + に堪えない
pub fn nikotaenai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match に (助詞/格助詞/一般)
    #[derive(Debug)]
    struct NiMatcher;
    impl super::Matcher for NiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "に"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            })
        }
    }

    // Match たえ (動詞/自立, base=たえる, 未然形)
    #[derive(Debug)]
    struct TaeMatcher;
    impl super::Matcher for TaeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "たえ"
                    && token.base_form == "たえる"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "自立")
            })
        }
    }

    // Match ない/なかっ (助動詞, base=ない, 特殊・ナイ)
    #[derive(Debug)]
    struct NaiMatcher;
    impl super::Matcher for NaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.surface == "ない" || token.surface == "なかっ")
                    && token.base_form == "ない"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(NiMatcher)),
        TokenMatcher::Custom(Arc::new(TaeMatcher)),
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
        optional(past_auxiliary()), // Match た (助動詞, base=た) - optional past tense marker
    ]
}

// Pattern: 始末だ (wind up as / end up as / culminate in - negative outcome)
// Structures: Verb[る] + 始末だ, この + 始末だ
pub fn shimatsuda() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match し (助動詞, base=き, 文語・キ, 体言接続)
    #[derive(Debug)]
    struct ShiMatcher;
    impl super::Matcher for ShiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "し"
                    && token.base_form == "き"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.features.get(4).is_some_and(|f| f == "文語・キ")
                    && token.features.get(5).is_some_and(|f| f == "体言接続")
            })
        }
    }

    // Match まつ (名詞/一般, base=まつ)
    #[derive(Debug)]
    struct MatsuMatcher;
    impl super::Matcher for MatsuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "まつ"
                    && token.base_form == "まつ"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "一般")
            })
        }
    }

    // Match だ (助動詞, base=だ, 特殊・ダ)
    #[derive(Debug)]
    struct DaMatcher;
    impl super::Matcher for DaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "だ"
                    && token.base_form == "だ"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.features.get(4).is_some_and(|f| f == "特殊・ダ")
            })
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(ShiMatcher)),
        TokenMatcher::Custom(Arc::new(MatsuMatcher)),
        TokenMatcher::Custom(Arc::new(DaMatcher)),
    ]
}

// Pattern: ものなら② (if you were to / if one happens to)
// Structures: Verb[volitional] + ものなら/もんなら
pub fn mononara_u2461() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match volitional auxiliary う (助動詞, 不変化型)
    #[derive(Debug)]
    struct VolitionalUMatcher;
    impl super::Matcher for VolitionalUMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "う"
                    && token.base_form == "う"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.features.get(4).is_some_and(|f| f == "不変化型")
            })
        }
    }

    // Match もの or もん (名詞/非自立/一般)
    #[derive(Debug)]
    struct MonoOrMonMatcher;
    impl super::Matcher for MonoOrMonMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.surface == "もの" || token.surface == "もん")
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "非自立")
            })
        }
    }

    // Match なら (助動詞, base=だ, 仮定形)
    #[derive(Debug)]
    struct NaraMatcher;
    impl super::Matcher for NaraMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "なら"
                    && token.base_form == "だ"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.features.get(5).is_some_and(|f| f == "仮定形")
            })
        }
    }

    vec![
        verb_form("未然ウ接続"),
        TokenMatcher::Custom(Arc::new(VolitionalUMatcher)),
        TokenMatcher::Custom(Arc::new(MonoOrMonMatcher)),
        TokenMatcher::Custom(Arc::new(NaraMatcher)),
    ]
}

// Pattern: にひきかえ (in stark contrast to / in comparison to)
// Structures:
//   - Noun + にひきかえ
//   - な-Adjective + な/である + の + にひきかえ
//   - い-Adjective + の + にひきかえ
//   - Verb + の + にひきかえ
//   - それ + にひきかえ
pub fn nihikikae() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match に as 助詞/格助詞/一般
    #[derive(Debug)]
    struct NiMatcher;
    impl super::Matcher for NiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "に"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                        && token.pos.get(2).is_some_and(|pos| pos == "一般") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match ひきかえ as 動詞/自立, base=ひきかえる, 連用形
    #[derive(Debug)]
    struct HikikaeruMatcher;
    impl super::Matcher for HikikaeruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ひきかえ"
                        && token.base_form == "ひきかえる"
                        && token.pos.first().is_some_and(|pos| pos == "動詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "自立")
                        && token.features.get(5).is_some_and(|f| f == "連用形") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Preceding word (noun, の, それ, etc.)
        TokenMatcher::Custom(Arc::new(NiMatcher)),
        TokenMatcher::Custom(Arc::new(HikikaeruMatcher)),
    ]
}

// Pattern: それまでだ (if that happens, it's all over)
// Structures: Verb[たら] + それまでだ / Verb[ば] + それまでだ
pub fn soremadeda() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match それ pronoun
    #[derive(Debug)]
    struct SoreMatcher;
    impl super::Matcher for SoreMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "それ"
                        && token.pos.first().is_some_and(|pos| pos == "名詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "代名詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match まで particle
    #[derive(Debug)]
    struct MadeParticleMatcher;
    impl super::Matcher for MadeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "まで"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "副助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match either たら or ば
    #[derive(Debug)]
    struct ConditionalMarkerMatcher;
    impl super::Matcher for ConditionalMarkerMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match たら (助動詞)
                if token.surface == "たら"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.base_form == "た"
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
                false
            })
        }
    }

    vec![
        or(vec![verb_form("連用タ接続"), verb_form("仮定形")]),
        TokenMatcher::Custom(Arc::new(ConditionalMarkerMatcher)),
        TokenMatcher::Custom(Arc::new(SoreMatcher)),
        TokenMatcher::Custom(Arc::new(MadeParticleMatcher)),
        past_auxiliary(),
    ]
}

// Pattern: といおうか (how to put it / shall I say)
// Structures: Any + といおうか
pub fn toiouka() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match と as quotation particle (助詞/格助詞/引用)
    #[derive(Debug)]
    struct ToMatcher;
    impl super::Matcher for ToMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "と"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                        && token.pos.get(2).is_some_and(|pos| pos == "引用") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match いお (verb いう in 未然ウ接続 form)
    #[derive(Debug)]
    struct IoMatcher;
    impl super::Matcher for IoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "いお"
                        && token.base_form == "いう"
                        && token.pos.first().is_some_and(|pos| pos == "動詞")
                        && token.features.get(5).is_some_and(|f| f == "未然ウ接続") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match う auxiliary (助動詞, 不変化型)
    #[derive(Debug)]
    struct UMatcher;
    impl super::Matcher for UMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "う"
                        && token.base_form == "う"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match か particle (助詞/副助詞／並立助詞／終助詞)
    #[derive(Debug)]
    struct KaMatcher;
    impl super::Matcher for KaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "か"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token
                            .pos
                            .get(1)
                            .is_some_and(|pos| pos == "副助詞／並立助詞／終助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(ToMatcher)),
        TokenMatcher::Custom(Arc::new(IoMatcher)),
        TokenMatcher::Custom(Arc::new(UMatcher)),
        TokenMatcher::Custom(Arc::new(KaMatcher)),
    ]
}

// Pattern: ずにはおかない (will certainly do / will not fail to do)
// Structure: Verb[ない-stem] + ず + には + おかない
pub fn zunihaokanai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match verb in mizenkei (negative stem) form for ず
    #[derive(Debug)]
    struct MizenkeiVerbMatcher;
    impl super::Matcher for MizenkeiVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                if token.pos.first().is_none_or(|pos| pos != "動詞") {
                    return false;
                }
                // Match verbs in 未然形, 未然ヌ接続, or 未然レル接続
                token
                    .features
                    .get(5)
                    .is_some_and(|f| f == "未然形" || f == "未然ヌ接続" || f == "未然レル接続")
            })
        }
    }

    // Match ず (classical negative auxiliary)
    #[derive(Debug)]
    struct ZuMatcher;
    impl super::Matcher for ZuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ず"
                        && token.base_form == "ぬ"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.features.get(4).is_some_and(|f| f == "特殊・ヌ") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match ない (negative auxiliary)
    #[derive(Debug)]
    struct NaiMatcher;
    impl super::Matcher for NaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ない"
                        && token.base_form == "ない"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.features.get(4).is_some_and(|f| f == "特殊・ナイ") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match おか (from おく)
    #[derive(Debug)]
    struct OkaMatcher;
    impl super::Matcher for OkaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "おか"
                        && token.base_form == "おく"
                        && token.pos.first().is_some_and(|pos| pos == "動詞")
                        && token.features.get(5).is_some_and(|f| f == "未然形") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(MizenkeiVerbMatcher)),
        TokenMatcher::Custom(Arc::new(ZuMatcher)),
        surface("に"),
        surface("は"),
        TokenMatcher::Custom(Arc::new(OkaMatcher)),
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
    ]
}

// Pattern: ないではおかない (will certainly do / will not fail to do - ないでは form)
// Structure: Verb[ない] + では + おかない
pub fn naidewaokanai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ない (negative auxiliary)
    #[derive(Debug)]
    struct NaiMatcher;
    impl super::Matcher for NaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ない"
                        && token.base_form == "ない"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.features.get(4).is_some_and(|f| f == "特殊・ナイ") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match で (from だ)
    #[derive(Debug)]
    struct DeMatcher;
    impl super::Matcher for DeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "で"
                        && token.base_form == "だ"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.features.get(4).is_some_and(|f| f == "特殊・ダ") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match おか (from おく)
    #[derive(Debug)]
    struct OkaMatcher;
    impl super::Matcher for OkaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "おか"
                        && token.base_form == "おく"
                        && token.pos.first().is_some_and(|pos| pos == "動詞")
                        && token.features.get(5).is_some_and(|f| f == "未然形") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        verb_form("未然形"),
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
        TokenMatcher::Custom(Arc::new(DeMatcher)),
        surface("は"),
        TokenMatcher::Custom(Arc::new(OkaMatcher)),
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
    ]
}

// Pattern: を限りに (ending with / no longer than / as of)
// Structures: Noun + を限（かぎ）りに, Noun + 限（かぎ）りで
pub fn wokagirini() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct WoParticle;
    impl super::Matcher for WoParticle {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "を"
                    && token.pos.first().is_some_and(|p| p == "助詞")
                    && token.pos.get(1).is_some_and(|p| p == "格助詞")
            })
        }
    }

    #[derive(Debug)]
    struct KagiriNoun;
    impl super::Matcher for KagiriNoun {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.surface == "かぎり" || token.surface == "限り")
                    && (token.base_form == "かぎり" || token.base_form == "限り")
                    && token.pos.first().is_some_and(|p| p == "名詞")
            })
        }
    }

    #[derive(Debug)]
    struct NiDeParticle;
    impl super::Matcher for NiDeParticle {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.surface == "に" || token.surface == "で")
                    && token.pos.first().is_some_and(|p| p == "助詞")
                    && token.pos.get(1).is_some_and(|p| p == "格助詞")
            })
        }
    }

    vec![
        super::noun(),
        optional(TokenMatcher::Custom(Arc::new(WoParticle))),
        TokenMatcher::Custom(Arc::new(KagiriNoun)),
        TokenMatcher::Custom(Arc::new(NiDeParticle)),
    ]
}

// Pattern: てはかなわない (can't stand, unbearable)
// Structures: Verb[て]/Adj[くて]/Noun[で] + は + かなわない
pub fn tehakanawanai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match て or で (conjunctive particle or copula)
    #[derive(Debug)]
    struct TeOrDe;
    impl super::Matcher for TeOrDe {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token|
                // て as 助詞/接続助詞 (after verb or い-adj)
                (token.surface == "て"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "接続助詞"))
                ||
                // で as 助動詞 from だ (after な-adj or noun)
                (token.surface == "で"
                    && token.base_form == "だ"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")))
        }
    }

    // Match は as 係助詞
    #[derive(Debug)]
    struct WaParticle;
    impl super::Matcher for WaParticle {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "は"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
            })
        }
    }

    // Match かなわ (未然形 of かなう)
    #[derive(Debug)]
    struct KanawaVerb;
    impl super::Matcher for KanawaVerb {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "かなわ"
                    && token.base_form == "かなう"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
            })
        }
    }

    // Match ない (auxiliary)
    #[derive(Debug)]
    struct NaiAuxiliary;
    impl super::Matcher for NaiAuxiliary {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ない" && token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    vec![
        any(), // Verb, Adjective, or Noun before て/で
        TokenMatcher::Custom(Arc::new(TeOrDe)),
        TokenMatcher::Custom(Arc::new(WaParticle)),
        TokenMatcher::Custom(Arc::new(KanawaVerb)),
        TokenMatcher::Custom(Arc::new(NaiAuxiliary)),
    ]
}

// Pattern: かたがた (in addition to, along with, while doing)
// Structures: Noun + かたがた
pub fn katagata() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct KatagataNoun;
    impl super::Matcher for KatagataNoun {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "かたがた"
                    && token.base_form == "かたがた"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
            })
        }
    }

    vec![super::noun(), TokenMatcher::Custom(Arc::new(KatagataNoun))]
}

// Pattern: を余儀なくさせる (force/compel to)
// Structures: Noun/Verb[こと] + を + 余儀なく + させる/させます
pub fn woyoginakusaseru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match を as case particle
    #[derive(Debug)]
    struct WoParticle;
    impl super::Matcher for WoParticle {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "を"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            })
        }
    }

    // Match 余儀なく (adjective, base=余儀ない, 連用テ接続)
    #[derive(Debug)]
    struct YoginakuAdj;
    impl super::Matcher for YoginakuAdj {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "余儀なく"
                    && token.base_form == "余儀ない"
                    && token.pos.first().is_some_and(|pos| pos == "形容詞")
            })
        }
    }

    // Match さ (from する, 未然レル接続)
    #[derive(Debug)]
    struct SaVerb;
    impl super::Matcher for SaVerb {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "さ"
                    && token.base_form == "する"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
            })
        }
    }

    // Match せ (auxiliary verb せる, 一段, 連用形)
    #[derive(Debug)]
    struct SeVerb;
    impl super::Matcher for SeVerb {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "せ"
                    && token.base_form == "せる"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "接尾")
            })
        }
    }

    vec![
        super::noun(),
        TokenMatcher::Custom(Arc::new(WoParticle)),
        TokenMatcher::Custom(Arc::new(YoginakuAdj)),
        TokenMatcher::Custom(Arc::new(SaVerb)),
        TokenMatcher::Custom(Arc::new(SeVerb)),
        any(), // た, ます, る, etc.
    ]
}

// Pattern: ～てやる (do for someone / I'll do it!)
// Structures: Verb[て] + やる/やります
pub fn uff5e_teyaru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match て particle
    #[derive(Debug)]
    struct TeParticle;
    impl super::Matcher for TeParticle {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "て"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
            })
        }
    }

    // Match やる as auxiliary verb (動詞/非自立, base=やる)
    #[derive(Debug)]
    struct YaruAuxiliary;
    impl super::Matcher for YaruAuxiliary {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.base_form == "やる"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "非自立")
            })
        }
    }

    vec![
        super::flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(TeParticle)),
        TokenMatcher::Custom(Arc::new(YaruAuxiliary)),
    ]
}

// Pattern: ただ〜のみ (nothing but, all that remains)
// Structures: Verb[る] + のみ + (だ/です/である)
//            Noun[サ変] + ある + のみ + (だ/です/である)
pub fn tada_u301c_nomi() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ある (動詞, for サ変 + ある construction)
    #[derive(Debug)]
    struct AruVerbMatcher;
    impl super::Matcher for AruVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ある"
                        && token.base_form == "ある"
                        && token.pos.first().is_some_and(|pos| pos == "動詞")
                        && token.features.get(5).is_some_and(|form| form == "基本形") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match のみ as 助詞/副助詞
    #[derive(Debug)]
    struct NomiMatcher;
    impl super::Matcher for NomiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "のみ"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "副助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match だ (助動詞/特殊・ダ/基本形)
    #[derive(Debug)]
    struct DaCopulaMatcher;
    impl super::Matcher for DaCopulaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "だ"
                        && token.base_form == "だ"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match です (助動詞/特殊・デス/基本形)
    #[derive(Debug)]
    struct DesuCopulaMatcher;
    impl super::Matcher for DesuCopulaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "です"
                        && token.base_form == "です"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match で from である (助動詞/特殊・ダ/連用形, base=だ)
    #[derive(Debug)]
    struct DeCopulaMatcher;
    impl super::Matcher for DeCopulaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "で"
                        && token.base_form == "だ"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.features.get(5).is_some_and(|form| form == "連用形") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match ある from である (助動詞/五段・ラ行アル/基本形)
    #[derive(Debug)]
    struct AruCopulaMatcher;
    impl super::Matcher for AruCopulaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "ある"
                        && token.base_form == "ある"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        // Match verb in 基本形 or noun
        or(vec![verb_form("基本形"), noun()]),
        optional(TokenMatcher::Custom(Arc::new(AruVerbMatcher))),
        TokenMatcher::Custom(Arc::new(NomiMatcher)),
        optional(TokenMatcher::Custom(Arc::new(DaCopulaMatcher))),
        optional(TokenMatcher::Custom(Arc::new(DesuCopulaMatcher))),
        optional(TokenMatcher::Custom(Arc::new(DeCopulaMatcher))),
        optional(TokenMatcher::Custom(Arc::new(AruCopulaMatcher))),
    ]
}

// Pattern: ものとする (shall / supposing that / on the assumption that)
// Meaning: Presents a determination that (A) is true for the sake of discussion
// Structures: Verb + もの + と + する/します
pub fn monotosuru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match もの as non-autonomous noun
    #[derive(Debug)]
    struct MonoNounMatcher;
    impl super::Matcher for MonoNounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "もの"
                        && token.pos.first().is_some_and(|pos| pos == "名詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "非自立") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match と as case particle
    #[derive(Debug)]
    struct ToParticleMatcher;
    impl super::Matcher for ToParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "と"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        verb(),
        TokenMatcher::Custom(Arc::new(MonoNounMatcher)),
        TokenMatcher::Custom(Arc::new(ToParticleMatcher)),
        verb_base("する"),
    ]
}

// Pattern: との (quotation particle + nominalizer)
// Meaning: "that" (marks a statement as quoted information)
// Structures: Verb/Adj/Noun + (だ) + との + Noun
pub fn tono() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match と as quotation particle
    #[derive(Debug)]
    struct ToQuotationMatcher;
    impl super::Matcher for ToQuotationMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "と"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                        && token.pos.get(2).is_some_and(|pos| pos == "引用") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match の as nominalizer/connector particle (助詞/連体化)
    #[derive(Debug)]
    struct NoNominalizerMatcher;
    impl super::Matcher for NoNominalizerMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "の"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "連体化") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(ToQuotationMatcher)),
        TokenMatcher::Custom(Arc::new(NoNominalizerMatcher)),
        super::noun(),
    ]
}

// Pattern: 以前
// Pattern: 以前 (before even, prior to - emphatic criticism)
// Structures: 以前 + に/の
pub fn izen() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match 以前 as 名詞/副詞可能
    #[derive(Debug)]
    struct IzenMatcher;
    impl super::Matcher for IzenMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "以前"
                        && token.base_form == "以前"
                        && token.pos.first().is_some_and(|pos| pos == "名詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "副詞可能") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match に or の particle after 以前
    #[derive(Debug)]
    struct NiNoParticleMatcher;
    impl super::Matcher for NiNoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if (token.surface == "に" || token.surface == "の")
                        && token.pos.first().is_some_and(|pos| pos == "助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(IzenMatcher)),
        TokenMatcher::Custom(Arc::new(NiNoParticleMatcher)),
    ]
}

// Pattern: ともあろう (of all people, such as)
// Structures: Noun + と + も + あろう + もの/方/人 + が
pub fn tomoarou() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match と as 格助詞 (引用 or 一般)
    #[derive(Debug)]
    struct ToParticleMatcher;
    impl super::Matcher for ToParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "と"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match も as 係助詞
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl super::Matcher for MoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "も"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "係助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match あろ (ある in 未然ウ接続 form)
    #[derive(Debug)]
    struct AroMatcher;
    impl super::Matcher for AroMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "あろ"
                        && token.base_form == "ある"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match う (volitional auxiliary)
    #[derive(Debug)]
    struct UMatcher;
    impl super::Matcher for UMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "う"
                        && token.base_form == "う"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match が as 格助詞
    #[derive(Debug)]
    struct GaParticleMatcher;
    impl super::Matcher for GaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "が"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        noun(),
        TokenMatcher::Custom(Arc::new(ToParticleMatcher)),
        TokenMatcher::Custom(Arc::new(MoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(AroMatcher)),
        TokenMatcher::Custom(Arc::new(UMatcher)),
        noun(), // Match noun (もの/方/人 etc.)
        TokenMatcher::Custom(Arc::new(GaParticleMatcher)),
    ]
}

// Pattern: こそすれ〜ない (certainly not B, but A)
// Structures: Noun/Verb + こそ + すれ
// Note: Classical realis form of する, but tokenizes as すれる (verb, 連用形)
pub fn kososure_u301c_nai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match こそ particle
    #[derive(Debug)]
    struct KosoMatcher;
    impl super::Matcher for KosoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "こそ"
                        && token.base_form == "こそ"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "係助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match すれ (tokenized as すれる verb in 連用形)
    #[derive(Debug)]
    struct SureMatcher;
    impl super::Matcher for SureMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "すれ"
                        && token.base_form == "すれる"
                        && token.pos.first().is_some_and(|pos| pos == "動詞")
                        && token.features.get(5).is_some_and(|f| f.contains("連用形")) =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(KosoMatcher)),
        TokenMatcher::Custom(Arc::new(SureMatcher)),
    ]
}

// Pattern: 並み (on par with, as good as)
// Structures: Noun + 並み + (だ/です/の/に)
pub fn nami() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match 並み as noun suffix
    #[derive(Debug)]
    struct NamiMatcher;
    impl super::Matcher for NamiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "並み"
                        && token.base_form == "並み"
                        && token.pos.first().is_some_and(|pos| pos == "名詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "接尾") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![super::noun(), TokenMatcher::Custom(Arc::new(NamiMatcher))]
}

// Pattern: に先駆けて (ahead of, in advance of)
// Structures: Noun + に + 先駆け + (て)
pub fn nisakigakete() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match 先駆ける verb in 連用形
    #[derive(Debug)]
    struct SakigakeMatcher;
    impl super::Matcher for SakigakeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.base_form == "先駆ける"
                        && token.pos.first().is_some_and(|pos| pos == "動詞")
                        && token.features.get(5).is_some_and(|form| form == "連用形") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match て as 接続助詞
    #[derive(Debug)]
    struct TeFormMatcher;
    impl super::Matcher for TeFormMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "て"
                        && token.pos.first().is_some_and(|pos| pos == "助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        super::noun(),
        surface("に"),
        TokenMatcher::Custom(Arc::new(SakigakeMatcher)),
        optional(TokenMatcher::Custom(Arc::new(TeFormMatcher))),
    ]
}

// Pattern: を機に (taking advantage of, on the occasion of)
// Structures: Verb[た] + の + を機に or Noun + を機に
pub fn wokini() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match を as 格助詞
    #[derive(Debug)]
    struct WoParticleMatcher;
    impl super::Matcher for WoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "を"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match 機 as noun
    #[derive(Debug)]
    struct KiNounMatcher;
    impl super::Matcher for KiNounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "機"
                        && token.base_form == "機"
                        && token.pos.first().is_some_and(|pos| pos == "名詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "一般") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match に as 格助詞
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "に"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    // Match の as nominalizer (名詞/非自立)
    #[derive(Debug)]
    struct NoNominalizerMatcher;
    impl super::Matcher for NoNominalizerMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token)
                    if token.surface == "の"
                        && token.base_form == "の"
                        && token.pos.first().is_some_and(|pos| pos == "名詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "非自立") =>
                {
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),                                                          // Noun or (Verb + た)
        optional(TokenMatcher::Custom(Arc::new(NoNominalizerMatcher))), // Optional の for verb nominalization
        TokenMatcher::Custom(Arc::new(WoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(KiNounMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
    ]
}
