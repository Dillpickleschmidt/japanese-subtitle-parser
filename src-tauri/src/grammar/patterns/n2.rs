use crate::grammar::pattern_matcher::{GrammarPattern, TokenMatcher};
use crate::grammar::types::ConjugationPattern;

/// JLPT N2 level grammar patterns (upper intermediate forms)
pub fn get_patterns() -> Vec<(GrammarPattern, ConjugationPattern, &'static str)> {
    vec![
        // Phase 1: Adverbs & Standalone Expressions (18 patterns)
        (
            GrammarPattern {
                name: "toutei",
                tokens: vec![TokenMatcher::Surface("とうてい")],
                priority: 5,
            },
            ConjugationPattern::Toutei,
            "n2",
        ),
        (
            GrammarPattern {
                name: "yoppodo",
                tokens: vec![TokenMatcher::Surface("よっぽど")],
                priority: 5,
            },
            ConjugationPattern::Yoppodo,
            "n2",
        ),
        (
            GrammarPattern {
                name: "iyoiyo",
                tokens: vec![TokenMatcher::Surface("いよいよ")],
                priority: 5,
            },
            ConjugationPattern::Iyoiyo,
            "n2",
        ),
        (
            GrammarPattern {
                name: "sekkaku",
                tokens: vec![TokenMatcher::Surface("せっかく")],
                priority: 5,
            },
            ConjugationPattern::Sekkaku,
            "n2",
        ),
        // yamuoezu: Kagome tokenizes as verb やむをえる + ず
        (
            GrammarPattern {
                name: "yamuoezu_verb",
                tokens: vec![
                    TokenMatcher::specific_verb("やむをえる"),
                    TokenMatcher::Surface("ず"),
                ],
                priority: 7,
            },
            ConjugationPattern::Yamuoezu,
            "n2",
        ),
        (
            GrammarPattern {
                name: "yappari",
                tokens: vec![TokenMatcher::Surface("やっぱり")],
                priority: 5,
            },
            ConjugationPattern::Yappari,
            "n2",
        ),
        (
            GrammarPattern {
                name: "narubeku",
                tokens: vec![TokenMatcher::Surface("なるべく")],
                priority: 5,
            },
            ConjugationPattern::Narubeku,
            "n2",
        ),
        (
            GrammarPattern {
                name: "tashika",
                tokens: vec![TokenMatcher::Surface("たしか")],
                priority: 5,
            },
            ConjugationPattern::Tashika,
            "n2",
        ),
        (
            GrammarPattern {
                name: "man_ichi",
                tokens: vec![TokenMatcher::Surface("万一")],
                priority: 5,
            },
            ConjugationPattern::ManIchi,
            "n2",
        ),
        (
            GrammarPattern {
                name: "man_ichi_kana",
                tokens: vec![TokenMatcher::Surface("まんいち")],
                priority: 5,
            },
            ConjugationPattern::ManIchi,
            "n2",
        ),
        (
            GrammarPattern {
                name: "nanishiro",
                tokens: vec![TokenMatcher::Surface("なにしろ")],
                priority: 5,
            },
            ConjugationPattern::Nanishiro,
            "n2",
        ),
        (
            GrammarPattern {
                name: "nanishiro_kanji",
                tokens: vec![TokenMatcher::Surface("何しろ")],
                priority: 5,
            },
            ConjugationPattern::Nanishiro,
            "n2",
        ),
        (
            GrammarPattern {
                name: "sorenishitemo",
                tokens: vec![TokenMatcher::Surface("それにしても")],
                priority: 5,
            },
            ConjugationPattern::SoreniShitemo,
            "n2",
        ),
        (
            GrammarPattern {
                name: "tachimachi",
                tokens: vec![TokenMatcher::Surface("たちまち")],
                priority: 5,
            },
            ConjugationPattern::Tachimachi,
            "n2",
        ),
        // sasugani: Kagome tokenizes as さすが + に
        (
            GrammarPattern {
                name: "sasugani_split",
                tokens: vec![TokenMatcher::Surface("さすが"), TokenMatcher::Surface("に")],
                priority: 6,
            },
            ConjugationPattern::Sasugani,
            "n2",
        ),
        (
            GrammarPattern {
                name: "itsunomanika",
                tokens: vec![TokenMatcher::Surface("いつのまにか")],
                priority: 5,
            },
            ConjugationPattern::Itsunomanika,
            "n2",
        ),
        (
            GrammarPattern {
                name: "itsunomanika_split",
                tokens: vec![TokenMatcher::Surface("いつの間にか")],
                priority: 5,
            },
            ConjugationPattern::Itsunomanika,
            "n2",
        ),
        (
            GrammarPattern {
                name: "aete",
                tokens: vec![TokenMatcher::Surface("あえて")],
                priority: 5,
            },
            ConjugationPattern::Aete,
            "n2",
        ),
        (
            GrammarPattern {
                name: "semete",
                tokens: vec![TokenMatcher::Surface("せめて")],
                priority: 5,
            },
            ConjugationPattern::Semete,
            "n2",
        ),
        // nantoittemo: Kagome tokenizes kanji version as 何+と+いっ+て+も, kana as なんと(adverb)+いっ+て+も
        (
            GrammarPattern {
                name: "nantoittemo_split_kanji",
                tokens: vec![
                    TokenMatcher::Surface("何"),
                    TokenMatcher::Surface("と"),
                    TokenMatcher::Surface("いっ"),
                    TokenMatcher::Surface("て"),
                    TokenMatcher::Surface("も"),
                ],
                priority: 9,
            },
            ConjugationPattern::Nantoittemo,
            "n2",
        ),
        (
            GrammarPattern {
                name: "nantoittemo_adverb_split",
                tokens: vec![
                    TokenMatcher::Surface("なんと"),
                    TokenMatcher::Surface("いっ"),
                    TokenMatcher::Surface("て"),
                    TokenMatcher::Surface("も"),
                ],
                priority: 9,
            },
            ConjugationPattern::Nantoittemo,
            "n2",
        ),
        (
            GrammarPattern {
                name: "rou_ni",
                tokens: vec![TokenMatcher::Surface("ろくに")],
                priority: 5,
            },
            ConjugationPattern::RouNi,
            "n2",
        ),
        // Phase 2: Verb Suffixes & Auxiliaries (11 patterns)
        (
            GrammarPattern {
                name: "kaneru",
                tokens: vec![
                    TokenMatcher::verb_with_form("連用形"),
                    TokenMatcher::specific_verb("かねる"),
                ],
                priority: 7,
            },
            ConjugationPattern::Kaneru,
            "n2",
        ),
        (
            GrammarPattern {
                name: "kanenai",
                tokens: vec![
                    TokenMatcher::verb_with_form("連用形"),
                    TokenMatcher::specific_verb("かねる"),
                    TokenMatcher::Surface("ない"),
                ],
                priority: 8,
            },
            ConjugationPattern::Kanenai,
            "n2",
        ),
        (
            GrammarPattern {
                name: "tamaranai",
                tokens: vec![TokenMatcher::Any, TokenMatcher::Surface("たまらない")],
                priority: 6,
            },
            ConjugationPattern::Tamaranai,
            "n2",
        ),
        // naide_sumu: Kagome tokenizes ないで as ない + で
        (
            GrammarPattern {
                name: "naide_sumu_split",
                tokens: vec![
                    TokenMatcher::verb_with_form("未然形"),
                    TokenMatcher::Surface("ない"),
                    TokenMatcher::Surface("で"),
                    TokenMatcher::specific_verb("済む"),
                ],
                priority: 9,
            },
            ConjugationPattern::NaideSumu,
            "n2",
        ),
        (
            GrammarPattern {
                name: "kara_naru",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("から"),
                    TokenMatcher::specific_verb("なる"),
                ],
                priority: 7,
            },
            ConjugationPattern::KaraNaru,
            "n2",
        ),
        (
            GrammarPattern {
                name: "yori_shikata_ganai",
                tokens: vec![
                    TokenMatcher::Any, // Verb dictionary form or noun
                    TokenMatcher::Surface("より"),
                    TokenMatcher::Surface("仕方"),
                    TokenMatcher::Surface("が"),
                    TokenMatcher::Surface("ない"),
                ],
                priority: 9,
            },
            ConjugationPattern::YoriShikataGanai,
            "n2",
        ),
        (
            GrammarPattern {
                name: "yori_shikata_ganai_kana",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("より"),
                    TokenMatcher::Surface("しかた"),
                    TokenMatcher::Surface("が"),
                    TokenMatcher::Surface("ない"),
                ],
                priority: 9,
            },
            ConjugationPattern::YoriShikataGanai,
            "n2",
        ),
        (
            GrammarPattern {
                name: "ta_ue_de",
                tokens: vec![
                    TokenMatcher::Any, // Verb past form
                    TokenMatcher::Surface("上"),
                    TokenMatcher::Surface("で"),
                ],
                priority: 7,
            },
            ConjugationPattern::TaUeDe,
            "n2",
        ),
        // ni_ataru: Kagome tokenizes as compound particles にあたる/に当たる
        (
            GrammarPattern {
                name: "ni_ataru_compound",
                tokens: vec![TokenMatcher::Any, TokenMatcher::Surface("にあたる")],
                priority: 7,
            },
            ConjugationPattern::NiAtaru,
            "n2",
        ),
        (
            GrammarPattern {
                name: "ni_ataru_compound_kanji",
                tokens: vec![TokenMatcher::Any, TokenMatcher::Surface("に当たる")],
                priority: 7,
            },
            ConjugationPattern::NiAtaru,
            "n2",
        ),
        (
            GrammarPattern {
                name: "gotoshi",
                tokens: vec![TokenMatcher::Any, TokenMatcher::Surface("ごとし")],
                priority: 6,
            },
            ConjugationPattern::Gotoshi,
            "n2",
        ),
        (
            GrammarPattern {
                name: "gotoshi_kanji",
                tokens: vec![TokenMatcher::Any, TokenMatcher::Surface("如し")],
                priority: 6,
            },
            ConjugationPattern::Gotoshi,
            "n2",
        ),
        // tsuujite: Kagome tokenizes as verb 通じる/つうじる + て
        (
            GrammarPattern {
                name: "tsuujite_verb_kanji",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::specific_verb("通じる"),
                    TokenMatcher::Surface("て"),
                ],
                priority: 7,
            },
            ConjugationPattern::Tsuujite,
            "n2",
        ),
        (
            GrammarPattern {
                name: "tsuujite_verb_kana",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::specific_verb("つうじる"),
                    TokenMatcher::Surface("て"),
                ],
                priority: 7,
            },
            ConjugationPattern::Tsuujite,
            "n2",
        ),
        (
            GrammarPattern {
                name: "noboru",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("に"),
                    TokenMatcher::specific_verb("のぼる"),
                ],
                priority: 7,
            },
            ConjugationPattern::Noboru,
            "n2",
        ),
        // Phase 3: Particle Patterns & Conjunctions (11 patterns)
        // gatera: Kagome tokenizes as が + てら
        (
            GrammarPattern {
                name: "gatera_split",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("が"),
                    TokenMatcher::Surface("てら"),
                ],
                priority: 8,
            },
            ConjugationPattern::Gatera,
            "n2",
        ),
        (
            GrammarPattern {
                name: "oyobi",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("および"),
                    TokenMatcher::Any,
                ],
                priority: 7,
            },
            ConjugationPattern::Oyobi,
            "n2",
        ),
        (
            GrammarPattern {
                name: "oyobi_kanji",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("及び"),
                    TokenMatcher::Any,
                ],
                priority: 7,
            },
            ConjugationPattern::Oyobi,
            "n2",
        ),
        (
            GrammarPattern {
                name: "katawara_kanji",
                tokens: vec![TokenMatcher::Any, TokenMatcher::Surface("傍ら")],
                priority: 6,
            },
            ConjugationPattern::Katawara,
            "n2",
        ),
        (
            GrammarPattern {
                name: "katawara_kana",
                tokens: vec![TokenMatcher::Any, TokenMatcher::Surface("かたわら")],
                priority: 6,
            },
            ConjugationPattern::Katawara,
            "n2",
        ),
        (
            GrammarPattern {
                name: "sei_ka",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("せい"),
                    TokenMatcher::Surface("か"),
                ],
                priority: 7,
            },
            ConjugationPattern::SeiKa,
            "n2",
        ),
        // yueni: Kagome tokenizes as ゆえ + に
        (
            GrammarPattern {
                name: "yueni_split",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("ゆえ"),
                    TokenMatcher::Surface("に"),
                ],
                priority: 7,
            },
            ConjugationPattern::Yueni,
            "n2",
        ),
        // ippou_dewa: Kagome tokenizes では as で + は
        (
            GrammarPattern {
                name: "ippou_dewa_split",
                tokens: vec![
                    TokenMatcher::Surface("一方"),
                    TokenMatcher::Surface("で"),
                    TokenMatcher::Surface("は"),
                ],
                priority: 8,
            },
            ConjugationPattern::IppouDewa,
            "n2",
        ),
        (
            GrammarPattern {
                name: "mono_no",
                tokens: vec![TokenMatcher::Any, TokenMatcher::Surface("ものの")],
                priority: 6,
            },
            ConjugationPattern::MonoNo,
            "n2",
        ),
        // kuse_ni: Kagome tokenizes as くせ + に
        (
            GrammarPattern {
                name: "kuse_ni_split",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("くせ"),
                    TokenMatcher::Surface("に"),
                ],
                priority: 7,
            },
            ConjugationPattern::KuseNi,
            "n2",
        ),
        // kaketeha: Kagome tokenizes as compound particle にかけて + は
        (
            GrammarPattern {
                name: "kaketeha_compound",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("にかけて"),
                    TokenMatcher::Surface("は"),
                ],
                priority: 8,
            },
            ConjugationPattern::KaketeHa,
            "n2",
        ),
        (
            GrammarPattern {
                name: "itaru_made",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("いたる"),
                    TokenMatcher::Surface("まで"),
                ],
                priority: 7,
            },
            ConjugationPattern::ItaruMade,
            "n2",
        ),
        (
            GrammarPattern {
                name: "itaru_made_kanji",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("至る"),
                    TokenMatcher::Surface("まで"),
                ],
                priority: 7,
            },
            ConjugationPattern::ItaruMade,
            "n2",
        ),
        (
            GrammarPattern {
                name: "ni_itaru_made",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("に"),
                    TokenMatcher::Surface("いたる"),
                    TokenMatcher::Surface("まで"),
                ],
                priority: 8,
            },
            ConjugationPattern::ItaruMade,
            "n2",
        ),
        (
            GrammarPattern {
                name: "ni_itaru_made_kanji",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("に"),
                    TokenMatcher::Surface("至る"),
                    TokenMatcher::Surface("まで"),
                ],
                priority: 8,
            },
            ConjugationPattern::ItaruMade,
            "n2",
        ),
        (
            GrammarPattern {
                name: "igai_no",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("以外"),
                    TokenMatcher::Surface("の"),
                ],
                priority: 7,
            },
            ConjugationPattern::IgaiNo,
            "n2",
        ),
        // Phase 4: Fixed Expressions (20 patterns)
        (
            GrammarPattern {
                name: "ba_ii_noni",
                tokens: vec![
                    TokenMatcher::verb_with_form("仮定形"),
                    TokenMatcher::Surface("ば"),
                    TokenMatcher::Any, // いい or 良い
                    TokenMatcher::Surface("のに"),
                ],
                priority: 9,
            },
            ConjugationPattern::BaIiNoni,
            "n2",
        ),
        // ba_yoi_noni: When using 良い specifically, Kagome tokenizes のに as の + に
        (
            GrammarPattern {
                name: "ba_yoi_noni_split",
                tokens: vec![
                    TokenMatcher::verb_with_form("仮定形"),
                    TokenMatcher::Surface("ば"),
                    TokenMatcher::Surface("良い"),
                    TokenMatcher::Surface("の"),
                    TokenMatcher::Surface("に"),
                ],
                priority: 11,
            },
            ConjugationPattern::BaIiNoni,
            "n2",
        ),
        (
            GrammarPattern {
                name: "wake_desu",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("わけ"),
                    TokenMatcher::Surface("です"),
                ],
                priority: 7,
            },
            ConjugationPattern::WakeDesu,
            "n2",
        ),
        (
            GrammarPattern {
                name: "wake_da",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("わけ"),
                    TokenMatcher::Surface("だ"),
                ],
                priority: 7,
            },
            ConjugationPattern::WakeDesu,
            "n2",
        ),
        (
            GrammarPattern {
                name: "you_na_ki_ga_suru",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("よう"),
                    TokenMatcher::Surface("な"),
                    TokenMatcher::Surface("気"),
                    TokenMatcher::Surface("が"),
                    TokenMatcher::specific_verb("する"),
                ],
                priority: 10,
            },
            ConjugationPattern::YouNaKiGaSuru,
            "n2",
        ),
        (
            GrammarPattern {
                name: "ni_ki_wo_tsukeru",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("に"),
                    TokenMatcher::Surface("気"),
                    TokenMatcher::Surface("を"),
                    TokenMatcher::specific_verb("つける"),
                ],
                priority: 9,
            },
            ConjugationPattern::NiKiWoTsukeru,
            "n2",
        ),
        // betsuni_nai: Kagome tokenizes 別に as 別 + に
        (
            GrammarPattern {
                name: "betsuni_nai_split",
                tokens: vec![
                    TokenMatcher::Surface("別"),
                    TokenMatcher::Surface("に"),
                    TokenMatcher::Any, // Optional content
                    TokenMatcher::Surface("ない"),
                ],
                priority: 8,
            },
            ConjugationPattern::BetsuniNai,
            "n2",
        ),
        // wake_niwa_ikanai: Kagome tokenizes には as に + は
        (
            GrammarPattern {
                name: "wake_niwa_ikanai_short",
                tokens: vec![
                    TokenMatcher::verb_with_form("基本形"),
                    TokenMatcher::Surface("わけ"),
                    TokenMatcher::Surface("に"),
                    TokenMatcher::Surface("は"),
                    TokenMatcher::Surface("いか"),
                    TokenMatcher::Surface("ない"),
                ],
                priority: 10,
            },
            ConjugationPattern::WakeNiwaIkanai,
            "n2",
        ),
        // dewa_nai_darou_ka: Kagome tokenizes では as で+は and だろう as だろ+う
        (
            GrammarPattern {
                name: "dewa_nai_darou_ka_full_split",
                tokens: vec![
                    TokenMatcher::Surface("で"),
                    TokenMatcher::Surface("は"),
                    TokenMatcher::Surface("ない"),
                    TokenMatcher::Surface("だろ"),
                    TokenMatcher::Surface("う"),
                    TokenMatcher::Surface("か"),
                ],
                priority: 11,
            },
            ConjugationPattern::DewaNaiDarouKa,
            "n2",
        ),
        // to_iu_wake_dewa_nai: Kagome tokenizes という as compound and では as で+は
        (
            GrammarPattern {
                name: "to_iu_wake_dewa_nai_compound",
                tokens: vec![
                    TokenMatcher::Surface("という"),
                    TokenMatcher::Surface("わけ"),
                    TokenMatcher::Surface("で"),
                    TokenMatcher::Surface("は"),
                    TokenMatcher::Surface("ない"),
                ],
                priority: 11,
            },
            ConjugationPattern::ToIuWakeDewaNai,
            "n2",
        ),
        (
            GrammarPattern {
                name: "ni_koshita_koto_wa_nai",
                tokens: vec![
                    TokenMatcher::verb_with_form("基本形"),
                    TokenMatcher::Surface("に"),
                    TokenMatcher::Surface("越し"),
                    TokenMatcher::Surface("た"),
                    TokenMatcher::Surface("こと"),
                    TokenMatcher::Surface("は"),
                    TokenMatcher::Surface("ない"),
                ],
                priority: 11,
            },
            ConjugationPattern::NiKoshitaKotoWaNai,
            "n2",
        ),
        (
            GrammarPattern {
                name: "sashitsukaenai",
                tokens: vec![
                    TokenMatcher::Surface("さしつかえ"),
                    TokenMatcher::Surface("ない"),
                ],
                priority: 7,
            },
            ConjugationPattern::Sashitsukaenai,
            "n2",
        ),
        (
            GrammarPattern {
                name: "sashitsukaenai_kanji",
                tokens: vec![
                    TokenMatcher::Surface("差し支え"),
                    TokenMatcher::Surface("ない"),
                ],
                priority: 7,
            },
            ConjugationPattern::Sashitsukaenai,
            "n2",
        ),
        // nai_wake_niwa_ikanai: Kagome tokenizes には as に + は
        (
            GrammarPattern {
                name: "nai_wake_niwa_ikanai_short",
                tokens: vec![
                    TokenMatcher::verb_with_form("未然形"),
                    TokenMatcher::Surface("ない"),
                    TokenMatcher::Surface("わけ"),
                    TokenMatcher::Surface("に"),
                    TokenMatcher::Surface("は"),
                    TokenMatcher::Surface("いか"),
                    TokenMatcher::Surface("ない"),
                ],
                priority: 11,
            },
            ConjugationPattern::NaiWakeNiwaIkanai,
            "n2",
        ),
        (
            GrammarPattern {
                name: "to_ittemo",
                tokens: vec![
                    TokenMatcher::Surface("と"),
                    TokenMatcher::Surface("言っ"),
                    TokenMatcher::Surface("て"),
                    TokenMatcher::Surface("も"),
                ],
                priority: 8,
            },
            ConjugationPattern::ToIttemo,
            "n2",
        ),
        (
            GrammarPattern {
                name: "to_ittemo_kana",
                tokens: vec![
                    TokenMatcher::Surface("と"),
                    TokenMatcher::Surface("いっ"),
                    TokenMatcher::Surface("て"),
                    TokenMatcher::Surface("も"),
                ],
                priority: 8,
            },
            ConjugationPattern::ToIttemo,
            "n2",
        ),
        (
            GrammarPattern {
                name: "ga_ki_ni_naru",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("が"),
                    TokenMatcher::Surface("気"),
                    TokenMatcher::Surface("に"),
                    TokenMatcher::specific_verb("なる"),
                ],
                priority: 9,
            },
            ConjugationPattern::GaKiNiNaru,
            "n2",
        ),
        (
            GrammarPattern {
                name: "omou_you_ni",
                tokens: vec![
                    TokenMatcher::specific_verb("思う"),
                    TokenMatcher::Surface("よう"),
                    TokenMatcher::Surface("に"),
                ],
                priority: 8,
            },
            ConjugationPattern::OmouYouni,
            "n2",
        ),
        (
            GrammarPattern {
                name: "mono_desukara",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("もの"),
                    TokenMatcher::Surface("です"),
                    TokenMatcher::Surface("から"),
                ],
                priority: 8,
            },
            ConjugationPattern::MonoDesukara,
            "n2",
        ),
        (
            GrammarPattern {
                name: "mono_dakara",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("もの"),
                    TokenMatcher::Surface("だ"),
                    TokenMatcher::Surface("から"),
                ],
                priority: 8,
            },
            ConjugationPattern::MonoDesukara,
            "n2",
        ),
    ]
}
