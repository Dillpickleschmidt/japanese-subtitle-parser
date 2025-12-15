use crate::pattern_matcher::TokenMatcher;
use std::sync::Arc;

use super::Matcher;

// Pattern: という (called/named)
// Structures: Noun (A) + という + Noun (B)
pub fn toiu() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct ToiuMatcher;
    impl Matcher for ToiuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "という"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(ToiuMatcher)),
        super::noun_matcher(),
    ]
}

// Match まま (unchanged state noun)
fn mama_matcher() -> TokenMatcher {
    #[derive(Debug)]
    struct MamaMatcher;
    impl Matcher for MamaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "まま"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }
    TokenMatcher::Custom(Arc::new(MamaMatcher))
}

// Match に particle (optional after まま)
fn ni_particle_matcher() -> TokenMatcher {
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }
    TokenMatcher::Custom(Arc::new(NiParticleMatcher))
}

// Match any verb form that can precede た
fn verb_before_ta_matcher() -> TokenMatcher {
    #[derive(Debug)]
    struct VerbBeforeTaMatcher;
    impl Matcher for VerbBeforeTaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }
    TokenMatcher::Custom(Arc::new(VerbBeforeTaMatcher))
}

// Match た (past auxiliary) - strict surface match only
fn ta_auxiliary_matcher() -> TokenMatcher {
    #[derive(Debug)]
    struct TaAuxiliaryMatcher;
    impl Matcher for TaAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "た"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }
    TokenMatcher::Custom(Arc::new(TaAuxiliaryMatcher))
}

// Pattern: まま(に) - Verb[た] + まま
// Structure: Verb stem + た + まま (WITHOUT に)
pub fn mama_ni() -> Vec<TokenMatcher> {
    vec![
        verb_before_ta_matcher(),
        ta_auxiliary_matcher(),
        mama_matcher(),
    ]
}

// Pattern: まま(に) - Verb[た] + まま + に
// Structure: Verb stem + た + まま + に (WITH required に)
pub fn mama_ni_with_ni() -> Vec<TokenMatcher> {
    vec![
        verb_before_ta_matcher(),
        ta_auxiliary_matcher(),
        mama_matcher(),
        ni_particle_matcher(),
    ]
}

// Pattern: まま(に) - Verb[ない] + まま (+ に)
// Structure: Verb negative + ない + まま (+ に)
pub fn mama_ni_nai() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct VerbNegativeFormMatcher;
    impl Matcher for VerbNegativeFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然形")
        }
    }

    #[derive(Debug)]
    struct NaiAuxiliaryMatcher;
    impl Matcher for NaiAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbNegativeFormMatcher)),
        TokenMatcher::Custom(Arc::new(NaiAuxiliaryMatcher)),
        mama_matcher(),
        TokenMatcher::Optional(Box::new(ni_particle_matcher())),
    ]
}

// Pattern: まま(に) - い-Adjective + まま (+ に)
// Structure: い-Adjective + まま (+ に)
pub fn mama_ni_i_adj() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct IAdjMatcher;
    impl Matcher for IAdjMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "形容詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(IAdjMatcher)),
        mama_matcher(),
        TokenMatcher::Optional(Box::new(ni_particle_matcher())),
    ]
}

// Pattern: まま(に) - な-Adjective + な + まま (+ に)
// Structure: な-Adjective stem + な (だ auxiliary) + まま (+ に)
pub fn mama_ni_na_adj() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct NaAdjStemMatcher;
    impl Matcher for NaAdjStemMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹")
        }
    }

    #[derive(Debug)]
    struct NaCopulaMatcher;
    impl Matcher for NaCopulaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "な"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NaAdjStemMatcher)),
        TokenMatcher::Custom(Arc::new(NaCopulaMatcher)),
        mama_matcher(),
        TokenMatcher::Optional(Box::new(ni_particle_matcher())),
    ]
}

// Pattern: まま(に) - Noun + の + まま (+ に)
// Structure: Noun + の (connective particle) + まま (+ に)
pub fn mama_ni_noun() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct NoParticleMatcher;
    impl Matcher for NoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(NoParticleMatcher)),
        mama_matcher(),
        TokenMatcher::Optional(Box::new(ni_particle_matcher())),
    ]
}

// Pattern: に至るまで
pub fn niitarumade() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: たところで (even if, even though)
// Matches: Verb (連用形/連用タ接続) + た/だ (past auxiliary) + ところ (名詞/非自立) + で (格助詞)
pub fn tatokorode() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct TokoroMatcher;
    impl Matcher for TokoroMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ところ"
                && token.base_form == "ところ"
                && token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "非自立")
        }
    }

    #[derive(Debug)]
    struct DeParticleMatcher;
    impl Matcher for DeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "で"
                && token.base_form == "で"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    vec![
        super::flexible_verb_form(),
        super::past_auxiliary(),
        TokenMatcher::Custom(Arc::new(TokoroMatcher)),
        TokenMatcher::Custom(Arc::new(DeParticleMatcher)),
    ]
}

// Pattern: 如く・如き・如し
pub fn gotoku_u30fb_shiki_u30fb_gotoshi() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に足る
pub fn nitaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 極まりない・極まる
pub fn kiwamarinai_u30fb_kiwamaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: といえども
pub fn toiedomo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: を以て (by means of, with) - compound particle
// Matches: をもって as a single compound particle (助詞/格助詞/連語)
pub fn womotte() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct WomotteMatcher;
    impl Matcher for WomotteMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "をもって"
                && token.base_form == "をもって"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
                && token.pos.get(2).is_some_and(|p| p == "連語")
        }
    }
    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(WomotteMatcher)),
    ]
}

// Pattern: を以て (by means of, with) - split tokenization
// Matches: Noun + を + もつ (verb) + て
pub fn womotte_split() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct WoParticleMatcher;
    impl Matcher for WoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "を"
                && token.base_form == "を"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    #[derive(Debug)]
    struct MotsuVerbMatcher;
    impl Matcher for MotsuVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "もつ"
                && token.pos.first().is_some_and(|p| p == "動詞")
        }
    }

    #[derive(Debug)]
    struct TeParticleMatcher;
    impl Matcher for TeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.base_form == "て"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(WoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(MotsuVerbMatcher)),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
    ]
}

// Pattern: きらいがある
pub fn kiraigaaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ならまだしも (if A, that's fine, but B)
// Matches: なら (助動詞/仮定形) + まだしも (副詞)
// Note: Pattern range will include preceding token automatically
pub fn naramadashimo() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct NaraMatcher;
    impl Matcher for NaraMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "なら"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|p| p == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "仮定形")
        }
    }

    #[derive(Debug)]
    struct MadashimoMatcher;
    impl Matcher for MadashimoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "まだしも"
                && token.base_form == "まだしも"
                && token.pos.first().is_some_and(|p| p == "副詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NaraMatcher)),
        TokenMatcher::Custom(Arc::new(MadashimoMatcher)),
    ]
}

// Pattern: までもない
pub fn mademonai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ともなると・にもなると
pub fn tomonaruto_u30fb_nimonaruto() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: をいいことに
pub fn woiikotoni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 如何
pub fn ika_2() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～るまでだ
pub fn uff5e_rumadeda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にあって
pub fn niatte() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: を余儀なくされる
pub fn woyoginakusareru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: とは
pub fn toha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: じゃあるまいし
pub fn jaarumaishi() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: てからというもの
pub fn tekaratoiumono() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: かたわら
pub fn katawara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: を皮切りに
pub fn wokawakirini() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に至っては
pub fn niitatteha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なり
pub fn nari() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ともなく・ともなしに
pub fn tomonaku_u30fb_tomonashini() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 塗れ
pub fn nure() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ようが～まいが
pub fn youga_uff5e_maiga() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: からする
pub fn karasuru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にして①
pub fn nishite_u2460() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ものを
pub fn monowo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: であれ
pub fn deare() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: をおいてほかに〜ない
pub fn wooitehokani_u301c_nai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: をもって
pub fn womotte_2() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: とはいえ
pub fn tohaie() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ならでは
pub fn naradeha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: すら
pub fn sura() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: あっての
pub fn atteno() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～たまでだ
pub fn uff5e_tamadeda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: を経て
pub fn wohete() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ながらに
pub fn nagarani() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: たなり・なり
pub fn tanari_u30fb_nari() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: の極み
pub fn nokiwami() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にしてみれば
pub fn nishitemireba() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: だの
pub fn dano() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: あくまでも
pub fn akumademo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: べく
pub fn beku() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ところを
pub fn tokorowo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: からある
pub fn karaaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にして②
pub fn nishite_u2461() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: つ〜つ
pub fn tsu_u301c_tsu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 飽くまで(も)
pub fn akumade_mo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: であれ〜であれ
pub fn deare_u301c_deare() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: たら最後
pub fn tarasaigo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: いかなる
pub fn ikanaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なりに
pub fn narini() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: れる・られる + ままに
pub fn reru_u30fb_rareru_mamani() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にまつわる
pub fn nimatsuwaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: たる
pub fn taru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なら〜で
pub fn nara_u301c_de() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: をものともせず
pub fn womonotomosezu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: には当たらない
pub fn nihaataranai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ものと思う
pub fn monotoomou() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: を踏まえて
pub fn wofumaete() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ゆえに
pub fn yueni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にとどまらず
pub fn nitodomarazu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: と思いきや
pub fn toomoikiya() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: どうにも
pub fn dounimo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ことだし
pub fn kotodashi() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: がん～
pub fn gan_uff5e() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: か否か
pub fn kainaka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: たら〜で
pub fn tara_u301c_de() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: べくして
pub fn bekushite() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: かれ〜かれ
pub fn kare_u301c_kare() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 〜に〜ない
pub fn u301c_ni_u301c_nai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なくして(は)
pub fn nakushite_ha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: のなんのって
pub fn nonannotte() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にかかっている
pub fn nikakatteiru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: てやまない
pub fn teyamanai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ぐらいなら
pub fn gurainara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ってば・ったら
pub fn tteba_u30fb_ttara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ずとも
pub fn zutomo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: とあって
pub fn toatte() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: でもなんでもない
pub fn demonandemonai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ぐるみで
pub fn gurumide() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: そばから
pub fn sobakara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 訳あり(訳あって)
pub fn wakeari_yakuatte() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に至って・に至り
pub fn niitatte_u30fb_niitari() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: だに + しない
pub fn dani_shinai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: がてら
pub fn gatera() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: んがため(に)
pub fn ngatame_ni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: いかん〜ず
pub fn ikan_u301c_zu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にも～ない
pub fn nimo_uff5e_nai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: い-Adj[く] + もなんともない
pub fn i_adj_ku_monantomonai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Verb + だに
pub fn verb_dani() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～なり～なり
pub fn uff5e_nari_uff5e_nari() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ないでもない
pub fn naidemonai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: もさることながら
pub fn mosarukotonagara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ものと思っていた
pub fn monotoomotteita() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: でなくてなんだろう
pub fn denakutenandarou() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: はさておき・はさておいて
pub fn hasateoki_u30fb_hasateoite() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 折には
pub fn oriniha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: とばかり（に）
pub fn tobakari_uff08_ni_uff09() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: わ〜わ
pub fn wa_u301c_wa() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なりとも
pub fn naritomo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に至っても
pub fn niitattemo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: を兼ねて
pub fn wokanete() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Verb[ない]もの(だろう)か
pub fn verb_nai_mono_darou_ka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Verb[て] + みせる
pub fn verb_te_miseru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 相まって
pub fn aimatte() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に足りない
pub fn nitarinai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: べからず
pub fn bekarazu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: んばかりに
pub fn nbakarini() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に則って・に則り
pub fn ninottotte_u30fb_ninottori() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Adj限りだ
pub fn adjkagirida() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: はおろか
pub fn haoroka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: めく・めいた
pub fn meku_u30fb_meita() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: といわず
pub fn toiwazu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にもほどがある
pub fn nimohodogaaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にもまして
pub fn nimomashite() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: まくる
pub fn makuru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: わ〜わ（で）
pub fn wa_u301c_wa_uff08_de_uff09() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: どうにか
pub fn dounika() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: や否や
pub fn yainaya() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 次第です
pub fn shidaidesu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: というところ
pub fn toiutokoro() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: １～たりとも～ない
pub fn ichi_uff5e_taritomo_uff5e_nai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ったらない・といったらない
pub fn ttaranai_u30fb_toittaranai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に照らして・に照らすと
pub fn niterashite_u30fb_niterasuto() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: とあれば
pub fn toareba() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: さぞ
pub fn sazo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ときたら
pub fn tokitara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: びる
pub fn biru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にしたところで
pub fn nishitatokorode() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～ばこそ
pub fn uff5e_bakoso() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ても差し支えない
pub fn temosashitsukaenai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: には及ばない①
pub fn nihaoyobanai_u2460() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に即して
pub fn nisokushite() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ないまでも
pub fn naimademo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: をよそに
pub fn woyosoni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に限ったことではない
pub fn nikagittakotodehanai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: とは比べものにならない
pub fn tohakurabemononinaranai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: まじき
pub fn majiki() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: の至り
pub fn noitari() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に恥じない
pub fn nihajinai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ずじまい
pub fn zujimai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に言わせれば・に言わせると・に言わせたら
pub fn niiwasereba_u30fb_niiwaseruto_u30fb_niiwasetara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ったら・といったら
pub fn ttara_u30fb_toittara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: こととて
pub fn kototote() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ずくめ
pub fn zukume() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: には及ばない②
pub fn nihaoyobanai_u2461() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: とは言うものの
pub fn tohaiumonono() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: が早いか
pub fn gahayaika() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に難くない
pub fn nikatakunai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ならいざ知らず
pub fn naraizashirazu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: を禁じ得ない
pub fn wokinjienai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にかこつけて
pub fn nikakotsukete() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ようによっては
pub fn youniyotteha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: べくもない
pub fn bekumonai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: と来たら
pub fn tokitara_2() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ものとして
pub fn monotoshite() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: を前提に
pub fn wozenteini() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ずにはすまない
pub fn zunihasumanai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に堪えない
pub fn nikotaenai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 始末だ
pub fn shimatsuda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ものなら②
pub fn mononara_u2461() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にひきかえ
pub fn nihikikae() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: それまでだ
pub fn soremadeda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: といおうか
pub fn toiouka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ずにはおかない
pub fn zunihaokanai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: を限りに
pub fn wokagirini() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: てはかなわない
pub fn tehakanawanai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: かたがた
pub fn katagata() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: を余儀なくさせる
pub fn woyoginakusaseru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～てやる
pub fn uff5e_teyaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ただ〜のみ
pub fn tada_u301c_nomi() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ものとする
pub fn monotosuru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: との
pub fn tono() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 以前
pub fn izen() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ともあろう
pub fn tomoarou() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: こそすれ〜ない
pub fn kososure_u301c_nai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 並み
pub fn nami() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に先駆けて
pub fn nisakigakete() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: を機に (taking advantage of, on the occasion of)
// Structures: Verb[た] + の + を機に or Noun + を機に
pub fn wokini() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match を as 格助詞
    #[derive(Debug)]
    struct WoParticleMatcher;
    impl Matcher for WoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "を"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match 機 as noun
    #[derive(Debug)]
    struct KiNounMatcher;
    impl Matcher for KiNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "機"
                && token.base_form == "機"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
        }
    }

    // Match に as 格助詞
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match の as nominalizer (名詞/非自立)
    #[derive(Debug)]
    struct NoNominalizerMatcher;
    impl Matcher for NoNominalizerMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.base_form == "の"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    vec![
        TokenMatcher::Any, // Noun or (Verb + た)
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NoNominalizerMatcher,
        )))), // Optional の for verb nominalization
        TokenMatcher::Custom(Arc::new(WoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(KiNounMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
    ]
}
