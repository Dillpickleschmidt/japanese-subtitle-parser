use super::*;

#[test]
fn test_potential_detection() {
    let sentence = "食べられる";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "potential"),
        "Expected potential pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_causative_detection() {
    let sentence = "食べさせる";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "causative"),
        "Expected causative pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_passive_detection() {
    let sentence = "食べられる";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    // Note: られる can be both potential and passive
    // We expect at least one of them to be detected
    let has_potential_or_passive =
        has_pattern(&patterns, "potential") || has_pattern(&patterns, "passive");

    assert!(
        has_potential_or_passive,
        "Expected potential or passive pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ba_conditional_detection() {
    let sentence = "食べれば";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ba_conditional"),
        "Expected ba_conditional pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_tara_conditional_detection() {
    let sentence = "食べたら";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "tara_conditional"),
        "Expected tara_conditional pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_causative_passive_detection() {
    let sentence = "食べさせられる";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "causative_passive"),
        "Expected causative_passive pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_tari_form_detection() {
    let sentence = "食べたりする";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "tari_form"),
        "Expected tari_form pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_must_nakereba_detection() {
    let sentence = "食べなければならない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "must_nakereba"),
        "Expected must_nakereba pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_must_nakute_wa_detection() {
    let sentence = "食べなくてはいけない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "must_nakute_wa"),
        "Expected must_nakute_wa pattern not detected in '{}'",
        sentence
    );
}
