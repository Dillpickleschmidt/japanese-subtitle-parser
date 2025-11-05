use crate::pattern_components::*;
use crate::pattern_matcher::{CustomMatcher, GrammarPattern, PatternCategory, TokenMatcher};
use crate::types::ConjugationPattern;

/// JLPT N4 level grammar patterns (intermediate forms)
pub fn get_patterns() -> Vec<(GrammarPattern, ConjugationPattern, &'static str)> {
    vec![
        // Complex te-form patterns
        (
            GrammarPattern {
                name: "te_miru",
                tokens: te_miru(),
                priority: 10,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TeMiru,
            "n4",
        ),
        (
            GrammarPattern {
                name: "te_shimau",
                tokens: te_shimau(),
                priority: 10,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TeShimau,
            "n4",
        ),
        (
            GrammarPattern {
                name: "tari_suru_single",
                tokens: tari_suru_single(),
                priority: 8,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TariForm,
            "n4",
        ),
        (
            GrammarPattern {
                name: "tari_suru",
                tokens: tari_suru(),
                priority: 8,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TariForm,
            "n4",
        ),
        // Conditionals
        (
            GrammarPattern {
                name: "ba_conditional",
                tokens: ba_conditional(),
                priority: 6,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::BaConditional,
            "n4",
        ),
        (
            GrammarPattern {
                name: "tara_conditional",
                tokens: tara_conditional(),
                priority: 7,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TaraConditional,
            "n4",
        ),
        // Causative/Passive forms
        (
            GrammarPattern {
                name: "potential_godan",
                tokens: potential_godan(),
                priority: 5,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::Potential,
            "n4",
        ),
        (
            GrammarPattern {
                name: "passive_ichidan",
                tokens: passive_ichidan(),
                priority: 4,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::Passive,
            "n4",
        ),
        (
            GrammarPattern {
                name: "passive_godan",
                tokens: passive_godan(),
                priority: 4,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::Passive,
            "n4",
        ),
        (
            GrammarPattern {
                name: "potential_ga_verb",
                tokens: potential_ga_verb(),
                priority: 6,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::Potential,
            "n4",
        ),
        (
            GrammarPattern {
                name: "potential_ga_ichidan",
                tokens: potential_ga_ichidan(),
                priority: 6,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::Potential,
            "n4",
        ),
        (
            GrammarPattern {
                name: "causative",
                tokens: causative(),
                priority: 5,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::Causative,
            "n4",
        ),
        (
            GrammarPattern {
                name: "causative_passive",
                tokens: causative_passive(),
                priority: 11,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::CausativePassive,
            "n4",
        ),
        // Volitional patterns (plain and polite forms)
        (
            GrammarPattern {
                name: "volitional",
                tokens: volitional_u_form(),
                priority: 6,
                category: PatternCategory::Conjugation,
            },
            ConjugationPattern::Volitional,
            "n4",
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
            "n4",
        ),
        (
            GrammarPattern {
                name: "imperative",
                tokens: imperative(),
                priority: 5,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::Imperative,
            "n4",
        ),
        (
            GrammarPattern {
                name: "nagara",
                tokens: nagara(),
                priority: 7,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::Nagara,
            "n4",
        ),
        (
            GrammarPattern {
                name: "past_negative",
                tokens: past_negative(),
                priority: 6,
                category: PatternCategory::Construction,
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
                    TokenMatcher::Surface("ない"),
                ],
                priority: 10,
                category: PatternCategory::Construction,
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
                category: PatternCategory::Construction,
            },
            ConjugationPattern::Must,
            "n4",
        ),
        // Te-form extensions
        (
            GrammarPattern {
                name: "te_aru",
                tokens: te_aru(),
                priority: 10,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TeAru,
            "n4",
        ),
        (
            GrammarPattern {
                name: "te_kureru",
                tokens: te_kureru(),
                priority: 10,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TeKureru,
            "n4",
        ),
        (
            GrammarPattern {
                name: "te_ageru",
                tokens: te_ageru(),
                priority: 10,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TeAgeru,
            "n4",
        ),
        (
            GrammarPattern {
                name: "te_oku",
                tokens: te_oku(),
                priority: 10,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TeOku,
            "n4",
        ),
        // Auxiliary verbs
        (
            GrammarPattern {
                name: "yasui",
                tokens: yasui(),
                priority: 6,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::Yasui,
            "n4",
        ),
        (
            GrammarPattern {
                name: "nikui",
                tokens: nikui(),
                priority: 6,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::Nikui,
            "n4",
        ),
        // Additional te-form patterns
        (
            GrammarPattern {
                name: "te_morau",
                tokens: te_morau(),
                priority: 10,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TeMorau,
            "n4",
        ),
        (
            GrammarPattern {
                name: "te_sumimasen",
                tokens: te_sumimasen(),
                priority: 11,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TeSumimasen,
            "n4",
        ),
        (
            GrammarPattern {
                name: "te_kurete_arigatou",
                tokens: te_kurete_arigatou(),
                priority: 13,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TeKureteArigatou,
            "n4",
        ),
        (
            GrammarPattern {
                name: "te_yokatta",
                tokens: te_yokatta(),
                priority: 11,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TeYokatta,
            "n4",
        ),
        (
            GrammarPattern {
                name: "te_mo",
                tokens: te_mo(),
                priority: 8,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TeMo,
            "n4",
        ),
        // Nai-form patterns
        (
            GrammarPattern {
                name: "naide",
                tokens: naide(),
                priority: 7,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::Naide,
            "n4",
        ),
        (
            GrammarPattern {
                name: "nakute_mo_ii",
                tokens: nakute_mo_ii(),
                priority: 11,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::NakuteMoIi,
            "n4",
        ),
        (
            GrammarPattern {
                name: "ba_yokatta",
                tokens: ba_yokatta(),
                priority: 10,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::BaYokatta,
            "n4",
        ),
        // Auxiliary/modal patterns
        (
            GrammarPattern {
                name: "nasai",
                tokens: nasai(),
                priority: 6,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::Nasai,
            "n4",
        ),
        (
            GrammarPattern {
                name: "hazu_desu",
                tokens: hazu_desu(),
                priority: 9,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::HazuDesu,
            "n4",
        ),
        (
            GrammarPattern {
                name: "tagaru",
                tokens: vec![
                    TokenMatcher::verb_with_form("連用形"),
                    TokenMatcher::Custom(CustomMatcher::TagaruForm),
                    TokenMatcher::specific_verb("がる"),
                ],
                priority: 7,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::Tagaru,
            "n4",
        ),
        (
            GrammarPattern {
                name: "te_itadakemasen_ka",
                tokens: te_itadakemasen_ka(),
                priority: 10,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TeItadakemasenKa,
            "n4",
        ),
        // Other common patterns
        (
            GrammarPattern {
                name: "tara_dou",
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm),
                    TokenMatcher::Custom(CustomMatcher::TaraForm),
                    TokenMatcher::Surface("どう"),
                    TokenMatcher::Surface("です"),
                    TokenMatcher::Surface("か"),
                ],
                priority: 12,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TaraDou,
            "n4",
        ),
        (
            GrammarPattern {
                name: "to_ii",
                tokens: to_ii(),
                priority: 8,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::ToIi,
            "n4",
        ),
        (
            GrammarPattern {
                name: "ga_hoshii",
                tokens: ga_hoshii(),
                priority: 5,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::GaHoshii,
            "n4",
        ),
        // Additional N4 patterns
        (
            GrammarPattern {
                name: "shika_nai",
                tokens: shika_nai(),
                priority: 8,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::ShikaNai,
            "n4",
        ),
        (
            GrammarPattern {
                name: "to_iu",
                tokens: to_iu(),
                priority: 5,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::ToIu,
            "n4",
        ),
        (
            GrammarPattern {
                name: "dictionary_to",
                tokens: dictionary_to(),
                priority: 4,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::DictionaryTo,
            "n4",
        ),
        (
            GrammarPattern {
                name: "nara",
                tokens: nara_conditional(),
                priority: 6,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::Nara,
            "n4",
        ),
        (
            GrammarPattern {
                name: "shi",
                tokens: vec![TokenMatcher::Custom(CustomMatcher::ShiParticle)],
                priority: 3,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::Shi,
            "n4",
        ),
        (
            GrammarPattern {
                name: "ka_dou_ka",
                tokens: ka_dou_ka(),
                priority: 9,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::KaDouKa,
            "n4",
        ),
        (
            GrammarPattern {
                name: "koto_ni_suru",
                tokens: koto_ni_suru(),
                priority: 10,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::KotoNiSuru,
            "n4",
        ),
        (
            GrammarPattern {
                name: "noni",
                tokens: noni(),
                priority: 5,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::Noni,
            "n4",
        ),
        (
            GrammarPattern {
                name: "koto_ni_naru",
                tokens: koto_ni_naru(),
                priority: 10,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::KotoNiNaru,
            "n4",
        ),
        (
            GrammarPattern {
                name: "o_ni_naru",
                tokens: o_ni_naru(),
                priority: 8,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::ONiNaru,
            "n4",
        ),
        // Sou desu patterns
        (
            GrammarPattern {
                name: "sou_desu_appearance",
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::SugiruStem),
                    TokenMatcher::Surface("そう"),
                    TokenMatcher::Surface("です"),
                ],
                priority: 9,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::SouDesuAppearance,
            "n4",
        ),
        (
            GrammarPattern {
                name: "sou_desu_hearsay",
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::SouHearsayStem),
                    TokenMatcher::Surface("そう"),
                    TokenMatcher::Surface("です"),
                ],
                priority: 8,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::SouDesuHearsay,
            "n4",
        ),
        // Na-adjective hearsay pattern: 静かだそうです (needs だ between stem and そう)
        (
            GrammarPattern {
                name: "sou_desu_hearsay_na",
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::SouHearsayStem),
                    TokenMatcher::Surface("だ"),
                    TokenMatcher::Surface("そう"),
                    TokenMatcher::Surface("です"),
                ],
                priority: 9,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::SouDesuHearsay,
            "n4",
        ),
        // Kamo shirenai patterns (might/maybe)
        // Tokenizes as: かも + しれ (未然形) + ない
        (
            GrammarPattern {
                name: "kamo_shirenai",
                tokens: kamo_shirenai(),
                priority: 8,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::KamoShirenai,
            "n4",
        ),
        // Tokenizes as: かも + しれ (未然形) + ませ + ん
        (
            GrammarPattern {
                name: "kamo_shiremasen",
                tokens: kamo_shiremasen(),
                priority: 8,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::KamoShirenai,
            "n4",
        ),
        // Kamo shirenai with adjectives/nouns
        (
            GrammarPattern {
                name: "kamo_shirenai_adj_noun",
                tokens: vec![
                    TokenMatcher::Any, // Match any i-adj, na-adj, or noun
                    TokenMatcher::Surface("かも"),
                    TokenMatcher::specific_verb("しれる"),
                    TokenMatcher::Surface("ない"),
                ],
                priority: 7,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::KamoShirenai,
            "n4",
        ),
        (
            GrammarPattern {
                name: "kamo_shiremasen_adj_noun",
                tokens: vec![
                    TokenMatcher::Any, // Match any i-adj, na-adj, or noun
                    TokenMatcher::Surface("かも"),
                    TokenMatcher::specific_verb("しれる"),
                    TokenMatcher::Surface("ませ"),
                    TokenMatcher::Surface("ん"),
                ],
                priority: 7,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::KamoShirenai,
            "n4",
        ),
        // Mitai pattern (looks like/seems)
        (
            GrammarPattern {
                name: "mitai",
                tokens: mitai(),
                priority: 6,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::Mitai,
            "n4",
        ),
        (
            GrammarPattern {
                name: "mitai_adj_noun",
                tokens: vec![
                    TokenMatcher::Any, // Match any i-adj, na-adj, or noun
                    TokenMatcher::Surface("みたい"),
                ],
                priority: 5,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::Mitai,
            "n4",
        ),
    ]
}
