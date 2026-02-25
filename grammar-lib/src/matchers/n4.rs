use crate::pattern_matcher::{MatchContext, TokenMatcher};
use crate::matchers::{Matcher, noun, verb, adjective, check_token, verb_form, verb_base, surface, any, optional, wildcard,
    ichidan_mizen, godan_mizen, rareru_suffix, reru_suffix, eru_suffix, or, concat, flexible_verb_form, mashi_form, ii_form, past_auxiliary, surface_adjective_subtype, noun_subtype, surface_noun_suffix, surface_particle};
use std::sync::Arc;

// ========== Natural れる/める/える Verbs (not potential forms) ==========
// These verbs naturally end in れる/める/える and should not be treated as potential forms
// れる verbs: naturally end in れる (not potential forms)
// める verbs: includes 知れる (from かもしれない), 覚める, 目覚める, etc.
// える verbs: natural ichidan verbs like 覚える, 考える, 教える, etc.
const NATURAL_RERU_VERBS: &[&str] = &[
    "くれる", "入れる", "切れる", "晴れる", "慣れる", "汚れる",
    "疲れる", "腫れる", "暮れる", "揺れる", "枯れる", "破れる", "触れる", "溺れる",
    "覚める", "目覚める", "知れる", "しれる", "閉める", "決める", "止める", "始める",
    "攻める", "責める", "包める", "詰める", "進める", "勤める", "務める", "離れる", "分かれる",
    "崩れる",
];

// Natural ichidan verbs ending in える (not potential forms)
const NATURAL_ERU_VERBS: &[&str] = &[
    "覚える", "考える", "教える", "変える", "聞こえる", "見える", "答える", "消える",
    "超える", "越える", "耐える", "絶える", "冷える", "肥える", "捉える", "加える",
    "控える", "据える", "抱える", "備える", "燃える", "萌える", "栄える", "映える",
    "忘れる",  // to forget - common verb, not a potential form
    "助ける",  // to help/save - common verb, not a potential form
    "並べる",  // to arrange/line up - common verb, not a potential form
    // Natural verbs ending in てる (not potential forms)
    "捨てる", "見捨てる", "建てる", "立てる", "育てる", "当てる", "充てる", "宛てる",
    "打ち立てる", "企てる", "仕立てる", "持てる", "待てる",
];

// Natural ichidan verbs ending in せる (not potential forms)
// These are natural causative-form verbs or verbs that happen to end in せる
const NATURAL_SERU_VERBS: &[&str] = &[
    "知らせる", "見せる", "聞かせる", "着せる", "食べさせる", "寄せる", "合わせる",
    "乗せる", "載せる", "混ぜる", "見せる", "痩せる", "幸せる", "忘れさせる", "任せる",
];

// Natural ichidan verbs ending in める (not potential forms)
// These are natural dictionary verbs, not potential forms like 歩ける (potential of 歩く)
const NATURAL_MERU_VERBS: &[&str] = &[
    "覚める", "決める", "止める", "始める", "進める", "認める", "勤める", "努める",
    "集める", "閉める", "締める", "詰める", "極める", "定める", "求める", "責める",
    "攻める", "痛める", "固める", "高める", "強める", "弱める", "広める", "深める",
    "縮める", "早める", "速める", "遅らせる", "改める", "確める", "包める", "褒める",
    "苛める", "虐める", "悔やめる", "諦める", "慎める", "謹める", "占める", "染める",
];

// Natural godan verbs ending in す (not causative forms)
// These are natural dictionary verbs, not causative forms like 飲ます (causative of 飲む)
const NATURAL_SU_VERBS: &[&str] = &[
    "話す", "出す", "消す", "返す", "回す", "渡す", "流す", "貸す", "外す", "落とす",
    "起こす", "動かす", "探す", "直す", "隠す", "冷ます", "任す", "押す", "足す",
    "殺す", "刺す", "指す", "差す", "増す", "騙す", "壊す", "潰す", "写す", "移す",
    "恋す", "期す", "発す", "帰す", "亡くす", "無くす", "失くす", "無す", "引き戻す",
];

/// Match 未然形 verbs that are NOT potential forms (excludes れる/られる base forms)
fn non_potential_mizen() -> TokenMatcher {
    #[derive(Debug)]
    struct NonPotentialMizenMatcher;
    impl Matcher for NonPotentialMizenMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) => {
                    // Must be verb in 未然形
                    if !token.pos.first().is_some_and(|pos| pos == "動詞") {
                        return (false, 0);
                    }
                    if !token.features.get(5).is_some_and(|f| f == "未然形") {
                        return (false, 0);
                    }

                    // Always exclude られる endings (always potential for ichidan verbs)
                    if token.base_form.ends_with("られる") {
                        return (false, 0);
                    }

                    // For れる endings: allow if in whitelist, exclude otherwise
                    if token.base_form.ends_with("れる") {
                        return (NATURAL_RERU_VERBS.contains(&token.base_form.as_str()), 1);
                    }

                    // All other verbs are allowed
                    (true, 1)
                }
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(NonPotentialMizenMatcher))
}

// ========== Ichidan/Godan Specific Passive/Potential Patterns ==========

/// Passive form for ichidan verbs: Verb(一段・未然形) + られる
/// Example: 食べ + られる = 食べられる (to be eaten)
pub fn passive_ichidan() -> Vec<TokenMatcher> {
    vec![ichidan_mizen(), rareru_suffix()]
}

/// Passive form for godan verbs: Verb(五段・未然形) + れる
/// Example: 書か + れる = 書かれる (to be written)
pub fn passive_godan() -> Vec<TokenMatcher> {
    vec![godan_mizen(), reru_suffix()]
}

/// Potential form for godan verbs: Verb(五段・未然形) + える
/// Example: 書か + える = 書ける (can write)
pub fn potential_godan() -> Vec<TokenMatcher> {
    vec![godan_mizen(), eru_suffix()]
}

// Pattern: と (conditional - definite result)
// Structures: Verb + と / い-Adjective + と / な-Adjective + だ + と / Noun + だ + と
//
// Meaning: "if/when (A), then (B) will definitely happen"
// Note: Implies a definite/inevitable result, different from hypothetical conditionals
pub fn to() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct ToConditionalMatcher;
    impl Matcher for ToConditionalMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "と"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞") => {
                    // Exclude ないと - this is handled by the ないと pattern
                    if let Some(prev) = ctx.lookbehind(1) {
                        if prev.surface.ends_with("ない") {
                            return (false, 0);
                        }
                    }
                    (true, 1)
                },
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Matches verb, adjective, or noun/な-adj + だ before と
        TokenMatcher::Custom(Arc::new(ToConditionalMatcher)),
    ]
}

// Pattern: でも (even, or something, any-)
// Structures: Noun + でも (or Noun + で + も when split)
//
// Two tokenization patterns:
// 1. Noun + で (助詞/格助詞) + も (助詞/係助詞) - お茶でも, だれでも
// 2. Noun + でも (助詞/副助詞) - なんでも, どこでも, いつでも
pub fn demo() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct DemoOrDeMoMatcher;
    impl Matcher for DemoOrDeMoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            // Case 1: でも as single adverbial particle (副助詞) - e.g., なんでも, だれでも
            if let Some(token) = ctx.current() {
                if token.surface == "でも"
                    && token.pos.first().is_some_and(|p| p == "助詞")
                    && token.pos.get(1).is_some_and(|p| p == "副助詞")
                {
                    return (true, 1);
                }
            }

            // Case 2: で (格助詞) + も (係助詞) - e.g., お茶でも, お前でも
            // Must verify that も actually follows
            let _de_token = match ctx.current() {
                Some(token) if token.surface == "で"
                    && token.pos.first().is_some_and(|p| p == "助詞")
                    && token.pos.get(1).is_some_and(|p| p == "格助詞") => token,
                _ => return (false, 0),
            };

            // Verify も follows
            let _mo_token = match ctx.lookahead(1) {
                Some(token) if token.surface == "も"
                    && token.pos.first().is_some_and(|p| p == "助詞")
                    && token.pos.get(1).is_some_and(|p| p == "係助詞") => token,
                _ => return (false, 0),
            };

            (true, 2) // Matched で + も
        }
    }

    vec![
        super::noun(),
        TokenMatcher::Custom(Arc::new(DemoOrDeMoMatcher)),
    ]
}

// Pattern: やすい (easy to / prone to)
// Structure: Verb[stem/連用形] + やすい
//
// Tokenization: Verb (連用形) + やすい (形容詞/非自立)
// Meaning: "easy to (A)" or "prone to (A)" (with emotion verbs)
pub fn yasui() -> Vec<TokenMatcher> {
    vec![
        super::flexible_verb_form(),  // Verb in 連用形 or 連用タ接続
        surface_adjective_subtype("やすい", "非自立"),
    ]
}

// Pattern: にくい (difficult to)
// Structures: Verb[stem] + にくい/にくいです
pub fn nikui() -> Vec<TokenMatcher> {
    vec![
        super::flexible_verb_form(),  // Verb in 連用形 or 連用タ接続
        surface_adjective_subtype("にくい", "非自立"),
    ]
}

// Pattern: だんだん (gradually/steadily)
// Structures: だんだん + (と) + Phrase
pub fn dandan() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct DandanMatcher;
    impl Matcher for DandanMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "だんだん"
                && token.pos.first().is_some_and(|p| p == "副詞")
                && token.pos.get(1).is_some_and(|p| p == "助詞類接続")
            })
        }
    }

    #[derive(Debug)]
    struct ToParticleMatcher;
    impl Matcher for ToParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "と"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "副詞化")
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(DandanMatcher)),
        optional(TokenMatcher::Custom(Arc::new(ToParticleMatcher))),
    ]
}

// Pattern: どんどん (rapidly/quickly)
// Structures: どんどん + (と) + Phrase
pub fn dondon() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct DondonMatcher;
    impl Matcher for DondonMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "どんどん"
                && token.pos.first().is_some_and(|p| p == "副詞")
                && token.pos.get(1).is_some_and(|p| p == "助詞類接続")
            })
        }
    }

    #[derive(Debug)]
    struct ToParticleMatcher;
    impl Matcher for ToParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "と"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "副詞化")
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(DondonMatcher)),
        optional(TokenMatcher::Custom(Arc::new(ToParticleMatcher))),
    ]
}

// Pattern: ～ら (pluralizing suffix for pronouns)
// Structures: Pronoun + ら
//
// Note: Some pronouns like 彼ら tokenize as single tokens (名詞/代名詞/一般)
// but most like 私ら/お前ら split into Pronoun + ら(名詞/接尾).
// This matcher handles the split pattern (Pronoun + ら suffix).
pub fn uff5e_ra() -> Vec<TokenMatcher> {
    vec![
        noun_subtype("代名詞"),
        surface_noun_suffix("ら")
    ]
}

// Pattern: ていく (to go on to)
// Structures: Verb[て] + いく
pub fn teiku() -> Vec<TokenMatcher> {
    concat(vec![
        vec![flexible_verb_form()],
        vec![or(vec![surface("て"), surface("で")])],
        vec![verb_base("いく")],
    ])
}

// Pattern: てくる (to come to)
// Structures: Verb[て] + くる
pub fn tekuru() -> Vec<TokenMatcher> {
    concat(vec![
        vec![flexible_verb_form()],
        vec![or(vec![surface("て"), surface("で")])],
        vec![verb_base("くる")],
    ])
}

// Pattern: かた (how to/way of)
// Structures: Verb[stem] + 方（かた） / Noun(サ変) + の + 仕方（しかた）
pub fn kata() -> Vec<TokenMatcher> {
    // Pattern: Verb (連用形) + かた
    vec![
        verb_form("連用形"),
        or(vec![surface_noun_suffix("かた"), surface_noun_suffix("方")]),
    ]
}

// Pattern: かた (shikata variant - suru-verb + の + しかた)
// Structures: Noun(サ変) + の + 仕方（しかた）
pub fn kata_shikata() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for しかた/仕方 as noun
    #[derive(Debug)]
    struct ShikataMatcher;
    impl Matcher for ShikataMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.surface == "しかた" || token.surface == "仕方")
                && token.pos.first().is_some_and(|pos| pos == "名詞")
            })
        }
    }

    // Pattern: Suru-verb noun + の + しかた
    vec![
        noun_subtype("サ変接続"),
        surface("の"),
        TokenMatcher::Custom(Arc::new(ShikataMatcher)),
    ]
}

// Pattern: だけで (just by/with only)
// Structures: Verb + だけで / Noun + だけで
pub fn dakede() -> Vec<TokenMatcher> {
    vec![
        any(), // Verb or Noun
        surface_particle("だけ", "副助詞"),
        surface_particle("で", "格助詞"),
    ]
}

// Pattern: だが・ですが
// Pattern: だが・ですが (but, however - formal)
// Structures: だが + Phrase, ですが + Phrase
// Two tokenization patterns:
//   1. Single token: だが (接続詞) - at sentence start/after punctuation
//   2. Split tokens: だ/です (助動詞) + が (助詞/接続助詞) - mid-sentence
pub fn daga_u30fb_desuga() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Composite matcher that handles both forms:
    // 1. だが as single conjunction token
    // 2. だ/です + が as two tokens
    #[derive(Debug)]
    struct DagaDesugaCompositeMatcher;
    impl Matcher for DagaDesugaCompositeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            let Some(token) = ctx.current() else {
                return (false, 0);
            };

            // Pattern 1: だが as single conjunction token (sentence-start)
            if token.surface == "だが"
                && token.base_form == "だが"
                && token.pos.first().is_some_and(|pos| pos == "接続詞")
            {
                return (true, 1);
            }

            // Pattern 2: だ/です + が (must have both)
            if (token.surface == "だ" || token.surface == "です")
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
            {
                // Exclude だ with "特殊・タ" conjugation - this is past tense auxiliary
                // (e.g., 呼んだ = 呼ぶ + だ[past])
                if token.features.get(4).is_some_and(|f| f.starts_with("特殊・タ")) {
                    return (false, 0);
                }

                // Exclude だ that follows そう (auxiliary verb stem)
                // This だ is part of そうだ (appearance), not the copula
                if let Some(prev) = ctx.lookbehind(1) {
                    if prev.surface == "そう"
                        && prev.pos.first().is_some_and(|pos| pos == "名詞")
                        && prev.pos.get(1).is_some_and(|pos| pos == "接尾" || pos == "特殊") {
                        return (false, 0);
                    }
                }

                // Check if next token is が as conjunction particle
                if let Some(next_token) = ctx.lookahead(1) {
                    if next_token.surface == "が"
                        && next_token.pos.first().is_some_and(|pos| pos == "助詞")
                        && next_token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
                    {
                        return (true, 2); // Consume both tokens
                    }
                }
            }

            (false, 0)
        }
    }

    vec![TokenMatcher::Custom(Arc::new(DagaDesugaCompositeMatcher))]
}

// Pattern: なくて (negative て-form)
// Structures: Verb/Adjective + なくて
// Matches: Verb[未然形] + なく(助動詞) + て OR なく(形容詞) + て
pub fn nakute() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match なく as auxiliary or adjective (from ない)
    #[derive(Debug)]
    struct NakuMatcher;
    impl Matcher for NakuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "なく"
                && token.base_form == "ない"
                && (token.pos.first().is_some_and(|pos| pos == "助動詞")
                    || token.pos.first().is_some_and(|pos| pos == "形容詞"))
                && token.features.get(5).is_some_and(|f| f == "連用テ接続")
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NakuMatcher)),
        surface_particle("て", "接続助詞"),
    ]
}

// Pattern: ないで (without doing)
// Structure: Verb[未然形] + ない + で
// Note: Uses non_potential_mizen() to exclude potential forms (e.g., 食べられないで)
pub fn naide() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ない auxiliary with specific 連用デ接続 feature
    #[derive(Debug)]
    struct NaiAuxiliaryMatcher;
    impl Matcher for NaiAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "連用デ接続")
            })
        }
    }

    vec![
        non_potential_mizen(),
        TokenMatcher::Custom(Arc::new(NaiAuxiliaryMatcher)),
        surface_particle("で", "接続助詞"),
    ]
}

// Pattern: Verb［れる・られる］(Passive form - something happens to the subject)
// Structures: Verb[未然形] + れる/られる
// Note: For ichidan/godan-specific patterns, see passive_ichidan() and passive_godan()
pub fn verb_uff3b_reru_u30fb_rareru_uff3d() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for verbs in 未然形 (negative/passive stem)
    // This includes all verb types before passive auxiliary れる/られる
    #[derive(Debug)]
    struct PassiveStemMatcher;
    impl Matcher for PassiveStemMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                if !token.pos.first().is_some_and(|pos| pos == "動詞") {
                    return false;
                }
                // Match 未然形 (negative/passive stem) or 未然レル接続 (for する verbs)
                token.features.get(5).is_some_and(|form| {
                    form == "未然形" || form == "未然レル接続"
                })
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(PassiveStemMatcher)),
        rareru_suffix(),  // Uses shared helper for られる/れる suffix
        optional(mashi_form()),  // Uses shared helper for ます
    ]
}

// Pattern: 他動詞・自動詞
pub fn tadoushi_u30fb_jidoushi() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: なおす (to redo/fix) - split tokenization
// Structures: Verb[stem] + なおす/なおします
pub fn naosu() -> Vec<TokenMatcher> {
    vec![
        super::flexible_verb_form(),  // Verb in 連用形 or 連用タ接続
        verb_base("なおす"),
    ]
}

// Pattern: ということ (that means / you mean)
// Structures: Phrase + ということ, Phrase + ってこと
pub fn toiukoto() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct ToiuTteMatcher;
    impl Matcher for ToiuTteMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "という" || token.surface == "って")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "連語") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct KotoMatcher;
    impl Matcher for KotoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(ToiuTteMatcher)),
        TokenMatcher::Custom(Arc::new(KotoMatcher)),
    ]
}

// Pattern: とき (when / at the time)
// Structures:
//   - Verb + とき
//   - い-Adjective + とき
//   - な-Adjective + な + とき
//   - Noun + の + とき
//
// Meaning: "when", "at the time" - temporal noun indicating when something happens
pub fn toki() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct TokiMatcher;
    impl Matcher for TokiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "とき"
                && token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),
        TokenMatcher::Custom(Arc::new(TokiMatcher)),
    ]
}

// Pattern: まず (first of all / to start with)
// Structure: まず + Phrase
//
// Meaning: "first", "to begin with", "starting with" - indicates priority/sequence
pub fn mazu() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct MazuMatcher;
    impl Matcher for MazuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "まず"
                && token.pos.first().is_some_and(|p| p == "副詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![TokenMatcher::Custom(Arc::new(MazuMatcher))]
}

// Pattern: まで (even, to the extent)
// Structures: Noun + まで(も)
//
// This is the N4 "even" meaning of まで (adverbial particle).
// Tokenizes identically to N5 まで (until/to) - only semantic difference.
// Both N5 and N4 patterns will match the same text.
// Application should show both grammar explanations to user.
//
// Tokenization: Noun + まで (助詞/副助詞)
// Note: まで + も is matched as separate tokens, not a compound
pub fn made() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct MadeParticleMatcher;
    impl Matcher for MadeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "まで"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "副助詞")
            })
        }
    }

    vec![
        noun(),
        TokenMatcher::Custom(Arc::new(MadeParticleMatcher)),
    ]
}

// Pattern: までに (by/until - deadline)
// Structures: Verb/Noun + まで + に
//
// Tokenization: Content word + まで (助詞/副助詞) + に (助詞/格助詞)
// Meaning: "by" (deadline), NOT "until" (continuous action)
pub fn madeni() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct MadeParticleMatcher;
    impl Matcher for MadeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "まで"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "副助詞")
            })
        }
    }

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "に"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
            })
        }
    }

    vec![
        any(),  // Verb or Noun
        TokenMatcher::Custom(Arc::new(MadeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
    ]
}

// Pattern: また (again/also)
// Structures: また + Phrase
pub fn mata() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct MataMatcher;
    impl Matcher for MataMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "また"
                && token.base_form == "また"
                && token.pos.first().is_some_and(|pos| pos == "接続詞")
            })
        }
    }
    vec![TokenMatcher::Custom(Arc::new(MataMatcher))]
}

// Pattern: はじめる (start doing)
// Structure: Verb[stem/連用形] + はじめる
//
// Example tokenizations:
// - ためはじめます: ため(動詞/連用形) + はじめ(動詞/非自立/連用形) + ます
// - 歌いはじめる: 歌い(動詞/連用形) + はじめる(動詞/非自立/基本形)
// - ならいはじめた: ならい(動詞/連用形) + はじめ(動詞/非自立/連用形) + た
pub fn hajimeru() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct HajimeruMatcher;
    impl Matcher for HajimeruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.base_form == "はじめる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
            })
        }
    }

    vec![
        super::flexible_verb_form(), // Verb in 連用形 or 連用タ接続
        TokenMatcher::Custom(Arc::new(HajimeruMatcher)), // はじめる (auxiliary verb)
    ]
}

// Pattern: おわる (finish doing)
// Structure: Verb[stem/連用形] + 終わる
//
// Example tokenizations:
// - 払いおわる: 払い(動詞/連用形) + おわる(動詞/基本形)
// - 飲みおわって: 飲み(動詞/連用形) + おわっ(動詞/連用タ接続) + て
// - 読みおわりました: 読み(動詞/連用形) + おわり(動詞/連用形) + ました
pub fn owaru() -> Vec<TokenMatcher> {
    vec![
        super::flexible_verb_form(), // Verb in 連用形 or 連用タ接続
        verb_base("おわる"), // おわる (auxiliary verb)
    ]
}

// Pattern: ごろ (around/about time)
// Structures: Noun + ごろ, Noun + の + ころ
pub fn goro() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct GoroKoroMatcher;
    impl Matcher for GoroKoroMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.surface == "ごろ" || token.surface == "ころ")
                && (token.base_form == "ごろ" || token.base_form == "ころ")
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "接尾")
                    || token.pos.get(1).is_some_and(|pos| pos == "非自立"))
            })
        }
    }
    vec![TokenMatcher::Custom(Arc::new(GoroKoroMatcher))]
}

// Pattern: こと (nominalization)
// Structure: Verb + こと
//
// こと is a bound noun (名詞/非自立) used for nominalization.
// Converts verbs into noun phrases (e.g., "doing X", "the act of X")
//
// Examples:
// - することが嫌い (dislike faxing / the act of faxing)
// - 過ぎないことが大事 (not using too much is important)
// - なることをして (do things that cause...)
//
// Tokenization:
// - Verb (any form: basic, negative, past, etc.) OR auxiliary verb
// - こと (名詞/非自立/一般)
//
// Note: Matches verbs and auxiliary verbs only (not nouns like 勉強)
// For compound verbs like 勉強する, matches just "する + こと", not the whole phrase
pub fn koto() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct VerbOrAuxiliaryMatcher;
    impl Matcher for VerbOrAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match verbs and auxiliary verbs only
                // Exclude nouns (even if they're サ変接続)
                let is_verb = token.pos.first().is_some_and(|pos| pos == "動詞");
                let is_auxiliary = token.pos.first().is_some_and(|pos| pos == "助動詞");
                let is_noun = token.pos.first().is_some_and(|pos| pos == "名詞");

                // Exclude copula だ (base_form == "だ") - this is used with adjectives, not verb nominalization
                // Examples: バカなこと (na-adjective + copula + noun), not a verb nominalization
                let is_copula = token.base_form == "だ";

                (is_verb || is_auxiliary) && !is_noun && !is_copula
            })
        }
    }

    #[derive(Debug)]
    struct KotoMatcher;
    impl Matcher for KotoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "こと"
                && token.base_form == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbOrAuxiliaryMatcher)), // Verb or auxiliary verb
        TokenMatcher::Custom(Arc::new(KotoMatcher)), // こと (bound noun)
    ]
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
    // Match あと noun (名詞/一般)
    #[derive(Debug)]
    struct AtoMatcher;
    impl Matcher for AtoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "あと"
                && token.base_form == "あと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般")
            })
        }
    }

    // Match で particle or copula after あと
    // Can be:
    // - 助詞/格助詞/一般 (particle: "with/at")
    // - 助動詞 (copula, base=だ: "is")
    #[derive(Debug)]
    struct DeAfterAtoMatcher;
    impl Matcher for DeAfterAtoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
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
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(AtoMatcher)),
        TokenMatcher::Custom(Arc::new(DeAfterAtoMatcher)),
    ]
}

// Pattern: ていた (was doing / past progressive)
// Structures: Verb[ている] + た / Verb[ている] + ました
pub fn teita() -> Vec<TokenMatcher> {
    // Match て or で particle
    #[derive(Debug)]
    struct TeDeFormMatcher;
    impl Matcher for TeDeFormMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|p| p == "助詞")
            })
        }
    }

    // Match いる in 連用形 (い)
    #[derive(Debug)]
    struct IruRenyoukeiMatcher;
    impl Matcher for IruRenyoukeiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.base_form == "いる"
                && token.pos.first().is_some_and(|p| p == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形")
            })
        }
    }

    concat(vec![
        vec![super::flexible_verb_form()],
        vec![TokenMatcher::Custom(Arc::new(TeDeFormMatcher))],
        vec![TokenMatcher::Custom(Arc::new(IruRenyoukeiMatcher))],
        vec![optional(super::mashi_form())], // Optional まし for polite form
        vec![super::past_auxiliary()], // た
    ])
}

// Pattern: に (Frequency)
// Pattern: に (Frequency) - per/every
// Structures: Timeframe + に + Number of Times
pub fn ni_frequency() -> Vec<TokenMatcher> {
    // Matches number (名詞/数)
    #[derive(Debug)]
    struct NumberMatcher;
    impl Matcher for NumberMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "数")
            })
        }
    }

    // Matches counter suffix (名詞/接尾/助数詞)
    #[derive(Debug)]
    struct CounterSuffixMatcher;
    impl Matcher for CounterSuffixMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                && token.pos.get(2).is_some_and(|pos| pos == "助数詞")
            })
        }
    }

    // Matches に particle (助詞/格助詞/一般)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "一般")
            })
        }
    }

    vec![
        // Timeframe: Number + Counter
        TokenMatcher::Custom(Arc::new(NumberMatcher)),
        TokenMatcher::Custom(Arc::new(CounterSuffixMatcher)),
        // に particle
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        // Frequency: Number + Counter
        TokenMatcher::Custom(Arc::new(NumberMatcher)),
        TokenMatcher::Custom(Arc::new(CounterSuffixMatcher)),
    ]
}

// Pattern: とうとう (finally/at last)
// Structures: とうとう + Phrase
pub fn toutou() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct ToutouMatcher;
    impl Matcher for ToutouMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "とうとう" && token.pos.first().is_some_and(|pos| pos == "副詞")
            })
        }
    }
    vec![TokenMatcher::Custom(Arc::new(ToutouMatcher))]
}

// Pattern: より (than/more than)
// Structures: Noun + より, Verb + より
pub fn yori() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct YoriMatcher;
    impl Matcher for YoriMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "より"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            })
        }
    }
    vec![TokenMatcher::Custom(Arc::new(YoriMatcher))]
}

// Pattern: ごとに (every/each time)
// Structures: Verb/Noun + ごと + に
pub fn gotoni() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct GotoMatcher;
    impl Matcher for GotoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ごと"
                && token.base_form == "ごと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
            })
        }
    }

    vec![
        any(), // Verb or Noun
        TokenMatcher::Custom(Arc::new(GotoMatcher)), // ごと (名詞)
        surface_particle("に", "格助詞"), // に (助詞/格助詞)
    ]
}

// Pattern: なるべく (as much as possible)
// Structure: なるべく + Phrase
pub fn narubeku() -> Vec<TokenMatcher> {
    // Match なるべく adverb
    #[derive(Debug)]
    struct NarubekuMatcher;
    impl Matcher for NarubekuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "なるべく"
                && token.base_form == "なるべく"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
            })
        }
    }

    vec![TokenMatcher::Custom(Arc::new(NarubekuMatcher))]
}

// Pattern: るところだ (about to do, on the verge of)
// Structures: Verb[る] + ところ + だ/です
pub fn rutokoroda() -> Vec<TokenMatcher> {
    // Match ところ as dependent noun (非自立) with 副詞可能
    #[derive(Debug)]
    struct TokoroMatcher;
    impl Matcher for TokoroMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ところ"
                && token.base_form == "ところ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
            })
        }
    }

    // Match だ or です (auxiliary verbs)
    #[derive(Debug)]
    struct DaDesuMatcher;
    impl Matcher for DaDesuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.surface == "だ" || token.surface == "です")
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && (token.features.get(4).is_some_and(|f| f == "特殊・ダ")
                    || token.features.get(4).is_some_and(|f| f == "特殊・デス"))
            })
        }
    }

    vec![
        verb_form("基本形"),
        TokenMatcher::Custom(Arc::new(TokoroMatcher)),
        TokenMatcher::Custom(Arc::new(DaDesuMatcher)),
    ]
}

// Pattern: のに (despite)
// Structures: Verb/い-Adj + のに, Noun/な-Adj + な + のに
pub fn noni() -> Vec<TokenMatcher> {
    vec![
        any(),
        optional(TokenMatcher::Custom(Arc::new({
            #[derive(Debug)]
            struct NaAuxiliary;
            impl Matcher for NaAuxiliary {
                fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
                    check_token(ctx, |token| {
                        token.surface == "な"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞")
                        && token.features.get(4).is_some_and(|f| f == "特殊・ダ")
                        && token.features.get(5).is_some_and(|f| f == "体言接続")
                    })
                }
            }
            NaAuxiliary
        }))),
        TokenMatcher::Custom(Arc::new({
            #[derive(Debug)]
            struct NoniParticle;
            impl Matcher for NoniParticle {
                fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
                    check_token(ctx, |token| {
                        token.surface == "のに"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
                    })
                }
            }
            NoniParticle
        })),
    ]
}

// Pattern: とおもう (I think that)
// Structures: Verb/Adj + とおもう, Noun/な-Adj + だ + とおもう
pub fn toomou() -> Vec<TokenMatcher> {
    vec![
        // Match と quotation particle (助詞/格助詞/引用)
        TokenMatcher::Custom(Arc::new({
            #[derive(Debug)]
            struct ToQuotation;
            impl Matcher for ToQuotation {
                fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
                    check_token(ctx, |token| {
                        token.surface == "と"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                        && token.pos.get(2).is_some_and(|pos| pos == "引用")
                    })
                }
            }
            ToQuotation
        })),
        // Match おもう/思う verb
        or(vec![
            verb_base("おもう"),
            verb_base("思う"),
        ]),
    ]
}

// Pattern: など (such as, and so on)
// Structures: Noun + など
pub fn nado() -> Vec<TokenMatcher> {
    vec![
        noun(),
        // Match など adverbial particle (助詞/副助詞)
        TokenMatcher::Custom(Arc::new({
            #[derive(Debug)]
            struct Nado;
            impl Matcher for Nado {
                fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
                    check_token(ctx, |token| {
                        token.surface == "など"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "副助詞")
                    })
                }
            }
            Nado
        })),
    ]
}

// Pattern: みたい
// Pattern: みたい (seems like/looks like - resemblance)
// Structures: Verb/Adj/Noun + みたい + だ/です
pub fn mitai() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    use super::Matcher;

    // Match みたい as na-adjective stem (dependent noun)
    #[derive(Debug)]
    struct MitaiMatcher;
    impl Matcher for MitaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "みたい"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                && token.pos.get(2).is_some_and(|pos| pos == "形容動詞語幹")
            })
        }
    }

    // Match だ or です (auxiliary verbs)
    #[derive(Debug)]
    struct DaDesuMatcher;
    impl Matcher for DaDesuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.pos.first().is_some_and(|pos| pos == "助動詞")
                && (token.base_form == "だ" || token.base_form == "です")
            })
        }
    }

    vec![
        any(), // Can follow any word (verb, adjective, noun)
        TokenMatcher::Custom(Arc::new(MitaiMatcher)),
        optional(TokenMatcher::Custom(Arc::new(DaDesuMatcher))),
    ]
}

// Pattern: そう (looks like/seems like - appearance-based conjecture)
// Structures: Verb[stem] + そう、い-Adj[stem] + そう、な-Adj + そう
pub fn sou() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match さ suffix (for negative forms: なさそう)
    #[derive(Debug)]
    struct SaSuffixMatcher;
    impl Matcher for SaSuffixMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "さ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
            })
        }
    }

    // Match verb/adjective stem OR な in negative forms
    #[derive(Debug)]
    struct StemOrNaiMatcher;
    impl Matcher for StemOrNaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Verb stem (連用形)
            (token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形"))
            ||
            // い-Adjective stem (ガル接続)
            (token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "ガル接続"))
            ||
            // な-Adjective (形容動詞語幹)
            (token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹"))
            ||
            // ない auxiliary in ガル接続 (for negative forms)
            (token.surface == "な"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "ガル接続"))
            })
        }
    }

    // Match そう auxiliary (名詞/接尾/助動詞語幹)
    #[derive(Debug)]
    struct SouAuxiliaryMatcher;
    impl Matcher for SouAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "そう"
                && token.base_form == "そう"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                && token.pos.get(2).is_some_and(|pos| pos == "助動詞語幹")
            })
        }
    }

    // Match だ or です copula
    #[derive(Debug)]
    struct DaDesuCopulaMatcher;
    impl Matcher for DaDesuCopulaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.surface == "だ" || token.surface == "です")
                && (token.base_form == "だ" || token.base_form == "です")
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(StemOrNaiMatcher)), // Verb/Adj stem OR な (negative)
        optional(TokenMatcher::Custom(Arc::new(SaSuffixMatcher))), // Optional さ (for なさそう)
        TokenMatcher::Custom(Arc::new(SouAuxiliaryMatcher)), // そう
        optional(TokenMatcher::Custom(Arc::new(DaDesuCopulaMatcher))), // Optional だ/です
    ]
}

// Pattern: さ (degree/amount suffix)
// Structures: い-Adjective[い] + さ / な-Adjective + さ
pub fn sa() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match さ suffix (名詞/接尾/特殊)
    #[derive(Debug)]
    struct SaSuffixMatcher;
    impl Matcher for SaSuffixMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "さ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                && token.pos.get(2).is_some_and(|pos| pos == "特殊")
            })
        }
    }

    // Match either い-adjective stem or な-adjective stem
    #[derive(Debug)]
    struct AdjectiveStemMatcher;
    impl Matcher for AdjectiveStemMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.pos.first().is_some_and(|pos| pos == "形容詞")
                    && token.features.get(5).is_some_and(|f| f == "ガル接続"))
                ||
                (token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹"))
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(AdjectiveStemMatcher)),
        TokenMatcher::Custom(Arc::new(SaSuffixMatcher)),
    ]
}

// Pattern: とか～とか (things like... and...)
// Structures: Verb/Adj/Noun + とか (+ Verb/Adj/Noun + とか)*
// Listing particle for non-exhaustive examples
pub fn toka_uff5e_toka() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for content words (nouns, verbs, adjectives) - excludes punctuation
    #[derive(Debug)]
    struct ContentWordMatcher;
    impl Matcher for ContentWordMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) => {
                    // Reject tokens that are clearly punctuation (empty surface, single symbol chars, etc.)
                    if token.surface.is_empty() || token.surface.chars().all(|c| !c.is_alphanumeric()) {
                        return (false, 0);
                    }
                    // Accept nouns, verbs, adjectives
                    let is_content = if let Some(pos) = token.pos.first() {
                        matches!(pos.as_str(), "名詞" | "動詞" | "形容詞")
                    } else {
                        false
                    };
                    if is_content {
                        (true, 1)
                    } else {
                        (false, 0)
                    }
                }
                None => (false, 0),
            }
        }
    }

    // Matcher for とか as 助詞/並立助詞 (coordinating particle)
    #[derive(Debug)]
    struct TokaParticleMatcher;
    impl Matcher for TokaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "とか"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "並立助詞")
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(ContentWordMatcher)), // Content word (verb, noun, adjective)
        TokenMatcher::Custom(Arc::new(TokaParticleMatcher)), // とか particle
    ]
}

// Pattern: そういう
// Pattern: そういう (like that, that kind of)
// Structures: こういう/そういう/どういう (single token) OR ああ + いう (two tokens)
pub fn souiu() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Custom matcher that handles both single-token and two-token patterns
    #[derive(Debug)]
    struct SouiuPatternMatcher;
    impl Matcher for SouiuPatternMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            let Some(token) = ctx.current() else {
                return (false, 0);
            };

            // Pattern 1: Single-token forms (こういう/そういう/どういう)
            if token.pos.first().is_some_and(|pos| pos == "連体詞")
                && (token.base_form == "こういう"
                    || token.base_form == "そういう"
                    || token.base_form == "どういう")
            {
                return (true, 1);
            }

            // Pattern 2: Two-token form (ああ + いう)
            // Only match "ああ" if it's followed by "いう"
            if token.surface == "ああ"
                && token.base_form == "ああ"
                && token.pos.first().is_some_and(|pos| pos == "感動詞")
            {
                // Check if next token is いう
                if let Some(next_token) = ctx.lookahead(1) {
                    if next_token.base_form == "いう"
                        && next_token.pos.first().is_some_and(|pos| pos == "動詞")
                        && next_token.pos.get(1).is_some_and(|pos| pos == "自立")
                    {
                        return (true, 2); // Consume both ああ and いう
                    }
                }
                // If ああ is not followed by いう, don't match
                return (false, 0);
            }

            (false, 0)
        }
    }

    vec![TokenMatcher::Custom(Arc::new(SouiuPatternMatcher))]
}

// Pattern: Verb[よう]
// Pattern: Verb[よう] (volitional form - casual let's, shall)
// Structures:
//   Plain: Verb[未然ウ接続] + う (見よう, 歩こう, etc.)
// Note: Polite ましょう is covered by separate N5 patterns (ましょう, ～ましょうか)
pub fn verb_you() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Plain volitional: Verb[未然ウ接続] + う
    #[derive(Debug)]
    struct PlainVolitionalMatcher;
    impl Matcher for PlainVolitionalMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            // First token: verb in 未然ウ接続
            let Some(token1) = ctx.current() else { return (false, 0); };
            if !token1.pos.first().is_some_and(|pos| pos == "動詞") {
                return (false, 0);
            }
            if !token1.pos.get(1).is_some_and(|pos| pos == "自立") {
                return (false, 0);
            }
            if !token1.features.get(5).is_some_and(|form| form == "未然ウ接続") {
                return (false, 0);
            }

            // Second token: う (auxiliary verb for volitional)
            let Some(token2) = ctx.lookahead(1) else { return (false, 0); };
            if token2.surface == "う"
                && token2.base_form == "う"
                && token2.pos.first().is_some_and(|pos| pos == "助動詞")
                && token2.features.get(4).is_some_and(|t| t == "不変化型")
            {
                return (true, 2);  // Consume 2 tokens
            }

            (false, 0)
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(PlainVolitionalMatcher)),
    ]
}

// Pattern: ようだ (seems like/appears to be - formal observation)
// Structures: Verb/Adj + よう + だ/です、な-Adj + な + よう + だ/です、Noun + の + よう + だ/です
pub fn youda() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    use super::Matcher;

    // Match よう as dependent noun with auxiliary verb stem
    // But NOT when followed by に particle (that would be ように, not ようだ)
    #[derive(Debug)]
    struct YouMatcher;
    impl Matcher for YouMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "よう"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                && token.pos.get(2).is_some_and(|pos| pos == "助動詞語幹") => {
                    // Check that next token is NOT に (any type of に particle)
                    // This could be 副詞化, 格助詞, etc. - all indicate ように, not ようだ
                    if let Some(next_token) = ctx.lookahead(1) {
                        if next_token.surface == "に"
                        && next_token.pos.first().is_some_and(|pos| pos == "助詞") {
                            return (false, 0); // This is ように, not ようだ
                        }
                    }
                    (true, 1)
                },
                _ => (false, 0),
            }
        }
    }

    // Match だ or です (auxiliary verbs)
    #[derive(Debug)]
    struct DaDesuMatcher;
    impl Matcher for DaDesuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "助動詞")
                && (token.base_form == "だ" || token.base_form == "です") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Can follow any word (verb, adjective, noun)
        TokenMatcher::Custom(Arc::new(YouMatcher)),
        optional(TokenMatcher::Custom(Arc::new(DaDesuMatcher))),
    ]
}

// Pattern: ぜんぜん (not at all - with negative expressions)
// Structure: ぜんぜん
pub fn zenzen() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct ZenzenMatcher;
    impl Matcher for ZenzenMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ぜんぜん"
                && token.base_form == "ぜんぜん"
                && token.pos.first().is_some_and(|pos| pos == "副詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    vec![TokenMatcher::Custom(Arc::new(ZenzenMatcher))]
}

// Pattern: かな (I wonder)
// Structure: Sentence + か + な
pub fn kana() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KaParticleMatcher;
    impl Matcher for KaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "か"
                && token.base_form == "か"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token
                    .pos
                    .get(1)
                    .is_some_and(|pos| pos == "副助詞／並立助詞／終助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NaEndingParticleMatcher;
    impl Matcher for NaEndingParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "な"
                && token.base_form == "な"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "終助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(KaParticleMatcher)),
        TokenMatcher::Custom(Arc::new(NaEndingParticleMatcher)),
    ]
}

// Pattern: あまり～ない (not very)
// Structures:
// - あまり + Verb[ない]
// - あまり + い-Adjective[ない]
// - あまり + Noun + ではない/じゃない
// - あまり + な-Adjective + ではない/じゃない
// - Casual variant: あんまり (instead of あまり)
//
// Examples:
// - あまり並ばない (not stand in line very long)
// - あまり寂しくない (not feel very lonely)
// - あまり平和ではない (not very peaceful)
// - あんまり食べたくない (don't want to eat very much)
//
// Tokenization pattern:
// - あまり OR あんまり (副詞/助詞類接続 OR 名詞/一般)
// - Wildcard (0-5 tokens)
// - ない (助動詞)
//
// Note: あまり can tokenize as:
// - 副詞/助詞類接続 (adverb, before verbs/adjectives)
// - 名詞/一般 (noun, before adjectives/nouns)
pub fn amari_uff5e_nai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match あまり or あんまり (flexible POS matching)
    #[derive(Debug)]
    struct AmariMatcher;
    impl Matcher for AmariMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match surface: あまり OR あんまり
                if token.surface != "あまり" && token.surface != "あんまり" {
                    return false;
                }
                // Match base form
                if token.base_form != "あまり" && token.base_form != "あんまり" {
                    return false;
                }

                // Can be either:
                // - 副詞/助詞類接続 (adverb)
                // - 名詞/一般 (noun)
                let is_adverb = token.pos.first().is_some_and(|pos| pos == "副詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "助詞類接続");

                let is_noun = token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "一般");

                is_adverb || is_noun
            })
        }
    }

    // Match ない (auxiliary verb OR adjective)
    #[derive(Debug)]
    struct NaiMatcher;
    impl Matcher for NaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                if token.surface != "ない" || token.base_form != "ない" {
                    return false;
                }

                // Check if it's auxiliary verb (助動詞)
                let is_auxiliary = token.pos.first().is_some_and(|pos| pos == "助動詞");

                // Check if it's adjective (形容詞/自立)
                let is_adjective = token.pos.first().is_some_and(|pos| pos == "形容詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "自立");

                is_auxiliary || is_adjective
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(AmariMatcher)),
        wildcard(0, 5, vec![]),
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
    ]
}

// Pattern: ば (conditional "if")
// Structures: Verb/Adj/Noun + conditional form + ば
//
// Tokenization patterns:
// 1. Verb in 仮定形 + ば (e.g., 見れば, 座れば, 歌えば)
// 2. い-Adjective in 仮定形 + ば (e.g., 痛ければ)
// 3. な-Adj/Noun + なら + ば (e.g., 嫌いならば, バイクならば)
// 4. Negative forms: なければ, でなければ
//
// Key tokenization:
// - 仮定形 (hypothetical form) is the conjugation form
// - ば is 助詞/接続助詞
//
// Pattern range: includes the word in 仮定形 + ば
pub fn ba() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match any token in 仮定形 (hypothetical/conditional form)
    #[derive(Debug)]
    struct KateiFormMatcher;
    impl Matcher for KateiFormMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Check if token is in 仮定形 (hypothetical form)
                if !token.features.get(5).is_some_and(|f| f == "仮定形") {
                    return false;
                }

                // Exclude "そういえば" (そう + いえ[仮定形] + ば)
                // If this is いえ (conditional form of 言う), check if previous token is そう
                if token.base_form == "いう" && token.surface == "いえ" {
                    if let Some(prev_token) = ctx.lookbehind(1) {
                        if prev_token.surface == "そう" && prev_token.pos.first().is_some_and(|pos| pos == "副詞") {
                            return false;
                        }
                    }
                }

                true
            })
        }
    }

    // Match ば particle
    #[derive(Debug)]
    struct BaParticleMatcher;
    impl Matcher for BaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ば"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(KateiFormMatcher)),
        TokenMatcher::Custom(Arc::new(BaParticleMatcher)),
    ]
}

// Pattern: なら (conditional "if")
// Structures: Verb/い-Adj/な-Adj/Noun + (の) + なら(ば)
//
// Tokenization:
// なら (助動詞, 特殊・ダ, 仮定形, base="だ")
// + ば (助詞/接続助詞) - optional
//
// Pattern range: なら or ならば (not including preceding word)
pub fn nara() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match なら (conditional form of だ auxiliary verb)
    #[derive(Debug)]
    struct NaraMatcher;
    impl Matcher for NaraMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "なら"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(4).is_some_and(|f| f == "特殊・ダ")
                && token.features.get(5).is_some_and(|f| f == "仮定形")
            })
        }
    }

    // Match ば (conditional particle)
    #[derive(Debug)]
    struct BaParticleMatcher;
    impl Matcher for BaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ば"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NaraMatcher)),
        optional(TokenMatcher::Custom(Arc::new(
            BaParticleMatcher,
        ))),
    ]
}

// Pattern: がる (to show signs of / to act like)
// Structures: Adjective + がる/がります
//
// Tokenization patterns:
// 1. Dictionary form (compound): 強がる → 強がる (動詞, base_form=強がる) - single token
// 2. Split conjugated form: 欲しがります → 欲し (形容詞, ガル接続) + がり (動詞/接尾) + ます
//
// This pattern has two separate matchers to handle both tokenization cases.

// Match dictionary form がる verbs (single token compounds like 強がる)
pub fn garu_compound() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct GaruVerbMatcher;
    impl Matcher for GaruVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            // Standalone verbs ending in がる that are NOT がる compounds
            const STANDALONE_VERBS: &[&str] = &[
                "上がる",   // to rise, go up
                "下がる",   // to fall, go down
                "揚がる",   // to be fried, to be raised
                "誇る",     // to boast (ends in がる in some forms)
            ];

            check_token(ctx, |token| {
                token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.base_form.ends_with("がる")
                && token.base_form != "がる"
                && !STANDALONE_VERBS.contains(&token.base_form.as_str())
            })
        }
    }

    vec![TokenMatcher::Custom(Arc::new(GaruVerbMatcher))]
}

// Match split conjugated がる forms (Adjective + がる suffix)
pub fn garu_split() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match adjective in ガル接続 form (stem for がる attachment)
    #[derive(Debug)]
    struct GaruConnectingAdjMatcher;
    impl Matcher for GaruConnectingAdjMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "ガル接続")
            })
        }
    }

    // Match がる as verb suffix (動詞/接尾, base_form=がる)
    #[derive(Debug)]
    struct GaruSuffixMatcher;
    impl Matcher for GaruSuffixMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                && token.base_form == "がる"
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(GaruConnectingAdjMatcher)),
        TokenMatcher::Custom(Arc::new(GaruSuffixMatcher)),
    ]
}

// Pattern: がする (sensory experience)
// Structure: Noun + が + する/します
//
// Example tokenizations:
// - 匂いがする: 匂い(名詞) + が(助詞/格助詞) + する(動詞/サ変・スル)
// - 音がします: 音(名詞) + が(助詞/格助詞) + し(動詞/連用形) + ます(助動詞)
//
// Common sensory nouns: 匂い (smell), 音 (sound), 味 (taste), 感じ (feeling), 気 (sense/feeling)
pub fn gasuru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct GaParticleMatcher;
    impl Matcher for GaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "が"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            })
        }
    }

    vec![
        noun(), // Sensory noun (匂い, 音, 味, 感じ, 気, etc.)
        TokenMatcher::Custom(Arc::new(GaParticleMatcher)), // が (格助詞)
        verb_base("する"), // する (verb)
    ]
}

// Pattern: たがる
// Pattern: たがる - wanting to do (third person observable desire)
// Structures: Verb[stem] + た (from たい) + がる
pub fn tagaru() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct TaiGaruMatcher;
    impl Matcher for TaiGaruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.base_form == "たい"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "ガル接続")
            })
        }
    }

    #[derive(Debug)]
    struct GaruMatcher;
    impl Matcher for GaruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.base_form == "がる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
            })
        }
    }

    vec![
        super::flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(TaiGaruMatcher)),
        TokenMatcher::Custom(Arc::new(GaruMatcher)),
    ]
}

// Pattern: かもしれない (might/maybe)
// Structures: Verb/Adjective/Noun + かもしれない/かもしれません
pub fn kamoshirenai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match かも (副助詞)
    #[derive(Debug)]
    struct KamoMatcher;
    impl Matcher for KamoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "かも"
                && token.base_form == "かも"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match しれ (verb form of しれる)
    #[derive(Debug)]
    struct ShireMatcher;
    impl Matcher for ShireMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "しれ"
                && token.base_form == "しれる"
                && token.pos.first().is_some_and(|pos| pos == "動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match ない (auxiliary) or ません pattern
    #[derive(Debug)]
    struct NaiOrMasenMatcher;
    impl Matcher for NaiOrMasenMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Either ない (助動詞) or ませ (for ません)
            if token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
            {
                return true;
            }
            // For ません pattern: ませ + ん
            if token.surface == "ませ"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
            {
                return true;
            }
            false
            })
        }
    }

    // Match ん (auxiliary for ません)
    #[derive(Debug)]
    struct NMasenMatcher;
    impl Matcher for NMasenMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ん"
                && token.base_form == "ん"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Verb, Adjective, or Noun
        TokenMatcher::Custom(Arc::new(KamoMatcher)),
        TokenMatcher::Custom(Arc::new(ShireMatcher)),
        TokenMatcher::Custom(Arc::new(NaiOrMasenMatcher)),
        optional(TokenMatcher::Custom(Arc::new(NMasenMatcher))),
    ]
}

// Pattern: みたいに・みたいな (like, as - casual form)
// Structures: Noun/Verb/Adj + みたい + に, Noun/Verb/Adj + みたい + な
// Note: Casual equivalent of ように・ような
pub fn mitaini_u30fb_mitaina() -> Vec<TokenMatcher> {
    // Match みたい as dependent noun with 形容動詞語幹 (na-adjective stem)
    #[derive(Debug)]
    struct MitaiMatcher;
    impl Matcher for MitaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "みたい"
                && token.base_form == "みたい"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                && token.pos.get(2).is_some_and(|pos| pos == "形容動詞語幹")
            })
        }
    }

    // Match either に particle or な auxiliary after みたい
    let ni_or_na_matcher = or(vec![
        surface("に"), // に particle for みたいに
        surface("な")  // な auxiliary for みたいな
    ]);

    // Match noun, verb, or adjective
    let noun_verb_or_adj_matcher = or(vec![
        noun(),
        verb(),
        adjective()
    ]);

    vec![
        noun_verb_or_adj_matcher,
        TokenMatcher::Custom(Arc::new(MitaiMatcher)),
        ni_or_na_matcher,
    ]
}

// Pattern: そうに・そうな (seems like/looks like - adverbial and attributive forms)
// Structures: Verb/Adj[stem] + そう + に/な
pub fn souni_u30fb_souna() -> Vec<TokenMatcher> {
    // Match verb/adjective stem OR な in negative forms (reused from そう pattern)
    #[derive(Debug)]
    struct StemOrNaiMatcher;
    impl Matcher for StemOrNaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Verb stem (連用形)
            (token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形"))
            ||
            // い-Adjective stem (ガル接続)
            (token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "ガル接続"))
            ||
            // な-Adjective (形容動詞語幹)
            (token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹"))
            ||
            // ない auxiliary in ガル接続 (for negative forms)
            (token.surface == "な"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "ガル接続"))
            ||
            // ない adjective in ガル接続 (for negative forms: 大事じゃな)
            (token.surface == "な"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "ガル接続"))
            })
        }
    }

    // Match そう auxiliary (名詞/接尾/助動詞語幹 OR 副詞/助詞類接続 for negative forms)
    #[derive(Debug)]
    struct SouAuxiliaryMatcher;
    impl Matcher for SouAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "そう"
                && token.base_form == "そう"
                && ((token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                    && token.pos.get(2).is_some_and(|pos| pos == "助動詞語幹"))
                || (token.pos.first().is_some_and(|pos| pos == "副詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "助詞類接続")))
            })
        }
    }

    // Create helper matchers using simple surface matching where appropriate
    let sa_suffix_matcher = surface("さ"); // さ suffix for negative forms: なさそう
    let ni_or_na_matcher = or(vec![
        surface("に"), // に particle for adverbial form
        surface("な")  // な auxiliary for attributive form
    ]);

    vec![
        TokenMatcher::Custom(Arc::new(StemOrNaiMatcher)), // Verb/Adj stem OR な (negative)
        optional(sa_suffix_matcher), // Optional さ (for なさそう)
        TokenMatcher::Custom(Arc::new(SouAuxiliaryMatcher)), // そう
        ni_or_na_matcher, // に (adverbial) or な (attributive)
    ]
}

// Pattern: のように・のような (like/as - with noun)
// Structures: Noun + のように + Verb/Adj, Noun + のような + Noun
pub fn noyouni_u30fb_noyouna() -> Vec<TokenMatcher> {
    // Match よう (名詞/非自立/助動詞語幹)
    #[derive(Debug)]
    struct YouMatcher;
    impl Matcher for YouMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "よう"
                && token.base_form == "よう"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
            })
        }
    }

    vec![
        noun(), // Match any noun
        surface("の"), // Match の particle
        TokenMatcher::Custom(Arc::new(YouMatcher)), // Match よう with specific POS
        or(vec![
            surface("に"), // に particle for のように (adverbial)
            surface("な")  // な auxiliary for のような (attributive)
        ]),
    ]
}

// Pattern: 〜ようと思う・〜おうと思う (intend to/thinking of doing)
// Structures: Verb[未然ウ接続] + う + と + 思う/思っている/思います/思っています
pub fn u301c_youtoomou_u30fb_u301c_outoomou() -> Vec<TokenMatcher> {
    // Match quotation particle と
    #[derive(Debug)]
    struct QuotationToMatcher;
    impl Matcher for QuotationToMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "と"
                && token.base_form == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "引用")
            })
        }
    }

    super::concat(vec![
        verb_you(),  // Verb[未然ウ接続] + う
        vec![TokenMatcher::Custom(Arc::new(QuotationToMatcher))], // と quotation particle
        vec![verb_base("思う")], // 思う verb in any conjugation
        // Optional: て + いる (for 思っている)
        vec![optional(surface("て"))], // て particle
        vec![optional(verb_base("いる"))], // いる verb
        // Optional: ます (for polite forms)
        vec![optional(mashi_form())], // ます polite form
    ])
}

// Pattern: く・に (adverb formation)
// Structures: い-Adj[く] + Verb, な-Adj + に + Verb, Exception: いい→よく
// Note: This pattern is very broad and matches adverbial forms modifying verbs.
// It's not highlighted in overlays (priority < 5) but useful for detection.
pub fn ku_u30fb_ni() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for adverbial forms (い-adj[く], な-adj+に, or よく exception)
    #[derive(Debug)]
    struct AdverbialFormMatcher;
    impl Matcher for AdverbialFormMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Pattern 1: い-adjective in 連用テ接続 form (ends with く)
                let is_i_adj_ku = token.pos.first().is_some_and(|pos| pos == "形容詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "自立")
                    && token.features.get(5).is_some_and(|form| form == "連用テ接続")
                    && token.surface.ends_with("く");

                // Pattern 2: な-adjective stem (形容動詞語幹)
                let is_na_adj_stem = token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹");

                // Pattern 3: よく adverb (exception for いい)
                let is_yoku = token.surface == "よく"
                    && token.base_form == "よく"
                    && token.pos.first().is_some_and(|pos| pos == "副詞");

                is_i_adj_ku || is_na_adj_stem || is_yoku
            })
        }
    }

    // Matcher for に adverbial particle (optional - only for な-adj)
    #[derive(Debug)]
    struct NiAdverbialMatcher;
    impl Matcher for NiAdverbialMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "に"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "副詞化")
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(AdverbialFormMatcher)),
        optional(TokenMatcher::Custom(Arc::new(NiAdverbialMatcher))),
        verb(),
    ]
}

// Pattern: ～にする・～くする
// Pattern: ～にする・～くする (to make/do something)
// Structures: な-Adj + に + する, い-Adj[く] + する, Noun + に + する
pub fn uff5e_nisuru_u30fb_uff5e_kusuru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Custom matcher that handles the full pattern with conditional に requirement
    // - い-adjective[く] + する (no に needed)
    // - な-adjective + に + する (に required)
    // - Noun + に + する (に required)
    #[derive(Debug)]
    struct NisuruKusuruFullMatcher;
    impl Matcher for NisuruKusuruFullMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            // First token must be an adjective or noun
            let first = match ctx.current() {
                Some(token) => token,
                None => return (false, 0),
            };

            // Exclude compound suru-verbs (e.g., 分解する, 説明する)
            // These are 動詞 with サ変 conjugation type, NOT にする pattern
            if first.pos.first().is_some_and(|pos| pos == "動詞")
                && first.pos.get(1).is_some_and(|pos| pos == "サ変・スル" || pos == "サ変・−スル")
            {
                return (false, 0);
            }

            // Check if it's an い-adjective in 連用テ接続 form (く form)
            let is_i_adj_ku = first.pos.first().is_some_and(|pos| pos == "形容詞")
                && first.pos.get(1).is_some_and(|pos| pos == "自立")
                && first.features.get(5).is_some_and(|f| f == "連用テ接続");

            // For い-adjective[く] + する, check if there's a を object marker nearby
            // The pattern ～くする means "to make (something) ADJ", not "to do ADJ-ly"
            // So we need evidence of a transformative use (object を) vs. adverbial use
            if is_i_adj_ku {
                // Look back up to 3 tokens for を particle
                // This helps distinguish "XをADJ-くする" (transform X) from "ADJ-くする" (do ADJ-ly)
                let mut has_wo_object = false;
                for i in 1..=3 {
                    if let Some(prev_token) = ctx.lookbehind(i) {
                        if prev_token.surface == "を"
                            && prev_token.pos.first().is_some_and(|p| p == "助詞")
                            && prev_token.pos.get(1).is_some_and(|p| p == "格助詞")
                        {
                            has_wo_object = true;
                            break;
                        }
                        // Stop if we hit a major punctuation or sentence boundary
                        if prev_token.surface == "。" || prev_token.surface == "、" {
                            break;
                        }
                    }
                }

                // If no を object found for い-adj[く], it's likely adverbial use, not にする pattern
                if !has_wo_object {
                    return (false, 0);
                }
            }

            // Check if it's a な-adjective (形容動詞語幹) specifically
            // Regular nouns are handled by the N5 にする pattern, not this N4 pattern
            let is_na_adj_or_noun = first.pos.first().is_some_and(|pos| pos == "名詞")
                && first.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹");

            if !is_i_adj_ku && !is_na_adj_or_noun {
                return (false, 0);
            }

            // Check next token
            let second = match ctx.lookahead(1) {
                Some(token) => token,
                None => return (false, 0),
            };

            // Exclude cases where the next token is する in 未然形 (followed by causative)
            // Example: こと + さ (未然形) + せる is causative, not にする pattern
            if second.base_form == "する" && second.features.get(5).is_some_and(|f| f == "未然形") {
                return (false, 0);
            }

            // For い-adjective: next token can be する directly OR に + する
            // For な-adjective/noun: next token MUST be に, followed by する
            if is_i_adj_ku {
                // Exclude causative せる/させる (these are not にする pattern)
                if (second.base_form == "せる" || second.base_form == "させる")
                    && second.pos.get(1).is_some_and(|p| p == "接尾")
                {
                    return (false, 0);
                }

                // Check if second token is する (no に)
                // Exclude imperative forms (命令ｒｏ/命令yo) as they belong to different patterns
                if second.base_form == "する"
                    && second.pos.first().is_some_and(|pos| pos == "動詞")
                    && !second.features.get(5).is_some_and(|f| f.contains("命令"))
                    && !second.features.get(5).is_some_and(|f| f == "未然形")
                {
                    // Double-check that the first token is actually an い-adjective
                    // and not something else that might have slipped through
                    if !first.pos.first().is_some_and(|pos| pos == "形容詞") {
                        return (false, 0);
                    }
                    return (true, 2); // い-Adj[く] + する
                }

                // Check if second token is に, followed by する
                if second.surface == "に"
                    && second.pos.first().is_some_and(|pos| pos == "助詞")
                    && second.pos.get(1).is_some_and(|pos| pos == "格助詞")
                {
                    if let Some(third) = ctx.lookahead(2) {
                        if third.base_form == "する"
                            && third.pos.first().is_some_and(|pos| pos == "動詞")
                            && !third.features.get(5).is_some_and(|f| f.contains("命令"))
                            && !third.features.get(5).is_some_and(|f| f == "未然形")
                        {
                            // Double-check that the first token is actually an い-adjective
                            if !first.pos.first().is_some_and(|pos| pos == "形容詞") {
                                return (false, 0);
                            }
                            return (true, 3); // い-Adj[く] + に + する
                        }
                    }
                }

                return (false, 0);
            } else {
                // Exclude causative せる/させる (these are not にする pattern)
                // Example: 全滅させる is causative, not にする pattern
                if (second.base_form == "せる" || second.base_form == "させる")
                    && second.pos.get(1).is_some_and(|p| p == "接尾")
                {
                    return (false, 0);
                }

                // For な-adjective/noun: に is REQUIRED
                if second.surface != "に"
                    || !second.pos.first().is_some_and(|pos| pos == "助詞")
                    || !second.pos.get(1).is_some_and(|pos| pos == "格助詞")
                {
                    return (false, 0);
                }

                // Third token must be する
                if let Some(third) = ctx.lookahead(2) {
                    if third.base_form == "する"
                        && third.pos.first().is_some_and(|pos| pos == "動詞")
                        && !third.features.get(5).is_some_and(|f| f.contains("命令"))
                        && !third.features.get(5).is_some_and(|f| f == "未然形")
                    {
                        // For な-adjectives, require を object marker to distinguish
                        // from N5 にする (decision) pattern.
                        // N4: Xをきれいにする (make X clean) - transformation
                        // N5: コーヒーにする (I'll have coffee) - decision
                        let mut has_wo_object = false;
                        for i in 1..=5 {
                            if let Some(prev_token) = ctx.lookbehind(i) {
                                if prev_token.surface == "を"
                                    && prev_token.pos.first().is_some_and(|p| p == "助詞")
                                    && prev_token.pos.get(1).is_some_and(|p| p == "格助詞")
                                {
                                    has_wo_object = true;
                                    break;
                                }
                                // Stop if we hit punctuation or sentence boundary
                                if prev_token.surface == "。" || prev_token.surface == "、" {
                                    break;
                                }
                            }
                        }

                        if has_wo_object {
                            return (true, 3); // な-Adj + に + する (with を object)
                        }
                    }
                }

                return (false, 0);
            }
        }
    }

    vec![TokenMatcher::Custom(Arc::new(NisuruKusuruFullMatcher))]
}

// Pattern: といい (it would be good if)
// Structures: Noun/な-Adj + だ + と + いい(verb)
// Note: When だと precedes いい, Kagome tokenizes いい as いう(verb/連用形) not いい(adjective)
// This pattern catches that specific case, while the N3 pattern たらいい・といい_と handles
// Verb/い-Adj + と + いい(adjective)
pub fn toii() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match だ copula (助動詞)
    #[derive(Debug)]
    struct DaCopulaMatcher;
    impl Matcher for DaCopulaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "だ" && token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    // Match と as quotation particle (格助詞/引用)
    #[derive(Debug)]
    struct ToQuotationMatcher;
    impl Matcher for ToQuotationMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "と"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                    && token.pos.get(2).is_some_and(|pos| pos == "引用")
            })
        }
    }

    // Match いい when tokenized as いう verb in 連用形
    #[derive(Debug)]
    struct IiVerbMatcher;
    impl Matcher for IiVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "いい"
                    && token.base_form == "いう"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token.features.get(5).is_some_and(|f| f == "連用形")
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(DaCopulaMatcher)),
        TokenMatcher::Custom(Arc::new(ToQuotationMatcher)),
        TokenMatcher::Custom(Arc::new(IiVerbMatcher)),
    ]
}

// Pattern: といってもいい (you could say / one might say)
// Structures: Verb/い-Adj/な-Adj/Noun + といってもいい
// Note: This is the quotative と + 言う(say) in て-form + も + いい
pub fn to_itte_mo_ii() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match と as quotation particle (格助詞/引用)
    #[derive(Debug)]
    struct ToQuotationMatcher;
    impl Matcher for ToQuotationMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "と"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                    && token.pos.get(2).is_some_and(|pos| pos == "引用")
            })
        }
    }

    // Match いう verb in 連用タ接続 form (いっ)
    #[derive(Debug)]
    struct IuVerbMatcher;
    impl Matcher for IuVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.base_form == "いう"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token.features.get(5).is_some_and(|f| f == "連用タ接続")
            })
        }
    }

    // Match て connecting particle
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl Matcher for TeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "て"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
            })
        }
    }

    // Match も particle
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "も"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(ToQuotationMatcher)),
        TokenMatcher::Custom(Arc::new(IuVerbMatcher)),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(MoParticleMatcher)),
        super::ii_form(),
    ]
}

// Pattern: ようになる (to reach the point of / to come to)
// Structures: Verb[できる/る/ない] + ように + なる
pub fn youninaru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ない auxiliary (negative)
    #[derive(Debug)]
    struct NaiMatcher;
    impl Matcher for NaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ない"
                    && token.base_form == "ない"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    // Match よう as dependent noun with auxiliary verb stem
    #[derive(Debug)]
    struct YouMatcher;
    impl Matcher for YouMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "よう"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                    && token.pos.get(2).is_some_and(|pos| pos == "助動詞語幹")
            })
        }
    }

    // Match に as case particle
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "に"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            })
        }
    }

    vec![
        verb(),
        optional(TokenMatcher::Custom(Arc::new(NaiMatcher))),
        TokenMatcher::Custom(Arc::new(YouMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        verb_base("なる"),
    ]
}

// Pattern: まい～のように (almost every [time period])
// Structures: まい + Time Word (日/週/月/年 etc.) + のように
pub fn mai_uff5e_noyouni() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match まい (tokenized as verb まう in 連用形)
    // Note: まい should be the prefix 毎 (every), but Kagome tokenizes it as the verb まう (to dance)
    #[derive(Debug)]
    struct MaiMatcher;
    impl Matcher for MaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "まい"
                && token.base_form == "まう"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立")
                && token
                    .features
                    .get(5)
                    .is_some_and(|form| form == "連用形") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match よう (名詞/非自立/助動詞語幹)
    #[derive(Debug)]
    struct YouMatcher;
    impl Matcher for YouMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "よう"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                && token
                    .pos
                    .get(2)
                    .is_some_and(|pos| pos == "助動詞語幹") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(MaiMatcher)),
        noun(), // Match time expression nouns (日, 週, 月, 年, etc.)
        surface("の"), // Match の particle (連体化)
        TokenMatcher::Custom(Arc::new(YouMatcher)),
        surface("に"), // Match に particle (副詞化)
    ]
}

// Pattern: じゃないか (isn't it?)
// Structures: Phrase + じゃない + か OR Phrase + ではない + か
pub fn janaika() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ん (explanatory の) - optional
    #[derive(Debug)]
    struct NMatcher;
    impl Matcher for NMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ん"
                && token.base_form == "ん"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match じゃ (casual) or で (formal)
    #[derive(Debug)]
    struct JyaOrDeMatcher;
    impl Matcher for JyaOrDeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "じゃ" && token.base_form == "じゃ"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞"))
            || (token.surface == "で" && token.base_form == "で"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match は (only for ではないか formal form)
    #[derive(Debug)]
    struct WaParticleMatcher;
    impl Matcher for WaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "は"
                && token.base_form == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match ない (auxiliary verb)
    #[derive(Debug)]
    struct NaiAuxMatcher;
    impl Matcher for NaiAuxMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match か (sentence-ending particle)
    #[derive(Debug)]
    struct KaEndingMatcher;
    impl Matcher for KaEndingMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "か"
                && token.base_form == "か"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos.contains("終助詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        optional(TokenMatcher::Custom(Arc::new(NMatcher))),
        TokenMatcher::Custom(Arc::new(JyaOrDeMatcher)),
        optional(TokenMatcher::Custom(Arc::new(WaParticleMatcher))),
        TokenMatcher::Custom(Arc::new(NaiAuxMatcher)),
        TokenMatcher::Custom(Arc::new(KaEndingMatcher)),
    ]
}

// Pattern: らしい ① (seems like/apparently - hearsay/conjecture)
// Structures: Verb/Adj/Noun + らしい (助動詞)
pub fn rashii_u2460() -> Vec<TokenMatcher> {
    use super::Matcher;
    use std::sync::Arc;

    // Match らしい as auxiliary verb (not い-adjective)
    #[derive(Debug)]
    struct RashiiAuxMatcher;
    impl Matcher for RashiiAuxMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "らしい"
                && token.base_form == "らしい"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        or(vec![verb(), adjective(), noun()]),
        TokenMatcher::Custom(Arc::new(RashiiAuxMatcher)),
    ]
}

// Pattern: ておく (do in advance, leave as is)
// Structures: Verb[て] + おく/とく (casual), polite forms with ます
pub fn teoku() -> Vec<TokenMatcher> {
    use super::Matcher;
    use std::sync::Arc;

    // Match て or で particle
    #[derive(Debug)]
    struct TeDeFormMatcher;
    impl Matcher for TeDeFormMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match おく or とく as auxiliary verb
    #[derive(Debug)]
    struct OkuMatcher;
    impl Matcher for OkuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.base_form == "おく" || token.base_form == "とく")
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        super::flexible_verb_form(),
        // て or で particle is optional because of the とく contraction
        optional(TokenMatcher::Custom(Arc::new(TeDeFormMatcher))),
        TokenMatcher::Custom(Arc::new(OkuMatcher)),
        // Optional ます for polite form
        optional(surface("ます")),
    ]
}

// Pattern: がほしい (want something)
// Structures: Noun + が + ほしい (+ です)
pub fn gahoshii() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct GaParticleMatcher;
    impl Matcher for GaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "が"
                && token.base_form == "が"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        noun(),                                      // Noun (犬, 車, 時間, etc.)
        TokenMatcher::Custom(Arc::new(GaParticleMatcher)),   // が (格助詞)
        surface("ほしい"),                           // ほしい (specifically this adjective)
    ]
}

// Pattern: てほしい (want someone to do)
// Structures: Verb[て] + ほしい
pub fn tehoshii() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct TeDeFormMatcher;
    impl Matcher for TeDeFormMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct HoshiiMatcher;
    impl Matcher for HoshiiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.base_form == "ほしい"
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        super::flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(TeDeFormMatcher)),
        TokenMatcher::Custom(Arc::new(HoshiiMatcher)),
    ]
}

// Pattern: ときいた (I heard that)
// Structures: Phrase/Verb/Adj/Noun + (だ) + ときいた/と聞きました
pub fn tokiita() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match と quotation particle (can be 接続助詞 or 格助詞/引用)
    #[derive(Debug)]
    struct ToQuotationMatcher;
    impl Matcher for ToQuotationMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "と"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && (token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
                        || (token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                            && token.pos.get(2).is_some_and(|pos| pos == "引用")))
            })
        }
    }

    // Match きく/聞く verb in 連用形 or 連用タ接続 (for both polite and standard past)
    #[derive(Debug)]
    struct KikuVerbMatcher;
    impl Matcher for KikuVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.base_form == "きく" || token.base_form == "聞く")
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "自立")
                    && (token.features.get(5).is_some_and(|f| f == "連用形")
                        || token.features.get(5).is_some_and(|f| f == "連用タ接続"))
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(ToQuotationMatcher)),
        TokenMatcher::Custom(Arc::new(KikuVerbMatcher)),
        // Pattern matcher will automatically extend to include:
        // - た (standard past: ときいた)
        // - ます + た (polite past: と聞きました)
    ]
}

// Pattern: 聞こえる (to be audible, can be heard)
// Structures: Noun + が + 聞こえる
pub fn kikoeru() -> Vec<TokenMatcher> {
    vec![
        super::noun(),
        surface_particle("が", "格助詞"),
        verb_base("聞こえる"),
    ]
}

// Pattern: 見える (to be visible, can be seen)
// Structures: Noun + が + 見える
pub fn mieru() -> Vec<TokenMatcher> {
    vec![
        super::noun(),
        surface_particle("が", "格助詞"),
        verb_base("見える"),
    ]
}

// Pattern: だす
// Pattern: だす - suddenly start doing (unintentional/uncontrolled)
// Structures: Verb[stem] + だす/だし
pub fn dasu() -> Vec<TokenMatcher> {
    vec![super::flexible_verb_form(), verb_base("だす")]
}

// Pattern: ～代 (decade/era suffix)
// Structures: Decade of age + 代 / Decade + 年代
pub fn uff5e_dai() -> Vec<TokenMatcher> {
    vec![
        noun_subtype("数"),
        wildcard(0, 3, vec![]), // Allow 0-3 more number tokens (for multi-digit numbers)
        or(vec![
            surface_noun_suffix("代"),
            surface_noun_suffix("年代")
        ]),
    ]
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
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "数") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match counter tokens (名詞/接尾/助数詞)
    #[derive(Debug)]
    struct CounterMatcher;
    impl Matcher for CounterMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                && token.pos.get(2).is_some_and(|pos| pos == "助数詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Pattern: One or more numbers + counter + も
    // Numbers can be 1-many tokens (一 vs １２ vs ２０万円)
    // We'll use a flexible approach with optional number tokens

    concat(vec![
        vec![TokenMatcher::Custom(Arc::new(NumberMatcher))], // First number (required)
        vec![
            optional(TokenMatcher::Custom(Arc::new(NumberMatcher))),
            optional(TokenMatcher::Custom(Arc::new(NumberMatcher))),
            optional(TokenMatcher::Custom(Arc::new(NumberMatcher))),
            optional(TokenMatcher::Custom(Arc::new(NumberMatcher))),
            optional(TokenMatcher::Custom(Arc::new(NumberMatcher))),
        ], // Up to 5 additional numbers (should be enough for most cases)
        vec![TokenMatcher::Custom(Arc::new(CounterMatcher))],
        vec![surface_particle("も", "係助詞")],
    ])
}

// Pattern: ほとんど
// Pattern: ほとんど (most, almost, hardly any)
// Structures: ほとんど (adverb)
pub fn hotondo() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    #[derive(Debug)]
    struct HotondoMatcher;
    impl Matcher for HotondoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ほとんど"
                && token.base_form == "ほとんど"
                && token.pos.first().is_some_and(|pos| pos == "副詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    vec![TokenMatcher::Custom(Arc::new(HotondoMatcher))]
}

// Pattern: そんな・こんな・あんな・どんな
// Pattern: そんな・こんな・あんな・どんな (like that, like this, what kind of)
// Structures: そんな/こんな/あんな/どんな + Noun (may have adjectives in between)
// Abbreviations from: そのような, このような, あのような, どのような
pub fn sonna_u30fb_konna_u30fb_anna_u30fb_donna() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for all four forms that modify a noun (directly or through adjectives)
    #[derive(Debug)]
    struct SonnaMatcher;
    impl Matcher for SonnaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            // Check current token is そんな/こんな/あんな/どんな
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "連体詞")
                && (token.base_form == "そんな"
                    || token.base_form == "こんな"
                    || token.base_form == "あんな"
                    || token.base_form == "どんな") => {
                    // Check that next token is a noun OR an adjective (which would then modify a noun)
                    if let Some(next_token) = ctx.lookahead(1) {
                        if next_token.pos.first().is_some_and(|pos| {
                            pos == "名詞" || pos == "形容詞"
                        }) {
                            return (true, 1);
                        }
                    }
                    (false, 0)
                },
                _ => (false, 0),
            }
        }
    }

    vec![TokenMatcher::Custom(Arc::new(SonnaMatcher))]
}

// Pattern: 各
pub fn kaku() -> Vec<TokenMatcher> {
    use super::Matcher;

    // Match 各 as prefix (接頭詞/名詞接続)
    #[derive(Debug)]
    struct KakuPrefixMatcher;
    impl Matcher for KakuPrefixMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "各"
                && token.base_form == "各"
                && token.pos.first().is_some_and(|pos| pos == "接頭詞")
                && token.pos.get(1).is_some_and(|pos| pos == "名詞接続") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(KakuPrefixMatcher)),
        super::noun(),
    ]
}

// Pattern: 以上 ① (at least / more than / equal to or more than / that's all)
// Structures: Noun/Amount + 以上, それ/これ/あれ + 以上, standalone 以上
pub fn ijou_u2460() -> Vec<TokenMatcher> {
    use super::Matcher;

    // Match 以上 as 名詞/非自立
    #[derive(Debug)]
    struct IjouMatcher;
    impl Matcher for IjouMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "以上"
                && token.base_form == "以上"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match numbers, counters, or demonstratives (名詞/数, 名詞/接尾/助数詞, or demonstratives)
    #[derive(Debug)]
    struct NumberCounterOrDemonstrativeMatcher;
    impl Matcher for NumberCounterOrDemonstrativeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) => {
                    // Match demonstratives (それ, これ, あれ)
                    let is_demonstrative = ["それ", "これ", "あれ"].contains(&token.surface.as_str())
                        && token.pos.first().is_some_and(|pos| pos == "名詞");

                    // Match numbers or counters
                    let is_number_or_counter = token.pos.first().is_some_and(|pos| pos == "名詞")
                        && (token.pos.get(1).is_some_and(|pos| pos == "数")
                            || (token.pos.get(1).is_some_and(|pos| pos == "接尾")
                                && token.pos.get(2).is_some_and(|pos| pos == "助数詞")));

                    if is_demonstrative || is_number_or_counter {
                        (true, 1)
                    } else {
                        (false, 0)
                    }
                }
                _ => (false, 0),
            }
        }
    }

    vec![
        optional(TokenMatcher::Custom(Arc::new(NumberCounterOrDemonstrativeMatcher))),
        TokenMatcher::Custom(Arc::new(IjouMatcher)),
    ]
}

// Pattern: いか (equal to or less than / the following)
// Structures: Noun/Amount + 以下, standalone 以下
pub fn ika() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct IkaMatcher;
    impl Matcher for IkaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "いか"
                    && token.base_form == "いか"
                    && (token.pos.first().is_some_and(|pos| pos == "名詞")
                        || token.pos.first().is_some_and(|pos| pos == "副詞"))
            })
        }
    }

    vec![TokenMatcher::Custom(Arc::new(IkaMatcher))]
}

// いがい: Except/besides (except A, other than A)
// Structures: Verb + 以外, Noun + 以外
pub fn igai() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct IgaiMatcher;
    impl Matcher for IgaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "以外"
                    && token.base_form == "以外"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "非自立")
            })
        }
    }

    vec![
        any(), // Verb or Noun
        TokenMatcher::Custom(Arc::new(IgaiMatcher)), // 以外 (名詞/非自立/副詞可能)
    ]
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
    // Match ずっと adverb (副詞/一般)
    #[derive(Debug)]
    struct ZuttoMatcher;
    impl Matcher for ZuttoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ずっと"
                    && token.base_form == "ずっと"
                    && token.pos.first().is_some_and(|pos| pos == "副詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "一般")
            })
        }
    }

    vec![TokenMatcher::Custom(Arc::new(ZuttoMatcher))]
}

// Pattern: だいたい
// Pattern: だいたい (generally, mostly, approximately, in the first place)
// Structures: だいたい as adverb or noun
pub fn daitai() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct DaitaiMatcher;
    impl Matcher for DaitaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "だいたい"
                    && token.base_form == "だいたい"
                    && (token.pos.first().is_some_and(|pos| pos == "副詞")
                        || token.pos.first().is_some_and(|pos| pos == "名詞"))
            })
        }
    }
    vec![TokenMatcher::Custom(Arc::new(DaitaiMatcher))]
}

// Pattern: Among/in/within (Noun/この/その + の + なか + で)
// Structures: Noun + の + 中（なか）で, その + 中（なか）で, この + 中（なか）で
pub fn nonakade() -> Vec<TokenMatcher> {
    // Noun or demonstrative (この, その, あの)
    #[derive(Debug)]
    struct NounOrDemonstrativeMatcher;
    impl Matcher for NounOrDemonstrativeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.pos.first().is_some_and(|pos| pos == "名詞" || pos == "連体詞")
            })
        }
    }

    // なか (名詞/非自立)
    #[derive(Debug)]
    struct NakaMatcher;
    impl Matcher for NakaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "なか"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "非自立")
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NounOrDemonstrativeMatcher)),
        surface_particle("の", "連体化"),
        TokenMatcher::Custom(Arc::new(NakaMatcher)),
        surface_particle("で", "格助詞"),
    ]
}

// Pattern: ように・ような (like, as - adverbial/attributive forms)
// Structures: Verb/Adj + よう + に, Verb/Adj + よう + な
// Note: This pattern matches ように/ような when NOT preceded by の (that's handled by のように・のような)
pub fn youni_u30fb_youna() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match よう as dependent noun with auxiliary verb stem
    #[derive(Debug)]
    struct YouMatcher;
    impl Matcher for YouMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "よう"
                && token.base_form == "よう"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                && token.pos.get(2).is_some_and(|pos| pos == "助動詞語幹")
            })
        }
    }

    // Match に particle (格助詞/一般 for adverbial, or 副詞化 for purpose)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                    || token.pos.get(1).is_some_and(|pos| pos == "副詞化"))
            })
        }
    }

    // Match な auxiliary (体言接続 form of だ)
    #[derive(Debug)]
    struct NaAuxiliaryMatcher;
    impl Matcher for NaAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "な"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "体言接続")
            })
        }
    }

    // Match either に or な after よう
    let ni_or_na_matcher = or(vec![
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(NaAuxiliaryMatcher))
    ]);

    // Match verb or adjective (but NOT の particle, which is handled by のように・のような)
    let verb_or_adj_matcher = or(vec![
        verb(),
        adjective(),
        past_auxiliary() // for auxiliaries like た
    ]);

    vec![
        verb_or_adj_matcher,
        TokenMatcher::Custom(Arc::new(YouMatcher)),
        ni_or_na_matcher,
    ]
}

// Pattern: Number/Amount + は (at least, or so)
// Structures: Counter + (くらい/ぐらい) + は  (e.g., 回は, キロくらいは)
// Examples: ５回は (at least 5 times), ２キロくらいは (at least 2 kg)
//
// Note: This pattern detects the contrastive use of は after counters to mean "at least" or "or so".
// The pattern starts from the counter (not the number), as the counter + は is the key construction.
pub fn number_amount_ha() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match counter (助数詞) like 回, キロ, 時間, etc.
    #[derive(Debug)]
    struct CounterMatcher;
    impl Matcher for CounterMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                && token.pos.get(2).is_some_and(|pos| pos == "助数詞")
            })
        }
    }

    // Match くらい or ぐらい (副助詞)
    #[derive(Debug)]
    struct KuraiMatcher;
    impl Matcher for KuraiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.surface == "くらい" || token.surface == "ぐらい")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞")
            })
        }
    }

    // Match は (係助詞) - use existing helper
    #[derive(Debug)]
    struct HaMatcher;
    impl Matcher for HaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "は"
                && token.base_form == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(CounterMatcher)),
        optional(TokenMatcher::Custom(Arc::new(KuraiMatcher))),
        TokenMatcher::Custom(Arc::new(HaMatcher)),
    ]
}

// Pattern: なん + counter + か (uncertain number)
// Structures: なん + Counter + か, いく + Counter + か, いくつか
pub fn nan_counter_ka() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for なん (代名詞) or いく (名詞/数) or いくつ (代名詞)
    #[derive(Debug)]
    struct NanOrIkuMatcher;
    impl Matcher for NanOrIkuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match なん (代名詞/一般)
            if token.base_form == "なん"
                && token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "代名詞")
            {
                return true;
            }
            // Match いく (名詞/数)
            if token.base_form == "いく"
                && token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "数")
            {
                return true;
            }
            // Match いくつ (代名詞/一般)
            if token.base_form == "いくつ"
                && token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "代名詞")
            {
                return true;
            }
            false
            })
        }
    }

    // Matcher for counter (助数詞) - optional for いくつか
    #[derive(Debug)]
    struct CounterMatcher;
    impl Matcher for CounterMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.pos.first().is_some_and(|p| p == "名詞")
                && token.pos.get(1).is_some_and(|p| p == "接尾")
                && token.pos.get(2).is_some_and(|p| p == "助数詞")
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NanOrIkuMatcher)),
        optional(TokenMatcher::Custom(Arc::new(CounterMatcher))),
        surface("か"), // Match か particle - simplified since か is very specific
    ]
}

// Pattern: 真(っ) (ma- prefix for "completely")
// Structures: Matches both prefix (真っ + Noun) and compound words (真ん中, 真っ赤, etc.)
pub fn ma() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct MaPatternMatcher;
    impl Matcher for MaPatternMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Pattern 1: 真っ as prefix (接頭詞/名詞接続) - TWO tokens (prefix + noun)
            let is_prefix = token.surface == "真っ"
                && token.base_form == "真っ"
                && token.pos.first().is_some_and(|p| p == "接頭詞")
                && token.pos.get(1).is_some_and(|p| p == "名詞接続");

            // Pattern 2: Compound words starting with 真ん or 真っ - SINGLE token
            // Examples: 真ん中, 真ん丸, 真っ赤, 真っ青, 真っ裸
            let is_compound = (token.surface.starts_with("真ん") || token.surface.starts_with("真っ"))
                && (token.base_form.starts_with("真ん") || token.base_form.starts_with("真っ"))
                && token.pos.first().is_some_and(|p| p == "名詞")
                && token.surface.chars().count() > 2;  // More than just 真ん or 真っ

            is_prefix || is_compound
            })
        }
    }

    vec![TokenMatcher::Custom(Arc::new(MaPatternMatcher))]
}

// Pattern: Number + しか〜ない
// Pattern: Number + しか〜ない - "only (number)" with negative verb
// Structures: Number + しか + Verb[ない]
// Meaning: "only (number)" - しか must be used with negative verbs
// Example: 五キロしか走れない。 (I can only run 5 km.)
pub fn number_shika_u301c_nai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for counters (名詞/接尾/助数詞)
    #[derive(Debug)]
    struct CounterMatcher;
    impl Matcher for CounterMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                && token.pos.get(2).is_some_and(|pos| pos == "助数詞")
            })
        }
    }

    // Matcher for negative auxiliary ない or ん (for ません)
    #[derive(Debug)]
    struct NegativeAuxiliaryMatcher;
    impl Matcher for NegativeAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.pos.first().is_some_and(|pos| pos == "助動詞")
                && ((token.base_form == "ない") || (token.base_form == "ん"))
            })
        }
    }

    vec![
        // Match the counter (which comes after numbers in sequences like 五キロ, １００円)
        // The pattern starts matching from the counter token for simplicity
        TokenMatcher::Custom(Arc::new(CounterMatcher)),
        surface("しか"), // Match しか particle - simplified since しか is very specific
        // Wildcard to allow various verb forms before negative (0-10 tokens)
        wildcard(0, 10, vec![]),
        TokenMatcher::Custom(Arc::new(NegativeAuxiliaryMatcher)),
    ]
}

// Pattern: ～は～の一つだ
// Pattern: ～は～の一つだ (A is one of B)
// Structures: Noun (A) + は + Noun (B) + の + Counter + だ/です
pub fn uff5e_ha_uff5e_nohitotsuda() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match counter nouns: ひとつ, ひとり, ふたり, etc.
    // These are typically 名詞 with surface matching counter patterns
    #[derive(Debug)]
    struct CounterNounMatcher;
    impl Matcher for CounterNounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Common counters that are reliably tokenized as nouns
            const COMMON_COUNTERS: &[&str] = &[
                "ひとつ", "ふたつ", "みっつ", "よっつ", "いつつ",
                "むっつ", "ななつ", "やっつ", "ここのつ", "とお",
                "ひとり", "ふたり",  // people counters
                "一つ", "二つ", "三つ",  // kanji variants
                "一人", "二人",
            ];

            token.pos.first().is_some_and(|p| p == "名詞")
                && (COMMON_COUNTERS.contains(&token.surface.as_str())
                    || COMMON_COUNTERS.contains(&token.base_form.as_str()))
            })
        }
    }

    // Match だ or です auxiliary
    #[derive(Debug)]
    struct DaDesuMatcher;
    impl Matcher for DaDesuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|p| p == "助動詞")
                && (token.base_form == "だ" || token.base_form == "です") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        // Noun (category)
        super::noun(),
        // の particle
        surface("の"),
        // Counter noun
        TokenMatcher::Custom(Arc::new(CounterNounMatcher)),
        // だ or です
        TokenMatcher::Custom(Arc::new(DaDesuMatcher)),
    ]
}

// Pattern: ～ない～はない (double negative - there isn't X that doesn't Y)
// Structures: [Verb/Adj + ない] + Noun + は + [Verb/Adj + ない]
pub fn uff5e_nai_uff5e_hanai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ない (either as auxiliary or adjective)
    #[derive(Debug)]
    struct NaiMatcher;
    impl Matcher for NaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ない"
                && (token.pos.first().is_some_and(|pos| pos == "助動詞")
                    || token.pos.first().is_some_and(|pos| pos == "形容詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match は particle (係助詞)
    #[derive(Debug)]
    struct WaParticleMatcher;
    impl Matcher for WaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "は"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NaiMatcher)),        // First ない
        super::noun(),                             // Noun
        TokenMatcher::Custom(Arc::new(WaParticleMatcher)), // は
        wildcard(0, 2, vec![]), // Optional verb/adjective stem (0-2 tokens)
        TokenMatcher::Custom(Arc::new(NaiMatcher)),        // Second ない
    ]
}

// Pattern: すこしも～ない (not even a little)
// Structure: すこしも
pub fn sukoshimo_uff5e_nai() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct SukoshimoMatcher;
    impl Matcher for SukoshimoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "すこしも"
                && token.base_form == "すこしも"
                && token.pos.first().is_some_and(|pos| pos == "副詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    vec![TokenMatcher::Custom(Arc::new(SukoshimoMatcher))]
}

// Pattern: すくなくない - not few (quite a few, many)
// Structures: 少なく + ない / 少なく + ありません
pub fn sukunakunai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match 少なく (the adjective 少ない in 連用テ接続 form)
    #[derive(Debug)]
    struct SukunakuMatcher;
    impl Matcher for SukunakuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "少なく"
                && token.base_form == "少ない"
                && token.pos.first().is_some_and(|p| p == "形容詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match ない (auxiliary verb)
    #[derive(Debug)]
    struct NaiAuxMatcher;
    impl Matcher for NaiAuxMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ない"
                && token.pos.first().is_some_and(|p| p == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // The pattern can be either:
    // 1. 少なく + ない (casual)
    // 2. 少なく + あり + ませ + ん (polite)
    // We need to use alternatives or check both patterns
    // For simplicity, we'll match the beginning and use optional matchers

    vec![
        TokenMatcher::Custom(Arc::new(SukunakuMatcher)),
        TokenMatcher::Custom(Arc::new(NaiAuxMatcher)),  // Matches casual form
    ]
}

// Pattern: すくなくない (polite) - not few (quite a few, many)
// Structures: 少なく + ありません
pub fn sukunakunai_polite() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match 少なく (the adjective 少ない in 連用テ接続 form)
    #[derive(Debug)]
    struct SukunakuMatcher;
    impl Matcher for SukunakuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "少なく"
                && token.base_form == "少ない"
                && token.pos.first().is_some_and(|p| p == "形容詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match あり (verb ある in 連用形)
    #[derive(Debug)]
    struct AriMatcher;
    impl Matcher for AriMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "あり"
                && token.base_form == "ある"
                && token.pos.first().is_some_and(|p| p == "動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match ませ (auxiliary verb ます in 未然形)
    #[derive(Debug)]
    struct MaseMatcher;
    impl Matcher for MaseMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ませ"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|p| p == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match ん (auxiliary verb, negative)
    #[derive(Debug)]
    struct NMatcher;
    impl Matcher for NMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ん"
                && token.pos.first().is_some_and(|p| p == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(SukunakuMatcher)),
        TokenMatcher::Custom(Arc::new(AriMatcher)),
        TokenMatcher::Custom(Arc::new(MaseMatcher)),
        TokenMatcher::Custom(Arc::new(NMatcher)),
    ]
}

// Pattern: ばあいは (in the event of/in the case of)
// Structures: Verb/い-Adj + 場合(は)
//            な-Adj + な + 場合(は)
//            Noun + の + 場合(は)
pub fn baaiha() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    vec![
        or(vec![verb(), adjective(), noun()]), // Match verb, adjective, or noun
        optional(or(vec![
            // Match な particle (助動詞, base="だ", 体言接続)
            TokenMatcher::Custom(Arc::new({
                #[derive(Debug)]
                struct NaParticleMatcher;
                impl Matcher for NaParticleMatcher {
                    fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
                        check_token(ctx, |token| token.surface == "な"
                            && token.base_form == "だ"
                            && token.pos.first().is_some_and(|pos| pos == "助動詞")
                            && token.features.get(5).is_some_and(|f| f == "体言接続"))
                    }
                }
                NaParticleMatcher
            })),
            // Match の particle (連体化)
            TokenMatcher::Custom(Arc::new({
                #[derive(Debug)]
                struct NoParticleMatcher;
                impl Matcher for NoParticleMatcher {
                    fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
                        check_token(ctx, |token| token.surface == "の"
                            && token.base_form == "の"
                            && token.pos.first().is_some_and(|pos| pos == "助詞")
                            && token.pos.get(1).is_some_and(|pos| pos == "連体化"))
                    }
                }
                NoParticleMatcher
            }))
        ])),
        // Match 場合 (名詞/副詞可能)
        TokenMatcher::Custom(Arc::new({
            #[derive(Debug)]
            struct BaaiMatcher;
            impl Matcher for BaaiMatcher {
                fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
                    check_token(ctx, |token| token.surface == "場合"
                        && token.base_form == "場合"
                        && token.pos.first().is_some_and(|pos| pos == "名詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "副詞可能"))
                }
            }
            BaaiMatcher
        })),
        // Match は topic particle
        optional(TokenMatcher::Custom(Arc::new({
            #[derive(Debug)]
            struct WaParticleMatcher;
            impl Matcher for WaParticleMatcher {
                fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
                    check_token(ctx, |token| token.surface == "は"
                        && token.base_form == "は"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "係助詞"))
                }
            }
            WaParticleMatcher
        }))),
    ]
}

// Pattern: Verb[て] - casual imperative (て at sentence end)
// Structures: Verb[て]。
// Meaning: Shortened form of てください used for friendly requests
// Example: 片付けて。 (Please clean up.)
pub fn verb_te_2() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for て particle (接続助詞) that is NOT followed by auxiliary verbs
    // This ensures we only match sentence-end imperatives, not て-form conjunctions
    #[derive(Debug)]
    struct TeImperativeMatcher;
    impl Matcher for TeImperativeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            // Must be て particle
            if !check_token(ctx, |token| token.surface == "て"
                && token.base_form == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")).0 {
                return (false, 0);
            }

            // て imperative should be sentence-final OR only followed by sentence-ending particles
            // Any other token after て means it's a conjunction, not an imperative
            if let Some(next_token) = ctx.lookahead(1) {
                // Exclude ほしい (てほしい pattern)
                let is_hoshii = next_token.base_form == "ほしい"
                    && next_token.pos.first().is_some_and(|p| p == "形容詞");

                if is_hoshii {
                    return (false, 0);
                }

                // Allow only sentence-ending particles like ね, よ, な, etc.
                let is_sentence_final_particle = next_token.pos.first().is_some_and(|p| p == "助詞")
                    && next_token.pos.get(1).is_some_and(|p| p == "終助詞");

                // Allow punctuation marks
                let is_punctuation = next_token.pos.first().is_some_and(|p| p == "記号");

                // If not one of these allowed cases, this is a conjunction, not imperative
                if !is_sentence_final_particle && !is_punctuation {
                    return (false, 0);
                }
            }

            (true, 1)
        }
    }

    // Matcher for verb, excluding て/で with base てる/でる (tokenizer artifacts)
    #[derive(Debug)]
    struct VerbNotTeruMatcher;
    impl Matcher for VerbNotTeruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            let (is_verb_form, _) = check_token(ctx, |token| token.pos.first().is_some_and(|p| p == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形" || f == "連用タ接続"));

            if !is_verb_form {
                return (false, 0);
            }

            // Exclude て/で with base てる/でる - these are tokenizer artifacts from
            // continuous form repetitions like してて (not valid て-form conjugations)
            check_token(ctx, |token| !((token.surface == "て" || token.surface == "で")
                && (token.base_form == "てる" || token.base_form == "でる")))
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbNotTeruMatcher)),
        TokenMatcher::Custom(Arc::new(TeImperativeMatcher)),
    ]
}

// Pattern: てよかった - "glad that" / "I'm glad that..."
// Structures: Various + て/で + よかった (+ optional です)
// Can attach to verbs, adjectives, nouns via て or で
pub fn teyokatta() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    vec![
        // Match て/で particle OR で copula
        or(vec![
            // て or で particle (after verbs/adjectives/ない)
            TokenMatcher::Custom(Arc::new({
                #[derive(Debug)]
                struct TeDeParticleMatcher;
                impl Matcher for TeDeParticleMatcher {
                    fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
                        check_token(ctx, |token| (token.surface == "て" || token.surface == "で")
                            && token.pos.first().is_some_and(|pos| pos == "助詞")
                            && token.pos.get(1).is_some_and(|pos| pos == "接続助詞"))
                    }
                }
                TeDeParticleMatcher
            })),
            // で copula (after nouns/な-adjectives)
            TokenMatcher::Custom(Arc::new({
                #[derive(Debug)]
                struct DeCopulaMatcher;
                impl Matcher for DeCopulaMatcher {
                    fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
                        check_token(ctx, |token| token.surface == "で"
                            && token.base_form == "だ"
                            && token.pos.first().is_some_and(|pos| pos == "助動詞"))
                    }
                }
                DeCopulaMatcher
            })),
        ]),
        // よかっ (the te-connecting form of よい)
        TokenMatcher::Custom(Arc::new({
            #[derive(Debug)]
            struct YokattaMatcher;
            impl Matcher for YokattaMatcher {
                fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
                    check_token(ctx, |token| token.surface == "よかっ"
                        && token.base_form == "よい"
                        && token.pos.first().is_some_and(|pos| pos == "形容詞"))
                }
            }
            YokattaMatcher
        })),
        // た
        past_auxiliary(),
        // Optional です
        optional(surface("です")),
    ]
}

// Pattern: Verb［せる・させる］(Causative form - make/let someone do)
// Structures: Verb[未然形] + せる/させる
pub fn verb_uff3b_seru_u30fb_saseru_uff3d() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    vec![
        // Verbs in 未然形 (negative/causative stem)
        // This includes all verb types before causative auxiliary せる/させる
        TokenMatcher::Custom(Arc::new({
            #[derive(Debug)]
            struct CausativeStemMatcher;
            impl Matcher for CausativeStemMatcher {
                fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
                    check_token(ctx, |token| token.pos.first().is_some_and(|pos| pos == "動詞")
                        && token.features.get(5).is_some_and(|form| {
                            form == "未然形" || form == "未然レル接続"
                        }))
                }
            }
            CausativeStemMatcher
        })),
        // Causative auxiliary せる/させる as suffix verb
        // Tokenized as 動詞/接尾 with base form せる or させる
        TokenMatcher::Custom(Arc::new({
            #[derive(Debug)]
            struct CausativeAuxiliaryMatcher;
            impl Matcher for CausativeAuxiliaryMatcher {
                fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
                    check_token(ctx, |token| token.pos.first().is_some_and(|pos| pos == "動詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                        && (token.base_form == "せる" || token.base_form == "させる"))
                }
            }
            CausativeAuxiliaryMatcher
        })),
        optional(mashi_form()),
    ]
}

// Pattern: といってもいい
pub fn toittemoii() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ても (even if/even though)
// Structures:
//   - Verb[て] + も / Verb[なくて] + も
//   - い-Adjective[て] + も / い-Adj[なくて] + も
//   - な-Adjective + でも (single token) / な-Adj + じゃなくて + も
//   - Noun + で + も (two tokens) / Noun + じゃなくて + も
pub fn temo() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    use super::Matcher;

    // Custom matcher that handles all ても/でも variations
    // Must match one of:
    // - Verb/Adj + て + も (3 tokens)
    // - Noun + で + も (3 tokens)
    // - Noun + でも (2 tokens, where でも is single particle)
    #[derive(Debug)]
    struct TemoFullMatcher;
    impl Matcher for TemoFullMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            // First token: verb/adj/noun/なく
            let first = match ctx.current() {
                Some(token) => token,
                None => return (false, 0),
            };

            // Check if first token is valid
            let is_valid_first = {
                // Verb in 連用タ接続 or 連用テ接続
                if first.pos.first().is_some_and(|p| p == "動詞") {
                    first.features.get(5).is_some_and(|f| f == "連用タ接続" || f == "連用テ接続" || f == "未然形")
                }
                // い-adjective in 連用テ接続
                else if first.pos.first().is_some_and(|p| p == "形容詞") {
                    first.features.get(5).is_some_and(|f| f == "連用テ接続")
                }
                // なく (negative auxiliary)
                else if first.surface == "なく" && first.base_form == "ない" {
                    first.pos.first().is_some_and(|p| p == "助動詞")
                        && first.features.get(5).is_some_and(|f| f == "連用テ接続")
                }
                // な-adjective or noun
                else if first.pos.first().is_some_and(|p| p == "名詞") {
                    true
                }
                else {
                    false
                }
            };

            if !is_valid_first {
                return (false, 0);
            }

            // Second token: て/で/じゃ/でも
            let second = match ctx.lookahead(1) {
                Some(token) => token,
                None => return (false, 0),
            };

            // Case 1: でも as single particle (副助詞) - only need 2 tokens
            // BUT: Only for な-adjectives, not regular nouns (those should use でも pattern instead)
            if second.surface == "でも" && second.base_form == "でも"
                && second.pos.first().is_some_and(|p| p == "助詞")
                && second.pos.get(1).is_some_and(|p| p == "副助詞")
            {
                // Check if first token is な-adjective (not regular noun)
                let is_na_adj = first.pos.first().is_some_and(|p| p == "名詞")
                    && first.pos.get(1).is_some_and(|p| p == "形容動詞語幹");

                if is_na_adj {
                    return (true, 2);
                }
                // If it's a regular noun + でも, don't match (let でも pattern handle it)
                return (false, 0);
            }

            // Case 2: て/で particle - must be followed by も
            // Note: じゃ is NOT a valid form for ても - it's a contraction of では used in ては pattern
            let is_te_de = {
                // て (conjunction particle)
                if second.surface == "て" {
                    second.pos.first().is_some_and(|p| p == "助詞")
                        && second.pos.get(1).is_some_and(|p| p == "接続助詞")
                }
                // で (case marking particle)
                else if second.surface == "で" {
                    second.pos.first().is_some_and(|p| p == "助詞")
                        && second.pos.get(1).is_some_and(|p| p == "格助詞")
                }
                else {
                    false
                }
            };

            if !is_te_de {
                return (false, 0);
            }

            // Third token: must be も
            let third = match ctx.lookahead(2) {
                Some(token) => token,
                None => return (false, 0),
            };

            // Exclude conjunction でも (but/however) when it appears as で + も
            // This is typically at sentence boundaries where there's no meaningful content before で
            if second.surface == "で" && third.surface == "も" {
                // Check if the first token is actually meaningful content or just a particle/auxiliary
                // The conjunction でも doesn't attach to actual content words
                if first.pos.first().is_some_and(|p| p == "助詞" || p == "助動詞") {
                    return (false, 0);
                }
            }

            if third.surface == "も"
                && third.pos.first().is_some_and(|p| p == "助詞")
                && third.pos.get(1).is_some_and(|p| p == "係助詞")
            {
                return (true, 3);
            }

            (false, 0)
        }
    }

    vec![TokenMatcher::Custom(Arc::new(TemoFullMatcher))]
}

// Pattern: てしまう・ちゃう (completion/regret)
// Structures: Verb[て] + しまう / Verb + ちゃう / Verb + じゃう (+ ます optional)
pub fn teshimau_u30fb_chau() -> Vec<TokenMatcher> {
    use super::Matcher;
    use std::sync::Arc;

    // Match verb in 連用形 or 連用タ接続 (needed before て/ちゃう/じゃう)
    #[derive(Debug)]
    struct VerbRenyouFormMatcher;
    impl Matcher for VerbRenyouFormMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "連用形" || f == "連用タ接続") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match て particle with POS check
    #[derive(Debug)]
    struct TeParticleMatcher;
    impl Matcher for TeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // てしまう pattern: Verb(連用タ接続) + て + しまう (+ Optional ます)
    // OR
    // ちゃう/じゃう pattern: Verb(連用タ接続) + ちゃう/じゃう (+ Optional ます)
    //
    // We need to match both patterns, so we use Optional for て and check for all three verb forms
    vec![
        TokenMatcher::Custom(Arc::new(VerbRenyouFormMatcher)),
        optional(TokenMatcher::Custom(Arc::new(TeParticleMatcher))), // て is optional (for ちゃう/じゃう)
        or(vec![
            verb_base("しまう"),
            verb_base("ちゃう"),
            verb_base("じゃう")
        ]), // Match しまう, ちゃう, or じゃう
        optional(mashi_form()),
    ]
}

// Pattern: Verb[て] + B
// Pattern: Verb[て] + B (Contrastive conjunction)
// Expresses contrast using て-form with equal weight for both clauses
// Structures: Verb[連用形/連用タ接続] + て + (optional comma) + (noun) + は
// Examples:
//   - 姉ちゃんは勉強をして弟はゲームをしている (Sister studies, AND brother plays games)
//   - 妻は買い物に行って、私はごみを捨てに行った (Wife went shopping WHILE I threw trash)
//
// Note: Comma (、) is often used before the contrasting clause but is optional.
// This pattern handles both cases.
pub fn verb_te_b_2() -> Vec<TokenMatcher> {
    // Pattern: Verb[連用形/連用タ接続] + て/で + (optional comma) + (0-3 tokens) + は
    // Note: Using two separate approaches to handle with/without comma
    // The wildcard stops at punctuation, so we need to explicitly include comma as optional
    super::concat(vec![
        vec![
            super::flexible_verb_form(),
            or(vec![
                surface_particle("て", "接続助詞"),
                surface_particle("で", "接続助詞")
            ]),
            optional(surface("、")), // Optional comma
        ],
        vec![wildcard(0, 3, vec![])],
        vec![surface_particle("は", "係助詞")], // は topic/contrast particle
    ])
}

// Pattern: Causative-Passive (to be made to do)
// Structures:
//   Long form: Verb[未然形] + せ/させ + られる (e.g., 食べさせられる, 歩かせられる)
//   Short form: Verb_causative[未然形] + れる (e.g., 飲まされる = 飲ます + れる)
pub fn causative_passive() -> Vec<TokenMatcher> {
    use super::Matcher;
    use std::sync::Arc;

    // Match verb in 未然形 that is either:
    // 1. A regular verb (for long form with explicit causative suffix)
    // 2. A causative verb (base_form ends in ます/す for short form)
    #[derive(Debug)]
    struct MizenFormMatcher;
    impl Matcher for MizenFormMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "動詞") => {
                    // Check conjugation form in features[5]
                    if let Some(conj_form) = token.features.get(5) {
                        if conj_form == "未然形" || conj_form == "未然レル接続" {
                            (true, 1)
                        } else {
                            (false, 0)
                        }
                    } else {
                        (false, 0)
                    }
                },
                _ => (false, 0),
            }
        }
    }

    // Match causative suffix せる/させる in 未然形 (optional for short form)
    #[derive(Debug)]
    struct CausativeSuffixMatcher;
    impl Matcher for CausativeSuffixMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                && (token.base_form == "せる" || token.base_form == "させる")
                && token.features.get(5).is_some_and(|form| form == "未然形") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match passive suffix られる/れる
    // Only match if there's an explicit causative suffix OR a causative verb form before it
    #[derive(Debug)]
    struct PassiveSuffixMatcher;
    impl Matcher for PassiveSuffixMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾")
                && (token.base_form == "られる" || token.base_form == "れる") => {
                    // Check if there's an explicit causative suffix OR short causative verb before this
                    // lookbehind(1) will give us the token immediately before the passive suffix
                    if let Some(prev) = ctx.lookbehind(1) {
                        // Case 1: Long form with explicit causative suffix (せる/させる)
                        if prev.base_form == "せる" || prev.base_form == "させる" {
                            return (true, 1);
                        }

                        // Case 2: Short causative form - verb ending in す/ます that's NOT a natural verb
                        // Examples: 飲ます (causative of 飲む), 食べさす (short causative of 食べる)
                        // Non-examples: 流す, 話す (natural verbs), あしらう (regular verb, not causative)
                        if prev.pos.first().is_some_and(|pos| pos == "動詞")
                            && (prev.base_form.ends_with("す") || prev.base_form.ends_with("ます"))
                            && !NATURAL_SU_VERBS.contains(&prev.base_form.as_str()) {
                            return (true, 1);
                        }
                    }
                    (false, 0)
                },
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(MizenFormMatcher)),
        optional(TokenMatcher::Custom(Arc::new(
            CausativeSuffixMatcher,
        ))),
        TokenMatcher::Custom(Arc::new(PassiveSuffixMatcher)),
    ]
}

// Pattern: Verb[て]・Noun[で] + B (means/method/circumstances)
// Structures: Verb[て] + Phrase / Noun + で + Phrase
//
// This pattern highlights て/で when expressing means, method, or circumstances.
// Note: This overlaps significantly with "Verb + て", "Adjective + て・Noun + で", and "で" patterns.
// Given the overlap and low priority (1), we implement it to match either variant.
pub fn verb_te_u30fb_noun_de_b() -> Vec<TokenMatcher> {
    // Custom matcher that checks for Verb/Noun + て/で + Following content
    // This pattern is for て/で connecting to a following phrase (means/method usage)
    // NOT for standalone て imperatives or sentence-final usage
    #[derive(Debug)]
    struct VerbTeNounDeBMatcher;
    impl Matcher for VerbTeNounDeBMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            // First token: verb or noun
            let first = match ctx.current() {
                Some(token) => token,
                None => return (false, 0),
            };

            let is_verb = first.pos.first().is_some_and(|pos| pos == "動詞")
                && (first.features.get(5).is_some_and(|form| form == "連用形")
                    || first.features.get(5).is_some_and(|form| form == "連用タ接続"));

            // Exclude て/で with base てる/でる - these are tokenizer artifacts from
            // continuous form repetitions like してて (not valid て-form conjugations)
            if is_verb && (first.surface == "て" || first.surface == "で")
                && (first.base_form == "てる" || first.base_form == "でる") {
                return (false, 0);
            }

            let is_noun = first.pos.first().is_some_and(|pos| pos == "名詞");

            if !is_verb && !is_noun {
                return (false, 0);
            }

            // Second token: て/で particle
            let second = match ctx.lookahead(1) {
                Some(token) => token,
                None => return (false, 0),
            };

            // Match て or で as conjunction particle (after verbs)
            let is_conjunction = (second.surface == "て" || second.surface == "で")
                && second.pos.first().is_some_and(|pos| pos == "助詞")
                && second.pos.get(1).is_some_and(|pos| pos == "接続助詞");

            // Match で as case particle (after nouns - means/method)
            // Exclude collective nouns that use で for manner/means without clause connection
            let is_case_particle = second.surface == "で"
                && second.pos.first().is_some_and(|pos| pos == "助詞")
                && second.pos.get(1).is_some_and(|pos| pos == "格助詞");

            if is_case_particle && is_noun {
                // Exclude で after collective nouns (全員で, みんなで, etc.)
                // These use で for manner/means, not for connecting clauses
                let collective_nouns = ["全員", "みんな", "皆", "全部", "全て", "すべて",
                                       "一同", "一緒", "共", "二人", "三人", "四人", "五人"];
                if collective_nouns.contains(&first.surface.as_str())
                    || collective_nouns.contains(&first.base_form.as_str()) {
                    return (false, 0);
                }
            }

            if !is_conjunction && !is_case_particle {
                return (false, 0);
            }

            // Check that there's actually content following (not punctuation, not end)
            // This distinguishes connecting usage from standalone て imperatives
            match ctx.lookahead(2) {
                Some(token) => {
                    // Don't match if followed by punctuation or sentence-ending particles
                    let is_punctuation = token.pos.first().is_some_and(|pos| pos == "記号");
                    let is_sentence_end = token.surface == "。" || token.surface == "！" || token.surface == "？" || token.surface == "…";

                    // Also don't match if followed by another て-form verb (likely repeated command)
                    let is_te_form = token.pos.first().is_some_and(|pos| pos == "動詞")
                        && token.features.get(5).is_some_and(|f| f == "連用タ接続");

                    // Don't match if followed by auxiliary verbs (part of ている, てくれる, etc.)
                    let auxiliary_verbs = ["いる", "くれる", "もらう", "あげる", "みる", "おく", "しまう", "ある"];
                    let is_auxiliary = token.pos.first().is_some_and(|p| p == "動詞")
                        && auxiliary_verbs.contains(&token.base_form.as_str());

                    // Don't match if followed by colloquial negatives (part of ている in casual speech)
                    let is_colloquial_neg = token.pos.first().is_some_and(|p| p == "助動詞")
                        && (token.surface == "ねえ" || token.surface == "ねー");

                    // Don't match if followed by another て particle (part of ていて contracted to ってて)
                    let is_te_particle = token.surface == "て"
                        && token.pos.first().is_some_and(|p| p == "助詞");

                    // Don't match if followed by ばかり (part of てばかり construction)
                    let is_bakari = token.surface == "ばかり"
                        && token.pos.first().is_some_and(|p| p == "助詞");

                    if is_punctuation || is_sentence_end || is_te_form || is_auxiliary || is_colloquial_neg || is_te_particle || is_bakari {
                        return (false, 0);
                    }

                    // There is following content - this is the connecting usage
                    (true, 2)
                },
                None => (false, 0), // No following content - not the connecting usage
            }
        }
    }

    vec![TokenMatcher::Custom(Arc::new(VerbTeNounDeBMatcher))]
}

// Pattern: てある (state of completion / left in state)
// Structures: (Transitive) Verb[て] + ある / (Transitive) Verb[て] + あります
pub fn tearu() -> Vec<TokenMatcher> {
    use super::concat;

    concat(vec![
        vec![super::flexible_verb_form()],
        vec![or(vec![surface("て"), surface("で")])], // て or で particle
        vec![verb_base("ある")], // ある as auxiliary verb
        vec![optional(mashi_form())], // Optional ます
    ])
}

// Pattern: ように～てほしい (want someone to do in order to)
// Structures:
//   - Verb/Adj/Noun + (の) + ように + ... + Verb[て] + ほしい
//
// Note: This pattern combines ように with てほしい to express "want someone to do
// something in order to achieve X" or "want someone to do something like X".
pub fn youni_uff5e_tehoshii() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct YouMatcher;
    impl Matcher for YouMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| token.surface == "よう"
                && token.base_form == "よう"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                && token.pos.get(2).is_some_and(|pos| pos == "助動詞語幹"))
        }
    }

    #[derive(Debug)]
    struct HoshiiMatcher;
    impl Matcher for HoshiiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| token.base_form == "ほしい"
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立"))
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(YouMatcher)),
        or(vec![
            surface_particle("に", "格助詞"),
            surface_particle("に", "副詞化")
        ]),
        wildcard(0, 10, vec![]), // Allow up to 10 tokens between ように and Verb[て]
        flexible_verb_form(),
        or(vec![
            surface_particle("て", "接続助詞"),
            surface_particle("で", "接続助詞")
        ]),
        TokenMatcher::Custom(Arc::new(HoshiiMatcher)),
    ]
}

// Pattern: ているあいだに (while/during)
// Structures: Verb[ている] + 間（あいだ）に
pub fn teiruaidani() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct TeDeParticleMatcher;
    impl Matcher for TeDeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞"))
        }
    }

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| token.surface == "に"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞"))
        }
    }

    concat(vec![
        vec![flexible_verb_form()],
        vec![TokenMatcher::Custom(Arc::new(TeDeParticleMatcher))],
        vec![verb_base("いる")],
        vec![surface("あいだ")],
        vec![TokenMatcher::Custom(Arc::new(NiParticleMatcher))],
    ])
}

// Pattern: なくてもいい (don't have to / it's okay not to)
// Structures: Verb[なくて] + (も) + いい (+ です)
pub fn nakutemoii() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct NakuAuxiliaryMatcher;
    impl Matcher for NakuAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| token.surface == "なく"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|form| form == "連用テ接続"))
        }
    }

    #[derive(Debug)]
    struct TeParticleMatcher;
    impl Matcher for TeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞"))
        }
    }

    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞"))
        }
    }

    vec![
        any(), // Verb in 未然形 (before なく)
        TokenMatcher::Custom(Arc::new(NakuAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
        optional(TokenMatcher::Custom(Arc::new(MoParticleMatcher))),
        ii_form(),
    ]
}

// Pattern: てみる (try doing)
// Structures: Verb[て] + みる
pub fn temiru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct TeDeFormMatcher;
    impl Matcher for TeDeFormMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞"))
        }
    }

    #[derive(Debug)]
    struct MiruMatcher;
    impl Matcher for MiruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| token.base_form == "みる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立"))
        }
    }

    vec![
        flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(TeDeFormMatcher)),
        TokenMatcher::Custom(Arc::new(MiruMatcher)),
    ]
}

// Pattern: てすみません (sorry for doing)
// Structures: Verb[て] + すみません
pub fn tesumimasen() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct TeDeParticleMatcher;
    impl Matcher for TeDeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞"))
        }
    }

    #[derive(Debug)]
    struct SumimasenMatcher;
    impl Matcher for SumimasenMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| token.surface == "すみません"
                && token.base_form == "すみません"
                && token.pos.first().is_some_and(|p| p == "感動詞"))
        }
    }

    #[derive(Debug)]
    struct DesuAuxMatcher;
    impl Matcher for DesuAuxMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| token.base_form == "です"
                && token.pos.first().is_some_and(|p| p == "助動詞"))
        }
    }

    concat(vec![
        vec![flexible_verb_form()],
        vec![TokenMatcher::Custom(Arc::new(TeDeParticleMatcher))],
        vec![TokenMatcher::Custom(Arc::new(SumimasenMatcher))],
        vec![optional(TokenMatcher::Custom(Arc::new(DesuAuxMatcher)))],
        vec![optional(past_auxiliary())],
    ])
}

// Pattern: てあげる (to do for someone)
// Structures: Verb[て] + あげる/あげます
pub fn teageru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match て or で particle - need custom logic for te/de forms with POS check
    #[derive(Debug)]
    struct TeDeFormMatcher;
    impl Matcher for TeDeFormMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    concat(vec![
        vec![flexible_verb_form()],
        vec![TokenMatcher::Custom(Arc::new(TeDeFormMatcher))],
        vec![verb_base("あげる")],
    ])
}

// Pattern: てくれる
pub fn tekureru() -> Vec<TokenMatcher> {
    use super::concat;

    // Custom verb form matcher for て-form and ないで-form
    #[derive(Debug)]
    struct VerbFormMatcher;
    impl Matcher for VerbFormMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                if token.pos.first().is_none_or(|pos| pos != "動詞") {
                false
            } else {
                let form = token.features.get(5);
                // Match 連用形, 連用タ接続 (for て-form), or 未然形 (for ないで-form)
                form.is_some_and(|f| f == "連用形" || f == "連用タ接続" || f == "未然形")
            }
            })
        }
    }

    // Custom matcher for ない auxiliary verb
    #[derive(Debug)]
    struct NaiAuxiliaryMatcher;
    impl Matcher for NaiAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Custom matcher for て/で particles
    #[derive(Debug)]
    struct TeDeFormMatcher;
    impl Matcher for TeDeFormMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match both て-form and ないで-form
    // て-form: Verb(連用形/連用タ接続) + て/で + くれる
    // ないで-form: Verb(未然形) + ない + で + くれる
    concat(vec![
        vec![TokenMatcher::Custom(Arc::new(VerbFormMatcher))],
        vec![optional(TokenMatcher::Custom(Arc::new(NaiAuxiliaryMatcher)))],
        vec![TokenMatcher::Custom(Arc::new(TeDeFormMatcher))],
        vec![verb_base("くれる")],
    ])
}

// Pattern: てもらう
pub fn temorau() -> Vec<TokenMatcher> {
    use super::concat;

    // Match て or で particle
    #[derive(Debug)]
    struct TeDeFormMatcher;
    impl Matcher for TeDeFormMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    concat(vec![
        vec![super::flexible_verb_form()],
        vec![TokenMatcher::Custom(Arc::new(TeDeFormMatcher))],
        vec![verb_base("もらう")],
    ])
}

// Pattern: なさい (imperative command)
// Structures: Verb[連用形] + なさい
pub fn nasai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Custom matcher for verb stem before なさい
    // Excludes とく/おく which are part of ておく pattern
    #[derive(Debug)]
    struct VerbStemBeforeNasaiMatcher;
    impl Matcher for VerbStemBeforeNasaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|form| form == "連用形")
                // Exclude とく/おく which are part of ておく construction
                && token.base_form != "とく"
                && token.base_form != "おく" => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct NasaiMatcher;
    impl Matcher for NasaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "なさい"
                && token.base_form == "なさる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                && token.features.get(5).is_some_and(|form| form == "命令ｉ") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbStemBeforeNasaiMatcher)),
        TokenMatcher::Custom(Arc::new(NasaiMatcher)),
    ]
}

// Pattern: Verb[ないで]
// Pattern: Verb[ないで] (without doing)
// Structure: Verb[未然形] + ない + で
pub fn verb_naide() -> Vec<TokenMatcher> {
    naide()  // Same implementation as ないで
}

// Pattern: てくれてありがとう (thank you for doing)
// Structures: Verb[て] + くれて + ありがとう(+ ございます)
pub fn tekuretearigatou() -> Vec<TokenMatcher> {
    use super::concat;

    // Custom matcher for ありがとう as interjection
    #[derive(Debug)]
    struct ArigatouMatcher;
    impl Matcher for ArigatouMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ありがとう"
                && token.pos.first().is_some_and(|pos| pos == "感動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Custom matcher for ござい (from ござる auxiliary)
    #[derive(Debug)]
    struct GozaiMatcher;
    impl Matcher for GozaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.base_form == "ござる"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Pattern structure: Verb(連用形/連用タ接続) + て + くれ + て + ありがとう + optional(ござい + ます)
    concat(vec![
        // Match verb in 連用形 or 連用タ接続 (the main verb before て)
        vec![or(vec![verb_form("連用形"), verb_form("連用タ接続")])],
        // Match て particle (connects verb to くれる)
        vec![surface("て")],
        // Match くれる verb in 連用形 (くれ)
        vec![verb_base("くれる")],
        // Match て particle again
        vec![surface("て")],
        // Match ありがとう as interjection
        vec![TokenMatcher::Custom(Arc::new(ArigatouMatcher))],
        // Match optional ござい (from ござる auxiliary)
        vec![optional(TokenMatcher::Custom(Arc::new(GozaiMatcher)))],
        // Match optional ます auxiliary
        vec![optional(mashi_form())],
    ])
}

// Pattern: てくれない・てもらえない (won't you do for me?)
// Structures: Verb[て/ないで] + くれない(か)/くれません(か)/もらえない(か)/もらえません(か)
pub fn tekurenai_u30fb_temoraenai() -> Vec<TokenMatcher> {
    use super::concat;

    // Custom matcher for ない auxiliary verb
    #[derive(Debug)]
    struct NaiAuxMatcher;
    impl Matcher for NaiAuxMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Custom matcher for て/で particles
    #[derive(Debug)]
    struct TeDeParticleMatcher;
    impl Matcher for TeDeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Custom matcher for ない or ませ auxiliary verbs
    #[derive(Debug)]
    struct NaiMaseMatcher;
    impl Matcher for NaiMaseMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "ない" && token.base_form == "ない" && token.pos.first().is_some_and(|pos| pos == "助動詞"))
                || (token.surface == "ませ" && token.base_form == "ます" && token.pos.first().is_some_and(|pos| pos == "助動詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Custom matcher for ん auxiliary verb
    #[derive(Debug)]
    struct NMatcher;
    impl Matcher for NMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ん"
                && token.base_form == "ん"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Pattern: Verb + (ない) + て/で + くれる/もらえる + ない/ません
    concat(vec![
        // Match verb in 連用形, 連用タ接続 (for て-form), or 未然形 (for ないで-form)
        vec![or(vec![verb_form("連用形"), verb_form("連用タ接続"), verb_form("未然形")])],
        // Match optional ない auxiliary (used in ないで construction)
        vec![optional(TokenMatcher::Custom(Arc::new(NaiAuxMatcher)))],
        // Match て or で particle
        vec![TokenMatcher::Custom(Arc::new(TeDeParticleMatcher))],
        // Match くれる or もらえる
        vec![or(vec![verb_base("くれる"), verb_base("もらえる")])],
        // Match ない (negative) or ませ (polite negative)
        vec![TokenMatcher::Custom(Arc::new(NaiMaseMatcher))],
        // Match optional ん (polite negative contraction)
        vec![optional(TokenMatcher::Custom(Arc::new(NMatcher)))],
    ])
}

// Pattern: ～のだろうか (I wonder if...)
// Structures: (の/ん/なの/なん) + だろうか/でしょうか
pub fn uff5e_nodarouka() -> Vec<TokenMatcher> {
    vec![
        // Optional な (for な-adj/noun) - keep custom for specific auxiliary checking
        optional(TokenMatcher::Custom(Arc::new({
            #[derive(Debug)]
            struct NaAuxiliary;
            impl Matcher for NaAuxiliary {
                fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
                    match ctx.current() {
                        Some(token) if token.surface == "な"
                        && token.base_form == "だ"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                        _ => (false, 0),
                    }
                }
            }
            NaAuxiliary
        }))),
        // Optional の/ん (nominalizer) - keep custom for specific noun subtype checking
        optional(TokenMatcher::Custom(Arc::new({
            #[derive(Debug)]
            struct NoNNominalizer;
            impl Matcher for NoNNominalizer {
                fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
                    match ctx.current() {
                        Some(token) if (token.surface == "の" || token.surface == "ん")
                        && token.pos.first().is_some_and(|pos| pos == "名詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                        _ => (false, 0),
                    }
                }
            }
            NoNNominalizer
        }))),
        // だろ or でしょ - keep custom for specific base_form checking
        TokenMatcher::Custom(Arc::new({
            #[derive(Debug)]
            struct DarouDeshouAux;
            impl Matcher for DarouDeshouAux {
                fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
                    match ctx.current() {
                        Some(token) if (token.surface == "だろ" && token.base_form == "だ"
                        || token.surface == "でしょ" && token.base_form == "です")
                        && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                        _ => (false, 0),
                    }
                }
            }
            DarouDeshouAux
        })),
        // う - keep custom for auxiliary checking
        TokenMatcher::Custom(Arc::new({
            #[derive(Debug)]
            struct UAuxiliary;
            impl Matcher for UAuxiliary {
                fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
                    match ctx.current() {
                        Some(token) if token.surface == "う"
                        && token.base_form == "う"
                        && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                        _ => (false, 0),
                    }
                }
            }
            UAuxiliary
        })),
        // か - keep custom for general particle checking
        TokenMatcher::Custom(Arc::new({
            #[derive(Debug)]
            struct KaParticle;
            impl Matcher for KaParticle {
                fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
                    match ctx.current() {
                        Some(token) if token.surface == "か" && token.pos.first().is_some_and(|pos| pos == "助詞") => (true, 1),
                        _ => (false, 0),
                    }
                }
            }
            KaParticle
        })),
    ]
}

// Pattern: お～になる (honorific speech)
// Structures: (お/ご) + Noun + に + なる + (ます)
pub fn o_uff5e_ninaru() -> Vec<TokenMatcher> {
    vec![
        // Complex honorific pattern matcher - keep custom due to lookahead logic
        TokenMatcher::Custom(Arc::new({
            #[derive(Debug)]
            struct HonorificNounPatternMatcher;
            impl Matcher for HonorificNounPatternMatcher {
                fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
                    // Special honorific nouns that don't need prefix (already contain honorific お)
                    const HONORIFIC_NOUNS: &[&str] = &["おいで", "ご存知", "存知"];

                    let current = match ctx.current() {
                        Some(t) => t,
                        None => return (false, 0),
                    };

                    // Check if current token is a special honorific noun
                    if current.pos.first().is_some_and(|pos| pos == "名詞")
                        && HONORIFIC_NOUNS.contains(&current.surface.as_str()) {
                        return (true, 1);
                    }

                    // Check for お/ご prefix pattern: prefix + noun
                    if (current.surface == "お" || current.surface == "ご")
                        && current.base_form == current.surface
                        && current.pos.first().is_some_and(|pos| pos == "接頭詞")
                        && current.pos.get(1).is_some_and(|pos| pos == "名詞接続") {
                        // Check if next token is a noun
                        if let Some(next) = ctx.lookahead(1) {
                            if next.pos.first().is_some_and(|pos| pos == "名詞") {
                                return (true, 2); // Consume prefix + noun
                            }
                        }
                    }

                    (false, 0)
                }
            }
            HonorificNounPatternMatcher
        })),
        // に particle
        surface_particle("に", "格助詞"),
        // なる verb
        verb_base("なる"),
        // Optional ます
        optional(mashi_form()),
    ]
}

// Pattern: なさる (honorific verb - respects actions of others)
// Structures: する → なさる, Noun + する → Noun + なさる
// Conjugates like 五段・ラ行特殊: なさる/なさらない/なさった/なさいます
pub fn nasaru() -> Vec<TokenMatcher> {
    vec![
        // Match なさる verb with specific conjugation type (五段・ラ行特殊)
        TokenMatcher::Custom(Arc::new({
            #[derive(Debug)]
            struct NasaruMatcher;
            impl Matcher for NasaruMatcher {
                fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
                    match ctx.current() {
                        Some(token) if token.base_form == "なさる"
                        && token.pos.first().is_some_and(|pos| pos == "動詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "自立")
                        && token.features.get(4).is_some_and(|f| f == "五段・ラ行特殊") => (true, 1),
                        _ => (false, 0),
                    }
                }
            }
            NasaruMatcher
        }))
    ]
}

// Pattern: お～ください (honorific request)
// Structures: お + Verb[連用形] + ください
pub fn o_uff5e_kudasai() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct OPrefixMatcher;
    impl Matcher for OPrefixMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "お"
                && token.base_form == "お"
                && token.pos.first().is_some_and(|pos| pos == "接頭詞")
                && token.pos.get(1).is_some_and(|pos| pos == "名詞接続") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct KudasaiMatcher;
    impl Matcher for KudasaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ください"
                && token.base_form == "くださる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(OPrefixMatcher)),
        flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(KudasaiMatcher)),
    ]
}

// Pattern: いらっしゃる (honorific - to be/come/go)
// Structures:
//   1. いらっしゃる standalone (replacing いる/くる/いく)
//   2. Verb[て] + いらっしゃる (as auxiliary verb)
pub fn irassharu() -> Vec<TokenMatcher> {
    // Pattern: いらっしゃる + (Optional ます) + (Optional た)
    // Handles both standalone usage and as auxiliary verb after て-form
    vec![
        verb_base("いらっしゃる"),
        optional(mashi_form()),
        optional(past_auxiliary()),
    ]
}

// Pattern: ございます (polite form of ある)
// Structures: ござる (historical) / ござい + ます (modern polite)
pub fn gozaimasu() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct GozaruMatcher;
    impl Matcher for GozaruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.base_form == "ござる"
                && (token.pos.first().is_some_and(|pos| pos == "助動詞")
                    || token.pos.first().is_some_and(|pos| pos == "動詞")) => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct MasuMatcher;
    impl Matcher for MasuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ます"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(GozaruMatcher)),
        optional(TokenMatcher::Custom(Arc::new(MasuMatcher))),
    ]
}

// Pattern: でございます (more polite than です)
// Structures: で + ござい + ます / で + ござる (historical)
pub fn degozaimasu() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct DeMatcher;
    impl Matcher for DeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "で"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct GozaiGozaruMatcher;
    impl Matcher for GozaiGozaruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "ござい" || token.surface == "ござる")
                && token.base_form == "ござる"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct MasuMatcher;
    impl Matcher for MasuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ます"
                && token.base_form == "ます"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(DeMatcher)),
        TokenMatcher::Custom(Arc::new(GozaiGozaruMatcher)),
        optional(TokenMatcher::Custom(Arc::new(MasuMatcher))),
    ]
}

// Pattern: お〜する
// Pattern: お〜する (humble speech - お/ご + Noun + する)
// Structures:
//   1. お/ご (接頭詞) + Noun[サ変接続] + する (split form like ご確認します)
//   2. Noun[サ変接続 starting with お/ご] + する (compound form like お守りします)
pub fn o_u301c_suru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match お or ご prefix
    #[derive(Debug)]
    struct OGoPrefixMatcher;
    impl Matcher for OGoPrefixMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "お" || token.surface == "ご")
                && token.pos.first().is_some_and(|pos| pos == "接頭詞")
                && token.pos.get(1).is_some_and(|pos| pos == "名詞接続") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match ANY サ変接続 noun
    #[derive(Debug)]
    struct SahenNounMatcher;
    impl Matcher for SahenNounMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "サ変接続") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Pattern: お/ご (prefix) + Noun[サ変] + する + (optional ます)
    // Matches split form: ご + 確認 + します
    // Note: Does NOT match compound forms like お守りします (where お守り is a single token)
    // Priority is set low (1) to avoid over-matching plain サ変 verbs

    vec![
        TokenMatcher::Custom(Arc::new(OGoPrefixMatcher)),  // REQUIRED prefix
        TokenMatcher::Custom(Arc::new(SahenNounMatcher)),
        verb_base("する"),
        optional(mashi_form()),
    ]
}

// Pattern: いたす (humble speech - to do)
// Structures:
//   1. Noun[サ変接続] + いたす (する → いたす)
//   2. お + Verb[連用形] + いたす
//   3. ご + Noun[サ変接続] + いたす
pub fn itasu() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match お or ご prefix
    #[derive(Debug)]
    struct OGoPrefixMatcher;
    impl Matcher for OGoPrefixMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "お" || token.surface == "ご")
                && token.pos.first().is_some_and(|pos| pos == "接頭詞")
                && token.pos.get(1).is_some_and(|pos| pos == "名詞接続") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match いたす verb (non-independent: 非自立)

    // Pattern: (Optional お/ご) + (Verb[連用形] OR Noun[サ変接続]) + いたす + (Optional ます)
    vec![
        optional(TokenMatcher::Custom(Arc::new(
            OGoPrefixMatcher,
        ))),
        TokenMatcher::Custom(Arc::new(VerbStemOrSahenNounMatcher)),
        verb_base("いたす"),
        optional(mashi_form()),
    ]
}

// Helper matcher: Verb[連用形] OR Noun[サ変接続]
#[derive(Debug)]
struct VerbStemOrSahenNounMatcher;
impl Matcher for VerbStemOrSahenNounMatcher {
    fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
        match ctx.current() {
            Some(token) => {
                // Verb in 連用形
                let is_verb_stem = token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token.features.get(5).is_some_and(|f| f == "連用形");

                // Noun with サ変接続
                let is_sahen_noun = token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "サ変接続");

                if is_verb_stem || is_sahen_noun {
                    (true, 1)
                } else {
                    (false, 0)
                }
            }
            _ => (false, 0),
        }
    }
}

// Pattern: ていただけませんか (could you please - humble polite request)
// Structures: Verb[て] + いただけませんか / Verb[て] + もらえませんか
//
// Note: Kagome tokenizes いただけませんか incorrectly as:
//   い(いる) + た + だけ + ませんか
// Instead of the correct:
//   いただけ(potential of いただく) + ませんか
//
// We match both the incorrect tokenization and the correct もらえませんか form
pub fn teitadakemasenka() -> Vec<TokenMatcher> {
    use super::Matcher;
    use std::sync::Arc;

    // Match either い (from いただけませんか) or もらえ (from もらえませんか)
    #[derive(Debug)]
    struct ItadakeMoraeMatcher;
    impl Matcher for ItadakeMoraeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) => {
                    // Match い (tokenized as いる verb)
                    if token.surface == "い"
                        && token.pos.first().is_some_and(|pos| pos == "動詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                        && token.base_form == "いる"
                    {
                        return (true, 1);
                    }
                    // Match もらえる
                    if token.pos.first().is_some_and(|pos| pos == "動詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                        && token.base_form == "もらえる"
                    {
                        return (true, 1);
                    }
                    (false, 0)
                }
                _ => (false, 0),
            }
        }
    }


    // Match ませ (未然形 of ます)
    #[derive(Debug)]
    struct MaseAuxiliaryMatcher;
    impl Matcher for MaseAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ませ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "ます" => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match ん (negative auxiliary)
    #[derive(Debug)]
    struct NNegativeMatcher;
    impl Matcher for NNegativeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ん"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }


    // Match て or で particle
    #[derive(Debug)]
    struct TeDeParticleMatcher;
    impl Matcher for TeDeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "て" || token.surface == "で")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match か (question particle)
    #[derive(Debug)]
    struct KaQuestionMatcher;
    impl Matcher for KaQuestionMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "か"
                && token.pos.first().is_some_and(|pos| pos == "助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Pattern: Verb[連用形/連用タ接続] + て/で + (い|もらえ) + [た] + [だけ] + ませ + ん + か
    // The た and だけ are only present in いただけませんか (mis-tokenized)
    vec![
        super::flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(TeDeParticleMatcher)),
        TokenMatcher::Custom(Arc::new(ItadakeMoraeMatcher)),
        optional(past_auxiliary()),
        optional(surface_particle("だけ", "副助詞")),
        TokenMatcher::Custom(Arc::new(MaseAuxiliaryMatcher)),
        TokenMatcher::Custom(Arc::new(NNegativeMatcher)),
        TokenMatcher::Custom(Arc::new(KaQuestionMatcher)),
    ]
}

// Pattern: たら (conditional "if/when")
// Structures: Verb[た] + ら / い-Adj[た] + ら / な-Adj/Noun + だった + ら
pub fn tara() -> Vec<TokenMatcher> {
    use super::Matcher;
    use std::sync::Arc;

    // Match たら (仮定形 of た auxiliary)
    #[derive(Debug)]
    struct TaraAuxiliaryMatcher;
    impl Matcher for TaraAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "たら"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "た"
                && token.features.get(5).is_some_and(|f| f == "仮定形") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match だっ (連用タ接続 of だ auxiliary) - for な-Adj and Nouns
    #[derive(Debug)]
    struct DattaMatcher;
    impl Matcher for DattaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "だっ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "だ"
                && token.features.get(5).is_some_and(|f| f == "連用タ接続") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // たら can follow:
    // 1. Verb (連用形/連用タ接続) + たら
    // 2. い-Adj (連用タ接続) + たら
    // 3. な-Adj/Noun + だっ + たら
    vec![
        any(), // Verb, い-Adj, or な-Adj/Noun
        optional(TokenMatcher::Custom(Arc::new(DattaMatcher))), // Optional だっ for pattern 3
        TokenMatcher::Custom(Arc::new(TaraAuxiliaryMatcher)),
    ]
}

// Pattern: ほかに(も)・ほか(に)は (other than, besides, anything else)
// Structures: ほか + の/に/にも/には/にも
pub fn hokani_mo_u30fb_hoka_ni_ha() -> Vec<TokenMatcher> {
    use super::Matcher;
    use std::sync::Arc;

    // Match ほか as 名詞/副詞可能
    #[derive(Debug)]
    struct HokaMatcher;
    impl Matcher for HokaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ほか"
                && token.base_form == "ほか"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副詞可能") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match の or に particle
    #[derive(Debug)]
    struct NoOrNiParticleMatcher;
    impl Matcher for NoOrNiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) => {
                    // の (助詞/連体化) or に (助詞/格助詞)
                    if token.surface == "の" {
                        if token.pos.first().is_some_and(|pos| pos == "助詞")
                            && token.pos.get(1).is_some_and(|pos| pos == "連体化") {
                            (true, 1)
                        } else {
                            (false, 0)
                        }
                    } else if token.surface == "に" {
                        if token.pos.first().is_some_and(|pos| pos == "助詞")
                            && token.pos.get(1).is_some_and(|pos| pos == "格助詞") {
                            (true, 1)
                        } else {
                            (false, 0)
                        }
                    } else {
                        (false, 0)
                    }
                }
                _ => (false, 0),
            }
        }
    }

    // Match も or は particle (optional)
    #[derive(Debug)]
    struct MoOrHaParticleMatcher;
    impl Matcher for MoOrHaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "も" || token.surface == "は")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(HokaMatcher)),
        TokenMatcher::Custom(Arc::new(NoOrNiParticleMatcher)),
        optional(TokenMatcher::Custom(Arc::new(
            MoOrHaParticleMatcher,
        ))),
    ]
}

// Pattern: がひつよう (is necessary)
// Structure: が + ひつ + よう (+ だ/です optional)
pub fn gahitsuyou() -> Vec<TokenMatcher> {
    use super::Matcher;
    use std::sync::Arc;

    #[derive(Debug)]
    struct HitsuMatcher;
    impl Matcher for HitsuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ひつ"
                && token.base_form == "ひつ"
                && token.pos.first().is_some_and(|pos| pos == "名詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct YouSuffixMatcher;
    impl Matcher for YouSuffixMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "よう"
                && token.base_form == "よう"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接尾") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        surface_particle("が", "格助詞"),
        TokenMatcher::Custom(Arc::new(HitsuMatcher)),
        TokenMatcher::Custom(Arc::new(YouSuffixMatcher)),
    ]
}

// Pattern: そんなに
// Pattern: そんなに (that much/so much - demonstrative adverb)
// Structures: そんなに + Verb/Adjective
pub fn sonnani() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct SonnaniMatcher;
    impl Matcher for SonnaniMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "そんなに"
                && token.base_form == "そんなに"
                && token.pos.first().is_some_and(|p| p == "副詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![TokenMatcher::Custom(Arc::new(SonnaniMatcher))]
}

// Pattern: ひつようがある (need to, necessary to)
// Structures: Verb + 必要 + が + ある/ない/あります/ありません
pub fn hitsuyougaaru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match 必要 (need/necessity)
    #[derive(Debug)]
    struct HitsuyouMatcher;
    impl Matcher for HitsuyouMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "必要"
                && token.base_form == "必要"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "形容動詞語幹")
            })
        }
    }


    // Match ある (verb) or ない (adjective for negative)
    #[derive(Debug)]
    struct AruOrNaiMatcher;
    impl Matcher for AruOrNaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match ある verb and its conjugations (ある, あり, ありません, etc.)
                (token.base_form == "ある" && token.pos.first().is_some_and(|pos| pos == "動詞"))
                    // OR match ない adjective for negative
                    || (token.surface == "ない"
                        && token.base_form == "ない"
                        && token.pos.first().is_some_and(|pos| pos == "形容詞"))
            })
        }
    }

    vec![
        // Match verb in dictionary form (基本形)
        verb_form("基本形"),
        TokenMatcher::Custom(Arc::new(HitsuyouMatcher)),
        surface_particle("が", "格助詞"),
        TokenMatcher::Custom(Arc::new(AruOrNaiMatcher)),
    ]
}

// Pattern: たとえば (for example)
// Structure: たとえば + Phrase
pub fn tatoeba() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct TatoebaMatcher;
    impl Matcher for TatoebaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "たとえば"
                && token.base_form == "たとえば"
                && token.pos.first().is_some_and(|pos| pos == "接続詞")
            })
        }
    }

    vec![TokenMatcher::Custom(Arc::new(TatoebaMatcher))]
}

// Pattern: れる・られる (Potential) - ability/possibility
// Structures:
//   - Godan potential verbs: Single token (歩ける, 話せる, 飛べる, etc.)
//   - Godan potential 2-token: Verb(未然形・五段) + える(動詞/接尾)
//   - Ichidan + られる: Verb(未然形・一段) + られる(動詞/接尾)
//   - できる: Exception for する verbs (included here)
//   - ら抜き: 見れる (casual, sometimes considered incorrect)
//
// Note: Ichidan + られる is structurally identical to passive form.
// Both patterns will detect it, which is semantically correct (ambiguous without context).
pub fn reru_u30fb_rareru_potential() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for potential verbs (both godan single-token and ら抜き forms)
    // These are single tokens with base_form ending in える/ける/せる/てる/ねる/べる/める/げる/れる
    // Examples: 歩ける (歩く → 歩ける), 話せる (話す → 話せる), 見れる (見る → 見れる - ら抜き), できる
    // Note: Excludes natural れる verbs (くれる, 入れる, etc.) that aren't potential forms
    #[derive(Debug)]
    struct PotentialVerbMatcher;
    impl Matcher for PotentialVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Must be a verb
                if !token.pos.first().is_some_and(|pos| pos == "動詞") {
                    return false;
                }

                // Exclude imperative forms - these are not potential forms
                // Imperative forms may superficially look like potential (e.g., くれ from くれる)
                if let Some(form) = token.features.get(5) {
                    if form.starts_with("命令") {
                        return false;
                    }
                }

                // Exclude past tense forms - these are not potential forms
                // Past tense forms like やめた have conjugation 連用タ接続
                if let Some(form) = token.features.get(5) {
                    if form == "連用タ接続" {
                        return false;
                    }
                }

                // Exclude volitional forms (未然ウ接続) - these are not potential forms
                // Volitional forms like 逃げよ (from 逃げよう) have conjugation 未然ウ接続
                if let Some(form) = token.features.get(5) {
                    if form == "未然ウ接続" {
                        return false;
                    }
                }

                // Exclude negative forms - these are not potential forms
                // Negative forms like 考えない have base_form ending in potential-like える
                // but the surface ends with ない
                if token.surface.ends_with("ない") || token.surface.ends_with("なかった")
                    || token.surface.ends_with("なく") || token.surface.ends_with("ぬ") {
                    return false;
                }

                // Exclude ます-related forms (ましょう should not match as potential)
                // ます auxiliary can be in 未然ウ接続 form (ましょ), which superficially ends with potential-like patterns
                if token.base_form == "ます" {
                    return false;
                }

                // Get conjugation type from features[4]
                let _conjugation_type = token.features.get(4);

                // Check if base_form ends with potential suffix
                // Godan potential verbs end in: える, ける, せる, てる, ねる, べる, める, げる, れる
                let potential_endings = ["える", "ける", "せる", "てる", "ねる", "べる", "める", "げる", "れる"];

                for ending in &potential_endings {
                    if token.base_form.ends_with(ending) {
                        // Exclude the auxiliary れる/られる themselves
                        if token.base_form == "れる" || token.base_form == "られる" {
                            return false;
                        }

                        // Exclude auxiliary てる (ている contracted form) - not the natural verb 照る
                        // てる with pos 動詞/非自立 is the auxiliary, not a potential form
                        // Also exclude if surface contains verb stem + てる pattern (like してる, 見てる, etc.)
                        if token.base_form == "てる" {
                            // てる as auxiliary (non-independent verb)
                            if token.pos.get(1).is_some_and(|p| p == "非自立") {
                                return false;
                            }
                            // Also exclude if surface ends with してる, いてる, いでる, etc. (て-form + る contractions)
                            // These are progressive aspect, not potential
                            if token.surface.len() >= 3 && (
                                token.surface.ends_with("してる") ||
                                token.surface.ends_with("いてる") ||
                                token.surface.ends_with("いでる") ||
                                token.surface.ends_with("んでる") ||
                                token.surface.ends_with("えてる") ||
                                token.surface.ends_with("けてる") ||
                                token.surface.ends_with("せてる") ||
                                token.surface.ends_with("てる") && !token.surface.ends_with("照てる")  // general て + てる but not 照る
                            ) {
                                return false;
                            }
                            // CRITICAL: Check if てる is preceded by a て-form verb
                            // When tokenized separately (e.g., "言って" + "てる" or "し" + "てる"), the preceding token
                            // will have surface ending in て/で and be in 連用形/連用タ接続, OR
                            // be a 連用形 verb stem (like "し" from する) where the て is implicit in the てる
                            if let Some(prev) = ctx.lookbehind(1) {
                                // Case 1: Previous token ends with て/で (e.g., "言って" + "てる")
                                if (prev.surface.ends_with("て") || prev.surface.ends_with("で"))
                                    && prev.pos.first().is_some_and(|p| p == "動詞")
                                    && (prev.features.get(5).is_some_and(|f| f == "連用形")
                                        || prev.features.get(5).is_some_and(|f| f == "連用タ接続"))
                                {
                                    return false;
                                }

                                // Case 2: Previous token is a 連用形 verb stem (e.g., "し" from する + "てる")
                                // The "て" is part of "てる" here, making this a continuous form, not potential
                                if prev.pos.first().is_some_and(|p| p == "動詞")
                                    && prev.features.get(5).is_some_and(|f| f == "連用形")
                                {
                                    return false;
                                }

                                // Case 3: Previous token is in 連用タ接続 form without explicit て/で in surface
                                // This happens when tokenizer splits "言っ" + "てる" instead of "言って" + "る"
                                if prev.pos.first().is_some_and(|p| p == "動詞")
                                    && prev.features.get(5).is_some_and(|f| f == "連用タ接続")
                                {
                                    return false;
                                }
                            }
                        }

                        // Exclude the particle て when it appears as a single character
                        // This can happen when て is tokenized as a verb in certain contexts
                        if token.base_form == "て" || token.surface == "て" {
                            return false;
                        }

                        // Exclude 連用形 stems that are followed by て (these are て-forms, not potential)
                        // e.g., 助け + て is the conjunctive form of 助ける, not the potential 助けられる
                        if token.features.get(5).is_some_and(|f| f == "連用形" || f == "連用タ接続") {
                            // Check if followed by て particle
                            if let Some(next) = ctx.lookahead(1) {
                                if next.surface == "て" && next.pos.first().is_some_and(|p| p == "助詞") {
                                    return false;
                                }
                            }
                        }

                        // Exclude natural れる verbs (くれる, 入れる, etc.)
                        if NATURAL_RERU_VERBS.contains(&token.base_form.as_str()) {
                            return false;
                        }

                        // Exclude known natural える verbs (natural ichidan verbs)
                        // These are verbs like 食べる, 考える, 教える that are NOT derived potential forms
                        if NATURAL_ERU_VERBS.contains(&token.base_form.as_str()) {
                            return false;
                        }

                        // Exclude natural せる verbs (causative-form verbs or verbs ending in せる)
                        // These are verbs like 知らせる, 見せる that are NOT potential forms
                        if NATURAL_SERU_VERBS.contains(&token.base_form.as_str()) {
                            return false;
                        }

                        // Exclude natural める verbs (natural ichidan verbs)
                        // These are verbs like 覚める, 決める, 始める that are NOT derived potential forms
                        if NATURAL_MERU_VERBS.contains(&token.base_form.as_str()) {
                            return false;
                        }

                        // Exclude causative auxiliary せる/させる (tokenized as 動詞/接尾)
                        // Examples: 全滅させる (全滅さ + せる), 食べさせる (食べさ + せる)
                        // Also exclude させる entirely as it's the causative auxiliary base form
                        if token.base_form == "させる" || token.base_form == "せる" {
                            // Always exclude させる (causative auxiliary)
                            if token.base_form == "させる" {
                                return false;
                            }
                            // Exclude せる when it's marked as 接尾
                            if token.pos.get(1).is_some_and(|p| p == "接尾") {
                                return false;
                            }
                        }

                        // Exclude causative forms preceded by 未然形 verbs
                        // Examples: 死なせる (死な + せる), 死なせない (死な + せない)
                        // When the causative auxiliary is conjugated (e.g., せない, せた), it may
                        // still have base_form == "せる" and end with potential-like "せる" pattern
                        if token.base_form == "せる" {
                            if let Some(prev) = ctx.lookbehind(1) {
                                // Check if previous token is a verb in 未然形 (causative stem)
                                if prev.pos.first().is_some_and(|p| p == "動詞")
                                    && prev.features.get(5).is_some_and(|f| f == "未然形" || f == "未然レル接続")
                                {
                                    return false;
                                }
                            }
                        }

                        // SPECIAL CASE: 食べる and other common ichidan verbs
                        // Add common ichidan verbs that might not be in the whitelist
                        let common_ichidan = ["食べる", "寝る", "起きる", "出る", "着る", "見る", "いる", "得る", "いじめる"];
                        if common_ichidan.contains(&token.base_form.as_str()) {
                            return false;
                        }

                        // Exclude dialectal progressive forms like いてる (from いている)
                        // These are Kansai/dialectal contractions of ている, not potential forms
                        if token.base_form == "いてる" || token.base_form == "おてる" {
                            return false;
                        }

                        // Note: Godan potential forms like 歩ける (from 歩く) are tokenized as ichidan
                        // but they ARE potential forms, so we allow them through
                        return true;
                    }
                }
                false
            })
        }
    }

    // 2-token godan potential: verb(五段・未然形) + える(接尾)
    // This handles cases where the potential form is tokenized as two tokens
    #[derive(Debug)]
    struct GodanPotential2TokenMatcher;
    impl Matcher for GodanPotential2TokenMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            // Check first token: godan verb in 未然形
            let Some(token1) = ctx.current() else { return (false, 0); };
            if !token1.pos.first().is_some_and(|p| p == "動詞") {
                return (false, 0);
            }
            if !token1.features.get(4).is_some_and(|f| f.starts_with("五段")) {
                return (false, 0);
            }
            if !token1.features.get(5).is_some_and(|f| f == "未然形") {
                return (false, 0);
            }

            // Check second token: える suffix
            let Some(token2) = ctx.lookahead(1) else { return (false, 0); };
            if token2.base_form == "える"
                && token2.pos.first().is_some_and(|p| p == "動詞")
                && token2.pos.get(1).is_some_and(|p| p == "接尾")
            {
                return (true, 2);  // Consume 2 tokens
            }
            (false, 0)
        }
    }

    // Pattern: Single-token potential verb OR 2-token godan potential
    // Single-token: godan potential verbs (歩ける, 話せる, etc.) and できる
    // 2-token: godan verb 未然形 + える suffix
    // Note: Ichidan + られる is handled by the passive pattern (ambiguous)
    vec![
        TokenMatcher::Or(vec![
            TokenMatcher::Custom(Arc::new(PotentialVerbMatcher)),
            TokenMatcher::Custom(Arc::new(GodanPotential2TokenMatcher)),
        ]),
    ]
}

// Pattern: んだけど・んですが (explanatory + but/however)
// Structures: んだ/のだ + けど/けれど/けれども/けども/が, んです/のです + が/けど/けれど/けれども/けども
pub fn ndakedo_u30fb_ndesuga() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Optional: Match な (for Noun + なんですが pattern)
    #[derive(Debug)]
    struct NaAuxMatcher;
    impl Matcher for NaAuxMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "な"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.base_form == "だ"
            })
        }
    }

    // Match ん or の (explanatory nominalizer)
    #[derive(Debug)]
    struct NOrNoMatcher;
    impl Matcher for NOrNoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.surface == "ん" || token.surface == "の")
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
            })
        }
    }

    // Match だ or です (copula)
    #[derive(Debug)]
    struct DaDesuMatcher;
    impl Matcher for DaDesuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.surface == "だ" || token.surface == "です")
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    vec![
        optional(TokenMatcher::Custom(Arc::new(NaAuxMatcher))),
        TokenMatcher::Custom(Arc::new(NOrNoMatcher)),
        TokenMatcher::Custom(Arc::new(DaDesuMatcher)),
        or(vec![
            surface_particle("けど", "接続助詞"),
            surface_particle("けれど", "接続助詞"),
            surface_particle("けれども", "接続助詞"),
            surface_particle("けども", "接続助詞"),
            surface_particle("が", "接続助詞"),
        ]),
    ]
}

// Pattern: はずだ (should be, bound to be, supposed to)
// Structures: Verb + はずだ, i-Adj + はずだ, Na-Adj + な + はずだ, Noun + の + はずだ
pub fn hazuda() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match verb, i-adjective, or na-adjective/noun stem
    let pre_hazu_matcher = or(vec![
        verb(),
        adjective(),
        noun(),
    ]);

    // Match optional な (for na-adjectives) or の (for nouns)
    #[derive(Debug)]
    struct NaOrNoParticleMatcher;
    impl Matcher for NaOrNoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match な (auxiliary verb, copula)
                (token.surface == "な"
                    && token.base_form == "だ"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞"))
                // OR match の (possessive/modifier particle)
                || (token.surface == "の"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "連体化"))
            })
        }
    }

    // Match はず (dependent noun)
    #[derive(Debug)]
    struct HazuMatcher;
    impl Matcher for HazuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "はず"
                && token.base_form == "はず"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
            })
        }
    }

    // Match だ or です (auxiliary verb)
    #[derive(Debug)]
    struct DaDesuMatcher;
    impl Matcher for DaDesuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.pos.first().is_some_and(|pos| pos == "助動詞")
                && (token.base_form == "だ" || token.base_form == "です")
            })
        }
    }

    vec![
        pre_hazu_matcher,
        optional(TokenMatcher::Custom(Arc::new(NaOrNoParticleMatcher))),
        TokenMatcher::Custom(Arc::new(HazuMatcher)),
        TokenMatcher::Custom(Arc::new(DaDesuMatcher)),
    ]
}

// Pattern: かどうか (whether or not)
// Structures: Verb/Adjective/Noun + か + どう + か
pub fn kadouka() -> Vec<TokenMatcher> {
    use super::Matcher;
    use std::sync::Arc;

    // Match どう adverb
    #[derive(Debug)]
    struct DouAdverbMatcher;
    impl Matcher for DouAdverbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            super::check_token(ctx, |token| {
                token.surface == "どう"
                && token.base_form == "どう"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
            })
        }
    }

    vec![
        any(), // Preceding element (verb/adjective/noun)
        surface_particle("か", "副助詞／並立助詞／終助詞"),
        TokenMatcher::Custom(Arc::new(DouAdverbMatcher)),
        surface_particle("か", "副助詞／並立助詞／終助詞"),
    ]
}

// Pattern: ないと (must/have to)
// Structures: Verb[未然形] + ない + と (+ いけない/だめ)
pub fn naito() -> Vec<TokenMatcher> {
    use super::Matcher;
    use std::sync::Arc;

    // Match verb in 未然形 (mizen form, used before ない)
    #[derive(Debug)]
    struct VerbMizenMatcher;
    impl Matcher for VerbMizenMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            super::check_token(ctx, |token| {
                token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "未然形")
            })
        }
    }

    // Match ない (auxiliary verb for negation)
    #[derive(Debug)]
    struct NaiAuxMatcher;
    impl Matcher for NaiAuxMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            super::check_token(ctx, |token| {
                token.surface == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(4).is_some_and(|f| f == "特殊・ナイ")
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbMizenMatcher)),
        TokenMatcher::Custom(Arc::new(NaiAuxMatcher)),
        surface_particle("と", "接続助詞"),
    ]
}

// Pattern: はずがない (hardly possible, improbable, unlikely)
// Structures: Verb/i-Adj + はずがない, Na-Adj + な + はずがない, Noun + の + はずがない
pub fn hazuganai() -> Vec<TokenMatcher> {
    use super::Matcher;
    use std::sync::Arc;

    // Match optional な (for na-adjectives) or の (for nouns)
    #[derive(Debug)]
    struct NaOrNoParticleMatcher;
    impl Matcher for NaOrNoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            super::check_token(ctx, |token| {
                // Match な (auxiliary verb, copula)
                (token.surface == "な"
                    && token.base_form == "だ"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞"))
                // OR match の (possessive/modifier particle)
                || (token.surface == "の"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "連体化"))
            })
        }
    }

    // Match はず (dependent noun)
    #[derive(Debug)]
    struct HazuMatcher;
    impl Matcher for HazuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            super::check_token(ctx, |token| {
                token.surface == "はず"
                && token.base_form == "はず"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
            })
        }
    }

    // Match ない (i-adjective) or ある (for polite ありません)
    #[derive(Debug)]
    struct NaiOrAruMatcher;
    impl Matcher for NaiOrAruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            super::check_token(ctx, |token| {
                // Match ない (adjective)
                (token.surface == "ない"
                    && token.base_form == "ない"
                    && token.pos.first().is_some_and(|pos| pos == "形容詞"))
                // OR Match ある (verb, for ありません)
                || (token.base_form == "ある"
                    && token.pos.first().is_some_and(|pos| pos == "動詞"))
            })
        }
    }

    vec![
        // Match verb, i-adjective, or na-adjective/noun stem
        or(vec![
            verb(),
            adjective(),
            noun(),
        ]),
        optional(TokenMatcher::Custom(Arc::new(NaOrNoParticleMatcher))),
        TokenMatcher::Custom(Arc::new(HazuMatcher)),
        surface_particle("が", "格助詞"),
        TokenMatcher::Custom(Arc::new(NaiOrAruMatcher)),
    ]
}

// Pattern: しか～ない (only/nothing but)
// Structures: Noun + しか + Verb[ない]
pub fn shika_uff5e_nai() -> Vec<TokenMatcher> {
    use super::Matcher;
    use std::sync::Arc;

    // Match negative: ない (auxiliary verb or adjective) or ん (for ません)
    #[derive(Debug)]
    struct NegativeAuxiliaryMatcher;
    impl Matcher for NegativeAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            super::check_token(ctx, |token| {
                ((token.surface == "ない" && token.base_form == "ない")
                && (token.pos.first().is_some_and(|pos| pos == "助動詞")
                    || token.pos.first().is_some_and(|pos| pos == "形容詞")))
                || (token.surface == "ん" && token.base_form == "ん"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞"))
            })
        }
    }

    vec![
        noun(),
        surface_particle("しか", "係助詞"),
        wildcard(0, 5, vec![]),
        TokenMatcher::Custom(Arc::new(NegativeAuxiliaryMatcher)),
    ]
}

// Pattern: だけでなく (not only)
// Structures: Verb/い-Adj/な-Adj/Noun + だけ + で/では/じゃ + なく(て)
pub fn dakedenaku() -> Vec<TokenMatcher> {
    use super::Matcher;
    use std::sync::Arc;

    // Matcher for で (auxiliary verb だ in 連用形) or じゃ
    #[derive(Debug)]
    struct DeJaMatcher;
    impl Matcher for DeJaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            super::check_token(ctx, |token| {
                (token.surface == "で"
                    && token.base_form == "だ"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞"))
                || (token.surface == "じゃ"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "副助詞"))
            })
        }
    }

    // Matcher for なく (助動詞, base=ない)
    #[derive(Debug)]
    struct NakuMatcher;
    impl Matcher for NakuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            super::check_token(ctx, |token| {
                token.surface == "なく"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    vec![
        any(), // Any word (verb/adjective/noun)
        surface_particle("だけ", "副助詞"),
        TokenMatcher::Custom(Arc::new(DeJaMatcher)),
        optional(surface_particle("は", "係助詞")),
        TokenMatcher::Custom(Arc::new(NakuMatcher)),
        optional(surface_particle("て", "接続助詞")),
    ]
}

// Pattern: ことができる (can do / be able to)
// Structures: Verb + ことができる, Noun + ができる
pub fn kotogadekiru() -> Vec<TokenMatcher> {
    use super::{Matcher, verb_base, or, surface};
    use std::sync::Arc;

    #[derive(Debug)]
    struct KotoMatcher;
    impl Matcher for KotoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "こと"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),
        optional(TokenMatcher::Custom(Arc::new(KotoMatcher))),
        or(vec![
            surface("が"),
            surface("は"),
        ]),
        verb_base("できる"),
    ]
}

// Pattern: かい (casual question particle)
// Structures: Word + (な) + (の) + かい
pub fn kai() -> Vec<TokenMatcher> {
    use super::{Matcher, surface_particle, optional};
    use std::sync::Arc;

    // Matches な as 助動詞 (for noun/na-adjective)
    #[derive(Debug)]
    struct NaAuxiliaryMatcher;
    impl Matcher for NaAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "な"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Matches の as 名詞/非自立/一般 (nominalizer)
    #[derive(Debug)]
    struct NoNominalizerMatcher;
    impl Matcher for NoNominalizerMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(),
        optional(TokenMatcher::Custom(Arc::new(
            NaAuxiliaryMatcher,
        ))),
        optional(TokenMatcher::Custom(Arc::new(
            NoNominalizerMatcher,
        ))),
        surface_particle("かい", "終助詞"),
    ]
}

// Pattern: もし (if/suppose - conditional emphasis)
// Structure: もし (as 副詞)
pub fn moshi() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matches もし as 副詞/一般
    #[derive(Debug)]
    struct MoshiMatcher;
    impl Matcher for MoshiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "もし"
                && token.base_form == "もし"
                && token.pos.first().is_some_and(|pos| pos == "副詞")
                && token.pos.get(1).is_some_and(|pos| pos == "一般") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![TokenMatcher::Custom(Arc::new(MoshiMatcher))]
}

// Pattern: し～し 
pub fn shi_uff5e_shi() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: でできる・からできる (made from/out of)
// Structures: Noun + で/から + できる/できている/できます
pub fn dedekiru_u30fb_karadekiru() -> Vec<TokenMatcher> {
    vec![
        noun(),
        // で or から as case particle
        or(vec![
            surface_particle("で", "格助詞"),
            surface_particle("から", "格助詞"),
        ]),
        verb_base("できる"),
    ]
}

#[derive(Debug)]
struct DeParticleMatcher;
impl Matcher for DeParticleMatcher {
    fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
        match ctx.current() {
            Some(token) if token.surface == "で"
            && token.pos.first().is_some_and(|pos| pos == "助詞")
            && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
            _ => (false, 0),
        }
    }
}

#[derive(Debug)]
struct KaraParticleMatcher;
impl Matcher for KaraParticleMatcher {
    fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
        match ctx.current() {
            Some(token) if token.surface == "から"
            && token.pos.first().is_some_and(|pos| pos == "助詞")
            && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
            _ => (false, 0),
        }
    }
}

// Pattern: ながら (while doing)
// Structures: Verb[stem] + ながら
pub fn nagara() -> Vec<TokenMatcher> {
    vec![
        verb_form("連用形"), // Verb in stem form
        surface_particle("ながら", "接続助詞"),
    ]
}

// Pattern: たところだ (just did)
// Structures: Verb[た] + ところ (+ だ/です)
pub fn tatokoroda() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct TokoroMatcher;
    impl Matcher for TokoroMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ところ"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "非自立")
            })
        }
    }

    #[derive(Debug)]
    struct DaDesuMatcher;
    impl Matcher for DaDesuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.surface == "だ" || token.surface == "です")
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    // Pattern: Verb (連用形/連用タ接続) + た + ところ (+ optional だ/です)
    vec![
        flexible_verb_form(),
        past_auxiliary(),
        TokenMatcher::Custom(Arc::new(TokoroMatcher)),
        optional(TokenMatcher::Custom(Arc::new(DaDesuMatcher))),
    ]
}

// Pattern: ているところだ (in the middle of doing)
// Structures: Verb[ている] + ところ + だ/です
pub fn teirutokoroda() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher for ところ as 名詞/非自立/副詞可能
    #[derive(Debug)]
    struct TokoroMatcher;
    impl Matcher for TokoroMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ところ"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "非自立")
            })
        }
    }

    // Matcher for だ or です as auxiliary
    #[derive(Debug)]
    struct DaDesuMatcher;
    impl Matcher for DaDesuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.surface == "だ" || token.surface == "です")
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    // Pattern: Verb[ている] + ところ + だ/です
    // We need to match the ている pattern first, then ところ, then optional だ/です

    // Get the ている pattern from N5
    let teiru_pattern = crate::matchers::n5::teiru_u2460();

    concat(vec![
        teiru_pattern,
        vec![TokenMatcher::Custom(Arc::new(TokoroMatcher))],
        vec![optional(TokenMatcher::Custom(Arc::new(DaDesuMatcher)))],
    ])
}

// Pattern: と～と、どちらが 
// Pattern: と～と、どちらが (which is... ?)
// Structures: どちら/どっち + (のほう) + が
pub fn to_uff5e_to_u3001_dochiraga() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match どちら or どっち (pronoun)
    #[derive(Debug)]
    struct DochiraDotchiMatcher;
    impl Matcher for DochiraDotchiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.surface == "どちら" || token.surface == "どっち")
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "代名詞")
            })
        }
    }

    // Match ほう (direction/side)
    #[derive(Debug)]
    struct HouMatcher;
    impl Matcher for HouMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ほう"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "非自立")
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(DochiraDotchiMatcher)),
        optional(surface_particle("の", "連体化")),
        optional(TokenMatcher::Custom(Arc::new(HouMatcher))),
        surface_particle("が", "格助詞"),
    ]
}

// Pattern: ようにする (try to / make sure to)
// Structures: Verb[る] + ように + する / Verb[ない] + ように + する
pub fn younisuru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct NaiMatcher;
    impl Matcher for NaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ない"
                    && token.base_form == "ない"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    #[derive(Debug)]
    struct YouMatcher;
    impl Matcher for YouMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "よう"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                    && token.pos.get(2).is_some_and(|pos| pos == "助動詞語幹")
            })
        }
    }

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "に"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            })
        }
    }

    vec![
        verb(),
        optional(TokenMatcher::Custom(Arc::new(NaiMatcher))),
        TokenMatcher::Custom(Arc::new(YouMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        verb_base("する"),
    ]
}

// Pattern: なければいけない (must do / have to)
// Structures: Verb[未然形] + なければ + いけない / Verb[未然形] + なきゃ + いけない
pub fn nakerebaikenai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Combined matcher for なければ+ば OR なきゃ
    #[derive(Debug)]
    struct NakerebaOrNakyaMatcher;
    impl Matcher for NakerebaOrNakyaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match なけれ (仮定形 of ない auxiliary) - will be followed by ば
                (token.surface == "なけれ"
                    && token.base_form == "ない"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.features.get(5).is_some_and(|f| f == "仮定形"))
                // OR match なきゃ (仮定縮約２ of ない auxiliary)
                || (token.surface == "なきゃ"
                    && token.base_form == "ない"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.features.get(5).is_some_and(|f| f == "仮定縮約２"))
            })
        }
    }

    // Match ば (connecting particle) - optional because なきゃ doesn't need it
    #[derive(Debug)]
    struct BaParticleMatcher;
    impl Matcher for BaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ば"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
            })
        }
    }

    // Match いけ (いける verb in 未然形 or 連用形)
    #[derive(Debug)]
    struct IkeMatcher;
    impl Matcher for IkeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "いけ"
                    && token.base_form == "いける"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                    && (token.features.get(5).is_some_and(|f| f == "未然形")
                        || token.features.get(5).is_some_and(|f| f == "連用形"))
            })
        }
    }

    // Match ない (auxiliary in 基本形)
    #[derive(Debug)]
    struct NaiAuxiliaryMatcher;
    impl Matcher for NaiAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ない"
                    && token.base_form == "ない"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.features.get(5).is_some_and(|f| f == "基本形")
            })
        }
    }

    // Pattern: (なければ + ば OR なきゃ) + いけ + ない (or polite forms with ます)
    // The ない is optional to allow automatic extension for polite forms (いけません)
    vec![
        TokenMatcher::Custom(Arc::new(NakerebaOrNakyaMatcher)),
        optional(TokenMatcher::Custom(Arc::new(BaParticleMatcher))),
        TokenMatcher::Custom(Arc::new(IkeMatcher)),
        optional(TokenMatcher::Custom(Arc::new(NaiAuxiliaryMatcher))),
    ]
}

// Pattern: なければならない (must do / have to)
// Structures: Verb[未然形] + なければ + ならない / Verb[未然形] + なきゃ + ならない
pub fn nakerebanaranai() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Combined matcher for なければ+ば OR なきゃ
    #[derive(Debug)]
    struct NakerebaOrNakyaMatcher;
    impl Matcher for NakerebaOrNakyaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match なけれ (仮定形 of ない auxiliary) - will be followed by ば
                (token.surface == "なけれ"
                    && token.base_form == "ない"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.features.get(5).is_some_and(|f| f == "仮定形"))
                // OR match なきゃ (仮定縮約２ of ない auxiliary)
                || (token.surface == "なきゃ"
                    && token.base_form == "ない"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.features.get(5).is_some_and(|f| f == "仮定縮約２"))
            })
        }
    }

    // Match ば (connecting particle) - optional because なきゃ doesn't need it
    #[derive(Debug)]
    struct BaParticleMatcher;
    impl Matcher for BaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ば"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
            })
        }
    }

    // Match なら or なり (なる verb in 未然形 or 連用形)
    // なら = 未然形 (used with ない in standard form)
    // なり = 連用形 (used with ます in polite form)
    #[derive(Debug)]
    struct NaraMatcher;
    impl Matcher for NaraMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.surface == "なら" || token.surface == "なり")
                    && token.base_form == "なる"
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                    && (token.pos.get(1).is_some_and(|pos| pos == "非自立")
                        || token.pos.get(1).is_some_and(|pos| pos == "自立"))
                    && (token.features.get(5).is_some_and(|f| f == "未然形")
                        || token.features.get(5).is_some_and(|f| f == "連用形"))
            })
        }
    }

    // Match ない (auxiliary in 基本形) - optional to allow automatic extension for polite forms
    #[derive(Debug)]
    struct NaiAuxiliaryMatcher;
    impl Matcher for NaiAuxiliaryMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ない"
                    && token.base_form == "ない"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
                    && token.features.get(5).is_some_and(|f| f == "基本形")
            })
        }
    }

    // Pattern: (なければ + ば OR なきゃ) + なら/なり + ない (or polite forms with ます)
    // The ない is optional to allow automatic extension for polite forms (なりません)
    vec![
        TokenMatcher::Custom(Arc::new(NakerebaOrNakyaMatcher)),
        optional(TokenMatcher::Custom(Arc::new(BaParticleMatcher))),
        TokenMatcher::Custom(Arc::new(NaraMatcher)),
        optional(TokenMatcher::Custom(Arc::new(NaiAuxiliaryMatcher))),
    ]
}

// Pattern: つづける
// Pattern: つづける - continue doing
// Structures: Verb[stem] + 続ける/つづける
pub fn tsuzukeru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct TsuzukeruMatcher;
    impl Matcher for TsuzukeruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                (token.base_form == "つづける" || token.base_form == "続ける")
                    && token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "非自立")
            })
        }
    }
    vec![flexible_verb_form(), TokenMatcher::Custom(Arc::new(TsuzukeruMatcher))]
}

// Pattern: ようにいう (to tell/ask someone to do)
// Structures: Verb[未然形] + ない + ように + 言う/頼む/命じる
pub fn youniiu() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ない auxiliary
    #[derive(Debug)]
    struct NaiMatcher;
    impl Matcher for NaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ない"
                    && token.base_form == "ない"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    // Match よう as dependent noun with auxiliary verb stem
    #[derive(Debug)]
    struct YouMatcher;
    impl Matcher for YouMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "よう"
                    && token.base_form == "よう"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                    && token.pos.get(2).is_some_and(|pos| pos == "助動詞語幹")
            })
        }
    }

    // Match に as adverbial particle (副詞化)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "に"
                    && token.base_form == "に"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "副詞化")
            })
        }
    }

    // Match いう/言う, たのむ/頼む, or めいじる/命じる
    #[derive(Debug)]
    struct IuTanomuMeijiruMatcher;
    impl Matcher for IuTanomuMeijiruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "自立")
                    && (token.base_form == "いう"
                        || token.base_form == "たのむ"
                        || token.base_form == "めいじる")
            })
        }
    }

    vec![
        verb(),  // Match any verb (will be in 未然形 before ない)
        TokenMatcher::Custom(Arc::new(NaiMatcher)),
        TokenMatcher::Custom(Arc::new(YouMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(IuTanomuMeijiruMatcher)),
    ]
}

// Pattern: よていだ (plan to)
// Structures: Verb[る] + 予定 + だ/です, Noun + の + 予定 + だ/です
pub fn yoteida() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match Verb in 基本形 OR の particle (連体化)
    // This handles both "Verb + 予定" and "Noun + の + 予定"
    #[derive(Debug)]
    struct VerbOrNoMatcher;
    impl Matcher for VerbOrNoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match verb in dictionary form
            let is_verb = token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|f| f == "基本形");

            // Match の particle (連体化)
            let is_no = token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "連体化");

            is_verb || is_no
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(VerbOrNoMatcher)),
        surface("予定"),
        or(vec![surface("だ"), surface("です")]),
    ]
}

// Pattern: ようにいのる (to pray that, to hope)
// Structures: Verb + (ない) + ように + 祈る
pub fn youniinoru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ない auxiliary (optional for negative forms)
    #[derive(Debug)]
    struct NaiMatcher;
    impl Matcher for NaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ない"
                    && token.base_form == "ない"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    // Match ます auxiliary (optional for polite forms)
    #[derive(Debug)]
    struct MasuMatcher;
    impl Matcher for MasuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ます"
                    && token.base_form == "ます"
                    && token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    // Match よう as dependent noun with auxiliary verb stem
    #[derive(Debug)]
    struct YouMatcher;
    impl Matcher for YouMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "よう"
                    && token.base_form == "よう"
                    && token.pos.first().is_some_and(|pos| pos == "名詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                    && token.pos.get(2).is_some_and(|pos| pos == "助動詞語幹")
            })
        }
    }

    // Match に particle (格助詞 or 副詞化)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "に"
                    && token.base_form == "に"
                    && token.pos.first().is_some_and(|pos| pos == "助詞")
                    && (token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                        || token.pos.get(1).is_some_and(|pos| pos == "副詞化"))
            })
        }
    }

    // Match いのる/祈る verb
    #[derive(Debug)]
    struct InoruMatcher;
    impl Matcher for InoruMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.pos.first().is_some_and(|pos| pos == "動詞")
                    && token.pos.get(1).is_some_and(|pos| pos == "自立")
                    && token.base_form == "いのる"
            })
        }
    }

    vec![
        verb(),  // Match any verb (will be in 基本形 or 未然形 before ない)
        optional(TokenMatcher::Custom(Arc::new(NaiMatcher))),
        optional(TokenMatcher::Custom(Arc::new(MasuMatcher))),
        TokenMatcher::Custom(Arc::new(YouMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        TokenMatcher::Custom(Arc::new(InoruMatcher)),
    ]
}

// Pattern: Just finished doing (買ったばかり - just bought)
// Structures: Verb[た] + ばかり
fn bakari_particle() -> TokenMatcher {
    #[derive(Debug)]
    struct BakariParticleMatcher;
    impl Matcher for BakariParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ばかり"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }
    TokenMatcher::Custom(Arc::new(BakariParticleMatcher))
}

pub fn tabakari() -> Vec<TokenMatcher> {
    vec![
        super::flexible_verb_form(),
        super::past_auxiliary(),
        bakari_particle(),
    ]
}

// Pattern: 化する (to become/transform into ~)
// Structures: Noun + 化（か）+ する
pub fn kasuru() -> Vec<TokenMatcher> {
    vec![
        noun(),
        surface_noun_suffix("化"),
        verb_base("する"),
    ]
}

// Pattern: 命令形 (imperative form)
// Structures: Verb in imperative conjugation (命令ｅ, 命令ｒｏ, 命令ｙｏ)
pub fn meireigata() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct ImperativeMatcher;
    impl Matcher for ImperativeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Check if it's a verb
                if !token.pos.first().is_some_and(|p| p == "動詞") {
                    return false;
                }

                // Exclude auxiliary verbs (非自立) - these are part of compound patterns like なさい, ください
                // Only match standalone verbs (自立)
                if token.pos.get(1).is_some_and(|p| p == "非自立") {
                    return false;
                }

                // Explicitly exclude ください and なさい (polite request forms, not true imperatives)
                // These have their own patterns (てください, なさい)
                if token.base_form == "くださる" || token.base_form == "なさる" {
                    return false;
                }

                // Check if conjugation form is imperative (命令ｅ, 命令ｒｏ, or 命令ｙｏ, 命令ｉ)
                // The conjugation form is at features index 5
                token.features.get(5).is_some_and(|form| {
                    form.starts_with("命令")
                })
            })
        }
    }

    vec![TokenMatcher::Custom(Arc::new(ImperativeMatcher))]
}

// Pattern: ように (so that, in order to)
// Structures: Verb[る/できる/ない] + ように + Phrase
pub fn youni() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct NaiMatcher;
    impl Matcher for NaiMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "ない"
                && token.base_form == "ない"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
            })
        }
    }

    #[derive(Debug)]
    struct YouMatcher;
    impl Matcher for YouMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "よう"
                && token.base_form == "よう"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
                && token.pos.get(2).is_some_and(|pos| pos == "助動詞語幹")
            })
        }
    }

    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "に"
                && token.base_form == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "副詞化")
            })
        }
    }

    vec![
        verb(),
        optional(TokenMatcher::Custom(Arc::new(NaiMatcher))),
        TokenMatcher::Custom(Arc::new(YouMatcher)),
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
    ]
}

// Pattern: かしら (I wonder)
// Structures: Phrase + かしら
pub fn kashira() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct KashiraParticle;
    impl Matcher for KashiraParticle {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "かしら"
                && token.base_form == "かしら"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "終助詞")
            })
        }
    }

    vec![TokenMatcher::Custom(Arc::new(KashiraParticle))]
}

// Pattern: らしい ② (typical of, characteristic)
// Structures: Noun + らしい, Noun + らしく + Phrase, Noun + らしい + Noun
//
// This pattern matches the い-adjective usage of らしい meaning "typical of" or "befitting".
// Unlike らしい ① (auxiliary verb for hearsay), this specifically matches:
// 1. Compound adjectives tokenized as single い-Adjective: 男らしい, 女らしい, 春らしい
// 2. The auxiliary verb form when used to express characteristic (same tokenization as らしい ①)
//
// Key distinction:
// - らしい ① (助動詞): Hearsay/conjecture - "apparently", "seems like" (based on evidence)
// - らしい ② (形容詞 or 助動詞): Characteristic - "typical of", "befitting", "like"
//
// Note: When らしい appears as 助動詞 after a noun, it's structurally identical to らしい ①.
// Only the semantic meaning differs. This pattern focuses on capturing the compound adjective
// forms (形容詞/自立) which are unambiguously らしい ②.
pub fn rashii_u2461() -> Vec<TokenMatcher> {
    // Matcher for い-Adjective compounds ending in らしい (e.g., 男らしい, 女らしい, 春らしい)
    #[derive(Debug)]
    struct RashiiAdjMatcher;
    impl Matcher for RashiiAdjMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.pos.get(1).is_some_and(|pos| pos == "自立")
                && token.base_form.ends_with("らしい")
            })
        }
    }

    vec![TokenMatcher::Custom(Arc::new(RashiiAdjMatcher))]
}

// Pattern: にみえる (appears/looks like)
// Structures: Verb + ように + みえる, Adj + そうに + みえる, Noun + (のように/に) + みえる
pub fn nimieru() -> Vec<TokenMatcher> {
    // Match よう or そう (auxiliary verb stems) OR a noun (for direct "Noun + に + みえる")
    #[derive(Debug)]
    struct NounOrYouSouMatcher;
    impl Matcher for NounOrYouSouMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // Match よう or そう (auxiliary verb stems)
            ((token.surface == "よう" || token.surface == "そう")
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && (token.pos.get(1).is_some_and(|pos| pos == "非自立" || pos == "接尾")))
            ||
            // OR match any noun (for direct "Noun + に + みえる" pattern)
            (token.pos.first().is_some_and(|pos| pos == "名詞")
                && !token.pos.get(1).is_some_and(|pos| pos == "非自立" || pos == "接尾"))
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NounOrYouSouMatcher)),
        or(vec![
            surface_particle("に", "副詞化"),
            surface_particle("に", "格助詞"),
        ]),
        verb_base("みえる"),
    ]
}

// Pattern: とみえる (it seems/can be deduced that)
// Structures: Verb/Adjective/Noun + (だ) + と + みえる/みえます
pub fn tomieru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match と as quotation particle (助詞/格助詞/引用)
    #[derive(Debug)]
    struct ToQuotationMatcher;
    impl Matcher for ToQuotationMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "引用") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(ToQuotationMatcher)),
        verb_base("みえる"),
    ]
}

// Pattern: 風
pub fn kaze() -> Vec<TokenMatcher> {
    // Match 風 as suffix (名詞/接尾/一般)
    // Note: Always pronounced ふう in this usage, not かぜ
    vec![
        noun(),
        surface_noun_suffix("風"),
    ]
}

// Pattern: がみられる (can be seen/observed)
// Structures: Noun + が/も + 見られる/見られます
pub fn gamirareru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match が or も particle
    #[derive(Debug)]
    struct GaMoParticleMatcher;
    impl Matcher for GaMoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.surface == "が" || token.surface == "も")
                && token.pos.first().is_some_and(|pos| pos == "助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match 見 verb in 未然形
    #[derive(Debug)]
    struct MiruVerbMatcher;
    impl Matcher for MiruVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "見"
                && token.base_form == "見る"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token
                    .features
                    .get(5)
                    .is_some_and(|form| form == "未然形") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        noun(),
        TokenMatcher::Custom(Arc::new(GaMoParticleMatcher)),
        TokenMatcher::Custom(Arc::new(MiruVerbMatcher)),
        rareru_suffix(),
        optional(mashi_form()),
    ]
}

// Pattern: にきがつく (to notice/realize) - compound form
// Structures: Verb/Noun + (こと/の) + に + (も) + 気がつく
// This handles the kanji form where 気がつく tokenizes as a single verb
pub fn nikigatsuku() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match に particle (格助詞)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match も particle (係助詞) - optional
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        optional(TokenMatcher::Custom(Arc::new(MoParticleMatcher))),
        verb_base("気がつく"),
    ]
}

// Pattern: にきがつく (to notice/realize) - split form
// Structures: Verb/Noun + (こと/の) + に + (も) + き + が + つく
// This handles the hiragana form where きがつく tokenizes as separate tokens
pub fn nikigatsuku_split() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match に particle (格助詞)
    #[derive(Debug)]
    struct NiParticleMatcher;
    impl Matcher for NiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "に"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
            })
        }
    }

    // Match も particle (係助詞) - optional
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "係助詞")
            })
        }
    }

    // Match き from くる (come) - 連用形
    #[derive(Debug)]
    struct KiVerbMatcher;
    impl Matcher for KiVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "き"
                && token.base_form == "くる"
                && token.pos.first().is_some_and(|pos| pos == "動詞")
            })
        }
    }

    // Match が particle
    #[derive(Debug)]
    struct GaParticleMatcher;
    impl Matcher for GaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "が"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NiParticleMatcher)),
        optional(TokenMatcher::Custom(Arc::new(MoParticleMatcher))),
        TokenMatcher::Custom(Arc::new(KiVerbMatcher)),
        TokenMatcher::Custom(Arc::new(GaParticleMatcher)),
        verb_base("つく"),
    ]
}

// Pattern: 〜でも 〜でも (whether...or, even if...or)
// Structures:
// - い-Adj(連用テ接続) + て + も + い-Adj(連用テ接続) + て + も
// - な-Adj/Noun + でも + な-Adj/Noun + でも
// - Noun + で + も + Noun + で + も (alternative tokenization)
pub fn u301c_demo_u301c_demo() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match first word: い-adjective, な-adjective, or noun
    #[derive(Debug)]
    struct FirstWordMatcher;
    impl Matcher for FirstWordMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // い-adjective in 連用テ接続 form
            if token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token
                    .features
                    .get(5)
                    .is_some_and(|f| f == "連用テ接続")
            {
                return true;
            }
            // な-adjective (形容動詞語幹) or regular noun
            token.pos.first().is_some_and(|pos| pos == "名詞")
            })
        }
    }

    // Match て/で/でも particle
    // - て: 接続助詞 (for い-adjectives)
    // - で: 格助詞 (for nouns)
    // - でも: 副助詞 (for な-adj/noun, single token)
    #[derive(Debug)]
    struct TeDeOrDemoMatcher;
    impl Matcher for TeDeOrDemoMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                // て as 接続助詞
            if token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "接続助詞")
            {
                return true;
            }
            // で as 格助詞
            if token.surface == "で"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
            {
                return true;
            }
            // でも as single 副助詞 token
            token.surface == "でも"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "副助詞")
            })
        }
    }

    // Match も particle
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "も"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "係助詞")
            })
        }
    }

    vec![
        // First instance: Word + (て/で/でも) + も
        TokenMatcher::Custom(Arc::new(FirstWordMatcher)),
        TokenMatcher::Custom(Arc::new(TeDeOrDemoMatcher)), // て, で, or でも
        optional(TokenMatcher::Custom(Arc::new(MoParticleMatcher))), // も (optional for でも case)
        // Second instance: Word + (て/で/でも) + も
        TokenMatcher::Custom(Arc::new(FirstWordMatcher)),
        TokenMatcher::Custom(Arc::new(TeDeOrDemoMatcher)), // て, で, or でも
        optional(TokenMatcher::Custom(Arc::new(MoParticleMatcher))), // も (optional for でも case)
    ]
}

// Pattern: それに (moreover/in addition/what's more)
// Structure: それに + (Additional Information) Phrase
// Note: Can be tokenized as single conjunction token OR as それ + に (two tokens)
pub fn soreni() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Matcher that handles BOTH tokenizations:
    // 1. Single token: それに (接続詞)
    // 2. Two-token sequence: それ (名詞/代名詞) + に (助詞)
    #[derive(Debug)]
    struct SoreniMatcher;
    impl Matcher for SoreniMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            let Some(token) = ctx.current() else {
                return (false, 0);
            };

            // Pattern 1: Single-token それに (接続詞)
            if token.surface == "それに"
                && token.base_form == "それに"
                && token.pos.first().is_some_and(|pos| pos == "接続詞")
            {
                return (true, 1);
            }

            // Pattern 2: Two-token sequence それ + に
            if token.surface == "それ"
                && token.base_form == "それ"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "代名詞")
            {
                // Check if next token is に (case particle)
                if let Some(next_token) = ctx.lookahead(1) {
                    if next_token.surface == "に"
                        && next_token.base_form == "に"
                        && next_token.pos.first().is_some_and(|pos| pos == "助詞")
                        && next_token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                    {
                        return (true, 2);
                    }
                }
            }

            (false, 0)
        }
    }

    vec![TokenMatcher::Custom(Arc::new(SoreniMatcher))]
}

// Pattern: それで (therefore/so/as a result)
// Structure: Phrase (A)。それで + Phrase (B)
pub fn sorede() -> Vec<TokenMatcher> {
    use std::sync::Arc;
    #[derive(Debug)]
    struct SoredeMatcher;
    impl Matcher for SoredeMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.surface == "それで"
                && token.base_form == "それで"
                && token.pos.first().is_some_and(|pos| pos == "接続詞")
            })
        }
    }
    vec![TokenMatcher::Custom(Arc::new(SoredeMatcher))]
}

// Pattern: Question-phrase + か (embedded question)
// Structures: Verb/Phrase + か + わかる/知る/決める/覚える etc.
// Examples: 来るか分かる (know if coming), 何で壊すか知る (know why destroying)
//
// Note: This か is the adverbial particle (副助詞), not the sentence-ending question marker.
// It marks embedded questions - uncertain things that are being inquired about.
// IMPORTANT: Must exclude indefinite pronouns (何か=something, 誰か=someone, どこか=somewhere)
pub fn question_phrase_ka() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match か as adverbial particle (but not part of indefinite pronouns)
    #[derive(Debug)]
    struct KaParticleMatcher;
    impl Matcher for KaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            if !check_token(ctx, |token| {
                token.surface == "か"
                && token.base_form == "か"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
            }).0 {
                return (false, 0);
            }

            // Check if previous token is a question word forming an indefinite pronoun
            if let Some(prev_token) = ctx.lookbehind(1) {
                // Exclude: 何か (something), 誰か (someone), どこか (somewhere),
                // いつか (sometime), どれか (one of them), etc.
                let indefinite_bases = ["何", "なに", "なん", "誰", "だれ", "どこ", "いつ", "どれ", "どちら"];
                if indefinite_bases.contains(&prev_token.surface.as_str())
                    || indefinite_bases.contains(&prev_token.base_form.as_str()) {
                    return (false, 0);
                }
            }
            (true, 1)
        }
    }

    // Match information-seeking verbs (わかる, 知る, 決める, 覚える, etc.)
    #[derive(Debug)]
    struct InfoVerbMatcher;
    impl Matcher for InfoVerbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            check_token(ctx, |token| {
                token.pos.first().is_some_and(|pos| pos == "動詞")
                && [
                    "分かる", "わかる", "判る", "解る",
                    "知る",
                    "決める",
                    "覚える",
                    "教える",
                    "確かめる",
                    "調べる",
                    "聞く",
                    "考える",
                    "見る",
                ]
                .contains(&token.base_form.as_str())
            })
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(KaParticleMatcher)),
        TokenMatcher::Custom(Arc::new(InfoVerbMatcher)),
    ]
}

// Pattern: それでも (even so/nevertheless)
// Structure: Phrase (A) + それでも + Phrase (B)
pub fn soredemo() -> Vec<TokenMatcher> {
    vec![surface("それでも")]
}

// Pattern: たらどう (why don't you / how about)
// Structures: Verb［たら］+ どう + (だ/か/です + か)
pub fn taradou() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match たら conditional auxiliary (仮定形)
    #[derive(Debug)]
    struct TaraConditionalMatcher;
    impl Matcher for TaraConditionalMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "たら"
                && token.base_form == "た"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token.features.get(5).is_some_and(|f| f == "仮定形") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match どう adverb
    #[derive(Debug)]
    struct DouAdverbMatcher;
    impl Matcher for DouAdverbMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "どう"
                && token.base_form == "どう"
                && token.pos.first().is_some_and(|pos| pos == "副詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match だ copula (optional)
    #[derive(Debug)]
    struct DaCopulaMatcher;
    impl Matcher for DaCopulaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "だ"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match か question particle (optional)
    #[derive(Debug)]
    struct KaParticleMatcher;
    impl Matcher for KaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "か"
                && token.base_form == "か"
                && token.pos.first().is_some_and(|pos| pos == "助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match です polite auxiliary (optional)
    #[derive(Debug)]
    struct DesuMatcher;
    impl Matcher for DesuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "です"
                && token.base_form == "です"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        super::flexible_verb_form(),
        TokenMatcher::Custom(Arc::new(TaraConditionalMatcher)),
        TokenMatcher::Custom(Arc::new(DouAdverbMatcher)),
        optional(TokenMatcher::Custom(Arc::new(DesuMatcher))),
        optional(TokenMatcher::Custom(Arc::new(KaParticleMatcher))),
        optional(TokenMatcher::Custom(Arc::new(DaCopulaMatcher))),
    ]
}

// Pattern: とかんがえられている / とおもわれている (it is thought/considered that)
// Structures: Phrase + と考えられている/と思われている/と考えられています/と思われています
pub fn tokangaerareteiru() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    #[derive(Debug)]
    struct ToQuotationMatcher;
    impl Matcher for ToQuotationMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "引用") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match かんがえる or おもう in 未然形
    #[derive(Debug)]
    struct ThinkVerbMizenMatcher;
    impl Matcher for ThinkVerbMizenMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if (token.base_form == "かんがえる" || token.base_form == "考える"
                || token.base_form == "おもう" || token.base_form == "思う")
                && token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.features.get(5).is_some_and(|form| form == "未然形") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match passive れる or られる - using helper functions
    let passive_matcher = or(vec![rareru_suffix(), reru_suffix()]);

    #[derive(Debug)]
    struct TeParticleMatcher;
    impl Matcher for TeParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "て"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(ToQuotationMatcher)),
        TokenMatcher::Custom(Arc::new(ThinkVerbMizenMatcher)),
        passive_matcher,
        TokenMatcher::Custom(Arc::new(TeParticleMatcher)),
        verb_base("いる"),
    ]
}

// Pattern: とされている
pub fn tosareteiru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: といわれている
pub fn toiwareteiru() -> Vec<TokenMatcher> {
    vec![]  // TODO: Implement
}

// Pattern: ばよかった (should have / wish I had)
// Structures: Verb［ば］+ よかった + (です)
pub fn bayokatta() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match ば conditional particle
    #[derive(Debug)]
    struct BaParticleMatcher;
    impl Matcher for BaParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "ば"
                && token.base_form == "ば"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    // Match よかっ (good, past form conjugation)
    #[derive(Debug)]
    struct YokattaMatcher;
    impl Matcher for YokattaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "よかっ"
                && token.base_form == "よい"
                && token.pos.first().is_some_and(|pos| pos == "形容詞")
                && token.features.get(5).is_some_and(|f| f == "連用タ接続") => (true, 1),
                _ => (false, 0),
            }
        }
    }


    // Match です polite auxiliary (optional)
    #[derive(Debug)]
    struct DesuMatcher;
    impl Matcher for DesuMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "です"
                && token.base_form == "です"
                && token.pos.first().is_some_and(|pos| pos == "助動詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        verb_form("仮定形"),
        TokenMatcher::Custom(Arc::new(BaParticleMatcher)),
        TokenMatcher::Custom(Arc::new(YokattaMatcher)),
        past_auxiliary(),
        optional(TokenMatcher::Custom(Arc::new(DesuMatcher))),
    ]
}

// Pattern: し～し (listing reasons with equal weight)
// Structures: Verb/い-Adj + し, な-Adj/Noun + だ + し
pub fn shi_u301c_shi() -> Vec<TokenMatcher> {
    #[derive(Debug)]
    struct ShiParticleMatcher;
    impl Matcher for ShiParticleMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "し"
                && token.base_form == "し"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "接続助詞") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    #[derive(Debug)]
    struct DaCopulaMatcher;
    impl Matcher for DaCopulaMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "だ"
                && token.base_form == "だ"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
                && token
                    .features
                    .get(4)
                    .is_some_and(|f| f == "特殊・ダ")
                && token.features.get(5).is_some_and(|f| f == "基本形") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        any(), // Verb, い-Adjective, な-Adjective (名詞/形容動詞語幹), or Noun
        optional(TokenMatcher::Custom(Arc::new(DaCopulaMatcher))), // Optional だ (for na-adj/noun)
        TokenMatcher::Custom(Arc::new(ShiParticleMatcher)), // し (接続助詞)
    ]
}

// Pattern: といわれている (it is said that)
// Structures: Phrase + と + いわれている/いわれています
pub fn to_iwareteiru() -> Vec<TokenMatcher> {
    vec![
        // Match と quotation particle (助詞/格助詞/引用)
        TokenMatcher::Custom(Arc::new({
            #[derive(Debug)]
            struct ToQuotation;
            impl Matcher for ToQuotation {
                fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
                    check_token(ctx, |token| {
                        token.surface == "と"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                        && token.pos.get(2).is_some_and(|pos| pos == "引用")
                    })
                }
            }
            ToQuotation
        })),
        // Match いう/言う verb in mizen form
        TokenMatcher::Custom(Arc::new({
            #[derive(Debug)]
            struct IuVerbMizen;
            impl Matcher for IuVerbMizen {
                fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
                    check_token(ctx, |token| {
                        (token.base_form == "いう" || token.base_form == "言う")
                        && token.pos.first().is_some_and(|pos| pos == "動詞")
                        && token.features.get(5).is_some_and(|form| form == "未然形")
                    })
                }
            }
            IuVerbMizen
        })),
        reru_suffix(),
        // Match て connecting particle
        TokenMatcher::Custom(Arc::new({
            #[derive(Debug)]
            struct TeParticle;
            impl Matcher for TeParticle {
                fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
                    check_token(ctx, |token| {
                        token.surface == "て"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
                    })
                }
            }
            TeParticle
        })),
        verb_base("いる"),
    ]
}

// Pattern: とされている (it is considered that)
// Structures: Phrase + と + されている/されています
pub fn to_sareteiru() -> Vec<TokenMatcher> {
    vec![
        // Match と quotation particle (助詞/格助詞/引用)
        TokenMatcher::Custom(Arc::new({
            #[derive(Debug)]
            struct ToQuotation;
            impl Matcher for ToQuotation {
                fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
                    check_token(ctx, |token| {
                        token.surface == "と"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                        && token.pos.get(2).is_some_and(|pos| pos == "引用")
                    })
                }
            }
            ToQuotation
        })),
        // Match する verb in special mizen form (未然レル接続)
        TokenMatcher::Custom(Arc::new({
            #[derive(Debug)]
            struct SuruVerbMizen;
            impl Matcher for SuruVerbMizen {
                fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
                    check_token(ctx, |token| {
                        token.base_form == "する"
                        && token.pos.first().is_some_and(|pos| pos == "動詞")
                        && token.features.get(5).is_some_and(|form| form == "未然レル接続")
                    })
                }
            }
            SuruVerbMizen
        })),
        reru_suffix(),
        // Match て connecting particle
        TokenMatcher::Custom(Arc::new({
            #[derive(Debug)]
            struct TeParticle;
            impl Matcher for TeParticle {
                fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
                    check_token(ctx, |token| {
                        token.surface == "て"
                        && token.pos.first().is_some_and(|pos| pos == "助詞")
                        && token.pos.get(1).is_some_and(|pos| pos == "接続助詞")
                    })
                }
            }
            TeParticle
        })),
        verb_base("いる"),
    ]
}
// Pattern: の1 (nominalization - that which / the one who)
// Structures: Verb + の + は/が
//
// Meaning: Nominalizes verb clauses to use them as nouns
// Examples: 食べるのは (the one that eats), 乗るのが (that which is riding)
//
// Key distinction from N5 の:
// - N5 の: Pronoun replacement (私の "mine") - can stand alone
// - N4 の1: Nominalization (食べるのは) - MUST be followed by は or が
pub fn no1() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match の as dependent noun (nominalizer)
    #[derive(Debug)]
    struct NoNominalizerMatcher;
    impl Matcher for NoNominalizerMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立") => (true, 1),
                _ => (false, 0),
            }
        }
    }

    vec![
        TokenMatcher::Custom(Arc::new(NoNominalizerMatcher)),
        // Match は or が following の
        or(vec![surface("は"), surface("が")]),
    ]
}
// Pattern: と2 (quotation particle)
// Structures: Phrase + と + Verb (言う, 思う, 聞く, etc.)
//
// Meaning: Quotation/citation marker, shows what was said/thought/heard
// Examples: 「危ない！」と叫んだ (yelled "Watch out!"), 猫だと思う (think it's a cat)
//
// Note: と is tokenized as 助詞/格助詞/引用 (quotation particle)
// The verb following と is often omitted at sentence end
pub fn to2() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match と as quotation particle
    #[derive(Debug)]
    struct ToQuotationMatcher;
    impl Matcher for ToQuotationMatcher {
        fn matches(&self, ctx: &MatchContext) -> (bool, usize) {
            match ctx.current() {
                Some(token) if token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "引用") => {
                    // Exclude 二度と (never again) - fixed expression, not quotation
                    if let Some(prev) = ctx.lookbehind(1) {
                        if prev.surface == "度" {
                            if let Some(prev2) = ctx.lookbehind(2) {
                                if prev2.surface == "二" {
                                    return (false, 0);
                                }
                            }
                        }
                    }
                    (true, 1)
                },
                _ => (false, 0),
            }
        }
    }

    vec![TokenMatcher::Custom(Arc::new(ToQuotationMatcher))]
}
