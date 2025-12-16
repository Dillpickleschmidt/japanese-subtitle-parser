use crate::pattern_matcher::TokenMatcher;
use std::sync::Arc;

use super::Matcher;

// Pattern: という (called/named)
// Structures: Noun (A) + という + Noun (B)
pub fn toiu() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct ToiuMatcher;
    impl Matcher for ToiuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "という"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(ToiuMatcher)),
        super::noun_matcher(),
    ]
}

// Pattern: という1 (every single A)
// Structures: Noun (A) + という + Noun (A) - same noun repeated
// Meaning: "every single (A)" / "(A) of all (A)s"
// という1 is UNDETECTABLE - requires cross-token validation not supported by current architecture
// See tests/n1_patterns.rs toiu1 comment for full explanation
// Users should rely on basic という pattern and manually check if nouns are identical
pub fn toiu1() -> Vec<TokenMatcher> {
    vec![]  // Intentionally empty - pattern cannot be detected
}

// Match まま (unchanged state noun)
fn mama_matcher() -> TokenMatcher {
    #[derive(Debug)]
    struct MamaMatcher;
    impl Matcher for MamaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "まま"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }
    TokenMatcher::Custom(Arc::new(MamaMatcher))
}

// Match に particle (optional after まま)
fn ni_particle_matcher() -> TokenMatcher {
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }
    TokenMatcher::Custom(Arc::new(NiParticleMatcher))
}

// Match any verb form that can precede た
fn verb_before_ta_matcher() -> TokenMatcher {
    #[derive(Debug)]
    struct VerbBeforeTaMatcher;
    impl Matcher for VerbBeforeTaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }
    TokenMatcher::Custom(Arc::new(VerbBeforeTaMatcher))
}

// Match た (past auxiliary) - strict surface match only
fn ta_auxiliary_matcher() -> TokenMatcher {
    #[derive(Debug)]
    struct TaAuxiliaryMatcher;
    impl Matcher for TaAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "た"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }
    TokenMatcher::Custom(Arc::new(TaAuxiliaryMatcher))
}

// Pattern: まま(に) - Verb[た] + まま
// Structure: Verb stem + た + まま (WITHOUT に)
pub fn mama_ni() -> Vec<TokenMatcher> {
    vec![
        verb_before_ta_matcher(),
        ta_auxiliary_matcher(),
        mama_matcher(),
    ]
}

// Pattern: まま(に) - Verb[た] + まま + に
// Structure: Verb stem + た + まま + に (WITH required に)
pub fn mama_ni_with_ni() -> Vec<TokenMatcher> {
    vec![
        verb_before_ta_matcher(),
        ta_auxiliary_matcher(),
        mama_matcher(),
        ni_particle_matcher(),
    ]
}

// Pattern: まま(に) - Verb[ない] + まま (+ に)
// Structure: Verb negative + ない + まま (+ に)
pub fn mama_ni_nai() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct VerbNegativeFormMatcher;
    impl Matcher for VerbNegativeFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然形")
        }
    }

    #[derive(Debug)]
    struct NaiAuxiliaryMatcher;
    impl Matcher for NaiAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbNegativeFormMatcher)),
        TokenMatcher::Custom(Arc::new(NaiAuxiliaryMatcher)),
        mama_matcher(),
        TokenMatcher::Optional(Box::new(ni_particle_matcher())),
    ]
}

// Pattern: まま(に) - い-Adjective + まま (+ に)
// Structure: い-Adjective + まま (+ に)
pub fn mama_ni_i_adj() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct IAdjMatcher;
    impl Matcher for IAdjMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "形容詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(IAdjMatcher)),
        mama_matcher(),
        TokenMatcher::Optional(Box::new(ni_particle_matcher())),
    ]
}

// Pattern: まま(に) - な-Adjective + な + まま (+ に)
// Structure: な-Adjective stem + な (だ auxiliary) + まま (+ に)
pub fn mama_ni_na_adj() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct NaAdjStemMatcher;
    impl Matcher for NaAdjStemMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹")
        }
    }

    #[derive(Debug)]
    struct NaCopulaMatcher;
    impl Matcher for NaCopulaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "な"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NaAdjStemMatcher)),
        TokenMatcher::Custom(Arc::new(NaCopulaMatcher)),
        mama_matcher(),
        TokenMatcher::Optional(Box::new(ni_particle_matcher())),
    ]
}

// Pattern: まま(に) - Noun + の + まま (+ に)
// Structure: Noun + の (connective particle) + まま (+ に)
pub fn mama_ni_noun() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct NoParticleMatcher;
    impl Matcher for NoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(NoParticleMatcher)),
        mama_matcher(),
        TokenMatcher::Optional(Box::new(ni_particle_matcher())),
    ]
}

// Pattern: まま(に)1 (as one wishes, on a whim)
// Structure: Verb[dictionary form] + (が) + まま(に)
// Meaning: "to do as one wishes/desires", "on a whim"
// Different from basic まま(に) which uses past/negative forms and means "left unchanged"
pub fn mama_ni_1() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct DictionaryFormVerbMatcher;
    impl Matcher for DictionaryFormVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "基本形")
        }
    }

    #[derive(Debug)]
    struct GaParticleMatcher;
    impl Matcher for GaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "が"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(DictionaryFormVerbMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(GaParticleMatcher)))),
        mama_matcher(),
        TokenMatcher::Optional(Box::new(ni_particle_matcher())),
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match 至る verb in 基本形
    #[derive(Debug)]
    struct ItaruMatcher;
    impl super::Matcher for ItaruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "至る"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "基本形")
        }
    }

    // Match まで as adverbial particle
    #[derive(Debug)]
    struct MadeParticleMatcher;
    impl super::Matcher for MadeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "まで"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞")
        }
    }

    // Match の as relativizing particle (連体化)
    #[derive(Debug)]
    struct NoRentaikaMatcher;
    impl super::Matcher for NoRentaikaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
        }
    }

    vec![
        // Required: Noun B + に
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        // Required: 至る + まで
        TokenMatcher::Custom(Arc::new(ItaruMatcher)),
        TokenMatcher::Custom(Arc::new(MadeParticleMatcher)),
        // Optional: の + Noun C
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NoRentaikaMatcher)))),
        TokenMatcher::Optional(Box::new(super::noun_matcher())),
    ]
}

// Pattern: たところで (even if, even though)
// Matches: Verb (連用形/連用タ接続) + た/だ (past auxiliary) + ところ (名詞/非自立) + で (格助詞)
pub fn tatokorode() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct TokoroMatcher;
    impl Matcher for TokoroMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ところ"
                && token.base_form == "ところ"
                && token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "非自立")
        }
    }

    #[derive(Debug)]
    struct DeParticleMatcher;
    impl Matcher for DeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "で"
                && token.base_form == "で"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
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
    impl Matcher for NoRentaikaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "連体化")
        }
    }

    // Matcher for が (接続助詞 - conjunctive particle for classical usage)
    #[derive(Debug)]
    struct GaSetsuzokuMatcher;
    impl Matcher for GaSetsuzokuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "が"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
        }
    }

    // Matcher for ごとし/ごとく/ごとき (all forms of classical auxiliary)
    #[derive(Debug)]
    struct GotoshiMatcher;
    impl Matcher for GotoshiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "ごとし"
                && token.pos.first().is_some_and(|p| p == "助動詞")
                && (token.surface == "ごとし"
                    || token.surface == "ごとく"
                    || token.surface == "ごとき")
        }
    }

    // Pattern: (Any) + (の or が or nothing) + ごとし/ごとく/ごとき
    // - の(連体化) for most cases
    // - が(接続助詞) for classical verb/auxiliary usage
    // - nothing (direct) for ごとき after nouns (体言接続)
    vec![
        TokenMatcher::Any,
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NoRentaikaMatcher)))),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(GaSetsuzokuMatcher)))),
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match 足る verb in 基本形
    #[derive(Debug)]
    struct TaruMatcher;
    impl super::Matcher for TaruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "足る"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "基本形")
        }
    }

    // Match verb in 基本形 or noun
    #[derive(Debug)]
    struct VerbOrNounMatcher;
    impl super::Matcher for VerbOrNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            let is_verb = token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "基本形");
            let is_noun = token.pos.first().is_some_and(|pos| pos == "名詞");
            is_verb || is_noun
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbOrNounMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(TaruMatcher)),
        super::noun_matcher(),
    ]
}

// Pattern: 極まりない・極まる (extremely)
// Structures:
// - な-Adj + (な) + (こと) + 極まりない
// - な-Adj + 極まる
// - い-Adj + こと + 極まりない
pub fn kiwamarinai_u30fb_kiwamaru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match な-Adjective (名詞/形容動詞語幹)
    #[derive(Debug)]
    struct NaAdjectiveMatcher;
    impl Matcher for NaAdjectiveMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.get(0).is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "形容動詞語幹")
        }
    }

    // Match い-Adjective (形容詞/自立)
    #[derive(Debug)]
    struct IAdjectiveMatcher;
    impl Matcher for IAdjectiveMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.get(0).is_some_and(|p| p == "形容詞")
                && token.pos.get(1).is_some_and(|p| p == "自立")
        }
    }

    // Match な particle from だ (助動詞, 特殊・ダ, 体言接続)
    #[derive(Debug)]
    struct NaParticleMatcher;
    impl Matcher for NaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "な"
                && token.pos.get(0).is_some_and(|p| p == "助動詞")
                && token.features.get(4).is_some_and(|f| f == "特殊・ダ")
                && token.features.get(5).is_some_and(|f| f == "体言接続")
        }
    }

    // Match こと (名詞/非自立/一般)
    #[derive(Debug)]
    struct KotoMatcher;
    impl Matcher for KotoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こと"
                && token.pos.get(0).is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "非自立")
        }
    }

    // Match 極まりない (形容詞/自立)
    #[derive(Debug)]
    struct KiwamarinaiMatcher;
    impl Matcher for KiwamarinaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "極まりない"
                && token.pos.get(0).is_some_and(|p| p == "形容詞")
        }
    }

    // Match 極まる (動詞/自立)
    #[derive(Debug)]
    struct KiwamaruMatcher;
    impl Matcher for KiwamaruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "極まる"
                && token.pos.get(0).is_some_and(|p| p == "動詞")
        }
    }

    // Match either な-Adj or い-Adj
    #[derive(Debug)]
    struct AdjectiveMatcher;
    impl Matcher for AdjectiveMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // な-Adjective
            (token.pos.get(0).is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "形容動詞語幹"))
            ||
            // い-Adjective
            (token.pos.get(0).is_some_and(|p| p == "形容詞")
                && token.pos.get(1).is_some_and(|p| p == "自立"))
        }
    }

    // Match 極まりない or 極まる
    #[derive(Debug)]
    struct KiwamaMatcher;
    impl Matcher for KiwamaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.base_form == "極まりない" && token.pos.get(0).is_some_and(|p| p == "形容詞"))
                || (token.base_form == "極まる" && token.pos.get(0).is_some_and(|p| p == "動詞"))
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(AdjectiveMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NaParticleMatcher)))),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(KotoMatcher)))),
        TokenMatcher::Custom(Arc::new(KiwamaMatcher)),
    ]
}

// Pattern: といえども (even if, although)
// Structures: Verb/Noun/Adj + と + いえ + ども
pub fn toiedomo() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match と (助詞/格助詞/引用)
    #[derive(Debug)]
    struct ToQuoteMatcher;
    impl Matcher for ToQuoteMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "と"
                && token.pos.get(0).is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
                && token.pos.get(2).is_some_and(|p| p == "引用")
        }
    }

    // Match いえ (動詞, 仮定形, base=いう)
    #[derive(Debug)]
    struct IeMatcher;
    impl Matcher for IeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "いえ"
                && token.base_form == "いう"
                && token.pos.get(0).is_some_and(|p| p == "動詞")
                && token.features.get(5).is_some_and(|f| f == "仮定形")
        }
    }

    // Match ども (助詞/接続助詞)
    #[derive(Debug)]
    struct DomoMatcher;
    impl Matcher for DomoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ども"
                && token.pos.get(0).is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
        }
    }

    vec![
        TokenMatcher::Any,
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
    impl Matcher for WomotteMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "をもって"
                && token.base_form == "をもって"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
                && token.pos.get(2).is_some_and(|p| p == "連語")
        }
    }
    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(WomotteMatcher)),
    ]
}

// Pattern: を以て (by means of, with) - split tokenization
// Matches: Noun + を + もつ (verb) + て
pub fn womotte_split() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct WoParticleMatcher;
    impl Matcher for WoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "を"
                && token.base_form == "を"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    #[derive(Debug)]
    struct MotsuVerbMatcher;
    impl Matcher for MotsuVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "もつ"
                && token.pos.first().is_some_and(|p| p == "動詞")
        }
    }

    #[derive(Debug)]
    struct TeParticleMatcher;
    impl Matcher for TeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.base_form == "て"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(WoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(MotsuVerbMatcher)),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
    ]
}

// Pattern: きらいがある (tends to, has a tendency to)
// Structures: Verb/Noun + きらい + が + ある/あります
pub fn kiraigaaru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match きらい as 名詞/非自立
    #[derive(Debug)]
    struct KiraiMatcher;
    impl Matcher for KiraiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "きらい"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Match が as 助詞/格助詞
    #[derive(Debug)]
    struct GaKakuMatcher;
    impl Matcher for GaKakuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "が"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match ある (基本形) or あり (連用形)
    #[derive(Debug)]
    struct AruAriMatcher;
    impl Matcher for AruAriMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "ある"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && (token.surface == "ある" || token.surface == "あり")
        }
    }

    // Match ます
    #[derive(Debug)]
    struct MasuMatcher;
    impl Matcher for MasuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ます" && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        TokenMatcher::Any,
        TokenMatcher::Custom(Arc::new(KiraiMatcher)),
        TokenMatcher::Custom(Arc::new(GaKakuMatcher)),
        TokenMatcher::Custom(Arc::new(AruAriMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(MasuMatcher)))),
    ]
}

// Pattern: ならまだしも (if A, that's fine, but B)
// Matches: なら (助動詞/仮定形) + まだしも (副詞)
// Note: Pattern range will include preceding token automatically
pub fn naramadashimo() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct NaraMatcher;
    impl Matcher for NaraMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "なら"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|p| p == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "仮定形")
        }
    }

    #[derive(Debug)]
    struct MadashimoMatcher;
    impl Matcher for MadashimoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "まだしも"
                && token.base_form == "まだしも"
                && token.pos.first().is_some_and(|p| p == "副詞")
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
    use std::sync::Arc;

    // Match まで as 助詞/副助詞
    #[derive(Debug)]
    struct MadeMatcher;
    impl Matcher for MadeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "まで"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞")
        }
    }

    // Match も as 助詞/係助詞
    #[derive(Debug)]
    struct MoKakariMatcher;
    impl Matcher for MoKakariMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Match ない (形容詞, 基本形), なく (形容詞, 連用テ接続), OR あり (動詞)
    #[derive(Debug)]
    struct NaiNakuAriMatcher;
    impl Matcher for NaiNakuAriMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
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
        }
    }

    // Match て (for なくて form)
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl Matcher for TeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て" && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    // Match ませ (for polite form)
    #[derive(Debug)]
    struct MaseMatcher;
    impl Matcher for MaseMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ませ"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Match ん (for polite form)
    #[derive(Debug)]
    struct NMatcher;
    impl Matcher for NMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ん" && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        TokenMatcher::verb_with_form("基本形"),
        TokenMatcher::Custom(Arc::new(MadeMatcher)),
        TokenMatcher::Custom(Arc::new(MoKakariMatcher)),
        TokenMatcher::Custom(Arc::new(NaiNakuAriMatcher)),
        // Optional て, ませ, ん to handle various endings
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(TeParticleMatcher)))),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(MaseMatcher)))),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NMatcher)))),
    ]
}

// Pattern: ともなると・にもなると (when it comes to, once)
// Structures: Noun/Verb + と/に + (も) + なる(と/ば)
pub fn tomonaruto_u30fb_nimonaruto() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match と or に as quotation/general case particle
    #[derive(Debug)]
    struct ToNiMatcher;
    impl Matcher for ToNiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "と" || token.surface == "に")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match optional も (係助詞)
    #[derive(Debug)]
    struct MoKakariMatcher;
    impl Matcher for MoKakariMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Match なる (基本形) or なれ (仮定形)
    #[derive(Debug)]
    struct NaruNareMatcher;
    impl Matcher for NaruNareMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "なる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && ((token.surface == "なる" && token.features.get(5).is_some_and(|f| f == "基本形"))
                    || (token.surface == "なれ" && token.features.get(5).is_some_and(|f| f == "仮定形")))
        }
    }

    // Match と (接続助詞) or ば (接続助詞)
    #[derive(Debug)]
    struct ToBaMatcher;
    impl Matcher for ToBaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "と" || token.surface == "ば")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    vec![
        TokenMatcher::Any,
        TokenMatcher::Custom(Arc::new(ToNiMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(MoKakariMatcher)))),
        TokenMatcher::Custom(Arc::new(NaruNareMatcher)),
        TokenMatcher::Custom(Arc::new(ToBaMatcher)),
    ]
}

// Pattern: をいいことに (take advantage of)
// Structures: (の/なの) + を + いい + こと + に + (して)
pub fn woiikotoni() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match の as nominalizer (名詞/非自立/一般)
    #[derive(Debug)]
    struct NoNominalizerMatcher;
    impl Matcher for NoNominalizerMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Match な from だ (助動詞, 特殊・ダ, 体言接続)
    #[derive(Debug)]
    struct NaFromDaMatcher;
    impl Matcher for NaFromDaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "な"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Match を (格助詞)
    #[derive(Debug)]
    struct WoMatcher;
    impl Matcher for WoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "を"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match いい (形容詞, 基本形)
    #[derive(Debug)]
    struct IiMatcher;
    impl Matcher for IiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "いい"
                && token.base_form == "いい"
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "基本形")
        }
    }

    // Match こと (名詞/非自立/一般)
    #[derive(Debug)]
    struct KotoMatcher;
    impl Matcher for KotoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Match に (格助詞)
    #[derive(Debug)]
    struct NiMatcher;
    impl Matcher for NiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match し from する (動詞, 連用形)
    #[derive(Debug)]
    struct ShiMatcher;
    impl Matcher for ShiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "し"
                && token.base_form == "する"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形")
        }
    }

    // Match て (接続助詞)
    #[derive(Debug)]
    struct TeMatcher;
    impl Matcher for TeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    vec![
        TokenMatcher::Any, // Preceding verb/noun/adjective
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NaFromDaMatcher)))),
        TokenMatcher::Custom(Arc::new(NoNominalizerMatcher)),
        TokenMatcher::Custom(Arc::new(WoMatcher)),
        TokenMatcher::Custom(Arc::new(IiMatcher)),
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
        TokenMatcher::Custom(Arc::new(NiMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(ShiMatcher)))),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(TeMatcher)))),
    ]
}

// Pattern: 如何 (いかん - depending on)
// Structures: Noun + (の) + いかん + で/だ/によって/である
pub fn ika_2() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match の particle (助詞/連体化)
    #[derive(Debug)]
    struct NoParticleMatcher;
    impl Matcher for NoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
        }
    }

    // Match いかん (名詞/一般 or 名詞/接尾)
    #[derive(Debug)]
    struct IkanMatcher;
    impl Matcher for IkanMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "いかん"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "一般")
                    || token.pos.get(1).is_some_and(|pos| pos == "接尾"))
        }
    }

    // Match で (助詞/格助詞/一般 OR 助動詞/特殊・ダ/連用形)
    #[derive(Debug)]
    struct DeMatcher;
    impl Matcher for DeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "で"
                && ((token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞"))
                    || (token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.base_form == "だ"))
        }
    }

    // Match だ (助動詞/特殊・ダ/基本形)
    #[derive(Debug)]
    struct DaMatcher;
    impl Matcher for DaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "だ"
                && token.features.get(5).is_some_and(|f| f == "基本形")
        }
    }

    // Match によって (助詞/格助詞/連語)
    #[derive(Debug)]
    struct NiyotteMatcher;
    impl Matcher for NiyotteMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "によって"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "連語")
        }
    }

    // Match ある auxiliary (助動詞/五段・ラ行アル/基本形)
    #[derive(Debug)]
    struct AruAuxMatcher;
    impl Matcher for AruAuxMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ある"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "ある"
        }
    }

    // Match は particle (助詞/係助詞) - optional after で or によって
    #[derive(Debug)]
    struct HaParticleMatcher;
    impl Matcher for HaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    vec![
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NoParticleMatcher,
        )))),
        TokenMatcher::Custom(Arc::new(IkanMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(DeMatcher)))),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(DaMatcher)))),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NiyotteMatcher,
        )))),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(AruAuxMatcher)))),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            HaParticleMatcher,
        )))),
    ]
}

// Pattern: ～るまでだ (merely, simply, one can only but)
// Structures: Verb[る] + まで + (の + こと)? + だ/です
pub fn uff5e_rumadeda() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match verb in dictionary form (基本形)
    #[derive(Debug)]
    struct DictionaryFormVerbMatcher;
    impl Matcher for DictionaryFormVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|form| form == "基本形")
        }
    }

    // Match まで (助詞/副助詞)
    #[derive(Debug)]
    struct MadeParticleMatcher;
    impl Matcher for MadeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "まで"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞")
        }
    }

    // Match の (助詞/連体化)
    #[derive(Debug)]
    struct NoRentaikaMatcher;
    impl Matcher for NoRentaikaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
        }
    }

    // Match こと (名詞/非自立/一般)
    #[derive(Debug)]
    struct KotoNounMatcher;
    impl Matcher for KotoNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Match だ (助動詞, 特殊・ダ, 基本形) or です (助動詞, 特殊・デス, 基本形)
    #[derive(Debug)]
    struct DaDesuAuxMatcher;
    impl Matcher for DaDesuAuxMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|form| form == "基本形")
                && (token.base_form == "だ" || token.base_form == "です")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(DictionaryFormVerbMatcher)),
        TokenMatcher::Custom(Arc::new(MadeParticleMatcher)),
        // Optional: の + こと sequence
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NoRentaikaMatcher,
        )))),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            KotoNounMatcher,
        )))),
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
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match あっ (動詞, base=ある, 連用タ接続)
    // Important: This is ある (to be), NOT 遭う (to meet/encounter)
    #[derive(Debug)]
    struct AtteMatcher;
    impl Matcher for AtteMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "あっ"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.base_form == "ある"
                && token.features.get(5).is_some_and(|form| form == "連用タ接続")
        }
    }

    // Match て particle (助詞/接続助詞)
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl Matcher for TeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Match も particle (助詞/係助詞)
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(AtteMatcher)),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            MoParticleMatcher,
        )))),
    ]
}

// Pattern: を余儀なくされる (to be forced to)
// Structure: Noun + を + よぎなく + さ + れ + た/ます
pub fn woyoginakusareru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match よぎなく (形容詞/自立, base=よぎない, 連用テ接続)
    #[derive(Debug)]
    struct YoginakuMatcher;
    impl Matcher for YoginakuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "よぎなく"
                && token.base_form == "よぎない"
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
        }
    }

    // Match さ from する (動詞/自立, base=する, サ変・スル/未然レル接続)
    #[derive(Debug)]
    struct SasuruMatcher;
    impl Matcher for SasuruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "さ"
                && token.base_form == "する"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(4).is_some_and(|f| f.contains("サ変"))
                && token.features.get(5).is_some_and(|f| f == "未然レル接続")
        }
    }

    // Match れ passive auxiliary (動詞/接尾, base=れる, 一段/連用形)
    #[derive(Debug)]
    struct RePassiveMatcher;
    impl Matcher for RePassiveMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "れ"
                && token.base_form == "れる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
        }
    }

    // Match た or ます (助動詞)
    #[derive(Debug)]
    struct TaMasuMatcher;
    impl Matcher for TaMasuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "た" || token.surface == "ます")
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Match を (格助詞)
    #[derive(Debug)]
    struct WoParticleMatcher;
    impl Matcher for WoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "を"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(WoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(YoginakuMatcher)),
        TokenMatcher::Custom(Arc::new(SasuruMatcher)),
        TokenMatcher::Custom(Arc::new(RePassiveMatcher)),
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
    impl Matcher for ToQuotationMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "引用")
        }
    }

    // Match は (topic particle)
    #[derive(Debug)]
    struct HaTopicMatcher;
    impl Matcher for HaTopicMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Match any preceding token (verb/noun/adjective)
    // Optional など can appear before とは but we'll keep the matcher simple
    vec![
        TokenMatcher::Any,
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
    impl Matcher for JaOrDeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
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
        }
    }

    // Match は (助詞/係助詞) - optional, only after で
    #[derive(Debug)]
    struct HaTopicParticleMatcher;
    impl Matcher for HaTopicParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "係助詞")
        }
    }

    // Match ある (助動詞, 五段・ラ行アル, 基本形)
    #[derive(Debug)]
    struct AruAuxiliaryMatcher;
    impl Matcher for AruAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ある"
                && token.pos.first().is_some_and(|p| p == "助動詞")
                && token.base_form == "ある"
        }
    }

    // Match まい (助動詞, 不変化型, 基本形)
    #[derive(Debug)]
    struct MaiAuxiliaryMatcher;
    impl Matcher for MaiAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "まい"
                && token.pos.first().is_some_and(|p| p == "助動詞")
                && token.base_form == "まい"
        }
    }

    // Match し (助詞/接続助詞)
    #[derive(Debug)]
    struct ShiConjunctionMatcher;
    impl Matcher for ShiConjunctionMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "し"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
        }
    }

    // Pattern: (じゃ OR で) + optional は + ある + まい + し
    vec![
        TokenMatcher::Any, // Noun/ん/わけ - we match any preceding token
        TokenMatcher::Custom(Arc::new(JaOrDeMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            HaTopicParticleMatcher,
        )))),
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
    impl Matcher for TeOrSorekaraMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            if token.surface == "て" {
                token.pos.first().is_some_and(|p| p == "助詞")
                    && token.pos.get(1).is_some_and(|p| p == "接続助詞")
            } else if token.surface == "それから" {
                token.pos.first().is_some_and(|p| p == "接続詞")
            } else {
                false
            }
        }
    }

    // Match から (助詞/格助詞/一般) - only after て, not after それから
    #[derive(Debug)]
    struct KaraParticleMatcher;
    impl Matcher for KaraParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "から"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    // Match という (助詞/格助詞/連語)
    #[derive(Debug)]
    struct ToiuMatcher;
    impl Matcher for ToiuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "という"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
                && token.pos.get(2).is_some_and(|p| p == "連語")
        }
    }

    // Match もの (名詞/非自立/一般)
    #[derive(Debug)]
    struct MonoMatcher;
    impl Matcher for MonoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "もの"
                && token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "非自立")
        }
    }

    // Pattern: (て OR それから) + optional から + という + もの
    // When it's て, から is required; when it's それから, から is already included
    vec![
        TokenMatcher::Custom(Arc::new(TeOrSorekaraMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            KaraParticleMatcher,
        )))),
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
    impl Matcher for KatawaraMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "かたわら"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副詞可能")
        }
    }

    // Match verb in dictionary form (基本形) OR noun
    #[derive(Debug)]
    struct VerbOrNounMatcher;
    impl Matcher for VerbOrNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match verb in dictionary form
            if token.pos.first().is_some_and(|pos| pos == "動詞") {
                return token
                    .features
                    .get(5)
                    .is_some_and(|form| form == "基本形");
            }
            // Or match any noun
            token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    // Match の particle (助詞/連体化)
    #[derive(Debug)]
    struct NoParticleMatcher;
    impl Matcher for NoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbOrNounMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NoParticleMatcher,
        )))),
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
    impl Matcher for WoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "を"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match 皮切り (名詞/一般)
    #[derive(Debug)]
    struct KawakiriMatcher;
    impl Matcher for KawakiriMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "皮切り"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
        }
    }

    // Match に particle OR として (助詞/格助詞)
    #[derive(Debug)]
    struct NiOrToshiteMatcher;
    impl Matcher for NiOrToshiteMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
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
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match いたっ (動詞/自立, base=いたる, 連用タ接続)
    #[derive(Debug)]
    struct ItattaMatcher;
    impl Matcher for ItattaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "いたっ"
                && token.base_form == "いたる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    // Match て (接続助詞)
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl Matcher for TeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Match は (係助詞)
    #[derive(Debug)]
    struct WaParticleMatcher;
    impl Matcher for WaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    vec![
        super::noun_matcher(),
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

    // Match dictionary form verb (基本形)
    #[derive(Debug)]
    struct DictionaryFormVerbMatcher;
    impl super::Matcher for DictionaryFormVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token
                    .features
                    .get(5)
                    .is_some_and(|form| form == "基本形")
        }
    }

    // Match なり as conjunction particle (助詞/接続助詞)
    #[derive(Debug)]
    struct NariParticleMatcher;
    impl super::Matcher for NariParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "なり"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(DictionaryFormVerbMatcher)),
        TokenMatcher::Custom(Arc::new(NariParticleMatcher)),
    ]
}

// Pattern: ともなく・ともなしに (absentmindedly, without paying attention)
// Structures: Verb[る] + ともなく, Verb[る] + ともなしに
pub fn tomonaku_u30fb_tomonashini() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match dictionary form verb (基本形)
    #[derive(Debug)]
    struct DictionaryFormVerbMatcher;
    impl super::Matcher for DictionaryFormVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token
                    .features
                    .get(5)
                    .is_some_and(|form| form == "基本形")
        }
    }

    // Match とも (助詞/接続助詞) OR と (助詞/格助詞/引用)
    #[derive(Debug)]
    struct ToOrTomoMatcher;
    impl super::Matcher for ToOrTomoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "とも"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞"))
                || (token.surface == "と"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                    && token.pos.get(2).is_some_and(|pos| pos == "引用"))
        }
    }

    // Match も (助詞/係助詞) - optional, only for ともなしに variant
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl super::Matcher for MoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Match なく (形容詞/自立, 連用テ接続, base=ない) OR なし (形容詞/自立, 文語基本形, base=ない)
    #[derive(Debug)]
    struct NakuOrNashiMatcher;
    impl super::Matcher for NakuOrNashiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "なく" || token.surface == "なし")
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立")
        }
    }

    // Match に particle (助詞/格助詞/一般) - optional, only for ともなしに
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Pattern 1: Verb + とも + なく (3 tokens)
    // Pattern 2: Verb + と + も + なし + に (5 tokens)
    // Use optional matchers for も and に to handle both variants
    vec![
        TokenMatcher::Custom(Arc::new(DictionaryFormVerbMatcher)),
        TokenMatcher::Custom(Arc::new(ToOrTomoMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            MoParticleMatcher,
        )))),
        TokenMatcher::Custom(Arc::new(NakuOrNashiMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NiParticleMatcher,
        )))),
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
    impl Matcher for MamireCompoundMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.base_form.ends_with("まみれ")
                && token.base_form != "まみれ"
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
    impl Matcher for MamireSuffixMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "まみれ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
        }
    }

    vec![super::noun_matcher(), TokenMatcher::Custom(Arc::new(MamireSuffixMatcher))]
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
    impl Matcher for VolitionalAuxMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "う" || token.surface == "よう")
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Match が as conjunction particle
    #[derive(Debug)]
    struct GaConjunctionMatcher;
    impl Matcher for GaConjunctionMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "が"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Match まい auxiliary verb
    #[derive(Debug)]
    struct MaiAuxiliaryMatcher;
    impl Matcher for MaiAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "まい" && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        // First part: Verb(未然ウ接続) + う/よう + が
        TokenMatcher::Any, // Verb in 未然ウ接続 form
        TokenMatcher::Custom(Arc::new(VolitionalAuxMatcher)),
        TokenMatcher::Custom(Arc::new(GaConjunctionMatcher)),
        // Wildcard to allow different verb or same verb
        TokenMatcher::Wildcard {
            min: 0,
            max: 3,
            stop_conditions: vec![],
        },
        // Second part: Verb(基本形) + まい + が
        TokenMatcher::Any, // Verb in dictionary form
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match 名詞/数 or 名詞/接尾/助数詞
            if let Some(pos1) = token.pos.first() {
                if pos1 == "名詞" {
                    if let Some(pos2) = token.pos.get(1) {
                        return pos2 == "数" || (pos2 == "接尾" && token.pos.get(2).is_some_and(|p| p == "助数詞"));
                    }
                }
            }
            false
        }
    }

    // Match から particle (格助詞)
    #[derive(Debug)]
    struct KaraParticleMatcher;
    impl super::Matcher for KaraParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "から"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    // Match する verb (for からする/からします)
    #[derive(Debug)]
    struct SuruVerbMatcher;
    impl super::Matcher for SuruVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "する"
                && token.pos.first().is_some_and(|p| p == "動詞")
        }
    }

    vec![
        // Match 1+ number/counter tokens
        TokenMatcher::Custom(Arc::new(NumberOrCounterMatcher)),
        TokenMatcher::Wildcard {
            min: 0,
            max: 5,
            stop_conditions: vec![],
        },
        // Match から
        TokenMatcher::Custom(Arc::new(KaraParticleMatcher)),
        // Match する verb
        TokenMatcher::Custom(Arc::new(SuruVerbMatcher)),
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match 名詞/数 or 名詞/接尾/助数詞
            if let Some(pos1) = token.pos.first() {
                if pos1 == "名詞" {
                    if let Some(pos2) = token.pos.get(1) {
                        return pos2 == "数" || (pos2 == "接尾" && token.pos.get(2).is_some_and(|p| p == "助数詞"));
                    }
                }
            }
            false
        }
    }

    // Match から particle (格助詞)
    #[derive(Debug)]
    struct KaraParticleMatcher;
    impl super::Matcher for KaraParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "から"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    // Match の particle (for からの)
    #[derive(Debug)]
    struct NoParticleMatcher;
    impl super::Matcher for NoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "連体化")
        }
    }

    vec![
        // Match 1+ number/counter tokens
        TokenMatcher::Custom(Arc::new(NumberOrCounterMatcher)),
        TokenMatcher::Wildcard {
            min: 0,
            max: 5,
            stop_conditions: vec![],
        },
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

    // Match number/counter or regular noun
    #[derive(Debug)]
    struct NumberCounterOrNounMatcher;
    impl super::Matcher for NumberCounterOrNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match 名詞 (any noun type)
            token.pos.first().is_some_and(|p| p == "名詞")
        }
    }

    // Match に particle (格助詞)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    // Match し (する in 連用形)
    #[derive(Debug)]
    struct ShiVerbMatcher;
    impl super::Matcher for ShiVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "し"
                && token.base_form == "する"
                && token.pos.first().is_some_and(|p| p == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形")
        }
    }

    // Match て particle (接続助詞)
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl super::Matcher for TeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
        }
    }

    vec![
        // Match number/counter or noun - use specific noun types to avoid matching across particles
        TokenMatcher::Custom(Arc::new(NumberCounterOrNounMatcher)),
        // Optional: allow one more noun token (for multi-token numbers like ３０)
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NumberCounterOrNounMatcher)))),
        // Optional: allow one more noun token (for counters like 歳, or suffixes like 目)
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NumberCounterOrNounMatcher)))),
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "もの"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Match を particle
    #[derive(Debug)]
    struct WoParticleMatcher;
    impl super::Matcher for WoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "を"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        TokenMatcher::Any, // Could be verb, adjective, or auxiliary (な)
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
    impl Matcher for DeAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "だ"
        }
    }

    // Match あれ (auxiliary verb ある in 命令ｅ form)
    #[derive(Debug)]
    struct AreImperativeMatcher;
    impl Matcher for AreImperativeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "あれ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "ある"
                && token.features.get(5).is_some_and(|f| f.contains("命令"))
        }
    }

    vec![
        TokenMatcher::Any, // Noun, な-Adjective stem, or question word
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
    impl Matcher for OkuVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "おく"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|form| form == "連用タ接続")
        }
    }

    // Match て as conjunctive particle
    #[derive(Debug)]
    struct TeConjunctiveMatcher;
    impl Matcher for TeConjunctiveMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Match ほか as noun
    #[derive(Debug)]
    struct HokaMatcher;
    impl Matcher for HokaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ほか" && token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    // Match ない (either 助動詞 or 形容詞)
    #[derive(Debug)]
    struct NaiNegativeMatcher;
    impl Matcher for NaiNegativeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ない"
                && (token.pos.first().is_some_and(|pos| pos == "助動詞")
                    || token.pos.first().is_some_and(|pos| pos == "形容詞"))
        }
    }

    // Match は particle (optional)
    #[derive(Debug)]
    struct WaParticleMatcher;
    impl Matcher for WaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は" && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Surface("を"),
        TokenMatcher::Custom(Arc::new(OkuVerbMatcher)),
        TokenMatcher::Custom(Arc::new(TeConjunctiveMatcher)),
        TokenMatcher::Custom(Arc::new(HokaMatcher)),
        TokenMatcher::Surface("に"),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(WaParticleMatcher)))),
        TokenMatcher::Wildcard {
            min: 0,
            max: 10,
            stop_conditions: vec![],
        },
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
    impl Matcher for WomotteTimeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "をもって" || token.surface == "をもちまして")
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
                && token.pos.get(2).is_some_and(|p| p == "連語")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(WomotteTimeMatcher)),
    ]
}

// Pattern: とはいえ (although, be that as it may)
// Structures: Verb/Adj/Noun + と + は + いえ
pub fn tohaie() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match と as quotation particle (格助詞/引用)
    #[derive(Debug)]
    struct ToQuoteMatcher;
    impl super::Matcher for ToQuoteMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "と"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    // Match は as topic particle (係助詞)
    #[derive(Debug)]
    struct HaTopicMatcher;
    impl super::Matcher for HaTopicMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "係助詞")
        }
    }

    // Match いえ as verb いう in imperative form (命令ｅ)
    #[derive(Debug)]
    struct IeMatcher;
    impl super::Matcher for IeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "いえ"
                && token.base_form == "いう"
                && token.pos.first().is_some_and(|p| p == "動詞")
                && token
                    .features
                    .get(5)
                    .is_some_and(|f| f.contains("命令"))
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ならでは"
                && token.base_form == "ならでは"
                && token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "接尾")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(NaradehaMatcher)),
    ]
}

// Pattern: すら
// Pattern: すら (even - extreme example)
// Structures: Noun + (Particle) + すら(も)
pub fn sura() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match すら as 係助詞
    #[derive(Debug)]
    struct SuraParticleMatcher;
    impl Matcher for SuraParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "すら"
                && token.base_form == "すら"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Match optional も after すら
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Match optional particle (に, で from だ, etc.)
    #[derive(Debug)]
    struct OptionalParticleMatcher;
    impl Matcher for OptionalParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
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
        }
    }

    vec![
        TokenMatcher::Any, // Noun
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            OptionalParticleMatcher,
        )))), // Optional particle
        TokenMatcher::Custom(Arc::new(SuraParticleMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            MoParticleMatcher,
        )))), // Optional も
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
    impl Matcher for GaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "が"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match ある verb in 連用タ接続 form (あっ)
    #[derive(Debug)]
    struct AruVerbMatcher;
    impl Matcher for AruVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "ある"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用タ接続")
        }
    }

    // Match て as conjunctive particle
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl Matcher for TeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Match の as nominalizing particle
    #[derive(Debug)]
    struct NoParticleMatcher;
    impl Matcher for NoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(GaParticleMatcher)))),
        TokenMatcher::Custom(Arc::new(AruVerbMatcher)),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(NoParticleMatcher)),
        super::noun_matcher(),
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "まで"
                && token.base_form == "まで"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "副助詞")
        }
    }

    // Helper: Match の as 連体化 particle
    #[derive(Debug)]
    struct NoParticleMatcher;
    impl super::Matcher for NoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.base_form == "の"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "連体化")
        }
    }

    // Helper: Match こと as 非自立 noun
    #[derive(Debug)]
    struct KotoNounMatcher;
    impl super::Matcher for KotoNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こと"
                && token.base_form == "こと"
                && token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "非自立")
        }
    }

    // Helper: Match だ or です as auxiliary verb
    #[derive(Debug)]
    struct DaDesuMatcher;
    impl super::Matcher for DaDesuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "だ" || token.surface == "です")
                && token.pos.first().is_some_and(|p| p == "助動詞")
                && (token.base_form == "だ" || token.base_form == "です")
        }
    }

    // Pattern: Verb[連用形/連用タ接続] + た + まで + Optional(の + こと) + だ/です
    super::concat(vec![
        vec![super::flexible_verb_form()],
        vec![super::past_auxiliary()],
        vec![TokenMatcher::Custom(Arc::new(MadeParticleMatcher))],
        vec![
            TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
                NoParticleMatcher,
            )))),
            TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
                KotoNounMatcher,
            )))),
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "を"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match 経る verb in 連用形
    #[derive(Debug)]
    struct HeruVerbMatcher;
    impl super::Matcher for HeruVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "経る"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形")
        }
    }

    // Match て particle (接続助詞)
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl super::Matcher for TeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    vec![
        super::noun_matcher(),
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

    // Matcher for noun or verb stem before ながら (non-compound cases)
    #[derive(Debug)]
    struct NounOrVerbStemMatcher;
    impl Matcher for NounOrVerbStemMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Noun
            if token.pos.first().is_some_and(|pos| pos == "名詞") {
                return true;
            }
            // Verb stem (連用形)
            if token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形")
            {
                return true;
            }
            // Adverb (for いつもながら case)
            if token.pos.first().is_some_and(|pos| pos == "副詞") {
                return true;
            }
            false
        }
    }

    // Matcher for ながら as 助詞/接続助詞
    #[derive(Debug)]
    struct NagaraParticleMatcher;
    impl Matcher for NagaraParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ながら"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Matcher for compound adverbs ending in ながら
    #[derive(Debug)]
    struct NagaraCompoundMatcher;
    impl Matcher for NagaraCompoundMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.surface.ends_with("ながら")
        }
    }

    // Matcher for ながら: either as 助詞 or compound adverb
    #[derive(Debug)]
    struct NagaraMatcher;
    impl Matcher for NagaraMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match ながら as 助詞/接続助詞
            if token.surface == "ながら"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
            {
                return true;
            }
            // Match compound adverbs ending in ながら
            if token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.surface.ends_with("ながら")
            {
                return true;
            }
            false
        }
    }

    // Matcher for に or の particle after ながら
    #[derive(Debug)]
    struct NiNoParticleMatcher;
    impl Matcher for NiNoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
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
            if token.surface == "の" && token.pos.get(1).is_some_and(|pos| pos == "連体化") {
                return true;
            }
            false
        }
    }

    // Matcher for して (する in て-form)
    #[derive(Debug)]
    struct ShiteMatcher;
    impl Matcher for ShiteMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "し"
                && token.base_form == "する"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    // Matcher for て particle
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl Matcher for TeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Pattern: (Noun/Verb/Adverb) + (ながら or compound) + Optional(に/の) + Optional(して)
    vec![
        TokenMatcher::Any,
        TokenMatcher::Custom(Arc::new(NagaraMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NiNoParticleMatcher,
        )))),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(ShiteMatcher)))),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            TeParticleMatcher,
        )))),
    ]
}

// Pattern: たなり・なり (remain as is, stay in that state)
// Structures: Verb[た] + なり + Optional(で)
pub fn tanari_u30fb_nari() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct NariParticleMatcher;
    impl super::Matcher for NariParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "なり"
                && token.base_form == "なり"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
                    || token.pos.get(1).is_some_and(|pos| pos == "副助詞"))
        }
    }

    #[derive(Debug)]
    struct DeParticleMatcher;
    impl super::Matcher for DeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "で"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        super::flexible_verb_form(),
        super::past_auxiliary(),
        TokenMatcher::Custom(Arc::new(NariParticleMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(DeParticleMatcher)))),
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
        }
    }

    // Match 極み as noun
    #[derive(Debug)]
    struct KiwamiMatcher;
    impl super::Matcher for KiwamiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "極み"
                && token.base_form == "極み"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
        }
    }

    vec![
        super::noun_matcher(),
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
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Matcher for し (する in 連用形)
    #[derive(Debug)]
    struct ShiMatcher;
    impl Matcher for ShiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "し"
                && token.base_form == "する"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形")
        }
    }

    // Matcher for て particle (接続助詞)
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl Matcher for TeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Matcher for み or みれ (みる in 連用形 or 仮定形)
    #[derive(Debug)]
    struct MiMatcher;
    impl Matcher for MiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "み" || token.surface == "みれ")
                && token.base_form == "みる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Matcher for れば or たら (conditional endings)
    #[derive(Debug)]
    struct ConditionalEndingMatcher;
    impl Matcher for ConditionalEndingMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
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
        }
    }

    vec![
        super::noun_matcher(),
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
    impl Matcher for DanoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "だの"
                && token.base_form == "だの"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "並立助詞")
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
    impl Matcher for DaAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "だ"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Matcher for の (名詞/非自立/一般)
    #[derive(Debug)]
    struct NoNounMatcher;
    impl Matcher for NoNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.base_form == "の"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "あくまでも" || token.surface == "あくまで")
                && token.base_form == token.surface  // base_form matches surface
                && token.pos.first().is_some_and(|pos| pos == "副詞")  // adverb
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "べく"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "べし"
        }
    }

    // Match verb in dictionary form (基本形 or 文語基本形)
    #[derive(Debug)]
    struct DictionaryFormVerbMatcher;
    impl super::Matcher for DictionaryFormVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token
                    .features
                    .get(5)
                    .is_some_and(|form| form == "基本形" || form == "文語基本形")
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ところ"
                && token.base_form == "ところ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    // Match を as 格助詞
    #[derive(Debug)]
    struct WoParticleMatcher;
    impl super::Matcher for WoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "を"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        TokenMatcher::Any, // Verb, Adjective, Noun, or particle (な, の)
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
    impl Matcher for CounterMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                && token.pos.get(2).is_some_and(|pos| pos == "助数詞")
        }
    }

    // Match から particle
    #[derive(Debug)]
    struct KaraParticleMatcher;
    impl Matcher for KaraParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "から"
                && token.base_form == "から"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match ある or いる verb
    #[derive(Debug)]
    struct AruIruMatcher;
    impl Matcher for AruIruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.base_form == "ある" || token.base_form == "いる")
                && token.pos.first().is_some_and(|pos| pos == "動詞")
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match 名詞 (noun), preferably 形容動詞語幹 (na-adjective stem) or 一般/代名詞/サ変接続
            token.pos.first().is_some_and(|p| p == "名詞")
                && !is_number_or_counter(token)
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    // Match し (する in 連用形)
    #[derive(Debug)]
    struct ShiVerbMatcher;
    impl super::Matcher for ShiVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "し"
                && token.base_form == "する"
                && token.pos.first().is_some_and(|p| p == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形")
        }
    }

    // Match て particle (接続助詞)
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl super::Matcher for TeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "つ"
                && token.base_form == "つ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Match れる/られる auxiliary (for passive forms like 持たれつ)
    #[derive(Debug)]
    struct ReruAuxiliaryMatcher;
    impl super::Matcher for ReruAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "れる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
        }
    }

    vec![
        // First verb in 連用形 (conjunctive form)
        TokenMatcher::verb_with_form("連用形"),
        // First つ
        TokenMatcher::Custom(Arc::new(TsuAuxiliaryMatcher)),
        // Second verb (can be any form - 連用形 for regular, 未然形+れ連用形 for passive)
        TokenMatcher::Any,
        // Allow optional れる/られる auxiliary (for passive forms)
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(ReruAuxiliaryMatcher)))),
        // Second つ
        TokenMatcher::Custom(Arc::new(TsuAuxiliaryMatcher)),
    ]
}

// Pattern: 飽くまで(も)
pub fn akumade_mo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: であれ〜であれ (whether X or Y, no matter if X or Y)
// Structures: Noun/な-Adj + であれ + Noun/な-Adj + であれ
// Note: Handles compound nouns like 日本製 (multiple consecutive noun tokens)
pub fn deare_u301c_deare() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match で (auxiliary verb だ in 連用形)
    #[derive(Debug)]
    struct DeAuxiliaryMatcher;
    impl Matcher for DeAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "だ"
        }
    }

    // Match あれ (auxiliary verb ある in 命令ｅ form)
    #[derive(Debug)]
    struct AreImperativeMatcher;
    impl Matcher for AreImperativeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "あれ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "ある"
                && token.features.get(5).is_some_and(|f| f.contains("命令"))
        }
    }

    // Noun or な-Adjective stem (形容動詞語幹)
    // Both are categorized as 名詞 in pos.first()
    #[derive(Debug)]
    struct NounOrNaAdjectiveMatcher;
    impl Matcher for NounOrNaAdjectiveMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    vec![
        // First noun/な-adjective + であれ
        TokenMatcher::Custom(Arc::new(NounOrNaAdjectiveMatcher)),
        TokenMatcher::Custom(Arc::new(DeAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(AreImperativeMatcher)),
        // Second noun/な-adjective + であれ
        TokenMatcher::Custom(Arc::new(NounOrNaAdjectiveMatcher)),
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
    impl Matcher for TaOrTaraMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "た"
                && (token.features.get(5).is_some_and(|f| f == "基本形")
                    || token.features.get(5).is_some_and(|f| f == "仮定形"))
        }
    }

    // Match が as 接続助詞 (used in た + が + 最後)
    #[derive(Debug)]
    struct GaConjunctionMatcher;
    impl Matcher for GaConjunctionMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "が"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Match 最後 as noun
    #[derive(Debug)]
    struct SaigoMatcher;
    impl Matcher for SaigoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "最後" && token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    vec![
        super::flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(TaOrTaraMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            GaConjunctionMatcher,
        )))),
        TokenMatcher::Custom(Arc::new(SaigoMatcher)),
    ]
}

// Pattern: いかなる (no matter what, any kind of)
// Structures: いかなる + Noun
pub fn ikanaru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct IkanaruMatcher;
    impl Matcher for IkanaruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "いかなる"
                && token.base_form == "いかなる"
                && token.pos.first().is_some_and(|pos| pos == "連体詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(IkanaruMatcher)),
        TokenMatcher::Any, // Noun that follows
    ]
}

// Pattern: なりに (in one's own way, for what it is)
// Structures: Noun/Adjective/Verb + なり + に/の
pub fn narini() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match noun, adjective, verb, or auxiliary verb (for past tense た)
    #[derive(Debug)]
    struct NounAdjVerbMatcher;
    impl Matcher for NounAdjVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| {
                pos == "名詞" || pos == "形容詞" || pos == "動詞" || pos == "助動詞"
            })
        }
    }

    // Match なり as 副助詞 (adverbial particle)
    #[derive(Debug)]
    struct NariParticleMatcher;
    impl Matcher for NariParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "なり"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞")
        }
    }

    // Match に as 格助詞 or の as 連体化
    #[derive(Debug)]
    struct NiOrNoMatcher;
    impl Matcher for NiOrNoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞"))
                || (token.surface == "の"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "連体化"))
        }
    }

    // Pattern: (Noun/Adj/Verb/AuxVerb) + なり(副助詞) + (に OR の)
    // Note: そ れなり is a special case that's tokenized as a single noun "それなり",
    // so it won't be caught by this pattern. We handle it separately below.
    vec![
        TokenMatcher::Custom(Arc::new(NounAdjVerbMatcher)),
        TokenMatcher::Custom(Arc::new(NariParticleMatcher)),
        TokenMatcher::Custom(Arc::new(NiOrNoMatcher)),
    ]
}

// Pattern: それなり + に/の (variant of なりに for fixed expression)
// When "それなり" is tokenized as a single noun
pub fn narini_sorenari() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match "それなり" as a single noun
    #[derive(Debug)]
    struct SorenariMatcher;
    impl Matcher for SorenariMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "それなり" && token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    // Match に as 格助詞 or の as 連体化
    #[derive(Debug)]
    struct NiOrNoMatcher;
    impl Matcher for NiOrNoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞"))
                || (token.surface == "の"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "連体化"))
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(SorenariMatcher)),
        TokenMatcher::Custom(Arc::new(NiOrNoMatcher)),
    ]
}

// Pattern: れる・られる + ままに (as one is told/ordered)
// Structures: Verb[られる] + (が) + まま + (に)
pub fn reru_u30fb_rareru_mamani() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match れる/られる passive auxiliary (動詞/接尾/基本形, base れる or られる)
    #[derive(Debug)]
    struct ReruRareruMatcher;
    impl Matcher for ReruRareruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|p| p == "動詞")
                && token.pos.get(1).is_some_and(|p| p == "接尾")
                && (token.base_form == "れる" || token.base_form == "られる")
                && token.features.get(5).is_some_and(|f| f == "基本形")
        }
    }

    // Match が as conjunction particle (助詞/接続助詞)
    #[derive(Debug)]
    struct GaConjunctionMatcher;
    impl Matcher for GaConjunctionMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "が"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
        }
    }

    // Match まま noun (名詞/非自立/副詞可能)
    #[derive(Debug)]
    struct MamaMatcher;
    impl Matcher for MamaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "まま"
                && token.base_form == "まま"
                && token.pos.first().is_some_and(|p| p == "名詞")
        }
    }

    // Match に case particle (助詞/格助詞)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    vec![
        // Verb[未然形] is already matched, we look for れる/られる
        TokenMatcher::Any, // The verb in 未然形 before れる/られる
        TokenMatcher::Custom(Arc::new(ReruRareruMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(GaConjunctionMatcher)))),
        TokenMatcher::Custom(Arc::new(MamaMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NiParticleMatcher)))),
    ]
}

// Pattern: にまつわる (related to, connected to, surrounding)
// Structures: Noun (A) + にまつわる + (Adjective) + Noun (B)
pub fn nimatsuwaru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match にまつわる as a compound particle
    #[derive(Debug)]
    struct NimatsuwaruMatcher;
    impl Matcher for NimatsuwaruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "にまつわる"
                && token.base_form == "にまつわる"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "連語")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(NimatsuwaruMatcher)),
        // Allow 0-2 tokens between にまつわる and the noun (for adjectives)
        TokenMatcher::Wildcard {
            min: 0,
            max: 2,
            stop_conditions: vec![],
        },
        super::noun_matcher(),
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
    impl Matcher for TaruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "たる"
                && ((token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.base_form == "たる")
                    || (token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.base_form == "たり"))
        }
    }

    // Optional に particle (for たるに) or もの noun (for たるもの)
    #[derive(Debug)]
    struct TaruFollowMatcher;
    impl Matcher for TaruFollowMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match に (格助詞) for たるに
            (token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞"))
                // Or match もの (名詞) for たるもの
                || (token.surface == "もの"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.base_form == "もの")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(TaruMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            TaruFollowMatcher,
        )))),
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "なら"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "だ"
        }
    }

    // Matches で as 助詞/接続助詞 or 助詞/格助詞
    #[derive(Debug)]
    struct DeParticleMatcher;
    impl super::Matcher for DeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
                    || token.pos.get(1).is_some_and(|pos| pos == "格助詞"))
        }
    }

    // Matches verbs (動詞), い-adjectives (形容詞), な-adjectives (名詞/形容動詞語幹)
    #[derive(Debug)]
    struct FirstWordMatcher;
    impl super::Matcher for FirstWordMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                || token.pos.first().is_some_and(|pos| pos == "形容詞")
                || (token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹"))
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(FirstWordMatcher)),
        TokenMatcher::Custom(Arc::new(NaraMatcher)),
        TokenMatcher::Custom(Arc::new(FirstWordMatcher)),
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
    impl Matcher for NoNominalizerMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.base_form == "の"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Matches を case particle (助詞/格助詞/一般)
    #[derive(Debug)]
    struct WoParticleMatcher;
    impl Matcher for WoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "を"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Matches もの (名詞/非自立/一般)
    #[derive(Debug)]
    struct MonoNounMatcher;
    impl Matcher for MonoNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "もの"
                && token.base_form == "もの"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Matches と as quotation particle (助詞/格助詞/引用)
    #[derive(Debug)]
    struct ToQuotationMatcher;
    impl Matcher for ToQuotationMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "引用")
        }
    }

    // Matches も as binding particle (助詞/係助詞)
    #[derive(Debug)]
    struct MoBindingMatcher;
    impl Matcher for MoBindingMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Matches せ (する in 未然ヌ接続)
    #[derive(Debug)]
    struct SeVerbMatcher;
    impl Matcher for SeVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "せ"
                && token.base_form == "する"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然ヌ接続")
        }
    }

    // Matches ず (助動詞, base=ぬ)
    #[derive(Debug)]
    struct ZuAuxiliaryMatcher;
    impl Matcher for ZuAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ず"
                && token.base_form == "ぬ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Matches optional に particle
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        // Optional の (for Verb/Adjective + の patterns)
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NoNominalizerMatcher)))),
        TokenMatcher::Custom(Arc::new(WoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(MonoNounMatcher)),
        TokenMatcher::Custom(Arc::new(ToQuotationMatcher)),
        TokenMatcher::Custom(Arc::new(MoBindingMatcher)),
        TokenMatcher::Custom(Arc::new(SeVerbMatcher)),
        TokenMatcher::Custom(Arc::new(ZuAuxiliaryMatcher)),
        // Optional に at the end
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NiParticleMatcher)))),
    ]
}

// Pattern: には当たらない (not worth doing, no need to)
// Structures: Verb + に(は) + あたらない/あたりません
pub fn nihaataranai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for に particle
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Matcher for は particle
    #[derive(Debug)]
    struct WaParticleMatcher;
    impl Matcher for WaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Matcher for あたる verb in 未然形 (for ない)
    #[derive(Debug)]
    struct AtaraMizenMatcher;
    impl Matcher for AtaraMizenMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "あたる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然形")
        }
    }

    // Matcher for ない auxiliary (must follow あたら)
    #[derive(Debug)]
    struct NaiAuxiliaryMatcher;
    impl Matcher for NaiAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Match: Verb (dictionary form) + に + (は optional) + あたら + ない
    vec![
        TokenMatcher::Verb {
            conjugation_form: None, // Dictionary form
            base_form: None,
        },
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(WaParticleMatcher)))),
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
    impl Matcher for MonoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "もの"
                && token.base_form == "もの"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Match と as quotation particle (助詞/格助詞/引用)
    #[derive(Debug)]
    struct ToQuotationMatcher;
    impl Matcher for ToQuotationMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "と"
                && token.base_form == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "引用")
        }
    }

    // Match おもう verb
    #[derive(Debug)]
    struct OmouVerbMatcher;
    impl Matcher for OmouVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "おもう"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(MonoMatcher)),
        TokenMatcher::Custom(Arc::new(ToQuotationMatcher)),
        TokenMatcher::Custom(Arc::new(OmouVerbMatcher)),
    ]
}

// Pattern: を踏まえて (considering, based on)
// Structures: Noun + を + ふまえ(る) + て/た/た上で/ての
pub fn wofumaete() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match を particle
    #[derive(Debug)]
    struct WoParticleMatcher;
    impl Matcher for WoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "を"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    // Match ふまえ verb (base=ふまえる)
    #[derive(Debug)]
    struct FumaeVerbMatcher;
    impl Matcher for FumaeVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "ふまえる"
                && token.pos.first().is_some_and(|p| p == "動詞")
        }
    }

    // Match て or た (after ふまえ)
    #[derive(Debug)]
    struct TeTaMatcher;
    impl Matcher for TeTaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "て" && token.pos.first().is_some_and(|p| p == "助詞"))
                || (token.surface == "た" && token.pos.first().is_some_and(|p| p == "助動詞"))
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(WoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(FumaeVerbMatcher)),
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
    impl Matcher for YueMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ゆえ"
                && token.base_form == "ゆえ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "接尾" || pos == "非自立"))
        }
    }

    // Match に as 助詞/格助詞
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.base_form == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match ゆえに as single token (接続詞) - used after が or at sentence start
    #[derive(Debug)]
    struct YueniConjunctionMatcher;
    impl Matcher for YueniConjunctionMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ゆえに"
                && token.base_form == "ゆえに"
                && token.pos.first().is_some_and(|pos| pos == "接続詞")
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
    impl Matcher for YueniConjunctionMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ゆえに"
                && token.base_form == "ゆえに"
                && token.pos.first().is_some_and(|pos| pos == "接続詞")
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
    impl Matcher for YueMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ゆえ"
                && token.base_form == "ゆえ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    // Match の as 助詞/連体化
    #[derive(Debug)]
    struct NoParticleMatcher;
    impl Matcher for NoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.base_form == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
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
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    // Match とどまる verb in 未然形 (とどまら)
    #[derive(Debug)]
    struct TodomaruVerbMatcher;
    impl Matcher for TodomaruVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "とどまる"
                && token.pos.first().is_some_and(|p| p == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然形")
        }
    }

    // Match ず auxiliary (base=ぬ)
    #[derive(Debug)]
    struct ZuAuxiliaryMatcher;
    impl Matcher for ZuAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ず"
                && token.base_form == "ぬ"
                && token.pos.first().is_some_and(|p| p == "助動詞")
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
    use std::sync::Arc;
    use super::concat;

    // Match か particle (optional)
    #[derive(Debug)]
    struct KaParticleMatcher;
    impl Matcher for KaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "か"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    // Match と particle (格助詞/引用)
    #[derive(Debug)]
    struct ToQuotativeMatcher;
    impl Matcher for ToQuotativeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match 思う/おもう verb in 連用形
    #[derive(Debug)]
    struct OmouRenyoukeiMatcher;
    impl Matcher for OmouRenyoukeiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.base_form == "思う" || token.base_form == "おもう")
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形")
        }
    }

    // Match き auxiliary verb (classical past tense)
    #[derive(Debug)]
    struct KiAuxiliaryMatcher;
    impl Matcher for KiAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "き"
                && token.base_form == "き"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Match や particle (接続助詞)
    #[derive(Debug)]
    struct YaConjunctiveMatcher;
    impl Matcher for YaConjunctiveMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "や"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Matches any word (verb, noun, adjective) or だ auxiliary
    #[derive(Debug)]
    struct PredicateMatcher;
    impl Matcher for PredicateMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            let is_verb = token.pos.first().is_some_and(|pos| pos == "動詞");
            let is_noun = token.pos.first().is_some_and(|pos| pos == "名詞");
            let is_adj = token.pos.first().is_some_and(|pos| pos == "形容詞");
            let is_da = token.surface == "だ" && token.pos.first().is_some_and(|pos| pos == "助動詞");

            is_verb || is_noun || is_adj || is_da
        }
    }

    // Pattern: [Verb/Noun/Adj/だ] + (optional か) + と + 思い + き + や
    concat(vec![
        vec![TokenMatcher::Custom(Arc::new(PredicateMatcher))],
        vec![TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(KaParticleMatcher))))],
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
    impl Matcher for DounimoAdverbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "どうにも"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
        }
    }

    // Match どう (副詞/助詞類接続)
    #[derive(Debug)]
    struct DouAdverbMatcher;
    impl Matcher for DouAdverbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "どう"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
        }
    }

    // Match こう (副詞/助詞類接続)
    #[derive(Debug)]
    struct KouAdverbMatcher;
    impl Matcher for KouAdverbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こう"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
        }
    }

    // Match に (助詞/格助詞)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match も (助詞/係助詞)
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // どうにも as single token OR どう + に + も
    let dounimo_variants = vec![
        vec![TokenMatcher::Custom(Arc::new(DounimoAdverbMatcher))],
        vec![
            TokenMatcher::Custom(Arc::new(DouAdverbMatcher)),
            TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
            TokenMatcher::Custom(Arc::new(MoParticleMatcher)),
        ],
    ];

    // Optional こう + に + も
    let kounimo = vec![
        TokenMatcher::Custom(Arc::new(KouAdverbMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(MoParticleMatcher)),
    ];

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
    impl Matcher for DouAdverbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "どう"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
        }
    }

    // Match に (助詞/格助詞)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match も (助詞/係助詞)
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こと"
                && token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "非自立")
        }
    }

    // Matcher for だ (auxiliary verb)
    #[derive(Debug)]
    struct DaMatcher;
    impl super::Matcher for DaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "だ"
                && token.pos.first().is_some_and(|p| p == "助動詞")
        }
    }

    // Matcher for し (conjunction particle)
    #[derive(Debug)]
    struct ShiMatcher;
    impl super::Matcher for ShiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "し"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
        }
    }

    vec![
        TokenMatcher::Any,  // Verb or い-Adjective (immediately before こと)
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "の"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "連体化"))
            || (token.surface == "な"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|p| p == "助動詞"))
        }
    }

    // Reuse matchers from kotodashi()
    #[derive(Debug)]
    struct KotoMatcher;
    impl super::Matcher for KotoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こと"
                && token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "非自立")
        }
    }

    #[derive(Debug)]
    struct DaMatcher;
    impl super::Matcher for DaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "だ"
                && token.pos.first().is_some_and(|p| p == "助動詞")
        }
    }

    #[derive(Debug)]
    struct ShiMatcher;
    impl super::Matcher for ShiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "し"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
        }
    }

    vec![
        TokenMatcher::Any,  // Noun or な-Adjective stem
        TokenMatcher::Custom(Arc::new(NoNaMatcher)),  // の or な
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
        TokenMatcher::Custom(Arc::new(DaMatcher)),
        TokenMatcher::Custom(Arc::new(ShiMatcher)),
    ]
}

// Pattern: がん～ (colloquial intensifier - "majorly/completely/furiously")
// Structures: ガン + Verb[stem], ガン + Noun (suru-verb)
pub fn gan_uff5e() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match "ガン" (katakana) as a noun
    #[derive(Debug)]
    struct GanMatcher;
    impl super::Matcher for GanMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ガン"
                && token.base_form == "ガン"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    // Match verb (any conjugation form including stem) or noun following ガン
    #[derive(Debug)]
    struct VerbOrNounMatcher;
    impl super::Matcher for VerbOrNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match verbs (including stem form 連用形)
            if token.pos.first().is_some_and(|pos| pos == "動詞") {
                return true;
            }
            // Match nouns (including サ変接続 and 接尾 like 無視, 切れ)
            if token.pos.first().is_some_and(|pos| pos == "名詞") {
                return true;
            }
            false
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(GanMatcher)),
        TokenMatcher::Custom(Arc::new(VerbOrNounMatcher)),
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "かいな" && token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    // Matcher for か particle
    #[derive(Debug)]
    struct KaParticleMatcher;
    impl super::Matcher for KaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "か" && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    // Matches: Any token + かいな(noun) + か(particle)
    vec![
        TokenMatcher::Any,
        TokenMatcher::Custom(Arc::new(KainaMatcher)),
        TokenMatcher::Custom(Arc::new(KaParticleMatcher)),
    ]
}

// Pattern: か否か (whether or not) - Variant 2
// Structures: Verb/Adj/Noun + である + か + いなか
// This variant appears after である or in certain contexts
pub fn kainaka_ka_inaka() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for いなか (noun)
    #[derive(Debug)]
    struct InakaMatcher;
    impl super::Matcher for InakaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "いなか" && token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    // Matcher for か particle
    #[derive(Debug)]
    struct KaParticleMatcher;
    impl super::Matcher for KaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "か" && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    // Matches: Any token + か(particle) + いなか(noun)
    vec![
        TokenMatcher::Any,
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
    impl Matcher for TaraConditionalMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "たら"
                && token.base_form == "た"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "仮定形")
        }
    }

    // Match た (past auxiliary, 基本形)
    #[derive(Debug)]
    struct TaPastBasicMatcher;
    impl Matcher for TaPastBasicMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "た"
                && token.base_form == "た"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "基本形")
        }
    }

    // Match で (conjunction particle after た)
    #[derive(Debug)]
    struct DeConjunctionMatcher;
    impl Matcher for DeConjunctionMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Verb[連用タ接続] + たら + Verb[連用タ接続] + た + で
    vec![
        super::flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(TaraConditionalMatcher)),
        super::flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(TaPastBasicMatcher)),
        TokenMatcher::Custom(Arc::new(DeConjunctionMatcher)),
    ]
}

// Pattern: たら〜で - ば variant (verb)
// Structures: Verb[仮定形] + ば + Verb[連用タ接続] + た + で
pub fn tara_u301c_de_ba_verb() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match verb in 仮定形 (hypothetical form)
    #[derive(Debug)]
    struct VerbKateiMatcher;
    impl Matcher for VerbKateiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "仮定形")
        }
    }

    // Match ば (conditional particle)
    #[derive(Debug)]
    struct BaMatcher;
    impl Matcher for BaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ば"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Match た (past auxiliary, 基本形)
    #[derive(Debug)]
    struct TaPastBasicMatcher;
    impl Matcher for TaPastBasicMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "た"
                && token.base_form == "た"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "基本形")
        }
    }

    // Match で (conjunction particle after た)
    #[derive(Debug)]
    struct DeConjunctionMatcher;
    impl Matcher for DeConjunctionMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbKateiMatcher)),
        TokenMatcher::Custom(Arc::new(BaMatcher)),
        super::flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(TaPastBasicMatcher)),
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
    impl Matcher for IAdjRenyouMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "連用タ接続")
        }
    }

    // Match たら (conditional auxiliary)
    #[derive(Debug)]
    struct TaraConditionalMatcher;
    impl Matcher for TaraConditionalMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "たら"
                && token.base_form == "た"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "仮定形")
        }
    }

    // Match い-adjective in 基本形
    #[derive(Debug)]
    struct IAdjBasicMatcher;
    impl Matcher for IAdjBasicMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "基本形")
        }
    }

    // Match で (conjunction particle or だ auxiliary in 連用形)
    #[derive(Debug)]
    struct DeAfterAdjMatcher;
    impl Matcher for DeAfterAdjMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "で"
                && ((token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "接続助詞"))
                    || (token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.base_form == "だ"))
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(IAdjRenyouMatcher)),
        TokenMatcher::Custom(Arc::new(TaraConditionalMatcher)),
        TokenMatcher::Custom(Arc::new(IAdjBasicMatcher)),
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
    impl Matcher for IAdjKateiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "仮定形")
        }
    }

    // Match ば (conditional particle)
    #[derive(Debug)]
    struct BaMatcher;
    impl Matcher for BaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ば"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Match い-adjective in 基本形
    #[derive(Debug)]
    struct IAdjBasicMatcher;
    impl Matcher for IAdjBasicMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "基本形")
        }
    }

    // Match で (auxiliary だ in 連用形 after adjective)
    #[derive(Debug)]
    struct DeAuxiliaryMatcher;
    impl Matcher for DeAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "だ"
                && token.features.get(5).is_some_and(|f| f == "連用形")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(IAdjKateiMatcher)),
        TokenMatcher::Custom(Arc::new(BaMatcher)),
        TokenMatcher::Custom(Arc::new(IAdjBasicMatcher)),
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
    impl Matcher for NaAdjMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹")
        }
    }

    // Match なら (conditional auxiliary, 仮定形 of だ)
    #[derive(Debug)]
    struct NaraConditionalMatcher;
    impl Matcher for NaraConditionalMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "なら"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "仮定形")
        }
    }

    // Match で (auxiliary だ in 連用形)
    #[derive(Debug)]
    struct DeAuxiliaryMatcher;
    impl Matcher for DeAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "だ"
                && token.features.get(5).is_some_and(|f| f == "連用形")
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
    impl Matcher for BekuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "べく"
                && token.base_form == "べし"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Match し from する (連用形)
    #[derive(Debug)]
    struct ShiMatcher;
    impl Matcher for ShiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "し"
                && token.base_form == "する"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    // Match て (conjunctive particle)
    #[derive(Debug)]
    struct TeMatcher;
    impl Matcher for TeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    vec![
        TokenMatcher::Any, // Verb before べくして
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
    impl Matcher for KareFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface.ends_with("かれ")
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立")
                && token.features.get(5).is_some_and(|f| f == "命令ｅ")
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
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Matcher for optional volitional う/よう
    // Note: For godan verbs, it's う auxiliary (助動詞)
    //       For ichidan verbs, it's よう noun suffix (名詞/接尾)
    #[derive(Debug)]
    struct VolitionalMatcher;
    impl Matcher for VolitionalMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
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
        }
    }

    // Matcher for optional も particle
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Matcher for optional verb before potential auxiliary (for ichidan verbs)
    // Matches: 辞め (base=辞める, 未然形) that comes before られる
    #[derive(Debug)]
    struct OptionalVerbBeforePotentialMatcher;
    impl Matcher for OptionalVerbBeforePotentialMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Only match verbs in 未然形 (ichidan verb stems before られる)
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立")
                && token.features.get(5).is_some_and(|f| f == "未然形")
                // Should NOT be a potential form itself (not ending in える/れる/できる/られる)
                && !token.base_form.ends_with("える")
                && !token.base_form.ends_with("れる")
                && token.base_form != "できる"
                && token.base_form != "られる"
        }
    }

    // Matcher for potential form verb in 未然形 + ない/なかった
    // This matches:
    // - Godan verbs: 笑え (base=笑える), 断れ (base=断れる)
    // - Ichidan potential auxiliary: られ (base=られる)
    // - Special: でき (base=できる) for する verbs
    #[derive(Debug)]
    struct PotentialNegativeMatcher;
    impl Matcher for PotentialNegativeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Matches potential verb forms (笑える, 断れる, できる, られる) in 未然形
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "自立")
                    || token.pos.get(1).is_some_and(|pos| pos == "接尾"))
                && token.features.get(5).is_some_and(|f| f == "未然形")
                && (token.base_form.ends_with("える")
                    || token.base_form.ends_with("れる")
                    || token.base_form == "できる"
                    || token.base_form == "られる")
        }
    }

    // Matcher for ない auxiliary (present or past)
    #[derive(Debug)]
    struct NaiMatcher;
    impl Matcher for NaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "ない"
        }
    }

    // Matcher for optional た auxiliary (for past tense)
    #[derive(Debug)]
    struct TaMatcher;
    impl Matcher for TaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "た"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "た"
        }
    }

    vec![
        TokenMatcher::Any, // First verb (dictionary or volitional form)
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(VolitionalMatcher)))),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(MoParticleMatcher)))),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(OptionalVerbBeforePotentialMatcher)))), // Optional verb stem (for ichidan: 辞め before られ)
        TokenMatcher::Custom(Arc::new(PotentialNegativeMatcher)),
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(TaMatcher)))),
    ]
}

// Pattern: なくして(は)
// Pattern: なくして(は) (without)
// Structures: Noun + なくして(は) OR Verb + ことなくして(は)
pub fn nakushite_ha() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for なくし or なく (verb なくす or adjective ない)
    #[derive(Debug)]
    struct NakushiNakuMatcher;
    impl Matcher for NakushiNakuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match なくし (verb なくす in 連用形)
            (token.surface == "なくし"
                && token.base_form == "なくす"
                && token.pos.first().is_some_and(|p| p == "動詞"))
            ||
            // Match なく (adjective ない in 連用テ接続)
            (token.surface == "なく"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|p| p == "形容詞"))
        }
    }

    // Optional matcher for し (verb する in 連用形) - only for なく variant
    #[derive(Debug)]
    struct ShiVerbMatcher;
    impl Matcher for ShiVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "し"
                && token.base_form == "する"
                && token.pos.first().is_some_and(|p| p == "動詞")
        }
    }

    // Matcher for て particle
    #[derive(Debug)]
    struct TeMatcher;
    impl Matcher for TeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
        }
    }

    // Matcher for は particle
    #[derive(Debug)]
    struct HaMatcher;
    impl Matcher for HaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "係助詞")
        }
    }

    vec![
        TokenMatcher::Any, // Noun or こと
        TokenMatcher::Custom(Arc::new(NakushiNakuMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(ShiVerbMatcher)))),
        TokenMatcher::Custom(Arc::new(TeMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(HaMatcher)))),
    ]
}

// Pattern: のなんのって (extremely, so much that)
// Structures: Verb + のなんのって, い-Adj + のなんのって, な-Adj + な + のなんのって
pub fn nonannotte() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match の as any form (particle or noun)
    #[derive(Debug)]
    struct NoMatcher;
    impl super::Matcher for NoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
        }
    }

    // Match な auxiliary (da copula in rentaikei form)
    #[derive(Debug)]
    struct NaAuxiliaryMatcher;
    impl super::Matcher for NaAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "な"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Match なんの as proper noun - used after verbs
    #[derive(Debug)]
    struct NannoProperNounMatcher;
    impl super::Matcher for NannoProperNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "なんの"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "固有名詞")
        }
    }

    // Match ん as dependent noun (only for adjective patterns)
    #[derive(Debug)]
    struct NNounMatcher;
    impl super::Matcher for NNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ん"
                && token.base_form == "ん"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Match って particle
    #[derive(Debug)]
    struct TteParticleMatcher;
    impl super::Matcher for TteParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "って"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
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
        TokenMatcher::Wildcard {
            min: 1,
            max: 1,
            stop_conditions: vec![],
        },
        TokenMatcher::Custom(Arc::new(NoMatcher)),                 // の (any form)
        TokenMatcher::Custom(Arc::new(NaAuxiliaryMatcher)),       // な (auxiliary only)
        TokenMatcher::Custom(Arc::new(NNounMatcher)),             // ん
        TokenMatcher::Custom(Arc::new(NoMatcher)),                 // の (again)
        TokenMatcher::Custom(Arc::new(TteParticleMatcher)),       // って
    ]
}

// Pattern: のなんのって (extremely) - Verb variant
// Structures: Verb[た] + のなんのって
pub fn nonannotte_verb() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Reuse the same matchers
    #[derive(Debug)]
    struct NoParticleMatcher;
    impl super::Matcher for NoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
        }
    }

    #[derive(Debug)]
    struct NannoMatcher;
    impl super::Matcher for NannoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "なんの"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "固有名詞")
        }
    }

    #[derive(Debug)]
    struct TteMatcher;
    impl super::Matcher for TteMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "って"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Verb pattern: Wildcard + の(連体化) + なんの(固有名詞) + って
    vec![
        TokenMatcher::Wildcard {
            min: 1,
            max: 1,
            stop_conditions: vec![],
        },
        TokenMatcher::Custom(Arc::new(NoParticleMatcher)),        // の (particle, not noun)
        TokenMatcher::Custom(Arc::new(NannoMatcher)),             // なんの (proper noun)
        TokenMatcher::Custom(Arc::new(TteMatcher)),               // って
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    // Match verb かかる in te-form (連用タ接続)
    #[derive(Debug)]
    struct KakaruVerbMatcher;
    impl super::Matcher for KakaruVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "かかる"
                && token.pos.first().is_some_and(|p| p == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用タ接続")
        }
    }

    // Match て particle
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl super::Matcher for TeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(KakaruVerbMatcher)),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
        TokenMatcher::specific_verb("いる"),
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "やむ"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && (token.features.get(5).map(|f| f.as_str()) == Some("未然形")
                    || token.features.get(5).map(|f| f.as_str()) == Some("連用形"))
        }
    }

    // Match ない (auxiliary) or ません
    #[derive(Debug)]
    struct NaiMasenMatcher;
    impl super::Matcher for NaiMasenMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match ない (助動詞)
            if token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
            {
                return true;
            }
            // Match ませ (ます, 未然形)
            if token.surface == "ませ"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).map(|f| f.as_str()) == Some("未然形")
            {
                return true;
            }
            false
        }
    }

    // Match ん (for ません)
    #[derive(Debug)]
    struct NMatcher;
    impl super::Matcher for NMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ん"
                && token.base_form == "ん"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Match て or で particle
    #[derive(Debug)]
    struct TeFormMatcher;
    impl super::Matcher for TeFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    vec![
        super::flexible_verb_form(),  // Verb in 連用形 or 連用タ接続
        TokenMatcher::Custom(Arc::new(TeFormMatcher)),  // て or で
        TokenMatcher::Custom(Arc::new(YamuMatcher)),
        TokenMatcher::Custom(Arc::new(NaiMasenMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NMatcher)))),  // Optional ん for polite
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "ぐらい" || token.surface == "くらい")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞")
        }
    }

    // Match なら (助動詞 with base だ in 仮定形)
    #[derive(Debug)]
    struct NaraMatcher;
    impl super::Matcher for NaraMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "なら"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token
                    .features
                    .get(5)
                    .is_some_and(|f| f == "仮定形")
        }
    }

    vec![
        TokenMatcher::verb_with_form("基本形"), // Dictionary form verb
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
    impl Matcher for TteMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "って"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
                && token.pos.get(2).is_some_and(|p| p == "連語")
        }
    }

    // Matcher for ば (助詞/接続助詞)
    #[derive(Debug)]
    struct BaMatcher;
    impl Matcher for BaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ば"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
        }
    }

    // Pattern: [Any] + って + ば
    vec![
        TokenMatcher::Any,
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
    impl Matcher for DatteMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "だって"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "副助詞")
        }
    }

    // Matcher for ば (助詞/接続助詞)
    #[derive(Debug)]
    struct BaMatcher;
    impl Matcher for BaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ば"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
        }
    }

    // Pattern: [Any] + だって + ば
    vec![
        TokenMatcher::Any,
        TokenMatcher::Custom(Arc::new(DatteMatcher)),
        TokenMatcher::Custom(Arc::new(BaMatcher)),
    ]
}

// Pattern: ってば・ったら - だったら variant
// Handles Pattern 3: だっ + たら (e.g., 大人だったら)
pub fn tteba_u30fb_ttara_dattara() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for だっ (助動詞 だ in 連用タ接続)
    #[derive(Debug)]
    struct DatMatcher;
    impl Matcher for DatMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "だっ"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|p| p == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "連用タ接続")
        }
    }

    // Matcher for たら (助動詞 た in 仮定形)
    #[derive(Debug)]
    struct TaraMatcher;
    impl Matcher for TaraMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "たら"
                && token.base_form == "た"
                && token.pos.first().is_some_and(|p| p == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "仮定形")
        }
    }

    // Pattern: [Any] + だっ + たら
    vec![
        TokenMatcher::Any,
        TokenMatcher::Custom(Arc::new(DatMatcher)),
        TokenMatcher::Custom(Arc::new(TaraMatcher)),
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
    impl Matcher for ZuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ず"
                && token.pos.first().is_some_and(|p| p == "助動詞")
                && token.base_form == "ぬ"
        }
    }

    // Match とも as single token (助詞/接続助詞)
    #[derive(Debug)]
    struct TomoMatcher;
    impl Matcher for TomoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "とも"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
        }
    }

    // Match と (quotation particle)
    #[derive(Debug)]
    struct ToParticleMatcher;
    impl Matcher for ToParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "と"
                && token.pos.first().is_some_and(|p| p == "助詞")
        }
    }

    // Match も (binding particle)
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "も"
                && token.pos.first().is_some_and(|p| p == "助詞")
        }
    }

    // Match verb in 未然形 or 未然ヌ接続 (nai-stem for classical ず)
    #[derive(Debug)]
    struct NaiStemVerbMatcher;
    impl Matcher for NaiStemVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|p| p == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然形" || f == "未然ヌ接続")
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
    impl Matcher for ZuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ず"
                && token.pos.first().is_some_and(|p| p == "助動詞")
                && token.base_form == "ぬ"
        }
    }

    // Match と particle
    #[derive(Debug)]
    struct ToParticleMatcher;
    impl Matcher for ToParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "と"
                && token.pos.first().is_some_and(|p| p == "助詞")
        }
    }

    // Match も particle
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "も"
                && token.pos.first().is_some_and(|p| p == "助詞")
        }
    }

    // Match verb in 未然形 or 未然ヌ接続
    #[derive(Debug)]
    struct NaiStemVerbMatcher;
    impl Matcher for NaiStemVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|p| p == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然形" || f == "未然ヌ接続")
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
    impl Matcher for ToParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match あっ (verb ある in 連用タ接続 form)
    #[derive(Debug)]
    struct AtteMatcher;
    impl Matcher for AtteMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "ある"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|form| form == "連用タ接続")
        }
    }

    // Match て particle
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl Matcher for TeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    vec![
        TokenMatcher::Any,
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match も (助詞/係助詞)
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl super::Matcher for MoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Match なん (名詞/代名詞/一般)
    #[derive(Debug)]
    struct NanNounMatcher;
    impl super::Matcher for NanNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "なん"
                && token.base_form == "なん"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    // Match ない (形容詞/自立)
    #[derive(Debug)]
    struct NaiAdjectiveMatcher;
    impl super::Matcher for NaiAdjectiveMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
        }
    }

    // Pattern: [Any] + で + も + なん + で + も + ない
    // This handles nouns: ファン + で + も + なん + で + も + ない
    vec![
        TokenMatcher::Any,
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

    // Match でも (助詞/副助詞) - single token
    #[derive(Debug)]
    struct DemoParticleMatcher;
    impl super::Matcher for DemoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "でも"
                && token.base_form == "でも"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞")
        }
    }

    // Match で (助詞/格助詞/一般) case particle
    #[derive(Debug)]
    struct DeParticleMatcher;
    impl super::Matcher for DeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match も (助詞/係助詞)
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl super::Matcher for MoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Match なん (名詞/代名詞/一般)
    #[derive(Debug)]
    struct NanNounMatcher;
    impl super::Matcher for NanNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "なん"
                && token.base_form == "なん"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    // Match ない (形容詞/自立)
    #[derive(Debug)]
    struct NaiAdjectiveMatcher;
    impl super::Matcher for NaiAdjectiveMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
        }
    }

    // Pattern: [Any] + でも + なん + で + も + ない
    // This handles な-adjectives: 迷惑 + でも + なん + で + も + ない
    vec![
        TokenMatcher::Any,
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Match で (助詞/格助詞/一般) case particle
    #[derive(Debug)]
    struct DeParticleMatcher;
    impl super::Matcher for DeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match なん (名詞/代名詞/一般)
    #[derive(Debug)]
    struct NanNounMatcher;
    impl super::Matcher for NanNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "なん"
                && token.base_form == "なん"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    // Match ない (形容詞/自立)
    #[derive(Debug)]
    struct NaiAdjectiveMatcher;
    impl super::Matcher for NaiAdjectiveMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
        }
    }

    // Pattern: [Any] + も + なん + で + も + ない
    // This handles い-adjectives in ku-form: 重く + も + なん + で + も + ない
    vec![
        TokenMatcher::Any,
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ぐるみ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
        }
    }

    // Match で (case particle) or の (nominalizer)
    #[derive(Debug)]
    struct DeNoMatcher;
    impl super::Matcher for DeNoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞"))
            || (token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化"))
        }
    }

    vec![
        TokenMatcher::Any,  // Preceding noun
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
    impl Matcher for SobaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "そば"
                && token.base_form == "そば"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    // Match から as case particle (格助詞)
    #[derive(Debug)]
    struct KaraParticleMatcher;
    impl Matcher for KaraParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "から"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match verb in dictionary form or continuative form (連用形/連用タ接続)
    #[derive(Debug)]
    struct VerbBeforeSobaMatcher;
    impl Matcher for VerbBeforeSobaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            if !token.pos.first().is_some_and(|pos| pos == "動詞") {
                return false;
            }
            // Dictionary form (基本形) or continuative forms
            if let Some(form) = token.features.get(5) {
                form == "基本形" || form == "連用形" || form == "連用タ接続"
            } else {
                false
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbBeforeSobaMatcher)),
        TokenMatcher::Optional(Box::new(super::past_auxiliary())), // た/だ for past tense
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // わけ as 名詞/一般 or 動詞/自立/連用形
            // 訳 as 名詞/一般 or 名詞/接尾/一般
            (token.surface == "わけ" || token.surface == "訳")
                && (token.pos.first().is_some_and(|pos| pos == "名詞")
                    || (token.pos.first().is_some_and(|pos| pos == "動詞")
                        && token.features.get(5).is_some_and(|f| f == "連用形")))
        }
    }

    // Match あり (ある verb in various forms)
    #[derive(Debug)]
    struct AriMatcher;
    impl super::Matcher for AriMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // あり as 動詞 with base ある or あり (classical ラ変)
            token.surface == "あり"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && (token.base_form == "ある" || token.base_form == "あり")
        }
    }

    // Match あっ (ある verb in 連用タ接続 for あって form)
    #[derive(Debug)]
    struct AtteMatcher;
    impl super::Matcher for AtteMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // あっ as 動詞/連用タ接続 with base あう (misanalyzed) or ある
            (token.surface == "あっ" || token.surface == "あり")
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| {
                    f == "連用タ接続" || f == "連用形" || f == "基本形"
                })
        }
    }

    // Match particles/auxiliaries that can follow: な, の, で, て
    #[derive(Debug)]
    struct FollowingParticleMatcher;
    impl super::Matcher for FollowingParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // な (助詞/終助詞)
            // の (助詞/連体化)
            // で (助動詞/連用形 with base だ)
            // て (助詞/接続助詞)
            (token.surface == "な" && token.pos.first().is_some_and(|pos| pos == "助詞"))
                || (token.surface == "の" && token.pos.first().is_some_and(|pos| pos == "助詞"))
                || (token.surface == "で"
                    && (token.pos.first().is_some_and(|pos| pos == "助動詞")
                        || token.pos.first().is_some_and(|pos| pos == "助詞")))
                || (token.surface == "て" && token.pos.first().is_some_and(|pos| pos == "助詞"))
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(WakeMatcher)),
        TokenMatcher::Custom(Arc::new(AtteMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            FollowingParticleMatcher,
        )))),
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "至る"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && (token.surface == "至っ" || token.surface == "至り")
        }
    }

    // Match て particle following 至っ
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl super::Matcher for TeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    vec![
        TokenMatcher::Any, // Verb or Noun
        TokenMatcher::Surface("に"),
        TokenMatcher::Custom(Arc::new(ItatteMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(TeParticleMatcher)))),
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

    // Matches だ auxiliary (特殊・ダ/基本形)
    #[derive(Debug)]
    struct DaAuxiliaryMatcher;
    impl Matcher for DaAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "だ"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token
                    .features
                    .get(4)
                    .is_some_and(|f| f.as_str() == "特殊・ダ")
        }
    }

    // Matches に particle (格助詞)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Matches し (from する) in 未然形 or 連用形
    #[derive(Debug)]
    struct ShiVerbMatcher;
    impl Matcher for ShiVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "し"
                && token.base_form == "する"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    // Matches ない auxiliary
    #[derive(Debug)]
    struct NaiAuxiliaryMatcher;
    impl Matcher for NaiAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Pattern for サ変接続 nouns: Noun + だ + に + し + ない
    vec![
        TokenMatcher::Any, // Noun (usually サ変接続 like 予想, 想像)
        TokenMatcher::Custom(Arc::new(DaAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(ShiVerbMatcher)),
        TokenMatcher::Custom(Arc::new(NaiAuxiliaryMatcher)),
    ]
}

// Pattern: Noun + だに (as single particle) + Verb + ない
// This handles cases where だに tokenizes as 助詞/副助詞
pub fn dani_shinai_particle() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matches だに as a single particle (助詞/副助詞)
    #[derive(Debug)]
    struct DaniParticleMatcher;
    impl Matcher for DaniParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "だに"
                && token.base_form == "だに"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞")
        }
    }

    // Matches ない auxiliary
    #[derive(Debug)]
    struct NaiAuxiliaryMatcher;
    impl Matcher for NaiAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        TokenMatcher::Any, // Noun (like 思い, 夢)
        TokenMatcher::Custom(Arc::new(DaniParticleMatcher)),
        TokenMatcher::Wildcard {
            min: 1,
            max: 3,
            stop_conditions: vec![],
        }, // Verb phrase (may include て, いる, etc.)
        TokenMatcher::Custom(Arc::new(NaiAuxiliaryMatcher)),
    ]
}

// Pattern: がてら (while doing, on the occasion of)
// Structures: Verb[stem] + がてら, Noun + がてら
pub fn gatera() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matches が particle (格助詞)
    #[derive(Debug)]
    struct GaParticleMatcher;
    impl Matcher for GaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "が"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Matches てら (tokenized as verb, base=てる, 未然形)
    #[derive(Debug)]
    struct TeraMatcher;
    impl Matcher for TeraMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "てら"
                && token.base_form == "てる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    vec![
        TokenMatcher::Any, // Verb or Noun
        TokenMatcher::Custom(Arc::new(GaParticleMatcher)),
        TokenMatcher::Custom(Arc::new(TeraMatcher)),
    ]
}

// Pattern: んがため(に) (for the purpose of, in order to)
// Structures: Verb[未然形] + ん + が + ため + に/の
pub fn ngatame_ni() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ん (can be 助動詞 or 名詞 depending on context)
    #[derive(Debug)]
    struct NMatcher;
    impl super::Matcher for NMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ん"
                && (token.pos.first().is_some_and(|pos| pos == "助動詞")
                    || token.pos.first().is_some_and(|pos| pos == "名詞"))
        }
    }

    // Match が (接続助詞 or 格助詞)
    #[derive(Debug)]
    struct GaMatcher;
    impl super::Matcher for GaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "が"
                && token.base_form == "が"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
                    || token.pos.get(1).is_some_and(|pos| pos == "格助詞"))
        }
    }

    // Match ため as noun
    #[derive(Debug)]
    struct TameMatcher;
    impl super::Matcher for TameMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ため"
                && token.base_form == "ため"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    // Match に or の particle
    #[derive(Debug)]
    struct NiOrNoParticleMatcher;
    impl super::Matcher for NiOrNoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "に" || token.surface == "の")
                && token.base_form == token.surface
                && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    // Match verb in 未然形 or 連用形 (for causative forms like させる → させん)
    #[derive(Debug)]
    struct VerbBeforeNMatcher;
    impl super::Matcher for VerbBeforeNMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token
                    .features
                    .get(5)
                    .is_some_and(|f| f == "未然形" || f == "連用形")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbBeforeNMatcher)),
        TokenMatcher::Custom(Arc::new(NMatcher)),
        TokenMatcher::Custom(Arc::new(GaMatcher)),
        TokenMatcher::Custom(Arc::new(TameMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NiOrNoParticleMatcher,
        )))),
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "いかん"
                && token.base_form == "いかん"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    // Match に or を particle
    #[derive(Debug)]
    struct NiOrWoParticleMatcher;
    impl super::Matcher for NiOrWoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "に" || token.surface == "を")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match かかわら/よら/とわ verbs in 未然形
    #[derive(Debug)]
    struct IkanVerbMatcher;
    impl super::Matcher for IkanVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && (token.base_form == "かかわる" || token.base_form == "よる" || token.base_form == "とう")
                && token.features.get(5).is_some_and(|f| f == "未然形")
        }
    }

    // Match ず (classical negative auxiliary)
    #[derive(Debug)]
    struct ZuMatcher;
    impl super::Matcher for ZuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ず"
                && token.base_form == "ぬ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
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
        }
    }

    // Matcher for に particle
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Matcher for も particle (REQUIRED for this pattern)
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl super::Matcher for MoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Matcher for ない auxiliary
    #[derive(Debug)]
    struct NaiMatcher;
    impl super::Matcher for NaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        // Start with any verb (will be before volitional)
        TokenMatcher::Any,
        // Volitional form (う or よう)
        TokenMatcher::Custom(Arc::new(VolitionalMatcher)),
        // に particle
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        // も particle (required)
        TokenMatcher::Custom(Arc::new(MoParticleMatcher)),
        // Match tokens until ない (potential verb forms, auxiliary verbs, etc.)
        TokenMatcher::Wildcard {
            min: 1,
            max: 5,
            stop_conditions: vec![],
        },
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
    impl Matcher for IAdjKuOrTaiKuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match い-adjectives ending in く (連用テ接続 or 連用形)
            let is_i_adj_ku = token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.surface.ends_with("く")
                && token.features.get(5).is_some_and(|f| f == "連用テ接続" || f == "連用形");

            // Match たい auxiliary in たく form (連用テ接続)
            let is_tai_ku = token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "たい"
                && token.surface == "たく"
                && token.features.get(5).is_some_and(|f| f == "連用テ接続");

            is_i_adj_ku || is_tai_ku
        }
    }

    // Match も particle (係助詞)
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Match なんとも adverb
    #[derive(Debug)]
    struct NantomoMatcher;
    impl Matcher for NantomoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "なんとも"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.base_form == "なんとも"
        }
    }

    // Match ない adjective (基本形)
    #[derive(Debug)]
    struct NaiMatcher;
    impl Matcher for NaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ない"
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.base_form == "ない"
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

    // Match verbs in dictionary form (基本形)
    #[derive(Debug)]
    struct DictionaryFormVerbMatcher;
    impl Matcher for DictionaryFormVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "基本形")
        }
    }

    // Match だに particle (助詞/副助詞)
    #[derive(Debug)]
    struct DaniParticleMatcher;
    impl Matcher for DaniParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "だに"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(DictionaryFormVerbMatcher)),
        TokenMatcher::Custom(Arc::new(DaniParticleMatcher)),
    ]
}

// Pattern: ～なり～なり (either...or...)
// Structures: Item1 + なり + Item2 + なり
// Item can be: Verb(dictionary), Noun, or Noun + Particle
// Note: なり can be 助詞/接続助詞, 助詞/並立助詞, or 助詞/副助詞
pub fn uff5e_nari_uff5e_nari() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for なり as adverbial particle (not verb なる)
    #[derive(Debug)]
    struct NariParticleMatcher;
    impl Matcher for NariParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            if token.surface != "なり" {
                return false;
            }
            if !token.pos.first().is_some_and(|pos| pos == "助詞") {
                return false;
            }
            // Check if second POS element is one of the valid particle types
            token.pos.get(1).is_some_and(|p| {
                p == "接続助詞" || p == "並立助詞" || p == "副助詞"
            })
        }
    }

    // Matcher for verb or noun
    #[derive(Debug)]
    struct VerbOrNounMatcher;
    impl Matcher for VerbOrNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞" || pos == "名詞")
        }
    }

    // Pattern: (Verb|Noun) + なり + (1-8 tokens) + なり
    // Simple pattern without wildcards between item and なり for now
    vec![
        TokenMatcher::Custom(Arc::new(VerbOrNounMatcher)),
        TokenMatcher::Custom(Arc::new(NariParticleMatcher)),
        TokenMatcher::Wildcard {
            min: 1,
            max: 8,
            stop_conditions: vec![],
        },
        TokenMatcher::Custom(Arc::new(NariParticleMatcher)),
    ]
}

// Pattern: ないでもない
pub fn naidemonai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: もさることながら
pub fn mosarukotonagara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ものと思っていた
pub fn monotoomotteita() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: でなくてなんだろう
pub fn denakutenandarou() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: はさておき・はさておいて
pub fn hasateoki_u30fb_hasateoite() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 折には
pub fn oriniha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: とばかり（に）
pub fn tobakari_uff08_ni_uff09() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: わ〜わ
pub fn wa_u301c_wa() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なりとも
pub fn naritomo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に至っても
pub fn niitattemo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: を兼ねて
pub fn wokanete() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Verb[ない]もの(だろう)か (if only, isn't there a way to)
// Structures: Verb[ない] + もの + (だろう/でしょう) + か
pub fn verb_nai_mono_darou_ka() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ない (助動詞 or 形容詞 for existence)
    #[derive(Debug)]
    struct NaiMatcher;
    impl Matcher for NaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ない"
                && token.base_form == "ない"
                && (token.pos.first().is_some_and(|p| p == "助動詞")
                    || token.pos.first().is_some_and(|p| p == "形容詞"))
        }
    }

    // Match もの (名詞/非自立/一般)
    #[derive(Debug)]
    struct MonoMatcher;
    impl Matcher for MonoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "もの"
                && token.base_form == "もの"
                && token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "非自立")
        }
    }

    // Match だろ or でしょ (助動詞, 未然形 of だ or です)
    #[derive(Debug)]
    struct DarouDeshouMatcher;
    impl Matcher for DarouDeshouMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "だろ" || token.surface == "でしょ")
                && (token.base_form == "だ" || token.base_form == "です")
                && token.pos.first().is_some_and(|p| p == "助動詞")
                && token
                    .features
                    .get(5)
                    .is_some_and(|f| f.contains("未然"))
        }
    }

    // Match う (助動詞, 基本形)
    #[derive(Debug)]
    struct UMatcher;
    impl Matcher for UMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "う"
                && token.base_form == "う"
                && token.pos.first().is_some_and(|p| p == "助動詞")
        }
    }

    // Match か (助詞/副助詞／並立助詞／終助詞)
    #[derive(Debug)]
    struct KaMatcher;
    impl Matcher for KaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "か"
                && token.pos.first().is_some_and(|p| p == "助詞")
        }
    }

    // Pattern: ない + もの + Optional(だろう/でしょう) + か
    vec![
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
        TokenMatcher::Custom(Arc::new(MonoMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            DarouDeshouMatcher,
        )))),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(UMatcher)))),
        TokenMatcher::Custom(Arc::new(KaMatcher)),
    ]
}

// Pattern: Verb[て] + みせる (I will definitely do, I swear I will do)
// Structures: Verb[て] + みせる/みせます
pub fn verb_te_miseru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match て or で particle (接続助詞)
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl Matcher for TeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Match みせる as auxiliary verb (動詞/非自立)
    #[derive(Debug)]
    struct MiseruAuxiliaryMatcher;
    impl Matcher for MiseruAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "みせる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    vec![
        super::flexible_verb_form(),
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "が" || token.surface == "と")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    // Match 相まって as adverb
    #[derive(Debug)]
    struct AimatteMatcher;
    impl super::Matcher for AimatteMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "相まって"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(GaToParticleMatcher)),
        TokenMatcher::Custom(Arc::new(AimatteMatcher)),
    ]
}

// Pattern: に足りない
pub fn nitarinai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: べからず
pub fn bekarazu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: んばかりに
pub fn nbakarini() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に則って・に則り
pub fn ninottotte_u30fb_ninottori() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
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

    // Match い-adjective or な-adjective
    #[derive(Debug)]
    struct AdjectiveMatcher;
    impl Matcher for AdjectiveMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // い-adjective: 形容詞/自立, in 基本形
            (token.pos.first().is_some_and(|p| p == "形容詞")
                && token.pos.get(1).is_some_and(|p| p == "自立")
                && token.features.get(5).is_some_and(|f| f == "基本形"))
            ||
            // な-adjective: 名詞/形容動詞語幹
            (token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "形容動詞語幹"))
        }
    }

    // Match な copula (for な-adjectives)
    #[derive(Debug)]
    struct NaCopulaMatcher;
    impl Matcher for NaCopulaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "な"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|p| p == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "体言接続")
        }
    }

    // Match 限り noun
    #[derive(Debug)]
    struct KagiriMatcher;
    impl Matcher for KagiriMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "限り"
                && token.base_form == "限り"
                && token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "非自立")
        }
    }

    // Match だ or です auxiliary
    #[derive(Debug)]
    struct DaDesuMatcher;
    impl Matcher for DaDesuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|p| p == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "基本形")
                && ((token.surface == "だ" && token.base_form == "だ")
                    || (token.surface == "です" && token.base_form == "です"))
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(AdjectiveMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NaCopulaMatcher)))),
        TokenMatcher::Custom(Arc::new(KagiriMatcher)),
        TokenMatcher::Custom(Arc::new(DaDesuMatcher)),
    ]
}

// Pattern: はおろか
pub fn haoroka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: めく・めいた
pub fn meku_u30fb_meita() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: といわず
pub fn toiwazu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にもほどがある
pub fn nimohodogaaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にもまして
pub fn nimomashite() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: まくる
pub fn makuru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: わ〜わ（で）
pub fn wa_u301c_wa_uff08_de_uff09() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: どうにか
pub fn dounika() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: や否や
pub fn yainaya() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 次第です (because, the reason is)
// Structures: Verb + 次第 + です
pub fn shidaidesu() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match 次第 as noun
    #[derive(Debug)]
    struct ShidaiMatcher;
    impl super::Matcher for ShidaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "次第"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    // Match です auxiliary
    #[derive(Debug)]
    struct DesuMatcher;
    impl super::Matcher for DesuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "です"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(ShidaiMatcher)),
        TokenMatcher::Custom(Arc::new(DesuMatcher)),
    ]
}

// Pattern: というところ
pub fn toiutokoro() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
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
    impl Matcher for TariMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "たり"
                && token.base_form == "たり"
                && (
                    // Case 1: たり as auxiliary verb (with counter words)
                    (token.pos.first().is_some_and(|p| p == "助動詞")
                        && token.features.get(4).is_some_and(|f| f == "文語・ナリ")
                        && token.features.get(5).is_some_and(|f| f == "基本形"))
                    ||
                    // Case 2: たり as parallel particle (with adverbs like 少し)
                    (token.pos.first().is_some_and(|p| p == "助詞")
                        && token.pos.get(1).is_some_and(|p| p == "並立助詞"))
                )
        }
    }

    // Match と particle (quotation/comparison)
    #[derive(Debug)]
    struct ToQuoteMatcher;
    impl Matcher for ToQuoteMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "と"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
                && token.pos.get(2).is_some_and(|p| p == "引用")
        }
    }

    // Match も particle (binding particle)
    #[derive(Debug)]
    struct MoBindingMatcher;
    impl Matcher for MoBindingMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "も"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "係助詞")
        }
    }

    vec![
        TokenMatcher::Any, // Any word (counter, quantifier, noun, etc.)
        TokenMatcher::Custom(Arc::new(TariMatcher)),
        TokenMatcher::Custom(Arc::new(ToQuoteMatcher)),
        TokenMatcher::Custom(Arc::new(MoBindingMatcher)),
    ]
}

// Pattern: ったらない・といったらない
pub fn ttaranai_u30fb_toittaranai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に照らして・に照らすと
pub fn niterashite_u30fb_niterasuto() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: とあれば
pub fn toareba() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: さぞ (you must be very, I dare say that)
// Structures: さぞ・さぞや・さぞかし + Phrase + だろう/でしょう
pub fn sazo() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    #[derive(Debug)]
    struct SazoMatcher;
    impl super::Matcher for SazoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "副詞")
                && (token.base_form == "さぞ"
                    || token.base_form == "さぞや"
                    || token.base_form == "さぞかし")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(SazoMatcher))]
}

// Pattern: ときたら
pub fn tokitara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: びる
pub fn biru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にしたところで
pub fn nishitatokorode() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～ばこそ
pub fn uff5e_bakoso() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ても差し支えない
pub fn temosashitsukaenai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: には及ばない①
pub fn nihaoyobanai_u2460() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に即して
pub fn nisokushite() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ないまでも
pub fn naimademo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: をよそに
pub fn woyosoni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に限ったことではない
pub fn nikagittakotodehanai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: とは比べものにならない
pub fn tohakurabemononinaranai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: まじき
pub fn majiki() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: の至り
pub fn noitari() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に恥じない
pub fn nihajinai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ずじまい (end up not doing)
// Structures: Verb［ない］+ ず + じまい
pub fn zujimai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct ZuAuxiliaryMatcher;
    impl super::Matcher for ZuAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ず"
                && token.base_form == "ぬ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    #[derive(Debug)]
    struct JimaiMatcher;
    impl super::Matcher for JimaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "じまい"
                && token.base_form == "じまい"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
        }
    }

    // Match verb in 未然形 or 未然ヌ接続 (for する verbs)
    #[derive(Debug)]
    struct MizenVerbMatcher;
    impl super::Matcher for MizenVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|form| {
                    form == "未然形" || form == "未然ヌ接続"
                })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(MizenVerbMatcher)),
        TokenMatcher::Custom(Arc::new(ZuAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(JimaiMatcher)),
    ]
}

// Pattern: に言わせれば・に言わせると・に言わせたら
pub fn niiwasereba_u30fb_niiwaseruto_u30fb_niiwasetara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ったら・といったら
pub fn ttara_u30fb_toittara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: こととて (due to / because of)
// Structures: Verb + こととて, Verb[ぬ] + こととて, Noun + の + こととて
pub fn kototote() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match こと (名詞/非自立/一般)
    #[derive(Debug)]
    struct KotoMatcher;
    impl Matcher for KotoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Match と (助詞/格助詞/引用)
    #[derive(Debug)]
    struct ToParticleMatcher;
    impl Matcher for ToParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match て (助詞/接続助詞)
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl Matcher for TeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Stop conditions to prevent capturing sentence-boundary particles
    // But allow の (for noun patterns) and don't stop at adverbs/nouns
    #[derive(Debug)]
    struct StopAtSentenceBoundary;
    impl Matcher for StopAtSentenceBoundary {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Stop if we hit case particles like に, を, で, etc. (but NOT の)
            if token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.surface != "の"
            {
                return true; // This IS a stop condition
            }
            // Stop if we hit topic/binding particles は, も
            if token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
            {
                return true; // This IS a stop condition
            }
            false // Not a stop condition, continue
        }
    }

    vec![
        TokenMatcher::Wildcard {
            min: 1,
            max: 2,
            stop_conditions: vec![TokenMatcher::Custom(Arc::new(StopAtSentenceBoundary))],
        },
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ずくめ"
                && token.base_form == "ずくめ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(ZukumeMatcher)),
    ]
}

// Pattern: には及ばない②
pub fn nihaoyobanai_u2461() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: とは言うものの
pub fn tohaiumonono() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: が早いか (as soon as, no sooner than)
// Structures: Verb[dictionary] + が + 早い + か
//            Verb[連用タ接続] + た + が + 早い + か
pub fn gahayaika() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match verbs in dictionary form OR past auxiliary た/だ
    #[derive(Debug)]
    struct VerbOrPastMatcher;
    impl Matcher for VerbOrPastMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Dictionary form verb
            (token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "基本形"))
            // OR past auxiliary た/だ
            || (token.pos.first().is_some_and(|pos| pos == "助動詞")
                && (token.base_form == "た" || token.base_form == "だ"))
        }
    }

    // Match が as 助詞/接続助詞 (conjunctive particle)
    #[derive(Debug)]
    struct GaConjunctionMatcher;
    impl Matcher for GaConjunctionMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "が"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Match 早い as adjective in basic form
    #[derive(Debug)]
    struct HayaiMatcher;
    impl Matcher for HayaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "早い"
                && token.base_form == "早い"
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "基本形")
        }
    }

    // Match か as adverbial particle
    #[derive(Debug)]
    struct KaAdverbialMatcher;
    impl Matcher for KaAdverbialMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "か"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞／並立助詞／終助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbOrPastMatcher)),
        TokenMatcher::Custom(Arc::new(GaConjunctionMatcher)),
        TokenMatcher::Custom(Arc::new(HayaiMatcher)),
        TokenMatcher::Custom(Arc::new(KaAdverbialMatcher)),
    ]
}

// Pattern: に難くない
pub fn nikatakunai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ならいざ知らず
pub fn naraizashirazu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: を禁じ得ない
pub fn wokinjienai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にかこつけて
pub fn nikakotsukete() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ようによっては
pub fn youniyotteha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: べくもない
pub fn bekumonai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: と来たら
pub fn tokitara_2() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ものとして
pub fn monotoshite() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: を前提に
pub fn wozenteini() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ずにはすまない (won't get away without doing / have no choice but to do)
// Structures: Verb[ない] + ず + には + すまない
pub fn zunihasumanai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match verb in mizenkei (negative stem) form for ず
    #[derive(Debug)]
    struct MizenkeiVerbMatcher;
    impl Matcher for MizenkeiVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            if token.pos.first().is_none_or(|pos| pos != "動詞") {
                return false;
            }
            // Match verbs in 未然形, 未然ヌ接続, or 未然レル接続
            token.features.get(5).is_some_and(|f| {
                f == "未然形" || f == "未然ヌ接続" || f == "未然レル接続"
            })
        }
    }

    // Match ず (classical negative auxiliary)
    #[derive(Debug)]
    struct ZuMatcher;
    impl Matcher for ZuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ず"
                && token.base_form == "ぬ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(4).is_some_and(|f| f == "特殊・ヌ")
        }
    }

    // Match ない (negative auxiliary)
    #[derive(Debug)]
    struct NaiMatcher;
    impl Matcher for NaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(4).is_some_and(|f| f == "特殊・ナイ")
        }
    }

    // Match すま (from すむ)
    #[derive(Debug)]
    struct SumaMatcher;
    impl Matcher for SumaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "すま"
                && token.base_form == "すむ"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然形")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(MizenkeiVerbMatcher)),
        TokenMatcher::Custom(Arc::new(ZuMatcher)),
        TokenMatcher::Surface("に"),
        TokenMatcher::Surface("は"),
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
    impl Matcher for MizenkeiVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            if token.pos.first().is_none_or(|pos| pos != "動詞") {
                return false;
            }
            // Match verbs in 未然形
            token.features.get(5).is_some_and(|f| f == "未然形")
        }
    }

    // Match ない (negative auxiliary)
    #[derive(Debug)]
    struct NaiMatcher;
    impl Matcher for NaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(4).is_some_and(|f| f == "特殊・ナイ")
        }
    }

    // Match で (from だ or as conjunctive particle)
    // Handles two tokenization variants:
    // 1. で (助動詞, base=だ) - copula form
    // 2. で (助詞/接続助詞, base=で) - conjunctive particle
    #[derive(Debug)]
    struct DeMatcher;
    impl Matcher for DeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            if token.surface != "で" {
                return false;
            }
            // Case 1: で as copula auxiliary (助動詞, base=だ)
            if token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "だ"
                && token.features.get(4).is_some_and(|f| f == "特殊・ダ")
            {
                return true;
            }
            // Case 2: で as conjunctive particle (助詞/接続助詞, base=で)
            if token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
                && token.base_form == "で"
            {
                return true;
            }
            false
        }
    }

    // Match すま (from すむ)
    #[derive(Debug)]
    struct SumaMatcher;
    impl Matcher for SumaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "すま"
                && token.base_form == "すむ"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然形")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(MizenkeiVerbMatcher)),
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
        TokenMatcher::Custom(Arc::new(DeMatcher)),
        TokenMatcher::Surface("は"),
        TokenMatcher::Custom(Arc::new(SumaMatcher)),
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
    ]
}

// Pattern: に堪えない
pub fn nikotaenai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 始末だ
pub fn shimatsuda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ものなら②
pub fn mononara_u2461() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にひきかえ
pub fn nihikikae() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: それまでだ
pub fn soremadeda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: といおうか
pub fn toiouka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ずにはおかない (will certainly do / will not fail to do)
// Structure: Verb[ない-stem] + ず + には + おかない
pub fn zunihaokanai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match verb in mizenkei (negative stem) form for ず
    #[derive(Debug)]
    struct MizenkeiVerbMatcher;
    impl Matcher for MizenkeiVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            if token.pos.first().is_none_or(|pos| pos != "動詞") {
                return false;
            }
            // Match verbs in 未然形, 未然ヌ接続, or 未然レル接続
            token.features.get(5).is_some_and(|f| {
                f == "未然形" || f == "未然ヌ接続" || f == "未然レル接続"
            })
        }
    }

    // Match ず (classical negative auxiliary)
    #[derive(Debug)]
    struct ZuMatcher;
    impl Matcher for ZuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ず"
                && token.base_form == "ぬ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(4).is_some_and(|f| f == "特殊・ヌ")
        }
    }

    // Match ない (negative auxiliary)
    #[derive(Debug)]
    struct NaiMatcher;
    impl Matcher for NaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(4).is_some_and(|f| f == "特殊・ナイ")
        }
    }

    // Match おか (from おく)
    #[derive(Debug)]
    struct OkaMatcher;
    impl Matcher for OkaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "おか"
                && token.base_form == "おく"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然形")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(MizenkeiVerbMatcher)),
        TokenMatcher::Custom(Arc::new(ZuMatcher)),
        TokenMatcher::Surface("に"),
        TokenMatcher::Surface("は"),
        TokenMatcher::Custom(Arc::new(OkaMatcher)),
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
    ]
}

// Pattern: ないではおかない (will certainly do / will not fail to do - ないでは form)
// Structure: Verb[ない] + では + おかない
pub fn naidewaokanai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match verb in mizenkei (negative stem) form
    #[derive(Debug)]
    struct MizenkeiVerbMatcher;
    impl Matcher for MizenkeiVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            if token.pos.first().is_none_or(|pos| pos != "動詞") {
                return false;
            }
            // Match verbs in 未然形
            token.features.get(5).is_some_and(|f| f == "未然形")
        }
    }

    // Match ない (negative auxiliary)
    #[derive(Debug)]
    struct NaiMatcher;
    impl Matcher for NaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(4).is_some_and(|f| f == "特殊・ナイ")
        }
    }

    // Match で (from だ)
    #[derive(Debug)]
    struct DeMatcher;
    impl Matcher for DeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "で"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(4).is_some_and(|f| f == "特殊・ダ")
        }
    }

    // Match おか (from おく)
    #[derive(Debug)]
    struct OkaMatcher;
    impl Matcher for OkaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "おか"
                && token.base_form == "おく"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然形")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(MizenkeiVerbMatcher)),
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
        TokenMatcher::Custom(Arc::new(DeMatcher)),
        TokenMatcher::Surface("は"),
        TokenMatcher::Custom(Arc::new(OkaMatcher)),
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
    ]
}

// Pattern: を限りに
pub fn wokagirini() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: てはかなわない
pub fn tehakanawanai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: かたがた (in addition to, along with, while doing)
// Structures: Noun + かたがた
pub fn katagata() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct KatagataMatcher;
    impl Matcher for KatagataMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "かたがた"
                && token.base_form == "かたがた"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(KatagataMatcher)),
    ]
}

// Pattern: を余儀なくさせる
pub fn woyoginakusaseru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～てやる
pub fn uff5e_teyaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ただ〜のみ
pub fn tada_u301c_nomi() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ものとする
pub fn monotosuru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: との (quotation particle + nominalizer)
// Meaning: "that" (marks a statement as quoted information)
// Structures: Verb/Adj/Noun + (だ) + との + Noun
pub fn tono() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match と as quotation particle
    #[derive(Debug)]
    struct ToQuotationMatcher;
    impl Matcher for ToQuotationMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "引用")
        }
    }

    // Match の as nominalizer/connector particle (助詞/連体化)
    #[derive(Debug)]
    struct NoNominalizerMatcher;
    impl Matcher for NoNominalizerMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(ToQuotationMatcher)),
        TokenMatcher::Custom(Arc::new(NoNominalizerMatcher)),
        super::noun_matcher(),
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "以前"
                && token.base_form == "以前"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副詞可能")
        }
    }

    // Match に or の particle after 以前
    #[derive(Debug)]
    struct NiNoParticleMatcher;
    impl super::Matcher for NiNoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "に" || token.surface == "の")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
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
    impl Matcher for ToParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match も as 係助詞
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Match あろ (ある in 未然ウ接続 form)
    #[derive(Debug)]
    struct AroMatcher;
    impl Matcher for AroMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "あろ"
                && token.base_form == "ある"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Match う (volitional auxiliary)
    #[derive(Debug)]
    struct UMatcher;
    impl Matcher for UMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "う"
                && token.base_form == "う"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Match noun (もの/方/人 etc.) - typically 名詞/一般 or 名詞/非自立
    #[derive(Debug)]
    struct NounAfterMatcher;
    impl Matcher for NounAfterMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    // Match が as 格助詞
    #[derive(Debug)]
    struct GaParticleMatcher;
    impl Matcher for GaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "が"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(ToParticleMatcher)),
        TokenMatcher::Custom(Arc::new(MoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(AroMatcher)),
        TokenMatcher::Custom(Arc::new(UMatcher)),
        TokenMatcher::Custom(Arc::new(NounAfterMatcher)),
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こそ"
                && token.base_form == "こそ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Match すれ (tokenized as すれる verb in 連用形)
    #[derive(Debug)]
    struct SureMatcher;
    impl super::Matcher for SureMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "すれ"
                && token.base_form == "すれる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token
                    .features
                    .get(5)
                    .is_some_and(|f| f.contains("連用形"))
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
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "並み"
                && token.base_form == "並み"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(NamiMatcher)),
    ]
}

// Pattern: に先駆けて (ahead of, in advance of)
// Structures: Noun + に + 先駆け + (て)
pub fn nisakigakete() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match 先駆ける verb in 連用形
    #[derive(Debug)]
    struct SakigakeMatcher;
    impl Matcher for SakigakeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "先駆ける"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token
                    .features
                    .get(5)
                    .is_some_and(|form| form == "連用形")
        }
    }

    // Match て as 接続助詞
    #[derive(Debug)]
    struct TeFormMatcher;
    impl Matcher for TeFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て" && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Surface("に"),
        TokenMatcher::Custom(Arc::new(SakigakeMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(TeFormMatcher)))),
    ]
}

// Pattern: を機に (taking advantage of, on the occasion of)
// Structures: Verb[た] + の + を機に or Noun + を機に
pub fn wokini() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match を as 格助詞
    #[derive(Debug)]
    struct WoParticleMatcher;
    impl Matcher for WoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "を"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match 機 as noun
    #[derive(Debug)]
    struct KiNounMatcher;
    impl Matcher for KiNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "機"
                && token.base_form == "機"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
        }
    }

    // Match に as 格助詞
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match の as nominalizer (名詞/非自立)
    #[derive(Debug)]
    struct NoNominalizerMatcher;
    impl Matcher for NoNominalizerMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.base_form == "の"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    vec![
        TokenMatcher::Any, // Noun or (Verb + た)
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NoNominalizerMatcher,
        )))), // Optional の for verb nominalization
        TokenMatcher::Custom(Arc::new(WoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(KiNounMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
    ]
}
