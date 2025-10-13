use super::TokenMatcherLogic;
use crate::analysis::morphology::KagomeToken;

// ========== Potential and Causative Forms ==========

/// Match られる or れる (potential/passive)
#[derive(Debug)]
pub struct RareruFormMatcher;

impl TokenMatcherLogic for RareruFormMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        (token.surface == "られる" || token.surface == "れる")
            && (token.base_form == "られる" || token.base_form == "れる")
    }
}

/// Match させる or せる (causative)
#[derive(Debug)]
pub struct CausativeFormMatcher;

impl TokenMatcherLogic for CausativeFormMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        (token.surface == "させる" || token.surface == "せる")
            && (token.base_form == "させる" || token.base_form == "せる")
    }
}

/// Match させ from させる (causative stem)
#[derive(Debug)]
pub struct SaseFormMatcher;

impl TokenMatcherLogic for SaseFormMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        token.surface == "させ" && token.base_form == "させる"
    }
}

// ========== Imperative and Conditional ==========

/// Match verb imperative forms (命令形, 命令ｒｏ, 命令ｉ, 命令ｅ)
#[derive(Debug)]
pub struct ImperativeFormMatcher;

impl TokenMatcherLogic for ImperativeFormMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        if token.pos.first().is_none_or(|pos| pos != "動詞") {
            false
        } else {
            let form = token.features.get(5);
            form.is_some_and(|f| f == "命令形" || f == "命令ｒｏ" || f == "命令ｉ" || f == "命令ｅ")
        }
    }
}

/// Match たら or だら (conditional)
#[derive(Debug)]
pub struct TaraFormMatcher;

impl TokenMatcherLogic for TaraFormMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        (token.surface == "たら" || token.surface == "だら")
            && (token.pos.first().is_some_and(|pos| pos == "助動詞")
                || token.base_form == "た"
                || token.base_form == "だ")
    }
}

// ========== Particles ==========

/// Match たり or だり particle (並立助詞)
#[derive(Debug)]
pub struct TariParticleMatcher;

impl TokenMatcherLogic for TariParticleMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        (token.surface == "たり" || token.surface == "だり")
            && token.pos.first().is_some_and(|pos| pos == "助詞")
            && token.pos.get(1).is_some_and(|pos| pos == "並立助詞")
    }
}

/// Match し particle (接続助詞)
#[derive(Debug)]
pub struct ShiParticleMatcher;

impl TokenMatcherLogic for ShiParticleMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        token.surface == "し"
            && token.pos.first().is_some_and(|pos| pos == "助詞")
            && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
    }
}

// ========== Negative Forms ==========

/// Match なかっ from ない (past negative)
#[derive(Debug)]
pub struct NakattaFormMatcher;

impl TokenMatcherLogic for NakattaFormMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        token.surface == "なかっ" && token.base_form == "ない"
    }
}

/// Match なけれ from ない (conditional negative)
#[derive(Debug)]
pub struct NakereFormMatcher;

impl TokenMatcherLogic for NakereFormMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        token.surface == "なけれ" && token.base_form == "ない"
    }
}

/// Match なく from ない (negative form)
#[derive(Debug)]
pub struct NakuFormMatcher;

impl TokenMatcherLogic for NakuFormMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        token.surface == "なく" && token.base_form == "ない"
    }
}

/// Match specific verb forms for must patterns (なら/いけ/だめ)
#[derive(Debug)]
pub struct MustPatternMatcher;

impl TokenMatcherLogic for MustPatternMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        token.surface == "なら" || token.surface == "いけ" || token.surface == "だめ"
    }
}

/// Natural verbs ending in れる (not potential forms)
const NATURAL_RERU_VERBS: &[&str] = &[
    "くれる",
    "入れる",
    "切れる",
    "晴れる",
    "慣れる",
    "汚れる",
    "疲れる",
    "腫れる",
    "暮れる",
    "揺れる",
    "枯れる",
    "破れる",
    "触れる",
];

/// Match 未然形 verbs that are NOT potential forms (excludes れる/られる base forms)
#[derive(Debug)]
pub struct NonPotentialMizenMatcher;

impl TokenMatcherLogic for NonPotentialMizenMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        // Must be verb in 未然形
        if token.pos.first().is_none_or(|pos| pos != "動詞") {
            return false;
        }

        let form = token.features.get(5);
        if form.is_none_or(|f| f != "未然形") {
            return false;
        }

        // Always exclude られる endings (always potential for ichidan verbs)
        if token.base_form.ends_with("られる") {
            return false;
        }

        // For れる endings: allow if in whitelist, exclude otherwise
        if token.base_form.ends_with("れる") {
            return NATURAL_RERU_VERBS.contains(&token.base_form.as_str());
        }

        // All other verbs are allowed
        true
    }
}

/// Match 未然形 verbs excluding なる (for shika_nai pattern)
#[derive(Debug)]
pub struct NonNaruMizenMatcher;

impl TokenMatcherLogic for NonNaruMizenMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        // Must be verb in 未然形
        if token.pos.first().is_none_or(|pos| pos != "動詞") {
            return false;
        }

        let form = token.features.get(5);
        if form.is_none_or(|f| f != "未然形") {
            return false;
        }

        // Exclude なる (become) - "しかならない" doesn't make semantic sense
        token.base_form != "なる"
    }
}

// ========== Desire and Past Forms ==========

/// Match よかっ or 良かっ (was good/glad)
#[derive(Debug)]
pub struct YokattaFormMatcher;

impl TokenMatcherLogic for YokattaFormMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        (token.surface == "よかっ" || token.surface == "良かっ")
            && (token.base_form == "よい" || token.base_form == "良い" || token.base_form == "いい")
    }
}

/// Match た from たい + がる pattern
#[derive(Debug)]
pub struct TagaruFormMatcher;

impl TokenMatcherLogic for TagaruFormMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        token.surface == "た"
            && token.base_form == "たい"
            && token.pos.first().is_some_and(|pos| pos == "助動詞")
    }
}

/// Match いい in といい context (can be verb いう or adjective いい)
#[derive(Debug)]
pub struct ToIiFormMatcher;

impl TokenMatcherLogic for ToIiFormMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        token.surface == "いい" && (token.base_form == "いう" || token.base_form == "いい")
            || (token.surface == "良い" && token.base_form == "良い")
    }
}

// ========== Appearance and Hearsay ==========

/// Match verb 連用形 or adjective stem (for そう appearance pattern)
#[derive(Debug)]
pub struct SouAppearanceStemMatcher;

impl TokenMatcherLogic for SouAppearanceStemMatcher {
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

/// Match plain/dictionary form verb or adjective (for そうだ hearsay pattern)
#[derive(Debug)]
pub struct SouHearsayStemMatcher;

impl TokenMatcherLogic for SouHearsayStemMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        if token.pos.first().is_some_and(|pos| pos == "動詞") {
            let form = token.features.get(5);
            form.is_some_and(|f| f == "基本形")
        } else if token.pos.first().is_some_and(|pos| pos == "形容詞") {
            let form = token.features.get(5);
            form.is_some_and(|f| f == "基本形")
        } else if token.pos.first().is_some_and(|pos| pos == "形容動詞") {
            true
        } else if token.pos.first().is_some_and(|pos| pos == "名詞") {
            token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹")
        } else {
            false
        }
    }
}
