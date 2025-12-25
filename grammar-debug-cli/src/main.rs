use grammar_lib::{get_jlpt_level, pattern_text, strip_parentheses, PatternCategory};
use kagome_client::KagomeServer;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

#[derive(Debug, Serialize)]
struct PatternMatch {
    name: String,
    start: u32,
    end: u32,
    jlpt: String,
    matched_text: String,
    confidence: f32,
    category: String,
}

#[derive(Debug, Serialize)]
struct SubtitleAnalysis {
    text: String,
    patterns: Vec<PatternMatch>,
}

fn convert_token(t: kagome_client::KagomeToken) -> grammar_lib::KagomeToken {
    grammar_lib::KagomeToken {
        id: t.id,
        start: t.start,
        end: t.end,
        surface: t.surface,
        class: t.class,
        pos: t.pos,
        base_form: t.base_form,
        reading: t.reading,
        pronunciation: t.pronunciation,
        features: t.features,
    }
}

fn parse_srt(content: &str) -> Vec<(usize, String)> {
    let content = content.trim_start_matches('\u{feff}');
    let content = if content.contains('\r') {
        content.replace('\r', "")
    } else {
        content.to_string()
    };

    let mut subtitles = Vec::new();

    enum State {
        ExpectingNumber,
        ExpectingTimestamp,
        ReadingText,
    }

    let mut state = State::ExpectingNumber;
    let mut current_number = 0usize;
    let mut text_lines: Vec<&str> = Vec::new();

    for line in content.lines() {
        let line = line.trim();

        match state {
            State::ExpectingNumber => {
                if line.is_empty() {
                    continue;
                }
                if let Ok(num) = line.parse::<usize>() {
                    current_number = num;
                    state = State::ExpectingTimestamp;
                }
            }
            State::ExpectingTimestamp => {
                if line.contains(" --> ") {
                    state = State::ReadingText;
                    text_lines.clear();
                }
            }
            State::ReadingText => {
                if line.is_empty() {
                    if !text_lines.is_empty() {
                        let text = text_lines.join("\n");
                        subtitles.push((current_number, text));
                    }
                    state = State::ExpectingNumber;
                } else {
                    text_lines.push(line);
                }
            }
        }
    }

    // Handle last subtitle if file doesn't end with blank line
    if matches!(state, State::ReadingText) && !text_lines.is_empty() {
        let text = text_lines.join("\n");
        subtitles.push((current_number, text));
    }

    subtitles
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: grammar-debug-cli <path-to-srt-file>");
        std::process::exit(1);
    }

    let srt_path = Path::new(&args[1]);
    if !srt_path.exists() {
        eprintln!("Error: File not found: {}", srt_path.display());
        std::process::exit(1);
    }

    // Start Kagome server on port 6062 to avoid conflict with dev server (6061)
    let server = KagomeServer::start(6062)?;

    // Parse SRT file
    let content = fs::read_to_string(srt_path)?;
    let subtitles = parse_srt(&content);

    // Analyze each subtitle
    let mut results: HashMap<String, SubtitleAnalysis> = HashMap::new();

    for (number, text) in subtitles {
        // Strip parentheses before tokenizing
        let (stripped, char_map) = strip_parentheses(&text);

        // Tokenize stripped text
        let kagome_tokens = server.tokenize(&stripped, "normal")?;

        // Convert tokens
        let tokens: Vec<grammar_lib::KagomeToken> =
            kagome_tokens.into_iter().map(convert_token).collect();

        // Analyze, remapping positions if parentheses were stripped
        let analysis = match &char_map {
            Some(map) => grammar_lib::analyze_and_remap(&stripped, &tokens, map),
            None => grammar_lib::analyze(&stripped, &tokens),
        };

        // Collect pattern matches
        let patterns: Vec<PatternMatch> = analysis
            .grammar_matches
            .iter()
            .map(|m| PatternMatch {
                name: m.pattern_name.to_string(),
                start: m.start_char,
                end: m.end_char,
                jlpt: get_jlpt_level(m.pattern_name).to_string(),
                matched_text: pattern_text(&text, m),
                confidence: m.confidence,
                category: match m.category {
                    PatternCategory::Construction => "Construction".to_string(),
                    PatternCategory::Conjugation => "Conjugation".to_string(),
                },
            })
            .collect();

        results.insert(
            number.to_string(),
            SubtitleAnalysis {
                text: text.clone(),
                patterns,
            },
        );
    }

    // Write JSON to file (same name as input but with .json extension)
    let output_path = srt_path.with_extension("json");
    let mut file = File::create(&output_path)?;
    file.write_all(serde_json::to_string_pretty(&results)?.as_bytes())?;

    eprintln!("Wrote {} subtitles to {}", results.len(), output_path.display());

    Ok(())
}
