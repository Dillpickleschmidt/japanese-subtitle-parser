use super::*;

#[test]
fn test_compound_shiri_tsukusu() {
    let sentence = "知り尽くしていた";
    let tokens = tokenize_sentence(sentence);
    let result = crate::analyze(sentence, &tokens);

    // Should find compound "知り尽くす"
    assert!(
        result
            .compound_spans
            .iter()
            .any(|span| span.text == "知り尽くす"),
        "Expected compound '知り尽くす' not found in '{}'",
        sentence
    );
}

#[test]
fn test_compound_me_ga_sameru() {
    let sentence = "目が覚めていた";
    let tokens = tokenize_sentence(sentence);
    let result = crate::analyze(sentence, &tokens);

    // Should find compound "目が覚める"
    assert!(
        result
            .compound_spans
            .iter()
            .any(|span| span.text == "目が覚める"),
        "Expected compound '目が覚める' not found in '{}'",
        sentence
    );
}
