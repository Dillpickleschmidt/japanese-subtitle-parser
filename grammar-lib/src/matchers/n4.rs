use crate::pattern_matcher::TokenMatcher;
use std::sync::Arc;

// Pattern: と
pub fn to() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: でも
pub fn demo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: やすい
pub fn yasui() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にくい
pub fn nikui() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: だんだん
pub fn dandan() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: どんどん
pub fn dondon() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～ら
pub fn uff5e_ra() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ていく
pub fn teiku() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: てくる 
pub fn tekuru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: かた
pub fn kata() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: だけで
pub fn dakede() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: だが・ですが
pub fn daga_u30fb_desuga() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なくて
pub fn nakute() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ないで
// Pattern: ないで (without doing)
// Structure: Verb[未然形] + ない + で
pub fn naide() -> Vec<TokenMatcher> {
    // Match verb in 未然形 (negative stem form)
    #[derive(Debug)]
    struct VerbMizenMatcher;
    impl super::Matcher for VerbMizenMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然形")
        }
    }

    // Match ない auxiliary in 連用デ接続 form
    #[derive(Debug)]
    struct NaiAuxiliaryMatcher;
    impl super::Matcher for NaiAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "連用デ接続")
        }
    }

    // Match で as conjunction particle
    #[derive(Debug)]
    struct DeParticleMatcher;
    impl super::Matcher for DeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbMizenMatcher)),
        TokenMatcher::Custom(Arc::new(NaiAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(DeParticleMatcher)),
    ]
}

// Pattern: Verb［れる・られる］
pub fn verb_uff3b_reru_u30fb_rareru_uff3d() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 他動詞・自動詞
pub fn tadoushi_u30fb_jidoushi() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なおす
pub fn naosu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ということ
pub fn toiukoto() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: とき
pub fn toki() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: まず
pub fn mazu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: まで
pub fn made() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: までに
pub fn madeni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: また
pub fn mata() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: はじめる
pub fn hajimeru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: おわる
pub fn owaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ごろ
pub fn goro() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: あとで (after/later)
// Structures:
// - Verb[た] + あとで
// - Noun + の + あとで
// - あとで + Phrase (at start)
// - Verb + のは + あとで
//
// Examples:
// - 食べたあとで (after eating)
// - 仕事のあとで (after work)
// - あとで洗濯もの干してね (please hang the laundry later)
// - コピーを取るのはあとでいい (it's fine to make copies later)
//
// Tokenization pattern:
// - あと (名詞/一般)
// - で (助詞/格助詞/一般 OR 助動詞, base=だ)
//
// Note: We match "あと + で" regardless of what comes before,
// since all variants end with this combination.
pub fn atode() -> Vec<TokenMatcher> {
    use super::Matcher;

    // Match あと noun (名詞/一般)
    #[derive(Debug)]
    struct AtoMatcher;
    impl Matcher for AtoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "あと"
                && token.base_form == "あと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
        }
    }

    // Match で particle or copula after あと
    // Can be:
    // - 助詞/格助詞/一般 (particle: "with/at")
    // - 助動詞 (copula, base=だ: "is")
    #[derive(Debug)]
    struct DeAfterAtoMatcher;
    impl Matcher for DeAfterAtoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            if token.surface != "で" {
                return false;
            }

            // Check if it's a particle (助詞/格助詞)
            let is_particle = token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞");

            // Check if it's a copula (助動詞, base=だ)
            let is_copula = token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "だ";

            is_particle || is_copula
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(AtoMatcher)),
        TokenMatcher::Custom(Arc::new(DeAfterAtoMatcher)),
    ]
}

// Pattern: ていた 
pub fn teita() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に (Frequency)
pub fn ni_frequency() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: とうとう
pub fn toutou() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: より
pub fn yori() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ごとに
pub fn gotoni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なるべく
pub fn narubeku() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: るところだ
pub fn rutokoroda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: のに 
pub fn noni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: とおもう
pub fn toomou() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: など
pub fn nado() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: みたい
pub fn mitai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: こと
pub fn koto() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: そう 
pub fn sou() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: さ
pub fn sa() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: とか～とか
pub fn toka_uff5e_toka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: そういう
pub fn souiu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Verb[よう]
pub fn verb_you() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ようだ
pub fn youda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ぜんぜん
pub fn zenzen() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: かな
pub fn kana() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: あまり～ない
pub fn amari_uff5e_nai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ば
pub fn ba() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なら
pub fn nara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: がる
pub fn garu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: がする
pub fn gasuru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: たがる
pub fn tagaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: かもしれない
pub fn kamoshirenai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: みたいに・みたいな
pub fn mitaini_u30fb_mitaina() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: そうに・そうな 
pub fn souni_u30fb_souna() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: のように・のような 
pub fn noyouni_u30fb_noyouna() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 〜ようと思う・〜おうと思う
pub fn u301c_youtoomou_u30fb_u301c_outoomou() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: く・に
pub fn ku_u30fb_ni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～にする・～くする
pub fn uff5e_nisuru_u30fb_uff5e_kusuru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: といい
pub fn toii() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ようになる
pub fn youninaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: まい～のように
pub fn mai_uff5e_noyouni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: じゃないか
pub fn janaika() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: らしい ①
pub fn rashii_u2460() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ておく
pub fn teoku() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: がほしい
pub fn gahoshii() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: てほしい
pub fn tehoshii() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ときいた
pub fn tokiita() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 聞こえる
pub fn kikoeru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 見える
pub fn mieru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: だす
pub fn dasu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～代
pub fn uff5e_dai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Number + も (as many as / not even)
// Structures: Number + Counter + も
//
// Examples:
// - １２時間も (as many as 12 hours)
// - ２０万円も (as much as 200,000 yen)
// - 一回も (not even once)
//
// Tokenization pattern:
// - One or more number tokens (名詞/数)
// - Counter token (名詞/接尾/助数詞)
// - も particle (助詞/係助詞)
pub fn number_mo() -> Vec<TokenMatcher> {
    use super::Matcher;

    // Match number tokens (名詞/数)
    #[derive(Debug)]
    struct NumberMatcher;
    impl Matcher for NumberMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "数")
        }
    }

    // Match counter tokens (名詞/接尾/助数詞)
    #[derive(Debug)]
    struct CounterMatcher;
    impl Matcher for CounterMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                && token.pos.get(2).is_some_and(|pos| pos == "助数詞")
        }
    }

    // Match も particle after numbers (助詞/係助詞)
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Pattern: One or more numbers + counter + も
    // Numbers can be 1-many tokens (一 vs １２ vs ２０万円)
    // We'll use a flexible approach with optional number tokens
    use super::concat;

    concat(vec![
        vec![TokenMatcher::Custom(Arc::new(NumberMatcher))], // First number (required)
        vec![
            TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NumberMatcher)))),
            TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NumberMatcher)))),
            TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NumberMatcher)))),
            TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NumberMatcher)))),
            TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NumberMatcher)))),
        ], // Up to 5 additional numbers (should be enough for most cases)
        vec![TokenMatcher::Custom(Arc::new(CounterMatcher))],
        vec![TokenMatcher::Custom(Arc::new(MoParticleMatcher))],
    ])
}

// Pattern: ほとんど
pub fn hotondo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: そんな・こんな・あんな・どんな
pub fn sonna_u30fb_konna_u30fb_anna_u30fb_donna() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 各
pub fn kaku() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 以上 ①
pub fn ijou_u2460() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: いか
pub fn ika() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: いがい
pub fn igai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ずっと ① (continuously/the whole time)
// Structure: ずっと + Phrase
//
// Examples:
// - ずっとゲームをしないで (instead of continuously gaming)
// - ずっと立ってた (standing the whole time)
// - からずっと寝てない (haven't slept at all since...)
//
// Tokenization: ずっと (副詞/一般)
pub fn zutto_u2460() -> Vec<TokenMatcher> {
    use super::Matcher;

    // Match ずっと adverb (副詞/一般)
    #[derive(Debug)]
    struct ZuttoMatcher;
    impl Matcher for ZuttoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ずっと"
                && token.base_form == "ずっと"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(ZuttoMatcher))]
}

// Pattern: だいたい
pub fn daitai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: のなかで
pub fn nonakade() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ように・ような
pub fn youni_u30fb_youna() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Number/Amount + は
pub fn number_amount_ha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なん + counter + か
pub fn nan_counter_ka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 真(っ)
pub fn ma() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Number + しか〜ない
pub fn number_shika_u301c_nai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～は～の一つだ
pub fn uff5e_ha_uff5e_nohitotsuda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～ない～はない
pub fn uff5e_nai_uff5e_hanai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: すこしも～ない
pub fn sukoshimo_uff5e_nai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: すくなくない
pub fn sukunakunai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ばあいは
pub fn baaiha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Verb[て]
pub fn verb_te_2() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: てよかった
pub fn teyokatta() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Verb［せる・させる］
pub fn verb_uff3b_seru_u30fb_saseru_uff3d() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: といってもいい
pub fn toittemoii() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ても
pub fn temo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: てしまう・ちゃう
pub fn teshimau_u30fb_chau() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Verb[て] + B
pub fn verb_te_b_2() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Causative-Passive
pub fn causative_passive() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Verb[て]・Noun[で] + B
pub fn verb_te_u30fb_noun_de_b() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: てある 
pub fn tearu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ように～てほしい
pub fn youni_uff5e_tehoshii() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ているあいだに
pub fn teiruaidani() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なくてもいい
pub fn nakutemoii() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: てみる
pub fn temiru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: てすみません
pub fn tesumimasen() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: てあげる
pub fn teageru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: てくれる
pub fn tekureru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: てもらう
pub fn temorau() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なさい
pub fn nasai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Verb[ないで]
// Pattern: Verb[ないで] (without doing)
// Structure: Verb[未然形] + ない + で
pub fn verb_naide() -> Vec<TokenMatcher> {
    naide()  // Same implementation as ないで
}

// Pattern: てくれてありがとう
pub fn tekuretearigatou() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: てくれない・てもらえない
pub fn tekurenai_u30fb_temoraenai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～のだろうか
pub fn uff5e_nodarouka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: お～になる 
pub fn o_uff5e_ninaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なさる
pub fn nasaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: お～ください 
pub fn o_uff5e_kudasai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: いらっしゃる
pub fn irassharu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ございます
pub fn gozaimasu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: でございます
pub fn degozaimasu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: お〜する
pub fn o_u301c_suru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: いたす
pub fn itasu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ていただけませんか
pub fn teitadakemasenka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: たら
pub fn tara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ほかに(も)・ほか(に)は
pub fn hokani_mo_u30fb_hoka_ni_ha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: がひつよう
pub fn gahitsuyou() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: そんなに
pub fn sonnani() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ひつようがある
pub fn hitsuyougaaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: たとえば
pub fn tatoeba() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: れる・られる (Potential)
pub fn reru_u30fb_rareru_potential() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: んだけど・んですが
pub fn ndakedo_u30fb_ndesuga() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: はずだ
pub fn hazuda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: かどうか
pub fn kadouka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ないと
pub fn naito() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: はずがない
pub fn hazuganai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: しか～ない 
pub fn shika_uff5e_nai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: だけでなく
pub fn dakedenaku() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ことができる
pub fn kotogadekiru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: かい
pub fn kai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: もし
pub fn moshi() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: し～し 
pub fn shi_uff5e_shi() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: でできる・からできる
pub fn dedekiru_u30fb_karadekiru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ながら
pub fn nagara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: たところだ
pub fn tatokoroda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ているところだ
pub fn teirutokoroda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: と～と、どちらが 
pub fn to_uff5e_to_u3001_dochiraga() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ようにする
pub fn younisuru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なければいけない
pub fn nakerebaikenai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なければならない
pub fn nakerebanaranai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: つづける
pub fn tsuzukeru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ようにいう
pub fn youniiu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: よていだ
pub fn yoteida() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ようにいのる
pub fn youniinoru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: たばかり
pub fn tabakari() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 化する
pub fn kasuru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 命令形
pub fn meireigata() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ように
pub fn youni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: かしら
pub fn kashira() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: らしい ②
pub fn rashii_u2461() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にみえる
pub fn nimieru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: とみえる
pub fn tomieru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 風
pub fn kaze() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: がみられる
pub fn gamirareru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にきがつく
pub fn nikigatsuku() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 〜でも 〜でも
pub fn u301c_demo_u301c_demo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: それに
pub fn soreni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: それで
pub fn sorede() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Question-phrase + か
pub fn question_phrase_ka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: それでも
pub fn soredemo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: たらどう
pub fn taradou() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: とかんがえられている
pub fn tokangaerareteiru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: とされている
pub fn tosareteiru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: といわれている
pub fn toiwareteiru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ばよかった
pub fn bayokatta() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}
