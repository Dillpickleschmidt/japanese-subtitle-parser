use crate::pattern_matcher::TokenMatcher;
use std::sync::Arc;
use super::Matcher;

// って: Casual topic marker (replacing は)
// Structures: Sentence topic + って
pub fn tte() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for って as topic marker particle
    #[derive(Debug)]
    struct TteParticleMatcher;
    impl Matcher for TteParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match って as 助詞/格助詞/連語 (particle/case particle/compound)
            token.surface == "って"
                && token.base_form == "って"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "連語")
        }
    }

    vec![
        TokenMatcher::Any, // Sentence topic (usually a noun)
        TokenMatcher::Custom(Arc::new(TteParticleMatcher)),
    ]
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

// Pattern: なかなか～ない (hardly/not easily/far from)
// Structures: なかなか + Phrase + Verb[ない]
pub fn nakanaka_uff5e_nai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match なかなか as adverb
    #[derive(Debug)]
    struct NakanakaMatcher;
    impl super::Matcher for NakanakaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "なかなか"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
        }
    }

    // Match negative auxiliary ない or ません/ん
    #[derive(Debug)]
    struct NegativeMatcher;
    impl super::Matcher for NegativeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match ない auxiliary
            (token.surface == "ない" && token.pos.first().is_some_and(|pos| pos == "助動詞"))
                // Or ません (polite negative)
                || (token.surface == "ませ" && token.base_form == "ます" && token.pos.first().is_some_and(|pos| pos == "助動詞"))
                // Or ん (negative contraction after ませ)
                || (token.surface == "ん" && token.pos.first().is_some_and(|pos| pos == "助動詞"))
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NakanakaMatcher)),
        TokenMatcher::Wildcard {
            min: 0,
            max: 10, // Allow up to 10 tokens between なかなか and negative
            stop_conditions: vec![],
        },
        TokenMatcher::Custom(Arc::new(NegativeMatcher)),
    ]
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

// Pattern: そうだ (hearsay - I heard that)
// Structures:
//   - Verb + そうだ/そうです
//   - い-Adjective + そうだ/そうです
//   - Noun + だそうだ/だそうです
//   - な-Adjective + だそうだ/だそうです
pub fn souda() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match だ (助動詞, 基本形) - optional for Verb/i-Adj, required for Noun/na-Adj
    #[derive(Debug)]
    struct DaCopulaMatcher;
    impl Matcher for DaCopulaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "だ"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    // Match そう (名詞/特殊/助動詞語幹 OR 名詞/接尾/助動詞語幹)
    #[derive(Debug)]
    struct SouAuxiliaryMatcher;
    impl Matcher for SouAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "そう"
                && token.base_form == "そう"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "特殊" || pos == "接尾"))
        }
    }

    // Match だ or です (助動詞, 基本形)
    #[derive(Debug)]
    struct DaDesuMatcher;
    impl Matcher for DaDesuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "だ" || token.surface == "です")
                && (token.base_form == "だ" || token.base_form == "です")
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        TokenMatcher::Any, // Content word (Verb, i-Adj, Noun, na-Adj)
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(DaCopulaMatcher)))), // Optional だ (for Noun/na-Adj)
        TokenMatcher::Custom(Arc::new(SouAuxiliaryMatcher)), // そう
        TokenMatcher::Custom(Arc::new(DaDesuMatcher)), // だ or です
    ]
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

// てごらん: Please try to (honorific suggestion)
// Structures: Verb[て] + ごらん, Verb[て] + ごらんなさい
pub fn tegoran() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for て/で conjunction particle
    #[derive(Debug)]
    struct TeDeConjunctionMatcher;
    impl Matcher for TeDeConjunctionMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Matcher for ごらん as bound verb noun
    #[derive(Debug)]
    struct GoranMatcher;
    impl Matcher for GoranMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ごらん"
                && token.base_form == "ごらん"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "動詞非自立的")
        }
    }

    // Matcher for optional なさい (imperative form)
    #[derive(Debug)]
    struct NasaiMatcher;
    impl Matcher for NasaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "なさい"
                && token.base_form == "なさる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
        }
    }

    vec![
        super::flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(TeDeConjunctionMatcher)),
        TokenMatcher::Custom(Arc::new(GoranMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NasaiMatcher)))),
    ]
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
// Pattern: である (formal copula - formal equivalent of だ)
// Structures: Noun/な-Adjective + である / Noun/な-Adjective + であります
pub fn dearu() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct DeAuxiliaryMatcher;
    impl Matcher for DeAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "で"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形")
        }
    }

    #[derive(Debug)]
    struct AruAuxiliaryMatcher;
    impl Matcher for AruAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "ある" || token.surface == "あり")
                && token.base_form == "ある"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    #[derive(Debug)]
    struct MasuMatcher;
    impl Matcher for MasuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ます"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }

    vec![
        TokenMatcher::Any, // Noun or na-Adjective
        TokenMatcher::Custom(Arc::new(DeAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(AruAuxiliaryMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(MasuMatcher)))),
    ]
}

// Pattern: ところが
// Pattern: ところが (however, but unexpectedly)
// Structures: ところが (conjunction showing unexpected result)
pub fn tokoroga() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct TokorogaMatcher;
    impl super::Matcher for TokorogaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ところが"
                && token.base_form == "ところが"
                && token.pos.first().is_some_and(|pos| pos == "接続詞")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(TokorogaMatcher))]
}

// Pattern: ところで (by the way, incidentally)
// Structures: ところで (conjunction for introducing new topics)
pub fn tokorode() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct TokorodeMatcher;
    impl super::Matcher for TokorodeMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ところで"
                && token.base_form == "ところで"
                && token.pos.first().is_some_and(|pos| pos == "接続詞")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(TokorodeMatcher))]
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

// Pattern: では・それでは・じゃあ (conjunction/transition)
// Structures: それでは/では/じゃあ/じゃ + Phrase
pub fn deha_u30fb_soredeha_u30fb_jaa() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match それでは, では, じゃあ, or じゃ as conjunction
    #[derive(Debug)]
    struct DehaJaaMatcher;
    impl super::Matcher for DehaJaaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // All variants tokenize as 接続詞 (conjunction)
            token.pos.first().is_some_and(|pos| pos == "接続詞")
                && (token.surface == "それでは"
                    || token.surface == "では"
                    || token.surface == "じゃあ"
                    || token.surface == "じゃ")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(DehaJaaMatcher))]
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

// Pattern: ために (due to, because of, for the sake of)
// Structures: Verb + ため(に) / い-Adjective + ため(に) / な-Adjective + な + ため(に) / Noun + の + ため(に)
pub fn tameni() -> Vec<TokenMatcher> {
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
    struct NaAdjectiveStemMatcher;
    impl Matcher for NaAdjectiveStemMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match な-adjective stem: 名詞/形容動詞語幹
            token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹")
        }
    }

    #[derive(Debug)]
    struct NaCopulaMatcher;
    impl Matcher for NaCopulaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match な as auxiliary verb だ in 体言接続 form
            token.surface == "な"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "体言接続")
        }
    }

    #[derive(Debug)]
    struct NoRentaikaMatcher;
    impl Matcher for NoRentaikaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match の as 助詞/連体化
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
        }
    }

    // We need to handle 4 cases:
    // 1. Verb + ため(に)
    // 2. い-Adjective + ため(に)
    // 3. な-Adjective + な + ため(に)
    // 4. Noun + の + ため(に)
    //
    // The challenge is that な-adjective needs TWO tokens before ため (stem + な),
    // while the others need only ONE token.
    // We'll use a custom matcher that handles all cases:

    #[derive(Debug)]
    struct PreTameMatcherAny;
    impl Matcher for PreTameMatcherAny {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match Verb, い-Adjective, な-Adjective stem, Noun, or particles の/な
            let is_verb = token.pos.first().is_some_and(|pos| pos == "動詞");
            let is_i_adj = token.pos.first().is_some_and(|pos| pos == "形容詞");
            let is_na_adj_stem = token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹");
            let is_noun = token.pos.first().is_some_and(|pos| pos == "名詞");
            let is_na = token.surface == "な" && token.base_form == "だ";
            let is_no = token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化");

            is_verb || is_i_adj || is_na_adj_stem || is_noun || is_na || is_no
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(PreTameMatcherAny)), // Verb/Adj/Noun or particle
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            PreTameMatcherAny,
        )))), // Optional second token (for な-Adj or Noun+の)
        TokenMatcher::Custom(Arc::new(TameMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NiParticleMatcher,
        )))), // Optional に
    ]
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

// Pattern: 的 (like / -ish / -ly)
// Structures: Noun + 的 + に / Noun + 的 + な + Noun
pub fn teki() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match 的 as suffix (名詞/接尾/形容動詞語幹)
    #[derive(Debug)]
    struct TekiSuffixMatcher;
    impl super::Matcher for TekiSuffixMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "的"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
        }
    }

    // Match に as adverbializing particle OR な as copula
    #[derive(Debug)]
    struct NiOrNaMatcher;
    impl super::Matcher for NiOrNaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // に as adverbializing particle (助詞/副詞化)
            (token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副詞化"))
                // OR な as copula (助動詞) with 体言接続
                || (token.surface == "な"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.features.get(5).is_some_and(|f| f == "体言接続"))
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(TekiSuffixMatcher)),
        TokenMatcher::Custom(Arc::new(NiOrNaMatcher)),
    ]
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

// ちゃんと・きちんと: Properly/neatly (adverbs)
// Structures: ちゃんと/きちんと + Phrase
pub fn chanto_u30fb_kichinto() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for ちゃんと or きちんと as adverbs
    #[derive(Debug)]
    struct ChantoKichintoMatcher;
    impl Matcher for ChantoKichintoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "ちゃんと" || token.surface == "きちんと")
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(ChantoKichintoMatcher))]
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

// Pattern: だって (because/but/even)
// Structures: Noun + だって (particle "even") OR だって + Phrase (conjunction "because/but")
pub fn datte() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct DatteMatcher;
    impl Matcher for DatteMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Matches だって as either:
            // 1. 接続詞 (conjunction) - sentence beginning "because/but"
            // 2. 助詞/副助詞 (adverbial particle) - after noun "even"
            token.surface == "だって"
                && (token.pos.first().is_some_and(|pos| pos == "接続詞")
                    || (token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "副助詞")))
        }
    }

    // For particle form (Noun + だって), optionally match preceding noun to include it in range
    // For conjunction form (だって + Phrase), just match だって
    vec![
        TokenMatcher::Optional(Box::new(super::noun_matcher())),
        TokenMatcher::Custom(Arc::new(DatteMatcher)),
    ]
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

// さ - Casual よ: Sentence-ending particle (drawing attention with confidence)
// Structures: Phrase + さ
pub fn sa_casual_yo() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for さ as sentence-ending particle
    #[derive(Debug)]
    struct SaCasualYoMatcher;
    impl Matcher for SaCasualYoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match さ as 助詞/終助詞 (sentence-ending particle)
            token.surface == "さ"
                && token.base_form == "さ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "終助詞")
        }
    }

    vec![
        TokenMatcher::Any, // Phrase/content word before さ
        TokenMatcher::Custom(Arc::new(SaCasualYoMatcher)),
    ]
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
// てもかまわない: Doesn't matter / don't mind
// Structures: Verb[ても] + かまわない, Adj[ても] + かまわない, Noun/な-Adj + でも + かまわない
// Also polite forms: かまいません
pub fn temokamawanai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for ても or でも particle sequences
    // Can be either:
    //   - て (助詞/接続助詞) + も (助詞/係助詞) for verbs/i-adjectives
    //   - で (助詞/接続助詞) + も (助詞/係助詞) for verbs in negative て-form
    //   - で (助詞/格助詞) + も (助詞/係助詞) for nouns (when tokenized separately)
    //   - でも (助詞/副助詞) as single token for nouns/na-adjectives
    #[derive(Debug)]
    struct TemoOrDemoMatcher;
    impl super::Matcher for TemoOrDemoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match て/で as conjunction/case particle OR でも as single adverbial particle
            if token.surface == "でも"
                && token.base_form == "でも"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞") {
                // でも as single token (副助詞) - for nouns/na-adjectives
                return true;
            }

            // て or で as conjunction/case particle followed by も
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
                    || token.pos.get(1).is_some_and(|pos| pos == "格助詞"))
        }
    }

    // Matcher for も particle (only needed when て/で are separate tokens)
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl super::Matcher for MoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Matcher for かまう verb (かまわ, かまい forms)
    #[derive(Debug)]
    struct KamauVerbMatcher;
    impl super::Matcher for KamauVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "かまう"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && (token.surface == "かまわ" || token.surface == "かまい")
        }
    }

    // Matcher for negative forms: ない, ませ, ん
    #[derive(Debug)]
    struct NegativeFormMatcher;
    impl super::Matcher for NegativeFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "助動詞")
                && (token.base_form == "ない" || token.base_form == "ます" || token.base_form == "ん")
        }
    }

    vec![
        TokenMatcher::Any, // Content word before ても/でも
        TokenMatcher::Custom(Arc::new(TemoOrDemoMatcher)), // て/で or でも
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(MoParticleMatcher)))), // も (optional - not present for でも single token)
        TokenMatcher::Custom(Arc::new(KamauVerbMatcher)), // かまう verb
        TokenMatcher::Custom(Arc::new(NegativeFormMatcher)), // ない or ます
    ]
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

// さえ: even
// Structures:
//   - Noun + (Particle) + さえ
//   - Verb[stem] + さえ
//   - Verb[て] + さえ
//   - Verb + こと/の + さえ
pub fn sae() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match さえ as 助詞/係助詞
    #[derive(Debug)]
    struct SaeParticleMatcher;
    impl super::Matcher for SaeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "さえ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Match any case particle (で, に, を, etc.)
    #[derive(Debug)]
    struct CaseParticleMatcher;
    impl super::Matcher for CaseParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    vec![
        TokenMatcher::Any, // Content word (noun, verb, adjective, etc.)
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            CaseParticleMatcher,
        )))),
        TokenMatcher::Custom(Arc::new(SaeParticleMatcher)),
    ]
}

// Pattern: さえ〜ば (if only / as long as)
// Structures:
//   Verb[stem] + さえ + すれば
//   Verb[て] + さえ + いれば
//   Noun + さえ + Verb[ば]
//   い-Adjective[く] + さえ + あれば
//   Noun + さえ + い-Adjective[ば]
//   な-Adjective + (で) + さえ + あれば
pub fn sae_u301c_ba() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match さえ as 助詞/係助詞 (reuse from sae() pattern)
    #[derive(Debug)]
    struct SaeParticleMatcher;
    impl super::Matcher for SaeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "さえ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Match ば as 助詞/接続助詞
    #[derive(Debug)]
    struct BaParticleMatcher;
    impl super::Matcher for BaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ば"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    vec![
        TokenMatcher::Any, // Content word before さえ (verb, noun, adjective, etc.)
        TokenMatcher::Custom(Arc::new(SaeParticleMatcher)),
        TokenMatcher::Wildcard {
            min: 0,
            max: 6,
            stop_conditions: vec![],
        }, // 0-6 tokens between さえ and ば (e.g., すれ, いれ, あれ, verb in 仮定形, etc.)
        TokenMatcher::Custom(Arc::new(BaParticleMatcher)),
    ]
}

// Pattern: たものだ
// Pattern: たものだ (used to / would often)
// Structures: Verb[た] + ものだ, Verb[た] + ものです
pub fn tamonoda() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct MonoBoundNounMatcher;
    impl Matcher for MonoBoundNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "もの"
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
        }
    }

    vec![
        super::flexible_verb_form(),
        super::past_auxiliary(),
        TokenMatcher::Custom(Arc::new(MonoBoundNounMatcher)),
        TokenMatcher::Custom(Arc::new(DaDesuMatcher)),
    ]
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

// Pattern: むしろ (rather/instead)
// Structures: むしろ + (Preferred Choice) Phrase
pub fn mushiro() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct MushiroMatcher;
    impl Matcher for MushiroMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "むしろ"
                && token.base_form == "むしろ"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(MushiroMatcher))]
}

// Pattern: つまり (in other words/in short)
// Structures: Phrase (A)。つまり + (Summary) Phrase (B)
pub fn tsumari() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct TsumariMatcher;
    impl Matcher for TsumariMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match つまり as either noun (名詞/一般) or conjunction (接続詞)
            token.surface == "つまり"
                && token.base_form == "つまり"
                && (token.pos.first().is_some_and(|pos| pos == "名詞")
                    || token.pos.first().is_some_and(|pos| pos == "接続詞"))
        }
    }

    vec![TokenMatcher::Custom(Arc::new(TsumariMatcher))]
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
// Pattern: ではなくて・じゃなくて (negative copula te-form)
// Structures: (の) + で/じゃ + (は) + なく + (て)
pub fn dehanakute_u30fb_janakute() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match の as nominalizer (optional)
    #[derive(Debug)]
    struct NoNominalizerMatcher;
    impl super::Matcher for NoNominalizerMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Match で (助動詞, base: だ) or で (助詞/格助詞) or じゃ (助詞/副助詞)
    #[derive(Debug)]
    struct DeJaMatcher;
    impl super::Matcher for DeJaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            if token.surface == "で" {
                // Can be 助動詞 (base: だ) or 助詞/格助詞
                (token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.base_form == "だ")
                    || (token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞"))
            } else if token.surface == "じゃ" {
                // じゃ as 助詞/副助詞
                token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "副助詞")
            } else {
                false
            }
        }
    }

    // Match は as 助詞/係助詞 (optional)
    #[derive(Debug)]
    struct WaParticleMatcher;
    impl super::Matcher for WaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Match なく (形容詞 or 助動詞, base: ない)
    #[derive(Debug)]
    struct NakuMatcher;
    impl super::Matcher for NakuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "なく"
                && token.base_form == "ない"
                && (token.pos.first().is_some_and(|pos| pos == "形容詞")
                    || token.pos.first().is_some_and(|pos| pos == "助動詞"))
        }
    }

    // Match て as 助詞/接続助詞 (optional)
    #[derive(Debug)]
    struct TeConjunctionMatcher;
    impl super::Matcher for TeConjunctionMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    vec![
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NoNominalizerMatcher,
        )))),
        TokenMatcher::Custom(Arc::new(DeJaMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            WaParticleMatcher,
        )))),
        TokenMatcher::Custom(Arc::new(NakuMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            TeConjunctionMatcher,
        )))),
    ]
}

// Pattern: だけでなく(て)～も
pub fn dakedenaku_te_uff5e_mo() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: だけしか (only/nothing but)
// Structures: Noun + だけ + しか + ない
pub fn dakeshika() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match だけ as adverbial particle
    #[derive(Debug)]
    struct DakeParticleMatcher;
    impl super::Matcher for DakeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "だけ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞")
        }
    }

    // Match しか as bound particle
    #[derive(Debug)]
    struct ShikaParticleMatcher;
    impl super::Matcher for ShikaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "しか"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Match negative forms: ない (助動詞 or 形容詞), ません, ん
    #[derive(Debug)]
    struct NegativeFormMatcher;
    impl super::Matcher for NegativeFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // ない can be 助動詞 (auxiliary) or 形容詞 (i-adjective)
            if token.base_form == "ない" {
                token.pos.first().is_some_and(|pos| pos == "助動詞" || pos == "形容詞")
            } else {
                // ます or ん are always 助動詞
                token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && (token.base_form == "ます" || token.base_form == "ん")
            }
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(DakeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(ShikaParticleMatcher)),
        TokenMatcher::Wildcard {
            min: 0,
            max: 10,
            stop_conditions: vec![],
        },
        TokenMatcher::Custom(Arc::new(NegativeFormMatcher)),
    ]
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

// そうもない: very unlikely / doesn't even appear likely
// Structures:
//   - Verb[stem] + そう + も + ない
//   - Verb[stem] + そう + も + ありません
pub fn soumonai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match そう as 名詞/接尾/助動詞語幹
    #[derive(Debug)]
    struct SouAuxiliaryMatcher;
    impl super::Matcher for SouAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "そう"
                && token.base_form == "そう"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                && token.pos.get(2).is_some_and(|pos| pos == "助動詞語幹")
        }
    }

    // Match も as 助詞/係助詞
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl super::Matcher for MoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "も"
                && token.base_form == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Match ない (i-adjective form), ある, ます, or ん (for polite negative)
    #[derive(Debug)]
    struct NegativeOrAruMatcher;
    impl super::Matcher for NegativeOrAruMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // ない as i-adjective
            if token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
            {
                return true;
            }
            // ある or あり (for polite form)
            if (token.surface == "ある" || token.surface == "あり")
                && token.base_form == "ある"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
            {
                return true;
            }
            // ます (for polite negative)
            if token.surface == "ませ"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
            {
                return true;
            }
            // ん (for polite negative ending)
            if token.surface == "ん"
                && token.base_form == "ん"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
            {
                return true;
            }
            false
        }
    }

    vec![
        super::flexible_verb_form(), // Verb in 連用形 or 連用タ接続
        TokenMatcher::Custom(Arc::new(SouAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(MoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(NegativeOrAruMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NegativeOrAruMatcher,
        )))),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NegativeOrAruMatcher,
        )))),
    ]
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

// ついでに: While you're at it / on the occasion of
// Structures: Verb + ついでに, Noun + の + ついでに, Phrase。ついでに + Phrase
pub fn tsuideni() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct TsuideMatcher;
    impl Matcher for TsuideMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "ついで"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && (token.pos.get(1).is_some_and(|p| p == "一般")
                    || token.pos.get(1).is_some_and(|p| p == "非自立"))
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

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }

    vec![
        TokenMatcher::Any,                                  // Verb or Noun
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NoRentaikaMatcher,
        )))), // Optional の (for nouns)
        TokenMatcher::Custom(Arc::new(TsuideMatcher)),      // ついで (名詞)
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),  // に (格助詞)
    ]
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

// Pattern: 向き (suitable for / facing toward)
// Structures: Noun + 向き
pub fn muki() -> Vec<TokenMatcher> {
    // Matcher for 向き as noun suffix
    #[derive(Debug)]
    struct MukiSuffixMatcher;
    impl Matcher for MukiSuffixMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "向き"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
        }
    }

    vec![
        TokenMatcher::Any, // Noun before 向き
        TokenMatcher::Custom(Arc::new(MukiSuffixMatcher)),
    ]
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

// Pattern: たて (freshly/just finished)
// Structures: Verb[stem] + たて / Verb[stem] + たて + の + Noun
pub fn tate() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct TateSuffixMatcher;
    impl Matcher for TateSuffixMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match たて as 名詞/接尾/一般 (noun suffix)
            token.surface == "たて"
                && token.base_form == "たて"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
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
        TokenMatcher::Any, // Verb (連用形) or Noun
        TokenMatcher::Custom(Arc::new(TateSuffixMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            NoRentaikaMatcher,
        )))), // Optional の for noun modification
    ]
}

// Pattern: たとえ〜ても (even if)
// Structures: たとえ + Verb[ても] / たとえ + い-Adj[ても] / たとえ + な-Adj + でも / たとえ + Noun + でも
pub fn tatoetemo() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct TatoeAdverbMatcher;
    impl Matcher for TatoeAdverbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match たとえ as 副詞/一般 (adverb)
            token.surface == "たとえ"
                && token.base_form == "たとえ"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
        }
    }

    #[derive(Debug)]
    struct TeMoOrDemoStartMatcher;
    impl Matcher for TeMoOrDemoStartMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match て (connecting particle), で (case particle), OR でも (single token for na-adj)
            (token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞"))
                || (token.surface == "で"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞"))
                || (token.surface == "でも"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "副助詞"))
        }
    }

    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match も as 助詞/係助詞 (binding particle)
            token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
        }
    }

    // Match the pattern: たとえ + <content> + (て|で)+も OR でも
    // Note: でも can be either 1 token (副助詞 for na-adj) or 2 tokens (格助詞+係助詞 for nouns)
    vec![
        TokenMatcher::Custom(Arc::new(TatoeAdverbMatcher)),
        TokenMatcher::Wildcard {
            min: 1,
            max: 5,  // Increase to allow more content words between たとえ and ending
            stop_conditions: vec![],
        },
        TokenMatcher::Custom(Arc::new(TeMoOrDemoStartMatcher)), // Match て, で, or でも
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            MoParticleMatcher,
        )))), // Optional も (present after て/で, absent after でも as single token)
    ]
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

// Pattern: でよければ (if...is okay)
// Structures: Noun + で + よければ
// Tokenization: で (助詞/格助詞) + よけれ (形容詞/自立, base: よい, 仮定形) + ば (助詞/接続助詞)
pub fn deyokereba() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match で as case particle
    #[derive(Debug)]
    struct DeParticleMatcher;
    impl super::Matcher for DeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
        }
    }

    // Match よけれ (仮定形 of よい)
    #[derive(Debug)]
    struct YokereMatcher;
    impl super::Matcher for YokereMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "よけれ"
                && token.base_form == "よい"
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token
                    .features
                    .get(5)
                    .is_some_and(|form| form == "仮定形")
        }
    }

    // Match ば as conjunction particle
    #[derive(Debug)]
    struct BaParticleMatcher;
    impl super::Matcher for BaParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ば"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(DeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(YokereMatcher)),
        TokenMatcher::Custom(Arc::new(BaParticleMatcher)),
    ]
}

// Pattern: 次第
pub fn shidai() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// とおり: in that way / just like
// Structures:
//   - Verb + とおり
//   - Noun + どおり
//   - Noun + の + とおり
pub fn toori() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match とおり or どおり as noun suffix or bound noun
    #[derive(Debug)]
    struct TooriDooriMatcher;
    impl super::Matcher for TooriDooriMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "とおり" || token.surface == "どおり")
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "非自立")
                    || token.pos.get(1).is_some_and(|pos| pos == "接尾"))
        }
    }

    // Match の as 助詞/連体化
    #[derive(Debug)]
    struct NoRentaiMatcher;
    impl super::Matcher for NoRentaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化")
        }
    }

    vec![
        TokenMatcher::Any, // Content word (verb or noun)
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NoRentaiMatcher)))),
        TokenMatcher::Custom(Arc::new(TooriDooriMatcher)),
    ]
}

// Pattern: でもある
// Pattern: でもある (is also)
// Structures:
//   - Noun + でもある/でもあります (で can be separate or part of でも)
//   - い-Adjective[く] + もある/もあります
//   - な-Adjective + でもある/でもあります (で can be separate or part of でも)
pub fn demoaru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matches で (格助詞), も (係助詞), or でも (副助詞 single token)
    #[derive(Debug)]
    struct DemoOrMoMatcher;
    impl super::Matcher for DemoOrMoMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // で = 助詞/格助詞
            (token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞"))
                // OR も = 助詞/係助詞
                || (token.surface == "も"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "係助詞"))
                // OR でも = 助詞/副助詞 (single token)
                || (token.surface == "でも"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "副助詞"))
        }
    }

    // Matches ある verb (基本形 or 連用形)
    #[derive(Debug)]
    struct AruVerbMatcher;
    impl super::Matcher for AruVerbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.base_form == "ある"
                && (token.surface == "ある" || token.surface == "あり")
        }
    }

    // Matches ます
    #[derive(Debug)]
    struct MasuMatcher;
    impl super::Matcher for MasuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "ます"
        }
    }

    // Pattern: Any word + で/も/でも + optional も + ある + optional ます
    // This handles:
    //   - Noun + で + も + ある (4 tokens)
    //   - Noun + でも + ある (3 tokens, でも as single token)
    //   - い-Adj[く] + も + ある (3 tokens)
    //   - な-Adj + でも + ある (3 tokens, でも as single token)
    vec![
        TokenMatcher::Any,
        TokenMatcher::Custom(Arc::new(DemoOrMoMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(
            DemoOrMoMatcher,
        )))),
        TokenMatcher::Custom(Arc::new(AruVerbMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(MasuMatcher)))),
    ]
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
// Pattern: 同士 (fellow/mutually/together)
// Structures: Noun + 同士
pub fn doushi() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct DoushiMatcher;
    impl Matcher for DoushiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "同士"
                && token.base_form == "同士"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
        }
    }

    vec![
        super::noun_matcher(),
        TokenMatcher::Custom(Arc::new(DoushiMatcher)),
    ]
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

// Pattern: つい (accidentally/unconsciously/against one's better judgment)
// Structures: つい + Phrase
pub fn tsui() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct TsuiAdverbMatcher;
    impl Matcher for TsuiAdverbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match つい as adverb
            token.surface == "つい"
                && token.base_form == "つい"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(TsuiAdverbMatcher))]
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

// Pattern: っぽい (ish/like/tendency to)
// Structures: Verb[stem]/Adjective/Noun + っぽい
//
// Tokenization:
//   Split form: Content word + っぽい (形容詞/接尾) - detectable
//   Compound form: 白っぽい, 安っぽい (形容詞/自立) - not detectable (lexicalized)
//
// Note: っぽい creates い-Adjectives, so it conjugates (っぽい, っぽく, っぽかった, etc.)
pub fn ppoi() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct PpoiSuffixMatcher;
    impl Matcher for PpoiSuffixMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "っぽい"
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
        }
    }

    vec![
        TokenMatcher::Any,  // Verb, Noun, or な-Adjective
        TokenMatcher::Custom(Arc::new(PpoiSuffixMatcher)),
    ]
}

// Pattern: っぱなし (left in a state / left unchecked)
// Structures: Verb[stem] + っぱなし
//
// Tokenization:
//   Split form: Verb/Noun (any form) + っぱなし (名詞/接尾/一般) - detectable
//   Compound form: 開けっぱなし (名詞/一般) - not detectable as pattern
//
// Note: Sometimes verb stems are tokenized as nouns (勝ち, つけ), so we match both
pub fn ppanashi() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct PpanashiSuffixMatcher;
    impl Matcher for PpanashiSuffixMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "っぱなし"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
        }
    }

    vec![
        TokenMatcher::Any,  // Match verbs or nouns (verb stems can be nouns)
        TokenMatcher::Custom(Arc::new(PpanashiSuffixMatcher)),
    ]
}

// Pattern: わざわざ (to go out of one's way)
// Structures: わざわざ + Phrase
pub fn wazawaza() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct WazawazaMatcher;
    impl Matcher for WazawazaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "わざわざ"
                && token.base_form == "わざわざ"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(WazawazaMatcher))]
}

// Pattern: 一体 (on earth/in the world)
// Structures: いったい + Question Word + Phrase
pub fn ittai() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct IttaiMatcher;
    impl Matcher for IttaiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "いったい"
                && token.base_form == "いったい"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
        }
    }
    vec![TokenMatcher::Custom(Arc::new(IttaiMatcher))]
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

// っけ: Recall/confirmation particle (trying to remember or confirm information)
// Structures:
//   - Verb[た] + っけ
//   - Verb[る] + んだ + っけ
//   - い-Adjective[た] + っけ
//   - な-Adjective/Noun + だった + っけ
pub fn kke() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for っけ as sentence-ending particle
    #[derive(Debug)]
    struct KkeParticleMatcher;
    impl Matcher for KkeParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match っけ as 助詞/終助詞 (sentence-ending particle)
            token.surface == "っけ"
                && token.base_form == "っけ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "終助詞")
        }
    }

    // Matcher for た or だ in base form (基本形) before っけ
    #[derive(Debug)]
    struct TaDaAuxiliaryMatcher;
    impl Matcher for TaDaAuxiliaryMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match た (特殊・タ/基本形) or だ (特殊・ダ/基本形)
            (token.surface == "た" || token.surface == "だ")
                && (token.base_form == "た" || token.base_form == "だ")
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "基本形")
        }
    }

    vec![
        TokenMatcher::Any, // Content word before auxiliary (verb, adjective, noun, etc.)
        TokenMatcher::Custom(Arc::new(TaDaAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(KkeParticleMatcher)),
    ]
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
// Pattern: 左右する (influence/dictate/control)
// Structures: 左右 + する (all conjugations including passive される)
pub fn sayuusuru() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct SayuuMatcher;
    impl Matcher for SayuuMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "左右"
                && token.base_form == "左右"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "サ変接続")
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(SayuuMatcher)),
        TokenMatcher::Verb {
            conjugation_form: None,  // Any conjugation form
            base_form: Some("する"),
        },
    ]
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

// て初めて: Only after/not until
// Structures: Verb[て] + 初めて
pub fn te_hajimete() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for て/で as conjunction particle
    #[derive(Debug)]
    struct TeDeConjunctionMatcher;
    impl Matcher for TeDeConjunctionMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
        }
    }

    // Matcher for 初めて as adverb
    #[derive(Debug)]
    struct HajimeteAdverbMatcher;
    impl Matcher for HajimeteAdverbMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "初めて" || token.surface == "はじめて")
                && token.base_form == "初めて"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
        }
    }

    vec![
        super::flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(TeDeConjunctionMatcher)),
        TokenMatcher::Custom(Arc::new(HajimeteAdverbMatcher)),
    ]
}

// 中: During/throughout/in the middle of
// Structures: Noun + 中（ちゅう/じゅう）（に）
pub fn chuu() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for 中 or じゅう as suffix
    #[derive(Debug)]
    struct ChuuJuuSuffixMatcher;
    impl Matcher for ChuuJuuSuffixMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // Match 中 (kanji) or じゅう (hiragana) as noun suffix
            // 中 = 名詞/接尾/副詞可能 (ちゅう reading)
            // じゅう = 名詞/接尾/副詞可能 or 名詞/接尾/一般
            ((token.surface == "中" && token.base_form == "中")
                || (token.surface == "じゅう" && token.base_form == "じゅう"))
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
        }
    }

    // Matcher for optional に particle
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
        TokenMatcher::Any, // Noun before 中/じゅう
        TokenMatcher::Custom(Arc::new(ChuuJuuSuffixMatcher)),
        TokenMatcher::Optional(Box::new(TokenMatcher::Custom(Arc::new(NiParticleMatcher)))),
    ]
}

// Pattern: できれば・できたら (if possible)
// Structures: できれば/できたら + Phrase
pub fn dekireba_dekitara() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // できれ (仮定形 of できる) OR でき (連用形 of できる)
    #[derive(Debug)]
    struct DekirebaDekiraraMatcher;
    impl Matcher for DekirebaDekiraraMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.base_form == "できる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && (token.features.get(5).is_some_and(|form| form == "仮定形")
                    || token.features.get(5).is_some_and(|form| form == "連用形"))
        }
    }

    // ば (接続助詞) OR たら (た in 仮定形)
    #[derive(Debug)]
    struct BaTaraMatcher;
    impl Matcher for BaTaraMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            // ば (接続助詞)
            (token.surface == "ば"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞"))
            // OR たら (た in 仮定形)
            || (token.surface == "たら"
                && token.base_form == "た"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|form| form == "仮定形"))
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(DekirebaDekiraraMatcher)),
        TokenMatcher::Custom(Arc::new(BaTaraMatcher)),
    ]
}
