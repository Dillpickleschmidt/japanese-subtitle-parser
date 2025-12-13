use crate::pattern_matcher::TokenMatcher;
use crate::matchers::{Matcher, noun_matcher};
use std::sync::Arc;

// Pattern: と
pub fn to() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: でも (even, or something, any-)
// Structures: Noun + でも (or Noun + で + も when split)
//
// Two tokenization patterns:
// 1. Noun + で (助詞/格助詞) + も (助詞/係助詞) - お茶でも, だれでも
// 2. Noun + でも (助詞/副助詞) - なんでも, どこでも, いつでも
pub fn demo() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct DemoMatcher;
    impl Matcher for DemoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match either "で" (格助詞) or "でも" (副助詞)
            (token.surface == "で"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞"))
                || (token.surface == "でも"
                    && token.pos.first().is_some_and(|p| p == "助詞")
                    && token.pos.get(1).is_some_and(|p| p == "副助詞"))
        }
    }

    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "も"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "係助詞")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(DemoMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(MoParticleMatcher)))),
    ]
}

// Pattern: やすい (easy to / prone to)
// Structure: Verb[stem/連用形] + やすい
//
// Tokenization: Verb (連用形) + やすい (形容詞/非自立)
// Meaning: "easy to (A)" or "prone to (A)" (with emotion verbs)
pub fn yasui() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct YasuiMatcher;
    impl Matcher for YasuiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "やすい"
                && token.pos.first().is_some_and(|p| p == "形容詞")
                && token.pos.get(1).is_some_and(|p| p == "非自立")
        }
    }

    vec![
        super::flexible_verb_form(),  // Verb in 連用形 or 連用タ接続
        TokenMatcher::Custom(Arc::new(YasuiMatcher)),
    ]
}

// Pattern: にくい (difficult to)
// Structures: Verb[stem] + にくい/にくいです
pub fn nikui() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct NikuiMatcher;
    impl Matcher for NikuiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "にくい"
                && token.pos.first().is_some_and(|p| p == "形容詞")
                && token.pos.get(1).is_some_and(|p| p == "非自立")
        }
    }

    vec![
        super::flexible_verb_form(),  // Verb in 連用形 or 連用タ接続
        TokenMatcher::Custom(Arc::new(NikuiMatcher)),
    ]
}

// Pattern: だんだん (gradually/steadily)
// Structures: だんだん + (と) + Phrase
pub fn dandan() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct DandanMatcher;
    impl Matcher for DandanMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "だんだん"
                && token.pos.first().is_some_and(|p| p == "副詞")
                && token.pos.get(1).is_some_and(|p| p == "助詞類接続")
        }
    }

    #[derive(Debug)]
    struct ToParticleMatcher;
    impl Matcher for ToParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "と"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "副詞化")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(DandanMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(ToParticleMatcher)))),
    ]
}

// Pattern: どんどん (rapidly/quickly)
// Structures: どんどん + (と) + Phrase
pub fn dondon() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct DondonMatcher;
    impl Matcher for DondonMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "どんどん"
                && token.pos.first().is_some_and(|p| p == "副詞")
                && token.pos.get(1).is_some_and(|p| p == "助詞類接続")
        }
    }

    #[derive(Debug)]
    struct ToParticleMatcher;
    impl Matcher for ToParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "と"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "副詞化")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(DondonMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(ToParticleMatcher)))),
    ]
}

// Pattern: ～ら
pub fn uff5e_ra() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ていく (to go on to)
// Structures: Verb[て] + いく
pub fn teiku() -> Vec<TokenMatcher> {
    use super::concat;

    // Match て or で particle
    #[derive(Debug)]
    struct TeDeFormMatcher;
    impl Matcher for TeDeFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|p| p == "助詞")
        }
    }

    // Match いく as auxiliary verb
    #[derive(Debug)]
    struct IkuMatcher;
    impl Matcher for IkuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "いく"
                && token.pos.first().is_some_and(|p| p == "動詞")
        }
    }

    concat(vec![
        vec![super::flexible_verb_form()],
        vec![TokenMatcher::Custom(Arc::new(TeDeFormMatcher))],
        vec![TokenMatcher::Custom(Arc::new(IkuMatcher))],
    ])
}

// Pattern: てくる (to come to)
// Structures: Verb[て] + くる
pub fn tekuru() -> Vec<TokenMatcher> {
    use super::concat;

    // Match て or で particle
    #[derive(Debug)]
    struct TeDeFormMatcher;
    impl Matcher for TeDeFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|p| p == "助詞")
        }
    }

    // Match くる as auxiliary verb
    #[derive(Debug)]
    struct KuruMatcher;
    impl Matcher for KuruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "くる"
                && token.pos.first().is_some_and(|p| p == "動詞")
        }
    }

    concat(vec![
        vec![super::flexible_verb_form()],
        vec![TokenMatcher::Custom(Arc::new(TeDeFormMatcher))],
        vec![TokenMatcher::Custom(Arc::new(KuruMatcher))],
    ])
}

// Pattern: かた (how to/way of)
// Structures: Verb[stem] + 方（かた） / Noun(サ変) + の + 仕方（しかた）
pub fn kata() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for かた/方 as suffix (名詞/接尾)
    #[derive(Debug)]
    struct KataSuffixMatcher;
    impl super::Matcher for KataSuffixMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "かた" || token.surface == "方")
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
        }
    }

    // Matcher for しかた/仕方 as noun
    #[derive(Debug)]
    struct ShikataMatcher;
    impl super::Matcher for ShikataMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "しかた" || token.surface == "仕方")
                && token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    // Matcher for suru-verb nouns (名詞/サ変接続)
    #[derive(Debug)]
    struct SuruVerbNounMatcher;
    impl super::Matcher for SuruVerbNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "サ変接続")
        }
    }

    // Pattern 1: Verb (連用形) + かた/方
    // Pattern 2: Suru-verb noun + の + しかた/仕方
    // We'll use a custom matcher to handle both patterns

    #[derive(Debug)]
    struct KataPatternMatcher;
    impl super::Matcher for KataPatternMatcher {
        fn matches(&self, _token: &crate::KagomeToken) -> bool {
            true // This is a placeholder - we'll use proper matchers below
        }
    }

    // Pattern: Verb (連用形) + かた
    vec![
        TokenMatcher::verb_with_form("連用形"),
        TokenMatcher::Custom(Arc::new(KataSuffixMatcher)),
    ]
}

// Pattern: かた (shikata variant - suru-verb + の + しかた)
// Structures: Noun(サ変) + の + 仕方（しかた）
pub fn kata_shikata() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for suru-verb nouns (名詞/サ変接続)
    #[derive(Debug)]
    struct SuruVerbNounMatcher;
    impl super::Matcher for SuruVerbNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "サ変接続")
        }
    }

    // Matcher for しかた/仕方 as noun
    #[derive(Debug)]
    struct ShikataMatcher;
    impl super::Matcher for ShikataMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "しかた" || token.surface == "仕方")
                && token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    // Pattern: Suru-verb noun + の + しかた
    vec![
        TokenMatcher::Custom(Arc::new(SuruVerbNounMatcher)),
        TokenMatcher::Surface("の"),
        TokenMatcher::Custom(Arc::new(ShikataMatcher)),
    ]
}

// Pattern: だけで (just by/with only)
// Structures: Verb + だけで / Noun + だけで
pub fn dakede() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match だけ particle (副助詞)
    #[derive(Debug)]
    struct DakeParticleMatcher;
    impl super::Matcher for DakeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "だけ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞")
        }
    }

    // Match で particle (格助詞)
    #[derive(Debug)]
    struct DeParticleMatcher;
    impl super::Matcher for DeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        TokenMatcher::Any, // Verb or Noun
        TokenMatcher::Custom(Arc::new(DakeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(DeParticleMatcher)),
    ]
}

// Pattern: だが・ですが
pub fn daga_u30fb_desuga() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なくて
pub fn nakute() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ないで
// Pattern: ないで (without doing)
// Structure: Verb[未然形] + ない + で
pub fn naide() -> Vec<TokenMatcher> {
    // Match verb in 未然形 (negative stem form)
    #[derive(Debug)]
    struct VerbMizenMatcher;
    impl super::Matcher for VerbMizenMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然形")
        }
    }

    // Match ない auxiliary in 連用デ接続 form
    #[derive(Debug)]
    struct NaiAuxiliaryMatcher;
    impl super::Matcher for NaiAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "連用デ接続")
        }
    }

    // Match で as conjunction particle
    #[derive(Debug)]
    struct DeParticleMatcher;
    impl super::Matcher for DeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbMizenMatcher)),
        TokenMatcher::Custom(Arc::new(NaiAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(DeParticleMatcher)),
    ]
}

// Pattern: Verb［れる・られる］
pub fn verb_uff3b_reru_u30fb_rareru_uff3d() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 他動詞・自動詞
pub fn tadoushi_u30fb_jidoushi() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なおす (to redo/fix) - split tokenization
// Structures: Verb[stem] + なおす/なおします
pub fn naosu() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct NaosuMatcher;
    impl Matcher for NaosuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "なおす"
                && token.pos.first().is_some_and(|p| p == "動詞")
                && token.pos.get(1).is_some_and(|p| p == "非自立")
        }
    }

    vec![
        super::flexible_verb_form(),  // Verb in 連用形 or 連用タ接続
        TokenMatcher::Custom(Arc::new(NaosuMatcher)),
    ]
}

// Pattern: ということ (that means / you mean)
// Structures: Phrase + ということ, Phrase + ってこと
pub fn toiukoto() -> Vec<TokenMatcher> {
    use super::Matcher;

    #[derive(Debug)]
    struct ToiuTteMatcher;
    impl Matcher for ToiuTteMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "という" || token.surface == "って")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "連語")
        }
    }

    #[derive(Debug)]
    struct KotoMatcher;
    impl Matcher for KotoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    vec![
        TokenMatcher::Any,
        TokenMatcher::Custom(Arc::new(ToiuTteMatcher)),
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
    ]
}

// Pattern: とき
pub fn toki() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: まず
pub fn mazu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: まで
pub fn made() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: までに (by/until - deadline)
// Structures: Verb/Noun + まで + に
//
// Tokenization: Content word + まで (助詞/副助詞) + に (助詞/格助詞)
// Meaning: "by" (deadline), NOT "until" (continuous action)
pub fn madeni() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct MadeParticleMatcher;
    impl Matcher for MadeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "まで"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "副助詞")
        }
    }

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
        TokenMatcher::Any,  // Verb or Noun
        TokenMatcher::Custom(Arc::new(MadeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
    ]
}

// Pattern: また
pub fn mata() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: はじめる (start doing)
// Structure: Verb[stem/連用形] + はじめる
//
// Example tokenizations:
// - ためはじめます: ため(動詞/連用形) + はじめ(動詞/非自立/連用形) + ます
// - 歌いはじめる: 歌い(動詞/連用形) + はじめる(動詞/非自立/基本形)
// - ならいはじめた: ならい(動詞/連用形) + はじめ(動詞/非自立/連用形) + た
pub fn hajimeru() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct HajimeruMatcher;
    impl Matcher for HajimeruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "はじめる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    vec![
        super::flexible_verb_form(), // Verb in 連用形 or 連用タ接続
        TokenMatcher::Custom(Arc::new(HajimeruMatcher)), // はじめる (auxiliary verb)
    ]
}

// Pattern: おわる (finish doing)
// Structure: Verb[stem/連用形] + 終わる
//
// Example tokenizations:
// - 払いおわる: 払い(動詞/連用形) + おわる(動詞/基本形)
// - 飲みおわって: 飲み(動詞/連用形) + おわっ(動詞/連用タ接続) + て
// - 読みおわりました: 読み(動詞/連用形) + おわり(動詞/連用形) + ました
pub fn owaru() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct OwaruMatcher;
    impl Matcher for OwaruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "おわる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    vec![
        super::flexible_verb_form(), // Verb in 連用形 or 連用タ接続
        TokenMatcher::Custom(Arc::new(OwaruMatcher)), // おわる (auxiliary verb)
    ]
}

// Pattern: ごろ
pub fn goro() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: こと (nominalization)
// Structure: Verb + こと
//
// こと is a bound noun (名詞/非自立) used for nominalization.
// Converts verbs into noun phrases (e.g., "doing X", "the act of X")
//
// Examples:
// - することが嫌い (dislike faxing / the act of faxing)
// - 過ぎないことが大事 (not using too much is important)
// - なることをして (do things that cause...)
//
// Tokenization:
// - Verb (any form: basic, negative, past, etc.) OR auxiliary verb
// - こと (名詞/非自立/一般)
//
// Note: Matches verbs and auxiliary verbs only (not nouns like 勉強)
// For compound verbs like 勉強する, matches just "する + こと", not the whole phrase
pub fn koto() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct VerbOrAuxiliaryMatcher;
    impl Matcher for VerbOrAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match verbs and auxiliary verbs only
            // Exclude nouns (even if they're サ変接続)
            let is_verb = token.pos.first().is_some_and(|pos| pos == "動詞");
            let is_auxiliary = token.pos.first().is_some_and(|pos| pos == "助動詞");
            let is_noun = token.pos.first().is_some_and(|pos| pos == "名詞");

            (is_verb || is_auxiliary) && !is_noun
        }
    }

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

    vec![
        TokenMatcher::Custom(Arc::new(VerbOrAuxiliaryMatcher)), // Verb or auxiliary verb
        TokenMatcher::Custom(Arc::new(KotoMatcher)), // こと (bound noun)
    ]
}

// Pattern: あとで (after/later)
// Structures:
// - Verb[た] + あとで
// - Noun + の + あとで
// - あとで + Phrase (at start)
// - Verb + のは + あとで
//
// Examples:
// - 食べたあとで (after eating)
// - 仕事のあとで (after work)
// - あとで洗濯もの干してね (please hang the laundry later)
// - コピーを取るのはあとでいい (it's fine to make copies later)
//
// Tokenization pattern:
// - あと (名詞/一般)
// - で (助詞/格助詞/一般 OR 助動詞, base=だ)
//
// Note: We match "あと + で" regardless of what comes before,
// since all variants end with this combination.
pub fn atode() -> Vec<TokenMatcher> {
    use super::Matcher;

    // Match あと noun (名詞/一般)
    #[derive(Debug)]
    struct AtoMatcher;
    impl Matcher for AtoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "あと"
                && token.base_form == "あと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
        }
    }

    // Match で particle or copula after あと
    // Can be:
    // - 助詞/格助詞/一般 (particle: "with/at")
    // - 助動詞 (copula, base=だ: "is")
    #[derive(Debug)]
    struct DeAfterAtoMatcher;
    impl Matcher for DeAfterAtoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            if token.surface != "で" {
                return false;
            }

            // Check if it's a particle (助詞/格助詞)
            let is_particle = token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞");

            // Check if it's a copula (助動詞, base=だ)
            let is_copula = token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "だ";

            is_particle || is_copula
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(AtoMatcher)),
        TokenMatcher::Custom(Arc::new(DeAfterAtoMatcher)),
    ]
}

// Pattern: ていた (was doing / past progressive)
// Structures: Verb[ている] + た / Verb[ている] + ました
pub fn teita() -> Vec<TokenMatcher> {
    use super::concat;

    // Match て or で particle
    #[derive(Debug)]
    struct TeDeFormMatcher;
    impl Matcher for TeDeFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|p| p == "助詞")
        }
    }

    // Match いる in 連用形 (い)
    #[derive(Debug)]
    struct IruRenyoukeiMatcher;
    impl Matcher for IruRenyoukeiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "いる"
                && token.pos.first().is_some_and(|p| p == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形")
        }
    }

    concat(vec![
        vec![super::flexible_verb_form()],
        vec![TokenMatcher::Custom(Arc::new(TeDeFormMatcher))],
        vec![TokenMatcher::Custom(Arc::new(IruRenyoukeiMatcher))],
        vec![TokenMatcher::Optional(Box::new(super::mashi_form()))], // Optional まし for polite form
        vec![super::past_auxiliary()], // た
    ])
}

// Pattern: に (Frequency)
pub fn ni_frequency() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: とうとう (finally/at last)
// Structures: とうとう + Phrase
pub fn toutou() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct ToutouMatcher;
    impl Matcher for ToutouMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "とうとう"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(ToutouMatcher))]
}

// Pattern: より (than/more than)
// Structures: Noun + より, Verb + より
pub fn yori() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct YoriMatcher;
    impl Matcher for YoriMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "より"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(YoriMatcher))]
}

// Pattern: ごとに (every/each time)
// Structures: Verb/Noun + ごと + に
pub fn gotoni() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct GotoMatcher;
    impl super::Matcher for GotoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ごと"
                && token.base_form == "ごと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.base_form == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        TokenMatcher::Any, // Verb or Noun
        TokenMatcher::Custom(Arc::new(GotoMatcher)), // ごと (名詞)
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)), // に (助詞/格助詞)
    ]
}

// Pattern: なるべく
pub fn narubeku() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: るところだ
pub fn rutokoroda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: のに 
pub fn noni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: とおもう
pub fn toomou() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: など
pub fn nado() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: みたい
pub fn mitai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: そう
pub fn sou() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: さ (degree/amount suffix)
// Structures: い-Adjective[い] + さ / な-Adjective + さ
pub fn sa() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match い-adjective in ガル接続 form (stem without い)
    #[derive(Debug)]
    struct IAdjectiveStemMatcher;
    impl super::Matcher for IAdjectiveStemMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "ガル接続")
        }
    }

    // Match な-adjective (名詞/形容動詞語幹)
    #[derive(Debug)]
    struct NaAdjectiveStemMatcher;
    impl super::Matcher for NaAdjectiveStemMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹")
        }
    }

    // Match さ suffix (名詞/接尾/特殊)
    #[derive(Debug)]
    struct SaSuffixMatcher;
    impl super::Matcher for SaSuffixMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "さ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                && token.pos.get(2).is_some_and(|pos| pos == "特殊")
        }
    }

    // Match either い-adjective stem or な-adjective stem
    #[derive(Debug)]
    struct AdjectiveStemMatcher;
    impl super::Matcher for AdjectiveStemMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // い-adjective stem (ガル接続)
            (token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "ガル接続"))
            ||
            // な-adjective (形容動詞語幹)
            (token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹"))
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(AdjectiveStemMatcher)),
        TokenMatcher::Custom(Arc::new(SaSuffixMatcher)),
    ]
}

// Pattern: とか～とか
pub fn toka_uff5e_toka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: そういう
pub fn souiu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Verb[よう]
pub fn verb_you() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ようだ
pub fn youda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ぜんぜん
pub fn zenzen() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: かな (I wonder)
// Structure: Sentence + か + な
pub fn kana() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KaParticleMatcher;
    impl super::Matcher for KaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "か"
                && token.base_form == "か"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token
                    .pos
                    .get(1)
                    .is_some_and(|pos| pos == "副助詞／並立助詞／終助詞")
        }
    }

    #[derive(Debug)]
    struct NaEndingParticleMatcher;
    impl super::Matcher for NaEndingParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "な"
                && token.base_form == "な"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "終助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(KaParticleMatcher)),
        TokenMatcher::Custom(Arc::new(NaEndingParticleMatcher)),
    ]
}

// Pattern: あまり～ない (not very)
// Structures:
// - あまり + Verb[ない]
// - あまり + い-Adjective[ない]
// - あまり + Noun + ではない/じゃない
// - あまり + な-Adjective + ではない/じゃない
// - Casual variant: あんまり (instead of あまり)
//
// Examples:
// - あまり並ばない (not stand in line very long)
// - あまり寂しくない (not feel very lonely)
// - あまり平和ではない (not very peaceful)
// - あんまり食べたくない (don't want to eat very much)
//
// Tokenization pattern:
// - あまり OR あんまり (副詞/助詞類接続 OR 名詞/一般)
// - Wildcard (0-5 tokens)
// - ない (助動詞)
//
// Note: あまり can tokenize as:
// - 副詞/助詞類接続 (adverb, before verbs/adjectives)
// - 名詞/一般 (noun, before adjectives/nouns)
pub fn amari_uff5e_nai() -> Vec<TokenMatcher> {
    use super::Matcher;

    // Match あまり or あんまり (flexible POS matching)
    #[derive(Debug)]
    struct AmariMatcher;
    impl Matcher for AmariMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match surface: あまり OR あんまり
            if token.surface != "あまり" && token.surface != "あんまり" {
                return false;
            }
            // Match base form
            if token.base_form != "あまり" && token.base_form != "あんまり" {
                return false;
            }

            // Can be either:
            // - 副詞/助詞類接続 (adverb)
            // - 名詞/一般 (noun)
            let is_adverb = token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "助詞類接続");

            let is_noun = token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般");

            is_adverb || is_noun
        }
    }

    // Match ない (auxiliary verb OR adjective)
    // Can be:
    // - 助動詞 (after verbs: 並ばない)
    // - 形容詞/自立 (after で: ではない)
    #[derive(Debug)]
    struct NaiMatcher;
    impl Matcher for NaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            if token.surface != "ない" || token.base_form != "ない" {
                return false;
            }

            // Check if it's auxiliary verb (助動詞)
            let is_auxiliary = token.pos.first().is_some_and(|pos| pos == "助動詞");

            // Check if it's adjective (形容詞/自立)
            let is_adjective = token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立");

            is_auxiliary || is_adjective
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(AmariMatcher)),
        TokenMatcher::Wildcard {
            min: 0,
            max: 5,
            stop_conditions: vec![],
        },
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
    ]
}

// Pattern: ば
pub fn ba() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なら
pub fn nara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: がる (to show signs of / to act like)
// Structures: Adjective + がる/がります
//
// Tokenization patterns:
// 1. Dictionary form (compound): 強がる → 強がる (動詞, base_form=強がる) - single token
// 2. Split conjugated form: 欲しがります → 欲し (形容詞, ガル接続) + がり (動詞/接尾) + ます
//
// This pattern has two separate matchers to handle both tokenization cases.
pub fn garu() -> Vec<TokenMatcher> {
    vec![]  // Placeholder - pattern handled by garu_compound and garu_split
}

// Match dictionary form がる verbs (single token compounds like 強がる)
pub fn garu_compound() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct GaruVerbMatcher;
    impl Matcher for GaruVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.base_form.ends_with("がる")
                && token.base_form != "がる"
        }
    }

    vec![TokenMatcher::Custom(Arc::new(GaruVerbMatcher))]
}

// Match split conjugated がる forms (Adjective + がる suffix)
pub fn garu_split() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match adjective in ガル接続 form (stem for がる attachment)
    #[derive(Debug)]
    struct GaruConnectingAdjMatcher;
    impl Matcher for GaruConnectingAdjMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "ガル接続")
        }
    }

    // Match がる as verb suffix (動詞/接尾, base_form=がる)
    #[derive(Debug)]
    struct GaruSuffixMatcher;
    impl Matcher for GaruSuffixMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                && token.base_form == "がる"
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(GaruConnectingAdjMatcher)),
        TokenMatcher::Custom(Arc::new(GaruSuffixMatcher)),
    ]
}

// Pattern: がする (sensory experience)
// Structure: Noun + が + する/します
//
// Example tokenizations:
// - 匂いがする: 匂い(名詞) + が(助詞/格助詞) + する(動詞/サ変・スル)
// - 音がします: 音(名詞) + が(助詞/格助詞) + し(動詞/連用形) + ます(助動詞)
//
// Common sensory nouns: 匂い (smell), 音 (sound), 味 (taste), 感じ (feeling), 気 (sense/feeling)
pub fn gasuru() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct GaParticleMatcher;
    impl Matcher for GaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "が"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    #[derive(Debug)]
    struct SuruVerbMatcher;
    impl Matcher for SuruVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "する"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    vec![
        super::noun_matcher(), // Sensory noun (匂い, 音, 味, 感じ, 気, etc.)
        TokenMatcher::Custom(Arc::new(GaParticleMatcher)), // が (格助詞)
        TokenMatcher::Custom(Arc::new(SuruVerbMatcher)), // する (verb)
    ]
}

// Pattern: たがる
// Pattern: たがる - wanting to do (third person observable desire)
// Structures: Verb[stem] + た (from たい) + がる
pub fn tagaru() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct TaiGaruMatcher;
    impl Matcher for TaiGaruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "たい"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "ガル接続")
        }
    }

    #[derive(Debug)]
    struct GaruMatcher;
    impl Matcher for GaruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "がる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
        }
    }

    vec![
        super::flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(TaiGaruMatcher)),
        TokenMatcher::Custom(Arc::new(GaruMatcher)),
    ]
}

// Pattern: かもしれない (might/maybe)
// Structures: Verb/Adjective/Noun + かもしれない/かもしれません
pub fn kamoshirenai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match かも (副助詞)
    #[derive(Debug)]
    struct KamoMatcher;
    impl super::Matcher for KamoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "かも"
                && token.base_form == "かも"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞")
        }
    }

    // Match しれ (verb form of しれる)
    #[derive(Debug)]
    struct ShireMatcher;
    impl super::Matcher for ShireMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "しれ"
                && token.base_form == "しれる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    // Match ない (auxiliary) or ません pattern
    #[derive(Debug)]
    struct NaiOrMasenMatcher;
    impl super::Matcher for NaiOrMasenMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Either ない (助動詞) or ませ (for ません)
            if token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
            {
                return true;
            }
            // For ません pattern: ませ + ん
            if token.surface == "ませ"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
            {
                return true;
            }
            false
        }
    }

    // Match ん (auxiliary for ません)
    #[derive(Debug)]
    struct NMasenMatcher;
    impl super::Matcher for NMasenMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ん"
                && token.base_form == "ん"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        TokenMatcher::Any, // Verb, Adjective, or Noun
        TokenMatcher::Custom(Arc::new(KamoMatcher)),
        TokenMatcher::Custom(Arc::new(ShireMatcher)),
        TokenMatcher::Custom(Arc::new(NaiOrMasenMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NMasenMatcher)))),
    ]
}

// Pattern: みたいに・みたいな
pub fn mitaini_u30fb_mitaina() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: そうに・そうな 
pub fn souni_u30fb_souna() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: のように・のような 
pub fn noyouni_u30fb_noyouna() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 〜ようと思う・〜おうと思う
pub fn u301c_youtoomou_u30fb_u301c_outoomou() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: く・に
pub fn ku_u30fb_ni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～にする・～くする
pub fn uff5e_nisuru_u30fb_uff5e_kusuru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: といい
pub fn toii() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ようになる
pub fn youninaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: まい～のように
pub fn mai_uff5e_noyouni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: じゃないか (isn't it?)
// Structures: Phrase + じゃない + か OR Phrase + ではない + か
pub fn janaika() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ん (explanatory の) - optional
    #[derive(Debug)]
    struct NMatcher;
    impl Matcher for NMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ん"
                && token.base_form == "ん"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Match じゃ (casual) or で (formal)
    #[derive(Debug)]
    struct JyaOrDeMatcher;
    impl Matcher for JyaOrDeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "じゃ" && token.base_form == "じゃ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞"))
            || (token.surface == "で" && token.base_form == "で"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞"))
        }
    }

    // Match は (only for ではないか formal form)
    #[derive(Debug)]
    struct WaParticleMatcher;
    impl Matcher for WaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.base_form == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Match ない (auxiliary verb)
    #[derive(Debug)]
    struct NaiAuxMatcher;
    impl Matcher for NaiAuxMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Match か (sentence-ending particle)
    #[derive(Debug)]
    struct KaEndingMatcher;
    impl Matcher for KaEndingMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "か"
                && token.base_form == "か"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos.contains("終助詞"))
        }
    }

    vec![
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NMatcher)))),
        TokenMatcher::Custom(Arc::new(JyaOrDeMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(WaParticleMatcher)))),
        TokenMatcher::Custom(Arc::new(NaiAuxMatcher)),
        TokenMatcher::Custom(Arc::new(KaEndingMatcher)),
    ]
}

// Pattern: らしい ①
pub fn rashii_u2460() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ておく (do in advance, leave as is)
// Structures: Verb[て] + おく/とく (casual), polite forms with ます
pub fn teoku() -> Vec<TokenMatcher> {
    use super::Matcher;
    use std::sync::Arc;

    // Match て or で particle
    #[derive(Debug)]
    struct TeDeFormMatcher;
    impl Matcher for TeDeFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Match おく or とく as auxiliary verb
    #[derive(Debug)]
    struct OkuMatcher;
    impl Matcher for OkuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.base_form == "おく" || token.base_form == "とく")
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Match ます (optional for polite form)
    #[derive(Debug)]
    struct MasuFormMatcher;
    impl Matcher for MasuFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ます" && token.base_form == "ます"
        }
    }

    vec![
        super::flexible_verb_form(),
        // て or で particle is optional because of the とく contraction
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(TeDeFormMatcher)))),
        TokenMatcher::Custom(Arc::new(OkuMatcher)),
        // Optional ます for polite form
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(MasuFormMatcher)))),
    ]
}

// Pattern: がほしい (want something)
// Structures: Noun + が + ほしい (+ です)
pub fn gahoshii() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct GaParticleMatcher;
    impl Matcher for GaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "が"
                && token.base_form == "が"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    #[derive(Debug)]
    struct HoshiiMatcher;
    impl Matcher for HoshiiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ほしい"
                && token.base_form == "ほしい"
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立")
        }
    }

    vec![
        super::noun_matcher(),                               // Noun (犬, 車, 時間, etc.)
        TokenMatcher::Custom(Arc::new(GaParticleMatcher)),   // が (格助詞)
        TokenMatcher::Custom(Arc::new(HoshiiMatcher)),       // ほしい (形容詞/自立)
    ]
}

// Pattern: てほしい (want someone to do)
// Structures: Verb[て] + ほしい
pub fn tehoshii() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct TeDeFormMatcher;
    impl Matcher for TeDeFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    #[derive(Debug)]
    struct HoshiiMatcher;
    impl Matcher for HoshiiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "ほしい"
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    vec![
        super::flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(TeDeFormMatcher)),
        TokenMatcher::Custom(Arc::new(HoshiiMatcher)),
    ]
}

// Pattern: ときいた
pub fn tokiita() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 聞こえる
pub fn kikoeru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 見える
pub fn mieru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: だす
// Pattern: だす - suddenly start doing (unintentional/uncontrolled)
// Structures: Verb[stem] + だす/だし
pub fn dasu() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct DasuMatcher;
    impl Matcher for DasuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "だす"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }
    vec![super::flexible_verb_form(), TokenMatcher::Custom(Arc::new(DasuMatcher))]
}

// Pattern: ～代 (decade/era suffix)
// Structures: Decade of age + 代 / Decade + 年代
pub fn uff5e_dai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match number (名詞/数)
    #[derive(Debug)]
    struct NumberMatcher;
    impl super::Matcher for NumberMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "数")
        }
    }

    // Match 代 or 年代 suffix (名詞/接尾/助数詞)
    #[derive(Debug)]
    struct DaiSuffixMatcher;
    impl super::Matcher for DaiSuffixMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "代" || token.surface == "年代")
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                && token.pos.get(2).is_some_and(|pos| pos == "助数詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NumberMatcher)),
        TokenMatcher::Wildcard {
            min: 0,
            max: 3,
            stop_conditions: vec![],
        }, // Allow 0-3 more number tokens (for multi-digit numbers)
        TokenMatcher::Custom(Arc::new(DaiSuffixMatcher)),
    ]
}

// Pattern: Number + も (as many as / not even)
// Structures: Number + Counter + も
//
// Examples:
// - １２時間も (as many as 12 hours)
// - ２０万円も (as much as 200,000 yen)
// - 一回も (not even once)
//
// Tokenization pattern:
// - One or more number tokens (名詞/数)
// - Counter token (名詞/接尾/助数詞)
// - も particle (助詞/係助詞)
pub fn number_mo() -> Vec<TokenMatcher> {
    use super::Matcher;

    // Match number tokens (名詞/数)
    #[derive(Debug)]
    struct NumberMatcher;
    impl Matcher for NumberMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "数")
        }
    }

    // Match counter tokens (名詞/接尾/助数詞)
    #[derive(Debug)]
    struct CounterMatcher;
    impl Matcher for CounterMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                && token.pos.get(2).is_some_and(|pos| pos == "助数詞")
        }
    }

    // Match も particle after numbers (助詞/係助詞)
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Pattern: One or more numbers + counter + も
    // Numbers can be 1-many tokens (一 vs １２ vs ２０万円)
    // We'll use a flexible approach with optional number tokens
    use super::concat;

    concat(vec![
        vec![TokenMatcher::Custom(Arc::new(NumberMatcher))], // First number (required)
        vec![
            TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NumberMatcher)))),
            TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NumberMatcher)))),
            TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NumberMatcher)))),
            TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NumberMatcher)))),
            TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NumberMatcher)))),
        ], // Up to 5 additional numbers (should be enough for most cases)
        vec![TokenMatcher::Custom(Arc::new(CounterMatcher))],
        vec![TokenMatcher::Custom(Arc::new(MoParticleMatcher))],
    ])
}

// Pattern: ほとんど
pub fn hotondo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: そんな・こんな・あんな・どんな
pub fn sonna_u30fb_konna_u30fb_anna_u30fb_donna() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 各
pub fn kaku() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 以上 ①
pub fn ijou_u2460() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: いか (equal to or less than / the following)
// Structures: Noun/Amount + 以下, standalone 以下
pub fn ika() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match いか (noun or adverb)
    #[derive(Debug)]
    struct IkaMatcher;
    impl super::Matcher for IkaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "いか"
                && token.base_form == "いか"
                && (token.pos.first().is_some_and(|pos| pos == "名詞")
                    || token.pos.first().is_some_and(|pos| pos == "副詞"))
        }
    }

    vec![TokenMatcher::Custom(Arc::new(IkaMatcher))]
}

// いがい: Except/besides (except A, other than A)
// Structures: Verb + 以外, Noun + 以外
pub fn igai() -> Vec<TokenMatcher> {
    use super::Matcher;

    #[derive(Debug)]
    struct IgaiMatcher;
    impl Matcher for IgaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "以外"
                && token.base_form == "以外"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    vec![
        TokenMatcher::Any, // Verb or Noun
        TokenMatcher::Custom(Arc::new(IgaiMatcher)), // 以外 (名詞/非自立/副詞可能)
    ]
}

// Pattern: ずっと ① (continuously/the whole time)
// Structure: ずっと + Phrase
//
// Examples:
// - ずっとゲームをしないで (instead of continuously gaming)
// - ずっと立ってた (standing the whole time)
// - からずっと寝てない (haven't slept at all since...)
//
// Tokenization: ずっと (副詞/一般)
pub fn zutto_u2460() -> Vec<TokenMatcher> {
    use super::Matcher;

    // Match ずっと adverb (副詞/一般)
    #[derive(Debug)]
    struct ZuttoMatcher;
    impl Matcher for ZuttoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ずっと"
                && token.base_form == "ずっと"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(ZuttoMatcher))]
}

// Pattern: だいたい
pub fn daitai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: のなかで
pub fn nonakade() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ように・ような
pub fn youni_u30fb_youna() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Number/Amount + は
pub fn number_amount_ha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なん + counter + か
pub fn nan_counter_ka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 真(っ)
pub fn ma() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Number + しか〜ない
// Pattern: Number + しか〜ない - "only (number)" with negative verb
// Structures: Number + しか + Verb[ない]
// Meaning: "only (number)" - しか must be used with negative verbs
// Example: 五キロしか走れない。 (I can only run 5 km.)
pub fn number_shika_u301c_nai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for numbers (名詞/数)
    #[derive(Debug)]
    struct NumberMatcher;
    impl super::Matcher for NumberMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "数")
        }
    }

    // Matcher for counters (名詞/接尾/助数詞)
    #[derive(Debug)]
    struct CounterMatcher;
    impl super::Matcher for CounterMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                && token.pos.get(2).is_some_and(|pos| pos == "助数詞")
        }
    }

    // Matcher for しか particle (助詞/係助詞)
    #[derive(Debug)]
    struct ShikaMatcher;
    impl super::Matcher for ShikaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "しか"
                && token.base_form == "しか"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Matcher for negative auxiliary ない or ん (for ません)
    #[derive(Debug)]
    struct NegativeAuxiliaryMatcher;
    impl super::Matcher for NegativeAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "助動詞")
                && ((token.base_form == "ない") || (token.base_form == "ん"))
        }
    }

    vec![
        // Match the counter (which comes after numbers in sequences like 五キロ, １００円)
        // The pattern starts matching from the counter token for simplicity
        TokenMatcher::Custom(Arc::new(CounterMatcher)),
        TokenMatcher::Custom(Arc::new(ShikaMatcher)),
        // Wildcard to allow various verb forms before negative (0-10 tokens)
        TokenMatcher::Wildcard {
            min: 0,
            max: 10,
            stop_conditions: vec![],
        },
        TokenMatcher::Custom(Arc::new(NegativeAuxiliaryMatcher)),
    ]
}

// Pattern: ～は～の一つだ
pub fn uff5e_ha_uff5e_nohitotsuda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～ない～はない
pub fn uff5e_nai_uff5e_hanai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: すこしも～ない
pub fn sukoshimo_uff5e_nai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: すくなくない
pub fn sukunakunai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ばあいは
pub fn baaiha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Verb[て] - casual imperative (て at sentence end)
// Structures: Verb[て]。
// Meaning: Shortened form of てください used for friendly requests
// Example: 片付けて。 (Please clean up.)
pub fn verb_te_2() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for て particle (接続助詞)
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl super::Matcher for TeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.base_form == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    vec![
        super::flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
    ]
}

// Pattern: てよかった
pub fn teyokatta() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Verb［せる・させる］
pub fn verb_uff3b_seru_u30fb_saseru_uff3d() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: といってもいい
pub fn toittemoii() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ても
pub fn temo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: てしまう・ちゃう
pub fn teshimau_u30fb_chau() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Verb[て] + B
pub fn verb_te_b_2() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Causative-Passive
pub fn causative_passive() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Verb[て]・Noun[で] + B
pub fn verb_te_u30fb_noun_de_b() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: てある (state of completion / left in state)
// Structures: (Transitive) Verb[て] + ある / (Transitive) Verb[て] + あります
pub fn tearu() -> Vec<TokenMatcher> {
    use super::concat;

    // Match て or で particle
    #[derive(Debug)]
    struct TeDeFormMatcher;
    impl Matcher for TeDeFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|p| p == "助詞")
        }
    }

    // Match ある as auxiliary verb (can be 自立 or 非自立)
    #[derive(Debug)]
    struct AruMatcher;
    impl Matcher for AruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "ある"
                && token.pos.first().is_some_and(|p| p == "動詞")
        }
    }

    // Match ます (optional for polite form)
    #[derive(Debug)]
    struct MasuFormMatcher;
    impl Matcher for MasuFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ます" && token.base_form == "ます"
        }
    }

    concat(vec![
        vec![super::flexible_verb_form()],
        vec![TokenMatcher::Custom(Arc::new(TeDeFormMatcher))],
        vec![TokenMatcher::Custom(Arc::new(AruMatcher))],
        vec![TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(MasuFormMatcher))))], // Optional ます
    ])
}

// Pattern: ように～てほしい
pub fn youni_uff5e_tehoshii() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ているあいだに
pub fn teiruaidani() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なくてもいい
pub fn nakutemoii() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: てみる (try doing)
// Structures: Verb[て] + みる
pub fn temiru() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct TeDeFormMatcher;
    impl Matcher for TeDeFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    #[derive(Debug)]
    struct MiruMatcher;
    impl Matcher for MiruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "みる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    vec![
        super::flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(TeDeFormMatcher)),
        TokenMatcher::Custom(Arc::new(MiruMatcher)),
    ]
}

// Pattern: てすみません
pub fn tesumimasen() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: てあげる (to do for someone)
// Structures: Verb[て] + あげる/あげます
pub fn teageru() -> Vec<TokenMatcher> {
    use super::concat;

    // Match て or で particle
    #[derive(Debug)]
    struct TeDeFormMatcher;
    impl Matcher for TeDeFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    #[derive(Debug)]
    struct AgeruMatcher;
    impl Matcher for AgeruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "あげる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    concat(vec![
        vec![super::flexible_verb_form()],
        vec![TokenMatcher::Custom(Arc::new(TeDeFormMatcher))],
        vec![TokenMatcher::Custom(Arc::new(AgeruMatcher))],
    ])
}

// Pattern: てくれる
pub fn tekureru() -> Vec<TokenMatcher> {
    use super::concat;

    // Match て/で particle OR ないで construction
    // For て-form: Verb(連用形/連用タ接続) + て/で
    // For ないで-form: Verb(未然形) + ない + で
    #[derive(Debug)]
    struct VerbFormMatcher;
    impl Matcher for VerbFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            if token.pos.first().is_none_or(|pos| pos != "動詞") {
                false
            } else {
                let form = token.features.get(5);
                // Match 連用形, 連用タ接続 (for て-form), or 未然形 (for ないで-form)
                form.is_some_and(|f| f == "連用形" || f == "連用タ接続" || f == "未然形")
            }
        }
    }

    #[derive(Debug)]
    struct TeDeFormMatcher;
    impl Matcher for TeDeFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
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

    #[derive(Debug)]
    struct KureruMatcher;
    impl Matcher for KureruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "くれる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    // Match both て-form and ないで-form
    // て-form: Verb(連用形/連用タ接続) + て/で + くれる
    // ないで-form: Verb(未然形) + ない + で + くれる
    concat(vec![
        vec![TokenMatcher::Custom(Arc::new(VerbFormMatcher))],
        vec![TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NaiAuxiliaryMatcher))))],
        vec![TokenMatcher::Custom(Arc::new(TeDeFormMatcher))],
        vec![TokenMatcher::Custom(Arc::new(KureruMatcher))],
    ])
}

// Pattern: てもらう
pub fn temorau() -> Vec<TokenMatcher> {
    use super::concat;

    // Match て or で particle
    #[derive(Debug)]
    struct TeDeFormMatcher;
    impl Matcher for TeDeFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    #[derive(Debug)]
    struct MorauMatcher;
    impl Matcher for MorauMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "もらう"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    concat(vec![
        vec![super::flexible_verb_form()],
        vec![TokenMatcher::Custom(Arc::new(TeDeFormMatcher))],
        vec![TokenMatcher::Custom(Arc::new(MorauMatcher))],
    ])
}

// Pattern: なさい
pub fn nasai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Verb[ないで]
// Pattern: Verb[ないで] (without doing)
// Structure: Verb[未然形] + ない + で
pub fn verb_naide() -> Vec<TokenMatcher> {
    naide()  // Same implementation as ないで
}

// Pattern: てくれてありがとう
pub fn tekuretearigatou() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: てくれない・てもらえない
pub fn tekurenai_u30fb_temoraenai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～のだろうか
pub fn uff5e_nodarouka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: お～になる 
pub fn o_uff5e_ninaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なさる
pub fn nasaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: お～ください (honorific request)
// Structures: お + Verb[連用形] + ください
pub fn o_uff5e_kudasai() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct OPrefixMatcher;
    impl super::Matcher for OPrefixMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "お"
                && token.base_form == "お"
                && token.pos.first().is_some_and(|pos| pos == "接頭詞")
                && token.pos.get(1).is_some_and(|pos| pos == "名詞接続")
        }
    }

    #[derive(Debug)]
    struct KudasaiMatcher;
    impl super::Matcher for KudasaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ください"
                && token.base_form == "くださる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(OPrefixMatcher)),
        super::flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(KudasaiMatcher)),
    ]
}

// Pattern: いらっしゃる
pub fn irassharu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ございます (polite form of ある)
// Structures: ござる (historical) / ござい + ます (modern polite)
pub fn gozaimasu() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct GozaruMatcher;
    impl super::Matcher for GozaruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "ござる"
                && (token.pos.first().is_some_and(|pos| pos == "助動詞")
                    || token.pos.first().is_some_and(|pos| pos == "動詞"))
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
        TokenMatcher::Custom(Arc::new(GozaruMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(MasuMatcher)))),
    ]
}

// Pattern: でございます
pub fn degozaimasu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: お〜する
pub fn o_u301c_suru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: いたす
pub fn itasu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ていただけませんか
pub fn teitadakemasenka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: たら
pub fn tara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ほかに(も)・ほか(に)は
pub fn hokani_mo_u30fb_hoka_ni_ha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: がひつよう
pub fn gahitsuyou() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: そんなに
pub fn sonnani() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ひつようがある
pub fn hitsuyougaaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: たとえば (for example)
// Structure: たとえば + Phrase
pub fn tatoeba() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct TatoebaMatcher;
    impl super::Matcher for TatoebaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "たとえば"
                && token.base_form == "たとえば"
                && token.pos.first().is_some_and(|pos| pos == "接続詞")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(TatoebaMatcher))]
}

// Pattern: れる・られる (Potential)
pub fn reru_u30fb_rareru_potential() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: んだけど・んですが
pub fn ndakedo_u30fb_ndesuga() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: はずだ
pub fn hazuda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: かどうか
pub fn kadouka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ないと
pub fn naito() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: はずがない
pub fn hazuganai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: しか～ない (only/nothing but)
// Structures: Noun + しか + Verb[ない]
pub fn shika_uff5e_nai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match しか particle
    #[derive(Debug)]
    struct ShikaMatcher;
    impl Matcher for ShikaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "しか"
                && token.base_form == "しか"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Match negative: ない (auxiliary verb or adjective) or ん (for ません)
    #[derive(Debug)]
    struct NegativeAuxiliaryMatcher;
    impl Matcher for NegativeAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            ((token.surface == "ない" && token.base_form == "ない")
                && (token.pos.first().is_some_and(|pos| pos == "助動詞")
                    || token.pos.first().is_some_and(|pos| pos == "形容詞")))
                || (token.surface == "ん" && token.base_form == "ん"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞"))
        }
    }

    vec![
        noun_matcher(),
        TokenMatcher::Custom(Arc::new(ShikaMatcher)),
        TokenMatcher::Wildcard {
            min: 0,
            max: 5,
            stop_conditions: vec![],
        },
        TokenMatcher::Custom(Arc::new(NegativeAuxiliaryMatcher)),
    ]
}

// Pattern: だけでなく
pub fn dakedenaku() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ことができる (can do / be able to)
// Structures: Verb + ことができる, Noun + ができる
pub fn kotogadekiru() -> Vec<TokenMatcher> {
    use super::Matcher;

    #[derive(Debug)]
    struct KotoMatcher;
    impl Matcher for KotoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    #[derive(Debug)]
    struct GaHaParticleMatcher;
    impl Matcher for GaHaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "が" || token.surface == "は")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    vec![
        TokenMatcher::Any,
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(KotoMatcher)))),
        TokenMatcher::Custom(Arc::new(GaHaParticleMatcher)),
        TokenMatcher::specific_verb("できる"),
    ]
}

// Pattern: かい
pub fn kai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: もし
pub fn moshi() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: し～し 
pub fn shi_uff5e_shi() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: でできる・からできる
pub fn dedekiru_u30fb_karadekiru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ながら
pub fn nagara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: たところだ
pub fn tatokoroda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ているところだ
pub fn teirutokoroda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: と～と、どちらが 
pub fn to_uff5e_to_u3001_dochiraga() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ようにする
pub fn younisuru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なければいけない
pub fn nakerebaikenai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なければならない
pub fn nakerebanaranai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: つづける
// Pattern: つづける - continue doing
// Structures: Verb[stem] + 続ける/つづける
pub fn tsuzukeru() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct TsuzukeruMatcher;
    impl Matcher for TsuzukeruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.base_form == "つづける" || token.base_form == "続ける")
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }
    vec![super::flexible_verb_form(), TokenMatcher::Custom(Arc::new(TsuzukeruMatcher))]
}

// Pattern: ようにいう
pub fn youniiu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: よていだ
pub fn yoteida() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ようにいのる
pub fn youniinoru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: たばかり
pub fn tabakari() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 化する
pub fn kasuru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 命令形
pub fn meireigata() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ように
pub fn youni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: かしら (I wonder)
// Structures: Phrase + かしら
pub fn kashira() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KashiraMatcher;
    impl super::Matcher for KashiraMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "かしら"
                && token.base_form == "かしら"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "終助詞")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(KashiraMatcher))]
}

// Pattern: らしい ②
pub fn rashii_u2461() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にみえる
pub fn nimieru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: とみえる
pub fn tomieru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 風
pub fn kaze() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: がみられる
pub fn gamirareru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にきがつく
pub fn nikigatsuku() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 〜でも 〜でも
pub fn u301c_demo_u301c_demo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: それに (moreover/in addition/what's more)
// Structure: それに + (Additional Information) Phrase
// Note: Can be tokenized as single conjunction token OR as それ + に (two tokens)
pub fn soreni() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher that handles BOTH tokenizations:
    // 1. Single token: それに (接続詞)
    // 2. First token of two-token sequence: それ (名詞/代名詞)
    #[derive(Debug)]
    struct SoreniOrSoreMatcher;
    impl super::Matcher for SoreniOrSoreMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match single-token それに (接続詞)
            if token.surface == "それに"
                && token.base_form == "それに"
                && token.pos.first().is_some_and(|pos| pos == "接続詞")
            {
                return true;
            }

            // Match それ (pronoun) - first token of two-token sequence
            if token.surface == "それ"
                && token.base_form == "それ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "代名詞")
            {
                return true;
            }

            false
        }
    }

    // Matcher for に (case particle) - only used for two-token sequence
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.base_form == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(SoreniOrSoreMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NiParticleMatcher,
        )))),
    ]
}

// Pattern: それで (therefore/so/as a result)
// Structure: Phrase (A)。それで + Phrase (B)
pub fn sorede() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    #[derive(Debug)]
    struct SoredeMatcher;
    impl super::Matcher for SoredeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "それで"
                && token.base_form == "それで"
                && token.pos.first().is_some_and(|pos| pos == "接続詞")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(SoredeMatcher))]
}

// Pattern: Question-phrase + か
pub fn question_phrase_ka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: それでも (even so/nevertheless)
// Structure: Phrase (A) + それでも + Phrase (B)
pub fn soredemo() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    #[derive(Debug)]
    struct SoredemoMatcher;
    impl super::Matcher for SoredemoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "それでも"
                && token.base_form == "それでも"
                && token.pos.first().is_some_and(|pos| pos == "接続詞")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(SoredemoMatcher))]
}

// Pattern: たらどう
pub fn taradou() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: とかんがえられている
pub fn tokangaerareteiru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: とされている
pub fn tosareteiru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: といわれている
pub fn toiwareteiru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ばよかった
pub fn bayokatta() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: し～し (listing reasons with equal weight)
// Structures: Verb/い-Adj + し, な-Adj/Noun + だ + し
pub fn shi_u301c_shi() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct ShiParticleMatcher;
    impl super::Matcher for ShiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "し"
                && token.base_form == "し"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    #[derive(Debug)]
    struct DaCopulaMatcher;
    impl super::Matcher for DaCopulaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "だ"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token
                    .features
                    .get(4)
                    .is_some_and(|f| f == "特殊・ダ")
                && token.features.get(5).is_some_and(|f| f == "基本形")
        }
    }

    vec![
        TokenMatcher::Any, // Verb, い-Adjective, な-Adjective (名詞/形容動詞語幹), or Noun
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(DaCopulaMatcher)))), // Optional だ (for na-adj/noun)
        TokenMatcher::Custom(Arc::new(ShiParticleMatcher)), // し (接続助詞)
    ]
}
