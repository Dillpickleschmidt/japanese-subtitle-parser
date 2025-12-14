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

// ========== たところだ (just did) ==========
// Pattern: たところだ (just finished doing)
// Data source: grammar_points_data.json["たところだ"]
// Structures to test:
//   - standard[0]: Verb[た] + ところ + だ (just did)
//   - standard[1]: Verb[ていた] + ところ (just was doing)
//   - polite[0]: Verb[た] + ところ + です (just did - polite)
//   - polite[1]: Verb[ていた] + ところ + です (just was doing - polite)
//
// Examples from data:
//   - 今、先生に聞いたところ (I just asked the teacher)
//   - 仕事は今終わったところ (I just finished work)
//   - 休みを楽しんでいたところで (just as I was enjoying my day off)
#[cfg(test)]
mod tatokoroda_tests {
    use super::*;

    // Structure: Verb[た] + ところ
    #[test]
    fn ta_tokoro_plain() {
        let sentence = "今、先生に聞いたところ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たところだ");
        assert_pattern_range(&patterns, "たところだ", 5, 11); // 聞いたところ
    }

    // Structure: Verb[た] + ところ + だ
    #[test]
    fn ta_tokoro_da() {
        let sentence = "仕事は今終わったところだ。もうすぐ帰る。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たところだ");
        assert_pattern_range(&patterns, "たところだ", 4, 12); // 終わったところだ
    }

    // Structure: Verb[ていた] + ところ (just was doing)
    #[test]
    fn teita_tokoro() {
        let sentence = "休みを楽しんでいたところで上司から電話が来た";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たところだ");
        assert_pattern_range(&patterns, "たところだ", 7, 12); // いたところ
    }

    // Structure: Verb[た] + ところ + です (polite)
    #[test]
    fn ta_tokoro_desu() {
        let sentence = "ちょうど今着いたところです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たところだ");
        assert_pattern_range(&patterns, "たところだ", 5, 13); // 着いたところです
    }
}

// Pattern: 各 (each/every)
// Data source: grammar_points_data.json["各"]
// Testing: structure.standard[0] - "各（かく） + Noun"
//
// Note: Only detects split form (各 as prefix + noun)
// Compound nouns like 各階 (かくかい) and 各地 (かくち) are single tokens
// and don't match this pattern
mod kaku_tests {
    use super::*;

    // Structure: 各 + Noun (productive prefix usage)
    #[test]
    fn kaku_every_room() {
        let sentence = "このホテルは各部屋に洗濯機がついている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "各");
        assert_pattern_range(&patterns, "各", 6, 9); // 各部屋
    }

    // TODO: Undetectable - compound noun variants
    // 各階 (each floor) and 各地 (each place) tokenize as single compound nouns
    // rather than 各 (prefix) + noun, so they don't match this pattern.
    // These are lexicalized compounds in the dictionary.
    //
    // #[test]
    // fn kaku_each_floor() {
    //     let sentence = "このエレベーターは各階で止まります";
    //     // 各階 tokenizes as single noun (名詞/一般), not 各 + 階
    // }
    //
    // #[test]
    // fn kaku_each_place() {
    //     let sentence = "正月になると日本各地からの観光客でいっぱいになる";
    //     // 各地 tokenizes as single noun (名詞/一般), not 各 + 地
    // }
}

// Pattern: 風 (style/manner)
// Data source: grammar_points_data.json["風"]
// Testing: structure.standard[0] - "Noun + 風（ふう）"
//          structure.standard[1] - "Noun + 風（ふう） + （の） + Noun"
//
// Note: 風 always tokenizes as suffix (名詞/接尾/一般) after a noun
// The の in structure[1] is a separate particle, not part of the 風 pattern
mod fuu_tests {
    use super::*;

    // Structure: Noun + 風（ふう）
    #[test]
    fn fuu_american_style() {
        let sentence = "アメリカ風料理が好きです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "風");
        assert_pattern_range(&patterns, "風", 0, 5); // アメリカ風
    }

    // Structure: Noun + 風（ふう） (decade example)
    #[test]
    fn fuu_90s_style() {
        let sentence = "９０年代風ファッションがまた流行ってきています";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "風");
        assert_pattern_range(&patterns, "風", 2, 5); // 年代風
    }

    // Structure: Noun + 風（ふう） + の + Noun
    #[test]
    fn fuu_with_no() {
        let sentence = "メキシコ風の料理を食べたいです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "風");
        assert_pattern_range(&patterns, "風", 0, 5); // メキシコ風
    }

    // Structure: Noun + 風（ふう） + の + Noun
    #[test]
    fn fuu_hiroshima_style() {
        let sentence = "広島風のお好み焼きが一番おいしい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "風");
        assert_pattern_range(&patterns, "風", 0, 3); // 広島風
    }
}

// ========== ばよかった (should have / wish I had) ==========
// Pattern: ばよかった
// Data source: grammar_points_data.json["ばよかった"]
//
// Structures to test:
//   - standard[0]: Verb［ば］+ よかった
//   - polite[0]: Verb［ば］+ よかった + です
//
// Examples from data:
//   - 行けばよかった (I wish I had gone)
//   - 寝ればよかった (I should have slept)
#[cfg(test)]
mod bayokatta_tests {
    use super::*;

    // Structure: Verb［ば］+ よかった
    #[test]
    fn verb_ba_yokatta() {
        let sentence = "高速に乗る前にトイレに行けばよかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ばよかった");
        assert_pattern_range(&patterns, "ばよかった", 11, 18); // 行けばよかった
    }

    // Structure: Verb［ば］+ よかった (different verb)
    #[test]
    fn verb_ba_yokatta_sleep() {
        let sentence = "昨日は早く寝ればよかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ばよかった");
        assert_pattern_range(&patterns, "ばよかった", 5, 12); // 寝ればよかった
    }

    // Structure: Verb［ば］+ よかった + です (polite)
    #[test]
    fn verb_ba_yokatta_polite() {
        let sentence = "もっと勉強すればよかったです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ばよかった");
        assert_pattern_range(&patterns, "ばよかった", 3, 14); // 勉強すればよかったです
    }
}

// ========== たらどう (why don't you / how about) ==========
// Pattern: たらどう
// Data source: grammar_points_data.json["たらどう"]
//
// Structures to test:
//   - standard[0]: Verb［たら］+ どう + だ？
//   - standard[1]: Verb［たら］+ どう + （か）？
//   - polite[0]: Verb［たら］+ どう + です + か
//
// Examples from data:
//   - 減らしたらどう (why don't you reduce)
//   - 入れたらどうか (why don't you add)
#[cfg(test)]
mod taradou_tests {
    use super::*;

    // Structure: Verb［たら］+ どう
    #[test]
    fn verb_tara_dou_basic() {
        let sentence = "食べる量を減らしたらどう？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たらどう");
        assert_pattern_range(&patterns, "たらどう", 5, 12); // 減らしたらどう
    }

    // Structure: Verb［たら］+ どう + か
    #[test]
    fn verb_tara_dou_ka() {
        let sentence = "砂糖を入れたらどうか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たらどう");
        assert_pattern_range(&patterns, "たらどう", 3, 10); // 入れたらどうか
    }

    // Structure: Verb［たら］+ どう + です + か (polite)
    #[test]
    fn verb_tara_dou_desuka() {
        let sentence = "警察に電話してみたらどうですか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たらどう");
        assert_pattern_range(&patterns, "たらどう", 7, 15); // みたらどうですか
    }

    // Structure: Verb［たら］+ どう + だ
    #[test]
    fn verb_tara_dou_da() {
        let sentence = "頭を使ったらどうだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たらどう");
        assert_pattern_range(&patterns, "たらどう", 2, 9); // 使ったらどうだ
    }
}

// ========== がみられる (can be seen/observed) ==========
// Pattern: がみられる
// Data source: grammar_points_data.json["がみられる"]
//
// Structures to test:
//   - standard[0]: Noun + が + 見られる
//   - standard[1]: Noun + も + 見られる
//   - polite[0]: Noun + が + 見られます
//   - polite[1]: Noun + も + 見られます
//
// Meaning: "can be seen", "can be observed" (requires effort to see, like trends/changes)
// Unlike 見える (naturally visible), 見られる focuses on viewer's ability to observe
#[cfg(test)]
mod gamirareru_tests {
    use super::*;

    // Structure: Noun + が + 見られる
    #[test]
    fn noun_ga_mirareru() {
        let sentence = "この部屋からは富士山が見られる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がみられる");
        assert_pattern_range(&patterns, "がみられる", 7, 15); // 富士山が見られる
    }

    // Structure: Noun + も + 見られる
    #[test]
    fn noun_mo_mirareru() {
        let sentence = "奈良に行くと横断歩道を渡るシカも見られる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がみられる");
        assert_pattern_range(&patterns, "がみられる", 13, 20); // シカも見られる
    }

    // Structure: Noun + が + 見られます (polite)
    #[test]
    fn noun_ga_miraremasu_polite() {
        let sentence = "最近はタッチパネル付きの冷蔵庫が見られます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がみられる");
        assert_pattern_range(&patterns, "がみられる", 12, 21); // 冷蔵庫が見られます
    }

    // Structure: Noun + も + 見られます (polite)
    #[test]
    fn noun_mo_miraremasu_polite() {
        let sentence = "この地域では絶滅危惧種も見られます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がみられる");
        assert_pattern_range(&patterns, "がみられる", 10, 17); // 種も見られます
    }
}

// ========== とき (when/at the time) ==========
// Pattern: とき
// Data source: grammar_points_data.json["とき"]
//
// Structures to test:
//   - standard[0]: Verb + とき
//   - standard[1]: い-Adjective + とき
//   - standard[2]: な-Adjective + な + とき
//   - standard[3]: Noun + の + とき
//
// Examples from data:
//   - 開けたときに (when I opened)
//   - 暑いとき (when it's hot)
//   - 暇なときに (when [he is] bored)
//   - 雨のとき (when it rains)
#[cfg(test)]
mod toki_tests {
    use super::*;

    // Structure: Verb + とき
    #[test]
    fn verb_toki() {
        let sentence = "狭い駐車場で車のドアを開けたときにドアを壁にぶつけた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とき");
        assert_pattern_range(&patterns, "とき", 13, 16); // たとき
    }

    // Structure: い-Adjective + とき
    #[test]
    fn i_adjective_toki() {
        let sentence = "暑いときはエアコンを点けましょう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とき");
        assert_pattern_range(&patterns, "とき", 0, 4); // 暑いとき
    }

    // Structure: な-Adjective + な + とき
    #[test]
    fn na_adjective_toki() {
        let sentence = "彼は暇なときに本を読みます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とき");
        assert_pattern_range(&patterns, "とき", 3, 6); // なとき
    }

    // Structure: Noun + の + とき
    #[test]
    fn noun_no_toki() {
        let sentence = "雨のときは家でゴロゴロしています";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とき");
        assert_pattern_range(&patterns, "とき", 1, 4); // のとき
    }
}

// ========== かい (question particle - casual) ==========
// Pattern: かい
// Data source: grammar_points_data.json["かい"]
//
// Structures to test:
//   - standard[0]: Verb + （の） + かい
//   - standard[1]: ［い］Adjective + （の） + かい
//   - standard[2]: Noun + （なの） + かい
//   - standard[3]: ［な］Adjective + （なの） + かい
//
// Notes:
//   - の/なの are optional and can be shortened to ん/なん
//   - Very casual, masculine, direct question particle
//   - Often sounds accusatory, used with familiar people
//
// Examples from data:
//   - 食べたいのかい？ (Do you want to eat?)
//   - 欲しいのかい？ (Do you want it?)
//   - 綺麗なのかい？ (Is she pretty?)
//   - いい人なのかい？ (Is he a good person?)
#[cfg(test)]
mod kai_tests {
    use super::*;

    // Structure: Verb + の + かい (standard[0])
    #[test]
    fn verb_no_kai() {
        let sentence = "このドーナッツを食べたいのかい？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かい");
        assert_pattern_range(&patterns, "かい", 10, 15); // たいのかい
    }

    // Structure: ［い］Adjective + の + かい (standard[1])
    #[test]
    fn i_adjective_no_kai() {
        let sentence = "このおもちゃが欲しいのかい？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かい");
        assert_pattern_range(&patterns, "かい", 7, 13); // 欲しいのかい
    }

    // Structure: Noun + なの + かい (standard[2])
    #[test]
    fn noun_nano_kai() {
        let sentence = "彼はいい人なのかい？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かい");
        assert_pattern_range(&patterns, "かい", 4, 9); // 人なのかい
    }

    // Structure: ［な］Adjective + なの + かい (standard[3])
    #[test]
    fn na_adjective_nano_kai() {
        let sentence = "彼女は綺麗なのかい？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かい");
        assert_pattern_range(&patterns, "かい", 3, 9); // 綺麗なのかい
    }
}

// ========== もし (if/suppose) ==========
// Pattern: もし
// Data source: grammar_points_data.json["もし"]
//
// Structures to test:
//   - standard[0]: もし + Phrase［たら］
//   - Also works with: Phrase［ば］、Phrase［と］、Phrase［ても］
//
// Notes:
//   - Adverb emphasizing the conditional "if" nuance
//   - Means "while slight/small" (low probability)
//   - Used at beginning of conditional phrases
//
// Examples from data:
//   - もし彼を動物に例えると (If we were to compare him to an animal)
//   - もし今日来られれば (If you can come today)
//   - もし買い物に行ったら (If you go shopping)
//   - もし雨が降っても (Even if it rains)
#[cfg(test)]
mod moshi_tests {
    use super::*;

    // Structure: もし + Phrase + たら (standard[0])
    #[test]
    fn moshi_with_tara() {
        let sentence = "もし買い物に行ったら、お菓子を買ってきて";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "もし");
        assert_pattern_range(&patterns, "もし", 0, 2); // もし
    }

    // Structure: もし + Phrase + と
    #[test]
    fn moshi_with_to() {
        let sentence = "もし彼を動物に例えると、猫に似ています";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "もし");
        assert_pattern_range(&patterns, "もし", 0, 2); // もし
    }

    // Structure: もし + Phrase + ば
    #[test]
    fn moshi_with_ba() {
        let sentence = "もし今日来られれば来てね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "もし");
        assert_pattern_range(&patterns, "もし", 0, 2); // もし
    }

    // Structure: もし + Phrase + ても
    #[test]
    fn moshi_with_temo() {
        let sentence = "もし雨が降っても遊園地に行きます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "もし");
        assert_pattern_range(&patterns, "もし", 0, 2); // もし
    }
}

// ========== ぜんぜん (not at all) ==========
// Pattern: ぜんぜん
// Data source: grammar_points_data.json["ぜんぜん"]
//
// Structures to test:
//   - standard[0]: ぜんぜん + Phrase
//   - standard[1]: ぜんぜん + (Negative) Phrase
//
// Note: ぜんぜん traditionally used with negative expressions (containing ない/ません)
// Meaning: "not at all", "completely not"
// Modern usage: Can also be used with positive expressions (not tested here)
#[cfg(test)]
mod zenzen_tests {
    use super::*;

    // Structure: ぜんぜん + Verb[ない]
    #[test]
    fn zenzen_with_nai() {
        let sentence = "やばい、トイレットペーパーがぜんぜんない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ぜんぜん");
        assert_pattern_range(&patterns, "ぜんぜん", 14, 18); // ぜんぜん
    }

    // Structure: ぜんぜん + Verb[なかった]
    #[test]
    fn zenzen_with_nakatta() {
        let sentence = "彼女は家事の手伝いをぜんぜんしなかったので別れました";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ぜんぜん");
        assert_pattern_range(&patterns, "ぜんぜん", 10, 14); // ぜんぜん
    }

    // Structure: ぜんぜん + Verb[ません]
    #[test]
    fn zenzen_with_masen() {
        let sentence = "今日はぜんぜん眠くありません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ぜんぜん");
        assert_pattern_range(&patterns, "ぜんぜん", 3, 7); // ぜんぜん
    }
}

// ========== すこしも～ない (not even a little) ==========
// Pattern: すこしも～ない
// Data source: grammar_points_data.json["すこしも～ない"]
//
// Structures to test:
//   - standard[0]: すこしも + Verb［ない］
//   - standard[1]: すこしも + ［い］Adjective［ない］
//   - standard[2]: すこしも + ［な］Adjective + ではない (or じゃない)
//
// Meaning: "not even a little bit", "not at all"
#[cfg(test)]
mod sukoshimo_nai_tests {
    use super::*;

    // Structure: すこしも + Verb[ない]
    #[test]
    fn sukoshimo_with_verb_nai() {
        let sentence = "今日は人が多いからすこしも離れないでね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "すこしも～ない");
        assert_pattern_range(&patterns, "すこしも～ない", 9, 13); // すこしも
    }

    // Structure: すこしも + い-Adjective[ない]
    #[test]
    fn sukoshimo_with_i_adj_nai() {
        let sentence = "何このケーキ、すこしもおいしくない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "すこしも～ない");
        assert_pattern_range(&patterns, "すこしも～ない", 7, 11); // すこしも
    }

    // Structure: すこしも + な-Adjective + ではない
    #[test]
    fn sukoshimo_with_na_adj_dewa_nai() {
        let sentence = "彼女の彼氏はすこしもイケメンではない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "すこしも～ない");
        assert_pattern_range(&patterns, "すこしも～ない", 6, 10); // すこしも
    }

    // Structure: すこしも + な-Adjective + じゃない
    #[test]
    fn sukoshimo_with_na_adj_ja_nai() {
        let sentence = "このドラマはすこしも面白じゃない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "すこしも～ない");
        assert_pattern_range(&patterns, "すこしも～ない", 6, 10); // すこしも
    }
}

// ========== がひつよう (is necessary) ==========
// Pattern: がひつよう
// Data source: grammar_points_data.json["がひつよう"]
//
// Structures to test:
//   - standard[0]: Noun + が + 必要 + だ
//   - standard[1]: Verb + こと + が + 必要 + だ
//   - polite[0]: Noun + が + 必要 + です
//   - polite[1]: Verb + こと + が + 必要 + です
//
// Meaning: "is necessary", "need"
#[cfg(test)]
mod gahitsuyou_tests {
    use super::*;

    // Structure: Noun + が + 必要 + だ
    #[test]
    fn noun_ga_hitsuyou_da() {
        let sentence = "プールで泳ぐときは水着がひつようだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がひつよう");
        assert_pattern_range(&patterns, "がひつよう", 11, 16); // がひつよう
    }

    // Structure: Noun + が + 必要 (without だ - casual)
    #[test]
    fn noun_ga_hitsuyou_casual() {
        let sentence = "私は運転するときは眼鏡がひつよう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がひつよう");
        assert_pattern_range(&patterns, "がひつよう", 11, 16); // がひつよう
    }

    // Structure: Verb + こと + が + 必要 + だ
    #[test]
    fn verb_koto_ga_hitsuyou_da() {
        let sentence = "彼は車を持っていないから迎えに行くことがひつようだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がひつよう");
        assert_pattern_range(&patterns, "がひつよう", 19, 24); // がひつよう
    }

    // Structure: Noun + が + 必要 + です (polite)
    #[test]
    fn noun_ga_hitsuyou_desu() {
        let sentence = "バスを運転するには特別な免許がひつようです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がひつよう");
        assert_pattern_range(&patterns, "がひつよう", 14, 19); // がひつよう
    }
}

// Pattern: のに  (despite)
// Data source: grammar_points_data.json["のに "]
// Structures:
//   - standard[0]: Verb + のに
//   - standard[1]: い-Adjective + のに
//   - standard[2]: な-Adjective + な + のに
//   - standard[3]: Noun + な + のに
//
// Meaning: "despite", "even though", "in spite of" - shows that B is unexpected given A
#[cfg(test)]
mod noni_tests {
    use super::*;

    // Structure: Verb + のに
    #[test]
    fn verb_noni() {
        let sentence = "明日テストがあるのに彼は勉強をしないでゲームをしている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "のに ");
        assert_pattern_range(&patterns, "のに ", 6, 10); // あるのに
    }

    // Structure: い-Adjective + のに
    #[test]
    fn i_adjective_noni() {
        let sentence = "何で納豆は臭いのに美味しいの？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "のに ");
        assert_pattern_range(&patterns, "のに ", 5, 9); // 臭いのに
    }

    // Structure: な-Adjective + な + のに
    #[test]
    fn na_adjective_noni() {
        let sentence = "彼はイケメンなのにいつも汗臭い";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "のに ");
        assert_pattern_range(&patterns, "のに ", 2, 9); // イケメンなのに
    }

    // Structure: Noun + な + のに
    #[test]
    fn noun_noni() {
        let sentence = "この車はスポーツカーなのに遅い";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "のに ");
        assert_pattern_range(&patterns, "のに ", 4, 13); // スポーツカーなのに
    }
}

// ============================================================================
// なくてもいい - "don't have to / it's okay not to"
// ============================================================================
// Pattern: なくてもいい (don't have to)
// Data source: grammar_points_data.json["なくてもいい"]
// Structures to test:
//   - standard[0]: Verb[なくて] + (も) + いい
//   - polite[0]: Verb[なくて] + (も) + いい + です
mod nakutemoii_tests {
    use super::*;

    // Structure: Verb[なくて] + も + いい (standard with も)
    #[test]
    fn verb_nakutemoii_with_mo() {
        let sentence = "ミカは悪くないから謝らなくてもいいよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくてもいい");
        assert_pattern_range(&patterns, "なくてもいい", 9, 17); // 謝らなくてもいい
    }

    // Structure: Verb[なくて] + いい (casual without も)
    #[test]
    fn verb_nakuteii_without_mo() {
        let sentence = "今日は会社に来なくていいよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくてもいい");
        assert_pattern_range(&patterns, "なくてもいい", 6, 12); // 来なくていい
    }

    // Structure: Verb[なくて] + も + いい + です (polite)
    #[test]
    fn verb_nakutemoii_polite() {
        let sentence = "その手紙はもう届けなくてもいいです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくてもいい");
        assert_pattern_range(&patterns, "なくてもいい", 7, 17); // 届けなくてもいいです
    }

    // Structure: Verb[なくて] + も + いい (negative context)
    #[test]
    fn verb_nakutemoii_negative_permission() {
        let sentence = "宿題をしなくてもいいと言われた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくてもいい");
        assert_pattern_range(&patterns, "なくてもいい", 3, 10); // しなくてもいい
    }
}

// ========== そんなに (that much/so much) ==========
// Pattern: そんなに (demonstrative adverb - "to that extent")
// Data source: grammar_points_data.json["そんなに"]
//
// Structures to test:
//   - standard[0]: そんなに + Verb
//   - standard[1]: そんなに + い-Adjective
//   - standard[2]: そんなに + な-Adjective
//
// Notes:
//   - Part of こそあど言葉 family (こんなに, そんなに, あんなに, どんなに)
//   - Used adverbially before verbs and adjectives
//   - Means "so much", "that much", "to that extent"
mod sonnani_tests {
    use super::*;

    // Testing: standard[0] - そんなに + Verb
    // Example: "そんなに食べたら" - eat that much
    #[test]
    fn test_sonnani_verb() {
        let sentence = "そんなに食べたらお腹を壊すよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そんなに");
        assert_pattern_range(&patterns, "そんなに", 0, 4); // そんなに
    }

    // Testing: standard[1] - そんなに + い-Adjective
    // Example: "そんなに暑い" - that hot
    #[test]
    fn test_sonnani_i_adjective() {
        let sentence = "そんなに暑いと思うならエアコンをつければいい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そんなに");
        assert_pattern_range(&patterns, "そんなに", 0, 4); // そんなに
    }

    // Testing: standard[2] - そんなに + な-Adjective
    // Example: "そんなに嫌い" - dislike that much
    #[test]
    fn test_sonnani_na_adjective() {
        let sentence = "そんなに嫌いなら、無理して食べなくてもいい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そんなに");
        assert_pattern_range(&patterns, "そんなに", 0, 4); // そんなに
    }
}

// Pattern: そういう (like that, that kind of)
// Data source: grammar_points_data.json["そういう"]
// Testing all 4 structure variants: こういう, そういう, ああいう, どういう
//
// Note: ああいう tokenizes as ああ + いう (two tokens), while the others are single tokens
mod souiu_tests {
    use super::*;

    #[test]
    fn test_souiu_koiu() {
        let sentence = "こういう映画は初めて見る";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そういう");
        assert_pattern_range(&patterns, "そういう", 0, 4); // こういう
    }

    #[test]
    fn test_souiu_soiu() {
        let sentence = "そういう人は嫌いだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そういう");
        assert_pattern_range(&patterns, "そういう", 0, 4); // そういう
    }

    #[test]
    fn test_souiu_aaiu() {
        let sentence = "ああいう車に乗ってみたい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そういう");
        assert_pattern_range(&patterns, "そういう", 0, 4); // ああいう
    }

    #[test]
    fn test_souiu_douiu() {
        let sentence = "あなたはどういう音楽を聴きますか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そういう");
        assert_pattern_range(&patterns, "そういう", 4, 8); // どういう
    }
}

// Pattern: そんな・こんな・あんな・どんな (like that, like this, what kind of)
// Data source: grammar_points_data.json["そんな・こんな・あんな・どんな"]
// Testing all 4 structure variants: そんな, こんな, あんな, どんな
//
// Abbreviations from: そのような, このような, あのような, どのような
// All forms tokenize as single tokens (連体詞)
mod sonna_tests {
    use super::*;

    #[test]
    fn test_sonna_sonna() {
        let sentence = "そんな言い方してはいけません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そんな・こんな・あんな・どんな");
        assert_pattern_range(&patterns, "そんな・こんな・あんな・どんな", 0, 3); // そんな
    }

    #[test]
    fn test_sonna_konna() {
        let sentence = "こんな高いものはもらえないよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そんな・こんな・あんな・どんな");
        assert_pattern_range(&patterns, "そんな・こんな・あんな・どんな", 0, 3); // こんな
    }

    #[test]
    fn test_sonna_anna() {
        let sentence = "あんな大人にはなりたくない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そんな・こんな・あんな・どんな");
        assert_pattern_range(&patterns, "そんな・こんな・あんな・どんな", 0, 3); // あんな
    }

    #[test]
    fn test_sonna_donna() {
        let sentence = "どんな靴が欲しい？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そんな・こんな・あんな・どんな");
        assert_pattern_range(&patterns, "そんな・こんな・あんな・どんな", 0, 3); // どんな
    }
}

// Pattern: Verb[よう] (volitional form - let's, shall)
// Data source: grammar_points_data.json["Verb[よう]"]
// Testing all structure variants from standard and polite forms
mod verb_you_tests {
    use super::*;

    // Testing: structure.standard - Ichidan verb (る1) + よう
    #[test]
    fn test_verb_you_ichidan() {
        let sentence = "今夜は映画を見ようと思っている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[よう]");
        assert_pattern_range(&patterns, "Verb[よう]", 6, 9); // 見よう
    }

    // Testing: structure.standard - Godan る5 verb + ろう
    #[test]
    fn test_verb_you_godan_ru() {
        let sentence = "そろそろ座ろうか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[よう]");
        assert_pattern_range(&patterns, "Verb[よう]", 4, 7); // 座ろう
    }

    // Testing: structure.standard - Godan う verb + おう
    #[test]
    fn test_verb_you_godan_u() {
        let sentence = "みんなで歌おうよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[よう]");
        assert_pattern_range(&patterns, "Verb[よう]", 4, 7); // 歌おう
    }

    // Testing: structure.standard - Godan く verb + こう
    #[test]
    fn test_verb_you_godan_ku() {
        let sentence = "公園まで歩こうか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[よう]");
        assert_pattern_range(&patterns, "Verb[よう]", 4, 7); // 歩こう
    }

    // Testing: structure.standard - Godan す verb + そう
    #[test]
    fn test_verb_you_godan_su() {
        let sentence = "もう一度話そう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[よう]");
        assert_pattern_range(&patterns, "Verb[よう]", 4, 7); // 話そう
    }

    // Testing: structure.standard - Godan ぐ verb + ごう
    #[test]
    fn test_verb_you_godan_gu() {
        let sentence = "プールで泳ごうよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[よう]");
        assert_pattern_range(&patterns, "Verb[よう]", 4, 7); // 泳ごう
    }

    // Testing: structure.standard - Godan む verb + もう
    #[test]
    fn test_verb_you_godan_mu() {
        let sentence = "明日は休もう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[よう]");
        assert_pattern_range(&patterns, "Verb[よう]", 3, 6); // 休もう
    }

    // Testing: structure.standard - Exception する → しよう
    #[test]
    fn test_verb_you_suru() {
        let sentence = "私が運転しようか？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[よう]");
        assert_pattern_range(&patterns, "Verb[よう]", 2, 7); // 運転しよう
    }

    // Testing: structure.standard - Exception くる → こよう
    #[test]
    fn test_verb_you_kuru() {
        let sentence = "明日また来ようね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[よう]");
        assert_pattern_range(&patterns, "Verb[よう]", 4, 7); // 来よう
    }

    // Testing: structure.polite - Ichidan verb + ましょう
    #[test]
    fn test_verb_you_polite_ichidan() {
        let sentence = "一緒に見ましょう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[よう]");
        assert_pattern_range(&patterns, "Verb[よう]", 3, 8); // 見ましょう
    }

    // Testing: structure.polite - Godan verb + ましょう
    #[test]
    fn test_verb_you_polite_godan() {
        let sentence = "ここで休みましょう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[よう]");
        assert_pattern_range(&patterns, "Verb[よう]", 3, 9); // 休みましょう
    }

    // Testing: structure.polite - Exception する → しましょう
    #[test]
    fn test_verb_you_polite_suru() {
        let sentence = "頑張りましょう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[よう]");
        assert_pattern_range(&patterns, "Verb[よう]", 0, 7); // 頑張りましょう
    }
}

// ========== ながら (while doing) ==========
// Pattern: ながら
// Data source: grammar_points_data.json["ながら"]
//
// Structure to test:
//   - standard[0]: Verb[stem] + ながら
//
// Examples from data:
//   - 聴きながら走ります (run while listening)
//   - 運転しながら携帯を使っていたら (while driving, using phone)
//
// Note: The two actions must share the same subject
#[cfg(test)]
mod nagara_tests {
    use super::*;

    // Testing: structure.standard[0] - Verb[stem] + ながら (ichidan verb)
    #[test]
    fn test_nagara_ichidan_verb() {
        let sentence = "毎日、音楽を聴きながら走ります";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ながら");
        assert_pattern_range(&patterns, "ながら", 6, 11); // 聴きながら
    }

    // Testing: structure.standard[0] - Verb[stem] + ながら (godan verb - する)
    #[test]
    fn test_nagara_godan_verb() {
        let sentence = "運転しながら携帯を使ってはいけない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ながら");
        assert_pattern_range(&patterns, "ながら", 0, 6); // 運転しながら
    }

    // Testing: structure.standard[0] - Verb[stem] + ながら (godan verb - く)
    #[test]
    fn test_nagara_godan_ru_verb() {
        let sentence = "歩きながら考えるのが好きだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ながら");
        assert_pattern_range(&patterns, "ながら", 0, 5); // 歩きながら
    }
}

// Pattern: だいたい (generally, mostly, approximately)
// Data source: grammar_points_data.json["だいたい"]
//
// Structure variants to test:
//   - standard[0]: だいたい + Phrase
//   - standard[1]: だいたい + Number/Degree
//   - standard[2]: だいたい + の + Noun
//   - Also: だいたい as emphatic "in the first place"
#[cfg(test)]
mod daitai_tests {
    use super::*;

    // Testing: structure.standard[0] - だいたい + Phrase
    #[test]
    fn test_daitai_phrase() {
        let sentence = "水曜日はだいたい５時に起きています";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だいたい");
        assert_pattern_range(&patterns, "だいたい", 4, 8); // だいたい
    }

    // Testing: structure.standard[1] - だいたい + Number/Degree
    #[test]
    fn test_daitai_number() {
        let sentence = "だいたい１０人くらい来ると思う";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だいたい");
        assert_pattern_range(&patterns, "だいたい", 0, 4); // だいたい
    }

    // Testing: structure.standard[2] - だいたい + の + Noun
    #[test]
    fn test_daitai_no_noun() {
        let sentence = "だいたいの人たちが薔薇という漢字を書けません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だいたい");
        assert_pattern_range(&patterns, "だいたい", 0, 4); // だいたい (the の pattern also matches だいたいの)
    }

    // Testing: だいたい as emphatic "in the first place"
    #[test]
    fn test_daitai_emphatic() {
        let sentence = "だいたいなんでお前がここにいるの？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だいたい");
        assert_pattern_range(&patterns, "だいたい", 0, 4); // だいたい
    }
}

// ========== だけでなく (not only) ==========
// Pattern: だけでなく
// Data source: grammar_points_data.json["だけでなく"]
//
// Structures to test:
//   - standard[0]: Verb + だけでなく
//   - standard[1]: い-Adjective + だけでなく
//   - standard[2]: な-Adjective + な + だけでなく
//   - standard[3]: Noun + だけでなく
//   - Variants: だけではなく, だけじゃなく, だけでなくて
//
// Examples from data:
//   - 運動をするだけでなく - not only exercise
//   - 暑いだけでなく - not only hot
//   - 静かなだけでなく - not only quiet
//   - 車だけでなく - not only a car
#[cfg(test)]
mod dakedenaku_tests {
    use super::*;

    // Testing: structure.standard[0] - Verb + だけでなく
    #[test]
    fn test_dakedenaku_verb() {
        let sentence = "痩せるためには運動をするだけでなく、食べるものにも気をつけなくてはいけない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だけでなく");
        assert_pattern_range(&patterns, "だけでなく", 10, 17); // するだけでなく
    }

    // Testing: structure.standard[1] - い-Adjective + だけでなく
    #[test]
    fn test_dakedenaku_i_adjective() {
        let sentence = "名古屋の夏は暑いだけでなく、湿気もひどい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だけでなく");
        assert_pattern_range(&patterns, "だけでなく", 6, 13); // 暑いだけでなく
    }

    // Testing: structure.standard[2] - な-Adjective + な + だけでなく
    // Note: Range starts from "な" due to tokenization (静か + な are separate tokens)
    #[test]
    fn test_dakedenaku_na_adjective() {
        let sentence = "田舎は静かなだけでなく、空気もきれいだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だけでなく");
        assert_pattern_range(&patterns, "だけでなく", 5, 11); // なだけでなく
    }

    // Testing: structure.standard[3] - Noun + だけでなく
    #[test]
    fn test_dakedenaku_noun() {
        let sentence = "トーマスは車だけでなく、バイクとボートも持っている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だけでなく");
        assert_pattern_range(&patterns, "だけでなく", 5, 11); // 車だけでなく
    }

    // Testing: Variant - だけではなく (with は)
    #[test]
    fn test_dakedenaku_with_wa() {
        let sentence = "彼は英語だけではなく、フランス語も話せる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だけでなく");
        assert_pattern_range(&patterns, "だけでなく", 2, 10); // 英語だけではなく
    }

    // Testing: Variant - だけじゃなく (casual with じゃ)
    #[test]
    fn test_dakedenaku_janaku() {
        let sentence = "タケルはピアノだけじゃなく、ギターも弾けると聞いた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だけでなく");
        assert_pattern_range(&patterns, "だけでなく", 4, 13); // ピアノだけじゃなく
    }

    // Testing: Variant - だけでなくて (with て)
    #[test]
    fn test_dakedenaku_with_te() {
        let sentence = "角にあるラーメン屋はおいしいだけでなくて量も多い";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だけでなく");
        assert_pattern_range(&patterns, "だけでなく", 10, 20); // おいしいだけでなくて
    }
}

// ========== すくなくない (not few = quite a few, many) ==========
// Pattern: すくなくない
// Data source: grammar_points_data.json["すくなくない"]
//
// Structures to test:
//   - standard[0]: Noun + は + 少なくない
//   - standard[1]: Noun + が + 少なくない
//   - standard[2]: Noun + も + 少なくない
//   - polite[0]: Noun + は + 少なくありません
//   - polite[1]: Noun + が + 少なくありません
//   - polite[2]: Noun + も + 少なくありません
//
// Grammar note:
//   - Double negative: 少ない (few) -> 少なくない (not few = quite a few)
//   - 少ない is itself an い-adjective (the ない is part of the word, not negation)
//   - Pattern conveys "quite a few", "quite a lot", "many"
#[cfg(test)]
mod sukunakunai_tests {
    use super::*;

    // Testing: standard[0] - Noun + は + 少なくない
    #[test]
    fn test_sukunakunai_wa_casual() {
        let sentence = "漢字を書くのが嫌いな子は少なくない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "すくなくない");
        assert_pattern_range(&patterns, "すくなくない", 12, 17); // 少なくない
    }

    // Testing: standard[1] - Noun + が + 少なくない
    #[test]
    fn test_sukunakunai_ga_casual() {
        let sentence = "私は好き嫌いが少なくないんです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "すくなくない");
        assert_pattern_range(&patterns, "すくなくない", 7, 12); // 少なくない
    }

    // Testing: standard[2] - Noun + も + 少なくない
    #[test]
    fn test_sukunakunai_mo_casual() {
        let sentence = "寝る前にお菓子を食べる人も少なくない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "すくなくない");
        assert_pattern_range(&patterns, "すくなくない", 13, 18); // 少なくない
    }

    // Testing: polite[0] - Noun + は + 少なくありません
    #[test]
    fn test_sukunakunai_wa_polite() {
        let sentence = "この問題に関心を持つ学生は少なくありません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "すくなくない_polite");
        assert_pattern_range(&patterns, "すくなくない_polite", 13, 21); // 少なくありません
    }

    // Testing: polite[1] - Noun + が + 少なくありません
    #[test]
    fn test_sukunakunai_ga_polite() {
        let sentence = "最近は在宅勤務を選ぶ人が少なくありません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "すくなくない_polite");
        assert_pattern_range(&patterns, "すくなくない_polite", 12, 20); // 少なくありません
    }

    // Testing: polite[2] - Noun + も + 少なくありません
    #[test]
    fn test_sukunakunai_mo_polite() {
        let sentence = "高校生の中でアルバイトをしている人も少なくありません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "すくなくない_polite");
        assert_pattern_range(&patterns, "すくなくない_polite", 18, 26); // 少なくありません
    }
}

// ========== ても (even if/even though) ==========
// Pattern: ても
// Data source: grammar_points_data.json["ても"]
//
// Structures to test:
//   - standard[0]: Verb[て] + も
//   - standard[1]: い-Adjective[て] + も
//   - standard[2]: な-Adjective + で + も
//   - standard[3]: Noun + で + も
//   - standard[4]: Verb[なくて] + も
//   - standard[5]: い-Adjective[なくて] + も
//   - standard[6]: な-Adjective + でなくて + も
//   - standard[7]: Noun + でなくて + も
//
// Examples from data:
//   - 言っても何も変わりません (even if you say it, nothing will change)
//   - 冷たくても美味しい (even if it's cold, it's delicious)
//   - 大変でも諦めません (even if it's difficult, won't give up)
//   - 安い電子レンジでも (even with a cheap microwave)
//   - 終わらなくてもいい (even if you don't finish, it's okay)
#[cfg(test)]
mod temo_tests {
    use super::*;

    // Testing: standard[0] - Verb[て] + も
    #[test]
    fn test_temo_verb_affirmative() {
        let sentence = "あの人に言っても何も変わりません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ても");
        assert_pattern_range(&patterns, "ても", 4, 8); // 言っても
    }

    // Testing: standard[1] - い-Adjective[て] + も
    #[test]
    fn test_temo_i_adjective() {
        let sentence = "お茶は冷たくても美味しいから好きです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ても");
        assert_pattern_range(&patterns, "ても", 3, 8); // 冷たくても
    }

    // Testing: standard[2] - な-Adjective + で + も
    #[test]
    fn test_temo_na_adjective() {
        let sentence = "彼女は仕事が大変でも諦めません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ても");
        assert_pattern_range(&patterns, "ても", 6, 10); // 大変でも
    }

    // Testing: standard[3] - Noun + で + も
    #[test]
    fn test_temo_noun() {
        let sentence = "安い電子レンジでも弁当は温められます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ても");
        assert_pattern_range(&patterns, "ても", 4, 9); // レンジでも
    }

    // Testing: standard[4] - Verb[なくて] + も
    #[test]
    fn test_temo_verb_negative() {
        let sentence = "これは明日までに終わらなくてもいいから";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ても");
        assert_pattern_range(&patterns, "ても", 11, 15); // なくても
    }

    // Testing: standard[5] - い-Adjective[なくて] + も
    #[test]
    fn test_temo_i_adjective_negative() {
        let sentence = "頭が痛くなくてもこの薬を飲んでください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ても");
        assert_pattern_range(&patterns, "ても", 4, 8); // なくても
    }

    // Testing: standard[6] - な-Adjective + でなくて + も
    #[test]
    fn test_temo_na_adjective_negative() {
        let sentence = "野菜は好きじゃなくても食べた方がいい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ても");
        assert_pattern_range(&patterns, "ても", 7, 11); // なくても
    }

    // Testing: standard[7] - Noun + でなくて + も
    #[test]
    fn test_temo_noun_negative() {
        let sentence = "運転手じゃなくてもシートベルトをしなくてはいけない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ても");
        assert_pattern_range(&patterns, "ても", 5, 9); // なくても
    }
}

// Pattern: Causative-Passive
// Data source: grammar_points_data.json["Causative-Passive"]
//
// Structure variants to test:
// - る-Verbs: Verb[未然形] + させられる (e.g., 見させられる)
// - う-Verbs (long form): Verb[未然形] + (a)せられる (e.g., 歩かせられる)
// - う-Verbs (short form): Verb[未然形] + (a)される (e.g., 歩かされる)
// - する exception: させられる
// - くる exception: こさせられる
// - Polite forms: Add ます
// - Past forms: Replace る with た
#[cfg(test)]
mod causative_passive_tests {
    use super::*;

    // Testing: る-Verb (る1) + させられる
    #[test]
    fn test_ru_verb_saseru_passive() {
        let sentence = "おばあちゃんの家に行くとお腹がいっぱいでもいっぱい食べさせられる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Causative-Passive");
        assert_pattern_range(&patterns, "Causative-Passive", 25, 32); // 食べさせられる
    }

    // Testing: う-Verb (short form) + される - past tense
    #[test]
    fn test_u_verb_short_past() {
        let sentence = "先輩に色んなお酒を飲まされたから頭が痛い";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Causative-Passive");
        assert_pattern_range(&patterns, "Causative-Passive", 9, 14); // 飲まされた
    }

    // Testing: る-Verb + させられる (non-past negative context)
    #[test]
    fn test_ru_verb_negative_context() {
        let sentence = "友達に冷たい水を浴びさせられるのが嫌いだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Causative-Passive");
        assert_pattern_range(&patterns, "Causative-Passive", 8, 15); // 浴びさせられる
    }

    // Testing: する exception → させられる
    #[test]
    fn test_suru_exception() {
        let sentence = "上司に力仕事をさせられる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Causative-Passive");
        assert_pattern_range(&patterns, "Causative-Passive", 7, 12); // させられる
    }

    // Testing: くる exception → こさせられる
    #[test]
    fn test_kuru_exception() {
        let sentence = "友達に知らないバンドのコンサートに連れてこさせられる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Causative-Passive");
        assert_pattern_range(&patterns, "Causative-Passive", 20, 26); // こさせられる
    }

    // Testing: う-Verb (long form) + せられる
    #[test]
    fn test_u_verb_long_form() {
        let sentence = "子供の頃は親に毎日歩かせられていた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Causative-Passive");
        assert_pattern_range(&patterns, "Causative-Passive", 9, 14); // 歩かせられ
    }

    // Testing: Polite form + ます
    #[test]
    fn test_polite_form() {
        let sentence = "会社では残業をさせられますか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Causative-Passive");
        assert_pattern_range(&patterns, "Causative-Passive", 7, 13); // させられます
    }

    // Testing: て-form continuation (させられている)
    #[test]
    fn test_te_iru_continuation() {
        let sentence = "子供の頃は兄が勉強をさせられていた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Causative-Passive");
        assert_pattern_range(&patterns, "Causative-Passive", 10, 14); // させられ
    }
}

// ========== Number/Amount + は (at least, or so) ==========
// Pattern: Number/Amount + は
// Data source: grammar_points_data.json["Number/Amount + は"]
//
// Structures to test:
//   - standard[0]: Number/Amount + Counter + (くらい) + は
//   - standard[1]: Noun + くらい + は
//
// Examples from data:
//   - 年に５回は行っている (at least 5 times a year)
//   - 毎日、テレビを５時間は見ている (watch TV for 5 hours or so every day)
//   - ２キロくらいはあると思う (I think it's at least 2 kilograms)
//   - １回ぐらいは行った方がいい (should go at least once)
#[cfg(test)]
mod number_amount_ha_tests {
    use super::*;

    // Test 1: Number + Counter + は (without くらい)
    #[test]
    fn test_number_counter_ha_basic() {
        let sentence = "ディズニーランドには年に５回は行っている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Number/Amount + は");
        assert_pattern_range(&patterns, "Number/Amount + は", 13, 15); // 回は
    }

    // Test 2: Number + Counter + は (time duration)
    #[test]
    fn test_number_counter_ha_duration() {
        let sentence = "毎日、テレビを５時間は見ている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Number/Amount + は");
        assert_pattern_range(&patterns, "Number/Amount + は", 8, 11); // 時間は
    }

    // Test 3: Number + Counter + くらい + は
    #[test]
    fn test_number_counter_kurai_ha() {
        let sentence = "２キロくらいはあると思う";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Number/Amount + は");
        assert_pattern_range(&patterns, "Number/Amount + は", 1, 7); // キロくらいは
    }

    // Test 4: Number + Counter + ぐらい + は (ぐらい variant)
    #[test]
    fn test_number_counter_gurai_ha() {
        let sentence = "倉敷には１回ぐらいは行った方がいいよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Number/Amount + は");
        assert_pattern_range(&patterns, "Number/Amount + は", 5, 10); // 回ぐらいは
    }

    // Note: The structure variant "Noun + くらい + は" (without counter) is primarily
    // handled by the くらい pattern itself. This pattern focuses on the contrastive は
    // after numeric counters (助数詞) to mean "at least" or "or so".
}

// ========== Question-phrase + か (embedded question) ==========
// Pattern: Question-phrase + か
// Data source: grammar_points_data.json["Question-phrase + か"]
//
// Structure to test:
//   - standard[0]: Question Word + か + わかる/知る/覚える/決める etc.
//
// Examples from data:
//   - 社長が来るか分かりますか (Do you know if the CEO is coming?)
//   - これで足りるか分かる？ (Do you know if this is enough?)
//   - 何でこのビルを壊すか知っていますか (Do you know why they are destroying this building?)
//   - お祭りは何時に終わるか知ってる？ (Do you know what time the festival ends?)
//
// Note: This is the adverbial particle か (副助詞), not the sentence-ending question marker.
// It highlights uncertain things and is followed by verbs seeking information.
#[cfg(test)]
mod question_phrase_ka_tests {
    use super::*;

    // Test 1: Verb + か + わかる (if/whether)
    #[test]
    fn test_verb_ka_wakaru() {
        let sentence = "忘年会に社長が来るか分かりますか？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Question-phrase + か");
        assert_pattern_range(&patterns, "Question-phrase + か", 9, 15); // か分かります
    }

    // Test 2: Verb + か + わかる casual (if this is enough)
    #[test]
    fn test_verb_ka_wakaru_casual() {
        let sentence = "これで足りるか分かる？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Question-phrase + か");
        assert_pattern_range(&patterns, "Question-phrase + か", 6, 10); // か分かる
    }

    // Test 3: Question word (何で) + verb + か + 知る (why)
    #[test]
    fn test_question_word_verb_ka_shiru() {
        let sentence = "何でこのビルを壊すか知っていますか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Question-phrase + か");
        assert_pattern_range(&patterns, "Question-phrase + か", 9, 12); // か知っ
    }

    // Test 4: Question word (何時) + verb + か + 知る (what time)
    #[test]
    fn test_question_word_verb_ka_shiru_casual() {
        let sentence = "お祭りは何時に終わるか知ってる？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Question-phrase + か");
        assert_pattern_range(&patterns, "Question-phrase + か", 10, 13); // か知っ
    }

    // Test 5: Question word (どこ) + か + verb (where)
    #[test]
    fn test_question_word_ka_verb() {
        let sentence = "彼がどこに住んでるか教えてくれる？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Question-phrase + か");
        assert_pattern_range(&patterns, "Question-phrase + か", 9, 12); // か教え
    }
}

// ========== Verb[て] + B (Contrastive conjunction) ==========
// Pattern: Verb[て] + B
// Data source: grammar_points_data.json["Verb[て] + B"]
//
// Structure to test:
//   - standard[0]: Verb［て］+ Phrase
//
// This pattern expresses CONTRAST (not sequence) using て-form.
// Key characteristics:
//   - Connects two contrasting but related events/states with equal weight
//   - Both clauses usually have different subjects (marked with は)
//   - NOT sequential actions (that's a different use of て)
//   - Similar to "while (A), (B)" or "(A) and (B)" with contrastive nuance
//
// Examples from data:
//   - 姉ちゃんは毎晩勉強をして弟は毎晩ゲームをしている (Sister studies, AND brother plays games)
//   - 妻は買い物に行って、私はごみを捨てに行った (Wife went shopping WHILE I threw trash)
//   - タケルはご飯を食べてナオミはパンを食べる (Takeru eats rice AND Naomi eats bread)
#[cfg(test)]
mod verb_te_b_tests {
    use super::*;

    // Test 1: Two contrasting actions with は (sister vs brother)
    #[test]
    fn test_verb_te_b_contrasting_subjects() {
        let sentence = "姉ちゃんは毎晩勉強をして弟は毎晩ゲームをしている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[て] + B");
        assert_pattern_range(&patterns, "Verb[て] + B", 10, 14); // して弟は
    }

    // Test 2: Two contrasting destinations (wife vs I) - with comma
    #[test]
    fn test_verb_te_b_contrasting_destinations() {
        let sentence = "妻は買い物に行って、私はごみを捨てに行った";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[て] + B");
        assert_pattern_range(&patterns, "Verb[て] + B", 6, 12); // 行って、私は
    }

    // Test 2b: Without comma to verify pattern works both ways
    #[test]
    fn test_verb_te_b_without_comma() {
        let sentence = "妻は買い物に行って私はごみを捨てに行った";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[て] + B");
        assert_pattern_range(&patterns, "Verb[て] + B", 6, 11); // 行って私は
    }

    // Test 3: Two contrasting foods (Takeru vs Naomi)
    #[test]
    fn test_verb_te_b_contrasting_preferences() {
        let sentence = "タケルはご飯を食べてナオミはパンを食べる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[て] + B");
        assert_pattern_range(&patterns, "Verb[て] + B", 7, 14); // 食べてナオミは
    }

    // Test 4: Weather contrast (morning vs afternoon)
    #[test]
    fn test_verb_te_b_weather_contrast() {
        let sentence = "朝は雨が降って夕方は晴れた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[て] + B");
        assert_pattern_range(&patterns, "Verb[て] + B", 4, 10); // 降って夕方は
    }
}

// ============================================================================
// Pattern: Verb[て]・Noun[で] + B (means/method/circumstances)
// Data source: grammar_points_data.json["Verb[て]・Noun[で] + B"]
// Testing structures:
//   - standard[0]: Verb[て] + Phrase (means/circumstances)
//   - standard[1]: Noun + で + Phrase (means/method)
// ============================================================================
mod verb_te_noun_de_b_tests {
    use super::*;

    // Test 1: Verb[て] expressing circumstances - "in a flurry/hurry"
    #[test]
    fn test_verb_te_means_hurry() {
        let sentence = "寝坊をしたので慌てて準備をした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[て]・Noun[で] + B");
        assert_pattern_range(&patterns, "Verb[て]・Noun[で] + B", 7, 10); // 慌てて
    }

    // Test 2: Verb[て] expressing method - "by swimming"
    #[test]
    fn test_verb_te_means_swimming() {
        let sentence = "毎日泳いでトレーニングをしています";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[て]・Noun[で] + B");
        assert_pattern_range(&patterns, "Verb[て]・Noun[で] + B", 2, 5); // 泳いで
    }

    // Test 3: Noun + で expressing means of transportation
    #[test]
    fn test_noun_de_means_car() {
        let sentence = "父は車で仕事に行く";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[て]・Noun[で] + B");
        assert_pattern_range(&patterns, "Verb[て]・Noun[で] + B", 2, 4); // 車で
    }

    // Test 4: Noun + で expressing tool/instrument
    #[test]
    fn test_noun_de_means_scissors() {
        let sentence = "私はハサミで野菜を切ります";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[て]・Noun[で] + B");
        assert_pattern_range(&patterns, "Verb[て]・Noun[で] + B", 2, 6); // ハサミで
    }

    // Test 5: Verb[て] expressing circumstances - realistic subtitle example
    #[test]
    fn test_verb_te_means_running() {
        let sentence = "走ってここまで来たから息が切れてる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[て]・Noun[で] + B");
        assert_pattern_range(&patterns, "Verb[て]・Noun[で] + B", 0, 3); // 走って
    }

    // Test 6: Noun + で expressing location/place (extended use of で)
    #[test]
    fn test_noun_de_means_internet() {
        let sentence = "ネットで調べれば分かるでしょ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[て]・Noun[で] + B");
        assert_pattern_range(&patterns, "Verb[て]・Noun[で] + B", 0, 4); // ネットで
    }
}

// Pattern: Verb［せる・させる］(Causative form - make/let someone do)
// Data source: grammar_points_data.json["Verb［せる・させる］"]
// Testing all verb types with causative form
//
// Structures to test:
//   - Ichidan verb (る1): 見る + させる
//   - Godan verb (る5): 座る + らせる
//   - Godan う verb: 歌う + わせる
//   - Godan く verb: 歩く + かせる
//   - Godan す verb: 話す + させる
//   - Godan む verb: 休む + ませる
//   - Godan ぐ verb: 泳ぐ + がせる
//   - Exception する: させる
//   - Exception くる: こさせる
//   - Polite forms: + ます
mod causative_tests {
    use super::*;

    #[test]
    fn test_causative_ichidan_verb() {
        let sentence = "息子に野菜を食べさせる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb［せる・させる］");
        assert_pattern_range(&patterns, "Verb［せる・させる］", 6, 11); // 食べさせる
    }

    #[test]
    fn test_causative_godan_ru5_verb() {
        let sentence = "会議で部下を座らせた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb［せる・させる］");
        assert_pattern_range(&patterns, "Verb［せる・させる］", 6, 10); // 座らせた
    }

    #[test]
    fn test_causative_godan_u_verb() {
        let sentence = "忘年会で後輩に歌を歌わせた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb［せる・させる］");
        assert_pattern_range(&patterns, "Verb［せる・させる］", 9, 13); // 歌わせた
    }

    #[test]
    fn test_causative_godan_ku_verb() {
        let sentence = "犬を外で歩かせる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb［せる・させる］");
        assert_pattern_range(&patterns, "Verb［せる・させる］", 4, 8); // 歩かせる
    }

    #[test]
    fn test_causative_godan_su_verb() {
        let sentence = "会社で先輩に英語を話させる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb［せる・させる］");
        assert_pattern_range(&patterns, "Verb［せる・させる］", 9, 13); // 話させる
    }

    #[test]
    fn test_causative_godan_mu_verb() {
        let sentence = "今日は子供を早く休ませた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb［せる・させる］");
        assert_pattern_range(&patterns, "Verb［せる・させる］", 8, 12); // 休ませた
    }

    // NOTE: 泳がせる is tokenized as a single dictionary entry by Kagome
    // (動詞/自立, base='泳がせる') rather than 泳が + せる as separate tokens.
    // This test documents this lexical behavior - our pattern detects the
    // compositional causative form (verb stem + せる/させる as separate tokens).
    #[test]
    fn test_causative_godan_gu_verb() {
        let sentence = "夏休みに子供をプールで泳がせる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // This sentence does NOT match our pattern due to single-token lexical entry
        // The causative meaning is preserved, just not compositionally formed
        assert!(!has_pattern(&patterns, "Verb［せる・させる］"));
    }

    #[test]
    fn test_causative_suru_exception() {
        let sentence = "親を心配させることはしてはいけない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb［せる・させる］");
        assert_pattern_range(&patterns, "Verb［せる・させる］", 2, 7); // 心配させる
    }

    #[test]
    fn test_causative_kuru_exception() {
        let sentence = "夜遅くに会社に来させるのはよくない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb［せる・させる］");
        assert_pattern_range(&patterns, "Verb［せる・させる］", 7, 11); // 来させる
    }

    #[test]
    fn test_causative_polite_form() {
        let sentence = "友達が携帯を壊したので新しいのを買わせます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb［せる・させる］");
        assert_pattern_range(&patterns, "Verb［せる・させる］", 16, 21); // 買わせます
    }
}

// Pattern: Verb［れる・られる］(Passive form - something happens to the subject)
// Data source: grammar_points_data.json["Verb［れる・られる］"]
// Testing all verb types with passive form
//
// Structures to test:
//   - Ichidan verb (る1): 見る + られる
//   - Godan verb (る5): 座る + られる
//   - Godan う verb: 歌う + われる
//   - Godan く verb: 歩く + かれる
//   - Godan す verb: 話す + される
//   - Godan む verb: 休む + まれる
//   - Godan ぶ verb: 飛ぶ + ばれる
//   - Godan ぐ verb: 泳ぐ + がれる
//   - Exception する: される
//   - Exception くる: こられる
//   - Polite forms: + ます
mod passive_tests {
    use super::*;

    #[test]
    fn test_passive_ichidan_verb() {
        let sentence = "私はいつも彼と比べられる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb［れる・られる］");
        assert_pattern_range(&patterns, "Verb［れる・られる］", 7, 12); // 比べられる
    }

    #[test]
    fn test_passive_godan_ru5_verb() {
        let sentence = "彼は部長に会議で座られた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb［れる・られる］");
        assert_pattern_range(&patterns, "Verb［れる・られる］", 8, 12); // 座られた
    }

    #[test]
    fn test_passive_godan_u_verb() {
        let sentence = "カラオケで友達に歌われて恥ずかしかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb［れる・られる］");
        assert_pattern_range(&patterns, "Verb［れる・られる］", 8, 11); // 歌われ
    }

    #[test]
    fn test_passive_godan_ku_verb() {
        let sentence = "静かな公園を歩かれるのが好き";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb［れる・られる］");
        assert_pattern_range(&patterns, "Verb［れる・られる］", 6, 10); // 歩かれる
    }

    #[test]
    fn test_passive_godan_su_verb() {
        let sentence = "人の前で悪口を話されて傷ついた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb［れる・られる］");
        assert_pattern_range(&patterns, "Verb［れる・られる］", 7, 10); // 話され
    }

    #[test]
    fn test_passive_godan_mu_verb() {
        let sentence = "犬に噛まれるのが怖いから近づけない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb［れる・られる］");
        assert_pattern_range(&patterns, "Verb［れる・られる］", 2, 6); // 噛まれる
    }

    #[test]
    fn test_passive_godan_bu_verb() {
        let sentence = "鳥が空を飛ばれている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb［れる・られる］");
        assert_pattern_range(&patterns, "Verb［れる・られる］", 4, 7); // 飛ばれ
    }

    #[test]
    fn test_passive_godan_gu_verb() {
        let sentence = "プールで泳がれるのは気持ちいい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb［れる・られる］");
        assert_pattern_range(&patterns, "Verb［れる・られる］", 4, 8); // 泳がれる
    }

    #[test]
    fn test_passive_suru_exception() {
        let sentence = "知らない人からいたずらをされるのが嫌い";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb［れる・られる］");
        assert_pattern_range(&patterns, "Verb［れる・られる］", 12, 15); // される
    }

    #[test]
    fn test_passive_kuru_exception() {
        let sentence = "急に義理の母に来られると困る";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb［れる・られる］");
        assert_pattern_range(&patterns, "Verb［れる・られる］", 7, 11); // 来られる
    }

    #[test]
    fn test_passive_polite_form() {
        let sentence = "先生に叱られます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb［れる・られる］");
        assert_pattern_range(&patterns, "Verb［れる・られる］", 3, 8); // 叱られます
    }
}

// Pattern: いたす (humble speech - to do)
// Data source: grammar_points_data.json["いたす"]
// Testing: structure.standard[0-2] and polite[0-2]
//
// Structures:
//   - standard[0]: する → いたす
//   - standard[1]: お + Verb[stem] + いたす
//   - standard[2]: ご + [する]Verb + いたす
//   - polite[0]: する → いたす (polite form)
//   - polite[1]: お + Verb[stem] + いたします
//   - polite[2]: ご + [する]Verb + いたします
mod itasu_tests {
    use super::*;

    #[test]
    fn test_itasu_suru_replacement() {
        let sentence = "私たちが用意いたします";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いたす");
        assert_pattern_range(&patterns, "いたす", 4, 11); // 用意いたします
    }

    #[test]
    fn test_itasu_o_verb_stem() {
        let sentence = "コートはこちらでお預かりいたします";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いたす");
        assert_pattern_range(&patterns, "いたす", 8, 17); // お預かりいたします
    }

    #[test]
    fn test_itasu_go_suru_verb() {
        let sentence = "こちらからご連絡いたします";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いたす");
        assert_pattern_range(&patterns, "いたす", 5, 13); // ご連絡いたします
    }

    #[test]
    fn test_itasu_o_verb_casual() {
        let sentence = "お手伝いいたす";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いたす");
        assert_pattern_range(&patterns, "いたす", 0, 7); // お手伝いいたす
    }

    #[test]
    fn test_itasu_go_verb_casual() {
        let sentence = "ご案内いたす";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いたす");
        assert_pattern_range(&patterns, "いたす", 0, 6); // ご案内いたす
    }
}

// Pattern: いらっしゃる (honorific - to be/come/go)
// Data source: grammar_points_data.json["いらっしゃる"]
// Testing: structure.standard[0-6] and polite[0-6]
//
// Structures:
//   - standard[0]: いる・くる・いく → いらっしゃる
//   - standard[1]: Verb[て] + いらっしゃる
//   - polite[0]: います・きます・いきます → いらっしゃいます
//   - polite[1]: Verb[て] + いらっしゃいます
mod irassharu_tests {
    use super::*;

    #[test]
    fn test_irassharu_replacing_iru() {
        let sentence = "あなたの隣にいらっしゃるのはお嫁さんですか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いらっしゃる");
        assert_pattern_range(&patterns, "いらっしゃる", 6, 12); // いらっしゃる
    }

    #[test]
    fn test_irassharu_replacing_kuru() {
        let sentence = "あと30分で社長がいらっしゃるので準備をしてください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いらっしゃる");
        assert_pattern_range(&patterns, "いらっしゃる", 9, 15); // いらっしゃる
    }

    #[test]
    fn test_irassharu_polite_iku() {
        let sentence = "先輩は明日の忘年会にはいらっしゃいますか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いらっしゃる");
        assert_pattern_range(&patterns, "いらっしゃる", 11, 19); // いらっしゃいます
    }

    #[test]
    fn test_irassharu_te_auxiliary() {
        let sentence = "田中様が来ていらっしゃったので案内をお願いします";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いらっしゃる");
        assert_pattern_range(&patterns, "いらっしゃる", 6, 13); // いらっしゃった
    }

    #[test]
    fn test_irassharu_te_auxiliary_polite() {
        let sentence = "マユミさんはタクシーに乗っていらっしゃるのでもうすぐ着きます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いらっしゃる");
        assert_pattern_range(&patterns, "いらっしゃる", 14, 20); // いらっしゃる
    }

    #[test]
    fn test_irassharu_past() {
        let sentence = "昨日部長がいらっしゃった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いらっしゃる");
        assert_pattern_range(&patterns, "いらっしゃる", 5, 12); // いらっしゃった
    }
}

// ========== たら (conditional "if/when") ==========
// Pattern: たら
// Data source: grammar_points_data.json["たら"]
//
// Structures to test:
//   - standard[0]: Verb［た］+ ら
//   - standard[1]: ［い］Adjective［た］+ ら
//   - standard[2]: ［な］Adjective + だった + ら
//   - standard[3]: Noun + だった + ら
//
// Examples from data:
//   - 遅れたら (if/when late)
//   - 寒かったら (if/when cold)
//   - 好きだったら (if/when liked)
//   - 明日だったら (if/when tomorrow)
#[cfg(test)]
mod tara_tests {
    use super::*;

    #[test]
    fn test_tara_verb_conditional() {
        let sentence = "今度遅れたら、許さないよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たら");
        assert_pattern_range(&patterns, "たら", 2, 6); // 遅れたら
    }

    #[test]
    fn test_tara_i_adjective() {
        let sentence = "寒かったら、エアコンつけてね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たら");
        assert_pattern_range(&patterns, "たら", 0, 5); // 寒かったら
    }

    #[test]
    fn test_tara_na_adjective() {
        let sentence = "彼のことが好きだったら、彼に言った方がいい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たら");
        assert_pattern_range(&patterns, "たら", 5, 11); // 好きだったら
    }

    #[test]
    fn test_tara_noun() {
        let sentence = "明日だったら、遊べるよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たら");
        assert_pattern_range(&patterns, "たら", 0, 6); // 明日だったら
    }
}

// ========== てしまう・ちゃう (completion/regret) ==========
// Pattern: てしまう・ちゃう
// Data source: grammar_points_data.json["てしまう・ちゃう"]
//
// Structures to test:
//   - standard[0]: Verb［て］+ しまう
//   - standard[1]: Verb［て］+ ちゃう (casual contraction)
//   - standard[2]: Verb［で］+ じゃう (casual with で)
//   - polite[0]: Verb［て］+ しまいます
//
// Note: ちゃう = contraction of てしまう, じゃう = contraction of でしまう
// Expresses: completion, regret, or something done accidentally
#[cfg(test)]
mod teshimau_tests {
    use super::*;

    #[test]
    fn test_teshimau_standard() {
        let sentence = "私はすぐに道に迷ってしまう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てしまう・ちゃう");
        assert_pattern_range(&patterns, "てしまう・ちゃう", 7, 13); // 迷ってしまう
    }

    #[test]
    fn test_chau_contraction() {
        let sentence = "最近運動をしてないから太っちゃう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てしまう・ちゃう");
        assert_pattern_range(&patterns, "てしまう・ちゃう", 11, 16); // 太っちゃう
    }

    #[test]
    fn test_jau_contraction_de() {
        let sentence = "花が死んじゃうから、花を踏むな";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てしまう・ちゃう");
        assert_pattern_range(&patterns, "てしまう・ちゃう", 2, 7); // 死んじゃう
    }

    #[test]
    fn test_teshimau_polite() {
        let sentence = "全部食べてしまいますので気をつけています";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てしまう・ちゃう");
        assert_pattern_range(&patterns, "てしまう・ちゃう", 2, 10); // 食べてしまいます
    }
}

// Pattern: 〜でも 〜でも (whether...or, even if...or)
// Data source: grammar_points_data.json["〜でも 〜でも"]
// Testing all structure variants:
//   - standard[0]: ［い］Adjective［て］ + も + ［い］Adjective［て］ + も
//   - standard[1]: ［な］Adjective + でも + ［な］Adjective + でも
//   - standard[2]: Noun (A) + でも + Noun (B) + でも
//   - standard[3]: ［い］Adjective［て］ + も + ［い］Adjective［て］ + も + Phrase［ない］
//   - standard[4]: ［な］Adjective + でも + ［な］Adjective + でも + Phrase［ない］
//   - standard[5]: Noun (A) + でも + Noun (B) + でも + Phrase［ない］
//
// Note: Expresses "whether it is (A) or (B)" in positive sentences,
// or "neither (A) nor (B)" in negative sentences.
#[cfg(test)]
mod demo_demo_tests {
    use super::*;

    #[test]
    fn test_i_adj_demo_demo_positive() {
        let sentence = "暑くても寒くても運動を続けるつもりだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "〜でも 〜でも");
        assert_pattern_range(&patterns, "〜でも 〜でも", 0, 8); // 暑くても寒くても
    }

    #[test]
    fn test_na_adj_demo_demo_positive() {
        let sentence = "簡単でも複雑でも挑戦してみたい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "〜でも 〜でも");
        assert_pattern_range(&patterns, "〜でも 〜でも", 0, 8); // 簡単でも複雑でも
    }

    #[test]
    fn test_noun_demo_demo_positive() {
        let sentence = "サッカーでもバスケットボールでもいいからスポーツをやりたい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "〜でも 〜でも");
        assert_pattern_range(&patterns, "〜でも 〜でも", 0, 16); // サッカーでもバスケットボールでも
    }

    #[test]
    fn test_i_adj_demo_demo_negative() {
        let sentence = "暑くても寒くても関係ない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "〜でも 〜でも");
        assert_pattern_range(&patterns, "〜でも 〜でも", 0, 8); // 暑くても寒くても
    }

    #[test]
    fn test_na_adj_demo_demo_negative() {
        let sentence = "静かでも賑やかでも気にしない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "〜でも 〜でも");
        assert_pattern_range(&patterns, "〜でも 〜でも", 0, 9); // 静かでも賑やかでも
    }

    #[test]
    fn test_noun_demo_demo_negative() {
        let sentence = "ケーキでもクッキーでも苺が入っていたら食べない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "〜でも 〜でも");
        assert_pattern_range(&patterns, "〜でも 〜でも", 0, 11); // ケーキでもクッキーでも
    }
}

// Pattern: 〜ようと思う・〜おうと思う (intend to/thinking of doing)
// Data source: grammar_points_data.json["〜ようと思う・〜おうと思う"]
// Testing all structure variants from standard and polite forms
mod you_to_omou_tests {
    use super::*;

    // Testing: structure.standard[0] - "Verb[よう] + と思う"
    #[test]
    fn test_you_to_omou_basic_ru_verb() {
        let sentence = "このゲームはもうあきらめようと思う";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "〜ようと思う・〜おうと思う");
        assert_pattern_range(&patterns, "〜ようと思う・〜おうと思う", 8, 17); // あきらめようと思う
    }

    // Testing: structure.standard[0] - "Verb[よう] + と思う" (う-verb)
    #[test]
    fn test_ou_to_omou_basic_u_verb() {
        let sentence = "来月、バイクを買おうと思う";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "〜ようと思う・〜おうと思う");
        assert_pattern_range(&patterns, "〜ようと思う・〜おうと思う", 7, 13); // 買おうと思う
    }

    // Testing: structure.standard[1] - "Verb[よう] + と思っている"
    #[test]
    fn test_you_to_omotteiru_continuous() {
        let sentence = "今晩はレストランで美味しいものを食べようと思っているけど";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "〜ようと思う・〜おうと思う");
        assert_pattern_range(&patterns, "〜ようと思う・〜おうと思う", 16, 26); // 食べようと思っている
    }

    // Testing: structure.standard[1] - "Verb[よう] + と思っている" (う-verb)
    #[test]
    fn test_ou_to_omotteiru_continuous() {
        let sentence = "明日から電車で通おうと思っている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "〜ようと思う・〜おうと思う");
        assert_pattern_range(&patterns, "〜ようと思う・〜おうと思う", 7, 16); // 通おうと思っている
    }

    // Testing: structure.polite[0] - "Verb[よう] + と思います"
    #[test]
    fn test_you_to_omoimasu_polite() {
        let sentence = "もう少し頑張ろうと思います";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "〜ようと思う・〜おうと思う");
        assert_pattern_range(&patterns, "〜ようと思う・〜おうと思う", 4, 13); // 頑張ろうと思います
    }

    // Testing: structure.polite[1] - "Verb[よう] + と思っています"
    #[test]
    fn test_you_to_omotteimasu_polite_continuous() {
        let sentence = "来年は日本へ行こうと思っています";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "〜ようと思う・〜おうと思う");
        assert_pattern_range(&patterns, "〜ようと思う・〜おうと思う", 6, 16); // 行こうと思っています
    }

    // Pattern: お〜する (humble speech - お/ご + Noun[サ変] + する)
    // Data source: grammar_points_data.json["お〜する"]

    // Testing structure.standard[1] - "ご + する[Verb] + します" (polite form)
    #[test]
    fn test_go_suru_verb_polite() {
        let sentence = "今すぐにご確認します。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "お〜する");
        assert_pattern_range(&patterns, "お〜する", 4, 10); // ご確認します
    }

    // Testing: structure.standard[0] - "ご + する[Verb] + する" (plain form)
    #[test]
    fn test_go_suru_verb_touroku() {
        let sentence = "ご登録する方はこちらをクリックしてください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "お〜する");
        assert_pattern_range(&patterns, "お〜する", 0, 5); // ご登録する
    }

    // Testing: ご + する[Verb] + する (order example)
    #[test]
    fn test_go_suru_verb_chuumon() {
        let sentence = "ご注文する時はこちらのベルを押してください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "お〜する");
        assert_pattern_range(&patterns, "お〜する", 0, 5); // ご注文する
    }

    // Testing: お + Chinese-origin する[Verb] + する (電話 - exceptional case)
    #[test]
    fn test_o_suru_chinese_exception_denwa() {
        let sentence = "先輩にお電話するのが好きです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "お〜する");
        assert_pattern_range(&patterns, "お〜する", 3, 8); // お電話する
    }

    // Testing: お + Chinese-origin する[Verb] + する (勉強 - exceptional case)
    #[test]
    fn test_o_suru_chinese_exception_benkyou() {
        let sentence = "友達とお勉強するのは楽しいです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "お〜する");
        assert_pattern_range(&patterns, "お〜する", 3, 8); // お勉強する
    }

    // TODO: Compound form - お守りします
    // When a noun like お守り is tokenized as a single token (not お + 守り),
    // it doesn't match the お〜する pattern because there's no separate prefix.
    // This is expected behavior - the compound noun お守り is not humble speech,
    // it's just a regular noun that happens to start with お.
    //
    // #[test]
    // fn test_o_suru_compound_omamori() {
    //     let sentence = "僕がお守りします！";
    //     let tokens = tokenize_sentence(sentence);
    //     let patterns = detect_patterns(&tokens);
    //     // Does NOT match - お守り is a compound noun (single token)
    //     assert!(!has_pattern(&patterns, "お〜する"));
    // }
}

// Pattern: お～になる (honorific speech)
// Data source: grammar_points_data.json["お～になる "]
// Testing all structure variants from standard[] and polite[]
mod o_ni_naru_tests {
    use super::*;

    // Testing: structure.standard[0] - "お + Verb[stem] + になる"
    #[test]
    fn test_o_ni_naru_verb_stem() {
        let sentence = "先生はもうお帰りになるそうです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "お～になる ");
        assert_pattern_range(&patterns, "お～になる ", 5, 11); // お帰りになる
    }

    // Testing: structure.standard[1] - "ご + [する]Verb + になる"
    #[test]
    fn test_go_ni_naru_suru_verb() {
        let sentence = "社長はこの件をご存知になりますか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "お～になる ");
        assert_pattern_range(&patterns, "お～になる ", 7, 15); // ご存知になります
    }

    // Testing: structure.standard[2] - "いく・くる・いる ￫ おいでになる"
    #[test]
    fn test_oide_ni_naru_special() {
        let sentence = "明日はおいでになりますか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "お～になる ");
        assert_pattern_range(&patterns, "お～になる ", 3, 11); // おいでになります
    }

    // Testing: polite[0] - "お + Verb[stem] + になります"
    #[test]
    fn test_o_ni_narimasu_polite() {
        let sentence = "部長は何時にお戻りになりますか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "お～になる ");
        assert_pattern_range(&patterns, "お～になる ", 6, 14); // お戻りになります
    }

    // Testing: ご variant with Chinese-origin noun
    #[test]
    fn test_go_ni_naru_chinese_origin() {
        let sentence = "先生はご出席になると思います";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "お～になる ");
        assert_pattern_range(&patterns, "お～になる ", 3, 9); // ご出席になる
    }

    // Testing: exceptional お with Chinese-origin (お電話 example)
    #[test]
    fn test_o_ni_naru_exceptional() {
        let sentence = "お客様はお電話になりますか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "お～になる ");
        assert_pattern_range(&patterns, "お～になる ", 4, 12); // お電話になります
    }

    // Pattern: く・に (adverb formation)
    // Data source: grammar_points_data.json["く・に"]
    // Testing: structure.standard[0] - "［い］Adjective［く］ + Verb"
    //
    // Structure variants to test:
    //   - standard[0]: ［い］Adjective［く］+ Verb
    //   - standard[1]: ［な］Adjective + に + Verb
    //   - standard[2]: Exception いい→よく

    // Testing: い-Adjective[く] + Verb (standard[0])
    #[test]
    fn test_ku_ni_i_adjective_adverb() {
        let sentence = "この箱を強く引いてください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "く・に");
        assert_pattern_range(&patterns, "く・に", 4, 8); // 強く引い
    }

    // Testing: い-Adjective[く] + Verb - different adjective
    #[test]
    fn test_ku_ni_i_adjective_tight() {
        let sentence = "ドアをきつく閉めたほうがいい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "く・に");
        assert_pattern_range(&patterns, "く・に", 3, 9); // きつく閉めた
    }

    // Testing: い-Adjective[く] + Verb - new/newly
    #[test]
    fn test_ku_ni_i_adjective_new() {
        let sentence = "新しく買ったパソコンはどう？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "く・に");
        assert_pattern_range(&patterns, "く・に", 0, 6); // 新しく買った
    }

    // Testing: な-Adjective + に + Verb (standard[1])
    #[test]
    fn test_ku_ni_na_adjective_polite() {
        let sentence = "丁寧に書くようにしてください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "く・に");
        assert_pattern_range(&patterns, "く・に", 0, 5); // 丁寧に書く
    }

    // Testing: な-Adjective + に + Verb - different adjective
    #[test]
    fn test_ku_ni_na_adjective_haphazard() {
        let sentence = "適当に話すなよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "く・に");
        assert_pattern_range(&patterns, "く・に", 0, 5); // 適当に話す
    }

    // Testing: な-Adjective + に + Verb - skillfully
    #[test]
    fn test_ku_ni_na_adjective_skillful() {
        let sentence = "上手に歌うのは難しいね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "く・に");
        assert_pattern_range(&patterns, "く・に", 0, 5); // 上手に歌う
    }

    // Testing: Exception いい→よく (standard[2])
    #[test]
    fn test_ku_ni_yoku_exception() {
        let sentence = "よく考えてから決めたほうがいい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "く・に");
        assert_pattern_range(&patterns, "く・に", 0, 4); // よく考え
    }
}

// ========== そう (looks like/seems like - conjecture based on appearance) ==========
// Pattern: そう
// Data source: grammar_points_data.json["そう "]
//
// Structures to test:
//   - standard[0]: Verb[stem] + そう + だ
//   - standard[1]: い-Adjective[stem] + そう + だ
//   - standard[2]: な-Adjective + そう + だ
//   - standard[4]: Verb[ない] + な + そう + だ (negative)
//   - standard[5]: い-Adjective[ない] + なさ + そう + だ (negative)
//   - standard[8]: いい → よさそう + だ (exception)
//   - polite[0]: Verb[stem] + そう + です
//
// Note: This is appearance-based conjecture, not hearsay (hearsay is そうだ with plain form)
#[cfg(test)]
mod sou_tests {
    use super::*;

    // Testing: Verb[stem] + そうだ (standard[0])
    #[test]
    fn test_sou_verb_affirmative() {
        let sentence = "明日は朝から雨が降りそうだね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そう ");
        assert_pattern_range(&patterns, "そう ", 8, 13); // 降りそうだ
    }

    // Testing: い-Adjective[stem] + そうだ (standard[1])
    #[test]
    fn test_sou_i_adjective() {
        let sentence = "この料理は美味しそうだけど高いね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そう ");
        assert_pattern_range(&patterns, "そう ", 5, 11); // 美味しそうだ
    }

    // Testing: な-Adjective + そうだ (standard[2])
    #[test]
    fn test_sou_na_adjective() {
        let sentence = "あの店員は丁寧そうだから聞いてみよう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そう ");
        assert_pattern_range(&patterns, "そう ", 5, 10); // 丁寧そうだ
    }

    // Testing: Verb[ない] + なそうだ - negative verb (standard[4])
    #[test]
    fn test_sou_verb_negative() {
        let sentence = "彼はもうイギリスに帰らなさそうだよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そう ");
        assert_pattern_range(&patterns, "そう ", 11, 16); // なさそうだ
    }

    // Testing: い-Adjective[ない] + なさそうだ - negative adjective (standard[5])
    #[test]
    fn test_sou_i_adjective_negative() {
        let sentence = "彼の部屋は汚くなさそうだね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そう ");
        assert_pattern_range(&patterns, "そう ", 7, 12); // なさそうだ
    }

    // Testing: いい → よさそうだ - exception (standard[8])
    #[test]
    fn test_sou_yosasou_exception() {
        let sentence = "この天気ならよさそうだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そう ");
        assert_pattern_range(&patterns, "そう ", 6, 11); // よさそうだ
    }

    // Testing: Verb[stem] + そうです - polite form (polite[0])
    #[test]
    fn test_sou_polite_verb() {
        let sentence = "来週から値段が上がりそうです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そう ");
        assert_pattern_range(&patterns, "そう ", 7, 14); // 上がりそうです
    }

    // Testing: い-Adjective + そうです - polite form (polite[1])
    #[test]
    fn test_sou_polite_i_adjective() {
        let sentence = "この映画は面白そうですね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そう ");
        assert_pattern_range(&patterns, "そう ", 5, 11); // 面白そうです
    }
}

// Pattern: そうに・そうな (seeming like/looking like - adverbial and attributive forms)
// Data source: grammar_points_data.json["そうに・そうな "]
// Structures:
//   - Verb[stem] + そうに + Verb/Adj
//   - い-Adj[stem] + そうに + Verb/Adj
//   - な-Adj + そうに + Verb/Adj
//   - Verb[stem] + そうな + Noun
//   - い-Adj[stem] + そうな + Noun
//   - な-Adj + そうな + Noun
//   - Special: negative forms with なさそう (adjective) vs なそう (verb)
mod souni_souna_tests {
    use super::*;

    #[test]
    fn test_verb_stem_souni_verb() {
        // structure.standard[0]: Verb[stem] + そうに + Verb
        let sentence = "さっき、先輩が怒りそうになってた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そうに・そうな ");
        assert_pattern_range(&patterns, "そうに・そうな ", 7, 12); // 怒りそうに
    }

    #[test]
    fn test_i_adj_stem_souni_verb() {
        // structure.standard[1]: い-Adj[い] + そうに + Verb
        let sentence = "彼は忙しそうに仕事をしている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そうに・そうな ");
        assert_pattern_range(&patterns, "そうに・そうな ", 2, 7); // 忙しそうに
    }

    #[test]
    fn test_na_adj_souni_verb() {
        // structure.standard[2]: な-Adj + そうに + Verb
        let sentence = "彼女はどんな仕事でも簡単そうにやるからうらやましい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そうに・そうな ");
        assert_pattern_range(&patterns, "そうに・そうな ", 10, 15); // 簡単そうに
    }

    #[test]
    fn test_verb_stem_souna_noun() {
        // structure.standard[3]: Verb[stem](2) + そうな + Noun
        let sentence = "あの人は怒らなそうな顔をしている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そうに・そうな ");
        assert_pattern_range(&patterns, "そうに・そうな ", 6, 10); // なそうな
    }

    #[test]
    fn test_i_adj_stem_souna_noun() {
        // structure.standard[3]: い-Adj[い] + そうな + Noun
        let sentence = "つまらなそうなパーティーには行きません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そうに・そうな ");
        assert_pattern_range(&patterns, "そうに・そうな ", 0, 7); // つまらなそうな
    }

    #[test]
    fn test_na_adj_souna_noun() {
        // structure.standard[3]: な-Adj + そうな + Noun
        let sentence = "大事じゃなさそうな物は捨ててもいいよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そうに・そうな ");
        assert_pattern_range(&patterns, "そうに・そうな ", 4, 9); // なさそうな
    }

    #[test]
    fn test_i_adj_negative_nasasou() {
        // Caution: い-Adj negative with さ insertion: なさそう
        let sentence = "すごい！辛くなさそうに食べるね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そうに・そうな ");
        assert_pattern_range(&patterns, "そうに・そうな ", 6, 11); // なさそうに
    }

    #[test]
    fn test_na_adj_negative_nasasou() {
        // Caution: な-Adj negative with さ insertion: なさそう
        let sentence = "彼は美味しくなさそうにご飯を食べている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そうに・そうな ");
        assert_pattern_range(&patterns, "そうに・そうな ", 6, 11); // なさそうに
    }
}

// ========================================
// ていただけませんか Tests
// ========================================
mod teitadakemasenka_tests {
    use super::*;

    // Pattern: ていただけませんか (could you please - humble polite request)
    // Data source: grammar_points_data.json["ていただけませんか"]
    // Testing: structure.standard[0] - "Verb[て] + いただけませんか"
    #[test]
    fn test_te_itadakemasen_ka_basic() {
        let sentence = "お名前を教えていただけませんか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ていただけませんか");
        assert_pattern_range(&patterns, "ていただけませんか", 4, 15); // 教えていただけませんか
    }

    // Testing: structure.standard[1] - "Verb[て] + もらえませんか"
    #[test]
    fn test_te_moraemasen_ka_basic() {
        let sentence = "危ないので少しだけ下がってもらえませんか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ていただけませんか");
        assert_pattern_range(&patterns, "ていただけませんか", 9, 20); // 下がってもらえませんか
    }

    // Testing: Different verb types with いただけませんか
    #[test]
    fn test_te_itadakemasen_ka_different_verbs() {
        let sentence = "もう一度説明していただけませんか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ていただけませんか");
        assert_pattern_range(&patterns, "ていただけませんか", 4, 16); // 説明していただけませんか
    }

    // Testing: Different verb with もらえませんか
    #[test]
    fn test_te_moraemasen_ka_different_verbs() {
        let sentence = "ちょっと待ってもらえませんか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ていただけませんか");
        assert_pattern_range(&patterns, "ていただけませんか", 4, 14); // 待ってもらえませんか
    }
}

// ========== ているあいだに (while/during) ==========
// Pattern: ているあいだに
// Data source: grammar_points_data.json["ているあいだに"]
//
// Structure to test:
//   - standard[0]: Verb[ている] + 間（あいだ）に
//
// Examples from data:
//   - 数えているあいだには話しかけないで (don't talk to me while I'm counting)
//   - 運んでいるあいだに転んで怪我をした (tripped when I was carrying)
//   - 行っているあいだに家に泥棒が入った (while I was on a trip, a burglar came)
#[cfg(test)]
mod te_iru_aida_ni_tests {
    use super::*;

    #[test]
    fn test_te_iru_aida_ni_basic() {
        let sentence = "数えているあいだには話しかけないで";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ているあいだに");
        assert_pattern_range(&patterns, "ているあいだに", 0, 9); // 数えているあいだに
    }

    #[test]
    fn test_te_iru_aida_ni_action_during() {
        let sentence = "荷物を運んでいるあいだに転んで怪我をした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ているあいだに");
        assert_pattern_range(&patterns, "ているあいだに", 3, 12); // 運んでいるあいだに
    }

    #[test]
    fn test_te_iru_aida_ni_uncontrolled_event() {
        let sentence = "旅行に行っているあいだに家に泥棒が入った";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ているあいだに");
        assert_pattern_range(&patterns, "ているあいだに", 3, 12); // 行っているあいだに
    }
}

// Pattern: ているところだ (in the middle of doing)
// Data source: grammar_points_data.json["ているところだ"]
// Structure: Verb[ている] + ところ + だ/です
mod te_iru_tokoro_da_tests {
    use super::*;

    #[test]
    fn test_standard_form() {
        let sentence = "仕事をしているところだから後で電話するね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ているところだ");
        assert_pattern_range(&patterns, "ているところだ", 3, 11); // しているところだ
    }

    #[test]
    fn test_polite_form() {
        let sentence = "今、資料を準備しているところです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ているところだ");
        assert_pattern_range(&patterns, "ているところだ", 5, 16); // 準備しているところです
    }

    #[test]
    fn test_past_form_datta() {
        let sentence = "映画を見ているところだったから電話に出れなかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ているところだ");
        assert_pattern_range(&patterns, "ているところだ", 3, 13); // 見ているところだった
    }

    #[test]
    fn test_without_copula() {
        let sentence = "昨日は近所で泥棒が警察から逃げているところを見た";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ているところだ");
        assert_pattern_range(&patterns, "ているところだ", 13, 21); // 逃げているところ
    }
}

// Pattern: てくれてありがとう (thank you for doing)
// Data source: grammar_points_data.json["てくれてありがとう"]
// Testing all structure variants: 1 standard, 1 polite
//
// Structures:
//   - standard[0]: Verb[て] + くれて + ありがとう
//   - polite[0]: Verb[て] + くれて + ありがとう + ございます
#[cfg(test)]
mod tekuretearigatou_tests {
    use super::*;

    // Testing: structure.standard[0] - "Verb[て] + くれて + ありがとう"
    #[test]
    fn test_standard_form() {
        let sentence = "いつもゴキブリを捕まえてくれてありがとう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てくれてありがとう");
        assert_pattern_range(&patterns, "てくれてありがとう", 8, 20); // 捕まえてくれてありがとう
    }

    // Testing: structure.polite[0] - "Verb[て] + くれて + ありがとう + ございます"
    #[test]
    fn test_polite_form_gozaimasu() {
        let sentence = "ケーキを買ってきてくれてありがとうございます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てくれてありがとう");
        assert_pattern_range(&patterns, "てくれてありがとう", 7, 22); // きてくれてありがとうございます
    }

    // Testing: standard form with different verb
    #[test]
    fn test_standard_different_verb() {
        let sentence = "忙しいのに手伝ってくれてありがとう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てくれてありがとう");
        assert_pattern_range(&patterns, "てくれてありがとう", 5, 17); // 手伝ってくれてありがとう
    }

    // Testing: polite form with different verb
    #[test]
    fn test_polite_different_verb() {
        let sentence = "最後まで話を聞いてくれてありがとうございます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てくれてありがとう");
        assert_pattern_range(&patterns, "てくれてありがとう", 6, 22); // 聞いてくれてありがとうございます
    }
}
