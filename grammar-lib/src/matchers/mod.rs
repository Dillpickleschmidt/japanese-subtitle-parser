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
    TokenMatcher::Wildcard { min, max, stop_conditions, allow_commas: false }
}

pub fn wildcard_allow_commas(min: usize, max: usize, stop_conditions: Vec<TokenMatcher>) -> TokenMatcher {
    TokenMatcher::Wildcard { min, max, stop_conditions, allow_commas: true }
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

// ========== Ichidan/Godan Verb Matchers ==========

/// Match ichidan (一段) verb in 未然形
pub fn ichidan_mizen() -> TokenMatcher {
    #[derive(Debug)]
    struct IchidanMizenMatcher;
    impl Matcher for IchidanMizenMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(t) if t.pos.first().is_some_and(|p| p == "動詞")
                    && t.features.get(4).is_some_and(|f| f == "一段")
                    && t.features.get(5).is_some_and(|f| f == "未然形") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(IchidanMizenMatcher))
}

/// Match godan (五段) verb in 未然形
pub fn godan_mizen() -> TokenMatcher {
    #[derive(Debug)]
    struct GodanMizenMatcher;
    impl Matcher for GodanMizenMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(t) if t.pos.first().is_some_and(|p| p == "動詞")
                    && t.features.get(4).is_some_and(|f| f.starts_with("五段"))
                    && t.features.get(5).is_some_and(|f| f == "未然形") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(GodanMizenMatcher))
}

// ========== Verb Suffix Matchers ==========

/// Match られる or れる as suffix verb (for potential/passive)
pub fn rareru_suffix() -> TokenMatcher {
    #[derive(Debug)]
    struct RareruSuffixMatcher;
    impl Matcher for RareruSuffixMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(t) if (t.base_form == "られる" || t.base_form == "れる")
                    && t.pos.first().is_some_and(|p| p == "動詞")
                    && t.pos.get(1).is_some_and(|p| p == "接尾") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(RareruSuffixMatcher))
}

/// Match れる as suffix verb (godan passive)
pub fn reru_suffix() -> TokenMatcher {
    #[derive(Debug)]
    struct ReruSuffixMatcher;
    impl Matcher for ReruSuffixMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(t) if t.base_form == "れる"
                    && t.pos.first().is_some_and(|p| p == "動詞")
                    && t.pos.get(1).is_some_and(|p| p == "接尾") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(ReruSuffixMatcher))
}

/// Match える as suffix verb (godan potential)
pub fn eru_suffix() -> TokenMatcher {
    #[derive(Debug)]
    struct EruSuffixMatcher;
    impl Matcher for EruSuffixMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(t) if t.base_form == "える"
                    && t.pos.first().is_some_and(|p| p == "動詞")
                    && t.pos.get(1).is_some_and(|p| p == "接尾") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(EruSuffixMatcher))
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

// ========== Noun Matcher ==========

/// Match any noun (名詞)
pub fn noun() -> TokenMatcher {
    #[derive(Debug)]
    struct NounMatcher;
    impl Matcher for NounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(t) if t.pos.first().is_some_and(|p| p == "名詞")
                    // Exclude tokens that are ONLY whitespace/control/format characters
                    // (allows katakana loanwords with empty base_form like "ルームメイト")
                    && !t.surface.chars().all(|c| c.is_whitespace() || c.is_control() || ('\u{2000}'..='\u{206F}').contains(&c)) => (true, 1),
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

// ========== Particle Matchers ==========

/// Match specific particle surface with exact subtype checking
pub fn surface_particle(surface_text: &'static str, subtype: &'static str) -> TokenMatcher {
    #[derive(Debug)]
    struct SurfaceParticleMatcher(&'static str, &'static str);
    impl Matcher for SurfaceParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == self.0
                    && token.base_form == self.0
                    && token.pos.first().is_some_and(|p| p == "助詞")
                    && token.pos.get(1).is_some_and(|p| p == self.1) => (true, 1),
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(SurfaceParticleMatcher(surface_text, subtype)))
}

/// Match specific adjective surface with subtype checking
pub fn surface_adjective_subtype(surface_text: &'static str, subtype: &'static str) -> TokenMatcher {
    #[derive(Debug)]
    struct SurfaceAdjectiveSubtypeMatcher(&'static str, &'static str);
    impl Matcher for SurfaceAdjectiveSubtypeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == self.0
                    && token.pos.first().is_some_and(|p| p == "形容詞")
                    && token.pos.get(1).is_some_and(|p| p == self.1) => (true, 1),
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(SurfaceAdjectiveSubtypeMatcher(surface_text, subtype)))
}

// ========== Noun Subtype Matchers ==========

/// Match noun with specific subtype checking (e.g., 代名詞, 接尾)
pub fn noun_subtype(subtype: &'static str) -> TokenMatcher {
    #[derive(Debug)]
    struct NounSubtypeMatcher(&'static str);
    impl Matcher for NounSubtypeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|p| p == "名詞")
                    && token.pos.get(1).is_some_and(|p| p == self.0) => (true, 1),
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(NounSubtypeMatcher(subtype)))
}

/// Match specific noun suffix with exact surface checking
pub fn surface_noun_suffix(surface_text: &'static str) -> TokenMatcher {
    #[derive(Debug)]
    struct SurfaceNounSuffixMatcher(&'static str);
    impl Matcher for SurfaceNounSuffixMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == self.0
                    && token.base_form == self.0
                    && token.pos.first().is_some_and(|p| p == "名詞")
                    && token.pos.get(1).is_some_and(|p| p == "接尾") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(SurfaceNounSuffixMatcher(surface_text)))
}

// ========== Token Classification Helpers ==========

/// Check if a token refers to a person (pronoun, proper noun, or relationship noun).
pub fn is_person_reference(token: &KagomeToken) -> bool {
    if token.pos.first().is_some_and(|p| p == "名詞") {
        // Pronouns: あなた, 私, 彼, etc.
        if token.pos.get(1).is_some_and(|p| p == "代名詞") {
            return true;
        }
        // Proper nouns: person names, place names, etc.
        if token.pos.get(1).is_some_and(|p| p == "固有名詞") {
            return true;
        }
        // Common relationship nouns
        const PERSON_NOUNS: &[&str] = &[
            "友達", "友人", "先生", "先輩", "後輩", "家族", "親",
            "母", "父", "兄", "姉", "弟", "妹", "祖父", "祖母",
            "おばあちゃん", "おじいちゃん", "お母さん", "お父さん",
            "叔父", "叔母", "おじさん", "おばさん", "彼氏", "彼女",
            "恋人", "奥さん", "旦那", "主人", "嫁", "隣人", "知人",
        ];
        if PERSON_NOUNS.contains(&token.base_form.as_str()) {
            return true;
        }
    }
    false
}
