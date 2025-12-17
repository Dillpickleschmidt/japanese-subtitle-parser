use super::{assert_has_pattern, assert_pattern_range, detect_patterns, tokenize_sentence};

// Pattern: ぞ (emphatic sentence-ending particle)
// Data source: grammar_points_data.json["ぞ"]
// Testing: structure.standard[0] - "Phrase + ぞ"
//
// Note: ぞ is a sentence-ending particle used for emphasis, primarily by men
// Can follow verbs, adjectives, and auxiliary verbs
// Also has adverbial particle usage (less common)

mod zo_tests {
    use super::*;

    #[test]
    fn test_zo_verb_present() {
        let sentence = "今出ないと遅れるぞ！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ぞ");
        assert_pattern_range(&patterns, "ぞ", 5, 9); // 遅れるぞ
    }

    #[test]
    fn test_zo_adjective() {
        let sentence = "待った！気を付けろ、まだ熱いぞ！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ぞ");
        assert_pattern_range(&patterns, "ぞ", 12, 15); // 熱いぞ
    }

    #[test]
    fn test_zo_causative_passive() {
        let sentence = "そんな早く行ったら、また掃除させられるぞ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ぞ");
        assert_pattern_range(&patterns, "ぞ", 16, 20); // られるぞ
    }

    #[test]
    fn test_zo_self_encouragement() {
        let sentence = "これをやれば後は楽だぞ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ぞ");
        assert_pattern_range(&patterns, "ぞ", 9, 11); // だぞ
    }
}

// Pattern: ぜ (friendly emphatic sentence-ending particle)
// Data source: grammar_points_data.json["ぜ"]
// Testing: structure.standard[0] - "Phrase + ぜ"
//
// Note: ぜ is similar to ぞ but friendlier, less forceful
// Used primarily by men with familiar people
// Cannot follow directly after nouns (needs だ)

mod ze_tests {
    use super::*;

    #[test]
    fn test_ze_volitional() {
        let sentence = "今度暇なときランチでもしようぜ！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ぜ");
        assert_pattern_range(&patterns, "ぜ", 13, 15); // うぜ (volitional う + ぜ)
    }

    #[test]
    fn test_ze_volitional_short() {
        let sentence = "もうそろそろ寝ようぜ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ぜ");
        assert_pattern_range(&patterns, "ぜ", 8, 10); // うぜ (volitional う + ぜ)
    }

    #[test]
    fn test_ze_copula() {
        let sentence = "ポケモンゲットだぜ！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ぜ");
        assert_pattern_range(&patterns, "ぜ", 7, 9); // だぜ
    }
}
