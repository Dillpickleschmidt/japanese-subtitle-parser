use crate::grammar::pattern_matcher::{CustomMatcher, GrammarPattern, TokenMatcher};
use crate::grammar::types::ConjugationPattern;

/// JLPT N4 level grammar patterns (intermediate forms)
pub fn get_patterns() -> Vec<(GrammarPattern, ConjugationPattern, &'static str)> {
    vec![
        // Complex te-form patterns
        (
            GrammarPattern {
                name: "te_miru",
                tokens: vec![
                    TokenMatcher::verb_with_form("連用形"),
                    TokenMatcher::Surface("て"),
                    TokenMatcher::specific_verb("みる"),
                ],
                priority: 10,
            },
            ConjugationPattern::TeMiru,
            "n4",
        ),
        (
            GrammarPattern {
                name: "te_shimau",
                tokens: vec![
                    TokenMatcher::verb_with_form("連用形"),
                    TokenMatcher::Surface("て"),
                    TokenMatcher::specific_verb("しまう"),
                ],
                priority: 10,
            },
            ConjugationPattern::TeShimau,
            "n4",
        ),
        (
            GrammarPattern {
                name: "tari_form",
                tokens: vec![
                    TokenMatcher::verb_with_form("連用形"),
                    TokenMatcher::Surface("たり"),
                    TokenMatcher::Any,
                    TokenMatcher::Any,
                    TokenMatcher::specific_verb("する"),
                ],
                priority: 8,
            },
            ConjugationPattern::TariForm,
            "n4",
        ),
        // Conditionals
        (
            GrammarPattern {
                name: "ba_conditional",
                tokens: vec![
                    TokenMatcher::verb_with_form("仮定形"),
                    TokenMatcher::Surface("ば"),
                ],
                priority: 6,
            },
            ConjugationPattern::BaConditional,
            "n4",
        ),
        (
            GrammarPattern {
                name: "tara_conditional",
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm),
                    TokenMatcher::Custom(CustomMatcher::TaraForm),
                ],
                priority: 7,
            },
            ConjugationPattern::TaraConditional,
            "n4",
        ),
        // Causative/Passive forms
        (
            GrammarPattern {
                name: "potential",
                tokens: vec![
                    TokenMatcher::verb_with_form("未然形"),
                    TokenMatcher::Custom(CustomMatcher::RareruForm),
                ],
                priority: 5,
            },
            ConjugationPattern::Potential,
            "n4",
        ),
        (
            GrammarPattern {
                name: "passive",
                tokens: vec![
                    TokenMatcher::verb_with_form("未然形"),
                    TokenMatcher::Custom(CustomMatcher::RareruForm),
                ],
                priority: 4,
            },
            ConjugationPattern::Passive,
            "n4",
        ),
        (
            GrammarPattern {
                name: "causative",
                tokens: vec![
                    TokenMatcher::verb_with_form("未然形"),
                    TokenMatcher::Custom(CustomMatcher::CausativeForm),
                ],
                priority: 5,
            },
            ConjugationPattern::Causative,
            "n4",
        ),
        (
            GrammarPattern {
                name: "causative_passive",
                tokens: vec![
                    TokenMatcher::verb_with_form("未然形"),
                    TokenMatcher::Custom(CustomMatcher::SaseForm),
                    TokenMatcher::verb_with_form("未然形"),
                    TokenMatcher::Custom(CustomMatcher::RareruForm),
                ],
                priority: 10,
            },
            ConjugationPattern::CausativePassive,
            "n4",
        ),
        // Other N4 patterns
        (
            GrammarPattern {
                name: "volitional",
                tokens: vec![
                    TokenMatcher::verb_with_form("未然ウ接続"),
                    TokenMatcher::Surface("う"),
                ],
                priority: 6,
            },
            ConjugationPattern::Volitional,
            "n4",
        ),
        (
            GrammarPattern {
                name: "imperative",
                tokens: vec![TokenMatcher::verb_with_form("命令形")],
                priority: 5,
            },
            ConjugationPattern::Imperative,
            "n4",
        ),
        (
            GrammarPattern {
                name: "nagara",
                tokens: vec![
                    TokenMatcher::verb_with_form("連用形"),
                    TokenMatcher::Surface("ながら"),
                ],
                priority: 7,
            },
            ConjugationPattern::Nagara,
            "n4",
        ),
        (
            GrammarPattern {
                name: "past_negative",
                tokens: vec![
                    TokenMatcher::verb_with_form("未然形"),
                    TokenMatcher::Custom(CustomMatcher::NakattaForm),
                    TokenMatcher::Surface("た"),
                ],
                priority: 6,
            },
            ConjugationPattern::PastNegative,
            "n4",
        ),
        // Must patterns
        (
            GrammarPattern {
                name: "must_nakereba",
                tokens: vec![
                    TokenMatcher::verb_with_form("未然形"),
                    TokenMatcher::Custom(CustomMatcher::NakereForm),
                    TokenMatcher::Surface("ば"),
                    TokenMatcher::Custom(CustomMatcher::MustPattern),
                ],
                priority: 9,
            },
            ConjugationPattern::Must,
            "n4",
        ),
        (
            GrammarPattern {
                name: "must_nakute_wa",
                tokens: vec![
                    TokenMatcher::verb_with_form("未然形"),
                    TokenMatcher::Custom(CustomMatcher::NakuForm),
                    TokenMatcher::Surface("て"),
                    TokenMatcher::Surface("は"),
                    TokenMatcher::Custom(CustomMatcher::MustPattern),
                ],
                priority: 9,
            },
            ConjugationPattern::Must,
            "n4",
        ),
    ]
}
