# Reference Pattern Implementation Examples

These are exemplary implementations to follow when creating new grammar patterns.
Study these examples carefully before implementing any pattern.

## Key Concepts

### TokenMatcher Types
- `TokenMatcher::Surface("text")` - Exact surface form match
- `TokenMatcher::Verb { conjugation_form, base_form }` - Match verbs by conjugation
- `TokenMatcher::verb_with_form("連用形")` - Shorthand for verb conjugation
- `TokenMatcher::specific_verb("いる")` - Match specific verb by base form
- `TokenMatcher::Adjective { base_form }` - Match adjectives
- `TokenMatcher::Custom(Arc::new(Matcher))` - Custom matching logic
- `TokenMatcher::Optional(Box::new(...))` - Optional token
- `TokenMatcher::Wildcard { min, max, stop_conditions }` - Skip 0-N tokens

### Helper Functions (in mod.rs)
- `concat(vec![...])` - Combine multiple token sequences
- `optional(vec![...])` - Make all tokens in sequence optional
- `flexible_verb_form()` - Match verb in 連用形 or 連用タ接続
- `past_auxiliary()` - Match た or だ as past tense marker
- `noun_matcher()` - Match any noun
- `particle_matcher()` - Match any particle

### Pattern Range Boundaries

Pattern ranges should include the **full grammatical construction**:

- Verb patterns: Include the verb + suffix (e.g., `見たい`, not just `たい`)
- Noun + particle patterns: Include the noun (e.g., `俺も`, not just `も`)
- Multi-token patterns: Include all tokens that form the grammatical unit

Note: The `<target-japanese>` tags in `grammar_points_data.json` show minimal ranges for highlighting purposes.
Our detection ranges are intentionally wider to capture the full grammatical context.

**Examples from this file:**
- `tai_form`: `見たい` (7, 10) - verb stem + たい
- `te_iru`: `考えている` (10, 15) - verb + て + いる
- `mo_also`: `俺も` (0, 2) - noun + も
- `tara_conditional`: `気づいていたら` (5, 11) - verb + ていた + ら
- `ta_koto_ga_aru`: `会ったことがある` (7, 16) - full construction

---

## Example 1: Simple Verb Conjugation - `tai_form` (Want to do)

**Pattern:** Verb連用形 + たい
**Data source:** `jq '.["たい"]' grammar_points_data.json`

### Matcher (matchers/n5.rs)
```rust
/// Match たい (desire form as adjective or auxiliary)
pub fn tai_form_matcher() -> TokenMatcher {
    #[derive(Debug)]
    struct TaiFormMatcher;
    impl Matcher for TaiFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "たい"
                && (token.pos.first().is_some_and(|pos| pos == "形容詞")
                    || token.pos.first().is_some_and(|pos| pos == "助動詞"))
        }
    }
    TokenMatcher::Custom(Arc::new(TaiFormMatcher))
}

pub fn tai_form() -> Vec<TokenMatcher> {
    vec![TokenMatcher::verb_with_form("連用形"), tai_form_matcher()]
}
```

### Test (tests/n5_patterns.rs)
```rust
// Pattern: たい (want to do)
// Data source: grammar_points_data.json["たい"]
// Testing: structure.standard[0] - "Verb[stem] + たい"
//
// Other structures to test:
//   - standard[1]: Verb[stem] + たくない (negative)
//   - standard[2]: Verb[stem] + たかった (past)
//   - standard[3]: Verb[stem] + たくなかった (past negative)
//   - polite[0-3]: Same forms + です variants
#[test]
fn test_tai_form_non_past() {
    let sentence = "あの映画、絶対に見たいんだけど";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "tai_form");
    assert_pattern_range(&patterns, "tai_form", 7, 10); // 見たい
    assert_pattern_selected(&patterns, "tai_form");
}
```

### Key Points
- Custom matcher checks both surface AND part-of-speech
- Range includes verb stem (見) + auxiliary (たい)
- **Each structure variant needs its own test** (negative, past, polite, etc.)

---

## Example 2: Te-form Construction - `te_iru` (Progressive/Resultative)

**Pattern:** Verb て-form + いる
**Data source:** `jq '.["ている①"]' grammar_points_data.json`

### Matcher (matchers/n5.rs)
```rust
pub fn te_de_form() -> TokenMatcher {
    #[derive(Debug)]
    struct TeDeFormMatcher;
    impl Matcher for TeDeFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "て" || token.surface == "で"
        }
    }
    TokenMatcher::Custom(Arc::new(TeDeFormMatcher))
}

pub fn te_form() -> Vec<TokenMatcher> {
    concat(vec![vec![flexible_verb_form()], vec![te_de_form()]])
}

pub fn te_iru() -> Vec<TokenMatcher> {
    concat(vec![te_form(), vec![TokenMatcher::specific_verb("いる")]])
}
```

### Test (tests/n5_patterns.rs)
```rust
// Pattern: ている (progressive/resultative)
// Data source: grammar_points_data.json["ている"]
// Testing: structure.standard[0] - "Verb[て] + いる"
//
// Other structures to test:
//   - standard[1]: Verb[て] + いない (negative)
//   - standard[2]: Verb[て] + いた (past)
//   - standard[3]: Verb[て] + いなかった (past negative)
//   - polite[0-3]: Verb[て] + います/いません/いました/いませんでした
#[test]
fn test_te_iru_progressive() {
    let sentence = "最近ずっとあのことを考えている";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "te_iru");
    assert_pattern_range(&patterns, "te_iru", 10, 15); // 考えている
    assert_pattern_selected(&patterns, "te_iru");
}
```

### Key Points
- Reuses `te_form()` helper - compose patterns from smaller parts
- `flexible_verb_form()` handles both 連用形 and 連用タ接続
- **Test all conjugations**: ている, ていない, ていた, ていなかった, ています, etc.

---

## Example 3: Conditional Form - `tara_conditional`

**Pattern:** Verb + たら/だら
**Data source:** `jq '.["たら"]' grammar_points_data.json`

### Matcher (matchers/n4.rs)
```rust
fn tara_form() -> TokenMatcher {
    use std::sync::Arc;
    #[derive(Debug)]
    struct TaraFormMatcher;
    impl super::Matcher for TaraFormMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "たら" || token.surface == "だら")
                && (token.base_form == "た" || token.base_form == "だ")
        }
    }
    TokenMatcher::Custom(Arc::new(TaraFormMatcher))
}

pub fn tara_conditional() -> Vec<TokenMatcher> {
    concat(vec![vec![super::flexible_verb_form()], vec![tara_form()]])
}
```

### Test (tests/n4_patterns.rs)
```rust
// Pattern: たら (conditional)
// Data source: grammar_points_data.json["たら"]
// Testing: structure.standard[0] - "Verb[た] + ら"
//
// Other structures to test:
//   - standard[1]: い-Adj[かった] + ら (e.g., 寒かったら)
//   - standard[2]: な-Adj/Noun + だったら
//   - Negative forms if listed in structure array
#[test]
fn test_tara_verb_conditional() {
    let sentence = "もっと早く気づいていたら、こんなことにはならなかった";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "tara_conditional");
    assert_pattern_range(&patterns, "tara_conditional", 5, 11); // 気づいていたら
}
```

### Key Points
- Checks BOTH surface (たら/だら) AND base_form (た/だ) for accuracy
- **Test verb, adjective, and noun forms** as listed in structure variants

---

## Example 4: Particle Pattern with Exclusions - `mo_also`

**Pattern:** Noun + (particle) + も (excluding question words)
**Data source:** `jq '.["も"]' grammar_points_data.json`

### Matcher (matchers/n5.rs)
```rust
// Question words to exclude
const QUESTION_WORDS: &[&str] = &[
    "誰", "何", "どこ", "いつ", "どれ", "どちら", "どの", "なぜ", "なん",
];

fn non_question_noun_matcher() -> TokenMatcher {
    #[derive(Debug)]
    struct NonQuestionNounMatcher;
    impl Matcher for NonQuestionNounMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|p| p == "名詞")
                && !QUESTION_WORDS.contains(&token.base_form.as_str())
        }
    }
    TokenMatcher::Custom(Arc::new(NonQuestionNounMatcher))
}

fn case_particle_matcher() -> TokenMatcher {
    #[derive(Debug)]
    struct CaseParticleMatcher;
    impl Matcher for CaseParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "格助詞")
        }
    }
    TokenMatcher::Custom(Arc::new(CaseParticleMatcher))
}

fn mo_particle_matcher() -> TokenMatcher {
    #[derive(Debug)]
    struct MoParticleMatcher;
    impl Matcher for MoParticleMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "も"
                && token.pos.first().is_some_and(|p| p == "助詞")
                && token.pos.get(1).is_some_and(|p| p == "係助詞")
        }
    }
    TokenMatcher::Custom(Arc::new(MoParticleMatcher))
}

pub fn mo_also() -> Vec<TokenMatcher> {
    vec![
        non_question_noun_matcher(),
        TokenMatcher::Optional(Box::new(case_particle_matcher())),
        mo_particle_matcher(),
    ]
}
```

### Tests (tests/n5_patterns.rs)
```rust
// Pattern: も (also/too)
// Data source: grammar_points_data.json["も"]
// Testing: structure.standard[0] - "Noun + も"
//
// Note: This pattern has only one structure variant but multiple usages
// Test different contexts: pronoun + も, noun + particle + も
// NEGATIVE tests: Exclude 誰も/何も (different grammar - "nobody/nothing")
#[test]
fn pronoun_also() {
    let sentence = "俺もそう思ってたんだよ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "mo_also");
    assert_pattern_range(&patterns, "mo_also", 0, 2); // 俺も
}

#[test]
fn noun_particle_also() {
    let sentence = "こんな場所にも来たことあるの？";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "mo_also");
    assert_pattern_range(&patterns, "mo_also", 2, 6); // 場所にも
}

// NEGATIVE TEST - should NOT match (different grammar)
#[test]
fn question_dare_mo() {
    let sentence = "誰も来ないなんて寂しいな";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert!(!has_pattern(&patterns, "mo_also")); // 誰も is different grammar
}
```

### Key Points
- Excludes question words (誰も, 何も = "nobody", "nothing" - different grammar)
- Uses `TokenMatcher::Optional` for optional case particle
- **Include NEGATIVE tests** for things that look similar but shouldn't match

---

## Example 5: Complex Multi-token Pattern - `ta_koto_ga_aru`

**Pattern:** Verb た-form + こと + が + ある (experience)
**Data source:** `jq '.["たことがある"]' grammar_points_data.json`

### Matcher (matchers/n5.rs)
```rust
pub fn ta_form() -> Vec<TokenMatcher> {
    vec![flexible_verb_form(), past_auxiliary()]
}

pub fn ta_koto_ga_aru() -> Vec<TokenMatcher> {
    concat(vec![
        ta_form(),
        vec![TokenMatcher::Surface("こと")],
        vec![TokenMatcher::Surface("が")],
        vec![TokenMatcher::specific_verb("ある")],
    ])
}
```

### Test (tests/n5_patterns.rs)
```rust
// Pattern: たことがある (have experience of)
// Data source: grammar_points_data.json["たことがある"]
// Testing: structure.standard[0] - "Verb[た] + ことがある"
//
// Other structures to test:
//   - standard[1]: Verb[た] + ことがない (never done)
//   - polite forms: ことがあります, ことがありません
#[test]
fn test_ta_koto_ga_aru_affirmative() {
    let sentence = "あいつとは前に会ったことがあるはずだ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "ta_koto_ga_aru");
    assert_pattern_range(&patterns, "ta_koto_ga_aru", 7, 16); // 会ったことがある
    assert_pattern_selected(&patterns, "ta_koto_ga_aru");
}
```

### Key Points
- Composes from smaller patterns (`ta_form()`)
- Uses `concat` to chain multiple token sequences
- **Test both affirmative and negative** (ことがある vs ことがない)

---

## Example 6: Pattern with Wildcard - `mada_te_imasen`

**Pattern:** まだ + (anything) + て-form + いません
**Data source:** `jq '.["まだ～ていません"]' grammar_points_data.json`

### Matcher (matchers/n5.rs)
```rust
pub fn mada_te_imasen() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::Surface("まだ")],
        vec![TokenMatcher::Wildcard {
            min: 0,
            max: 5,
            stop_conditions: vec![],
        }],
        te_form(),
        vec![TokenMatcher::specific_verb("いる")],
        vec![masen_form()],
        vec![TokenMatcher::Surface("ん")],
    ])
}
```

### Test (tests/n5_patterns.rs)
```rust
// Pattern: まだ～ていません (haven't done yet)
// Data source: grammar_points_data.json["まだ～ていません"]
// Testing: structure.standard[0] - "まだ + Verb[て] + いない"
//
// Other structures to test:
//   - polite[0]: まだ + Verb[て] + いません
//   - Casual affirmative: まだ + Verb[て] + いる (still doing)
#[test]
fn test_mada_te_imasen_casual() {
    let sentence = "まだ何も決めていないんだって";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "mada_te_imasen");
    assert_pattern_range(&patterns, "mada_te_imasen", 0, 11); // まだ何も決めていない
    assert_pattern_selected(&patterns, "mada_te_imasen");
}
```

### Key Points
- `Wildcard { min: 0, max: 5 }` allows 0-5 tokens between まだ and て-form
- Can add `stop_conditions` to prevent matching across certain tokens

---

## Example 7: Pattern with Undetectable Variant - `rashii`

**Pattern:** Verb/Adj/Noun + らしい (seems like / typical of)
**Data source:** `jq '.["らしい ①"]' grammar_points_data.json` and `jq '.["らしい ②"]' grammar_points_data.json`

### Matcher (matchers/n4.rs)
```rust
/// Match らしい (auxiliary verb for hearsay/conjecture)
fn rashii_auxiliary() -> TokenMatcher {
    #[derive(Debug)]
    struct RashiiMatcher;
    impl Matcher for RashiiMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "らしい"
                && token.pos.first().is_some_and(|pos| pos == "助動詞")
        }
    }
    TokenMatcher::Custom(Arc::new(RashiiMatcher))
}

pub fn rashii() -> Vec<TokenMatcher> {
    // Matches Verb/Adjective/Noun + らしい
    // Note: Noun + らしい is ambiguous (could be らしい① or らしい②)
    vec![
        TokenMatcher::Any, // Verb, Adjective, or Noun
        rashii_auxiliary(),
    ]
}
```

### Test (tests/n4_patterns.rs)
```rust
// Pattern: らしい (seems like, apparently)
// Data source: grammar_points_data.json["らしい ①"] and ["らしい ②"]
// Testing: structure.standard[0] - "Verb + らしい"
//
// Detectable structures (らしい① only):
//   - standard[0]: Verb + らしい
//   - standard[1]: い-Adj + らしい
//   - standard[2]: な-Adj + らしい
//   - polite variants: + です
//
// UNDETECTABLE (らしい① vs らしい②):
//   - standard[3]: Noun + らしい (could be "seems like" OR "typical of")
#[test]
fn test_rashii_verb_hearsay() {
    let sentence = "来週から値段が上がるらしいよ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "rashii");
    assert_pattern_range(&patterns, "rashii", 7, 12); // 上がるらしい
    assert_pattern_selected(&patterns, "rashii");
}

#[test]
fn test_rashii_i_adjective() {
    let sentence = "あの店のラーメンは美味しいらしい";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "rashii");
    assert_pattern_range(&patterns, "rashii", 9, 15); // 美味しいらしい
}

// TODO: Undetectable - Noun + らしい (らしい① vs らしい②)
// Both "seems like" (hearsay) and "typical of" (characteristic) tokenize identically
// as Noun + らしい(助動詞). Only semantic context distinguishes them.
//
// #[test]
// fn test_rashii_noun_ambiguous() {
//     // らしい① "seems like": 彼は学生らしい (He seems to be a student)
//     // らしい② "typical of": 夏らしい天気だね (It's summer-like weather)
//     // Both: Noun + らしい(助動詞) - cannot distinguish structurally
//     //
//     // When Noun + らしい is detected, show BOTH grammar explanations
//     // and let the user determine meaning from context.
// }
```

### Key Points
- `Verb + らしい` and `Adjective + らしい` are **unambiguously らしい①** (hearsay)
- `Noun + らしい` is **ambiguous** - could be らしい① or らしい②
- When detecting `Noun + らしい`, show both grammar point explanations
- **Comment out undetectable tests** with TODO explaining the semantic ambiguity

---

## Test Structure Template

```rust
#[test]
fn test_pattern_name() {
    // 1. Use realistic, subtitle-quality sentence
    let sentence = "そんなこと言われても困るんだけど";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    // 2. Assert pattern exists
    assert_has_pattern(&patterns, "pattern_name");

    // 3. Assert correct character range (0-indexed)
    assert_pattern_range(&patterns, "pattern_name", start, end);

    // 4. Assert it's selected (highest priority for this position)
    assert_pattern_selected(&patterns, "pattern_name");
}
```

---

## Common Patterns to Follow

1. **Always check part-of-speech** - Don't just match surface forms
2. **Use `base_form` for lemma matching** - Handles conjugated forms
3. **Compose patterns** - Build complex patterns from simpler ones
4. **Test edge cases** - Include negative tests for things that shouldn't match
5. **Use realistic sentences** - Not textbook examples like "私は学生です"
6. **Check token.features** - Index 5 is usually conjugation form
7. **Prefer specific matchers** - `specific_verb("いる")` over generic verb match

## Debugging Workflow

1. Add `print_debug(sentence, &tokens, &patterns);` temporarily
2. Run with `cargo test test_name -- --nocapture`
3. Examine token structure (surface, base_form, pos, features)
4. Build matcher based on actual tokenization data
5. Remove `print_debug()` before committing
