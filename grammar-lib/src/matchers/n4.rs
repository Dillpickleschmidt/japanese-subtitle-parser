use crate::pattern_matcher::TokenMatcher;

#[allow(unused_imports)]
use super::concat;

// ========== N4 Pattern Functions ==========

// ========== Complex Te-forms ==========

pub fn te_miru() -> Vec<TokenMatcher> {
    concat(vec![
        super::n5::te_form(),
        vec![TokenMatcher::specific_verb("みる")],
    ])
}

pub fn te_shimau() -> Vec<TokenMatcher> {
    concat(vec![
        super::n5::te_form(),
        vec![TokenMatcher::specific_verb("しまう")],
    ])
}

pub fn te_aru() -> Vec<TokenMatcher> {
    concat(vec![
        super::n5::te_form(),
        vec![TokenMatcher::specific_verb("ある")],
    ])
}

pub fn te_kureru() -> Vec<TokenMatcher> {
    concat(vec![
        super::n5::te_form(),
        vec![TokenMatcher::specific_verb("くれる")],
    ])
}

pub fn te_kudasaru() -> Vec<TokenMatcher> {
    concat(vec![
        super::n5::te_form(),
        vec![TokenMatcher::specific_verb_with_form("くださる", "連用形")],
    ])
}

pub fn te_ageru() -> Vec<TokenMatcher> {
    concat(vec![
        super::n5::te_form(),
        vec![TokenMatcher::specific_verb("あげる")],
    ])
}

pub fn te_oku() -> Vec<TokenMatcher> {
    concat(vec![
        super::n5::te_form(),
        vec![TokenMatcher::specific_verb("おく")],
    ])
}

pub fn te_morau() -> Vec<TokenMatcher> {
    concat(vec![
        super::n5::te_form(),
        vec![TokenMatcher::specific_verb("もらう")],
    ])
}

// ========== TeMo & Te-form Expressions ==========

pub fn te_mo() -> Vec<TokenMatcher> {
    concat(vec![
        super::n5::te_form(),
        vec![TokenMatcher::Surface("も")],
    ])
}

pub fn te_sumimasen() -> Vec<TokenMatcher> {
    concat(vec![
        super::n5::te_form(),
        vec![TokenMatcher::Surface("すみません")],
    ])
}

pub fn te_kurete_arigatou() -> Vec<TokenMatcher> {
    concat(vec![
        super::n5::te_form(),
        vec![TokenMatcher::specific_verb("くれる")],
        vec![super::n5::te_de_form()],
        vec![TokenMatcher::Surface("ありがとう")],
    ])
}

pub fn te_yokatta() -> Vec<TokenMatcher> {
    concat(vec![super::n5::te_form(), vec![yokatta_form()]])
}

pub fn te_itadakemasen_ka() -> Vec<TokenMatcher> {
    concat(vec![
        super::n5::te_form(),
        vec![TokenMatcher::specific_verb("いただく")],
    ])
}

// ========== Tari Form ==========

/// Match たり or だり (parallel/coordinating particle)
fn tari_particle() -> TokenMatcher {
    use std::sync::Arc;
    #[derive(Debug)]
    struct TariParticleMatcher;
    impl super::Matcher for TariParticleMatcher {
        fn matches(&self, token: &kagome_client::KagomeToken) -> bool {
            (token.surface == "たり" || token.surface == "だり")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "並立助詞")
        }
    }
    TokenMatcher::Custom(Arc::new(TariParticleMatcher))
}

pub fn tari_suru_single() -> Vec<TokenMatcher> {
    concat(vec![
        vec![super::flexible_verb_form()],
        vec![tari_particle()],
        vec![TokenMatcher::specific_verb("する")],
    ])
}

pub fn tari_suru() -> Vec<TokenMatcher> {
    concat(vec![
        vec![super::flexible_verb_form()],
        vec![tari_particle()],
        vec![TokenMatcher::Wildcard {
            min: 0,
            max: 15,
            stop_conditions: vec![],
        }],
        vec![tari_particle()],
        vec![TokenMatcher::specific_verb("する")],
    ])
}

// ========== Conditional Forms ==========

/// Match たら or だら (conditional form)
fn tara_form() -> TokenMatcher {
    use std::sync::Arc;
    #[derive(Debug)]
    struct TaraFormMatcher;
    impl super::Matcher for TaraFormMatcher {
        fn matches(&self, token: &kagome_client::KagomeToken) -> bool {
            (token.surface == "たら" || token.surface == "だら")
                && (token.base_form == "た" || token.base_form == "だ")
        }
    }
    TokenMatcher::Custom(Arc::new(TaraFormMatcher))
}

/// Match よかっ or 良かっ (past adjective stem from よい/いい/良い)
pub fn yokatta_form() -> TokenMatcher {
    use std::sync::Arc;
    #[derive(Debug)]
    struct YokattaFormMatcher;
    impl super::Matcher for YokattaFormMatcher {
        fn matches(&self, token: &kagome_client::KagomeToken) -> bool {
            (token.surface == "よかっ" || token.surface == "良かっ")
                && (token.base_form == "よい"
                    || token.base_form == "良い"
                    || token.base_form == "いい")
        }
    }
    TokenMatcher::Custom(Arc::new(YokattaFormMatcher))
}

pub fn ba_conditional() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("仮定形"),
        TokenMatcher::Surface("ば"),
    ]
}

pub fn tara_conditional() -> Vec<TokenMatcher> {
    concat(vec![vec![super::flexible_verb_form()], vec![tara_form()]])
}

pub fn tara_dou() -> Vec<TokenMatcher> {
    concat(vec![
        vec![super::flexible_verb_form()],
        vec![tara_form()],
        vec![TokenMatcher::Surface("どう")],
    ])
}

pub fn ba_yokatta() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("仮定形")],
        vec![TokenMatcher::Surface("ば")],
        vec![yokatta_form()],
        vec![TokenMatcher::Surface("た")],
    ])
}

// ========== Potential/Passive/Causative Forms ==========

/// Match ichidan verb in 未然形
fn ichidan_mizen() -> TokenMatcher {
    use std::sync::Arc;
    #[derive(Debug)]
    struct IchidanMizenMatcher;
    impl super::Matcher for IchidanMizenMatcher {
        fn matches(&self, token: &kagome_client::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(4).is_some_and(|f| f == "一段")
                && token.features.get(5).is_some_and(|f| f == "未然形")
        }
    }
    TokenMatcher::Custom(Arc::new(IchidanMizenMatcher))
}

/// Match godan verb in 未然形
fn godan_mizen() -> TokenMatcher {
    use std::sync::Arc;
    #[derive(Debug)]
    struct GodanMizenMatcher;
    impl super::Matcher for GodanMizenMatcher {
        fn matches(&self, token: &kagome_client::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(4).is_some_and(|f| f.starts_with("五段"))
                && token.features.get(5).is_some_and(|f| f == "未然形")
        }
    }
    TokenMatcher::Custom(Arc::new(GodanMizenMatcher))
}

/// Match られる or れる (ichidan potential/passive)
fn rareru_form() -> TokenMatcher {
    use std::sync::Arc;
    #[derive(Debug)]
    struct RareruFormMatcher;
    impl super::Matcher for RareruFormMatcher {
        fn matches(&self, token: &kagome_client::KagomeToken) -> bool {
            (token.base_form == "られる" || token.base_form == "れる")
                && token.pos.first().is_some_and(|s| s == "動詞")
                && token.pos.get(1).is_some_and(|s| s == "接尾")
        }
    }
    TokenMatcher::Custom(Arc::new(RareruFormMatcher))
}

/// Match れる (godan passive)
fn reru_form() -> TokenMatcher {
    use std::sync::Arc;
    #[derive(Debug)]
    struct ReruFormMatcher;
    impl super::Matcher for ReruFormMatcher {
        fn matches(&self, token: &kagome_client::KagomeToken) -> bool {
            token.base_form == "れる"
                && token.pos.first().is_some_and(|s| s == "動詞")
                && token.pos.get(1).is_some_and(|s| s == "接尾")
        }
    }
    TokenMatcher::Custom(Arc::new(ReruFormMatcher))
}

/// Match える (godan potential)
fn eru_form() -> TokenMatcher {
    use std::sync::Arc;
    #[derive(Debug)]
    struct EruFormMatcher;
    impl super::Matcher for EruFormMatcher {
        fn matches(&self, token: &kagome_client::KagomeToken) -> bool {
            token.base_form == "える"
                && token.pos.first().is_some_and(|s| s == "動詞")
                && token.pos.get(1).is_some_and(|s| s == "接尾")
        }
    }
    TokenMatcher::Custom(Arc::new(EruFormMatcher))
}

/// Match させる or せる (causative)
fn causative_form() -> TokenMatcher {
    use std::sync::Arc;
    #[derive(Debug)]
    struct CausativeFormMatcher;
    impl super::Matcher for CausativeFormMatcher {
        fn matches(&self, token: &kagome_client::KagomeToken) -> bool {
            (token.surface == "させる" || token.surface == "せる")
                && (token.base_form == "させる" || token.base_form == "せる")
        }
    }
    TokenMatcher::Custom(Arc::new(CausativeFormMatcher))
}

/// Match させ (causative stem)
fn sase_form() -> TokenMatcher {
    use std::sync::Arc;
    #[derive(Debug)]
    struct SaseFormMatcher;
    impl super::Matcher for SaseFormMatcher {
        fn matches(&self, token: &kagome_client::KagomeToken) -> bool {
            token.surface == "させ" && token.base_form == "させる"
        }
    }
    TokenMatcher::Custom(Arc::new(SaseFormMatcher))
}

/// Match ichidan verb for potential (used after が particle)
fn ga_potential_verb() -> TokenMatcher {
    use std::sync::Arc;
    #[derive(Debug)]
    struct GaPotentialVerbMatcher;
    impl super::Matcher for GaPotentialVerbMatcher {
        fn matches(&self, token: &kagome_client::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(4).is_some_and(|f| f == "一段")
        }
    }
    TokenMatcher::Custom(Arc::new(GaPotentialVerbMatcher))
}

pub fn potential_godan() -> Vec<TokenMatcher> {
    vec![godan_mizen(), eru_form()]
}

pub fn potential_ga_verb() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Surface("が"),
        TokenMatcher::Wildcard {
            min: 0,
            max: 2,
            stop_conditions: vec![
                TokenMatcher::Verb {
                    conjugation_form: None,
                    base_form: None,
                },
                super::particle_matcher(),
            ],
        },
        ga_potential_verb(),
    ]
}

pub fn potential_ga_ichidan() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Surface("が"),
        TokenMatcher::Wildcard {
            min: 0,
            max: 2,
            stop_conditions: vec![
                TokenMatcher::Verb {
                    conjugation_form: None,
                    base_form: None,
                },
                super::particle_matcher(),
            ],
        },
        ichidan_mizen(),
        rareru_form(),
    ]
}

pub fn passive_ichidan() -> Vec<TokenMatcher> {
    vec![ichidan_mizen(), rareru_form()]
}

pub fn passive_godan() -> Vec<TokenMatcher> {
    vec![godan_mizen(), reru_form()]
}

pub fn causative() -> Vec<TokenMatcher> {
    vec![TokenMatcher::verb_with_form("未然形"), causative_form()]
}

pub fn causative_passive() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("未然形")],
        vec![sase_form()],
        vec![rareru_form()],
    ])
}

// ========== Tendency & Difficulty Forms ==========

pub fn yasui() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("連用形")],
        vec![TokenMatcher::Surface("やすい")],
    ])
}

pub fn nikui() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("連用形")],
        vec![TokenMatcher::Surface("にくい")],
    ])
}

// ========== Simultaneity & Progression ==========

pub fn nagara() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("連用形")],
        vec![TokenMatcher::Surface("ながら")],
    ])
}

// ========== Additional Suffixes & Forms ==========

pub fn nasai() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("連用形")],
        vec![TokenMatcher::Surface("なさい")],
    ])
}

// ========== Volitional Forms ==========

pub fn short_volitional() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("未然ウ接続"),
        TokenMatcher::Surface("う"),
    ]
}

// ========== Imperative & Commands ==========

/// Match verb imperative forms (命令形, 命令ｒｏ, 命令ｉ, 命令ｅ)
fn imperative_form() -> TokenMatcher {
    use std::sync::Arc;
    #[derive(Debug)]
    struct ImperativeFormMatcher;
    impl super::Matcher for ImperativeFormMatcher {
        fn matches(&self, token: &kagome_client::KagomeToken) -> bool {
            if token.pos.first().is_none_or(|pos| pos != "動詞") {
                false
            } else {
                let form = token.features.get(5);
                form.is_some_and(|f| {
                    f == "命令形" || f == "命令ｒｏ" || f == "命令ｉ" || f == "命令ｅ"
                })
            }
        }
    }
    TokenMatcher::Custom(Arc::new(ImperativeFormMatcher))
}

pub fn imperative() -> Vec<TokenMatcher> {
    vec![imperative_form()]
}

// ========== Negative Obligation Forms ==========

/// Match なく from ない (negative form)
fn naku_form() -> TokenMatcher {
    use std::sync::Arc;
    #[derive(Debug)]
    struct NakuFormMatcher;
    impl super::Matcher for NakuFormMatcher {
        fn matches(&self, token: &kagome_client::KagomeToken) -> bool {
            token.surface == "なく" && token.base_form == "ない"
        }
    }
    TokenMatcher::Custom(Arc::new(NakuFormMatcher))
}

/// Match なけれ from ない (conditional negative)
fn nakere_form() -> TokenMatcher {
    use std::sync::Arc;
    #[derive(Debug)]
    struct NakereFormMatcher;
    impl super::Matcher for NakereFormMatcher {
        fn matches(&self, token: &kagome_client::KagomeToken) -> bool {
            token.surface == "なけれ" && token.base_form == "ない"
        }
    }
    TokenMatcher::Custom(Arc::new(NakereFormMatcher))
}

pub fn nakucha_ikenai() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("未然形"),
        naku_form(),
        TokenMatcher::Surface("ちゃ"),
        super::ikenai_form(),
    ]
}

pub fn must_nakereba() -> Vec<TokenMatcher> {
    let mut result = concat(vec![
        vec![TokenMatcher::verb_with_form("未然形")],
        vec![nakere_form()],
        vec![TokenMatcher::Surface("ば")],
        vec![TokenMatcher::Surface("なら")],
    ]);
    result.extend(super::optional(vec![TokenMatcher::Surface("ない")]));
    result
}

pub fn must_nakute_wa() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("未然形")],
        vec![naku_form()],
        vec![TokenMatcher::Surface("て")],
        vec![TokenMatcher::Surface("は")],
        vec![super::ikenai_form()],
    ])
}

// ========== Without Doing / Without Need To ==========

/// Natural verbs ending in れる (not potential forms)
const NATURAL_RERU_VERBS: &[&str] = &[
    "くれる",
    "入れる",
    "切れる",
    "晴れる",
    "慣れる",
    "汚れる",
    "疲れる",
    "腫れる",
    "暮れる",
    "揺れる",
    "枯れる",
    "破れる",
    "触れる",
];

/// Match 未然形 verbs that are NOT potential forms (excludes れる/られる base forms)
fn non_potential_mizen() -> TokenMatcher {
    use std::sync::Arc;
    #[derive(Debug)]
    struct NonPotentialMizenMatcher;
    impl super::Matcher for NonPotentialMizenMatcher {
        fn matches(&self, token: &kagome_client::KagomeToken) -> bool {
            // Must be verb in 未然形
            if token.pos.first().is_none_or(|pos| pos != "動詞") {
                return false;
            }

            let form = token.features.get(5);
            if form.is_none_or(|f| f != "未然形") {
                return false;
            }

            // Always exclude られる endings (always potential for ichidan verbs)
            if token.base_form.ends_with("られる") {
                return false;
            }

            // For れる endings: allow if in whitelist, exclude otherwise
            if token.base_form.ends_with("れる") {
                return NATURAL_RERU_VERBS.contains(&token.base_form.as_str());
            }

            // All other verbs are allowed
            true
        }
    }
    TokenMatcher::Custom(Arc::new(NonPotentialMizenMatcher))
}

pub fn naide() -> Vec<TokenMatcher> {
    vec![
        non_potential_mizen(),
        TokenMatcher::Surface("ない"),
        TokenMatcher::Surface("で"),
    ]
}

pub fn nakute_mo_ii() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("未然形")],
        vec![naku_form()],
        vec![TokenMatcher::Surface("て")],
        vec![TokenMatcher::Surface("も")],
        vec![super::ii_form()],
    ])
}

// ========== Expectation & Desire ==========

pub fn hazu_desu() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("基本形")],
        vec![TokenMatcher::Surface("はず")],
        vec![TokenMatcher::Surface("です")],
    ])
}

/// Match た from たい base form (for tagaru pattern)
fn tagaru_form() -> TokenMatcher {
    use std::sync::Arc;
    #[derive(Debug)]
    struct TagaruFormMatcher;
    impl super::Matcher for TagaruFormMatcher {
        fn matches(&self, token: &kagome_client::KagomeToken) -> bool {
            token.surface == "た"
                && token.base_form == "たい"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }
    TokenMatcher::Custom(Arc::new(TagaruFormMatcher))
}

pub fn tagaru() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("連用形"),
        tagaru_form(),
        TokenMatcher::specific_verb("がる"),
    ]
}

// ========== Desire & Negation Particles ==========

pub fn ga_hoshii() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Surface("が"),
        TokenMatcher::Surface("ほしい"),
    ]
}

/// Match 未然形 verbs excluding なる (for shika_nai pattern)
fn non_naru_mizen() -> TokenMatcher {
    use std::sync::Arc;
    #[derive(Debug)]
    struct NonNaruMizenMatcher;
    impl super::Matcher for NonNaruMizenMatcher {
        fn matches(&self, token: &kagome_client::KagomeToken) -> bool {
            // Must be verb in 未然形
            if token.pos.first().is_none_or(|pos| pos != "動詞") {
                return false;
            }

            let form = token.features.get(5);
            if form.is_none_or(|f| f != "未然形") {
                return false;
            }

            // Exclude なる (become) - "しかならない" doesn't make semantic sense
            token.base_form != "なる"
        }
    }
    TokenMatcher::Custom(Arc::new(NonNaruMizenMatcher))
}

pub fn shika_nai() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::Surface("しか")],
        vec![non_naru_mizen()],
        vec![TokenMatcher::Surface("ない")],
    ])
}

// ========== Named/Called Constructions & Conditionals ==========

pub fn to_iu() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("という")]
}

pub fn dictionary_to() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("基本形")],
        vec![TokenMatcher::Surface("と")],
    ])
}

/// Match いい in といい context (can be verb いう or adjective いい)
fn to_ii_form() -> TokenMatcher {
    use std::sync::Arc;
    #[derive(Debug)]
    struct ToIiFormMatcher;
    impl super::Matcher for ToIiFormMatcher {
        fn matches(&self, token: &kagome_client::KagomeToken) -> bool {
            (token.surface == "いい" && (token.base_form == "いう" || token.base_form == "いい"))
                || (token.surface == "良い" && token.base_form == "良い")
        }
    }
    TokenMatcher::Custom(Arc::new(ToIiFormMatcher))
}

pub fn to_ii() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("基本形")],
        vec![TokenMatcher::Surface("と")],
        vec![to_ii_form()],
    ])
}

pub fn koto_ni_suru() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("基本形")],
        vec![TokenMatcher::Surface("こと")],
        vec![TokenMatcher::Surface("に")],
        vec![TokenMatcher::specific_verb("する")],
    ])
}

pub fn koto_ni_naru() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("基本形")],
        vec![TokenMatcher::Surface("こと")],
        vec![TokenMatcher::Surface("に")],
        vec![TokenMatcher::specific_verb("なる")],
    ])
}

pub fn noni() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("基本形")],
        vec![TokenMatcher::Surface("のに")],
    ])
}

pub fn nara() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("なら")]
}

/// Match し particle (接続助詞)
fn shi_particle() -> TokenMatcher {
    use std::sync::Arc;
    #[derive(Debug)]
    struct ShiParticleMatcher;
    impl super::Matcher for ShiParticleMatcher {
        fn matches(&self, token: &kagome_client::KagomeToken) -> bool {
            token.surface == "し"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }
    TokenMatcher::Custom(Arc::new(ShiParticleMatcher))
}

pub fn shi() -> Vec<TokenMatcher> {
    vec![shi_particle()]
}

pub fn ka_dou_ka() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("基本形")],
        vec![TokenMatcher::Surface("か")],
        vec![TokenMatcher::Surface("どう")],
        vec![TokenMatcher::Surface("か")],
    ])
}

// ========== Appearance & Speculation ==========

pub fn mitai() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("基本形")],
        vec![TokenMatcher::Surface("みたい")],
    ])
}

pub fn mitai_adj_noun() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::Surface("みたい")]
}

pub fn kamo_shirenai() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("基本形")],
        vec![TokenMatcher::Surface("かも")],
        vec![TokenMatcher::specific_verb("しれる")],
        vec![TokenMatcher::Surface("ない")],
    ])
}

pub fn kamo_shirenai_adj_noun() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("かも"),
        TokenMatcher::specific_verb("しれる"),
        TokenMatcher::Surface("ない"),
    ]
}

pub fn kamo_shiremasen() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("基本形")],
        vec![TokenMatcher::Surface("かも")],
        vec![TokenMatcher::specific_verb("しれる")],
        vec![TokenMatcher::Surface("ませ")],
        vec![TokenMatcher::Surface("ん")],
    ])
}

pub fn kamo_shiremasen_adj_noun() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("かも"),
        TokenMatcher::specific_verb("しれる"),
        TokenMatcher::Surface("ませ"),
        TokenMatcher::Surface("ん"),
    ]
}

// ========== Honorific ==========

/// Match noun with お prefix (honorific)
fn noun_with_o_prefix() -> TokenMatcher {
    use std::sync::Arc;
    #[derive(Debug)]
    struct NounWithOPrefixMatcher;
    impl super::Matcher for NounWithOPrefixMatcher {
        fn matches(&self, token: &kagome_client::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.base_form.starts_with("お")
        }
    }
    TokenMatcher::Custom(Arc::new(NounWithOPrefixMatcher))
}

pub fn o_ni_naru() -> Vec<TokenMatcher> {
    concat(vec![
        vec![noun_with_o_prefix()],
        vec![TokenMatcher::Surface("に")],
        vec![TokenMatcher::specific_verb("なる")],
    ])
}

// ========== Appearance & Hearsay (そうです) ==========

/// Match verb 連用形 or adjective stem (for すぎる and そう appearance patterns)
fn sugiru_stem() -> TokenMatcher {
    use std::sync::Arc;
    #[derive(Debug)]
    struct SugiruStemMatcher;
    impl super::Matcher for SugiruStemMatcher {
        fn matches(&self, token: &kagome_client::KagomeToken) -> bool {
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
    TokenMatcher::Custom(Arc::new(SugiruStemMatcher))
}

/// Match plain form verb or adjective (for そうだ hearsay pattern)
fn sou_hearsay_stem() -> TokenMatcher {
    use std::sync::Arc;
    #[derive(Debug)]
    struct SouHearsayStemMatcher;
    impl super::Matcher for SouHearsayStemMatcher {
        fn matches(&self, token: &kagome_client::KagomeToken) -> bool {
            if token.pos.first().is_some_and(|pos| pos == "動詞") {
                let form = token.features.get(5);
                form.is_some_and(|f| f == "基本形")
            } else if token.pos.first().is_some_and(|pos| pos == "形容詞") {
                let form = token.features.get(5);
                form.is_some_and(|f| f == "基本形")
            } else if token.pos.first().is_some_and(|pos| pos == "形容動詞") {
                true
            } else if token.pos.first().is_some_and(|pos| pos == "名詞") {
                token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹")
            } else {
                false
            }
        }
    }
    TokenMatcher::Custom(Arc::new(SouHearsayStemMatcher))
}

pub fn sou_desu_appearance() -> Vec<TokenMatcher> {
    vec![
        sugiru_stem(),
        TokenMatcher::Surface("そう"),
        TokenMatcher::Surface("です"),
    ]
}

pub fn sou_desu_hearsay() -> Vec<TokenMatcher> {
    vec![
        sou_hearsay_stem(),
        TokenMatcher::Surface("そう"),
        TokenMatcher::Surface("です"),
    ]
}

pub fn sou_desu_hearsay_na() -> Vec<TokenMatcher> {
    vec![
        sou_hearsay_stem(),
        TokenMatcher::Surface("だ"),
        TokenMatcher::Surface("そう"),
        TokenMatcher::Surface("です"),
    ]
}
