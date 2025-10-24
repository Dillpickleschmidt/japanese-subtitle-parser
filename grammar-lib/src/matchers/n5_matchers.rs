use super::TokenMatcherLogic;
use crate::types::KagomeToken;

// ========== Desire/Want Forms ==========

/// Match たい as auxiliary verb or adjective
#[derive(Debug)]
pub struct TaiFormMatcher;

impl TokenMatcherLogic for TaiFormMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        token.surface == "たい"
            && (token.pos.first().is_some_and(|pos| pos == "形容詞")
                || token.pos.first().is_some_and(|pos| pos == "助動詞"))
    }
}

/// Match たく from たい (desiderative stem)
#[derive(Debug)]
pub struct TakuFormMatcher;

impl TokenMatcherLogic for TakuFormMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        token.surface == "たく" && token.base_form == "たい"
    }
}

/// Match たかっ from たい (past tense of desiderative)
#[derive(Debug)]
pub struct TakattaFormMatcher;

impl TokenMatcherLogic for TakattaFormMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        token.surface == "たかっ" && token.base_form == "たい"
    }
}

// ========== Te-Form Particles ==========

/// Match て or で (te-form, either surface or particle)
#[derive(Debug)]
pub struct TeDeFormMatcher;

impl TokenMatcherLogic for TeDeFormMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        token.surface == "て" || token.surface == "で"
    }
}

// ========== Polite Forms ==========

/// Match ましょう or ましょ (let's/shall we)
#[derive(Debug)]
pub struct MashouFormMatcher;

impl TokenMatcherLogic for MashouFormMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        token.surface == "ましょう" || (token.surface == "ましょ" && token.base_form == "ます")
    }
}

/// Match ませ or ません (polite negative)
#[derive(Debug)]
pub struct MasenFormMatcher;

impl TokenMatcherLogic for MasenFormMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        (token.surface == "ませ" && token.base_form == "ます") || token.surface == "ません"
    }
}

/// Match まし (polite past stem)
#[derive(Debug)]
pub struct MashiFormMatcher;

impl TokenMatcherLogic for MashiFormMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        token.surface == "まし" && token.base_form == "ます"
    }
}

/// Match でし (copula past stem)
#[derive(Debug)]
pub struct DeshiFormMatcher;

impl TokenMatcherLogic for DeshiFormMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        token.surface == "でし" && token.base_form == "です"
    }
}

// ========== Adjectives and Expressions ==========

/// Match いい or 良い (good/okay)
#[derive(Debug)]
pub struct IiFormMatcher;

impl TokenMatcherLogic for IiFormMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        (token.surface == "いい" || token.surface == "良い")
            && (token.base_form == "いい" || token.base_form == "良い")
    }
}

/// Match いけ, いけない, or いけません (must not)
#[derive(Debug)]
pub struct IkenaiFormMatcher;

impl TokenMatcherLogic for IkenaiFormMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        token.surface == "いけ" || token.surface == "いけない" || token.surface == "いけません"
    }
}

// ========== Verb Stems and Auxiliaries ==========

/// Match verb 連用形 or adjective stem (for すぎる pattern)
#[derive(Debug)]
pub struct SugiruStemMatcher;

impl TokenMatcherLogic for SugiruStemMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        if token.pos.first().is_some_and(|pos| pos == "動詞") {
            let form = token.features.get(5);
            form.is_some_and(|f| f == "連用形")
        } else if token.pos.first().is_some_and(|pos| pos == "形容詞") {
            let form = token.features.get(5);
            form.is_some_and(|f| f == "ガル接続")
        } else if token.pos.first().is_some_and(|pos| pos == "名詞") {
            token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹")
        } else {
            false
        }
    }
}

/// Match だ or た (past auxiliary)
#[derive(Debug)]
pub struct PastAuxiliaryMatcher;

impl TokenMatcherLogic for PastAuxiliaryMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        (token.surface == "た" || token.surface == "だ")
            && (token.pos.first().is_some_and(|pos| pos == "助動詞")
                || token.base_form == "た"
                || token.base_form == "だ")
    }
}

// ========== Modal Expressions ==========

/// Match でしょう/だろう or でしょ/だろ (probably)
#[derive(Debug)]
pub struct DeshouFormMatcher;

impl TokenMatcherLogic for DeshouFormMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        (token.surface == "でしょう" || token.surface == "だろう")
            || (token.surface == "でしょ" && token.base_form == "です")
            || (token.surface == "だろ" && token.base_form == "だ")
    }
}

/// Match words that can precede でしょう (verbs, adjectives, nouns, auxiliaries)
#[derive(Debug)]
pub struct DeshouPrecedingMatcher;

impl TokenMatcherLogic for DeshouPrecedingMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        token.pos.first().is_some_and(|pos| {
            pos == "動詞" || pos == "形容詞" || pos == "名詞" || pos == "助動詞"
        })
    }
}

/// Match ん or の before です (explanatory)
#[derive(Debug)]
pub struct NDesuFormMatcher;

impl TokenMatcherLogic for NDesuFormMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        (token.surface == "ん" || token.surface == "の")
            && token.pos.first().is_some_and(|pos| pos == "名詞")
    }
}
