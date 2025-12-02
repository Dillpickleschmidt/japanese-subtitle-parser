use crate::pattern_matcher::{PatternCategory, PatternMatch};
use crate::types::KagomeToken;

/// Combines tokens covered by Conjugation patterns into single tokens.
/// For example: [拝み, たかっ, た] → [拝みたかった]
///
/// The combined token uses:
/// - surface: the combined text from the pattern range
/// - base_form: from the verb token (first token with pos[0] == "動詞"), or first token
/// - pos: from the verb token, or first token
/// - start/end: adjusted character positions
pub fn combine_conjugation_tokens(
    text: &str,
    tokens: &[KagomeToken],
    pattern_matches: &[PatternMatch],
) -> Vec<KagomeToken> {
    // Filter to Conjugation patterns only
    let conjugation_matches: Vec<_> = pattern_matches
        .iter()
        .filter(|m| m.category == PatternCategory::Conjugation)
        .collect();

    // Select best (non-redundant) patterns by confidence
    let best_conjugations = select_best_patterns(&conjugation_matches);

    // Sort by start_char for single-pass algorithm
    let mut sorted_patterns = best_conjugations;
    sorted_patterns.sort_by_key(|p| p.start_char);

    if sorted_patterns.is_empty() {
        return tokens.to_vec();
    }

    let mut result = Vec::new();
    let mut pattern_index = 0;
    let mut token_index = 0;

    while token_index < tokens.len() {
        let token = &tokens[token_index];

        // Skip patterns that end before this token
        while pattern_index < sorted_patterns.len()
            && sorted_patterns[pattern_index].end_char <= token.start
        {
            pattern_index += 1;
        }

        // Check if current token overlaps with current pattern
        let pattern = sorted_patterns.get(pattern_index);

        if pattern.is_none() || token.end <= pattern.unwrap().start_char {
            // No overlap - add token as-is
            result.push(token.clone());
            token_index += 1;
            continue;
        }

        let pattern = pattern.unwrap();

        // Token overlaps with pattern - handle overlap

        // If token starts before pattern, split and add the before part
        if token.start < pattern.start_char && token.end > pattern.start_char {
            let before_text = get_text_range(text, token.start, pattern.start_char);
            if !before_text.is_empty() {
                let before_token = create_split_token(token, token.start, pattern.start_char, &before_text);
                result.push(before_token);
            }
        }

        // Find all tokens fully or partially contained in this pattern
        let mut contained_tokens: Vec<&KagomeToken> = Vec::new();
        let mut pattern_end_token_index = token_index;

        while pattern_end_token_index < tokens.len()
            && tokens[pattern_end_token_index].start < pattern.end_char
        {
            let t = &tokens[pattern_end_token_index];

            if t.start >= pattern.start_char && t.end <= pattern.end_char {
                // Fully contained
                contained_tokens.push(t);
            } else if t.start < pattern.end_char && t.end > pattern.end_char {
                // Partially extends past pattern end - we'll handle the after part later
                contained_tokens.push(t);
            } else if t.start >= pattern.start_char {
                // Fully contained
                contained_tokens.push(t);
            }

            pattern_end_token_index += 1;
        }

        // Create combined token from pattern range
        if !contained_tokens.is_empty() {
            let combined_surface = get_text_range(text, pattern.start_char, pattern.end_char);
            let first_token = contained_tokens[0];

            // Prefer verb POS if any token in the group is a verb (e.g., 勉強します)
            // This ensures suru-verbs show as verbs, not nouns
            let verb_token = contained_tokens
                .iter()
                .find(|t| t.pos.first().map(|p| p.as_str()) == Some("動詞"));
            let reference_token = verb_token.unwrap_or(&first_token);

            let combined_token = KagomeToken {
                id: first_token.id,
                start: pattern.start_char,
                end: pattern.end_char,
                surface: combined_surface,
                class: first_token.class.clone(),
                pos: reference_token.pos.clone(),
                base_form: reference_token.base_form.clone(),
                reading: first_token.reading.clone(),
                pronunciation: first_token.pronunciation.clone(),
                features: first_token.features.clone(),
            };
            result.push(combined_token);
        }

        // Handle token that extends past pattern end
        if pattern_end_token_index > 0 {
            let last_overlapping_token = &tokens[pattern_end_token_index - 1];
            if last_overlapping_token.end > pattern.end_char {
                let after_text = get_text_range(text, pattern.end_char, last_overlapping_token.end);
                if !after_text.is_empty() {
                    let after_token = create_split_token(
                        last_overlapping_token,
                        pattern.end_char,
                        last_overlapping_token.end,
                        &after_text,
                    );
                    result.push(after_token);
                }
            }
        }

        token_index = pattern_end_token_index;
    }

    result
}

/// Select non-redundant patterns by confidence, within each category separately.
/// Construction and Conjugation patterns are filtered independently.
pub fn select_best_patterns<'a>(matches: &[&'a PatternMatch]) -> Vec<&'a PatternMatch> {
    if matches.is_empty() {
        return Vec::new();
    }

    // Separate by category
    let constructions: Vec<_> = matches
        .iter()
        .filter(|m| m.category == PatternCategory::Construction)
        .copied()
        .collect();
    let conjugations: Vec<_> = matches
        .iter()
        .filter(|m| m.category == PatternCategory::Conjugation)
        .copied()
        .collect();

    // Select best within each category
    let mut selected = select_best_within_category(&constructions);
    selected.extend(select_best_within_category(&conjugations));

    selected
}

/// Select non-redundant patterns within a single category.
fn select_best_within_category<'a>(matches: &[&'a PatternMatch]) -> Vec<&'a PatternMatch> {
    if matches.is_empty() {
        return Vec::new();
    }

    // Sort by confidence descending (highest confidence first)
    let mut sorted: Vec<_> = matches.to_vec();
    sorted.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());

    let mut selected: Vec<&PatternMatch> = Vec::new();

    for m in sorted {
        // Skip if completely contained in a higher-confidence match
        let is_redundant = selected.iter().any(|s| {
            s.start_char <= m.start_char
                && s.end_char >= m.end_char
                && s.confidence > m.confidence
        });

        if !is_redundant {
            selected.push(m);
        }
    }

    selected
}

/// Get text substring by character positions (not byte positions)
fn get_text_range(text: &str, start_char: u32, end_char: u32) -> String {
    text.chars()
        .skip(start_char as usize)
        .take((end_char - start_char) as usize)
        .collect()
}

/// Create a split token with adjusted positions and text
fn create_split_token(
    original: &KagomeToken,
    new_start: u32,
    new_end: u32,
    new_surface: &str,
) -> KagomeToken {
    // Approximate reading/pronunciation split proportionally
    let original_len = original.surface.chars().count();
    let new_len = new_surface.chars().count();
    let ratio = if original_len > 0 {
        new_len as f32 / original_len as f32
    } else {
        1.0
    };

    let reading_len = (original.reading.chars().count() as f32 * ratio).ceil() as usize;
    let pronunciation_len =
        (original.pronunciation.chars().count() as f32 * ratio).ceil() as usize;

    KagomeToken {
        id: original.id,
        start: new_start,
        end: new_end,
        surface: new_surface.to_string(),
        class: original.class.clone(),
        pos: original.pos.clone(),
        base_form: original.base_form.clone(),
        reading: original.reading.chars().take(reading_len).collect(),
        pronunciation: original.pronunciation.chars().take(pronunciation_len).collect(),
        features: original.features.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_token(surface: &str, base_form: &str, start: u32, end: u32, pos: &str) -> KagomeToken {
        KagomeToken {
            id: 0,
            start,
            end,
            surface: surface.to_string(),
            class: String::new(),
            pos: vec![pos.to_string()],
            base_form: base_form.to_string(),
            reading: String::new(),
            pronunciation: String::new(),
            features: vec![],
        }
    }

    fn make_pattern(start_char: u32, end_char: u32, confidence: f32) -> PatternMatch {
        PatternMatch {
            confidence,
            pattern_name: "test",
            category: PatternCategory::Conjugation,
            start_char,
            end_char,
        }
    }

    #[test]
    fn test_combine_simple() {
        let text = "見ていた";
        let tokens = vec![
            make_token("見", "見る", 0, 1, "動詞"),
            make_token("て", "て", 1, 2, "助詞"),
            make_token("い", "いる", 2, 3, "動詞"),
            make_token("た", "た", 3, 4, "助動詞"),
        ];
        let patterns = vec![make_pattern(0, 4, 10.0)];

        let result = combine_conjugation_tokens(text, &tokens, &patterns);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].surface, "見ていた");
        assert_eq!(result[0].base_form, "見る"); // From verb token
    }

    #[test]
    fn test_no_patterns() {
        let text = "食べる";
        let tokens = vec![make_token("食べる", "食べる", 0, 3, "動詞")];
        let patterns: Vec<PatternMatch> = vec![];

        let result = combine_conjugation_tokens(text, &tokens, &patterns);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].surface, "食べる");
    }

    #[test]
    fn test_select_best_patterns() {
        let p1 = make_pattern(0, 4, 10.0);
        let p2 = make_pattern(0, 2, 5.0); // Contained in p1, lower confidence

        let matches: Vec<&PatternMatch> = vec![&p1, &p2];
        let selected = select_best_patterns(&matches);

        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].confidence, 10.0);
    }
}
