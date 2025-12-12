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

// ========== あとで (after/later) ==========
// Pattern: あとで
// Data source: grammar_points_data.json["あとで"]
//
// Structures to test:
//   - standard[0]: Verb[た] + あとで
//   - standard[1]: Noun + の + あとで
//   - standard[2]: 後（あと）で + Phrase
//   - standard[3]: Verb + のは + あとで
//
// Examples from data:
//   - 食べたあとで (after eating)
//   - 仕事のあとで (after work)
//   - あとで洗濯もの干してね (please hang the laundry later)
//   - コピーを取るのはあとでいい (it's fine to make copies later)
#[cfg(test)]
mod atode_tests {
    use super::*;

    // Testing structure.standard[0]: Verb[た] + あとで
    #[test]
    fn test_atode_verb_ta() {
        let sentence = "食べたあとで歯を磨いてください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あとで");
        assert_pattern_range(&patterns, "あとで", 3, 6); // あとで
    }

    // Testing structure.standard[1]: Noun + の + あとで
    #[test]
    fn test_atode_noun_no() {
        let sentence = "仕事のあとで飲み会に行きます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あとで");
        assert_pattern_range(&patterns, "あとで", 3, 6); // あとで
    }

    // Testing structure.standard[2]: 後（あと）で + Phrase
    #[test]
    fn test_atode_at_start() {
        let sentence = "あとで洗濯もの干してね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あとで");
        assert_pattern_range(&patterns, "あとで", 0, 3); // あとで
    }

    // Testing structure.standard[3]: Verb + のは + あとで
    #[test]
    fn test_atode_verb_no_wa() {
        let sentence = "コピーを取るのはあとでいいから、今は上司に電話して";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あとで");
        assert_pattern_range(&patterns, "あとで", 8, 11); // あとで
    }
}
