use crate::pattern_components::*;
use crate::pattern_matcher::{CustomMatcher, GrammarPattern, PatternCategory, TokenMatcher};
use crate::types::ConjugationPattern;

/// JLPT N5 level grammar patterns (fundamental forms)
pub fn get_patterns() -> Vec<(GrammarPattern, ConjugationPattern, &'static str)> {
    vec![
        // Te-form patterns (constructions)
        (
            GrammarPattern {
                name: "te_iru",
                tokens: te_iru(),
                priority: 10,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TeIru,
            "n5",
        ),
        (
            GrammarPattern {
                name: "te_request",
                tokens: te_request(),
                priority: 10,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TeRequest,
            "n5",
        ),
        (
            GrammarPattern {
                name: "te_kudasai",
                tokens: te_kudasai_construction(),
                priority: 10,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TeKudasai,
            "n5",
        ),
        (
            GrammarPattern {
                name: "te_kara",
                tokens: concat(vec![te_construction(), kara_suffix()]),
                priority: 9,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TeKara,
            "n5",
        ),
        (
            GrammarPattern {
                name: "te_form_basic",
                tokens: te_construction(),
                priority: 3,
                category: PatternCategory::Conjugation,
            },
            ConjugationPattern::TeForm,
            "n5",
        ),
        // Tai-form patterns (basic conjugations)
        (
            GrammarPattern {
                name: "tai_form",
                tokens: tai_form(),
                priority: 5,
                category: PatternCategory::Conjugation,
            },
            ConjugationPattern::TaiForm,
            "n5",
        ),
        (
            GrammarPattern {
                name: "takatta_form",
                tokens: takatta_form(),
                priority: 6,
                category: PatternCategory::Conjugation,
            },
            ConjugationPattern::TakattaForm,
            "n5",
        ),
        (
            GrammarPattern {
                name: "takunai_form",
                tokens: takunai_form(),
                priority: 6,
                category: PatternCategory::Conjugation,
            },
            ConjugationPattern::TakunaiForm,
            "n5",
        ),
        // Basic conjugations
        (
            GrammarPattern {
                name: "dictionary_form",
                tokens: vec![TokenMatcher::verb_with_form("基本形")],
                priority: 1,
                category: PatternCategory::Conjugation,
            },
            ConjugationPattern::Dictionary,
            "n5",
        ),
        (
            GrammarPattern {
                name: "masu_form",
                tokens: masu_ending(),
                priority: 4,
                category: PatternCategory::Conjugation,
            },
            ConjugationPattern::MasuForm,
            "n5",
        ),
        (
            GrammarPattern {
                name: "negative",
                tokens: negative_ending(),
                priority: 4,
                category: PatternCategory::Conjugation,
            },
            ConjugationPattern::Negative,
            "n5",
        ),
        (
            GrammarPattern {
                name: "past_tense",
                tokens: ta_form(),
                priority: 4,
                category: PatternCategory::Conjugation,
            },
            ConjugationPattern::Past,
            "n5",
        ),
        (
            GrammarPattern {
                name: "mashou",
                tokens: vec![
                    TokenMatcher::verb_with_form("連用形"),
                    TokenMatcher::Custom(CustomMatcher::MashouForm),
                ],
                priority: 6,
                category: PatternCategory::Conjugation,
            },
            ConjugationPattern::Mashou,
            "n5",
        ),
        (
            GrammarPattern {
                name: "ta_koto_ga_aru",
                tokens: ta_koto_ga_aru(),
                priority: 11,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TaKotoGaAru,
            "n5",
        ),
        (
            GrammarPattern {
                name: "te_mo_ii",
                tokens: te_mo_ii(),
                priority: 11,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TeMoIi,
            "n5",
        ),
        (
            GrammarPattern {
                name: "te_wa_ikenai",
                tokens: te_wa_ikenai(),
                priority: 11,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TeWaIkenai,
            "n5",
        ),
        (
            GrammarPattern {
                name: "naide_kudasai",
                tokens: naide_kudasai(),
                priority: 11,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::NaideKudasai,
            "n5",
        ),
        (
            GrammarPattern {
                name: "masen_ka",
                tokens: vec![
                    TokenMatcher::verb_with_form("連用形"),
                    TokenMatcher::Custom(CustomMatcher::MasenForm),
                    TokenMatcher::Surface("ん"),
                    TokenMatcher::Surface("か"),
                ],
                priority: 7,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::MasenKa,
            "n5",
        ),
        (
            GrammarPattern {
                name: "mashou_ka",
                tokens: vec![
                    TokenMatcher::verb_with_form("連用形"),
                    TokenMatcher::Custom(CustomMatcher::MashouForm),
                    TokenMatcher::Surface("う"),
                    TokenMatcher::Surface("か"),
                ],
                priority: 8,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::MashouKa,
            "n5",
        ),
        (
            GrammarPattern {
                name: "sugiru",
                tokens: sugiru(),
                priority: 6,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::Sugiru,
            "n5",
        ),
        (
            GrammarPattern {
                name: "tsumori_desu",
                tokens: tsumori_desu(),
                priority: 9,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TsumoriDesu,
            "n5",
        ),
        (
            GrammarPattern {
                name: "hou_ga_ii",
                tokens: concat(vec![ta_form(), hou_ga_ii_suffix()]),
                priority: 11,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::HouGaIi,
            "n5",
        ),
        (
            GrammarPattern {
                name: "nakucha_ikenai",
                tokens: vec![
                    TokenMatcher::verb_with_form("未然形"),
                    TokenMatcher::Surface("なく"),
                    TokenMatcher::Surface("ちゃ"),
                    TokenMatcher::Custom(CustomMatcher::IkenaiForm),
                ],
                priority: 10,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::NakuchaIkenai,
            "n5",
        ),
        (
            GrammarPattern {
                name: "deshou",
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::DeshouPreceding),
                    TokenMatcher::Custom(CustomMatcher::DeshouForm),
                    TokenMatcher::Surface("う"),
                ],
                priority: 5,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::Deshou,
            "n5",
        ),
        (
            GrammarPattern {
                name: "mada_te_imasen",
                tokens: vec![
                    TokenMatcher::Surface("まだ"),
                    TokenMatcher::Wildcard {
                        min: 0,
                        max: 5,
                        stop_at_punctuation: true,
                    },
                    TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm),
                    TokenMatcher::Custom(CustomMatcher::TeDeForm),
                    TokenMatcher::specific_verb("いる"),
                    TokenMatcher::Custom(CustomMatcher::MasenForm),
                    TokenMatcher::Surface("ん"),
                ],
                priority: 12,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::MadaTeImasen,
            "n5",
        ),
        (
            GrammarPattern {
                name: "n_desu",
                tokens: n_desu(),
                priority: 5,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::NDesu,
            "n5",
        ),
        // Node pattern: multiple variations to handle verbs, adjectives, and nominals
        (
            GrammarPattern {
                name: "node_verb",
                tokens: node_verb(),
                priority: 6,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::Node,
            "n5",
        ),
        (
            GrammarPattern {
                name: "node_adjective",
                tokens: node_adjective(),
                priority: 5,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::Node,
            "n5",
        ),
        (
            GrammarPattern {
                name: "node_nominal",
                tokens: node_nominal(),
                priority: 4,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::Node,
            "n5",
        ),
        (
            GrammarPattern {
                name: "ni_iku",
                tokens: ni_iku(),
                priority: 8,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::NiIku,
            "n5",
        ),
        (
            GrammarPattern {
                name: "mae_ni",
                tokens: mae_ni(),
                priority: 7,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::MaeNi,
            "n5",
        ),
        // Adjective conjugation patterns
        (
            GrammarPattern {
                name: "adjective",
                tokens: vec![TokenMatcher::Adjective { base_form: None }],
                priority: 1,
                category: PatternCategory::Conjugation,
            },
            ConjugationPattern::Dictionary,
            "n5",
        ),
        (
            GrammarPattern {
                name: "adjective_past",
                tokens: vec![
                    TokenMatcher::Adjective { base_form: None },
                    TokenMatcher::Surface("た"),
                ],
                priority: 2,
                category: PatternCategory::Conjugation,
            },
            ConjugationPattern::Dictionary,
            "n5",
        ),
    ]
}
