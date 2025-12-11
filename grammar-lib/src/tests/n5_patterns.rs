use super::*;

// ========== たい (Want to do) ==========
// Pattern: たい (desire/want to do)
// Data source: grammar_points_data.json["たい"]
//
// Structure variants to test:
//   standard[0]: Verb[stem] + たい
//   standard[1]: Verb[stem] + たくない (negative)
//   standard[2]: Verb[stem] + たかった (past)
//   standard[3]: Verb[stem] + たくなかった (past negative)
//   polite[0-5]: Same forms + です variants

mod tai_form_tests {
    use super::*;

    #[test]
    fn test_tai_form_non_past() {
        let sentence = "あの映画、絶対に見たいんだけど";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たい");
        assert_pattern_range(&patterns, "たい", 8, 11); // 見たい
    }

    #[test]
    fn test_tai_form_negative_polite() {
        let sentence = "虫は食べたくないです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たい");
        assert_pattern_range(&patterns, "たい", 2, 10); // 食べたくないです (extended with です)
    }

    #[test]
    fn test_tai_form_past() {
        let sentence = "警察官になりたかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たい");
        assert_pattern_range(&patterns, "たい", 4, 10); // なりたかった
    }

    #[test]
    fn test_tai_form_past_negative() {
        let sentence = "昔は先生になりたくなかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たい");
        assert_pattern_range(&patterns, "たい", 5, 13); // なりたくなかった
    }
}

// ========== ている① (Progressive/Resultative) ==========
// Pattern: ている① (progressive/resultative state)
// Data source: grammar_points_data.json["ている①"]
//
// Structure variants to test:
//   standard[0]: Verb[て] + いる
//   standard[1]: Verb[て] + る (contracted)
//   polite[0]: Verb[て] + います
//   polite[1]: Verb[て] + ます (contracted)

mod te_iru_tests {
    use super::*;

    #[test]
    fn test_te_iru_progressive() {
        let sentence = "最近ずっとあのことを考えている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ている①");
        assert_pattern_range(&patterns, "ている①", 10, 15); // 考えている
    }

    #[test]
    fn test_te_iru_contracted() {
        let sentence = "あいつ、どこ行ってるんだよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ている①");
        assert_pattern_range(&patterns, "ている①", 6, 10); // 行ってる
    }

    #[test]
    fn test_te_imasu_polite() {
        let sentence = "今ラーメンを食べています";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ている①");
        assert_pattern_range(&patterns, "ている①", 6, 12); // 食べています
    }

    #[test]
    fn test_te_masu_contracted_polite() {
        let sentence = "彼は何してます？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ている①");
        assert_pattern_range(&patterns, "ている①", 3, 7); // してます
    }
}

// ========== も (Also/Too) ==========
// Pattern: も (also/too/even)
// Data source: grammar_points_data.json["も"]
//
// Structure variants to test:
//   standard[0]: Noun + も
//
// Note: This pattern has only one structure variant but multiple contexts
// Test different contexts: pronoun + も, noun + particle + も
// NEGATIVE tests: Exclude 誰も/何も (different grammar - "nobody/nothing")

mod mo_also_tests {
    use super::*;

    #[test]
    fn test_pronoun_also() {
        let sentence = "俺もそう思ってたんだよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "も");
        assert_pattern_range(&patterns, "も", 0, 2); // 俺も
    }

    #[test]
    fn test_noun_particle_also() {
        let sentence = "こんな場所にも来たことあるの？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "も");
        assert_pattern_range(&patterns, "も", 3, 7); // 場所にも
    }

    // NEGATIVE TEST - should NOT match (different grammar)
    #[test]
    fn test_question_dare_mo() {
        let sentence = "誰も来ないなんて寂しいな";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert!(!has_pattern(&patterns, "も")); // 誰も is different grammar
    }
}
