use crate::pattern_matcher::TokenMatcher;
use super::Matcher;
use std::sync::Arc;

// Pattern: 得る・得る (to be possible, can do)
// Structures: Verb[stem] + える/うる
pub fn eru_u30fb_eru() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct VerbStemMatcher;
    impl Matcher for VerbStemMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形")
        }
    }

    #[derive(Debug)]
    struct EruUruMatcher;
    impl Matcher for EruUruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match える (base_form="える", 一段) or うる (base_form="うる", 一段・得ル)
            // Must be non-independent verb (非自立)
            (token.base_form == "える" || token.base_form == "うる")
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbStemMatcher)),
        TokenMatcher::Custom(Arc::new(EruUruMatcher)),
    ]
}

// Pattern: 〜得ない (cannot, impossible)
// Structures: Verb[stem] + えない / えません
pub fn u301c_enai() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct VerbStemMatcher;
    impl Matcher for VerbStemMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形")
        }
    }

    #[derive(Debug)]
    struct EruVerbMatcher;
    impl Matcher for EruVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "え"
                && token.base_form == "える"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    #[derive(Debug)]
    struct NaiOrMasenMatcher;
    impl Matcher for NaiOrMasenMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match ない (助動詞)
            (token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞"))
            // OR match ませ (助動詞, ます base form)
            || (token.surface == "ませ"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞"))
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbStemMatcher)),
        TokenMatcher::Custom(Arc::new(EruVerbMatcher)),
        TokenMatcher::Custom(Arc::new(NaiOrMasenMatcher)),
        // Optional ん for ません form
        TokenMatcher::Optional(Box::new(TokenMatcher::Surface("ん"))),
    ]
}

// Pattern: ざるを得ない (cannot help but / have no choice but to)
// Structures: Verb[未然形] + ざる + を + 得 + ない/ません
pub fn zaruwoenai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match verb in 未然形 (negative form stem)
    #[derive(Debug)]
    struct MizenVerbMatcher;
    impl Matcher for MizenVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| {
                    f == "未然形" || f == "未然レル接続" || f == "未然ヌ接続"
                })
        }
    }

    // Match ざる (classical negative auxiliary)
    #[derive(Debug)]
    struct ZaruMatcher;
    impl Matcher for ZaruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ざる"
                && token.base_form == "ぬ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Match を particle
    #[derive(Debug)]
    struct WoParticleMatcher;
    impl Matcher for WoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "を"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    // Match 得 (える) verb
    #[derive(Debug)]
    struct EruVerbMatcher;
    impl Matcher for EruVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "得る"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    // Match ない OR ませ auxiliary verb
    #[derive(Debug)]
    struct NaiMaseMatcher;
    impl Matcher for NaiMaseMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "助動詞")
                && ((token.surface == "ない" && token.base_form == "ない")
                    || (token.surface == "ませ" && token.base_form == "ます"))
        }
    }

    // Match ん auxiliary verb (for ません)
    #[derive(Debug)]
    struct NMatcher;
    impl Matcher for NMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ん"
                && token.base_form == "ん"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(MizenVerbMatcher)),
        TokenMatcher::Custom(Arc::new(ZaruMatcher)),
        TokenMatcher::Custom(Arc::new(WoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(EruVerbMatcher)),
        TokenMatcher::Custom(Arc::new(NaiMaseMatcher)),
        // Don't use Optional - it causes issues. We'll just match up to ない or ませ
        // For ません, a separate pattern detection would be needed, but for now this works.
    ]
}

// Pattern: ～ざる (attributive form of classical negative auxiliary ず)
// Structures: Verb[未然形] + ざる
pub fn uff5e_zaru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ざる (classical negative auxiliary, attributive form)
    #[derive(Debug)]
    struct ZaruMatcher;
    impl super::Matcher for ZaruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ざる"
                && token.base_form == "ぬ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "体言接続")
        }
    }

    vec![
        TokenMatcher::verb_with_form("未然形"),
        TokenMatcher::Custom(Arc::new(ZaruMatcher)),
    ]
}

// Pattern: つもりで (with the intention of / as if)
// Structures: [Verb/Adjective/Noun] + つもりで
pub fn tsumoride() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match つもり (noun: intention)
    #[derive(Debug)]
    struct TsumoriMatcher;
    impl super::Matcher for TsumoriMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "つもり"
                && token.base_form == "つもり"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    // Match で (particle)
    #[derive(Debug)]
    struct DeMatcher;
    impl super::Matcher for DeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "で"
                && token.base_form == "で"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        TokenMatcher::Optional(Box::new(TokenMatcher::Any)),
        TokenMatcher::Custom(Arc::new(TsumoriMatcher)),
        TokenMatcher::Custom(Arc::new(DeMatcher)),
    ]
}

// Pattern: どうせ (in any case, anyway)
// Structure: どうせ (adverb)
pub fn douse() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    #[derive(Debug)]
    struct DouseMatcher;
    impl super::Matcher for DouseMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "どうせ"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(DouseMatcher))]
}

// Pattern: せめて (at least)
// Structure: せめて (adverb)
pub fn semete() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    #[derive(Debug)]
    struct SemeteMatcher;
    impl super::Matcher for SemeteMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "せめて"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(SemeteMatcher))]
}

// Pattern: どうやら (apparently, it seems like)
// Structures: どうやら
pub fn douyara() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct DouyaraMatcher;

    impl super::Matcher for DouyaraMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "どうやら"
                && token.base_form == "どうやら"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(DouyaraMatcher))]
}

// Pattern: なにやら (something or other, some kind of, for some reason)
// Structures: 何（なに）やら
pub fn naniyara() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct NaniyaraMatcher;

    impl super::Matcher for NaniyaraMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "何やら"
                && token.base_form == "何やら"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(NaniyaraMatcher))]
}

// Pattern: よりほかない (have no choice but / nothing but)
// Structure: Verb + より + ほか + (は/に/には) + ない
pub fn yorihokanai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct YoriMatcher;
    impl Matcher for YoriMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "より"
                && token.base_form == "より"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    #[derive(Debug)]
    struct HokaMatcher;
    impl Matcher for HokaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ほか"
                && token.base_form == "ほか"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副詞可能")
        }
    }

    #[derive(Debug)]
    struct WaParticleMatcher;
    impl Matcher for WaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match ない in any conjugation form (ない, なかった, etc.)
    #[derive(Debug)]
    struct NaiMatcher;
    impl Matcher for NaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "ない"
                && (token.pos.first().is_some_and(|pos| pos == "形容詞")
                    || token.pos.first().is_some_and(|pos| pos == "助動詞"))
        }
    }

    vec![
        TokenMatcher::verb_with_form("基本形"),
        TokenMatcher::Custom(Arc::new(YoriMatcher)),
        TokenMatcher::Custom(Arc::new(HokaMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(WaParticleMatcher)))),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NiParticleMatcher)))),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(WaParticleMatcher)))),
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
    ]
}

// Pattern: 確かに (certainly, surely)
// Structure: たしかに (adverb) OR 確か (na-adjective stem) + に (adverbial particle)
pub fn tashikani() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for に as adverbial particle
    #[derive(Debug)]
    struct NiAdverbialMatcher;
    impl super::Matcher for NiAdverbialMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副詞化")
        }
    }

    // Combined matcher for たしかに (adverb) OR 確か (na-adjective stem)
    #[derive(Debug)]
    struct TashikaniCombinedMatcher;
    impl super::Matcher for TashikaniCombinedMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match たしかに as adverb
            (token.surface == "たしかに"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般"))
            ||
            // Match 確か as na-adjective stem (will be followed by に)
            (token.surface == "確か"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹"))
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(TashikaniCombinedMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NiAdverbialMatcher)))),
    ]
}

// Pattern: 一応 ① (just in case, just to be sure)
// Pattern: 一応 ② (more or less, for the time being, tentatively)
// Structures: 一応 (single adverb token)
// Note: Both patterns are structurally identical - meaning differs by context
fn ichiou_matcher() -> TokenMatcher {
    use std::sync::Arc;
    #[derive(Debug)]
    struct IchiouMatcher;
    impl super::Matcher for IchiouMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "一応"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
        }
    }
    TokenMatcher::Custom(Arc::new(IchiouMatcher))
}

pub fn ichiou_u2460() -> Vec<TokenMatcher> {
    vec![ichiou_matcher()]
}

pub fn ichiou_u2461() -> Vec<TokenMatcher> {
    vec![ichiou_matcher()]
}

// Pattern: に相違ない (without a doubt, no mistaking)
// Structure: Verb/Adj/Noun/(から) + に + 相違 + ない
pub fn nisouinai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.base_form == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                    || token.pos.get(1).is_some_and(|pos| pos == "副詞化"))
        }
    }

    #[derive(Debug)]
    struct SouiMatcher;
    impl Matcher for SouiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "相違"
                && token.base_form == "相違"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "サ変接続")
        }
    }

    #[derive(Debug)]
    struct NaiAdjMatcher;
    impl Matcher for NaiAdjMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "ない" && token.pos.first().is_some_and(|pos| pos == "形容詞")
        }
    }

    vec![
        TokenMatcher::Optional(Box::new(TokenMatcher::Any)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(SouiMatcher)),
        TokenMatcher::Custom(Arc::new(NaiAdjMatcher)),
    ]
}

// Pattern: 万が一 (in the unlikely event, just in case)
// Structures: 万が一/万一 (both single tokens, different POS)
pub fn mangaichi() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    #[derive(Debug)]
    struct MangaichiMatcher;
    impl super::Matcher for MangaichiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match 万が一 as 名詞/一般 or 万一 as 副詞/助詞類接続
            (token.surface == "万が一" && token.pos.first().is_some_and(|pos| pos == "名詞"))
                || (token.surface == "万一" && token.pos.first().is_some_and(|pos| pos == "副詞"))
        }
    }
    vec![TokenMatcher::Custom(Arc::new(MangaichiMatcher))]
}

// Pattern: ようがない・ようもない (there is no way to / impossible to)
// Structures: Verb[stem] + よう + が/も + ない/ありません
//            する Verb + (の) + しよう + が/も + ない/ありません
pub fn youganai_u30fb_youmonai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for よう (noun suffix) or しよう (noun)
    #[derive(Debug)]
    struct YouShiyouMatcher;
    impl Matcher for YouShiyouMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|p| p == "名詞")
                && ((token.surface == "よう"
                    && token.base_form == "よう"
                    && token.pos.get(1).is_some_and(|p| p == "接尾"))
                    || (token.surface == "しよう"
                        && token.base_form == "しよう"
                        && token.pos.get(1).is_some_and(|p| p == "一般")))
        }
    }

    // Matcher for が or も particle
    #[derive(Debug)]
    struct GaMoParticleMatcher;
    impl Matcher for GaMoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "が" || token.surface == "も")
                && token.pos.first().is_some_and(|p| p == "助詞")
        }
    }

    // Matcher for ない (adjective or auxiliary) or ありません
    #[derive(Debug)]
    struct NaiArimasenMatcher;
    impl Matcher for NaiArimasenMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // ない as adjective or auxiliary
            (token.base_form == "ない"
                && (token.pos.first().is_some_and(|p| p == "形容詞")
                    || token.pos.first().is_some_and(|p| p == "助動詞")))
            // or ある verb (for ありません)
            || (token.base_form == "ある"
                && token.pos.first().is_some_and(|p| p == "動詞"))
        }
    }

    vec![
        TokenMatcher::Any, // Verb stem (連用形) or noun before しよう, or の particle
        TokenMatcher::Custom(Arc::new(YouShiyouMatcher)),
        TokenMatcher::Custom(Arc::new(GaMoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(NaiArimasenMatcher)),
    ]
}

// Pattern: にほかならない (nothing but, simply)
// Structures: Noun + に + ほかなら + ない/ぬ/なりません
pub fn nihokanaranai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for に particle (格助詞 or 副詞化)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.base_form == "に"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && (token.pos.get(1).is_some_and(|p| p == "格助詞")
                    || token.pos.get(1).is_some_and(|p| p == "副詞化"))
        }
    }

    // Matcher for ほかなら (compound verb) or ほか (noun)
    #[derive(Debug)]
    struct HokaMatcher;
    impl Matcher for HokaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Compound verb: ほかなら (base='ほかなる', 未然形)
            (token.surface == "ほかなら"
                && token.base_form == "ほかなる"
                && token.pos.first().is_some_and(|p| p == "動詞"))
            ||
            // Noun: ほか (for polite form with なり)
            (token.surface == "ほか"
                && token.base_form == "ほか"
                && token.pos.first().is_some_and(|p| p == "名詞"))
        }
    }

    // Matcher for negative auxiliaries: ない (助動詞), ぬ (助動詞), or なる verb (for polite form)
    #[derive(Debug)]
    struct NegativeOrNaruMatcher;
    impl Matcher for NegativeOrNaruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // ない auxiliary (特殊・ナイ)
            (token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|p| p == "助動詞"))
            ||
            // ぬ auxiliary (特殊・ヌ, formal negative)
            (token.surface == "ぬ"
                && token.base_form == "ぬ"
                && token.pos.first().is_some_and(|p| p == "助動詞"))
            ||
            // なり (verb, for polite forms like なりません)
            (token.base_form == "なる"
                && token.pos.first().is_some_and(|p| p == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形"))
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(HokaMatcher)),
        TokenMatcher::Custom(Arc::new(NegativeOrNaruMatcher)),
        // Optional polite auxiliaries: ませ + ん
        TokenMatcher::Wildcard {
            min: 0,
            max: 2,
            stop_conditions: vec![],
        },
    ]
}

// Pattern: っこない (there is no chance of / impossible)
// Structures: Verb[stem] + っこない
pub fn kkonai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for verb stem OR verb+っ
    #[derive(Debug)]
    struct VerbOrVerbWithKkoMatcher;
    impl Matcher for VerbOrVerbWithKkoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|p| p == "動詞")
                && (
                    // Regular verb stem (連用形)
                    token.features.get(5).is_some_and(|f| f == "連用形")
                    ||
                    // Verb+っ compound (連用タ接続, like きっ, てっ)
                    token.features.get(5).is_some_and(|f| f == "連用タ接続")
                )
        }
    }

    // Matcher for っ alone (when verb stem is separate)
    #[derive(Debug)]
    struct KkuMatcher;
    impl Matcher for KkuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "っ"
                && token.base_form == "く"
                && token.pos.first().is_some_and(|p| p == "動詞")
                && token.pos.get(1).is_some_and(|p| p == "非自立")
        }
    }

    // Matcher for こ (tokenizes as verb base='くる')
    #[derive(Debug)]
    struct KoMatcher;
    impl Matcher for KoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こ"
                && token.base_form == "くる"
                && token.pos.first().is_some_and(|p| p == "動詞")
                && token.pos.get(1).is_some_and(|p| p == "非自立")
                && token.features.get(5).is_some_and(|f| f == "未然形")
        }
    }

    // Matcher for ない (助動詞)
    #[derive(Debug)]
    struct NaiAuxMatcher;
    impl Matcher for NaiAuxMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "ない"
                && token.pos.first().is_some_and(|p| p == "助動詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbOrVerbWithKkoMatcher)),  // Verb stem (may include っ)
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(KkuMatcher)))),  // Optional っ (if not included in verb)
        TokenMatcher::Custom(Arc::new(KoMatcher)),  // こ
        TokenMatcher::Custom(Arc::new(NaiAuxMatcher)),  // ない
    ]
}

// Pattern: それなら (if that's the case, then)
// Structure: それなら + Phrase
// Note: だったら and それだったら variants are already matched by the たら pattern
pub fn sorenara() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct SorenaraMatcher;
    impl Matcher for SorenaraMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "それなら"
                && token.base_form == "それなら"
                && token.pos.first().is_some_and(|pos| pos == "接続詞")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(SorenaraMatcher))]
}

// Pattern: ものなら① (if one could / if it were possible)
// Structure: Verb[potential] + もの + なら
// Used after potential form verbs to express hypothetical possibility
pub fn mononara_u2460() -> Vec<TokenMatcher> {
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

    #[derive(Debug)]
    struct NaraMatcher;
    impl Matcher for NaraMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "なら"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "仮定形")
        }
    }

    vec![
        TokenMatcher::Any,  // Potential form verb (できる or られる/れる form)
        TokenMatcher::Custom(Arc::new(MonoMatcher)),
        TokenMatcher::Custom(Arc::new(NaraMatcher)),
    ]
}

// Pattern: ～を～に任せる (entrust X to Y)
// Structures: Nounを + Nounに + 任せる
pub fn uff5e_wo_uff5e_nimakaseru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match を particle (object marker)
    #[derive(Debug)]
    struct WoMatcher;
    impl super::Matcher for WoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "を"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    // Match に particle (target marker)
    #[derive(Debug)]
    struct NiMatcher;
    impl super::Matcher for NiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    // Match 任せる verb in any conjugation
    #[derive(Debug)]
    struct MakaseruMatcher;
    impl super::Matcher for MakaseruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "任せる"
                && token.pos.first().is_some_and(|p| p == "動詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(WoMatcher)),
        TokenMatcher::Wildcard {
            min: 1,
            max: 1,
            stop_conditions: vec![],
        },
        TokenMatcher::Custom(Arc::new(NiMatcher)),
        TokenMatcher::Custom(Arc::new(MakaseruMatcher)),
    ]
}

// Pattern: ～を～に任せる (target-task order: に...を)
// This handles sentences where the target comes before the task, like:
// "新人君にあの重要なプレゼンを任せた" (entrusted presentation to newcomer)
pub fn uff5e_wo_uff5e_nimakaseru_reverse() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Reuse matchers from the main pattern
    #[derive(Debug)]
    struct WoMatcher;
    impl super::Matcher for WoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "を"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    #[derive(Debug)]
    struct NiMatcher;
    impl super::Matcher for NiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    #[derive(Debug)]
    struct MakaseruMatcher;
    impl super::Matcher for MakaseruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "任せる"
                && token.pos.first().is_some_and(|p| p == "動詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NiMatcher)),
        TokenMatcher::Wildcard {
            min: 4,
            max: 4,
            stop_conditions: vec![],
        },
        TokenMatcher::Custom(Arc::new(WoMatcher)),
        TokenMatcher::Custom(Arc::new(MakaseruMatcher)),
    ]
}

// Pattern: 活かす (to make good use of, to leverage)
// Structure: (Noun + を) + 活かす/生かす
// Matches the verb 活かす or 生かす in any conjugation
pub fn ikasu() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct IkasuMatcher;
    impl Matcher for IkasuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && (token.base_form == "活かす" || token.base_form == "生かす")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(IkasuMatcher))]
}

// Pattern: おおよそ (approximately, roughly)
// Structure: おおよそ/およそ + (Noun/Number)
pub fn ooyoso() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct OoyosoMatcher;
    impl super::Matcher for OoyosoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match both おおよそ (副詞) and およそ (接頭詞/副詞)
            (token.surface == "おおよそ" && token.pos.first().is_some_and(|pos| pos == "副詞"))
                || (token.surface == "およそ"
                    && (token.pos.first().is_some_and(|pos| pos == "接頭詞")
                        || token.pos.first().is_some_and(|pos| pos == "副詞")))
        }
    }

    vec![TokenMatcher::Custom(Arc::new(OoyosoMatcher))]
}

// Pattern: まい (won't, intend not to, probably not)
// Structures: Verb + まい, Verb[stem] + まい, Verb(Polite) + まい
pub fn mai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct MaiMatcher;
    impl super::Matcher for MaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "まい"
                && token.base_form == "まい"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    #[derive(Debug)]
    struct VerbMatcher;
    impl super::Matcher for VerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    #[derive(Debug)]
    struct MasuMatcher;
    impl super::Matcher for MasuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ます"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(MasuMatcher)))),
        TokenMatcher::Custom(Arc::new(MaiMatcher)),
    ]
}

// Pattern: From the standpoint of (Noun + 上じょう)
// Structures: Noun + 上（じょう）
pub fn ue() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct UeSuffixMatcher;
    impl super::Matcher for UeSuffixMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "上"
                && token.base_form == "上"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(UeSuffixMatcher)),
    ]
}

// Pattern: In addition to / as well as (X + 上に)
// Structures: Verb/い-Adj + 上（うえ）(に), な-Adj + な + 上, Noun + の + 上, X + である + 上
pub fn ueni() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matches verbs, adjectives, or particles/auxiliaries that can precede うえ
    #[derive(Debug)]
    struct UePreMatcher;
    impl super::Matcher for UePreMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            if let Some(first_pos) = token.pos.first() {
                match first_pos.as_str() {
                    "動詞" => true, // Verbs in basic form
                    "形容詞" => true, // い-Adjectives in basic form
                    "助動詞" => {
                        // な (for な-adjectives) or ある (for である)
                        token.surface == "な" || token.surface == "ある"
                    }
                    "助詞" => {
                        // の (for nouns)
                        token.surface == "の" && token.pos.get(1).is_some_and(|p| p == "連体化")
                    }
                    _ => false,
                }
            } else {
                false
            }
        }
    }

    #[derive(Debug)]
    struct UeNounMatcher;
    impl super::Matcher for UeNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "うえ"
                && token.base_form == "うえ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(UePreMatcher)),
        TokenMatcher::Custom(Arc::new(UeNounMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NiParticleMatcher)))),
    ]
}

// Pattern: 以上 ② (since, now that, as long as)
// Structures: Verb + 以上, い-Adj + 以上, な-Adj/Noun + である + 以上, (optional は)
pub fn ijou_u2461() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matches any token that can precede 以上② (Verb, い-Adj, or ある from である)
    #[derive(Debug)]
    struct IjouPreMatcher;
    impl super::Matcher for IjouPreMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            if let Some(first_pos) = token.pos.first() {
                match first_pos.as_str() {
                    "動詞" => true, // Any verb form
                    "形容詞" => true, // い-Adjectives
                    "助動詞" => {
                        // ある (from である), た (past tense), etc.
                        token.base_form == "ある" || token.base_form == "た" || token.base_form == "だ"
                    }
                    _ => false,
                }
            } else {
                false
            }
        }
    }

    // Matches 以上 (名詞/非自立/副詞可能)
    #[derive(Debug)]
    struct IjouMatcher;
    impl super::Matcher for IjouMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "以上"
                && token.base_form == "以上"
                && token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "非自立")
        }
    }

    // Matches は (係助詞) - optional
    #[derive(Debug)]
    struct WaMatcher;
    impl super::Matcher for WaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "係助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(IjouPreMatcher)),
        TokenMatcher::Custom(Arc::new(IjouMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(WaMatcher)))),
    ]
}

// Pattern: 以上に (more than, even more than)
// Structures: Verb/Adj/Noun + 以上に, 以上 + の + Noun
pub fn ijouni() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matches any token that can precede 以上に (Verb, い-Adj, な-Adj, Noun, た auxiliary)
    #[derive(Debug)]
    struct IjouNiPreMatcher;
    impl super::Matcher for IjouNiPreMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            if let Some(first_pos) = token.pos.first() {
                match first_pos.as_str() {
                    "動詞" => true, // Any verb form
                    "形容詞" => true, // い-Adjectives
                    "名詞" => true, // Nouns and な-Adjective stems (形容動詞語幹)
                    "助動詞" => {
                        // た (past tense auxiliary)
                        token.base_form == "た"
                    }
                    _ => false,
                }
            } else {
                false
            }
        }
    }

    // Matches 以上 (名詞/非自立/副詞可能)
    #[derive(Debug)]
    struct IjouMatcher;
    impl super::Matcher for IjouMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "以上"
                && token.base_form == "以上"
                && token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "非自立")
        }
    }

    // Matches に (格助詞) or の (連体化)
    #[derive(Debug)]
    struct NiNoMatcher;
    impl super::Matcher for NiNoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            if token.pos.first().is_some_and(|p| p == "助詞") {
                if token.surface == "に" {
                    token.pos.get(1).is_some_and(|p| p == "格助詞")
                } else if token.surface == "の" {
                    token.pos.get(1).is_some_and(|p| p == "連体化")
                } else {
                    false
                }
            } else {
                false
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(IjouNiPreMatcher)),
        TokenMatcher::Custom(Arc::new(IjouMatcher)),
        TokenMatcher::Custom(Arc::new(NiNoMatcher)),
    ]
}

// Pattern: 途中に・途中で (on the way, partway through, in the middle of)
// Structures: Verb[る] + 途中 + に/で, Noun + の + 途中 + に/で
pub fn tochuuni_u30fb_tochuude() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match 途中 (noun, adverbial)
    #[derive(Debug)]
    struct TochuuMatcher;
    impl super::Matcher for TochuuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "途中"
                && token.base_form == "途中"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副詞可能")
        }
    }

    // Match に or で particle
    #[derive(Debug)]
    struct NiDeMatcher;
    impl super::Matcher for NiDeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "に" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match の particle (for Noun + の structure)
    #[derive(Debug)]
    struct NoParticleMatcher;
    impl super::Matcher for NoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
        }
    }

    vec![
        TokenMatcher::Any, // Verb or Noun
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NoParticleMatcher)))), // Optional の
        TokenMatcher::Custom(Arc::new(TochuuMatcher)),
        TokenMatcher::Custom(Arc::new(NiDeMatcher)),
    ]
}

// Pattern: Doing B in/on/inside A (Noun + の + 中を)
// Structures: Noun + の + 中（なか）を
pub fn nakawo() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct NakaNounMatcher;
    impl super::Matcher for NakaNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "中"
                && token.base_form == "中"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

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
    struct WoParticleMatcher;
    impl super::Matcher for WoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "を"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(NakaNounMatcher)),
        TokenMatcher::Custom(Arc::new(WoParticleMatcher)),
    ]
}

// Pattern: を中心に (focused on, centered around, mainly)
// Structures: Noun + を中心に/として/にして/にする/にした/とする/とした
pub fn wochuushinni() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match を particle
    #[derive(Debug)]
    struct WoMatcher;
    impl super::Matcher for WoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "を"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match 中心 (noun)
    #[derive(Debug)]
    struct ChuushinMatcher;
    impl super::Matcher for ChuushinMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "中心"
                && token.base_form == "中心"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    // Match に or と particle, or として compound particle
    #[derive(Debug)]
    struct NiToToshiteMatcher;
    impl super::Matcher for NiToToshiteMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞"))
            || (token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞"))
            || (token.surface == "として"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞"))
        }
    }

    vec![
        TokenMatcher::Any, // Noun
        TokenMatcher::Custom(Arc::new(WoMatcher)),
        TokenMatcher::Custom(Arc::new(ChuushinMatcher)),
        TokenMatcher::Custom(Arc::new(NiToToshiteMatcher)),
        // Optionally followed by する in various forms (する, した, して)
        TokenMatcher::Optional(Box::new(TokenMatcher::specific_verb("する"))),
    ]
}

// Pattern: その上 (besides, in addition to, furthermore)
// Structures: その上 + Phrase (conjunction)
pub fn sonoue() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match その (demonstrative)
    #[derive(Debug)]
    struct SonoMatcher;
    impl super::Matcher for SonoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "その"
                && token.base_form == "その"
                && token.pos.first().is_some_and(|pos| pos == "連体詞")
        }
    }

    // Match 上 (noun, non-independent, adverbial)
    #[derive(Debug)]
    struct UeMatcher;
    impl super::Matcher for UeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "上"
                && token.base_form == "上"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                && token.pos.get(2).is_some_and(|pos| pos == "副詞可能")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(SonoMatcher)),
        TokenMatcher::Custom(Arc::new(UeMatcher)),
    ]
}

// Pattern: 上は (now that, since, as long as)
// Structures: Verb[る/た] + 上は
pub fn ueha() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct UeMatcher;
    impl Matcher for UeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "上"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                && token.pos.get(2).is_some_and(|pos| pos == "副詞可能")
        }
    }

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
        TokenMatcher::Any, // Verb (基本形 or 連用形)
        TokenMatcher::Optional(Box::new(super::past_auxiliary())), // Optional た/だ
        TokenMatcher::Custom(Arc::new(UeMatcher)),
        TokenMatcher::Custom(Arc::new(WaParticleMatcher)),
    ]
}

// Pattern: の下で (under, on the basis of)
// Structures: Noun + のもと + (で|に|ø)
pub fn noshitade() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct NoParticleMatcher;
    impl Matcher for NoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
        }
    }

    #[derive(Debug)]
    struct MotoMatcher;
    impl Matcher for MotoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match もと as either 名詞/非自立/一般 or 名詞/一般
            token.surface == "もと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "非自立")
                    || token.pos.get(1).is_some_and(|pos| pos == "一般"))
        }
    }

    #[derive(Debug)]
    struct DeNiParticleMatcher;
    impl Matcher for DeNiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "で" || token.surface == "に")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        TokenMatcher::Any, // Noun
        TokenMatcher::Custom(Arc::new(NoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(MotoMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(DeNiParticleMatcher)))),
    ]
}

// Pattern: 後(の) Noun (the rest of, what's remaining)
// Structures: あと + の + Noun | あと + Phrase | あと + Number + (Counter)
pub fn kou_no_noun() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match 後/あと as either 名詞/接尾/副詞可能 or 接頭詞/名詞接続
    #[derive(Debug)]
    struct AtoMatcher;
    impl Matcher for AtoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "後"
                && token.base_form == "後"
                && (
                    // Structure 1 & 2: 名詞/接尾/副詞可能
                    (token.pos.first().is_some_and(|pos| pos == "名詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                        && token.pos.get(2).is_some_and(|pos| pos == "副詞可能"))
                    ||
                    // Structure 3: 接頭詞/名詞接続
                    (token.pos.first().is_some_and(|pos| pos == "接頭詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "名詞接続"))
                )
        }
    }

    // Match の particle
    #[derive(Debug)]
    struct NoParticleMatcher;
    impl Matcher for NoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
        }
    }

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

    vec![
        TokenMatcher::Custom(Arc::new(AtoMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NoParticleMatcher)))),
        TokenMatcher::Any, // Noun, Phrase element, or Number
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(CounterMatcher)))), // Optional counter for numbers
    ]
}

// Pattern: 手前 (in front of, given the circumstances)
// Structures: Verb + 手前 | Noun + の + 手前
pub fn temae() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct NoParticleMatcher;
    impl Matcher for NoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
        }
    }

    #[derive(Debug)]
    struct TemaeMatcher;
    impl Matcher for TemaeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "手前"
                && token.base_form == "手前"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    vec![
        TokenMatcher::Any, // Verb or Noun
        TokenMatcher::Optional(Box::new(super::past_auxiliary())), // Optional た/だ for verbs
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NoParticleMatcher)))), // Optional の for nouns
        TokenMatcher::Custom(Arc::new(TemaeMatcher)),
    ]
}

// Pattern: を巡って (concerning, in regard to, about)
// Structures: Noun + をめぐって | Noun + を + めぐる + Noun
pub fn womegutte() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match either:
    // 1. をめぐって as single token (助詞/格助詞/連語)
    // 2. を particle (when followed by めぐる verb)
    #[derive(Debug)]
    struct WomegutteMatcher;
    impl Matcher for WomegutteMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Case 1: Single token をめぐって
            (token.surface == "をめぐって"
                && token.base_form == "をめぐって"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "連語"))
            ||
            // Case 2: を particle (will be followed by めぐる)
            (token.surface == "を"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞"))
        }
    }

    // Match めぐる verb (in various forms: めぐる, めぐって, めぐり)
    // This is optional because sometimes it's a single token をめぐって
    #[derive(Debug)]
    struct MeguruVerbMatcher;
    impl Matcher for MeguruVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "めぐる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    vec![
        TokenMatcher::Any, // Noun
        TokenMatcher::Custom(Arc::new(WomegutteMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(MeguruVerbMatcher)))),
    ]
}

// Pattern: にわたって (across, throughout, over the period of)
// Structures: Noun + にわたって/にわたる/にわたり/にわたった
pub fn niwatatte() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match either:
    // 1. Compound particle forms (にわたって/にわたる/にわたり) - single token
    // 2. に + わたる verb - multi-token form (e.g., にわたった)
    #[derive(Debug)]
    struct NiwatatteMatcher;
    impl super::Matcher for NiwatatteMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Case 1: Compound particle
            if token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "連語")
                && (token.base_form == "にわたって"
                    || token.base_form == "にわたる"
                    || token.base_form == "にわたり")
            {
                return true;
            }

            // Case 2: Particle に
            if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            {
                return true;
            }

            false
        }
    }

    // Match わたる verb (for multi-token form like にわたった)
    #[derive(Debug)]
    struct WataruVerbMatcher;
    impl super::Matcher for WataruVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "わたる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    vec![
        TokenMatcher::Any,  // Preceding noun
        TokenMatcher::Custom(Arc::new(NiwatatteMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(WataruVerbMatcher)))),
    ]
}

// Pattern: に沿って (along, in accordance with, in line with)
// Structures: Noun + にそって/にそった/にそう
pub fn nisotte() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match particle に
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match そ verb (base='そう')
    #[derive(Debug)]
    struct SouVerbMatcher;
    impl super::Matcher for SouVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "そう"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    // Match て particle or た auxiliary (for にそって/にそった forms)
    #[derive(Debug)]
    struct TeOrTaMatcher;
    impl super::Matcher for TeOrTaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match て (接続助詞)
            if token.surface == "て" && token.pos.first().is_some_and(|pos| pos == "助詞") {
                return true;
            }
            // Match た (助動詞)
            if token.surface == "た" && token.pos.first().is_some_and(|pos| pos == "助動詞") {
                return true;
            }
            false
        }
    }

    vec![
        TokenMatcher::Any,  // Preceding noun
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(SouVerbMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(TeOrTaMatcher)))),
    ]
}

// Pattern: た末・の末 (after, as a result of)
// Structures: Verb[た] + すえ (に) / Noun + の + すえ (に)
pub fn tasue_u30fb_nosue() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match すえ (末) as 名詞/非自立
    #[derive(Debug)]
    struct SueMatcher;
    impl super::Matcher for SueMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "すえ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Match の particle (連体化)
    #[derive(Debug)]
    struct NoParticleMatcher;
    impl super::Matcher for NoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
        }
    }

    // Match に particle (格助詞)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match either Verb[た] or Noun + の before すえ
    // This matches: (Verb + た) OR (Noun + の)
    #[derive(Debug)]
    struct VerbTaOrNounNoMatcher;
    impl super::Matcher for VerbTaOrNounNoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match た (past auxiliary)
            if token.surface == "た" && token.pos.first().is_some_and(|pos| pos == "助動詞") {
                return true;
            }
            // Match の (particle/連体化)
            if token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化") {
                return true;
            }
            false
        }
    }

    vec![
        TokenMatcher::Any, // Verb (連用形/連用タ接続) or Noun
        TokenMatcher::Custom(Arc::new(VerbTaOrNounNoMatcher)), // た or の
        TokenMatcher::Custom(Arc::new(SueMatcher)), // すえ
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NiParticleMatcher)))), // optional に
    ]
}

// Pattern: にしたがって (in accordance with, as, following)
// Structures: Verb[る] + にしたがって/にしたがい / Noun + にしたがって/にしたがい
pub fn nishitagatte() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match に particle (格助詞)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match したがう verb (base_form = したがう)
    #[derive(Debug)]
    struct ShitagauVerbMatcher;
    impl super::Matcher for ShitagauVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "したがう"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    // Match て or い (connecting particles/forms)
    #[derive(Debug)]
    struct TeOrIMatcher;
    impl super::Matcher for TeOrIMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match て (助詞/接続助詞)
            if token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") {
                return true;
            }
            // Match い as auxiliary (for にしたがい form)
            if token.surface == "い"
                && token.pos.first().is_some_and(|pos| pos == "動詞" || pos == "助動詞") {
                return true;
            }
            false
        }
    }

    vec![
        TokenMatcher::Any, // Verb (dictionary form) or Noun
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)), // に
        TokenMatcher::Custom(Arc::new(ShitagauVerbMatcher)), // したがう (conjugated)
        TokenMatcher::Custom(Arc::new(TeOrIMatcher)), // て or い
    ]
}

// Pattern: に伴って・に伴い (along with, in conjunction with, due to)
// Structures: Verb[る]+(の)+ に伴って/に伴い, Noun + に伴って/に伴い, Verb/Noun + に伴う + Noun
pub fn nitomonatte_u30fb_nitomonai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for Noun or Verb in dictionary form
    #[derive(Debug)]
    struct NounOrVerbMatcher;
    impl super::Matcher for NounOrVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Accept nouns
            if token.pos.first().is_some_and(|pos| pos == "名詞") {
                return true;
            }
            // Accept verbs in dictionary form (基本形)
            if token.pos.first().is_some_and(|pos| pos == "動詞") {
                return token.features.get(5).is_some_and(|f| f == "基本形");
            }
            false
        }
    }

    // Matcher for に particle (格助詞)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Matcher for 伴う verb (伴っ in 連用タ接続, 伴い in 連用形, or 伴う in 基本形)
    #[derive(Debug)]
    struct TomonauVerbMatcher;
    impl super::Matcher for TomonauVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "伴う"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    // Matcher for て particle (接続助詞) - only for 伴って form
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl super::Matcher for TeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Matcher for の nominalizer (名詞/非自立)
    #[derive(Debug)]
    struct NoNominalizerMatcher;
    impl super::Matcher for NoNominalizerMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    vec![
        // Accept: Verb(dictionary form) or Noun
        TokenMatcher::Custom(Arc::new(NounOrVerbMatcher)),
        // Optional の nominalizer (for verb + の + に伴って)
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NoNominalizerMatcher,
        )))),
        // に particle
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        // 伴う verb
        TokenMatcher::Custom(Arc::new(TomonauVerbMatcher)),
        // Optional て particle (for 伴って, not for 伴い or 伴う)
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            TeParticleMatcher,
        )))),
    ]
}

// Pattern: につき (due to, per) - split tokenization
// Structures: Noun + に + つき (as separate tokens)
// Example: "閉店につき" → 閉店(noun) + に(particle) + つき(verb)
pub fn nitsuki() -> Vec<TokenMatcher> {
    use std::sync::Arc;

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

    // Matcher for つき verb (連用形)
    #[derive(Debug)]
    struct TsukiVerbMatcher;
    impl super::Matcher for TsukiVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "つき"
                && token.base_form == "つく"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(TsukiVerbMatcher)),
    ]
}

// Pattern: につき (due to, per) - compound tokenization
// Structures: Noun + につき (as single particle token)
// Example: "一人につき" → 一(noun) + 人(noun) + につき(particle)
pub fn nitsuki_compound() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for につき as a single particle token
    #[derive(Debug)]
    struct NitsukiParticleMatcher;
    impl super::Matcher for NitsukiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "につき"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(NitsukiParticleMatcher)),
    ]
}

// Pattern: につけ (every time, whenever)
// Structures: Verb/Noun + につけ
// Tokenization: につけ as compound particle (助詞/格助詞/連語)
pub fn nitsuke() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match につけ as compound particle
    #[derive(Debug)]
    struct NitsukeMatcher;
    impl super::Matcher for NitsukeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "につけ"
                && token.base_form == "につけ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(NitsukeMatcher))]
}

// Pattern: につけて (every time, whenever - less common variant)
// Structures: Verb/Noun + につけて
// Tokenization: に + つけ(動詞/連用形) + て
pub fn nitsukete() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match つけ as verb in 連用形
    #[derive(Debug)]
    struct TsukeVerbMatcher;
    impl super::Matcher for TsukeVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "つけ"
                && token.base_form == "つける"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token
                    .features
                    .get(5)
                    .is_some_and(|form| form == "連用形")
        }
    }

    vec![
        TokenMatcher::Surface("に"),
        TokenMatcher::Custom(Arc::new(TsukeVerbMatcher)),
        TokenMatcher::Surface("て"),
    ]
}

// Pattern: にかかわる (relating to, concerning)
// Structures: Noun + にかかわる, Noun + にかかわる + Noun
pub fn nikakawaru() -> Vec<TokenMatcher> {
    use super::noun_matcher;

    // Custom matcher for に particle (case particle)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    // Custom matcher for かかわる verb (any conjugation)
    #[derive(Debug)]
    struct KakawaruVerbMatcher;
    impl Matcher for KakawaruVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "かかわる"
                && token.pos.first().is_some_and(|p| p == "動詞")
        }
    }

    vec![
        noun_matcher(),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(KakawaruVerbMatcher)),
    ]
}

// Pattern: に向かって・に向けて (towards, facing, aimed at)
// Structures: Noun + に向（む）かって, Noun + に向（む）けて, Noun + に向（む）けて + の + Noun
pub fn nimukatte_u30fb_nimukete() -> Vec<TokenMatcher> {
    use super::noun_matcher;

    // Custom matcher for に particle (case particle)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    // Custom matcher for むかう or むける verb (both mean "to face/direct towards")
    #[derive(Debug)]
    struct MukauMukeruVerbMatcher;
    impl Matcher for MukauMukeruVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.base_form == "むかう" || token.base_form == "むける")
                && token.pos.first().is_some_and(|p| p == "動詞")
        }
    }

    // Custom matcher for て particle (connecting particle)
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl Matcher for TeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
        }
    }

    vec![
        noun_matcher(),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(MukauMukeruVerbMatcher)),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
    ]
}

// Pattern: が気になる (be concerned about, be interested in)
// Structures: Noun/こと/の + が + 気 + に + なる/なります
pub fn gakininaru() -> Vec<TokenMatcher> {
    use super::concat;

    // Custom matcher for noun-like elements (nouns, こと, の)
    #[derive(Debug)]
    struct NounLikeMatcher;
    impl Matcher for NounLikeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|p| p == "名詞")
        }
    }

    // Custom matcher for が particle (case particle)
    #[derive(Debug)]
    struct GaParticleMatcher;
    impl Matcher for GaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "が"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    // Custom matcher for 気 (ki - attention/focus) as noun
    #[derive(Debug)]
    struct KiNounMatcher;
    impl Matcher for KiNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "気"
                && token.pos.first().is_some_and(|p| p == "名詞")
        }
    }

    // Custom matcher for に particle (case particle)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    // Custom matcher for ます (polite auxiliary verb)
    #[derive(Debug)]
    struct MasuMatcher;
    impl Matcher for MasuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    concat(vec![
        vec![TokenMatcher::Custom(Arc::new(NounLikeMatcher))],
        vec![TokenMatcher::Custom(Arc::new(GaParticleMatcher))],
        vec![TokenMatcher::Custom(Arc::new(KiNounMatcher))],
        vec![TokenMatcher::Custom(Arc::new(NiParticleMatcher))],
        vec![TokenMatcher::specific_verb("なる")],
        vec![TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(MasuMatcher))))],
    ])
}

// Pattern: に気をつける (be careful of, watch out for, pay attention to)
// Structures: Noun + に気をつける / Verb[ない] + ように気をつける
pub fn nikiwotsukeru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match Noun or Verb (to start the pattern)
    // Exclude よう (nominalizer) which should not be the start
    #[derive(Debug)]
    struct NounOrVerbMatcher;
    impl Matcher for NounOrVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Exclude よう (nominalizer)
            if token.surface == "よう" && token.base_form == "よう" {
                return false;
            }
            token.pos.first().is_some_and(|pos| pos == "名詞" || pos == "動詞")
        }
    }

    // Match に (case or adverb particle)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に" && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    // Match き (verb くる in 連用形)
    #[derive(Debug)]
    struct KiVerbMatcher;
    impl Matcher for KiVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "き"
                && token.base_form == "くる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形")
        }
    }

    // Match を (case particle)
    #[derive(Debug)]
    struct WoParticleMatcher;
    impl Matcher for WoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "を"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match つける verb (any conjugation)
    #[derive(Debug)]
    struct TsukeruVerbMatcher;
    impl Matcher for TsukeruVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "つける"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NounOrVerbMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(KiVerbMatcher)),
        TokenMatcher::Custom(Arc::new(WoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(TsukeruVerbMatcher)),
    ]
}

// Pattern: も構わず (without worrying about, without minding)
// Structures: Verb/Adj + の + も + かまわず, Noun + も + かまわず, Any + にも + かまわず
pub fn mokamawazu() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match の as nominalizer (名詞/非自立)
    #[derive(Debug)]
    struct NoNominalizerMatcher;
    impl Matcher for NoNominalizerMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Match に particle (助詞/格助詞)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
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

    // Match かまう verb in 未然形 (動詞/自立/五段・ワ行促音便/未然形)
    #[derive(Debug)]
    struct KamawaMatcher;
    impl Matcher for KamawaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "かまわ"
                && token.base_form == "かまう"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然形")
        }
    }

    // Match ず auxiliary verb (助動詞/特殊・ヌ, base=ぬ)
    #[derive(Debug)]
    struct ZuAuxiliaryMatcher;
    impl Matcher for ZuAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ず"
                && token.base_form == "ぬ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        TokenMatcher::Any, // Verb, Adjective, or Noun
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NoNominalizerMatcher)))), // Optional の for nominalization
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NiParticleMatcher)))), // Optional に for にも variant
        TokenMatcher::Custom(Arc::new(MoParticleMatcher)), // も particle
        TokenMatcher::Custom(Arc::new(KamawaMatcher)), // かまわ verb
        TokenMatcher::Custom(Arc::new(ZuAuxiliaryMatcher)), // ず auxiliary
    ]
}

// Pattern: かねる
// Pattern: かねる (cannot, difficult to do)
// Structures: Verb[stem] + かねる / かねます
pub fn kaneru() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct VerbStemMatcher;
    impl Matcher for VerbStemMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形")
        }
    }

    #[derive(Debug)]
    struct KaneruVerbMatcher;
    impl Matcher for KaneruVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "かね" || token.surface == "兼ね")
                && token.base_form == "かねる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    #[derive(Debug)]
    struct MasuOrRuMatcher;
    impl Matcher for MasuOrRuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match ます (助動詞)
            (token.surface == "ます"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞"))
            // OR match る (基本形 of かねる)
            || (token.surface == "る"
                && token.base_form == "かねる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立"))
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbStemMatcher)),
        TokenMatcher::Custom(Arc::new(KaneruVerbMatcher)),
        TokenMatcher::Custom(Arc::new(MasuOrRuMatcher)),
    ]
}

// Pattern: かねない (might, capable of)
// Structures: Verb[stem] + かねない / かねません
pub fn kanenai() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct VerbStemMatcher;
    impl Matcher for VerbStemMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形")
        }
    }

    #[derive(Debug)]
    struct KaneruVerbMatcher;
    impl Matcher for KaneruVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "かね" || token.surface == "兼ね")
                && token.base_form == "かねる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    #[derive(Debug)]
    struct NaiOrMasenMatcher;
    impl Matcher for NaiOrMasenMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match ない (助動詞)
            (token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞"))
            // OR match ませ (助動詞, ます base form)
            || (token.surface == "ませ"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞"))
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbStemMatcher)),
        TokenMatcher::Custom(Arc::new(KaneruVerbMatcher)),
        TokenMatcher::Custom(Arc::new(NaiOrMasenMatcher)),
        // Optional ん for ません form
        TokenMatcher::Optional(Box::new(TokenMatcher::Surface("ん"))),
    ]
}

// Pattern: を除いて (except for, excluding)
// Structures: Noun + を + 除いて/除く/除き
pub fn wonozoite() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match 除く verb in any conjugation form
    #[derive(Debug)]
    struct NozokuVerbMatcher;
    impl super::Matcher for NozokuVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "除く"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    // Match て particle
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl super::Matcher for TeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // This matches three variants:
    // 1. Noun + を + 除い + て (most common: を除いて)
    // 2. Noun + を + 除く (dictionary form: を除く)
    // We use Optional for て to match both patterns
    vec![
        super::noun_matcher(),
        TokenMatcher::Surface("を"),
        TokenMatcher::Custom(Arc::new(NozokuVerbMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(TeParticleMatcher)))),
    ]
}

// Pattern: にかかわらず (regardless of)
// Structures: Noun/Verb/Adjective (+ variations) + にかかわらず
pub fn nikakawarazu() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match に particle (助詞/格助詞)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    // Match かかわる verb in 未然形
    #[derive(Debug)]
    struct KakawaranaiMatcher;
    impl Matcher for KakawaranaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "かかわら"
                && token.base_form == "かかわる"
                && token.pos.first().is_some_and(|p| p == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然形")
        }
    }

    // Match ず auxiliary verb (助動詞/特殊・ヌ, base=ぬ)
    #[derive(Debug)]
    struct ZuAuxiliaryMatcher;
    impl Matcher for ZuAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ず"
                && token.base_form == "ぬ"
                && token.pos.first().is_some_and(|p| p == "助動詞")
        }
    }

    // Match は particle (助詞/係助詞) as stop condition
    #[derive(Debug)]
    struct WaParticleMatcher;
    impl Matcher for WaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "係助詞")
        }
    }

    // Match が particle (助詞/格助詞) as stop condition
    #[derive(Debug)]
    struct GaParticleMatcher;
    impl Matcher for GaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "が"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    vec![
        TokenMatcher::Wildcard {
            min: 1,
            max: 10,
            stop_conditions: vec![
                TokenMatcher::Custom(Arc::new(WaParticleMatcher)),
                TokenMatcher::Custom(Arc::new(GaParticleMatcher)),
            ],
        },
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(KakawaranaiMatcher)),
        TokenMatcher::Custom(Arc::new(ZuAuxiliaryMatcher)),
    ]
}

// Pattern: にもかかわらず (despite, in spite of)
// Structures:
//   - Verb/Adjective + の + にもかかわらず
//   - な-Adjective + である + にもかかわらず
//   - Noun + にもかかわらず
pub fn nimokakawarazu() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match の as nominalizer (名詞/非自立)
    #[derive(Debug)]
    struct NoNominalizerMatcher;
    impl Matcher for NoNominalizerMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "非自立")
        }
    }

    // Match である sequence: で(助動詞/特殊・ダ) + ある(助動詞)
    #[derive(Debug)]
    struct DeAruSequenceMatcher;
    impl Matcher for DeAruSequenceMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "で"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|p| p == "助動詞")
        }
    }

    // Match に particle (助詞/格助詞)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    // Match も particle (助詞/係助詞)
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "も"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "係助詞")
        }
    }

    // Match かかわら verb (動詞/未然形, base=かかわる)
    #[derive(Debug)]
    struct KakawaruMatcher;
    impl Matcher for KakawaruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "かかわる"
                && token.pos.first().is_some_and(|p| p == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然形")
        }
    }

    // Match ある auxiliary (助動詞, base=ある)
    #[derive(Debug)]
    struct AruAuxiliaryMatcher;
    impl Matcher for AruAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ある"
                && token.base_form == "ある"
                && token.pos.first().is_some_and(|p| p == "助動詞")
        }
    }

    // Match ず (助動詞/特殊・ヌ, base=ぬ)
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
        // Any verb/adjective/noun
        TokenMatcher::Any,
        // Optional の nominalizer for verbs/adjectives
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NoNominalizerMatcher,
        )))),
        // Optional である for na-adjectives: で(助動詞) + ある(助動詞)
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            DeAruSequenceMatcher,
        )))),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            AruAuxiliaryMatcher,
        )))),
        // Core structure: に + も + かかわら + ず
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(MoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(KakawaruMatcher)),
        TokenMatcher::Custom(Arc::new(ZuAuxiliaryMatcher)),
    ]
}

// Pattern: に限らず
pub fn nikagirazu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なお① (still, even, yet)
// Structures: なお (conjunction)
pub fn nao_u2460() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match なお as conjunction
    // Note: Tokenizes identically to なお② but has different meaning
    // なお① = "still, even, yet" (emphasizing continuation despite circumstances)
    // なお② = "furthermore, moreover" (adding new information)
    // Both patterns will be detected; context determines which meaning applies
    #[derive(Debug)]
    struct NaoConjunctionMatcher;
    impl super::Matcher for NaoConjunctionMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "なお"
                && token.base_form == "なお"
                && token.pos.first().is_some_and(|pos| pos == "接続詞")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(NaoConjunctionMatcher))]
}

// Pattern: なお② (furthermore, moreover, in addition)
// Structures: なお (conjunction)
pub fn nao_u2461() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match なお as conjunction
    #[derive(Debug)]
    struct NaoConjunctionMatcher;
    impl super::Matcher for NaoConjunctionMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "なお"
                && token.base_form == "なお"
                && token.pos.first().is_some_and(|pos| pos == "接続詞")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(NaoConjunctionMatcher))]
}

// Pattern: 限り (as long as, as far as, while, assuming)
// Structures:
//   - Verb[る/ない/た/ている] + 限り
//   - Noun + である + 限り
pub fn kagiri() -> Vec<TokenMatcher> {
    use super::Matcher;
    use std::sync::Arc;

    // Match かぎり as noun (非自立/副詞可能)
    #[derive(Debug)]
    struct KagiriMatcher;
    impl Matcher for KagiriMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "かぎり" || token.surface == "限り")
                && token.base_form == "かぎり"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Use TokenMatcher::Any to match any preceding word/phrase
    // This allows verb (any form), noun, adjective, etc. before かぎり
    vec![TokenMatcher::Any, TokenMatcher::Custom(Arc::new(KagiriMatcher))]
}

// Pattern: 次第だ・次第で (depending on, depends on)
// Structures: Noun + しだい + だ/です/で
pub fn shidaida_u30fb_shidaide() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match しだい (noun suffix)
    #[derive(Debug)]
    struct ShidaiMatcher;
    impl Matcher for ShidaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "しだい"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
        }
    }

    // Match だ, です, or で
    #[derive(Debug)]
    struct DaDesuDeMatcher;
    impl Matcher for DaDesuDeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // だ or です (auxiliary)
            if (token.surface == "だ" || token.surface == "です")
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
            {
                return true;
            }
            // で (case particle)
            if token.surface == "で" && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            {
                return true;
            }
            false
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(ShidaiMatcher)),
        TokenMatcher::Custom(Arc::new(DaDesuDeMatcher)),
    ]
}

// Pattern: 次第に (gradually, bit by bit)
// Structures: 次第に (adverb)
pub fn shidaini() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct ShidainiMatcher;
    impl super::Matcher for ShidainiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "次第に"
                && token.base_form == "次第に"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(ShidainiMatcher))]
}

// Pattern: ～てこそ (only if, only by, only when)
// Structures: Verb[て] + こそ
pub fn uff5e_tekoso() -> Vec<TokenMatcher> {
    use std::sync::Arc;

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

    // Match こそ particle (係助詞)
    #[derive(Debug)]
    struct KosoParticleMatcher;
    impl super::Matcher for KosoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こそ"
                && token.base_form == "こそ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    vec![
        super::flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(KosoParticleMatcher)),
    ]
}

// Pattern: を問わず (regardless of, irrespective of, whether or not)
// Structures:
//   - [Content] + を/は + 問わ(未然形) + ず
//   - Noun/か/かどうか + を/は + 問わず
pub fn wotowazu() -> Vec<TokenMatcher> {
    use super::Matcher;
    use std::sync::Arc;

    // Match を or は particle
    #[derive(Debug)]
    struct WoHaParticleMatcher;
    impl Matcher for WoHaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "を" || token.surface == "は")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    // Match とわ/問わ verb (base=とう, 未然形)
    #[derive(Debug)]
    struct TowaVerbMatcher;
    impl Matcher for TowaVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "とわ" || token.surface == "問わ")
                && token.base_form == "とう"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token
                    .features
                    .get(5)
                    .is_some_and(|form| form == "未然形")
        }
    }

    // Match ず auxiliary verb (base=ぬ)
    #[derive(Debug)]
    struct ZuAuxiliaryMatcher;
    impl Matcher for ZuAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ず"
                && token.base_form == "ぬ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Stop condition matcher for は/が particles
    #[derive(Debug)]
    struct HaGaStopMatcher;
    impl Matcher for HaGaStopMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "は" || token.surface == "が")
                && token.pos.first().is_some_and(|p| p == "助詞")
                && (token.pos.get(1).is_some_and(|p| p == "係助詞")
                    || token.pos.get(1).is_some_and(|p| p == "格助詞"))
        }
    }

    // Pattern: [Content] + を/は + とわ/問わ + ず
    // Use wildcard to capture preceding content with stop conditions
    vec![
        TokenMatcher::Wildcard {
            min: 1,
            max: 10,
            stop_conditions: vec![TokenMatcher::Custom(Arc::new(HaGaStopMatcher))],
        },
        TokenMatcher::Custom(Arc::new(WoHaParticleMatcher)),
        TokenMatcher::Custom(Arc::new(TowaVerbMatcher)),
        TokenMatcher::Custom(Arc::new(ZuAuxiliaryMatcher)),
    ]
}

// Pattern: よりしかたがない (there is no choice but, cannot be helped)
// Structures: Verb[る] + より + 仕方(が) + ない
pub fn yorishikataganai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match より particle (格助詞)
    #[derive(Debug)]
    struct YoriParticleMatcher;
    impl super::Matcher for YoriParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "より"
                && token.base_form == "より"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    // Match しかた (名詞/ナイ形容詞語幹)
    #[derive(Debug)]
    struct ShikataMatcher;
    impl super::Matcher for ShikataMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "しかた"
                && token.base_form == "しかた"
                && token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "ナイ形容詞語幹")
        }
    }

    // Match が particle (optional)
    #[derive(Debug)]
    struct GaParticleMatcher;
    impl super::Matcher for GaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "が"
                && token.base_form == "が"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    // Match ない (either 形容詞 or 助動詞)
    #[derive(Debug)]
    struct NaiMatcher;
    impl super::Matcher for NaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ない"
                && token.base_form == "ない"
                && (token.pos.first().is_some_and(|p| p == "形容詞")
                    || token.pos.first().is_some_and(|p| p == "助動詞"))
        }
    }

    vec![
        TokenMatcher::Verb {
            conjugation_form: Some("基本形"),
            base_form: None,
        },
        TokenMatcher::Custom(Arc::new(YoriParticleMatcher)),
        TokenMatcher::Custom(Arc::new(ShikataMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(GaParticleMatcher)))),
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
    ]
}

// Pattern: に越したことはない (there is nothing better than)
// Structures: (Verb/Adjective/Noun + である) + にこしたことはない/ありません
pub fn nikoshitakotohanai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match に particle (can be 格助詞 or 副詞化)
    #[derive(Debug)]
    struct NiKoshitaMatcher;
    impl Matcher for NiKoshitaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && (token
                    .pos
                    .get(1)
                    .is_some_and(|pos| pos == "格助詞" || pos == "副詞化"))
        }
    }

    // Match こし verb (base=こす, 連用形)
    #[derive(Debug)]
    struct KoshiMatcher;
    impl Matcher for KoshiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こし"
                && token.base_form == "こす"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形")
        }
    }

    // Match こと (名詞/非自立)
    #[derive(Debug)]
    struct KotoNonIndependentMatcher;
    impl Matcher for KotoNonIndependentMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こと"
                && token.base_form == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Match は係助詞
    #[derive(Debug)]
    struct WaKakariMatcher;
    impl Matcher for WaKakariMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Match ない (形容詞/自立 or 助動詞/特殊・ナイ) or あり (start of ありません)
    #[derive(Debug)]
    struct NaiOrAriMatcher;
    impl Matcher for NaiOrAriMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // ない ending
            (token.surface == "ない"
                && ((token.pos.first().is_some_and(|pos| pos == "形容詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "自立"))
                    || (token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.features.get(4).is_some_and(|f| f == "特殊・ナイ"))))
                // あり from ありません
                || (token.surface == "あり"
                    && token.base_form == "ある"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token.features.get(5).is_some_and(|f| f == "連用形"))
        }
    }

    // Match ませ from ます (助動詞, 未然形)
    #[derive(Debug)]
    struct MaseMatcher;
    impl Matcher for MaseMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ませ"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "未然形")
        }
    }

    // Match ん negative auxiliary (助動詞, 不変化型)
    #[derive(Debug)]
    struct NNegativeMatcher;
    impl Matcher for NNegativeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ん"
                && token.base_form == "ん"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(4).is_some_and(|f| f == "不変化型")
        }
    }

    vec![
        TokenMatcher::Any, // Verb, Adjective, or Noun (with optional である)
        TokenMatcher::Custom(Arc::new(NiKoshitaMatcher)),
        TokenMatcher::Custom(Arc::new(KoshiMatcher)),
        super::past_auxiliary(),
        TokenMatcher::Custom(Arc::new(KotoNonIndependentMatcher)),
        TokenMatcher::Custom(Arc::new(WaKakariMatcher)),
        // Match either ない or あり (start of ありません)
        TokenMatcher::Custom(Arc::new(NaiOrAriMatcher)),
        // Optional ませ + ん for polite form
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            MaseMatcher,
        )))),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NNegativeMatcher,
        )))),
    ]
}

// Pattern: 要するに (To sum up, in summary, in short)
// Structure: 要するに + Phrase (sentence-initial discourse marker)
pub fn yousuruni() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct YousuruniMatcher;
    impl Matcher for YousuruniMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "要するに"
                && token.base_form == "要するに"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(YousuruniMatcher))]
}

// Pattern: てからでないと (unless you do, until you do)
// Structures: Verb[て] + から + で + ない + と/なければ
pub fn tekaradenaito() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match て or で particle (conjunction particle)
    #[derive(Debug)]
    struct TeDeParticleMatcher;
    impl Matcher for TeDeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Match から as case particle (after)
    #[derive(Debug)]
    struct KaraAfterMatcher;
    impl Matcher for KaraAfterMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "から"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match で (copula auxiliary だ in 連用形)
    #[derive(Debug)]
    struct DeAuxiliaryMatcher;
    impl Matcher for DeAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "だ"
        }
    }

    // Match ない or なけれ (negative auxiliary)
    #[derive(Debug)]
    struct NaiNakerebaMatcher;
    impl Matcher for NaiNakerebaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "ない" || token.surface == "なけれ")
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "ない"
        }
    }

    // Match と or ば (conditional particles)
    #[derive(Debug)]
    struct ToOrBaMatcher;
    impl Matcher for ToOrBaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "と" || token.surface == "ば")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    vec![
        super::flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(TeDeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(KaraAfterMatcher)),
        TokenMatcher::Custom(Arc::new(DeAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(NaiNakerebaMatcher)),
        TokenMatcher::Custom(Arc::new(ToOrBaMatcher)),
    ]
}

// Pattern: なくはない (it's not that it isn't, somewhat)
// Structures: なく + は + ない (double negative construction)
pub fn nakuhanai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match なく (auxiliary or adjective)
    #[derive(Debug)]
    struct NakuMatcher;
    impl Matcher for NakuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "なく"
                && (token.pos.first().is_some_and(|pos| pos == "助動詞")
                    || token.pos.first().is_some_and(|pos| pos == "形容詞"))
        }
    }

    // Match は particle
    #[derive(Debug)]
    struct WaMatcher;
    impl Matcher for WaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は" && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    // Match ない (auxiliary or adjective)
    #[derive(Debug)]
    struct NaiMatcher;
    impl Matcher for NaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ない"
                && (token.pos.first().is_some_and(|pos| pos == "助動詞")
                    || token.pos.first().is_some_and(|pos| pos == "形容詞"))
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NakuMatcher)),
        TokenMatcher::Custom(Arc::new(WaMatcher)),
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
    ]
}

// Pattern: ないことには～ない
// Pattern: ないことには～ない (unless, without)
// Structures:
//   1. Verb[ない] + ことには
//   2. い-Adjective[ない] + ことには
//   3. な-Adjective + でない + ことには
//   4. Noun + でない + ことには
pub fn naikotoniha_uff5e_nai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Custom matcher for で auxiliary (from だ copula)
    #[derive(Debug)]
    struct DeAuxiliaryMatcher;
    impl Matcher for DeAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "で"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Custom matcher for ない (auxiliary or adjective)
    #[derive(Debug)]
    struct NaiMatcher;
    impl Matcher for NaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ない"
                && token.base_form == "ない"
                && (token.pos.first().is_some_and(|pos| pos == "助動詞")
                    || token.pos.first().is_some_and(|pos| pos == "形容詞"))
        }
    }

    // Custom matcher for こと (dependent noun)
    #[derive(Debug)]
    struct KotoMatcher;
    impl Matcher for KotoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Custom matcher for に particle
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Custom matcher for は particle
    #[derive(Debug)]
    struct WaParticleMatcher;
    impl Matcher for WaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Pattern matches (で)ないことには
    // で is optional - present for な-adjectives/nouns (でない), absent for verbs/い-adjectives (ない)
    vec![
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            DeAuxiliaryMatcher,
        )))), // Optional で
        TokenMatcher::Custom(Arc::new(NaiMatcher)), // ない
        TokenMatcher::Custom(Arc::new(KotoMatcher)), // こと
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)), // に
        TokenMatcher::Custom(Arc::new(WaParticleMatcher)), // は
    ]
}

// Pattern: ないではいられない (can't help but, can't resist)
// Structures: Verb[ない] + で/じゃ + (は) + いられない/ません
pub fn naidehairarenai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ない (助動詞) - either 連用デ接続 or 基本形
    #[derive(Debug)]
    struct NaiAuxiliaryMatcher;
    impl Matcher for NaiAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(4).is_some_and(|f| f == "特殊・ナイ")
        }
    }

    // Match で (助詞/接続助詞 or 助動詞/だ) or じゃ (助詞/接続助詞)
    #[derive(Debug)]
    struct DeJyaMatcher;
    impl Matcher for DeJyaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // で as 助詞/接続助詞
            (token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞"))
                // で as 助動詞/だ
                || (token.surface == "で"
                    && token.base_form == "だ"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞"))
                // じゃ as 助詞/接続助詞
                || (token.surface == "じゃ"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "接続助詞"))
        }
    }

    // Match は (助詞/係助詞) - optional for では form
    #[derive(Debug)]
    struct WaKakariParticleMatcher;
    impl Matcher for WaKakariParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Match い from いる (動詞, 未然形)
    #[derive(Debug)]
    struct IruMizenMatcher;
    impl Matcher for IruMizenMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "い"
                && token.base_form == "いる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然形")
        }
    }

    // Match られ from られる (動詞/接尾) - potential suffix (either 未然形 or 連用形)
    #[derive(Debug)]
    struct RareMatcher;
    impl Matcher for RareMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "られ"
                && token.base_form == "られる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
        }
    }

    vec![
        TokenMatcher::Any, // Verb in 未然形
        TokenMatcher::Custom(Arc::new(NaiAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(DeJyaMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            WaKakariParticleMatcher,
        )))), // は is optional (present in では but not in じゃ)
        TokenMatcher::Custom(Arc::new(IruMizenMatcher)),
        TokenMatcher::Custom(Arc::new(RareMatcher)),
        TokenMatcher::Any, // ない, なく, なかった, ませ, etc. (various negative endings)
    ]
}

// Pattern: ねばならない
pub fn nebanaranai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: たまえ (polite imperative)
// Structure: Verb[stem] + たまえ
pub fn tamae() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct TamaeMatcher;
    impl Matcher for TamaeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Correctly tokenized: たまえ as verb with base たまう in imperative form
            token.surface == "たまえ"
                && token.base_form == "たまう"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "命令ｅ")
        }
    }
    vec![
        TokenMatcher::verb_with_form("連用形"),
        TokenMatcher::Custom(Arc::new(TamaeMatcher)),
    ]
}

// Pattern: ～のうち(で)
// Pattern: ～のうち(で) (among, out of)
// Structures: この/その + うち + (で/の/から) OR Any + の + うち + (で/の/から)
// Note: Using two matcher patterns due to different prefix structures
pub fn uff5e_nouchi_de() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match うち as dependent noun
    #[derive(Debug)]
    struct UchiMatcher;
    impl Matcher for UchiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "うち"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Match の as 助詞/連体化
    #[derive(Debug)]
    struct NoRentaikaMatcher;
    impl Matcher for NoRentaikaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
        }
    }

    // Match で/の/から as particles after うち
    #[derive(Debug)]
    struct DeNoKaraMatcher;
    impl Matcher for DeNoKaraMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "で" || token.surface == "の" || token.surface == "から")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    // Pattern: Any + の + うち + optional(で/の/から)
    vec![
        TokenMatcher::Any,
        TokenMatcher::Custom(Arc::new(NoRentaikaMatcher)),
        TokenMatcher::Custom(Arc::new(UchiMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(DeNoKaraMatcher)))),
    ]
}

// Pattern: ～のうち(で) variant for この/その + うち
// Structures: この/その/あの + うち + (で/の/から)
pub fn uff5e_nouchi_de_kono() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match うち as dependent noun
    #[derive(Debug)]
    struct UchiMatcher;
    impl Matcher for UchiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "うち"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Match この/その/あの as 連体詞
    #[derive(Debug)]
    struct KonoSonoMatcher;
    impl Matcher for KonoSonoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "この" || token.surface == "その" || token.surface == "あの")
                && token.pos.first().is_some_and(|pos| pos == "連体詞")
        }
    }

    // Match で/の/から as particles after うち
    #[derive(Debug)]
    struct DeNoKaraMatcher;
    impl Matcher for DeNoKaraMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "で" || token.surface == "の" || token.surface == "から")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    // Pattern: この/その/あの + うち + optional(で/の/から)
    vec![
        TokenMatcher::Custom(Arc::new(KonoSonoMatcher)),
        TokenMatcher::Custom(Arc::new(UchiMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(DeNoKaraMatcher)))),
    ]
}

// Pattern: つつ (while doing, in the course of)
// Structures: Verb[stem] + つつ
pub fn tsutsu() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct TsutsuParticleMatcher;
    impl super::Matcher for TsutsuParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "つつ"
                && token.base_form == "つつ"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
        }
    }

    vec![
        TokenMatcher::verb_with_form("連用形"),
        TokenMatcher::Custom(Arc::new(TsutsuParticleMatcher)),
    ]
}

// Pattern: つつ(も) (even while doing, although doing)
// Structures: Verb[stem] + つつ + も
pub fn tsutsu_mo() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct TsutsuParticleMatcher;
    impl super::Matcher for TsutsuParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "つつ"
                && token.base_form == "つつ"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
        }
    }

    #[derive(Debug)]
    struct MoParticleMatcher;
    impl super::Matcher for MoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "も"
                && token.base_form == "も"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "係助詞")
        }
    }

    vec![
        TokenMatcher::verb_with_form("連用形"),
        TokenMatcher::Custom(Arc::new(TsutsuParticleMatcher)),
        TokenMatcher::Custom(Arc::new(MoParticleMatcher)),
    ]
}

// Pattern: に際して (on the occasion of, at the time of)
// Structures: Verb[る] + に際して, Noun + に際して, Noun + に際しての + Noun
pub fn nisaishite() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match に際して as a single compound particle token
    #[derive(Debug)]
    struct NisaishiteMatcher;
    impl Matcher for NisaishiteMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に際して"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match verbs in dictionary form (基本形) or nouns
    #[derive(Debug)]
    struct VerbOrNounMatcher;
    impl Matcher for VerbOrNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match verb in dictionary form
            let is_verb_kihonkei = token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "基本形");

            // Match any noun
            let is_noun = token.pos.first().is_some_and(|pos| pos == "名詞");

            is_verb_kihonkei || is_noun
        }
    }

    // Match の as a nominalizer/relativizer particle
    #[derive(Debug)]
    struct NoRentaikaMatcher;
    impl Matcher for NoRentaikaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbOrNounMatcher)),
        TokenMatcher::Custom(Arc::new(NisaishiteMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NoRentaikaMatcher,
        )))),
    ]
}

// Pattern: 際に
// Pattern: 際に (on the occasion of, when)
// Structures:
//   1. Verb[る] + 際に
//   2. Verb[た] + 際に (Verb連用形 + た + 際に)
//   3. Noun + の + 際に
pub fn saini() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Custom matcher for 際 (sai) as a dependent noun
    #[derive(Debug)]
    struct SaiMatcher;
    impl Matcher for SaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "際"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Custom matcher for に particle after 際
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に" && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    // Custom matcher for の nominalizer/relativizer particle
    #[derive(Debug)]
    struct NoParticleMatcher;
    impl Matcher for NoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
        }
    }

    // Pattern matches:
    // - (Verb or Noun) + (optional た) + (optional の) + 際 + に
    //
    // This handles all three cases:
    // 1. Verb[る] + 際に (e.g., 入る際に)
    // 2. Verb連用形 + た + 際に (e.g., 飛び散った際に)
    // 3. Noun + の + 際に (e.g., 面接の際に)
    vec![
        TokenMatcher::Any, // Verb or Noun
        TokenMatcher::Optional(Box::new(super::past_auxiliary())), // Optional た
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NoParticleMatcher,
        )))), // Optional の
        TokenMatcher::Custom(Arc::new(SaiMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
    ]
}

// Pattern: にあたり・にあたって
// Pattern: にあたり・にあたって (on the occasion of, at the time of)
// Structures: Verb[る] + にあたり/にあたって, Noun + にあたり/にあたって
pub fn niatari_u30fb_niatatte() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match にあたり or にあたって as single compound particle tokens
    #[derive(Debug)]
    struct NiatariNiatatteMatcher;
    impl Matcher for NiatariNiatatteMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "にあたり" || token.surface == "にあたって")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match verbs in dictionary form (基本形) or nouns
    #[derive(Debug)]
    struct VerbOrNounMatcher;
    impl Matcher for VerbOrNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match verb in dictionary form
            let is_verb_kihonkei = token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "基本形");

            // Match any noun
            let is_noun = token.pos.first().is_some_and(|pos| pos == "名詞");

            is_verb_kihonkei || is_noun
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbOrNounMatcher)),
        TokenMatcher::Custom(Arc::new(NiatariNiatatteMatcher)),
    ]
}

// Pattern: を契機に (as a trigger/opportunity, led to)
// Structures: [Noun/の/こと] + を + 契機 + に/として/にして
pub fn wokeikini() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match を particle
    #[derive(Debug)]
    struct WoMatcher;
    impl Matcher for WoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "を"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    // Match 契機 (noun)
    #[derive(Debug)]
    struct KeikiMatcher;
    impl Matcher for KeikiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "契機"
                && token.pos.first().is_some_and(|p| p == "名詞")
        }
    }

    // Match に or として
    #[derive(Debug)]
    struct NiToshiteMatcher;
    impl Matcher for NiToshiteMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // に (助詞/格助詞/一般)
            (token.surface == "に"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞"))
            // Or として (助詞/格助詞/連語)
            || (token.surface == "として"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞"))
        }
    }

    // Match し (from する, for にして variation)
    #[derive(Debug)]
    struct ShiMatcher;
    impl Matcher for ShiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "し"
                && token.base_form == "する"
                && token.pos.first().is_some_and(|p| p == "動詞")
        }
    }

    // Match て (for にして variation)
    #[derive(Debug)]
    struct TeMatcher;
    impl Matcher for TeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
        }
    }

    vec![
        TokenMatcher::Any,  // Noun, の, or こと
        TokenMatcher::Custom(Arc::new(WoMatcher)),
        TokenMatcher::Custom(Arc::new(KeikiMatcher)),
        TokenMatcher::Custom(Arc::new(NiToshiteMatcher)),
        // Optional して (for にして variation)
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(ShiMatcher)))),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(TeMatcher)))),
    ]
}

// Pattern: つつある
pub fn tsutsuaru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct TsutsuParticleMatcher;
    impl super::Matcher for TsutsuParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "つつ"
                && token.base_form == "つつ"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
        }
    }

    vec![
        TokenMatcher::verb_with_form("連用形"),
        TokenMatcher::Custom(Arc::new(TsutsuParticleMatcher)),
        TokenMatcher::specific_verb("ある"),
    ]
}

// Pattern: ～ところに・～ところへ (at the time of, while, when)
// Structures: Verb［ている］+ ところに/へ, Verb［ていた］+ ところに/へ
pub fn uff5e_tokoroni_u30fb_uff5e_tokorohe() -> Vec<TokenMatcher> {
    use super::{concat, flexible_verb_form};
    use std::sync::Arc;

    // Match て or で particle
    #[derive(Debug)]
    struct TeDeFormMatcher;
    impl Matcher for TeDeFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|p| p == "助詞")
        }
    }

    // Match いる in 基本形 (いる) or 連用形 (い)
    #[derive(Debug)]
    struct IruMatcher;
    impl Matcher for IruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "いる"
                && token.pos.first().is_some_and(|p| p == "動詞")
                && (token.features.get(5).is_some_and(|f| f == "基本形")
                    || token.features.get(5).is_some_and(|f| f == "連用形"))
        }
    }

    // Match ところ noun
    #[derive(Debug)]
    struct TokoroMatcher;
    impl Matcher for TokoroMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ところ"
                && token.pos.first().is_some_and(|p| p == "名詞")
        }
    }

    // Match に or へ particle
    #[derive(Debug)]
    struct NiHeParticleMatcher;
    impl Matcher for NiHeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "に" || token.surface == "へ")
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    concat(vec![
        vec![flexible_verb_form()],
        vec![TokenMatcher::Custom(Arc::new(TeDeFormMatcher))],
        vec![TokenMatcher::Custom(Arc::new(IruMatcher))],
        vec![TokenMatcher::Optional(Box::new(super::past_auxiliary()))], // Optional た
        vec![TokenMatcher::Custom(Arc::new(TokoroMatcher))],
        vec![TokenMatcher::Custom(Arc::new(NiHeParticleMatcher))],
    ])
}

// Pattern: か〜ないかのうちに (as soon as, just when, barely when)
// Structure: Verb[基本形] + か + Verb[未然形] + ない + かのうちに
// NOTE: Grammar requires same verb repeated, but matcher doesn't enforce this
//       due to TokenMatcher API limitations. In practice, this specific pattern
//       is rarely written with different verbs.
pub fn ka_u301c_naikanouchini() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for か particle (副助詞／並立助詞／終助詞)
    #[derive(Debug)]
    struct KaParticleMatcher;
    impl Matcher for KaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "か"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token
                    .pos
                    .get(1)
                    .is_some_and(|pos| pos == "副助詞／並立助詞／終助詞")
        }
    }

    // Matcher for ない auxiliary
    #[derive(Debug)]
    struct NaiMatcher;
    impl Matcher for NaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Matcher for の particle (連体化)
    #[derive(Debug)]
    struct NoMatcher;
    impl Matcher for NoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
        }
    }

    // Matcher for うち noun (名詞/非自立/副詞可能)
    #[derive(Debug)]
    struct UchiMatcher;
    impl Matcher for UchiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "うち"
                && token.base_form == "うち"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Matcher for に particle (格助詞)
    #[derive(Debug)]
    struct NiMatcher;
    impl Matcher for NiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        // Verb in 基本形 (dictionary form)
        TokenMatcher::Verb {
            conjugation_form: Some("基本形"),
            base_form: None,
        },
        // か particle
        TokenMatcher::Custom(Arc::new(KaParticleMatcher)),
        // Optional wildcard for compound verbs (e.g., 飲み in 飲み終わる)
        // Matches 0-1 verb tokens in 連用形 that precede the main verb
        TokenMatcher::Wildcard {
            min: 0,
            max: 1,
            stop_conditions: vec![],
        },
        // Verb in 未然形 (negative stem)
        // TODO: Should verify same verb as first token, but TokenMatcher API doesn't support this
        TokenMatcher::Verb {
            conjugation_form: Some("未然形"),
            base_form: None,
        },
        // ない auxiliary
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
        // か particle (second occurrence)
        TokenMatcher::Custom(Arc::new(KaParticleMatcher)),
        // の particle
        TokenMatcher::Custom(Arc::new(NoMatcher)),
        // うち noun
        TokenMatcher::Custom(Arc::new(UchiMatcher)),
        // に particle
        TokenMatcher::Custom(Arc::new(NiMatcher)),
    ]
}

// Pattern: がけに (on the way, as you go)
// Structure: Verb[stem] + がけ + に
// Note: Tokenizes as compound noun (帰りがけ/行きがけ/通りがけ) + に
pub fn gakeni() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for compound noun ending in がけ
    #[derive(Debug)]
    struct GakeNounMatcher;
    impl Matcher for GakeNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface.ends_with("がけ")
                && token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(GakeNounMatcher)),
        TokenMatcher::Surface("に"),
    ]
}

// Pattern: ていては (if you keep doing, if one continues with)
// Structures: Verb[て] + いては
pub fn teiteha() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for い (いる verb in 連用形, non-auxiliary)
    #[derive(Debug)]
    struct IRuyoukeiMatcher;
    impl super::Matcher for IRuyoukeiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "い"
                && token.base_form == "いる"
                && token.pos.first().is_some_and(|p| p == "動詞")
                && token.pos.get(1).is_some_and(|p| p == "非自立")
                && token.features.get(5).is_some_and(|f| f == "連用形")
        }
    }

    // Matcher for て or で particle (after verb)
    #[derive(Debug)]
    struct TeDeParticleMatcher;
    impl super::Matcher for TeDeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
        }
    }

    // Matcher for て particle (after いる)
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl super::Matcher for TeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.base_form == "て"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
        }
    }

    // Matcher for は particle
    #[derive(Debug)]
    struct HaParticleMatcher;
    impl super::Matcher for HaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.base_form == "は"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "係助詞")
        }
    }

    vec![
        super::flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(TeDeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(IRuyoukeiMatcher)),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(HaParticleMatcher)),
    ]
}

// Pattern: ところだった ② (was just about to, was in the middle of)
// Structures: Verb[る] + ところ + だった/でした
pub fn tokorodatta_u2461() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for ところ as 名詞/非自立
    #[derive(Debug)]
    struct TokoroMatcher;
    impl Matcher for TokoroMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ところ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Matcher for だった (だ + た) or でした (です + た)
    // This matches the だっ/でし part
    #[derive(Debug)]
    struct DaDattaMatcher;
    impl Matcher for DaDattaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            ((token.surface == "だっ" && token.base_form == "だ")
                || (token.surface == "でし" && token.base_form == "です"))
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Pattern: Verb[基本形] + ところ + だった/でした
    // Note: ところだった ② (N2) uses only Verb[る] form, unlike ところだった ① (N3) which includes ない
    vec![
        TokenMatcher::Any,  // Verb in 基本形
        TokenMatcher::Custom(Arc::new(TokoroMatcher)),
        TokenMatcher::Custom(Arc::new(DaDattaMatcher)),
        super::past_auxiliary(),  // た
    ]
}

// Pattern: どころではない (far from, out of the question)
// Structures: Phrase + どころ + ではない/じゃない/ではありません/じゃありません
pub fn dokorodehanai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for どころ as 名詞/非自立
    #[derive(Debug)]
    struct DokoroMatcher;
    impl Matcher for DokoroMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "どころ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Matcher for で (from だ) or じゃ (casual contraction)
    #[derive(Debug)]
    struct DeJaMatcher;
    impl Matcher for DeJaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // で (from だ): 助動詞/特殊・ダ/連用形
            (token.surface == "で" && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞"))
            // じゃ: 助詞/副助詞
            || (token.surface == "じゃ" && token.pos.first().is_some_and(|pos| pos == "助詞"))
        }
    }

    // Matcher for は particle
    #[derive(Debug)]
    struct HaParticleMatcher;
    impl Matcher for HaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Matcher for negative ending: ない or あり (for ありません)
    #[derive(Debug)]
    struct NegativeEndingMatcher;
    impl Matcher for NegativeEndingMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // ない (adjective or auxiliary verb)
            (token.surface == "ない"
                && (token.pos.first().is_some_and(|pos| pos == "形容詞")
                    || token.pos.first().is_some_and(|pos| pos == "助動詞")))
            // あり (ある in 連用形 for ありません)
            || (token.surface == "あり" && token.base_form == "ある"
                && token.pos.first().is_some_and(|pos| pos == "動詞"))
        }
    }

    // Pattern: Phrase + どころ + (で/じゃ) + (は) + (ない/あり)
    // Matches: ではない, じゃない, ではありません (will continue to ません)
    // Using TokenMatcher::Any to match any preceding phrase
    vec![
        TokenMatcher::Any,  // Phrase (verb, adjective, or noun)
        TokenMatcher::Custom(Arc::new(DokoroMatcher)),
        TokenMatcher::Custom(Arc::new(DeJaMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(HaParticleMatcher)))),  // は is optional for じゃない
        TokenMatcher::Custom(Arc::new(NegativeEndingMatcher)),  // ない or あり
    ]
}

// Pattern: ぶりに (for the first time in [time period])
// Structures: Noun + ぶり + だ/です/に/の
pub fn burini() -> Vec<TokenMatcher> {
    use super::Matcher;

    // Match ぶり as a suffix noun
    #[derive(Debug)]
    struct BuriMatcher;
    impl Matcher for BuriMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ぶり"
                && token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "接尾")
        }
    }

    // Match time-related nouns (numbers, counters, time words)
    #[derive(Debug)]
    struct TimeNounMatcher;
    impl Matcher for TimeNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|p| p == "名詞")
        }
    }

    // Match に, の, だ, or です after ぶり
    #[derive(Debug)]
    struct BuriFollowMatcher;
    impl Matcher for BuriFollowMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match に (格助詞)
            (token.surface == "に" && token.pos.first().is_some_and(|p| p == "助詞"))
            // Match の (連体化)
            || (token.surface == "の" && token.pos.first().is_some_and(|p| p == "助詞"))
            // Match だ (助動詞)
            || (token.surface == "だ" && token.pos.first().is_some_and(|p| p == "助動詞"))
            // Match です (助動詞)
            || (token.surface == "です" && token.pos.first().is_some_and(|p| p == "助動詞"))
        }
    }

    vec![
        // One or more time nouns before ぶり (e.g., 一 + 年, or just 三年)
        TokenMatcher::Custom(Arc::new(TimeNounMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(TimeNounMatcher)))),
        TokenMatcher::Custom(Arc::new(BuriMatcher)),
        // Optional ending (に, の, だ, です)
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(BuriFollowMatcher)))),
    ]
}

// Pattern: ては (if/when - conditional with negative expectation)
// Structures: Verb[て] + は, い-Adj[て] + は, な-Adj/Noun + では, Verb[て] + ちゃ, Noun + じゃ
pub fn teha() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match は as topic/contrast particle
    #[derive(Debug)]
    struct HaParticleMatcher;
    impl super::Matcher for HaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Match any verb, adjective, noun, or auxiliary that can precede ては/では
    #[derive(Debug)]
    struct PrecedingElementMatcher;
    impl super::Matcher for PrecedingElementMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| {
                pos == "動詞" || pos == "形容詞" || pos == "名詞" || pos == "助動詞"
            })
        }
    }

    // Match て/で/ちゃ/じゃ followed by は (or standalone ちゃ/じゃ)
    #[derive(Debug)]
    struct TeDeJaChaMatcher;
    impl super::Matcher for TeDeJaChaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match て or で as particles (after verb/adjective)
            if (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
            {
                return true;
            }
            // Match で as auxiliary/copula (after na-adj/noun)
            if token.surface == "で"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
            {
                return true;
            }
            // Match ちゃ (contraction of ては)
            if token.surface == "ちゃ" && token.pos.first().is_some_and(|pos| pos == "助詞") {
                return true;
            }
            // Match じゃ (contraction of では)
            if token.surface == "じゃ" && token.pos.first().is_some_and(|pos| pos == "助詞") {
                return true;
            }
            false
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(PrecedingElementMatcher)),
        TokenMatcher::Custom(Arc::new(TeDeJaChaMatcher)),
        // は is only required for て/で forms, not for ちゃ/じゃ contractions
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            HaParticleMatcher,
        )))),
    ]
}

// Pattern: ては〜ては (doing A and B repeatedly)
// Structures: Verb[て] + は + (gap) + Verb[て] + は
pub fn teha_u301c_teha() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match て/で particle (from verb te-form)
    #[derive(Debug)]
    struct TeDeParticleMatcher;
    impl super::Matcher for TeDeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Match ちゃ/じゃ particle (casual contractions that include は)
    #[derive(Debug)]
    struct ChaJaParticleMatcher;
    impl super::Matcher for ChaJaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "ちゃ" || token.surface == "じゃ")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Match は particle
    #[derive(Debug)]
    struct HaKakariMatcher;
    impl super::Matcher for HaKakariMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Match either て/で OR ちゃ/じゃ
    #[derive(Debug)]
    struct TeOrCasualMatcher;
    impl super::Matcher for TeOrCasualMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            if !token.pos.first().is_some_and(|pos| pos == "助詞") {
                return false;
            }
            if !token.pos.get(1).is_some_and(|pos| pos == "接続助詞") {
                return false;
            }
            token.surface == "て"
                || token.surface == "で"
                || token.surface == "ちゃ"
                || token.surface == "じゃ"
        }
    }

    // Pattern: Verb + (て/で/ちゃ/じゃ) + [optional は] + (gap) + Verb + (て/で/ちゃ/じゃ) + [optional は]
    // Note: ちゃ/じゃ already include the は meaning, so は is only needed after て/で
    vec![
        super::flexible_verb_form(),                                                        // First verb
        TokenMatcher::Custom(Arc::new(TeOrCasualMatcher)),                                  // て/で/ちゃ/じゃ
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(HaKakariMatcher)))), // Optional は
        TokenMatcher::Wildcard {
            min: 0,
            max: 15,
            stop_conditions: vec![],
        }, // Gap between patterns (0-15 tokens)
        super::flexible_verb_form(),                                                        // Second verb
        TokenMatcher::Custom(Arc::new(TeOrCasualMatcher)),                                  // て/で/ちゃ/じゃ
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(HaKakariMatcher)))), // Optional は
    ]
}

// Pattern: も又 (also, in addition)
// Structures: Noun/Adj/Verb + (optional nominalizers) + も + また
pub fn momata() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl super::Matcher for MoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    #[derive(Debug)]
    struct MataMatcher;
    impl super::Matcher for MataMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "また"
                && token.pos.first().is_some_and(|pos| pos == "接続詞")
        }
    }

    vec![
        TokenMatcher::Any, // Noun, Adjective, or Verb
        TokenMatcher::Custom(Arc::new(MoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(MataMatcher)),
    ]
}

// Pattern: 結果・の結果 (as a result of) - Verb[た] + 結果
// Structures: Verb[た] + 結果
pub fn kekka_u30fb_nokekka() -> Vec<TokenMatcher> {
    vec![
        super::flexible_verb_form(),
        super::past_auxiliary(),
        kekka_noun_matcher(),
    ]
}

// Pattern: 結果・の結果 (as a result of) - Noun + の + 結果
// Structures: Noun + の + 結果
pub fn kekka_u30fb_nokekka_noun() -> Vec<TokenMatcher> {
    vec![
        super::noun_matcher(),
        no_particle_rentaika_matcher(),
        kekka_noun_matcher(),
    ]
}

// Helper: Match 結果 (result) as 名詞/副詞可能
fn kekka_noun_matcher() -> TokenMatcher {
    use std::sync::Arc;
    #[derive(Debug)]
    struct KekkaMatcher;
    impl super::Matcher for KekkaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "結果"
                && token.base_form == "結果"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }
    TokenMatcher::Custom(Arc::new(KekkaMatcher))
}

// Helper: Match の particle (連体化)
fn no_particle_rentaika_matcher() -> TokenMatcher {
    use std::sync::Arc;
    #[derive(Debug)]
    struct NoParticleMatcher;
    impl super::Matcher for NoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
        }
    }
    TokenMatcher::Custom(Arc::new(NoParticleMatcher))
}

// Pattern: 以来
// Pattern: 以来 (since, ever since)
// Structures: Verb[て] + いらい/以来, Noun + 以来, Demonstrative + 以来
pub fn irai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match いらい (hiragana, 副詞/一般) or 以来 (kanji, 名詞/副詞可能)
    #[derive(Debug)]
    struct IraiMatcher;
    impl super::Matcher for IraiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "いらい" || token.surface == "以来")
                && (token.base_form == "いらい" || token.base_form == "以来")
                && (token.pos.first().is_some_and(|pos| pos == "副詞")
                    || token.pos.first().is_some_and(|pos| pos == "名詞"))
        }
    }

    vec![
        TokenMatcher::Any, // Verb[て] / Noun / Demonstrative
        TokenMatcher::Custom(Arc::new(IraiMatcher)),
    ]
}

// Pattern: に先立ち (prior to, before)
// Structures: Verb/Noun + に + 先立って/先立ち/先立つ
pub fn nisakidachi() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct NiMatcher;
    impl Matcher for NiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    #[derive(Debug)]
    struct SakidatsuMatcher;
    impl Matcher for SakidatsuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "先立つ"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立")
        }
    }

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
        TokenMatcher::Any, // Verb or Noun
        TokenMatcher::Custom(Arc::new(NiMatcher)),
        TokenMatcher::Custom(Arc::new(SakidatsuMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(TeMatcher)))),
    ]
}

// Pattern: はたして (I wonder if, as expected)
// Structures: はたして + (Question/Speculation) / はたして + Statement
pub fn hatashite() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    #[derive(Debug)]
    struct HatashiteMatcher;
    impl super::Matcher for HatashiteMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "はたして"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(HatashiteMatcher))]
}

// Pattern: 甲斐がある (worth doing, pays off)
// Structures: Verb/Noun + かい/がい + がある/がない
pub fn kaigaaru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct KaiGaiMatcher;
    impl Matcher for KaiGaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // かい: 名詞/非自立/一般
            // がい: 名詞/接尾/一般
            (token.surface == "かい" || token.surface == "がい")
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "非自立")
                    || token.pos.get(1).is_some_and(|pos| pos == "接尾"))
        }
    }

    #[derive(Debug)]
    struct GaMatcher;
    impl Matcher for GaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "が"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    #[derive(Debug)]
    struct AruNaiMatcher;
    impl Matcher for AruNaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // ある (動詞) or ない (形容詞)
            (token.base_form == "ある"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立"))
                || (token.surface == "ない"
                    && token.pos.first().is_some_and(|pos| pos == "形容詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "自立"))
        }
    }

    vec![
        TokenMatcher::Any, // Verb (past/stem) or Noun
        TokenMatcher::Custom(Arc::new(KaiGaiMatcher)),
        TokenMatcher::Custom(Arc::new(GaMatcher)),
        TokenMatcher::Custom(Arc::new(AruNaiMatcher)),
    ]
}

// Pattern: やがて (before long, eventually, soon)
// Structures: やがて + Phrase
pub fn yagate() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for やがて (adverb)
    #[derive(Debug)]
    struct YagateMatcher;
    impl super::Matcher for YagateMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "やがて"
                && token.base_form == "やがて"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(YagateMatcher))]
}

// Pattern: したがって (therefore, accordingly)
// Structures: (Cause) + したがって + (Result)
pub fn shitagatte() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    #[derive(Debug)]
    struct ShitagatteMatcher;
    impl super::Matcher for ShitagatteMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "したがって"
                && token.pos.first().is_some_and(|pos| pos == "接続詞")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(ShitagatteMatcher))]
}

// Pattern: あげく (in the end, after all)
// Structures: Verb[た] + あげく / Noun + の + あげく
pub fn ageku() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // あげく is always a noun (can function as adverb)
    #[derive(Debug)]
    struct AgekuMatcher;
    impl super::Matcher for AgekuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "あげく"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副詞可能")
        }
    }

    // の particle matcher for noun + の + あげく pattern
    #[derive(Debug)]
    struct NoParticleMatcher;
    impl super::Matcher for NoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の" && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    // Match: (Noun/Verb) + (optional の) + あげく
    // The optional の handles both verb+あげく and noun+の+あげく patterns
    vec![
        TokenMatcher::Any, // Verb or Noun before あげく
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NoParticleMatcher,
        )))),
        TokenMatcher::Custom(Arc::new(AgekuMatcher)),
    ]
}

// Pattern: きっかけ (opportunity, chance, trigger)
// Structures: を/が + きっかけ + に/で
pub fn kikkake() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for を or が particle
    #[derive(Debug)]
    struct WoGaMatcher;
    impl Matcher for WoGaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "を" || token.surface == "が")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Matcher for きっかけ noun
    #[derive(Debug)]
    struct KikkakeMatcher;
    impl Matcher for KikkakeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "きっかけ"
                && token.base_form == "きっかけ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
        }
    }

    // Matcher for に or で particle
    #[derive(Debug)]
    struct NiDeMatcher;
    impl Matcher for NiDeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "に" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(WoGaMatcher)),
        TokenMatcher::Custom(Arc::new(KikkakeMatcher)),
        TokenMatcher::Custom(Arc::new(NiDeMatcher)),
    ]
}

// Pattern: にかけては (when it comes to, regarding)
// Structures: Noun + にかけては
pub fn nikaketeha() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct NikaketeMatcher;
    impl Matcher for NikaketeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "にかけて"
                && token.base_form == "にかけて"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "連語")
        }
    }

    #[derive(Debug)]
    struct WaKakariMatcher;
    impl Matcher for WaKakariMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    vec![
        TokenMatcher::Any,
        TokenMatcher::Custom(Arc::new(NikaketeMatcher)),
        TokenMatcher::Custom(Arc::new(WaKakariMatcher)),
    ]
}

// Pattern: とっくに (long ago, already, ages ago)
// Structures: とっくに + Phrase
pub fn tokkuni() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct TokkuniMatcher;
    impl Matcher for TokkuniMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "とっくに"
                && token.base_form == "とっくに"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(TokkuniMatcher))]
}

// Pattern: 未だに (still, even now)
// Structures: 未（いま）だに + Verb［る/ない］
pub fn imadani() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Tokenization: Can be either:
    // 1. Single token: いまだに (副詞/一般)
    // 2. Two tokens: いまだ (副詞/助詞類接続) + に (助詞/副詞化)

    // Since we need to match EITHER case, we use a custom matcher on the first token
    // and make the second token optional

    // Match いまだ or いまだに
    #[derive(Debug)]
    struct ImadaOrImadaniMatcher;
    impl Matcher for ImadaOrImadaniMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "副詞")
                && ((token.surface == "いまだに" && token.base_form == "いまだに")
                    || (token.surface == "いまだ" && token.base_form == "いまだ"))
        }
    }

    // Match に as adverbializer (optional - only present in split tokenization)
    #[derive(Debug)]
    struct NiFukushikaMatcher;
    impl Matcher for NiFukushikaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副詞化")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(ImadaOrImadaniMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NiFukushikaMatcher,
        )))),
    ]
}

// Pattern: をもとに (based on)
// Structures: Noun + をもとに（して）, Noun + をもとにした + Noun
pub fn womotoni() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // をもとに = を (case particle) + もと (noun) + に (case particle)
    // Can optionally be followed by する in various forms:
    // - して (te-form)
    // - した (past/pre-noun)

    // Match を as case particle
    #[derive(Debug)]
    struct WoCaseMatcher;
    impl Matcher for WoCaseMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "を"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match もと as noun
    #[derive(Debug)]
    struct MotoNounMatcher;
    impl Matcher for MotoNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "もと"
                && token.base_form == "もと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    // Match に as case particle
    #[derive(Debug)]
    struct NiCaseMatcher;
    impl Matcher for NiCaseMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(WoCaseMatcher)),
        TokenMatcher::Custom(Arc::new(MotoNounMatcher)),
        TokenMatcher::Custom(Arc::new(NiCaseMatcher)),
        // The following particles/verbs (して, した, する, etc.) are optional
        // and will be captured by wildcard if present, but the core pattern is を + もと + に
    ]
}

// Pattern: からには (as long as, since, given that)
// Structures: Verb[る/た] + からには, Adjective + からには, Noun/な-Adj + である + からには
pub fn karaniha() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // からには is always tokenized as a single conjunctive particle
    #[derive(Debug)]
    struct KaranihaMatcher;
    impl Matcher for KaranihaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "からには"
                && token.base_form == "からには"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Use TokenMatcher::Any for simplicity - からには can attach to various forms
    // Note: This will match just the immediately preceding token, which may be:
    // - A verb (dictionary form or stem)
    // - An adjective
    // - An auxiliary (た in verb phrases, ある in である)
    // The full grammatical construction may span multiple tokens, but this
    // is the minimal meaningful unit for pattern detection.
    vec![
        TokenMatcher::Any,
        TokenMatcher::Custom(Arc::new(KaranihaMatcher)),
    ]
}

// Pattern: いつの間にか (before one knows it, suddenly)
// Structures: いつのまにか + Phrase
pub fn itsunomanika() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // いつのまにか is always tokenized as a single adverb token
    #[derive(Debug)]
    struct ItsunomaniकaMatcher;
    impl super::Matcher for ItsunomaniकaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "いつのまにか"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(ItsunomaniकaMatcher))]
}

// Pattern: 一旦 (once, for a moment)
// Structures: 一旦 + Verb[ば/たら] + と
pub fn ittan() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct IttanMatcher;
    impl Matcher for IttanMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "いったん"
                && token.base_form == "いったん"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(IttanMatcher))]
}

// Pattern: はもとより (not only... but also, let alone)
// Structures: Noun/の/こと + は + もとより
pub fn hamotoyori() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct WaKakariMatcher;
    impl Matcher for WaKakariMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.base_form == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    #[derive(Debug)]
    struct MotoyoriMatcher;
    impl Matcher for MotoyoriMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "もとより"
                && token.base_form == "もとより"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
        }
    }

    vec![
        TokenMatcher::Any,
        TokenMatcher::Custom(Arc::new(WaKakariMatcher)),
        TokenMatcher::Custom(Arc::new(MotoyoriMatcher)),
    ]
}

// Pattern: そうにない
pub fn souninai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に反して
pub fn nihanshite() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 逆に
pub fn gyakuni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 反面
pub fn hanmen() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 抜く
pub fn nuku() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 抜きで
pub fn nukide() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: いよいよ (finally, at last, more and more)
// Structures: いよいよ + Phrase
pub fn iyoiyo() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct IyoiyoMatcher;
    impl Matcher for IyoiyoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "いよいよ"
                && token.base_form == "いよいよ"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(IyoiyoMatcher))]
}

// Pattern: ずに済む (get away without doing, can avoid doing)
// Structures: Verb[未然形] + ずに済む / なくて済む / ないで済む
pub fn zunisumu() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ず/なく/ない (negative auxiliaries)
    // - ず (classical auxiliary, base='ぬ', 連用ニ接続)
    // - なく (auxiliary, base='ない', 連用テ接続)
    // - ない (auxiliary, base='ない', 連用デ接続)
    #[derive(Debug)]
    struct NegativeAuxMatcher;
    impl super::Matcher for NegativeAuxMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            if !token.pos.first().is_some_and(|pos| pos == "助動詞") {
                return false;
            }

            // ず variant
            if token.surface == "ず" && token.base_form == "ぬ" {
                return true;
            }

            // なく variant (連用テ接続)
            if token.surface == "なく"
                && token.base_form == "ない"
                && token.features.get(5).is_some_and(|f| f == "連用テ接続") {
                return true;
            }

            // ない variant (連用デ接続)
            if token.surface == "ない"
                && token.base_form == "ない"
                && token.features.get(5).is_some_and(|f| f == "連用デ接続") {
                return true;
            }

            false
        }
    }

    // Match に/て/で particles (depending on which negative aux is used)
    #[derive(Debug)]
    struct NiTeDeParticleMatcher;
    impl super::Matcher for NiTeDeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "に" || token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    // Match 済む verb
    #[derive(Debug)]
    struct SumuMatcher;
    impl super::Matcher for SumuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "済む"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    // Three variants:
    // 1. Verb[未然形] + ず + に + 済む
    // 2. Verb[未然形] + なく + て + 済む
    // 3. Verb[未然形] + ない + で + 済む
    vec![
        TokenMatcher::verb_with_form("未然形"),
        TokenMatcher::Custom(Arc::new(NegativeAuxMatcher)),
        TokenMatcher::Custom(Arc::new(NiTeDeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(SumuMatcher)),
    ]
}

// Pattern: に応じて
pub fn nioujite() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: を通じて・を通して
pub fn wotsuujite_u30fb_wotooshite() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に応えて
pub fn nikotaete() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: それとも
pub fn soretomo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にしたら
pub fn nishitara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にしても～にしても
pub fn nishitemo_uff5e_nishitemo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: としては
pub fn toshiteha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: としても
pub fn toshitemo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: それにしても
pub fn sorenishitemo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ぬ
pub fn nu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ことなく (without doing)
// Structure: Verb[基本形] + こと + なく
// More formal than ないで
pub fn kotonaku() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for verb in dictionary form (基本形)
    #[derive(Debug)]
    struct DictionaryVerbMatcher;
    impl Matcher for DictionaryVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "基本形")
        }
    }

    // Matcher for こと as 名詞/非自立
    #[derive(Debug)]
    struct KotoMatcher;
    impl Matcher for KotoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Matcher for なく (ない in 連用テ接続 form)
    #[derive(Debug)]
    struct NakuMatcher;
    impl Matcher for NakuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "なく"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(DictionaryVerbMatcher)),
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
        TokenMatcher::Custom(Arc::new(NakuMatcher)),
    ]
}

// Pattern: にて
pub fn nite() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: には
pub fn niha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 思うように
pub fn omouyouni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: かと思ったら・かと思うと (just when I thought, no sooner than)
// Structures: Verb[た/る] + (の) + かと思ったら/かと思うと/かと思えば
//            Noun/Adjective + かと思ったら/かと思うと/かと思えば
pub fn katoomottara_u30fb_katoomouto() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    use super::{flexible_verb_form, past_auxiliary, concat};

    // Match か particle (副助詞)
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

    // Match 思う verb (connected to た or in base form)
    #[derive(Debug)]
    struct OmouVerbMatcher;
    impl Matcher for OmouVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "思う"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    // Match たら/と/ば endings
    #[derive(Debug)]
    struct ConditionalEndingMatcher;
    impl Matcher for ConditionalEndingMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // たら (助動詞 仮定形)
            if token.surface == "たら" && token.base_form == "た"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") {
                return true;
            }
            // と (接続助詞)
            if token.surface == "と" && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") {
                return true;
            }
            // ば (接続助詞) - comes after 思え (verb in 仮定形)
            if token.surface == "ば" && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") {
                return true;
            }
            false
        }
    }

    // Optional の (nominalizer)
    #[derive(Debug)]
    struct NoNominalizerMatcher;
    impl Matcher for NoNominalizerMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    // Match verb/noun/adjective or verb+た combination before か
    #[derive(Debug)]
    struct PredicateMatcher;
    impl Matcher for PredicateMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Verbs in base form, nouns, or adjectives
            let is_verb = token.pos.first().is_some_and(|pos| pos == "動詞");
            let is_noun = token.pos.first().is_some_and(|pos| pos == "名詞");
            let is_adj = token.pos.first().is_some_and(|pos| pos == "形容詞");

            is_verb || is_noun || is_adj
        }
    }

    // For Verb[た] + かと思ったら: Verb + た/だ + か...
    // For Verb[る] + (の) + かと思うと: Verb + (の) + か...
    // For Noun/Adj + かと思ったら: Noun/Adj + か...
    concat(vec![
        vec![TokenMatcher::Custom(Arc::new(PredicateMatcher))],
        vec![TokenMatcher::Optional(Box::new(past_auxiliary()))],  // Optional た/だ for past verb
        vec![TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NoNominalizerMatcher))))],  // Optional の
        vec![TokenMatcher::Custom(Arc::new(KaParticleMatcher))],
        vec![TokenMatcher::Custom(Arc::new(ToQuotativeMatcher))],
        vec![TokenMatcher::Custom(Arc::new(OmouVerbMatcher))],
        vec![TokenMatcher::Custom(Arc::new(ConditionalEndingMatcher))],
    ])
}

// Pattern: というものでもない
pub fn toiumonodemonai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: と考えられる
pub fn tokangaerareru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: という点から考えると
pub fn toiutenkarakangaeruto() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ということは
pub fn toiukotoha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ふうに
pub fn fuuni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: という風に
pub fn toiukazeni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ものの
pub fn monono() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: というものだ
pub fn toiumonoda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: から見ると (from the perspective of, judging from)
// Structures: Noun + から + 見る + と/ば/て/たら
pub fn karamiruto() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match から as case particle
    #[derive(Debug)]
    struct KaraMatcher;
    impl super::Matcher for KaraMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "から"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match みる verb in any conjugation form (基本形, 仮定形, 連用形)
    #[derive(Debug)]
    struct MiruVerbMatcher;
    impl super::Matcher for MiruVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "みる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立")
        }
    }

    // Match と, ば, て, or たら as ending
    #[derive(Debug)]
    struct EndingMatcher;
    impl super::Matcher for EndingMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // と or ば or て (all connective particles)
            if (token.surface == "と" || token.surface == "ば" || token.surface == "て")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
            {
                return true;
            }
            // たら (auxiliary verb in hypothetical form)
            if token.surface == "たら"
                && token.base_form == "た"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
            {
                return true;
            }
            false
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(KaraMatcher)),
        TokenMatcher::Custom(Arc::new(MiruVerbMatcher)),
        TokenMatcher::Custom(Arc::new(EndingMatcher)),
    ]
}

// Pattern: ところを見ると
pub fn tokorowomiruto() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: からすると・からすれば (judging from, considering)
// Structures: Noun + から + する + と/ば
pub fn karasuruto_u30fb_karasureba() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match から as case particle
    #[derive(Debug)]
    struct KaraMatcher;
    impl super::Matcher for KaraMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "から"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match する verb in either base form (基本形) or hypothetical form (仮定形)
    #[derive(Debug)]
    struct SuruVerbMatcher;
    impl super::Matcher for SuruVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "する"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立")
                && (token.features.get(5).is_some_and(|f| f == "基本形")
                    || token.features.get(5).is_some_and(|f| f == "仮定形"))
        }
    }

    // Match と or ば as connective particle
    #[derive(Debug)]
    struct ToOrBaMatcher;
    impl super::Matcher for ToOrBaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "と" || token.surface == "ば")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(KaraMatcher)),
        TokenMatcher::Custom(Arc::new(SuruVerbMatcher)),
        TokenMatcher::Custom(Arc::new(ToOrBaMatcher)),
    ]
}

// Pattern: からして (based on, judging from)
// Structures: Noun + から + し + て
pub fn karashite() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match から as case particle
    #[derive(Debug)]
    struct KaraMatcher;
    impl Matcher for KaraMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "から"
                && token.base_form == "から"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match し as the 連用形 of する
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

    // Match て as connective particle
    #[derive(Debug)]
    struct TeMatcher;
    impl Matcher for TeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.base_form == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    vec![
        TokenMatcher::Any, // Noun
        TokenMatcher::Custom(Arc::new(KaraMatcher)),
        TokenMatcher::Custom(Arc::new(ShiMatcher)),
        TokenMatcher::Custom(Arc::new(TeMatcher)),
    ]
}

// Pattern: からといって (just because)
// Structures: Verb/い-Adj + から + と + いって, な-Adj/Noun + だ + から + と + いって
pub fn karatoitte() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match から as connective particle
    #[derive(Debug)]
    struct KaraConnectiveMatcher;
    impl Matcher for KaraConnectiveMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "から"
                && token.base_form == "から"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Match と as quotation particle
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

    // Match いう verb (various forms: いっ, いわ, etc.)
    #[derive(Debug)]
    struct IuVerbMatcher;
    impl Matcher for IuVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "いう"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    // Match て as connective particle
    #[derive(Debug)]
    struct TeConnectiveMatcher;
    impl Matcher for TeConnectiveMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.base_form == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    vec![
        TokenMatcher::Any, // Verb, い-Adj, or だ (for な-Adj/Noun)
        TokenMatcher::Custom(Arc::new(KaraConnectiveMatcher)),
        TokenMatcher::Custom(Arc::new(ToQuotationMatcher)),
        TokenMatcher::Custom(Arc::new(IuVerbMatcher)),
        TokenMatcher::Custom(Arc::new(TeConnectiveMatcher)),
    ]
}

// Pattern: そういえば
pub fn souieba() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: お～願う (humble request: please do)
// Structures:
//   - お + Verb[stem] + 願う/願います
//   - ご + Chinese-origin Noun + 願う/願います
//   - Western-origin Noun + 願う/願います (no prefix)
pub fn o_uff5e_negau() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for お or ご prefix (optional for Western nouns)
    #[derive(Debug)]
    struct OGoMatcher;
    impl Matcher for OGoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "お" || token.surface == "ご")
                && token.pos.first().is_some_and(|pos| pos == "接頭詞")
                && token.pos.get(1).is_some_and(|pos| pos == "名詞接続")
        }
    }

    // Matcher for verb stem or sahen noun before 願う
    // This should match: Verb[連用形] OR Noun[サ変接続]
    #[derive(Debug)]
    struct VerbOrNounMatcher;
    impl Matcher for VerbOrNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Verb in conjunctive form (連用形)
            let is_verb_stem = token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形");

            // Sahen-setsuzoku noun (can become verb with する)
            let is_sahen_noun = token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "サ変接続");

            is_verb_stem || is_sahen_noun
        }
    }

    // Matcher for 願う verb (either 非自立 after verb, or 自立 after noun)
    #[derive(Debug)]
    struct NegauVerbMatcher;
    impl Matcher for NegauVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "願う"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "非自立")
                    || token.pos.get(1).is_some_and(|pos| pos == "自立"))
        }
    }

    // Matcher for ます auxiliary (optional)
    #[derive(Debug)]
    struct MasuMatcher;
    impl Matcher for MasuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(OGoMatcher)))),
        TokenMatcher::Custom(Arc::new(VerbOrNounMatcher)),
        TokenMatcher::Custom(Arc::new(NegauVerbMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(MasuMatcher)))),
    ]
}

// Pattern: ～て頂戴
pub fn uff5e_techoudai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: とか
pub fn toka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 〜ようではないか (why don't we, let's)
// Structures: Verb[volitional] + (で/じゃ) + は? + ない + か
// Two tokenization patterns:
//   1. Godan verbs: Verb[未然ウ接続] + う(助動詞) + ...
//   2. Ichidan verbs: Verb[連用形] + よう(名詞/接尾) + ...
pub fn u301c_youdehanaika() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match verb in volitional form - handles both tokenization patterns
    #[derive(Debug)]
    struct VolitionalVerbMatcher;
    impl Matcher for VolitionalVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && (token.features.get(5).is_some_and(|f| f == "未然ウ接続") // Godan: 戦お
                    || token.features.get(5).is_some_and(|f| f == "連用形"))  // Ichidan: 考え
        }
    }

    // Match う auxiliary (for godan) or よう suffix (for ichidan)
    #[derive(Debug)]
    struct VolitionalAuxiliaryMatcher;
    impl Matcher for VolitionalAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // う auxiliary (godan verbs: 戦おう)
            (token.surface == "う"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "う")
            ||
            // よう suffix (ichidan verbs: 考えよう)
            (token.surface == "よう"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾"))
        }
    }

    // Match で (copula da in te-form) or じゃ (abbreviated)
    #[derive(Debug)]
    struct DeOrJaMatcher;
    impl Matcher for DeOrJaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // で from だ (copula) - used after godan verbs
            (token.surface == "で"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞"))
            ||
            // で as case particle - used after よう (noun suffix) in ichidan verbs
            (token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞"))
            ||
            // じゃ (abbreviated)
            (token.surface == "じゃ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞"))
        }
    }

    // Match は (only for ではないか, not for じゃないか)
    #[derive(Debug)]
    struct HaParticleMatcher;
    impl Matcher for HaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Match ない auxiliary
    #[derive(Debug)]
    struct NaiAuxiliaryMatcher;
    impl Matcher for NaiAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "ない"
        }
    }

    // Match か ending particle
    #[derive(Debug)]
    struct KaParticleMatcher;
    impl Matcher for KaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "か"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VolitionalVerbMatcher)),
        TokenMatcher::Custom(Arc::new(VolitionalAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(DeOrJaMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(HaParticleMatcher)))),
        TokenMatcher::Custom(Arc::new(NaiAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(KaParticleMatcher)),
    ]
}

// Pattern: かのようだ (as if, seems like)
// Structures: Any + か + の + よう + だ/です/に/な
pub fn kanoyouda() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for か particle (adverbial/parallel/sentence-ending)
    #[derive(Debug)]
    struct KaMatcher;
    impl Matcher for KaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "か"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token
                    .pos
                    .get(1)
                    .is_some_and(|pos| pos == "副助詞／並立助詞／終助詞")
        }
    }

    // Matcher for の particle (nominalization/adnominal)
    #[derive(Debug)]
    struct NoMatcher;
    impl Matcher for NoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
        }
    }

    // Matcher for よう (auxiliary verb stem)
    #[derive(Debug)]
    struct YouMatcher;
    impl Matcher for YouMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "よう"
                && token.base_form == "よう"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                && token.pos.get(2).is_some_and(|pos| pos == "助動詞語幹")
        }
    }

    // Matcher for ending: だ/です/に/な
    #[derive(Debug)]
    struct EndingMatcher;
    impl Matcher for EndingMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // だ or です (auxiliary verb)
            if (token.surface == "だ" || token.surface == "です")
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
            {
                return true;
            }
            // に (adverbializer)
            if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副詞化")
            {
                return true;
            }
            // な (adnominal form of だ)
            if token.surface == "な"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token
                    .features
                    .get(5)
                    .is_some_and(|f| f == "体言接続")
            {
                return true;
            }
            false
        }
    }

    vec![
        TokenMatcher::Any,
        TokenMatcher::Custom(Arc::new(KaMatcher)),
        TokenMatcher::Custom(Arc::new(NoMatcher)),
        TokenMatcher::Custom(Arc::new(YouMatcher)),
        TokenMatcher::Custom(Arc::new(EndingMatcher)),
    ]
}

// Pattern: のではないだろうか
pub fn nodehanaidarouka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: て当然だ
pub fn tetouzenda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: のも当然だ
pub fn nomotouzenda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: たった(の)
pub fn tatta_no() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 恐れがある
pub fn osoregaaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: おそらく (probably, perhaps)
// Structures: おそらく + Phrase
pub fn osoraku() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct OsorakuMatcher;
    impl Matcher for OsorakuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "おそらく"
                && token.base_form == "おそらく"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(OsorakuMatcher))]
}

// Pattern: ものか
pub fn monoka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: おまけに (besides, in addition, to make matters worse)
// Structure: おまけ + に
pub fn omakeni() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct OmakeMatcher;
    impl super::Matcher for OmakeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "おまけ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(OmakeMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
    ]
}

// Pattern: に決まっている
pub fn nikimatteiru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ことになっている (it is expected / scheduled to)
// Structures: (Verb[る]/Verb[ない]) + こと + に + なっている/なっています
pub fn kotoninatteiru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match こと as 名詞/非自立
    #[derive(Debug)]
    struct KotoMatcher;
    impl Matcher for KotoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こと"
                && token.base_form == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Match に as 助詞/格助詞
    #[derive(Debug)]
    struct NiMatcher;
    impl Matcher for NiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.base_form == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match なる in 連用タ接続 (なっ)
    #[derive(Debug)]
    struct NaruMatcher;
    impl Matcher for NaruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "なる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    // Match て as 助詞/接続助詞
    #[derive(Debug)]
    struct TeMatcher;
    impl Matcher for TeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.base_form == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Match いる or います (動詞/非自立 or with ます)
    // We need to match just いる, and let the pattern matcher extend to います if present
    #[derive(Debug)]
    struct IruMatcher;
    impl Matcher for IruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "いる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
        TokenMatcher::Custom(Arc::new(NiMatcher)),
        TokenMatcher::Custom(Arc::new(NaruMatcher)),
        TokenMatcher::Custom(Arc::new(TeMatcher)),
        TokenMatcher::Custom(Arc::new(IruMatcher)),
    ]
}

// Pattern: 気
pub fn ki() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: げ (seeming, appearance)
// Structures: Adj[stem]/Verb[連用形]/Noun + げ + に/な
pub fn ge() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for げ suffix (名詞/接尾/一般)
    #[derive(Debug)]
    struct GeMatcher;
    impl Matcher for GeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "げ"
                && token.base_form == "げ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                && token.pos.get(2).is_some_and(|pos| pos == "一般")
        }
    }

    // Matcher for に/な ending
    #[derive(Debug)]
    struct NiNaMatcher;
    impl Matcher for NiNaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // に (case particle)
            if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            {
                return true;
            }
            // な (adnominal form of だ)
            if token.surface == "な"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token
                    .features
                    .get(5)
                    .is_some_and(|f| f == "体言接続")
            {
                return true;
            }
            false
        }
    }

    vec![
        TokenMatcher::Any,
        TokenMatcher::Custom(Arc::new(GeMatcher)),
        TokenMatcher::Custom(Arc::new(NiNaMatcher)),
    ]
}

// Pattern: ことだから (it is exactly because / precisely because)
// Structures: (Verb/Noun + の/な-Adj + な) + こと + (だ/です) + から
pub fn kotodakara() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match こと as 名詞/非自立
    #[derive(Debug)]
    struct KotoMatcher;
    impl Matcher for KotoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こと"
                && token.base_form == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Match だ or です (助動詞)
    #[derive(Debug)]
    struct DaDesuMatcher;
    impl Matcher for DaDesuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "だ" || token.surface == "です")
                && (token.base_form == "だ" || token.base_form == "です")
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Match から as 助詞/接続助詞
    #[derive(Debug)]
    struct KaraMatcher;
    impl Matcher for KaraMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "から"
                && token.base_form == "から"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
        TokenMatcher::Custom(Arc::new(DaDesuMatcher)),
        TokenMatcher::Custom(Arc::new(KaraMatcher)),
    ]
}

// Pattern: ものだから
pub fn monodakara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ものですから・もので
pub fn monodesukara_u30fb_monode() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ものがある
pub fn monogaaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 傾向がある
pub fn keikougaaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～に値する
pub fn uff5e_niataisuru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: てしょうがない
pub fn teshouganai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: だけましだ
pub fn dakemashida() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 幸い・幸いなことに
pub fn saiwai_u30fb_saiwainakotoni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ようでは・ようじゃ
pub fn youdeha_u30fb_youja() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: さすが (as expected of / that is just like)
// Structures: さすが + （に）+ Phrase, さすが + （の）+ Noun
pub fn sasuga() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct SasugaMatcher;
    impl Matcher for SasugaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "さすが"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
        }
    }

    #[derive(Debug)]
    struct NiNoParticleMatcher;
    impl Matcher for NiNoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "に" && token.pos.first().is_some_and(|pos| pos == "助詞"))
                || (token.surface == "の" && token.pos.first().is_some_and(|pos| pos == "助詞"))
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(SasugaMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NiNoParticleMatcher)))),
    ]
}

// Pattern: ことは〜が - "(A) is true, but (B)" / "although (A), (B)"
// Structures: Word + ことは + Word(*) + が/けど/けれど/けども
// (*) The same word is repeated before and after ことは
// Note: This matcher cannot validate that the words are the same (would require state tracking)
//       but it matches the structural pattern
pub fn kotoha_u301c_ga() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match こと (non-independent noun)
    #[derive(Debug)]
    struct KotoNounMatcher;
    impl Matcher for KotoNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Match は (topic particle)
    #[derive(Debug)]
    struct WaParticleMatcher;
    impl Matcher for WaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Match が/けど/けれど/けども (conjunctive particles)
    #[derive(Debug)]
    struct GaKedoMatcher;
    impl Matcher for GaKedoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "が" || token.surface == "けど"
                || token.surface == "けれど" || token.surface == "けれども")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    vec![
        TokenMatcher::Any,  // Preceding word (verb, adjective, or noun)
        TokenMatcher::Custom(Arc::new(KotoNounMatcher)),
        TokenMatcher::Custom(Arc::new(WaParticleMatcher)),
        TokenMatcher::Wildcard {
            min: 1,
            max: 3,
            stop_conditions: vec![],
        },  // Repeated word - 1 token for verbs/い-adj, 2-3 tokens for な-adj/noun (である, だ, etc.)
        TokenMatcher::Custom(Arc::new(GaKedoMatcher)),
    ]
}

// Pattern: 更に
pub fn sarani() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 精々
pub fn kiyoshi_u3005() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 僅かに
pub fn wazukani() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: および (and, as well as)
// Structures: Noun + および
pub fn oyobi() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // および can be tokenized as either:
    // - 助詞/接続助詞 (connective particle) when following a noun directly
    // - 接続詞 (conjunction) when starting a clause (e.g., after punctuation)
    #[derive(Debug)]
    struct OyobiMatcher;
    impl super::Matcher for OyobiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            if token.surface != "および" {
                return false;
            }
            // Match either conjunction or connective particle
            let is_conjunction = token.pos.first().is_some_and(|pos| pos == "接続詞");
            let is_particle = token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞");
            is_conjunction || is_particle
        }
    }

    // Punctuation matcher for optional comma before および
    #[derive(Debug)]
    struct PunctuationMatcher;
    impl super::Matcher for PunctuationMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "記号")
                && token.pos.get(1).is_some_and(|pos| pos == "読点")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            PunctuationMatcher,
        )))),
        TokenMatcher::Custom(Arc::new(OyobiMatcher)),
    ]
}

// Pattern: たちまち
pub fn tachimachi() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: いきなり (suddenly, all of a sudden)
// Structures: いきなり + (Action) Phrase
pub fn ikinari() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct IkinariMatcher;
    impl Matcher for IkinariMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "いきなり"
                && token.base_form == "いきなり"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token
                    .pos
                    .get(1)
                    .is_some_and(|pos| pos == "助詞類接続")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(IkinariMatcher))]
}

// Pattern: といった
pub fn toitta() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: を込めて
pub fn wokomete() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に加えて
pub fn nikuwaete() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 何から何まで
pub fn nanikarananimade() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: は別として
pub fn habetsutoshite() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: だけに
pub fn dakeni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: だけは
pub fn dakeha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: だけあって
pub fn dakeatte() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 何より
pub fn naniyori() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 何といっても
pub fn nanitoittemo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: か何か (or something, or something like that)
// Structures: Noun + か + なに + か (or かなにか as single adverb)
pub fn kananika() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match か as particle
    #[derive(Debug)]
    struct KaMatcher;
    impl super::Matcher for KaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "か"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token
                    .pos
                    .get(1)
                    .is_some_and(|pos| pos == "副助詞／並立助詞／終助詞")
        }
    }

    // Match なに (pronoun) OR なにか (adverb)
    #[derive(Debug)]
    struct NaniOrNanikaMatcher;
    impl super::Matcher for NaniOrNanikaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // なに as pronoun
            if token.surface == "なに"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "代名詞")
            {
                return true;
            }
            // かなにか as single adverb token
            if token.surface == "なにか"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
            {
                return true;
            }
            false
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(KaMatcher)),
        TokenMatcher::Custom(Arc::new(NaniOrNanikaMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(KaMatcher)))),
    ]
}

// Pattern: てならない
pub fn tenaranai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: のみならず
pub fn nominarazu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: それなのに
pub fn sorenanoni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: いわゆる (so-called, what is called)
// Structure: いわゆる + Noun
pub fn iwayuru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct IwayuruMatcher;
    impl super::Matcher for IwayuruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "いわゆる"
                && token.pos.first().is_some_and(|pos| pos == "連体詞")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(IwayuruMatcher))]
}

// Pattern: にすぎない
pub fn nisuginai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: も～ば～も
pub fn mo_uff5e_ba_uff5e_mo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: でしかない
pub fn deshikanai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: てたまらない
pub fn tetamaranai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にせよ・にしろ
pub fn niseyo_u30fb_nishiro() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 何しろ
pub fn nanishiro() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にしろ～にしろ
pub fn nishiro_uff5e_nishiro() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: はともかく
pub fn hatomokaku() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ならともかく (if it's A, sure, but...)
// Structures: Any + なら (助動詞, 仮定形) + ともかく (副詞)
pub fn naratomokaku() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for なら (助動詞, 仮定形, base=だ)
    #[derive(Debug)]
    struct NaraMatcher;
    impl super::Matcher for NaraMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "なら"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "だ"
                && token.features.get(5).is_some_and(|form| form == "仮定形")
        }
    }

    // Matcher for ともかく (副詞/一般)
    #[derive(Debug)]
    struct TomokakuMatcher;
    impl super::Matcher for TomokakuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ともかく"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
        }
    }

    vec![
        TokenMatcher::Any,
        TokenMatcher::Custom(Arc::new(NaraMatcher)),
        TokenMatcher::Custom(Arc::new(TomokakuMatcher)),
    ]
}

// Pattern: やら～やら (A and B, and so on)
// Structures: A + やら + B + やら (where A/B can be Verb[る], Noun, い-Adj)
pub fn yara_uff5e_yara() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for やら (並立助詞 or 終助詞)
    #[derive(Debug)]
    struct YaraMatcher;
    impl super::Matcher for YaraMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "やら"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "並立助詞")
                    || token.pos.get(1).is_some_and(|pos| pos == "終助詞"))
        }
    }

    vec![
        TokenMatcher::Any,  // Matches preceding word (verb, adjective, noun)
        TokenMatcher::Custom(Arc::new(YaraMatcher)),
    ]
}

// Pattern: しかしながら (however, nevertheless)
// Structures: Phrase。 しかしながら + Phrase
pub fn shikashinagara() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct ShikashiNagaraMatcher;
    impl super::Matcher for ShikashiNagaraMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "しかしながら"
                && token.pos.first().is_some_and(|pos| pos == "接続詞")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(ShikashiNagaraMatcher))]
}

// Pattern: ことにはならない - "just because (A), it doesn't mean that (B)"
// Structures: Phrase + ことにはならない/ことにはなりません
// Optional という before ことにはならない for emphasis
pub fn kotonihanaranai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match こと (non-independent noun)
    #[derive(Debug)]
    struct KotoNounMatcher;
    impl Matcher for KotoNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Match に (case particle)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match は (topic particle)
    #[derive(Debug)]
    struct WaParticleMatcher;
    impl Matcher for WaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Match という (optional, for emphasis)
    #[derive(Debug)]
    struct ToiuParticleMatcher;
    impl Matcher for ToiuParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "という"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        TokenMatcher::Any,  // Preceding phrase (can be anything)
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(ToiuParticleMatcher)))),
        TokenMatcher::Custom(Arc::new(KotoNounMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(WaParticleMatcher)),
        TokenMatcher::specific_verb("なる"),  // Matches both なら (未然形) and なり (連用形)
        TokenMatcher::Any,  // Matches ない (negative) OR ませ (polite continuation)
    ]
}

// Pattern: だけのことはある (no wonder, as expected)
// Structures: [Verb/い-Adj/Noun/な-Adj] + だけのことはある/ありま��
pub fn dakenokotohaaru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for だけ (副助詞)
    #[derive(Debug)]
    struct DakeMatcher;
    impl super::Matcher for DakeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "だけ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞")
        }
    }

    // Matcher for の (連体化)
    #[derive(Debug)]
    struct NoRentaikaMatcher;
    impl super::Matcher for NoRentaikaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
        }
    }

    // Matcher for こと (名詞/非自立/一般)
    #[derive(Debug)]
    struct KotoMatcher;
    impl super::Matcher for KotoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Matcher for は (係助詞)
    #[derive(Debug)]
    struct WaKakariMatcher;
    impl super::Matcher for WaKakariMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Matcher for ある (動詞/自立) - matches both 基本形 and 連用形 (for ありません, あります)
    #[derive(Debug)]
    struct AruVerbMatcher;
    impl super::Matcher for AruVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "ある"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立")
        }
    }

    vec![
        TokenMatcher::Any,  // Matches preceding word (verb, adjective, noun, etc.)
        TokenMatcher::Custom(Arc::new(DakeMatcher)),
        TokenMatcher::Custom(Arc::new(NoRentaikaMatcher)),
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
        TokenMatcher::Custom(Arc::new(WaKakariMatcher)),
        TokenMatcher::Custom(Arc::new(AruVerbMatcher)),
    ]
}

// Pattern: てはならない (must not do)
// Structures: Verb[て/で] + は + ならない/なりません
pub fn tehanaranai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for て/で (conjunction particle from verb)
    #[derive(Debug)]
    struct TeDeParticleMatcher;
    impl super::Matcher for TeDeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Matcher for は particle
    #[derive(Debug)]
    struct WaKakariMatcher;
    impl super::Matcher for WaKakariMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Matcher for なる (in 未然形 or 連用形) - for ならない or なりません
    #[derive(Debug)]
    struct NaruFormMatcher;
    impl super::Matcher for NaruFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "なる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                && (token.features.get(5).is_some_and(|f| f == "未然形")
                    || token.features.get(5).is_some_and(|f| f == "連用形"))
        }
    }

    vec![
        super::flexible_verb_form(), // Matches 連用形 or 連用タ接続
        TokenMatcher::Custom(Arc::new(TeDeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(WaKakariMatcher)),
        TokenMatcher::Custom(Arc::new(NaruFormMatcher)),
        TokenMatcher::Any, // Matches ない (助動詞) or ませ + ん
    ]
}

// Pattern: てはいられない (cannot afford to, unable to)
// Structures: Verb[て/で] + は + いられない, い-Adj[て] + は + いられない, な-Adj/Noun + では + いられない
pub fn tehairarenai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for て/で (conjunction particle OR copula)
    #[derive(Debug)]
    struct TeDeWaMatcher;
    impl super::Matcher for TeDeWaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // て/で as conjunction particle (from verb/adjective)
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                ||
            // で as copula (from な-adjective/noun)
            (token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "だ")
        }
    }

    // Matcher for は particle
    #[derive(Debug)]
    struct WaParticleMatcher;
    impl super::Matcher for WaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Matcher for い (いる in 未然形)
    #[derive(Debug)]
    struct IruMizenMatcher;
    impl super::Matcher for IruMizenMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "い"
                && token.base_form == "いる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然形")
        }
    }

    // Matcher for られ (られる in 未然形)
    #[derive(Debug)]
    struct RareMatcher;
    impl super::Matcher for RareMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "られ"
                && token.base_form == "られる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然形")
        }
    }

    // Matcher for ない (negative auxiliary)
    #[derive(Debug)]
    struct NaiMatcher;
    impl super::Matcher for NaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(TeDeWaMatcher)),
        TokenMatcher::Custom(Arc::new(WaParticleMatcher)),
        TokenMatcher::Custom(Arc::new(IruMizenMatcher)),
        TokenMatcher::Custom(Arc::new(RareMatcher)),
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
    ]
}

// Pattern: 陸に～ない (barely, hardly, not properly)
// Structure: ろくに + (Negative) Phrase
pub fn rikuni_uff5e_nai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for ろくに adverb (can be written as ろくに, 陸に, or 碌に)
    #[derive(Debug)]
    struct RokuNiMatcher;
    impl super::Matcher for RokuNiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "ろくに" || token.surface == "陸に" || token.surface == "碌に")
                && token.pos.first().is_some_and(|pos| pos == "副詞")
        }
    }

    // Matcher for negative auxiliary ない/なかった
    #[derive(Debug)]
    struct NaiMatcher;
    impl super::Matcher for NaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "助動詞")
                && (token.base_form == "ない" || token.surface == "ない" || token.surface == "なかった")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(RokuNiMatcher)),
        TokenMatcher::Wildcard {
            min: 0,
            max: 10,
            stop_conditions: vec![],
        },
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
    ]
}

// Pattern: しかも (moreover, furthermore)
// Structure: しかも (conjunction)
pub fn shikamo() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    #[derive(Debug)]
    struct ShikamoMatcher;
    impl super::Matcher for ShikamoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "しかも" && token.pos.first().is_some_and(|pos| pos == "接続詞")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(ShikamoMatcher))]
}

// Pattern: てでも (even if I have to)
// Structures: Verb[て] + でも
pub fn tedemo() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for て/で conjunction particle
    #[derive(Debug)]
    struct TeDeConjunctionMatcher;
    impl super::Matcher for TeDeConjunctionMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Matcher for でも (can be either 助詞/副助詞 or 接続詞)
    #[derive(Debug)]
    struct DemoParticleMatcher;
    impl super::Matcher for DemoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "でも"
                && (token.pos.first().is_some_and(|pos| pos == "助詞")
                    || token.pos.first().is_some_and(|pos| pos == "接続詞"))
        }
    }

    vec![
        super::flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(TeDeConjunctionMatcher)),
        TokenMatcher::Custom(Arc::new(DemoParticleMatcher)),
    ]
}

// Pattern: とも (even if, no matter if)
// Structures:
//   - Verb[未然ウ接続] + う + とも (volitional form + とも)
//   - い-Adj[連用テ接続/く] + とも (conjunctive form + とも)
//   - い-Adj[未然ウ接続/かろう] + う + とも (alternate volitional-like form)
//   - な-Adj + で + あろ + う + とも (であろう + とも)
pub fn tomo() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match とも particle (conjunction particle)
    #[derive(Debug)]
    struct TomoParticle;
    impl super::Matcher for TomoParticle {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "とも"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
        }
    }

    // This pattern needs to match multiple different structures, so we use TokenMatcher::Any
    // with a custom post-validation in the first token that checks the entire sequence.
    // We'll create a matcher that looks ahead to validate the full とも pattern.

    #[derive(Debug)]
    struct TomoPatternMatcher;
    impl super::Matcher for TomoPatternMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Check if this token could be the start of any とも pattern:
            // 1. Verb in 未然ウ接続 (for おう/よう + とも)
            // 2. い-Adjective in 連用テ接続 (for く + とも)
            // 3. い-Adjective in 未然ウ接続 (for かろう + とも)
            // 4. な-Adjective (for で + あろ + う + とも)

            let is_verb_mizen = token.pos.first().is_some_and(|p| p == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然ウ接続");

            let is_i_adj_conjunctive = token.pos.first().is_some_and(|p| p == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "連用テ接続");

            let is_i_adj_mizen = token.pos.first().is_some_and(|p| p == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "未然ウ接続");

            let is_na_adj = token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "形容動詞語幹");

            is_verb_mizen || is_i_adj_conjunctive || is_i_adj_mizen || is_na_adj
        }
    }

    // Match volitional auxiliary う (for verb/adjective volitional + とも)
    #[derive(Debug)]
    struct VolitionalU;
    impl super::Matcher for VolitionalU {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "う"
                && token.pos.first().is_some_and(|p| p == "助動詞")
                && token.base_form == "う"
        }
    }

    // Match で from だ (for な-adjective + であろう)
    #[derive(Debug)]
    struct DeCopula;
    impl super::Matcher for DeCopula {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "で"
                && token.pos.first().is_some_and(|p| p == "助動詞")
                && token.base_form == "だ"
        }
    }

    // Match あろ (from ある, for であろう)
    #[derive(Debug)]
    struct Aro;
    impl super::Matcher for Aro {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "あろ"
                && token.pos.first().is_some_and(|p| p == "助動詞")
                && token.base_form == "ある"
        }
    }

    // We need to use a flexible pattern that can match different sequences.
    // The pattern engine will try to match: First token + optional tokens + final とも
    // We'll use wildcards with specific matchers for the components.

    // Actually, let's use multiple pattern variants - this is cleaner.
    // We'll match the shortest common pattern and let the wildcards handle variations.
    // Pattern: (Verb/Adj) + (optional う) + (optional で + あろ + う) + とも

    vec![
        TokenMatcher::Custom(Arc::new(TomoPatternMatcher)),
        TokenMatcher::Wildcard {
            min: 0,
            max: 4,  // Max: で + あろ + う + とも = 4 tokens before とも
            stop_conditions: vec![],
        },
        TokenMatcher::Custom(Arc::new(TomoParticle)),
    ]
}

// Pattern: ないわけにはいかない (can't not do, must do)
// Structures: Verb[未然形] + ない + わけにはいかない/わけにはいきません
pub fn naiwakenihaikanai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ない auxiliary after verb
    #[derive(Debug)]
    struct NaiAuxiliary;
    impl super::Matcher for NaiAuxiliary {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ない"
                && token.pos.first().is_some_and(|p| p == "助動詞")
        }
    }

    // Match わけ as noun
    #[derive(Debug)]
    struct WakeMatcher;
    impl super::Matcher for WakeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "わけ"
                && token.pos.first().is_some_and(|p| p == "名詞")
        }
    }

    // Match いく verb (未然形 for ない, 連用形 for ません)
    #[derive(Debug)]
    struct IkuVerb;
    impl super::Matcher for IkuVerb {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "いく"
                && token.pos.first().is_some_and(|p| p == "動詞")
                && (token.features.get(5).is_some_and(|f| f == "未然形")
                    || token.features.get(5).is_some_and(|f| f == "連用形"))
        }
    }

    // Match ない, ませ, or ん as ending
    #[derive(Debug)]
    struct EndingMatcher;
    impl super::Matcher for EndingMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|p| p == "助動詞")
                && (token.surface == "ない"
                    || token.surface == "ませ"
                    || token.surface == "ん")
        }
    }

    // Standard: Verb[未然形] + ない + わけ + に + は + いか + ない
    // Polite: Verb[未然形] + ない + わけ + に + は + いき + ませ + ん
    vec![
        TokenMatcher::verb_with_form("未然形"),
        TokenMatcher::Custom(Arc::new(NaiAuxiliary)),
        TokenMatcher::Custom(Arc::new(WakeMatcher)),
        TokenMatcher::Surface("に"),
        TokenMatcher::Surface("は"),
        TokenMatcher::Custom(Arc::new(IkuVerb)),
        TokenMatcher::Custom(Arc::new(EndingMatcher)),
        // Optional second ending token (ん after ませ in polite form)
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(EndingMatcher)))),
    ]
}

// Pattern: というわけではない (doesn't mean that, it's not that)
// Structures: [Verb/Adj/Noun] + という + わけ + では/じゃ + ない/ありません
pub fn toiuwakedehanai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match という as quotation particle
    #[derive(Debug)]
    struct ToiuMatcher;
    impl Matcher for ToiuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "という"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    // Match わけ as non-independent noun
    #[derive(Debug)]
    struct WakeMatcher;
    impl Matcher for WakeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "わけ"
                && token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "非自立")
        }
    }

    // Match で (from だ) or じゃ
    #[derive(Debug)]
    struct DeJaMatcher;
    impl Matcher for DeJaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // で from だ (助動詞)
            (token.surface == "で"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|p| p == "助動詞"))
            // Or じゃ (助詞/副助詞)
            || (token.surface == "じゃ"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "副助詞"))
        }
    }

    // Match は particle (optional for じゃ form)
    #[derive(Debug)]
    struct HaParticleMatcher;
    impl Matcher for HaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "係助詞")
        }
    }

    // Match ない (form depends on context)
    #[derive(Debug)]
    struct NaiMatcher;
    impl Matcher for NaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ない"
                && (
                    // As auxiliary verb (after じゃ)
                    token.pos.first().is_some_and(|p| p == "助動詞")
                    // As adjective (after では)
                    || token.pos.first().is_some_and(|p| p == "形容詞")
                )
        }
    }

    // Match ありません (polite negative)
    #[derive(Debug)]
    struct ArimasenMatcher;
    impl Matcher for ArimasenMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "ある"
                && token.pos.first().is_some_and(|p| p == "動詞")
        }
    }

    // Match ませ (part of ありません)
    #[derive(Debug)]
    struct MaseMatcher;
    impl Matcher for MaseMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "ます"
                && token.pos.first().is_some_and(|p| p == "助動詞")
        }
    }

    // Match ん (part of ありません)
    #[derive(Debug)]
    struct NMatcher;
    impl Matcher for NMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ん"
                && token.pos.first().is_some_and(|p| p == "助動詞")
        }
    }

    // We need to support two forms:
    // 1. というわけではない/じゃない (standard/casual)
    // 2. というわけではありません (polite)

    // Since TokenMatcher doesn't support OR logic easily, we'll use a combined matcher
    // that matches both patterns
    #[derive(Debug)]
    struct NegativeEndingMatcher;
    impl Matcher for NegativeEndingMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match ない (either adjective after では or auxiliary after じゃ)
            (token.surface == "ない"
                && (token.pos.first().is_some_and(|p| p == "助動詞")
                    || token.pos.first().is_some_and(|p| p == "形容詞")))
            // Or match あり (start of ありません)
            || (token.base_form == "ある"
                && token.pos.first().is_some_and(|p| p == "動詞"))
        }
    }

    vec![
        TokenMatcher::Any,  // Verb, Adjective, or Noun
        TokenMatcher::Custom(Arc::new(ToiuMatcher)),
        TokenMatcher::Custom(Arc::new(WakeMatcher)),
        TokenMatcher::Custom(Arc::new(DeJaMatcher)),
        // Optional は (present in では, absent in じゃ)
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(HaParticleMatcher)))),
        // ない or あり (start of ありません)
        TokenMatcher::Custom(Arc::new(NegativeEndingMatcher)),
        // Optional polite ending (ませ + ん) - only present if previous was あり
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(MaseMatcher)))),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NMatcher)))),
    ]
}

// Pattern: のももっともだ (it's only natural that, it's reasonable that)
// Structures:
//   - Verb/い-Adj + の + も/は + もっとも + だ/です
//   - な-Adj + な + の + も/は + もっとも + だ/です
//   - Noun + も/は + もっとも + だ/です
pub fn nomomottomoda() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match の as nominalizer (名詞/非自立)
    #[derive(Debug)]
    struct NoNominalizer;
    impl Matcher for NoNominalizer {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "非自立")
        }
    }

    // Match も or は particle (助詞/係助詞)
    #[derive(Debug)]
    struct MoHaParticle;
    impl Matcher for MoHaParticle {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "も" || token.surface == "は")
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "係助詞")
        }
    }

    // Match もっとも (副詞/一般 or 名詞/形容動詞語幹)
    #[derive(Debug)]
    struct Mottomo;
    impl Matcher for Mottomo {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "もっとも"
                && ((token.pos.first().is_some_and(|p| p == "副詞")
                    && token.pos.get(1).is_some_and(|p| p == "一般"))
                    || (token.pos.first().is_some_and(|p| p == "名詞")
                        && token.pos.get(1).is_some_and(|p| p == "形容動詞語幹")))
        }
    }

    // Match だ or です (助動詞)
    #[derive(Debug)]
    struct DaDes;
    impl Matcher for DaDes {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "だ" || token.surface == "です")
                && token.pos.first().is_some_and(|p| p == "助動詞")
        }
    }

    // Match な (助動詞, 特殊・ダ, 体言接続) - for な-adjectives
    #[derive(Debug)]
    struct NaAuxiliary;
    impl Matcher for NaAuxiliary {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "な"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|p| p == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "体言接続")
        }
    }

    // We need to match:
    // 1. Any token (verb/adjective/noun) - using Any
    // 2. Optional な (for な-adjectives)
    // 3. Optional の (for non-noun cases)
    // 4. も/は particle
    // 5. もっとも + だ/です

    vec![
        TokenMatcher::Any,  // The word before の/も/は
        // Optional な for な-adjectives
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NaAuxiliary)))),
        // Optional の nominalizer (not present for bare nouns)
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NoNominalizer)))),
        // も or は particle
        TokenMatcher::Custom(Arc::new(MoHaParticle)),
        // もっとも (adverb or na-adjective)
        TokenMatcher::Custom(Arc::new(Mottomo)),
        // だ or です
        TokenMatcher::Custom(Arc::new(DaDes)),
    ]
}

// Pattern: たって (even if, even though, no matter how)
// Structure: Verb[連用タ接続] + たって
pub fn tatte() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for たって as 助詞/接続助詞
    #[derive(Debug)]
    struct TatteParticleMatcher;
    impl super::Matcher for TatteParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "たって"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Matches verb in 連用タ接続 + たって (e.g., 謝ったって)
    vec![
        super::flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(TatteParticleMatcher)),
    ]
}

// Pattern: たって (negative forms with なく)
// Structures: Verb/Adj + なく + たって, Noun/な-Adj + じゃ + なく + たって
pub fn tatte_naku() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for たって as 助詞/接続助詞
    #[derive(Debug)]
    struct TatteParticleMatcher;
    impl super::Matcher for TatteParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "たって"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Matcher for なく (negative auxiliary) in 連用テ接続
    #[derive(Debug)]
    struct NakuMatcher;
    impl super::Matcher for NakuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "なく"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Matcher for じゃ (助詞/副助詞)
    #[derive(Debug)]
    struct JyaParticleMatcher;
    impl super::Matcher for JyaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "じゃ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞")
        }
    }

    // Matches:
    // - Adj/Verb + なく + たって (楽しくなくたって, 難しくなくたって)
    // - Noun/な-Adj + じゃ + なく + たって (有名じゃなくたって, 専門家じゃなくたって)
    vec![
        TokenMatcher::Any,
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(JyaParticleMatcher)))),
        TokenMatcher::Custom(Arc::new(NakuMatcher)),
        TokenMatcher::Custom(Arc::new(TatteParticleMatcher)),
    ]
}

// Pattern: たって (i-adjective く form - special tokenization)
// Structure: い-Adjective[く] + たって
// Note: This gets tokenized as Adj[く] + たっ(verb) + て(particle)
pub fn tatte_i_adj_ku() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for たっ as verb たつ
    #[derive(Debug)]
    struct TatsuVerbMatcher;
    impl super::Matcher for TatsuVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "たっ"
                && token.base_form == "たつ"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    // Matcher for て particle
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl super::Matcher for TeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // い-Adjective in 連用テ接続 + たっ + て
    vec![
        TokenMatcher::Adjective { base_form: None }, // Any i-adjective in 連用テ接続
        TokenMatcher::Custom(Arc::new(TatsuVerbMatcher)),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
    ]
}

// Pattern: に限って (particularly when, only when, those who)
// Structures: Noun + に + 限って
pub fn ni_kagitte() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KagitteVerbMatcher;
    impl super::Matcher for KagitteVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "限る"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用タ接続")
        }
    }

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
        TokenMatcher::Surface("に"),
        TokenMatcher::Custom(Arc::new(KagitteVerbMatcher)),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
    ]
}

// Pattern: に限らず (not only, not just)
// Structures: Noun + に + 限らず
pub fn ni_kagirazu() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KagirazuVerbMatcher;
    impl super::Matcher for KagirazuVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "限る"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然形")
        }
    }

    #[derive(Debug)]
    struct ZuAuxiliaryMatcher;
    impl super::Matcher for ZuAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ず"
                && token.base_form == "ぬ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Surface("に"),
        TokenMatcher::Custom(Arc::new(KagirazuVerbMatcher)),
        TokenMatcher::Custom(Arc::new(ZuAuxiliaryMatcher)),
    ]
}

// Pattern: ねばならない (must, have to)
// Structures: Verb[未然形] + ねばならない / ねばなりません
//             する → せねばならない / せねばなりません
pub fn nebanaranaי() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct VerbMizenMatcher;
    impl super::Matcher for VerbMizenMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match verb in 未然形 (including せ from する with 未然ヌ接続)
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| {
                    f == "未然形" || f == "未然ヌ接続"
                })
        }
    }

    #[derive(Debug)]
    struct NeAuxiliaryMatcher;
    impl super::Matcher for NeAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match ね (助動詞, ぬ, 仮定形)
            token.surface == "ね"
                && token.base_form == "ぬ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "仮定形")
        }
    }

    #[derive(Debug)]
    struct BaParticleMatcher;
    impl super::Matcher for BaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ば"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    #[derive(Debug)]
    struct NariNaruMatcher;
    impl super::Matcher for NariNaruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match なら (未然形) or なり (連用形)
            token.base_form == "なる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                && token.features.get(5).is_some_and(|f| {
                    f == "未然形" || f == "連用形"
                })
        }
    }

    #[derive(Debug)]
    struct NaiOrMasenMatcher;
    impl super::Matcher for NaiOrMasenMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match ない (助動詞) OR ませ (助動詞, ます, 未然形)
            if token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
            {
                return true;
            }

            if token.surface == "ませ"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "未然形")
            {
                return true;
            }

            false
        }
    }

    #[derive(Debug)]
    struct NParticleMatcher;
    impl super::Matcher for NParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match ん (助動詞, 不変化型) - negative marker
            token.surface == "ん"
                && token.base_form == "ん"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbMizenMatcher)),
        TokenMatcher::Custom(Arc::new(NeAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(BaParticleMatcher)),
        TokenMatcher::Custom(Arc::new(NariNaruMatcher)),
        TokenMatcher::Custom(Arc::new(NaiOrMasenMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NParticleMatcher)))),
    ]
}
