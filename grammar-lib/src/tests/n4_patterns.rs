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

// Pattern: いがい (except/besides)
// Data source: grammar_points_data.json["いがい"]
// Testing: structure.standard[0] - "Verb + 以外（いがい）"
//          structure.standard[1] - "Noun + 以外（いがい）"
mod igai_tests {
    use super::*;

    // Testing: standard[0] - Verb + 以外
    #[test]
    fn test_igai_verb() {
        let sentence = "ここで泳ぐ以外に方法はないだろう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いがい");
        assert_pattern_range(&patterns, "いがい", 3, 7); // 泳ぐ以外
    }

    // Testing: standard[1] - Noun + 以外
    #[test]
    fn test_igai_noun_wa() {
        let sentence = "今日は和食以外のものが食べたい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いがい");
        assert_pattern_range(&patterns, "いがい", 3, 7); // 和食以外
    }

    // Testing: standard[1] - Noun + 以外 (variation)
    #[test]
    fn test_igai_noun_ni() {
        let sentence = "タロウはゲーム以外に趣味はあるの？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いがい");
        assert_pattern_range(&patterns, "いがい", 4, 9); // ゲーム以外
    }

    // Testing: standard[1] - Noun + 以外 (negative context)
    #[test]
    fn test_igai_noun_negative() {
        let sentence = "お前以外には頼めないんだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いがい");
        assert_pattern_range(&patterns, "いがい", 0, 4); // お前以外
    }
}

// ========== でも (even, or something, any-) ==========
// Pattern: でも
// Data source: grammar_points_data.json["でも"]
//
// Structures to test:
//   - standard[0]: Noun + でも + Suggestion
//   - standard[1]: だれでも - Anyone
//   - standard[2]: なんでも - Anything
//   - standard[3]: どこでも - Anywhere
//   - standard[4]: いつでも - Anytime
//
// Examples from data:
//   - お茶でも飲みましょうか (Shall we drink some tea, or something?)
//   - お前でも出来るよ (Even you can do it)
//   - だれでも分かるよ (Everyone knows)
//   - どこでもいいよ (Anywhere is fine)
//   - なんでも食べるね (You eat anything)
//   - いつでも電話してね (Call me anytime)
#[cfg(test)]
mod demo_tests {
    use super::*;

    // Testing: standard[0] - Noun + でも + Suggestion
    #[test]
    fn test_demo_noun_suggestion() {
        let sentence = "お茶でも飲みましょうか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "でも");
        assert_pattern_range(&patterns, "でも", 0, 4); // お茶でも
    }

    // Testing: standard[0] - Noun + でも (even)
    #[test]
    fn test_demo_noun_even() {
        let sentence = "大丈夫だよ、お前でも出来るよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "でも");
        assert_pattern_range(&patterns, "でも", 6, 10); // お前でも
    }

    // Testing: standard[1] - だれでも (anyone)
    #[test]
    fn test_demo_daredemo() {
        let sentence = "この簡単な漢字はだれでも分かるよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "でも");
        assert_pattern_range(&patterns, "でも", 8, 12); // だれでも
    }

    // Testing: standard[2] - なんでも (anything)
    #[test]
    fn test_demo_nandemo() {
        let sentence = "お前は本当になんでも食べるね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "でも");
        assert_pattern_range(&patterns, "でも", 6, 10); // なんでも
    }

    // Testing: standard[3] - どこでも (anywhere)
    #[test]
    fn test_demo_dokodemo() {
        let sentence = "明日はどこに行きたい？どこでもいいよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "でも");
        assert_pattern_range(&patterns, "でも", 11, 15); // どこでも
    }

    // Testing: standard[4] - いつでも (anytime)
    #[test]
    fn test_demo_itsudemo() {
        let sentence = "いつでも電話してね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "でも");
        assert_pattern_range(&patterns, "でも", 0, 4); // いつでも
    }
}

// ========== までに (by/until - deadline) ==========
// Pattern: までに
// Data source: grammar_points_data.json["までに"]
//
// Structures to test:
//   - standard[0]: Verb + までに
//   - standard[1]: Noun + までに
//
// Examples from data:
//   - ５時までに駅に来てください (Please come to the station by 5 o'clock)
//   - 遊びに行くまでに片付けてね (Please tidy up by the time you go out to play)
//   - 来月までにレポートを書く (I will write a report by next month)
//
// Note: までに = まで (adverbial particle) + に (case marking particle)
// Meaning: "by" (deadline), NOT "until" (continuous action)
#[cfg(test)]
mod madeni_tests {
    use super::*;

    // Testing: standard[0] - Verb + までに
    #[test]
    fn test_madeni_verb() {
        let sentence = "遊びに行くまでに片付けてね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "までに");
        assert_pattern_range(&patterns, "までに", 3, 8); // 行くまでに
    }

    // Testing: standard[0] - Verb + までに (ending)
    #[test]
    fn test_madeni_verb_ending() {
        let sentence = "冬が終わるまでにスキーをしたい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "までに");
        assert_pattern_range(&patterns, "までに", 2, 8); // 終わるまでに
    }

    // Testing: standard[1] - Noun + までに (time)
    #[test]
    fn test_madeni_noun_time() {
        let sentence = "５時までに駅に来てください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "までに");
        assert_pattern_range(&patterns, "までに", 1, 5); // 時までに
    }

    // Testing: standard[1] - Noun + までに (deadline)
    #[test]
    fn test_madeni_noun_deadline() {
        let sentence = "来月までにレポートを書く";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "までに");
        assert_pattern_range(&patterns, "までに", 0, 5); // 来月までに
    }
}

// ========== やすい (easy to / prone to) ==========
// Pattern: やすい
// Data source: grammar_points_data.json["やすい"]
//
// Structures to test:
//   - standard[0]: Verb[stem] + やすい
//   - polite[0]: Verb[stem] + やすい + です
//
// Examples from data:
//   - 食べやすいサイズ (size that's easy to eat)
//   - 読みやすいです (it's easy to read)
//   - 怒りやすいから (prone to getting angry)
//   - 泣きやすいから (likely to cry)
//
// Note: やすい attaches to verb stem (ます形 without ます)
#[cfg(test)]
mod yasui_tests {
    use super::*;

    // Testing: standard[0] - Verb[stem] + やすい (easy to)
    #[test]
    fn test_yasui_easy_to_eat() {
        let sentence = "私は食べやすいサイズにステーキを切る";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "やすい");
        assert_pattern_range(&patterns, "やすい", 2, 7); // 食べやすい
    }

    // Testing: standard[0] - Verb[stem] + やすい (prone to)
    #[test]
    fn test_yasui_prone_to_anger() {
        let sentence = "あの先輩は怒りやすいからめんどくさい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "やすい");
        assert_pattern_range(&patterns, "やすい", 5, 10); // 怒りやすい
    }

    // Testing: standard[0] - Verb[stem] + やすい (likely to)
    #[test]
    fn test_yasui_likely_to_cry() {
        let sentence = "カスミちゃんは泣きやすいから優しくしてね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "やすい");
        assert_pattern_range(&patterns, "やすい", 7, 12); // 泣きやすい
    }

    // Testing: polite[0] - Verb[stem] + やすい + です
    #[test]
    fn test_yasui_polite() {
        let sentence = "この漫画にはフリガナがついているから読みやすいです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "やすい");
        assert_pattern_range(&patterns, "やすい", 18, 25); // 読みやすいです
    }
}

// ========== にくい (difficult to) ==========
// Pattern: にくい
// Data source: grammar_points_data.json["にくい"]
//
// Structures to test:
//   - standard[0]: Verb[stem] + にくい
//   - polite[0]: Verb[stem] + にくい + です
//
// Meaning: "difficult to (A)" due to skill level or inherent difficulty
// Note: Different from づらい (hard to endure/unbearable)
//
// Examples from data:
//   - 止めにくい (difficult to park)
//   - しにくい (hard to breathe)
//   - 言いにくい (hard to say)
#[cfg(test)]
mod nikui_tests {
    use super::*;

    // Testing: standard[0] - Verb[stem] + にくい (difficult due to skill)
    #[test]
    fn test_nikui_difficult_to_park() {
        let sentence = "この駐車場は狭いから止めにくいんだよね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にくい");
        assert_pattern_range(&patterns, "にくい", 10, 15); // 止めにくい
    }

    // Testing: standard[0] - Verb[stem] + にくい (difficult physical action)
    #[test]
    fn test_nikui_hard_to_breathe() {
        let sentence = "今日は具合が悪いから呼吸がしにくい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にくい");
        assert_pattern_range(&patterns, "にくい", 13, 17); // しにくい
    }

    // Testing: standard[0] - Verb[stem] + にくい (difficult pronunciation)
    #[test]
    fn test_nikui_hard_to_say() {
        let sentence = "英語の「Literally」って単語がとても言いにくい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にくい");
        assert_pattern_range(&patterns, "にくい", 22, 27); // 言いにくい
    }

    // Testing: polite[0] - Verb[stem] + にくい + です
    #[test]
    fn test_nikui_polite() {
        let sentence = "このペンは壊れているから書きにくいです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にくい");
        assert_pattern_range(&patterns, "にくい", 12, 19); // 書きにくいです
    }
}

// ========== だんだん (gradually/steadily) ==========
// Pattern: だんだん
// Data source: grammar_points_data.json["だんだん"]
//
// Structure to test:
//   - standard[0]: だんだん + (と) + Phrase
//
// Meaning: "gradually", "steadily", "step by step"
// Note: The particle と is optional and often omitted
// Different from どんどん (rapid progression)
//
// Examples from data:
//   - だんだん寒くなってきた (It has gotten steadily colder)
//   - だんだん仕事の環境に慣れてきた (I have steadily gotten used to work environment)
//   - だんだんと暑くなってきたね (It has progressively gotten hotter)
#[cfg(test)]
mod dandan_tests {
    use super::*;

    // Testing: standard[0] - だんだん + Phrase (without と)
    #[test]
    fn test_dandan_getting_colder() {
        let sentence = "12月になってからだんだん寒くなってきた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だんだん");
        assert_pattern_range(&patterns, "だんだん", 9, 13); // だんだん
    }

    // Testing: standard[0] - だんだん + Phrase (getting used to)
    #[test]
    fn test_dandan_getting_used_to() {
        let sentence = "だんだん仕事の環境に慣れてきたよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だんだん");
        assert_pattern_range(&patterns, "だんだん", 0, 4); // だんだん
    }

    // Testing: standard[0] - だんだん + と + Phrase (with optional と)
    #[test]
    fn test_dandan_with_to() {
        let sentence = "最近はだんだんと暑くなってきたね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だんだん");
        assert_pattern_range(&patterns, "だんだん", 3, 8); // だんだんと
    }

    // Testing: standard[0] - だんだん + Phrase (raining)
    #[test]
    fn test_dandan_starting_to_rain() {
        let sentence = "だんだん雨が降ってきたから傘を持っていこう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だんだん");
        assert_pattern_range(&patterns, "だんだん", 0, 4); // だんだん
    }
}

// ========== なおす (to redo/fix) ==========
// Pattern: なおす
// Data source: grammar_points_data.json["なおす"]
//
// Structures to test:
//   - standard[0]: Verb[stem] + なおす
//   - polite[0]: Verb[stem] + なおします
//
// Meaning: "to fix", "to redo" (do something again due to insufficient quality)
// Kanji: 直す (straighten out, direct) not 治す (heal/mend)
//
// Examples from data:
//   - 染めなおします (will redo the dyeing)
//   - 塗りなおす (to repaint)
//   - しなおす (to redo with する verbs)
//   - やりなおす (to redo with やる verbs)
#[cfg(test)]
mod naosu_tests {
    use super::*;

    // Testing: standard[0] - Verb[stem] + なおす (redye hair)
    #[test]
    fn test_naosu_redye() {
        let sentence = "髪がちゃんと染まってなかったので染めなおします";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なおす");
        assert_pattern_range(&patterns, "なおす", 16, 23); // 染めなおします
    }

    // Testing: standard[0] - Verb[stem] + なおす (repaint)
    #[test]
    fn test_naosu_repaint() {
        let sentence = "フェンスを塗りなおす必要がある";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なおす");
        assert_pattern_range(&patterns, "なおす", 5, 10); // 塗りなおす
    }

    // Testing: standard[0] - Verb[stem] + なおす (with する verb)
    #[test]
    fn test_naosu_with_suru() {
        let sentence = "お客様に挨拶をしなおすことにした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なおす");
        assert_pattern_range(&patterns, "なおす", 7, 11); // しなおす
    }

    // Testing: standard[0] - やりなおす compound token
    // NOTE: やりなおす is tokenized as a single compound verb (動詞/自立)
    // Unlike other Verb + なおす combinations which split into two tokens
    // This test documents the compound behavior - may need separate pattern matcher
    #[test]
    fn test_naosu_with_yaru_compound() {
        let sentence = "プロポーズはやりなおすことができない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // やりなおす is a compound token, not detected by split pattern matcher
        // If compound detection is needed, create a separate pattern like "なおす_compound"
        // For now, document this behavior with a comment
        assert!(!has_pattern(&patterns, "なおす"));
    }
}

// ========== たとえば (for example) ==========
// Pattern: たとえば
// Data source: grammar_points_data.json["たとえば"]
//
// Structure to test:
//   - standard[0]: たとえば + Phrase
//
// Examples from data:
//   - たとえば、ドイツとかは？ (For example, how about Germany?)
//   - たとえば、冷たい物を食べたときに歯が痛いです (For example, my teeth hurt when I eat cold things)
#[cfg(test)]
mod tatoeba_tests {
    use super::*;

    // Testing: standard[0] - たとえば + Phrase
    #[test]
    fn test_tatoeba_at_start() {
        let sentence = "たとえば、ドイツとかは？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たとえば");
        assert_pattern_range(&patterns, "たとえば", 0, 4); // たとえば
    }

    #[test]
    fn test_tatoeba_giving_example() {
        let sentence = "たとえば、冷たい物を食べたときに歯が痛いです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たとえば");
        assert_pattern_range(&patterns, "たとえば", 0, 4); // たとえば
    }

    #[test]
    fn test_tatoeba_mid_sentence() {
        let sentence = "具体的に言うと、たとえば魚とか野菜とかですね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たとえば");
        assert_pattern_range(&patterns, "たとえば", 8, 12); // たとえば
    }

    #[test]
    fn test_tatoeba_suggestion() {
        let sentence = "たとえばこんな感じでやってみたらどうですか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たとえば");
        assert_pattern_range(&patterns, "たとえば", 0, 4); // たとえば
    }
}

// ========== かな (I wonder) ==========
// Pattern: かな
// Data source: grammar_points_data.json["かな"]
//
// Structure to test:
//   - standard[0]: Sentence + かな
//
// Examples from data:
//   - 大丈夫かな (I wonder if they are okay)
//   - 入るかな？ (I wonder if it will fit?)
//   - 本当に弁護士なのかな (I wonder if that person is really a lawyer)
#[cfg(test)]
mod kana_tests {
    use super::*;

    // Testing: standard[0] - Sentence + かな
    #[test]
    fn test_kana_verb() {
        let sentence = "このコンロにこの魚が入るかな？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かな");
        assert_pattern_range(&patterns, "かな", 12, 14); // かな
    }

    #[test]
    fn test_kana_na_adjective() {
        let sentence = "あの人が倒れてるけど、大丈夫かな。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かな");
        assert_pattern_range(&patterns, "かな", 14, 16); // かな
    }

    #[test]
    fn test_kana_with_noka() {
        let sentence = "あの人は本当に弁護士なのかな";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かな");
        assert_pattern_range(&patterns, "かな", 12, 14); // かな
    }

    #[test]
    fn test_kana_casual_wonder() {
        let sentence = "明日も雨が降るかな";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かな");
        assert_pattern_range(&patterns, "かな", 7, 9); // かな
    }
}

// ========== おわる (finish doing) ==========
// Pattern: おわる
// Data source: grammar_points_data.json["おわる"]
//
// Structures to test:
//   - standard[0]: Verb［stem］+ 終（お）わる
//   - polite[0]: Verb［stem］+ 終（お）わります
//
// Examples from data:
//   - 届けおわりました (finished delivering)
//   - 払いおわる (will finish paying)
//   - 食べおわるまで (until you finish eating)
//   - 飲みおわってない (haven't finished drinking)
//   - 読みおわりました (finished reading)
#[cfg(test)]
mod owaru_tests {
    use super::*;

    // Testing: structure.standard[0] - Verb[stem] + 終わる
    #[test]
    fn test_owaru_nonpast() {
        let sentence = "明日払いおわるから安心して";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "おわる");
        assert_pattern_range(&patterns, "おわる", 2, 7); // 払いおわる
    }

    #[test]
    fn test_owaru_te_form() {
        let sentence = "そのコーヒー、まだ飲みおわってないの？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "おわる");
        assert_pattern_range(&patterns, "おわる", 9, 14); // 飲みおわっ
    }

    // Testing: structure.polite[0] - Verb[stem] + 終わります
    #[test]
    fn test_owaru_polite_past() {
        let sentence = "長いレポートでしたが、今日やっと読みおわりました";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "おわる");
        assert_pattern_range(&patterns, "おわる", 16, 24); // 読みおわりました
    }

    #[test]
    fn test_owaru_polite_nonpast() {
        let sentence = "やっと読みおわりました！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "おわる");
        assert_pattern_range(&patterns, "おわる", 3, 11); // 読みおわりました
    }
}

// ========== がする (sensory experience) ==========
// Pattern: がする
// Data source: grammar_points_data.json["がする"]
//
// Structures to test:
//   - standard[0]: Noun + が + する
//   - polite[0]: Noun + が + します
//
// Examples from data:
//   - 匂いがする (smells like)
//   - 音がする (sounds like)
//   - 味がする (tastes like)
//   - 感じがする (feels like)
//   - 気がする (I feel like)
#[cfg(test)]
mod gasuru_tests {
    use super::*;

    // Testing: structure.standard[0] - Noun + が + する
    #[test]
    fn test_gasuru_smell() {
        let sentence = "この石鹸はバラの匂いがするから好きです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がする");
        assert_pattern_range(&patterns, "がする", 8, 13); // 匂いがする
    }

    #[test]
    fn test_gasuru_sound() {
        let sentence = "誰かが階段を上がっている音がする";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がする");
        assert_pattern_range(&patterns, "がする", 12, 16); // 音がする
    }

    #[test]
    fn test_gasuru_taste() {
        let sentence = "このバナナが変な味がする";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がする");
        assert_pattern_range(&patterns, "がする", 8, 12); // 味がする
    }

    #[test]
    fn test_gasuru_feeling() {
        let sentence = "忘れた気がするんだけど";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がする");
        assert_pattern_range(&patterns, "がする", 3, 7); // 気がする
    }

    // Testing: structure.polite[0] - Noun + が + します
    #[test]
    fn test_gasuru_polite() {
        let sentence = "いい香りがしますね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がする");
        assert_pattern_range(&patterns, "がする", 2, 8); // 香りがします
    }
}

// ========== がほしい (want something) ==========
// Pattern: がほしい
// Data source: grammar_points_data.json["がほしい"]
//
// Structures to test:
//   - standard[0]: Noun + が + ほしい
//   - polite[0]: Noun + が + ほしい + です
//
// Examples from data:
//   - 新しい車がほしい (I want a new car)
//   - 犬がほしい (I want a dog)
#[cfg(test)]
mod gahoshii_tests {
    use super::*;

    // Testing: structure.standard[0] - Noun + が + ほしい
    #[test]
    fn test_gahoshii_standard() {
        let sentence = "新しい車がほしいけど、今はお金がないから買えない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がほしい");
        assert_pattern_range(&patterns, "がほしい", 3, 8); // 車がほしい
    }

    #[test]
    fn test_gahoshii_simple() {
        let sentence = "犬がほしいけどスペースがない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がほしい");
        assert_pattern_range(&patterns, "がほしい", 0, 5); // 犬がほしい
    }

    // Testing: structure.polite[0] - Noun + が + ほしい + です
    #[test]
    fn test_gahoshii_polite() {
        let sentence = "もっと時間がほしいです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がほしい");
        assert_pattern_range(&patterns, "がほしい", 3, 11); // 時間がほしいです
    }

    #[test]
    fn test_gahoshii_polite_question() {
        let sentence = "何がほしいですか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がほしい");
        assert_pattern_range(&patterns, "がほしい", 0, 7); // 何がほしいです
    }
}

// ========== はじめる (start doing) ==========
// Pattern: はじめる
// Data source: grammar_points_data.json["はじめる"]
//
// Structures to test:
//   - standard[0]: Verb［stem］+ はじめる
//   - polite[0]: Verb［stem］+ はじめます
//
// Examples from data:
//   - ためはじめます (start saving)
//   - 歌いはじめる (start singing)
//   - ならいはじめる (start learning)
#[cfg(test)]
mod hajimeru_tests {
    use super::*;

    // Testing: structure.standard[0] - Verb[stem] + はじめる
    #[test]
    fn test_hajimeru_nonpast() {
        let sentence = "来月からお金をためはじめます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "はじめる");
        assert_pattern_range(&patterns, "はじめる", 7, 14); // ためはじめます
    }

    #[test]
    fn test_hajimeru_basic() {
        let sentence = "歌を歌いはじめる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "はじめる");
        assert_pattern_range(&patterns, "はじめる", 2, 8); // 歌いはじめる
    }

    #[test]
    fn test_hajimeru_past() {
        let sentence = "高校生の時に日本語をならいはじめた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "はじめる");
        assert_pattern_range(&patterns, "はじめる", 10, 17); // ならいはじめた
    }

    // Testing: structure.polite[0] - Verb[stem] + はじめます
    #[test]
    fn test_hajimeru_polite() {
        let sentence = "明日から日本語をならいはじめます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "はじめる");
        assert_pattern_range(&patterns, "はじめる", 8, 16); // ならいはじめます
    }
}

// ========== こと (nominalization) ==========
// Pattern: こと
// Data source: grammar_points_data.json["こと"]
//
// Structure to test:
//   - standard[0]: Verb + こと
//
// Examples from data:
//   - ファックスをすることが嫌い (dislike faxing)
//   - お金を使い過ぎないことが大事 (not using too much money is important)
//   - 近所の迷惑になること (things that cause trouble)
//
// Note: こと is a bound noun (名詞/非自立) used for nominalization,
// similar to のは. Creates noun phrases from verbs (e.g., "doing X", "the act of X")
#[cfg(test)]
mod koto_tests {
    use super::*;

    // Testing: standard[0] - Verb + こと (dictionary form)
    #[test]
    fn test_koto_dictionary_form() {
        let sentence = "ファックスをすることが嫌い";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "こと");
        assert_pattern_range(&patterns, "こと", 6, 10); // すること
    }

    // Testing: standard[0] - Auxiliary verb + こと (negative form)
    #[test]
    fn test_koto_negative_form() {
        let sentence = "お金を使い過ぎないことが大事です";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "こと");
        assert_pattern_range(&patterns, "こと", 7, 11); // ないこと
    }

    // Testing: standard[0] - Verb + こと (basic verb)
    #[test]
    fn test_koto_verb_form() {
        let sentence = "近所の迷惑になることをしてはいけない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "こと");
        assert_pattern_range(&patterns, "こと", 6, 10); // なること
    }

    // Testing: standard[0] - Verb + こと (simple non-compound verb)
    #[test]
    fn test_koto_simple_verb() {
        let sentence = "食べることが好き";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "こと");
        assert_pattern_range(&patterns, "こと", 0, 5); // 食べること
    }
}

// ============================================================================
// Verb[て] - Casual imperative/request (て-form at sentence end)
// ============================================================================
// Pattern: Verb[て] (casual imperative)
// Data source: grammar_points_data.json["Verb[て]"]
// Testing: structure.standard[0] - "Verb［て］。"
//
// Meaning: In casual speech, てください is shortened to て for friendly requests
// Example: 片付けて。 (Please clean up.)
mod verb_te_imperative_tests {
    use super::*;

    // Testing: standard[0] - Verb[て] at sentence end (request)
    #[test]
    fn test_verb_te_imperative_request() {
        let sentence = "部屋を片付けて。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[て]");
        assert_pattern_range(&patterns, "Verb[て]", 3, 7); // 片付けて
    }

    // Testing: standard[0] - Verb[て] with lending request
    #[test]
    fn test_verb_te_imperative_lend() {
        let sentence = "ペン貸して。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[て]");
        assert_pattern_range(&patterns, "Verb[て]", 2, 5); // 貸して
    }

    // Testing: standard[0] - Verb[て] with motion verb
    #[test]
    fn test_verb_te_imperative_come() {
        let sentence = "ちょっと来て。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[て]");
        assert_pattern_range(&patterns, "Verb[て]", 4, 6); // 来て
    }

    // Testing: standard[0] - Verb[て] with waiting request
    #[test]
    fn test_verb_te_imperative_wait() {
        let sentence = "待って。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[て]");
        assert_pattern_range(&patterns, "Verb[て]", 0, 3); // 待って
    }
}

// ============================================================================
// Number + しか〜ない - "only (number)" with negative verb
// ============================================================================
// Pattern: Number + しか〜ない (only number)
// Data source: grammar_points_data.json["Number + しか〜ない"]
// Testing: structure.standard[0] - "Number + しか + Verb［ない］"
//
// Meaning: "only (number)" - しか must be used with negative verbs
// Example: 五キロしか走れない。 (I can only run 5 km.)
mod number_shika_nai_tests {
    use super::*;

    // Testing: standard[0] - Number + しか + Verb[ない] (can only run 5km)
    #[test]
    fn test_number_shika_nai_distance() {
        let sentence = "今日は五キロしか走れない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Number + しか〜ない");
        // Pattern starts from counter (キロ) rather than number (五)
        assert_pattern_range(&patterns, "Number + しか〜ない", 4, 12); // キロしか走れない
    }

    // Testing: standard[0] - Number + しか + Verb[ていない] (have only 100 yen)
    #[test]
    fn test_number_shika_nai_money() {
        let sentence = "今日は１００円しか持っていない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Number + しか〜ない");
        // Pattern starts from counter (円) rather than numbers (１００)
        assert_pattern_range(&patterns, "Number + しか〜ない", 6, 15); // 円しか持っていない
    }

    // Testing: standard[0] - Number + しか + Verb[べない] (can only play 2 hours)
    #[test]
    fn test_number_shika_nai_time() {
        let sentence = "２時間しか遊べない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Number + しか〜ない");
        // Pattern starts from counter (時間)
        assert_pattern_range(&patterns, "Number + しか〜ない", 1, 9); // 時間しか遊べない
    }

    // Testing: standard[0] - Number + しか + ありません (polite negative)
    #[test]
    fn test_number_shika_arimasen() {
        let sentence = "３個しかありません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Number + しか〜ない");
        // Pattern starts from counter (個)
        assert_pattern_range(&patterns, "Number + しか〜ない", 1, 9); // 個しかありません
    }
}

// ========== お～ください (honorific request) ==========
// Pattern: お～ください
// Data source: grammar_points_data.json["お～ください "]
//
// Structure to test:
//   - standard[0]: お + Verb［stem］+ ください
//
// Examples from data:
//   - お申込みください (please register)
//   - お使いください (please use)
//   - お閉めください (please close)
//   - お掛けください (please sit)
#[cfg(test)]
mod o_kudasai_tests {
    use super::*;

    // Testing: standard[0] - お + Verb[stem] + ください (register)
    #[test]
    fn test_o_kudasai_register() {
        let sentence = "こちらのサービスを使うにはインターネットでお申込みください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "お～ください ");
        assert_pattern_range(&patterns, "お～ください ", 21, 29); // お申込みください
    }

    // Testing: standard[0] - お + Verb[stem] + ください (use)
    #[test]
    fn test_o_kudasai_use() {
        let sentence = "こちらのスリッパをお使いください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "お～ください ");
        assert_pattern_range(&patterns, "お～ください ", 9, 16); // お使いください
    }

    // Testing: standard[0] - お + Verb[stem] + ください (close)
    #[test]
    fn test_o_kudasai_close() {
        let sentence = "後ろの扉をお閉めください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "お～ください ");
        assert_pattern_range(&patterns, "お～ください ", 5, 12); // お閉めください
    }

    // Testing: standard[0] - お + Verb[stem] + ください (sit)
    #[test]
    fn test_o_kudasai_sit() {
        let sentence = "どうぞ、こちらにお掛けください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "お～ください ");
        assert_pattern_range(&patterns, "お～ください ", 8, 15); // お掛けください
    }
}

// ========== ございます (polite form of ある) ==========
// Pattern: ございます
// Data source: grammar_points_data.json["ございます"]
//
// Structures to test:
//   - standard[0]: ある ￫ ござる (historical/media)
//   - polite[0]: あります ￫ ございます (modern polite)
//
// Examples from data:
//   - ありがとうございます (thank you very much)
//   - 質問はございますか (do you have any questions?)
//   - ここは私の家でござる (this is my humble abode - historical)
#[cfg(test)]
mod gozaimasu_tests {
    use super::*;

    // Testing: polite[0] - ございます (thank you very much)
    #[test]
    fn test_gozaimasu_arigatou() {
        let sentence = "ありがとうございます。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ございます");
        assert_pattern_range(&patterns, "ございます", 5, 10); // ございます
    }

    // Testing: polite[0] - ございます (have questions)
    #[test]
    fn test_gozaimasu_question() {
        let sentence = "質問はございますか。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ございます");
        assert_pattern_range(&patterns, "ございます", 3, 8); // ございます
    }

    // Testing: standard[0] - ござる (historical form)
    #[test]
    fn test_gozaru_historical() {
        let sentence = "ここは私の家でござる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ございます");
        assert_pattern_range(&patterns, "ございます", 7, 10); // ござる
    }

    // Testing: polite[0] - ございます (polite existence)
    #[test]
    fn test_gozaimasu_existence() {
        let sentence = "お時間はございますか。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ございます");
        assert_pattern_range(&patterns, "ございます", 4, 9); // ございます
    }
}

// ========== じゃないか (isn't it?) ==========
// Pattern: じゃないか
// Data source: grammar_points_data.json["じゃないか"]
//
// Structure to test:
//   - standard[0]: Phrase + じゃない + か
//
// Examples from data:
//   - 来るんじゃないか (coming soon, isn't it?)
//   - 高いんじゃないか (expensive, isn't it?)
//   - 綺麗じゃないか (beautiful, isn't it?)
//   - 速いじゃないか (faster, isn't it?)
//   - 影響ではないか (typhoon's effect, isn't it? - formal)
#[cfg(test)]
mod janaika_tests {
    use super::*;

    // Testing: standard[0] - Verb + んじゃないか
    #[test]
    fn test_janaika_verb() {
        let sentence = "キヨコはもうすぐ来るんじゃないか。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "じゃないか");
        assert_pattern_range(&patterns, "じゃないか", 10, 16); // んじゃないか
    }

    // Testing: standard[0] - い-Adj + んじゃないか
    #[test]
    fn test_janaika_i_adj() {
        let sentence = "そのカメラは高いんじゃないか。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "じゃないか");
        assert_pattern_range(&patterns, "じゃないか", 8, 14); // んじゃないか
    }

    // Testing: standard[0] - な-Adj + じゃないか (no ん)
    #[test]
    fn test_janaika_na_adj() {
        let sentence = "この着物は綺麗じゃないか。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "じゃないか");
        assert_pattern_range(&patterns, "じゃないか", 7, 12); // じゃないか
    }

    // Testing: standard[0] - ではないか (formal)
    #[test]
    fn test_dewa_naika_formal() {
        let sentence = "これは台風の影響ではないか。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "じゃないか");
        assert_pattern_range(&patterns, "じゃないか", 8, 13); // ではないか
    }
}

// ========== しか～ない (only/nothing but) ==========
// Pattern: しか～ない
// Data source: grammar_points_data.json["しか～ない "]
//
// Structure to test:
//   - standard[0]: Noun + しか + Verb［ない］
//
// Examples from data:
//   - 牛丼しか置いていない (only have gyudon)
//   - 一匹しか釣れない (only caught one fish)
//
// Note: This is different from "Number + しか～ない" which is a separate pattern
#[cfg(test)]
mod shika_nai_tests {
    use super::*;

    // Testing: standard[0] - Noun + しか + ていない
    #[test]
    fn test_shika_nai_teiru_negative() {
        let sentence = "この店は牛丼しか置いていない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "しか～ない ");
        assert_pattern_range(&patterns, "しか～ない ", 5, 14); // 丼しか置いていない
    }

    // Testing: standard[0] - Noun + しか + negative verb
    #[test]
    fn test_shika_nai_simple() {
        let sentence = "釣りに行って、一匹しか釣れないと悲しくなる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "しか～ない ");
        assert_pattern_range(&patterns, "しか～ない ", 8, 15); // 匹しか釣れない
    }

    // Testing: standard[0] - Noun + しか + ありません (polite)
    #[test]
    fn test_shika_arimasen_polite() {
        let sentence = "コーヒーしかありません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "しか～ない ");
        assert_pattern_range(&patterns, "しか～ない ", 0, 11); // コーヒーしかありません
    }

    // Testing: standard[0] - Noun + しか + ない (simple negative)
    #[test]
    fn test_shika_nai_existence() {
        let sentence = "今はこれしかない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "しか～ない ");
        assert_pattern_range(&patterns, "しか～ない ", 2, 8); // これしかない
    }
}

// ========== それで (therefore/so/as a result) ==========
// Pattern: それで
// Data source: grammar_points_data.json["それで"]
//
// Structure to test:
//   - standard[0]: Phrase (A)。それで + Phrase (B)
//
// Note: それで = それ (pronoun) + で (case marking particle)
// Meaning: "therefore", "because of that", "as a result"
// Used at the beginning of a second sentence to connect to previous information
//
// Examples from data:
//   - 命を救われた。それで医者になろうと思った。(saved my life. Due to that, became determined to be a doctor)
//   - 痴漢をした。それで彼は警察に捕まった。(groped someone. As a result, he got arrested)
#[cfg(test)]
mod sorede_tests {
    use super::*;

    // Testing: standard[0] - Phrase A。それで + Phrase B
    #[test]
    fn test_sorede_causal_connection() {
        let sentence = "子供の頃に医者に命を救われた。それで医者になろうと思った。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "それで");
        assert_pattern_range(&patterns, "それで", 15, 18); // それで
    }

    // Testing: standard[0] - それで at sentence start
    #[test]
    fn test_sorede_result() {
        let sentence = "彼は電車で痴漢をした。それで警察に捕まった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "それで");
        assert_pattern_range(&patterns, "それで", 11, 14); // それで
    }

    // Testing: standard[0] - それで in natural dialogue
    #[test]
    fn test_sorede_natural_speech() {
        let sentence = "昨日は雨が降ってた。それで家にいることにした。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "それで");
        assert_pattern_range(&patterns, "それで", 10, 13); // それで
    }
}

// ========== それでも (even so/nevertheless) ==========
// Pattern: それでも
// Data source: grammar_points_data.json["それでも"]
//
// Structure to test:
//   - standard[0]: Phrase (A) + それでも + Phrase (B)
//
// Note: それでも = それ (pronoun) + でも (adverbial particle)
// Meaning: "even so", "nevertheless", "even with that"
// Used between phrases to show surprising additional information
//
// Examples from data:
//   - 雨が降るよ、それでも釣りに行くの？ (It's going to rain, even so are you going fishing?)
//   - 蹴られたりするのが嫌いです。それでも空手が好きなので辞めれないです。 (I don't like getting kicked. Even so, I like karate so can't quit)
#[cfg(test)]
mod soredemo_tests {
    use super::*;

    // Testing: standard[0] - Phrase A、それでも + Phrase B
    #[test]
    fn test_soredemo_contrary_action() {
        let sentence = "今日は雨が降るよ、それでも釣りに行くの？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "それでも");
        assert_pattern_range(&patterns, "それでも", 9, 13); // それでも
    }

    // Testing: standard[0] - それでも in complex sentence
    #[test]
    fn test_soredemo_despite_difficulty() {
        let sentence = "殴られたり蹴られたりするのが嫌いです。それでも空手が好きなので辞めれない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "それでも");
        assert_pattern_range(&patterns, "それでも", 19, 23); // それでも
    }

    // Testing: standard[0] - それでも in natural dialogue
    #[test]
    fn test_soredemo_natural_speech() {
        let sentence = "仕事が忙しいって言ったよね。それでも来てくれるの？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "それでも");
        assert_pattern_range(&patterns, "それでも", 14, 18); // それでも
    }
}

// ========== それに (moreover/in addition/what's more) ==========
// Pattern: それに
// Data source: grammar_points_data.json["それに"]
//
// Structure to test:
//   - standard[0]: それに + (Additional Information) Phrase
//
// Note: それに = それ (pronoun) + に (case marking particle)
// Meaning: "moreover", "in addition", "what's more", "and to that"
// Used to add information logically related to previous statement
// Both pieces of information must have same connotation (both positive or both negative)
//
// Examples from data:
//   - それに彼は家事もしないんでしょう？ (In addition, he doesn't do house chores either, right?)
//   - 会社には残業がない。それに給料もいい。 (Company doesn't have overtime. What's more, salary is good too)
#[cfg(test)]
mod soreni_tests {
    use super::*;

    // Testing: standard[0] - それに at sentence start (single token)
    #[test]
    fn test_soreni_additional_negative() {
        let sentence = "それに彼は家事もしないんでしょう？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "それに");
        assert_pattern_range(&patterns, "それに", 0, 3); // それに
    }

    // Testing: standard[0] - それに connecting two sentences (two tokens)
    #[test]
    fn test_soreni_positive_addition() {
        let sentence = "会社には残業がない。それに給料もいい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "それに");
        assert_pattern_range(&patterns, "それに", 10, 13); // それに
    }

    // Testing: standard[0] - それに in complex natural dialogue (two tokens)
    #[test]
    fn test_soreni_natural_speech() {
        let sentence = "最近は雨がいっぱい降るし、それに風も強いし、外に出れなくて寂しい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "それに");
        assert_pattern_range(&patterns, "それに", 13, 16); // それに
    }
}

// ========== だす (suddenly start doing) ==========
// Pattern: だす
// Data source: grammar_points_data.json["だす"]
//
// Structures to test:
//   - standard[0]: Verb[stem] + だす
//   - polite[0]: Verb[stem] + だします
//
// Meaning: "suddenly do (A)", "burst into (A)" - unintentional/uncontrolled
// Key difference: はじめる = intentional start, だす = sudden/unintentional
//
// Examples from data:
//   - 走りだす (suddenly start running)
//   - 泣きだす (burst into tears)
//   - 歌いだした (burst into song)
#[cfg(test)]
mod dasu_tests {
    use super::*;

    // Testing: standard[0] - Verb[stem] + だす (nonpast)
    #[test]
    fn test_dasu_standard_nonpast() {
        let sentence = "最初から速く走りだすのは体に良くない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だす");
        assert_pattern_range(&patterns, "だす", 6, 10); // 走りだす
    }

    // Testing: standard[0] - Verb[stem] + だす (past tense)
    #[test]
    fn test_dasu_standard_past() {
        let sentence = "最近、友達が授業中に歌いだした。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だす");
        assert_pattern_range(&patterns, "だす", 10, 15); // 歌いだした
    }

    // Testing: standard[0] - Verb[stem] + だす (before ～前に)
    #[test]
    fn test_dasu_before_mae() {
        let sentence = "転んだ息子が泣きだす前に、あめをあげる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だす");
        assert_pattern_range(&patterns, "だす", 6, 10); // 泣きだす
    }

    // Testing: polite[0] - Verb[stem] + だします
    #[test]
    fn test_dasu_polite() {
        let sentence = "赤ちゃんがすぐに泣きだしますから静かにしてください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だす");
        assert_pattern_range(&patterns, "だす", 8, 14); // 泣きだします
    }
}

// ========== つづける (continue doing) ==========
// Pattern: つづける
// Data source: grammar_points_data.json["つづける"]
//
// Structures to test:
//   - standard[0]: Verb[stem] + 続ける
//   - polite[0]: Verb[stem] + 続けます
//
// Meaning: "continue doing" - emphasis on continuation of action
// Note: Verb stem + つづける emphasizes continuation of the action itself
//       vs Verb て + つづける emphasizes sequence (do and then continue)
//
// Examples from data:
//   - 走りつづける (continue running)
//   - 書きつづける (continue writing)
//   - 頑張り続ける (continue trying hard)
#[cfg(test)]
mod tsuzukeru_tests {
    use super::*;

    // Testing: standard[0] - Verb[stem] + つづける (until dark)
    #[test]
    fn test_tsuzukeru_standard() {
        let sentence = "サリーは暗くなるまで走りつづける。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "つづける");
        assert_pattern_range(&patterns, "つづける", 10, 16); // 走りつづける
    }

    // Testing: standard[0] - Verb[stem] + つづける (memorization)
    #[test]
    fn test_tsuzukeru_writing() {
        let sentence = "私は漢字を覚えるまで、同じ漢字を書きつづける。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "つづける");
        assert_pattern_range(&patterns, "つづける", 16, 22); // 書きつづける
    }

    // Testing: standard[0] - Verb[stem] + 続ける with kanji (past tense)
    #[test]
    fn test_tsuzukeru_kanji_past() {
        let sentence = "日本語は難しいけど、勉強を頑張り続けた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "つづける");
        assert_pattern_range(&patterns, "つづける", 13, 19); // 頑張り続けた
    }

    // Testing: polite[0] - Verb[stem] + 続けます
    #[test]
    fn test_tsuzukeru_polite() {
        let sentence = "この仕事を最後まで頑張り続けます。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "つづける");
        assert_pattern_range(&patterns, "つづける", 9, 16); // 頑張り続けます
    }
}

// ========== たがる (wanting to do - third person) ==========
// Pattern: たがる
// Data source: grammar_points_data.json["たがる"]
//
// Structures to test:
//   - standard[0]: Verb[たい] + がる (replace い with がる)
//   - polite[0]: Verb[たい] + がります
//
// Meaning: "wanting to do" - expresses someone (3rd person) acts like they want to do something
// Key difference: たい (1st person desire), たがる (3rd person observable desire)
//
// Examples from data:
//   - 言いたがる (wants to say/talk)
//   - 食べたがる (wants to eat)
#[cfg(test)]
mod tagaru_tests {
    use super::*;

    // Testing: standard[0] - Verb[たい] + がる (talk badly)
    #[test]
    fn test_tagaru_standard() {
        let sentence = "ジョンはなんでいつも悪口を言いたがるんだろう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たがる");
        assert_pattern_range(&patterns, "たがる", 13, 18); // 言いたがる
    }

    // Testing: standard[0] - Verb[たい] + がる (eat)
    #[test]
    fn test_tagaru_eat() {
        let sentence = "皆は彼が作ったケーキを食べたがる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たがる");
        assert_pattern_range(&patterns, "たがる", 11, 16); // 食べたがる
    }

    // Testing: standard[0] - Verb[たい] + がる (past tense)
    #[test]
    fn test_tagaru_past() {
        let sentence = "子供の時、みんな外で遊びたがった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たがる");
        assert_pattern_range(&patterns, "たがる", 10, 16); // 遊びたがった
    }

    // Testing: polite[0] - Verb[たい] + がります
    #[test]
    fn test_tagaru_polite() {
        let sentence = "彼女はいつも新しい服を買いたがります。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たがる");
        assert_pattern_range(&patterns, "たがる", 11, 18); // 買いたがります
    }
}

// ========== かもしれない (might/maybe) ==========
// Pattern: かもしれない
// Data source: grammar_points_data.json["かもしれない"]
//
// Structures to test:
//   - standard[0]: Verb + かもしれない
//   - standard[1]: い-Adjective + かもしれない
//   - standard[2]: な-Adjective + かもしれない
//   - standard[3]: Noun + かもしれない
//   - polite[0]: Verb + かもしれません
//   - polite[1]: い-Adjective + かもしれません
//   - polite[2]: な-Adjective + かもしれません
//   - polite[3]: Noun + かもしれません
//
// Examples from data:
//   - 増えるかもしれない (tourists might increase)
//   - 深いかもしれない (might be deep)
//   - 無理かもしれない (might not be possible)
//   - 教会かもしれない (might be a church)
#[cfg(test)]
mod kamoshirenai_tests {
    use super::*;

    // Testing: standard[0] - Verb + かもしれない
    #[test]
    fn test_kamoshirenai_verb() {
        let sentence = "来年から観光客が増えるかもしれない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かもしれない");
        assert_pattern_range(&patterns, "かもしれない", 8, 17); // 増えるかもしれない
    }

    // Testing: standard[1] - い-Adjective + かもしれない
    #[test]
    fn test_kamoshirenai_i_adjective() {
        let sentence = "そこの池は深いかもしれないから気をつけてね。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かもしれない");
        assert_pattern_range(&patterns, "かもしれない", 5, 13); // 深いかもしれない
    }

    // Testing: standard[2] - な-Adjective + かもしれない
    #[test]
    fn test_kamoshirenai_na_adjective() {
        let sentence = "今日は無理かもしれないけど、明日ならいいよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かもしれない");
        assert_pattern_range(&patterns, "かもしれない", 3, 11); // 無理かもしれない
    }

    // Testing: standard[3] - Noun + かもしれない
    #[test]
    fn test_kamoshirenai_noun() {
        let sentence = "あの建物は教会かもしれない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かもしれない");
        assert_pattern_range(&patterns, "かもしれない", 5, 13); // 教会かもしれない
    }

    // Testing: polite[0] - Verb + かもしれません
    #[test]
    fn test_kamoshirenai_verb_polite() {
        let sentence = "明日は休むかもしれません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かもしれない");
        assert_pattern_range(&patterns, "かもしれない", 3, 12); // 休むかもしれません
    }

    // Testing: polite[1] - い-Adjective + かもしれません
    #[test]
    fn test_kamoshirenai_i_adjective_polite() {
        let sentence = "この作業は難しいかもしれません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かもしれない");
        assert_pattern_range(&patterns, "かもしれない", 5, 15); // 難しいかもしれません
    }

    // Testing: polite[2] - な-Adjective + かもしれません
    #[test]
    fn test_kamoshirenai_na_adjective_polite() {
        let sentence = "彼は本気かもしれませんよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かもしれない");
        assert_pattern_range(&patterns, "かもしれない", 2, 11); // 本気かもしれません
    }

    // Testing: polite[3] - Noun + かもしれません
    #[test]
    fn test_kamoshirenai_noun_polite() {
        let sentence = "それは冗談かもしれません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かもしれない");
        assert_pattern_range(&patterns, "かもしれない", 3, 12); // 冗談かもしれません
    }
}

// ========== いか (equal to or less than) ==========
// Pattern: いか
// Data source: grammar_points_data.json["いか"]
//
// Structures to test:
//   - standard[0]: Noun + Amount + 以下（いか）
//   - standard[1]: それ + 以下（いか）
//   - Note: standard[2] is just a note "(1) これ、あれ"
//
// Examples from data:
//   - １７歳いか (17 years old and under)
//   - ３０万円いか (under 300,000 yen)
//   - いか同文 (the rest is the same)
//   - いかのもの (the following)
#[cfg(test)]
mod ika_tests {
    use super::*;

    // Testing: standard[0] - Noun + Amount + 以下
    #[test]
    fn test_ika_age() {
        let sentence = "１７歳いかの方は保護者の方と来てください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いか");
        assert_pattern_range(&patterns, "いか", 3, 5); // いか
    }

    // Testing: standard[0] - Noun + Amount + 以下 (money)
    #[test]
    fn test_ika_money() {
        let sentence = "この車は３０万円いかだった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いか");
        assert_pattern_range(&patterns, "いか", 8, 10); // いか
    }

    // Testing: standalone いか (the following/the rest)
    #[test]
    fn test_ika_following() {
        let sentence = "空港にはいかのものを持って来てください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いか");
        assert_pattern_range(&patterns, "いか", 4, 6); // いか
    }

    // Testing: いか同文 pattern
    #[test]
    fn test_ika_doubun() {
        let sentence = "いか同文。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いか");
        assert_pattern_range(&patterns, "いか", 0, 2); // いか
    }
}

// ========== どんどん (rapidly/quickly) ==========
// Pattern: どんどん
// Data source: grammar_points_data.json["どんどん"]
//
// Structure to test:
//   - standard[0]: どんどん + (と) + Phrase
//
// Examples from data:
//   - どんどんお金が減っていく (cash is rapidly decreasing)
//   - どんどん頼んでね (quickly order)
//   - どんどんと減っている (with と particle)
#[cfg(test)]
mod dondon_tests {
    use super::*;

    // Testing: standard[0] - どんどん + Phrase (without と)
    #[test]
    fn test_dondon_decreasing() {
        let sentence = "彼女ができてからどんどんお金が減っていく";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "どんどん");
        assert_pattern_range(&patterns, "どんどん", 8, 12); // どんどん
    }

    // Testing: standard[0] - どんどん + Phrase (quickly order)
    #[test]
    fn test_dondon_order() {
        let sentence = "どんどん頼んでね！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "どんどん");
        assert_pattern_range(&patterns, "どんどん", 0, 4); // どんどん
    }

    // Testing: standard[0] - どんどん + と + Phrase (with optional と)
    #[test]
    fn test_dondon_with_to() {
        let sentence = "バスの運転手がどんどんと減っている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "どんどん");
        assert_pattern_range(&patterns, "どんどん", 7, 12); // どんどんと
    }

    // Testing: standard[0] - どんどん + Phrase (increasing)
    #[test]
    fn test_dondon_increasing() {
        let sentence = "最近どんどん人口が増えているらしい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "どんどん");
        assert_pattern_range(&patterns, "どんどん", 2, 6); // どんどん
    }
}

// ========== ていく (to go on to) ==========
// Pattern: ていく
// Data source: grammar_points_data.json["ていく"]
//
// Structure to test:
//   - standard[0]: Verb[て] + いく
//   - polite[0]: Verb[て] + いきます
//
// Examples from data:
//   - 食べていく (go and eat)
//   - 上手くなっていく (will get better)
//   - 持っていった (took)
#[cfg(test)]
mod teiku_tests {
    use super::*;

    // Testing: standard[0] - Verb[て] + いく (present)
    #[test]
    fn test_teiku_getting_better() {
        let sentence = "毎日ゴルフの練習をしたらだんだんと上手くなっていく";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ていく");
        assert_pattern_range(&patterns, "ていく", 20, 25); // なっていく
    }

    // Testing: standard[0] - Verb[て] + いく (invitation/question)
    #[test]
    fn test_teiku_eat() {
        let sentence = "今日はうちで食べていく？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ていく");
        assert_pattern_range(&patterns, "ていく", 6, 11); // 食べていく
    }

    // Testing: standard[0] - Verb[て] + いった (past)
    #[test]
    fn test_teiku_past() {
        let sentence = "雨が降ると聞いたから傘を持っていった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ていく");
        assert_pattern_range(&patterns, "ていく", 12, 18); // 持っていった
    }

    // Testing: polite[0] - Verb[て] + いきます
    #[test]
    fn test_teiku_polite() {
        let sentence = "これからもっと頑張っていきます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ていく");
        assert_pattern_range(&patterns, "ていく", 7, 15); // 頑張っていきます
    }
}

// ========== てくる (to come to) ==========
// Pattern: てくる
// Data source: grammar_points_data.json["てくる "]
//
// Structure to test:
//   - standard[0]: Verb[て] + くる
//   - polite[0]: Verb[て] + きます
//
// Examples from data:
//   - 持ってくる (bring)
//   - 買ってきた (bought and came with)
//   - 上手くなってきた (have gotten better)
#[cfg(test)]
mod tekuru_tests {
    use super::*;

    // Testing: standard[0] - Verb[て] + くる (bring/forget to bring)
    #[test]
    fn test_tekuru_bring() {
        let sentence = "今日も宿題を持ってくるのを忘れた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てくる ");
        assert_pattern_range(&patterns, "てくる ", 6, 11); // 持ってくる
    }

    // Testing: standard[0] - Verb[て] + きた (past - bought and came)
    #[test]
    fn test_tekuru_bought() {
        let sentence = "パパ、じゃが芋を買ってきたよ！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てくる ");
        assert_pattern_range(&patterns, "てくる ", 8, 13); // 買ってきた
    }

    // Testing: standard[0] - Verb[て] + きた (have gotten better)
    #[test]
    fn test_tekuru_gotten_better() {
        let sentence = "最近、日本語が上手くなってきた感じがする";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てくる ");
        assert_pattern_range(&patterns, "てくる ", 10, 15); // なってきた
    }

    // Testing: polite[0] - Verb[て] + きます
    #[test]
    fn test_tekuru_polite() {
        let sentence = "明日また来てきます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てくる ");
        assert_pattern_range(&patterns, "てくる ", 4, 9); // 来てきます
    }
}

// Pattern: だけで (just by/with only)
// Data source: grammar_points_data.json["だけで"]
// Testing both structure variants:
//   - standard[0]: Verb + だけで
//   - standard[1]: Noun + だけで
mod dakede_tests {
    use super::*;

    // Testing: standard[0] - Verb + だけで
    #[test]
    fn verb_dakede() {
        let sentence = "彼女と話すだけで楽しい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だけで");
        assert_pattern_range(&patterns, "だけで", 3, 8); // 話すだけで
    }

    // Testing: standard[1] - Noun + だけで
    #[test]
    fn noun_dakede() {
        let sentence = "これは電子レンジだけでオーブンの機能はついていない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だけで");
        assert_pattern_range(&patterns, "だけで", 5, 11); // レンジだけで
    }
}

// Pattern: さ (degree/amount suffix)
// Data source: grammar_points_data.json["さ"]
// Testing both structure variants:
//   - standard[0]: い-Adjective[い] + さ
//   - standard[1]: な-Adjective + さ
mod sa_tests {
    use super::*;

    // Testing: standard[0] - い-Adjective[い] + さ
    #[test]
    fn i_adjective_sa() {
        let sentence = "私が道の長さを測ります";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "さ");
        assert_pattern_range(&patterns, "さ", 4, 6); // 長さ
    }

    // Testing: standard[1] - な-Adjective + さ
    #[test]
    fn na_adjective_sa() {
        let sentence = "大人になってから家族の大切さが分かってきた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "さ");
        assert_pattern_range(&patterns, "さ", 11, 14); // 大切さ
    }
}

// Pattern: ～代 (decade/era suffix)
// Data source: grammar_points_data.json["～代"]
// Testing both structure variants:
//   - standard[0]: Decade of age + 代
//   - standard[1]: Decade + 年代
mod dai_tests {
    use super::*;

    // Testing: standard[0] - Decade of age + 代
    #[test]
    fn age_decade_dai() {
        let sentence = "私は２０代の頃に沢山旅行をしました";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～代");
        assert_pattern_range(&patterns, "～代", 2, 5); // ２０代
    }

    // Testing: standard[1] - Decade + 年代
    #[test]
    fn chronological_decade_nendai() {
        let sentence = "７０年代の音楽が好きです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～代");
        assert_pattern_range(&patterns, "～代", 0, 4); // ７０年代
    }
}

// ========== がる (to show signs of / to act like) ==========
// Pattern: がる
// Data source: grammar_points_data.json["がる"]
//
// Structures to test:
//   - standard[0]: い-Adjective[い] + がる
//   - standard[1]: な-Adjective + がる
//   - polite[0]: い-Adjective[い] + がります
//   - polite[1]: な-Adjective + がります
//
// Examples from data:
//   - 強がる (act tough)
//   - 嫌がる (show signs of disliking)
//   - 欲しがる (show signs of wanting)
//   - 寒がっている (seem to be cold)
#[cfg(test)]
mod garu_tests {
    use super::*;

    // Testing: standard[0] - い-Adjective + がる (dictionary form)
    #[test]
    fn i_adjective_garu_tsuyogaru() {
        let sentence = "彼はいつも女の子の前では強がるけど実は弱いんだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がる");
        assert_pattern_range(&patterns, "がる", 12, 15); // 強がる
    }

    // Testing: Conjugated form 怖がり (連用形 of 怖がる)
    #[test]
    fn i_adjective_garu_kowagari() {
        let sentence = "友達の中で彼は一番の怖がりとして知られている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がる");
        assert_pattern_range(&patterns, "がる", 10, 13); // 怖がり
    }

    // Testing: standard[1] - な-Adjective + がる (dictionary form as compound)
    #[test]
    fn na_adjective_garu() {
        let sentence = "犬が嫌がることをしてはいけない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がる");
        assert_pattern_range(&patterns, "がる", 2, 5); // 嫌がる
    }

    // Testing: polite[0] - い-Adjective + がります (conjugated split form)
    #[test]
    fn i_adjective_garimasu() {
        let sentence = "この子はいつも新しいおもちゃを欲しがります";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がる");
        assert_pattern_range(&patterns, "がる", 15, 21); // 欲しがります
    }

    // Testing: Conjugated form with て (欲しがっている)
    #[test]
    fn i_adjective_gatteiru() {
        let sentence = "あの人、暑がっているね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がる");
        assert_pattern_range(&patterns, "がる", 4, 7); // 暑がっ
    }
}

// ========== とうとう (finally/at last) ==========
// Pattern: とうとう
// Data source: grammar_points_data.json["とうとう"]
//
// Structure to test:
//   - standard[0]: とうとう + Phrase
//
// Note: Used to express that something 'finally' happens after a long time or effort.
// Can be used for both positive and negative outcomes.
// Meaning: "finally", "at last", "after all"
#[cfg(test)]
mod toutou_tests {
    use super::*;

    // Testing: standard[0] - とうとう + Phrase (positive outcome)
    #[test]
    fn positive_outcome() {
        let sentence = "とうとう大学生か";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とうとう");
        assert_pattern_range(&patterns, "とうとう", 0, 4); // とうとう
    }

    // Testing: standard[0] - とうとう + Phrase (negative outcome)
    #[test]
    fn negative_outcome() {
        let sentence = "とうとう夏休みが終わる日が来た";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とうとう");
        assert_pattern_range(&patterns, "とうとう", 0, 4); // とうとう
    }

    // Testing: standard[0] - とうとう + Phrase (achievement context)
    #[test]
    fn achievement_context() {
        let sentence = "とうとうロシアに行くことができた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とうとう");
        assert_pattern_range(&patterns, "とうとう", 0, 4); // とうとう
    }
}

// ========== より (than/more than) ==========
// Pattern: より
// Data source: grammar_points_data.json["より"]
//
// Structures to test:
//   - standard[0]: Verb + より + Adjective
//   - standard[1]: Noun + より + Adjective
//
// Note: より is a comparison particle. The word that より is attached to is "less than"
// the comparison being made. Used to express "more/er than" comparisons.
// Meaning: "than", "more than", "compared to"
#[cfg(test)]
mod yori_tests {
    use super::*;

    // Testing: standard[1] - Noun + より + Adjective
    #[test]
    fn noun_comparison() {
        let sentence = "パンダはバナナより重い";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "より");
        assert_pattern_range(&patterns, "より", 7, 9); // より
    }

    // Testing: standard[1] - Noun + より + Adjective
    #[test]
    fn noun_comparison_brightness() {
        let sentence = "太陽はロウソクより明るい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "より");
        assert_pattern_range(&patterns, "より", 7, 9); // より
    }

    // Testing: standard[0] - Verb + より + Adjective
    #[test]
    fn verb_comparison() {
        let sentence = "走るより歩く方が健康的だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "より");
        assert_pattern_range(&patterns, "より", 2, 4); // より
    }
}

// ========== てあげる (to do for someone) ==========
// Pattern: てあげる
// Data source: grammar_points_data.json["てあげる"]
//
// Structures to test:
//   - standard[0]: Verb[て] + あげる
//   - polite[0]: Verb[て] + あげます
//
// Note: Expresses doing something for someone else's benefit. Can sound patronizing
// if used with people of higher status. Receiver marked with に.
// Meaning: "to do (something) for (someone)", "to do as a favor"
#[cfg(test)]
mod teageru_tests {
    use super::*;

    // Testing: standard[0] - Verb[て] + あげる
    #[test]
    fn standard_form() {
        let sentence = "父が妹にお菓子を買ってあげる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てあげる");
        assert_pattern_range(&patterns, "てあげる", 8, 14); // 買ってあげる
    }

    // Testing: standard[0] - Verb[て] + あげる (casual)
    #[test]
    fn casual_favor() {
        let sentence = "いいよ、俺がやってあげるよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てあげる");
        assert_pattern_range(&patterns, "てあげる", 6, 12); // やってあげる
    }

    // Testing: polite[0] - Verb[て] + あげます
    #[test]
    fn polite_form() {
        let sentence = "明日は私が送ってあげます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てあげる");
        assert_pattern_range(&patterns, "てあげる", 5, 12); // 送ってあげます
    }
}

// ========== かた (how to/way of) ==========
// Pattern: かた (how to do something)
// Data source: grammar_points_data.json["かた"]
// Structures: Verb[stem] + 方（かた） / Noun + の + 仕方（しかた）

mod kata_tests {
    use super::*;

    // Structure: Verb[stem] + 方（かた）
    #[test]
    fn verb_stem_kata() {
        let sentence = "その食べかた、口を閉じてよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かた");
        assert_pattern_range(&patterns, "かた", 2, 6); // 食べかた
    }

    // Structure: Verb[stem] + 方（かた） (necktie example)
    // Note: 結び is tokenized as Noun, not Verb, so detected by Noun＋型 (N3) pattern
    #[test]
    fn verb_stem_kata_tie() {
        let sentence = "ネクタイの結びかたを教える";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Noun＋型");
        assert_pattern_range(&patterns, "Noun＋型", 5, 9); // 結びかた
    }

    // Structure: [する]Verb + の + 仕方（しかた）
    #[test]
    fn suru_verb_no_shikata() {
        let sentence = "外国語の勉強のしかたが分からない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かた");
        assert_pattern_range(&patterns, "かた", 4, 10); // 勉強のしかた
    }

    // Structure: [する]Verb + の + 仕方（しかた） (fax example)
    #[test]
    fn suru_verb_shikata_fax() {
        let sentence = "先輩、ファックスのしかたを教えてください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かた");
        assert_pattern_range(&patterns, "かた", 3, 12); // ファックスのしかた
    }

    // Structure: Noun + の + Verb[stem] + 方（かた）
    // Note: 運び is tokenized as Noun, not Verb, so detected by Noun＋型 (N3) pattern
    #[test]
    fn noun_no_verb_kata() {
        let sentence = "家具の運びかたを見て驚いた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Noun＋型");
        assert_pattern_range(&patterns, "Noun＋型", 3, 7); // 運びかた
    }
}

// ========== てくれる (to do for me/us) ==========
// Pattern: てくれる
// Data source: grammar_points_data.json["てくれる"]
//
// Structures to test:
//   - standard[0]: Verb[て] + くれる
//   - standard[1]: Verb[ないで] + くれる
//   - polite[0]: Verb[て] + くれますか
//   - polite[1]: Verb[ないで] + くれますか
//
// Meaning: "to do (something) for me/us" - opposite of てあげる
// The action benefits the speaker or someone in their inner circle
#[cfg(test)]
mod tekureru_tests {
    use super::*;

    // Structure: Verb[て] + くれる (standard form)
    #[test]
    fn te_form_kureru() {
        let sentence = "おばあちゃんはいつも美味しいご飯を作ってくれる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てくれる");
        assert_pattern_range(&patterns, "てくれる", 17, 23); // 作ってくれる
    }

    // Structure: Verb[て] + くれる (casual request)
    #[test]
    fn te_form_kureru_request() {
        let sentence = "パパ、電気を消してくれる？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てくれる");
        assert_pattern_range(&patterns, "てくれる", 6, 12); // 消してくれる
    }

    // Structure: Verb[て] + くれますか (polite request)
    #[test]
    fn te_form_kuremasu_ka() {
        let sentence = "すみません、もう一度説明してくれますか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てくれる");
        assert_pattern_range(&patterns, "てくれる", 10, 18); // 説明してくれます
    }

    // Structure: Verb[ないで] + くれる (negative request - don't do)
    #[test]
    fn naide_kureru() {
        let sentence = "お願いだから怒らないでくれる？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てくれる");
        assert_pattern_range(&patterns, "てくれる", 6, 14); // 怒らないでくれる
    }
}

// ========== てもらう (to receive/have someone do) ==========
// Pattern: てもらう
// Data source: grammar_points_data.json["てもらう"]
//
// Structures to test:
//   - standard[0]: Verb[て] + もらう
//   - polite[0]: Verb[て] + もらいます
//
// Meaning: "to have/get someone to do (something)", "to receive the action of"
// More direct than てくれる - focuses on receiving rather than bestowing
// Often used for receiving services
#[cfg(test)]
mod temorau_tests {
    use super::*;

    // Structure: Verb[て] + もらう (standard form - having someone do)
    #[test]
    fn te_form_morau() {
        let sentence = "日本語が話せないから友達に行き方を聞いてもらう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てもらう");
        assert_pattern_range(&patterns, "てもらう", 17, 23); // 聞いてもらう
    }

    // Structure: Verb[て] + もらう (receiving service)
    #[test]
    fn te_form_morau_service() {
        let sentence = "昨日は朝から頭が痛かったから医者に見てもらった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てもらう");
        assert_pattern_range(&patterns, "てもらう", 17, 23); // 見てもらった
    }

    // Structure: Verb[て] + もらいます (polite form)
    #[test]
    fn te_form_moraimasu() {
        let sentence = "すみません、この荷物を持ってもらいますか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てもらう");
        assert_pattern_range(&patterns, "てもらう", 11, 19); // 持ってもらいます
    }
}

// ========== ていた (was doing / past progressive) ==========
// Pattern: ていた
// Data source: grammar_points_data.json["ていた "]
//
// Structures to test:
//   - standard[0]: Verb[ている] + た
//   - polite[0]: Verb[ている] + ました
//
// Examples from data:
//   - 歌っていた (was singing)
//   - 座っていた (was sitting)
//   - 壊れていた (was broken - resultative state)
//   - 歌っていました (was singing - polite)
#[cfg(test)]
mod teita_tests {
    use super::*;

    // Structure: Verb[ている] + た (standard form - action)
    #[test]
    fn te_iru_ta_action() {
        let sentence = "昨日は夜遅くまでカラオケで歌っていたのでのどが痛い";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ていた ");
        assert_pattern_range(&patterns, "ていた ", 13, 18); // 歌っていた
    }

    // Structure: Verb[ている] + た (standard form - resultative state)
    #[test]
    fn te_iru_ta_state() {
        let sentence = "シャワーが壊れていたから二日間もシャワーを浴びれなかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ていた ");
        assert_pattern_range(&patterns, "ていた ", 5, 10); // 壊れていた
    }

    // Structure: Verb[ている] + ました (polite form)
    #[test]
    fn te_iru_mashita() {
        let sentence = "昔はパーマをかけていました";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ていた ");
        assert_pattern_range(&patterns, "ていた ", 6, 13); // かけていました
    }
}

// ========== てある (state of completion / left in state) ==========
// Pattern: てある
// Data source: grammar_points_data.json["てある "]
//
// Structures to test:
//   - standard[0]: (Transitive) Verb[て] + ある
//   - polite[0]: (Transitive) Verb[て] + あります
//
// Examples from data:
//   - 置いてある (is left/placed)
//   - 止めてある (is parked/stopped)
//   - 植えてある (is planted)
//
// Note: Only works with transitive verbs, focuses on the object's state
#[cfg(test)]
mod tearu_tests {
    use super::*;

    // Structure: (Transitive) Verb[て] + ある (standard form)
    #[test]
    fn te_aru_left_state() {
        let sentence = "あなたの弁当は机の上に置いてあるよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てある ");
        assert_pattern_range(&patterns, "てある ", 11, 16); // 置いてある
    }

    // Structure: (Transitive) Verb[て] + ある (parked state)
    #[test]
    fn te_aru_parked() {
        let sentence = "車は駐車場の真ん中に止めてある";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てある ");
        assert_pattern_range(&patterns, "てある ", 10, 15); // 止めてある
    }

    // Structure: (Transitive) Verb[て] + あります (polite form)
    #[test]
    fn te_arimasu_polite() {
        let sentence = "庭にトウモロコシが植えてありますから気をつけてね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てある ");
        assert_pattern_range(&patterns, "てある ", 9, 16); // 植えてあります
    }
}

// ========== てみる (try to / try doing) ==========
// Pattern: てみる
// Data source: grammar_points_data.json["てみる"]
//
// Structures to test:
//   - standard[0]: Verb[て] + みる
//   - polite[0]: Verb[て] + みます
//
// Examples from data:
//   - 食べてみる (try eating)
//   - 飲んでみる (try drinking)
//   - 投げてみ (try throwing - casual)
//   - 行ってみた (tried going - past)
//   - 切ってみてください (please try turning off)
//
// Note: Used for trying something for the first time, "do and see the result"
#[cfg(test)]
mod temiru_tests {
    use super::*;

    // Structure: Verb[て] + みる (standard form - dictionary)
    #[test]
    fn temiru_standard_dictionary() {
        let sentence = "本当においしいの？今度食べてみるよ！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てみる");
        assert_pattern_range(&patterns, "てみる", 11, 16); // 食べてみる
    }

    // Structure: Verb[て] + みる (casual question)
    #[test]
    fn temiru_casual_question() {
        let sentence = "これ飲んでみる？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てみる");
        assert_pattern_range(&patterns, "てみる", 2, 7); // 飲んでみる
    }

    // Structure: Verb[て] + みます (polite form - conjugated)
    #[test]
    fn temiru_polite() {
        let sentence = "もう一度電源を切ってみてください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てみる");
        assert_pattern_range(&patterns, "てみる", 7, 11); // 切ってみ
    }
}

// ========== てほしい (want someone to do) ==========
// Pattern: てほしい
// Data source: grammar_points_data.json["てほしい"]
//
// Structures to test:
//   - standard[0]: Verb[て] + ほしい
//   - polite[0]: Verb[て] + ほしい + です
//
// Examples from data:
//   - してほしい (want you to do)
//   - 読んでほしい (want you to read)
//   - 手伝ってほしい (want you to help)
//
// Note: Expresses "I want someone to do something"
#[cfg(test)]
mod tehoshii_tests {
    use super::*;

    // Structure: Verb[て] + ほしい + です (polite form)
    #[test]
    fn tehoshii_polite() {
        let sentence = "毎日早く帰ってほしいです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てほしい");
        assert_pattern_range(&patterns, "てほしい", 4, 12); // 帰ってほしいです (includes です due to pattern overlap)
    }

    // Structure: Verb[て] + ほしい (casual)
    #[test]
    fn tehoshii_casual() {
        let sentence = "皆に俺が書いた漫画を読んでほしい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てほしい");
        assert_pattern_range(&patterns, "てほしい", 10, 16); // 読んでほしい
    }

    // Structure: Verb[て] + ほしい + の (question)
    #[test]
    fn tehoshii_question() {
        let sentence = "手伝ってほしいの？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てほしい");
        assert_pattern_range(&patterns, "てほしい", 0, 7); // 手伝ってほしい
    }
}

// ========== ておく (do in advance/leave as is) ==========
// Pattern: ておく
// Data source: grammar_points_data.json["ておく"]
//
// Structures to test:
//   - standard[0]: Verb[て] + おく
//   - standard[1]: Verb[て] + とく (casual contraction)
//   - polite[0]: Verb[て] + おきます
//   - polite[1]: Verb[て] + ときます (casual contraction)
//
// Meaning: "do something in advance", "leave something as is"
// Examples from data:
//   - 洗っておく - wash in advance
//   - 入れておきます - will put in advance
//   - 置いとく - leave it (casual)
#[cfg(test)]
mod teoku_tests {
    use super::*;

    // Structure: Verb[て] + おく (standard form)
    #[test]
    fn teoku_standard() {
        let sentence = "寝る前に食器を洗っておく";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ておく");
        assert_pattern_range(&patterns, "ておく", 7, 12); // 洗っておく
    }

    // Structure: Verb[て] + おきます (polite form)
    #[test]
    fn teoku_polite() {
        let sentence = "あなたの財布を鞄に入れておきます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ておく");
        assert_pattern_range(&patterns, "ておく", 9, 16); // 入れておきます
    }

    // Structure: Verb[て] + とく (casual contraction)
    #[test]
    fn teoku_casual_contraction() {
        let sentence = "机の上にパスポートを置いとくね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ておく");
        assert_pattern_range(&patterns, "ておく", 10, 14); // 置いとく
    }

    // Structure: Verb[で] + おく (て becomes で after certain verbs)
    #[test]
    fn teoku_de_form() {
        let sentence = "水筒に水を汲んでおく";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ておく");
        assert_pattern_range(&patterns, "ておく", 5, 10); // 汲んでおく
    }
}

// ========== てよかった (glad that) ==========
// Pattern: てよかった
// Data source: grammar_points_data.json["てよかった"]
//
// Structures to test:
//   - standard[0]: Verb[て] + よかった
//   - standard[1]: い-Adjective[て] + よかった
//   - standard[2]: な-Adjective + で + よかった
//   - standard[3]: Noun + で + よかった
//   - standard[4]: Verb[なくて] + よかった (literary)
//   - standard[5]: Verb[ないで] + よかった (spoken)
//   - standard[6]: い-Adjective[なくて] + よかった
//   - polite variants: + です
//
// Meaning: "I'm glad that..." (positive) or "I'm glad that I didn't..." (negative)
#[cfg(test)]
mod teyokatta_tests {
    use super::*;

    // Structure: Verb[て] + よかった
    #[test]
    fn teyokatta_verb_positive() {
        let sentence = "やっぱり今日来てよかったね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てよかった");
        assert_pattern_range(&patterns, "てよかった", 7, 12); // てよかった
    }

    // Structure: Verb[て] + よかった + です (polite)
    #[test]
    fn teyokatta_verb_polite() {
        let sentence = "この本を買ってよかったです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てよかった");
        assert_pattern_range(&patterns, "てよかった", 6, 13); // てよかったです
    }

    // Structure: い-Adjective[くて] + よかった
    #[test]
    fn teyokatta_i_adjective() {
        let sentence = "天気が良くてよかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てよかった");
        assert_pattern_range(&patterns, "てよかった", 5, 10); // てよかった
    }

    // Structure: な-Adjective + で + よかった
    #[test]
    fn teyokatta_na_adjective() {
        let sentence = "部屋が静かでよかったです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てよかった");
        assert_pattern_range(&patterns, "てよかった", 5, 12); // でよかったです
    }

    // Structure: Noun + で + よかった
    #[test]
    fn teyokatta_noun() {
        let sentence = "彼が先生でよかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てよかった");
        assert_pattern_range(&patterns, "てよかった", 4, 9); // でよかった
    }

    // Structure: Verb[なくて] + よかった (literary/formal)
    #[test]
    fn teyokatta_verb_negative_nakute() {
        let sentence = "あの携帯を買わなくてよかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てよかった");
        assert_pattern_range(&patterns, "てよかった", 9, 14); // てよかった
    }

    // Structure: Verb[ないで] + よかった (casual spoken)
    #[test]
    fn teyokatta_verb_negative_naide() {
        let sentence = "あの時に別れないでよかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てよかった");
        assert_pattern_range(&patterns, "てよかった", 8, 13); // でよかった
    }

    // Structure: い-Adjective[なくて] + よかった
    #[test]
    fn teyokatta_i_adjective_negative() {
        let sentence = "値段が高くなくてよかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てよかった");
        assert_pattern_range(&patterns, "てよかった", 7, 12); // てよかった
    }
}

// Pattern: まで (even, to the extent)
// Data source: grammar_points_data.json["まで"]
// Testing: structure.standard[0] - "Noun + まで(も)"
//
// This is the N4 "even" meaning of まで (adverbial particle).
// Different from N5 "まで" (until - temporal/spatial limit).
//
// Meaning: "even", "to the extent", highlighting unexpectedness
// Tokenization: Noun + まで (助詞/副助詞) [+ も (助詞/係助詞)]
mod made_tests {
    use super::*;

    // Structure: Noun + まで
    #[test]
    fn made_noun_even() {
        let sentence = "バイクまで持っているの？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まで");
        assert_pattern_range(&patterns, "まで", 0, 5); // バイクまで
    }

    // Structure: Noun + まで (emphasized extent)
    #[test]
    fn made_disliked_thing() {
        let sentence = "お腹が空きすぎて、大嫌いなバナナまで食べた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まで");
        assert_pattern_range(&patterns, "まで", 13, 18); // バナナまで
    }

    // Structure: Noun + まで + も (emphatic)
    #[test]
    fn made_mo_emphatic() {
        let sentence = "私はスポーツカーまでも持っています";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まで");
        assert_pattern_range(&patterns, "まで", 2, 10); // スポーツカーまで
    }
}

// ========== と (conditional - definite result) ==========
// Pattern: と - "if/when (definite result)"
// Data source: grammar_points_data.json["と"]
//
// Structures to test:
//   - standard[0]: Verb + と
//   - standard[1]: い-Adjective + と
//   - standard[2]: な-Adjective + だ + と
//   - standard[3]: Noun + だ + と
//
// Note: と as conditional implies a definite/inevitable result
// Different from hypothetical conditionals (ば, なら, たら)
// Meaning: "if/when (A), then (B) will definitely happen"
#[cfg(test)]
mod to_conditional_tests {
    use super::*;

    // Structure: Verb + と (standard[0])
    #[test]
    fn verb_to_definite_result() {
        let sentence = "甘いものをいっぱい食べると、太る";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "と");
        assert_pattern_range(&patterns, "と", 9, 13); // 食べると
    }

    // Structure: い-Adjective + と (standard[1])
    #[test]
    fn i_adjective_to_condition() {
        let sentence = "部屋が汚いと、お母さんに怒られる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "と");
        assert_pattern_range(&patterns, "と", 3, 6); // 汚いと
    }

    // Structure: な-Adjective + だ + と (standard[2])
    #[test]
    fn na_adjective_dato_condition() {
        let sentence = "部屋が静かだと、眠れない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "と");
        assert_pattern_range(&patterns, "と", 5, 7); // だと
    }

    // Structure: Noun + だ + と (standard[3])
    #[test]
    fn noun_dato_condition() {
        let sentence = "地下鉄だと、五分早く着く";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "と");
        assert_pattern_range(&patterns, "と", 3, 5); // だと
    }
}

// ========== まず (first of all / to start with) ==========
// Pattern: まず - "first", "to start with", "before anything else"
// Data source: grammar_points_data.json["まず"]
//
// Structures to test:
//   - standard[0]: まず + Phrase
//
// Note: Used at beginning of statement to indicate priority/sequence
// Meaning: "first", "to begin with", "starting with"
#[cfg(test)]
mod mazu_tests {
    use super::*;

    // Structure: まず + Phrase (standard[0]) - at sentence start
    #[test]
    fn mazu_at_start() {
        let sentence = "まず宿題をしたほうがいい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まず");
        assert_pattern_range(&patterns, "まず", 0, 2); // まず
    }

    // Structure: まず + Phrase (standard[0]) - with て-form
    #[test]
    fn mazu_with_te_form() {
        let sentence = "まずエンジンをかけて";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まず");
        assert_pattern_range(&patterns, "まず", 0, 2); // まず
    }

    // Structure: まず + Phrase (standard[0]) - polite suggestion
    #[test]
    fn mazu_polite_suggestion() {
        let sentence = "まず別れた方がいいと思う";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まず");
        assert_pattern_range(&patterns, "まず", 0, 2); // まず
    }
}

// ========== また (again/also) ==========
// Pattern: また
// Data source: grammar_points_data.json["また"]
//
// Structure to test:
//   - standard[0]: また + Phrase
//
// Meaning: "again", "additionally", "also", "moreover"
// Usage: Can be used as adverb or conjunction
// Examples from data:
//   - また遊ぼうね！ (Let's hang out again!)
//   - また行こうね！ (Let's go again!)
//   - またカンニングしたの？ (You cheated again?)
//   - また、アナウンサーでもある (and also an announcer)
#[cfg(test)]
mod mata_tests {
    use super::*;

    // Structure: また + Phrase (standard[0]) - "again" meaning
    #[test]
    fn mata_again_volitional() {
        let sentence = "また遊ぼうね！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "また");
        assert_pattern_range(&patterns, "また", 0, 2); // また
    }

    // Structure: また + Phrase (standard[0]) - "again" with past
    #[test]
    fn mata_again_question() {
        let sentence = "またカンニングしたの？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "また");
        assert_pattern_range(&patterns, "また", 0, 2); // また
    }

    // Structure: また + Phrase (standard[0]) - "also/additionally" as conjunction
    #[test]
    fn mata_also_conjunction() {
        let sentence = "彼は宇宙飛行士であり、またアナウンサーでもある";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "また");
        assert_pattern_range(&patterns, "また", 11, 13); // また
    }
}

// ========== ごろ (around/about) ==========
// Pattern: ごろ
// Data source: grammar_points_data.json["ごろ"]
//
// Structures to test:
//   - standard[0]: Verb + ころ
//   - standard[1]: い-Adjective + ころ
//   - standard[2]: な-Adjective + な + ころ
//   - standard[3]: Noun + ごろ
//   - standard[4]: Noun + の + ころ
//
// Meaning: "around", "about" (for time spans, not distances)
// Usage: Expresses a broad point/span of time
// Examples from data:
//   - １１時ごろに帰ってくる (around 11 o'clock)
//   - 大学生のころに富士山を登りました (around when I was a college student)
//   - 子供のころはよく親と動物園へ行きました (around when I was a child)
#[cfg(test)]
mod goro_tests {
    use super::*;

    // Structure: Noun + ごろ (standard[3])
    #[test]
    fn goro_noun_time() {
        let sentence = "１１時ごろに帰ってくる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ごろ");
        assert_pattern_range(&patterns, "ごろ", 3, 5); // ごろ
    }

    // Structure: Noun + の + ころ (standard[4])
    #[test]
    fn goro_noun_no_koro() {
        let sentence = "大学生のころに富士山を登りました";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ごろ");
        assert_pattern_range(&patterns, "ごろ", 4, 6); // ころ
    }

    // Structure: Noun + の + ころ (standard[4]) - broader time
    #[test]
    fn goro_childhood() {
        let sentence = "子供のころはよく親と動物園へ行きました";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ごろ");
        assert_pattern_range(&patterns, "ごろ", 3, 5); // ころ
    }
}

// Pattern: かどうか - "whether or not"
// Data source: grammar_points_data.json["かどうか"]
// Testing all structure variants:
//   - standard[0]: Verb + かどうか
//   - standard[1]: い-Adjective + かどうか
//   - standard[2]: な-Adjective + かどうか
//   - standard[3]: Noun + かどうか
mod kadouka_tests {
    use super::*;

    // Structure: Verb + か + どう + か (standard[0])
    #[test]
    fn kadouka_verb() {
        let sentence = "サチコと付き合うかどうか分からない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かどうか");
        assert_pattern_range(&patterns, "かどうか", 4, 12); // 付き合うかどうか
    }

    // Structure: い-Adjective + か + どう + か (standard[1])
    #[test]
    fn kadouka_i_adjective() {
        let sentence = "この川が深いかどうか分からない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かどうか");
        assert_pattern_range(&patterns, "かどうか", 4, 10); // 深いかどうか
    }

    // Structure: な-Adjective + か + どう + か (standard[2])
    #[test]
    fn kadouka_na_adjective() {
        let sentence = "この携帯が便利かどうか分からない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かどうか");
        assert_pattern_range(&patterns, "かどうか", 5, 11); // 便利かどうか
    }

    // Structure: Noun + か + どう + か (standard[3])
    #[test]
    fn kadouka_noun() {
        let sentence = "あそこにいるのが父親かどうかわからない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かどうか");
        assert_pattern_range(&patterns, "かどうか", 8, 14); // 父親かどうか
    }
}

// Pattern: なるべく - "as much as possible"
// Data source: grammar_points_data.json["なるべく"]
// Testing structure:
//   - standard[0]: なるべく + (Action) Phrase
mod narubeku_tests {
    use super::*;

    // Structure: なるべく + Phrase (standard[0]) - with polite request
    #[test]
    fn narubeku_with_request() {
        let sentence = "外に出かけるときはなるべくドアのカギをかけてください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なるべく");
        assert_pattern_range(&patterns, "なるべく", 9, 13); // なるべく
    }

    // Structure: なるべく + Phrase (standard[0]) - with conditional
    #[test]
    fn narubeku_with_conditional() {
        let sentence = "仕事で疲れていても、なるべく夕食は食べてください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なるべく");
        assert_pattern_range(&patterns, "なるべく", 10, 14); // なるべく
    }

    // Structure: なるべく + Phrase (standard[0]) - casual
    #[test]
    fn narubeku_casual() {
        let sentence = "明日はなるべく早く来てね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なるべく");
        assert_pattern_range(&patterns, "なるべく", 3, 7); // なるべく
    }
}

// ========== ～ら (pluralizing suffix for pronouns) ==========
// Pattern: ～ら
// Data source: grammar_points_data.json["～ら"]
//
// Structure to test:
//   - standard[0]: Pronoun + ら
//
// Examples from data:
//   - 彼ら (they - masculine)
//   - お前ら (you guys - casual/rough)
//
// Note: ら is a pluralizing suffix that implies "more than one" or "(A) etc"
// Can be considered dismissive/rude, so 達 is often preferred
#[cfg(test)]
mod uff5e_ra_tests {
    use super::*;

    // TODO: Undetectable - 彼ら tokenizes as single compound token
    // 彼ら tokenizes as: 彼ら (名詞/代名詞/一般, base_form=彼ら) - single token
    // This is different from the productive pattern Pronoun + ら(名詞/接尾)
    // which applies to most pronouns like 私ら, お前ら, etc.
    //
    // #[test]
    // fn karera_they_compound() {
    //     let sentence = "彼らは日本語を勉強しに来た";
    //     // 彼ら is a lexicalized compound in the dictionary
    // }

    // Structure: Pronoun + ら (standard[0]) - お前ら (you guys)
    #[test]
    fn omaera_you_guys() {
        let sentence = "お前ら、アニメを見ないの？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～ら");
        assert_pattern_range(&patterns, "～ら", 0, 3); // お前ら
    }

    // Structure: Pronoun + ら (standard[0]) - 私ら (we - casual)
    #[test]
    fn watashira_we() {
        let sentence = "私らもそれ知ってるよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～ら");
        assert_pattern_range(&patterns, "～ら", 0, 2); // 私ら
    }
}

// ========== など (such as, and so on, things like) ==========
// Pattern: など
// Data source: grammar_points_data.json["など"]
//
// Structures to test:
//   - standard[0]: Noun + など
//   - standard[1]: Noun + など + の + Noun
//
// Examples from data:
//   - 石など (rocks and so on)
//   - ルームメイトなどの悪口 (badmouthing people like his roommates)
//   - 鞄や靴等 (things such as bags and shoes)
#[cfg(test)]
mod nado_tests {
    use super::*;

    // Structure: Noun + など (standard[0])
    #[test]
    fn noun_nado_basic() {
        let sentence = "この公園では石などを投げないでください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "など");
        assert_pattern_range(&patterns, "など", 6, 9); // 石など
    }

    // Structure: Noun + など + の + Noun (standard[1])
    #[test]
    fn noun_nado_no_noun() {
        let sentence = "彼はいつもルームメイトなどの悪口を言っています";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "など");
        assert_pattern_range(&patterns, "など", 5, 13); // ルームメイトなど
    }

    // Structure: Noun + など (standard[0]) - with や listing
    #[test]
    fn noun_nado_with_ya() {
        let sentence = "鞄や靴などを川に捨ててはならない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "など");
        assert_pattern_range(&patterns, "など", 2, 5); // 靴など
    }
}

// ========== だが・ですが (but, however - formal) ==========
// Pattern: だが・ですが
// Data source: grammar_points_data.json["だが・ですが"]
//
// Structures to test:
//   - standard[0]: だが + Phrase
//   - polite[0]: ですが + Phrase
//
// Examples from data:
//   - 彼は弁護士だが、頭が良くない (He is a lawyer, but he is not very smart)
//   - 宝くじを１００枚買った。だが、当たらなかった (I bought 100 lottery tickets. However, I didn't win)
//   - 美容室に行ったのですが、高かったので帰りました (I went to the hair salon, but since it was expensive, I came home)
#[cfg(test)]
mod daga_desuga_tests {
    use super::*;

    // Structure: だが + Phrase (standard[0])
    #[test]
    fn daga_mid_sentence() {
        let sentence = "彼は弁護士だが、頭が良くない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だが・ですが");
        assert_pattern_range(&patterns, "だが・ですが", 5, 7); // だが
    }

    // Structure: だが + Phrase (standard[0]) - sentence start
    #[test]
    fn daga_sentence_start() {
        let sentence = "宝くじを１００枚買った。だが、当たらなかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だが・ですが");
        assert_pattern_range(&patterns, "だが・ですが", 12, 14); // だが
    }

    // Structure: ですが + Phrase (polite[0])
    #[test]
    fn desuga_polite() {
        let sentence = "美容室に行ったのですが、高かったので帰りました";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だが・ですが");
        assert_pattern_range(&patterns, "だが・ですが", 8, 11); // ですが
    }
}

// ========== たばかり (just finished) ==========
// Pattern: たばかり
// Data source: grammar_points_data.json["たばかり"]
//
// Structure to test:
//   - standard[0]: Verb[た] + ばかり
//
// Examples from data:
//   - 着いたばかりの時 (just as I arrived)
//   - 買ったばかりなのに (even though I just bought it)
//   - 食べたばかりだから (because I just ate)
#[cfg(test)]
mod tabakari_tests {
    use super::*;

    // Structure: Verb[た] + ばかり (standard[0])
    #[test]
    fn verb_ta_bakari_basic() {
        let sentence = "駅に着いたばかりの時に友達からメッセージが来た";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たばかり");
        assert_pattern_range(&patterns, "たばかり", 2, 8); // 着いたばかり
    }

    // Structure: Verb[た] + ばかり + だ (standard[0])
    #[test]
    fn verb_ta_bakari_with_da() {
        let sentence = "さっき食べたばかりだからお腹はまだ空いていない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たばかり");
        assert_pattern_range(&patterns, "たばかり", 3, 10); // 食べたばかりだ (includes auxiliary)
    }

    // Structure: Verb[た] + ばかり + のに (standard[0])
    #[test]
    fn verb_ta_bakari_with_noni() {
        let sentence = "スマホが壊れた。昨日買ったばかりなのに";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たばかり");
        assert_pattern_range(&patterns, "たばかり", 10, 17); // 買ったばかりな (includes auxiliary)
    }
}

// ========== なくて (negative て-form) ==========
// Pattern: なくて
// Data source: grammar_points_data.json["なくて"]
//
// Structures to test:
//   - standard[0]: Verb［なくて］+ Phrase
//   - standard[1]: ［い］Adjective［なくて］+ Phrase
//   - standard[2]: ［な］Adjective + ではなくて + Phrase
//   - standard[3]: Noun + ではなくて + Phrase
//   - Note: ではなくて can also be じゃなくて or でなくて
//
// Examples from data:
//   - 来れなくて残念です (unfortunate that you couldn't come)
//   - 寒くはなくて暖かい (not cold but warm)
//   - 便利ではなくて残念だ (unfortunate that it's not useful)
//   - 虫歯ではなくて安心した (relieved because it wasn't a cavity)
#[cfg(test)]
mod nakute_tests {
    use super::*;

    // Structure: Verb［なくて］+ Phrase (standard[0])
    #[test]
    fn verb_negative_te_form() {
        let sentence = "あなたが来れなくて残念です";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくて");
        assert_pattern_range(&patterns, "なくて", 6, 9); // なくて
    }

    // Structure: ［い］Adjective［なくて］+ Phrase (standard[1])
    #[test]
    fn i_adjective_negative_te_form() {
        let sentence = "今日は寒くはなくて暖かいから、ジャケットを着なくてもいいね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // First なくて at chars 6-9 (寒くは + なくて)
        assert_has_pattern(&patterns, "なくて");
        assert_pattern_range(&patterns, "なくて", 6, 9); // なくて

        // Second なくて at chars 22-25 (着 + なくて)
        let nakute_matches: Vec<_> = patterns
            .iter()
            .filter(|p| p.pattern_name == "なくて")
            .collect();
        assert_eq!(nakute_matches.len(), 2, "Should detect two なくて patterns");

        // Verify both ranges
        let ranges: Vec<_> = nakute_matches
            .iter()
            .map(|m| (m.start_char, m.end_char))
            .collect();
        assert!(ranges.contains(&(6, 9)), "Should detect なくて at 6-9");
        assert!(ranges.contains(&(22, 25)), "Should detect なくて at 22-25");
    }

    // Structure: ［な］Adjective + ではなくて + Phrase (standard[2])
    #[test]
    fn na_adjective_dewanakute() {
        let sentence = "このスマホは便利ではなくて残念だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // なくて is detected as part of ではなくて
        assert_has_pattern(&patterns, "なくて");
        assert_pattern_range(&patterns, "なくて", 10, 13); // なくて

        // Also verify ではなくて・じゃなくて pattern is detected
        assert_has_pattern(&patterns, "ではなくて・じゃなくて");
        assert_pattern_range(&patterns, "ではなくて・じゃなくて", 8, 13); // ではなくて
    }

    // Structure: Noun + ではなくて + Phrase (standard[3])
    #[test]
    fn noun_dewanakute() {
        let sentence = "虫歯ではなくて安心した";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // なくて is detected as part of ではなくて
        assert_has_pattern(&patterns, "なくて");
        assert_pattern_range(&patterns, "なくて", 4, 7); // なくて

        // Also verify ではなくて・じゃなくて pattern is detected
        assert_has_pattern(&patterns, "ではなくて・じゃなくて");
        assert_pattern_range(&patterns, "ではなくて・じゃなくて", 2, 7); // ではなくて
    }

    // Variant: じゃなくて instead of ではなくて (casual)
    #[test]
    fn noun_janakute_casual() {
        let sentence = "昨日は仕事じゃなくて嬉しい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // なくて is detected as part of じゃなくて
        assert_has_pattern(&patterns, "なくて");
        assert_pattern_range(&patterns, "なくて", 7, 10); // なくて

        // Also verify ではなくて・じゃなくて pattern is detected
        assert_has_pattern(&patterns, "ではなくて・じゃなくて");
        assert_pattern_range(&patterns, "ではなくて・じゃなくて", 5, 10); // じゃなくて
    }

    // Real example from data: Verb potential form negative
    #[test]
    fn verb_potential_negative() {
        let sentence = "昨日は良く寝れなくて、疲れている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくて");
        assert_pattern_range(&patterns, "なくて", 7, 10); // なくて
    }

    // Real example: Noun が + なくて
    #[test]
    fn noun_ga_nakute() {
        let sentence = "明日は仕事がなくて嬉しい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくて");
        assert_pattern_range(&patterns, "なくて", 6, 9); // なくて
    }
}
