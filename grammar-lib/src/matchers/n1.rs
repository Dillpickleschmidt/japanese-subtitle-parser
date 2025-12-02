use crate::pattern_matcher::TokenMatcher;
use kagome_client::KagomeToken;
use std::sync::Arc;

#[allow(unused_imports)]
use super::concat;

// ========== N1 Pattern Functions ==========

// めく (split): Shows signs of (謎めく - mysterious)
pub fn meku() -> Vec<TokenMatcher> {
    vec![super::noun_matcher(), TokenMatcher::specific_verb("めく")]
}

// めく (compound): Single-token compounds (春めく - spring-like)
fn verb_with_base_suffix_meku() -> TokenMatcher {
    #[derive(Debug)]
    struct VerbWithBaseSuffixMeku;
    impl super::Matcher for VerbWithBaseSuffixMeku {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.base_form.ends_with("めく")
        }
    }
    TokenMatcher::Custom(Arc::new(VerbWithBaseSuffixMeku))
}

pub fn meku_compound() -> Vec<TokenMatcher> {
    vec![verb_with_base_suffix_meku()]
}

// まみれ (split): Covered with, smeared with (血まみれ - covered in blood)
pub fn mamire() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::Surface("まみれ")]
}

// まみれ (compound): Single-token compounds (泥まみれ - covered in mud)
fn noun_with_base_suffix_mamire() -> TokenMatcher {
    #[derive(Debug)]
    struct NounWithBaseSuffixMamire;
    impl super::Matcher for NounWithBaseSuffixMamire {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.base_form.ends_with("まみれ")
        }
    }
    TokenMatcher::Custom(Arc::new(NounWithBaseSuffixMamire))
}

pub fn mamire_compound() -> Vec<TokenMatcher> {
    vec![noun_with_base_suffix_mamire()]
}

// ずくめ: Entirely, nothing but (suffix to noun)
pub fn zukume() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::Surface("ずくめ")]
}

// っぱなし: Leaving as is, leaving undone (suffix to verb stem)
pub fn ppanashi() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::Surface("っぱなし")]
}

// 極まる: Extremely, to the utmost (suffix to na-adjective stem)
pub fn kiwamaru() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::specific_verb("極まる")]
}

// 極まりない: Kagome lexicalizes this as single adjective, not 極まる+ない
pub fn kiwamarinai() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::Surface("極まりない")]
}

// べく: In order to, for the purpose of (suffix to verb stem)
pub fn beku() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("基本形"),
        TokenMatcher::Surface("べく"),
    ]
}

// べからず: Must not, should not (classical prohibition, suffix to verb stem)
pub fn bekarazu() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("基本形"),
        TokenMatcher::Surface("べから"),
        TokenMatcher::Surface("ず"),
    ]
}

// まじき: Should not, unworthy of (classical negative, suffix to verb stem)
pub fn majiki() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::Surface("まじき")]
}

// なり: As soon as (verb + なり)
pub fn nari() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("基本形"),
        TokenMatcher::Surface("なり"),
    ]
}

// や否や: As soon as (verb + や + 否や)
pub fn ya_inaya() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("基本形"),
        TokenMatcher::Surface("や"),
        TokenMatcher::Surface("否や"),
    ]
}

// Kagome lexicalizes やいなや (kana form) as single particle
pub fn ya_inaya_single() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("基本形"),
        TokenMatcher::Surface("やいなや"),
    ]
}

// が早いか: As soon as (verb + が + 早い + か)
pub fn ga_hayai_ka() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("基本形"),
        TokenMatcher::Surface("が"),
        TokenMatcher::Surface("早い"),
        TokenMatcher::Surface("か"),
    ]
}

// が最後: Once ~ then forever (verb + が + 最後)
pub fn ga_saigo() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("基本形"),
        TokenMatcher::Surface("が"),
        TokenMatcher::Surface("最後"),
    ]
}

// ごとき: Like/such as (noun + ごとき)
pub fn gotoki() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::Surface("ごとき")]
}

// を皮切りに: Starting with (noun + を + 皮切りに)
pub fn wo_kawakiri_ni() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("を"),
        TokenMatcher::Surface("皮切り"),
        TokenMatcher::Surface("に"),
    ]
}

// をもって: With/by means of (noun + をもって)
pub fn wo_motte() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::Surface("をもって")]
}

// なくしては: Without (noun + なく + し + て + は)
pub fn nakushiteha() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("なく"),
        TokenMatcher::Surface("し"),
        TokenMatcher::Surface("て"),
        TokenMatcher::Surface("は"),
    ]
}

// なしに: Without (noun + なしに)
pub fn nashini() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("なし"),
        TokenMatcher::Surface("に"),
    ]
}

// ならでは: Unique to (noun + ならでは)
pub fn naradewa() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::Surface("ならでは")]
}

// に足る: Worth/deserve (noun + に + 足る)
pub fn ni_taru() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("に"),
        TokenMatcher::specific_verb("足る"),
    ]
}

// とあって: Because/being (noun + と + あっ + て)
pub fn toatte() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("と"),
        TokenMatcher::Surface("あっ"),
        TokenMatcher::Surface("て"),
    ]
}

// かたがた: While/also to (verb stem + かたがた)
pub fn katagata() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::Surface("かたがた")]
}

// を限りに: As the last time (を + 限りに)
pub fn wo_kagiri_ni() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("を"),
        TokenMatcher::Surface("限り"),
        TokenMatcher::Surface("に"),
    ]
}

// を経て: Through/via (を + 経て)
pub fn wo_hete() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("を"),
        TokenMatcher::Surface("経"),
        TokenMatcher::Surface("て"),
    ]
}

// をおして: In spite of (を + おして)
pub fn wo_oshite() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("を"),
        TokenMatcher::Surface("おして"),
    ]
}

// をふまえて: Based on (を + 踏まえて)
pub fn wo_fumaete() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("を"),
        TokenMatcher::Surface("踏まえ"),
        TokenMatcher::Surface("て"),
    ]
}

// てやまない: Never cease (te-form + やまない)
pub fn te_yamanai() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Surface("て"),
        TokenMatcher::Surface("やま"),
        TokenMatcher::Surface("ない"),
    ]
}

// と思いきや: Contrary to expectations
pub fn to_omoikiya() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("と"),
        TokenMatcher::Surface("思い"),
        TokenMatcher::Surface("き"),
        TokenMatcher::Surface("や"),
    ]
}

// とあれば: If it's the case
pub fn to_areba() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("と"),
        TokenMatcher::Surface("あれ"),
        TokenMatcher::Surface("ば"),
    ]
}

// たところで: Even if
pub fn ta_tokoro_de() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("連用タ接続"),
        TokenMatcher::Surface("た"),
        TokenMatcher::Surface("ところ"),
        TokenMatcher::Surface("で"),
    ]
}

// であれ: Whether/even if
pub fn de_are() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("で"),
        TokenMatcher::Surface("あれ"),
    ]
}

// とはいえ: Although/even though
pub fn to_wa_ie() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("と"),
        TokenMatcher::Surface("は"),
        TokenMatcher::Surface("いえ"),
    ]
}

// ようが: No matter/even if
pub fn you_ga() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("未然ウ接続"),
        TokenMatcher::Surface("う"),
        TokenMatcher::Surface("が"),
    ]
}

// ないまでも: Even if not
pub fn nai_made_mo() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("ない"),
        TokenMatcher::Surface("まで"),
        TokenMatcher::Surface("も"),
    ]
}

// ながらも: While/though
pub fn nagara_mo() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("ながら"),
        TokenMatcher::Surface("も"),
    ]
}

// ではあるまいし: It's not like
pub fn dewa_arumaishi() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("で"),
        TokenMatcher::Surface("は"),
        TokenMatcher::Surface("ある"),
        TokenMatcher::Surface("まい"),
        TokenMatcher::Surface("し"),
    ]
}

// としたところで: Even if
pub fn to_shita_tokoro_de() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("と"),
        TokenMatcher::Surface("し"),
        TokenMatcher::Surface("た"),
        TokenMatcher::Surface("ところ"),
        TokenMatcher::Surface("で"),
    ]
}

// といえども: Even though/although
pub fn to_iedomo() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("と"),
        TokenMatcher::Surface("いえ"),
        TokenMatcher::Surface("ども"),
    ]
}

// ともなると: When it comes to
pub fn tomo_naruto() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("と"),
        TokenMatcher::Surface("も"),
        TokenMatcher::Surface("なる"),
        TokenMatcher::Surface("と"),
    ]
}

// にたえない: Cannot bear/unbearable
pub fn ni_taenai() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("に"),
        TokenMatcher::Surface("堪え"),
        TokenMatcher::Surface("ない"),
    ]
}

// にそくして: In accordance with
pub fn ni_sokushite() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("に"),
        TokenMatcher::Surface("即し"),
        TokenMatcher::Surface("て"),
    ]
}

// と相まって: Combined with
pub fn to_aimatte() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("と"),
        TokenMatcher::Surface("相まって"),
    ]
}

// をよそに: In spite of/ignoring
pub fn wo_yosoni() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("を"),
        TokenMatcher::Surface("よそ"),
        TokenMatcher::Surface("に"),
    ]
}

// てもさしつかえない: It's okay to
pub fn temo_sashitsukaenai() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Surface("て"),
        TokenMatcher::Surface("も"),
        TokenMatcher::Surface("さしつかえ"),
        TokenMatcher::Surface("ない"),
    ]
}

// を禁じ得ない: Cannot help but
pub fn wo_kinjienai() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("を"),
        TokenMatcher::Surface("禁じ"),
        TokenMatcher::Surface("得"),
        TokenMatcher::Surface("ない"),
    ]
}

// を余儀なくされる: Be forced to
pub fn wo_yoginakusareru() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("を"),
        TokenMatcher::Surface("余儀なく"),
        TokenMatcher::Surface("さ"),
        TokenMatcher::Surface("れる"),
    ]
}

// てからというもの: Since/ever since
pub fn te_karatoiumono() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Surface("て"),
        TokenMatcher::Surface("から"),
        TokenMatcher::Surface("という"),
        TokenMatcher::Surface("もの"),
    ]
}

// にもまして: More than/even more
pub fn nimo_mashite() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("に"),
        TokenMatcher::Surface("も"),
        TokenMatcher::Surface("まして"),
    ]
}

// にひきかえ: In contrast to
pub fn ni_hikikae() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("に"),
        TokenMatcher::Surface("ひきかえ"),
    ]
}

// いかん patterns (depending on)
pub fn ikan_de() -> Vec<TokenMatcher> {
    vec![
        super::noun_matcher(),
        TokenMatcher::Surface("いかん"),
        TokenMatcher::Surface("で"),
    ]
}

pub fn ikan_no_da() -> Vec<TokenMatcher> {
    vec![
        super::noun_matcher(),
        TokenMatcher::Surface("の"),
        TokenMatcher::Surface("いかん"),
        TokenMatcher::Surface("だ"),
    ]
}

pub fn ikan_niyotte() -> Vec<TokenMatcher> {
    vec![
        super::noun_matcher(),
        TokenMatcher::Surface("いかん"),
        TokenMatcher::Surface("によって"),
    ]
}

pub fn ikan_shidai() -> Vec<TokenMatcher> {
    vec![
        super::noun_matcher(),
        TokenMatcher::Surface("いかん"),
        TokenMatcher::Surface("次第"),
    ]
}

// きらい: dislike/tendency
fn kiraiga_form() -> TokenMatcher {
    #[derive(Debug)]
    struct KiraiGaFormMatcher;
    impl super::Matcher for KiraiGaFormMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "嫌い" || token.surface == "きらい"
        }
    }
    TokenMatcher::Custom(Arc::new(KiraiGaFormMatcher))
}

pub fn kirai_ga_aru() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, kiraiga_form(), TokenMatcher::Surface("が"), TokenMatcher::Surface("ある")]
}

// わりに/わりには: Considering/proportionally
pub fn warini() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::Surface("割り"), TokenMatcher::Surface("に")]
}

pub fn wariniha() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::Surface("割り"), TokenMatcher::Surface("に"), TokenMatcher::Surface("は")]
}

// すら: even (emphatic)
pub fn sura() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::Surface("すら")]
}

// またたく間に: in the blink of an eye
pub fn taritomo() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("たり"),
        TokenMatcher::Surface("と"),
        TokenMatcher::Surface("も"),
    ]
}

// 如何だ: how/what (formal/archaic)
pub fn koto_nashini() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("こと"),
        TokenMatcher::Surface("なし"),
        TokenMatcher::Surface("に"),
    ]
}

// しまつだ: shameful/what a [noun]!
pub fn shimatsu_da() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::Surface("始末"), TokenMatcher::Surface("だ")]
}

pub fn shimatsu_datta() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::Surface("始末"), TokenMatcher::Surface("だっ")]
}

// ですら: even (with copula)
pub fn de_sura() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::Surface("で"), TokenMatcher::Surface("すら")]
}

// 甲斐もなく: without avail/despite efforts (甲斐 = benefit/avail)
pub fn kai_mo_naku() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("甲斐"),
        TokenMatcher::Surface("も"),
        TokenMatcher::Surface("なく"),
    ]
}

// だけまし: at least/only better for
pub fn dake_mashi() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::Surface("だけ"), TokenMatcher::Surface("まし")]
}

// ないではすまない: cannot escape/must do
pub fn naide_wa_sumanai() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("ない"),
        TokenMatcher::Surface("で"),
        TokenMatcher::Surface("は"),
        TokenMatcher::Surface("すま"),
        TokenMatcher::Surface("ない"),
    ]
}

// 生まれながらに: from birth/innately (umare form)
pub fn nagarani_umare() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Surface("生まれながら"),
        TokenMatcher::Surface("に"),
    ]
}

// 生まれながらにして: from birth (umare + shite form)
pub fn nagarani_umare_shite() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Surface("生まれながら"),
        TokenMatcher::Surface("に"),
        TokenMatcher::Surface("し"),
        TokenMatcher::Surface("て"),
    ]
}

// ながらに: while/in spite of (split noun + nagara + ni)
pub fn nagarani_split() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("ながら"),
        TokenMatcher::Surface("に"),
    ]
}

// ながらにして: while doing/despite (split + shite)
pub fn nagarani_shite() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("ながら"),
        TokenMatcher::Surface("に"),
        TokenMatcher::Surface("し"),
        TokenMatcher::Surface("て"),
    ]
}

// はおろか: let alone/not to mention (は + oroka)
pub fn ha_oroka() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Any,
        TokenMatcher::Surface("は"),
        TokenMatcher::Surface("おろか"),
    ]
}
