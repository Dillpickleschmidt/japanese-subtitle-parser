use super::*;

// Basic conjugations (N5)

#[test]
fn test_dictionary_form_detection() {
    let sentence = "明日映画を見る予定だ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "dictionary_form");
    assert_pattern_range(&patterns, "dictionary_form", 5, 7); // 見る
    assert_pattern_selected(&patterns, "dictionary_form");
}

#[test]
fn test_masu_form_detection() {
    let sentence = "毎朝コーヒーを飲みます";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "masu_form");
    assert_pattern_range(&patterns, "masu_form", 7, 11); // 飲みます
    assert_pattern_selected(&patterns, "masu_form");
}

#[test]
fn test_polite_past_detection() {
    let sentence = "昨日映画を見ました";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "polite_past");
    assert_pattern_range(&patterns, "polite_past", 5, 9); // 見ました
    assert_pattern_selected(&patterns, "polite_past");
}

#[test]
fn test_deshita_detection() {
    let sentence = "昨日は月曜日でしたよ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "deshita");
    assert_pattern_range(&patterns, "deshita", 6, 9); // でした
    assert_pattern_selected(&patterns, "deshita");
}

#[test]
fn test_short_negative_detection() {
    let sentence = "今日は行かない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "short_negative");
    assert_pattern_range(&patterns, "short_negative", 3, 7); // 行かない
    assert_pattern_selected(&patterns, "short_negative");
}

#[test]
fn test_polite_negative_detection() {
    let sentence = "今日は行きません";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "polite_negative");
    assert_pattern_range(&patterns, "polite_negative", 3, 8); // 行きません
    assert_pattern_selected(&patterns, "polite_negative");
}

#[test]
fn test_past_tense_detection() {
    let sentence = "昨日友達に会った";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "past_tense");
    assert_pattern_range(&patterns, "past_tense", 5, 8); // 会った
    assert_pattern_selected(&patterns, "past_tense");
}

#[test]
fn test_short_past_negative_detection() {
    let sentence = "昨日は学校に行かなかった";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "short_past_negative");
    assert_pattern_range(&patterns, "short_past_negative", 6, 12); // 行かなかった
    assert_pattern_selected(&patterns, "short_past_negative");
}

#[test]
fn test_tai_form_detection() {
    let sentence = "日本に行きたい";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "tai_form");
    assert_pattern_range(&patterns, "tai_form", 3, 7); // 行きたい
    assert_pattern_selected(&patterns, "tai_form");
}

#[test]
fn test_takatta_form_detection() {
    let sentence = "もっと勉強したかった";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "takatta_form");
    assert_pattern_range(&patterns, "takatta_form", 3, 10); // 勉強したかった
    assert_pattern_selected(&patterns, "takatta_form");
}

#[test]
fn test_takunai_form_detection() {
    let sentence = "甘いものを食べたくない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "takunai_form");
    assert_pattern_range(&patterns, "takunai_form", 5, 11); // 食べたくない
    assert_pattern_selected(&patterns, "takunai_form");
}

#[test]
fn test_te_form_detection() {
    let sentence = "朝ごはんを食べて学校に行く";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "te_form");
    assert_pattern_range(&patterns, "te_form", 5, 8); // 食べて
    assert_pattern_selected(&patterns, "te_form");
}

#[test]
fn test_te_iru_detection() {
    let sentence = "今勉強しています";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "te_iru");
    assert_pattern_range(&patterns, "te_iru", 1, 8); // 勉強しています
    assert_pattern_selected(&patterns, "te_iru");
}

#[test]
fn test_te_kara_detection() {
    let sentence = "宿題をしてから遊ぶ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "te_kara");
    assert_pattern_range(&patterns, "te_kara", 3, 7); // してから
    assert_pattern_selected(&patterns, "te_kara");
}

#[test]
fn test_te_kudasai_detection() {
    let sentence = "窓を開けてください";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "te_kudasai");
    assert_pattern_range(&patterns, "te_kudasai", 2, 9); // 開けてください
    assert_pattern_selected(&patterns, "te_kudasai");
}

#[test]
fn test_naide_kudasai_detection() {
    let sentence = "触らないでください";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "naide_kudasai");
    assert_pattern_range(&patterns, "naide_kudasai", 0, 9); // 触らないでください
    assert_pattern_selected(&patterns, "naide_kudasai");
}

#[test]
fn test_te_mo_ii_detection() {
    let sentence = "ここに座ってもいいですか";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "te_mo_ii");
    assert_pattern_range(&patterns, "te_mo_ii", 3, 11); // 座ってもいいです
    assert_pattern_selected(&patterns, "te_mo_ii");
}

#[test]
fn test_te_wa_ikenai_detection() {
    let sentence = "ここで写真を撮ってはいけません";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "te_wa_ikenai");
    assert_pattern_range(&patterns, "te_wa_ikenai", 6, 15); // 撮ってはいけません
    assert_pattern_selected(&patterns, "te_wa_ikenai");
}

#[test]
fn test_ta_koto_ga_aru_detection() {
    let sentence = "富士山に登ったことがある";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "ta_koto_ga_aru");
    assert_pattern_range(&patterns, "ta_koto_ga_aru", 4, 12); // 登ったことがある
    assert_pattern_selected(&patterns, "ta_koto_ga_aru");
}

#[test]
fn test_masen_ka_detection() {
    let sentence = "コーヒーを飲みませんか";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "masen_ka");
    assert_pattern_range(&patterns, "masen_ka", 5, 11); // 飲みませんか
    assert_pattern_selected(&patterns, "masen_ka");
}

#[test]
fn test_masen_ka_with_location() {
    let sentence = "図書館で勉強しませんか";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    // Both patterns should be detected
    assert_has_pattern(&patterns, "masen_ka");
    assert_pattern_range(&patterns, "masen_ka", 4, 11); // 勉強しませんか

    assert_has_pattern(&patterns, "polite_negative");
    assert_pattern_range(&patterns, "polite_negative", 4, 10); // 勉強しません

    assert_pattern_selected(&patterns, "masen_ka");
}

#[test]
fn test_mashou_ka_detection() {
    let sentence = "映画を見ましょうか";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "mashou_ka");
    assert_pattern_range(&patterns, "mashou_ka", 3, 9); // 見ましょうか
    assert_pattern_selected(&patterns, "mashou_ka");
}

#[test]
fn test_polite_volitional_detection() {
    let sentence = "一緒に昼ごはんを食べましょう";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "polite_volitional");
    assert_pattern_range(&patterns, "polite_volitional", 8, 14); // 食べましょう
    assert_pattern_selected(&patterns, "polite_volitional");
}

#[test]
fn test_hou_ga_ii_detection() {
    let sentence = "早く寝たほうがいい";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "hou_ga_ii");
    assert_pattern_range(&patterns, "hou_ga_ii", 2, 9); // 寝たほうがいい
    assert_pattern_selected(&patterns, "hou_ga_ii");
}

#[test]
fn test_sugiru_detection() {
    let sentence = "この料理は辛すぎる";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "sugiru");
    assert_pattern_range(&patterns, "sugiru", 5, 9); // 辛すぎる
    assert_pattern_selected(&patterns, "sugiru");
}

#[test]
fn test_tsumori_desu_detection() {
    let sentence = "来年日本に行くつもりです";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "tsumori_desu");
    assert_pattern_range(&patterns, "tsumori_desu", 5, 12); // 行くつもりです
    assert_pattern_selected(&patterns, "tsumori_desu");
}

#[test]
fn test_deshou_detection() {
    let sentence = "明日は雨が降るでしょう";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);
    assert_has_pattern(&patterns, "deshou");
    assert_pattern_range(&patterns, "deshou", 5, 11); // 降るでしょう
    assert_pattern_selected(&patterns, "deshou");
}

#[test]
fn test_mada_te_imasen_detection() {
    let sentence = "まだ宿題をしていません";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "mada_te_imasen");
    assert_pattern_range(&patterns, "mada_te_imasen", 0, 11); // まだ宿題をしていません
    assert_pattern_selected(&patterns, "mada_te_imasen");
}

#[test]
fn test_n_desu_detection() {
    let sentence = "どうして遅れたんですか";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);
    assert_has_pattern(&patterns, "n_desu");
    assert_pattern_range(&patterns, "n_desu", 7, 10); // んです
    assert_pattern_selected(&patterns, "n_desu");
}

// Node pattern with different preceding word types - grouped

mod node_tests {
    use super::*;

    #[test]
    fn verb_form() {
        let sentence = "雨が降っているので傘を持っていく";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);
        assert_pattern_range(&patterns, "node_verb", 2, 9); // 降っているので
        assert_pattern_selected(&patterns, "node_verb");
    }

    #[test]
    fn i_adjective_form() {
        let sentence = "寒いので家にいる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);
        assert_pattern_range(&patterns, "node_adjective", 0, 4); // 寒いので
        assert_pattern_selected(&patterns, "node_adjective");
    }

    #[test]
    fn na_adjective_form() {
        let sentence = "静かなので勉強できる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);
        assert_pattern_range(&patterns, "node_nominal", 0, 5); // 静かなので
        assert_pattern_selected(&patterns, "node_nominal");
    }

    #[test]
    fn noun_form() {
        let sentence = "雨なので行けない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);
        assert_pattern_range(&patterns, "node_nominal", 0, 4); // 雨なので
        assert_pattern_selected(&patterns, "node_nominal");
    }
}

#[test]
fn test_ni_iku_detection() {
    let sentence = "友達に会いに行く";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "ni_iku");
    assert_pattern_range(&patterns, "ni_iku", 3, 8); // 会いに行く
    assert_pattern_selected(&patterns, "ni_iku");
}

#[test]
fn test_mae_ni_detection() {
    let sentence = "寝る前に歯を磨く";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "mae_ni");
    assert_pattern_range(&patterns, "mae_ni", 0, 4); // 寝る前に
    assert_pattern_selected(&patterns, "mae_ni");
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
        assert_pattern_selected(&patterns, "adjective");
    }

    #[test]
    fn test_i_adjective_desu() {
        let sentence = "この本は面白いです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "adjective");
        assert_pattern_range(&patterns, "adjective", 4, 9); // 面白いです
        assert_pattern_selected(&patterns, "adjective");
    }

    #[test]
    fn test_i_adjective_past_desu() {
        let sentence = "昨日は寒かったです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "adjective_past");
        assert_pattern_range(&patterns, "adjective_past", 3, 9); // 寒かったです
        assert_pattern_selected(&patterns, "adjective_past");
    }

    #[test]
    fn test_na_adjective_plain() {
        let sentence = "親切な人です";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "adjective");
        assert_pattern_range(&patterns, "adjective", 0, 3); // 親切な
        assert_pattern_selected(&patterns, "adjective");
    }

    #[test]
    fn test_na_adjective_desu() {
        let sentence = "彼女は親切です";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "adjective");
        assert_pattern_range(&patterns, "adjective", 3, 7); // 親切です
        assert_pattern_selected(&patterns, "adjective");
    }
}

// X は Y です patterns - copula constructions
mod x_wa_y_desu_tests {
    use super::*;

    // Noun は Noun です (polite)
    #[test]
    fn test_noun_wa_noun_desu() {
        let sentence = "私は学生です";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "x_wa_y_desu");
        assert_pattern_range(&patterns, "x_wa_y_desu", 0, 6); // 私は学生です
    }

    // Noun は Noun だ (plain)
    #[test]
    fn test_noun_wa_noun_da() {
        let sentence = "私は学生だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "x_wa_y_desu");
        assert_pattern_range(&patterns, "x_wa_y_desu", 0, 5); // 私は学生だ
    }

    // Noun は Noun だった (past)
    #[test]
    fn test_noun_wa_noun_datta() {
        let sentence = "私は学生だった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "x_wa_y_desu");
        assert_pattern_range(&patterns, "x_wa_y_desu", 0, 7); // 私は学生だった
    }

    // Demonstrative は Noun です
    #[test]
    fn test_demonstrative_wa_noun_desu() {
        let sentence = "これは本です";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "x_wa_y_desu");
        assert_pattern_range(&patterns, "x_wa_y_desu", 0, 6); // これは本です
    }

    // Question word は Noun です
    #[test]
    fn test_question_wa_noun_desu() {
        let sentence = "これは何ですか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "x_wa_y_desu");
        assert_pattern_range(&patterns, "x_wa_y_desu", 0, 6); // これは何です (excludes か)
    }
}

// か particle ending (sentence-final question marker)
mod ka_particle_ending_tests {
    use super::*;

    #[test]
    fn test_question_ka() {
        let sentence = "何ですか？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ka_particle_ending");
        assert_pattern_range(&patterns, "ka_particle_ending", 3, 5); // か？
        assert_pattern_selected(&patterns, "ka_particle_ending");
    }

    #[test]
    fn test_embedded_ka_not_matched() {
        let sentence = "行くかどうかわからない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // Mid-sentence か should NOT match (no punctuation after)
        assert!(!has_pattern(&patterns, "ka_particle_ending"));
    }
}

// の particle modifier (Noun + の + Noun possessive/attributive)
mod no_particle_modifier_tests {
    use super::*;

    #[test]
    fn test_possessive_no() {
        let sentence = "あいつの気持ちがわからない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "no_particle_modifier");
        assert_pattern_range(&patterns, "no_particle_modifier", 0, 7); // あいつの気持ち
        assert_pattern_selected(&patterns, "no_particle_modifier");
    }

    #[test]
    fn test_sentence_final_no_not_matched() {
        let sentence = "どこ行くの？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // Sentence-final の should NOT match (not followed by noun)
        assert!(!has_pattern(&patterns, "no_particle_modifier"));
    }
}

// が particle subject marker
mod ga_particle_subject_tests {
    use super::*;

    // === POSITIVE CASES (should match subject marker) ===
    #[test]
    fn subject_with_polite_verb() {
        let sentence = "友達が来ました";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ga_particle_subject");
        assert_pattern_range(&patterns, "ga_particle_subject", 2, 3); // が
    }

    #[test]
    fn embedded_clause_subject() {
        let sentence = "友達が作った料理";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ga_particle_subject");
        assert_pattern_range(&patterns, "ga_particle_subject", 2, 3); // が
    }

    #[test]
    fn subject_with_adjective() {
        let sentence = "彼女が好きです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ga_particle_subject");
        assert_pattern_range(&patterns, "ga_particle_subject", 2, 3); // が
    }

    #[test]
    fn subject_in_question() {
        let sentence = "誰が来たの？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ga_particle_subject");
        assert_pattern_range(&patterns, "ga_particle_subject", 1, 2); // が
    }

    // === NEGATIVE CASES (conjunction "but" - should NOT match) ===

    #[test]
    fn conjunction_after_verb() {
        // が as "but" after verb - first が should NOT match, second が SHOULD match
        let sentence = "行きたいが、時間がない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // Should have exactly one match (時間が, not 行きたいが)
        let ga_matches: Vec<_> = patterns
            .iter()
            .filter(|p| p.pattern_name == "ga_particle_subject")
            .collect();
        assert_eq!(ga_matches.len(), 1);
        assert_eq!(ga_matches[0].start_char, 8); // 時間が position
    }

    #[test]
    fn conjunction_after_adjective() {
        let sentence = "難しいが、面白い";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // This が is conjunction "but" - should NOT match
        assert!(!has_pattern(&patterns, "ga_particle_subject"));
    }

    #[test]
    fn conjunction_after_copula() {
        let sentence = "学生だが、働いている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // This が is conjunction "but" - should NOT match
        assert!(!has_pattern(&patterns, "ga_particle_subject"));
    }
}

// も particle "also"
mod mo_also_tests {
    use super::*;

    // === POSITIVE CASES (should match "also/too") ===

    #[test]
    fn pronoun_also() {
        let sentence = "私も学生です";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "mo_also");
        assert_pattern_range(&patterns, "mo_also", 0, 2); // 私も
    }

    #[test]
    fn noun_also_multiple() {
        let sentence = "猫も犬も好きです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // Should have two mo_also matches
        let mo_matches: Vec<_> = patterns
            .iter()
            .filter(|p| p.pattern_name == "mo_also")
            .collect();
        assert_eq!(mo_matches.len(), 2);
    }

    #[test]
    fn demonstrative_also() {
        let sentence = "これも私の鞄です";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "mo_also");
        assert_pattern_range(&patterns, "mo_also", 0, 3); // これも
    }

    #[test]
    fn noun_particle_also() {
        // Noun + particle + も
        let sentence = "東京にも行きます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "mo_also");
        assert_pattern_range(&patterns, "mo_also", 0, 4); // 東京にも
    }

    // === NEGATIVE CASES (should NOT match) ===

    #[test]
    fn question_dare_mo() {
        let sentence = "誰も来ない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert!(!has_pattern(&patterns, "mo_also"));
    }

    #[test]
    fn question_nani_mo() {
        let sentence = "何も食べない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert!(!has_pattern(&patterns, "mo_also"));
    }

    #[test]
    fn question_doko_mo() {
        let sentence = "どこも行かない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert!(!has_pattern(&patterns, "mo_also"));
    }

    #[test]
    fn question_doko_ni_mo() {
        // Question word + particle + も should NOT match
        let sentence = "どこにも行かない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert!(!has_pattern(&patterns, "mo_also"));
    }

    #[test]
    fn te_form_mo() {
        let sentence = "雨が降っても、バーベキューをします";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // Should NOT have mo_also (te_mo pattern handles this)
        assert!(!has_pattern(&patterns, "mo_also"));
    }
}

// negative noun patterns
mod negative_noun_tests {
    use super::*;

    #[test]
    fn janai_casual() {
        let sentence = "学生じゃない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "negative_noun_janai");
        assert_pattern_range(&patterns, "negative_noun_janai", 0, 6); // 学生じゃない
    }

    #[test]
    fn ja_arimasen_polite() {
        let sentence = "学生じゃありません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "negative_noun_ja_arimasen");
        assert_pattern_range(&patterns, "negative_noun_ja_arimasen", 0, 9); // 学生じゃありません
    }

    #[test]
    fn dewa_arimasen_formal() {
        let sentence = "学生ではありません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "negative_noun_dewa_arimasen");
        assert_pattern_range(&patterns, "negative_noun_dewa_arimasen", 0, 9); // 学生ではありません
    }

    #[test]
    fn janai_question() {
        // Question form - should still match the negative noun pattern
        let sentence = "学生じゃないですか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "negative_noun_janai");
        assert_pattern_range(&patterns, "negative_noun_janai", 0, 6); // 学生じゃない
    }
}

// adverb patterns
mod adverb_tests {
    use super::*;

    // === Pure adverbs (副詞) ===

    #[test]
    fn test_pure_adverb_totemo() {
        let sentence = "とても速い";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "adverb");
        assert_pattern_range(&patterns, "adverb", 0, 3); // とても
    }

    #[test]
    fn test_pure_adverb_yukkuri() {
        let sentence = "ゆっくり歩く";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "adverb");
        assert_pattern_range(&patterns, "adverb", 0, 4); // ゆっくり
    }

    // === I-adjective adverbial form (く form) ===

    #[test]
    fn test_i_adj_ku_form() {
        let sentence = "早く起きる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "adverb_i_adj");
        assert_pattern_range(&patterns, "adverb_i_adj", 0, 2); // 早く
    }

    #[test]
    fn test_i_adj_ku_form_hayaku() {
        let sentence = "速く走る";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "adverb_i_adj");
        assert_pattern_range(&patterns, "adverb_i_adj", 0, 2); // 速く
    }

    // === Na-adjective adverbial form (に form) ===

    #[test]
    fn test_na_adj_ni_form() {
        let sentence = "静かに話す";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "adverb_na_adj");
        assert_pattern_range(&patterns, "adverb_na_adj", 0, 3); // 静かに
    }

    #[test]
    fn test_na_adj_ni_form_kantan() {
        let sentence = "簡単に解ける";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "adverb_na_adj");
        assert_pattern_range(&patterns, "adverb_na_adj", 0, 3); // 簡単に
    }
}
