use super::*;

// Basic conjugations (N5)
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

// Tai-form patterns (N5)
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

// Te-form patterns (N5)
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

// Additional N5 patterns
#[test]
fn test_te_kudasai_detection() {
    let sentence = "食べてください";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "te_kudasai"),
        "Expected te_kudasai pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_te_kara_detection() {
    let sentence = "食べてから";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "te_kara"),
        "Expected te_kara pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_mashou_detection() {
    let sentence = "食べましょう";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "mashou"),
        "Expected mashou pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ta_koto_ga_aru_detection() {
    let sentence = "食べたことがある";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ta_koto_ga_aru"),
        "Expected ta_koto_ga_aru pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_te_mo_ii_detection() {
    let sentence = "食べてもいい";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "te_mo_ii"),
        "Expected te_mo_ii pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_te_wa_ikenai_detection() {
    let sentence = "食べてはいけない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "te_wa_ikenai"),
        "Expected te_wa_ikenai pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_naide_kudasai_detection() {
    let sentence = "食べないでください";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "naide_kudasai"),
        "Expected naide_kudasai pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_masen_ka_detection() {
    let sentence = "食べませんか";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "masen_ka"),
        "Expected masen_ka pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_mashou_ka_detection() {
    let sentence = "食べましょうか";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "mashou_ka"),
        "Expected mashou_ka pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_sugiru_detection() {
    let sentence = "食べすぎる";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "sugiru"),
        "Expected sugiru pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_sugiru_i_adjective() {
    let sentence = "高すぎる"; // i-adjective: 高い -> 高 + すぎる (too expensive)
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "sugiru"),
        "Expected sugiru pattern not detected in '{}' (i-adjective)",
        sentence
    );
}

#[test]
fn test_sugiru_na_adjective() {
    let sentence = "静かすぎる"; // na-adjective: 静か + すぎる (too quiet)
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "sugiru"),
        "Expected sugiru pattern not detected in '{}' (na-adjective)",
        sentence
    );
}

#[test]
fn test_tsumori_desu_detection() {
    let sentence = "食べるつもりです";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "tsumori_desu"),
        "Expected tsumori_desu pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_hou_ga_ii_detection() {
    let sentence = "食べたほうがいい";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "hou_ga_ii"),
        "Expected hou_ga_ii pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_nakucha_ikenai_detection() {
    let sentence = "食べなくちゃいけない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "nakucha_ikenai"),
        "Expected nakucha_ikenai pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_deshou_detection() {
    let sentence = "食べるでしょう";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "deshou"),
        "Expected deshou pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_mada_te_imasen_detection() {
    let sentence = "まだ食べていません";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "mada_te_imasen"),
        "Expected mada_te_imasen pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_n_desu_detection() {
    let sentence = "食べるんです";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "n_desu"),
        "Expected n_desu pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_node_detection() {
    let sentence = "食べるので";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "node"),
        "Expected node pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ni_iku_detection() {
    let sentence = "食べに行く";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ni_iku"),
        "Expected ni_iku pattern not detected in '{}'",
        sentence
    );
}

// Edge case tests for different verb types and forms

#[test]
fn test_te_form_de_variation() {
    let sentence = "飲んで"; // Godan verb with んで ending
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "te_form_basic"),
        "Expected te_form_basic pattern not detected in '{}' (de-form variation)",
        sentence
    );
}

#[test]
fn test_te_iru_de_variation() {
    let sentence = "飲んでいる"; // Godan verb with んで
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "te_iru"),
        "Expected te_iru pattern not detected in '{}' (de-form variation)",
        sentence
    );
}

#[test]
fn test_te_kudasai_de_variation() {
    let sentence = "飲んでください"; // Godan verb with んで
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "te_kudasai"),
        "Expected te_kudasai pattern not detected in '{}' (de-form variation)",
        sentence
    );
}

#[test]
fn test_mae_ni_detection() {
    let sentence = "食べる前に"; // Before eating
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "mae_ni"),
        "Expected mae_ni pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_mae_ni_godan() {
    let sentence = "寝る前に"; // Before sleeping (godan verb)
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "mae_ni"),
        "Expected mae_ni pattern not detected in '{}' (godan verb)",
        sentence
    );
}
