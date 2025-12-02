use crate::pattern_matcher::TokenMatcher;
use crate::KagomeToken;
use std::sync::Arc;

/// Trait for token matching logic
///
/// Implementations of this trait provide matching logic for tokens
pub trait Matcher: std::fmt::Debug + Send + Sync {
    /// Check if the token matches this matcher's criteria
    fn matches(&self, token: &KagomeToken) -> bool;
}

// ========== Helper Functions for Pattern Building ==========

/// Concatenate multiple token sequences into one
pub fn concat(parts: Vec<Vec<TokenMatcher>>) -> Vec<TokenMatcher> {
    parts.into_iter().flatten().collect()
}

/// Wrap all tokens in a sequence as Optional (can be skipped during matching)
pub fn optional(tokens: Vec<TokenMatcher>) -> Vec<TokenMatcher> {
    tokens
        .into_iter()
        .map(|t| TokenMatcher::Optional(Box::new(t)))
        .collect()
}

/// Wrap a single matcher as Optional
#[allow(dead_code)]
pub fn optional_single(matcher: TokenMatcher) -> TokenMatcher {
    TokenMatcher::Optional(Box::new(matcher))
}

// ========== Common Matcher Helpers ==========

/// Match verb in 連用形 or 連用タ接続 (for flexible patterns like past tense)
/// Used in: ta_form, te_construction, conditionals, etc.
pub fn flexible_verb_form() -> TokenMatcher {
    #[derive(Debug)]
    struct FlexibleVerbFormMatcher;
    impl Matcher for FlexibleVerbFormMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            if token.pos.first().is_none_or(|pos| pos != "動詞") {
                false
            } else {
                let form = token.features.get(5);
                form.is_some_and(|f| f == "連用形" || f == "連用タ接続")
            }
        }
    }
    TokenMatcher::Custom(Arc::new(FlexibleVerbFormMatcher))
}

/// Match た or だ as past auxiliary (助動詞)
/// Used in: ta_form, polite_past_ending, deshita, etc.
pub fn past_auxiliary() -> TokenMatcher {
    #[derive(Debug)]
    struct PastAuxiliaryMatcher;
    impl Matcher for PastAuxiliaryMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            (token.surface == "た" || token.surface == "だ")
                && (token.pos.first().is_some_and(|pos| pos == "助動詞")
                    || token.base_form == "た"
                    || token.base_form == "だ")
        }
    }
    TokenMatcher::Custom(Arc::new(PastAuxiliaryMatcher))
}

/// Match ませ or ません (polite negative form)
/// Used in: polite_negative
pub fn masen_form() -> TokenMatcher {
    #[derive(Debug)]
    struct MasenFormMatcher;
    impl Matcher for MasenFormMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            (token.surface == "ませ" && token.base_form == "ます") || token.surface == "ません"
        }
    }
    TokenMatcher::Custom(Arc::new(MasenFormMatcher))
}

/// Match まし (polite past stem)
/// Used in: polite_past_ending
pub fn mashi_form() -> TokenMatcher {
    #[derive(Debug)]
    struct MashiFormMatcher;
    impl Matcher for MashiFormMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "まし" && token.base_form == "ます"
        }
    }
    TokenMatcher::Custom(Arc::new(MashiFormMatcher))
}

/// Match でし (copula past stem)
/// Used in: deshita
pub fn deshi_form() -> TokenMatcher {
    #[derive(Debug)]
    struct DeshiFormMatcher;
    impl Matcher for DeshiFormMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "でし" && token.base_form == "です"
        }
    }
    TokenMatcher::Custom(Arc::new(DeshiFormMatcher))
}

/// Match たかっ (past tense of desiderative from たい)
/// Used in: takatta_form
pub fn takatta_form_matcher() -> TokenMatcher {
    #[derive(Debug)]
    struct TakattaFormMatcher;
    impl Matcher for TakattaFormMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "たかっ" && token.base_form == "たい"
        }
    }
    TokenMatcher::Custom(Arc::new(TakattaFormMatcher))
}

/// Match いい or 良い (good/okay)
/// Used in: te_mo_ii, hou_ga_ii
pub fn ii_form() -> TokenMatcher {
    #[derive(Debug)]
    struct IiFormMatcher;
    impl Matcher for IiFormMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            (token.surface == "いい" || token.surface == "良い")
                && (token.base_form == "いい" || token.base_form == "良い")
        }
    }
    TokenMatcher::Custom(Arc::new(IiFormMatcher))
}

/// Match いけ, いけない, or いけません (must not)
/// Used in: te_wa_ikenai, nakucha_ikenai
pub fn ikenai_form() -> TokenMatcher {
    #[derive(Debug)]
    struct IkenaiFormMatcher;
    impl Matcher for IkenaiFormMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.surface == "いけ" || token.surface == "いけない" || token.surface == "いけません"
        }
    }
    TokenMatcher::Custom(Arc::new(IkenaiFormMatcher))
}

/// Match any particle (助詞)
/// Used in: wildcard stop_conditions to prevent skipping over particle boundaries
pub fn particle_matcher() -> TokenMatcher {
    #[derive(Debug)]
    struct ParticleMatcher;
    impl Matcher for ParticleMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }
    TokenMatcher::Custom(Arc::new(ParticleMatcher))
}

/// Match any noun (名詞)
/// Used in: kiri_noun and other patterns requiring noun matching
pub fn noun_matcher() -> TokenMatcher {
    #[derive(Debug)]
    struct NounMatcher;
    impl Matcher for NounMatcher {
        fn matches(&self, token: &KagomeToken) -> bool {
            token.pos.first().is_some_and(|pos| pos == "名詞")
        }
    }
    TokenMatcher::Custom(Arc::new(NounMatcher))
}

pub mod n1;
pub mod n2;
pub mod n3;
pub mod n4;
pub mod n5;

#[cfg(test)]
mod tests {
    #[test]
    fn test_matcher_trait_exists() {
        // Verify trait is properly defined
        // Note: Structs are now scoped inside functions (lexically scoped)
        // so they cannot be referenced from here. This is intentional design.
    }
}
