use crate::pattern_matcher::TokenMatcher;
use crate::types::KagomeToken;
use std::sync::Arc;

#[allow(unused_imports)]
use super::concat;

// ========== N3 Pattern Functions ==========

pub fn hajimeru() -> Vec<TokenMatcher> {
    super::concat(vec![
        vec![TokenMatcher::verb_with_form("連用形")],
        vec![TokenMatcher::specific_verb("始める")],
    ])
}

pub fn rashii() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::Surface("らしい")]
}

pub fn you_ni_naru() -> Vec<TokenMatcher> {
    super::concat(vec![
        vec![TokenMatcher::verb_with_form("基本形")],
        vec![TokenMatcher::Surface("よう")],
        vec![TokenMatcher::Surface("に")],
        vec![TokenMatcher::specific_verb("なる")],
    ])
}

pub fn you_ni_suru() -> Vec<TokenMatcher> {
    super::concat(vec![
        vec![TokenMatcher::verb_with_form("基本形")],
        vec![TokenMatcher::Surface("よう")],
        vec![TokenMatcher::Surface("に")],
        vec![TokenMatcher::specific_verb("する")],
    ])
}

pub fn tame_ni() -> Vec<TokenMatcher> {
    super::concat(vec![
        vec![TokenMatcher::verb_with_form("基本形")],
        vec![TokenMatcher::Surface("ため")],
        vec![TokenMatcher::Surface("に")],
    ])
}

pub fn zu() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("未然形"),
        TokenMatcher::Surface("ず"),
    ]
}

pub fn gachi() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::Surface("がち")]
}

fn ppoi_form() -> TokenMatcher {
    #[derive(Debug)]
    struct PpoiFormMatcher;
    impl super::Matcher for PpoiFormMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            (token.surface == "っぽい" || token.surface == "ぽい" || token.surface.ends_with("っぽい"))
                && (token
                    .pos
                    .first()
                    .is_some_and(|pos| pos == "接尾辞" || pos == "形容詞"))
        }
    }
    TokenMatcher::Custom(Arc::new(PpoiFormMatcher))
}

pub fn ppoi_split() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, ppoi_form()]
}

pub fn ppoi_compound() -> Vec<TokenMatcher> {
    vec![ppoi_form()]
}

pub fn ta_bakari() -> Vec<TokenMatcher> {
    super::concat(vec![
        vec![super::flexible_verb_form()],
        vec![super::past_auxiliary()],
        vec![TokenMatcher::Surface("ばかり")],
    ])
}

pub fn ta_mono_da() -> Vec<TokenMatcher> {
    super::concat(vec![
        vec![super::flexible_verb_form()],
        vec![super::past_auxiliary()],
        vec![TokenMatcher::Surface("もの")],
        vec![TokenMatcher::Surface("だ")],
    ])
}

pub fn ta_mono_desu() -> Vec<TokenMatcher> {
    super::concat(vec![
        vec![super::flexible_verb_form()],
        vec![super::past_auxiliary()],
        vec![TokenMatcher::Surface("もの")],
        vec![TokenMatcher::Surface("です")],
    ])
}

pub fn ni_chigainai() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("に"),
        TokenMatcher::Surface("違い"),
        TokenMatcher::Surface("ない"),
    ]
}

pub fn mama() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::Surface("まま")]
}

pub fn furi() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::Surface("ふり")]
}

pub fn nai_uchi_ni() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("未然形"),
        TokenMatcher::Surface("ない"),
        TokenMatcher::Surface("うち"),
        TokenMatcher::Surface("に"),
    ]
}

pub fn to_shitara() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("と"),
        TokenMatcher::Surface("し"),
        TokenMatcher::Surface("たら"),
    ]
}

pub fn bakari() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::Surface("ばかり")]
}

pub fn kiri_past() -> Vec<TokenMatcher> {
    vec![super::past_auxiliary(), TokenMatcher::Surface("きり")]
}

pub fn kiri_noun() -> Vec<TokenMatcher> {
    vec![super::noun_matcher(), TokenMatcher::Surface("きり")]
}

pub fn kawari() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::Surface("代わり")]
}

pub fn okage_de() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("おかげ"),
        TokenMatcher::Surface("で"),
    ]
}

pub fn sae() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::Surface("さえ")]
}

pub fn sae_ba() -> Vec<TokenMatcher> {
    super::concat(vec![
        vec![TokenMatcher::Any],
        vec![TokenMatcher::Surface("さえ")],
        vec![TokenMatcher::verb_with_form("仮定形")],
        vec![TokenMatcher::Surface("ば")],
    ])
}

pub fn koso() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::Surface("こそ")]
}

pub fn you_ni_standalone() -> Vec<TokenMatcher> {
    super::concat(vec![
        vec![TokenMatcher::verb_with_form("基本形")],
        vec![TokenMatcher::Surface("よう")],
        vec![TokenMatcher::Surface("に")],
    ])
}

pub fn masaka() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("まさか")]
}

pub fn mushiro() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("むしろ")]
}

pub fn sudeni() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("すでに")]
}

pub fn tsui() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("つい")]
}

pub fn doushitemo() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("どうしても")]
}

fn teki_suffix_matcher() -> TokenMatcher {
    #[derive(Debug)]
    struct TekiSuffixMatcher;
    impl super::Matcher for TekiSuffixMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "的"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
        }
    }
    TokenMatcher::Custom(Arc::new(TekiSuffixMatcher))
}

pub fn teki_suffix() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, teki_suffix_matcher()]
}

fn tate_suffix_matcher() -> TokenMatcher {
    #[derive(Debug)]
    struct TateSuffixMatcher;
    impl super::Matcher for TateSuffixMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "たて"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
        }
    }
    TokenMatcher::Custom(Arc::new(TateSuffixMatcher))
}

pub fn tate_suffix() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, tate_suffix_matcher()]
}

pub fn ni_yotte() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::Surface("によって")]
}

pub fn ni_yoru_to() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("に"),
        TokenMatcher::Surface("よる"),
        TokenMatcher::Surface("と"),
    ]
}

fn oite_form_matcher() -> TokenMatcher {
    #[derive(Debug)]
    struct OiteFormMatcher;
    impl super::Matcher for OiteFormMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "において" || (token.surface == "おい" && token.base_form == "おく")
        }
    }
    TokenMatcher::Custom(Arc::new(OiteFormMatcher))
}

pub fn oite_compound() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, oite_form_matcher()]
}

pub fn oite_split() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("に"),
        oite_form_matcher(),
    ]
}

pub fn tsumori_de() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("基本形"),
        TokenMatcher::Surface("つもり"),
        TokenMatcher::Surface("で"),
    ]
}

pub fn toshite() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::Surface("として")]
}

fn ni_kansuru_matcher() -> TokenMatcher {
    #[derive(Debug)]
    struct NiKansuruFormMatcher;
    impl super::Matcher for NiKansuruFormMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "に関する" || token.surface == "に関して"
        }
    }
    TokenMatcher::Custom(Arc::new(NiKansuruFormMatcher))
}

pub fn ni_kansuru() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, ni_kansuru_matcher()]
}

pub fn suginai() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("過ぎ"),
        TokenMatcher::Surface("ない"),
    ]
}

pub fn to_tomoni() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::Surface("とともに")]
}

fn hajimete_adverb() -> TokenMatcher {
    #[derive(Debug)]
    struct HajimeteAdverbMatcher;
    impl super::Matcher for HajimeteAdverbMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "初めて" && token.pos.first().is_some_and(|pos| pos == "副詞")
        }
    }
    TokenMatcher::Custom(Arc::new(HajimeteAdverbMatcher))
}

pub fn te_hajimete() -> Vec<TokenMatcher> {
    let mut result = super::n5::te_form();
    result.push(hajimete_adverb());
    result
}

pub fn seizei() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("せいぜい")]
}

pub fn douyara() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("どうやら")]
}

pub fn kaette() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("かえって")]
}

pub fn sarani() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("さらに")]
}

pub fn wazawaza() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("わざわざ")]
}

pub fn nakanaka() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("なかなか")]
}

pub fn ittai() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("いったい")]
}

pub fn wo_hajime() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("を"),
        TokenMatcher::Surface("始め"),
    ]
}

fn gurai_form_matcher() -> TokenMatcher {
    #[derive(Debug)]
    struct GuraiFormMatcher;
    impl super::Matcher for GuraiFormMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "ぐらい" || token.surface == "くらい"
        }
    }
    TokenMatcher::Custom(Arc::new(GuraiFormMatcher))
}

pub fn gurai() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, gurai_form_matcher()]
}

pub fn ba_hodo() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("仮定形"),
        TokenMatcher::Surface("ば"),
        TokenMatcher::Any,
        TokenMatcher::Surface("ほど"),
    ]
}

pub fn kagiru() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("に"),
        TokenMatcher::specific_verb("限る"),
    ]
}

fn mai_form_matcher() -> TokenMatcher {
    #[derive(Debug)]
    struct MaiFormMatcher;
    impl super::Matcher for MaiFormMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "まい" && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }
    TokenMatcher::Custom(Arc::new(MaiFormMatcher))
}

pub fn mai() -> Vec<TokenMatcher> {
    vec![TokenMatcher::verb_with_form("基本形"), mai_form_matcher()]
}
