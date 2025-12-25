//! Tests for parentheses stripping and position remapping

use super::{detect_patterns_with_parens, has_pattern};

#[test]
fn test_no_match_inside_fullwidth_parens() {
    // Sound effect notation - entire content is in parentheses
    let matches = detect_patterns_with_parens("（セミの鳴き声）");
    assert!(
        !has_pattern(&matches, "の"),
        "Should not match の inside parentheses"
    );
}

#[test]
fn test_no_match_inside_ascii_parens() {
    let matches = detect_patterns_with_parens("(test)");
    assert!(matches.is_empty(), "Should not match anything inside ASCII parens");
}

#[test]
fn test_match_outside_parens() {
    // Pattern outside parens should still match
    let matches = detect_patterns_with_parens("私の本（ほん）です");
    assert!(
        has_pattern(&matches, "の"),
        "Should match の outside parentheses"
    );
}

#[test]
fn test_inline_furigana_verb() {
    // Verb with inline furigana: 食（た）べる -> stripped to 食べる
    let matches = detect_patterns_with_parens("食（た）べる");
    assert!(
        has_pattern(&matches, "る-Verb (Dictionary)"),
        "Should match verb pattern across parentheses: {:?}",
        matches
    );
}

#[test]
fn test_strip_preserves_positions() {
    // Original: 私の本（ほん）です - 9 chars (indices 0-8)
    // Stripped: 私の本です - 5 chars
    // char_map: [0, 1, 2, 7, 8] - maps stripped indices to original
    let original = "私の本（ほん）です";
    let matches = super::detect_patterns_with_parens(original);

    // の pattern matches 私の (chars 0-2 in original)
    let no_match = matches.iter().find(|m| m.pattern_name == "の").unwrap();
    assert_eq!(no_match.start_char, 0, "の pattern should start at char 0");
    assert_eq!(no_match.end_char, 2, "の pattern should end at char 2");

    // です pattern matches 本です (noun + copula) = chars 2-9 in original
    // (本 is at char 2, です spans chars 7-8, exclusive end is 9)
    let desu_match = matches.iter().find(|m| m.pattern_name == "です").unwrap();
    assert_eq!(desu_match.start_char, 2, "です pattern should start at char 2 (本)");
    assert_eq!(desu_match.end_char, 9, "です pattern should end at char 9");
}
