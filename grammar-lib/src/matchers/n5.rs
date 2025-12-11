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

// Pattern: これ
pub fn kore() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: それ
pub fn sore() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: あれ
pub fn are() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
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

// Pattern: ここ
pub fn koko() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: そこ
pub fn soko() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: あそこ
pub fn asoko() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
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

// Pattern: この
pub fn kono() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: その
pub fn sono() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: あの
pub fn ano() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
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

// Pattern: けど・だけど
pub fn kedo_u30fb_dakedo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
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

// Pattern: Verb + にいく
pub fn verb_niiku() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 誰
pub fn dare() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
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
pub fn dore() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: どこ
pub fn doko() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: どの
pub fn dono() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
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

// Pattern: Verb + まで
pub fn verb_made() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
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

// Pattern: Verb + てもいい
pub fn verb_temoii() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
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

// Pattern: Adjective + て・Noun + で
pub fn adjective_te_u30fb_noun_de() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Adjective + て + B
pub fn adjective_te_b() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
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
pub fn adjective_no_ha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
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
