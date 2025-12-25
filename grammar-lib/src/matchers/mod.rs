use crate::pattern_matcher::{MatchContext, TokenMatcher};
use crate::KagomeToken;
use std::sync::Arc;

pub mod n1;
pub mod n2;
pub mod n3;
pub mod n4;
pub mod n5;
pub mod nt;

/// Trait for token matching logic with lookahead support
pub trait Matcher: std::fmt::Debug + Send + Sync {
    /// Returns (matches, tokens_consumed)
    fn matches(&self, ctx: &MatchContext) -> (bool, usize);
}

/// Helper to convert a boolean check on a token to match result
#[inline]
pub fn check_token<F>(ctx: &MatchContext, f: F) -> (bool, usize)
where
    F: FnOnce(&KagomeToken) -> bool,
{
    match ctx.current() {
        Some(token) if f(token) => (true, 1),
        _ => (false, 0),
    }
}

// ========== Pattern Building Helpers ==========

pub fn concat(parts: Vec<Vec<TokenMatcher>>) -> Vec<TokenMatcher> {
    parts.into_iter().flatten().collect()
}

pub fn optional_seq(tokens: Vec<TokenMatcher>) -> Vec<TokenMatcher> {
    tokens
        .into_iter()
        .map(|t| TokenMatcher::Optional(Box::new(t)))
        .collect()
}

// ========== Basic Matchers ==========

pub fn surface(s: &'static str) -> TokenMatcher {
    TokenMatcher::Surface(s)
}

pub fn any() -> TokenMatcher {
    TokenMatcher::Any
}

pub fn optional(m: TokenMatcher) -> TokenMatcher {
    TokenMatcher::Optional(Box::new(m))
}

#[allow(dead_code)]
pub fn or(alts: Vec<TokenMatcher>) -> TokenMatcher {
    TokenMatcher::Or(alts)
}

pub fn wildcard(min: usize, max: usize, stop_conditions: Vec<TokenMatcher>) -> TokenMatcher {
    TokenMatcher::Wildcard { min, max, stop_conditions }
}

// ========== Verb Matchers ==========

/// Match any verb
pub fn verb() -> TokenMatcher {
    #[derive(Debug)]
    struct VerbMatcher;
    impl Matcher for VerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(t) if t.pos.first().is_some_and(|p| p == "動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(VerbMatcher))
}

/// Match verb with specific conjugation form (e.g., "連用形", "未然形")
pub fn verb_form(form: &'static str) -> TokenMatcher {
    #[derive(Debug)]
    struct VerbFormMatcher(&'static str);
    impl Matcher for VerbFormMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(t) if t.pos.first().is_some_and(|p| p == "動詞")
                    && t.features.get(5).is_some_and(|f| f == self.0) => (true, 1),
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(VerbFormMatcher(form)))
}

/// Match verb with specific base form (e.g., "する", "いる")
pub fn verb_base(base: &'static str) -> TokenMatcher {
    #[derive(Debug)]
    struct VerbBaseMatcher(&'static str);
    impl Matcher for VerbBaseMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(t) if t.pos.first().is_some_and(|p| p == "動詞")
                    && t.base_form == self.0 => (true, 1),
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(VerbBaseMatcher(base)))
}

/// Match verb with both base form and conjugation form
pub fn verb_base_form(base: &'static str, form: &'static str) -> TokenMatcher {
    #[derive(Debug)]
    struct VerbBaseFormMatcher(&'static str, &'static str);
    impl Matcher for VerbBaseFormMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(t) if t.pos.first().is_some_and(|p| p == "動詞")
                    && t.base_form == self.0
                    && t.features.get(5).is_some_and(|f| f == self.1) => (true, 1),
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(VerbBaseFormMatcher(base, form)))
}

/// Match 五段 verb with specific conjugation form
pub fn godan_verb(form: &'static str) -> TokenMatcher {
    #[derive(Debug)]
    struct GodanVerbMatcher(&'static str);
    impl Matcher for GodanVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(t) if t.pos.first().is_some_and(|p| p == "動詞")
                    && t.features.get(4).is_some_and(|f| f.contains("五段"))
                    && t.features.get(5).is_some_and(|f| f == self.0) => (true, 1),
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(GodanVerbMatcher(form)))
}

/// Match 一段 verb with specific conjugation form
pub fn ichidan_verb(form: &'static str) -> TokenMatcher {
    #[derive(Debug)]
    struct IchidanVerbMatcher(&'static str);
    impl Matcher for IchidanVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(t) if t.pos.first().is_some_and(|p| p == "動詞")
                    && t.features.get(4).is_some_and(|f| f.contains("一段"))
                    && t.features.get(5).is_some_and(|f| f == self.0) => (true, 1),
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(IchidanVerbMatcher(form)))
}

/// Match verb in 連用形 or 連用タ接続
pub fn flexible_verb_form() -> TokenMatcher {
    #[derive(Debug)]
    struct FlexibleVerbFormMatcher;
    impl Matcher for FlexibleVerbFormMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(t) if t.pos.first().is_some_and(|p| p == "動詞")
                    && t.features.get(5).is_some_and(|f| f == "連用形" || f == "連用タ接続") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(FlexibleVerbFormMatcher))
}

// ========== Adjective Matchers ==========

/// Match any adjective (い-adjective or な-adjective)
pub fn adjective() -> TokenMatcher {
    #[derive(Debug)]
    struct AdjectiveMatcher;
    impl Matcher for AdjectiveMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(t) => {
                    let is_i = t.pos.first().is_some_and(|p| p == "形容詞");
                    let is_na = t.pos.first().is_some_and(|p| p == "名詞")
                        && t.pos.get(1).is_some_and(|s| s == "形容動詞語幹");
                    if is_i || is_na { (true, 1) } else { (false, 0) }
                }
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(AdjectiveMatcher))
}

/// Match い-adjective
pub fn i_adjective() -> TokenMatcher {
    #[derive(Debug)]
    struct IAdjMatcher;
    impl Matcher for IAdjMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(t) if t.pos.first().is_some_and(|p| p == "形容詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(IAdjMatcher))
}

/// Match な-adjective (形容動詞語幹)
pub fn na_adjective() -> TokenMatcher {
    #[derive(Debug)]
    struct NaAdjMatcher;
    impl Matcher for NaAdjMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(t) if t.pos.first().is_some_and(|p| p == "名詞")
                    && t.pos.get(1).is_some_and(|s| s == "形容動詞語幹") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(NaAdjMatcher))
}

/// Match adjective with specific base form
pub fn adjective_base(base: &'static str) -> TokenMatcher {
    #[derive(Debug)]
    struct AdjBaseMatcher(&'static str);
    impl Matcher for AdjBaseMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(t) => {
                    let is_i = t.pos.first().is_some_and(|p| p == "形容詞");
                    let is_na = t.pos.first().is_some_and(|p| p == "名詞")
                        && t.pos.get(1).is_some_and(|s| s == "形容動詞語幹");
                    if (is_i || is_na) && t.base_form == self.0 { (true, 1) } else { (false, 0) }
                }
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(AdjBaseMatcher(base)))
}

// ========== Particle/Noun Matchers ==========

/// Match any particle (助詞)
pub fn particle() -> TokenMatcher {
    #[derive(Debug)]
    struct ParticleMatcher;
    impl Matcher for ParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(t) if t.pos.first().is_some_and(|p| p == "助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(ParticleMatcher))
}

/// Match particle with specific surface (but verify it's actually a particle)
pub fn particle_surface(s: &'static str) -> TokenMatcher {
    #[derive(Debug)]
    struct ParticleSurfaceMatcher(&'static str);
    impl Matcher for ParticleSurfaceMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(t) if t.surface == self.0 && t.pos.first().is_some_and(|p| p == "助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(ParticleSurfaceMatcher(s)))
}

/// Match any noun (名詞)
pub fn noun() -> TokenMatcher {
    #[derive(Debug)]
    struct NounMatcher;
    impl Matcher for NounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(t) if t.pos.first().is_some_and(|p| p == "名詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(NounMatcher))
}

// ========== Auxiliary Verb Matchers ==========

/// Match た or だ as past auxiliary
pub fn past_auxiliary() -> TokenMatcher {
    #[derive(Debug)]
    struct PastAuxMatcher;
    impl Matcher for PastAuxMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(t) if (t.surface == "た" || t.surface == "だ")
                    && (t.pos.first().is_some_and(|p| p == "助動詞")
                        || t.base_form == "た" || t.base_form == "だ") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(PastAuxMatcher))
}

/// Match ませ or ません
pub fn masen_form() -> TokenMatcher {
    #[derive(Debug)]
    struct MasenMatcher;
    impl Matcher for MasenMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(t) if (t.surface == "ませ" && t.base_form == "ます") || t.surface == "ません" => (true, 1),
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(MasenMatcher))
}

/// Match まし
pub fn mashi_form() -> TokenMatcher {
    #[derive(Debug)]
    struct MashiMatcher;
    impl Matcher for MashiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(t) if t.surface == "まし" && t.base_form == "ます" => (true, 1),
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(MashiMatcher))
}

/// Match でし
pub fn deshi_form() -> TokenMatcher {
    #[derive(Debug)]
    struct DeshiMatcher;
    impl Matcher for DeshiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(t) if t.surface == "でし" && t.base_form == "です" => (true, 1),
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(DeshiMatcher))
}

/// Match たかっ (past desiderative stem)
pub fn takatta_form_matcher() -> TokenMatcher {
    #[derive(Debug)]
    struct TakattaMatcher;
    impl Matcher for TakattaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(t) if t.surface == "たかっ" && t.base_form == "たい" => (true, 1),
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(TakattaMatcher))
}

/// Match いい or 良い
pub fn ii_form() -> TokenMatcher {
    #[derive(Debug)]
    struct IiMatcher;
    impl Matcher for IiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(t) if (t.surface == "いい" || t.surface == "良い")
                    && (t.base_form == "いい" || t.base_form == "良い") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(IiMatcher))
}

/// Match いけ, いけない, or いけません
pub fn ikenai_form() -> TokenMatcher {
    #[derive(Debug)]
    struct IkenaiMatcher;
    impl Matcher for IkenaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(t) if t.surface == "いけ" || t.surface == "いけない" || t.surface == "いけません" => (true, 1),
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(IkenaiMatcher))
}

// Legacy aliases for backwards compatibility during migration
pub fn particle_matcher() -> TokenMatcher { particle() }
pub fn noun_matcher() -> TokenMatcher { noun() }
