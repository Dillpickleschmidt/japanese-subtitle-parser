use super::pattern_matcher::{GrammarPattern, PatternMatcher, TokenMatcher, CustomMatcher};
use super::types::ConjugationPattern;

/// Create a pattern matcher configured with Genki-specific grammar patterns
pub fn create_genki_pattern_matcher() -> PatternMatcher<ConjugationPattern> {
    let mut matcher = PatternMatcher::new();
    
    // Add all patterns - ordered roughly by complexity/priority
    let mut patterns = vec![
        // Complex multi-token patterns first (higher priority)
        create_te_iru_pattern(),
        create_te_request_pattern(),
        create_te_miru_pattern(),
        create_te_shimau_pattern(),
        create_causative_passive_pattern(),
        create_tari_form_pattern(),
        create_must_nakereba_pattern(),
        create_must_nakute_wa_pattern(),
        create_nagara_pattern(),
        create_tara_conditional_pattern(),
        
        // Medium complexity patterns
        create_potential_pattern(),
        create_passive_pattern(),
        create_causative_pattern(),
        create_ba_conditional_pattern(),
        create_volitional_pattern(),
        create_imperative_pattern(),
        
        // Simple patterns
        create_te_form_pattern(),
        create_past_pattern(),
        create_past_negative_pattern(),
        create_negative_pattern(),
        create_masu_form_pattern(),
        create_dictionary_form_pattern(),
    ];
    
    // Add tai form patterns
    patterns.extend(create_tai_form_patterns());
    
    matcher.add_patterns(patterns);
    
    matcher
}

// Complex patterns (highest priority)

fn create_te_iru_pattern() -> (GrammarPattern, ConjugationPattern) {
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
    )
}

fn create_te_request_pattern() -> (GrammarPattern, ConjugationPattern) {
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
    )
}

fn create_te_miru_pattern() -> (GrammarPattern, ConjugationPattern) {
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
    )
}

fn create_te_shimau_pattern() -> (GrammarPattern, ConjugationPattern) {
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
    )
}

fn create_causative_passive_pattern() -> (GrammarPattern, ConjugationPattern) {
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
    )
}

fn create_tari_form_pattern() -> (GrammarPattern, ConjugationPattern) {
    (
        GrammarPattern {
            name: "tari_form",
            tokens: vec![
                TokenMatcher::verb_with_form("連用形"),
                TokenMatcher::Surface("たり"),
                TokenMatcher::Any, // Could be another verb or particle
                TokenMatcher::Any, // Allow flexible middle content
                TokenMatcher::specific_verb("する"),
            ],
            priority: 8,
        },
        ConjugationPattern::TariForm,
    )
}

fn create_must_nakereba_pattern() -> (GrammarPattern, ConjugationPattern) {
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
    )
}

fn create_must_nakute_wa_pattern() -> (GrammarPattern, ConjugationPattern) {
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
    )
}

fn create_nagara_pattern() -> (GrammarPattern, ConjugationPattern) {
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
    )
}

fn create_tara_conditional_pattern() -> (GrammarPattern, ConjugationPattern) {
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
    )
}

// Medium complexity patterns

fn create_tai_form_patterns() -> Vec<(GrammarPattern, ConjugationPattern)> {
    vec![
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
        ),
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
        ),
    ]
}

fn create_potential_pattern() -> (GrammarPattern, ConjugationPattern) {
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
    )
}

fn create_passive_pattern() -> (GrammarPattern, ConjugationPattern) {
    (
        GrammarPattern {
            name: "passive",
            tokens: vec![
                TokenMatcher::verb_with_form("未然形"),
                TokenMatcher::Custom(CustomMatcher::RareruForm),
            ],
            priority: 4, // Lower priority than potential (same form, context-dependent)
        },
        ConjugationPattern::Passive,
    )
}

fn create_causative_pattern() -> (GrammarPattern, ConjugationPattern) {
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
    )
}

fn create_ba_conditional_pattern() -> (GrammarPattern, ConjugationPattern) {
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
    )
}

fn create_volitional_pattern() -> (GrammarPattern, ConjugationPattern) {
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
    )
}

fn create_imperative_pattern() -> (GrammarPattern, ConjugationPattern) {
    (
        GrammarPattern {
            name: "imperative",
            tokens: vec![
                TokenMatcher::verb_with_form("命令形"),
            ],
            priority: 5,
        },
        ConjugationPattern::Imperative,
    )
}

// Simple patterns (lower priority)

fn create_te_form_pattern() -> (GrammarPattern, ConjugationPattern) {
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
    )
}

fn create_past_pattern() -> (GrammarPattern, ConjugationPattern) {
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
    )
}

fn create_past_negative_pattern() -> (GrammarPattern, ConjugationPattern) {
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
    )
}

fn create_negative_pattern() -> (GrammarPattern, ConjugationPattern) {
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
    )
}

fn create_masu_form_pattern() -> (GrammarPattern, ConjugationPattern) {
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
    )
}

fn create_dictionary_form_pattern() -> (GrammarPattern, ConjugationPattern) {
    (
        GrammarPattern {
            name: "dictionary_form",
            tokens: vec![
                TokenMatcher::verb_with_form("基本形"),
            ],
            priority: 1, // Lowest priority - default fallback
        },
        ConjugationPattern::Dictionary,
    )
}

