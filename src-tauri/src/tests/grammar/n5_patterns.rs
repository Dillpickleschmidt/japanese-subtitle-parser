use super::*;

// Basic conjugations (N5)

#[test]
fn test_dictionary_form_detection() {
    let sentence = "明日映画を見る予定だ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "dictionary_form");
    assert_pattern_range(&patterns, "dictionary_form", 5, 7); // 見る
    assert_pattern_selected(&patterns, &tokens, "dictionary_form");
}

#[test]
fn test_masu_form_detection() {
    let sentence = "毎朝コーヒーを飲みます";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "masu_form");
    assert_pattern_range(&patterns, "masu_form", 7, 11); // 飲みます
    assert_pattern_selected(&patterns, &tokens, "masu_form");
}

#[test]
fn test_negative_detection() {
    let sentence = "今日は行かない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "negative");
    assert_pattern_range(&patterns, "negative", 3, 7); // 行かない
    assert_pattern_selected(&patterns, &tokens, "negative");
}

#[test]
fn test_past_tense_detection() {
    let sentence = "昨日友達に会った";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "past_tense");
    assert_pattern_range(&patterns, "past_tense", 5, 8); // 会った
    assert_pattern_selected(&patterns, &tokens, "past_tense");
}

// Tai-form patterns (N5)

#[test]
fn test_tai_form_detection() {
    let sentence = "日本に行きたい";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "tai_form");
    assert_pattern_range(&patterns, "tai_form", 3, 7); // 行きたい
    assert_pattern_selected(&patterns, &tokens, "tai_form");
}

#[test]
fn test_takatta_form_detection() {
    let sentence = "もっと勉強したかった";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "takatta_form");
    assert_pattern_range(&patterns, "takatta_form", 3, 10); // 勉強したかった
    assert_pattern_selected(&patterns, &tokens, "takatta_form");
}

#[test]
fn test_takunai_form_detection() {
    let sentence = "甘いものを食べたくない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "takunai_form");
    assert_pattern_range(&patterns, "takunai_form", 5, 11); // 食べたくない
    assert_pattern_selected(&patterns, &tokens, "takunai_form");
}

// Te-form patterns (N5)

#[test]
fn test_basic_te_form_detection() {
    let sentence = "朝ごはんを食べて学校に行く";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "te_form_basic");
    assert_pattern_range(&patterns, "te_form_basic", 5, 8); // 食べて
    assert_pattern_selected(&patterns, &tokens, "te_form_basic");
}

#[test]
fn test_te_iru_detection() {
    let sentence = "今勉強しています";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "te_iru");
    assert_pattern_range(&patterns, "te_iru", 1, 8); // 勉強しています
    assert_pattern_selected(&patterns, &tokens, "te_iru");
}

#[test]
fn test_te_request_detection() {
    let sentence = "ここで待っていてくださいませんか";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);
    assert_has_pattern(&patterns, "te_request");
    assert_pattern_range(&patterns, "te_request", 6, 15); // いてくださいません
    assert_pattern_selected(&patterns, &tokens, "te_request");
}

#[test]
fn test_te_kudasai_detection() {
    let sentence = "窓を開けてください";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "te_kudasai");
    assert_pattern_range(&patterns, "te_kudasai", 2, 9); // 開けてください
    assert_pattern_selected(&patterns, &tokens, "te_kudasai");
}

#[test]
fn test_te_kara_detection() {
    let sentence = "宿題をしてから遊ぶ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "te_kara");
    assert_pattern_range(&patterns, "te_kara", 3, 7); // してから
    assert_pattern_selected(&patterns, &tokens, "te_kara");
}

#[test]
fn test_mashou_detection() {
    let sentence = "一緒に昼ごはんを食べましょう";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);
    assert_has_pattern(&patterns, "mashou");
    assert_pattern_range(&patterns, "mashou", 8, 14); // 食べましょう
    assert_pattern_selected(&patterns, &tokens, "mashou");
}

#[test]
fn test_ta_koto_ga_aru_detection() {
    let sentence = "富士山に登ったことがある";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "ta_koto_ga_aru");
    assert_pattern_range(&patterns, "ta_koto_ga_aru", 4, 12); // 登ったことがある
    assert_pattern_selected(&patterns, &tokens, "ta_koto_ga_aru");
}

#[test]
fn test_te_mo_ii_detection() {
    let sentence = "ここに座ってもいいですか";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "te_mo_ii");
    assert_pattern_range(&patterns, "te_mo_ii", 3, 11); // 座ってもいいです
    assert_pattern_selected(&patterns, &tokens, "te_mo_ii");
}

#[test]
fn test_te_wa_ikenai_detection() {
    let sentence = "ここで写真を撮ってはいけません";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "te_wa_ikenai");
    assert_pattern_range(&patterns, "te_wa_ikenai", 6, 15); // 撮ってはいけません
    assert_pattern_selected(&patterns, &tokens, "te_wa_ikenai");
}

#[test]
fn test_naide_kudasai_detection() {
    let sentence = "触らないでください";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "naide_kudasai");
    assert_pattern_range(&patterns, "naide_kudasai", 0, 9); // 触らないでください
    assert_pattern_selected(&patterns, &tokens, "naide_kudasai");
}

#[test]
fn test_masen_ka_detection() {
    let sentence = "コーヒーを飲みませんか";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "masen_ka");
    assert_pattern_range(&patterns, "masen_ka", 5, 11); // 飲みませんか
    assert_pattern_selected(&patterns, &tokens, "masen_ka");
}

#[test]
fn test_mashou_ka_detection() {
    let sentence = "映画を見ましょうか";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "mashou_ka");
    assert_pattern_range(&patterns, "mashou_ka", 3, 9); // 見ましょうか
    assert_pattern_selected(&patterns, &tokens, "mashou_ka");
}

// Sugiru pattern with multiple variations (grouped)

mod sugiru_tests {
    use super::*;

    #[test]
    fn verb_form() {
        let sentence = "この料理は辛すぎる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "sugiru");
        assert_pattern_range(&patterns, "sugiru", 5, 9); // 辛すぎる
        assert_pattern_selected(&patterns, &tokens, "sugiru");
    }

    #[test]
    fn i_adjective_form() {
        let sentence = "このカバンは重すぎる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "sugiru");
        assert_pattern_range(&patterns, "sugiru", 6, 10); // 重すぎる
        assert_pattern_selected(&patterns, &tokens, "sugiru");
    }

    #[test]
    fn na_adjective_form() {
        let sentence = "彼女は親切すぎる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "sugiru");
        assert_pattern_range(&patterns, "sugiru", 3, 8); // 親切すぎる
        assert_pattern_selected(&patterns, &tokens, "sugiru");
    }
}

#[test]
fn test_tsumori_desu_detection() {
    let sentence = "来年日本に行くつもりです";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "tsumori_desu");
    assert_pattern_range(&patterns, "tsumori_desu", 5, 12); // 行くつもりです
    assert_pattern_selected(&patterns, &tokens, "tsumori_desu");
}

#[test]
fn test_hou_ga_ii_detection() {
    let sentence = "早く寝たほうがいい";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "hou_ga_ii");
    assert_pattern_range(&patterns, "hou_ga_ii", 2, 9); // 寝たほうがいい
    assert_pattern_selected(&patterns, &tokens, "hou_ga_ii");
}

#[test]
fn test_nakucha_ikenai_detection() {
    let sentence = "早く帰らなくちゃいけない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);
    assert_has_pattern(&patterns, "nakucha_ikenai");
    assert_pattern_range(&patterns, "nakucha_ikenai", 2, 12); // 帰らなくちゃいけない
    assert_pattern_selected(&patterns, &tokens, "nakucha_ikenai");
}

#[test]
fn test_deshou_detection() {
    let sentence = "明日は雨が降るでしょう";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);
    assert_has_pattern(&patterns, "deshou");
    assert_pattern_range(&patterns, "deshou", 5, 11); // 降るでしょう
    assert_pattern_selected(&patterns, &tokens, "deshou");
}

#[test]
fn test_mada_te_imasen_detection() {
    let sentence = "まだ宿題をしていません";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "mada_te_imasen");
    assert_pattern_range(&patterns, "mada_te_imasen", 0, 11); // まだ宿題をしていません
    assert_pattern_selected(&patterns, &tokens, "mada_te_imasen");
}

#[test]
fn test_n_desu_detection() {
    let sentence = "どうして遅れたんですか";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);
    assert_has_pattern(&patterns, "n_desu");
    assert_pattern_range(&patterns, "n_desu", 7, 10); // んです
    assert_pattern_selected(&patterns, &tokens, "n_desu");
}

// Node pattern with different preceding word types - grouped

mod node_tests {
    use super::*;

    #[test]
    fn verb_form() {
        let sentence = "雨が降っているので傘を持っていく";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);
        print_debug(sentence, &tokens, &patterns);
        assert_has_pattern(&patterns, "node_verb");
        assert_pattern_range(&patterns, "node_verb", 2, 9); // 降っているので
        assert_pattern_selected(&patterns, &tokens, "node_verb");
    }

    #[test]
    fn i_adjective_form() {
        let sentence = "寒いので家にいる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);
        print_debug(sentence, &tokens, &patterns);
        assert_has_pattern(&patterns, "node_adjective");
        assert_pattern_range(&patterns, "node_adjective", 0, 4); // 寒いので
        assert_pattern_selected(&patterns, &tokens, "node_adjective");
    }

    #[test]
    fn na_adjective_form() {
        let sentence = "静かなので勉強できる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);
        print_debug(sentence, &tokens, &patterns);
        assert_has_pattern(&patterns, "node_nominal");
        assert_pattern_range(&patterns, "node_nominal", 0, 5); // 静かなので
        assert_pattern_selected(&patterns, &tokens, "node_nominal");
    }

    #[test]
    fn noun_form() {
        let sentence = "雨なので行けない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);
        print_debug(sentence, &tokens, &patterns);
        assert_has_pattern(&patterns, "node_nominal");
        assert_pattern_range(&patterns, "node_nominal", 0, 4); // 雨なので
        assert_pattern_selected(&patterns, &tokens, "node_nominal");
    }
}

#[test]
fn test_ni_iku_detection() {
    let sentence = "友達に会いに行く";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "ni_iku");
    assert_pattern_range(&patterns, "ni_iku", 3, 8); // 会いに行く
    assert_pattern_selected(&patterns, &tokens, "ni_iku");
}

// Te-form variations (de-form for godan verbs) - grouped

mod te_form_variations {
    use super::*;

    #[test]
    fn de_iru_form() {
        let sentence = "誰かが外で待っていんです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);
        assert_has_pattern(&patterns, "te_iru");
        assert_pattern_range(&patterns, "te_iru", 5, 9); // 待ってい
        assert_pattern_selected(&patterns, &tokens, "te_iru");
    }

    #[test]
    fn de_kudasai_form() {
        let sentence = "静かに飲んでください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "te_kudasai");
        assert_pattern_range(&patterns, "te_kudasai", 3, 10); // 飲んでください
        assert_pattern_selected(&patterns, &tokens, "te_kudasai");
    }
}

// Mae-ni pattern with different verb types - grouped

mod mae_ni_tests {
    use super::*;

    #[test]
    fn ru_verb() {
        let sentence = "寝る前に歯を磨く";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "mae_ni");
        assert_pattern_range(&patterns, "mae_ni", 0, 4); // 寝る前に
        assert_pattern_selected(&patterns, &tokens, "mae_ni");
    }

    #[test]
    fn u_verb() {
        let sentence = "出かける前に鍵を確認する";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "mae_ni");
        assert_pattern_range(&patterns, "mae_ni", 0, 6); // 出かける前に
        assert_pattern_selected(&patterns, &tokens, "mae_ni");
    }
}

// Adjective conjugation patterns
mod adjective_patterns {
    use super::*;

    #[test]
    fn test_i_adjective_plain() {
        let sentence = "新しい本を買った";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "adjective");
        assert_pattern_range(&patterns, "adjective", 0, 3); // 新しい
        assert_pattern_selected(&patterns, &tokens, "adjective");
    }

    #[test]
    fn test_i_adjective_desu() {
        let sentence = "この本は面白いです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "adjective");
        assert_pattern_range(&patterns, "adjective", 4, 9); // 面白いです
        assert_pattern_selected(&patterns, &tokens, "adjective");
    }

    #[test]
    fn test_i_adjective_past_desu() {
        let sentence = "昨日は寒かったです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "adjective_past");
        assert_pattern_range(&patterns, "adjective_past", 3, 9); // 寒かったです
        assert_pattern_selected(&patterns, &tokens, "adjective_past");
    }

    #[test]
    fn test_na_adjective_plain() {
        let sentence = "親切な人です";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "adjective");
        assert_pattern_range(&patterns, "adjective", 0, 3); // 親切な
        assert_pattern_selected(&patterns, &tokens, "adjective");
    }

    #[test]
    fn test_na_adjective_desu() {
        let sentence = "彼女は親切です";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "adjective");
        assert_pattern_range(&patterns, "adjective", 3, 7); // 親切です
        assert_pattern_selected(&patterns, &tokens, "adjective");
    }
}
