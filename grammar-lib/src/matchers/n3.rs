use crate::pattern_matcher::TokenMatcher;
use std::sync::Arc;
use super::Matcher;

// Pattern: って
pub fn tte() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ばいい
pub fn baii() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: たらいい・といい
pub fn taraii_u30fb_toii() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 中
pub fn naka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: の間に
pub fn nomani() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: うちに
pub fn uchini() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ないうちに
pub fn naiuchini() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: べき
pub fn beki() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: べきではない
pub fn bekidehanai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なかなか (quite/considerably/very)
// Structures: なかなか + Adjective, なかなか + の + Noun
pub fn nakanaka() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct NakanakaMatcher;
    impl Matcher for NakanakaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "なかなか"
                && token.base_form == "なかなか"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "助詞類接続")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(NakanakaMatcher))]
}

// Pattern: あまり
pub fn amari() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なかなか～ない
pub fn nakanaka_uff5e_nai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: によると・によれば
pub fn niyoruto_u30fb_niyoreba() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: によって・による
pub fn niyotte_u30fb_niyoru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 全く～ない
pub fn mattaku_uff5e_nai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ことだ
pub fn kotoda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: そうだ 
pub fn souda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: すると
pub fn suruto() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: そうすると
pub fn sousuruto() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: のはXの方だ
pub fn nohaxnohouda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Noun＋型
pub fn nountasukata() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: てごらん
pub fn tegoran() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Particle + の
pub fn particle_no() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: である
pub fn dearu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ところが
pub fn tokoroga() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ところで
pub fn tokorode() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ほど
pub fn hodo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ば〜ほど
pub fn ba_u301c_hodo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ほど～ない
pub fn hodo_uff5e_nai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: では・それでは・じゃあ
pub fn deha_u30fb_soredeha_u30fb_jaa() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: のに
pub fn noni_2() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ため(に)
pub fn tame_ni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ために
pub fn tameni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ということだ
pub fn toiukotoda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: というのは
pub fn toiunoha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 的
pub fn teki() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: もの・もん
pub fn mono_u30fb_mon() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ものだ
pub fn monoda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 最中に
pub fn saichuuni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 上で
pub fn uede() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: おかげで (thanks to / because of)
// Structures:
//   - Verb (attributive) + おかげで
//   - い-Adjective + おかげで
//   - な-Adjective + な + おかげで
//   - Noun + の + おかげで
pub fn okagede() -> Vec<TokenMatcher> {
    use crate::pattern_matcher::TokenMatcher;

    // Match おかげ as noun
    #[derive(Debug)]
    struct OkageMatcher;
    impl Matcher for OkageMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "おかげ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    // Match で as case particle
    #[derive(Debug)]
    struct DeCaseParticleMatcher;
    impl Matcher for DeCaseParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match any verb, adjective, or noun that can precede おかげで
    // This includes:
    // - Verbs in dictionary/attributive form
    // - Past tense auxiliary た (after verbs)
    // - い-adjectives in basic form
    // - な-adjectives with な
    // - Nouns with の
    #[derive(Debug)]
    struct AttributivePrecedingMatcher;
    impl Matcher for AttributivePrecedingMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Verb in dictionary/attributive form
            let is_verb = token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "基本形" || f == "連体形");

            // Past tense auxiliary た (基本形)
            let is_past_aux = token.surface == "た"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "た"
                && token.features.get(5).is_some_and(|f| f == "基本形");

            // い-adjective in basic form
            let is_i_adj = token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "基本形");

            // な (copula in attributive form after na-adjective)
            let is_na_copula = token.surface == "な"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "だ"
                && token.features.get(5).is_some_and(|f| f == "体言接続");

            // の particle (after noun)
            let is_no_particle = token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "連体化")
                    || token.pos.get(1).is_some_and(|pos| pos == "格助詞"));

            is_verb || is_past_aux || is_i_adj || is_na_copula || is_no_particle
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(AttributivePrecedingMatcher)),
        TokenMatcher::Custom(Arc::new(OkageMatcher)),
        TokenMatcher::Custom(Arc::new(DeCaseParticleMatcher)),
    ]
}

// Pattern: にもとづいて
pub fn nimotozuite() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 点
pub fn ten() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なぜなら〜から
pub fn nazenara_u301c_kara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: こそ
pub fn koso() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: からこそ
pub fn karakoso() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ばかり
pub fn bakari() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ばかりだ
pub fn bakarida() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ばかりに
pub fn bakarini() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ことがある
pub fn kotogaaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ことにする
pub fn kotonisuru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ことなの
pub fn kotonano() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ことになる
pub fn kotoninaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～は～で有名
pub fn uff5e_ha_uff5e_deyuumei() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ことはない
pub fn kotohanai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern:  ～と言っても
pub fn uff5e_toittemo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: といえば
pub fn toieba() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 合う
pub fn au() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に合わせて・に合った
pub fn niawasete_u30fb_niatta() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: について
pub fn nitsuite() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～(の)姿
pub fn uff5e_no_sugata() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: と言える
pub fn toieru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ちゃんと・きちんと
pub fn chanto_u30fb_kichinto() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: そのため(に)
pub fn sonotame_ni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: その結果
pub fn sonokekka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に比べて
pub fn nikurabete() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: どんなに〜ても
pub fn donnani_u301c_temo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: いくら〜でも
pub fn ikura_u301c_demo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 〜かは〜によって違う
pub fn u301c_kaha_u301c_niyottechigau() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: かなり
pub fn kanari() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: あまりに
pub fn amarini() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: わけだ
pub fn wakeda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: わけではない
pub fn wakedehanai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: と同時に
pub fn todoujini() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ところだった ①
pub fn tokorodatta_u2460() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: だって
pub fn datte() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: んだって
pub fn ndatte() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 関係がある
pub fn kankeigaaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に関する・に関して
pub fn nikansuru_u30fb_nikanshite() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に対して
pub fn nitaishite() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: くらい ②
pub fn kurai_u2461() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: は～くらいです
pub fn ha_uff5e_kuraidesu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: さ - Interjection
pub fn sa_interjection() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: さ - Filler
pub fn sa_filler() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: さ - Casual よ
pub fn sa_casual_yo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: それぞれ
pub fn sorezore() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: そこで
pub fn sokode() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: しかない
pub fn shikanai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: てもかまわない
pub fn temokamawanai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～ても～なくても
pub fn uff5e_temo_uff5e_nakutemo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: んじゃない
pub fn njanai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: わけがない
pub fn wakeganai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: としたら・とすれば・とすると
pub fn toshitara_u30fb_tosureba_u30fb_tosuruto() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: として
pub fn toshite() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にしては
pub fn nishiteha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にしても
pub fn nishitemo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～というのは事実だ
pub fn uff5e_toiunohajijitsuda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: から言うと
pub fn karaiuto() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に取って
pub fn nitotte() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ことから
pub fn kotokara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: というより
pub fn toiuyori() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: はもちろん
pub fn hamochiron() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: をはじめ
pub fn wohajime() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: て初めて
pub fn tehajimete() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: さえ
pub fn sae() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: さえ〜ば
pub fn sae_u301c_ba() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: たものだ
pub fn tamonoda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: さて
pub fn sate() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: むしろ
pub fn mushiro() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: つまり
pub fn tsumari() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 即ち
pub fn sunawachi() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 却って
pub fn kaette() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: まるで…ようだ
pub fn marude_u2026_youda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ような気がする
pub fn younakigasuru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: とても～ない
pub fn totemo_uff5e_nai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 別に〜ない
pub fn betsuni_u301c_nai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ばかりでなく
pub fn bakaridenaku() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ではなくて・じゃなくて
pub fn dehanakute_u30fb_janakute() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: だけでなく(て)～も
pub fn dakedenaku_te_uff5e_mo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: だけしか
pub fn dakeshika() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: は言うまでもない ①
pub fn haiumademonai_u2460() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 決して〜ない
pub fn kesshite_u301c_nai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: わけにはいかない
pub fn wakenihaikanai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 〜ようとしない
pub fn u301c_youtoshinai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: もしかしたら
pub fn moshikashitara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: たとえ〜ても
pub fn tatoe_u301c_temo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ことに
pub fn kotoni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ことか
pub fn kotoka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～かというと ①
pub fn uff5e_katoiuto_u2460() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～かというと ②
pub fn uff5e_katoiuto_u2461() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: で言うと
pub fn deiuto() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～ずつ
pub fn uff5e_zutsu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ずっと ②
pub fn zutto_u2461() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: だらけ
pub fn darake() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: もっとも
pub fn mottomo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 再び
pub fn futatabi() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: み
pub fn mi() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: と同じくらい
pub fn toonajikurai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: と同じで・と違って
pub fn toonajide_u30fb_tochigatte() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: と並んで
pub fn tonarande() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に違いない
pub fn nichigainai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 当たり
pub fn atari() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に当たる
pub fn niataru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に限る
pub fn nikagiru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: とは限らない
pub fn tohakagiranai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: めったに〜ない
pub fn mettani_u301c_nai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 割に
pub fn warini() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Verb[volitional]とする
pub fn verb_volitional_tosuru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Verb[volitional] + としたが
pub fn verb_volitional_toshitaga() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 言うまでもない ②
pub fn iumademonai_u2461() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: そうもない
pub fn soumonai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ないことはない
pub fn naikotohanai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なんか・なんて
pub fn nanka_u30fb_nante() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 又〜も
pub fn mata_u301c_mo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ついでに
pub fn tsuideni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: と共に
pub fn totomoni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: につれて
pub fn nitsurete() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 直ちに
pub fn tadachini() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: たとたんに
pub fn tatotanni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: おきに
pub fn okini() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: たびに
pub fn tabini() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: あるいは
pub fn aruiha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ながらも
pub fn nagaramo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: において・における
pub fn nioite_u30fb_niokeru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 第一
pub fn daiichi() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ますます
pub fn masumasu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 一方だ
pub fn ippouda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 一方で
pub fn ippoude() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 遂に
pub fn tsuini() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: すでに
pub fn sudeni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ずに
pub fn zuni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ずにはいられない
pub fn zunihairarenai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なし
pub fn nashi() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: あり
pub fn ari() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 考えられない
pub fn kangaerarenai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 必ずしも 
pub fn kanarazushimo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 連用形
pub fn renyoukei() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 向き
pub fn muki() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 向け
pub fn muke() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 上がる・上げる
pub fn agaru_u30fb_ageru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 切る
pub fn kiru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 切れない
pub fn kirenai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: きり
pub fn kiri() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: かけ
pub fn kake() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にかけて
pub fn nikakete() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: たて
pub fn tate() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 込む ①
pub fn komu_u2460() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 込む ②
pub fn komu_u2461() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ふりをする
pub fn furiwosuru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: できれば・できたら
pub fn dekireba_u30fb_dekitara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: でよければ
pub fn deyokereba() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 次第
pub fn shidai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: とおり
pub fn toori() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: でもある
pub fn demoaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: どうしても
pub fn doushitemo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: もしも～なら・もしも～でも
pub fn moshimo_uff5e_nara_u30fb_moshimo_uff5e_demo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 同士
pub fn doushi() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: がたい
pub fn gatai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: まさか
pub fn masaka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 前者は・後者は
pub fn zenshaha_u30fb_koushaha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: つい
pub fn tsui() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: せいで
pub fn seide() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: くせに
pub fn kuseni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: がち
pub fn gachi() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ぎみ
pub fn gimi() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: っぽい
pub fn ppoi() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: っぱなし
pub fn ppanashi() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: わざわざ
pub fn wazawaza() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 一体
pub fn ittai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 折角
pub fn sekkaku() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: っけ
pub fn kke() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 代わりに
pub fn kawarini() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に代わって
pub fn nikawatte() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: どころか
pub fn dokoroka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: という理由で
pub fn toiuriyuude() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～は～となっている
pub fn uff5e_ha_uff5e_tonatteiru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 左右する
pub fn sayuusuru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}
