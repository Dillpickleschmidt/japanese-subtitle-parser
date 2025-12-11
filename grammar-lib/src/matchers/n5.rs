use crate::pattern_matcher::TokenMatcher;
use crate::KagomeToken;
use std::sync::Arc;
use super::Matcher;

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
pub fn ii() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
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
pub fn ka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: が
pub fn ga() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: よ
pub fn yo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ね
pub fn ne() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: る-Verb (Dictionary)
pub fn ru_verb_dictionary() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: う-Verb (Dictionary)
pub fn u_verb_dictionary() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: を
pub fn wo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
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
pub fn deshou() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: だろう
pub fn darou() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: がある
pub fn gaaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: がいる
pub fn gairu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
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

// Pattern: ～んです・のです
pub fn uff5e_ndesu_u30fb_nodesu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: い-Adjective (Past)
pub fn i_adjective_past() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: い-Adjective + Noun
pub fn i_adjective_noun() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
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

// Pattern: 好き
pub fn suki() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: きらい
pub fn kirai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: のがすき
pub fn nogasuki() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: がある + Noun
pub fn gaaru_noun() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: い-Adjectives くない
pub fn i_adjectives_kunai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
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

// Pattern: Verb + て
pub fn verb_te() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
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

// Pattern: だった・でした
pub fn datta_u30fb_deshita() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: じゃない
pub fn janai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: じゃなかった
pub fn janakatta() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: い-Adjective くなかった
pub fn i_adjective_kunakatta() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
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

// Pattern: だけ
pub fn dake() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
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

// Pattern: てから
pub fn tekara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Verb + て+ B
pub fn verb_te_b() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: もう
pub fn mou() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: まだ
pub fn mada() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: まだ～ていません
pub fn mada_uff5e_teimasen() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: てもいい
pub fn temoii() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
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
pub fn tari_uff5e_tarisuru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: けっこう
pub fn kekkou() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: たくさん
pub fn takusan() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: まえに
pub fn maeni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: くらい ①
pub fn kurai_u2460() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
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

// Pattern: すぎる
pub fn sugiru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
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

// Pattern: 誰か・どこか・誰も・どこも
pub fn dareka_u30fb_dokoka_u30fb_daremo_u30fb_dokomo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ましょう
pub fn mashou() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～ましょうか
pub fn uff5e_mashouka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ませんか
pub fn masenka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
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

// Pattern: てください
pub fn tekudasai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ないでください
pub fn naidekudasai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: てはいけない
pub fn tehaikenai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なくてはいけない
pub fn nakutehaikenai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なくてはならない
pub fn nakutehanaranai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: たほうがいい
pub fn tahougaii() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ないほうがいい
pub fn naihougaii() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なくちゃ・なきゃ
pub fn nakucha_u30fb_nakya() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: や
pub fn ya() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
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

// Pattern: あげる
pub fn ageru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
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
pub fn keredomo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: つもりだ
pub fn tsumorida() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}
