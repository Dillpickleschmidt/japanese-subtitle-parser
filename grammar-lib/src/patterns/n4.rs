use crate::pattern_matcher::{
    CustomMatcher, GrammarPattern, PatternCategory, TokenMatcher,
};
use crate::types::ConjugationPattern;

/// JLPT N4 level grammar patterns (intermediate forms)
pub fn get_patterns() -> Vec<(GrammarPattern, ConjugationPattern, &'static str)> {
    vec![
        // Complex te-form patterns
        (
            GrammarPattern {
                name: "te_miru",
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm),
                    TokenMatcher::Custom(CustomMatcher::TeDeForm),
                    TokenMatcher::specific_verb("みる"),
                ],
                priority: 10,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TeMiru,
            "n4",
        ),
        (
            GrammarPattern {
                name: "te_shimau",
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm),
                    TokenMatcher::Custom(CustomMatcher::TeDeForm),
                    TokenMatcher::specific_verb("しまう"),
                ],
                priority: 10,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TeShimau,
            "n4",
        ),
        (
            GrammarPattern {
                name: "tari_form",
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm),
                    TokenMatcher::Custom(CustomMatcher::TariParticle),
                    TokenMatcher::specific_verb("する"),
                ],
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
                tokens: vec![
                    TokenMatcher::verb_with_form("仮定形"),
                    TokenMatcher::Surface("ば"),
                ],
                priority: 6,
                category: PatternCategory::Construction,
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
                category: PatternCategory::Construction,
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
                category: PatternCategory::Construction,
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
                category: PatternCategory::Construction,
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
                category: PatternCategory::Construction,
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
                    TokenMatcher::Custom(CustomMatcher::RareruForm),
                ],
                priority: 11,
                category: PatternCategory::Construction,
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
                category: PatternCategory::Construction,
            },
            ConjugationPattern::Volitional,
            "n4",
        ),
        (
            GrammarPattern {
                name: "imperative",
                tokens: vec![TokenMatcher::Custom(CustomMatcher::ImperativeForm)],
                priority: 5,
                category: PatternCategory::Construction,
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
                category: PatternCategory::Construction,
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
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm),
                    TokenMatcher::Custom(CustomMatcher::TeDeForm),
                    TokenMatcher::specific_verb("ある"),
                ],
                priority: 10,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TeAru,
            "n4",
        ),
        (
            GrammarPattern {
                name: "te_kureru",
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm),
                    TokenMatcher::Custom(CustomMatcher::TeDeForm),
                    TokenMatcher::specific_verb("くれる"),
                ],
                priority: 10,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TeKureru,
            "n4",
        ),
        (
            GrammarPattern {
                name: "te_ageru",
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm),
                    TokenMatcher::Custom(CustomMatcher::TeDeForm),
                    TokenMatcher::specific_verb("あげる"),
                ],
                priority: 10,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TeAgeru,
            "n4",
        ),
        (
            GrammarPattern {
                name: "te_oku",
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm),
                    TokenMatcher::Custom(CustomMatcher::TeDeForm),
                    TokenMatcher::specific_verb("おく"),
                ],
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
                tokens: vec![
                    TokenMatcher::verb_with_form("連用形"),
                    TokenMatcher::Surface("やすい"),
                ],
                priority: 6,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::Yasui,
            "n4",
        ),
        (
            GrammarPattern {
                name: "nikui",
                tokens: vec![
                    TokenMatcher::verb_with_form("連用形"),
                    TokenMatcher::Surface("にくい"),
                ],
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
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm),
                    TokenMatcher::Custom(CustomMatcher::TeDeForm),
                    TokenMatcher::specific_verb("もらう"),
                ],
                priority: 10,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TeMorau,
            "n4",
        ),
        (
            GrammarPattern {
                name: "te_sumimasen",
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm),
                    TokenMatcher::Custom(CustomMatcher::TeDeForm),
                    TokenMatcher::Surface("すみません"),
                ],
                priority: 11,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TeSumimasen,
            "n4",
        ),
        (
            GrammarPattern {
                name: "te_kurete_arigatou",
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm),
                    TokenMatcher::Custom(CustomMatcher::TeDeForm),
                    TokenMatcher::specific_verb("くれる"),
                    TokenMatcher::Custom(CustomMatcher::TeDeForm),
                    TokenMatcher::Surface("ありがとう"),
                ],
                priority: 13,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TeKureteArigatou,
            "n4",
        ),
        (
            GrammarPattern {
                name: "te_yokatta",
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm),
                    TokenMatcher::Custom(CustomMatcher::TeDeForm),
                    TokenMatcher::Custom(CustomMatcher::YokattaForm),
                ],
                priority: 11,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::TeYokatta,
            "n4",
        ),
        (
            GrammarPattern {
                name: "te_mo",
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm),
                    TokenMatcher::Custom(CustomMatcher::TeDeForm),
                    TokenMatcher::Surface("も"),
                ],
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
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::NonPotentialMizen),
                    TokenMatcher::Surface("ない"),
                    TokenMatcher::Surface("で"),
                ],
                priority: 7,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::Naide,
            "n4",
        ),
        (
            GrammarPattern {
                name: "nakute_mo_ii",
                tokens: vec![
                    TokenMatcher::verb_with_form("未然形"),
                    TokenMatcher::Custom(CustomMatcher::NakuForm),
                    TokenMatcher::Surface("て"),
                    TokenMatcher::Surface("も"),
                    TokenMatcher::Custom(CustomMatcher::IiForm),
                ],
                priority: 11,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::NakuteMoIi,
            "n4",
        ),
        (
            GrammarPattern {
                name: "ba_yokatta",
                tokens: vec![
                    TokenMatcher::verb_with_form("仮定形"),
                    TokenMatcher::Surface("ば"),
                    TokenMatcher::Custom(CustomMatcher::YokattaForm),
                    TokenMatcher::Surface("た"),
                ],
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
                tokens: vec![
                    TokenMatcher::verb_with_form("連用形"),
                    TokenMatcher::Surface("なさい"),
                ],
                priority: 6,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::Nasai,
            "n4",
        ),
        (
            GrammarPattern {
                name: "hazu_desu",
                tokens: vec![
                    TokenMatcher::verb_with_form("基本形"),
                    TokenMatcher::Surface("はず"),
                    TokenMatcher::Surface("です"),
                ],
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
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm),
                    TokenMatcher::Custom(CustomMatcher::TeDeForm),
                    TokenMatcher::specific_verb("いただく"),
                ],
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
                tokens: vec![
                    TokenMatcher::verb_with_form("基本形"),
                    TokenMatcher::Surface("と"),
                    TokenMatcher::Custom(CustomMatcher::ToIiForm),
                ],
                priority: 8,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::ToIi,
            "n4",
        ),
        (
            GrammarPattern {
                name: "ga_hoshii",
                tokens: vec![TokenMatcher::Surface("が"), TokenMatcher::Surface("ほしい")],
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
                tokens: vec![
                    TokenMatcher::Surface("しか"),
                    TokenMatcher::Custom(CustomMatcher::NonNaruMizen),
                    TokenMatcher::Surface("ない"),
                ],
                priority: 8,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::ShikaNai,
            "n4",
        ),
        (
            GrammarPattern {
                name: "to_iu",
                tokens: vec![TokenMatcher::Surface("という")],
                priority: 5,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::ToIu,
            "n4",
        ),
        (
            GrammarPattern {
                name: "dictionary_to",
                tokens: vec![
                    TokenMatcher::verb_with_form("基本形"),
                    TokenMatcher::Surface("と"),
                ],
                priority: 4,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::DictionaryTo,
            "n4",
        ),
        (
            GrammarPattern {
                name: "nara",
                tokens: vec![
                    TokenMatcher::verb_with_form("基本形"),
                    TokenMatcher::Surface("なら"),
                ],
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
                tokens: vec![
                    TokenMatcher::verb_with_form("基本形"),
                    TokenMatcher::Surface("か"),
                    TokenMatcher::Surface("どう"),
                    TokenMatcher::Surface("か"),
                ],
                priority: 9,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::KaDouKa,
            "n4",
        ),
        (
            GrammarPattern {
                name: "koto_ni_suru",
                tokens: vec![
                    TokenMatcher::verb_with_form("基本形"),
                    TokenMatcher::Surface("こと"),
                    TokenMatcher::Surface("に"),
                    TokenMatcher::specific_verb("する"),
                ],
                priority: 10,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::KotoNiSuru,
            "n4",
        ),
        (
            GrammarPattern {
                name: "noni",
                tokens: vec![
                    TokenMatcher::verb_with_form("基本形"),
                    TokenMatcher::Surface("のに"),
                ],
                priority: 5,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::Noni,
            "n4",
        ),
        (
            GrammarPattern {
                name: "koto_ni_naru",
                tokens: vec![
                    TokenMatcher::verb_with_form("基本形"),
                    TokenMatcher::Surface("こと"),
                    TokenMatcher::Surface("に"),
                    TokenMatcher::specific_verb("なる"),
                ],
                priority: 10,
                category: PatternCategory::Construction,
            },
            ConjugationPattern::KotoNiNaru,
            "n4",
        ),
        // Sou desu patterns
        (
            GrammarPattern {
                name: "sou_desu_appearance",
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::SouAppearanceStem),
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
                tokens: vec![
                    TokenMatcher::verb_with_form("基本形"),
                    TokenMatcher::Surface("かも"),
                    TokenMatcher::specific_verb("しれる"),
                    TokenMatcher::Surface("ない"),
                ],
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
                tokens: vec![
                    TokenMatcher::verb_with_form("基本形"),
                    TokenMatcher::Surface("かも"),
                    TokenMatcher::specific_verb("しれる"),
                    TokenMatcher::Surface("ませ"),
                    TokenMatcher::Surface("ん"),
                ],
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
                tokens: vec![
                    TokenMatcher::verb_with_form("基本形"),
                    TokenMatcher::Surface("みたい"),
                ],
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
