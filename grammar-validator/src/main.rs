use grammar_lib::PatternCategory;
use kagome_client::KagomeServer;
use serde::{Deserialize, Serialize};
use std::io::{self, BufRead, Write};

const KAGOME_PORT: u16 = 6063;

#[derive(Deserialize)]
struct Request {
    sentence: String,
    expected_pattern: String,
}

#[derive(Serialize)]
struct Response {
    valid: bool,
    detected_patterns: Vec<DetectedPattern>,
}

#[derive(Serialize)]
struct DetectedPattern {
    name: String,
    matched_text: String,
    confidence: f32,
    category: String,
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize grammar-lib statics before starting Kagome to avoid CPU contention
    grammar_lib::initialize();

    let server = KagomeServer::start(KAGOME_PORT)?;

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        let req: Request = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                let err = serde_json::json!({"error": format!("Invalid JSON: {}", e)});
                writeln!(stdout, "{}", err)?;
                stdout.flush()?;
                continue;
            }
        };

        let kagome_tokens = match server.tokenize(&req.sentence, "normal") {
            Ok(t) => t,
            Err(e) => {
                let err = serde_json::json!({"error": format!("Tokenization failed: {}", e)});
                writeln!(stdout, "{}", err)?;
                stdout.flush()?;
                continue;
            }
        };

        let tokens: Vec<grammar_lib::KagomeToken> =
            kagome_tokens.into_iter().map(convert_token).collect();

        let analysis = grammar_lib::analyze(&req.sentence, &tokens);

        let detected_patterns: Vec<DetectedPattern> = analysis
            .grammar_matches
            .iter()
            .map(|m| DetectedPattern {
                name: m.pattern_name.to_string(),
                matched_text: grammar_lib::pattern_text(&req.sentence, m),
                confidence: m.confidence,
                category: match m.category {
                    PatternCategory::Construction => "Construction".to_string(),
                    PatternCategory::Conjugation => "Conjugation".to_string(),
                },
            })
            .collect();

        let valid = detected_patterns.iter().any(|p| p.name == req.expected_pattern);

        let response = Response {
            valid,
            detected_patterns,
        };

        writeln!(stdout, "{}", serde_json::to_string(&response)?)?;
        stdout.flush()?;
    }

    drop(server);
    Ok(())
}
