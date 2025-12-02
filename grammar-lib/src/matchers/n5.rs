use crate::pattern_matcher::TokenMatcher;
use std::sync::Arc;

#[allow(unused_imports)]
use super::{
    concat, deshi_form, flexible_verb_form, ii_form, ikenai_form, masen_form, mashi_form, optional,
    past_auxiliary, takatta_form_matcher, Matcher,
};

// ========== N5 Helper Matchers ==========

/// Match たい (desire form as adjective or auxiliary)
/// Used in: tai_form
pub fn tai_form_matcher() -> TokenMatcher {
    #[derive(Debug)]
    struct TaiFormMatcher;
    impl Matcher for TaiFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "たい"
                && (token.pos.first().is_some_and(|pos| pos == "形容詞")
                    || token.pos.first().is_some_and(|pos| pos == "助動詞"))
        }
    }
    TokenMatcher::Custom(Arc::new(TaiFormMatcher))
}

/// Match たく (desiderative stem from たい)
/// Used in: takunai_form
pub fn taku_form() -> TokenMatcher {
    #[derive(Debug)]
    struct TakuFormMatcher;
    impl Matcher for TakuFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "たく" && token.base_form == "たい"
        }
    }
    TokenMatcher::Custom(Arc::new(TakuFormMatcher))
}

/// Match て or で (te/de form particle)
/// Used in: te_form and other te-constructions
pub fn te_de_form() -> TokenMatcher {
    #[derive(Debug)]
    struct TeDeFormMatcher;
    impl Matcher for TeDeFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て" || token.surface == "で"
        }
    }
    TokenMatcher::Custom(Arc::new(TeDeFormMatcher))
}

/// Match なかっ (past negative form from ない)
/// Used in: short_past_negative
pub fn nakatta_form() -> TokenMatcher {
    #[derive(Debug)]
    struct NakattaFormMatcher;
    impl Matcher for NakattaFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "なかっ" && token.base_form == "ない"
        }
    }
    TokenMatcher::Custom(Arc::new(NakattaFormMatcher))
}

/// Match copula forms: です, だ, だった, でした
/// Used in: x_wa_y_desu
pub fn copula_matcher() -> TokenMatcher {
    #[derive(Debug)]
    struct CopulaMatcher;
    impl Matcher for CopulaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match です (auxiliary verb)
            if token.surface == "です" && token.pos.first().is_some_and(|pos| pos == "助動詞")
            {
                return true;
            }
            // Match だ (auxiliary verb or particle in some analyses)
            if token.surface == "だ" && token.base_form == "だ" {
                return true;
            }
            // Match だっ (past form stem of だ, followed by た)
            if token.surface == "だっ" && token.base_form == "だ" {
                return true;
            }
            false
        }
    }
    TokenMatcher::Custom(Arc::new(CopulaMatcher))
}

// ========== N5 Pattern Functions ==========

pub fn dictionary_form() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Verb {
        conjugation_form: Some("基本形"),
        base_form: None,
    }]
}

pub fn masu_ending() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("連用形"),
        TokenMatcher::Surface("ます"),
    ]
}

pub fn short_negative() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("未然形"),
        TokenMatcher::Surface("ない"),
    ]
}

pub fn polite_negative() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("連用形"),
        masen_form(),
        TokenMatcher::Surface("ん"),
    ]
}

pub fn ta_form() -> Vec<TokenMatcher> {
    vec![flexible_verb_form(), past_auxiliary()]
}

pub fn short_past_negative() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("未然形")],
        vec![nakatta_form()],
        vec![TokenMatcher::Surface("た")],
    ])
}

pub fn polite_past_ending() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("連用形"),
        mashi_form(),
        past_auxiliary(),
    ]
}

pub fn deshita() -> Vec<TokenMatcher> {
    vec![deshi_form(), past_auxiliary()]
}

pub fn tai_form() -> Vec<TokenMatcher> {
    vec![TokenMatcher::verb_with_form("連用形"), tai_form_matcher()]
}

pub fn takunai_form() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("連用形")],
        vec![taku_form()],
        vec![TokenMatcher::Surface("ない")],
    ])
}

pub fn takatta_form() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("連用形")],
        vec![takatta_form_matcher()],
        vec![TokenMatcher::Surface("た")],
    ])
}

pub fn te_form() -> Vec<TokenMatcher> {
    concat(vec![vec![flexible_verb_form()], vec![te_de_form()]])
}

pub fn te_iru() -> Vec<TokenMatcher> {
    concat(vec![te_form(), vec![TokenMatcher::specific_verb("いる")]])
}

pub fn te_kara() -> Vec<TokenMatcher> {
    concat(vec![te_form(), vec![TokenMatcher::Surface("から")]])
}

pub fn te_kudasai() -> Vec<TokenMatcher> {
    concat(vec![te_form(), vec![TokenMatcher::Surface("ください")]])
}

// ========== Permission & Prohibition Forms ==========

pub fn te_mo_ii() -> Vec<TokenMatcher> {
    concat(vec![
        te_form(),
        vec![TokenMatcher::Surface("も")],
        vec![ii_form()],
    ])
}

pub fn te_wa_ikenai() -> Vec<TokenMatcher> {
    concat(vec![
        te_form(),
        vec![TokenMatcher::Surface("は")],
        vec![ikenai_form()],
    ])
}

pub fn naide_kudasai() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("未然形"),
        TokenMatcher::Surface("ない"),
        TokenMatcher::Surface("で"),
        TokenMatcher::Surface("ください"),
    ]
}

// ========== Polite Questions & Suggestions ==========

pub fn polite_volitional() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    #[derive(Debug)]
    struct MashouFormMatcher;
    impl super::Matcher for MashouFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ましょう" || (token.surface == "ましょ" && token.base_form == "ます")
        }
    }
    concat(vec![
        vec![TokenMatcher::verb_with_form("連用形")],
        vec![TokenMatcher::Custom(Arc::new(MashouFormMatcher))],
    ])
}

pub fn masen_ka() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("連用形")],
        vec![masen_form()],
        vec![TokenMatcher::Surface("ん")],
        vec![TokenMatcher::Surface("か")],
    ])
}

pub fn mashou_ka() -> Vec<TokenMatcher> {
    concat(vec![
        polite_volitional(),
        vec![TokenMatcher::Surface("う")],
        vec![TokenMatcher::Surface("か")],
    ])
}

// ========== Experience & Modality ==========

pub fn ta_koto_ga_aru() -> Vec<TokenMatcher> {
    concat(vec![
        ta_form(),
        vec![TokenMatcher::Surface("こと")],
        vec![TokenMatcher::Surface("が")],
        vec![TokenMatcher::specific_verb("ある")],
    ])
}

pub fn sugiru() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    #[derive(Debug)]
    struct SugiruStemMatcher;
    impl super::Matcher for SugiruStemMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            if token.pos.first().is_some_and(|pos| pos == "動詞") {
                let form = token.features.get(5);
                form.is_some_and(|f| f == "連用形")
            } else if token.pos.first().is_some_and(|pos| pos == "形容詞") {
                let form = token.features.get(5);
                form.is_some_and(|f| f == "ガル接続")
            } else if token.pos.first().is_some_and(|pos| pos == "名詞") {
                token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹")
            } else {
                false
            }
        }
    }
    concat(vec![
        vec![TokenMatcher::Custom(Arc::new(SugiruStemMatcher))],
        vec![TokenMatcher::specific_verb("すぎる")],
    ])
}

pub fn tsumori_desu() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("基本形")],
        vec![TokenMatcher::Surface("つもり")],
        vec![TokenMatcher::Surface("です")],
    ])
}

pub fn hou_ga_ii() -> Vec<TokenMatcher> {
    concat(vec![
        ta_form(),
        vec![TokenMatcher::Surface("ほう")],
        vec![TokenMatcher::Surface("が")],
        vec![ii_form()],
    ])
}

pub fn deshou() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    #[derive(Debug)]
    struct DeshouFormMatcher;
    impl super::Matcher for DeshouFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "でしょう" || token.surface == "だろう")
                || (token.surface == "でしょ" && token.base_form == "です")
                || (token.surface == "だろ" && token.base_form == "だ")
        }
    }
    #[derive(Debug)]
    struct DeshouPrecedingMatcher;
    impl super::Matcher for DeshouPrecedingMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| {
                pos == "動詞" || pos == "形容詞" || pos == "名詞" || pos == "助動詞"
            })
        }
    }
    concat(vec![
        vec![TokenMatcher::Custom(Arc::new(DeshouPrecedingMatcher))],
        vec![TokenMatcher::Custom(Arc::new(DeshouFormMatcher))],
    ])
}

// ========== Reason/Explanation ==========

pub fn n_desu() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    #[derive(Debug)]
    struct NDesuFormMatcher;
    impl super::Matcher for NDesuFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "ん" || token.surface == "の")
                && token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }
    concat(vec![
        vec![TokenMatcher::Custom(Arc::new(NDesuFormMatcher))],
        vec![TokenMatcher::Surface("です")],
    ])
}

pub fn node_verb() -> Vec<TokenMatcher> {
    concat(vec![
        optional(te_form()),
        vec![TokenMatcher::verb_with_form("基本形")],
        vec![TokenMatcher::Surface("ので")],
    ])
}

pub fn node_adjective() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::Adjective { base_form: None }],
        vec![TokenMatcher::Surface("ので")],
    ])
}

pub fn node_nominal() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::Any],
        vec![TokenMatcher::Surface("な")],
        vec![TokenMatcher::Surface("ので")],
    ])
}

// ========== Time & Purpose ==========

pub fn ni_iku() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("連用形")],
        vec![TokenMatcher::Surface("に")],
        vec![TokenMatcher::specific_verb("行く")],
    ])
}

pub fn mae_ni() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("基本形")],
        vec![TokenMatcher::Surface("前")],
        vec![TokenMatcher::Surface("に")],
    ])
}

// ========== State Expression ==========

pub fn mada_te_imasen() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::Surface("まだ")],
        vec![TokenMatcher::Wildcard {
            min: 0,
            max: 5,
            stop_conditions: vec![],
        }],
        te_form(),
        vec![TokenMatcher::specific_verb("いる")],
        vec![masen_form()],
        vec![TokenMatcher::Surface("ん")],
    ])
}

// ========== Adjective Forms ==========

pub fn adjective() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Adjective { base_form: None }]
}

pub fn adjective_past() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Adjective { base_form: None },
        TokenMatcher::Surface("た"),
    ]
}

// ========== Particle Patterns ==========

// Noun + の + Noun: Possessive/attributive modifier (あいつの気持ち)
pub fn no_particle_modifier() -> Vec<TokenMatcher> {
    vec![
        super::noun_matcher(),
        TokenMatcher::Surface("の"),
        super::noun_matcher(),
    ]
}

// か + punctuation: Sentence-final question marker (何ですか？)
pub fn ka_particle_ending() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KaParticleMatcher;
    impl Matcher for KaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "か" && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }
    #[derive(Debug)]
    struct PunctuationMatcher;
    impl Matcher for PunctuationMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "記号")
        }
    }
    vec![
        TokenMatcher::Custom(Arc::new(KaParticleMatcher)),
        TokenMatcher::Custom(Arc::new(PunctuationMatcher)),
    ]
}

// ========== Copula Constructions ==========

// XはYです: Basic copula construction (私は学生です)
// Structures: Noun/Pronoun + は + Noun + です/だ/だった
pub fn x_wa_y_desu() -> Vec<TokenMatcher> {
    vec![
        super::noun_matcher(), // X (noun, pronoun, demonstrative - all are 名詞)
        TokenMatcher::Surface("は"),
        super::noun_matcher(), // Y (noun)
        copula_matcher(),      // です/だ/だっ
        TokenMatcher::Optional(Box::new(TokenMatcher::Surface("た"))), // た for だった
    ]
}
