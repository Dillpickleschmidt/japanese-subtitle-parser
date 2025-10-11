use crate::grammar::pattern_matcher::{CustomMatcher, GrammarPattern, TokenMatcher};
use crate::grammar::types::ConjugationPattern;

/// JLPT N5 level grammar patterns (fundamental forms)
pub fn get_patterns() -> Vec<(GrammarPattern, ConjugationPattern, &'static str)> {
    vec![
        // Te-form patterns
        (
            GrammarPattern {
                name: "te_iru",
                tokens: vec![
                    TokenMatcher::verb_with_form("連用形"),
                    TokenMatcher::Surface("て"),
                    TokenMatcher::specific_verb("いる"),
                ],
                priority: 10,
            },
            ConjugationPattern::TeIru,
            "n5",
        ),
        (
            GrammarPattern {
                name: "te_request",
                tokens: vec![
                    TokenMatcher::verb_with_form("連用形"),
                    TokenMatcher::Surface("て"),
                    TokenMatcher::specific_verb("くださる"),
                ],
                priority: 10,
            },
            ConjugationPattern::TeRequest,
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
            },
            ConjugationPattern::TeForm,
            "n5",
        ),
        // Tai-form patterns
        (
            GrammarPattern {
                name: "tai_form",
                tokens: vec![
                    TokenMatcher::verb_with_form("連用形"),
                    TokenMatcher::Custom(CustomMatcher::TaiForm),
                ],
                priority: 5,
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
            },
            ConjugationPattern::Past,
            "n5",
        ),
    ]
}
