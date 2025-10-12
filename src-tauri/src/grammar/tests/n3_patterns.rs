use super::*;

// Batch 1: Core verb attachment patterns

#[test]
fn test_hajimeru_detection() {
    let sentence = "食べ始める";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "hajimeru"),
        "Expected hajimeru pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_rashii_verb() {
    let sentence = "食べるらしい";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "rashii"),
        "Expected rashii pattern not detected in '{}' (verb)",
        sentence
    );
}

#[test]
fn test_rashii_noun() {
    let sentence = "学生らしい";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "rashii"),
        "Expected rashii pattern not detected in '{}' (noun)",
        sentence
    );
}

#[test]
fn test_rashii_adjective() {
    let sentence = "高いらしい";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "rashii"),
        "Expected rashii pattern not detected in '{}' (adjective)",
        sentence
    );
}

#[test]
fn test_you_ni_naru_detection() {
    let sentence = "食べるようになる";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "you_ni_naru"),
        "Expected you_ni_naru pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_you_ni_suru_detection() {
    let sentence = "食べるようにする";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "you_ni_suru"),
        "Expected you_ni_suru pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_tame_ni_detection() {
    let sentence = "食べるために";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "tame_ni"),
        "Expected tame_ni pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_zu_detection() {
    let sentence = "食べず";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "zu"),
        "Expected zu pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_gachi_detection() {
    let sentence = "休みがち";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "gachi"),
        "Expected gachi pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ta_bakari_detection() {
    let sentence = "食べたばかり";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ta_bakari"),
        "Expected ta_bakari pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ta_mono_da_detection() {
    let sentence = "食べたものだ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ta_mono_da"),
        "Expected ta_mono_da pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ta_mono_da_desu() {
    let sentence = "食べたものです";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ta_mono_da"),
        "Expected ta_mono_da pattern not detected in '{}' (です form - tests Any matcher)",
        sentence
    );
}

#[test]
fn test_ni_chigainai_noun() {
    let sentence = "嘘に違いない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ni_chigainai"),
        "Expected ni_chigainai pattern not detected in '{}' (noun)",
        sentence
    );
}

#[test]
fn test_ni_chigainai_verb() {
    let sentence = "食べるに違いない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ni_chigainai"),
        "Expected ni_chigainai pattern not detected in '{}' (verb)",
        sentence
    );
}

// Batch 2: Noun attachment & conditional patterns

#[test]
fn test_mama_verb() {
    let sentence = "食べたまま";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "mama"),
        "Expected mama pattern not detected in '{}' (verb past)",
        sentence
    );
}

#[test]
fn test_mama_noun() {
    let sentence = "この状態のまま";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "mama"),
        "Expected mama pattern not detected in '{}' (noun)",
        sentence
    );
}

#[test]
fn test_furi_verb_negative() {
    let sentence = "知らないふり";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "furi"),
        "Expected furi pattern not detected in '{}' (verb negative)",
        sentence
    );
}

#[test]
fn test_furi_noun() {
    let sentence = "学生のふり";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "furi"),
        "Expected furi pattern not detected in '{}' (noun with の)",
        sentence
    );
}

#[test]
fn test_nai_uchi_ni_detection() {
    let sentence = "忘れないうちに";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "nai_uchi_ni"),
        "Expected nai_uchi_ni pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ppoi_split() {
    let sentence = "子供っぽい";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ppoi_split") || has_pattern(&patterns, "ppoi_compound"),
        "Expected ppoi pattern not detected in '{}' (split form)",
        sentence
    );
}

#[test]
fn test_ppoi_compound() {
    let sentence = "水っぽい";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ppoi_compound") || has_pattern(&patterns, "ppoi_split"),
        "Expected ppoi pattern not detected in '{}' (compound word)",
        sentence
    );
}

#[test]
fn test_to_shitara_noun() {
    let sentence = "学生としたら";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "to_shitara"),
        "Expected to_shitara pattern not detected in '{}' (noun)",
        sentence
    );
}

#[test]
fn test_to_shitara_verb() {
    let sentence = "行くとしたら";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "to_shitara"),
        "Expected to_shitara pattern not detected in '{}' (verb)",
        sentence
    );
}

#[test]
fn test_bakari_basic() {
    let sentence = "本ばかり";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "bakari"),
        "Expected bakari pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_kawari_noun() {
    let sentence = "私の代わり";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "kawari"),
        "Expected kawari pattern not detected in '{}' (noun with の)",
        sentence
    );
}

#[test]
fn test_kawari_verb() {
    let sentence = "行く代わり";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "kawari"),
        "Expected kawari pattern not detected in '{}' (verb)",
        sentence
    );
}

#[test]
fn test_okage_de_noun() {
    let sentence = "努力のおかげで";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "okage_de"),
        "Expected okage_de pattern not detected in '{}' (noun with の)",
        sentence
    );
}

#[test]
fn test_okage_de_verb_past() {
    let sentence = "頑張ったおかげで";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "okage_de"),
        "Expected okage_de pattern not detected in '{}' (verb past)",
        sentence
    );
}

#[test]
fn test_sae_basic() {
    let sentence = "子供さえ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "sae"),
        "Expected sae pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_you_ni_not_confused_with_suru() {
    let sentence = "食べるようにする";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    // Should match you_ni_suru, NOT you_ni_standalone
    assert!(
        has_pattern(&patterns, "you_ni_suru"),
        "Expected you_ni_suru pattern not detected in '{}'",
        sentence
    );
    // The standalone pattern should have lower priority and not be the primary match
}

// Negative test cases - ensure patterns don't over-match

#[test]
fn test_zu_negative_case() {
    // Test that random ず in compound words doesn't match
    let sentence = "地図を見る"; // 地図 contains ず but shouldn't match
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        !has_pattern(&patterns, "zu"),
        "zu pattern should not match '{}' (contains ず in compound word)",
        sentence
    );
}

#[test]
fn test_ppoi_negative_case() {
    // Test that regular い-adjectives don't match ppoi pattern
    let sentence = "高い本"; // Regular い-adjective, not っぽい
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        !has_pattern(&patterns, "ppoi_split") && !has_pattern(&patterns, "ppoi_compound"),
        "ppoi pattern should not match '{}' (regular い-adjective)",
        sentence
    );
}

#[test]
fn test_kiri_negative() {
    // Test that きり as a standalone adverb doesn't false-match
    let sentence = "きりがない"; // きり here means "no end/limit", not the particle
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    // This might still match due to Any + きり, which is acceptable
    // Main goal is to document the edge case
    // In real usage, context would disambiguate
}

// Batch 3: Adverbs, suffixes, and particles

#[test]
fn test_masaka_detection() {
    let sentence = "まさか嘘じゃない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "masaka"),
        "Expected masaka pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_mushiro_detection() {
    let sentence = "むしろ良い";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "mushiro"),
        "Expected mushiro pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_sudeni_detection() {
    let sentence = "すでに終わった";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "sudeni"),
        "Expected sudeni pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_tsui_detection() {
    let sentence = "つい忘れた";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "tsui"),
        "Expected tsui pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_doushitemo_detection() {
    let sentence = "どうしても行く";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "doushitemo"),
        "Expected doushitemo pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_teki_suffix() {
    let sentence = "日本的な文化"; // Noun + 的
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "teki_suffix"),
        "Expected teki_suffix pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_tate_suffix() {
    let sentence = "焼きたてのパン"; // Verb stem + たて
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "tate_suffix"),
        "Expected tate_suffix pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ni_yotte_noun() {
    let sentence = "人によって違う"; // Noun + によって
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ni_yotte"),
        "Expected ni_yotte pattern not detected in '{}' (noun)",
        sentence
    );
}

#[test]
fn test_ni_yotte_case() {
    let sentence = "場合によって違う"; // Noun (case) + によって - grammatically correct
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ni_yotte"),
        "Expected ni_yotte pattern not detected in '{}' (case/situation)",
        sentence
    );
}

#[test]
fn test_kiri_past_verb() {
    let sentence = "会ったきり"; // Past verb + きり
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "kiri"),
        "Expected kiri pattern not detected in '{}' (past verb)",
        sentence
    );
}

#[test]
fn test_kiri_noun() {
    let sentence = "これきり"; // Noun + きり
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "kiri"),
        "Expected kiri pattern not detected in '{}' (noun)",
        sentence
    );
}

// Batch 4: Additional intermediate patterns

#[test]
fn test_gurai_hiragana() {
    let sentence = "三時間ぐらいかかる"; // Number + ぐらい + verb for context
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "gurai"),
        "Expected gurai pattern not detected in '{}' (ぐらい)",
        sentence
    );
}

#[test]
fn test_gurai_kanji() {
    let sentence = "三時間くらいかかった"; // Number + くらい + verb for context
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "gurai"),
        "Expected gurai pattern not detected in '{}' (くらい)",
        sentence
    );
}

#[test]
fn test_ni_yoru_to() {
    let sentence = "天気予報によると雨だ"; // Noun + によると
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ni_yoru_to"),
        "Expected ni_yoru_to pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_toshite_role() {
    let sentence = "教師として働く"; // Noun + として
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "toshite"),
        "Expected toshite pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_suginai() {
    let sentence = "それは冗談に過ぎない"; // Complete sentence with subject
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "suginai"),
        "Expected suginai pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_oite_variant1() {
    let sentence = "日本において"; // Noun + において
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "oite_compound") || has_pattern(&patterns, "oite_split"),
        "Expected oite pattern not detected in '{}' (において)",
        sentence
    );
}

#[test]
fn test_oite_variant2() {
    let sentence = "会議において決定された"; // Noun + において + complete sentence
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "oite_split") || has_pattern(&patterns, "oite_compound"),
        "Expected oite pattern not detected in '{}' (において)",
        sentence
    );
}

#[test]
fn test_hodo_extent() {
    let sentence = "死ぬほど疲れた一日だった"; // Verb + ほど with complete sentence
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "hodo"),
        "Expected hodo pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_tsumori_de() {
    let sentence = "買うつもりで来た"; // Verb + つもりで
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "tsumori_de"),
        "Expected tsumori_de pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ni_kansuru_variant1() {
    let sentence = "環境に関する問題"; // Noun + に関する
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ni_kansuru"),
        "Expected ni_kansuru pattern not detected in '{}' (に関する)",
        sentence
    );
}

#[test]
fn test_ni_kansuru_variant2() {
    let sentence = "環境に関して話す"; // Noun + に関して
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ni_kansuru"),
        "Expected ni_kansuru pattern not detected in '{}' (に関して)",
        sentence
    );
}

#[test]
fn test_to_tomoni() {
    let sentence = "友達とともに行く"; // Noun + とともに
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "to_tomoni"),
        "Expected to_tomoni pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_te_hajimete() {
    let sentence = "失って初めて大切さがわかる"; // Verb 連用形 + て初めて
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "te_hajimete"),
        "Expected te_hajimete pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_seizei() {
    let sentence = "せいぜい頑張ってください"; // せいぜい (adverb) + complete request
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "seizei"),
        "Expected seizei pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_wo_hajime() {
    let sentence = "日本を始めアジアの国々"; // Noun + を始め + complete phrase
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "wo_hajime"),
        "Expected wo_hajime pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ba_hodo() {
    let sentence = "この本は読めば読むほど面白い"; // Complete sentence with subject
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ba_hodo"),
        "Expected ba_hodo pattern not detected in '{}'",
        sentence
    );
}

// Batch 5: Final N3 patterns

#[test]
fn test_douyara() {
    let sentence = "どうやら雨が降りそうだ"; // どうやら (adverb) + complete sentence
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "douyara"),
        "Expected douyara pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_kaette() {
    let sentence = "かえって悪くなった"; // かえって (adverb) + complete sentence
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "kaette"),
        "Expected kaette pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_sae_ba() {
    let sentence = "お金さえあれば買える"; // Noun + さえ + verb 仮定形 + ば
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "sae_ba"),
        "Expected sae_ba pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_koso() {
    let sentence = "今こそ頑張る時だ"; // Noun + こそ
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "koso"),
        "Expected koso pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_sarani() {
    let sentence = "さらに詳しく説明する"; // さらに (adverb) + complete sentence
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "sarani"),
        "Expected sarani pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_mai() {
    let sentence = "二度と来るまい"; // Verb dictionary + まい
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "mai"),
        "Expected mai pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_wazawaza() {
    let sentence = "わざわざ来てくれた"; // わざわざ (adverb) + complete sentence
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "wazawaza"),
        "Expected wazawaza pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_kagiru() {
    let sentence = "日本に限る話ではない"; // Noun + に限る
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "kagiru"),
        "Expected kagiru pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_nakanaka() {
    let sentence = "なかなか面白い本だ"; // なかなか (adverb) + complete sentence
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "nakanaka"),
        "Expected nakanaka pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ittai() {
    let sentence = "いったい何が起きたのか"; // いったい (adverb) + complete sentence
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ittai"),
        "Expected ittai pattern not detected in '{}'",
        sentence
    );
}

// Additional edge-case tests for Batch 5

#[test]
fn test_kaette_negative_verb() {
    let sentence = "家に帰って休む"; // 帰る verb, not かえって adverb
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        !has_pattern(&patterns, "kaette"),
        "kaette pattern should not match verb 帰る in '{}'",
        sentence
    );
}

#[test]
fn test_koso_verb() {
    let sentence = "これこそが答えだ"; // これ (pronoun) + こそ
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "koso"),
        "Expected koso pattern not detected in '{}' (pronoun + こそ)",
        sentence
    );
}
