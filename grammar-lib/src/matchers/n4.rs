use crate::pattern_matcher::TokenMatcher;
use crate::matchers::{Matcher, noun_matcher};
use std::sync::Arc;

// Pattern: と (conditional - definite result)
// Structures: Verb + と / い-Adjective + と / な-Adjective + だ + と / Noun + だ + と
//
// Meaning: "if/when (A), then (B) will definitely happen"
// Note: Implies a definite/inevitable result, different from hypothetical conditionals
pub fn to() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct ToConditionalMatcher;
    impl Matcher for ToConditionalMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "と"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
        }
    }

    vec![
        TokenMatcher::Any, // Matches verb, adjective, or noun/な-adj + だ before と
        TokenMatcher::Custom(Arc::new(ToConditionalMatcher)),
    ]
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

// Pattern: ～ら (pluralizing suffix for pronouns)
// Structures: Pronoun + ら
//
// Note: Some pronouns like 彼ら tokenize as single tokens (名詞/代名詞/一般)
// but most like 私ら/お前ら split into Pronoun + ら(名詞/接尾).
// This matcher handles the split pattern (Pronoun + ら suffix).
pub fn uff5e_ra() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for pronouns (名詞/代名詞)
    #[derive(Debug)]
    struct PronounMatcher;
    impl super::Matcher for PronounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "代名詞")
        }
    }

    // Matcher for ら suffix (名詞/接尾)
    #[derive(Debug)]
    struct RaSuffixMatcher;
    impl super::Matcher for RaSuffixMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ら"
                && token.base_form == "ら"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(PronounMatcher)),
        TokenMatcher::Custom(Arc::new(RaSuffixMatcher))
    ]
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
// Pattern: だが・ですが (but, however - formal)
// Structures: だが + Phrase, ですが + Phrase
// Two tokenization patterns:
//   1. Single token: だが (接続詞) - at sentence start/after punctuation
//   2. Split tokens: だ/です (助動詞) + が (助詞/接続助詞) - mid-sentence
pub fn daga_u30fb_desuga() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match だが as single conjunction token OR だ/です auxiliary
    #[derive(Debug)]
    struct DagaDesugaMatcher;
    impl super::Matcher for DagaDesugaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Pattern 1: だが as single conjunction token (sentence-start)
            if token.surface == "だが"
                && token.base_form == "だが"
                && token.pos.first().is_some_and(|pos| pos == "接続詞")
            {
                return true;
            }

            // Pattern 2: だ or です as auxiliary (will be followed by が)
            if (token.surface == "だ" || token.surface == "です")
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
            {
                return true;
            }

            false
        }
    }

    // Match が as conjunction particle (for split form)
    #[derive(Debug)]
    struct GaConjunctionMatcher;
    impl super::Matcher for GaConjunctionMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "が"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // The first matcher handles both だが (conjunction) and だ/です (auxiliary)
    // If it matches だが (conjunction), the optional が won't match (correct)
    // If it matches だ/です (auxiliary), the optional が will match (correct)
    vec![
        TokenMatcher::Custom(Arc::new(DagaDesugaMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(GaConjunctionMatcher)))),
    ]
}

// Pattern: なくて (negative て-form)
// Structures: Verb/Adjective + なくて
// Matches: Verb[未然形] + なく(助動詞) + て OR なく(形容詞) + て
pub fn nakute() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match なく as auxiliary or adjective (from ない)
    #[derive(Debug)]
    struct NakuMatcher;
    impl super::Matcher for NakuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "なく"
                && token.base_form == "ない"
                && (token.pos.first().is_some_and(|pos| pos == "助動詞")
                    || token.pos.first().is_some_and(|pos| pos == "形容詞"))
                && token.features.get(5).is_some_and(|f| f == "連用テ接続")
        }
    }

    // Match て as conjunction particle
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
        TokenMatcher::Custom(Arc::new(NakuMatcher)),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
    ]
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

// Pattern: Verb［れる・られる］(Passive form - something happens to the subject)
// Structures: Verb[未然形] + れる/られる
pub fn verb_uff3b_reru_u30fb_rareru_uff3d() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    use super::Matcher;

    // Matcher for verbs in 未然形 (negative/passive stem)
    // This includes all verb types before passive auxiliary れる/られる
    #[derive(Debug)]
    struct PassiveStemMatcher;
    impl Matcher for PassiveStemMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            if !token.pos.first().is_some_and(|pos| pos == "動詞") {
                return false;
            }
            // Match 未然形 (negative/passive stem) or 未然レル接続 (for する verbs)
            token.features.get(5).is_some_and(|form| {
                form == "未然形" || form == "未然レル接続"
            })
        }
    }

    // Matcher for passive auxiliary れる/られる as suffix verb
    // Tokenized as 動詞/接尾 with base form れる or られる
    #[derive(Debug)]
    struct PassiveAuxiliaryMatcher;
    impl Matcher for PassiveAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                && (token.base_form == "れる" || token.base_form == "られる")
        }
    }

    // Optional ます for polite form
    #[derive(Debug)]
    struct MasuMatcher;
    impl Matcher for MasuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ます"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(PassiveStemMatcher)),
        TokenMatcher::Custom(Arc::new(PassiveAuxiliaryMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(MasuMatcher)))),
    ]
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

// Pattern: とき (when / at the time)
// Structures:
//   - Verb + とき
//   - い-Adjective + とき
//   - な-Adjective + な + とき
//   - Noun + の + とき
//
// Meaning: "when", "at the time" - temporal noun indicating when something happens
pub fn toki() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct TokiMatcher;
    impl Matcher for TokiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "とき"
                && token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "非自立")
        }
    }

    vec![
        TokenMatcher::Any,
        TokenMatcher::Custom(Arc::new(TokiMatcher)),
    ]
}

// Pattern: まず (first of all / to start with)
// Structure: まず + Phrase
//
// Meaning: "first", "to begin with", "starting with" - indicates priority/sequence
pub fn mazu() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct MazuMatcher;
    impl Matcher for MazuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "まず"
                && token.pos.first().is_some_and(|p| p == "副詞")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(MazuMatcher))]
}

// Pattern: まで (even, to the extent)
// Structures: Noun + まで(も)
//
// This is the N4 "even" meaning of まで (adverbial particle).
// Tokenizes identically to N5 まで (until/to) - only semantic difference.
// Both N5 and N4 patterns will match the same text.
// Application should show both grammar explanations to user.
//
// Tokenization: Noun + まで (助詞/副助詞)
// Note: まで + も is matched as separate tokens, not a compound
pub fn made() -> Vec<TokenMatcher> {
    use super::noun_matcher;

    #[derive(Debug)]
    struct MadeParticleMatcher;
    impl Matcher for MadeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "まで"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "副助詞")
        }
    }

    vec![
        noun_matcher(),
        TokenMatcher::Custom(Arc::new(MadeParticleMatcher)),
    ]
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

// Pattern: また (again/also)
// Structures: また + Phrase
pub fn mata() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct MataMatcher;
    impl Matcher for MataMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "また"
                && token.base_form == "また"
                && token.pos.first().is_some_and(|pos| pos == "接続詞")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(MataMatcher))]
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

// Pattern: ごろ (around/about time)
// Structures: Noun + ごろ, Noun + の + ころ
pub fn goro() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct GoroKoroMatcher;
    impl Matcher for GoroKoroMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Matches both ごろ (接尾) and ころ (非自立)
            (token.surface == "ごろ" || token.surface == "ころ")
                && (token.base_form == "ごろ" || token.base_form == "ころ")
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "接尾")
                    || token.pos.get(1).is_some_and(|pos| pos == "非自立"))
        }
    }
    vec![TokenMatcher::Custom(Arc::new(GoroKoroMatcher))]
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

// Pattern: なるべく (as much as possible)
// Structure: なるべく + Phrase
pub fn narubeku() -> Vec<TokenMatcher> {
    use super::Matcher;
    use std::sync::Arc;

    // Match なるべく adverb
    #[derive(Debug)]
    struct NarubekuMatcher;
    impl Matcher for NarubekuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "なるべく"
                && token.base_form == "なるべく"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(NarubekuMatcher))]
}

// Pattern: るところだ
pub fn rutokoroda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: のに (despite)
// Structures: Verb/い-Adj + のに, Noun/な-Adj + な + のに
pub fn noni() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match な (助動詞, 特殊・ダ, 体言接続) for nouns and な-adjectives
    #[derive(Debug)]
    struct NaAuxiliaryMatcher;
    impl super::Matcher for NaAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "な"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(4).is_some_and(|f| f == "特殊・ダ")
                && token.features.get(5).is_some_and(|f| f == "体言接続")
        }
    }

    // Match のに (助詞/接続助詞)
    #[derive(Debug)]
    struct NoniParticleMatcher;
    impl super::Matcher for NoniParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "のに"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    vec![
        TokenMatcher::Any,
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NaAuxiliaryMatcher,
        )))),
        TokenMatcher::Custom(Arc::new(NoniParticleMatcher)),
    ]
}

// Pattern: とおもう
pub fn toomou() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: など (such as, and so on)
// Structures: Noun + など
pub fn nado() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct NadoMatcher;
    impl super::Matcher for NadoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "など"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(NadoMatcher)),
    ]
}

// Pattern: みたい
pub fn mitai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: そう (looks like/seems like - appearance-based conjecture)
// Structures: Verb[stem] + そう、い-Adj[stem] + そう、な-Adj + そう
pub fn sou() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match verb in 連用形 (stem form)
    #[derive(Debug)]
    struct VerbStemMatcher;
    impl super::Matcher for VerbStemMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形")
        }
    }

    // Match い-adjective in ガル接続 (stem form)
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
    struct NaAdjectiveMatcher;
    impl super::Matcher for NaAdjectiveMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹")
        }
    }

    // Match auxiliary ない in ガル接続 (for negative forms)
    #[derive(Debug)]
    struct NaiAuxiliaryGaruMatcher;
    impl super::Matcher for NaiAuxiliaryGaruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "な"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "ガル接続")
        }
    }

    // Match さ suffix (for negative forms: なさそう)
    #[derive(Debug)]
    struct SaSuffixMatcher;
    impl super::Matcher for SaSuffixMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "さ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
        }
    }

    // Match verb/adjective stem OR な in negative forms
    #[derive(Debug)]
    struct StemOrNaiMatcher;
    impl super::Matcher for StemOrNaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Verb stem (連用形)
            (token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形"))
            ||
            // い-Adjective stem (ガル接続)
            (token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "ガル接続"))
            ||
            // な-Adjective (形容動詞語幹)
            (token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹"))
            ||
            // ない auxiliary in ガル接続 (for negative forms)
            (token.surface == "な"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "ガル接続"))
        }
    }

    // Match そう auxiliary (名詞/接尾/助動詞語幹)
    #[derive(Debug)]
    struct SouAuxiliaryMatcher;
    impl super::Matcher for SouAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "そう"
                && token.base_form == "そう"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                && token.pos.get(2).is_some_and(|pos| pos == "助動詞語幹")
        }
    }

    // Match だ or です copula
    #[derive(Debug)]
    struct DaDesuCopulaMatcher;
    impl super::Matcher for DaDesuCopulaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "だ" || token.surface == "です")
                && (token.base_form == "だ" || token.base_form == "です")
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(StemOrNaiMatcher)), // Verb/Adj stem OR な (negative)
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(SaSuffixMatcher)))), // Optional さ (for なさそう)
        TokenMatcher::Custom(Arc::new(SouAuxiliaryMatcher)), // そう
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(DaDesuCopulaMatcher)))), // Optional だ/です
    ]
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
// Pattern: そういう (like that, that kind of)
// Structures: こういう/そういう/どういう (single token) OR ああ + いう (two tokens)
pub fn souiu() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for single-token forms: こういう, そういう, どういう
    #[derive(Debug)]
    struct SouiuSingleMatcher;
    impl Matcher for SouiuSingleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "連体詞")
                && (token.base_form == "こういう"
                    || token.base_form == "そういう"
                    || token.base_form == "どういう")
        }
    }

    // Matcher for ああ (interjection part of ああいう)
    #[derive(Debug)]
    struct AaMatcher;
    impl Matcher for AaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ああ"
                && token.base_form == "ああ"
                && token.pos.first().is_some_and(|pos| pos == "感動詞")
        }
    }

    // Matcher for いう (verb part)
    #[derive(Debug)]
    struct IuVerbMatcher;
    impl Matcher for IuVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "いう"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立")
        }
    }

    // We need to match BOTH patterns:
    // 1. Single token (こういう/そういう/どういう)
    // 2. Two tokens (ああ + いう)
    // Since TokenMatcher doesn't support OR logic, we'll use a custom matcher
    // that checks for either pattern

    #[derive(Debug)]
    struct SouiuPatternMatcher;
    impl Matcher for SouiuPatternMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Check single-token forms
            if token.pos.first().is_some_and(|pos| pos == "連体詞")
                && (token.base_form == "こういう"
                    || token.base_form == "そういう"
                    || token.base_form == "どういう")
            {
                return true;
            }

            // Check first part of ああいう
            if token.surface == "ああ"
                && token.base_form == "ああ"
                && token.pos.first().is_some_and(|pos| pos == "感動詞")
            {
                return true;
            }

            false
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(SouiuPatternMatcher)),
        // Optional second token for ああいう pattern
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(IuVerbMatcher)))),
    ]
}

// Pattern: Verb[よう]
// Pattern: Verb[よう] (volitional form - let's, shall)
// Structures: Verb[未然ウ接続] + う OR Verb[連用形] + ましょ + う
pub fn verb_you() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match verb in 未然ウ接続 (for standard form) or 連用形 (for polite form)
    #[derive(Debug)]
    struct VolitionalVerbMatcher;
    impl super::Matcher for VolitionalVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立")
                && token
                    .features
                    .get(5)
                    .is_some_and(|form| form == "未然ウ接続" || form == "連用形")
        }
    }

    // Match ましょ (auxiliary verb for polite volitional)
    #[derive(Debug)]
    struct MashouMatcher;
    impl super::Matcher for MashouMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ましょ"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Match う (auxiliary verb for volitional)
    #[derive(Debug)]
    struct UMatcher;
    impl super::Matcher for UMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "う"
                && token.base_form == "う"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(4).is_some_and(|t| t == "不変化型")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VolitionalVerbMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(MashouMatcher)))),
        TokenMatcher::Custom(Arc::new(UMatcher)),
    ]
}

// Pattern: ようだ
pub fn youda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ぜんぜん (not at all - with negative expressions)
// Structure: ぜんぜん
pub fn zenzen() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct ZenzenMatcher;
    impl super::Matcher for ZenzenMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ぜんぜん"
                && token.base_form == "ぜんぜん"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(ZenzenMatcher))]
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

// Pattern: そうに・そうな (seems like/looks like - adverbial and attributive forms)
// Structures: Verb/Adj[stem] + そう + に/な
pub fn souni_u30fb_souna() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match verb/adjective stem OR な in negative forms (reused from そう pattern)
    #[derive(Debug)]
    struct StemOrNaiMatcher;
    impl super::Matcher for StemOrNaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Verb stem (連用形)
            (token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形"))
            ||
            // い-Adjective stem (ガル接続)
            (token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "ガル接続"))
            ||
            // な-Adjective (形容動詞語幹)
            (token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹"))
            ||
            // ない auxiliary in ガル接続 (for negative forms)
            (token.surface == "な"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "ガル接続"))
            ||
            // ない adjective in ガル接続 (for negative forms: 大事じゃな)
            (token.surface == "な"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "ガル接続"))
        }
    }

    // Match さ suffix (for negative forms: なさそう)
    #[derive(Debug)]
    struct SaSuffixMatcher;
    impl super::Matcher for SaSuffixMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "さ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
        }
    }

    // Match そう auxiliary (名詞/接尾/助動詞語幹 OR 副詞/助詞類接続 for negative forms)
    #[derive(Debug)]
    struct SouAuxiliaryMatcher;
    impl super::Matcher for SouAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "そう"
                && token.base_form == "そう"
                && ((token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                    && token.pos.get(2).is_some_and(|pos| pos == "助動詞語幹"))
                || (token.pos.first().is_some_and(|pos| pos == "副詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "助詞類接続")))
        }
    }

    // Match に particle (adverbial form: そうに)
    // Can be 助詞/格助詞/一般 OR 助詞/副詞化
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.base_form == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "格助詞" || pos == "副詞化"))
        }
    }

    // Match な auxiliary (attributive form: そうな)
    // 助動詞, 体言接続, base=だ
    #[derive(Debug)]
    struct NaAuxiliaryMatcher;
    impl super::Matcher for NaAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "な"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "体言接続")
        }
    }

    // Match either に (adverbial) or な (attributive)
    #[derive(Debug)]
    struct NiOrNaMatcher;
    impl super::Matcher for NiOrNaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // に particle (adverbial)
            (token.surface == "に"
                && token.base_form == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "格助詞" || pos == "副詞化")))
            ||
            // な auxiliary (attributive)
            (token.surface == "な"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "体言接続"))
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(StemOrNaiMatcher)), // Verb/Adj stem OR な (negative)
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(SaSuffixMatcher)))), // Optional さ (for なさそう)
        TokenMatcher::Custom(Arc::new(SouAuxiliaryMatcher)), // そう
        TokenMatcher::Custom(Arc::new(NiOrNaMatcher)), // に (adverbial) or な (attributive)
    ]
}

// Pattern: のように・のような 
pub fn noyouni_u30fb_noyouna() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 〜ようと思う・〜おうと思う (intend to/thinking of doing)
// Structures: Verb[未然ウ接続] + う + と + 思う/思っている/思います/思っています
pub fn u301c_youtoomou_u30fb_u301c_outoomou() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match quotation particle と
    #[derive(Debug)]
    struct QuotationToMatcher;
    impl super::Matcher for QuotationToMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "と"
                && token.base_form == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "引用")
        }
    }

    // Match 思う (in any conjugation)
    #[derive(Debug)]
    struct OmouMatcher;
    impl super::Matcher for OmouMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "思う"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    // Match て particle (for 思っている forms)
    #[derive(Debug)]
    struct TeMatcher;
    impl super::Matcher for TeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Match いる/います (for ている forms)
    #[derive(Debug)]
    struct IruMatcher;
    impl super::Matcher for IruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "いる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    // Match ます (for polite forms)
    #[derive(Debug)]
    struct MasuMatcher;
    impl super::Matcher for MasuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    super::concat(vec![
        verb_you(),  // Verb[未然ウ接続] + う
        vec![TokenMatcher::Custom(Arc::new(QuotationToMatcher))],
        vec![TokenMatcher::Custom(Arc::new(OmouMatcher))],
        // Optional: て + いる (for 思っている)
        vec![TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(TeMatcher))))],
        vec![TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(IruMatcher))))],
        // Optional: ます (for polite forms)
        vec![TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(MasuMatcher))))],
    ])
}

// Pattern: く・に (adverb formation)
// Structures: い-Adj[く] + Verb, な-Adj + に + Verb, Exception: いい→よく
// Note: This pattern is very broad and matches adverbial forms modifying verbs.
// It's not highlighted in overlays (priority < 5) but useful for detection.
pub fn ku_u30fb_ni() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for adverbial forms (い-adj[く], な-adj+に, or よく exception)
    #[derive(Debug)]
    struct AdverbialFormMatcher;
    impl Matcher for AdverbialFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Pattern 1: い-adjective in 連用テ接続 form (ends with く)
            let is_i_adj_ku = token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立")
                && token.features.get(5).is_some_and(|form| form == "連用テ接続")
                && token.surface.ends_with("く");

            // Pattern 2: な-adjective stem (形容動詞語幹)
            let is_na_adj_stem = token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹");

            // Pattern 3: よく adverb (exception for いい)
            let is_yoku = token.surface == "よく"
                && token.base_form == "よく"
                && token.pos.first().is_some_and(|pos| pos == "副詞");

            is_i_adj_ku || is_na_adj_stem || is_yoku
        }
    }

    // Matcher for に adverbial particle (optional - only for な-adj)
    #[derive(Debug)]
    struct NiAdverbialMatcher;
    impl Matcher for NiAdverbialMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副詞化")
        }
    }

    // Matcher for any verb
    #[derive(Debug)]
    struct VerbMatcher;
    impl Matcher for VerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(AdverbialFormMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NiAdverbialMatcher)))),
        TokenMatcher::Custom(Arc::new(VerbMatcher)),
    ]
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
// Pattern: そんな・こんな・あんな・どんな (like that, like this, what kind of)
// Structures: そんな/こんな/あんな/どんな (all single tokens, 連体詞)
// Abbreviations from: そのような, このような, あのような, どのような
pub fn sonna_u30fb_konna_u30fb_anna_u30fb_donna() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for all four forms
    #[derive(Debug)]
    struct SonnaMatcher;
    impl Matcher for SonnaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "連体詞")
                && (token.base_form == "そんな"
                    || token.base_form == "こんな"
                    || token.base_form == "あんな"
                    || token.base_form == "どんな")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(SonnaMatcher))]
}

// Pattern: 各
pub fn kaku() -> Vec<TokenMatcher> {
    // Match 各 as prefix (接頭詞/名詞接続)
    #[derive(Debug)]
    struct KakuPrefixMatcher;
    impl super::Matcher for KakuPrefixMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "各"
                && token.base_form == "各"
                && token.pos.first().is_some_and(|pos| pos == "接頭詞")
                && token.pos.get(1).is_some_and(|pos| pos == "名詞接続")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(KakuPrefixMatcher)),
        super::noun_matcher(),
    ]
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
// Pattern: だいたい (generally, mostly, approximately, in the first place)
// Structures: だいたい as adverb or noun
pub fn daitai() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct DaitaiMatcher;
    impl Matcher for DaitaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "だいたい"
                && token.base_form == "だいたい"
                && (token.pos.first().is_some_and(|pos| pos == "副詞")
                    || token.pos.first().is_some_and(|pos| pos == "名詞"))
        }
    }
    vec![TokenMatcher::Custom(Arc::new(DaitaiMatcher))]
}

// Pattern: のなかで
pub fn nonakade() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ように・ような
pub fn youni_u30fb_youna() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Number/Amount + は (at least, or so)
// Structures: Counter + (くらい/ぐらい) + は  (e.g., 回は, キロくらいは)
// Examples: ５回は (at least 5 times), ２キロくらいは (at least 2 kg)
//
// Note: This pattern detects the contrastive use of は after counters to mean "at least" or "or so".
// The pattern starts from the counter (not the number), as the counter + は is the key construction.
pub fn number_amount_ha() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match counter (助数詞) like 回, キロ, 時間, etc.
    #[derive(Debug)]
    struct CounterMatcher;
    impl Matcher for CounterMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                && token.pos.get(2).is_some_and(|pos| pos == "助数詞")
        }
    }

    // Match くらい or ぐらい (副助詞)
    #[derive(Debug)]
    struct KuraiMatcher;
    impl Matcher for KuraiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "くらい" || token.surface == "ぐらい")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞")
        }
    }

    // Match は (係助詞)
    #[derive(Debug)]
    struct HaMatcher;
    impl Matcher for HaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.base_form == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(CounterMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(KuraiMatcher)))),
        TokenMatcher::Custom(Arc::new(HaMatcher)),
    ]
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

// Pattern: すこしも～ない (not even a little)
// Structure: すこしも
pub fn sukoshimo_uff5e_nai() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct SukoshimoMatcher;
    impl super::Matcher for SukoshimoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "すこしも"
                && token.base_form == "すこしも"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(SukoshimoMatcher))]
}

// Pattern: すくなくない - not few (quite a few, many)
// Structures: 少なく + ない / 少なく + ありません
pub fn sukunakunai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match 少なく (the adjective 少ない in 連用テ接続 form)
    #[derive(Debug)]
    struct SukunakuMatcher;
    impl Matcher for SukunakuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "少なく"
                && token.base_form == "少ない"
                && token.pos.first().is_some_and(|p| p == "形容詞")
        }
    }

    // Match ない (auxiliary verb)
    #[derive(Debug)]
    struct NaiAuxMatcher;
    impl Matcher for NaiAuxMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ない"
                && token.pos.first().is_some_and(|p| p == "助動詞")
        }
    }

    // Match あり (verb ある in 連用形)
    #[derive(Debug)]
    struct AriMatcher;
    impl Matcher for AriMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "あり"
                && token.base_form == "ある"
                && token.pos.first().is_some_and(|p| p == "動詞")
        }
    }

    // Match ませ (auxiliary verb ます in 未然形)
    #[derive(Debug)]
    struct MaseMatcher;
    impl Matcher for MaseMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ませ"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|p| p == "助動詞")
        }
    }

    // Match ん (auxiliary verb, negative)
    #[derive(Debug)]
    struct NMatcher;
    impl Matcher for NMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ん"
                && token.pos.first().is_some_and(|p| p == "助動詞")
        }
    }

    // The pattern can be either:
    // 1. 少なく + ない (casual)
    // 2. 少なく + あり + ませ + ん (polite)
    // We need to use alternatives or check both patterns
    // For simplicity, we'll match the beginning and use optional matchers

    vec![
        TokenMatcher::Custom(Arc::new(SukunakuMatcher)),
        TokenMatcher::Custom(Arc::new(NaiAuxMatcher)),  // Matches casual form
    ]
}

// Pattern: すくなくない (polite) - not few (quite a few, many)
// Structures: 少なく + ありません
pub fn sukunakunai_polite() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match 少なく (the adjective 少ない in 連用テ接続 form)
    #[derive(Debug)]
    struct SukunakuMatcher;
    impl Matcher for SukunakuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "少なく"
                && token.base_form == "少ない"
                && token.pos.first().is_some_and(|p| p == "形容詞")
        }
    }

    // Match あり (verb ある in 連用形)
    #[derive(Debug)]
    struct AriMatcher;
    impl Matcher for AriMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "あり"
                && token.base_form == "ある"
                && token.pos.first().is_some_and(|p| p == "動詞")
        }
    }

    // Match ませ (auxiliary verb ます in 未然形)
    #[derive(Debug)]
    struct MaseMatcher;
    impl Matcher for MaseMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ませ"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|p| p == "助動詞")
        }
    }

    // Match ん (auxiliary verb, negative)
    #[derive(Debug)]
    struct NMatcher;
    impl Matcher for NMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ん"
                && token.pos.first().is_some_and(|p| p == "助動詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(SukunakuMatcher)),
        TokenMatcher::Custom(Arc::new(AriMatcher)),
        TokenMatcher::Custom(Arc::new(MaseMatcher)),
        TokenMatcher::Custom(Arc::new(NMatcher)),
    ]
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

// Pattern: てよかった - "glad that" / "I'm glad that..."
// Structures: Various + て/で + よかった (+ optional です)
// Can attach to verbs, adjectives, nouns via て or で
pub fn teyokatta() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match て/で particle OR で copula
    // て or で particle: after て-form verbs/adjectives or ない
    // で copula: after nouns/な-adjectives
    #[derive(Debug)]
    struct TeDeOrCopulaMatcher;
    impl super::Matcher for TeDeOrCopulaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Case 1: て or で particle (after verbs/adjectives/ない)
            let is_particle = (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞");

            // Case 2: で copula (after nouns/な-adjectives)
            let is_copula = token.surface == "で"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞");

            is_particle || is_copula
        }
    }

    // Match よかっ (the te-connecting form of よい)
    #[derive(Debug)]
    struct YokattaMatcher;
    impl super::Matcher for YokattaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "よかっ"
                && token.base_form == "よい"
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
        }
    }

    // Match です (optional polite ending)
    #[derive(Debug)]
    struct DesuMatcher;
    impl super::Matcher for DesuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "です"
                && token.base_form == "です"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        // Match either て/で particle OR で copula
        TokenMatcher::Custom(Arc::new(TeDeOrCopulaMatcher)),
        // よかっ
        TokenMatcher::Custom(Arc::new(YokattaMatcher)),
        // た
        super::past_auxiliary(),
        // Optional です
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(DesuMatcher)))),
    ]
}

// Pattern: Verb［せる・させる］(Causative form - make/let someone do)
// Structures: Verb[未然形] + せる/させる
pub fn verb_uff3b_seru_u30fb_saseru_uff3d() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    use super::Matcher;

    // Matcher for verbs in 未然形 (negative/causative stem)
    // This includes all verb types before causative auxiliary せる/させる
    #[derive(Debug)]
    struct CausativeStemMatcher;
    impl Matcher for CausativeStemMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            if !token.pos.first().is_some_and(|pos| pos == "動詞") {
                return false;
            }
            // Match 未然形 (negative/causative stem) or 未然レル接続 (for する verbs)
            token.features.get(5).is_some_and(|form| {
                form == "未然形" || form == "未然レル接続"
            })
        }
    }

    // Matcher for causative auxiliary せる/させる as suffix verb
    // Tokenized as 動詞/接尾 with base form せる or させる
    #[derive(Debug)]
    struct CausativeAuxiliaryMatcher;
    impl Matcher for CausativeAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                && (token.base_form == "せる" || token.base_form == "させる")
        }
    }

    // Optional ます for polite form
    #[derive(Debug)]
    struct MasuMatcher;
    impl Matcher for MasuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ます"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(CausativeStemMatcher)),
        TokenMatcher::Custom(Arc::new(CausativeAuxiliaryMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(MasuMatcher)))),
    ]
}

// Pattern: といってもいい
pub fn toittemoii() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ても (even if/even though)
// Structures:
//   - Verb[て] + も / Verb[なくて] + も
//   - い-Adjective[て] + も / い-Adj[なくて] + も
//   - な-Adjective + でも (single token) / な-Adj + じゃなくて + も
//   - Noun + で + も (two tokens) / Noun + じゃなくて + も
pub fn temo() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    use super::Matcher;

    // Matcher for verbs/adjectives in te-form before ても
    // Includes: Verb[連用タ接続], い-Adj[連用テ接続], and Verb/Adj + なく
    #[derive(Debug)]
    struct TemoStemMatcher;
    impl Matcher for TemoStemMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Verb in 連用タ接続 or 連用テ接続 (for て-form)
            if token.pos.first().is_some_and(|p| p == "動詞") {
                token.features.get(5).is_some_and(|f| f == "連用タ接続" || f == "連用テ接続" || f == "未然形")
            }
            // い-adjective in 連用テ接続
            else if token.pos.first().is_some_and(|p| p == "形容詞") {
                token.features.get(5).is_some_and(|f| f == "連用テ接続")
            }
            // なく (negative auxiliary in 連用テ接続)
            else if token.surface == "なく" && token.base_form == "ない" {
                token.pos.first().is_some_and(|p| p == "助動詞")
                    && token.features.get(5).is_some_and(|f| f == "連用テ接続")
            }
            // な-adjective or noun (before でも or じゃ)
            else if token.pos.first().is_some_and(|p| p == "名詞") {
                true
            }
            else {
                false
            }
        }
    }

    // Matcher for て particle (conjunction) OR で particle (case marking) OR じゃ particle
    #[derive(Debug)]
    struct TeDeJaMatcher;
    impl Matcher for TeDeJaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // て (conjunction particle)
            if token.surface == "て" {
                token.pos.first().is_some_and(|p| p == "助詞")
                    && token.pos.get(1).is_some_and(|p| p == "接続助詞")
            }
            // で (case marking particle)
            else if token.surface == "で" {
                token.pos.first().is_some_and(|p| p == "助詞")
                    && token.pos.get(1).is_some_and(|p| p == "格助詞")
            }
            // じゃ (副助詞)
            else if token.surface == "じゃ" {
                token.pos.first().is_some_and(|p| p == "助詞")
                    && token.pos.get(1).is_some_and(|p| p == "副助詞")
            }
            // でも (single token - 副助詞)
            else if token.surface == "でも" && token.base_form == "でも" {
                token.pos.first().is_some_and(|p| p == "助詞")
                    && token.pos.get(1).is_some_and(|p| p == "副助詞")
            }
            else {
                false
            }
        }
    }

    // Matcher for も particle (binding particle) - marks the end of ても
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "も"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "係助詞")
        }
    }

    // This pattern is complex with many variations. The core suffix is:
    // - て + も (2 tokens)
    // - でも (1 token - single particle)
    // - で + も (2 tokens)
    //
    // The prefix can be 1-2 tokens (verb stem, adj, なく, じゃ+なく, etc.)
    // Rather than trying to match all prefixes, we'll match the suffix and let
    // the pattern matcher include appropriate preceding tokens via prioritization.
    //
    // Strategy: Match just the core suffix (て+も / で+も / でも)
    // This will be combined with the Wildcard to capture preceding context
    vec![
        // Match one preceding token (verb stem, adj stem, noun, or なく)
        TokenMatcher::Custom(Arc::new(TemoStemMatcher)),
        // Match て/で/じゃ/でも particle
        TokenMatcher::Custom(Arc::new(TeDeJaMatcher)),
        // Optionally match も (not needed when でも is single token)
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(MoParticleMatcher)))),
    ]
}

// Pattern: てしまう・ちゃう (completion/regret)
// Structures: Verb[て] + しまう / Verb + ちゃう / Verb + じゃう (+ ます optional)
pub fn teshimau_u30fb_chau() -> Vec<TokenMatcher> {
    use super::Matcher;
    use std::sync::Arc;

    // Match verb in 連用形 or 連用タ接続 (needed before て/ちゃう/じゃう)
    #[derive(Debug)]
    struct VerbRenyouFormMatcher;
    impl Matcher for VerbRenyouFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形" || f == "連用タ接続")
        }
    }

    // Match て particle (for てしまう pattern)
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl Matcher for TeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Match しまう verb (動詞/非自立)
    #[derive(Debug)]
    struct ShimauVerbMatcher;
    impl Matcher for ShimauVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "しまう"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    // Match ちゃう verb (contraction of てしまう)
    #[derive(Debug)]
    struct ChauVerbMatcher;
    impl Matcher for ChauVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "ちゃう"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    // Match じゃう verb (contraction of でしまう)
    #[derive(Debug)]
    struct JauVerbMatcher;
    impl Matcher for JauVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "じゃう"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    // Match ます polite auxiliary
    #[derive(Debug)]
    struct MasuMatcher;
    impl Matcher for MasuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Match しまう, ちゃう, or じゃう
    #[derive(Debug)]
    struct ShimauOrContractionMatcher;
    impl Matcher for ShimauOrContractionMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && (token.base_form == "しまう"
                    || token.base_form == "ちゃう"
                    || token.base_form == "じゃう")
        }
    }

    // てしまう pattern: Verb(連用タ接続) + て + しまう (+ Optional ます)
    // OR
    // ちゃう/じゃう pattern: Verb(連用タ接続) + ちゃう/じゃう (+ Optional ます)
    //
    // We need to match both patterns, so we use Optional for て and check for all three verb forms
    vec![
        TokenMatcher::Custom(Arc::new(VerbRenyouFormMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(TeParticleMatcher)))), // て is optional (for ちゃう/じゃう)
        TokenMatcher::Custom(Arc::new(ShimauOrContractionMatcher)), // Match しまう, ちゃう, or じゃう
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(MasuMatcher)))),
    ]
}

// Pattern: Verb[て] + B
// Pattern: Verb[て] + B (Contrastive conjunction)
// Expresses contrast using て-form with equal weight for both clauses
// Structures: Verb[連用形/連用タ接続] + て + (optional comma) + (noun) + は
// Examples:
//   - 姉ちゃんは勉強をして弟はゲームをしている (Sister studies, AND brother plays games)
//   - 妻は買い物に行って、私はごみを捨てに行った (Wife went shopping WHILE I threw trash)
//
// Note: Comma (、) is often used before the contrasting clause but is optional.
// This pattern handles both cases.
pub fn verb_te_b_2() -> Vec<TokenMatcher> {
    use super::Matcher;
    use std::sync::Arc;

    // Match て or で as conjunction particle
    #[derive(Debug)]
    struct TeConjunctionMatcher;
    impl Matcher for TeConjunctionMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Match は particle (topic/contrast marker)
    #[derive(Debug)]
    struct HaContrastMatcher;
    impl Matcher for HaContrastMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.base_form == "は"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "係助詞")
        }
    }

    // Match comma (、) punctuation
    #[derive(Debug)]
    struct CommaMatcher;
    impl Matcher for CommaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "、"
                && token.pos.first().is_some_and(|p| p == "記号")
                && token.pos.get(1).is_some_and(|p| p == "読点")
        }
    }

    // Pattern: Verb[連用形/連用タ接続] + て/で + (optional comma) + (0-3 tokens) + は
    // Note: Using two separate approaches to handle with/without comma
    // The wildcard stops at punctuation, so we need to explicitly include comma as optional
    super::concat(vec![
        vec![
            super::flexible_verb_form(),
            TokenMatcher::Custom(Arc::new(TeConjunctionMatcher)),
            TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(CommaMatcher)))),
        ],
        vec![TokenMatcher::Wildcard {
            min: 0,
            max: 3,
            stop_conditions: vec![],
        }],
        vec![TokenMatcher::Custom(Arc::new(HaContrastMatcher))],
    ])
}

// Pattern: Causative-Passive (to be made to do)
// Structures:
//   Long form: Verb[未然形] + せ/させ + られる (e.g., 食べさせられる, 歩かせられる)
//   Short form: Verb_causative[未然形] + れる (e.g., 飲まされる = 飲ます + れる)
pub fn causative_passive() -> Vec<TokenMatcher> {
    use super::Matcher;
    use std::sync::Arc;

    // Match verb in 未然形 or 未然レル接続 (for する)
    #[derive(Debug)]
    struct MizenFormMatcher;
    impl Matcher for MizenFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Must be a verb
            if !token.pos.first().is_some_and(|pos| pos == "動詞") {
                return false;
            }

            // Check conjugation form in features[5]
            if let Some(conj_form) = token.features.get(5) {
                conj_form == "未然形" || conj_form == "未然レル接続"
            } else {
                false
            }
        }
    }

    // Match causative suffix せる/させる in 未然形 (optional for short form)
    #[derive(Debug)]
    struct CausativeSuffixMatcher;
    impl Matcher for CausativeSuffixMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Must be a verb (接尾)
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                && (token.base_form == "せる" || token.base_form == "させる")
                && token.features.get(5).is_some_and(|form| form == "未然形")
        }
    }

    // Match passive suffix られる (or れる for short form)
    #[derive(Debug)]
    struct PassiveSuffixMatcher;
    impl Matcher for PassiveSuffixMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Must be a verb (接尾)
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                && (token.base_form == "られる" || token.base_form == "れる")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(MizenFormMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            CausativeSuffixMatcher,
        )))),
        TokenMatcher::Custom(Arc::new(PassiveSuffixMatcher)),
    ]
}

// Pattern: Verb[て]・Noun[で] + B (means/method/circumstances)
// Structures: Verb[て] + Phrase / Noun + で + Phrase
//
// This pattern highlights て/で when expressing means, method, or circumstances.
// Note: This overlaps significantly with "Verb + て", "Adjective + て・Noun + で", and "で" patterns.
// Given the overlap and low priority (1), we implement it to match either variant.
pub fn verb_te_u30fb_noun_de_b() -> Vec<TokenMatcher> {
    use super::concat;

    // Custom matcher for て/で particles expressing means/method
    #[derive(Debug)]
    struct TeDeMethodParticleMatcher;
    impl Matcher for TeDeMethodParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match て or で as conjunction particle (after verbs)
            let is_conjunction = (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞");

            // Match で as case particle (after nouns - means/method)
            let is_case_particle = token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞");

            is_conjunction || is_case_particle
        }
    }

    // Custom matcher that accepts either a verb OR a noun
    #[derive(Debug)]
    struct VerbOrNounMatcher;
    impl Matcher for VerbOrNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            let is_verb = token.pos.first().is_some_and(|pos| pos == "動詞")
                && (token.features.get(5).is_some_and(|form| form == "連用形")
                    || token.features.get(5).is_some_and(|form| form == "連用タ接続"));

            let is_noun = token.pos.first().is_some_and(|pos| pos == "名詞");

            is_verb || is_noun
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbOrNounMatcher)),
        TokenMatcher::Custom(Arc::new(TeDeMethodParticleMatcher)),
    ]
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

// Pattern: ているあいだに (while/during)
// Structures: Verb[ている] + 間（あいだ）に
pub fn teiruaidani() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match て or で particle
    #[derive(Debug)]
    struct TeDeParticleMatcher;
    impl super::Matcher for TeDeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
        }
    }

    // Match あいだ (interval/while)
    #[derive(Debug)]
    struct AidaMatcher;
    impl super::Matcher for AidaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "あいだ"
                && token.base_form == "あいだ"
                && token.pos.first().is_some_and(|p| p == "名詞")
        }
    }

    // Match に particle (case particle)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    super::concat(vec![
        vec![super::flexible_verb_form()],
        vec![TokenMatcher::Custom(Arc::new(TeDeParticleMatcher))],
        vec![TokenMatcher::specific_verb("いる")],
        vec![TokenMatcher::Custom(Arc::new(AidaMatcher))],
        vec![TokenMatcher::Custom(Arc::new(NiParticleMatcher))],
    ])
}

// Pattern: なくてもいい (don't have to / it's okay not to)
// Structures: Verb[なくて] + (も) + いい (+ です)
pub fn nakutemoii() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for なく (助動詞, 連用テ接続 form of ない)
    #[derive(Debug)]
    struct NakuAuxiliaryMatcher;
    impl Matcher for NakuAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "なく"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|form| form == "連用テ接続")
        }
    }

    // Matcher for て (助詞/接続助詞)
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl Matcher for TeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Matcher for も (助詞/係助詞) - optional
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Matcher for いい (形容詞/非自立)
    #[derive(Debug)]
    struct IiAdjectiveMatcher;
    impl Matcher for IiAdjectiveMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "いい"
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    vec![
        TokenMatcher::Any, // Verb in 未然形 (before なく)
        TokenMatcher::Custom(Arc::new(NakuAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(MoParticleMatcher)))),
        TokenMatcher::Custom(Arc::new(IiAdjectiveMatcher)),
    ]
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

// Pattern: てすみません (sorry for doing)
// Structures: Verb[て] + すみません
pub fn tesumimasen() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matches て or で as conjunction particle
    #[derive(Debug)]
    struct TeDeParticleMatcher;
    impl Matcher for TeDeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
        }
    }

    // Matches すみません as interjection
    #[derive(Debug)]
    struct SumimasenMatcher;
    impl Matcher for SumimasenMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "すみません"
                && token.base_form == "すみません"
                && token.pos.first().is_some_and(|p| p == "感動詞")
        }
    }

    // Matches です auxiliary (for でした)
    #[derive(Debug)]
    struct DesuAuxMatcher;
    impl Matcher for DesuAuxMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "です" && token.pos.first().is_some_and(|p| p == "助動詞")
        }
    }

    // Matches た auxiliary (for past tense)
    #[derive(Debug)]
    struct TaAuxMatcher;
    impl Matcher for TaAuxMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "た" && token.pos.first().is_some_and(|p| p == "助動詞")
        }
    }

    super::concat(vec![
        vec![super::flexible_verb_form()],
        vec![TokenMatcher::Custom(Arc::new(TeDeParticleMatcher))],
        vec![TokenMatcher::Custom(Arc::new(SumimasenMatcher))],
        vec![TokenMatcher::Optional(Box::new(TokenMatcher::Custom(
            Arc::new(DesuAuxMatcher),
        )))],
        vec![TokenMatcher::Optional(Box::new(TokenMatcher::Custom(
            Arc::new(TaAuxMatcher),
        )))],
    ])
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

// Pattern: てくれてありがとう (thank you for doing)
// Structures: Verb[て] + くれて + ありがとう(+ ございます)
pub fn tekuretearigatou() -> Vec<TokenMatcher> {
    use super::concat;

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

    // Match ありがとう as interjection
    #[derive(Debug)]
    struct ArigatouMatcher;
    impl Matcher for ArigatouMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ありがとう"
                && token.pos.first().is_some_and(|pos| pos == "感動詞")
        }
    }

    // Match ござい (from ござる auxiliary)
    #[derive(Debug)]
    struct GozaiMatcher;
    impl Matcher for GozaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "ござる"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Match ます auxiliary
    #[derive(Debug)]
    struct MasuMatcher;
    impl Matcher for MasuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Pattern: Verb[て] + くれ + て + ありがとう + (optional ござい + ます)
    // We need to match: くれ(動詞,連用形) + て + ありがとう + optional(ござい + ます)
    // But we want to include the leading verb too, so let's reuse てくれる structure

    // Match くれる verb in 連用形 (くれ)
    #[derive(Debug)]
    struct KureMatcher;
    impl Matcher for KureMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "くれる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形")
        }
    }

    // Match verb in 連用形 or 連用タ接続
    #[derive(Debug)]
    struct VerbRenyouMatcher;
    impl Matcher for VerbRenyouMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形" || f == "連用タ接続")
        }
    }

    // Pattern structure: Verb(連用形/連用タ接続) + て + くれ + て + ありがとう + optional(ござい + ます)
    concat(vec![
        vec![TokenMatcher::Custom(Arc::new(VerbRenyouMatcher))],
        vec![TokenMatcher::Custom(Arc::new(TeParticleMatcher))],
        vec![TokenMatcher::Custom(Arc::new(KureMatcher))],
        vec![TokenMatcher::Custom(Arc::new(TeParticleMatcher))],
        vec![TokenMatcher::Custom(Arc::new(ArigatouMatcher))],
        vec![TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(GozaiMatcher))))],
        vec![TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(MasuMatcher))))],
    ])
}

// Pattern: てくれない・てもらえない (won't you do for me?)
// Structures: Verb[て/ないで] + くれない(か)/くれません(か)/もらえない(か)/もらえません(か)
pub fn tekurenai_u30fb_temoraenai() -> Vec<TokenMatcher> {
    use super::concat;
    use std::sync::Arc;

    // Match verb in 連用形, 連用タ接続 (for て-form), or 未然形 (for ないで-form)
    #[derive(Debug)]
    struct VerbFormMatcher;
    impl super::Matcher for VerbFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            if token.pos.first().is_none_or(|pos| pos != "動詞") {
                false
            } else {
                let form = token.features.get(5);
                form.is_some_and(|f| f == "連用形" || f == "連用タ接続" || f == "未然形")
            }
        }
    }

    // Match て or で particle
    #[derive(Debug)]
    struct TeDeParticleMatcher;
    impl super::Matcher for TeDeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    // Match ない auxiliary (used in ないで construction)
    #[derive(Debug)]
    struct NaiAuxMatcher;
    impl super::Matcher for NaiAuxMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Match くれる or もらえる in 未然形 or 連用形
    #[derive(Debug)]
    struct KureruMoraeruMatcher;
    impl super::Matcher for KureruMoraeruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.base_form == "くれる" || token.base_form == "もらえる")
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    // Match ない (negative) or ませ (polite negative)
    #[derive(Debug)]
    struct NaiMaseMatcher;
    impl super::Matcher for NaiMaseMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "ない" && token.base_form == "ない" && token.pos.first().is_some_and(|pos| pos == "助動詞"))
            || (token.surface == "ませ" && token.base_form == "ます" && token.pos.first().is_some_and(|pos| pos == "助動詞"))
        }
    }

    // Match ん (polite negative contraction)
    #[derive(Debug)]
    struct NMatcher;
    impl super::Matcher for NMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ん"
                && token.base_form == "ん"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Pattern: Verb + (ない) + て/で + くれる/もらえる + ない/ません
    concat(vec![
        vec![TokenMatcher::Custom(Arc::new(VerbFormMatcher))],
        vec![TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NaiAuxMatcher))))],
        vec![TokenMatcher::Custom(Arc::new(TeDeParticleMatcher))],
        vec![TokenMatcher::Custom(Arc::new(KureruMoraeruMatcher))],
        vec![TokenMatcher::Custom(Arc::new(NaiMaseMatcher))],
        vec![TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NMatcher))))],
    ])
}

// Pattern: ～のだろうか
pub fn uff5e_nodarouka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: お～になる (honorific speech)
// Structures: (お/ご) + Noun + に + なる + (ます)
pub fn o_uff5e_ninaru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match お or ご prefix
    #[derive(Debug)]
    struct OGoPrefixMatcher;
    impl super::Matcher for OGoPrefixMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "お" || token.surface == "ご")
                && token.base_form == token.surface
                && token.pos.first().is_some_and(|pos| pos == "接頭詞")
                && token.pos.get(1).is_some_and(|pos| pos == "名詞接続")
        }
    }

    // Match に particle (case particle)
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

    // Match なる verb
    #[derive(Debug)]
    struct NaruMatcher;
    impl super::Matcher for NaruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "なる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    // Match ます
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
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(OGoPrefixMatcher)))),
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(NaruMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(MasuMatcher)))),
    ]
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

// Pattern: いらっしゃる (honorific - to be/come/go)
// Structures:
//   1. いらっしゃる standalone (replacing いる/くる/いく)
//   2. Verb[て] + いらっしゃる (as auxiliary verb)
pub fn irassharu() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match いらっしゃる verb
    #[derive(Debug)]
    struct IrassharuMatcher;
    impl super::Matcher for IrassharuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "いらっしゃる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "自立")
                    || token.pos.get(1).is_some_and(|pos| pos == "非自立"))
        }
    }

    // Match ます polite auxiliary (optional)
    #[derive(Debug)]
    struct MasuMatcher;
    impl super::Matcher for MasuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ます" && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Match た past auxiliary (optional)
    #[derive(Debug)]
    struct TaMatcher;
    impl super::Matcher for TaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "た" && token.base_form == "た"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Pattern: いらっしゃる + (Optional ます) + (Optional た)
    // Handles both standalone usage and as auxiliary verb after て-form
    vec![
        TokenMatcher::Custom(Arc::new(IrassharuMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(MasuMatcher)))),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(TaMatcher)))),
    ]
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
// Pattern: お〜する (humble speech - お/ご + Noun + する)
// Structures:
//   1. お/ご (接頭詞) + Noun[サ変接続] + する (split form like ご確認します)
//   2. Noun[サ変接続 starting with お/ご] + する (compound form like お守りします)
pub fn o_u301c_suru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match お or ご prefix
    #[derive(Debug)]
    struct OGoPrefixMatcher;
    impl super::Matcher for OGoPrefixMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "お" || token.surface == "ご")
                && token.pos.first().is_some_and(|pos| pos == "接頭詞")
                && token.pos.get(1).is_some_and(|pos| pos == "名詞接続")
        }
    }

    // Match ANY サ変接続 noun
    #[derive(Debug)]
    struct SahenNounMatcher;
    impl super::Matcher for SahenNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "サ変接続")
        }
    }

    // Match する verb (any conjugation)
    #[derive(Debug)]
    struct SuruMatcher;
    impl super::Matcher for SuruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "する"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    // Match ます polite auxiliary
    #[derive(Debug)]
    struct MasuMatcher;
    impl super::Matcher for MasuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ます" && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Pattern: お/ご (prefix) + Noun[サ変] + する + (optional ます)
    // Matches split form: ご + 確認 + します
    // Note: Does NOT match compound forms like お守りします (where お守り is a single token)
    // Priority is set low (1) to avoid over-matching plain サ変 verbs

    vec![
        TokenMatcher::Custom(Arc::new(OGoPrefixMatcher)),  // REQUIRED prefix
        TokenMatcher::Custom(Arc::new(SahenNounMatcher)),
        TokenMatcher::Custom(Arc::new(SuruMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(MasuMatcher)))),
    ]
}

// Pattern: いたす (humble speech - to do)
// Structures:
//   1. Noun[サ変接続] + いたす (する → いたす)
//   2. お + Verb[連用形] + いたす
//   3. ご + Noun[サ変接続] + いたす
pub fn itasu() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match サ変接続 nouns (can become する verbs)
    #[derive(Debug)]
    struct SahenNounMatcher;
    impl super::Matcher for SahenNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "サ変接続")
        }
    }

    // Match お or ご prefix
    #[derive(Debug)]
    struct OGoPrefixMatcher;
    impl super::Matcher for OGoPrefixMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "お" || token.surface == "ご")
                && token.pos.first().is_some_and(|pos| pos == "接頭詞")
                && token.pos.get(1).is_some_and(|pos| pos == "名詞接続")
        }
    }

    // Match verb in 連用形 (stem form)
    #[derive(Debug)]
    struct VerbStemMatcher;
    impl super::Matcher for VerbStemMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形")
        }
    }

    // Match いたす verb (non-independent: 非自立)
    #[derive(Debug)]
    struct ItasuMatcher;
    impl super::Matcher for ItasuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "いたす"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "非自立")
                    || token.pos.get(1).is_some_and(|pos| pos == "自立"))
        }
    }

    // Match ます polite auxiliary
    #[derive(Debug)]
    struct MasuMatcher;
    impl super::Matcher for MasuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ます" && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Pattern: (Optional お/ご) + (Verb[連用形] OR Noun[サ変接続]) + いたす + (Optional ます)
    vec![
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            OGoPrefixMatcher,
        )))),
        TokenMatcher::Custom(Arc::new(VerbStemOrSahenNounMatcher)),
        TokenMatcher::Custom(Arc::new(ItasuMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(MasuMatcher)))),
    ]
}

// Helper matcher: Verb[連用形] OR Noun[サ変接続]
#[derive(Debug)]
struct VerbStemOrSahenNounMatcher;
impl Matcher for VerbStemOrSahenNounMatcher {
    fn matches(&self, token: &crate::KagomeToken) -> bool {
        // Verb in 連用形
        let is_verb_stem = token.pos.first().is_some_and(|pos| pos == "動詞")
            && token.features.get(5).is_some_and(|f| f == "連用形");

        // Noun with サ変接続
        let is_sahen_noun = token.pos.first().is_some_and(|pos| pos == "名詞")
            && token.pos.get(1).is_some_and(|pos| pos == "サ変接続");

        is_verb_stem || is_sahen_noun
    }
}

// Pattern: ていただけませんか (could you please - humble polite request)
// Structures: Verb[て] + いただけませんか / Verb[て] + もらえませんか
//
// Note: Kagome tokenizes いただけませんか incorrectly as:
//   い(いる) + た + だけ + ませんか
// Instead of the correct:
//   いただけ(potential of いただく) + ませんか
//
// We match both the incorrect tokenization and the correct もらえませんか form
pub fn teitadakemasenka() -> Vec<TokenMatcher> {
    use super::Matcher;
    use std::sync::Arc;

    // Match either い (from いただけませんか) or もらえ (from もらえませんか)
    #[derive(Debug)]
    struct ItadakeMoraeMatcher;
    impl Matcher for ItadakeMoraeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match い (tokenized as いる verb)
            if token.surface == "い"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                && token.base_form == "いる"
            {
                return true;
            }
            // Match もらえる
            if token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                && token.base_form == "もらえる"
            {
                return true;
            }
            false
        }
    }

    // Match た from いただけませんか (optional for もらえませんか)
    #[derive(Debug)]
    struct TaAuxiliaryMatcher;
    impl Matcher for TaAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "た"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "た"
        }
    }

    // Match だけ particle (optional for もらえませんか)
    #[derive(Debug)]
    struct DakeParticleMatcher;
    impl Matcher for DakeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "だけ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞")
        }
    }

    // Match ませ (未然形 of ます)
    #[derive(Debug)]
    struct MaseAuxiliaryMatcher;
    impl Matcher for MaseAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ませ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "ます"
        }
    }

    // Match ん (negative auxiliary)
    #[derive(Debug)]
    struct NNegativeMatcher;
    impl Matcher for NNegativeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ん"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Match か (question particle)
    #[derive(Debug)]
    struct KaQuestionMatcher;
    impl Matcher for KaQuestionMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "か"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    // Match て or で particle
    #[derive(Debug)]
    struct TeDeParticleMatcher;
    impl Matcher for TeDeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Pattern: Verb[連用形/連用タ接続] + て/で + (い|もらえ) + [た] + [だけ] + ませ + ん + か
    // The た and だけ are only present in いただけませんか (mis-tokenized)
    vec![
        super::flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(TeDeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(ItadakeMoraeMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(TaAuxiliaryMatcher)))),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(DakeParticleMatcher)))),
        TokenMatcher::Custom(Arc::new(MaseAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(NNegativeMatcher)),
        TokenMatcher::Custom(Arc::new(KaQuestionMatcher)),
    ]
}

// Pattern: たら (conditional "if/when")
// Structures: Verb[た] + ら / い-Adj[た] + ら / な-Adj/Noun + だった + ら
pub fn tara() -> Vec<TokenMatcher> {
    use super::Matcher;
    use std::sync::Arc;

    // Match たら (仮定形 of た auxiliary)
    #[derive(Debug)]
    struct TaraAuxiliaryMatcher;
    impl Matcher for TaraAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "たら"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "た"
                && token.features.get(5).is_some_and(|f| f == "仮定形")
        }
    }

    // Match だっ (連用タ接続 of だ auxiliary) - for な-Adj and Nouns
    #[derive(Debug)]
    struct DattaMatcher;
    impl Matcher for DattaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "だっ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "だ"
                && token.features.get(5).is_some_and(|f| f == "連用タ接続")
        }
    }

    // Match verbs in 連用形 or 連用タ接続 (for verb + たら)
    #[derive(Debug)]
    struct VerbRenyouFormMatcher;
    impl Matcher for VerbRenyouFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形" || f == "連用タ接続")
        }
    }

    // Match い-adjectives in 連用タ接続 (for い-adj + たら)
    #[derive(Debug)]
    struct IAdjRenyouTaMatcher;
    impl Matcher for IAdjRenyouTaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "連用タ接続")
        }
    }

    // Match な-adjectives (名詞/形容動詞語幹) or nouns (for だったら pattern)
    #[derive(Debug)]
    struct NaAdjOrNounMatcher;
    impl Matcher for NaAdjOrNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    // たら can follow:
    // 1. Verb (連用形/連用タ接続) + たら
    // 2. い-Adj (連用タ接続) + たら
    // 3. な-Adj/Noun + だっ + たら
    vec![
        TokenMatcher::Any, // Verb, い-Adj, or な-Adj/Noun
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(DattaMatcher)))), // Optional だっ for pattern 3
        TokenMatcher::Custom(Arc::new(TaraAuxiliaryMatcher)),
    ]
}

// Pattern: ほかに(も)・ほか(に)は
pub fn hokani_mo_u30fb_hoka_ni_ha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: がひつよう (is necessary)
// Structure: が + ひつ + よう (+ だ/です optional)
pub fn gahitsuyou() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct GaParticleMatcher;
    impl super::Matcher for GaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "が"
                && token.base_form == "が"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    #[derive(Debug)]
    struct HitsuMatcher;
    impl super::Matcher for HitsuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ひつ"
                && token.base_form == "ひつ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    #[derive(Debug)]
    struct YouSuffixMatcher;
    impl super::Matcher for YouSuffixMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "よう"
                && token.base_form == "よう"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(GaParticleMatcher)),
        TokenMatcher::Custom(Arc::new(HitsuMatcher)),
        TokenMatcher::Custom(Arc::new(YouSuffixMatcher)),
    ]
}

// Pattern: そんなに
// Pattern: そんなに (that much/so much - demonstrative adverb)
// Structures: そんなに + Verb/Adjective
pub fn sonnani() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct SonnaniMatcher;
    impl super::Matcher for SonnaniMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "そんなに"
                && token.base_form == "そんなに"
                && token.pos.first().is_some_and(|p| p == "副詞")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(SonnaniMatcher))]
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

// Pattern: かどうか (whether or not)
// Structures: Verb/Adjective/Noun + か + どう + か
pub fn kadouka() -> Vec<TokenMatcher> {
    use super::Matcher;
    use std::sync::Arc;

    // Match か particle (副助詞)
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

    // Match どう adverb
    #[derive(Debug)]
    struct DouAdverbMatcher;
    impl Matcher for DouAdverbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "どう"
                && token.base_form == "どう"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
        }
    }

    vec![
        TokenMatcher::Any, // Preceding element (verb/adjective/noun)
        TokenMatcher::Custom(Arc::new(KaParticleMatcher)),
        TokenMatcher::Custom(Arc::new(DouAdverbMatcher)),
        TokenMatcher::Custom(Arc::new(KaParticleMatcher)),
    ]
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

// Pattern: だけでなく (not only)
// Structures: Verb/い-Adj/な-Adj/Noun + だけ + で/では/じゃ + なく(て)
pub fn dakedenaku() -> Vec<TokenMatcher> {
    use super::Matcher;
    use std::sync::Arc;

    // Matcher for だけ particle
    #[derive(Debug)]
    struct DakeMatcher;
    impl Matcher for DakeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "だけ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞")
        }
    }

    // Matcher for で (auxiliary verb だ in 連用形) or じゃ
    #[derive(Debug)]
    struct DeJaMatcher;
    impl Matcher for DeJaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match で (助動詞, base=だ, 連用形)
            (token.surface == "で"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞"))
            // Match じゃ (助詞/副助詞)
            || (token.surface == "じゃ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞"))
        }
    }

    // Matcher for は (optional, after で)
    #[derive(Debug)]
    struct WaMatcher;
    impl Matcher for WaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Matcher for なく (助動詞, base=ない)
    #[derive(Debug)]
    struct NakuMatcher;
    impl Matcher for NakuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "なく"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Matcher for て (助詞/接続助詞) - optional
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
        TokenMatcher::Any, // Any word (verb/adjective/noun)
        TokenMatcher::Custom(Arc::new(DakeMatcher)),
        TokenMatcher::Custom(Arc::new(DeJaMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(WaMatcher)))),
        TokenMatcher::Custom(Arc::new(NakuMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(TeMatcher)))),
    ]
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

// Pattern: かい (casual question particle)
// Structures: Word + (な) + (の) + かい
pub fn kai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matches な as 助動詞 (for noun/na-adjective)
    #[derive(Debug)]
    struct NaAuxiliaryMatcher;
    impl super::Matcher for NaAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "な"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Matches の as 名詞/非自立/一般 (nominalizer)
    #[derive(Debug)]
    struct NoNominalizerMatcher;
    impl super::Matcher for NoNominalizerMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Matches かい as 助詞/終助詞
    #[derive(Debug)]
    struct KaiParticleMatcher;
    impl super::Matcher for KaiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "かい"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "終助詞")
        }
    }

    vec![
        TokenMatcher::Any,
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NaAuxiliaryMatcher,
        )))),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NoNominalizerMatcher,
        )))),
        TokenMatcher::Custom(Arc::new(KaiParticleMatcher)),
    ]
}

// Pattern: もし (if/suppose - conditional emphasis)
// Structure: もし (as 副詞)
pub fn moshi() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matches もし as 副詞/一般
    #[derive(Debug)]
    struct MoshiMatcher;
    impl super::Matcher for MoshiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "もし"
                && token.base_form == "もし"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(MoshiMatcher))]
}

// Pattern: し～し 
pub fn shi_uff5e_shi() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: でできる・からできる
pub fn dedekiru_u30fb_karadekiru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ながら (while doing)
// Structures: Verb[stem] + ながら
pub fn nagara() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for verb in 連用形 (stem form)
    #[derive(Debug)]
    struct VerbStemMatcher;
    impl Matcher for VerbStemMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|form| form == "連用形")
        }
    }

    // Matcher for ながら as 助詞/接続助詞
    #[derive(Debug)]
    struct NagaraMatcher;
    impl Matcher for NagaraMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ながら"
                && token.base_form == "ながら"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbStemMatcher)),
        TokenMatcher::Custom(Arc::new(NagaraMatcher)),
    ]
}

// Pattern: たところだ (just did)
// Structures: Verb[た] + ところ (+ だ/です)
pub fn tatokoroda() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for ところ as 名詞/非自立/副詞可能
    #[derive(Debug)]
    struct TokoroMatcher;
    impl super::Matcher for TokoroMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ところ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Matcher for だ or です as auxiliary
    #[derive(Debug)]
    struct DaDesuMatcher;
    impl super::Matcher for DaDesuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "だ" || token.surface == "です")
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Pattern: Verb (連用形/連用タ接続) + た + ところ (+ optional だ/です)
    vec![
        super::flexible_verb_form(),
        super::past_auxiliary(),
        TokenMatcher::Custom(Arc::new(TokoroMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(DaDesuMatcher)))),
    ]
}

// Pattern: ているところだ (in the middle of doing)
// Structures: Verb[ている] + ところ + だ/です
pub fn teirutokoroda() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for ところ as 名詞/非自立/副詞可能
    #[derive(Debug)]
    struct TokoroMatcher;
    impl super::Matcher for TokoroMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ところ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Matcher for だ or です as auxiliary
    #[derive(Debug)]
    struct DaDesuMatcher;
    impl super::Matcher for DaDesuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "だ" || token.surface == "です")
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Pattern: Verb[ている] + ところ + だ/です
    // We need to match the ている pattern first, then ところ, then optional だ/です
    use super::concat;

    // Get the ている pattern from N5
    let teiru_pattern = crate::matchers::n5::teiru_u2460();

    concat(vec![
        teiru_pattern,
        vec![TokenMatcher::Custom(Arc::new(TokoroMatcher))],
        vec![TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(DaDesuMatcher))))],
    ])
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

// Pattern: Just finished doing (買ったばかり - just bought)
// Structures: Verb[た] + ばかり
fn bakari_particle() -> TokenMatcher {
    #[derive(Debug)]
    struct BakariParticleMatcher;
    impl Matcher for BakariParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ばかり"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞")
        }
    }
    TokenMatcher::Custom(Arc::new(BakariParticleMatcher))
}

pub fn tabakari() -> Vec<TokenMatcher> {
    vec![
        super::flexible_verb_form(),
        super::past_auxiliary(),
        bakari_particle(),
    ]
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
    // Match 風 as suffix (名詞/接尾/一般)
    // Note: Always pronounced ふう in this usage, not かぜ
    #[derive(Debug)]
    struct FuuSuffixMatcher;
    impl super::Matcher for FuuSuffixMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "風"
                && token.base_form == "風"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(FuuSuffixMatcher)),
    ]
}

// Pattern: がみられる (can be seen/observed)
// Structures: Noun + が/も + 見られる/見られます
pub fn gamirareru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match が or も particle
    #[derive(Debug)]
    struct GaMoParticleMatcher;
    impl Matcher for GaMoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "が" || token.surface == "も")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    // Match 見 verb in 未然形
    #[derive(Debug)]
    struct MiruVerbMatcher;
    impl Matcher for MiruVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "見"
                && token.base_form == "見る"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token
                    .features
                    .get(5)
                    .is_some_and(|form| form == "未然形")
        }
    }

    // Match られる/られ suffix
    #[derive(Debug)]
    struct RareruSuffixMatcher;
    impl Matcher for RareruSuffixMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "られる" || token.surface == "られ")
                && token.base_form == "られる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
        }
    }

    // Match ます auxiliary (optional for polite form)
    #[derive(Debug)]
    struct MasuAuxiliaryMatcher;
    impl Matcher for MasuAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ます"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        noun_matcher(),
        TokenMatcher::Custom(Arc::new(GaMoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(MiruVerbMatcher)),
        TokenMatcher::Custom(Arc::new(RareruSuffixMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            MasuAuxiliaryMatcher,
        )))),
    ]
}

// Pattern: にきがつく
pub fn nikigatsuku() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 〜でも 〜でも (whether...or, even if...or)
// Structures:
// - い-Adj(連用テ接続) + て + も + い-Adj(連用テ接続) + て + も
// - な-Adj/Noun + でも + な-Adj/Noun + でも
// - Noun + で + も + Noun + で + も (alternative tokenization)
pub fn u301c_demo_u301c_demo() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match first word: い-adjective, な-adjective, or noun
    #[derive(Debug)]
    struct FirstWordMatcher;
    impl super::Matcher for FirstWordMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // い-adjective in 連用テ接続 form
            if token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token
                    .features
                    .get(5)
                    .is_some_and(|f| f == "連用テ接続")
            {
                return true;
            }
            // な-adjective (形容動詞語幹) or regular noun
            token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    // Match て/で/でも particle
    // - て: 接続助詞 (for い-adjectives)
    // - で: 格助詞 (for nouns)
    // - でも: 副助詞 (for な-adj/noun, single token)
    #[derive(Debug)]
    struct TeDeOrDemoMatcher;
    impl super::Matcher for TeDeOrDemoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // て as 接続助詞
            if token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
            {
                return true;
            }
            // で as 格助詞
            if token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
            {
                return true;
            }
            // でも as single 副助詞 token
            token.surface == "でも"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "副助詞")
        }
    }

    // Match も particle
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl super::Matcher for MoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "係助詞")
        }
    }

    vec![
        // First instance: Word + (て/で/でも) + も
        TokenMatcher::Custom(Arc::new(FirstWordMatcher)),
        TokenMatcher::Custom(Arc::new(TeDeOrDemoMatcher)), // て, で, or でも
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(MoParticleMatcher)))), // も (optional for でも case)
        // Second instance: Word + (て/で/でも) + も
        TokenMatcher::Custom(Arc::new(FirstWordMatcher)),
        TokenMatcher::Custom(Arc::new(TeDeOrDemoMatcher)), // て, で, or でも
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(MoParticleMatcher)))), // も (optional for でも case)
    ]
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

// Pattern: Question-phrase + か (embedded question)
// Structures: Verb/Phrase + か + わかる/知る/決める/覚える etc.
// Examples: 来るか分かる (know if coming), 何で壊すか知る (know why destroying)
//
// Note: This か is the adverbial particle (副助詞), not the sentence-ending question marker.
// It marks embedded questions - uncertain things that are being inquired about.
pub fn question_phrase_ka() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match か as adverbial particle
    #[derive(Debug)]
    struct KaParticleMatcher;
    impl Matcher for KaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "か"
                && token.base_form == "か"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    // Match information-seeking verbs (わかる, 知る, 決める, 覚える, etc.)
    #[derive(Debug)]
    struct InfoVerbMatcher;
    impl Matcher for InfoVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && [
                    "分かる", "わかる", "判る", "解る", // understand, know
                    "知る",                           // know
                    "決める",                         // decide
                    "覚える",                         // remember, learn
                    "教える",                         // teach, tell
                    "確かめる",                       // confirm, verify
                    "調べる",                         // investigate, check
                    "聞く",                           // ask, hear
                    "考える",                         // think, consider
                    "見る",                           // see, look
                ]
                .contains(&token.base_form.as_str())
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(KaParticleMatcher)),
        TokenMatcher::Custom(Arc::new(InfoVerbMatcher)),
    ]
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

// Pattern: たらどう (why don't you / how about)
// Structures: Verb［たら］+ どう + (だ/か/です + か)
pub fn taradou() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match たら conditional auxiliary (仮定形)
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

    // Match どう adverb
    #[derive(Debug)]
    struct DouAdverbMatcher;
    impl Matcher for DouAdverbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "どう"
                && token.base_form == "どう"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
        }
    }

    // Match だ copula (optional)
    #[derive(Debug)]
    struct DaCopulaMatcher;
    impl Matcher for DaCopulaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "だ"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Match か question particle (optional)
    #[derive(Debug)]
    struct KaParticleMatcher;
    impl Matcher for KaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "か"
                && token.base_form == "か"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    // Match です polite auxiliary (optional)
    #[derive(Debug)]
    struct DesuMatcher;
    impl Matcher for DesuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "です"
                && token.base_form == "です"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        super::flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(TaraConditionalMatcher)),
        TokenMatcher::Custom(Arc::new(DouAdverbMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(DesuMatcher)))),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(KaParticleMatcher)))),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(DaCopulaMatcher)))),
    ]
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

// Pattern: ばよかった (should have / wish I had)
// Structures: Verb［ば］+ よかった + (です)
pub fn bayokatta() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ば conditional particle
    #[derive(Debug)]
    struct BaParticleMatcher;
    impl Matcher for BaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ば"
                && token.base_form == "ば"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Match よかっ (good, past form conjugation)
    #[derive(Debug)]
    struct YokattaMatcher;
    impl Matcher for YokattaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "よかっ"
                && token.base_form == "よい"
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "連用タ接続")
        }
    }

    // Match た past auxiliary
    #[derive(Debug)]
    struct TaPastMatcher;
    impl Matcher for TaPastMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "た"
                && token.base_form == "た"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Match です polite auxiliary (optional)
    #[derive(Debug)]
    struct DesuMatcher;
    impl Matcher for DesuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "です"
                && token.base_form == "です"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        TokenMatcher::verb_with_form("仮定形"),
        TokenMatcher::Custom(Arc::new(BaParticleMatcher)),
        TokenMatcher::Custom(Arc::new(YokattaMatcher)),
        TokenMatcher::Custom(Arc::new(TaPastMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(DesuMatcher)))),
    ]
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
