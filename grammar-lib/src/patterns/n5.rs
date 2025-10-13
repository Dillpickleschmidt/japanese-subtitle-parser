use crate::pattern_matcher::{
    CustomMatcher, GrammarPattern, PatternCategory, TokenMatcher,
};
use crate::types::ConjugationPattern;

/// JLPT N5 level grammar patterns (fundamental forms)
pub fn get_patterns() -> Vec<(GrammarPattern, ConjugationPattern, &'static str)> {
    vec![
        // Te-form patterns (constructions)
        (
            GrammarPattern {
                name: "te_iru",
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm),
                    TokenMatcher::Custom(CustomMatcher::TeDeForm),
                    TokenMatcher::specific_verb("いる"),
                ],
                priority: 10,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TeIru,
            "n5",
        ),
        (
            GrammarPattern {
                name: "te_request",
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm),
                    TokenMatcher::Custom(CustomMatcher::TeDeForm),
                    TokenMatcher::specific_verb("くださる"),
                ],
                priority: 10,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TeRequest,
            "n5",
        ),
        (
            GrammarPattern {
                name: "te_kudasai",
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm),
                    TokenMatcher::Custom(CustomMatcher::TeDeForm),
                    TokenMatcher::Surface("ください"),
                ],
                priority: 10,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TeKudasai,
            "n5",
        ),
        (
            GrammarPattern {
                name: "te_kara",
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm),
                    TokenMatcher::Custom(CustomMatcher::TeDeForm),
                    TokenMatcher::Surface("から"),
                ],
                priority: 9,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TeKara,
            "n5",
        ),
        (
            GrammarPattern {
                name: "te_form_basic",
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm),
                    TokenMatcher::Custom(CustomMatcher::TeParticle),
                ],
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
                tokens: vec![
                    TokenMatcher::verb_with_form("連用形"),
                    TokenMatcher::Custom(CustomMatcher::TaiForm),
                ],
                priority: 5,
                category: PatternCategory::Conjugation,
            },
            ConjugationPattern::TaiForm,
            "n5",
        ),
        (
            GrammarPattern {
                name: "takatta_form",
                tokens: vec![
                    TokenMatcher::verb_with_form("連用形"),
                    TokenMatcher::Custom(CustomMatcher::TakattaForm),
                    TokenMatcher::Surface("た"),
                ],
                priority: 6,
                category: PatternCategory::Conjugation,
            },
            ConjugationPattern::TakattaForm,
            "n5",
        ),
        (
            GrammarPattern {
                name: "takunai_form",
                tokens: vec![
                    TokenMatcher::verb_with_form("連用形"),
                    TokenMatcher::Custom(CustomMatcher::TakuForm),
                    TokenMatcher::Surface("ない"),
                ],
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
                tokens: vec![
                    TokenMatcher::verb_with_form("連用形"),
                    TokenMatcher::Surface("ます"),
                ],
                priority: 4,
                category: PatternCategory::Conjugation,
            },
            ConjugationPattern::MasuForm,
            "n5",
        ),
        (
            GrammarPattern {
                name: "negative",
                tokens: vec![
                    TokenMatcher::verb_with_form("未然形"),
                    TokenMatcher::Surface("ない"),
                ],
                priority: 4,
                category: PatternCategory::Conjugation,
            },
            ConjugationPattern::Negative,
            "n5",
        ),
        (
            GrammarPattern {
                name: "past_tense",
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm),
                    TokenMatcher::Custom(CustomMatcher::PastAuxiliary),
                ],
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
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm),
                    TokenMatcher::Custom(CustomMatcher::PastAuxiliary),
                    TokenMatcher::Surface("こと"),
                    TokenMatcher::Surface("が"),
                    TokenMatcher::specific_verb("ある"),
                ],
                priority: 11,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TaKotoGaAru,
            "n5",
        ),
        (
            GrammarPattern {
                name: "te_mo_ii",
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm),
                    TokenMatcher::Custom(CustomMatcher::TeDeForm),
                    TokenMatcher::Surface("も"),
                    TokenMatcher::Custom(CustomMatcher::IiForm),
                ],
                priority: 11,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TeMoIi,
            "n5",
        ),
        (
            GrammarPattern {
                name: "te_wa_ikenai",
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm),
                    TokenMatcher::Custom(CustomMatcher::TeDeForm),
                    TokenMatcher::Surface("は"),
                    TokenMatcher::Custom(CustomMatcher::IkenaiForm),
                ],
                priority: 11,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TeWaIkenai,
            "n5",
        ),
        (
            GrammarPattern {
                name: "naide_kudasai",
                tokens: vec![
                    TokenMatcher::verb_with_form("未然形"),
                    TokenMatcher::Surface("ない"),
                    TokenMatcher::Surface("で"),
                    TokenMatcher::Surface("ください"),
                ],
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
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::SugiruStem),
                    TokenMatcher::specific_verb("すぎる"),
                ],
                priority: 6,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::Sugiru,
            "n5",
        ),
        (
            GrammarPattern {
                name: "tsumori_desu",
                tokens: vec![
                    TokenMatcher::verb_with_form("基本形"),
                    TokenMatcher::Surface("つもり"),
                    TokenMatcher::Surface("です"),
                ],
                priority: 9,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TsumoriDesu,
            "n5",
        ),
        (
            GrammarPattern {
                name: "hou_ga_ii",
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm),
                    TokenMatcher::Custom(CustomMatcher::PastAuxiliary),
                    TokenMatcher::Surface("ほう"),
                    TokenMatcher::Surface("が"),
                    TokenMatcher::Custom(CustomMatcher::IiForm),
                ],
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
                    TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm),
                    TokenMatcher::Custom(CustomMatcher::TeDeForm),
                    TokenMatcher::Surface("い"),
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
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::NDesuForm),
                    TokenMatcher::Surface("です"),
                ],
                priority: 5,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::NDesu,
            "n5",
        ),
        (
            GrammarPattern {
                name: "node",
                tokens: vec![
                    TokenMatcher::verb_with_form("基本形"),
                    TokenMatcher::Surface("ので"),
                ],
                priority: 5,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::Node,
            "n5",
        ),
        (
            GrammarPattern {
                name: "ni_iku",
                tokens: vec![
                    TokenMatcher::verb_with_form("連用形"),
                    TokenMatcher::Surface("に"),
                    TokenMatcher::specific_verb("行く"),
                ],
                priority: 8,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::NiIku,
            "n5",
        ),
        (
            GrammarPattern {
                name: "mae_ni",
                tokens: vec![
                    TokenMatcher::verb_with_form("基本形"),
                    TokenMatcher::Surface("前"),
                    TokenMatcher::Surface("に"),
                ],
                priority: 7,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::MaeNi,
            "n5",
        ),
    ]
}
