#!/bin/bash
# Grammar-lib development environment setup

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "=== Grammar-lib Environment Setup ==="

# Check Kagome server
echo "Checking Kagome server on port 6061..."
if lsof -i :6061 > /dev/null 2>&1; then
  echo "✓ Kagome server is running on port 6061"
else
  echo "✗ Kagome server not running on port 6061"
  echo "  Please start Kagome server on port 6061"
  echo "  Example: kagome server -http :6061"
  exit 1
fi

# Build the library
echo ""
echo "Building grammar-lib..."
cargo build --manifest-path "$SCRIPT_DIR/grammar-lib/Cargo.toml" 2>&1 | tail -20
BUILD_EXIT=$?
if [ $BUILD_EXIT -eq 0 ]; then
  echo "✓ Build successful"
else
  echo "✗ Build failed (exit code: $BUILD_EXIT)"
  exit 1
fi

# Count current test status
echo ""
echo "=== Current Status ==="
TEST_OUTPUT=$(cargo test --manifest-path "$SCRIPT_DIR/grammar-lib/Cargo.toml" 2>&1)
TOTAL_TESTS=$(echo "$TEST_OUTPUT" | grep -E "^test result" | grep -oE "[0-9]+ passed" | grep -oE "[0-9]+" || echo "0")
echo "Tests passing: $TOTAL_TESTS"

echo ""
echo "=== Ready for Development ==="
echo "Commands:"
echo "  cargo test                           # Run all tests"
echo "  cargo test test_NAME                 # Run specific test"
echo "  cargo test test_NAME -- --nocapture  # Run with output"
echo "  cargo test n3_patterns               # Run all N3 tests"
echo ""
echo "Pattern count: 924 patterns defined in patterns.rs"
