// Temporary test file for は pattern development
// This will be integrated into n5_patterns.rs once complete

use crate::tests::{detect_patterns, print_debug, tokenize_sentence, assert_has_pattern, assert_pattern_range};

// Pattern: は (topic marker)
// Data source: grammar_points_data.json["は"]
// Testing: structure.standard[0] - "Sentence topic + は"
//
// This is one of the most fundamental particles in Japanese
// It marks the topic of the sentence (pronounced "wa" but written は)
// は is a 係助詞 (binding particle)

#[test]
fn test_ha_noun_topic() {
    let sentence = "田中さんは先生です";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);
    print_debug(sentence, &tokens, &patterns);
    // TODO: add assertions after implementation
}

#[test]
fn test_ha_pronoun_topic() {
    let sentence = "私はトムです";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);
    print_debug(sentence, &tokens, &patterns);
    // TODO: add assertions after implementation
}

#[test]
fn test_ha_noun_adjective() {
    let sentence = "カレーは辛い";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);
    print_debug(sentence, &tokens, &patterns);
    // TODO: add assertions after implementation
}

#[test]
fn test_ha_contrast() {
    let sentence = "私は、金曜日は好き";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);
    print_debug(sentence, &tokens, &patterns);
    // TODO: add assertions after implementation
}

#[test]
fn test_ha_casual_negative() {
    let sentence = "そんなこと、俺は知らないよ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);
    print_debug(sentence, &tokens, &patterns);
    // TODO: add assertions after implementation
}

#[test]
fn test_ha_question() {
    let sentence = "あなたは誰ですか";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);
    print_debug(sentence, &tokens, &patterns);
    // TODO: add assertions after implementation
}
