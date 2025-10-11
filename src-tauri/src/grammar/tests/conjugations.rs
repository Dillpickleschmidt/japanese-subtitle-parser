use super::*;

#[test]
fn test_past_tense_detection() {
    let sentence = "食べた";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "past_tense"),
        "Expected past_tense pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_negative_detection() {
    let sentence = "食べない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "negative"),
        "Expected negative pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_tai_form_detection() {
    let sentence = "食べたい";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "tai_form"),
        "Expected tai_form pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_masu_form_detection() {
    let sentence = "食べます";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "masu_form"),
        "Expected masu_form pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_past_negative_detection() {
    let sentence = "食べなかった";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "past_negative"),
        "Expected past_negative pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_takatta_form_detection() {
    let sentence = "食べたかった";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "takatta_form"),
        "Expected takatta_form pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_takunai_form_detection() {
    let sentence = "食べたくない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "takunai_form"),
        "Expected takunai_form pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_volitional_detection() {
    let sentence = "食べよう";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "volitional"),
        "Expected volitional pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_imperative_detection() {
    let sentence = "食べろ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "imperative"),
        "Expected imperative pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_dictionary_form_detection() {
    let sentence = "食べる";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "dictionary_form"),
        "Expected dictionary_form pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_nagara_detection() {
    let sentence = "食べながら";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "nagara"),
        "Expected nagara pattern not detected in '{}'",
        sentence
    );
}
