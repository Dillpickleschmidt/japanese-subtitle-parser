use super::*;

// ========== なかなか (quite/considerably) ==========
// Pattern: なかなか (quite/considerably/very)
// Data source: grammar_points_data.json[" なかなか"]
//
// Structure variants to test:
//   standard[0]: なかなか + Adjective
//   standard[1]: なかなか + の + Noun

mod nakanaka_tests {
    use super::*;

    // Test: なかなか + い-Adjective
    #[test]
    fn test_nakanaka_i_adjective() {
        let sentence = "ここのラーメンはなかなか美味しいね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, " なかなか");
        assert_pattern_range(&patterns, " なかなか", 8, 12); // なかなか
    }

    // Test: なかなか + な-Adjective
    #[test]
    fn test_nakanaka_na_adjective() {
        let sentence = "ミムラさんもなかなか可愛いよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, " なかなか");
        assert_pattern_range(&patterns, " なかなか", 6, 10); // なかなか
    }

    // Test: なかなか + の + Noun
    #[test]
    fn test_nakanaka_no_noun() {
        let sentence = "あのシェフが作るパスタはなかなかの物だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, " なかなか");
        assert_pattern_range(&patterns, " なかなか", 12, 16); // なかなか
    }

    // Test: なかなか + の + Noun (different example)
    #[test]
    fn test_nakanaka_no_noun_beauty() {
        let sentence = "元カノはなかなかの美人でした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, " なかなか");
        assert_pattern_range(&patterns, " なかなか", 4, 8); // なかなか
    }

    // Test: なかなか with "difficult to dismiss" nuance
    #[test]
    fn test_nakanaka_difficult_to_dismiss() {
        let sentence = "なかなかの事をしてくれたな";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, " なかなか");
        assert_pattern_range(&patterns, " なかなか", 0, 4); // なかなか
    }
}

// ========== おかげで (thanks to) ==========
// Pattern: おかげで (thanks to / because of)
// Data source: grammar_points_data.json["おかげで"]
//
// Structure variants to test:
//   standard[0]: Verb + おかげで
//   standard[1]: ［い］Adjective + おかげで
//   standard[2]: ［な］Adjective + な + おかげで
//   standard[3]: Noun + の + おかげで

mod okagede_tests {
    use super::*;

    // Test: Verb + おかげで
    #[test]
    fn test_okagede_verb() {
        let sentence = "タケルくんに手伝ってもらったおかげで仕事が早く終わったよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "おかげで");
        assert_pattern_range(&patterns, "おかげで", 13, 18); // たおかげで
    }

    // Test: い-Adjective + おかげで
    #[test]
    fn test_okagede_i_adjective() {
        let sentence = "部屋が汚いおかげでどこに何があるか全く分からない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "おかげで");
        assert_pattern_range(&patterns, "おかげで", 3, 9); // 汚いおかげで
    }

    // Test: な-Adjective + な + おかげで
    #[test]
    fn test_okagede_na_adjective() {
        let sentence = "友達が有名なおかげで、どんな高級レストランでも予約なしで入れる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "おかげで");
        assert_pattern_range(&patterns, "おかげで", 5, 10); // なおかげで
    }

    // Test: Noun + の + おかげで
    #[test]
    fn test_okagede_noun() {
        let sentence = "あなたのおかげで不自由のない生活ができている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "おかげで");
        assert_pattern_range(&patterns, "おかげで", 3, 8); // のおかげで
    }
}

// ========== うちに (while/during) ==========
// Pattern: うちに (while/during - temporal expression)
// Data source: grammar_points_data.json["うちに"]
//
// Structure variants to test:
//   standard[0]: Verb［ている］+ うちに
//   standard[1]: ［い］Adjective + うちに
//   standard[2]: ［な］Adjective + な + うちに
//   standard[3]: Noun + の + うちに

mod uchini_tests {
    use super::*;

    // Test: Verb［ている］+ うちに
    #[test]
    fn test_uchini_verb_teiru() {
        let sentence = "彼と毎日会ううちに、だんだんと彼のことが好きになってきた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "うちに");
        assert_pattern_range(&patterns, "うちに", 4, 9); // 会ううちに
    }

    // Test: い-Adjective + うちに
    #[test]
    fn test_uchini_i_adjective() {
        let sentence = "熱いうちに食べて！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "うちに");
        assert_pattern_range(&patterns, "うちに", 0, 5); // 熱いうちに
    }

    // Test: な-Adjective + な + うちに
    #[test]
    fn test_uchini_na_adjective() {
        let sentence = "お爺ちゃんがまだ元気なうちに家族皆で旅行に行こう！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "うちに");
        assert_pattern_range(&patterns, "うちに", 10, 14); // なうちに
    }

    // Test: Noun + の + うちに
    #[test]
    fn test_uchini_noun() {
        let sentence = "今のうちに明日の準備をしておこう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "うちに");
        assert_pattern_range(&patterns, "うちに", 1, 5); // のうちに
    }
}
