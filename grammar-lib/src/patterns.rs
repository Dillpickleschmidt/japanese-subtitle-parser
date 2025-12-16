use crate::pattern_matcher::{GrammarPattern, PatternCategory, PatternMatcher};

macro_rules! declare_patterns {
    (
        $(
            $variant:ident {
                name: $name:expr,
                matcher_fn: $matcher_fn:path,
                priority: $priority:expr,
                category: $category:path,
                jlpt: $jlpt:expr $(,)?
            }
        ),* $(,)?
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Pattern {
            $($variant,)*
        }

        impl Pattern {
            pub fn grammar_pattern(&self) -> GrammarPattern {
                match self {
                    $(Pattern::$variant => GrammarPattern {
                        name: $name,
                        tokens: $matcher_fn(),
                        priority: $priority,
                        category: $category,
                        jlpt_level: $jlpt,
                    },)*
                }
            }

            pub fn all() -> Vec<Pattern> {
                vec![$(Pattern::$variant,)*]
            }
        }
    }
}

declare_patterns! {
    // ========== N5 PATTERNS ==========

    Da {
        name: "だ",
        matcher_fn: crate::matchers::n5::da,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Desu {
        name: "です",
        matcher_fn: crate::matchers::n5::desu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Ha {
        name: "は",
        matcher_fn: crate::matchers::n5::ha,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Mo {
        name: "も",
        matcher_fn: crate::matchers::n5::mo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    ToAnd {
        name: "と",
        matcher_fn: crate::matchers::n5::to,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Kore {
        name: "これ",
        matcher_fn: crate::matchers::n5::kore,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Sore {
        name: "それ",
        matcher_fn: crate::matchers::n5::sore,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Are {
        name: "あれ",
        matcher_fn: crate::matchers::n5::are,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    No {
        name: "の",
        matcher_fn: crate::matchers::n5::no,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Ii {
        name: "いい",
        matcher_fn: crate::matchers::n5::ii,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    IAdjectives {
        name: "い-Adjectives",
        matcher_fn: crate::matchers::n5::i_adjectives,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    NaAdjectives {
        name: "な-Adjectives",
        matcher_fn: crate::matchers::n5::na_adjectives,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Ka {
        name: "か",
        matcher_fn: crate::matchers::n5::ka,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Ga {
        name: "が",
        matcher_fn: crate::matchers::n5::ga,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Yo {
        name: "よ",
        matcher_fn: crate::matchers::n5::yo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Ne {
        name: "ね",
        matcher_fn: crate::matchers::n5::ne,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    RuVerbDictionary {
        name: "る-Verb (Dictionary)",
        matcher_fn: crate::matchers::n5::ru_verb_dictionary,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    UVerbDictionary {
        name: "う-Verb (Dictionary)",
        matcher_fn: crate::matchers::n5::u_verb_dictionary,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Wo {
        name: "を",
        matcher_fn: crate::matchers::n5::wo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Masu {
        name: "ます",
        matcher_fn: crate::matchers::n5::masu,
        priority: 1,
        category: PatternCategory::Conjugation,
        jlpt: "n5",
    },

    RuVerbNegative {
        name: "る-Verb (Negative)",
        matcher_fn: crate::matchers::n5::ru_verb_negative,
        priority: 1,
        category: PatternCategory::Conjugation,
        jlpt: "n5",
    },

    UVerbNegative {
        name: "う-Verb (Negative)",
        matcher_fn: crate::matchers::n5::u_verb_negative,
        priority: 1,
        category: PatternCategory::Conjugation,
        jlpt: "n5",
    },

    Koko {
        name: "ここ",
        matcher_fn: crate::matchers::n5::koko,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Soko {
        name: "そこ",
        matcher_fn: crate::matchers::n5::soko,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Asoko {
        name: "あそこ",
        matcher_fn: crate::matchers::n5::asoko,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    De {
        name: "で",
        matcher_fn: crate::matchers::n5::de,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Ni {
        name: "に",
        matcher_fn: crate::matchers::n5::ni,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Deshou {
        name: "でしょう",
        matcher_fn: crate::matchers::n5::deshou,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Darou {
        name: "だろう",
        matcher_fn: crate::matchers::n5::darou,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Gaaru {
        name: "がある",
        matcher_fn: crate::matchers::n5::gaaru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Gairu {
        name: "がいる",
        matcher_fn: crate::matchers::n5::gairu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Kono {
        name: "この",
        matcher_fn: crate::matchers::n5::kono,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Sono {
        name: "その",
        matcher_fn: crate::matchers::n5::sono,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Ano {
        name: "あの",
        matcher_fn: crate::matchers::n5::ano,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Uff5eNdesuU30fbNodesu {
        name: "～んです・のです",
        matcher_fn: crate::matchers::n5::uff5e_ndesu_u30fb_nodesu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    IAdjectivePast {
        name: "い-Adjective (Past)",
        matcher_fn: crate::matchers::n5::i_adjective_past,
        priority: 1,
        category: PatternCategory::Conjugation,
        jlpt: "n5",
    },

    IAdjectiveNoun {
        name: "い-Adjective + Noun",
        matcher_fn: crate::matchers::n5::i_adjective_noun,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    NaAdjectiveNoun {
        name: "な-Adjective + Noun",
        matcher_fn: crate::matchers::n5::na_adjective_noun,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Heiku {
        name: "へいく",
        matcher_fn: crate::matchers::n5::heiku,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Suru {
        name: "する",
        matcher_fn: crate::matchers::n5::suru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    NiSuru {
        name: "にする",
        matcher_fn: crate::matchers::n5::ni_suru,
        priority: 2,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    MaeNi {
        name: "まえに",
        matcher_fn: crate::matchers::n5::mae_ni,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Nanika {
        name: "なにか",
        matcher_fn: crate::matchers::n5::nanika,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Nanimo {
        name: "なにも",
        matcher_fn: crate::matchers::n5::nanimo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    NakutewaNaranai {
        name: "なくてはならない",
        matcher_fn: crate::matchers::n5::nakutewa_naranai,
        priority: 6,
        category: PatternCategory::Conjugation,
        jlpt: "n5",
    },

    Kuru {
        name: "くる",
        matcher_fn: crate::matchers::n5::kuru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    RuVerbPast {
        name: "る-Verb (Past)",
        matcher_fn: crate::matchers::n5::ru_verb_past,
        priority: 1,
        category: PatternCategory::Conjugation,
        jlpt: "n5",
    },

    UVerbPast {
        name: "う-Verb (Past)",
        matcher_fn: crate::matchers::n5::u_verb_past,
        priority: 1,
        category: PatternCategory::Conjugation,
        jlpt: "n5",
    },

    Suki {
        name: "好き",
        matcher_fn: crate::matchers::n5::suki,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Kirai {
        name: "きらい",
        matcher_fn: crate::matchers::n5::kirai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Nogasuki {
        name: "のがすき",
        matcher_fn: crate::matchers::n5::nogasuki,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    GaaruNoun {
        name: "がある + Noun",
        matcher_fn: crate::matchers::n5::gaaru_noun,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    IAdjectivesKunai {
        name: "い-Adjectives くない",
        matcher_fn: crate::matchers::n5::i_adjectives_kunai,
        priority: 1,
        category: PatternCategory::Conjugation,
        jlpt: "n5",
    },

    Node {
        name: "ので",
        matcher_fn: crate::matchers::n5::node,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Kara {
        name: "から",
        matcher_fn: crate::matchers::n5::kara,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    KedoU30fbDakedo {
        name: "けど・だけど",
        matcher_fn: crate::matchers::n5::kedo_u30fb_dakedo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    RuVerbNegativePast {
        name: "る-Verb (Negative-Past)",
        matcher_fn: crate::matchers::n5::ru_verb_negative_past,
        priority: 1,
        category: PatternCategory::Conjugation,
        jlpt: "n5",
    },

    UVerbNegativePast {
        name: "う-Verb (Negative-Past)",
        matcher_fn: crate::matchers::n5::u_verb_negative_past,
        priority: 1,
        category: PatternCategory::Conjugation,
        jlpt: "n5",
    },

    VerbTe {
        name: "Verb + て",
        matcher_fn: crate::matchers::n5::verb_te,
        priority: 1,
        category: PatternCategory::Conjugation,
        jlpt: "n5",
    },

    TeiruU2460 {
        name: "ている①",
        matcher_fn: crate::matchers::n5::teiru_u2460,
        priority: 1,
        category: PatternCategory::Conjugation,
        jlpt: "n5",
    },

    He {
        name: "へ",
        matcher_fn: crate::matchers::n5::he,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    VerbNiiku {
        name: "Verb + にいく",
        matcher_fn: crate::matchers::n5::verb_niiku,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Dare {
        name: "誰",
        matcher_fn: crate::matchers::n5::dare,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    IAdjectivePredicate {
        name: "い-Adjective (Predicate)",
        matcher_fn: crate::matchers::n5::i_adjective_predicate,
        priority: 1,
        category: PatternCategory::Conjugation,
        jlpt: "n5",
    },

    NaAdjectiveDa {
        name: "な-Adjective だ",
        matcher_fn: crate::matchers::n5::na_adjective_da,
        priority: 1,
        category: PatternCategory::Conjugation,
        jlpt: "n5",
    },

    DattaU30fbDeshita {
        name: "だった・でした",
        matcher_fn: crate::matchers::n5::datta_u30fb_deshita,
        priority: 1,
        category: PatternCategory::Conjugation,
        jlpt: "n5",
    },

    Janai {
        name: "じゃない",
        matcher_fn: crate::matchers::n5::janai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Janakatta {
        name: "じゃなかった",
        matcher_fn: crate::matchers::n5::janakatta,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    IAdjectiveKunakatta {
        name: "い-Adjective くなかった",
        matcher_fn: crate::matchers::n5::i_adjective_kunakatta,
        priority: 1,
        category: PatternCategory::Conjugation,
        jlpt: "n5",
    },

    VerbsNonPast {
        name: "Verbs (Non-past)",
        matcher_fn: crate::matchers::n5::verbs_non_past,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    VerbUff3bTaU30fbTeiruUff3dNoun {
        name: "Verb［た・ている］+ Noun",
        matcher_fn: crate::matchers::n5::verb_uff3b_ta_u30fb_teiru_uff3d_noun,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Na {
        name: "な",
        matcher_fn: crate::matchers::n5::na,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Dake {
        name: "だけ",
        matcher_fn: crate::matchers::n5::dake,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Dore {
        name: "どれ",
        matcher_fn: crate::matchers::n5::dore,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Doko {
        name: "どこ",
        matcher_fn: crate::matchers::n5::doko,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Dono {
        name: "どの",
        matcher_fn: crate::matchers::n5::dono,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    TeiruU2461 {
        name: "ている②",
        matcher_fn: crate::matchers::n5::teiru_u2461,
        priority: 1,
        category: PatternCategory::Conjugation,
        jlpt: "n5",
    },

    Tekara {
        name: "てから",
        matcher_fn: crate::matchers::n5::tekara,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    VerbTeB {
        name: "Verb + て+ B",
        matcher_fn: crate::matchers::n5::verb_te_b,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Mou {
        name: "もう",
        matcher_fn: crate::matchers::n5::mou,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Mada {
        name: "まだ",
        matcher_fn: crate::matchers::n5::mada,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    MadaUff5eTeimasen {
        name: "まだ～ていません",
        matcher_fn: crate::matchers::n5::mada_uff5e_teimasen,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Temoii {
        name: "てもいい",
        matcher_fn: crate::matchers::n5::temoii,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Tai {
        name: "たい",
        matcher_fn: crate::matchers::n5::tai,
        priority: 1,
        category: PatternCategory::Conjugation,
        jlpt: "n5",
    },

    TariUff5eTarisuru {
        name: "たり～たりする",
        matcher_fn: crate::matchers::n5::tari_uff5e_tarisuru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Kekkou {
        name: "けっこう",
        matcher_fn: crate::matchers::n5::kekkou,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Takusan {
        name: "たくさん",
        matcher_fn: crate::matchers::n5::takusan,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Maeni {
        name: "まえに",
        matcher_fn: crate::matchers::n5::maeni,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    KuraiU2460 {
        name: "くらい ①",
        matcher_fn: crate::matchers::n5::kurai_u2460,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    NounMade {
        name: "Noun + まで",
        matcher_fn: crate::matchers::n5::noun_made,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    VerbMade {
        name: "Verb + まで",
        matcher_fn: crate::matchers::n5::verb_made,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Sugiru {
        name: "すぎる",
        matcher_fn: crate::matchers::n5::sugiru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Nisuru {
        name: "にする",
        matcher_fn: crate::matchers::n5::nisuru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Uff5eNinaruU30fbUff5eKunaru {
        name: "～になる・～くなる",
        matcher_fn: crate::matchers::n5::uff5e_ninaru_u30fb_uff5e_kunaru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    NonakadeUff5eGaichibanUff5e {
        name: "のなかで～がいちばん～",
        matcher_fn: crate::matchers::n5::nonakade_uff5e_gaichiban_uff5e,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    YoriUff5eNohouga {
        name: "より～のほうが",
        matcher_fn: crate::matchers::n5::yori_uff5e_nohouga,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    NanikaU30fbNanimo {
        name: "なにか・なにも",
        matcher_fn: crate::matchers::n5::nanika_u30fb_nanimo,
        priority: 1,
        category: PatternCategory::Construction, // only to combine these tokens
        jlpt: "n5",
    },

    DarekaU30fbDokokaU30fbDaremoU30fbDokomo {
        name: "誰か・どこか・誰も・どこも",
        matcher_fn: crate::matchers::n5::dareka_u30fb_dokoka_u30fb_daremo_u30fb_dokomo,
        priority: 1,
        category: PatternCategory::Conjugation, // only to combine these tokens
        jlpt: "n5",
    },

    Mashou {
        name: "ましょう",
        matcher_fn: crate::matchers::n5::mashou,
        priority: 1,
        category: PatternCategory::Conjugation,
        jlpt: "n5",
    },

    Uff5eMashouka {
        name: "～ましょうか",
        matcher_fn: crate::matchers::n5::uff5e_mashouka,
        priority: 1,
        category: PatternCategory::Conjugation,
        jlpt: "n5",
    },

    Masenka {
        name: "ませんか",
        matcher_fn: crate::matchers::n5::masenka,
        priority: 1,
        category: PatternCategory::Conjugation,
        jlpt: "n5",
    },

    VerbTemoii {
        name: "Verb + てもいい",
        matcher_fn: crate::matchers::n5::verb_temoii,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Tekudasai {
        name: "てください",
        matcher_fn: crate::matchers::n5::tekudasai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Naidekudasai {
        name: "ないでください",
        matcher_fn: crate::matchers::n5::naidekudasai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Tehaikenai {
        name: "てはいけない",
        matcher_fn: crate::matchers::n5::tehaikenai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Nakutehaikenai {
        name: "なくてはいけない",
        matcher_fn: crate::matchers::n5::nakutehaikenai,
        priority: 1,
        category: PatternCategory::Conjugation,
        jlpt: "n5",
    },

    Nakutehanaranai {
        name: "なくてはならない",
        matcher_fn: crate::matchers::n5::nakutehanaranai,
        priority: 1,
        category: PatternCategory::Conjugation,
        jlpt: "n5",
    },

    Tahougaii {
        name: "たほうがいい",
        matcher_fn: crate::matchers::n5::tahougaii,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Naihougaii {
        name: "ないほうがいい",
        matcher_fn: crate::matchers::n5::naihougaii,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    NakuchaU30fbNakya {
        name: "なくちゃ・なきゃ",
        matcher_fn: crate::matchers::n5::nakucha_u30fb_nakya,
        priority: 1,
        category: PatternCategory::Conjugation,
        jlpt: "n5",
    },

    NaProhibitive {
        name: "な",
        matcher_fn: crate::matchers::n5::na_prohibitive,
        priority: 1,
        category: PatternCategory::Conjugation,
        jlpt: "n5",
    },

    Ya {
        name: "や",
        matcher_fn: crate::matchers::n5::ya,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Takotogaaru {
        name: "たことがある",
        matcher_fn: crate::matchers::n5::takotogaaru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    TeiruU2462 {
        name: "ている③",
        matcher_fn: crate::matchers::n5::teiru_u2462,
        priority: 1,
        category: PatternCategory::Conjugation,
        jlpt: "n5",
    },

    AdjectiveTeU30fbNounDe {
        name: "Adjective + て・Noun + で",
        matcher_fn: crate::matchers::n5::adjective_te_u30fb_noun_de,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    AdjectiveTeB {
        name: "Adjective + て + B",
        matcher_fn: crate::matchers::n5::adjective_te_b,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Nogaheta {
        name: "のがへた",
        matcher_fn: crate::matchers::n5::nogaheta,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Nogajouzu {
        name: "のがじょうず",
        matcher_fn: crate::matchers::n5::nogajouzu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    NonakadeGaIchiban {
        name: "のなかで～がいちばん～",
        matcher_fn: crate::matchers::n5::nonakade_ga_ichiban,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    AdjectiveNoHa {
        name: "Adjective + の(は)",
        matcher_fn: crate::matchers::n5::adjective_no_ha,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Ageru {
        name: "あげる",
        matcher_fn: crate::matchers::n5::ageru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Kureru {
        name: "くれる",
        matcher_fn: crate::matchers::n5::kureru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Morau {
        name: "もらう",
        matcher_fn: crate::matchers::n5::morau,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Keredomo {
        name: "けれども",
        matcher_fn: crate::matchers::n5::keredomo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    Tsumorida {
        name: "つもりだ",
        matcher_fn: crate::matchers::n5::tsumorida,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    NiNaruKuNaru {
        name: "～になる・～くなる",
        matcher_fn: crate::matchers::n5::ni_naru_ku_naru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n5",
    },

    // ========== N4 PATTERNS ==========

    To {
        name: "と",
        matcher_fn: crate::matchers::n4::to,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    No1 {
        name: "の1",
        matcher_fn: crate::matchers::n4::no1,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    To2 {
        name: "と2",
        matcher_fn: crate::matchers::n4::to2,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Demo {
        name: "でも",
        matcher_fn: crate::matchers::n4::demo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Yasui {
        name: "やすい",
        matcher_fn: crate::matchers::n4::yasui,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Nikui {
        name: "にくい",
        matcher_fn: crate::matchers::n4::nikui,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Dandan {
        name: "だんだん",
        matcher_fn: crate::matchers::n4::dandan,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Dondon {
        name: "どんどん",
        matcher_fn: crate::matchers::n4::dondon,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Uff5eRa {
        name: "～ら",
        matcher_fn: crate::matchers::n4::uff5e_ra,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Teiku {
        name: "ていく",
        matcher_fn: crate::matchers::n4::teiku,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Tekuru {
        name: "てくる ",
        matcher_fn: crate::matchers::n4::tekuru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Kata {
        name: "かた",
        matcher_fn: crate::matchers::n4::kata,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    KataShikata {
        name: "かた",
        matcher_fn: crate::matchers::n4::kata_shikata,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Dakede {
        name: "だけで",
        matcher_fn: crate::matchers::n4::dakede,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    DagaU30fbDesuga {
        name: "だが・ですが",
        matcher_fn: crate::matchers::n4::daga_u30fb_desuga,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Nakute {
        name: "なくて",
        matcher_fn: crate::matchers::n4::nakute,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Naide {
        name: "ないで",
        matcher_fn: crate::matchers::n4::naide,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    VerbUff3bReruU30fbRareruUff3d {
        name: "Verb［れる・られる］",
        matcher_fn: crate::matchers::n4::verb_uff3b_reru_u30fb_rareru_uff3d,
        priority: 1,
        category: PatternCategory::Conjugation,
        jlpt: "n4",
    },

    TadoushiU30fbJidoushi {
        name: "他動詞・自動詞",
        matcher_fn: crate::matchers::n4::tadoushi_u30fb_jidoushi,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Naosu {
        name: "なおす",
        matcher_fn: crate::matchers::n4::naosu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Toiukoto {
        name: "ということ",
        matcher_fn: crate::matchers::n4::toiukoto,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Toki {
        name: "とき",
        matcher_fn: crate::matchers::n4::toki,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Mazu {
        name: "まず",
        matcher_fn: crate::matchers::n4::mazu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Made {
        name: "まで",
        matcher_fn: crate::matchers::n4::made,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Madeni {
        name: "までに",
        matcher_fn: crate::matchers::n4::madeni,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Mata {
        name: "また",
        matcher_fn: crate::matchers::n4::mata,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Hajimeru {
        name: "はじめる",
        matcher_fn: crate::matchers::n4::hajimeru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Owaru {
        name: "おわる",
        matcher_fn: crate::matchers::n4::owaru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Goro {
        name: "ごろ",
        matcher_fn: crate::matchers::n4::goro,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Atode {
        name: "あとで",
        matcher_fn: crate::matchers::n4::atode,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Teita {
        name: "ていた ",
        matcher_fn: crate::matchers::n4::teita,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    NiFrequency {
        name: "に (Frequency)",
        matcher_fn: crate::matchers::n4::ni_frequency,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Toutou {
        name: "とうとう",
        matcher_fn: crate::matchers::n4::toutou,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Yori {
        name: "より",
        matcher_fn: crate::matchers::n4::yori,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Gotoni {
        name: "ごとに",
        matcher_fn: crate::matchers::n4::gotoni,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Narubeku {
        name: "なるべく",
        matcher_fn: crate::matchers::n4::narubeku,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Rutokoroda {
        name: "るところだ",
        matcher_fn: crate::matchers::n4::rutokoroda,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Noni {
        name: "のに ",
        matcher_fn: crate::matchers::n4::noni,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Toomou {
        name: "とおもう",
        matcher_fn: crate::matchers::n4::toomou,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    ToIwareteiru {
        name: "といわれている",
        matcher_fn: crate::matchers::n4::to_iwareteiru,
        priority: 2,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    ToSareteiru {
        name: "とされている",
        matcher_fn: crate::matchers::n4::to_sareteiru,
        priority: 2,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Nado {
        name: "など",
        matcher_fn: crate::matchers::n4::nado,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Mitai {
        name: "みたい",
        matcher_fn: crate::matchers::n4::mitai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Koto {
        name: "こと",
        matcher_fn: crate::matchers::n4::koto,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Sou {
        name: "そう ",
        matcher_fn: crate::matchers::n4::sou,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Sa {
        name: "さ",
        matcher_fn: crate::matchers::n4::sa,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    TokaUff5eToka {
        name: "とか～とか",
        matcher_fn: crate::matchers::n4::toka_uff5e_toka,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Souiu {
        name: "そういう",
        matcher_fn: crate::matchers::n4::souiu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    VerbYou {
        name: "Verb[よう]",
        matcher_fn: crate::matchers::n4::verb_you,
        priority: 1,
        category: PatternCategory::Conjugation,
        jlpt: "n4",
    },

    Youda {
        name: "ようだ",
        matcher_fn: crate::matchers::n4::youda,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Zenzen {
        name: "ぜんぜん",
        matcher_fn: crate::matchers::n4::zenzen,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Kana {
        name: "かな",
        matcher_fn: crate::matchers::n4::kana,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    AmariUff5eNai {
        name: "あまり～ない",
        matcher_fn: crate::matchers::n4::amari_uff5e_nai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Ba {
        name: "ば",
        matcher_fn: crate::matchers::n4::ba,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Nara {
        name: "なら",
        matcher_fn: crate::matchers::n4::nara,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    GaruCompound {
        name: "がる",
        matcher_fn: crate::matchers::n4::garu_compound,
        priority: 3,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    GaruSplit {
        name: "がる",
        matcher_fn: crate::matchers::n4::garu_split,
        priority: 3,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Gasuru {
        name: "がする",
        matcher_fn: crate::matchers::n4::gasuru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Tagaru {
        name: "たがる",
        matcher_fn: crate::matchers::n4::tagaru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Kamoshirenai {
        name: "かもしれない",
        matcher_fn: crate::matchers::n4::kamoshirenai,
        priority: 1,
        category: PatternCategory::Conjugation, // only to combine these tokens
        jlpt: "n4",
    },

    MitainiU30fbMitaina {
        name: "みたいに・みたいな",
        matcher_fn: crate::matchers::n4::mitaini_u30fb_mitaina,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    SouniU30fbSouna {
        name: "そうに・そうな ",
        matcher_fn: crate::matchers::n4::souni_u30fb_souna,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    NoyouniU30fbNoyouna {
        name: "のように・のような ",
        matcher_fn: crate::matchers::n4::noyouni_u30fb_noyouna,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    U301cYoutoomouU30fbU301cOutoomou {
        name: "〜ようと思う・〜おうと思う",
        matcher_fn: crate::matchers::n4::u301c_youtoomou_u30fb_u301c_outoomou,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    KuU30fbNi {
        name: "く・に",
        matcher_fn: crate::matchers::n4::ku_u30fb_ni,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Uff5eNisuruU30fbUff5eKusuru {
        name: "～にする・～くする",
        matcher_fn: crate::matchers::n4::uff5e_nisuru_u30fb_uff5e_kusuru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Toii {
        name: "といい",
        matcher_fn: crate::matchers::n4::toii,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    ToIttemoii {
        name: "といってもいい",
        matcher_fn: crate::matchers::n4::to_itte_mo_ii,
        priority: 2,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Youninaru {
        name: "ようになる",
        matcher_fn: crate::matchers::n4::youninaru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    MaiUff5eNoyouni {
        name: "まい～のように",
        matcher_fn: crate::matchers::n4::mai_uff5e_noyouni,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Janaika {
        name: "じゃないか",
        matcher_fn: crate::matchers::n4::janaika,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    RashiiU2460 {
        name: "らしい ①",
        matcher_fn: crate::matchers::n4::rashii_u2460,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Teoku {
        name: "ておく",
        matcher_fn: crate::matchers::n4::teoku,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Gahoshii {
        name: "がほしい",
        matcher_fn: crate::matchers::n4::gahoshii,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Tehoshii {
        name: "てほしい",
        matcher_fn: crate::matchers::n4::tehoshii,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Tokiita {
        name: "ときいた",
        matcher_fn: crate::matchers::n4::tokiita,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Kikoeru {
        name: "聞こえる",
        matcher_fn: crate::matchers::n4::kikoeru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Mieru {
        name: "見える",
        matcher_fn: crate::matchers::n4::mieru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Dasu {
        name: "だす",
        matcher_fn: crate::matchers::n4::dasu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Uff5eDai {
        name: "～代",
        matcher_fn: crate::matchers::n4::uff5e_dai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    NumberMo {
        name: "Number + も",
        matcher_fn: crate::matchers::n4::number_mo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Hotondo {
        name: "ほとんど",
        matcher_fn: crate::matchers::n4::hotondo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    SonnaU30fbKonnaU30fbAnnaU30fbDonna {
        name: "そんな・こんな・あんな・どんな",
        matcher_fn: crate::matchers::n4::sonna_u30fb_konna_u30fb_anna_u30fb_donna,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Kaku {
        name: "各",
        matcher_fn: crate::matchers::n4::kaku,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    IjouU2460 {
        name: "以上 ①",
        matcher_fn: crate::matchers::n4::ijou_u2460,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Ika {
        name: "いか",
        matcher_fn: crate::matchers::n4::ika,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Igai {
        name: "いがい",
        matcher_fn: crate::matchers::n4::igai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    ZuttoU2460 {
        name: "ずっと ①",
        matcher_fn: crate::matchers::n4::zutto_u2460,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Daitai {
        name: "だいたい",
        matcher_fn: crate::matchers::n4::daitai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Nonakade {
        name: "のなかで",
        matcher_fn: crate::matchers::n4::nonakade,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    YouniU30fbYouna {
        name: "ように・ような",
        matcher_fn: crate::matchers::n4::youni_u30fb_youna,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    NumberAmountHa {
        name: "Number/Amount + は",
        matcher_fn: crate::matchers::n4::number_amount_ha,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    NanCounterKa {
        name: "なん + counter + か",
        matcher_fn: crate::matchers::n4::nan_counter_ka,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Ma {
        name: "真(っ)",
        matcher_fn: crate::matchers::n4::ma,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    NumberShikaU301cNai {
        name: "Number + しか〜ない",
        matcher_fn: crate::matchers::n4::number_shika_u301c_nai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Uff5eHaUff5eNohitotsuda {
        name: "～は～の一つだ",
        matcher_fn: crate::matchers::n4::uff5e_ha_uff5e_nohitotsuda,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Uff5eNaiUff5eHanai {
        name: "～ない～はない",
        matcher_fn: crate::matchers::n4::uff5e_nai_uff5e_hanai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    SukoshimoUff5eNai {
        name: "すこしも～ない",
        matcher_fn: crate::matchers::n4::sukoshimo_uff5e_nai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Sukunakunai {
        name: "すくなくない",
        matcher_fn: crate::matchers::n4::sukunakunai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    SukunakunaiPolite {
        name: "すくなくない_polite",
        matcher_fn: crate::matchers::n4::sukunakunai_polite,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Baaiha {
        name: "ばあいは",
        matcher_fn: crate::matchers::n4::baaiha,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    VerbTe2 {
        name: "Verb[て]",
        matcher_fn: crate::matchers::n4::verb_te_2,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Teyokatta {
        name: "てよかった",
        matcher_fn: crate::matchers::n4::teyokatta,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    VerbUff3bSeruU30fbSaseruUff3d {
        name: "Verb［せる・させる］",
        matcher_fn: crate::matchers::n4::verb_uff3b_seru_u30fb_saseru_uff3d,
        priority: 1,
        category: PatternCategory::Conjugation,
        jlpt: "n4",
    },

    Toittemoii {
        name: "といってもいい",
        matcher_fn: crate::matchers::n4::toittemoii,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Temo {
        name: "ても",
        matcher_fn: crate::matchers::n4::temo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    TeshimauU30fbChau {
        name: "てしまう・ちゃう",
        matcher_fn: crate::matchers::n4::teshimau_u30fb_chau,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    VerbTeB2 {
        name: "Verb[て] + B",
        matcher_fn: crate::matchers::n4::verb_te_b_2,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    CausativePassive {
        name: "Causative-Passive",
        matcher_fn: crate::matchers::n4::causative_passive,
        priority: 1,
        category: PatternCategory::Conjugation,
        jlpt: "n4",
    },

    VerbTeU30fbNounDeB {
        name: "Verb[て]・Noun[で] + B",
        matcher_fn: crate::matchers::n4::verb_te_u30fb_noun_de_b,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Tearu {
        name: "てある ",
        matcher_fn: crate::matchers::n4::tearu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    YouniUff5eTehoshii {
        name: "ように～てほしい",
        matcher_fn: crate::matchers::n4::youni_uff5e_tehoshii,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Teiruaidani {
        name: "ているあいだに",
        matcher_fn: crate::matchers::n4::teiruaidani,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Nakutemoii {
        name: "なくてもいい",
        matcher_fn: crate::matchers::n4::nakutemoii,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Temiru {
        name: "てみる",
        matcher_fn: crate::matchers::n4::temiru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Tesumimasen {
        name: "てすみません",
        matcher_fn: crate::matchers::n4::tesumimasen,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Teageru {
        name: "てあげる",
        matcher_fn: crate::matchers::n4::teageru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Tekureru {
        name: "てくれる",
        matcher_fn: crate::matchers::n4::tekureru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Temorau {
        name: "てもらう",
        matcher_fn: crate::matchers::n4::temorau,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Nasai {
        name: "なさい",
        matcher_fn: crate::matchers::n4::nasai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    VerbNaide {
        name: "Verb[ないで]",
        matcher_fn: crate::matchers::n4::verb_naide,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Tekuretearigatou {
        name: "てくれてありがとう",
        matcher_fn: crate::matchers::n4::tekuretearigatou,
        priority: 2,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    TekurenaiU30fbTemoraenai {
        name: "てくれない・てもらえない",
        matcher_fn: crate::matchers::n4::tekurenai_u30fb_temoraenai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Uff5eNodarouka {
        name: "～のだろうか",
        matcher_fn: crate::matchers::n4::uff5e_nodarouka,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    OUff5eNinaru {
        name: "お～になる ",
        matcher_fn: crate::matchers::n4::o_uff5e_ninaru,
        priority: 2,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Nasaru {
        name: "なさる",
        matcher_fn: crate::matchers::n4::nasaru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    OUff5eKudasai {
        name: "お～ください ",
        matcher_fn: crate::matchers::n4::o_uff5e_kudasai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Irassharu {
        name: "いらっしゃる",
        matcher_fn: crate::matchers::n4::irassharu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Gozaimasu {
        name: "ございます",
        matcher_fn: crate::matchers::n4::gozaimasu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Degozaimasu {
        name: "でございます",
        matcher_fn: crate::matchers::n4::degozaimasu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    OU301cSuru {
        name: "お〜する",
        matcher_fn: crate::matchers::n4::o_u301c_suru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Itasu {
        name: "いたす",
        matcher_fn: crate::matchers::n4::itasu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Teitadakemasenka {
        name: "ていただけませんか",
        matcher_fn: crate::matchers::n4::teitadakemasenka,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Tara {
        name: "たら",
        matcher_fn: crate::matchers::n4::tara,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    HokaniMoU30fbHokaNiHa {
        name: "ほかに(も)・ほか(に)は",
        matcher_fn: crate::matchers::n4::hokani_mo_u30fb_hoka_ni_ha,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Gahitsuyou {
        name: "がひつよう",
        matcher_fn: crate::matchers::n4::gahitsuyou,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Sonnani {
        name: "そんなに",
        matcher_fn: crate::matchers::n4::sonnani,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Hitsuyougaaru {
        name: "ひつようがある",
        matcher_fn: crate::matchers::n4::hitsuyougaaru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Tatoeba {
        name: "たとえば",
        matcher_fn: crate::matchers::n4::tatoeba,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    ReruU30fbRareruPotential {
        name: "れる・られる (Potential)",
        matcher_fn: crate::matchers::n4::reru_u30fb_rareru_potential,
        priority: 1,
        category: PatternCategory::Conjugation,
        jlpt: "n4",
    },

    NdakedoU30fbNdesuga {
        name: "んだけど・んですが",
        matcher_fn: crate::matchers::n4::ndakedo_u30fb_ndesuga,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Hazuda {
        name: "はずだ",
        matcher_fn: crate::matchers::n4::hazuda,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Kadouka {
        name: "かどうか",
        matcher_fn: crate::matchers::n4::kadouka,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Naito {
        name: "ないと",
        matcher_fn: crate::matchers::n4::naito,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Hazuganai {
        name: "はずがない",
        matcher_fn: crate::matchers::n4::hazuganai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    ShikaUff5eNai {
        name: "しか～ない ",
        matcher_fn: crate::matchers::n4::shika_uff5e_nai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Dakedenaku {
        name: "だけでなく",
        matcher_fn: crate::matchers::n4::dakedenaku,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Kotogadekiru {
        name: "ことができる",
        matcher_fn: crate::matchers::n4::kotogadekiru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Kai {
        name: "かい",
        matcher_fn: crate::matchers::n4::kai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Moshi {
        name: "もし",
        matcher_fn: crate::matchers::n4::moshi,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    ShiUff5eShi {
        name: "し～し ",
        matcher_fn: crate::matchers::n4::shi_uff5e_shi,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    DedekiruU30fbKaradekiru {
        name: "でできる・からできる",
        matcher_fn: crate::matchers::n4::dedekiru_u30fb_karadekiru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Nagara {
        name: "ながら",
        matcher_fn: crate::matchers::n4::nagara,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Tatokoroda {
        name: "たところだ",
        matcher_fn: crate::matchers::n4::tatokoroda,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Teirutokoroda {
        name: "ているところだ",
        matcher_fn: crate::matchers::n4::teirutokoroda,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    ToUff5eToU3001Dochiraga {
        name: "と～と、どちらが ",
        matcher_fn: crate::matchers::n4::to_uff5e_to_u3001_dochiraga,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Younisuru {
        name: "ようにする",
        matcher_fn: crate::matchers::n4::younisuru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Nakerebaikenai {
        name: "なければいけない",
        matcher_fn: crate::matchers::n4::nakerebaikenai,
        priority: 1,
        category: PatternCategory::Conjugation,
        jlpt: "n4",
    },

    Nakerebanaranai {
        name: "なければならない",
        matcher_fn: crate::matchers::n4::nakerebanaranai,
        priority: 1,
        category: PatternCategory::Conjugation,
        jlpt: "n4",
    },

    Tsuzukeru {
        name: "つづける",
        matcher_fn: crate::matchers::n4::tsuzukeru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Youniiu {
        name: "ようにいう",
        matcher_fn: crate::matchers::n4::youniiu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Yoteida {
        name: "よていだ",
        matcher_fn: crate::matchers::n4::yoteida,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Youniinoru {
        name: "ようにいのる",
        matcher_fn: crate::matchers::n4::youniinoru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Tabakari {
        name: "たばかり",
        matcher_fn: crate::matchers::n4::tabakari,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Kasuru {
        name: "化する",
        matcher_fn: crate::matchers::n4::kasuru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Meireigata {
        name: "命令形",
        matcher_fn: crate::matchers::n4::meireigata,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Youni {
        name: "ように",
        matcher_fn: crate::matchers::n4::youni,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Kashira {
        name: "かしら",
        matcher_fn: crate::matchers::n4::kashira,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    RashiiU2461 {
        name: "らしい ②",
        matcher_fn: crate::matchers::n4::rashii_u2461,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Nimieru {
        name: "にみえる",
        matcher_fn: crate::matchers::n4::nimieru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Tomieru {
        name: "とみえる",
        matcher_fn: crate::matchers::n4::tomieru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Kaze {
        name: "風",
        matcher_fn: crate::matchers::n4::kaze,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Gamirareru {
        name: "がみられる",
        matcher_fn: crate::matchers::n4::gamirareru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Nikigatsuku {
        name: "にきがつく",
        matcher_fn: crate::matchers::n4::nikigatsuku,
        priority: 2,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    NikigatsukuSplit {
        name: "にきがつく_split",
        matcher_fn: crate::matchers::n4::nikigatsuku_split,
        priority: 2,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    U301cDemoU301cDemo {
        name: "〜でも 〜でも",
        matcher_fn: crate::matchers::n4::u301c_demo_u301c_demo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Soreni {
        name: "それに",
        matcher_fn: crate::matchers::n4::soreni,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Sorede {
        name: "それで",
        matcher_fn: crate::matchers::n4::sorede,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    QuestionPhraseKa {
        name: "Question-phrase + か",
        matcher_fn: crate::matchers::n4::question_phrase_ka,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Soredemo {
        name: "それでも",
        matcher_fn: crate::matchers::n4::soredemo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Taradou {
        name: "たらどう",
        matcher_fn: crate::matchers::n4::taradou,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Tokangaerareteiru {
        name: "とかんがえられている",
        matcher_fn: crate::matchers::n4::tokangaerareteiru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Tosareteiru {
        name: "とされている",
        matcher_fn: crate::matchers::n4::tosareteiru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Toiwareteiru {
        name: "といわれている",
        matcher_fn: crate::matchers::n4::toiwareteiru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    Bayokatta {
        name: "ばよかった",
        matcher_fn: crate::matchers::n4::bayokatta,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    ShiU301cShi {
        name: "し～し ",
        matcher_fn: crate::matchers::n4::shi_u301c_shi,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n4",
    },

    // ========== N3 PATTERNS ==========

    Tte {
        name: "って",
        matcher_fn: crate::matchers::n3::tte,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Baii {
        name: "ばいい",
        matcher_fn: crate::matchers::n3::baii,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    // たらいい・といい variants - 7 separate patterns for different conditional forms
    TaraiiU30fbToiiBa {
        name: "たらいい・といい_ば",
        matcher_fn: crate::matchers::n3::taraii_u30fb_toii_ba,
        priority: 6,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    TaraiiU30fbToiiTaraTa {
        name: "たらいい・といい_たら連用タ",
        matcher_fn: crate::matchers::n3::taraii_u30fb_toii_tara_ta,
        priority: 6,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    TaraiiU30fbToiiTaraRen {
        name: "たらいい・といい_たら連用",
        matcher_fn: crate::matchers::n3::taraii_u30fb_toii_tara_ren,
        priority: 6,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    TaraiiU30fbToiiTo {
        name: "たらいい・といい_と",
        matcher_fn: crate::matchers::n3::taraii_u30fb_toii_to,
        priority: 6,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    TaraiiU30fbToiiNaDattara {
        name: "たらいい・といい_な形だったら",
        matcher_fn: crate::matchers::n3::taraii_u30fb_toii_na_dattara,
        priority: 6,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    TaraiiU30fbToiiNaDeareba {
        name: "たらいい・といい_な形であれば",
        matcher_fn: crate::matchers::n3::taraii_u30fb_toii_na_deareba,
        priority: 6,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    TaraiiU30fbToiiNaDato {
        name: "たらいい・といい_な形だと",
        matcher_fn: crate::matchers::n3::taraii_u30fb_toii_na_dato,
        priority: 6,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Naka {
        name: "中",
        matcher_fn: crate::matchers::n3::naka,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Nomani {
        name: "の間に",
        matcher_fn: crate::matchers::n3::nomani,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Uchini {
        name: "うちに",
        matcher_fn: crate::matchers::n3::uchini,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Naiuchini {
        name: "ないうちに",
        matcher_fn: crate::matchers::n3::naiuchini,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Beki {
        name: "べき",
        matcher_fn: crate::matchers::n3::beki,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Bekidehanai {
        name: "べきではない",
        matcher_fn: crate::matchers::n3::bekidehanai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Nakanaka {
        name: " なかなか",
        matcher_fn: crate::matchers::n3::nakanaka,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Amari {
        name: "あまり",
        matcher_fn: crate::matchers::n3::amari,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    NakanakaUff5eNai {
        name: "なかなか～ない",
        matcher_fn: crate::matchers::n3::nakanaka_uff5e_nai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    NiyorutoU30fbNiyoreba {
        name: "によると・によれば",
        matcher_fn: crate::matchers::n3::niyoruto_u30fb_niyoreba,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    NiyotteU30fbNiyoru {
        name: "によって・による",
        matcher_fn: crate::matchers::n3::niyotte_u30fb_niyoru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    MattakuUff5eNai {
        name: "全く～ない",
        matcher_fn: crate::matchers::n3::mattaku_uff5e_nai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Kotoda {
        name: "ことだ",
        matcher_fn: crate::matchers::n3::kotoda,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Souda {
        name: "そうだ ",
        matcher_fn: crate::matchers::n3::souda,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Suruto {
        name: "すると",
        matcher_fn: crate::matchers::n3::suruto,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Sousuruto {
        name: "そうすると",
        matcher_fn: crate::matchers::n3::sousuruto,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Nohaxnohouda {
        name: "のはXの方だ",
        matcher_fn: crate::matchers::n3::nohaxnohouda,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Nountasukata {
        name: "Noun＋型",
        matcher_fn: crate::matchers::n3::nountasukata,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    NountasukataCompound {
        name: "Noun＋型",
        matcher_fn: crate::matchers::n3::nountasukata_compound,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Tegoran {
        name: "てごらん",
        matcher_fn: crate::matchers::n3::tegoran,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    ParticleNo {
        name: "Particle + の",
        matcher_fn: crate::matchers::n3::particle_no,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Dearu {
        name: "である",
        matcher_fn: crate::matchers::n3::dearu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Tokoroga {
        name: "ところが",
        matcher_fn: crate::matchers::n3::tokoroga,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Tokorode {
        name: "ところで",
        matcher_fn: crate::matchers::n3::tokorode,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Hodo {
        name: "ほど",
        matcher_fn: crate::matchers::n3::hodo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    BaU301cHodo {
        name: "ば〜ほど",
        matcher_fn: crate::matchers::n3::ba_u301c_hodo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    HodoUff5eNai {
        name: "ほど～ない",
        matcher_fn: crate::matchers::n3::hodo_uff5e_nai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    DehaU30fbSoredehaU30fbJaa {
        name: "では・それでは・じゃあ",
        matcher_fn: crate::matchers::n3::deha_u30fb_soredeha_u30fb_jaa,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Noni2 {
        name: "のに",
        matcher_fn: crate::matchers::n3::noni_2,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    TameNi {
        name: "ため(に)",
        matcher_fn: crate::matchers::n3::tame_ni,
        priority: 3,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Tameni {
        name: "ために",
        matcher_fn: crate::matchers::n3::tameni,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Toiukotoda {
        name: "ということだ",
        matcher_fn: crate::matchers::n3::toiukotoda,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Toiunoha {
        name: "というのは",
        matcher_fn: crate::matchers::n3::toiunoha,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    ToiunohaAbbreviated {
        name: "というのは_abbreviated",
        matcher_fn: crate::matchers::n3::toha,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Teki {
        name: "的",
        matcher_fn: crate::matchers::n3::teki,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    MonoU30fbMon {
        name: "もの・もん",
        matcher_fn: crate::matchers::n3::mono_u30fb_mon,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Monoda {
        name: "ものだ",
        matcher_fn: crate::matchers::n3::monoda,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    MonodaDewanai {
        name: "ものではない",
        matcher_fn: crate::matchers::n3::monoda_dewanai,
        priority: 2,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    MonodaJanai {
        name: "ものじゃない",
        matcher_fn: crate::matchers::n3::monoda_janai,
        priority: 2,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Saichuuni {
        name: "最中に",
        matcher_fn: crate::matchers::n3::saichuuni,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Uede {
        name: "上で",
        matcher_fn: crate::matchers::n3::uede,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Okagede {
        name: "おかげで",
        matcher_fn: crate::matchers::n3::okagede,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Nimotozuite {
        name: "にもとづいて",
        matcher_fn: crate::matchers::n3::nimotozuite,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Ten {
        name: "点",
        matcher_fn: crate::matchers::n3::ten,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    NazenaraU301cKara {
        name: "なぜなら〜から",
        matcher_fn: crate::matchers::n3::nazenara_u301c_kara,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Koso {
        name: "こそ",
        matcher_fn: crate::matchers::n3::koso,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Karakoso {
        name: "からこそ",
        matcher_fn: crate::matchers::n3::karakoso,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Aruiwa {
        name: "あるいは",
        matcher_fn: crate::matchers::n3::aruiwa,
        priority: 3,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Bakari {
        name: "ばかり",
        matcher_fn: crate::matchers::n3::bakari,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Bakarida {
        name: "ばかりだ",
        matcher_fn: crate::matchers::n3::bakarida,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Bakarini {
        name: "ばかりに",
        matcher_fn: crate::matchers::n3::bakarini,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Kotogaaru {
        name: "ことがある",
        matcher_fn: crate::matchers::n3::kotogaaru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Kotonisuru {
        name: "ことにする",
        matcher_fn: crate::matchers::n3::kotonisuru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Kotonano {
        name: "ことなの",
        matcher_fn: crate::matchers::n3::kotonano,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Kotoninaru {
        name: "ことになる",
        matcher_fn: crate::matchers::n3::kotoninaru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Uff5eHaUff5eDeyuumei {
        name: "～は～で有名",
        matcher_fn: crate::matchers::n3::uff5e_ha_uff5e_deyuumei,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Kotohanai {
        name: "ことはない",
        matcher_fn: crate::matchers::n3::kotohanai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Uff5eToittemo {
        name: " ～と言っても",
        matcher_fn: crate::matchers::n3::uff5e_toittemo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    TemoNakutemo {
        name: "～ても～なくても",
        matcher_fn: crate::matchers::n3::temo_nakutemo,
        priority: 3,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Shikanai {
        name: "しかない",
        matcher_fn: crate::matchers::n3::shikanai,
        priority: 3,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    ShikanaiPolite {
        name: "しかない_polite",
        matcher_fn: crate::matchers::n3::shikanai_polite,
        priority: 3,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Toieba {
        name: "といえば",
        matcher_fn: crate::matchers::n3::toieba,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Au {
        name: "合う",
        matcher_fn: crate::matchers::n3::au,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    NiawaseteU30fbNiatta {
        name: "に合わせて・に合った",
        matcher_fn: crate::matchers::n3::niawasete_u30fb_niatta,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Nitsuite {
        name: "について",
        matcher_fn: crate::matchers::n3::nitsuite,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Uff5eNoSugata {
        name: "～(の)姿",
        matcher_fn: crate::matchers::n3::uff5e_no_sugata,
        priority: 3,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Toieru {
        name: "と言える",
        matcher_fn: crate::matchers::n3::toieru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    ChantoU30fbKichinto {
        name: "ちゃんと・きちんと",
        matcher_fn: crate::matchers::n3::chanto_u30fb_kichinto,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    SonotameNi {
        name: "そのため(に)",
        matcher_fn: crate::matchers::n3::sonotame_ni,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Sonokekka {
        name: "その結果",
        matcher_fn: crate::matchers::n3::sonokekka,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Nikurabete {
        name: "に比べて",
        matcher_fn: crate::matchers::n3::nikurabete,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    DonnaniU301cTemo {
        name: "どんなに〜ても",
        matcher_fn: crate::matchers::n3::donnani_u301c_temo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    IkuraU301cDemo {
        name: "いくら〜でも",
        matcher_fn: crate::matchers::n3::ikura_u301c_demo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    U301cKahaU301cNiyottechigau {
        name: "〜かは〜によって違う",
        matcher_fn: crate::matchers::n3::u301c_kaha_u301c_niyottechigau,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Kanari {
        name: "かなり",
        matcher_fn: crate::matchers::n3::kanari,
        priority: 3,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    KanariNoNoun {
        name: "かなり + の + Noun",
        matcher_fn: crate::matchers::n3::kanari_no_noun,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Amarini {
        name: "あまりに",
        matcher_fn: crate::matchers::n3::amarini,
        priority: 3,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    AmarinoNoun {
        name: "あまりの + Noun",
        matcher_fn: crate::matchers::n3::amarino_noun,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Amarinimo {
        name: "あまりにも",
        matcher_fn: crate::matchers::n3::amarinimo,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Wakeda {
        name: "わけだ",
        matcher_fn: crate::matchers::n3::wakeda,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Wakedehanai {
        name: "わけではない",
        matcher_fn: crate::matchers::n3::wakedehanai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Todoujini {
        name: "と同時に",
        matcher_fn: crate::matchers::n3::todoujini,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    TokorodattaU2460 {
        name: "ところだった ①",
        matcher_fn: crate::matchers::n3::tokorodatta_u2460,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Datte {
        name: "だって",
        matcher_fn: crate::matchers::n3::datte,
        priority: 3,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Ndatte {
        name: "んだって",
        matcher_fn: crate::matchers::n3::ndatte,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Kankeigaaru {
        name: "関係がある",
        matcher_fn: crate::matchers::n3::kankeigaaru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    // に関する・に関して pattern has multiple tokenization variants
    NikansuruParticle {
        name: "に関する・に関して_particle",
        matcher_fn: crate::matchers::n3::nikansuru_particle,
        priority: 3,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    NikansuruVerb {
        name: "に関する・に関して_verb",
        matcher_fn: crate::matchers::n3::nikansuru_verb,
        priority: 3,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    NikanshiteNoun {
        name: "に関する・に関して_noun",
        matcher_fn: crate::matchers::n3::nikanshite_noun,
        priority: 3,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    NikanshiteNounVerb {
        name: "に関する・に関して_noun_verb",
        matcher_fn: crate::matchers::n3::nikanshite_noun_verb,
        priority: 3,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Nitaishite {
        name: "に対して",
        matcher_fn: crate::matchers::n3::nitaishite,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    KuraiU2461 {
        name: "くらい ②",
        matcher_fn: crate::matchers::n3::kurai_u2461,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    HaUff5eKuraidesu {
        name: "は～くらいです",
        matcher_fn: crate::matchers::n3::ha_uff5e_kuraidesu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    SaInterjection {
        name: "さ - Interjection",
        matcher_fn: crate::matchers::n3::sa_interjection,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    SaFiller {
        name: "さ - Filler",
        matcher_fn: crate::matchers::n3::sa_filler,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    SaCasualYo {
        name: "さ - Casual よ",
        matcher_fn: crate::matchers::n3::sa_casual_yo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Sorezore {
        name: "それぞれ",
        matcher_fn: crate::matchers::n3::sorezore,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Sokode {
        name: "そこで",
        matcher_fn: crate::matchers::n3::sokode,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Temokamawanai {
        name: "てもかまわない",
        matcher_fn: crate::matchers::n3::temokamawanai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Uff5eTemoUff5eNakutemo {
        name: "～ても～なくても",
        matcher_fn: crate::matchers::n3::uff5e_temo_uff5e_nakutemo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Njanai {
        name: "んじゃない",
        matcher_fn: crate::matchers::n3::njanai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Wakeganai {
        name: "わけがない",
        matcher_fn: crate::matchers::n3::wakeganai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    ToshitaraU30fbTosurebaU30fbTosuruto {
        name: "としたら・とすれば・とすると",
        matcher_fn: crate::matchers::n3::toshitara_u30fb_tosureba_u30fb_tosuruto,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Toshite {
        name: "として",
        matcher_fn: crate::matchers::n3::toshite,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Nishiteha {
        name: "にしては",
        matcher_fn: crate::matchers::n3::nishiteha,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Nishitemo {
        name: "にしても",
        matcher_fn: crate::matchers::n3::nishitemo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Uff5eToiunohajijitsuda {
        name: "～というのは事実だ",
        matcher_fn: crate::matchers::n3::uff5e_toiunohajijitsuda,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Karaiuto {
        name: "から言うと",
        matcher_fn: crate::matchers::n3::karaiuto,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Nitotte {
        name: "に取って",
        matcher_fn: crate::matchers::n3::nitotte,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Kotokara {
        name: "ことから",
        matcher_fn: crate::matchers::n3::kotokara,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Toiuyori {
        name: "というより",
        matcher_fn: crate::matchers::n3::toiuyori,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Hamochiron {
        name: "はもちろん",
        matcher_fn: crate::matchers::n3::hamochiron,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Wohajime {
        name: "をはじめ",
        matcher_fn: crate::matchers::n3::wohajime,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Tehajimete {
        name: "て初めて",
        matcher_fn: crate::matchers::n3::tehajimete,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Sae {
        name: "さえ",
        matcher_fn: crate::matchers::n3::sae,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    SaeU301cBa {
        name: "さえ〜ば",
        matcher_fn: crate::matchers::n3::sae_u301c_ba,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Tamonoda {
        name: "たものだ",
        matcher_fn: crate::matchers::n3::tamonoda,
        priority: 3,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Sate {
        name: "さて",
        matcher_fn: crate::matchers::n3::sate,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Mushiro {
        name: "むしろ",
        matcher_fn: crate::matchers::n3::mushiro,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Tsumari {
        name: "つまり",
        matcher_fn: crate::matchers::n3::tsumari,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Sunawachi {
        name: "即ち",
        matcher_fn: crate::matchers::n3::sunawachi,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Kaette {
        name: "却って",
        matcher_fn: crate::matchers::n3::kaette,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    MarudeU2026Youda {
        name: "まるで…ようだ",
        matcher_fn: crate::matchers::n3::marude_u2026_youda,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Younakigasuru {
        name: "ような気がする",
        matcher_fn: crate::matchers::n3::younakigasuru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    TotemoUff5eNai {
        name: "とても～ない",
        matcher_fn: crate::matchers::n3::totemo_uff5e_nai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    BetsuniU301cNai {
        name: "別に〜ない",
        matcher_fn: crate::matchers::n3::betsuni_u301c_nai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Bakaridenaku {
        name: "ばかりでなく",
        matcher_fn: crate::matchers::n3::bakaridenaku,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Bakarika {
        name: "ばかりか",
        matcher_fn: crate::matchers::n3::bakarika,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    DehanakuteU30fbJanakute {
        name: "ではなくて・じゃなくて",
        matcher_fn: crate::matchers::n3::dehanakute_u30fb_janakute,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    DakedenakuTeUff5eMo {
        name: "だけでなく(て)～も",
        matcher_fn: crate::matchers::n3::dakedenaku_te_uff5e_mo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Dakeshika {
        name: "だけしか",
        matcher_fn: crate::matchers::n3::dakeshika,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    HaiumademonaiU2460Single {
        name: "は言うまでもない ①_single",
        matcher_fn: crate::matchers::n3::haiumademonai_u2460_single,
        priority: 3,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    HaiumademonaiU2460Split {
        name: "は言うまでもない ①_split",
        matcher_fn: crate::matchers::n3::haiumademonai_u2460_split,
        priority: 3,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    HaiumademonaiU2460Polite {
        name: "は言うまでもない ①_polite",
        matcher_fn: crate::matchers::n3::haiumademonai_u2460_polite,
        priority: 3,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    KesshiteU301cNai {
        name: "決して〜ない",
        matcher_fn: crate::matchers::n3::kesshite_u301c_nai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Wakenihaikanai {
        name: "わけにはいかない",
        matcher_fn: crate::matchers::n3::wakenihaikanai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    U301cYoutoshinai {
        name: "〜ようとしない",
        matcher_fn: crate::matchers::n3::u301c_youtoshinai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Moshikashitara {
        name: "もしかしたら",
        matcher_fn: crate::matchers::n3::moshikashitara,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    TatoeU301cTemo {
        name: "たとえ〜ても",
        matcher_fn: crate::matchers::n3::tatoe_u301c_temo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Kotoni {
        name: "ことに",
        matcher_fn: crate::matchers::n3::kotoni,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Kotoka {
        name: "ことか",
        matcher_fn: crate::matchers::n3::kotoka,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Uff5eKatoiutoU2460 {
        name: "～かというと ①",
        matcher_fn: crate::matchers::n3::uff5e_katoiuto_u2460,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Uff5eKatoiutoU2461 {
        name: "～かというと ②",
        matcher_fn: crate::matchers::n3::uff5e_katoiuto_u2461,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Deiuto {
        name: "で言うと",
        matcher_fn: crate::matchers::n3::deiuto,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Uff5eZutsu {
        name: "～ずつ",
        matcher_fn: crate::matchers::n3::uff5e_zutsu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    ZuttoU2461 {
        name: "ずっと ②",
        matcher_fn: crate::matchers::n3::zutto_u2461,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Darake {
        name: "だらけ",
        matcher_fn: crate::matchers::n3::darake,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Mottomo {
        name: "もっとも",
        matcher_fn: crate::matchers::n3::mottomo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Futatabi {
        name: "再び",
        matcher_fn: crate::matchers::n3::futatabi,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Mi {
        name: "み",
        matcher_fn: crate::matchers::n3::mi,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Toonajikurai {
        name: "と同じくらい",
        matcher_fn: crate::matchers::n3::toonajikurai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    ToonajideU30fbTochigatte {
        name: "と同じで・と違って",
        matcher_fn: crate::matchers::n3::toonajide_u30fb_tochigatte,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Tonarande {
        name: "と並んで",
        matcher_fn: crate::matchers::n3::tonarande,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Nichigainai {
        name: "に違いない",
        matcher_fn: crate::matchers::n3::nichigainai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Atari {
        name: "当たり",
        matcher_fn: crate::matchers::n3::atari,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Niataru {
        name: "に当たる",
        matcher_fn: crate::matchers::n3::niataru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    NiataruParticle {
        name: "に当たる_particle",
        matcher_fn: crate::matchers::n3::niataru_particle,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Nikagiru {
        name: "に限る",
        matcher_fn: crate::matchers::n3::nikagiru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Tohakagiranai {
        name: "とは限らない",
        matcher_fn: crate::matchers::n3::tohakagiranai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    MettaniU301cNai {
        name: "めったに〜ない",
        matcher_fn: crate::matchers::n3::mettani_u301c_nai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Warini {
        name: "割に",
        matcher_fn: crate::matchers::n3::warini,
        priority: 3,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    VerbVolitionalTosuru {
        name: "Verb[volitional]とする",
        matcher_fn: crate::matchers::n3::verb_volitional_tosuru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Youtoshinai {
        name: "〜ようとしない",
        matcher_fn: crate::matchers::n3::youtoshinai,
        priority: 2,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Toittemo {
        name: " ～と言っても",
        matcher_fn: crate::matchers::n3::toittemo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    VerbVolitionalToshitaga {
        name: "Verb[volitional] + としたが",
        matcher_fn: crate::matchers::n3::verb_volitional_toshitaga,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    IumademonaiU2461 {
        name: "言うまでもない ②",
        matcher_fn: crate::matchers::n3::iumademonai_u2461,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Soumonai {
        name: "そうもない",
        matcher_fn: crate::matchers::n3::soumonai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Naikotohanai {
        name: "ないことはない",
        matcher_fn: crate::matchers::n3::naikotohanai,
        priority: 2,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    NankaU30fbNante {
        name: "なんか・なんて",
        matcher_fn: crate::matchers::n3::nanka_u30fb_nante,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    MataU301cMo {
        name: "又〜も",
        matcher_fn: crate::matchers::n3::mata_u301c_mo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Tsuideni {
        name: "ついでに",
        matcher_fn: crate::matchers::n3::tsuideni,
        priority: 3,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Totomoni {
        name: "と共に",
        matcher_fn: crate::matchers::n3::totomoni,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Nitsurete {
        name: "につれて",
        matcher_fn: crate::matchers::n3::nitsurete,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Tadachini {
        name: "直ちに",
        matcher_fn: crate::matchers::n3::tadachini,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Tatotanni {
        name: "たとたんに",
        matcher_fn: crate::matchers::n3::tatotanni,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Okini {
        name: "おきに",
        matcher_fn: crate::matchers::n3::okini,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Tabini {
        name: "たびに",
        matcher_fn: crate::matchers::n3::tabini,
        priority: 3,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Aruiha {
        name: "あるいは",
        matcher_fn: crate::matchers::n3::aruiha,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Zutsu {
        name: "～ずつ",
        matcher_fn: crate::matchers::n3::zutsu,
        priority: 3,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Nagaramo {
        name: "ながらも",
        matcher_fn: crate::matchers::n3::nagaramo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    NioiteU30fbNiokeru {
        name: "において・における",
        matcher_fn: crate::matchers::n3::nioite_u30fb_niokeru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Daiichi {
        name: "第一",
        matcher_fn: crate::matchers::n3::daiichi,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Masumasu {
        name: "ますます",
        matcher_fn: crate::matchers::n3::masumasu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Ippouda {
        name: "一方だ",
        matcher_fn: crate::matchers::n3::ippouda,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Ippoude {
        name: "一方で",
        matcher_fn: crate::matchers::n3::ippoude,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Tsuini {
        name: "遂に",
        matcher_fn: crate::matchers::n3::tsuini,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Sudeni {
        name: "すでに",
        matcher_fn: crate::matchers::n3::sudeni,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Zuni {
        name: "ずに",
        matcher_fn: crate::matchers::n3::zuni,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Zunihairarenai {
        name: "ずにはいられない",
        matcher_fn: crate::matchers::n3::zunihairarenai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Nashi {
        name: "なし",
        matcher_fn: crate::matchers::n3::nashi,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Ari {
        name: "あり",
        matcher_fn: crate::matchers::n3::ari,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Kangaerarenai {
        name: "考えられない",
        matcher_fn: crate::matchers::n3::kangaerarenai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Kanarazushimo {
        name: "必ずしも ",
        matcher_fn: crate::matchers::n3::kanarazushimo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Renyoukei {
        name: "連用形",
        matcher_fn: crate::matchers::n3::renyoukei,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Muki {
        name: "向き",
        matcher_fn: crate::matchers::n3::muki,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Muke {
        name: "向け",
        matcher_fn: crate::matchers::n3::muke,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    AgaruU30fbAgeru {
        name: "上がる・上げる",
        matcher_fn: crate::matchers::n3::agaru_u30fb_ageru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Kiru {
        name: "切る",
        matcher_fn: crate::matchers::n3::kiru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Kirenai {
        name: "切れない",
        matcher_fn: crate::matchers::n3::kirenai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Kiri {
        name: "きり",
        matcher_fn: crate::matchers::n3::kiri,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Kake {
        name: "かけ",
        matcher_fn: crate::matchers::n3::kake,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    KakeCompound {
        name: "かけ_compound",
        matcher_fn: crate::matchers::n3::kake_compound,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Nikakete {
        name: "にかけて",
        matcher_fn: crate::matchers::n3::nikakete,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Tate {
        name: "たて",
        matcher_fn: crate::matchers::n3::tate,
        priority: 3,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Chuu {
        name: "中",
        matcher_fn: crate::matchers::n3::chuu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    TeHajimete {
        name: "て初めて",
        matcher_fn: crate::matchers::n3::te_hajimete,
        priority: 3,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Tatoetemo {
        name: "たとえ〜ても",
        matcher_fn: crate::matchers::n3::tatoetemo,
        priority: 3,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    KomuU2460 {
        name: "込む ①",
        matcher_fn: crate::matchers::n3::komu_u2460,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    KomuU2461 {
        name: "込む ②",
        matcher_fn: crate::matchers::n3::komu_u2461,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Furiwosuru {
        name: "ふりをする",
        matcher_fn: crate::matchers::n3::furiwosuru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    DekirebaU30fbDekitara {
        name: "できれば・できたら",
        matcher_fn: crate::matchers::n3::dekireba_dekitara,
        priority: 3,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Deyokereba {
        name: "でよければ",
        matcher_fn: crate::matchers::n3::deyokereba,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Shidai {
        name: "次第",
        matcher_fn: crate::matchers::n3::shidai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Toori {
        name: "とおり",
        matcher_fn: crate::matchers::n3::toori,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Demoaru {
        name: "でもある",
        matcher_fn: crate::matchers::n3::demoaru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Doushitemo {
        name: "どうしても",
        matcher_fn: crate::matchers::n3::doushitemo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    MoshimoUff5eNaraU30fbMoshimoUff5eDemo {
        name: "もしも～なら・もしも～でも",
        matcher_fn: crate::matchers::n3::moshimo_uff5e_nara_u30fb_moshimo_uff5e_demo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Doushi {
        name: "同士",
        matcher_fn: crate::matchers::n3::doushi,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Gatai {
        name: "がたい",
        matcher_fn: crate::matchers::n3::gatai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Masaka {
        name: "まさか",
        matcher_fn: crate::matchers::n3::masaka,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    ZenshahaU30fbKoushaha {
        name: "前者は・後者は",
        matcher_fn: crate::matchers::n3::zenshaha_u30fb_koushaha,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Tsui {
        name: "つい",
        matcher_fn: crate::matchers::n3::tsui,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Seide {
        name: "せいで",
        matcher_fn: crate::matchers::n3::seide,
        priority: 3,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Kuseni {
        name: "くせに",
        matcher_fn: crate::matchers::n3::kuseni,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Gachi {
        name: "がち",
        matcher_fn: crate::matchers::n3::gachi,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Gimi {
        name: "ぎみ",
        matcher_fn: crate::matchers::n3::gimi,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Ppoi {
        name: "っぽい",
        matcher_fn: crate::matchers::n3::ppoi,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Ppanashi {
        name: "っぱなし",
        matcher_fn: crate::matchers::n3::ppanashi,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Wazawaza {
        name: "わざわざ",
        matcher_fn: crate::matchers::n3::wazawaza,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Ittai {
        name: "一体",
        matcher_fn: crate::matchers::n3::ittai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Sekkaku {
        name: "折角",
        matcher_fn: crate::matchers::n3::sekkaku,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Kke {
        name: "っけ",
        matcher_fn: crate::matchers::n3::kke,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Kawarini {
        name: "代わりに",
        matcher_fn: crate::matchers::n3::kawarini,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Nikawatte {
        name: "に代わって",
        matcher_fn: crate::matchers::n3::nikawatte,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Dokoroka {
        name: "どころか",
        matcher_fn: crate::matchers::n3::dokoroka,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Toiuriyuude {
        name: "という理由で",
        matcher_fn: crate::matchers::n3::toiuriyuude,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Uff5eHaUff5eTonatteiru {
        name: "～は～となっている",
        matcher_fn: crate::matchers::n3::uff5e_ha_uff5e_tonatteiru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    Sayuusuru {
        name: "左右する",
        matcher_fn: crate::matchers::n3::sayuusuru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n3",
    },

    // ========== N2 PATTERNS ==========

    EruU30fbEru {
        name: "得る・得る",
        matcher_fn: crate::matchers::n2::eru_u30fb_eru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    U301cEnai {
        name: "〜得ない",
        matcher_fn: crate::matchers::n2::u301c_enai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Zaruwoenai {
        name: "ざるを得ない",
        matcher_fn: crate::matchers::n2::zaruwoenai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Uff5eZaru {
        name: "～ざる",
        matcher_fn: crate::matchers::n2::uff5e_zaru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Tsumoride {
        name: "つもりで",
        matcher_fn: crate::matchers::n2::tsumoride,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Douse {
        name: "どうせ",
        matcher_fn: crate::matchers::n2::douse,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Semete {
        name: "せめて",
        matcher_fn: crate::matchers::n2::semete,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Douyara {
        name: "どうやら",
        matcher_fn: crate::matchers::n2::douyara,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Naniyara {
        name: "なにやら",
        matcher_fn: crate::matchers::n2::naniyara,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Yorihokanai {
        name: "よりほかない",
        matcher_fn: crate::matchers::n2::yorihokanai,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Tashikani {
        name: "確かに",
        matcher_fn: crate::matchers::n2::tashikani,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    IchiouU2460 {
        name: "一応 ①",
        matcher_fn: crate::matchers::n2::ichiou_u2460,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    IchiouU2461 {
        name: "一応 ②",
        matcher_fn: crate::matchers::n2::ichiou_u2461,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nisouinai {
        name: "に相違ない",
        matcher_fn: crate::matchers::n2::nisouinai,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Mangaichi {
        name: "万が一",
        matcher_fn: crate::matchers::n2::mangaichi,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    YouganaiU30fbYoumonai {
        name: "ようがない・ようもない",
        matcher_fn: crate::matchers::n2::youganai_u30fb_youmonai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nihokanaranai {
        name: "にほかならない",
        matcher_fn: crate::matchers::n2::nihokanaranai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Kkonai {
        name: "っこない",
        matcher_fn: crate::matchers::n2::kkonai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Sorenara {
        name: "それなら",
        matcher_fn: crate::matchers::n2::sorenara,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    MononaraU2460 {
        name: "ものなら①",
        matcher_fn: crate::matchers::n2::mononara_u2460,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Uff5eWoUff5eNimakaseru {
        name: "～を～に任せる",
        matcher_fn: crate::matchers::n2::uff5e_wo_uff5e_nimakaseru,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Uff5eWoUff5eNimakaseruReverse {
        name: "～を～に任せる",
        matcher_fn: crate::matchers::n2::uff5e_wo_uff5e_nimakaseru_reverse,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Ikasu {
        name: "活かす",
        matcher_fn: crate::matchers::n2::ikasu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Ooyoso {
        name: "おおよそ",
        matcher_fn: crate::matchers::n2::ooyoso,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Mai {
        name: "まい",
        matcher_fn: crate::matchers::n2::mai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Ue {
        name: "上",
        matcher_fn: crate::matchers::n2::ue,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Ueni {
        name: "上に",
        matcher_fn: crate::matchers::n2::ueni,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    IjouU2461 {
        name: "以上 ②",
        matcher_fn: crate::matchers::n2::ijou_u2461,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Ijouni {
        name: "以上に",
        matcher_fn: crate::matchers::n2::ijouni,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    TochuuniU30fbTochuude {
        name: "途中に・途中で",
        matcher_fn: crate::matchers::n2::tochuuni_u30fb_tochuude,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nakawo {
        name: "中を",
        matcher_fn: crate::matchers::n2::nakawo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Wochuushinni {
        name: "を中心に",
        matcher_fn: crate::matchers::n2::wochuushinni,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Sonoue {
        name: "その上",
        matcher_fn: crate::matchers::n2::sonoue,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Ueha {
        name: "上は",
        matcher_fn: crate::matchers::n2::ueha,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Noshitade {
        name: "の下で",
        matcher_fn: crate::matchers::n2::noshitade,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    KouNoNoun {
        name: "後(の) Noun",
        matcher_fn: crate::matchers::n2::kou_no_noun,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Temae {
        name: "手前",
        matcher_fn: crate::matchers::n2::temae,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Womegutte {
        name: "を巡って",
        matcher_fn: crate::matchers::n2::womegutte,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Niwatatte {
        name: "にわたって",
        matcher_fn: crate::matchers::n2::niwatatte,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nisotte {
        name: "に沿って",
        matcher_fn: crate::matchers::n2::nisotte,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    TasueU30fbNosue {
        name: "た末・の末",
        matcher_fn: crate::matchers::n2::tasue_u30fb_nosue,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nishitagatte {
        name: "にしたがって",
        matcher_fn: crate::matchers::n2::nishitagatte,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    NitomonatteU30fbNitomonai {
        name: "に伴って・に伴い",
        matcher_fn: crate::matchers::n2::nitomonatte_u30fb_nitomonai,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nitsuki {
        name: "につき",
        matcher_fn: crate::matchers::n2::nitsuki,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    NitsukiCompound {
        name: "につき_compound",
        matcher_fn: crate::matchers::n2::nitsuki_compound,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nitsuke {
        name: "につけ",
        matcher_fn: crate::matchers::n2::nitsuke,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nitsukete {
        name: "につけて",
        matcher_fn: crate::matchers::n2::nitsukete,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nikakawaru {
        name: "にかかわる",
        matcher_fn: crate::matchers::n2::nikakawaru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    NimukatteU30fbNimukete {
        name: "に向かって・に向けて",
        matcher_fn: crate::matchers::n2::nimukatte_u30fb_nimukete,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Gakininaru {
        name: "が気になる",
        matcher_fn: crate::matchers::n2::gakininaru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nikiwotsukeru {
        name: "に気をつける",
        matcher_fn: crate::matchers::n2::nikiwotsukeru,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Mokamawazu {
        name: "も構わず",
        matcher_fn: crate::matchers::n2::mokamawazu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Kaneru {
        name: "かねる",
        matcher_fn: crate::matchers::n2::kaneru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Kanenai {
        name: "かねない",
        matcher_fn: crate::matchers::n2::kanenai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Wonozoite {
        name: "を除いて",
        matcher_fn: crate::matchers::n2::wonozoite,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nikakawarazu {
        name: "にかかわらず",
        matcher_fn: crate::matchers::n2::nikakawarazu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nimokakawarazu {
        name: "にもかかわらず",
        matcher_fn: crate::matchers::n2::nimokakawarazu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nikagirazu {
        name: "に限らず",
        matcher_fn: crate::matchers::n2::nikagirazu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    NaoU2460 {
        name: "なお①",
        matcher_fn: crate::matchers::n2::nao_u2460,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    NaoU2461 {
        name: "なお②",
        matcher_fn: crate::matchers::n2::nao_u2461,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Kagiri {
        name: "限り",
        matcher_fn: crate::matchers::n2::kagiri,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    ShidaidaU30fbShidaide {
        name: "次第だ・次第で",
        matcher_fn: crate::matchers::n2::shidaida_u30fb_shidaide,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Shidaini {
        name: "次第に",
        matcher_fn: crate::matchers::n2::shidaini,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    NiKagitte {
        name: "に限って",
        matcher_fn: crate::matchers::n2::ni_kagitte,
        priority: 6,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    NiKagirazu {
        name: "に限らず",
        matcher_fn: crate::matchers::n2::ni_kagirazu,
        priority: 6,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nebanaranaי {
        name: "ねばならない",
        matcher_fn: crate::matchers::n2::nebanaranaי,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Uff5eTekoso {
        name: "～てこそ",
        matcher_fn: crate::matchers::n2::uff5e_tekoso,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Wotowazu {
        name: "を問わず",
        matcher_fn: crate::matchers::n2::wotowazu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Yorishikataganai {
        name: "よりしかたがない",
        matcher_fn: crate::matchers::n2::yorishikataganai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nikoshitakotohanai {
        name: "に越したことはない",
        matcher_fn: crate::matchers::n2::nikoshitakotohanai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Yousuruni {
        name: "要するに",
        matcher_fn: crate::matchers::n2::yousuruni,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Tekaradenaito {
        name: "てからでないと",
        matcher_fn: crate::matchers::n2::tekaradenaito,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nakuhanai {
        name: "なくはない",
        matcher_fn: crate::matchers::n2::nakuhanai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    NaikotonihaUff5eNai {
        name: "ないことには～ない",
        matcher_fn: crate::matchers::n2::naikotoniha_uff5e_nai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Naidehairarenai {
        name: "ないではいられない",
        matcher_fn: crate::matchers::n2::naidehairarenai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nebanaranai {
        name: "ねばならない",
        matcher_fn: crate::matchers::n2::nebanaranai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Tamae {
        name: "たまえ",
        matcher_fn: crate::matchers::n2::tamae,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Uff5eNouchiDe {
        name: "～のうち(で)",
        matcher_fn: crate::matchers::n2::uff5e_nouchi_de,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Uff5eNouchiDeKono {
        name: "～のうち(で)",
        matcher_fn: crate::matchers::n2::uff5e_nouchi_de_kono,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Tsutsu {
        name: "つつ",
        matcher_fn: crate::matchers::n2::tsutsu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    TsutsuMo {
        name: "つつ(も)",
        matcher_fn: crate::matchers::n2::tsutsu_mo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nisaishite {
        name: "に際して",
        matcher_fn: crate::matchers::n2::nisaishite,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Saini {
        name: "際に",
        matcher_fn: crate::matchers::n2::saini,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    NiatariU30fbNiatatte {
        name: "にあたり・にあたって",
        matcher_fn: crate::matchers::n2::niatari_u30fb_niatatte,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Wokeikini {
        name: "を契機に",
        matcher_fn: crate::matchers::n2::wokeikini,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Tsutsuaru {
        name: "つつある",
        matcher_fn: crate::matchers::n2::tsutsuaru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Uff5eTokoroniU30fbUff5eTokorohe {
        name: "～ところに・～ところへ",
        matcher_fn: crate::matchers::n2::uff5e_tokoroni_u30fb_uff5e_tokorohe,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    KaU301cNaikanouchini {
        name: "か〜ないかのうちに",
        matcher_fn: crate::matchers::n2::ka_u301c_naikanouchini,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Gakeni {
        name: "がけに",
        matcher_fn: crate::matchers::n2::gakeni,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Teiteha {
        name: "ていては",
        matcher_fn: crate::matchers::n2::teiteha,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    TokorodattaU2461 {
        name: "ところだった ②",
        matcher_fn: crate::matchers::n2::tokorodatta_u2461,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Dokorodehanai {
        name: "どころではない",
        matcher_fn: crate::matchers::n2::dokorodehanai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Burini {
        name: "ぶりに",
        matcher_fn: crate::matchers::n2::burini,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Teha {
        name: "ては",
        matcher_fn: crate::matchers::n2::teha,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    TehaU301cTeha {
        name: "ては〜ては",
        matcher_fn: crate::matchers::n2::teha_u301c_teha,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Momata {
        name: "も又",
        matcher_fn: crate::matchers::n2::momata,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    KekkaU30fbNokekka {
        name: "結果・の結果",
        matcher_fn: crate::matchers::n2::kekka_u30fb_nokekka,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    KekkaU30fbNokekka_Noun {
        name: "結果・の結果",
        matcher_fn: crate::matchers::n2::kekka_u30fb_nokekka_noun,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Irai {
        name: "以来",
        matcher_fn: crate::matchers::n2::irai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nisakidachi {
        name: "に先立ち",
        matcher_fn: crate::matchers::n2::nisakidachi,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Hatashite {
        name: "はたして",
        matcher_fn: crate::matchers::n2::hatashite,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Kaigaaru {
        name: "甲斐がある",
        matcher_fn: crate::matchers::n2::kaigaaru,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Yagate {
        name: "やがて",
        matcher_fn: crate::matchers::n2::yagate,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Shitagatte {
        name: "したがって",
        matcher_fn: crate::matchers::n2::shitagatte,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Ageku {
        name: "あげく",
        matcher_fn: crate::matchers::n2::ageku,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Kikkake {
        name: "きっかけ",
        matcher_fn: crate::matchers::n2::kikkake,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nikaketeha {
        name: "にかけては",
        matcher_fn: crate::matchers::n2::nikaketeha,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Tokkuni {
        name: "とっくに",
        matcher_fn: crate::matchers::n2::tokkuni,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Imadani {
        name: "未だに",
        matcher_fn: crate::matchers::n2::imadani,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Womotoni {
        name: "をもとに",
        matcher_fn: crate::matchers::n2::womotoni,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Karaniha {
        name: "からには",
        matcher_fn: crate::matchers::n2::karaniha,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Itsunomanika {
        name: "いつの間にか",
        matcher_fn: crate::matchers::n2::itsunomanika,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Ittan {
        name: "一旦",
        matcher_fn: crate::matchers::n2::ittan,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Hamotoyori {
        name: "はもとより",
        matcher_fn: crate::matchers::n2::hamotoyori,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Souninai {
        name: "そうにない",
        matcher_fn: crate::matchers::n2::souninai,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nihanshite {
        name: "に反して",
        matcher_fn: crate::matchers::n2::nihanshite,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Gyakuni {
        name: "逆に",
        matcher_fn: crate::matchers::n2::gyakuni,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Hanmen {
        name: "反面",
        matcher_fn: crate::matchers::n2::hanmen,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nuku {
        name: "抜く",
        matcher_fn: crate::matchers::n2::nuku,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    NukuCompound {
        name: "抜く_compound",
        matcher_fn: crate::matchers::n2::nuku_compound,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nukide {
        name: "抜きで",
        matcher_fn: crate::matchers::n2::nukide,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Iyoiyo {
        name: "いよいよ",
        matcher_fn: crate::matchers::n2::iyoiyo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Zunisumu {
        name: "ずに済む",
        matcher_fn: crate::matchers::n2::zunisumu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nioujite {
        name: "に応じて",
        matcher_fn: crate::matchers::n2::nioujite,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    WotsuujiteU30fbWotooshite {
        name: "を通じて・を通して",
        matcher_fn: crate::matchers::n2::wotsuujite_u30fb_wotooshite,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nikotaete {
        name: "に応えて",
        matcher_fn: crate::matchers::n2::nikotaete,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Soretomo {
        name: "それとも",
        matcher_fn: crate::matchers::n2::soretomo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nishitara {
        name: "にしたら",
        matcher_fn: crate::matchers::n2::nishitara,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    NishitemoUff5eNishitemo {
        name: "にしても～にしても",
        matcher_fn: crate::matchers::n2::nishitemo_uff5e_nishitemo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Toshiteha {
        name: "としては",
        matcher_fn: crate::matchers::n2::toshiteha,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Toshitemo {
        name: "としても",
        matcher_fn: crate::matchers::n2::toshitemo,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Sorenishitemo {
        name: "それにしても",
        matcher_fn: crate::matchers::n2::sorenishitemo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nu {
        name: "ぬ",
        matcher_fn: crate::matchers::n2::nu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Kotonaku {
        name: "ことなく",
        matcher_fn: crate::matchers::n2::kotonaku,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nite {
        name: "にて",
        matcher_fn: crate::matchers::n2::nite,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Niha {
        name: "には",
        matcher_fn: crate::matchers::n2::niha,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Omouyouni {
        name: "思うように",
        matcher_fn: crate::matchers::n2::omouyouni,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    KatoomottaraU30fbKatoomouto {
        name: "かと思ったら・かと思うと",
        matcher_fn: crate::matchers::n2::katoomottara_u30fb_katoomouto,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Toiumonodemonai {
        name: "というものでもない",
        matcher_fn: crate::matchers::n2::toiumonodemonai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Tokangaerareru {
        name: "と考えられる",
        matcher_fn: crate::matchers::n2::tokangaerareru,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Toiutenkarakangaeruto {
        name: "という点から考えると",
        matcher_fn: crate::matchers::n2::toiutenkarakangaeruto,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Toiukotoha {
        name: "ということは",
        matcher_fn: crate::matchers::n2::toiukotoha,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Fuuni {
        name: "ふうに",
        matcher_fn: crate::matchers::n2::fuuni,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Toiukazeni {
        name: "という風に",
        matcher_fn: crate::matchers::n2::toiukazeni,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Monono {
        name: "ものの",
        matcher_fn: crate::matchers::n2::monono,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Toiumonoda {
        name: "というものだ",
        matcher_fn: crate::matchers::n2::toiumonoda,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Karamiruto {
        name: "から見ると",
        matcher_fn: crate::matchers::n2::karamiruto,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Tokorowomiruto {
        name: "ところを見ると",
        matcher_fn: crate::matchers::n2::tokorowomiruto,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    KarasurutoU30fbKarasureba {
        name: "からすると・からすれば",
        matcher_fn: crate::matchers::n2::karasuruto_u30fb_karasureba,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Karashite {
        name: "からして",
        matcher_fn: crate::matchers::n2::karashite,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Karatoitte {
        name: "からといって",
        matcher_fn: crate::matchers::n2::karatoitte,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Souieba {
        name: "そういえば",
        matcher_fn: crate::matchers::n2::souieba,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    OUff5eNegau {
        name: "お～願う",
        matcher_fn: crate::matchers::n2::o_uff5e_negau,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Uff5eTechoudai {
        name: "～て頂戴",
        matcher_fn: crate::matchers::n2::uff5e_techoudai,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Toka {
        name: "とか",
        matcher_fn: crate::matchers::n2::toka,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    U301cYoudehanaika {
        name: "〜ようではないか",
        matcher_fn: crate::matchers::n2::u301c_youdehanaika,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Kanoyouda {
        name: "かのようだ",
        matcher_fn: crate::matchers::n2::kanoyouda,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nodehanaidarouka {
        name: "のではないだろうか",
        matcher_fn: crate::matchers::n2::nodehanaidarouka,
        priority: 6,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Tetouzenda {
        name: "て当然だ",
        matcher_fn: crate::matchers::n2::tetouzenda,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nomotouzenda {
        name: "のも当然だ",
        matcher_fn: crate::matchers::n2::nomotouzenda,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    TattaNo {
        name: "たった(の)",
        matcher_fn: crate::matchers::n2::tatta_no,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Osoregaaru {
        name: "恐れがある",
        matcher_fn: crate::matchers::n2::osoregaaru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Osoraku {
        name: "おそらく",
        matcher_fn: crate::matchers::n2::osoraku,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Monoka {
        name: "ものか",
        matcher_fn: crate::matchers::n2::monoka,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Omakeni {
        name: "おまけに",
        matcher_fn: crate::matchers::n2::omakeni,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nikimatteiru {
        name: "に決まっている",
        matcher_fn: crate::matchers::n2::nikimatteiru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Kotoninatteiru {
        name: "ことになっている",
        matcher_fn: crate::matchers::n2::kotoninatteiru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Ki {
        name: "気",
        matcher_fn: crate::matchers::n2::ki,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Ge {
        name: "げ",
        matcher_fn: crate::matchers::n2::ge,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Kotodakara {
        name: "ことだから",
        matcher_fn: crate::matchers::n2::kotodakara,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Monodakara {
        name: "ものだから",
        matcher_fn: crate::matchers::n2::monodakara,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    MonodesukaraU30fbMonode {
        name: "ものですから・もので",
        matcher_fn: crate::matchers::n2::monodesukara_u30fb_monode,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Monogaaru {
        name: "ものがある",
        matcher_fn: crate::matchers::n2::monogaaru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Keikougaaru {
        name: "傾向がある",
        matcher_fn: crate::matchers::n2::keikougaaru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Uff5eNiataisuru {
        name: "～に値する",
        matcher_fn: crate::matchers::n2::uff5e_niataisuru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Teshouganai {
        name: "てしょうがない",
        matcher_fn: crate::matchers::n2::teshouganai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Dakemashida {
        name: "だけましだ",
        matcher_fn: crate::matchers::n2::dakemashida,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    SaiwaiU30fbSaiwainakotoni {
        name: "幸い・幸いなことに",
        matcher_fn: crate::matchers::n2::saiwai_u30fb_saiwainakotoni,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    YoudehaU30fbYouja {
        name: "ようでは・ようじゃ",
        matcher_fn: crate::matchers::n2::youdeha_u30fb_youja,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Sasuga {
        name: "さすが",
        matcher_fn: crate::matchers::n2::sasuga,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    KotohaU301cGa {
        name: "ことは〜が",
        matcher_fn: crate::matchers::n2::kotoha_u301c_ga,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Sarani {
        name: "更に",
        matcher_fn: crate::matchers::n2::sarani,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    KiyoshiU3005 {
        name: "精々",
        matcher_fn: crate::matchers::n2::kiyoshi_u3005,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Wazukani {
        name: "僅かに",
        matcher_fn: crate::matchers::n2::wazukani,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Oyobi {
        name: "および",
        matcher_fn: crate::matchers::n2::oyobi,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Tachimachi {
        name: "たちまち",
        matcher_fn: crate::matchers::n2::tachimachi,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Ikinari {
        name: "いきなり",
        matcher_fn: crate::matchers::n2::ikinari,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Toitta {
        name: "といった",
        matcher_fn: crate::matchers::n2::toitta,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Wokomete {
        name: "を込めて",
        matcher_fn: crate::matchers::n2::wokomete,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nikuwaete {
        name: "に加えて",
        matcher_fn: crate::matchers::n2::nikuwaete,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nanikarananimade {
        name: "何から何まで",
        matcher_fn: crate::matchers::n2::nanikarananimade,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Habetsutoshite {
        name: "は別として",
        matcher_fn: crate::matchers::n2::habetsutoshite,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Dakeni {
        name: "だけに",
        matcher_fn: crate::matchers::n2::dakeni,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Dakeha {
        name: "だけは",
        matcher_fn: crate::matchers::n2::dakeha,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Dakeatte {
        name: "だけあって",
        matcher_fn: crate::matchers::n2::dakeatte,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Naniyori {
        name: "何より",
        matcher_fn: crate::matchers::n2::naniyori,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nanitoittemo {
        name: "何といっても",
        matcher_fn: crate::matchers::n2::nanitoittemo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Kananika {
        name: "か何か",
        matcher_fn: crate::matchers::n2::kananika,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Tenaranai {
        name: "てならない",
        matcher_fn: crate::matchers::n2::tenaranai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nominarazu {
        name: "のみならず",
        matcher_fn: crate::matchers::n2::nominarazu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Sorenanoni {
        name: "それなのに",
        matcher_fn: crate::matchers::n2::sorenanoni,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Iwayuru {
        name: "いわゆる",
        matcher_fn: crate::matchers::n2::iwayuru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nisuginai {
        name: "にすぎない",
        matcher_fn: crate::matchers::n2::nisuginai,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    MoUff5eBaUff5eMo {
        name: "も～ば～も",
        matcher_fn: crate::matchers::n2::mo_uff5e_ba_uff5e_mo,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Deshikanai {
        name: "でしかない",
        matcher_fn: crate::matchers::n2::deshikanai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Tetamaranai {
        name: "てたまらない",
        matcher_fn: crate::matchers::n2::tetamaranai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    NiseyoU30fbNishiro {
        name: "にせよ・にしろ",
        matcher_fn: crate::matchers::n2::niseyo_u30fb_nishiro,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nanishiro {
        name: "何しろ",
        matcher_fn: crate::matchers::n2::nanishiro,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    NishiroUff5eNishiro {
        name: "にしろ～にしろ",
        matcher_fn: crate::matchers::n2::nishiro_uff5e_nishiro,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Hatomokaku {
        name: "はともかく",
        matcher_fn: crate::matchers::n2::hatomokaku,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Naratomokaku {
        name: "ならともかく",
        matcher_fn: crate::matchers::n2::naratomokaku,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    YaraUff5eYara {
        name: "やら～やら",
        matcher_fn: crate::matchers::n2::yara_uff5e_yara,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Shikashinagara {
        name: "しかしながら",
        matcher_fn: crate::matchers::n2::shikashinagara,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Kotonihanaranai {
        name: "ことにはならない",
        matcher_fn: crate::matchers::n2::kotonihanaranai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Dakenokotohaaru {
        name: "だけのことはある",
        matcher_fn: crate::matchers::n2::dakenokotohaaru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Tehanaranai {
        name: "てはならない",
        matcher_fn: crate::matchers::n2::tehanaranai,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Tehairarenai {
        name: "てはいられない",
        matcher_fn: crate::matchers::n2::tehairarenai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    RikuniUff5eNai {
        name: "陸に～ない",
        matcher_fn: crate::matchers::n2::rikuni_uff5e_nai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Shikamo {
        name: "しかも",
        matcher_fn: crate::matchers::n2::shikamo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Tedemo {
        name: "てでも",
        matcher_fn: crate::matchers::n2::tedemo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Tomo {
        name: "とも",
        matcher_fn: crate::matchers::n2::tomo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Naiwakenihaikanai {
        name: "ないわけにはいかない",
        matcher_fn: crate::matchers::n2::naiwakenihaikanai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Toiuwakedehanai {
        name: "というわけではない",
        matcher_fn: crate::matchers::n2::toiuwakedehanai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Nomomottomoda {
        name: "のももっともだ",
        matcher_fn: crate::matchers::n2::nomomottomoda,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    Tatte {
        name: "たって",
        matcher_fn: crate::matchers::n2::tatte,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    TatteNaku {
        name: "たって_naku",
        matcher_fn: crate::matchers::n2::tatte_naku,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    TatteIAdjKu {
        name: "たって_i_adj_ku",
        matcher_fn: crate::matchers::n2::tatte_i_adj_ku,
        priority: 4,
        category: PatternCategory::Construction,
        jlpt: "n2",
    },

    // ========== N1 PATTERNS ==========

    Toiu {
        name: "という",
        matcher_fn: crate::matchers::n1::toiu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Toiu1 {
        name: "という1",
        matcher_fn: crate::matchers::n1::toiu1,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    MamaNi {
        name: "まま(に)",
        matcher_fn: crate::matchers::n1::mama_ni,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    MamaNiWithNi {
        name: "まま(に)_with_ni",
        matcher_fn: crate::matchers::n1::mama_ni_with_ni,
        priority: 2,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    MamaNiNai {
        name: "まま(に)_nai",
        matcher_fn: crate::matchers::n1::mama_ni_nai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    MamaNiIAdj {
        name: "まま(に)_i_adj",
        matcher_fn: crate::matchers::n1::mama_ni_i_adj,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    MamaNiNaAdj {
        name: "まま(に)_na_adj",
        matcher_fn: crate::matchers::n1::mama_ni_na_adj,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    MamaNiNoun {
        name: "まま(に)_noun",
        matcher_fn: crate::matchers::n1::mama_ni_noun,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    MamaNi1 {
        name: "まま(に)1",
        matcher_fn: crate::matchers::n1::mama_ni_1,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Niitarumade {
        name: "に至るまで",
        matcher_fn: crate::matchers::n1::niitarumade,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Tatokorode {
        name: "たところで",
        matcher_fn: crate::matchers::n1::tatokorode,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    GotokuU30fbShikiU30fbGotoshi {
        name: "如く・如き・如し",
        matcher_fn: crate::matchers::n1::gotoku_u30fb_shiki_u30fb_gotoshi,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Nitaru {
        name: "に足る",
        matcher_fn: crate::matchers::n1::nitaru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    KiwamarinaiU30fbKiwamaru {
        name: "極まりない・極まる",
        matcher_fn: crate::matchers::n1::kiwamarinai_u30fb_kiwamaru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Toiedomo {
        name: "といえども",
        matcher_fn: crate::matchers::n1::toiedomo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Womotte {
        name: "を以て",
        matcher_fn: crate::matchers::n1::womotte,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    WomotteSplit {
        name: "を以て_split",
        matcher_fn: crate::matchers::n1::womotte_split,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Kiraigaaru {
        name: "きらいがある",
        matcher_fn: crate::matchers::n1::kiraigaaru,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Naramadashimo {
        name: "ならまだしも",
        matcher_fn: crate::matchers::n1::naramadashimo,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Mademonai {
        name: "までもない",
        matcher_fn: crate::matchers::n1::mademonai,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    TomonarutoU30fbNimonaruto {
        name: "ともなると・にもなると",
        matcher_fn: crate::matchers::n1::tomonaruto_u30fb_nimonaruto,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Woiikotoni {
        name: "をいいことに",
        matcher_fn: crate::matchers::n1::woiikotoni,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Ika2 {
        name: "如何",
        matcher_fn: crate::matchers::n1::ika_2,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Uff5eRumadeda {
        name: "～るまでだ",
        matcher_fn: crate::matchers::n1::uff5e_rumadeda,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Niatte {
        name: "にあって",
        matcher_fn: crate::matchers::n1::niatte,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Woyoginakusareru {
        name: "を余儀なくされる",
        matcher_fn: crate::matchers::n1::woyoginakusareru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Toha {
        name: "とは",
        matcher_fn: crate::matchers::n1::toha,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Jaarumaishi {
        name: "じゃあるまいし",
        matcher_fn: crate::matchers::n1::jaarumaishi,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Tekaratoiumono {
        name: "てからというもの",
        matcher_fn: crate::matchers::n1::tekaratoiumono,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Katawara {
        name: "かたわら",
        matcher_fn: crate::matchers::n1::katawara,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Wokawakirini {
        name: "を皮切りに",
        matcher_fn: crate::matchers::n1::wokawakirini,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Niitatteha {
        name: "に至っては",
        matcher_fn: crate::matchers::n1::niitatteha,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Nari {
        name: "なり",
        matcher_fn: crate::matchers::n1::nari,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    TomonakuU30fbTomonashini {
        name: "ともなく・ともなしに",
        matcher_fn: crate::matchers::n1::tomonaku_u30fb_tomonashini,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    NureCompound {
        name: "塗れ",
        matcher_fn: crate::matchers::n1::nure_compound,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Nure {
        name: "塗れ",
        matcher_fn: crate::matchers::n1::nure,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    YougaUff5eMaiga {
        name: "ようが～まいが",
        matcher_fn: crate::matchers::n1::youga_uff5e_maiga,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Karasuru {
        name: "からする",
        matcher_fn: crate::matchers::n1::karasuru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Karano {
        name: "からの",
        matcher_fn: crate::matchers::n1::karano,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    NishiteU2460 {
        name: "にして①",
        matcher_fn: crate::matchers::n1::nishite_u2460,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Monowo {
        name: "ものを",
        matcher_fn: crate::matchers::n1::monowo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Deare {
        name: "であれ",
        matcher_fn: crate::matchers::n1::deare,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    WooitehokaniU301cNai {
        name: "をおいてほかに〜ない",
        matcher_fn: crate::matchers::n1::wooitehokani_u301c_nai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Womotte2 {
        name: "をもって",
        matcher_fn: crate::matchers::n1::womotte_2,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Tohaie {
        name: "とはいえ",
        matcher_fn: crate::matchers::n1::tohaie,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Naradeha {
        name: "ならでは",
        matcher_fn: crate::matchers::n1::naradeha,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Sura {
        name: "すら",
        matcher_fn: crate::matchers::n1::sura,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Atteno {
        name: "あっての",
        matcher_fn: crate::matchers::n1::atteno,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Uff5eTamadeda {
        name: "～たまでだ",
        matcher_fn: crate::matchers::n1::uff5e_tamadeda,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Wohete {
        name: "を経て",
        matcher_fn: crate::matchers::n1::wohete,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Nagarani {
        name: "ながらに",
        matcher_fn: crate::matchers::n1::nagarani,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    TanariU30fbNari {
        name: "たなり・なり",
        matcher_fn: crate::matchers::n1::tanari_u30fb_nari,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Nokiwami {
        name: "の極み",
        matcher_fn: crate::matchers::n1::nokiwami,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Nishitemireba {
        name: "にしてみれば",
        matcher_fn: crate::matchers::n1::nishitemireba,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Dano {
        name: "だの",
        matcher_fn: crate::matchers::n1::dano,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    DanoSplit {
        name: "だの_split",
        matcher_fn: crate::matchers::n1::dano_split,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Akumademo {
        name: "あくまでも",
        matcher_fn: crate::matchers::n1::akumademo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Beku {
        name: "べく",
        matcher_fn: crate::matchers::n1::beku,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Tokorowo {
        name: "ところを",
        matcher_fn: crate::matchers::n1::tokorowo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Karaaru {
        name: "からある",
        matcher_fn: crate::matchers::n1::karaaru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    NishiteU2461 {
        name: "にして②",
        matcher_fn: crate::matchers::n1::nishite_u2461,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    TsuU301cTsu {
        name: "つ〜つ",
        matcher_fn: crate::matchers::n1::tsu_u301c_tsu,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    AkumadeMo {
        name: "飽くまで(も)",
        matcher_fn: crate::matchers::n1::akumade_mo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    DeareU301cDeare {
        name: "であれ〜であれ",
        matcher_fn: crate::matchers::n1::deare_u301c_deare,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Tarasaigo {
        name: "たら最後",
        matcher_fn: crate::matchers::n1::tarasaigo,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Ikanaru {
        name: "いかなる",
        matcher_fn: crate::matchers::n1::ikanaru,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Narini {
        name: "なりに",
        matcher_fn: crate::matchers::n1::narini,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    NariniSorenari {
        name: "なりに",
        matcher_fn: crate::matchers::n1::narini_sorenari,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    ReruU30fbRareruMamani {
        name: "れる・られる + ままに",
        matcher_fn: crate::matchers::n1::reru_u30fb_rareru_mamani,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Nimatsuwaru {
        name: "にまつわる",
        matcher_fn: crate::matchers::n1::nimatsuwaru,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Taru {
        name: "たる",
        matcher_fn: crate::matchers::n1::taru,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    NaraU301cDe {
        name: "なら〜で",
        matcher_fn: crate::matchers::n1::nara_u301c_de,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Womonotomosezu {
        name: "をものともせず",
        matcher_fn: crate::matchers::n1::womonotomosezu,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Nihaataranai {
        name: "には当たらない",
        matcher_fn: crate::matchers::n1::nihaataranai,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Monotoomou {
        name: "ものと思う",
        matcher_fn: crate::matchers::n1::monotoomou,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Wofumaete {
        name: "を踏まえて",
        matcher_fn: crate::matchers::n1::wofumaete,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Yueni {
        name: "ゆえに",
        matcher_fn: crate::matchers::n1::yueni,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Nitodomarazu {
        name: "にとどまらず",
        matcher_fn: crate::matchers::n1::nitodomarazu,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Toomoikiya {
        name: "と思いきや",
        matcher_fn: crate::matchers::n1::toomoikiya,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Dounimo {
        name: "どうにも",
        matcher_fn: crate::matchers::n1::dounimo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    DounimoSplit {
        name: "どうにも",
        matcher_fn: crate::matchers::n1::dounimo_split,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Kotodashi {
        name: "ことだし",
        matcher_fn: crate::matchers::n1::kotodashi,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    KotodashiCompound {
        name: "ことだし_compound",
        matcher_fn: crate::matchers::n1::kotodashi_compound,
        priority: 2,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    GanUff5e {
        name: "がん～",
        matcher_fn: crate::matchers::n1::gan_uff5e,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Kainaka {
        name: "か否か",
        matcher_fn: crate::matchers::n1::kainaka,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    KainakaKaInaka {
        name: "か否か",
        matcher_fn: crate::matchers::n1::kainaka_ka_inaka,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    TaraU301cDe {
        name: "たら〜で",
        matcher_fn: crate::matchers::n1::tara_u301c_de,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Bekushite {
        name: "べくして",
        matcher_fn: crate::matchers::n1::bekushite,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    KareU301cKare {
        name: "かれ〜かれ",
        matcher_fn: crate::matchers::n1::kare_u301c_kare,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    U301cNiU301cNai {
        name: "〜に〜ない",
        matcher_fn: crate::matchers::n1::u301c_ni_u301c_nai,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    NakushiteHa {
        name: "なくして(は)",
        matcher_fn: crate::matchers::n1::nakushite_ha,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Nonannotte {
        name: "のなんのって",
        matcher_fn: crate::matchers::n1::nonannotte,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    NonannotteVerb {
        name: "のなんのって",
        matcher_fn: crate::matchers::n1::nonannotte_verb,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Nikakatteiru {
        name: "にかかっている",
        matcher_fn: crate::matchers::n1::nikakatteiru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Teyamanai {
        name: "てやまない",
        matcher_fn: crate::matchers::n1::teyamanai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Gurainara {
        name: "ぐらいなら",
        matcher_fn: crate::matchers::n1::gurainara,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    TtebaU30fbTtara {
        name: "ってば・ったら",
        matcher_fn: crate::matchers::n1::tteba_u30fb_ttara,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Zutomo {
        name: "ずとも",
        matcher_fn: crate::matchers::n1::zutomo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    ZutomoSplit {
        name: "ずとも",
        matcher_fn: crate::matchers::n1::zutomo_split,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Toatte {
        name: "とあって",
        matcher_fn: crate::matchers::n1::toatte,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Demonandemonai {
        name: "でもなんでもない",
        matcher_fn: crate::matchers::n1::demonandemonai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Gurumide {
        name: "ぐるみで",
        matcher_fn: crate::matchers::n1::gurumide,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Sobakara {
        name: "そばから",
        matcher_fn: crate::matchers::n1::sobakara,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    WakeariYakuatte {
        name: "訳あり(訳あって)",
        matcher_fn: crate::matchers::n1::wakeari_yakuatte,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    NiitatteU30fbNiitari {
        name: "に至って・に至り",
        matcher_fn: crate::matchers::n1::niitatte_u30fb_niitari,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    DaniShinai {
        name: "だに + しない",
        matcher_fn: crate::matchers::n1::dani_shinai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Gatera {
        name: "がてら",
        matcher_fn: crate::matchers::n1::gatera,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    NgatameNi {
        name: "んがため(に)",
        matcher_fn: crate::matchers::n1::ngatame_ni,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    IkanU301cZu {
        name: "いかん〜ず",
        matcher_fn: crate::matchers::n1::ikan_u301c_zu,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    NimoUff5eNai {
        name: "にも～ない",
        matcher_fn: crate::matchers::n1::nimo_uff5e_nai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    IAdjKuMonantomonai {
        name: "い-Adj[く] + もなんともない",
        matcher_fn: crate::matchers::n1::i_adj_ku_monantomonai,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    VerbDani {
        name: "Verb + だに",
        matcher_fn: crate::matchers::n1::verb_dani,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Uff5eNariUff5eNari {
        name: "～なり～なり",
        matcher_fn: crate::matchers::n1::uff5e_nari_uff5e_nari,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Naidemonai {
        name: "ないでもない",
        matcher_fn: crate::matchers::n1::naidemonai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Mosarukotonagara {
        name: "もさることながら",
        matcher_fn: crate::matchers::n1::mosarukotonagara,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Monotoomotteita {
        name: "ものと思っていた",
        matcher_fn: crate::matchers::n1::monotoomotteita,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Denakutenandarou {
        name: "でなくてなんだろう",
        matcher_fn: crate::matchers::n1::denakutenandarou,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    HasateokiU30fbHasateoite {
        name: "はさておき・はさておいて",
        matcher_fn: crate::matchers::n1::hasateoki_u30fb_hasateoite,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Oriniha {
        name: "折には",
        matcher_fn: crate::matchers::n1::oriniha,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    TobakariUff08NiUff09 {
        name: "とばかり（に）",
        matcher_fn: crate::matchers::n1::tobakari_uff08_ni_uff09,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    WaU301cWa {
        name: "わ〜わ",
        matcher_fn: crate::matchers::n1::wa_u301c_wa,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Naritomo {
        name: "なりとも",
        matcher_fn: crate::matchers::n1::naritomo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Niitattemo {
        name: "に至っても",
        matcher_fn: crate::matchers::n1::niitattemo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Wokanete {
        name: "を兼ねて",
        matcher_fn: crate::matchers::n1::wokanete,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    VerbNaiMonoDarouKa {
        name: "Verb[ない]もの(だろう)か",
        matcher_fn: crate::matchers::n1::verb_nai_mono_darou_ka,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    VerbTeMiseru {
        name: "Verb[て] + みせる",
        matcher_fn: crate::matchers::n1::verb_te_miseru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Aimatte {
        name: "相まって",
        matcher_fn: crate::matchers::n1::aimatte,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Nitarinai {
        name: "に足りない",
        matcher_fn: crate::matchers::n1::nitarinai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Bekarazu {
        name: "べからず",
        matcher_fn: crate::matchers::n1::bekarazu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Nbakarini {
        name: "んばかりに",
        matcher_fn: crate::matchers::n1::nbakarini,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    NinottotteU30fbNinottori {
        name: "に則って・に則り",
        matcher_fn: crate::matchers::n1::ninottotte_u30fb_ninottori,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Adjkagirida {
        name: "Adj限りだ",
        matcher_fn: crate::matchers::n1::adjkagirida,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Haoroka {
        name: "はおろか",
        matcher_fn: crate::matchers::n1::haoroka,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    MekuU30fbMeita {
        name: "めく・めいた",
        matcher_fn: crate::matchers::n1::meku_u30fb_meita,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Toiwazu {
        name: "といわず",
        matcher_fn: crate::matchers::n1::toiwazu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Nimohodogaaru {
        name: "にもほどがある",
        matcher_fn: crate::matchers::n1::nimohodogaaru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Nimomashite {
        name: "にもまして",
        matcher_fn: crate::matchers::n1::nimomashite,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Makuru {
        name: "まくる",
        matcher_fn: crate::matchers::n1::makuru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    WaU301cWaUff08DeUff09 {
        name: "わ〜わ（で）",
        matcher_fn: crate::matchers::n1::wa_u301c_wa_uff08_de_uff09,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Dounika {
        name: "どうにか",
        matcher_fn: crate::matchers::n1::dounika,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Yainaya {
        name: "や否や",
        matcher_fn: crate::matchers::n1::yainaya,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Shidaidesu {
        name: "次第です",
        matcher_fn: crate::matchers::n1::shidaidesu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Toiutokoro {
        name: "というところ",
        matcher_fn: crate::matchers::n1::toiutokoro,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    IchiUff5eTaritomoUff5eNai {
        name: "１～たりとも～ない",
        matcher_fn: crate::matchers::n1::ichi_uff5e_taritomo_uff5e_nai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    TtaranaiU30fbToittaranai {
        name: "ったらない・といったらない",
        matcher_fn: crate::matchers::n1::ttaranai_u30fb_toittaranai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    NiterashiteU30fbNiterasuto {
        name: "に照らして・に照らすと",
        matcher_fn: crate::matchers::n1::niterashite_u30fb_niterasuto,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Toareba {
        name: "とあれば",
        matcher_fn: crate::matchers::n1::toareba,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Sazo {
        name: "さぞ",
        matcher_fn: crate::matchers::n1::sazo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Tokitara {
        name: "ときたら",
        matcher_fn: crate::matchers::n1::tokitara,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Biru {
        name: "びる",
        matcher_fn: crate::matchers::n1::biru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Nishitatokorode {
        name: "にしたところで",
        matcher_fn: crate::matchers::n1::nishitatokorode,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Uff5eBakoso {
        name: "～ばこそ",
        matcher_fn: crate::matchers::n1::uff5e_bakoso,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Temosashitsukaenai {
        name: "ても差し支えない",
        matcher_fn: crate::matchers::n1::temosashitsukaenai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    NihaoyobanaiU2460 {
        name: "には及ばない①",
        matcher_fn: crate::matchers::n1::nihaoyobanai_u2460,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Nisokushite {
        name: "に即して",
        matcher_fn: crate::matchers::n1::nisokushite,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Naimademo {
        name: "ないまでも",
        matcher_fn: crate::matchers::n1::naimademo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Woyosoni {
        name: "をよそに",
        matcher_fn: crate::matchers::n1::woyosoni,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Nikagittakotodehanai {
        name: "に限ったことではない",
        matcher_fn: crate::matchers::n1::nikagittakotodehanai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Tohakurabemononinaranai {
        name: "とは比べものにならない",
        matcher_fn: crate::matchers::n1::tohakurabemononinaranai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Majiki {
        name: "まじき",
        matcher_fn: crate::matchers::n1::majiki,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Noitari {
        name: "の至り",
        matcher_fn: crate::matchers::n1::noitari,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Nihajinai {
        name: "に恥じない",
        matcher_fn: crate::matchers::n1::nihajinai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Zujimai {
        name: "ずじまい",
        matcher_fn: crate::matchers::n1::zujimai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    NiiwaserebaU30fbNiiwaserutoU30fbNiiwasetara {
        name: "に言わせれば・に言わせると・に言わせたら",
        matcher_fn: crate::matchers::n1::niiwasereba_u30fb_niiwaseruto_u30fb_niiwasetara,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    TtaraU30fbToittara {
        name: "ったら・といったら",
        matcher_fn: crate::matchers::n1::ttara_u30fb_toittara,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Kototote {
        name: "こととて",
        matcher_fn: crate::matchers::n1::kototote,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Zukume {
        name: "ずくめ",
        matcher_fn: crate::matchers::n1::zukume,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    NihaoyobanaiU2461 {
        name: "には及ばない②",
        matcher_fn: crate::matchers::n1::nihaoyobanai_u2461,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Tohaiumonono {
        name: "とは言うものの",
        matcher_fn: crate::matchers::n1::tohaiumonono,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Gahayaika {
        name: "が早いか",
        matcher_fn: crate::matchers::n1::gahayaika,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Nikatakunai {
        name: "に難くない",
        matcher_fn: crate::matchers::n1::nikatakunai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Naraizashirazu {
        name: "ならいざ知らず",
        matcher_fn: crate::matchers::n1::naraizashirazu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Wokinjienai {
        name: "を禁じ得ない",
        matcher_fn: crate::matchers::n1::wokinjienai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Nikakotsukete {
        name: "にかこつけて",
        matcher_fn: crate::matchers::n1::nikakotsukete,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Youniyotteha {
        name: "ようによっては",
        matcher_fn: crate::matchers::n1::youniyotteha,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Bekumonai {
        name: "べくもない",
        matcher_fn: crate::matchers::n1::bekumonai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Tokitara2 {
        name: "と来たら",
        matcher_fn: crate::matchers::n1::tokitara_2,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Monotoshite {
        name: "ものとして",
        matcher_fn: crate::matchers::n1::monotoshite,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Wozenteini {
        name: "を前提に",
        matcher_fn: crate::matchers::n1::wozenteini,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Zunihasumanai {
        name: "ずにはすまない",
        matcher_fn: crate::matchers::n1::zunihasumanai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Nikotaenai {
        name: "に堪えない",
        matcher_fn: crate::matchers::n1::nikotaenai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Shimatsuda {
        name: "始末だ",
        matcher_fn: crate::matchers::n1::shimatsuda,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    MononaraU2461 {
        name: "ものなら②",
        matcher_fn: crate::matchers::n1::mononara_u2461,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Nihikikae {
        name: "にひきかえ",
        matcher_fn: crate::matchers::n1::nihikikae,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Soremadeda {
        name: "それまでだ",
        matcher_fn: crate::matchers::n1::soremadeda,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Toiouka {
        name: "といおうか",
        matcher_fn: crate::matchers::n1::toiouka,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Zunihaokanai {
        name: "ずにはおかない",
        matcher_fn: crate::matchers::n1::zunihaokanai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Wokagirini {
        name: "を限りに",
        matcher_fn: crate::matchers::n1::wokagirini,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Tehakanawanai {
        name: "てはかなわない",
        matcher_fn: crate::matchers::n1::tehakanawanai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Katagata {
        name: "かたがた",
        matcher_fn: crate::matchers::n1::katagata,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Woyoginakusaseru {
        name: "を余儀なくさせる",
        matcher_fn: crate::matchers::n1::woyoginakusaseru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Uff5eTeyaru {
        name: "～てやる",
        matcher_fn: crate::matchers::n1::uff5e_teyaru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    TadaU301cNomi {
        name: "ただ〜のみ",
        matcher_fn: crate::matchers::n1::tada_u301c_nomi,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Monotosuru {
        name: "ものとする",
        matcher_fn: crate::matchers::n1::monotosuru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Tono {
        name: "との",
        matcher_fn: crate::matchers::n1::tono,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Izen {
        name: "以前",
        matcher_fn: crate::matchers::n1::izen,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Tomoarou {
        name: "ともあろう",
        matcher_fn: crate::matchers::n1::tomoarou,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    KososureU301cNai {
        name: "こそすれ〜ない",
        matcher_fn: crate::matchers::n1::kososure_u301c_nai,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Nami {
        name: "並み",
        matcher_fn: crate::matchers::n1::nami,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Nisakigakete {
        name: "に先駆けて",
        matcher_fn: crate::matchers::n1::nisakigakete,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Wokini {
        name: "を機に",
        matcher_fn: crate::matchers::n1::wokini,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    YueniConjunction {
        name: "ゆえに_conjunction",
        matcher_fn: crate::matchers::n1::yueni_conjunction,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    Yueno {
        name: "ゆえの",
        matcher_fn: crate::matchers::n1::yueno,
        priority: 5,
        category: PatternCategory::Construction,
        jlpt: "n1",
    },

    // ========== NT PATTERNS ==========

    Zo {
        name: "ぞ",
        matcher_fn: crate::matchers::nt::zo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "nt",
    },

    Ze {
        name: "ぜ",
        matcher_fn: crate::matchers::nt::ze,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "nt",
    },

    Wa {
        name: "わ",
        matcher_fn: crate::matchers::nt::wa,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "nt",
    },

    I {
        name: "い",
        matcher_fn: crate::matchers::nt::i,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "nt",
    },

    NSlang {
        name: "ん (Slang)",
        matcher_fn: crate::matchers::nt::n_slang,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "nt",
    },

    TsuSlang {
        name: "つ (Slang)",
        matcher_fn: crate::matchers::nt::tsu_slang,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "nt",
    },

    Uff5eYagaru {
        name: "～やがる",
        matcher_fn: crate::matchers::nt::uff5e_yagaru,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "nt",
    },

    Gaii {
        name: "がいい",
        matcher_fn: crate::matchers::nt::gaii,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "nt",
    },

    Karou {
        name: "かろう",
        matcher_fn: crate::matchers::nt::karou,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "nt",
    },

    Yaya {
        name: "やや",
        matcher_fn: crate::matchers::nt::yaya,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "nt",
    },

    Izuremo {
        name: "いずれも",
        matcher_fn: crate::matchers::nt::izuremo,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "nt",
    },

    Awayokuba {
        name: "あわよくば",
        matcher_fn: crate::matchers::nt::awayokuba,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "nt",
    },

    Muzu {
        name: "むず",
        matcher_fn: crate::matchers::nt::muzu,
        priority: 1,
        category: PatternCategory::Construction,
        jlpt: "nt",
    },
}

pub fn create_pattern_matcher() -> PatternMatcher {
    let mut matcher = PatternMatcher::new();

    let matcher_patterns: Vec<_> = Pattern::all()
        .into_iter()
        .map(|p| p.grammar_pattern())
        .collect();

    matcher.add_patterns(matcher_patterns);

    matcher
}

pub fn get_all_patterns() -> Vec<(GrammarPattern, &'static str)> {
    Pattern::all()
        .into_iter()
        .map(|p| (p.grammar_pattern(), p.grammar_pattern().jlpt_level))
        .collect()
}
