// ========== の1 (nominalization - that which / the one who) ==========
// Pattern: の1
// Data source: grammar_points_data.json["の1"]
//
// Structures to test:
//   - standard[0]: Verb + の
//
// Examples from grammar data:
//   - 食べるのは (the one that eats)
//   - 乗るのが (that which is riding)
//   - 走るのが (that which is running)
//   - 食べたのは (the one that ate)
//   - 登っているのは (the one who is climbing)
//
// Note: の must be followed by は or が to be nominalization (not explanatory のです)
#[cfg(test)]
mod no1_tests {
    use super::*;

    #[test]
    fn test_no1_verb_ru_nowa() {
        let sentence = "沢山食べるのは彼です";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);
        print_debug(sentence, &tokens, &patterns);
        // TODO: add assertions after implementation
    }

    #[test]
    fn test_no1_verb_ru_noga() {
        let sentence = "私はバスに乗るのが嫌いです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);
        print_debug(sentence, &tokens, &patterns);
        // TODO: add assertions after implementation
    }

    #[test]
    fn test_no1_verb_ta_nowa() {
        let sentence = "お菓子を全部食べたのは、娘だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);
        print_debug(sentence, &tokens, &patterns);
        // TODO: add assertions after implementation
    }

    #[test]
    fn test_no1_verb_teiru_nowa() {
        let sentence = "山を登っているのは、今田さんだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);
        print_debug(sentence, &tokens, &patterns);
        // TODO: add assertions after implementation
    }

    #[test]
    fn test_no1_verb_suki_noga() {
        let sentence = "彼は走るのが好き";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);
        print_debug(sentence, &tokens, &patterns);
        // TODO: add assertions after implementation
    }
}
