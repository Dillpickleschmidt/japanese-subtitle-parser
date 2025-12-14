use crate::pattern_matcher::TokenMatcher;
use super::Matcher;
use std::sync::Arc;

// Pattern: 得る・得る
pub fn eru_u30fb_eru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 〜得ない (cannot, impossible)
// Structures: Verb[stem] + えない / えません
pub fn u301c_enai() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct VerbStemMatcher;
    impl Matcher for VerbStemMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形")
        }
    }

    #[derive(Debug)]
    struct EruVerbMatcher;
    impl Matcher for EruVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "え"
                && token.base_form == "える"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    #[derive(Debug)]
    struct NaiOrMasenMatcher;
    impl Matcher for NaiOrMasenMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match ない (助動詞)
            (token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞"))
            // OR match ませ (助動詞, ます base form)
            || (token.surface == "ませ"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞"))
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbStemMatcher)),
        TokenMatcher::Custom(Arc::new(EruVerbMatcher)),
        TokenMatcher::Custom(Arc::new(NaiOrMasenMatcher)),
        // Optional ん for ません form
        TokenMatcher::Optional(Box::new(TokenMatcher::Surface("ん"))),
    ]
}

// Pattern: ざるを得ない
pub fn zaruwoenai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～ざる
pub fn uff5e_zaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: つもりで
pub fn tsumoride() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: どうせ
pub fn douse() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: せめて
pub fn semete() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: どうやら
pub fn douyara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なにやら
pub fn naniyara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: よりほかない
pub fn yorihokanai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 確かに
pub fn tashikani() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 一応 ①
pub fn ichiou_u2460() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 一応 ②
pub fn ichiou_u2461() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に相違ない
pub fn nisouinai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 万が一
pub fn mangaichi() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ようがない・ようもない
pub fn youganai_u30fb_youmonai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にほかならない
pub fn nihokanaranai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: っこない
pub fn kkonai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: それなら
pub fn sorenara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ものなら①
pub fn mononara_u2460() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～を～に任せる (entrust X to Y)
// Structures: Nounを + Nounに + 任せる
pub fn uff5e_wo_uff5e_nimakaseru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match を particle (object marker)
    #[derive(Debug)]
    struct WoMatcher;
    impl super::Matcher for WoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "を"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    // Match に particle (target marker)
    #[derive(Debug)]
    struct NiMatcher;
    impl super::Matcher for NiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    // Match 任せる verb in any conjugation
    #[derive(Debug)]
    struct MakaseruMatcher;
    impl super::Matcher for MakaseruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "任せる"
                && token.pos.first().is_some_and(|p| p == "動詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(WoMatcher)),
        TokenMatcher::Wildcard {
            min: 1,
            max: 1,
            stop_conditions: vec![],
        },
        TokenMatcher::Custom(Arc::new(NiMatcher)),
        TokenMatcher::Custom(Arc::new(MakaseruMatcher)),
    ]
}

// Pattern: ～を～に任せる (target-task order: に...を)
// This handles sentences where the target comes before the task, like:
// "新人君にあの重要なプレゼンを任せた" (entrusted presentation to newcomer)
pub fn uff5e_wo_uff5e_nimakaseru_reverse() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Reuse matchers from the main pattern
    #[derive(Debug)]
    struct WoMatcher;
    impl super::Matcher for WoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "を"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    #[derive(Debug)]
    struct NiMatcher;
    impl super::Matcher for NiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    #[derive(Debug)]
    struct MakaseruMatcher;
    impl super::Matcher for MakaseruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "任せる"
                && token.pos.first().is_some_and(|p| p == "動詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NiMatcher)),
        TokenMatcher::Wildcard {
            min: 4,
            max: 4,
            stop_conditions: vec![],
        },
        TokenMatcher::Custom(Arc::new(WoMatcher)),
        TokenMatcher::Custom(Arc::new(MakaseruMatcher)),
    ]
}

// Pattern: 活かす
pub fn ikasu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: おおよそ (approximately, roughly)
// Structure: おおよそ/およそ + (Noun/Number)
pub fn ooyoso() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct OoyosoMatcher;
    impl super::Matcher for OoyosoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match both おおよそ (副詞) and およそ (接頭詞/副詞)
            (token.surface == "おおよそ" && token.pos.first().is_some_and(|pos| pos == "副詞"))
                || (token.surface == "およそ"
                    && (token.pos.first().is_some_and(|pos| pos == "接頭詞")
                        || token.pos.first().is_some_and(|pos| pos == "副詞")))
        }
    }

    vec![TokenMatcher::Custom(Arc::new(OoyosoMatcher))]
}

// Pattern: まい
pub fn mai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 上
pub fn ue() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 上に
pub fn ueni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 以上 ②
pub fn ijou_u2461() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 以上に
pub fn ijouni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 途中に・途中で
pub fn tochuuni_u30fb_tochuude() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 中を
pub fn nakawo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: を中心に
pub fn wochuushinni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: その上
pub fn sonoue() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 上は
pub fn ueha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: の下で
pub fn noshitade() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 後(の) Noun
pub fn kou_no_noun() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 手前
pub fn temae() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: を巡って
pub fn womegutte() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にわたって
pub fn niwatatte() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に沿って
pub fn nisotte() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: た末・の末
pub fn tasue_u30fb_nosue() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にしたがって
pub fn nishitagatte() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に伴って・に伴い
pub fn nitomonatte_u30fb_nitomonai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: につき
pub fn nitsuki() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: につけ
pub fn nitsuke() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にかかわる
pub fn nikakawaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に向かって・に向けて
pub fn nimukatte_u30fb_nimukete() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: が気になる
pub fn gakininaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に気をつける
pub fn nikiwotsukeru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: も構わず
pub fn mokamawazu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: かねる
// Pattern: かねる (cannot, difficult to do)
// Structures: Verb[stem] + かねる / かねます
pub fn kaneru() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct VerbStemMatcher;
    impl Matcher for VerbStemMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形")
        }
    }

    #[derive(Debug)]
    struct KaneruVerbMatcher;
    impl Matcher for KaneruVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "かね" || token.surface == "兼ね")
                && token.base_form == "かねる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    #[derive(Debug)]
    struct MasuOrRuMatcher;
    impl Matcher for MasuOrRuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match ます (助動詞)
            (token.surface == "ます"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞"))
            // OR match る (基本形 of かねる)
            || (token.surface == "る"
                && token.base_form == "かねる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立"))
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbStemMatcher)),
        TokenMatcher::Custom(Arc::new(KaneruVerbMatcher)),
        TokenMatcher::Custom(Arc::new(MasuOrRuMatcher)),
    ]
}

// Pattern: かねない (might, capable of)
// Structures: Verb[stem] + かねない / かねません
pub fn kanenai() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct VerbStemMatcher;
    impl Matcher for VerbStemMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形")
        }
    }

    #[derive(Debug)]
    struct KaneruVerbMatcher;
    impl Matcher for KaneruVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "かね" || token.surface == "兼ね")
                && token.base_form == "かねる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    #[derive(Debug)]
    struct NaiOrMasenMatcher;
    impl Matcher for NaiOrMasenMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match ない (助動詞)
            (token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞"))
            // OR match ませ (助動詞, ます base form)
            || (token.surface == "ませ"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞"))
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbStemMatcher)),
        TokenMatcher::Custom(Arc::new(KaneruVerbMatcher)),
        TokenMatcher::Custom(Arc::new(NaiOrMasenMatcher)),
        // Optional ん for ません form
        TokenMatcher::Optional(Box::new(TokenMatcher::Surface("ん"))),
    ]
}

// Pattern: を除いて
pub fn wonozoite() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にかかわらず
pub fn nikakawarazu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にもかかわらず
pub fn nimokakawarazu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に限って
pub fn nikagitte() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に限らず
pub fn nikagirazu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なお①
pub fn nao_u2460() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なお②
pub fn nao_u2461() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 限り
pub fn kagiri() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 次第だ・次第で
pub fn shidaida_u30fb_shidaide() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 次第に
pub fn shidaini() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～てこそ
pub fn uff5e_tekoso() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: を問わず
pub fn wotowazu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: よりしかたがない
pub fn yorishikataganai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に越したことはない
pub fn nikoshitakotohanai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 要するに
pub fn yousuruni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: てからでないと
pub fn tekaradenaito() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なくはない
pub fn nakuhanai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ないことには～ない
pub fn naikotoniha_uff5e_nai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ないではいられない
pub fn naidehairarenai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ねばならない
pub fn nebanaranai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: たまえ
pub fn tamae() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～のうち(で)
pub fn uff5e_nouchi_de() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: つつ
pub fn tsutsu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: つつ(も)
pub fn tsutsu_mo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に際して
pub fn nisaishite() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 際に
pub fn saini() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にあたり・にあたって
pub fn niatari_u30fb_niatatte() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: を契機に
pub fn wokeikini() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: つつある
pub fn tsutsuaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～ところに・～ところへ
pub fn uff5e_tokoroni_u30fb_uff5e_tokorohe() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: か〜ないかのうちに
pub fn ka_u301c_naikanouchini() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: がけに
pub fn gakeni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ていては
pub fn teiteha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ところだった ②
pub fn tokorodatta_u2461() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: どころではない
pub fn dokorodehanai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ぶりに
pub fn burini() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ては
pub fn teha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ては〜ては
pub fn teha_u301c_teha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: も又
pub fn momata() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 結果・の結果
pub fn kekka_u30fb_nokekka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 以来
pub fn irai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に先立ち
pub fn nisakidachi() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: はたして
pub fn hatashite() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 甲斐がある
pub fn kaigaaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: やがて
pub fn yagate() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: したがって
pub fn shitagatte() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: あげく (in the end, after all)
// Structures: Verb[た] + あげく / Noun + の + あげく
pub fn ageku() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // あげく is always a noun (can function as adverb)
    #[derive(Debug)]
    struct AgekuMatcher;
    impl super::Matcher for AgekuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "あげく"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副詞可能")
        }
    }

    // の particle matcher for noun + の + あげく pattern
    #[derive(Debug)]
    struct NoParticleMatcher;
    impl super::Matcher for NoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の" && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    // Match: (Noun/Verb) + (optional の) + あげく
    // The optional の handles both verb+あげく and noun+の+あげく patterns
    vec![
        TokenMatcher::Any, // Verb or Noun before あげく
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NoParticleMatcher,
        )))),
        TokenMatcher::Custom(Arc::new(AgekuMatcher)),
    ]
}

// Pattern: きっかけ (opportunity, chance, trigger)
// Structures: を/が + きっかけ + に/で
pub fn kikkake() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for を or が particle
    #[derive(Debug)]
    struct WoGaMatcher;
    impl Matcher for WoGaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "を" || token.surface == "が")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Matcher for きっかけ noun
    #[derive(Debug)]
    struct KikkakeMatcher;
    impl Matcher for KikkakeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "きっかけ"
                && token.base_form == "きっかけ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
        }
    }

    // Matcher for に or で particle
    #[derive(Debug)]
    struct NiDeMatcher;
    impl Matcher for NiDeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "に" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(WoGaMatcher)),
        TokenMatcher::Custom(Arc::new(KikkakeMatcher)),
        TokenMatcher::Custom(Arc::new(NiDeMatcher)),
    ]
}

// Pattern: にかけては
pub fn nikaketeha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: とっくに
pub fn tokkuni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 未だに
pub fn imadani() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: をもとに
pub fn womotoni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: からには (as long as, since, given that)
// Structures: Verb[る/た] + からには, Adjective + からには, Noun/な-Adj + である + からには
pub fn karaniha() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // からには is always tokenized as a single conjunctive particle
    #[derive(Debug)]
    struct KaranihaMatcher;
    impl Matcher for KaranihaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "からには"
                && token.base_form == "からには"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Use TokenMatcher::Any for simplicity - からには can attach to various forms
    // Note: This will match just the immediately preceding token, which may be:
    // - A verb (dictionary form or stem)
    // - An adjective
    // - An auxiliary (た in verb phrases, ある in である)
    // The full grammatical construction may span multiple tokens, but this
    // is the minimal meaningful unit for pattern detection.
    vec![
        TokenMatcher::Any,
        TokenMatcher::Custom(Arc::new(KaranihaMatcher)),
    ]
}

// Pattern: いつの間にか (before one knows it, suddenly)
// Structures: いつのまにか + Phrase
pub fn itsunomanika() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // いつのまにか is always tokenized as a single adverb token
    #[derive(Debug)]
    struct ItsunomaniकaMatcher;
    impl super::Matcher for ItsunomaniकaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "いつのまにか"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(ItsunomaniकaMatcher))]
}

// Pattern: 一旦
pub fn ittan() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: はもとより
pub fn hamotoyori() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: そうにない
pub fn souninai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に反して
pub fn nihanshite() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 逆に
pub fn gyakuni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 反面
pub fn hanmen() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 抜く
pub fn nuku() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 抜きで
pub fn nukide() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: いよいよ (finally, at last, more and more)
// Structures: いよいよ + Phrase
pub fn iyoiyo() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct IyoiyoMatcher;
    impl Matcher for IyoiyoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "いよいよ"
                && token.base_form == "いよいよ"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(IyoiyoMatcher))]
}

// Pattern: ずに済む
pub fn zunisumu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に応じて
pub fn nioujite() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: を通じて・を通して
pub fn wotsuujite_u30fb_wotooshite() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に応えて
pub fn nikotaete() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: それとも
pub fn soretomo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にしたら
pub fn nishitara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にしても～にしても
pub fn nishitemo_uff5e_nishitemo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: としては
pub fn toshiteha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: としても
pub fn toshitemo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: それにしても
pub fn sorenishitemo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ぬ
pub fn nu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ことなく
pub fn kotonaku() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にて
pub fn nite() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: には
pub fn niha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 思うように
pub fn omouyouni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: かと思ったら・かと思うと
pub fn katoomottara_u30fb_katoomouto() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: というものでもない
pub fn toiumonodemonai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: と考えられる
pub fn tokangaerareru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: という点から考えると
pub fn toiutenkarakangaeruto() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ということは
pub fn toiukotoha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ふうに
pub fn fuuni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: という風に
pub fn toiukazeni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ものの
pub fn monono() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: というものだ
pub fn toiumonoda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: から見ると (from the perspective of, judging from)
// Structures: Noun + から + 見る + と/ば/て/たら
pub fn karamiruto() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match から as case particle
    #[derive(Debug)]
    struct KaraMatcher;
    impl super::Matcher for KaraMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "から"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match みる verb in any conjugation form (基本形, 仮定形, 連用形)
    #[derive(Debug)]
    struct MiruVerbMatcher;
    impl super::Matcher for MiruVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "みる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立")
        }
    }

    // Match と, ば, て, or たら as ending
    #[derive(Debug)]
    struct EndingMatcher;
    impl super::Matcher for EndingMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // と or ば or て (all connective particles)
            if (token.surface == "と" || token.surface == "ば" || token.surface == "て")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
            {
                return true;
            }
            // たら (auxiliary verb in hypothetical form)
            if token.surface == "たら"
                && token.base_form == "た"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
            {
                return true;
            }
            false
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(KaraMatcher)),
        TokenMatcher::Custom(Arc::new(MiruVerbMatcher)),
        TokenMatcher::Custom(Arc::new(EndingMatcher)),
    ]
}

// Pattern: ところを見ると
pub fn tokorowomiruto() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: からすると・からすれば (judging from, considering)
// Structures: Noun + から + する + と/ば
pub fn karasuruto_u30fb_karasureba() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match から as case particle
    #[derive(Debug)]
    struct KaraMatcher;
    impl super::Matcher for KaraMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "から"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match する verb in either base form (基本形) or hypothetical form (仮定形)
    #[derive(Debug)]
    struct SuruVerbMatcher;
    impl super::Matcher for SuruVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "する"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立")
                && (token.features.get(5).is_some_and(|f| f == "基本形")
                    || token.features.get(5).is_some_and(|f| f == "仮定形"))
        }
    }

    // Match と or ば as connective particle
    #[derive(Debug)]
    struct ToOrBaMatcher;
    impl super::Matcher for ToOrBaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "と" || token.surface == "ば")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(KaraMatcher)),
        TokenMatcher::Custom(Arc::new(SuruVerbMatcher)),
        TokenMatcher::Custom(Arc::new(ToOrBaMatcher)),
    ]
}

// Pattern: からして (based on, judging from)
// Structures: Noun + から + し + て
pub fn karashite() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match から as case particle
    #[derive(Debug)]
    struct KaraMatcher;
    impl Matcher for KaraMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "から"
                && token.base_form == "から"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match し as the 連用形 of する
    #[derive(Debug)]
    struct ShiMatcher;
    impl Matcher for ShiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "し"
                && token.base_form == "する"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形")
        }
    }

    // Match て as connective particle
    #[derive(Debug)]
    struct TeMatcher;
    impl Matcher for TeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.base_form == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    vec![
        TokenMatcher::Any, // Noun
        TokenMatcher::Custom(Arc::new(KaraMatcher)),
        TokenMatcher::Custom(Arc::new(ShiMatcher)),
        TokenMatcher::Custom(Arc::new(TeMatcher)),
    ]
}

// Pattern: からといって (just because)
// Structures: Verb/い-Adj + から + と + いって, な-Adj/Noun + だ + から + と + いって
pub fn karatoitte() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match から as connective particle
    #[derive(Debug)]
    struct KaraConnectiveMatcher;
    impl Matcher for KaraConnectiveMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "から"
                && token.base_form == "から"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Match と as quotation particle
    #[derive(Debug)]
    struct ToQuotationMatcher;
    impl Matcher for ToQuotationMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "と"
                && token.base_form == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "引用")
        }
    }

    // Match いう verb (various forms: いっ, いわ, etc.)
    #[derive(Debug)]
    struct IuVerbMatcher;
    impl Matcher for IuVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "いう"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    // Match て as connective particle
    #[derive(Debug)]
    struct TeConnectiveMatcher;
    impl Matcher for TeConnectiveMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.base_form == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    vec![
        TokenMatcher::Any, // Verb, い-Adj, or だ (for な-Adj/Noun)
        TokenMatcher::Custom(Arc::new(KaraConnectiveMatcher)),
        TokenMatcher::Custom(Arc::new(ToQuotationMatcher)),
        TokenMatcher::Custom(Arc::new(IuVerbMatcher)),
        TokenMatcher::Custom(Arc::new(TeConnectiveMatcher)),
    ]
}

// Pattern: そういえば
pub fn souieba() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: お～願う (humble request: please do)
// Structures:
//   - お + Verb[stem] + 願う/願います
//   - ご + Chinese-origin Noun + 願う/願います
//   - Western-origin Noun + 願う/願います (no prefix)
pub fn o_uff5e_negau() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for お or ご prefix (optional for Western nouns)
    #[derive(Debug)]
    struct OGoMatcher;
    impl Matcher for OGoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "お" || token.surface == "ご")
                && token.pos.first().is_some_and(|pos| pos == "接頭詞")
                && token.pos.get(1).is_some_and(|pos| pos == "名詞接続")
        }
    }

    // Matcher for verb stem or sahen noun before 願う
    // This should match: Verb[連用形] OR Noun[サ変接続]
    #[derive(Debug)]
    struct VerbOrNounMatcher;
    impl Matcher for VerbOrNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Verb in conjunctive form (連用形)
            let is_verb_stem = token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形");

            // Sahen-setsuzoku noun (can become verb with する)
            let is_sahen_noun = token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "サ変接続");

            is_verb_stem || is_sahen_noun
        }
    }

    // Matcher for 願う verb (either 非自立 after verb, or 自立 after noun)
    #[derive(Debug)]
    struct NegauVerbMatcher;
    impl Matcher for NegauVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "願う"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "非自立")
                    || token.pos.get(1).is_some_and(|pos| pos == "自立"))
        }
    }

    // Matcher for ます auxiliary (optional)
    #[derive(Debug)]
    struct MasuMatcher;
    impl Matcher for MasuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(OGoMatcher)))),
        TokenMatcher::Custom(Arc::new(VerbOrNounMatcher)),
        TokenMatcher::Custom(Arc::new(NegauVerbMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(MasuMatcher)))),
    ]
}

// Pattern: ～て頂戴
pub fn uff5e_techoudai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: とか
pub fn toka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 〜ようではないか (why don't we, let's)
// Structures: Verb[volitional] + (で/じゃ) + は? + ない + か
// Two tokenization patterns:
//   1. Godan verbs: Verb[未然ウ接続] + う(助動詞) + ...
//   2. Ichidan verbs: Verb[連用形] + よう(名詞/接尾) + ...
pub fn u301c_youdehanaika() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match verb in volitional form - handles both tokenization patterns
    #[derive(Debug)]
    struct VolitionalVerbMatcher;
    impl Matcher for VolitionalVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && (token.features.get(5).is_some_and(|f| f == "未然ウ接続") // Godan: 戦お
                    || token.features.get(5).is_some_and(|f| f == "連用形"))  // Ichidan: 考え
        }
    }

    // Match う auxiliary (for godan) or よう suffix (for ichidan)
    #[derive(Debug)]
    struct VolitionalAuxiliaryMatcher;
    impl Matcher for VolitionalAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // う auxiliary (godan verbs: 戦おう)
            (token.surface == "う"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "う")
            ||
            // よう suffix (ichidan verbs: 考えよう)
            (token.surface == "よう"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾"))
        }
    }

    // Match で (copula da in te-form) or じゃ (abbreviated)
    #[derive(Debug)]
    struct DeOrJaMatcher;
    impl Matcher for DeOrJaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // で from だ (copula) - used after godan verbs
            (token.surface == "で"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞"))
            ||
            // で as case particle - used after よう (noun suffix) in ichidan verbs
            (token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞"))
            ||
            // じゃ (abbreviated)
            (token.surface == "じゃ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞"))
        }
    }

    // Match は (only for ではないか, not for じゃないか)
    #[derive(Debug)]
    struct HaParticleMatcher;
    impl Matcher for HaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Match ない auxiliary
    #[derive(Debug)]
    struct NaiAuxiliaryMatcher;
    impl Matcher for NaiAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "ない"
        }
    }

    // Match か ending particle
    #[derive(Debug)]
    struct KaParticleMatcher;
    impl Matcher for KaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "か"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VolitionalVerbMatcher)),
        TokenMatcher::Custom(Arc::new(VolitionalAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(DeOrJaMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(HaParticleMatcher)))),
        TokenMatcher::Custom(Arc::new(NaiAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(KaParticleMatcher)),
    ]
}

// Pattern: かのようだ (as if, seems like)
// Structures: Any + か + の + よう + だ/です/に/な
pub fn kanoyouda() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for か particle (adverbial/parallel/sentence-ending)
    #[derive(Debug)]
    struct KaMatcher;
    impl Matcher for KaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "か"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token
                    .pos
                    .get(1)
                    .is_some_and(|pos| pos == "副助詞／並立助詞／終助詞")
        }
    }

    // Matcher for の particle (nominalization/adnominal)
    #[derive(Debug)]
    struct NoMatcher;
    impl Matcher for NoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
        }
    }

    // Matcher for よう (auxiliary verb stem)
    #[derive(Debug)]
    struct YouMatcher;
    impl Matcher for YouMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "よう"
                && token.base_form == "よう"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                && token.pos.get(2).is_some_and(|pos| pos == "助動詞語幹")
        }
    }

    // Matcher for ending: だ/です/に/な
    #[derive(Debug)]
    struct EndingMatcher;
    impl Matcher for EndingMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // だ or です (auxiliary verb)
            if (token.surface == "だ" || token.surface == "です")
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
            {
                return true;
            }
            // に (adverbializer)
            if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副詞化")
            {
                return true;
            }
            // な (adnominal form of だ)
            if token.surface == "な"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token
                    .features
                    .get(5)
                    .is_some_and(|f| f == "体言接続")
            {
                return true;
            }
            false
        }
    }

    vec![
        TokenMatcher::Any,
        TokenMatcher::Custom(Arc::new(KaMatcher)),
        TokenMatcher::Custom(Arc::new(NoMatcher)),
        TokenMatcher::Custom(Arc::new(YouMatcher)),
        TokenMatcher::Custom(Arc::new(EndingMatcher)),
    ]
}

// Pattern: のではないだろうか
pub fn nodehanaidarouka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: て当然だ
pub fn tetouzenda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: のも当然だ
pub fn nomotouzenda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: たった(の)
pub fn tatta_no() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 恐れがある
pub fn osoregaaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: おそらく (probably, perhaps)
// Structures: おそらく + Phrase
pub fn osoraku() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct OsorakuMatcher;
    impl Matcher for OsorakuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "おそらく"
                && token.base_form == "おそらく"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(OsorakuMatcher))]
}

// Pattern: ものか
pub fn monoka() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: おまけに (besides, in addition, to make matters worse)
// Structure: おまけ + に
pub fn omakeni() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct OmakeMatcher;
    impl super::Matcher for OmakeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "おまけ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(OmakeMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
    ]
}

// Pattern: に決まっている
pub fn nikimatteiru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ことになっている
pub fn kotoninatteiru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 気
pub fn ki() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: げ (seeming, appearance)
// Structures: Adj[stem]/Verb[連用形]/Noun + げ + に/な
pub fn ge() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for げ suffix (名詞/接尾/一般)
    #[derive(Debug)]
    struct GeMatcher;
    impl Matcher for GeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "げ"
                && token.base_form == "げ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                && token.pos.get(2).is_some_and(|pos| pos == "一般")
        }
    }

    // Matcher for に/な ending
    #[derive(Debug)]
    struct NiNaMatcher;
    impl Matcher for NiNaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // に (case particle)
            if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            {
                return true;
            }
            // な (adnominal form of だ)
            if token.surface == "な"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token
                    .features
                    .get(5)
                    .is_some_and(|f| f == "体言接続")
            {
                return true;
            }
            false
        }
    }

    vec![
        TokenMatcher::Any,
        TokenMatcher::Custom(Arc::new(GeMatcher)),
        TokenMatcher::Custom(Arc::new(NiNaMatcher)),
    ]
}

// Pattern: ことだから
pub fn kotodakara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ものだから
pub fn monodakara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ものですから・もので
pub fn monodesukara_u30fb_monode() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ものがある
pub fn monogaaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 傾向がある
pub fn keikougaaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ～に値する
pub fn uff5e_niataisuru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: てしょうがない
pub fn teshouganai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: だけましだ
pub fn dakemashida() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 幸い・幸いなことに
pub fn saiwai_u30fb_saiwainakotoni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ようでは・ようじゃ
pub fn youdeha_u30fb_youja() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: さすが
pub fn sasuga() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ことは〜が
pub fn kotoha_u301c_ga() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 更に
pub fn sarani() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 精々
pub fn kiyoshi_u3005() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 僅かに
pub fn wazukani() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: および (and, as well as)
// Structures: Noun + および
pub fn oyobi() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // および can be tokenized as either:
    // - 助詞/接続助詞 (connective particle) when following a noun directly
    // - 接続詞 (conjunction) when starting a clause (e.g., after punctuation)
    #[derive(Debug)]
    struct OyobiMatcher;
    impl super::Matcher for OyobiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            if token.surface != "および" {
                return false;
            }
            // Match either conjunction or connective particle
            let is_conjunction = token.pos.first().is_some_and(|pos| pos == "接続詞");
            let is_particle = token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞");
            is_conjunction || is_particle
        }
    }

    // Punctuation matcher for optional comma before および
    #[derive(Debug)]
    struct PunctuationMatcher;
    impl super::Matcher for PunctuationMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "記号")
                && token.pos.get(1).is_some_and(|pos| pos == "読点")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            PunctuationMatcher,
        )))),
        TokenMatcher::Custom(Arc::new(OyobiMatcher)),
    ]
}

// Pattern: たちまち
pub fn tachimachi() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: いきなり (suddenly, all of a sudden)
// Structures: いきなり + (Action) Phrase
pub fn ikinari() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct IkinariMatcher;
    impl Matcher for IkinariMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "いきなり"
                && token.base_form == "いきなり"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token
                    .pos
                    .get(1)
                    .is_some_and(|pos| pos == "助詞類接続")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(IkinariMatcher))]
}

// Pattern: といった
pub fn toitta() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: を込めて
pub fn wokomete() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: に加えて
pub fn nikuwaete() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 何から何まで
pub fn nanikarananimade() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: は別として
pub fn habetsutoshite() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: だけに
pub fn dakeni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: だけは
pub fn dakeha() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: だけあって
pub fn dakeatte() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 何より
pub fn naniyori() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 何といっても
pub fn nanitoittemo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: か何か (or something, or something like that)
// Structures: Noun + か + なに + か (or かなにか as single adverb)
pub fn kananika() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match か as particle
    #[derive(Debug)]
    struct KaMatcher;
    impl super::Matcher for KaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "か"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token
                    .pos
                    .get(1)
                    .is_some_and(|pos| pos == "副助詞／並立助詞／終助詞")
        }
    }

    // Match なに (pronoun) OR なにか (adverb)
    #[derive(Debug)]
    struct NaniOrNanikaMatcher;
    impl super::Matcher for NaniOrNanikaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // なに as pronoun
            if token.surface == "なに"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "代名詞")
            {
                return true;
            }
            // かなにか as single adverb token
            if token.surface == "なにか"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
            {
                return true;
            }
            false
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(KaMatcher)),
        TokenMatcher::Custom(Arc::new(NaniOrNanikaMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(KaMatcher)))),
    ]
}

// Pattern: てならない
pub fn tenaranai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: のみならず
pub fn nominarazu() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: それなのに
pub fn sorenanoni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: いわゆる (so-called, what is called)
// Structure: いわゆる + Noun
pub fn iwayuru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct IwayuruMatcher;
    impl super::Matcher for IwayuruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "いわゆる"
                && token.pos.first().is_some_and(|pos| pos == "連体詞")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(IwayuruMatcher))]
}

// Pattern: にすぎない
pub fn nisuginai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: も～ば～も
pub fn mo_uff5e_ba_uff5e_mo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: でしかない
pub fn deshikanai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: てたまらない
pub fn tetamaranai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にせよ・にしろ
pub fn niseyo_u30fb_nishiro() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 何しろ
pub fn nanishiro() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: にしろ～にしろ
pub fn nishiro_uff5e_nishiro() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: はともかく
pub fn hatomokaku() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ならともかく
pub fn naratomokaku() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: やら～やら
pub fn yara_uff5e_yara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: しかしながら
pub fn shikashinagara() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ことにはならない
pub fn kotonihanaranai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: だけのことはある
pub fn dakenokotohaaru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: てはならない
pub fn tehanaranai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: てはいられない
pub fn tehairarenai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: 陸に～ない
pub fn rikuni_uff5e_nai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: しかも
pub fn shikamo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: てでも
pub fn tedemo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: とも
pub fn tomo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ないわけにはいかない
pub fn naiwakenihaikanai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: というわけではない
pub fn toiuwakedehanai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: のももっともだ
pub fn nomomottomoda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: たって
pub fn tatte() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}
