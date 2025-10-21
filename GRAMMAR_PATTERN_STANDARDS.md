# Grammar Pattern & Test Code Standards

Reference guide for developing and testing grammar patterns across JLPT levels.

## Test Helpers - Keep Them Simple & Focused

Maintain a minimal set of reusable helpers:

- **`assert_has_pattern()`** - Verify pattern exists
- **`assert_pattern_range()`** - Verify exact character range (start, end)
- **`pattern_text()`** - Utility to extract/inspect matched text

**Rule:** Never create overlapping helper functions with similar purposes.

## Pattern Range Testing - Always Test Both

- Test pattern **existence** AND **character ranges** together
- Use character positions (not byte offsets)
- Verify extracted text matches expected content

**Pattern Match Structure:**
- `start_char` and `end_char` are 0-indexed character positions
- Convert character positions to byte positions for UTF-8 extraction
- Always verify ranges in tests to catch tokenization bugs

## Multiple Patterns for One Concept - Use Unique Names

When one grammar concept has multiple patterns (different tokenizations), use suffixes:

**Example:**
- `"meku"` → Split tokenization (Noun + Verb)
- `"meku_compound"` → Single token compounds (like 春めく)

**Why:**
- Allows independent testing and verification
- Catches implementation bugs (both patterns may match same input)
- Tests can verify each variant's character ranges separately
- Better debugging clarity

**Rule:** Never reuse the same pattern name for different implementations.

## Custom Matchers - Make Them Reusable

Design matchers for reuse across multiple patterns:

**Example: VerbWithBaseSuffix**
```rust
CustomMatcher::VerbWithBaseSuffix("suffix")
```

**Logic:**
- Match verbs where `base_form.ends_with(suffix)`
- Exclude exact matches: `base_form != suffix`
- Works for any suffix (めく, etc.)

**Example: NounWithBaseSuffix**
```rust
CustomMatcher::NounWithBaseSuffix("suffix")
```

**Logic:**
- Match nouns where `base_form.ends_with(suffix)`
- Exclude exact matches: `base_form != suffix`
- Works for any noun suffix (まみれ, etc.)

**Implementation Pattern:**
```rust
// For verbs
token.pos.first().is_some_and(|pos| pos == "動詞")
    && token.base_form.ends_with(suffix)
    && token.base_form != *suffix

// For nouns
token.pos.first().is_some_and(|pos| pos == "名詞")
    && token.base_form.ends_with(suffix)
    && token.base_form != *suffix
```

## Pattern Comments - Concise & Structured

Keep comments minimal and focused:

**Keep only:**
- Grammar meaning
- One representative example
- Applicable linguistic structures

**Format:**
```rust
// Pattern: Meaning (example)
// Structures: Form1 + Form2 + Form3
```

**Examples:**
```rust
// めく (split): Shows signs of (謎めく - mysterious)
// Structures: Noun + めく/めいて/めいた/めいている

// めく (compound): Single-token compounds (春めく - spring-like)
// Structures: Noun[Season] + めく/めいてきた/めいている (single token)
```

**Remove:**
- Multiple examples
- Framework implementation details ("Kagome may tokenize as...")
- Obvious behavior statements
- Implementation-specific notes visible in code

## Test Organization

**File Organization:**
- Group all tests for a pattern together (same file)
- Group related pattern definitions together
- Avoid scattering related logic across files

**Test Grouping with Modules:**
Use nested test modules to group related tests. This creates readable output organization.

```rust
mod pattern_tests {
    use super::*;

    #[test]
    fn variant_one() {
        let sentence = "example sentence";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "pattern_name");
        assert_pattern_range(&patterns, "pattern_name", start, end);
    }

    #[test]
    fn variant_two() {
        // test code
    }
}
```

**Test Output:**
```
test pattern_tests::variant_one ... ok
test pattern_tests::variant_two ... ok
```

This makes it easy to see which tests belong to each pattern at a glance.

**Debug Output:**
Do NOT include `print_debug()` in standard tests. It clutters output and is only useful when debugging failing tests.

When needed for debugging:
1. Add `print_debug(sentence, &tokens, &patterns);` temporarily
2. Run test with: `cargo test pattern_tests::variant_one -- --nocapture`
3. Remove after debugging

This keeps test output clean by default while allowing detailed inspection when needed.

## Documentation in Pattern Code

**Pattern comments should document:**
- Linguistic structures the pattern matches
- All conjugation forms handled (e.g., めく/めいて/めいた/めいている)
- Tokenization variations if multiple patterns exist (split vs compound)

**Example:**
```rust
// めく (split): Shows signs of (謎めく - mysterious)
// Structures: Noun + めく/めいて/めいた/めいている
(
    GrammarPattern {
        name: "meku",
        tokens: vec![
            TokenMatcher::Custom(CustomMatcher::Noun),
            TokenMatcher::Verb {
                conjugation_form: None,
                base_form: Some("めく"),
            },
        ],
        priority: 6,
        category: PatternCategory::Construction,
    },
    ConjugationPattern::Meku,
    "n1",
),
```

## Key Principles

1. **Explicitness over convenience** - Unique names and clear tests catch bugs
2. **Reusability** - Design matchers and helpers for multiple use cases
3. **Conciseness** - Comments should inform, not repeat what code shows
4. **Testability** - Multiple variants need separate test verification
5. **Clarity** - Character ranges and tokenization must be explicit in tests
