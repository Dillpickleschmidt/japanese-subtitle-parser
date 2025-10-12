use crate::grammar::pattern_matcher::{CustomMatcher, GrammarPattern, TokenMatcher};
use crate::grammar::types::ConjugationPattern;

/// JLPT N3 level grammar patterns (intermediate forms)
pub fn get_patterns() -> Vec<(GrammarPattern, ConjugationPattern, &'static str)> {
    vec![
        // Batch 1: Core verb attachment patterns
        (
            GrammarPattern {
                name: "hajimeru",
                tokens: vec![
                    TokenMatcher::verb_with_form("連用形"),
                    TokenMatcher::specific_verb("始める"),
                ],
                priority: 7,
            },
            ConjugationPattern::Hajimeru,
            "n3",
        ),
        (
            GrammarPattern {
                name: "rashii",
                tokens: vec![TokenMatcher::Any, TokenMatcher::Surface("らしい")],
                priority: 5,
            },
            ConjugationPattern::Rashii,
            "n3",
        ),
        (
            GrammarPattern {
                name: "you_ni_naru",
                tokens: vec![
                    TokenMatcher::verb_with_form("基本形"),
                    TokenMatcher::Surface("よう"),
                    TokenMatcher::Surface("に"),
                    TokenMatcher::specific_verb("なる"),
                ],
                priority: 10,
            },
            ConjugationPattern::YouNiNaru,
            "n3",
        ),
        (
            GrammarPattern {
                name: "you_ni_suru",
                tokens: vec![
                    TokenMatcher::verb_with_form("基本形"),
                    TokenMatcher::Surface("よう"),
                    TokenMatcher::Surface("に"),
                    TokenMatcher::specific_verb("する"),
                ],
                priority: 10,
            },
            ConjugationPattern::YouNiSuru,
            "n3",
        ),
        (
            GrammarPattern {
                name: "tame_ni",
                tokens: vec![
                    TokenMatcher::verb_with_form("基本形"),
                    TokenMatcher::Surface("ため"),
                    TokenMatcher::Surface("に"),
                ],
                priority: 8,
            },
            ConjugationPattern::TameNi,
            "n3",
        ),
        (
            GrammarPattern {
                name: "zu",
                tokens: vec![
                    TokenMatcher::verb_with_form("未然形"),
                    TokenMatcher::Surface("ず"),
                ],
                priority: 6,
            },
            ConjugationPattern::Zu,
            "n3",
        ),
        (
            GrammarPattern {
                name: "gachi",
                tokens: vec![
                    TokenMatcher::Any, // Noun or verb stem
                    TokenMatcher::Surface("がち"),
                ],
                priority: 6,
            },
            ConjugationPattern::Gachi,
            "n3",
        ),
        (
            GrammarPattern {
                name: "ta_bakari",
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm),
                    TokenMatcher::Custom(CustomMatcher::PastAuxiliary),
                    TokenMatcher::Surface("ばかり"),
                ],
                priority: 9,
            },
            ConjugationPattern::TaBakari,
            "n3",
        ),
        (
            GrammarPattern {
                name: "ta_mono_da",
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm),
                    TokenMatcher::Custom(CustomMatcher::PastAuxiliary),
                    TokenMatcher::Surface("もの"),
                    TokenMatcher::Surface("だ"),
                ],
                priority: 10, // Higher priority now that it's more specific
            },
            ConjugationPattern::TaMonoDa,
            "n3",
        ),
        (
            GrammarPattern {
                name: "ta_mono_desu",
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm),
                    TokenMatcher::Custom(CustomMatcher::PastAuxiliary),
                    TokenMatcher::Surface("もの"),
                    TokenMatcher::Surface("です"),
                ],
                priority: 10,
            },
            ConjugationPattern::TaMonoDa,
            "n3",
        ),
        (
            GrammarPattern {
                name: "ni_chigainai",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("に"),
                    TokenMatcher::Surface("違い"),
                    TokenMatcher::Surface("ない"),
                ],
                priority: 8,
            },
            ConjugationPattern::NiChigainai,
            "n3",
        ),
        // Batch 2: Noun attachment & conditional patterns
        (
            GrammarPattern {
                name: "mama",
                tokens: vec![
                    TokenMatcher::Any, // Vた or noun
                    TokenMatcher::Surface("まま"),
                ],
                priority: 6,
            },
            ConjugationPattern::Mama,
            "n3",
        ),
        (
            GrammarPattern {
                name: "furi",
                tokens: vec![
                    TokenMatcher::Any, // Verb or noun (with の)
                    TokenMatcher::Surface("ふり"),
                ],
                priority: 6,
            },
            ConjugationPattern::Furi,
            "n3",
        ),
        (
            GrammarPattern {
                name: "nai_uchi_ni",
                tokens: vec![
                    TokenMatcher::verb_with_form("未然形"),
                    TokenMatcher::Surface("ない"),
                    TokenMatcher::Surface("うち"),
                    TokenMatcher::Surface("に"),
                ],
                priority: 10,
            },
            ConjugationPattern::NaiUchiNi,
            "n3",
        ),
        (
            GrammarPattern {
                name: "ppoi_split",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Custom(CustomMatcher::PpoiForm),
                ],
                priority: 6,
            },
            ConjugationPattern::Ppoi,
            "n3",
        ),
        (
            GrammarPattern {
                name: "ppoi_compound",
                tokens: vec![TokenMatcher::Custom(CustomMatcher::PpoiForm)],
                priority: 5, // Lower priority than split version
            },
            ConjugationPattern::Ppoi,
            "n3",
        ),
        (
            GrammarPattern {
                name: "to_shitara",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("と"),
                    TokenMatcher::Surface("し"),
                    TokenMatcher::Surface("たら"),
                ],
                priority: 8,
            },
            ConjugationPattern::ToShitara,
            "n3",
        ),
        (
            GrammarPattern {
                name: "bakari",
                tokens: vec![
                    TokenMatcher::Any, // Noun
                    TokenMatcher::Surface("ばかり"),
                ],
                priority: 5,
            },
            ConjugationPattern::Bakari,
            "n3",
        ),
        (
            GrammarPattern {
                name: "kawari",
                tokens: vec![TokenMatcher::Any, TokenMatcher::Surface("代わり")],
                priority: 6,
            },
            ConjugationPattern::Kawari,
            "n3",
        ),
        (
            GrammarPattern {
                name: "okage_de",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("おかげ"),
                    TokenMatcher::Surface("で"),
                ],
                priority: 7,
            },
            ConjugationPattern::OkageDe,
            "n3",
        ),
        (
            GrammarPattern {
                name: "sae",
                tokens: vec![
                    TokenMatcher::Any, // Noun
                    TokenMatcher::Surface("さえ"),
                ],
                priority: 5,
            },
            ConjugationPattern::Sae,
            "n3",
        ),
        (
            GrammarPattern {
                name: "you_ni_standalone",
                tokens: vec![
                    TokenMatcher::verb_with_form("基本形"),
                    TokenMatcher::Surface("よう"),
                    TokenMatcher::Surface("に"),
                ],
                priority: 7, // Lower than you_ni_suru/naru so those match first
            },
            ConjugationPattern::YouNi,
            "n3",
        ),
        // Batch 3: Adverbs, suffixes, and particle patterns
        (
            GrammarPattern {
                name: "masaka",
                tokens: vec![TokenMatcher::Surface("まさか")],
                priority: 5,
            },
            ConjugationPattern::Masaka,
            "n3",
        ),
        (
            GrammarPattern {
                name: "mushiro",
                tokens: vec![TokenMatcher::Surface("むしろ")],
                priority: 5,
            },
            ConjugationPattern::Mushiro,
            "n3",
        ),
        (
            GrammarPattern {
                name: "sudeni",
                tokens: vec![TokenMatcher::Surface("すでに")],
                priority: 5,
            },
            ConjugationPattern::Sudeni,
            "n3",
        ),
        (
            GrammarPattern {
                name: "tsui",
                tokens: vec![TokenMatcher::Surface("つい")],
                priority: 5,
            },
            ConjugationPattern::Tsui,
            "n3",
        ),
        (
            GrammarPattern {
                name: "doushitemo",
                tokens: vec![TokenMatcher::Surface("どうしても")],
                priority: 5,
            },
            ConjugationPattern::Doushitemo,
            "n3",
        ),
        (
            GrammarPattern {
                name: "teki_suffix",
                tokens: vec![
                    TokenMatcher::Any, // Noun
                    TokenMatcher::Custom(CustomMatcher::TekiSuffix),
                ],
                priority: 6,
            },
            ConjugationPattern::Teki,
            "n3",
        ),
        (
            GrammarPattern {
                name: "tate_suffix",
                tokens: vec![
                    TokenMatcher::Any, // Often tagged as noun (焼き)
                    TokenMatcher::Custom(CustomMatcher::TateSuffix),
                ],
                priority: 6,
            },
            ConjugationPattern::Tate,
            "n3",
        ),
        (
            GrammarPattern {
                name: "ni_yotte",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("によって"), // Tokenizes as single particle
                ],
                priority: 7,
            },
            ConjugationPattern::NiYotte,
            "n3",
        ),
        // kiri: きり (only/since) - matches after past auxiliary or noun
        // Naturally excludes particles (じゃ, は, たら) before きり
        (
            GrammarPattern {
                name: "kiri_past",
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::PastAuxiliary),
                    TokenMatcher::Surface("きり"),
                ],
                priority: 7,
            },
            ConjugationPattern::Kiri,
            "n3",
        ),
        (
            GrammarPattern {
                name: "kiri_noun",
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::Noun),
                    TokenMatcher::Surface("きり"),
                ],
                priority: 6,
            },
            ConjugationPattern::Kiri,
            "n3",
        ),
        // Batch 4: Additional intermediate patterns
        (
            GrammarPattern {
                name: "gurai",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Custom(CustomMatcher::GuraiForm),
                ],
                priority: 5,
            },
            ConjugationPattern::Gurai,
            "n3",
        ),
        (
            GrammarPattern {
                name: "ni_yoru_to",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("に"),
                    TokenMatcher::specific_verb("よる"),
                    TokenMatcher::Surface("と"),
                ],
                priority: 8,
            },
            ConjugationPattern::NiYoruTo,
            "n3",
        ),
        (
            GrammarPattern {
                name: "toshite",
                tokens: vec![TokenMatcher::Any, TokenMatcher::Surface("として")],
                priority: 6,
            },
            ConjugationPattern::Toshite,
            "n3",
        ),
        (
            GrammarPattern {
                name: "suginai",
                tokens: vec![
                    TokenMatcher::Any, // Noun or verb stem
                    TokenMatcher::Surface("過ぎ"),
                    TokenMatcher::Surface("ない"),
                ],
                priority: 7,
            },
            ConjugationPattern::Suginai,
            "n3",
        ),
        (
            GrammarPattern {
                name: "oite_compound",
                tokens: vec![TokenMatcher::Any, TokenMatcher::Surface("において")],
                priority: 7,
            },
            ConjugationPattern::Oite,
            "n3",
        ),
        (
            GrammarPattern {
                name: "oite_split",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Custom(CustomMatcher::OiteForm),
                    TokenMatcher::Surface("て"),
                ],
                priority: 7,
            },
            ConjugationPattern::Oite,
            "n3",
        ),
        (
            GrammarPattern {
                name: "tsumori_de",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("つもり"),
                    TokenMatcher::Surface("で"),
                ],
                priority: 7,
            },
            ConjugationPattern::TsumorideDe,
            "n3",
        ),
        (
            GrammarPattern {
                name: "ni_kansuru",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Custom(CustomMatcher::NiKansuruForm),
                ],
                priority: 7,
            },
            ConjugationPattern::NiKansuru,
            "n3",
        ),
        (
            GrammarPattern {
                name: "to_tomoni",
                tokens: vec![TokenMatcher::Any, TokenMatcher::Surface("とともに")],
                priority: 7,
            },
            ConjugationPattern::ToTomoni,
            "n3",
        ),
        (
            GrammarPattern {
                name: "te_hajimete",
                tokens: vec![
                    TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm),
                    TokenMatcher::Surface("て"),
                    TokenMatcher::Custom(CustomMatcher::HajimeteAdverb),
                ],
                priority: 8,
            },
            ConjugationPattern::TeHajimete,
            "n3",
        ),
        (
            GrammarPattern {
                name: "seizei",
                tokens: vec![TokenMatcher::Surface("せいぜい")],
                priority: 5,
            },
            ConjugationPattern::Seizei,
            "n3",
        ),
        (
            GrammarPattern {
                name: "wo_hajime",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("を"),
                    TokenMatcher::Surface("始め"),
                ],
                priority: 7,
            },
            ConjugationPattern::WoHajime,
            "n3",
        ),
        (
            GrammarPattern {
                name: "ba_hodo",
                tokens: vec![
                    TokenMatcher::verb_with_form("仮定形"),
                    TokenMatcher::Surface("ば"),
                    TokenMatcher::Any, // Often the same verb in dictionary form or another clause
                    TokenMatcher::Surface("ほど"),
                ],
                priority: 8,
            },
            ConjugationPattern::BaHodo,
            "n3",
        ),
        // Batch 5: Final N3 patterns (adverbs, particles, advanced forms)
        (
            GrammarPattern {
                name: "douyara",
                tokens: vec![TokenMatcher::Surface("どうやら")],
                priority: 5,
            },
            ConjugationPattern::Douyara,
            "n3",
        ),
        (
            GrammarPattern {
                name: "kaette",
                tokens: vec![TokenMatcher::Surface("かえって")],
                priority: 5,
            },
            ConjugationPattern::Kaette,
            "n3",
        ),
        (
            GrammarPattern {
                name: "sae_ba",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("さえ"),
                    TokenMatcher::verb_with_form("仮定形"),
                    TokenMatcher::Surface("ば"),
                ],
                priority: 9,
            },
            ConjugationPattern::SaeBa,
            "n3",
        ),
        (
            GrammarPattern {
                name: "koso",
                tokens: vec![TokenMatcher::Any, TokenMatcher::Surface("こそ")],
                priority: 6,
            },
            ConjugationPattern::Koso,
            "n3",
        ),
        (
            GrammarPattern {
                name: "sarani",
                tokens: vec![TokenMatcher::Surface("さらに")],
                priority: 5,
            },
            ConjugationPattern::Sarani,
            "n3",
        ),
        (
            GrammarPattern {
                name: "mai",
                tokens: vec![
                    TokenMatcher::verb_with_form("基本形"), // Or 未然形
                    TokenMatcher::Custom(CustomMatcher::MaiForm),
                ],
                priority: 7,
            },
            ConjugationPattern::Mai,
            "n3",
        ),
        (
            GrammarPattern {
                name: "wazawaza",
                tokens: vec![TokenMatcher::Surface("わざわざ")],
                priority: 5,
            },
            ConjugationPattern::Wazawaza,
            "n3",
        ),
        (
            GrammarPattern {
                name: "kagiru",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("に"),
                    TokenMatcher::specific_verb("限る"),
                ],
                priority: 7,
            },
            ConjugationPattern::Kagiru,
            "n3",
        ),
        (
            GrammarPattern {
                name: "nakanaka",
                tokens: vec![TokenMatcher::Surface("なかなか")],
                priority: 5,
            },
            ConjugationPattern::Nakanaka,
            "n3",
        ),
        (
            GrammarPattern {
                name: "ittai",
                tokens: vec![TokenMatcher::Surface("いったい")],
                priority: 5,
            },
            ConjugationPattern::Ittai,
            "n3",
        ),
    ]
}
