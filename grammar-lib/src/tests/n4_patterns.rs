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
