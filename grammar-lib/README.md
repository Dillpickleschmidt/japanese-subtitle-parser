# Grammar Library

Shared grammar pattern detection library for Japanese language learning.

## Building and Deploying

### Build WASM

```bash
cd grammar-wasm
wasm-pack build --target web --out-dir pkg
# or possibly:
~/.cargo/bin/wasm-pack build --target web --out-dir pkg
```

Note: `wasm-pack` is installed in `~/.cargo/bin/`. You can add this to your PATH or use the full path above.

### Build CLI

```bash
cd grammar-lib
cargo build --release --bin grammar-cli
```

### Deploy to Nihongo Ninja

From project root (`japanese-subtitle-search`):

```bash
# Copy CLI
cp target/release/grammar-cli ~/Programming-Projects/nihongo-ninja/bin/

# Copy WASM files
cp grammar-wasm/pkg/grammar_wasm_bg.wasm \
   grammar-wasm/pkg/grammar_wasm.js \
   grammar-wasm/pkg/grammar_wasm.d.ts \
   ~/Programming-Projects/nihongo-ninja/public/grammar/
```

## Development

### Running Tests

```bash
# All tests
cargo test

# Specific level
cargo test n5_patterns
cargo test n4_patterns

# Single test with debug output
cargo test test_name -- --nocapture
```

### Adding New Patterns

1. Create test first in `src-tauri/src/tests/grammar/nX_patterns.rs`
2. Run test to see Kagome tokenization with `print_debug()`
3. Add pattern component to `grammar-lib/src/pattern_components.rs` if reusable
4. Add pattern to `grammar-lib/src/patterns/nX.rs`
5. Add ConjugationPattern enum variant to `grammar-lib/src/types.rs` if needed
6. Verify tests pass
7. Rebuild and deploy (see above)

See `GRAMMAR_PATTERN_STANDARDS.md` for detailed guidelines.
