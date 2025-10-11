use super::*;

#[test]
fn test_te_iru_detection() {
    let sentence = "食べています";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "te_iru"),
        "Expected te_iru pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_basic_te_form_detection() {
    let sentence = "食べて";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "te_form_basic"),
        "Expected te_form_basic pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_te_miru_detection() {
    let sentence = "食べてみる";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "te_miru"),
        "Expected te_miru pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_te_shimau_detection() {
    let sentence = "食べてしまう";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "te_shimau"),
        "Expected te_shimau pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_te_request_detection() {
    let sentence = "食べてください";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "te_request"),
        "Expected te_request pattern not detected in '{}'",
        sentence
    );
}
