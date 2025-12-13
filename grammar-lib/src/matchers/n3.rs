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

// Pattern: うちに (while/during - temporal expression)
// Structures: Verb[る] + うちに / い-Adj + うちに / な-Adj + な + うちに / Noun + の + うちに
pub fn uchini() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct AttributivePrecedingMatcher;
    impl Matcher for AttributivePrecedingMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Matches: Verb (基本形), い-Adjective (基本形), な (助動詞 体言接続), の (助詞 連体化)
            if token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "基本形") {
                return true;
            }
            if token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "基本形") {
                return true;
            }
            if token.surface == "な"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "だ"
                && token.features.get(5).is_some_and(|f| f == "体言接続") {
                return true;
            }
            if token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化") {
                return true;
            }
            false
        }
    }

    #[derive(Debug)]
    struct UchiMatcher;
    impl Matcher for UchiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "うち"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    #[derive(Debug)]
    struct NiCaseParticleMatcher;
    impl Matcher for NiCaseParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(AttributivePrecedingMatcher)),
        TokenMatcher::Custom(Arc::new(UchiMatcher)),
        TokenMatcher::Custom(Arc::new(NiCaseParticleMatcher)),
    ]
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

// Pattern: あまり (so much that / excessive - leading to negative result)
// Structures:
//   1. Verb (基本形) + あまり
//   2. Noun + の + あまり (includes さ/み derived nouns)
//   3. な-Adjective + な + あまり
pub fn amari() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct AmariPrecedingMatcher;
    impl Matcher for AmariPrecedingMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match the token directly before あまり:
            // 1. Verb (基本形)
            // 2. の (助詞/連体化)
            // 3. な (助動詞/体言接続)

            // Verb in 基本形
            if token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "基本形") {
                return true;
            }

            // の (助詞/連体化)
            if token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化") {
                return true;
            }

            // な (助動詞/体言接続)
            if token.surface == "な"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "だ"
                && token.features.get(5).is_some_and(|f| f == "体言接続") {
                return true;
            }

            false
        }
    }

    #[derive(Debug)]
    struct AmariMatcher;
    impl Matcher for AmariMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match あまり as 名詞/一般 or 名詞/非自立/副詞可能
            token.surface == "あまり"
                && token.base_form == "あまり"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(AmariPrecedingMatcher)),
        TokenMatcher::Custom(Arc::new(AmariMatcher)),
    ]
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

// Pattern: ことだ (should/ought to - advice/weak command)
// Structures: Verb[る/ない] + こと + だ/です
pub fn kotoda() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct KotoMatcher;
    impl Matcher for KotoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    #[derive(Debug)]
    struct DaDesuMatcher;
    impl Matcher for DaDesuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "だ" || token.surface == "です")
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && (token.features.get(4).is_some_and(|f| f == "特殊・ダ")
                    || token.features.get(4).is_some_and(|f| f == "特殊・デス"))
        }
    }

    vec![
        TokenMatcher::Any,
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
        TokenMatcher::Custom(Arc::new(DaDesuMatcher)),
    ]
}

// Pattern: そうだ 
pub fn souda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: すると (then/upon that/in that case)
// Structures: する + と
pub fn suruto() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct SuruVerbMatcher;
    impl Matcher for SuruVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "する"
                && token.base_form == "する"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立")
                && token.features.get(5).is_some_and(|f| f == "基本形")
        }
    }

    #[derive(Debug)]
    struct ToConjunctionMatcher;
    impl Matcher for ToConjunctionMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(SuruVerbMatcher)),
        TokenMatcher::Custom(Arc::new(ToConjunctionMatcher)),
    ]
}

// Pattern: そうすると (then/if you do that/in that case)
// Structures: そう + する + と
pub fn sousuruto() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct SouAdverbMatcher;
    impl Matcher for SouAdverbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "そう"
                && token.base_form == "そう"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "助詞類接続")
        }
    }

    #[derive(Debug)]
    struct SuruVerbMatcher;
    impl Matcher for SuruVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "する"
                && token.base_form == "する"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立")
                && token.features.get(5).is_some_and(|f| f == "基本形")
        }
    }

    #[derive(Debug)]
    struct ToConjunctionMatcher;
    impl Matcher for ToConjunctionMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(SouAdverbMatcher)),
        TokenMatcher::Custom(Arc::new(SuruVerbMatcher)),
        TokenMatcher::Custom(Arc::new(ToConjunctionMatcher)),
    ]
}

// Pattern: のはXの方だ
pub fn nohaxnohouda() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Noun＋型 (split form: Noun/Adjective + がた/かた)
// Structures: Noun + がた, い-Adj + かた, Noun + の + かた
pub fn nountasukata() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct NounAdjOrNoMatcher;
    impl Matcher for NounAdjOrNoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match: Noun, Adjective, or の particle (not determiners)
            if token.pos.first().is_some_and(|pos| pos == "名詞") {
                return true;
            }
            if token.pos.first().is_some_and(|pos| pos == "形容詞") {
                return true;
            }
            if token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
            {
                return true;
            }
            false
        }
    }

    #[derive(Debug)]
    struct KataGataMatcher;
    impl Matcher for KataGataMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "がた" || token.surface == "かた")
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && (token.base_form == "がた" || token.base_form == "かた")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NounAdjOrNoMatcher)),
        TokenMatcher::Custom(Arc::new(KataGataMatcher)),
    ]
}

// Pattern: Noun＋型 (compound form: ～型 or ～形 as single token)
// Structures: 文型, etc. (compound nouns ending with 型 or 形)
pub fn nountasukata_compound() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KataKeiCompoundMatcher;
    impl Matcher for KataKeiCompoundMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "名詞")
                && (token.base_form.ends_with("型") || token.base_form.ends_with("形"))
                && token.base_form.len() > 3 // More than just "型" or "形" alone
        }
    }

    vec![TokenMatcher::Custom(Arc::new(KataKeiCompoundMatcher))]
}

// Pattern: てごらん
pub fn tegoran() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: Particle + の (nominalization with particles)
// Structures: Noun + Particle(から/と/へ/で/まで) + の
// Meaning: Forms a link between two nouns where noun B has qualities described by noun A + particle
pub fn particle_no() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match particle (から/と/へ/で/まで) that can precede の
    #[derive(Debug)]
    struct LinkingParticleMatcher;
    impl Matcher for LinkingParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match から/と/へ/で as 格助詞 or まで as 副助詞
            if !["から", "と", "へ", "で", "まで"].contains(&token.surface.as_str()) {
                return false;
            }

            // から, と, へ, で are 格助詞
            if (token.surface == "から" || token.surface == "と"
                || token.surface == "へ" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") {
                return true;
            }

            // まで is 副助詞
            if token.surface == "まで"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞") {
                return true;
            }

            false
        }
    }

    // Match の as nominalizing particle (連体化)
    #[derive(Debug)]
    struct NominalizingNoMatcher;
    impl Matcher for NominalizingNoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
        }
    }

    vec![
        super::noun_matcher(), // Preceding noun
        TokenMatcher::Custom(Arc::new(LinkingParticleMatcher)),
        TokenMatcher::Custom(Arc::new(NominalizingNoMatcher)),
    ]
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

// Pattern: ため(に) (for the sake of / in order to - purpose)
// Structures: Verb[る] + ため(に) / Noun + の + ため(に)
pub fn tame_ni() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct TameMatcher;
    impl Matcher for TameMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match ため as 名詞/非自立/副詞可能
            token.surface == "ため"
                && token.base_form == "ため"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match に as 助詞/格助詞
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    #[derive(Debug)]
    struct NoRentaikaMatcher;
    impl Matcher for NoRentaikaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match の as 助詞/連体化 (nominalizing particle)
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
        }
    }

    vec![
        TokenMatcher::Any, // Verb or Noun
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NoRentaikaMatcher,
        )))), // Optional の for nouns
        TokenMatcher::Custom(Arc::new(TameMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NiParticleMatcher,
        )))), // Optional に
    ]
}

// Pattern: ために
pub fn tameni() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ということだ (it is said that / it means that - hearsay/conclusion with certainty)
// Structures: Phrase + ということ + だ / Phrase + ということ + です
pub fn toiukotoda() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct ToiuMatcher;
    impl Matcher for ToiuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "という"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "連語")
        }
    }

    #[derive(Debug)]
    struct KotoMatcher;
    impl Matcher for KotoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    #[derive(Debug)]
    struct DaDesuMatcher;
    impl Matcher for DaDesuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "だ" || token.surface == "です")
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && (token.features.get(4).is_some_and(|f| f == "特殊・ダ")
                    || token.features.get(4).is_some_and(|f| f == "特殊・デス"))
        }
    }

    vec![
        TokenMatcher::Any,
        TokenMatcher::Custom(Arc::new(ToiuMatcher)),
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
        TokenMatcher::Custom(Arc::new(DaDesuMatcher)),
    ]
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

// Pattern: こそ (emphasis particle)
// Structures: Noun + こそ
pub fn koso() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KosoMatcher;
    impl Matcher for KosoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こそ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(KosoMatcher)),
    ]
}

// Pattern: からこそ (precisely because)
// Structures: Verb + た + から + こそ, Noun + だ + から + こそ
pub fn karakoso() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matches た (past auxiliary) or だ (copula) before から
    #[derive(Debug)]
    struct TaOrDaMatcher;
    impl super::Matcher for TaOrDaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "助動詞")
                && (token.base_form == "た" || token.base_form == "だ")
        }
    }

    // Matches から (conjunction particle)
    #[derive(Debug)]
    struct KaraConjunctionMatcher;
    impl super::Matcher for KaraConjunctionMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "から"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Matches こそ (emphatic particle)
    #[derive(Debug)]
    struct KosoMatcher;
    impl super::Matcher for KosoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こそ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(TaOrDaMatcher)),
        TokenMatcher::Custom(Arc::new(KaraConjunctionMatcher)),
        TokenMatcher::Custom(Arc::new(KosoMatcher)),
    ]
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

// Pattern: ことがある (sometimes happens / there are times when)
// Structures: Verb/Adj + こと + が/も + ある
pub fn kotogaaru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct KotoMatcher;
    impl Matcher for KotoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    #[derive(Debug)]
    struct GaMoParticleMatcher;
    impl Matcher for GaMoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "が" || token.surface == "も")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    vec![
        TokenMatcher::Any,
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
        TokenMatcher::Custom(Arc::new(GaMoParticleMatcher)),
        TokenMatcher::specific_verb("ある"),
    ]
}

// Pattern: ことにする
// Pattern: ことにする (decide to / make it that)
// Structures: Verb[る/ない] + ことにする/します
// Expresses conscious decision-making
pub fn kotonisuru() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KotoMatcher;
    impl Matcher for KotoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        TokenMatcher::Any,
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::specific_verb("する"),
    ]
}

// Pattern: ことなの
pub fn kotonano() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ことになる (it has been decided / will end up)
// Structures: Verb/Adjective + ことになる
pub fn kotoninaru() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KotoMatcher;
    impl Matcher for KotoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        TokenMatcher::Any,
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::specific_verb("なる"),
    ]
}

// Pattern: ～は～で有名
pub fn uff5e_ha_uff5e_deyuumei() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ことはない (no need to / never happens)
// Structures: Verb + ことはない/ありません
pub fn kotohanai() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KotoMatcher;
    impl Matcher for KotoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    #[derive(Debug)]
    struct HaParticleMatcher;
    impl Matcher for HaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    #[derive(Debug)]
    struct NaiAruMatcher;
    impl Matcher for NaiAruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match ない (i-adjective) or ある (verb for polite ありません)
            (token.surface == "ない" || token.base_form == "ない")
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
                || (token.base_form == "ある"
                    && token.pos.first().is_some_and(|pos| pos == "動詞"))
        }
    }

    vec![
        TokenMatcher::Any,
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
        TokenMatcher::Custom(Arc::new(HaParticleMatcher)),
        TokenMatcher::Custom(Arc::new(NaiAruMatcher)),
    ]
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

// Pattern: そのため(に) (for that reason/to that end)
// Structures: そのため + (に) + Phrase
pub fn sonotame_ni() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct SonoRentaishiMatcher;
    impl super::Matcher for SonoRentaishiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match その as 連体詞 (prenominal)
            token.surface == "その"
                && token.base_form == "その"
                && token.pos.first().is_some_and(|pos| pos == "連体詞")
        }
    }

    #[derive(Debug)]
    struct TameNounMatcher;
    impl super::Matcher for TameNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match ため as 名詞/非自立
            token.surface == "ため"
                && token.base_form == "ため"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match に as 格助詞
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(SonoRentaishiMatcher)),
        TokenMatcher::Custom(Arc::new(TameNounMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NiParticleMatcher,
        )))),
    ]
}

// Pattern: その結果 (as a result)
// Structure: その + 結果
pub fn sonokekka() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matches その as 連体詞
    #[derive(Debug)]
    struct SonoRentaishiMatcher;
    impl super::Matcher for SonoRentaishiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "その" && token.pos.first().is_some_and(|pos| pos == "連体詞")
        }
    }

    // Matches 結果 as 名詞/副詞可能
    #[derive(Debug)]
    struct KekkaFukushiKanouMatcher;
    impl super::Matcher for KekkaFukushiKanouMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "結果"
                && token.base_form == "結果"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副詞可能")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(SonoRentaishiMatcher)),
        TokenMatcher::Custom(Arc::new(KekkaFukushiKanouMatcher)),
    ]
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
// いくら〜でも: No matter how much (いくら + phrase + ても/でも)
// Structures:
//   - いくら + Verb[ても] (いくら...て + も)
//   - いくら + い-Adjective[ても] (いくら...て + も)
//   - いくら + Noun + でも (いくら...でも as single token)
//   - いくら + な-Adjective + でも (いくら...でも as single token)
pub fn ikura_u301c_demo() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match いくら as adverb or noun
    #[derive(Debug)]
    struct IkuraMatcher;
    impl Matcher for IkuraMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "いくら"
                && (token.pos.first().is_some_and(|pos| pos == "副詞")
                    || token.pos.first().is_some_and(|pos| pos == "名詞"))
        }
    }

    // Match ても (も after て) or でも (single particle)
    #[derive(Debug)]
    struct TemoOrDemoMatcher;
    impl Matcher for TemoOrDemoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match も (係助詞) - for ても pattern
            (token.surface == "も" && token.pos.get(1).is_some_and(|pos| pos == "係助詞"))
                // Match でも (副助詞) - for noun/na-adj + でも pattern
                || (token.surface == "でも" && token.pos.get(1).is_some_and(|pos| pos == "副助詞"))
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(IkuraMatcher)),
        TokenMatcher::Wildcard {
            min: 1,
            max: 5,
            stop_conditions: vec![],
        },
        TokenMatcher::Custom(Arc::new(TemoOrDemoMatcher)),
    ]
}

// Pattern: 〜かは〜によって違う
pub fn u301c_kaha_u301c_niyottechigau() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// かなり: Considerably/quite (adverb form)
// Structures: かなり + Phrase
pub fn kanari() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    #[derive(Debug)]
    struct KanariMatcher;
    impl Matcher for KanariMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "かなり"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(KanariMatcher))]
}

// Pattern: あまりに (excessively/so much)
// Structures: あまりに / あまり + の / あんまり / あまりにも
pub fn amarini() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct AmariniMatcher;
    impl Matcher for AmariniMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "あまりに" || token.surface == "あんまり")
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "助詞類接続")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(AmariniMatcher))]
}

// Pattern: あまりの (あまり + の + Noun)
pub fn amarino_noun() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct AmariMatcher;
    impl Matcher for AmariMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "あまり"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "助詞類接続")
        }
    }

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
        TokenMatcher::Custom(Arc::new(AmariMatcher)),
        TokenMatcher::Custom(Arc::new(NoParticleMatcher)),
        super::noun_matcher(),
    ]
}

// Pattern: あまりにも (あまりに + も for emphasis)
pub fn amarinimo() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct AmariniMatcher;
    impl Matcher for AmariniMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "あまりに"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "助詞類接続")
        }
    }

    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(AmariniMatcher)),
        TokenMatcher::Custom(Arc::new(MoParticleMatcher)),
    ]
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

// Pattern: くらい ② (degree/extent - so...that)
// Structures: Verb/Adjective/Noun + くらい/ぐらい
pub fn kurai_u2461() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KuraiMatcher;
    impl Matcher for KuraiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Matches くらい or ぐらい as 助詞/副助詞
            (token.surface == "くらい" || token.surface == "ぐらい")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞")
        }
    }

    vec![
        TokenMatcher::Any, // Matches verb/adjective/noun before くらい
        TokenMatcher::Custom(Arc::new(KuraiMatcher)),
    ]
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

// Pattern: それぞれ (each/respectively)
// Structures: それぞれ + Phrase, それぞれ + の + Noun
pub fn sorezore() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matches それぞれ as 名詞/副詞可能
    #[derive(Debug)]
    struct SoreZoreMatcher;
    impl super::Matcher for SoreZoreMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "それぞれ"
                && token.base_form == "それぞれ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副詞可能")
        }
    }

    // Matches の as 助詞/連体化 (nominalizing particle)
    #[derive(Debug)]
    struct NoRentaikaMatcher;
    impl super::Matcher for NoRentaikaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(SoreZoreMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NoRentaikaMatcher,
        )))),
    ]
}

// Pattern: そこで (accordingly/as such)
// Structures: (Situation) Phrase。そこで + (Solution) Phrase
pub fn sokode() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct SokodeMatcher;
    impl super::Matcher for SokodeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match そこで as 接続詞 (conjunction)
            token.surface == "そこで"
                && token.base_form == "そこで"
                && token.pos.first().is_some_and(|pos| pos == "接続詞")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(SokodeMatcher))]
}

// Pattern: しかない (no choice but to / there is only)
// Structures: Verb + しかない
pub fn shikanai() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct ShikaParticleMatcher;
    impl super::Matcher for ShikaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match しか (係助詞)
            token.surface == "しか"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    #[derive(Debug)]
    struct NaiAdjMatcher;
    impl super::Matcher for NaiAdjMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match ない as 形容詞/自立
            token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
        }
    }

    vec![
        // Verb (基本形)
        TokenMatcher::Verb {
            conjugation_form: Some("基本形"),
            base_form: None,
        },
        // しか (係助詞)
        TokenMatcher::Custom(Arc::new(ShikaParticleMatcher)),
        // ない (形容詞)
        TokenMatcher::Custom(Arc::new(NaiAdjMatcher)),
    ]
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
// Pattern: ～というのは事実だ (it is a fact that)
// Structures: Phrase + (という) + のは事実だ/です
// Meaning: "it is a fact that / it is true that" - strongly expresses something is true/factual
// Usage: Declares a statement as undeniable truth
// Examples:
//   - 殺したというのは事実だ (It is a fact that [someone] killed)
//   - 無視したのは事実だ (It is true that [I] ignored)
//   - 仲直りしたのは事実です (It is a fact that [we] made up - polite)
// Note: という is optional, making it sound slightly weaker when omitted
pub fn uff5e_toiunohajijitsuda() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct ToiuMatcher;
    impl Matcher for ToiuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "という"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "連語")
        }
    }

    #[derive(Debug)]
    struct NoMatcher;
    impl Matcher for NoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    #[derive(Debug)]
    struct WaParticleMatcher;
    impl Matcher for WaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    #[derive(Debug)]
    struct JijitsuMatcher;
    impl Matcher for JijitsuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "事実"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副詞可能")
        }
    }

    #[derive(Debug)]
    struct DaDesuMatcher;
    impl Matcher for DaDesuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "だ" || token.surface == "です")
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(ToiuMatcher)))),
        TokenMatcher::Custom(Arc::new(NoMatcher)),
        TokenMatcher::Custom(Arc::new(WaParticleMatcher)),
        TokenMatcher::Custom(Arc::new(JijitsuMatcher)),
        TokenMatcher::Custom(Arc::new(DaDesuMatcher)),
    ]
}

// Pattern: から言うと (speaking from, from the viewpoint of)
// Structures: Noun + から + 言う + と/ば/て
pub fn karaiuto() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KaraParticleMatcher;
    impl Matcher for KaraParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "から"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    #[derive(Debug)]
    struct IuVerbMatcher;
    impl Matcher for IuVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "言う"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    #[derive(Debug)]
    struct ConditionalParticleMatcher;
    impl Matcher for ConditionalParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "と" || token.surface == "ば" || token.surface == "て")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    vec![
        TokenMatcher::Any, // Noun
        TokenMatcher::Custom(Arc::new(KaraParticleMatcher)),
        TokenMatcher::Custom(Arc::new(IuVerbMatcher)),
        TokenMatcher::Custom(Arc::new(ConditionalParticleMatcher)),
    ]
}

// Pattern: に取って
pub fn nitotte() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ことから (from the fact that)
// Structures: Verb/Adjective/Noun + ことから
// Used to draw logical conclusions from facts
pub fn kotokara() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KotoMatcher;
    impl Matcher for KotoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    #[derive(Debug)]
    struct KaraParticleMatcher;
    impl Matcher for KaraParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "から"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        TokenMatcher::Any,
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
        TokenMatcher::Custom(Arc::new(KaraParticleMatcher)),
    ]
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
// Pattern: さて (well then / now)
// Structures: さて + (New Topic) Phrase
// Meaning: "well" or "well then" - topic change conjunction
// Usage: Used at sentence beginning to change topic or move to new point
// Examples:
//   - さて、そろそろ出ますか (Well then, shall we head off?)
//   - さて、とりあえず乾杯しましょう (Well, let's toast first)
//   - さて、この問題の答えが分かる人はいますか (Now, does anyone know the answer?)
pub fn sate() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct SateMatcher;
    impl Matcher for SateMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "さて"
                && token.pos.first().is_some_and(|pos| pos == "接続詞")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(SateMatcher))]
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
// Pattern: ことに (particularly/especially/to my...)
// Structures: Verb/Adjective/Noun + ことに
// Expresses emotional emphasis or notable circumstance
pub fn kotoni() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KotoMatcher;
    impl Matcher for KotoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        TokenMatcher::Any,
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
    ]
}

// Pattern: ことか (how / god knows / what)
// Structures: Verb/Adjective/Noun + ことか
// Expresses rhetorical emphasis on extent/magnitude
pub fn kotoka() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KotoMatcher;
    impl Matcher for KotoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    #[derive(Debug)]
    struct KaParticleMatcher;
    impl Matcher for KaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "か"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞／並立助詞／終助詞")
        }
    }

    vec![
        TokenMatcher::Any,
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
        TokenMatcher::Custom(Arc::new(KaParticleMatcher)),
    ]
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

// Pattern: ずっと ② (by far/much more - comparative)
// NOTE: Tokenization is identical to ずっと ① (continuously).
// Both patterns use 副詞/一般. The difference is semantic context:
// - ずっと ① = temporal continuity ("continuously", "the whole time")
// - ずっと ② = comparative degree ("by far", "much more")
//
// According to the Fun Fact in grammar_points_data.json, both meanings
// derive from the same core concept of "unwavering/unfaltering" and
// "far more (A)" / "to the maximum amount possible".
//
// Since we cannot reliably distinguish these structurally, both patterns
// will be detected when ずっと appears. The user should determine the
// meaning from context.
pub fn zutto_u2461() -> Vec<TokenMatcher> {
    use super::Matcher;

    // Match ずっと adverb (副詞/一般) - identical to ずっと ①
    #[derive(Debug)]
    struct ZuttoMatcher;
    impl Matcher for ZuttoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ずっと"
                && token.base_form == "ずっと"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(ZuttoMatcher))]
}

// Pattern: だらけ (covered with/full of - scattered state)
// Structures: Noun + だらけ, Noun + だらけ + の + Noun
pub fn darake() -> Vec<TokenMatcher> {
    use super::Matcher;

    // Match だらけ suffix (名詞/接尾/一般)
    #[derive(Debug)]
    struct DarakeSuffixMatcher;
    impl Matcher for DarakeSuffixMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "だらけ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
        }
    }

    // Match optional の particle (連体化)
    #[derive(Debug)]
    struct NoRentaikaMatcher;
    impl Matcher for NoRentaikaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(DarakeSuffixMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NoRentaikaMatcher)))),
    ]
}

// Pattern: もっとも (although/however/with that said)
// Structures: もっとも + Phrase
pub fn mottomo() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct MottomoMatcher;
    impl Matcher for MottomoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "もっとも"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(MottomoMatcher))]
}

// Pattern: 再び (again/once more/a second time)
// Structures: ふたたび + Phrase
pub fn futatabi() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct FutatabiMatcher;
    impl Matcher for FutatabiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "ふたたび" || token.surface == "再び")
                && token.pos.first().is_some_and(|pos| pos == "副詞")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(FutatabiMatcher))]
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

// Pattern: Verb[volitional]とする (try to / be about to)
// Structures: Verb[未然ウ接続] + う + と + する/します/した/etc.
pub fn verb_volitional_tosuru() -> Vec<TokenMatcher> {
    // Match verb in volitional form (未然ウ接続)
    #[derive(Debug)]
    struct VolitionalVerbMatcher;
    impl Matcher for VolitionalVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然ウ接続")
        }
    }

    // Match う as auxiliary verb (助動詞/不変化型/基本形)
    #[derive(Debug)]
    struct VolitionalAuxiliaryMatcher;
    impl Matcher for VolitionalAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "う"
                && token.base_form == "う"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Match と as quotation particle
    #[derive(Debug)]
    struct ToQuotationMatcher;
    impl Matcher for ToQuotationMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VolitionalVerbMatcher)),
        TokenMatcher::Custom(Arc::new(VolitionalAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(ToQuotationMatcher)),
        TokenMatcher::specific_verb("する"),
    ]
}

// Pattern: ～ようとしない (shall not / doesn't try to)
// Structures: Verb[おう] + としない / としません
pub fn youtoshinai() -> Vec<TokenMatcher> {
    // Match verb in volitional form (未然ウ接続)
    #[derive(Debug)]
    struct VolitionalVerbMatcher;
    impl Matcher for VolitionalVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然ウ接続")
        }
    }

    // Match う as auxiliary verb (助動詞/不変化型/基本形)
    #[derive(Debug)]
    struct VolitionalAuxiliaryMatcher;
    impl Matcher for VolitionalAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "う"
                && token.base_form == "う"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Match と as quotation particle
    #[derive(Debug)]
    struct ToQuotationMatcher;
    impl Matcher for ToQuotationMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match し from する in 未然形 or 連用形
    #[derive(Debug)]
    struct ShiMatcher;
    impl Matcher for ShiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "し"
                && token.base_form == "する"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && (token.features.get(5).is_some_and(|f| f == "未然形")
                    || token.features.get(5).is_some_and(|f| f == "連用形"))
        }
    }

    // Match ない (negative auxiliary) or ませ (polite negative stem)
    #[derive(Debug)]
    struct NaiMaseMatcher;
    impl Matcher for NaiMaseMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞"))
            || (token.surface == "ませ"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞"))
        }
    }

    // For polite form, optionally match ん after ませ
    #[derive(Debug)]
    struct NMatcher;
    impl Matcher for NMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ん"
                && token.base_form == "ん"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VolitionalVerbMatcher)),
        TokenMatcher::Custom(Arc::new(VolitionalAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(ToQuotationMatcher)),
        TokenMatcher::Custom(Arc::new(ShiMatcher)),
        TokenMatcher::Custom(Arc::new(NaiMaseMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NMatcher)))),
    ]
}

// Pattern: ～と言っても (even though / although I say)
// Structures: Verb/Adj/Noun + (だ) + といっても
pub fn toittemo() -> Vec<TokenMatcher> {
    // Match だ as auxiliary (optional for verb/adj, required for na-adj/noun)
    #[derive(Debug)]
    struct DaAuxiliaryMatcher;
    impl Matcher for DaAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "だ"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Match と as quotation particle
    #[derive(Debug)]
    struct ToQuotationMatcher;
    impl Matcher for ToQuotationMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "引用")
        }
    }

    // Match いう in て-form (いっ)
    #[derive(Debug)]
    struct IuVerbTeMatcher;
    impl Matcher for IuVerbTeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "いっ"
                && token.base_form == "いう"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用タ接続")
        }
    }

    // Match て as conjunction particle
    #[derive(Debug)]
    struct TeConjunctionMatcher;
    impl Matcher for TeConjunctionMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Match も as binding particle
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    vec![
        TokenMatcher::Any, // Verb, Adjective, or Noun
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(DaAuxiliaryMatcher)))),
        TokenMatcher::Custom(Arc::new(ToQuotationMatcher)),
        TokenMatcher::Custom(Arc::new(IuVerbTeMatcher)),
        TokenMatcher::Custom(Arc::new(TeConjunctionMatcher)),
        TokenMatcher::Custom(Arc::new(MoParticleMatcher)),
    ]
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
// Pattern: ないことはない (it's not impossible / it's not that...not)
// Structures: Verb[ない] + ことはない / い-Adj[ない] + ことはない / な-Adj + ではない + ことはない
// Double negative expressing possibility, often with half-hearted nuance
pub fn naikotohanai() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct NaiMatcher;
    impl Matcher for NaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ない"
                && (token.pos.first().is_some_and(|pos| pos == "助動詞")
                    || token.pos.first().is_some_and(|pos| pos == "形容詞"))
        }
    }

    #[derive(Debug)]
    struct KotoMatcher;
    impl Matcher for KotoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    #[derive(Debug)]
    struct HaMoParticleMatcher;
    impl Matcher for HaMoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "は" || token.surface == "も")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    #[derive(Debug)]
    struct NaiAruMatcher;
    impl Matcher for NaiAruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match ない (adjective or auxiliary) or ある (verb for polite ありません)
            (token.surface == "ない" || token.base_form == "ない")
                && (token.pos.first().is_some_and(|pos| pos == "形容詞")
                    || token.pos.first().is_some_and(|pos| pos == "助動詞"))
                || (token.base_form == "ある"
                    && token.pos.first().is_some_and(|pos| pos == "動詞"))
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
        TokenMatcher::Custom(Arc::new(HaMoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(NaiAruMatcher)),
    ]
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

// Pattern: 直ちに (immediately/at once - formal/purposeful)
// Structures: ただちに + Phrase
pub fn tadachini() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct TadachiniMatcher;
    impl Matcher for TadachiniMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "ただちに" || token.surface == "直ちに")
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(TadachiniMatcher))]
}

// Pattern: たとたんに (the instant/the moment)
// Structures: Verb［た］+ とたん(に)
pub fn tatotanni() -> Vec<TokenMatcher> {
    use super::{flexible_verb_form, past_auxiliary};

    #[derive(Debug)]
    struct TotanNounMatcher;
    impl Matcher for TotanNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "とたん" || token.surface == "途端")
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                && token.pos.get(2).is_some_and(|pos| pos == "副詞可能")
        }
    }

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        flexible_verb_form(),
        past_auxiliary(),
        TokenMatcher::Custom(Arc::new(TotanNounMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NiParticleMatcher)))),
    ]
}

// Pattern: おきに (at intervals of / every X)
// Structures: Number + Counter + おきに
pub fn okini() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct NumberMatcher;
    impl Matcher for NumberMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "数")
        }
    }

    #[derive(Debug)]
    struct CounterMatcher;
    impl Matcher for CounterMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                && token.pos.get(2).is_some_and(|pos| pos == "助数詞")
        }
    }

    #[derive(Debug)]
    struct OkiMatcher;
    impl Matcher for OkiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "おき"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
        }
    }

    #[derive(Debug)]
    struct NiCaseParticleMatcher;
    impl Matcher for NiCaseParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NumberMatcher)),
        TokenMatcher::Custom(Arc::new(CounterMatcher)),
        TokenMatcher::Custom(Arc::new(OkiMatcher)),
        TokenMatcher::Custom(Arc::new(NiCaseParticleMatcher)),
    ]
}

// Pattern: たびに
// たびに: Every time / Whenever
// Structures: Verb［る］+ たびに, Noun + の + たびに
pub fn tabini() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct TabiNounMatcher;
    impl super::Matcher for TabiNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match たび (名詞/非自立/副詞可能)
            token.surface == "たび"
                && token.base_form == "たび"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                && token.pos.get(2).is_some_and(|pos| pos == "副詞可能")
        }
    }

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl super::Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match に (助詞/格助詞/一般)
            token.surface == "に"
                && token.base_form == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    #[derive(Debug)]
    struct NoRentaikaMatcher;
    impl super::Matcher for NoRentaikaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match の (助詞/連体化)
            token.surface == "の"
                && token.base_form == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
        }
    }

    vec![
        // Any token (verb in 基本形 or noun)
        TokenMatcher::Any,
        // Optional の (連体化 particle for nouns)
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NoRentaikaMatcher,
        )))),
        // たび (名詞/非自立/副詞可能)
        TokenMatcher::Custom(Arc::new(TabiNounMatcher)),
        // に (助詞/格助詞)
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
    ]
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

// Pattern: 遂に (finally/at last - formal)
// Structures: ついに + Phrase
// Meaning: "finally" or "at last" - emphasizes completion after significant time/effort
// Usage: Indicates something has finally happened after a long period of time
// Examples:
//   - ついに日本上陸 (finally arrived in Japan)
//   - ついにドラゴンズが勝った (The Dragons have finally won)
//   - 遂に結婚した (finally married)
// Note: Both ついに (hiragana) and 遂に (with kanji) are common. Slightly more formal than とうとう.
pub fn tsuini() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct TsuiniMatcher;
    impl Matcher for TsuiniMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "ついに" || token.surface == "遂に")
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(TsuiniMatcher))]
}

// Pattern: すでに
// Pattern: すでに (already - formal)
// Structures: すでに + Phrase
// Meaning: "already" - formal alternative to もう
// Usage: Indicates something is already in a completed/unchanging state
// Examples:
//   - すでに沸いている (already boiled)
//   - すでに決まった (already decided)
//   - すでに遅すぎる (already too late)
pub fn sudeni() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct SudeniMatcher;
    impl Matcher for SudeniMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "すでに"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(SudeniMatcher))]
}

// Pattern: ずに (without doing)
// Structures: Verb[未然形] + ず(に)
// Exception: する → せず(に)
pub fn zuni() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match verb in 未然形 (imperfective/negative stem form)
    // This includes both regular verbs and する verbs (which become せ)
    #[derive(Debug)]
    struct MizenFormVerbMatcher;
    impl Matcher for MizenFormVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token
                    .features
                    .get(5)
                    .is_some_and(|f| f == "未然形" || f == "未然ヌ接続")
        }
    }

    // Match ず (助動詞, base=ぬ, 特殊・ヌ, 連用ニ接続)
    #[derive(Debug)]
    struct ZuAuxiliaryMatcher;
    impl Matcher for ZuAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ず"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "ぬ"
        }
    }

    // Match に particle (格助詞/一般) - optional
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(MizenFormVerbMatcher)),
        TokenMatcher::Custom(Arc::new(ZuAuxiliaryMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NiParticleMatcher,
        )))),
    ]
}

// Pattern: ずにはいられない (can't help but do / cannot resist doing)
// Structures: Verb[未然形] + ずにはいられない / ずにはいられません
// Exception: する → せずにはいられない
pub fn zunihairarenai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Reuse MizenFormVerbMatcher from zuni
    #[derive(Debug)]
    struct MizenFormVerbMatcher;
    impl Matcher for MizenFormVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token
                    .features
                    .get(5)
                    .is_some_and(|f| f == "未然形" || f == "未然ヌ接続")
        }
    }

    // Match ず (助動詞, base=ぬ)
    #[derive(Debug)]
    struct ZuAuxiliaryMatcher;
    impl Matcher for ZuAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ず"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "ぬ"
        }
    }

    // Match に particle (格助詞/一般)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match は particle (係助詞)
    #[derive(Debug)]
    struct WaParticleMatcher;
    impl Matcher for WaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Match い or はいら from いる/はいる verb (未然形)
    // Kagome may tokenize "いられ" differently depending on context
    #[derive(Debug)]
    struct IruMizenMatcher;
    impl Matcher for IruMizenMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然形")
                && ((token.surface == "い" && token.base_form == "いる")
                    || (token.surface == "はいら" && token.base_form == "はいる"))
        }
    }

    // Match られ or れ from られる/れる auxiliary (potential)
    // In standard form: られ (未然形)
    // In polite form: れ (連用形)
    #[derive(Debug)]
    struct RareruAuxiliaryMatcher;
    impl Matcher for RareruAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            ((token.surface == "られ" && token.base_form == "られる")
                || (token.surface == "れ" && token.base_form == "れる"))
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
        }
    }

    // Match ない (basic form) or ません (polite negative)
    // For standard: ない (助動詞, 基本形)
    // For polite: ませ + ん
    #[derive(Debug)]
    struct NaiMasenMatcher;
    impl Matcher for NaiMasenMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match ない
            (token.surface == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "ない")
                // Match ませ (from ません)
                || (token.surface == "ませ"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.base_form == "ます")
        }
    }

    // Optional ん for polite form (ません)
    #[derive(Debug)]
    struct NMatcher;
    impl Matcher for NMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ん"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "ん"
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(MizenFormVerbMatcher)),
        TokenMatcher::Custom(Arc::new(ZuAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        // は particle is optional - Kagome may tokenize "はいら" as verb "はいる" without separate は
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(WaParticleMatcher)))),
        TokenMatcher::Custom(Arc::new(IruMizenMatcher)),
        TokenMatcher::Custom(Arc::new(RareruAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(NaiMasenMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NMatcher)))),
    ]
}

// Pattern: なし
pub fn nashi() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: あり
// Pattern: あり (with/possible/exists - literary form of ある)
// Structures: Phrase + あり
// Meaning: "with (A), amongst other things" / "that's possible/acceptable"
// Usage: Indicates something as one possibility among many
// Examples:
//   - ラーメンもあり (Ramen is possible/acceptable)
//   - 駐車場ありのホテル (hotel with parking lot)
//   - 字幕ありで見たい (want to watch with subtitles)
//   - めっちゃありです (that's totally possible - casual response)
pub fn ari() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct AriMatcher;
    impl Matcher for AriMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match あり as verb ある in 連用形 (conjunctive form)
            token.surface == "あり"
                && token.base_form == "ある"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立")
                && token.features.get(5).is_some_and(|f| f == "連用形")
        }
    }

    vec![
        TokenMatcher::Any,  // Preceding word (noun, adjective, adverb, etc.)
        TokenMatcher::Custom(Arc::new(AriMatcher)),
    ]
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

// Pattern: きり (only/just/since)
// Structures: Verb[た] + きり, Noun/Counter + きり/っきり
pub fn kiri() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KiriMatcher;
    impl Matcher for KiriMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Matches きり or っきり as 名詞/非自立/副詞可能 or 名詞/接尾/副詞可能
            (token.surface == "きり" || token.surface == "っきり")
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "非自立")
                    || token.pos.get(1).is_some_and(|pos| pos == "接尾"))
        }
    }

    vec![
        TokenMatcher::Any, // Matches verb/noun/counter before きり
        TokenMatcher::Custom(Arc::new(KiriMatcher)),
    ]
}

// Pattern: かけ
// かけ: Half-finished/about to (Verb[stem] + かけ)
// Structures:
//   - Verb[stem] + かけだ (split: verb + かけ)
//   - Verb[stem] + かける (split: verb + かける)
//   - Verb[stem] + かけの + Noun (can be split or compound)
// Tokenization:
//   - Split form: 食べ (verb) + かけ (動詞/非自立) - match as 2 tokens
//   - Compound form: 死にかけ (single verb with base="死にかける") - match as 1 token
// Note: We need two separate patterns for different tokenizations
pub fn kake() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match かけ/かける as non-independent verb (split form only)
    #[derive(Debug)]
    struct KakeSplitMatcher;
    impl Matcher for KakeSplitMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Split form: かけ or かける as non-independent verb
            (token.surface == "かけ" || token.surface == "かける")
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                && token.base_form == "かける"
        }
    }

    // Match preceding verb stem
    #[derive(Debug)]
    struct VerbStemMatcher;
    impl Matcher for VerbStemMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbStemMatcher)),
        TokenMatcher::Custom(Arc::new(KakeSplitMatcher)),
    ]
}

// かけ (compound): Compound verbs ending in かける (like 死にかける)
pub fn kake_compound() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct KakeCompoundMatcher;
    impl Matcher for KakeCompoundMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Compound form: verbs ending in かける (like 死にかける)
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.surface.ends_with("かけ")
                && token.base_form.ends_with("かける")
                && token.base_form != "かける" // Exclude standalone かける
        }
    }

    vec![TokenMatcher::Custom(Arc::new(KakeCompoundMatcher))]
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

// Pattern: がたい (difficult to do)
// Structures: Verb[stem/連用形] + がたい (+ です optional)
pub fn gatai() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct GataiMatcher;
    impl Matcher for GataiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "がたい"
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    vec![
        super::flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(GataiMatcher)),
    ]
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

// Pattern: せいで (because of / due to - negative result)
// Structures: Verb/Adjective/Noun + (な/の) + せい + で
pub fn seide() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct SeiMatcher;
    impl Matcher for SeiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "せい"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    #[derive(Debug)]
    struct DeParticleMatcher;
    impl Matcher for DeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    #[derive(Debug)]
    struct NoNaConnectorMatcher;
    impl Matcher for NoNaConnectorMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match の (particle) or な (auxiliary)
            (token.surface == "の" && token.pos.first().is_some_and(|pos| pos == "助詞"))
                || (token.surface == "な" && token.pos.first().is_some_and(|pos| pos == "助動詞"))
        }
    }

    vec![
        TokenMatcher::Any,  // Verb/Adjective/Noun before せい
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NoNaConnectorMatcher)))),
        TokenMatcher::Custom(Arc::new(SeiMatcher)),
        TokenMatcher::Custom(Arc::new(DeParticleMatcher)),
    ]
}

// Pattern: くせに (despite/even though)
// Structures: Verb/Adjective/Noun + (な/の) + くせ + に
pub fn kuseni() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KuseMatcher;
    impl Matcher for KuseMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "くせ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        TokenMatcher::Any,
        TokenMatcher::Custom(Arc::new(KuseMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
    ]
}

// Pattern: がち (tend to/prone to)
// Structures: Verb[stem] + がち / Noun + がち
pub fn gachi() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct GachiPrecedingMatcher;
    impl Matcher for GachiPrecedingMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Matches: Verb in 連用形 (stem) OR Noun
            if token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形") {
                return true;
            }
            // Match nouns (especially サ変接続 and 一般)
            if token.pos.first().is_some_and(|pos| pos == "名詞") {
                return true;
            }
            false
        }
    }

    #[derive(Debug)]
    struct GachiMatcher;
    impl Matcher for GachiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "がち"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                && token.pos.get(2).is_some_and(|pos| pos == "形容動詞語幹")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(GachiPrecedingMatcher)),
        TokenMatcher::Custom(Arc::new(GachiMatcher)),
    ]
}

// Pattern: ぎみ (sensation of / tendency)
// Structures: Verb[stem/連用形] + ぎみ / Noun + ぎみ
pub fn gimi() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct GimiPrecedingMatcher;
    impl Matcher for GimiPrecedingMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Matches: Verb in 連用形 (stem) OR Noun
            if token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形") {
                return true;
            }
            // Match nouns
            if token.pos.first().is_some_and(|pos| pos == "名詞") {
                return true;
            }
            false
        }
    }

    #[derive(Debug)]
    struct GimiMatcher;
    impl Matcher for GimiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ぎみ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                && token.pos.get(2).is_some_and(|pos| pos == "一般")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(GimiPrecedingMatcher)),
        TokenMatcher::Custom(Arc::new(GimiMatcher)),
    ]
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

// Pattern: 折角 (with effort/specially/long-awaited)
// Structures: せっかく + Phrase OR せっかく + の + Noun
pub fn sekkaku() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct SekkakuMatcher;
    impl Matcher for SekkakuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "せっかく" || token.surface == "折角")
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
        }
    }

    #[derive(Debug)]
    struct NoRentaika;
    impl Matcher for NoRentaika {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(SekkakuMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NoRentaika)))),
    ]
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

// Pattern: あるいは (or/alternatively)
// Structures: (Optional か) + あるいは
pub fn aruiwa() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KaParticleMatcher;
    impl Matcher for KaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "か"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞／並立助詞／終助詞")
        }
    }

    #[derive(Debug)]
    struct AruiwaMatcher;
    impl Matcher for AruiwaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "あるいは"
                && token.pos.first().is_some_and(|pos| pos == "接続詞")
        }
    }

    vec![
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(KaParticleMatcher)))),
        TokenMatcher::Custom(Arc::new(AruiwaMatcher)),
    ]
}

// Pattern: ～ずつ (each/per/at a time)
// Structures: Number + Counter + ずつ / 少し + ずつ / いくらか + ずつ
// Note: Matches the immediate token before ずつ (counter, noun, adverb, or particle)
pub fn zutsu() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct ZutsuPrecedingMatcher;
    impl Matcher for ZutsuPrecedingMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match the immediate token before ずつ:
            // 1. Numbers (名詞/数) - for patterns like 一人ずつ (where 一 might be separate)
            if token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "数") {
                return true;
            }
            // 2. Counters (名詞/接尾/助数詞) - for 人, 日, etc.
            if token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾") {
                return true;
            }
            // 3. General nouns (名詞/一般) - for 一つ, いくら, etc.
            if token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般") {
                return true;
            }
            // 4. 少し (副詞/助詞類接続)
            if token.surface == "少し"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "助詞類接続") {
                return true;
            }
            // 5. か particle (for いくらか pattern)
            if token.surface == "か"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞／並立助詞／終助詞") {
                return true;
            }
            false
        }
    }

    #[derive(Debug)]
    struct ZutsuMatcher;
    impl Matcher for ZutsuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ずつ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(ZutsuPrecedingMatcher)),
        TokenMatcher::Custom(Arc::new(ZutsuMatcher)),
    ]
}

// かなり + の + Noun: Considerable amount of
// Structures: かなり + の + Noun
pub fn kanari_no_noun() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    #[derive(Debug)]
    struct KanariNounMatcher;
    impl Matcher for KanariNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "かなり"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }

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
        TokenMatcher::Custom(Arc::new(KanariNounMatcher)),
        TokenMatcher::Custom(Arc::new(NoParticleMatcher)),
        super::noun_matcher(),
    ]
}

// Pattern: ～ても～なくても (whether or not)
// Structures: Verb［ても］(A) + Verb［なくても］(A)
//
// Note: This pattern requires the same verb to appear twice with different conjugations.
// We use a simpler approach: match Verb + て + も + Verb + なく + て + も
pub fn temo_nakutemo() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct TemoNakutemoTeDeParticleMatcher;
    impl super::Matcher for TemoNakutemoTeDeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match て or で as 接続助詞
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    #[derive(Debug)]
    struct TemoNakutemoMoParticleMatcher;
    impl super::Matcher for TemoNakutemoMoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match も as 係助詞
            token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    #[derive(Debug)]
    struct MizenVerbMatcher;
    impl super::Matcher for MizenVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match verb in 未然形 (or 未然ウ接続 for volitional)
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然形" || f == "未然ウ接続")
        }
    }

    #[derive(Debug)]
    struct NakuNaiMatcher;
    impl super::Matcher for NakuNaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match なく (ない in 連用テ接続)
            (token.surface == "なく" || token.surface == "なくっ")
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        // First verb (連用形 or 連用タ接続)
        super::flexible_verb_form(),
        // て/で
        TokenMatcher::Custom(Arc::new(TemoNakutemoTeDeParticleMatcher)),
        // も
        TokenMatcher::Custom(Arc::new(TemoNakutemoMoParticleMatcher)),
        // Second verb (未然形) - ideally same base as first, but we can't validate that easily
        TokenMatcher::Custom(Arc::new(MizenVerbMatcher)),
        // なく (ない)
        TokenMatcher::Custom(Arc::new(NakuNaiMatcher)),
        // て
        TokenMatcher::Custom(Arc::new(TemoNakutemoTeDeParticleMatcher)),
        // も
        TokenMatcher::Custom(Arc::new(TemoNakutemoMoParticleMatcher)),
    ]
}

// Pattern: しかない (polite form - しかありません)
// Structures: Verb + しかありません
pub fn shikanai_polite() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct ShikaParticleMatcher;
    impl super::Matcher for ShikaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match しか (係助詞)
            token.surface == "しか"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    #[derive(Debug)]
    struct AriVerbMatcher;
    impl super::Matcher for AriVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match あり (ある verb in 連用形)
            token.surface == "あり"
                && token.base_form == "ある"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形")
        }
    }

    #[derive(Debug)]
    struct MaseMatcher;
    impl super::Matcher for MaseMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match ませ (ます auxiliary in 未然形)
            token.surface == "ませ"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "未然形")
        }
    }

    #[derive(Debug)]
    struct NMatcher;
    impl super::Matcher for NMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match ん (auxiliary)
            token.surface == "ん"
                && token.base_form == "ん"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        // Verb (基本形)
        TokenMatcher::Verb {
            conjugation_form: Some("基本形"),
            base_form: None,
        },
        // しか (係助詞)
        TokenMatcher::Custom(Arc::new(ShikaParticleMatcher)),
        // あり (ある verb, 連用形)
        TokenMatcher::Custom(Arc::new(AriVerbMatcher)),
        // ませ (ます auxiliary, 未然形)
        TokenMatcher::Custom(Arc::new(MaseMatcher)),
        // ん (auxiliary)
        TokenMatcher::Custom(Arc::new(NMatcher)),
    ]
}
