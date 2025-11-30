# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Run Commands

```bash
# Run all tests
cargo test

# Run specific JLPT level tests
cargo test n5_patterns
cargo test n4_patterns

# Single test with debug output
cargo test test_name -- --nocapture

# Dev server (Tauri + SolidJS)
bun install && bun run tauri dev
```

## Architecture

Tauri 2.0 desktop app for Japanese subtitle search with grammar pattern detection.

- **`grammar-lib/`** - Pattern matching library (the core logic)
- **`src-tauri/src/tests/grammar/`** - Pattern tests (N5-N1 levels)
- **`src-tauri/`** - Tauri backend
- **`src/`** - SolidJS frontend

## Grammar Pattern Implementation

When implementing patterns to pass failing tests, modify these files in `grammar-lib/src/`:

### 1. `patterns.rs` - Pattern Definitions

Uses `declare_patterns!` macro. Each pattern needs:

- Enum variant name
- `name`: string identifier (use suffixes for variants: `"meku"` vs `"meku_compound"`)
- `matcher_fn`: function returning `Vec<TokenMatcher>`
- `priority`: higher = more specific (Construction patterns with priority < 5 are matched but not highlighted in browser extension overlays - useful for common patterns like XはYです)
- `category`: `PatternCategory::Construction` or `PatternCategory::Conjugation`
- `jlpt`: level string (`"n5"`, `"n4"`, etc.)

### 2. `matchers/nX.rs` - Matcher Functions

Each JLPT level has its own file. Matcher functions return `Vec<TokenMatcher>`.

**TokenMatcher types:**

- `TokenMatcher::Verb { conjugation_form, base_form }` - Match verbs
- `TokenMatcher::Surface("text")` - Exact surface match
- `TokenMatcher::Custom(Arc::new(MatcherStruct))` - Custom logic
- `TokenMatcher::Optional(Box::new(...))` - Optional token
- `TokenMatcher::Wildcard { min, max, stop_conditions }` - Skip tokens

**Custom matchers** implement the `Matcher` trait:

```rust
#[derive(Debug)]
struct MyMatcher;
impl Matcher for MyMatcher {
    fn matches(&self, token: &KagomeToken) -> bool {
        // Check token.surface, token.base_form, token.pos, token.features
    }
}
TokenMatcher::Custom(Arc::new(MyMatcher))
```

### 3. `matchers/mod.rs` - Shared Helpers

Common matchers reused across levels: `flexible_verb_form()`, `past_auxiliary()`, `noun_matcher()`, `particle_matcher()`, etc.

### Pattern Comment Format

```rust
// Pattern: Meaning (example)
// Structures: Form1 + Form2 + Form3
```
