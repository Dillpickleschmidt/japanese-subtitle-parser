# grammar-debug-cli

Debug CLI for analyzing grammar patterns in SRT subtitle files.

## Usage

```bash
cargo run -p grammar-debug-cli -- /path/to/subtitles.srt
```

Or build and run directly:

```bash
cargo build -p grammar-debug-cli --release
./target/release/grammar-debug-cli /path/to/subtitles.srt
```

## Output

Creates a JSON file with the same name as the input (e.g., `subtitles.srt` → `subtitles.json`).

```json
{
  "1": {
    "text": "今日はいい天気ですね",
    "patterns": [
      {
        "name": "ですね",
        "start": 7,
        "end": 10,
        "jlpt": "n5",
        "matched_text": "ですね",
        "confidence": 5.0,
        "category": "Construction"
      }
    ]
  }
}
```

## Requirements

- Kagome tokenizer must be installed and available in PATH
- The CLI automatically starts/stops a Kagome server on port 6062
