![Application Image](./src-tauri/data/readme/image.png)

# Purpose

Find all dialogue lines containing a given keyword, filtered by show. Also retrieves surrounding lines for context.

<sub>\*The number of surrounding context lines can be modified easily.</sub>

## Rust + Solid via Tauri

This project makes use of Rust for fast transcript processing and Typescript + SolidJs for the frontend. It uses a local SQLite database for storage and [kagome](https://github.com/ikawaha/kagome) for Japanese morphological analysis.

## Prerequisites

- [Go](https://golang.org/dl/) (required for kagome)
- [kagome](https://github.com/ikawaha/kagome): `go install github.com/ikawaha/kagome/v2@latest`

## Getting Started

To start the development server, run:

```bash
bun install
bun run tauri dev
```

The application processes Japanese subtitle files directly using kagome for morphological analysis. Simply:

1. Select your subtitle directory (containing .srt files organized by show)
2. Click "Parse Subtitles" to process files and extract words
3. Click "Create Reverse Index" to build search indexes
4. Search for any Japanese word to find all occurrences with context

The app creates reverse indexes for extremely fast word searches across all your subtitle files.

More detailed setup instructions are provided in the application UI.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Additional Notes

Transcript file names must contain seasons and episodes formatted like such:

- S01E01
  ('S' followed by any number of digits and 'E' followed by any number of digits; the file names can have any surrounding text you want)
- ShowName.S03E134.EpisodeName would be fine :)

The folder names are used for extracting show names. Here's the expected folder structure:

```
transcripts/
├── Inuyasha/
│   ├── Inuyasha.S01E001.srt
│   └── Inuyasha.S01E002.srt
├── Chainsaw Man/
│   ├── Chainsaw Man.S01E01.srt
│   └── Chainsaw Man.S01E02.srt
└── [other shows]/
    └── ...
```

There's some limited support for other formats but you'd have to specify the format for each show. See `episode_info.rs` and `show_configs.rs` files to add more.

## License

TODO
