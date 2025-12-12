use crate::pattern_matcher::TokenMatcher;
use crate::KagomeToken;
use std::sync::Arc;
use super::{Matcher, noun_matcher};

// ========== たい (Want to do) ==========

// Pattern: だ
pub fn da() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: です
pub fn desu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: は
pub fn ha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: も (also/too/even)
// Structures: Noun + (particle) + も
// Excludes question words (誰も, 何も are different grammar)
pub fn mo() -> Vec<TokenMatcher> {
    // Question words to exclude (these form different grammar patterns)
    const QUESTION_WORDS: &[&str] = &[
        "誰", "何", "どこ", "いつ", "どれ", "どちら", "どの", "なぜ", "なん",
    ];

    #[derive(Debug)]
    struct NonQuestionNounMatcher;
    impl Matcher for NonQuestionNounMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.pos.first().is_some_and(|p| p == "名詞")
                && !QUESTION_WORDS.contains(&token.base_form.as_str())
        }
    }

    #[derive(Debug)]
    struct CaseParticleMatcher;
    impl Matcher for CaseParticleMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "も"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "係助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NonQuestionNounMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            CaseParticleMatcher,
        )))),
        TokenMatcher::Custom(Arc::new(MoParticleMatcher)),
    ]
}

// Pattern: これ (this)
// Structures: これ (demonstrative pronoun for things near speaker)
pub fn kore() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KoreMatcher;
    impl Matcher for KoreMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "これ"
                && token.base_form == "これ"
                && token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "代名詞")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(KoreMatcher))]
}

// Pattern: それ (that)
// Structures: それ (demonstrative pronoun for things near listener)
pub fn sore() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct SoreMatcher;
    impl Matcher for SoreMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "それ"
                && token.base_form == "それ"
                && token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "代名詞")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(SoreMatcher))]
}

// Pattern: あれ (that over there)
// Structures: あれ (demonstrative pronoun for things away from both speaker and listener)
pub fn are() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct AreMatcher;
    impl Matcher for AreMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "あれ"
                && token.base_form == "あれ"
                && token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "代名詞")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(AreMatcher))]
}

// Pattern: の
pub fn no() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: いい
// Pattern: いい (good - irregular i-adjective, affirmative form only)
// Structures: いい
// Note: Other forms are handled by general i-adjective patterns:
//   - よくない → い-Adjectives くない
//   - よかった → い-Adjective (Past)
//   - よくなかった → い-Adjective くなかった
pub fn ii() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for いい (affirmative form)
    #[derive(Debug)]
    struct IiMatcher;
    impl Matcher for IiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "いい"
                && token.base_form == "いい"
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.features.get(4).is_some_and(|f| f == "形容詞・イイ")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(IiMatcher))]
}

// Pattern: い-Adjectives
pub fn i_adjectives() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: な-Adjectives
pub fn na_adjectives() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: か
// Pattern: か (or - presenting options/alternatives)
// Structures: Option A + か + Option B + か
// Options can be single words or phrases (e.g., "お母さんの靴")
pub fn ka() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KaParticleMatcher;
    impl Matcher for KaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "か"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞／並立助詞／終助詞")
        }
    }

    vec![
        TokenMatcher::Any, // Option A (single token for now)
        TokenMatcher::Custom(Arc::new(KaParticleMatcher)),
        TokenMatcher::Any, // Option B (single token for now)
        TokenMatcher::Custom(Arc::new(KaParticleMatcher)),
    ]
}

// Pattern: が
// Pattern: が (but/however - sentence connector)
// Structures: Verb/Adjective/Noun + (だ/です) + が
pub fn ga() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match が particle (conjunction)
    #[derive(Debug)]
    struct GaParticleMatcher;
    impl Matcher for GaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "が"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
        }
    }

    // Match verb, i-adjective, or na-adjective/noun + da/desu/masu
    #[derive(Debug)]
    struct VerbOrAdjectiveMatcher;
    impl Matcher for VerbOrAdjectiveMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Verb (any conjugation)
            if token.pos.first().is_some_and(|p| p == "動詞") {
                return true;
            }

            // い-Adjective
            if token.pos.first().is_some_and(|p| p == "形容詞") {
                return true;
            }

            // です, だ, or ます auxiliary verb
            if token.pos.first().is_some_and(|p| p == "助動詞")
                && (token.base_form == "です" || token.base_form == "だ" || token.base_form == "ます")
            {
                return true;
            }

            false
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbOrAdjectiveMatcher)),
        TokenMatcher::Custom(Arc::new(GaParticleMatcher)),
    ]
}

// Pattern: よ (sentence-ending particle for emphasis/new information)
// Structure: Sentence + よ
pub fn yo() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct YoParticleMatcher;
    impl Matcher for YoParticleMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "よ"
                && token.base_form == "よ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "終助詞")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(YoParticleMatcher))]
}

// Pattern: ね (seeking agreement/confirmation)
// Structures: Sentence + ね
pub fn ne() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct NeParticleMatcher;
    impl Matcher for NeParticleMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "ね"
                && token.base_form == "ね"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "終助詞")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(NeParticleMatcher))]
}

// Pattern: る-Verb (Dictionary)
pub fn ru_verb_dictionary() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: う-Verb (Dictionary)
pub fn u_verb_dictionary() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: を (object marker particle)
// Structures: Object + を
pub fn wo() -> Vec<TokenMatcher> {
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
        noun_matcher(),
        TokenMatcher::Custom(Arc::new(WoParticleMatcher)),
    ]
}

// Pattern: ます
pub fn masu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: る-Verb (Negative)
pub fn ru_verb_negative() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: う-Verb (Negative)
pub fn u_verb_negative() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ここ (this place / here)
// Structures: ここ (demonstrative pronoun)
pub fn koko() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KokoMatcher;
    impl Matcher for KokoMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "ここ"
                && token.base_form == "ここ"
                && token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "代名詞")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(KokoMatcher))]
}

// Pattern: そこ (that place / there)
// Structures: そこ (demonstrative pronoun)
pub fn soko() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct SokoMatcher;
    impl Matcher for SokoMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "そこ"
                && token.base_form == "そこ"
                && token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "代名詞")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(SokoMatcher))]
}

// Pattern: あそこ (that place over there)
// Structures: あそこ (demonstrative pronoun)
pub fn asoko() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct AsokoMatcher;
    impl Matcher for AsokoMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "あそこ"
                && token.base_form == "あそこ"
                && token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "代名詞")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(AsokoMatcher))]
}

// Pattern: で
pub fn de() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に
pub fn ni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: でしょう
// Pattern: でしょう (probably/right? - tentative/assumption)
// Structures: Noun/Verb/Adjective + でしょう
// Tokenization: でしょ (助動詞, base=です, 未然形) + う (助動詞, 不変化型)
pub fn deshou() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct DeshoMatcher;
    impl Matcher for DeshoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "でしょ"
                && token.pos.first().is_some_and(|p| p == "助動詞")
                && token.base_form == "です"
        }
    }

    #[derive(Debug)]
    struct UMatcher;
    impl Matcher for UMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "う"
                && token.pos.first().is_some_and(|p| p == "助動詞")
        }
    }

    vec![
        TokenMatcher::Any,  // Can follow noun, verb, or adjective
        TokenMatcher::Custom(Arc::new(DeshoMatcher)),
        TokenMatcher::Custom(Arc::new(UMatcher)),
    ]
}

// Pattern: だろう (probably/right? - casual tentative/assumption)
// Structures: Noun/Verb/Adjective + だろう (or contracted だろ)
// Tokenization:
//   - Full form: だろ (助動詞, base=だ, 未然形) + う (助動詞, 不変化型)
//   - Contracted: だろ (助動詞, base=だ, 未然形) alone
pub fn darou() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct DaroMatcher;
    impl Matcher for DaroMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "だろ"
                && token.pos.first().is_some_and(|p| p == "助動詞")
                && token.base_form == "だ"
        }
    }

    #[derive(Debug)]
    struct UMatcher;
    impl Matcher for UMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "う"
                && token.pos.first().is_some_and(|p| p == "助動詞")
        }
    }

    vec![
        TokenMatcher::Any,  // Can follow noun, verb, or adjective
        TokenMatcher::Custom(Arc::new(DaroMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(UMatcher)))),  // う is optional for contracted form
    ]
}

// Pattern: がある (there is/exists - for inanimate objects)
// Structures: Noun + が + ある/あります
pub fn gaaru() -> Vec<TokenMatcher> {
    use super::Matcher;
    use std::sync::Arc;

    // Match が particle (case particle)
    #[derive(Debug)]
    struct GaParticleMatcher;
    impl Matcher for GaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "が"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match ある verb (base_form = ある)
    #[derive(Debug)]
    struct AruVerbMatcher;
    impl Matcher for AruVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "ある"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(GaParticleMatcher)),
        TokenMatcher::Custom(Arc::new(AruVerbMatcher)),
    ]
}

// Pattern: がいる (there is/exists - for animate objects)
// Structures: Noun + が + いる/います
pub fn gairu() -> Vec<TokenMatcher> {
    use super::Matcher;
    use std::sync::Arc;

    // Match が particle (case particle) - reuse from gaaru
    #[derive(Debug)]
    struct GaParticleMatcher;
    impl Matcher for GaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "が"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match いる verb (base_form = いる)
    #[derive(Debug)]
    struct IruVerbMatcher;
    impl Matcher for IruVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "いる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(GaParticleMatcher)),
        TokenMatcher::Custom(Arc::new(IruVerbMatcher)),
    ]
}

// Pattern: この (this ~)
// Structures: この + Noun (pre-noun adjectival/demonstrative determiner)
pub fn kono() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KonoMatcher;
    impl Matcher for KonoMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "この"
                && token.base_form == "この"
                && token.pos.first().is_some_and(|p| p == "連体詞")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(KonoMatcher))]
}

// Pattern: その (that ~)
// Structures: その + Noun (pre-noun adjectival/demonstrative determiner)
pub fn sono() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct SonoMatcher;
    impl Matcher for SonoMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "その"
                && token.base_form == "その"
                && token.pos.first().is_some_and(|p| p == "連体詞")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(SonoMatcher))]
}

// Pattern: あの (that ~ over there)
// Structures: あの + Noun (pre-noun adjectival/demonstrative determiner)
pub fn ano() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct AnoMatcher;
    impl Matcher for AnoMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "あの"
                && token.base_form == "あの"
                && token.pos.first().is_some_and(|p| p == "連体詞")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(AnoMatcher))]
}

// Pattern: ～んです・のです (explanatory/emphasis)
// Structures:
//   Verb/い-Adj + ん(の) + だ/です
//   な-Adj/Noun + な + ん(の) + だ/です
pub fn uff5e_ndesu_u30fb_nodesu() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ん or の (名詞/非自立)
    #[derive(Debug)]
    struct NNoMatcher;
    impl Matcher for NNoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "ん" || token.surface == "の")
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
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Match な (助動詞, base=だ) - for な-Adj/Noun forms
    #[derive(Debug)]
    struct NaAuxMatcher;
    impl Matcher for NaAuxMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "な"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "だ"
        }
    }

    // Match any token EXCEPT な auxiliary (to avoid double matching)
    #[derive(Debug)]
    struct NonNaAuxMatcher;
    impl Matcher for NonNaAuxMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match any token that is NOT な(助動詞, base=だ)
            !(token.surface == "な"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "だ")
        }
    }

    // Return two alternatives:
    // 1. Non-な token + ん/の + だ/です (for verb/い-adj)
    // 2. Non-な token + な + ん/の + だ/です (for な-adj/noun)

    vec![
        TokenMatcher::Custom(Arc::new(NonNaAuxMatcher)),  // Preceding word (not な aux)
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NaAuxMatcher)))),  // Optional な for nouns/な-adj
        TokenMatcher::Custom(Arc::new(NNoMatcher)),  // ん or の
        TokenMatcher::Custom(Arc::new(DaDesuMatcher)),  // だ or です
    ]
}

// Pattern: い-Adjective (Past) - Adjective[い] + かった
// Structures: い-Adjective[連用タ接続] + た
pub fn i_adjective_past() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct IAdjKattaMatcher;
    impl Matcher for IAdjKattaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "連用タ接続")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(IAdjKattaMatcher)),
        super::past_auxiliary(),
    ]
}

// Pattern: い-Adjective + Noun (adjective modifying noun)
// Structures: い-Adjective + Noun
pub fn i_adjective_noun() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct IAdjMatcher;
    impl Matcher for IAdjMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "形容詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(IAdjMatcher)),
        super::noun_matcher(),
    ]
}

// Pattern: な-Adjective + Noun
pub fn na_adjective_noun() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: へいく
pub fn heiku() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: する
pub fn suru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: くる
pub fn kuru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: る-Verb (Past)
pub fn ru_verb_past() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: う-Verb (Past)
pub fn u_verb_past() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 好き (like, likable)
// Structures:
//   好き (な-adjective)
//   大好き (な-adjective - love)
pub fn suki() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match 好き or 大好き (な-adjective / 形容動詞語幹)
    #[derive(Debug)]
    struct SukiMatcher;
    impl Matcher for SukiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.base_form == "好き" || token.base_form == "大好き")
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(SukiMatcher))]
}

/// Pattern: きらい (dislike/hate)
/// Structures:
///   - Noun + が + 嫌い (predicate)
///   - 嫌い + な + Noun (adjectival)
pub fn kirai() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KiraiMatcher;
    impl Matcher for KiraiMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            (token.surface == "嫌い" || token.surface == "大嫌い")
                && token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "形容動詞語幹")
        }
    }

    #[derive(Debug)]
    struct GaParticleMatcher;
    impl Matcher for GaParticleMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "が"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    #[derive(Debug)]
    struct NaParticleMatcher;
    impl Matcher for NaParticleMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "な"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|p| p == "助動詞")
        }
    }

    vec![
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(GaParticleMatcher)))),
        TokenMatcher::Custom(Arc::new(KiraiMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NaParticleMatcher)))),
    ]
}

// Pattern: のがすき
pub fn nogasuki() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: がある + Noun
pub fn gaaru_noun() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: い-Adjectives くない (negative present)
// Structures:
//   い-Adjective[連用テ接続] + ない
//   い-Adjective[連用テ接続] + ない + です (semi-polite)
//   い-Adjective[連用テ接続] + ありません (polite - handled by くなかった pattern)
pub fn i_adjectives_kunai() -> Vec<TokenMatcher> {
    // Match i-adjective in 連用テ接続 form (く conjugation)
    #[derive(Debug)]
    struct IAdjKuFormMatcher;
    impl Matcher for IAdjKuFormMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "連用テ接続")
        }
    }

    // Match ない as auxiliary verb (negative)
    #[derive(Debug)]
    struct NaiAuxiliaryMatcher;
    impl Matcher for NaiAuxiliaryMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(IAdjKuFormMatcher)),
        TokenMatcher::Custom(Arc::new(NaiAuxiliaryMatcher)),
    ]
}

// Pattern: ので (because/since - objective reasoning)
// Structures:
//   Verb + ので
//   い-Adj + ので
//   な-Adj + な + ので
//   Noun + な + ので
pub fn node() -> Vec<TokenMatcher> {
    // Helper: Match verb, adjective, or noun
    #[derive(Debug)]
    struct VerbAdjNounMatcher;
    impl Matcher for VerbAdjNounMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.pos.first().is_some_and(|p| {
                p == "動詞" || p == "形容詞" || p == "名詞"
            })
        }
    }

    // Helper: Match な as auxiliary verb (だ in 体言接続 form)
    #[derive(Debug)]
    struct NaCopulaMatcher;
    impl Matcher for NaCopulaMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "な"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|p| p == "助動詞")
        }
    }

    // Helper: Match ので as conjunction particle
    #[derive(Debug)]
    struct NodeParticleMatcher;
    impl Matcher for NodeParticleMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "ので"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbAdjNounMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NaCopulaMatcher,
        )))),
        TokenMatcher::Custom(Arc::new(NodeParticleMatcher)),
    ]
}

// Pattern: から (from a starting point)
// Structures: Starting Point + から
pub fn kara() -> Vec<TokenMatcher> {
    use super::noun_matcher;

    #[derive(Debug)]
    struct KaraParticleMatcher;
    impl Matcher for KaraParticleMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "から"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    vec![
        noun_matcher(),
        TokenMatcher::Custom(Arc::new(KaraParticleMatcher)),
    ]
}

// Pattern: けど・だけど (but/however - casual)
// Structures: Verb/い-Adj + けど, な-Adj/Noun + だ + けど
pub fn kedo_u30fb_dakedo() -> Vec<TokenMatcher> {
    // Helper: Match verb, adjective, or noun
    #[derive(Debug)]
    struct VerbAdjNounMatcher;
    impl Matcher for VerbAdjNounMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.pos.first().is_some_and(|p| {
                p == "動詞" || p == "形容詞" || p == "名詞"
            })
        }
    }

    // Helper: Match だ as auxiliary verb
    #[derive(Debug)]
    struct DaCopulaMatcher;
    impl Matcher for DaCopulaMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "だ"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|p| p == "助動詞")
        }
    }

    // Helper: Match けど as conjunction particle
    #[derive(Debug)]
    struct KedoParticleMatcher;
    impl Matcher for KedoParticleMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "けど"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbAdjNounMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            DaCopulaMatcher,
        )))),
        TokenMatcher::Custom(Arc::new(KedoParticleMatcher)),
    ]
}

// Pattern: る-Verb (Negative-Past)
pub fn ru_verb_negative_past() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: う-Verb (Negative-Past)
pub fn u_verb_negative_past() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Verb + て (te-form for sequential actions)
// Structures: Verb[連用形/連用タ接続] + て/で (as conjunction particle)
pub fn verb_te() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct TeDeConjunctionMatcher;
    impl Matcher for TeDeConjunctionMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    vec![
        super::flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(TeDeConjunctionMatcher)),
    ]
}

// Pattern: ている① (progressive/resultative state)
// Structures:
//   Full: Verb[連用形/連用タ接続] + て/で + いる
//   Contracted: Verb[連用タ接続] + てる (single token, NO て particle)
pub fn teiru_u2460() -> Vec<TokenMatcher> {
    use super::concat;

    // Helper: Match て or で (connecting particle) OR てる/でる (contracted verb)
    // This handles both cases:
    // 1. て/で particle (followed by いる)
    // 2. てる/でる verb (which already includes the て/で sound)
    #[derive(Debug)]
    struct TeOrTeruMatcher;
    impl Matcher for TeOrTeruMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            // Case 1: て/で particle
            if (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
            {
                return true;
            }
            // Case 2: てる/でる verb (contracted form)
            if token.pos.first().is_some_and(|pos| pos == "動詞")
                && (token.base_form == "てる" || token.base_form == "でる")
            {
                return true;
            }
            false
        }
    }

    // Helper: Match いる as auxiliary verb (only for full form)
    // For contracted form (てる), this won't match, but that's okay because
    // てる is already matched by the previous matcher
    #[derive(Debug)]
    struct IruMatcher;
    impl Matcher for IruMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.base_form == "いる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    // This matches:
    // 1. Full form: Verb[連用形/連用タ接続] + て/で + いる
    // 2. Contracted: Verb[連用タ接続] + てる (the optional いる won't match, but that's OK)
    concat(vec![
        vec![super::flexible_verb_form()],
        vec![TokenMatcher::Custom(Arc::new(TeOrTeruMatcher))],
        vec![TokenMatcher::Optional(Box::new(TokenMatcher::Custom(
            Arc::new(IruMatcher),
        )))],
    ])
}

// Pattern: へ
pub fn he() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にいく (go to do something)
// Structures: Verb[stem] + に + 行く
// Note: Verb stems in 連用形 OR suru-verb nouns (like 釣り)
pub fn verb_niiku() -> Vec<TokenMatcher> {
    // Match verb stem (連用形) OR suru-verb noun
    #[derive(Debug)]
    struct VerbStemOrNounMatcher;
    impl Matcher for VerbStemOrNounMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            // Match verbs in 連用形 (stem form)
            if token.pos.first().is_some_and(|pos| pos == "動詞") {
                return token.features.get(5).is_some_and(|f| f == "連用形");
            }
            // Match nouns (including suru-verb nouns like 釣り)
            if token.pos.first().is_some_and(|pos| pos == "名詞") {
                return true;
            }
            false
        }
    }

    // Match に particle (格助詞)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    // Match 行く or いく (any conjugation form)
    #[derive(Debug)]
    struct IkuVerbMatcher;
    impl Matcher for IkuVerbMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && (token.base_form == "行く" || token.base_form == "いく")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbStemOrNounMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(IkuVerbMatcher)),
    ]
}

// Pattern: 誰
// Pattern: 誰 (who - question word for person)
pub fn dare() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct DareMatcher;
    impl Matcher for DareMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "誰"
                && token.base_form == "誰"
                && token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "代名詞")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(DareMatcher))]
}

// Pattern: い-Adjective (Predicate)
pub fn i_adjective_predicate() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: な-Adjective だ
pub fn na_adjective_da() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: だった・でした (was/were - past copula)
// Structures: Noun/な-Adj + だった / Noun/な-Adj + でした
pub fn datta_u30fb_deshita() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match だっ (auxiliary verb, 連用タ接続 form of だ)
    #[derive(Debug)]
    struct DattaMatcher;
    impl Matcher for DattaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "だっ"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token
                    .features
                    .get(5)
                    .is_some_and(|conj| conj == "連用タ接続")
        }
    }

    // Match でし (auxiliary verb, 連用形 of です)
    #[derive(Debug)]
    struct DeshitaMatcher;
    impl Matcher for DeshitaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "でし"
                && token.base_form == "です"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|conj| conj == "連用形")
        }
    }

    // Match た (past tense auxiliary)
    #[derive(Debug)]
    struct TaPastMatcher;
    impl Matcher for TaPastMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "た"
                && token.base_form == "た"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Match だっ or でし
    #[derive(Debug)]
    struct DattaOrDeshitaMatcher;
    impl Matcher for DattaOrDeshitaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            DattaMatcher.matches(token) || DeshitaMatcher.matches(token)
        }
    }

    // Pattern: な-Adjective stem (or noun) + だっ/でし + た
    vec![
        TokenMatcher::Any, // な-Adjective stem or Noun
        TokenMatcher::Custom(Arc::new(DattaOrDeshitaMatcher)),
        TokenMatcher::Custom(Arc::new(TaPastMatcher)),
    ]
}

// Pattern: じゃない (is not - negative copula)
// Structures:
//   Standard: Noun/な-Adj + じゃ + ない
//   Standard: Noun/な-Adj + で + は + ない (formal)
//   Polite: Noun/な-Adj + じゃ + あり + ませ + ん
//   Polite: Noun/な-Adj + で + は + あり + ませ + ん (formal)
//   Polite casual: Noun/な-Adj + じゃ + ない + です
pub fn janai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match じゃ (particle) or で (particle OR auxiliary verb base=だ)
    #[derive(Debug)]
    struct JaDeParticleMatcher;
    impl Matcher for JaDeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match じゃ as particle
            if token.surface == "じゃ" && token.pos.first().is_some_and(|p| p == "助詞") {
                return true;
            }
            // Match で as particle (after noun)
            if token.surface == "で" && token.pos.first().is_some_and(|p| p == "助詞") {
                return true;
            }
            // Match で as auxiliary verb (after な-adjective, base=だ)
            if token.surface == "で"
                && token.pos.first().is_some_and(|p| p == "助動詞")
                && token.base_form == "だ"
            {
                return true;
            }
            false
        }
    }

    // Match は particle (optional - only after で)
    #[derive(Debug)]
    struct WaTopicParticleMatcher;
    impl Matcher for WaTopicParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "係助詞")
        }
    }

    // Match ない (助動詞 or 形容詞) OR あり (auxiliary or verb)
    #[derive(Debug)]
    struct NaiOrAriMatcher;
    impl Matcher for NaiOrAriMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match ない (助動詞 or 形容詞) - for casual/semi-polite
            if token.surface == "ない"
                && (token.pos.first().is_some_and(|p| p == "助動詞")
                    || token.pos.first().is_some_and(|p| p == "形容詞"))
            {
                return true;
            }
            // Match あり - as auxiliary verb (after noun) OR verb (after な-adj)
            if token.surface == "あり" && token.base_form == "ある" {
                return true;
            }
            false
        }
    }

    // Match ませ (助動詞, base=ます) - only after あり
    #[derive(Debug)]
    struct MaseMatcher;
    impl Matcher for MaseMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ませ"
                && token.pos.first().is_some_and(|p| p == "助動詞")
                && token.base_form == "ます"
        }
    }

    // Match ん (助動詞) - only after ませ
    #[derive(Debug)]
    struct NNegationMatcher;
    impl Matcher for NNegationMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ん"
                && token.pos.first().is_some_and(|p| p == "助動詞")
        }
    }

    vec![
        TokenMatcher::Any, // Noun or な-Adjective stem
        TokenMatcher::Custom(Arc::new(JaDeParticleMatcher)), // じゃ or で
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(WaTopicParticleMatcher)))), // は (optional)
        TokenMatcher::Custom(Arc::new(NaiOrAriMatcher)), // ない OR あり
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(MaseMatcher)))), // ませ (optional, after あり)
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NNegationMatcher)))), // ん (optional, after ませ)
        // Auto-extension will handle です
    ]
}

// Pattern: じゃなかった (was not - negative past copula)
// Structures:
//   Standard: Noun/な-Adj + じゃ + なかっ + た
//   Standard: Noun/な-Adj + で + は + なかっ + た (formal)
//   Polite: Noun/な-Adj + じゃ + あり + ませ + ん + でし + た
//   Polite: Noun/な-Adj + で + は + あり + ませ + ん + でし + た (formal)
//   Polite casual: Noun/な-Adj + じゃ + なかっ + た + です
pub fn janakatta() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match じゃ (particle) or で (particle OR auxiliary verb base=だ)
    #[derive(Debug)]
    struct JaDeParticleMatcher;
    impl Matcher for JaDeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match じゃ as particle
            if token.surface == "じゃ" && token.pos.first().is_some_and(|p| p == "助詞") {
                return true;
            }
            // Match で as particle (after noun)
            if token.surface == "で" && token.pos.first().is_some_and(|p| p == "助詞") {
                return true;
            }
            // Match で as auxiliary verb (after な-adjective, base=だ)
            if token.surface == "で"
                && token.pos.first().is_some_and(|p| p == "助動詞")
                && token.base_form == "だ"
            {
                return true;
            }
            false
        }
    }

    // Match は particle (optional - only after で)
    #[derive(Debug)]
    struct WaTopicParticleMatcher;
    impl Matcher for WaTopicParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "係助詞")
        }
    }

    // Match なかっ (助動詞 or 形容詞) OR あり (auxiliary or verb)
    #[derive(Debug)]
    struct NakattaOrAriMatcher;
    impl Matcher for NakattaOrAriMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match なかっ - as 助動詞 (with じゃ) or 形容詞 (with では), base=ない
            if token.surface == "なかっ"
                && token.base_form == "ない"
                && (token.pos.first().is_some_and(|p| p == "助動詞")
                    || token.pos.first().is_some_and(|p| p == "形容詞"))
            {
                return true;
            }
            // Match あり - as auxiliary verb (after noun) OR verb (after な-adj)
            if token.surface == "あり" && token.base_form == "ある" {
                return true;
            }
            false
        }
    }

    // Match た (past auxiliary)
    #[derive(Debug)]
    struct TaMatcher;
    impl Matcher for TaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "た"
                && token.pos.first().is_some_and(|p| p == "助動詞")
                && token.base_form == "た"
        }
    }

    // Match ませ (助動詞, base=ます) - only after あり
    #[derive(Debug)]
    struct MaseMatcher;
    impl Matcher for MaseMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ませ"
                && token.pos.first().is_some_and(|p| p == "助動詞")
                && token.base_form == "ます"
        }
    }

    // Match ん (助動詞) - only after ませ
    #[derive(Debug)]
    struct NNegationMatcher;
    impl Matcher for NNegationMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ん"
                && token.pos.first().is_some_and(|p| p == "助動詞")
        }
    }

    // Match でし (助動詞, base=です) - only after ん
    #[derive(Debug)]
    struct DeshiMatcher;
    impl Matcher for DeshiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "でし"
                && token.pos.first().is_some_and(|p| p == "助動詞")
                && token.base_form == "です"
        }
    }

    vec![
        TokenMatcher::Any, // Noun or な-Adjective stem
        TokenMatcher::Custom(Arc::new(JaDeParticleMatcher)), // じゃ or で
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(WaTopicParticleMatcher)))), // は (optional)
        TokenMatcher::Custom(Arc::new(NakattaOrAriMatcher)), // なかっ OR あり
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(MaseMatcher)))), // ませ (optional, after あり)
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NNegationMatcher)))), // ん (optional, after ませ)
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(DeshiMatcher)))), // でし (optional, after ん)
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(TaMatcher)))), // た (optional if after なかっ/でし)
        // Auto-extension will handle です
    ]
}

// Pattern: い-Adjective くなかった - Adjective[く] + なかった
// Structures:
//   Standard: い-Adjective[連用テ接続] + なかっ + た
//   Polite: い-Adjective[連用テ接続] + あり + ませ + ん + でし + た
pub fn i_adjective_kunakatta() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct IAdjKuFormMatcher;
    impl Matcher for IAdjKuFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "連用テ接続")
        }
    }

    #[derive(Debug)]
    struct NakattaMatcher;
    impl Matcher for NakattaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "なかっ"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    #[derive(Debug)]
    struct AriMatcher;
    impl Matcher for AriMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "あり"
                && token.base_form == "ある"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    #[derive(Debug)]
    struct MaseMatcher;
    impl Matcher for MaseMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ませ"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    #[derive(Debug)]
    struct NMatcher;
    impl Matcher for NMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ん"
                && token.base_form == "ん"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    #[derive(Debug)]
    struct DeshiMatcher;
    impl Matcher for DeshiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "でし"
                && token.base_form == "です"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Combined matcher for both standard and polite forms
    #[derive(Debug)]
    struct KunakattaOrArimasenMatcher;
    impl Matcher for KunakattaOrArimasenMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match なかっ (standard form)
            (token.surface == "なかっ"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞"))
            // OR match あり (polite form start)
            || (token.surface == "あり"
                && token.base_form == "ある"
                && token.pos.first().is_some_and(|pos| pos == "動詞"))
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(IAdjKuFormMatcher)),
        TokenMatcher::Custom(Arc::new(KunakattaOrArimasenMatcher)),
        // The rest (た or ませんでした) will extend automatically
    ]
}

// Pattern: Verbs (Non-past)
pub fn verbs_non_past() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Verb［た・ている］+ Noun
pub fn verb_uff3b_ta_u30fb_teiru_uff3d_noun() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: な
pub fn na() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: だけ (only/just - limiting particle)
// Structures: Verb + だけ / い-Adj + だけ / な-Adj + な + だけ / Noun + だけ
pub fn dake() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match だけ particle (副助詞)
    #[derive(Debug)]
    struct DakeParticleMatcher;
    impl Matcher for DakeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "だけ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞")
        }
    }

    // Match な-adjective stem (名詞/形容動詞語幹)
    #[derive(Debug)]
    struct NaAdjectiveStemMatcher;
    impl Matcher for NaAdjectiveStemMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹")
        }
    }

    // Match な (auxiliary verb, 体言接続 form)
    #[derive(Debug)]
    struct NaAuxiliaryMatcher;
    impl Matcher for NaAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "な"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // We need to handle both:
    // 1. Single token + だけ (verb, い-adj, noun)
    // 2. な-adj stem + な + だけ (two tokens before だけ)
    //
    // Since we can't express "one OR two tokens before だけ" in a single pattern,
    // we'll use a more permissive approach: optionally match な-adj stem, then match
    // any token (which could be verb/adj/noun OR な), then だけ.
    vec![
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NaAdjectiveStemMatcher,
        )))),
        TokenMatcher::Any,
        TokenMatcher::Custom(Arc::new(DakeParticleMatcher)),
    ]
}

// Pattern: どれ
// Pattern: どれ (which - question word for things)
pub fn dore() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct DoreMatcher;
    impl Matcher for DoreMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "どれ"
                && token.base_form == "どれ"
                && token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "代名詞")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(DoreMatcher))]
}

// Pattern: どこ (where - question word for place)
pub fn doko() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct DokoMatcher;
    impl Matcher for DokoMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "どこ"
                && token.base_form == "どこ"
                && token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "代名詞")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(DokoMatcher))]
}

// Pattern: どの (which ~ - pre-noun adjectival)
pub fn dono() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct DonoMatcher;
    impl Matcher for DonoMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "どの"
                && token.base_form == "どの"
                && token.pos.first().is_some_and(|p| p == "連体詞")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(DonoMatcher))]
}

// Pattern: ている②
pub fn teiru_u2461() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// てから: After doing
// Structures: Verb[て] + から
pub fn tekara() -> Vec<TokenMatcher> {
    use super::Matcher;

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

    // Match から as case particle (not conjunction particle)
    // This から indicates "after", not "because"
    #[derive(Debug)]
    struct KaraAfterMatcher;
    impl Matcher for KaraAfterMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "から"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        super::flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(TeDeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(KaraAfterMatcher)),
    ]
}

// Pattern: Verb + て+ B
// Pattern: Verb + て+ B (sequential actions)
// Structures: Verb[て/で] + (optional particles/nouns/etc.) + Verb/Action
// Meaning: Sequential actions - "do X, then do Y"
pub fn verb_te_b() -> Vec<TokenMatcher> {
    use super::concat;

    // Helper: Match any verb (the second verb in the sequence)
    #[derive(Debug)]
    struct SecondVerbMatcher;
    impl Matcher for SecondVerbMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    // Match: Verb[連用形/連用タ接続] + て/で + (0-5 tokens) + Verb
    // The wildcard allows for particles, objects, etc. between the te-form and next verb
    concat(vec![
        vec![
            super::flexible_verb_form(),
            te_de_conjunction(),
        ],
        vec![TokenMatcher::Wildcard {
            min: 0,
            max: 5,
            stop_conditions: vec![],
        }],
        vec![TokenMatcher::Custom(Arc::new(SecondVerbMatcher))],
    ])
}

// Helper: Match て or で as conjunction particle
fn te_de_conjunction() -> TokenMatcher {
    #[derive(Debug)]
    struct TeDeConjunctionMatcher;
    impl Matcher for TeDeConjunctionMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }
    TokenMatcher::Custom(Arc::new(TeDeConjunctionMatcher))
}

// Pattern: もう
// Pattern: もう (already/anymore - adverb)
// Structures: もう + (Past) Phrase | もう + (Negative) Phrase
pub fn mou() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct MouMatcher;
    impl Matcher for MouMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "もう"
                && token.pos.first().is_some_and(|p| p == "副詞")
                && token.pos.get(1).is_some_and(|p| p == "一般")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(MouMatcher))]
}

// Pattern: まだ (still, not yet)
// Structures: まだ + Phrase (various combinations)
pub fn mada() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct MadaMatcher;
    impl Matcher for MadaMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "まだ"
                && token.base_form == "まだ"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "助詞類接続")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(MadaMatcher))]
}

// Pattern: まだ～ていません (still haven't done / haven't done yet)
// Structures: まだ + Verb[て] + いない/いません
pub fn mada_uff5e_teimasen() -> Vec<TokenMatcher> {
    use super::concat;

    // Match て or で particle
    #[derive(Debug)]
    struct TeDeParticleMatcher;
    impl Matcher for TeDeParticleMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Match いる (auxiliary verb)
    #[derive(Debug)]
    struct IruAuxMatcher;
    impl Matcher for IruAuxMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.base_form == "いる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    concat(vec![
        vec![TokenMatcher::Surface("まだ")],
        vec![TokenMatcher::Wildcard {
            min: 0,
            max: 5,
            stop_conditions: vec![],
        }],
        vec![super::flexible_verb_form()],
        vec![TokenMatcher::Custom(Arc::new(TeDeParticleMatcher))],
        vec![TokenMatcher::Custom(Arc::new(IruAuxMatcher))],
    ])
}

// Pattern: てもいい (it's okay even if - for adjectives and nouns)
// Structures:
//   - い-Adjective[く] + て + も + いい
//   - な-Adjective/Noun + でも + いい
// Note: This is different from "Verb + てもいい" which is a separate pattern
pub fn temoii() -> Vec<TokenMatcher> {
    // Match い-adjective in 連用テ接続 form (く form) OR Noun/な-Adjective
    #[derive(Debug)]
    struct IAdjKuOrNounMatcher;
    impl Matcher for IAdjKuOrNounMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            // Match い-adjective in く form
            (token.pos.first().is_some_and(|p| p == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "連用テ接続"))
                // OR match noun/な-adjective
                || (token.pos.first().is_some_and(|p| p == "名詞")
                    && (token.pos.get(1).is_some_and(|p| p == "一般")
                        || token.pos.get(1).is_some_and(|p| p == "形容動詞語幹")))
        }
    }

    // Match て (conjunction particle) OR でも (副助詞)
    #[derive(Debug)]
    struct TeOrDemoMatcher;
    impl Matcher for TeOrDemoMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            // Match て (接続助詞)
            (token.surface == "て"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞"))
                // OR match でも (副助詞)
                || (token.surface == "でも"
                    && token.pos.first().is_some_and(|p| p == "助詞")
                    && token.pos.get(1).is_some_and(|p| p == "副助詞"))
        }
    }

    // Match も (binding particle - 係助詞)
    // This is only present after て (not after でも)
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "も"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "係助詞")
        }
    }

    // Match いい (any form - can be adjective OR verb)
    #[derive(Debug)]
    struct IiMatcher;
    impl Matcher for IiMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "いい" || token.surface == "良い"
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(IAdjKuOrNounMatcher)),
        TokenMatcher::Custom(Arc::new(TeOrDemoMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            MoParticleMatcher,
        )))), // も is only present after て (not after でも)
        TokenMatcher::Custom(Arc::new(IiMatcher)),
    ]
}

// Pattern: たい (desire form)
// Structures: Verb[連用形] + たい/たく/たかっ/たくなかっ
pub fn tai() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct TaiFormMatcher;
    impl Matcher for TaiFormMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            // Match たい as the base form (covers たい, たく, たかった, etc.)
            token.base_form == "たい"
                && (token.pos.first().is_some_and(|pos| pos == "形容詞")
                    || token.pos.first().is_some_and(|pos| pos == "助動詞"))
        }
    }

    vec![
        TokenMatcher::Verb {
            conjugation_form: Some("連用形"),
            base_form: None,
        },
        TokenMatcher::Custom(Arc::new(TaiFormMatcher)),
    ]
}

// Pattern: たり～たりする
// Pattern: たり～たりする (doing things like A and B)
// Structures: Verb[た]り + (Verb[た]り) + する
// Matches from the FIRST たり to する (includes all たり in between)
pub fn tari_uff5e_tarisuru() -> Vec<TokenMatcher> {
    use super::concat;

    // Match たり or だり (parallel particle)
    #[derive(Debug)]
    struct TariDariMatcher;
    impl Matcher for TariDariMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            (token.surface == "たり" || token.surface == "だり")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "並立助詞")
        }
    }

    // Match する (any form)
    #[derive(Debug)]
    struct SuruMatcher;
    impl Matcher for SuruMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.base_form == "する"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    concat(vec![
        vec![super::flexible_verb_form()],
        vec![TokenMatcher::Custom(Arc::new(TariDariMatcher))],
        vec![TokenMatcher::Wildcard {
            min: 0,
            max: 15,
            stop_conditions: vec![],
        }],
        vec![TokenMatcher::Custom(Arc::new(SuruMatcher))],
    ])
}

// Pattern: けっこう (quite, fairly, pretty)
// Structures: けっこう + Phrase / けっこうだ/です
pub fn kekkou() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KekkouMatcher;
    impl Matcher for KekkouMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "けっこう"
                && token.base_form == "けっこう"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(KekkouMatcher))]
}

// Pattern: たくさん (a lot, many)
// Structures: たくさん + Phrase / たくさん + (の) + Noun
pub fn takusan() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct TakusanMatcher;
    impl Matcher for TakusanMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "たくさん"
                && token.base_form == "たくさん"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副詞可能")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(TakusanMatcher))]
}

// Pattern: まえに
pub fn maeni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

/// Pattern: くらい ① (about/approximately)
/// Structures: Number/counter + くらい or ぐらい
pub fn kurai_u2460() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct NumberMatcher;
    impl Matcher for NumberMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "数")
        }
    }

    #[derive(Debug)]
    struct CounterMatcher;
    impl Matcher for CounterMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "接尾")
                && token.pos.get(2).is_some_and(|p| p == "助数詞")
        }
    }

    #[derive(Debug)]
    struct QuestionWordMatcher;
    impl Matcher for QuestionWordMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "どの" && token.pos.first().is_some_and(|p| p == "連体詞")
        }
    }

    #[derive(Debug)]
    struct KuraiParticleMatcher;
    impl Matcher for KuraiParticleMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            (token.surface == "くらい" || token.surface == "ぐらい")
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "副助詞")
        }
    }

    #[derive(Debug)]
    struct CounterOrQuestionMatcher;
    impl Matcher for CounterOrQuestionMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            // Match counters (名詞/接尾/助数詞)
            if token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "接尾")
                && token.pos.get(2).is_some_and(|p| p == "助数詞")
            {
                return true;
            }
            // Match question words like どの (連体詞)
            if token.surface == "どの" && token.pos.first().is_some_and(|p| p == "連体詞") {
                return true;
            }
            false
        }
    }

    // Match: [Optional Number] + [Counter OR Question Word] + くらい
    vec![
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NumberMatcher)))),
        TokenMatcher::Custom(Arc::new(CounterOrQuestionMatcher)),
        TokenMatcher::Custom(Arc::new(KuraiParticleMatcher)),
    ]
}

// Pattern: まで (until/to - ending point)
// Structures: Noun + まで
pub fn noun_made() -> Vec<TokenMatcher> {
    use super::noun_matcher;

    #[derive(Debug)]
    struct MadeParticleMatcher;
    impl Matcher for MadeParticleMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
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

// Pattern: Verb + まで (until [verb] happens)
// Structures: Verb + まで
pub fn verb_made() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct VerbMatcher;
    impl Matcher for VerbMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.pos.first().is_some_and(|p| p == "動詞")
        }
    }

    #[derive(Debug)]
    struct MadeParticleMatcher;
    impl Matcher for MadeParticleMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "まで"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "副助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbMatcher)),
        TokenMatcher::Custom(Arc::new(MadeParticleMatcher)),
    ]
}

// Pattern: すぎる (too much, excessive)
// Structures: Verb[stem] + すぎる, い-Adj[stem] + すぎる, な-Adj + すぎる, なさすぎる
pub fn sugiru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match すぎる as verb
    #[derive(Debug)]
    struct SugiruMatcher;
    impl Matcher for SugiruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "すぎる" && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    // Match verb stem, i-adjective stem, na-adjective, or な (from ない)
    #[derive(Debug)]
    struct SugiruPrefixMatcher;
    impl Matcher for SugiruPrefixMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Verb in 連用形 (stem form)
            let is_verb_stem = token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形");

            // い-adjective stem (ガル接続 form)
            let is_iadj_stem = token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "ガル接続");

            // な-adjective (形容動詞語幹)
            let is_naadj = token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹");

            is_verb_stem || is_iadj_stem || is_naadj
        }
    }

    // Match さ suffix for なさすぎる
    #[derive(Debug)]
    struct SaSuffixMatcher;
    impl Matcher for SaSuffixMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "さ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(SugiruPrefixMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            SaSuffixMatcher,
        )))),
        TokenMatcher::Custom(Arc::new(SugiruMatcher)),
    ]
}

// Pattern: にする
pub fn nisuru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～になる・～くなる
pub fn uff5e_ninaru_u30fb_uff5e_kunaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: のなかで～がいちばん～
pub fn nonakade_uff5e_gaichiban_uff5e() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: より～のほうが
pub fn yori_uff5e_nohouga() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なにか・なにも
pub fn nanika_u30fb_nanimo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 誰か・どこか・誰も・どこも (someone/somewhere/no one/nowhere)
// Structures:
//   1. WH-Word + か: 誰か, どこか (someone, somewhere)
//   2. WH-Word + も: 誰も, どこも (no one, nowhere)
//   3. WH-Word + か + Particle: 誰かに, どこかへ, 誰かと
//   4. WH-Word + Particle + も: 誰にも, どこへも, 誰とも
//
// This matcher needs to handle all four patterns, which have different structures.
// We'll use a combined approach with optional tokens.
pub fn dareka_u30fb_dokoka_u30fb_daremo_u30fb_dokomo() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match question words: 誰, どこ
    #[derive(Debug)]
    struct QuestionWordDareDoko;
    impl Matcher for QuestionWordDareDoko {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "誰" || token.surface == "どこ")
                && token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "代名詞")
        }
    }

    // Match か OR case particle (へ, に, と)
    #[derive(Debug)]
    struct KaOrCaseParticle;
    impl Matcher for KaOrCaseParticle {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // か particle
            if token.surface == "か"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "副助詞／並立助詞／終助詞")
            {
                return true;
            }

            // Case particles: へ, に, と
            if (token.surface == "へ" || token.surface == "に" || token.surface == "と")
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
            {
                return true;
            }

            // も particle (for patterns like 誰も)
            if token.surface == "も"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "係助詞")
            {
                return true;
            }

            false
        }
    }

    // Match optional も or case particle
    #[derive(Debug)]
    struct OptionalMoOrParticle;
    impl Matcher for OptionalMoOrParticle {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // も particle
            if token.surface == "も"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "係助詞")
            {
                return true;
            }

            // Case particles for patterns like 誰かに
            if (token.surface == "へ" || token.surface == "に" || token.surface == "と")
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
            {
                return true;
            }

            false
        }
    }

    // Pattern structure: WH-Word + (か OR particle OR も) + optional (particle OR も)
    // This matches:
    // - 誰か (WH + か)
    // - 誰も (WH + も)
    // - 誰かに (WH + か + に)
    // - 誰にも (WH + に + も)
    vec![
        TokenMatcher::Custom(Arc::new(QuestionWordDareDoko)),
        TokenMatcher::Custom(Arc::new(KaOrCaseParticle)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            OptionalMoOrParticle,
        )))),
    ]
}

// Pattern: ましょう (let's do / volitional)
// Structures: Verb[連用形] + ましょ + う
pub fn mashou() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    use super::flexible_verb_form;

    // Match ましょ (助動詞, base=ます)
    #[derive(Debug)]
    struct MashoMatcher;
    impl Matcher for MashoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ましょ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "ます"
        }
    }

    // Match う (助動詞)
    #[derive(Debug)]
    struct UMatcher;
    impl Matcher for UMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "う"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        flexible_verb_form(),  // Verb in 連用形
        TokenMatcher::Custom(Arc::new(MashoMatcher)),  // ましょ
        TokenMatcher::Custom(Arc::new(UMatcher)),  // う
    ]
}

// Pattern: ～ましょうか (shall we?)
// Structures: Verb[連用形] + ましょ + う + か
pub fn uff5e_mashouka() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    use super::flexible_verb_form;

    // Match ましょ (助動詞, base=ます)
    #[derive(Debug)]
    struct MashoMatcher;
    impl Matcher for MashoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ましょ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "ます"
        }
    }

    // Match う (助動詞)
    #[derive(Debug)]
    struct UMatcher;
    impl Matcher for UMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "う"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Match か (助詞/終助詞)
    #[derive(Debug)]
    struct KaMatcher;
    impl Matcher for KaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "か"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    vec![
        flexible_verb_form(),  // Verb in 連用形
        TokenMatcher::Custom(Arc::new(MashoMatcher)),  // ましょ
        TokenMatcher::Custom(Arc::new(UMatcher)),  // う
        TokenMatcher::Custom(Arc::new(KaMatcher)),  // か
    ]
}

// Pattern: ませんか (won't you? - polite invitation)
// Structures: Verb[連用形] + ませ + ん + か
pub fn masenka() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    use super::flexible_verb_form;

    // Match ませ (助動詞, base=ます, 未然形)
    #[derive(Debug)]
    struct MaseMatcher;
    impl Matcher for MaseMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ませ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "ます"
        }
    }

    // Match ん (助動詞, base=ん)
    #[derive(Debug)]
    struct NMatcher;
    impl Matcher for NMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ん"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "ん"
        }
    }

    // Match か (助詞)
    #[derive(Debug)]
    struct KaMatcher;
    impl Matcher for KaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "か"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    vec![
        flexible_verb_form(),  // Verb in 連用形
        TokenMatcher::Custom(Arc::new(MaseMatcher)),  // ませ
        TokenMatcher::Custom(Arc::new(NMatcher)),  // ん
        TokenMatcher::Custom(Arc::new(KaMatcher)),  // か
    ]
}

// Pattern: てもいい (permission/it's okay to do)
// Structures: Verb[て] + も + いい
pub fn verb_temoii() -> Vec<TokenMatcher> {
    use super::{concat, flexible_verb_form, ii_form};

    // Match て or で (conjunction particle after verb)
    #[derive(Debug)]
    struct TeDeParticleMatcher;
    impl Matcher for TeDeParticleMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
        }
    }

    // Match も (係助詞 - binding particle)
    #[derive(Debug)]
    struct MoBindingParticleMatcher;
    impl Matcher for MoBindingParticleMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "も"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "係助詞")
        }
    }

    vec![
        flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(TeDeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(MoBindingParticleMatcher)),
        ii_form(),
    ]
}

// Pattern: てください (please do - polite request)
// Structures: Verb[て] + ください
pub fn tekudasai() -> Vec<TokenMatcher> {
    use super::{flexible_verb_form, Matcher};

    // Match て or で (conjunction particle after verb)
    #[derive(Debug)]
    struct TeDeParticleMatcher;
    impl Matcher for TeDeParticleMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
        }
    }

    // Match ください as non-independent verb
    #[derive(Debug)]
    struct KudasaiMatcher;
    impl Matcher for KudasaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ください"
                && token.base_form == "くださる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    vec![
        flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(TeDeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(KudasaiMatcher)),
    ]
}

// Pattern: ないでください (please don't do - polite negative request)
// Structures: Verb[未然形] + ない + で + ください
pub fn naidekudasai() -> Vec<TokenMatcher> {
    use super::Matcher;

    // Match verb in 未然形 (irrealis/negative form)
    #[derive(Debug)]
    struct NegativeVerbFormMatcher;
    impl Matcher for NegativeVerbFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token
                    .features
                    .get(5)
                    .is_some_and(|f| f == "未然形" || f == "未然ウ接続")
        }
    }

    // Match ない auxiliary verb in 連用デ接続 form
    #[derive(Debug)]
    struct NaiAuxiliaryDeFormMatcher;
    impl Matcher for NaiAuxiliaryDeFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "連用デ接続")
        }
    }

    // Match で (conjunction particle)
    #[derive(Debug)]
    struct DeParticleMatcher;
    impl Matcher for DeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "で"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
        }
    }

    // Match ください as non-independent verb
    #[derive(Debug)]
    struct KudasaiMatcher;
    impl Matcher for KudasaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ください"
                && token.base_form == "くださる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NegativeVerbFormMatcher)),
        TokenMatcher::Custom(Arc::new(NaiAuxiliaryDeFormMatcher)),
        TokenMatcher::Custom(Arc::new(DeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(KudasaiMatcher)),
    ]
}

// Pattern: てはいけない (must not do - prohibition)
// Structures:
//   Verb[て] + は + いけない (standard)
//   Verb + ちゃ + いけない (casual contraction of てはいけない)
//   Verb + じゃ + いけない (casual contraction of ではいけない)
pub fn tehaikenai() -> Vec<TokenMatcher> {
    use super::Matcher;

    // Match て or で (conjunction particle)
    #[derive(Debug)]
    struct TeDeParticleMatcher;
    impl Matcher for TeDeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
        }
    }

    // Match ちゃ or じゃ (casual contraction particles)
    #[derive(Debug)]
    struct ChyaJyaParticleMatcher;
    impl Matcher for ChyaJyaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "ちゃ" || token.surface == "じゃ")
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
        }
    }

    // Match は particle
    #[derive(Debug)]
    struct WaParticleMatcher;
    impl Matcher for WaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "係助詞")
        }
    }

    // Match いけ (non-independent verb in negative or polite form)
    // 未然形 for ない (いけない), 連用形 for ます (いけません)
    #[derive(Debug)]
    struct IkeMatcher;
    impl Matcher for IkeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "いけ"
                && token.base_form == "いける"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                && token
                    .features
                    .get(5)
                    .is_some_and(|f| f == "未然形" || f == "連用形")
        }
    }

    // Match ない auxiliary
    #[derive(Debug)]
    struct NaiAuxiliaryMatcher;
    impl Matcher for NaiAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Two variants: てはいけない and ちゃいけない/じゃいけない
    // We'll create a matcher that handles both
    #[derive(Debug)]
    struct TeWaIkenaiMatcher;
    impl Matcher for TeWaIkenaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match て/で + は or ちゃ/じゃ
            let te_de = (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞");

            let chya_jya = (token.surface == "ちゃ" || token.surface == "じゃ")
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞");

            te_de || chya_jya
        }
    }

    vec![
        super::flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(TeWaIkenaiMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(WaParticleMatcher)))),
        TokenMatcher::Custom(Arc::new(IkeMatcher)),
        // ない is optional because polite form uses ませ + ん instead (extension system handles this)
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NaiAuxiliaryMatcher)))),
    ]
}

// Pattern: なくてはいけない (must do)
// Structures:
//   Verb[未然形] + なく + て + は + いけない
//   Verb[未然形] + なく + ちゃ + いけない (casual)
pub fn nakutehaikenai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match verb in 未然形 (negative stem)
    #[derive(Debug)]
    struct NegativeVerbFormMatcher;
    impl Matcher for NegativeVerbFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然形")
        }
    }

    // Match なく (auxiliary verb, 連用テ接続 form of ない)
    #[derive(Debug)]
    struct NakuFormMatcher;
    impl Matcher for NakuFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "なく"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "ない"
                && token.features.get(5).is_some_and(|f| f == "連用テ接続")
        }
    }

    // Match て or ちゃ (conjunction particle)
    #[derive(Debug)]
    struct TeChaMatcher;
    impl Matcher for TeChaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "て" || token.surface == "ちゃ")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Match は (particle - optional for ちゃ)
    #[derive(Debug)]
    struct WaParticleMatcher;
    impl Matcher for WaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Match いけ (auxiliary verb of いける)
    // Can be 未然形 (before ない) or 連用形 (before ます)
    #[derive(Debug)]
    struct IkeMatcher;
    impl Matcher for IkeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "いけ"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                && token.base_form == "いける"
                && (token.features.get(5).is_some_and(|f| f == "未然形" || f == "連用形"))
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NegativeVerbFormMatcher)),
        TokenMatcher::Custom(Arc::new(NakuFormMatcher)),
        TokenMatcher::Custom(Arc::new(TeChaMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(WaParticleMatcher)))),
        TokenMatcher::Custom(Arc::new(IkeMatcher)),
        // ない extends automatically
    ]
}

// Pattern: なくてはならない
pub fn nakutehanaranai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// たほうがいい: Should do / It would be better to
// Structures: Verb[た] + 方 + が + いい
pub fn tahougaii() -> Vec<TokenMatcher> {
    use super::Matcher;

    // Match ほう as non-independent noun
    #[derive(Debug)]
    struct HouMatcher;
    impl Matcher for HouMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ほう"
                && token.base_form == "ほう"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Match が particle
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
        super::flexible_verb_form(),
        super::past_auxiliary(),
        TokenMatcher::Custom(Arc::new(HouMatcher)),
        TokenMatcher::Custom(Arc::new(GaParticleMatcher)),
        super::ii_form(),
    ]
}

// Pattern: ないほうがいい (should not do - negative advice)
// Structures: Verb[ない] + 方 + が + いい
pub fn naihougaii() -> Vec<TokenMatcher> {
    use super::Matcher;

    // Match verb in 未然形 (negative/irrealis form)
    #[derive(Debug)]
    struct NegativeVerbFormMatcher;
    impl Matcher for NegativeVerbFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token
                    .features
                    .get(5)
                    .is_some_and(|form| form == "未然形")
        }
    }

    // Match ない auxiliary verb
    #[derive(Debug)]
    struct NaiAuxiliaryMatcher;
    impl Matcher for NaiAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Match ほう as non-independent noun (reuse from たほうがいい)
    #[derive(Debug)]
    struct HouMatcher;
    impl Matcher for HouMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ほう"
                && token.base_form == "ほう"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Match が particle (reuse from たほうがいい)
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
        TokenMatcher::Custom(Arc::new(NegativeVerbFormMatcher)),
        TokenMatcher::Custom(Arc::new(NaiAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(HouMatcher)),
        TokenMatcher::Custom(Arc::new(GaParticleMatcher)),
        super::ii_form(),
    ]
}

// Pattern: なくちゃ・なきゃ
pub fn nakucha_u30fb_nakya() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: や (non-exhaustive listing - "things like")
// Structures: Noun + や + Noun
pub fn ya() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct YaParticleMatcher;
    impl Matcher for YaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "や"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "並立助詞")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(YaParticleMatcher)),
        super::noun_matcher(),
    ]
}

// Pattern: たことがある (have experience of)
// Structures: Verb[た] + こと + が + ある/ない
pub fn takotogaaru() -> Vec<TokenMatcher> {
    use super::concat;

    // Helper: Match ある or ない (affirmative or negative)
    #[derive(Debug)]
    struct AruNaiMatcher;
    impl Matcher for AruNaiMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            // Match ある as verb
            if token.base_form == "ある" && token.pos.first().is_some_and(|p| p == "動詞") {
                return true;
            }
            // Match ない as adjective (negative form of ある)
            if token.base_form == "ない" && token.pos.first().is_some_and(|p| p == "形容詞") {
                return true;
            }
            false
        }
    }

    concat(vec![
        vec![super::flexible_verb_form()],
        vec![super::past_auxiliary()],
        vec![TokenMatcher::Surface("こと")],
        vec![TokenMatcher::Surface("が")],
        vec![TokenMatcher::Custom(Arc::new(AruNaiMatcher))],
    ])
}

// Pattern: ている③
pub fn teiru_u2462() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Adjective + て・Noun + で (basic linking form)
// Structures: Same as Adjective + て + B, but specifically for linking qualities
// This reuses the same matcher implementation
pub fn adjective_te_u30fb_noun_de() -> Vec<TokenMatcher> {
    // This is the same grammatical structure as adjective_te_b(),
    // just used in a different context (linking qualities vs. linking to phrases)
    adjective_te_b()
}

// Pattern: Adjective + て + B (linking adjectives/nouns to phrases)
// Structures: い-Adj[くて] + Phrase, な-Adj[で] + Phrase, Noun[で] + Phrase
pub fn adjective_te_b() -> Vec<TokenMatcher> {
    // Combined matcher that matches both い-Adj+て and な-Adj/Noun+で patterns
    #[derive(Debug)]
    struct AdjectiveTeBFirstToken;
    impl Matcher for AdjectiveTeBFirstToken {
        fn matches(&self, token: &KagomeToken) -> bool {
            // Match い-Adjective in 連用テ接続 form (e.g., 大きく)
            let is_i_adj_te_form = token.pos.first().is_some_and(|p| p == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "連用テ接続");

            // Match な-Adjective (名詞/形容動詞語幹) e.g., 綺麗
            let is_na_adj = token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "形容動詞語幹");

            // Match regular Noun (but not 非自立) e.g., 学生, 医者
            let is_noun = token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p != "非自立")
                && token.pos.get(1).is_some_and(|p| p != "代名詞"); // Exclude pronouns

            is_i_adj_te_form || is_na_adj || is_noun
        }
    }

    // Matcher for て or で (the linking particle/copula)
    #[derive(Debug)]
    struct TeDeLinkerMatcher;
    impl Matcher for TeDeLinkerMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            // て particle (used with い-Adjectives)
            let is_te = token.surface == "て"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞");

            // で copula (used with な-Adjectives and Nouns)
            let is_de_copula = token.surface == "で"
                && token.pos.first().is_some_and(|p| p == "助動詞")
                && token.base_form == "だ";

            // で case particle (sometimes used with Nouns in linking)
            let is_de_particle = token.surface == "で"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞");

            is_te || is_de_copula || is_de_particle
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(AdjectiveTeBFirstToken)),
        TokenMatcher::Custom(Arc::new(TeDeLinkerMatcher)),
    ]
}

// Pattern: のがへた
pub fn nogaheta() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: のがじょうず
pub fn nogajouzu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Adjective + の(は)
// Pattern: Adjective + の(は) - "the one that [adjective]"
// Structures:
//   i-Adjective + の + は/が/も
//   な-Adjective + な + の + は/が/も
pub fn adjective_no_ha() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match い-adjective
    #[derive(Debug)]
    struct IAdjMatcher;
    impl Matcher for IAdjMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立")
        }
    }

    // Match な-adjective (noun that can take な)
    #[derive(Debug)]
    struct NaAdjMatcher;
    impl Matcher for NaAdjMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "名詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "サ変接続")
                    || token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹"))
        }
    }

    // Match な (copula in 体言接続 form)
    #[derive(Debug)]
    struct NaCopulaMatcher;
    impl Matcher for NaCopulaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "な"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Match の (nominalizer)
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

    // Match は/が/も particle
    #[derive(Debug)]
    struct WaGaMoParticleMatcher;
    impl Matcher for WaGaMoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "は" || token.surface == "が" || token.surface == "も")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    // Match adjective (either i-adj or na-adj+な)
    #[derive(Debug)]
    struct AdjectiveOrNaAdjectiveMatcher;
    impl Matcher for AdjectiveOrNaAdjectiveMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // i-adjective
            (token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立"))
            // OR na-adjective (noun that can take な)
            || (token.pos.first().is_some_and(|pos| pos == "名詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "サ変接続")
                    || token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹")))
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(AdjectiveOrNaAdjectiveMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NaCopulaMatcher)))),
        TokenMatcher::Custom(Arc::new(NoNominalizerMatcher)),
        TokenMatcher::Custom(Arc::new(WaGaMoParticleMatcher)),
    ]
}

// Pattern: あげる (to give)
// Structures: Object(Noun) + を + [optional recipient] + あげる/あげます
pub fn ageru() -> Vec<TokenMatcher> {
    use super::noun_matcher;
    use std::sync::Arc;

    #[derive(Debug)]
    struct WoParticleMatcher;
    impl Matcher for WoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "を"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    #[derive(Debug)]
    struct AgeruVerbMatcher;
    impl Matcher for AgeruVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "あげる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    vec![
        noun_matcher(),
        TokenMatcher::Custom(Arc::new(WoParticleMatcher)),
        TokenMatcher::Wildcard {
            min: 0,
            max: 3,
            stop_conditions: vec![],
        },
        TokenMatcher::Custom(Arc::new(AgeruVerbMatcher)),
    ]
}

// Pattern: くれる
pub fn kureru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: もらう
pub fn morau() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: けれども
// Pattern: けれども (but/although - formal)
// Structures: Sentence + けれども
pub fn keredomo() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KeredomoMatcher;
    impl Matcher for KeredomoMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "けれども"
                && token.base_form == "けれども"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(KeredomoMatcher))]
}

// Pattern: つもりだ (intend to/plan to)
// Structures:
//   Verb[基本形] + つもり + だ/です
//   Verb[未然形] + ない + つもり + だ/です
pub fn tsumorida() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match つもり as non-independent noun
    #[derive(Debug)]
    struct TsumoriMatcher;
    impl Matcher for TsumoriMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "つもり"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Match verb in any form (including 未然形 before ない)
    #[derive(Debug)]
    struct VerbMatcher;
    impl Matcher for VerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    // Match ない auxiliary (optional, for negative intention)
    #[derive(Debug)]
    struct NaiAuxMatcher;
    impl Matcher for NaiAuxMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NaiAuxMatcher)))),
        TokenMatcher::Custom(Arc::new(TsumoriMatcher)),
        // だ/です extends automatically via auxiliary verb system
    ]
}

// Pattern: ～になる・～くなる (become)
// Structures:
//   い-Adjective[く] + なる
//   な-Adjective/Noun + に + なる
pub fn ni_naru_ku_naru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match い-adjective in く form (連用テ接続)
    #[derive(Debug)]
    struct IAdjKuFormMatcher;
    impl Matcher for IAdjKuFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "連用テ接続")
        }
    }

    // Match noun or な-adjective (形容動詞語幹)
    #[derive(Debug)]
    struct NounOrNaAdjMatcher;
    impl Matcher for NounOrNaAdjMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "名詞")
                && (token.pos.get(1).is_some_and(|p| p == "形容動詞語幹")
                    || token.pos.get(1).is_some_and(|p| p == "一般")
                    || token.pos.get(1).is_some_and(|p| p == "サ変接続")
                    || token.pos.get(1).is_some_and(|p| p == "固有名詞"))
        }
    }

    // Match に particle (格助詞)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match なる verb (any conjugation)
    #[derive(Debug)]
    struct NaruVerbMatcher;
    impl Matcher for NaruVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "なる" && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    // Two patterns in one:
    // 1. い-Adj[く] + なる
    // 2. Noun/な-Adj + に + なる
    // We'll use Custom matcher that tries both patterns
    #[derive(Debug)]
    struct KuNaruOrNiNaruMatcher;
    impl Matcher for KuNaruOrNiNaruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match い-adj in く form OR noun/な-adj
            (token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "連用テ接続"))
                || (token.pos.first().is_some_and(|pos| pos == "名詞")
                    && (token.pos.get(1).is_some_and(|p| p == "形容動詞語幹")
                        || token.pos.get(1).is_some_and(|p| p == "一般")
                        || token.pos.get(1).is_some_and(|p| p == "サ変接続")
                        || token.pos.get(1).is_some_and(|p| p == "固有名詞")))
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(KuNaruOrNiNaruMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NiParticleMatcher,
        )))),
        TokenMatcher::Custom(Arc::new(NaruVerbMatcher)),
    ]
}
