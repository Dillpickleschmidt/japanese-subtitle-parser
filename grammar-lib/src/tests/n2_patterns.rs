use super::*;

// Pattern: 〜ようではないか (why don't we, let's)
// Data source: grammar_points_data.json["〜ようではないか"]
// Testing: structure.standard[0] - "Verb［おう］ + ではないか"
// Testing: structure.standard[1] - "Verb［おう］ + じゃないか"
//
// Structure variants:
//   - standard[0]: Verb［おう］ + ではないか (formal)
//   - standard[1]: Verb［おう］ + じゃないか (casual/abbreviated)

mod youdehanaika_tests {
    use super::*;

    #[test]
    fn test_ou_dehanaika() {
        let sentence = "正々堂々と戦おうではないか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "〜ようではないか");
        assert_pattern_range(&patterns, "〜ようではないか", 5, 13); // 戦おうではないか
    }

    #[test]
    fn test_you_dehanaika() {
        let sentence = "この問題をどう解決するかみんなで考えようではないか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "〜ようではないか");
        assert_pattern_range(&patterns, "〜ようではないか", 16, 25); // 考えようではないか
    }

    #[test]
    fn test_ou_janai() {
        let sentence = "こういう時こそお互い助け合おうじゃないか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "〜ようではないか");
        assert_pattern_range(&patterns, "〜ようではないか", 10, 20); // 助け合おうじゃないか
    }

    #[test]
    fn test_you_janai() {
        let sentence = "食べれるだけの日本食を食べようじゃないか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "〜ようではないか");
        assert_pattern_range(&patterns, "〜ようではないか", 11, 20); // 食べようじゃないか
    }
}

// Pattern: いきなり (suddenly, all of a sudden)
// Data source: grammar_points_data.json["いきなり"]
// Testing: structure.standard[0] - "いきなり + (Action) Phrase"
//
// Structure variants:
//   - standard[0]: いきなり + (Action) Phrase
//   - No polite forms listed

mod ikinari_tests {
    use super::*;

    #[test]
    fn test_ikinari_before_verb() {
        let sentence = "いきなり電話してごめん";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いきなり");
        assert_pattern_range(&patterns, "いきなり", 0, 4); // いきなり
    }

    #[test]
    fn test_ikinari_mid_sentence() {
        let sentence = "ペットのワンちゃんがいきなり吠え出したからびっくりして起きた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いきなり");
        assert_pattern_range(&patterns, "いきなり", 10, 14); // いきなり
    }
}
