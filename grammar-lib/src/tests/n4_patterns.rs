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

// ========== あまり～ない (not very) ==========
// Pattern: あまり～ない
// Data source: grammar_points_data.json["あまり～ない"]
//
// Structures to test:
//   - standard[0]: あまり(1) + Verb[ない]
//   - standard[1]: あまり(1) + い-Adjective[ない]
//   - standard[2]: あまり(1) + Noun + ではない(2)
//   - standard[3]: あまり + な-Adjective + ではない(2)
//   - Note: (1) あんまり (casual variant)
//   - Note: (2) じゃない (casual variant)
//
// Examples from data:
//   - あまり並ばないと思う (not stand in line very long)
//   - あまり寂しくない (not feel very lonely)
//   - あまり平和ではない (not very peaceful)
//   - あまりいい肉ではない (hardly good meat)
#[cfg(test)]
mod amari_nai_tests {
    use super::*;

    // Testing structure.standard[0]: あまり + Verb[ない]
    #[test]
    fn test_amari_verb_nai() {
        let sentence = "今の時間だったらあまり並ばないと思うよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あまり～ない");
        assert_pattern_range(&patterns, "あまり～ない", 8, 15); // あまり並ばない
    }

    // Testing structure.standard[1]: あまり + い-Adjective[ない]
    #[test]
    fn test_amari_i_adj_nai() {
        let sentence = "私には犬がいるからあまり寂しくない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あまり～ない");
        assert_pattern_range(&patterns, "あまり～ない", 9, 17); // あまり寂しくない
    }

    // Testing structure.standard[2]: あまり + Noun + ではない
    #[test]
    fn test_amari_noun_dewa_nai() {
        let sentence = "最近はどこもあまり平和ではない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あまり～ない");
        assert_pattern_range(&patterns, "あまり～ない", 6, 15); // あまり平和ではない
    }

    // Testing structure.standard[3]: あまり + な-Adjective + ではない
    #[test]
    fn test_amari_na_adj_dewa_nai() {
        let sentence = "そこの肉屋の肉はあまりいい肉ではない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あまり～ない");
        assert_pattern_range(&patterns, "あまり～ない", 8, 18); // あまりいい肉ではない
    }

    // Testing casual variant: あんまり + Verb[ない]
    #[test]
    fn test_anmari_verb_nai() {
        let sentence = "あんまり食べたくないんだよね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あまり～ない");
        assert_pattern_range(&patterns, "あまり～ない", 0, 10); // あんまり食べたくない
    }

    // Testing casual variant: あまり + Noun + じゃない
    #[test]
    fn test_amari_noun_janai() {
        let sentence = "それはあまり良い考えじゃないと思う";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あまり～ない");
        assert_pattern_range(&patterns, "あまり～ない", 3, 14); // あまり良い考えじゃない
    }
}

// ========== ことができる (can do / be able to) ==========
// Pattern: ことができる
// Data source: grammar_points_data.json["ことができる"]
//
// Structures to test:
//   - standard[0]: Verb + こと + が + できる
//   - standard[1]: Noun + が + できる (no こと needed)
//   - polite[0]: Verb + こと + が + できます
//   - polite[1]: Noun + が + できます
//
// Examples from data:
//   - 泳ぐことができる (can swim)
//   - 料理をすることができる (can cook)
//   - 運転ができる (can drive, noun)
#[cfg(test)]
mod kotogadekiru_tests {
    use super::*;

    // Testing: Verb + ことができる
    #[test]
    fn test_kotogadekiru_verb() {
        let sentence = "魚のように泳ぐことができるようになりたい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことができる");
        assert_pattern_range(&patterns, "ことができる", 5, 13); // 泳ぐことができる
    }

    // Testing: Verb + ことができる (different context)
    #[test]
    fn test_kotogadekiru_verb_simple() {
        let sentence = "彼は料理をすることができる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことができる");
        assert_pattern_range(&patterns, "ことができる", 5, 13); // することができる
    }

    // Testing: Noun + ができる (no こと)
    #[test]
    fn test_kotogadekiru_noun() {
        let sentence = "運転ができる彼氏が欲しい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことができる");
        assert_pattern_range(&patterns, "ことができる", 0, 6); // 運転ができる
    }

    // Testing: Verb + ことができます (polite)
    #[test]
    fn test_kotogadekiru_polite() {
        let sentence = "私は英語を話すことができます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことができる");
        assert_pattern_range(&patterns, "ことができる", 5, 14); // 話すことができます
    }

    // Testing: Noun + ができる (polite context)
    #[test]
    fn test_kotogadekiru_noun_polite() {
        let sentence = "力仕事ができる人を探しています";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことができる");
        assert_pattern_range(&patterns, "ことができる", 0, 7); // 力仕事ができる
    }

    // Testing: Negative form - ことはできない (は instead of が)
    #[test]
    fn test_kotogadekiru_negative() {
        let sentence = "こんな難しいことはできない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことができる");
        assert_pattern_range(&patterns, "ことができる", 3, 13); // 難しいことはできない
    }
}

// ========== ということ (that means / you mean) ==========
// Pattern: ということ
// Data source: grammar_points_data.json["ということ"]
//
// Structure to test:
//   - standard[0]: Phrase + ということ
//   - Casual variant: Phrase + ってこと
//
// Examples from data:
//   - 宇宙人はいるということですか (Does that mean aliens exist?)
//   - ルームメイトだということだよね (That means they're your roommate?)
//   - 正しいってことですか (casual: Does that mean this is correct?)
#[cfg(test)]
mod toiukoto_tests {
    use super::*;

    // Testing: Phrase + ということ (formal)
    #[test]
    fn test_toiukoto_formal() {
        let sentence = "宇宙人はいるということですか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ということ");
        assert_pattern_range(&patterns, "ということ", 4, 13); // いるということです
    }

    // Testing: Phrase + ということ (statement)
    #[test]
    fn test_toiukoto_statement() {
        let sentence = "ルームメイトだということだよね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ということ");
        assert_pattern_range(&patterns, "ということ", 6, 12); // だということ
    }

    // Testing: Phrase + ということを (with を particle)
    #[test]
    fn test_toiukoto_with_particle() {
        let sentence = "あの先生の教え方が酷いということを聞いた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ということ");
        assert_pattern_range(&patterns, "ということ", 9, 16); // 酷いということ
    }

    // Testing: Phrase + ってこと (casual form)
    #[test]
    fn test_toiukoto_casual() {
        let sentence = "これが正しいってことですか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ということ");
        assert_pattern_range(&patterns, "ということ", 3, 12); // 正しいってことです
    }

    // Testing: Phrase + ってこと (casual, question)
    #[test]
    fn test_toiukoto_casual_question() {
        let sentence = "電車で来るってこと？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ということ");
        assert_pattern_range(&patterns, "ということ", 3, 9); // 来るってこと
    }

    // Testing: Phrase + ってこと (casual clarification)
    #[test]
    fn test_toiukoto_casual_clarify() {
        let sentence = "これは人工ってこと？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ということ");
        assert_pattern_range(&patterns, "ということ", 3, 9); // 人工ってこと
    }
}

// ========== し～し (and, conjunction for listing reasons) ==========
// Pattern: し～し
// Data source: grammar_points_data.json["し～し "]
//
// Structures to test:
//   - standard[0]: Verb (A) + し + （Verb (B) + し）
//   - standard[1]: ［い］Adjective (A) + し + （［い］Adjective (B) + し）
//   - standard[2]: ［な］Adjective (A) + だ + し + （［な］Adjective (B) + だ + し）
//   - standard[3]: Noun (A) + だ + し + （Noun (B) + だ + し）
//
// Examples from data:
//   - 弾けるし、できるし (can play and can do)
//   - 高いし、まずいし (expensive and doesn't taste good)
//   - 真面目だし、親切だし (serious and kind)
//   - 休みだし、晴れだし (day off and sunny)
#[cfg(test)]
mod shi_shi_tests {
    use super::*;

    // Testing: structure.standard[0] - Verb (A) + し + （Verb (B) + し）
    #[test]
    fn test_shi_shi_verb() {
        let sentence = "彼女はピアノが弾けるし、スポーツができるし、彼女に出来ないことはないと思う";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "し～し ");
        assert_pattern_range(&patterns, "し～し ", 7, 11); // 弾けるし
    }

    // Testing: structure.standard[1] - ［い］Adjective (A) + し + （［い］Adjective (B) + し）
    #[test]
    fn test_shi_shi_i_adjective() {
        let sentence = "このレストランは高いし、まずいし、何もいいところがない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "し～し ");
        assert_pattern_range(&patterns, "し～し ", 12, 16); // まずいし (first match)
    }

    // Testing: structure.standard[2] - ［な］Adjective (A) + だ + し + （［な］Adjective (B) + だ + し）
    #[test]
    fn test_shi_shi_na_adjective() {
        let sentence = "彼は真面目だし、親切だし、彼と友達でよかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "し～し ");
        assert_pattern_range(&patterns, "し～し ", 2, 7); // 真面目だし
    }

    // Testing: structure.standard[3] - Noun (A) + だ + し + （Noun (B) + だ + し）
    #[test]
    fn test_shi_shi_noun() {
        let sentence = "今日は休みだし、晴れだし、今日は公園に行こう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "し～し ");
        assert_pattern_range(&patterns, "し～し ", 3, 7); // 休みだし
    }
}

// ========== ごとに (every/each time) ==========
// Pattern: ごとに
// Data source: grammar_points_data.json["ごとに"]
//
// Structures to test:
//   - standard[0]: Verb + ごとに
//   - standard[1]: Noun + ごとに
//
// Examples from data:
//   - 失敗をするごとに (every time you fail)
//   - ３時間ごとに (every 3 hours)
#[cfg(test)]
mod gotoni_tests {
    use super::*;

    // Testing: structure.standard[0] - Verb + ごとに
    #[test]
    fn test_gotoni_verb() {
        let sentence = "失敗をする経験は無駄ではありません。失敗をするごとに上達します";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ごとに");
        assert_pattern_range(&patterns, "ごとに", 21, 26); // するごとに
    }

    // Testing: structure.standard[1] - Noun + ごとに (time interval)
    #[test]
    fn test_gotoni_noun_time() {
        let sentence = "私は３時間ごとにお菓子を食べます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ごとに");
        assert_pattern_range(&patterns, "ごとに", 3, 8); // 時間ごとに
    }
}

// ========== かしら (I wonder) ==========
// Pattern: かしら
// Data source: grammar_points_data.json["かしら"]
//
// Structure to test:
//   - standard[0]: Phrase + かしら
//
// Examples from data:
//   - あそこにいるのはタケル君かしら (I wonder if that person is Takeru-kun)
//   - 明日は晴れるかしら (I wonder if it will clear up tomorrow)
//
// Note: かしら is a feminine sentence-ending particle expressing uncertainty/wonder
#[cfg(test)]
mod kashira_tests {
    use super::*;

    // Testing: structure.standard[0] - Phrase + かしら (with noun phrase)
    #[test]
    fn test_kashira_noun_phrase() {
        let sentence = "あそこにいるのはタケル君かしら";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かしら");
        assert_pattern_range(&patterns, "かしら", 12, 15); // かしら
    }

    // Testing: structure.standard[0] - Phrase + かしら (with verb phrase)
    #[test]
    fn test_kashira_verb_phrase() {
        let sentence = "明日は晴れるかしら";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かしら");
        assert_pattern_range(&patterns, "かしら", 6, 9); // かしら
    }

    // Testing: structure.standard[0] - Phrase + かしら (with adjective)
    #[test]
    fn test_kashira_adjective() {
        let sentence = "この映画は面白いかしら";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かしら");
        assert_pattern_range(&patterns, "かしら", 8, 11); // かしら
    }
}
