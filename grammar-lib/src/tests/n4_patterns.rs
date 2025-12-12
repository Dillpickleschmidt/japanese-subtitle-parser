// N4 Grammar Pattern Tests
use super::*;

// ========== ずっと ① (continuously/the whole time) ==========
// Pattern: ずっと ①
// Data source: grammar_points_data.json["ずっと ①"]
//
// Structure to test:
//   - standard[0]: ずっと + Phrase
//
// Examples from data:
//   - ずっとゲームをしないで (instead of continuously gaming)
//   - ずっと立ってた (standing the whole time)
//   - からずっと寝てない (haven't slept at all since...)
#[cfg(test)]
mod zutto_tests {
    use super::*;

    #[test]
    fn test_zutto_at_start() {
        let sentence = "ずっとゲームをしないでたまには勉強もする";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずっと ①");
        assert_pattern_range(&patterns, "ずっと ①", 0, 3); // ずっと
    }

    #[test]
    fn test_zutto_continuous_action() {
        let sentence = "ずっと立ってたから足が痛い";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずっと ①");
        assert_pattern_range(&patterns, "ずっと ①", 0, 3); // ずっと
    }

    #[test]
    fn test_zutto_after_kara() {
        let sentence = "昨日からずっと寝てないからメチャ眠い";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずっと ①");
        assert_pattern_range(&patterns, "ずっと ①", 4, 7); // ずっと
    }
}

// ========== Number + も (as many as / not even) ==========
// Pattern: Number + も
// Data source: grammar_points_data.json["Number + も"]
//
// Structure to test:
//   - standard[0]: Number + Counter + も
//
// Examples:
//   - １２時間も (as many as 12 hours)
//   - ２０万円も (as much as 200,000 yen)
//   - 一回も (not even once)
#[cfg(test)]
mod number_mo_tests {
    use super::*;

    #[test]
    fn test_number_mo_hours() {
        let sentence = "１２時間も仕事をしたから疲れた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Number + も");
        assert_pattern_range(&patterns, "Number + も", 0, 5); // １２時間も
    }

    #[test]
    fn test_number_mo_yen() {
        let sentence = "その携帯２０万円もしたの？！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Number + も");
        assert_pattern_range(&patterns, "Number + も", 4, 9); // ２０万円も
    }

    #[test]
    fn test_number_mo_times() {
        let sentence = "一回も地下鉄に乗ったことが無い";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Number + も");
        assert_pattern_range(&patterns, "Number + も", 0, 3); // 一回も
    }
}
