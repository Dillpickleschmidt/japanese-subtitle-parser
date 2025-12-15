// Pattern: に伴って・に伴い (due to, along with, in conjunction with)
// Data source: grammar_points_data.json["に伴って・に伴い"]
//
// Structure variants:
//   - standard[0]: Verb[る]+(の)+ に伴って (or に伴い)
//   - standard[1]: Verb[る]+(の)+ に伴う + Noun
//   - standard[2]: Noun + に伴って (or に伴い)
//   - standard[3]: Noun + に伴う + Noun

use super::*;

mod nitomonatte_tests {
    use super::*;

    // Testing: structure.standard[0] - "Verb[る]+(の)+ に伴って"
    #[test]
    fn test_verb_ni_tomonatte() {
        // Example from grammar_points_data.json: 減少するにともなって
        let sentence = "この町の人口が減少するに伴って空き家が増加して来ました。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);
        print_debug(sentence, &tokens, &patterns);
        // TODO: add assertions after implementation
    }

    // Testing: structure.standard[0] with の - "Verb[る]+の+ に伴って"
    #[test]
    fn test_verb_no_ni_tomonatte() {
        // Example: 初期化をするのにともなって
        let sentence = "パソコンの初期化をするのに伴って、すべてのデータが消えます。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);
        print_debug(sentence, &tokens, &patterns);
        // TODO: add assertions after implementation
    }

    // Testing: structure.standard[0] with い form - "Verb[る]+ に伴い"
    #[test]
    fn test_verb_ni_tomonai() {
        // Example from grammar_points_data.json: 地震にともない
        let sentence = "津波は地震に伴い発生することが多いそうだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);
        print_debug(sentence, &tokens, &patterns);
        // TODO: add assertions after implementation
    }

    // Testing: structure.standard[0] - "Verb[る]+ に伴って" with progressive change
    #[test]
    fn test_verb_ni_tomonatte_progressive() {
        // Example from grammar_points_data.json: 歳を取るにともなって
        let sentence = "歳を取るに伴って物忘れが酷くなってきてる感じがする。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);
        print_debug(sentence, &tokens, &patterns);
        // TODO: add assertions after implementation
    }

    // Testing: structure.standard[1] - "Verb[る]+(の)+ に伴う + Noun"
    #[test]
    fn test_verb_ni_tomonau_noun() {
        // Example from grammar_points_data.json: するのにともなう手順
        let sentence = "パソコンの初期化をするのに伴う手順は取扱説明書の５１ページに記載されています。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);
        print_debug(sentence, &tokens, &patterns);
        // TODO: add assertions after implementation
    }

    // Testing: structure.standard[2] - "Noun + に伴って"
    #[test]
    fn test_noun_ni_tomonatte() {
        // Example from grammar_points_data.json: 普及にともなって
        let sentence = "インターネットの普及に伴って、オンラインで買い物を済ませる人が増えた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);
        print_debug(sentence, &tokens, &patterns);
        // TODO: add assertions after implementation
    }

    // Testing: structure.standard[2] - "Noun + に伴い"
    #[test]
    fn test_noun_ni_tomonai() {
        // Example from grammar_points_data.json: 火山現象に伴い
        let sentence = "火山現象に伴い津波が発生することも有るそうだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);
        print_debug(sentence, &tokens, &patterns);
        // TODO: add assertions after implementation
    }

    // Testing: structure.standard[3] - "Noun + に伴う + Noun"
    #[test]
    fn test_noun_ni_tomonau_noun() {
        // Example from grammar_points_data.json: 工事にともなう車線規制
        let sentence = "高速道路の情報サイト：リフレッシュ工事に伴う車線規制のお知らせ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);
        print_debug(sentence, &tokens, &patterns);
        // TODO: add assertions after implementation
    }
}
