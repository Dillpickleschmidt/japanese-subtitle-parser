use crate::pattern_matcher::TokenMatcher;
use crate::KagomeToken;
use std::sync::Arc;

#[allow(unused_imports)]
use super::concat;

// ========== N2 Pattern Functions ==========

pub fn toutei() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("とうてい")]
}

pub fn yoppodo() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("よっぽど")]
}

pub fn iyoiyo() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("いよいよ")]
}

pub fn sekkaku() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("せっかく")]
}

pub fn yappari() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("やっぱり")]
}

pub fn narubeku() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("なるべく")]
}

pub fn tashika() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("たしか")]
}

pub fn man_ichi_kanji() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("万一")]
}

pub fn man_ichi_kana() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("まんいち")]
}

pub fn nanishiro_kanji() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("何しろ")]
}

pub fn nanishiro_kana() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("なにしろ")]
}

pub fn soreni_shitemo() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("それにしても")]
}

pub fn tachimachi() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("たちまち")]
}

pub fn aete() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("あえて")]
}

pub fn semete() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("せめて")]
}

pub fn yamuoezu_verb() -> Vec<TokenMatcher> {
    super::concat(vec![
        vec![TokenMatcher::specific_verb("やむをえる")],
        vec![TokenMatcher::Surface("ず")],
    ])
}

pub fn rou_ni() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("ろくに")]
}

fn sasugani_form_matcher() -> TokenMatcher {
    #[derive(Debug)]
    struct SasuganiFormMatcher;
    impl super::Matcher for SasuganiFormMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "さすが"
        }
    }
    TokenMatcher::Custom(Arc::new(SasuganiFormMatcher))
}

pub fn sasugani_split() -> Vec<TokenMatcher> {
    vec![sasugani_form_matcher(), TokenMatcher::Surface("に")]
}

pub fn nantoittemo_split_kanji() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Surface("何"),
        TokenMatcher::Surface("と"),
        TokenMatcher::Surface("いっ"),
        TokenMatcher::Surface("て"),
        TokenMatcher::Surface("も"),
    ]
}

pub fn nantoittemo_adverb_split() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Surface("なんと"),
        TokenMatcher::Surface("いっ"),
        TokenMatcher::Surface("て"),
        TokenMatcher::Surface("も"),
    ]
}

pub fn kaneru() -> Vec<TokenMatcher> {
    super::concat(vec![
        vec![TokenMatcher::verb_with_form("連用形")],
        vec![TokenMatcher::specific_verb("かねる")],
    ])
}

pub fn kanenai() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("連用形"),
        TokenMatcher::specific_verb("かねる"),
        TokenMatcher::Surface("ない"),
    ]
}

pub fn tamaranai() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::Surface("たまらない")]
}

pub fn naide_sumu_split() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("未然形"),
        TokenMatcher::Surface("ない"),
        TokenMatcher::Surface("で"),
        TokenMatcher::specific_verb("済む"),
    ]
}

pub fn kara_naru() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("から"),
        TokenMatcher::specific_verb("なる"),
    ]
}

pub fn yori_shikata_ganai() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("より"),
        TokenMatcher::Surface("仕方"),
        TokenMatcher::Surface("が"),
        TokenMatcher::Surface("ない"),
    ]
}

pub fn yori_shikata_ganai_kana() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("より"),
        TokenMatcher::Surface("しかた"),
        TokenMatcher::Surface("が"),
        TokenMatcher::Surface("ない"),
    ]
}

pub fn ta_ue_de() -> Vec<TokenMatcher> {
    super::concat(vec![
        super::n5::ta_form(),
        vec![TokenMatcher::Surface("上")],
        vec![TokenMatcher::Surface("で")],
    ])
}

pub fn ni_ataru_compound() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::Surface("にあたる")]
}

pub fn ni_ataru_kanji() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::Surface("に当たる")]
}

pub fn gotoshi_kanji() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::Surface("如し")]
}

pub fn tsuujite_verb_kanji() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::specific_verb("通じる"),
        TokenMatcher::Surface("て"),
    ]
}

pub fn noboru() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("に"),
        TokenMatcher::specific_verb("のぼる"),
    ]
}

pub fn gatera_split() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("が"),
        TokenMatcher::Surface("てら"),
    ]
}

fn oyobi_form_matcher() -> TokenMatcher {
    #[derive(Debug)]
    struct OyobiFormMatcher;
    impl super::Matcher for OyobiFormMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            (token.surface == "および" || token.surface == "及び")
                && token.pos.first().is_some_and(|pos| pos == "接続詞")
        }
    }
    TokenMatcher::Custom(Arc::new(OyobiFormMatcher))
}

pub fn oyobi() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        oyobi_form_matcher(),
        TokenMatcher::Any,
    ]
}

pub fn sei_ka() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("せい"),
        TokenMatcher::Surface("か"),
    ]
}

pub fn itsunomanika() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("いつのまにか")]
}

pub fn itsunomanika_split() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("いつの間にか")]
}

pub fn yueni_split() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("ゆえ"),
        TokenMatcher::Surface("に"),
    ]
}

pub fn ippou_dewa_split() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Surface("一方"),
        TokenMatcher::Surface("で"),
        TokenMatcher::Surface("は"),
    ]
}

pub fn mono_no() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("ものの")]
}

fn dependent_noun_mono_matcher() -> TokenMatcher {
    #[derive(Debug)]
    struct DependentNounMonoMatcher;
    impl super::Matcher for DependentNounMonoMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "もの"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }
    TokenMatcher::Custom(Arc::new(DependentNounMonoMatcher))
}

pub fn mono_no_split() -> Vec<TokenMatcher> {
    vec![
        dependent_noun_mono_matcher(),
        TokenMatcher::Surface("の"),
    ]
}

pub fn kuse_ni_split() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("くせ"),
        TokenMatcher::Surface("に"),
    ]
}

pub fn kaketeha_compound() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("にかけて"),
        TokenMatcher::Surface("は"),
    ]
}

pub fn itaru_made() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("いたる"),
        TokenMatcher::Surface("まで"),
    ]
}

pub fn itaru_made_kanji() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("至る"),
        TokenMatcher::Surface("まで"),
    ]
}

pub fn ni_itaru_made() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("に"),
        TokenMatcher::Surface("いたる"),
        TokenMatcher::Surface("まで"),
    ]
}

pub fn ni_itaru_made_kanji() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("に"),
        TokenMatcher::Surface("至る"),
        TokenMatcher::Surface("まで"),
    ]
}

pub fn igai_no() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("以外"),
        TokenMatcher::Surface("の"),
    ]
}

pub fn ba_ii_noni() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("仮定形"),
        TokenMatcher::Surface("ば"),
        TokenMatcher::Any,
        TokenMatcher::Surface("のに"),
    ]
}

pub fn ba_yoi_noni_split() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("仮定形"),
        TokenMatcher::Surface("ば"),
        TokenMatcher::Surface("良い"),
        TokenMatcher::Surface("の"),
        TokenMatcher::Surface("に"),
    ]
}

pub fn wake_desu() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("わけ"),
        TokenMatcher::Surface("です"),
    ]
}

pub fn wake_da() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("わけ"),
        TokenMatcher::Surface("だ"),
    ]
}

pub fn you_na_ki_ga_suru() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("よう"),
        TokenMatcher::Surface("な"),
        TokenMatcher::Surface("気"),
        TokenMatcher::Surface("が"),
        TokenMatcher::specific_verb("する"),
    ]
}

pub fn ni_ki_wo_tsukeru() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("に"),
        TokenMatcher::Surface("気"),
        TokenMatcher::Surface("を"),
        TokenMatcher::specific_verb("つける"),
    ]
}

pub fn betsuni_nai_split() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Surface("別"),
        TokenMatcher::Surface("に"),
        TokenMatcher::Any,
        TokenMatcher::Surface("ない"),
    ]
}

pub fn wake_niwa_ikanai_short() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("基本形"),
        TokenMatcher::Surface("わけ"),
        TokenMatcher::Surface("に"),
        TokenMatcher::Surface("は"),
        TokenMatcher::Surface("いか"),
        TokenMatcher::Surface("ない"),
    ]
}

pub fn dewa_nai_darou_ka_full_split() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Surface("で"),
        TokenMatcher::Surface("は"),
        TokenMatcher::Surface("ない"),
        TokenMatcher::Surface("だろ"),
        TokenMatcher::Surface("う"),
        TokenMatcher::Surface("か"),
    ]
}

pub fn to_iu_wake_dewa_nai_compound() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Surface("という"),
        TokenMatcher::Surface("わけ"),
        TokenMatcher::Surface("で"),
        TokenMatcher::Surface("は"),
        TokenMatcher::Surface("ない"),
    ]
}

pub fn ni_koshita_koto_wa_nai() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("基本形"),
        TokenMatcher::Surface("に"),
        TokenMatcher::Surface("越し"),
        TokenMatcher::Surface("た"),
        TokenMatcher::Surface("こと"),
        TokenMatcher::Surface("は"),
        TokenMatcher::Surface("ない"),
    ]
}

pub fn sashitsukaenai() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Surface("さしつかえ"),
        TokenMatcher::Surface("ない"),
    ]
}

pub fn sashitsukaenai_kanji() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Surface("差し支え"),
        TokenMatcher::Surface("ない"),
    ]
}

pub fn nai_wake_niwa_ikanai_short() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("未然形"),
        TokenMatcher::Surface("ない"),
        TokenMatcher::Surface("わけ"),
        TokenMatcher::Surface("に"),
        TokenMatcher::Surface("は"),
        TokenMatcher::Surface("いか"),
        TokenMatcher::Surface("ない"),
    ]
}

pub fn to_ittemo() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Surface("と"),
        TokenMatcher::Surface("言っ"),
        TokenMatcher::Surface("て"),
        TokenMatcher::Surface("も"),
    ]
}

pub fn to_ittemo_kana() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Surface("と"),
        TokenMatcher::Surface("いっ"),
        TokenMatcher::Surface("て"),
        TokenMatcher::Surface("も"),
    ]
}

pub fn ga_ki_ni_naru() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("が"),
        TokenMatcher::Surface("気"),
        TokenMatcher::Surface("に"),
        TokenMatcher::specific_verb("なる"),
    ]
}

pub fn omou_you_ni() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::specific_verb("思う"),
        TokenMatcher::Surface("よう"),
        TokenMatcher::Surface("に"),
    ]
}

pub fn mono_desukara() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("もの"),
        TokenMatcher::Surface("です"),
        TokenMatcher::Surface("から"),
    ]
}

pub fn mono_dakara() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("もの"),
        TokenMatcher::Surface("だ"),
        TokenMatcher::Surface("から"),
    ]
}
