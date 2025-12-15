use super::*;

// Pattern: のみならず (not only...but also)
// Data source: grammar_points_data.json["のみならず"]
//
// Structure variants:
//   - standard[0]: Verb + のみならず
//   - standard[1]: い-Adjective + のみならず
//   - standard[2]: な-Adjective + のみならず
//   - standard[3]: Noun + (である) + のみならず
//   - standard[4]: Phrase。のみならず + Phrase (conjunction)

mod nominarazu_tests {
    use super::*;

    #[test]
    fn test_verb_nominarazu() {
        // "Not only was I late for work today"
        let sentence = "今日は仕事に遅刻したのみならず、帰りの終電も逃してしまった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        print_debug(sentence, &tokens, &patterns);
        // TODO: add assertions after implementation
    }

    #[test]
    fn test_i_adj_nominarazu() {
        // "Japanese summers are not only hot but also humid"
        let sentence = "日本の夏は暑いのみならず湿気が高いから熱中症になりやすい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        print_debug(sentence, &tokens, &patterns);
        // TODO: add assertions after implementation
    }

    #[test]
    fn test_na_adj_nominarazu() {
        // "That computer is not only convenient but also inexpensive"
        let sentence = "あのパソコンは便利なのみならず、安いので他のと比べてたくさん売れています";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        print_debug(sentence, &tokens, &patterns);
        // TODO: add assertions after implementation
    }

    #[test]
    fn test_noun_nominarazu() {
        // "This manga is popular not only among children but also among adults"
        let sentence = "この漫画は子供のみならず大人の間でも人気があります";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        print_debug(sentence, &tokens, &patterns);
        // TODO: add assertions after implementation
    }

    #[test]
    fn test_noun_dearu_nominarazu() {
        // "This is not only my teacher but also my mentor"
        let sentence = "この方は私の先生であるのみならず、人生の師でもあります";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        print_debug(sentence, &tokens, &patterns);
        // TODO: add assertions after implementation
    }

    #[test]
    fn test_conjunction_nominarazu() {
        // "Tanaka-san is my senpai. Furthermore, he is my life savior"
        let sentence = "田中さんは私の先輩だ。のみならず、命の恩人でもある";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        print_debug(sentence, &tokens, &patterns);
        // TODO: add assertions after implementation
    }
}
