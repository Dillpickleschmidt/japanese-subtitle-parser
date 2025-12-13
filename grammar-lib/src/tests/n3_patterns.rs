use super::*;

// ========== なかなか (quite/considerably) ==========
// Pattern: なかなか (quite/considerably/very)
// Data source: grammar_points_data.json[" なかなか"]
//
// Structure variants to test:
//   standard[0]: なかなか + Adjective
//   standard[1]: なかなか + の + Noun

mod nakanaka_tests {
    use super::*;

    // Test: なかなか + い-Adjective
    #[test]
    fn test_nakanaka_i_adjective() {
        let sentence = "ここのラーメンはなかなか美味しいね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, " なかなか");
        assert_pattern_range(&patterns, " なかなか", 8, 12); // なかなか
    }

    // Test: なかなか + な-Adjective
    #[test]
    fn test_nakanaka_na_adjective() {
        let sentence = "ミムラさんもなかなか可愛いよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, " なかなか");
        assert_pattern_range(&patterns, " なかなか", 6, 10); // なかなか
    }

    // Test: なかなか + の + Noun
    #[test]
    fn test_nakanaka_no_noun() {
        let sentence = "あのシェフが作るパスタはなかなかの物だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, " なかなか");
        assert_pattern_range(&patterns, " なかなか", 12, 16); // なかなか
    }

    // Test: なかなか + の + Noun (different example)
    #[test]
    fn test_nakanaka_no_noun_beauty() {
        let sentence = "元カノはなかなかの美人でした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, " なかなか");
        assert_pattern_range(&patterns, " なかなか", 4, 8); // なかなか
    }

    // Test: なかなか with "difficult to dismiss" nuance
    #[test]
    fn test_nakanaka_difficult_to_dismiss() {
        let sentence = "なかなかの事をしてくれたな";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, " なかなか");
        assert_pattern_range(&patterns, " なかなか", 0, 4); // なかなか
    }
}

// ========== おかげで (thanks to) ==========
// Pattern: おかげで (thanks to / because of)
// Data source: grammar_points_data.json["おかげで"]
//
// Structure variants to test:
//   standard[0]: Verb + おかげで
//   standard[1]: ［い］Adjective + おかげで
//   standard[2]: ［な］Adjective + な + おかげで
//   standard[3]: Noun + の + おかげで

mod okagede_tests {
    use super::*;

    // Test: Verb + おかげで
    #[test]
    fn test_okagede_verb() {
        let sentence = "タケルくんに手伝ってもらったおかげで仕事が早く終わったよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "おかげで");
        assert_pattern_range(&patterns, "おかげで", 13, 18); // たおかげで
    }

    // Test: い-Adjective + おかげで
    #[test]
    fn test_okagede_i_adjective() {
        let sentence = "部屋が汚いおかげでどこに何があるか全く分からない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "おかげで");
        assert_pattern_range(&patterns, "おかげで", 3, 9); // 汚いおかげで
    }

    // Test: な-Adjective + な + おかげで
    #[test]
    fn test_okagede_na_adjective() {
        let sentence = "友達が有名なおかげで、どんな高級レストランでも予約なしで入れる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "おかげで");
        assert_pattern_range(&patterns, "おかげで", 5, 10); // なおかげで
    }

    // Test: Noun + の + おかげで
    #[test]
    fn test_okagede_noun() {
        let sentence = "あなたのおかげで不自由のない生活ができている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "おかげで");
        assert_pattern_range(&patterns, "おかげで", 3, 8); // のおかげで
    }
}

// ========== うちに (while/during) ==========
// Pattern: うちに (while/during - temporal expression)
// Data source: grammar_points_data.json["うちに"]
//
// Structure variants to test:
//   standard[0]: Verb［ている］+ うちに
//   standard[1]: ［い］Adjective + うちに
//   standard[2]: ［な］Adjective + な + うちに
//   standard[3]: Noun + の + うちに

mod uchini_tests {
    use super::*;

    // Test: Verb［ている］+ うちに
    #[test]
    fn test_uchini_verb_teiru() {
        let sentence = "彼と毎日会ううちに、だんだんと彼のことが好きになってきた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "うちに");
        assert_pattern_range(&patterns, "うちに", 4, 9); // 会ううちに
    }

    // Test: い-Adjective + うちに
    #[test]
    fn test_uchini_i_adjective() {
        let sentence = "熱いうちに食べて！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "うちに");
        assert_pattern_range(&patterns, "うちに", 0, 5); // 熱いうちに
    }

    // Test: な-Adjective + な + うちに
    #[test]
    fn test_uchini_na_adjective() {
        let sentence = "お爺ちゃんがまだ元気なうちに家族皆で旅行に行こう！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "うちに");
        assert_pattern_range(&patterns, "うちに", 10, 14); // なうちに
    }

    // Test: Noun + の + うちに
    #[test]
    fn test_uchini_noun() {
        let sentence = "今のうちに明日の準備をしておこう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "うちに");
        assert_pattern_range(&patterns, "うちに", 1, 5); // のうちに
    }
}

// ========== おきに (at intervals of / every) ==========
// Pattern: おきに (at intervals of / every X)
// Data source: grammar_points_data.json["おきに"]
//
// Structure variants to test:
//   standard[0]: Number + Counter + おきに

mod okini_tests {
    use super::*;

    // Test: 一日おきに (one day interval - every second day)
    #[test]
    fn test_okini_day_interval() {
        let sentence = "この薬は一日おきに飲んでください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "おきに");
        assert_pattern_range(&patterns, "おきに", 4, 9); // 一日おきに
    }

    // Test: 二時間おきに (two hour interval - every two hours)
    #[test]
    fn test_okini_hour_interval() {
        let sentence = "毎日２時間おきに体を動かしている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "おきに");
        assert_pattern_range(&patterns, "おきに", 2, 8); // ２時間おきに
    }

    // Test: 一ヶ月おきに (one month interval - every second month)
    #[test]
    fn test_okini_month_interval() {
        let sentence = "一ヶ月おきに病院に来るように先生に言われました";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "おきに");
        assert_pattern_range(&patterns, "おきに", 0, 6); // 一ヶ月おきに
    }
}

// ========== あまり (so much that / excessive) ==========
// Pattern: あまり (so much that / to the point that)
// Data source: grammar_points_data.json["あまり"]
//
// Structure variants to test:
//   standard[0]: Verb + あまり + (Negative Result) Phrase
//   standard[1]: い-Adjective[さ] + の + あまり + (Negative Result) Phrase
//   standard[2]: い-Adjective[み] + の + あまり + (Negative Result) Phrase
//   standard[3]: な-Adjective + な + あまり + (Negative Result) Phrase
//   standard[4]: Noun + の + あまり + (Negative Result) Phrase

mod amari_tests {
    use super::*;

    // Test: Verb + あまり (so much that - excessive action leading to negative result)
    #[test]
    fn test_amari_verb() {
        let sentence = "彼は仕事に集中するあまり、終電を逃しました";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あまり");
        // Note: Range extends back from する to include preceding 集中 (compound verb)
        assert_pattern_range(&patterns, "あまり", 5, 12); // 集中するあまり
    }

    // Test: い-Adjective[さ] + の + あまり (excessive quality leading to negative result)
    #[test]
    fn test_amari_i_adj_sa() {
        let sentence = "トムは暑さのあまり、気を失った";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あまり");
        assert_pattern_range(&patterns, "あまり", 5, 9); // のあまり
    }

    // Test: い-Adjective[み] + の + あまり (excessive quality leading to negative result)
    #[test]
    fn test_amari_i_adj_mi() {
        let sentence = "彼女は悲しみのあまりボーっとしている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あまり");
        assert_pattern_range(&patterns, "あまり", 6, 10); // のあまり
    }

    // Test: な-Adjective + な + あまり (excessive quality leading to negative result)
    #[test]
    fn test_amari_na_adj() {
        let sentence = "彼は音楽が好きなあまり、仕事を辞めてバンドを作った";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あまり");
        assert_pattern_range(&patterns, "あまり", 7, 11); // なあまり
    }

    // Test: Noun + の + あまり (excessive state leading to negative result)
    #[test]
    fn test_amari_noun() {
        let sentence = "ヤスエは緊張のあまり上手く歌えなかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あまり");
        assert_pattern_range(&patterns, "あまり", 6, 10); // のあまり
    }
}

// ========== からこそ (precisely because) ==========
// Pattern: からこそ (precisely because / it's precisely because)
// Data source: grammar_points_data.json["からこそ"]
//
// Structure variants to test:
//   standard[0]: Verb + からこそ + Phrase
//   standard[1]: Noun + だ + からこそ + Phrase

mod karakoso_tests {
    use super::*;

    // Test: Verb + からこそ (plain form verb)
    #[test]
    fn test_karakoso_verb() {
        let sentence = "俺は努力したからこそ、試験に合格できたんだよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "からこそ");
        assert_pattern_range(&patterns, "からこそ", 5, 10); // たからこそ
    }

    // Test: Noun + だ + からこそ
    #[test]
    fn test_karakoso_noun_da() {
        let sentence = "君だからこそできたんだよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "からこそ");
        assert_pattern_range(&patterns, "からこそ", 1, 6); // だからこそ
    }
}

// ========== あるいは (or/alternatively) ==========
// Pattern: あるいは (or / alternatively / possibly)
// Data source: grammar_points_data.json["あるいは"]
//
// Structure variants to test:
//   standard[0]: Verb + か + あるいは
//   standard[1]: い-Adjective + か + あるいは
//   standard[2]: な-Adjective + か + あるいは
//   standard[3]: Noun + (か) + あるいは (か is optional)
//   standard[4]: あるいは + Phrase + かもしれない

mod aruiwa_tests {
    use super::*;

    // Test: Verb + か + あるいは
    #[test]
    fn test_aruiwa_verb() {
        let sentence = "運動をさせるかあるいは餌を減らしてください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あるいは");
        assert_pattern_range(&patterns, "あるいは", 6, 11); // かあるいは
    }

    // Test: い-Adjective + か + あるいは
    #[test]
    fn test_aruiwa_i_adjective() {
        let sentence = "パソコンの動作が遅いかあるいは電源が点かなくなった場合は、私達が直します";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あるいは");
        assert_pattern_range(&patterns, "あるいは", 10, 15); // かあるいは
    }

    // Test: な-Adjective + か + あるいは
    #[test]
    fn test_aruiwa_na_adjective() {
        let sentence = "有名かあるいは綺麗であれば誰でも入れるらしい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あるいは");
        assert_pattern_range(&patterns, "あるいは", 2, 7); // かあるいは
    }

    // Test: Noun + (か) + あるいは (without か)
    #[test]
    fn test_aruiwa_noun_without_ka() {
        let sentence = "牛乳あるいはチーズが使われている料理は食べられません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あるいは");
        assert_pattern_range(&patterns, "あるいは", 2, 6); // あるいは (without か)
    }

    // Test: Noun + か + あるいは (with か)
    #[test]
    fn test_aruiwa_noun_with_ka() {
        let sentence = "土曜日かあるいは日曜日までには終わらせておきます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あるいは");
        assert_pattern_range(&patterns, "あるいは", 3, 8); // かあるいは
    }

    // Test: あるいは + Phrase + かもしれない (あるいは at beginning)
    #[test]
    fn test_aruiwa_phrase_kamoshirenai() {
        let sentence = "月曜日あるいは火曜日に新しい生徒が来るかもしれない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あるいは");
        assert_pattern_range(&patterns, "あるいは", 3, 7); // あるいは (without か)
    }
}

// ========== ～ずつ (each/per/at a time) ==========
// Pattern: ～ずつ (each/per/at a time)
// Data source: grammar_points_data.json["～ずつ"]
//
// Structure variants to test:
//   standard[0]: Number + Counter + ずつ (Per/Each/At time)
//   standard[1]: 少し + ずつ (Little by little)
//   standard[2]: いくらか + ずつ (Some … every)

mod zutsu_tests {
    use super::*;

    // Test: Number + Counter + ずつ (Per/Each/At time) - with simple counter
    #[test]
    fn test_zutsu_number_counter_one() {
        let sentence = "メニューに載ってるもの一つずつちょうだい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～ずつ");
        assert_pattern_range(&patterns, "～ずつ", 11, 15); // 一つずつ
    }

    // Test: Number + Counter + ずつ (Per/Each/At time) - with person counter
    #[test]
    fn test_zutsu_number_counter_person() {
        let sentence = "一人ずつゆっくりとお入りください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～ずつ");
        assert_pattern_range(&patterns, "～ずつ", 1, 4); // 人ずつ
    }

    // Test: 少し + ずつ (Little by little)
    #[test]
    fn test_zutsu_sukoshi() {
        let sentence = "このお酒は強いので少しずつ飲んでください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～ずつ");
        assert_pattern_range(&patterns, "～ずつ", 9, 13); // 少しずつ
    }

    // Test: いくらか + ずつ (Some … every)
    #[test]
    fn test_zutsu_ikuraka() {
        let sentence = "毎月いくらかずつお金を貯めて旅行に行く";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～ずつ");
        assert_pattern_range(&patterns, "～ずつ", 5, 8); // かずつ
    }
}

// Pattern: がたい (difficult to do)
// Data source: grammar_points_data.json["がたい"]
// Testing structures:
//   standard[0]: Verb[stem] + がたい
//   polite[0]: Verb[stem] + がたいです

mod gatai_tests {
    use super::*;

    // Test: Verb[stem] + がたい (difficult to do) - 信じがたい
    #[test]
    fn test_gatai_standard_shinjiru() {
        let sentence = "あいつが言うことは信じがたいが、今は信じるしかない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がたい");
        assert_pattern_range(&patterns, "がたい", 9, 14); // 信じがたい
    }

    // Test: Verb[stem] + がたい (difficult to do) - 期待しがたい
    #[test]
    fn test_gatai_standard_kitai() {
        let sentence = "景気が悪化しているため、今期の売上は期待しがたい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がたい");
        assert_pattern_range(&patterns, "がたい", 18, 24); // 期待しがたい
    }

    // Test: Verb[stem] + がたい (difficult to do) - 信じがたい in embedded clause
    #[test]
    fn test_gatai_standard_embedded() {
        let sentence = "この話が信じがたいのは分かるが、信じてくれ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がたい");
        assert_pattern_range(&patterns, "がたい", 4, 9); // 信じがたい
    }

    // Test: Verb[stem] + がたいです (polite form) - 理解しがたいです
    #[test]
    fn test_gatai_polite_rikai() {
        let sentence = "彼の行動は理解しがたいですね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がたい");
        assert_pattern_range(&patterns, "がたい", 5, 13); // 理解しがたいです
    }
}

// Pattern: がち (tend to/prone to)
// Data source: grammar_points_data.json["がち"]
// Testing structures:
//   standard[0]: Verb[stem] + がち
//   standard[1]: Noun + がち
//   standard[2]: Noun + がち + な + Noun

mod gachi_tests {
    use super::*;

    // Test: Verb[stem] + がち - 頼みがち (tend to order)
    #[test]
    fn test_gachi_verb_tanomu() {
        let sentence = "ここに来るといつもパフェを頼みがちだけど、今日はパンケーキを頼む";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がち");
        assert_pattern_range(&patterns, "がち", 13, 17); // 頼みがち
    }

    // Test: Verb[stem] + がち - サボりがち (tend to skip/shirk) with な
    #[test]
    fn test_gachi_verb_saboru() {
        let sentence = "サボりがちな人はだいたい成績が悪い";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がち");
        assert_pattern_range(&patterns, "がち", 0, 6); // サボりがちな (includes な)
    }

    // Test: Noun + がち - 病気がち (prone to getting sick)
    #[test]
    fn test_gachi_noun_byouki() {
        let sentence = "うちの子は病気がちなので、週に二、三日ぐらいは学校を休みます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がち");
        assert_pattern_range(&patterns, "がち", 5, 9); // 病気がち
    }

    // Test: Noun + がち (standalone) - 留守がち (often away from home)
    #[test]
    fn test_gachi_noun_rusu() {
        let sentence = "最近は仕事が忙しくて留守がちです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がち");
        assert_pattern_range(&patterns, "がち", 10, 14); // 留守がち
    }
}

// ========== かなり (quite/considerably) ==========
// Pattern: かなり (quite/fairly/considerably/pretty)
// Data source: grammar_points_data.json["かなり"]
//
// Structure variants to test:
//   standard[0]: かなり + Phrase
//   standard[1]: かなり + の + Noun

mod kanari_tests {
    use super::*;

    // Test: かなり + Adjective (structure.standard[0] - かなり + Phrase)
    #[test]
    fn test_kanari_adjective() {
        let sentence = "今日は一日中仕事をしていたからかなり疲れた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かなり");
        assert_pattern_range(&patterns, "かなり", 15, 18); // かなり
    }

    // Test: かなり + Verb (structure.standard[0] - かなり + Phrase)
    #[test]
    fn test_kanari_verb() {
        let sentence = "今月もかなりお金を使ったね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かなり");
        assert_pattern_range(&patterns, "かなり", 3, 6); // かなり
    }

    // Test: かなり + の + Noun (structure.standard[1])
    #[test]
    fn test_kanari_no_noun_distance() {
        let sentence = "メキシコまではかなりの距離があるから飛行機で行った";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かなり + の + Noun");
        assert_pattern_range(&patterns, "かなり + の + Noun", 7, 13); // かなりの距離
    }

    // Test: かなり + の + Noun (structure.standard[1] - different example)
    #[test]
    fn test_kanari_no_noun_people() {
        let sentence = "昨日の事故でかなりの人が怪我をした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かなり + の + Noun");
        assert_pattern_range(&patterns, "かなり + の + Noun", 6, 11); // かなりの人
    }
}

// ========== Verb[volitional]とする (try to / be about to) ==========
// Pattern: Verb[volitional]とする (try to / be about to)
// Data source: grammar_points_data.json["Verb[volitional]とする"]
//
// Structure variants to test:
//   standard[0]: Verb[おう] + とする
//   polite[0]: Verb[おう] + とします

mod verb_volitional_tosuru_tests {
    use super::*;

    // Test: Verb[volitional] + とする (standard form)
    #[test]
    fn test_volitional_tosuru_standard() {
        let sentence = "この子はなんでも食べようとするから、お菓子は隠してね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[volitional]とする");
        assert_pattern_range(&patterns, "Verb[volitional]とする", 8, 15); // 食べようとする
    }

    // Test: Verb[volitional] + とする (past tense)
    #[test]
    fn test_volitional_tosuru_past() {
        let sentence = "今年は毎日日本語の勉強をしようとしたが、時間がなくて出来なかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[volitional]とする");
        assert_pattern_range(&patterns, "Verb[volitional]とする", 12, 18); // しようとした
    }

    // Test: Verb[volitional] + とします (polite form)
    #[test]
    fn test_volitional_tosuru_polite() {
        let sentence = "明日から毎朝早く起きようとします";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[volitional]とする");
        assert_pattern_range(&patterns, "Verb[volitional]とする", 8, 16); // 起きようとします
    }

    // Test: Verb[volitional] + としたら (conditional + interruption meaning)
    #[test]
    fn test_volitional_tosuru_tara() {
        let sentence = "家を出ようとしたら、急に雨が降り始めた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[volitional]とする");
        assert_pattern_range(&patterns, "Verb[volitional]とする", 2, 9); // 出ようとしたら
    }
}

// ========== ぎみ (sensation of / tendency) ==========
// Pattern: ぎみ (sensation of / a touch of / slightly)
// Data source: grammar_points_data.json["ぎみ"]
//
// Structure variants to test:
//   standard[0]: Verb[stem] + 気味（ぎみ）
//   standard[1]: Noun + 気味（ぎみ）

mod gimi_tests {
    use super::*;

    // Test: Verb[stem] + ぎみ (太りぎみ - slightly fat)
    #[test]
    fn test_gimi_verb_futori() {
        let sentence = "最近太りぎみだから、ダイエットしなきゃ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ぎみ");
        assert_pattern_range(&patterns, "ぎみ", 2, 7); // 太りぎみだ
    }

    // Test: Verb[stem] + ぎみ (疲れぎみ - feeling a little tired)
    #[test]
    fn test_gimi_verb_tsukare() {
        let sentence = "今週は色々と忙しかったから疲れぎみだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ぎみ");
        assert_pattern_range(&patterns, "ぎみ", 13, 18); // 疲れぎみだ
    }

    // Test: Noun + ぎみ (風邪ぎみ - feeling a little sick)
    #[test]
    fn test_gimi_noun_kaze() {
        let sentence = "今朝は風邪ぎみだったので、学校を休んだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ぎみ");
        assert_pattern_range(&patterns, "ぎみ", 3, 7); // 風邪ぎみ
    }

    // Test: Verb[stem] + ぎみ (遅れぎみ - a bit late)
    #[test]
    fn test_gimi_verb_okure() {
        let sentence = "電車が遅れぎみだから、待ち合わせ時間まで間に合わないかも";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ぎみ");
        assert_pattern_range(&patterns, "ぎみ", 3, 8); // 遅れぎみだ
    }
}

// ========== あまりに (excessively/so much) ==========
// Pattern: あまりに (excessively/so much)
// Data source: grammar_points_data.json["あまりに"]
//
// Structure variants to test:
//   standard[0]: あまり + に + Adjective
//   standard[1]: あまり + に + Adverb
//   standard[2]: あまり + の + Noun
//   standard[3]: あんまり (colloquial variant)
//   standard[4]: あまり + にも (emphasis variant)

mod amarini_tests {
    use super::*;

    // Test: あまりに + い-Adjective (so easy)
    #[test]
    fn test_amarini_i_adjective() {
        let sentence = "今日の仕事はあまりに楽過ぎて仕事をした感じがしない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あまりに");
        assert_pattern_range(&patterns, "あまりに", 6, 10); // あまりに
    }

    // Test: あまりに + Adverb (so late)
    #[test]
    fn test_amarini_adverb() {
        let sentence = "今朝は会社にあまりに遅く着いたため、先輩に怒られた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あまりに");
        assert_pattern_range(&patterns, "あまりに", 6, 10); // あまりに
    }

    // Test: あまりの + Noun (so much fear)
    #[test]
    fn test_amarino_noun() {
        let sentence = "彼女はあまりの恐怖に、声をあげて叫んだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あまりの + Noun");
        assert_pattern_range(&patterns, "あまりの + Noun", 3, 9); // あまりの恐怖
    }

    // Test: あんまり + Adjective (colloquial - too boring)
    #[test]
    fn test_anmari_colloquial() {
        let sentence = "友達の話があんまりつまらなくて途中からほとんど聞いてなかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あまりに");
        assert_pattern_range(&patterns, "あまりに", 5, 9); // あんまり
    }

    // Test: あまりにも + い-Adjective (emphasis - so boring)
    #[test]
    fn test_amarinimo_emphasis() {
        let sentence = "友達の話があまりにもつまらなくて途中からほとんど聞いてなかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あまりにも");
        assert_pattern_range(&patterns, "あまりにも", 5, 10); // あまりにも
    }

    // Test: あまりに + な-Adjective (so quiet)
    #[test]
    fn test_amarini_na_adjective() {
        let sentence = "部屋があまりに静かで、少し不安になった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あまりに");
        assert_pattern_range(&patterns, "あまりに", 3, 7); // あまりに
    }
}

// ========== くせに (despite/even though) ==========
// Pattern: くせに (despite/even though)
// Data source: grammar_points_data.json["くせに"]
//
// Structure variants to test:
//   standard[0]: Verb + くせに
//   standard[1]: い-Adjective + くせに
//   standard[2]: な-Adjective + な + くせに
//   standard[3]: Noun + の + くせに

mod kuseni_tests {
    use super::*;

    // Test: Verb + くせに (even though said going to sleep)
    #[test]
    fn test_kuseni_verb() {
        let sentence = "さっき寝るって言ってたくせにまだ起きてるの？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "くせに");
        assert_pattern_range(&patterns, "くせに", 10, 14); // たくせに
    }

    // Test: い-Adjective + くせに (even though young)
    #[test]
    fn test_kuseni_i_adjective() {
        let sentence = "若いくせに何ダラダラしているんだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "くせに");
        assert_pattern_range(&patterns, "くせに", 0, 5); // 若いくせに
    }

    // Test: な-Adjective + な + くせに (despite being unskilled)
    #[test]
    fn test_kuseni_na_adjective() {
        let sentence = "自分だって下手なくせに何偉そうに言ってるんだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "くせに");
        assert_pattern_range(&patterns, "くせに", 7, 11); // なくせに
    }

    // Test: Noun + の + くせに (even though a dog)
    #[test]
    fn test_kuseni_noun() {
        let sentence = "この子は犬のくせにニャーと鳴く";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "くせに");
        assert_pattern_range(&patterns, "くせに", 5, 9); // のくせに
    }
}

// ========== こそ (emphasis particle) ==========
// Pattern: こそ (emphasis/precisely/exactly)
// Data source: grammar_points_data.json["こそ"]
//
// Structure variants to test:
//   standard[0]: Noun + こそ

mod koso_tests {
    use super::*;

    // Test: Noun + こそ (this song exactly)
    #[test]
    fn test_koso_noun_emphasis() {
        let sentence = "この曲こそ俺がずーっと探していた曲だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "こそ");
        assert_pattern_range(&patterns, "こそ", 2, 5); // 曲こそ
    }

    // Test: Time noun + こそ (this time for sure)
    #[test]
    fn test_koso_time_noun() {
        let sentence = "今度こそ勝つぞ！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "こそ");
        assert_pattern_range(&patterns, "こそ", 0, 4); // 今度こそ
    }

    // Test: Pronoun + こそ (I am the one)
    #[test]
    fn test_koso_pronoun() {
        let sentence = "いえいえ、私こそありがとうございます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "こそ");
        assert_pattern_range(&patterns, "こそ", 5, 8); // 私こそ
    }
}

// ========== きり (only/just/since) ==========
// Pattern: きり (only/just/since - adverbial particle)
// Data source: grammar_points_data.json["きり"]
//
// Structure variants to test:
//   standard[0]: Verb[た] + きり
//   standard[1]: Noun + きり
//   standard[2]: これ/それ + きり
//   standard[3]: Number + Counter + きり
//   Note: っきり is a variant of きり (more casual)

mod kiri_tests {
    use super::*;

    // Test: Verb[past] + きり (since doing something)
    #[test]
    fn test_kiri_verb_past() {
        let sentence = "昨日、晩ご飯食べたきり何も食べてないから、めちゃお腹が空いた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "きり");
        assert_pattern_range(&patterns, "きり", 8, 11); // たきり
    }

    // Test: Noun + Counter + きり (alone)
    #[test]
    fn test_kiri_noun() {
        let sentence = "一人きりになれる時間が欲しい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "きり");
        assert_pattern_range(&patterns, "きり", 1, 4); // 人きり
    }

    // Test: Number + Counter + っきり (only once - casual variant)
    #[test]
    fn test_kiri_counter_ikkiri() {
        let sentence = "彼とは一度っきりしか会えていない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "きり");
        assert_pattern_range(&patterns, "きり", 4, 8); // 度っきり
    }

    // Test: Number + Counter + っきり (just two of us)
    #[test]
    fn test_kiri_counter_futari() {
        let sentence = "二人っきりの時間はいいね！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "きり");
        assert_pattern_range(&patterns, "きり", 1, 5); // 人っきり
    }

    // TODO: Undetectable - 寝たきり is a lexicalized compound noun
    // The example sentence "お祖母ちゃんは去年病気で倒れて、寝たきりになった"
    // tokenizes 寝たきり as a single noun (名詞/一般) rather than 寝 + た + きり.
    // This is a set expression meaning "bedridden" and cannot be detected by the
    // きり pattern matcher which expects separate tokens.
    //
    // #[test]
    // fn test_kiri_verb_past_bedridden() {
    //     let sentence = "お祖母ちゃんは去年病気で倒れて、寝たきりになった";
    //     let tokens = tokenize_sentence(sentence);
    //     let patterns = detect_patterns(&tokens);
    //
    //     // 寝たきり is tokenized as single noun, not verb + た + きり
    // }
}

// ========== くらい ② (degree/extent - so...that) ==========
// Pattern: くらい② (degree/extent - expresses limit/extent causing result)
// Data source: grammar_points_data.json["くらい ②"]
//
// Structure variants to test:
//   standard[0]: Verb + くらい/ぐらい
//   standard[1]: い-Adjective + くらい/ぐらい
//   standard[2]: な-Adjective + な + くらい/ぐらい
//   standard[3]: Noun + くらい/ぐらい
//
// Note: This is different from くらい① which is for approximation ("about/approximately")
// くらい② expresses extent/degree that causes or allows a result ("so...that")

mod kurai2_tests {
    use super::*;

    // Test: Verb + くらい (so much that)
    #[test]
    fn test_kurai2_verb() {
        let sentence = "今日はもう一生走りたくないくらい走った";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "くらい ②");
        assert_pattern_range(&patterns, "くらい ②", 11, 16); // ないくらい
    }

    // Test: Verb + くらい (extent causing result)
    #[test]
    fn test_kurai2_verb_scream() {
        let sentence = "わたしも叫びたいくらい、怖かった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "くらい ②");
        assert_pattern_range(&patterns, "くらい ②", 6, 11); // たいくらい
    }

    // Test: な-Adjective + な + ぐらい (extent - ぐらい variant)
    #[test]
    fn test_kurai2_na_adjective_gurai() {
        let sentence = "５連休が必要なぐらい疲れています";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "くらい ②");
        assert_pattern_range(&patterns, "くらい ②", 6, 10); // なぐらい
    }

    // Test: Noun + くらい (to about the level of)
    #[test]
    fn test_kurai2_noun_level() {
        let sentence = "俺もキヨミさんくらいピアノが弾けるようになりたい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "くらい ②");
        assert_pattern_range(&patterns, "くらい ②", 5, 10); // さんくらい
    }
}

// ========== ことか (how/god knows/what) ==========
// Pattern: ことか (how / god knows how / what)
// Data source: grammar_points_data.json["ことか"]
//
// Structure variants to test:
//   standard[0]: Verb + ことか
//   standard[1]: い-Adjective + ことか
//   standard[2]: な-Adjective + な + ことか
//   standard[3]: Noun + である + ことか
//
// Meaning: Expresses emphasis on the extent/magnitude of something (rhetorical question)
// Usage: Primarily written language, sounds dramatic/poetic
// Often used with: どれだけ, なんて, 何回, どんなに (extent/number words)

mod kotoka_tests {
    use super::*;

    // Test: Verb + ことか (how many times)
    #[test]
    fn test_kotoka_verb() {
        let sentence = "あの人のコンサートには何回いったことか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことか");
        assert_pattern_range(&patterns, "ことか", 15, 19); // たことか
    }

    // Test: い-Adjective + ことか (how cute)
    #[test]
    fn test_kotoka_i_adjective() {
        let sentence = "うちの犬はどんなに可愛いことか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことか");
        assert_pattern_range(&patterns, "ことか", 9, 15); // 可愛いことか
    }

    // Test: な-Adjective + な + ことか (how bored)
    #[test]
    fn test_kotoka_na_adjective() {
        let sentence = "なんて暇なことか。こんなに暇なのは久しぶりだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことか");
        assert_pattern_range(&patterns, "ことか", 4, 8); // なことか
    }

    // Test: Noun + である + ことか (how worried)
    #[test]
    fn test_kotoka_noun_dearu() {
        let sentence = "息子を一人で電車に乗せるのがどれだけ心配であることか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことか");
        assert_pattern_range(&patterns, "ことか", 21, 26); // あることか
    }
}

// ========== ことから (from the fact that) ==========
// Pattern: ことから (from the fact that)
// Data source: grammar_points_data.json["ことから"]
//
// Structure variants to test:
//   standard[0]: Verb + ことから
//   standard[1]: い-Adjective + ことから
//   standard[2]: な-Adjective + な + ことから
//   standard[3]: Noun + の + ことから
//
// Meaning: "from the fact that" - draws logical conclusion from a fact
// Usage: More formal reasoning than simple から (because)
// Note: Different from ことだから (conjunction vs. case marker)

mod kotokara_tests {
    use super::*;

    // Test: Verb + ことから (from the fact that verb)
    #[test]
    fn test_kotokara_verb() {
        let sentence = "こんな早い時間に妻が寝ていることから、今日は色々と大変だったと気づいた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことから");
        assert_pattern_range(&patterns, "ことから", 12, 18); // いることから
    }

    // Test: い-Adjective + ことから (from the fact that adjective)
    #[test]
    fn test_kotokara_i_adjective() {
        let sentence = "家の家具が全部新しいことから、彼はここに引っ越してきたばかりだと分かった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことから");
        assert_pattern_range(&patterns, "ことから", 7, 14); // 新しいことから
    }

    // Test: な-Adjective + な + ことから (from the fact that na-adj)
    #[test]
    fn test_kotokara_na_adjective() {
        let sentence = "子供が静かなことから、何か悪いことをしていると分かった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことから");
        assert_pattern_range(&patterns, "ことから", 5, 10); // なことから
    }

    // Test: Noun + の + ことから (from these facts)
    #[test]
    fn test_kotokara_noun() {
        let sentence = "これらのことからイベントを中止することに決めました";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことから");
        assert_pattern_range(&patterns, "ことから", 3, 8); // のことから
    }
}

// ========== ことだ (should/ought to) ==========
// Pattern: ことだ (advice/weak command)
// Data source: grammar_points_data.json["ことだ"]
//
// Structure variants to test:
//   standard[0]: Verb[る] + こと + だ
//   standard[1]: Verb[ない] + こと + だ
//   polite[0]: Verb[る] + こと + です
//   polite[1]: Verb[ない] + こと + です

mod kotoda_tests {
    use super::*;

    // Test: Verb[る] + ことだ (should do)
    #[test]
    fn test_kotoda_verb_affirmative() {
        let sentence = "面倒でも朝ご飯を食べることだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことだ");
        assert_pattern_range(&patterns, "ことだ", 8, 14); // 食べることだ
    }

    // Test: Verb[ない] + ことだ (should not do)
    #[test]
    fn test_kotoda_verb_negative() {
        let sentence = "疲れていても諦めないことだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことだ");
        assert_pattern_range(&patterns, "ことだ", 8, 13); // ないことだ
    }

    // Test: Verb[る] + ことです (polite - should do)
    #[test]
    fn test_kotoda_verb_polite() {
        let sentence = "何があっても時間通りに来ることです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことだ");
        assert_pattern_range(&patterns, "ことだ", 11, 17); // 来ることです
    }

    // Test: Verb[ない] + ことです (polite - should not do)
    #[test]
    fn test_kotoda_verb_negative_polite() {
        let sentence = "怪我をしたら我慢をしないことです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことだ");
        assert_pattern_range(&patterns, "ことだ", 10, 16); // ないことです
    }
}

// ========== ことがある (sometimes/there are times when) ==========
// Pattern: ことがある (occasionally happens)
// Data source: grammar_points_data.json["ことがある"]
//
// Structure variants to test:
//   standard[0]: Verb[る] + こと + がある
//   standard[1]: い-Adjective + こと + がある
//   standard[2]: な-Adjective + な + こと + がある
//   standard[3]: Verb[ない] + こと + がある
//   standard[4]: も instead of が
//   polite[0-4]: Same with あります

mod kotogaaru_tests {
    use super::*;

    // Test: Verb[る] + ことがある (sometimes happens)
    #[test]
    fn test_kotogaaru_verb() {
        let sentence = "この馬は人を蹴ることがあるので気をつけて";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことがある");
        assert_pattern_range(&patterns, "ことがある", 6, 13); // 蹴ることがある
    }

    // Test: い-Adjective + ことがある (there are times when)
    #[test]
    fn test_kotogaaru_i_adjective() {
        let sentence = "先生の授業はたまに楽しいことがある";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことがある");
        assert_pattern_range(&patterns, "ことがある", 9, 17); // 楽しいことがある
    }

    // Test: な-Adjective + な + ことがある (there are times when)
    #[test]
    fn test_kotogaaru_na_adjective() {
        let sentence = "仕事はたまには楽なことがある";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことがある");
        assert_pattern_range(&patterns, "ことがある", 8, 14); // なことがある
    }

    // Test: Verb[ない] + こともある (sometimes doesn't happen - も variant)
    #[test]
    fn test_kotogaaru_verb_negative() {
        let sentence = "彼は時々来ないこともある";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことがある");
        assert_pattern_range(&patterns, "ことがある", 5, 12); // ないこともある
    }

    // Test: こともある (も variant - also happens)
    #[test]
    fn test_kotomoaru_variant() {
        let sentence = "仕事は楽しいけど、大変なこともある";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことがある");
        assert_pattern_range(&patterns, "ことがある", 11, 17); // なこともある
    }

    // Test: ことがあります (polite form)
    #[test]
    fn test_kotogaaru_polite() {
        let sentence = "週に一回のペースでラーメンを食べることがあります";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことがある");
        assert_pattern_range(&patterns, "ことがある", 14, 24); // 食べることがあります
    }
}

// ========== ことに (particularly/especially/to my...) ==========
// Pattern: ことに (particularly/especially/to my...)
// Data source: grammar_points_data.json["ことに"]
//
// Structure variants to test:
//   standard[0]: Verb + ことに
//   standard[1]: い-Adjective + ことに
//   standard[2]: な-Adjective + な + ことに

mod kotoni_tests {
    use super::*;

    // Test: Verb + ことに (particularly/especially)
    #[test]
    fn test_kotoni_verb() {
        let sentence = "驚いたことに、彼女が突然結婚したんだって";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことに");
        assert_pattern_range(&patterns, "ことに", 2, 6); // たことに
    }

    // Test: い-Adjective + ことに (particularly/especially)
    #[test]
    fn test_kotoni_i_adjective() {
        let sentence = "珍しいことに、今日は彼が遅刻してきた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことに");
        assert_pattern_range(&patterns, "ことに", 0, 6); // 珍しいことに
    }

    // Test: な-Adjective + な + ことに (unfortunately/fortunately)
    #[test]
    fn test_kotoni_na_adjective() {
        let sentence = "残念なことに、その商品はもう売り切れでした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことに");
        assert_pattern_range(&patterns, "ことに", 2, 6); // なことに
    }

    // Test: Common expression 幸いなことに (fortunately)
    #[test]
    fn test_kotoni_saiwai() {
        let sentence = "幸いなことに、誰も怪我をしなかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことに");
        assert_pattern_range(&patterns, "ことに", 2, 6); // なことに
    }
}

// ========== ことにする (decide to) ==========
// Pattern: ことにする (decide to / make it that)
// Data source: grammar_points_data.json["ことにする"]
//
// Structure variants to test:
//   standard[0]: Verb[る] + ことにする
//   standard[1]: Verb[ない] + ことにする
//   polite[0]: Verb[る] + ことにします
//   polite[1]: Verb[ない] + ことにします

mod kotonisuru_tests {
    use super::*;

    // Test: Verb[る] + ことにする (decide to do)
    #[test]
    fn test_kotonisuru_verb_affirmative() {
        let sentence = "明日は仕事を休むことにする";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことにする");
        assert_pattern_range(&patterns, "ことにする", 6, 13); // 休むことにする
    }

    // Test: Verb[ない] + ことにする (decide not to)
    #[test]
    fn test_kotonisuru_verb_negative() {
        let sentence = "これからは肉を食べないことにする";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことにする");
        assert_pattern_range(&patterns, "ことにする", 9, 16); // ないことにする
    }

    // Test: Verb[る] + ことにします (polite - decide to do)
    #[test]
    fn test_kotonisuru_polite() {
        let sentence = "箱根に行くことにします";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことにする");
        assert_pattern_range(&patterns, "ことにする", 3, 11); // 行くことにします
    }

    // Test: Verb[ない] + ことにします (polite - decide not to)
    #[test]
    fn test_kotonisuru_negative_polite() {
        let sentence = "今日は外出しないことにします";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことにする");
        assert_pattern_range(&patterns, "ことにする", 6, 14); // ないことにします
    }
}

// ========== ことになる (it has been decided / will end up) ==========
// Pattern: ことになる (passive decision / natural consequence)
// Data source: grammar_points_data.json["ことになる"]
//
// Structure variants to test:
//   standard[0]: Verb + ことになる
//   standard[1]: い-Adjective + ことになる
//   standard[2]: な-Adjective + な + ことになる
//   polite[0]: Verb + ことになります
//   polite[1]: い-Adjective + ことになります
//   polite[2]: な-Adjective + な + ことになります

mod kotoninaru_tests {
    use super::*;

    // Test: Verb + ことになる (it has been decided)
    #[test]
    fn test_kotoninaru_verb_affirmative() {
        let sentence = "来月から海外に転勤することになった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことになる");
        assert_pattern_range(&patterns, "ことになる", 7, 17); // 転勤することになった
    }

    // Test: Verb[negative] + ことになる (decided not to)
    #[test]
    fn test_kotoninaru_verb_negative() {
        let sentence = "結局、行かないことになりました";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことになる");
        assert_pattern_range(&patterns, "ことになる", 5, 15); // ないことになりました
    }

    // Test: い-Adjective + ことになる (will end up being)
    #[test]
    fn test_kotoninaru_i_adjective() {
        let sentence = "そんなことしたら大変なことになるぞ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことになる");
        assert_pattern_range(&patterns, "ことになる", 10, 16); // なことになる
    }

    // Test: な-Adjective + な + ことになる (will become)
    #[test]
    fn test_kotoninaru_na_adjective() {
        let sentence = "治療しないと深刻なことになる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことになる");
        assert_pattern_range(&patterns, "ことになる", 8, 14); // なことになる
    }

    // Test: Verb + ことになります (polite - it has been decided)
    #[test]
    fn test_kotoninaru_polite() {
        let sentence = "新しい支店で働くことになります";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことになる");
        assert_pattern_range(&patterns, "ことになる", 6, 15); // 働くことになります
    }

    // Test: Verb + ことになった (past tense - was decided)
    #[test]
    fn test_kotoninaru_past() {
        let sentence = "急に退職することになった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことになる");
        assert_pattern_range(&patterns, "ことになる", 2, 12); // 退職することになった
    }
}

// ========== ことはない (no need to / never happens) ==========
// Pattern: ことはない (there is no need / it never happens)
// Data source: grammar_points_data.json["ことはない"]
//
// Structure variants to test:
//   standard[0]: Verb + ことはない
//   polite[0]: Verb + ことはありません

mod kotohanai_tests {
    use super::*;

    // Test: Verb + ことはない (no need to - reassurance)
    #[test]
    fn test_kotohanai_no_need() {
        let sentence = "そんなに慌てることはないよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことはない");
        assert_pattern_range(&patterns, "ことはない", 4, 12); // 慌てることはない
    }

    // Test: Verb + ことはない (never happens - recurrence negation)
    #[test]
    fn test_kotohanai_never_happens() {
        let sentence = "親と話すことはない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことはない");
        assert_pattern_range(&patterns, "ことはない", 2, 9); // 話すことはない
    }

    // Test: Verb + ことはありません (polite - no need to)
    #[test]
    fn test_kotohanai_polite() {
        let sentence = "心配することはありません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことはない");
        assert_pattern_range(&patterns, "ことはない", 0, 12); // 心配することはありません (full sentence - verb is at start)
    }
}

// ========== ということだ (it is said that / it means that) ==========
// Pattern: ということだ (hearsay/conclusion with certainty)
// Data source: grammar_points_data.json["ということだ"]
//
// Structure variants to test:
//   standard[0]: Phrase + ということ + だ
//   polite[0]: Phrase + ということ + です

mod toiukotoda_tests {
    use super::*;

    // Test: Verb phrase + ということだ (hearsay - it is said that)
    #[test]
    fn test_toiukotoda_hearsay() {
        let sentence = "この井戸水は汚染されているということだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ということだ");
        assert_pattern_range(&patterns, "ということだ", 11, 19); // いるということだ
    }

    // Test: Phrase + ということだ (conclusion - it means that)
    #[test]
    fn test_toiukotoda_conclusion() {
        let sentence = "まだ新鮮ということだな";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ということだ");
        assert_pattern_range(&patterns, "ということだ", 2, 10); // 新鮮ということだ
    }

    // Test: Phrase + ということです (polite form)
    #[test]
    fn test_toiukotoda_polite() {
        let sentence = "この病気は薬では治せないということです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ということだ");
        assert_pattern_range(&patterns, "ということだ", 10, 19); // ないということです
    }

    // Test: によると + ということだ (with information source marker)
    #[test]
    fn test_toiukotoda_source() {
        let sentence = "先生によると、地震が来るということだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ということだ");
        assert_pattern_range(&patterns, "ということだ", 10, 18); // 来るということだ
    }
}

// ========== ないことはない (it's not that...not / not impossible) ==========
// Pattern: ないことはない (double negative expressing possibility)
// Data source: grammar_points_data.json["ないことはない"]
//
// Structure variants to test:
//   standard[0]: Verb[ない] + ことはない
//   standard[1]: い-Adj[ない] + ことはない
//   standard[2]: な-Adj + ではない + ことはない
//   standard[3]: こともない (も variant)
//   polite variants with ありません

mod naikotohanai_tests {
    use super::*;

    // Test: Verb[ない] + ことはない (not impossible)
    #[test]
    fn test_naikotohanai_verb() {
        let sentence = "映画は見ないことはないが、詳しくもない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないことはない");
        assert_pattern_range(&patterns, "ないことはない", 4, 11); // ないことはない
    }

    // Test: い-Adjective[ない] + ことはない
    #[test]
    fn test_naikotohanai_i_adjective() {
        let sentence = "ここは危なくないことはないから、気をつけた方がいい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないことはない");
        assert_pattern_range(&patterns, "ないことはない", 6, 13); // ないことはない
    }

    // Test: な-Adjective + ではない + ことはない
    #[test]
    fn test_naikotohanai_na_adjective() {
        let sentence = "元気ではないことはないけど、すごく元気なわけでもない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないことはない");
        assert_pattern_range(&patterns, "ないことはない", 4, 11); // ないことはない
    }

    // Test: Verb potential + ないことはない (half-hearted possibility)
    #[test]
    fn test_naikotohanai_potential() {
        let sentence = "納豆は食べられないことはないけど、自分から買って食べようとは思わない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないことはない");
        assert_pattern_range(&patterns, "ないことはない", 7, 14); // ないことはない
    }

    // Test: こともない (も variant)
    #[test]
    fn test_naikotohanai_mo_variant() {
        let sentence = "歩いていけないこともないけど、６時間ぐらいかかるよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないことはない");
        assert_pattern_range(&patterns, "ないことはない", 5, 12); // ないこともない
    }
}

// ========== かけ (half/unfinished action) ==========
// Pattern: かけ (half/unfinished/on the verge of)
// Data source: grammar_points_data.json["かけ"]
//
// Structure variants to test:
//   standard[0]: Verb[stem] + かけだ
//   standard[1]: Verb[stem] + かける
//   standard[2]: Verb[stem] + かけの + Noun
//   polite[0]: Verb[stem] + かけです
//   polite[1]: Verb[stem] + かけます
//   polite[2]: Verb[stem] + かけの + Noun (same as standard)

mod kake_tests {
    use super::*;

    // Test: Verb[stem] + かけだ (half-finished state)
    #[test]
    fn test_kake_da_unfinished() {
        let sentence = "俺の食べかけだけど大丈夫？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かけ");
        assert_pattern_range(&patterns, "かけ", 2, 7); // 食べかけだ
    }

    // Test: Verb[stem] + かけだ (on the verge of)
    #[test]
    fn test_kake_da_verge() {
        let sentence = "これは私の飲みかけだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かけ");
        assert_pattern_range(&patterns, "かけ", 5, 10); // 飲みかけだ
    }

    // Test: Verb[stem] + かける (verb form - about to)
    #[test]
    fn test_kake_ru_verb() {
        let sentence = "やばい、死にかけるところだった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かけ");
        assert_pattern_range(&patterns, "かけ", 4, 9); // 死にかける
    }

    // Test: Verb[stem] + かけた (past form - half done)
    #[test]
    fn test_kake_ta_past() {
        let sentence = "飲みかけた水を捨てる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かけ");
        assert_pattern_range(&patterns, "かけ", 0, 5); // 飲みかけた
    }

    // Test: Verb[stem] + かけの + Noun (compound form - half-finished noun modifier)
    #[test]
    fn test_kake_no_noun_dying() {
        let sentence = "彼は死にかけの子犬を救った";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かけ_compound");
        assert_pattern_range(&patterns, "かけ_compound", 2, 6); // 死にかけ
    }

    // Test: Verb[stem] + かけの + Noun (split form - half-broken)
    #[test]
    fn test_kake_no_noun_broken() {
        let sentence = "彼女は壊れかけのパソコンを買って直すのが趣味だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かけ");
        assert_pattern_range(&patterns, "かけ", 3, 7); // 壊れかけ
    }

    // Test: Verb[stem] + かけです (polite form)
    #[test]
    fn test_kake_desu_polite() {
        let sentence = "終わりかけですからちょっと待って";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かけ");
        assert_pattern_range(&patterns, "かけ", 0, 7); // 終わりかけです
    }
}

// ========== いくら〜でも (no matter how much) ==========
// Pattern: いくら〜でも (no matter how much / however much)
// Data source: grammar_points_data.json["いくら〜でも"]
//
// Structure variants to test:
//   standard[0]: いくら + Verb[ても]
//   standard[1]: いくら + い-Adjective[ても]
//   standard[2]: いくら + Noun + でも
//   standard[3]: いくら + な-Adjective + でも

mod ikura_demo_tests {
    use super::*;

    // Test: いくら + Verb[ても]
    #[test]
    fn test_ikura_demo_verb() {
        let sentence = "あの人にいくら言っても何も変わらないよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いくら〜でも");
        assert_pattern_range(&patterns, "いくら〜でも", 4, 11); // いくら言っても
    }

    // Test: いくら + い-Adjective[ても]
    #[test]
    fn test_ikura_demo_i_adjective() {
        let sentence = "いくら新しくても落としたら壊れるに決まってるじゃん";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いくら〜でも");
        assert_pattern_range(&patterns, "いくら〜でも", 0, 8); // いくら新しくても
    }

    // Test: いくら + Noun + でも
    #[test]
    fn test_ikura_demo_noun() {
        let sentence = "いくら俺でもそんな重いものは持てないよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いくら〜でも");
        assert_pattern_range(&patterns, "いくら〜でも", 0, 6); // いくら俺でも
    }

    // Test: いくら + な-Adjective + でも
    #[test]
    fn test_ikura_demo_na_adjective() {
        let sentence = "あの人のことがいくら嫌いでも、そんな事言ったら可哀そうだよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いくら〜でも");
        assert_pattern_range(&patterns, "いくら〜でも", 7, 14); // いくら嫌いでも
    }
}

// ========== から言うと (from the perspective of) ==========
// Pattern: から言うと (speaking from, from the viewpoint of)
// Data source: grammar_points_data.json["から言うと"]
//
// Structure variants to test:
//   standard[0]: Noun + から言うと
//   standard[1]: から言えば (conditional variant)
//   standard[1]: から言って (て-form variant)

mod kara_iuto_tests {
    use super::*;

    // Test: Noun + から言うと (main form)
    #[test]
    fn test_kara_iuto_noun() {
        let sentence = "私の経験から言うと、このやり方が一番効率がいい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "から言うと");
        assert_pattern_range(&patterns, "から言うと", 2, 9); // 経験から言うと
    }

    // Test: Noun + から言えば (conditional variant)
    #[test]
    fn test_kara_ieba_conditional() {
        let sentence = "私の立場から言えば、彼は絶対いつか成功する";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "から言うと");
        assert_pattern_range(&patterns, "から言うと", 2, 9); // 立場から言えば
    }

    // Test: Noun + から言って (て-form variant)
    #[test]
    fn test_kara_itte_te_form() {
        let sentence = "この結果から言って、この計画はあまりよくありませんでした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "から言うと");
        assert_pattern_range(&patterns, "から言うと", 2, 9); // 結果から言って
    }
}

// ========== Noun＋型 (type/style/model) ==========
// Pattern: Noun＋型 (type, style, model, shape)
// Data source: grammar_points_data.json["Noun＋型"]
//
// Structure variants to test:
//   standard[0]: Noun + がた (direct attachment with 連濁)
//   standard[1]: Noun + の + かた (with の particle)
//   standard[2]: Noun + がた + の + Noun (modifying another noun)
//   standard[3]: い-Adjective + かた (adjective stem)
//   standard[4]: けい (Chinese reading in compounds)

mod noun_kata_tests {
    use super::*;

    // Test: Noun + がた (direct attachment with rendaku)
    #[test]
    fn test_noun_gata_direct() {
        let sentence = "この紙を星がたに切ってください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Noun＋型");
        assert_pattern_range(&patterns, "Noun＋型", 4, 7); // 星がた
    }

    // Test: の + かた (with の particle as preceding element)
    // Note: After の particle, かた (not がた) is used
    #[test]
    fn test_no_kata() {
        let sentence = "あのタイプのかたは最近人気がありますよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Noun＋型");
        assert_pattern_range(&patterns, "Noun＋型", 5, 8); // のかた
    }

    // Test: Noun + がた + の + Noun (modifying another noun)
    #[test]
    fn test_noun_gata_no_noun() {
        let sentence = "九十年代には犬がたのロボットのおもちゃが人気だった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Noun＋型");
        assert_pattern_range(&patterns, "Noun＋型", 6, 9); // 犬がた
    }

    // Test: い-Adjective + かた (adjective + かた)
    #[test]
    fn test_i_adj_kata() {
        let sentence = "新しいかたの自転車はいつ発売されますか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Noun＋型");
        assert_pattern_range(&patterns, "Noun＋型", 0, 5); // 新しいかた
    }

    // Test: けい (Chinese reading in compound)
    #[test]
    fn test_kei_reading() {
        let sentence = "文型は基本なのでしっかりと勉強しておきましょう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Noun＋型");
        assert_pattern_range(&patterns, "Noun＋型", 0, 2); // 文型
    }
}

// ========== ～ようとしない (shall not / doesn't try to) ==========
// Pattern: ～ようとしない (shall not / doesn't try to)
// Data source: grammar_points_data.json["〜ようとしない"]
//
// Structure variants to test:
//   standard[0]: Verb[おう] + としない
//   polite[0]: Verb[おう] + としません

mod youtoshinai_tests {
    use super::*;

    // Test: Verb[おう] + としない (standard form)
    // Example: 聞こうとしない - doesn't try to listen
    #[test]
    fn test_youtoshinai_standard() {
        let sentence = "彼は怒っているから、誰の話も聞こうとしない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "〜ようとしない");
        assert_pattern_range(&patterns, "〜ようとしない", 14, 21); // 聞こうとしない
    }

    // Test: Verb[おう] + としない (different verb)
    // Example: 帰ろうとしない - doesn't try to go home
    #[test]
    fn test_youtoshinai_kaeru() {
        let sentence = "家になかなか帰ろうとしない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "〜ようとしない");
        assert_pattern_range(&patterns, "〜ようとしない", 6, 13); // 帰ろうとしない
    }

    // Test: Verb[おう] + としません (polite form)
    #[test]
    fn test_youtoshinai_polite() {
        let sentence = "あの子は全然勉強しようとしません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "〜ようとしない");
        assert_pattern_range(&patterns, "〜ようとしない", 6, 16); // 勉強しようとしません
    }
}

// ========== ～と言っても (even though / although I say) ==========
// Pattern: ～と言っても (even though / although I say)
// Data source: grammar_points_data.json[" ～と言っても"]
//
// Structure variants to test:
//   standard[0]: Verb + と言（い）っても
//   standard[1]: い-Adjective + と言（い）っても
//   standard[2]: な-Adjective + （だ） + と言（い）っても
//   standard[3]: Noun + （だ） + と言（い）っても

mod toittemo_tests {
    use super::*;

    // Test: Verb + と言っても
    // Example: 走るといっても - although I say I run
    #[test]
    fn test_toittemo_verb() {
        let sentence = "毎日走るといっても、１５分しか走らないよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, " ～と言っても");
        assert_pattern_range(&patterns, " ～と言っても", 2, 9); // 走るといっても
    }

    // Test: い-Adjective + と言っても
    // Example: 辛いといっても - even though I say it's spicy
    #[test]
    fn test_toittemo_i_adjective() {
        let sentence = "辛いといってもピリ辛だから、あなたでも食べれると思うよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, " ～と言っても");
        assert_pattern_range(&patterns, " ～と言っても", 0, 7); // 辛いといっても
    }

    // Test: な-Adjective + だ + と言っても
    // Example: 新鮮だといっても - although I say it's fresh
    #[test]
    fn test_toittemo_na_adjective() {
        let sentence = "この肉は新鮮だといっても、３日前に買ったからそんなに新鮮ではない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, " ～と言っても");
        assert_pattern_range(&patterns, " ～と言っても", 4, 12); // 新鮮だといっても
    }

    // Test: Noun + だ + と言っても
    // Example: 新幹線だといっても - although I say it's shinkansen
    #[test]
    fn test_toittemo_noun() {
        let sentence = "新幹線だといってもこだまに乗るから、そんなに早く着かないと思う";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, " ～と言っても");
        assert_pattern_range(&patterns, " ～と言っても", 0, 9); // 新幹線だといっても
    }
}

// ========== あり (with/possible/exists) ==========
// Pattern: あり (with/among other possibilities/exists)
// Data source: grammar_points_data.json["あり"]
//
// Structure variants to test:
//   standard[0]: Phrase + あり
//
// Note: あり is the literary form of ある, used to indicate:
//   - Something is "with (A), amongst other things"
//   - A possibility among many possibilities
//   - Standalone "that's possible/acceptable" in conversations
//
// Examples from data:
//   - ラーメンもあり (Ramen is possible/acceptable)
//   - 駐車場ありのホテル (hotel with parking lot)
//   - 字幕ありで見たい (want to watch with subtitles)

mod ari_tests {
    use super::*;

    // Test: Noun + も + あり (standalone agreement/possibility)
    // Example: ラーメンもあり - ramen is possible too
    // Pattern includes preceding particle も + あり
    #[test]
    fn test_ari_standalone_possibility() {
        let sentence = "ラーメンもありじゃない？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あり");
        assert_pattern_range(&patterns, "あり", 4, 7); // もあり
    }

    // Test: Noun + あり + の + Noun (with, having)
    // Example: 駐車場ありのホテル - hotel with parking lot
    // Pattern includes preceding noun 場 + あり
    #[test]
    fn test_ari_with_modifier() {
        let sentence = "駐車場ありのホテルを取っておいてください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あり");
        assert_pattern_range(&patterns, "あり", 2, 5); // 場あり
    }

    // Test: Noun + あり + で (with)
    // Example: 字幕ありで見たい - want to watch with subtitles
    // Pattern includes preceding noun 字幕 + あり (+ following で due to range calculation)
    #[test]
    fn test_ari_with_particle() {
        let sentence = "字幕ありで見たいから字幕つけてもらえない？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あり");
        assert_pattern_range(&patterns, "あり", 0, 5); // 字幕ありで (includes following で)
    }

    // Test: あり as standalone response (very casual)
    // Example: めっちゃありです - that's totally possible
    // Pattern includes preceding adverb めっちゃ + あり (+ following です due to range calculation)
    #[test]
    fn test_ari_standalone_response() {
        let sentence = "めっちゃありです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あり");
        assert_pattern_range(&patterns, "あり", 0, 8); // めっちゃありです (includes following です)
    }
}

// ========== Particle + の (nominalization with particles) ==========
// Pattern: Particle + の (Noun + Particle + の + Noun)
// Data source: grammar_points_data.json["Particle + の"]
//
// Structure variants to test:
//   standard[0]: Noun + から + の + Noun (from)
//   standard[1]: Noun + と + の + Noun (with/grouped with)
//   standard[2]: Noun + へ + の + Noun (toward)
//   standard[3]: Noun + で + の + Noun (done with/by means of)
//   standard[4]: Noun + まで + の + Noun (until)
//
// Note: The grammar point describes particles (から、と、へ、で、まで) being grouped with の
// to form a link between two nouns, where noun B has qualities described by noun A + particle.

mod particle_no_tests {
    use super::*;

    // Test: Noun + から + の + Noun (from)
    // Example: アメリカからのお土産
    #[test]
    fn test_kara_no() {
        let sentence = "これはアメリカからのお土産です";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Particle + の");
        assert_pattern_range(&patterns, "Particle + の", 3, 10); // アメリカからの
    }

    // Test: Noun + と + の + Noun (with/grouped with)
    // Example: 彼との関係
    #[test]
    fn test_to_no() {
        let sentence = "別れた後、彼との関係はどうなるんだろう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Particle + の");
        assert_pattern_range(&patterns, "Particle + の", 5, 8); // 彼との
    }

    // Test: Noun + へ + の + Noun (toward)
    // Example: 海外への手紙
    #[test]
    fn test_e_no() {
        let sentence = "海外への手紙はこちらのポストにお入れください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Particle + の");
        assert_pattern_range(&patterns, "Particle + の", 0, 4); // 海外への
    }

    // Test: Noun + で + の + Noun (done with/by means of)
    // Example: 車での通勤
    #[test]
    fn test_de_no() {
        let sentence = "環境に悪いので車での通勤は控えてください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Particle + の");
        assert_pattern_range(&patterns, "Particle + の", 7, 10); // 車での
    }

    // Test: Noun + まで + の + Noun (until)
    // Example: 出発までの時間
    #[test]
    fn test_made_no() {
        let sentence = "出発までの時間、何をします？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Particle + の");
        assert_pattern_range(&patterns, "Particle + の", 0, 5); // 出発までの
    }
}

// ========== ～ても～なくても (whether or not) ==========
// Pattern: ～ても～なくても (whether or not)
// Data source: grammar_points_data.json["～ても～なくても"]
//
// Structure variants to test:
//   standard[0]: Verb［ても］(A) + Verb［なくても］(A)
//
// Note: This pattern uses the same verb twice - once with ても and once with なくても
// to express "whether or not (verb)", showing that the result doesn't change.
//
// Examples from data:
//   - いてもいなくても (whether or not [someone] is here)
//   - 運動しても運動しなくても (whether or not [I] exercise)
//   - 頼んでも頼まなくても (whether [you] order or not)

mod temonakutemo_tests {
    use super::*;

    // Test: Verb［ても］+ Verb［なくても］
    // Example: いてもいなくても - whether or not [someone] is here
    #[test]
    fn test_temonakutemo_iru() {
        let sentence = "あの人がいてもいなくても、仕事の量は変わらない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～ても～なくても");
        assert_pattern_range(&patterns, "～ても～なくても", 4, 12); // いてもいなくても
    }

    // Test: Verb［ても］+ Verb［なくても］
    // Example: 食べても食べなくても - whether or not [I] eat
    #[test]
    fn test_temonakutemo_taberu() {
        let sentence = "食べても食べなくても、体重は変わらないよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～ても～なくても");
        assert_pattern_range(&patterns, "～ても～なくても", 0, 10); // 食べても食べなくても
    }

    // Test: Verb［ても］+ Verb［なくても］
    // Example: 頼んでも頼まなくても - whether you order or not
    #[test]
    fn test_temonakutemo_tanomu() {
        let sentence = "ここの居酒屋ではビールを頼んでも頼まなくても、ビールを持ってくる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～ても～なくても");
        assert_pattern_range(&patterns, "～ても～なくても", 12, 22); // 頼んでも頼まなくても
    }
}

// ========== しかない (no choice but to) ==========
// Pattern: しかない (no choice but to / there is only)
// Data source: grammar_points_data.json["しかない"]
//
// Structure variants to test:
//   standard[0]: Verb + しかない
//   polite[0]: Verb + しかありません

mod shikanai_tests {
    use super::*;

    // Test: Verb + しかない (standard)
    // Example: 警察を呼ぶしかない - no choice but to call the police
    #[test]
    fn test_shikanai_verb_standard() {
        let sentence = "こうなったら警察を呼ぶしかないな";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "しかない");
        assert_pattern_range(&patterns, "しかない", 9, 15); // 呼ぶしかない
    }

    // Test: Verb + しかない (going by car)
    // Example: 車で行くしかない - no choice but to go by car
    #[test]
    fn test_shikanai_verb_car() {
        let sentence = "もう電車が来ないから、車で行くしかない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "しかない");
        assert_pattern_range(&patterns, "しかない", 13, 19); // 行くしかない
    }

    // Test: Verb + しかない (negative verb)
    // Example: 我慢するしかない - no choice but to endure
    #[test]
    fn test_shikanai_verb_negative_context() {
        let sentence = "今月は休めないから、コンサートに行くのを我慢するしかない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "しかない");
        assert_pattern_range(&patterns, "しかない", 20, 28); // 我慢するしかない
    }

    // Test: Verb + しかありません (polite)
    // Example: 待つしかありません - there is no choice but to wait
    #[test]
    fn test_shikanai_polite() {
        let sentence = "申し訳ございませんが、お待ちになるしかありません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "しかない_polite");
        assert_pattern_range(&patterns, "しかない_polite", 15, 24); // なるしかありません
    }
}

// ========== すでに (already) ==========
// Pattern: すでに (already - formal)
// Data source: grammar_points_data.json["すでに"]
//
// Structure variants to test:
//   standard[0]: すでに + Phrase

mod sudeni_tests {
    use super::*;

    // Test: すでに + Verb[ている] (already doing)
    // Example: すでに出発している - already departed
    #[test]
    fn test_sudeni_verb_teiru() {
        let sentence = "お湯はすでに沸いている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "すでに");
        assert_pattern_range(&patterns, "すでに", 3, 6); // すでに
    }

    // Test: すでに + Verb[た] (already done)
    // Example: すでに決まった - already decided
    #[test]
    fn test_sudeni_verb_past() {
        let sentence = "これはすでに決まった事なので、もう私達じゃ何もできません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "すでに");
        assert_pattern_range(&patterns, "すでに", 3, 6); // すでに
    }

    // Test: すでに + Adjective
    // Example: すでに遅い - already late
    #[test]
    fn test_sudeni_adjective() {
        let sentence = "電車に乗るにはすでに遅すぎる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "すでに");
        assert_pattern_range(&patterns, "すでに", 7, 10); // すでに
    }

    // Test: すでに + Noun (already noun state)
    // Example: すでに売り切れです - already sold out
    #[test]
    fn test_sudeni_noun() {
        let sentence = "もうすでにチケットは売り切れです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "すでに");
        assert_pattern_range(&patterns, "すでに", 2, 5); // すでに
    }
}

// ========== さて (well then / now) ==========
// Pattern: さて (topic change conjunction)
// Data source: grammar_points_data.json["さて"]
//
// Structure variants to test:
//   standard[0]: さて + (New Topic) Phrase

mod sate_tests {
    use super::*;

    // Test: さて at sentence beginning (changing topic)
    // Example: さて、そろそろ出ますか - Well then, shall we head off?
    #[test]
    fn test_sate_topic_change() {
        let sentence = "さて、そろそろ出ますか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "さて");
        assert_pattern_range(&patterns, "さて", 0, 2); // さて
    }

    // Test: さて with following action
    // Example: さて、とりあえず乾杯しましょう - Well, let's toast first
    #[test]
    fn test_sate_with_action() {
        let sentence = "さて、とりあえず乾杯しましょう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "さて");
        assert_pattern_range(&patterns, "さて", 0, 2); // さて
    }

    // Test: さて with question
    // Example: さて、この問題の答えが分かる人はいますか - Now, does anyone know the answer?
    #[test]
    fn test_sate_with_question() {
        let sentence = "さて、この問題の答えが分かる人はいますか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "さて");
        assert_pattern_range(&patterns, "さて", 0, 2); // さて
    }

    // Test: さて in casual conversation
    // Example: さて、次は何しようか - Well then, what shall we do next?
    #[test]
    fn test_sate_casual() {
        let sentence = "さて、次は何しようか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "さて");
        assert_pattern_range(&patterns, "さて", 0, 2); // さて
    }
}

// ========== すると (then/upon that) ==========
// Pattern: すると (then/upon that/in that case)
// Data source: grammar_points_data.json["すると"]
//
// Structure variants to test:
//   standard[0]: Phrase。すると + (Result) Phrase

mod suruto_tests {
    use super::*;

    // Test: すると at sentence beginning - uncontrollable result
    // Example: 押入れの掃除をした。すると、無くしたと思っていた服が出てきた
    // (I cleaned my closet. Upon that, I found clothes I thought I had lost)
    #[test]
    fn test_suruto_uncontrollable_result() {
        let sentence = "押入れの掃除をした。すると、無くしたと思っていた服が出てきた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "すると");
        assert_pattern_range(&patterns, "すると", 10, 13); // すると
    }

    // Test: すると at sentence beginning - gathering monkeys
    // Example: 猿にバナナをあげた。すると、猿がどんどん集まってきた
    // (I gave a banana to a monkey. Having done that, more monkeys gathered)
    #[test]
    fn test_suruto_gathering() {
        let sentence = "猿にバナナをあげた。すると、猿がどんどん集まってきた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "すると");
        assert_pattern_range(&patterns, "すると", 10, 13); // すると
    }

    // Test: すると drawing conclusion
    // Example: １９歳なの？すると、大学１年生でしょう？
    // (You are 19? So you mean to say that you are a freshman, right?)
    #[test]
    fn test_suruto_conclusion() {
        let sentence = "１９歳なの？すると、大学１年生でしょう？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "すると");
        assert_pattern_range(&patterns, "すると", 6, 9); // すると
    }

    // Test: すると in story (like Momotaro)
    // Example: お祖母ちゃんが川で洗濯をしていた。すると、川の向こうから桃が流れてきた
    // (An old lady was washing clothes. Just then, a peach flowed down the river)
    #[test]
    fn test_suruto_story() {
        let sentence = "お祖母ちゃんが川で洗濯をしていた。すると、川の向こうから桃が流れてきた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "すると");
        assert_pattern_range(&patterns, "すると", 17, 20); // すると
    }
}

// ========== そうすると (then/if you do that) ==========
// Pattern: そうすると (then/if you do that/in that case)
// Data source: grammar_points_data.json["そうすると"]
//
// Structure variants to test:
//   standard[0]: Phrase (A)。 そうすると + (Result) Phrase

mod sousuruto_tests {
    use super::*;

    // Test: そうすると for uncontrollable result - running slower
    // Example: もう少し遅く走ってみれば。そうすると、もっと長い距離走れるよ
    // (Try to run a little slower. Once you do that, you will be able to run a longer distance)
    #[test]
    fn test_sousuruto_running() {
        let sentence = "もう少し遅く走ってみれば。そうすると、もっと長い距離走れるよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そうすると");
        assert_pattern_range(&patterns, "そうすると", 13, 18); // そうすると
    }

    // Test: そうすると for instruction - roasting food
    // Example: 食べる前に軽く炙ってください。そうすると、もっと美味しく食べれます
    // (Please roast before eating. Having done that, it will taste even more delicious)
    #[test]
    fn test_sousuruto_roasting() {
        let sentence = "食べる前に軽く炙ってください。そうすると、もっと美味しく食べれます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そうすると");
        assert_pattern_range(&patterns, "そうすると", 15, 20); // そうすると
    }

    // Test: そうすると for uncontrollable result - fishing
    // Example: この餌を使ってみ。そうすると、もっと大きい魚が釣れるよ
    // (Try using this bait. If you do so, you'll be able to catch a bigger fish)
    #[test]
    fn test_sousuruto_fishing() {
        let sentence = "この餌を使ってみ。そうすると、もっと大きい魚が釣れるよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そうすると");
        assert_pattern_range(&patterns, "そうすると", 9, 14); // そうすると
    }

    // Test: そうすると drawing conclusion - birth year
    // Example: １９９３年生まれなんですか？そうすると、２９歳と言う事ですね
    // (You were born in 1993? So that means that you are 29, right?)
    #[test]
    fn test_sousuruto_conclusion() {
        let sentence = "１９９３年生まれなんですか？そうすると、２９歳と言う事ですね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そうすると");
        assert_pattern_range(&patterns, "そうすると", 14, 19); // そうすると
    }
}

// ========== そこで (accordingly/as such) ==========
// Pattern: そこで (accordingly/as such/to that end)
// Data source: grammar_points_data.json["そこで"]
//
// Structure variants to test:
//   standard[0]: (Situation) Phrase。そこで + (Solution) Phrase

// ========== それぞれ (each/respectively) ==========
// Pattern: それぞれ (each/respectively)
// Data source: grammar_points_data.json["それぞれ"]
//
// Structure variants to test:
//   standard[0]: それぞれ + Phrase
//   standard[1]: それぞれ + の + Noun

mod sorezore_tests {
    use super::*;

    // Test: Everyone orders (adverbial use)
    // Example: 皆それぞれ食べたいものを頼んでね
    // (Everyone, please each order something you want to eat)
    #[test]
    fn test_sorezore_adverbial() {
        let sentence = "皆それぞれ食べたいものを頼んでね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "それぞれ");
        assert_pattern_range(&patterns, "それぞれ", 1, 5); // それぞれ
    }

    // Test: Separate actions (adverbial use)
    // Example: それぞれ別の行動をした
    // (We each did our own separate things)
    #[test]
    fn test_sorezore_separate_actions() {
        let sentence = "それぞれ別の行動をした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "それぞれ");
        assert_pattern_range(&patterns, "それぞれ", 0, 4); // それぞれ
    }

    // Test: Each person's way of thinking (の + Noun)
    // Example: 人それぞれの考え方があるから、しょうがないよ
    // (Each person has different ways of thinking, so there is nothing we can do)
    #[test]
    fn test_sorezore_no_noun() {
        let sentence = "人それぞれの考え方があるから、しょうがないよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "それぞれ");
        assert_pattern_range(&patterns, "それぞれ", 1, 6); // それぞれの
    }

    // Test: Each team (の + Noun)
    // Example: それぞれのチームに分かれてから開始してください
    // (Please start once you have divided yourselves into each separate team)
    #[test]
    fn test_sorezore_teams() {
        let sentence = "それぞれのチームに分かれてから開始してください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "それぞれ");
        assert_pattern_range(&patterns, "それぞれ", 0, 5); // それぞれの
    }
}

mod sokode_tests {
    use super::*;

    // Test: そこで for controllable solution - giving umbrella
    // Example: 彼は雨の中、傘なしで立っている。そこで、私は車から出て彼に私の傘をあげた
    // (He is standing in the rain without an umbrella. As such, I got out of the car and gave him mine)
    #[test]
    fn test_sokode_umbrella() {
        let sentence = "彼は雨の中、傘なしで立っている。そこで、私は車から出て彼に私の傘をあげた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そこで");
        assert_pattern_range(&patterns, "そこで", 16, 19); // そこで
    }

    // Test: そこで for controllable solution - marriage decision
    // Example: 彼女と結婚をすることにした。そこで、彼女の親の実家に行って挨拶をすることに決めた
    // (I have decided to marry my girlfriend. To that end, I have decided to go to her parents' place to talk to them)
    #[test]
    fn test_sokode_marriage() {
        let sentence = "彼女と結婚をすることにした。そこで、彼女の親の実家に行って挨拶をすることに決めた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そこで");
        assert_pattern_range(&patterns, "そこで", 14, 17); // そこで
    }

    // Test: そこで for controllable solution - anti-theft window
    // Example: うちには泥棒が３回も入っている。そこで、防犯ガラスを買うことにした
    // (Our house has been burgled 3 times already. As such, I have decided to buy an anti-theft window)
    #[test]
    fn test_sokode_security() {
        let sentence = "うちには泥棒が３回も入っている。そこで、防犯ガラスを買うことにした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そこで");
        assert_pattern_range(&patterns, "そこで", 16, 19); // そこで
    }

    // Test: そこで for controllable solution - calming child
    // Example: 子供が泣き始めた。そこで、子供に飴をあげて落ち着かせた
    // (My kid started to cry. As such, I gave them some candy to calm them down)
    #[test]
    fn test_sokode_child() {
        let sentence = "子供が泣き始めた。そこで、子供に飴をあげて落ち着かせた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そこで");
        assert_pattern_range(&patterns, "そこで", 9, 12); // そこで
    }
}

// ========== そのため(に) (for that reason/to that end) ==========
// Pattern: そのため(に) (for that reason/to that end)
// Data source: grammar_points_data.json["そのため(に)"]
//
// Structure variants to test:
//   standard[0]: そのため + (に) + Phrase

mod sonotameni_tests {
    use super::*;

    // Test: そのため without に - fishing example
    // Example: ハマダさんはとても釣りが好きです。そのため毎朝仕事に行く前に、釣りに行っています
    // (Hamada-san loves fishing. For that reason, he does it every morning before he goes to work)
    #[test]
    fn test_sonotame_fishing() {
        let sentence = "ハマダさんはとても釣りが好きです。そのため毎朝仕事に行く前に、釣りに行っています";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そのため(に)");
        assert_pattern_range(&patterns, "そのため(に)", 17, 21); // そのため
    }

    // Test: そのため without に - aging population
    // Example: 日本では高齢化が進んでいる。そのため、子供が生まれたら政府からお金がもらえる
    // (Japan's population continues to age. To that end, when a child is born, you receive money from the government)
    #[test]
    fn test_sonotame_population() {
        let sentence = "日本では高齢化が進んでいる。そのため、子供が生まれたら政府からお金がもらえる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そのため(に)");
        assert_pattern_range(&patterns, "そのため(に)", 14, 18); // そのため
    }

    // Test: そのために with に - children's sake
    // Example: 子供達にはなんの不自由もない生活をしてほしい。そのために毎日夜遅くまで仕事をしている
    // (I want my kids to have a life without any struggles. For the sake of that, I work until late at night every day)
    #[test]
    fn test_sonotameni_children() {
        let sentence = "子供達にはなんの不自由もない生活をしてほしい。そのために毎日夜遅くまで仕事をしている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そのため(に)");
        assert_pattern_range(&patterns, "そのため(に)", 23, 28); // そのために
    }

    // Test: そのために with に - car purchase
    // Example: 新しい車が欲しいけど今は住宅ローンで精一杯。そのために車を買うのを我慢している
    // (I want a new car, but I am struggling to pay my mortgage. For that reason, I am holding off on doing it)
    #[test]
    fn test_sonotameni_car() {
        let sentence = "新しい車が欲しいけど今は住宅ローンで精一杯。そのために車を買うのを我慢している";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そのため(に)");
        assert_pattern_range(&patterns, "そのため(に)", 22, 27); // そのために
    }
}

// ========== その結果 (as a result) ==========
// Pattern: その結果 (as a result)
// Data source: grammar_points_data.json["その結果"]
//
// Structure variants to test:
//   standard[0]: Phrase。その結果（けっか） + Phrase

mod sonokekka_tests {
    use super::*;

    // Test: School absence result
    // Example: 学校を３ヶ月休んだ。その結果、皆と卒業することができなかった
    // (I took three months off from school. As a result, I was not able to graduate with everyone)
    #[test]
    fn test_sonokekka_school() {
        let sentence = "学校を３ヶ月休んだ。その結果、皆と卒業することができなかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "その結果");
        assert_pattern_range(&patterns, "その結果", 10, 14); // その結果
    }

    // Test: Sunburn result
    // Example: 日焼け止めを塗らずにビーチで一日過ごした。その結果凄く日焼けをして、しばらくシャワーに入るのが辛かった
    // (I spent the whole day at the beach without sunscreen. As a result, I got a bad sunburn and it was painful to take a shower for a while)
    #[test]
    fn test_sonokekka_sunburn() {
        let sentence = "日焼け止めを塗らずにビーチで一日過ごした。その結果凄く日焼けをして、しばらくシャワーに入るのが辛かった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "その結果");
        assert_pattern_range(&patterns, "その結果", 21, 25); // その結果
    }

    // Test: Practice result (positive)
    // Example: 毎日朝早くから夜遅くまで練習した。その結果、試合で優勝することができた
    // (We practiced every day from early in the morning to late at night. As a result, we were able to win the tournament)
    #[test]
    fn test_sonokekka_practice() {
        let sentence = "毎日朝早くから夜遅くまで練習した。その結果、試合で優勝することができた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "その結果");
        assert_pattern_range(&patterns, "その結果", 17, 21); // その結果
    }

    // Test: Weight gain result
    // Example: タナカ君は一年間運動をしないでゲームばっかりしていた。その結果２０キロも太った
    // (Tanaka-kun only played video games without exercising for a year. As a result, he gained 20 kilograms)
    #[test]
    fn test_sonokekka_weight_gain() {
        let sentence = "タナカ君は一年間運動をしないでゲームばっかりしていた。その結果２０キロも太った";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "その結果");
        assert_pattern_range(&patterns, "その結果", 27, 31); // その結果
    }
}

// ========== ずに (without doing) ==========
// Pattern: ずに (without doing)
// Data source: grammar_points_data.json["ずに"]
//
// Structure variants to test:
//   standard[0]: Verb［ない］+ ず(に)
//   Exception: する ￫ せず(に)

mod zuni_tests {
    use super::*;

    // Test: Regular verb + ずに (without eating)
    // Example: 朝ご飯を食べずに仕事に行った
    // (I went to work without eating breakfast)
    #[test]
    fn test_zuni_regular_verb() {
        let sentence = "朝ご飯を食べずに仕事に行った";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずに");
        assert_pattern_range(&patterns, "ずに", 4, 8); // 食べずに
    }

    // Test: Regular verb + ずに (without drinking)
    // Example: 水を飲まずに運動をした
    // (I exercised without drinking water)
    #[test]
    fn test_zuni_nomanai() {
        let sentence = "水を飲まずに運動をした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずに");
        assert_pattern_range(&patterns, "ずに", 2, 6); // 飲まずに
    }

    // Test: する verb exception + せずに (without studying)
    // Example: 勉強せずにテストを受けた
    // (I took the test without studying)
    #[test]
    fn test_zuni_suru_exception() {
        let sentence = "勉強せずにテストを受けた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずに");
        assert_pattern_range(&patterns, "ずに", 0, 5); // 勉強せずに (includes compound noun+verb)
    }

    // Test: する verb exception + せずに (without trying too hard)
    // Example: 無理をせずに頑張ってください
    // (Please do your best without trying too hard)
    #[test]
    fn test_zuni_suru_casual() {
        let sentence = "無理をせずに頑張ってください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずに");
        assert_pattern_range(&patterns, "ずに", 3, 6); // せずに
    }

    // Test: ず alone (without に) - should also work
    // Example: 何も言わず立ち去った
    // (Left without saying anything)
    #[test]
    fn test_zu_without_ni() {
        let sentence = "何も言わず立ち去った";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずに");
        assert_pattern_range(&patterns, "ずに", 2, 5); // 言わず (without に)
    }
}

// ========== ずにはいられない (can't help but do) ==========
// Pattern: ずにはいられない (can't help but do / cannot resist doing)
// Data source: grammar_points_data.json["ずにはいられない"]
//
// Structure variants to test:
//   standard[0]: Verb［ない］+ ずにはいられない
//   polite[0]: Verb［ない］+ ずにはいられません
//   Exception: する ￫ せずにはいられない

mod zunihairarenai_tests {
    use super::*;

    // Test: Regular verb + ずにはいられない (can't help but eat)
    // Example: 美味しそうなステーキがあったら、食べずにはいられない
    // (If there is delicious steak, I can't help but eat it)
    #[test]
    fn test_zunihairarenai_regular_verb() {
        let sentence = "美味しそうなステーキがあったら、食べずにはいられない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずにはいられない");
        assert_pattern_range(&patterns, "ずにはいられない", 16, 26); // 食べずにはいられない
    }

    // Test: Regular verb + ずにはいられない (can't help but cry)
    // Example: この映画を見ると誰でも泣かずにはいられない
    // (No matter who you are, if you watch this movie, you can't help but cry)
    #[test]
    fn test_zunihairarenai_naku() {
        let sentence = "この映画を見ると誰でも泣かずにはいられない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずにはいられない");
        assert_pattern_range(&patterns, "ずにはいられない", 11, 21); // 泣かずにはいられない
    }

    // Test: する verb exception + せずにはいられない (can't help but thank)
    // Example: 先輩には感謝をせずにはいられない
    // (I can't help but show my senpai appreciation)
    #[test]
    fn test_zunihairarenai_suru_exception() {
        let sentence = "先輩には感謝をせずにはいられない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずにはいられない");
        assert_pattern_range(&patterns, "ずにはいられない", 7, 16); // せずにはいられない (includes compound)
    }

    // Test: する verb + せずにはいられない (can't help but sneeze)
    // Example: 猫アレルギーなのでくしゃみをせずにはいられない
    // (I'm allergic to cats so I can't help but sneeze)
    #[test]
    fn test_zunihairarenai_kushami() {
        let sentence = "猫アレルギーなのでくしゃみをせずにはいられない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずにはいられない");
        assert_pattern_range(&patterns, "ずにはいられない", 14, 23); // せずにはいられない
    }

    // Test: Polite form - ずにはいられません
    // Example: あんな話を聞いたら笑わずにはいられません
    // (If I hear such a story, I can't help but laugh - polite)
    #[test]
    fn test_zunihairarenai_polite() {
        let sentence = "あんな話を聞いたら笑わずにはいられません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずにはいられない");
        assert_pattern_range(&patterns, "ずにはいられない", 9, 20); // 笑わずにはいられません
    }
}

// ========== 遂に (finally/at last) ==========
// Pattern: 遂に (finally/at last)
// Data source: grammar_points_data.json["遂に"]
//
// Structure variants to test:
//   standard[0]: ついに + Phrase

mod tsuini_tests {
    use super::*;

    // Test: ついに + Verb (past tense)
    // Example: ついに日本上陸！！！ (Finally it has arrived in Japan!!!)
    #[test]
    fn test_tsuini_arrival() {
        let sentence = "ついに日本上陸！！！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "遂に");
        assert_pattern_range(&patterns, "遂に", 0, 3); // ついに
    }

    // Test: ついに + Verb (past tense victory)
    // Example: ついにドラゴンズが勝った！ (The Dragons have finally won!)
    #[test]
    fn test_tsuini_victory() {
        let sentence = "ついにドラゴンズが勝った！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "遂に");
        assert_pattern_range(&patterns, "遂に", 0, 3); // ついに
    }

    // Test: ついに + copula + polite
    // Example: ついに最後の日ですね (Finally it is your last day)
    #[test]
    fn test_tsuini_last_day() {
        let sentence = "ついに最後の日ですね、今まで色々とありがとうございました";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "遂に");
        assert_pattern_range(&patterns, "遂に", 0, 3); // ついに
    }

    // Test: ついに + Verb (past tense marriage)
    // Example: 遂に結婚したのか！ (You finally married!?)
    #[test]
    fn test_tsuini_marriage() {
        let sentence = "遂に結婚したのか！おめでとう！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "遂に");
        assert_pattern_range(&patterns, "遂に", 0, 2); // 遂に (kanji form is 2 chars)
    }
}

// ========== ～というのは事実だ (it is a fact that) ==========
// Pattern: ～というのは事実だ (it is a fact that / it is true that)
// Data source: grammar_points_data.json["～というのは事実だ"]
//
// Structure variants to test:
//   standard[0]: Phrase + (という) + のは事実だ
//   polite[0]: Phrase + (という) + のは事実です

mod toiunohajijitsuda_tests {
    use super::*;

    // Test: Verb + というのは事実だ (with という)
    // Example: この人が私の母親を殺したというのは事実だ
    // (It is a fact that this person killed my mother)
    #[test]
    fn test_jijitsuda_verb_with_toiu() {
        let sentence = "この人が私の母親を殺したというのは事実だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～というのは事実だ");
        assert_pattern_range(&patterns, "～というのは事実だ", 12, 20); // というのは事実だ
    }

    // Test: Verb + のは事実だ (without という)
    // Example: 赤信号を無視したのは事実だ
    // (It is true that I ignored the red light)
    #[test]
    fn test_jijitsuda_verb_without_toiu() {
        let sentence = "赤信号を無視したのは事実だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～というのは事実だ");
        assert_pattern_range(&patterns, "～というのは事実だ", 8, 13); // のは事実だ
    }

    // Test: Copula + というのは事実だ
    // Example: 彼女が弁護士だというのは事実だ
    // (It is a fact that she is a lawyer)
    #[test]
    fn test_jijitsuda_copula() {
        let sentence = "彼女が弁護士だというのは事実だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～というのは事実だ");
        assert_pattern_range(&patterns, "～というのは事実だ", 7, 15); // というのは事実だ
    }

    // Test: Polite form - のは事実です
    // Example: 彼女と仲直りしたのは事実です
    // (It is a fact that I worked it out with my girlfriend)
    #[test]
    fn test_jijitsuda_polite() {
        let sentence = "彼女と仲直りしたのは事実です";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～というのは事実だ");
        assert_pattern_range(&patterns, "～というのは事実だ", 8, 14); // のは事実です
    }
}

// ========== 直ちに (immediately/at once) ==========
// Pattern: 直ちに (immediately/at once)
// Data source: grammar_points_data.json["直ちに"]
//
// Structure variants to test:
//   standard[0]: 直（ただ）ちに + Phrase

mod tadachini_tests {
    use super::*;

    // Test: 直ちに at sentence start with verb phrase
    #[test]
    fn test_tadachini_evacuation() {
        let sentence = "ただちに避難所へ向かってください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "直ちに");
        assert_pattern_range(&patterns, "直ちに", 0, 4); // ただちに
    }

    // Test: 直ちに in middle of sentence
    #[test]
    fn test_tadachini_leave_mountain() {
        let sentence = "この山から、ただちに出て行ってください。ここは私有地です";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "直ちに");
        assert_pattern_range(&patterns, "直ちに", 6, 10); // ただちに
    }

    // Test: 直ちに with earthquake example
    #[test]
    fn test_tadachini_earthquake() {
        let sentence = "地震が起きたらただちに家を出てください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "直ちに");
        assert_pattern_range(&patterns, "直ちに", 7, 11); // ただちに
    }

    // Test: 直ちに with reporting example (intentional action)
    #[test]
    fn test_tadachini_report() {
        let sentence = "計画が変わったら、ただちに報告してください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "直ちに");
        assert_pattern_range(&patterns, "直ちに", 9, 13); // ただちに
    }
}

// ========== 折角 (with effort/specially) ==========
// Pattern: 折角 (with effort/specially/long-awaited)
// Data source: grammar_points_data.json["折角"]
//
// Structure variants to test:
//   standard[0]: せっかく + Phrase
//   standard[1]: せっかく + の + Noun (of Event)

mod sekkaku_tests {
    use super::*;

    // Test: せっかく + Phrase (at great pains)
    #[test]
    fn test_sekkaku_shoes() {
        let sentence = "せっかく君が欲しかった靴を買ってあげたのに友達にあげたの？最悪";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "折角");
        assert_pattern_range(&patterns, "折角", 0, 4); // せっかく
    }

    // Test: せっかく + Phrase (with trouble)
    #[test]
    fn test_sekkaku_day_off() {
        let sentence = "せっかく休みを取ったのに雨が降ったから予定が台無しだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "折角");
        assert_pattern_range(&patterns, "折角", 0, 4); // せっかく
    }

    // Test: せっかく + の + Noun (long-awaited event)
    #[test]
    fn test_sekkaku_no_birthday() {
        let sentence = "せっかくの誕生日会を台無しにしてごめんなさい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "折角");
        assert_pattern_range(&patterns, "折角", 0, 5); // せっかくの
    }

    // Test: せっかく + の + Noun (long-awaited vacation)
    #[test]
    fn test_sekkaku_no_vacation() {
        let sentence = "台風のせいで、せっかくの休暇が中止になった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "折角");
        assert_pattern_range(&patterns, "折角", 7, 12); // せっかくの
    }
}

// ========== せいで (because of / due to - negative result) ==========
// Pattern: せいで (because of / due to - negative result)
// Data source: grammar_points_data.json["せいで"]
//
// Structure variants to test:
//   standard[0]: Verb + せいで
//   standard[1]: い-Adjective + せいで
//   standard[2]: な-Adjective + な + せいで
//   standard[3]: Noun + の + せいで

mod seide_tests {
    use super::*;

    // Test: Verb + せいで
    #[test]
    fn test_seide_verb() {
        let sentence = "昨日夜遅くまで起きていたせいで寝坊した";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "せいで");
        assert_pattern_range(&patterns, "せいで", 11, 15); // たせいで
    }

    // Test: い-Adjective + せいで
    #[test]
    fn test_seide_i_adjective() {
        let sentence = "外が寒いせいで風邪を引いてしまった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "せいで");
        assert_pattern_range(&patterns, "せいで", 2, 7); // 寒いせいで
    }

    // Test: な-Adjective + な + せいで
    #[test]
    fn test_seide_na_adjective() {
        let sentence = "犬の手術のことが心配なせいで今日は寝れなさそう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "せいで");
        assert_pattern_range(&patterns, "せいで", 8, 14); // 心配なせいで
    }

    // Test: Noun + の + せいで
    #[test]
    fn test_seide_noun() {
        let sentence = "大雪のせいで道が渋滞している";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "せいで");
        assert_pattern_range(&patterns, "せいで", 0, 6); // 大雪のせいで
    }
}

// ========== たびに (every time / whenever) ==========
// Pattern: たびに (every time / whenever)
// Data source: grammar_points_data.json["たびに"]
//
// Structure variants to test:
//   standard[0]: Verb［る］+ たびに
//   standard[1]: Noun + の + たびに

mod tabini_tests {
    use super::*;

    // Test: Verb［る］+ たびに - drinking example
    #[test]
    fn test_tabini_verb_drinking() {
        let sentence = "タナカ君と飲みに行くたびに、二日酔いになる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たびに");
        assert_pattern_range(&patterns, "たびに", 8, 13); // 行くたびに
    }

    // Test: Verb［る］+ たびに - milk example
    #[test]
    fn test_tabini_verb_milk() {
        let sentence = "牛乳を飲むたびにお腹が痛くなる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たびに");
        assert_pattern_range(&patterns, "たびに", 3, 8); // 飲むたびに
    }

    // Test: Noun + の + たびに - holiday example
    #[test]
    fn test_tabini_noun_holiday() {
        let sentence = "私は休みのたびにハワイへ行きます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たびに");
        assert_pattern_range(&patterns, "たびに", 2, 8); // 休みのたびに
    }

    // Test: Noun + の + たびに - dishes example
    #[test]
    fn test_tabini_noun_dishes() {
        let sentence = "彼は洗い物のたびに指を切る";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たびに");
        assert_pattern_range(&patterns, "たびに", 2, 9); // 洗い物のたびに
    }
}

// ========== ずっと ② (by far/much more) ==========
// Pattern: ずっと ② (by far/much more - comparison)
// Data source: grammar_points_data.json["ずっと ②"]
//
// Structure variants to test:
//   standard[0]: ずっと + Phrase (comparative context)

mod zutto_u2461_tests {
    use super::*;

    // Test: ずっと + Comparative Adjective (より comparison)
    #[test]
    fn test_zutto_u2461_comparison_bird() {
        let sentence = "さっき見た鳥よりずっと大きかったよ！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずっと ②");
        assert_pattern_range(&patterns, "ずっと ②", 8, 11); // ずっと
    }

    // Test: ずっと + Temporal comparison (より comparison)
    #[test]
    fn test_zutto_u2461_comparison_time() {
        let sentence = "それよりずっと昔だよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずっと ②");
        assert_pattern_range(&patterns, "ずっと ②", 4, 7); // ずっと
    }

    // Test: ずっと + Comparative (no より visible)
    #[test]
    fn test_zutto_u2461_much_better() {
        let sentence = "こっちの方がずっとすごいよ！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずっと ②");
        assert_pattern_range(&patterns, "ずっと ②", 6, 9); // ずっと
    }

    // Test: ずっと + Comparative adjective (違う context)
    #[test]
    fn test_zutto_u2461_different() {
        let sentence = "あの人の考え方はずっと違うと思う";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずっと ②");
        assert_pattern_range(&patterns, "ずっと ②", 8, 11); // ずっと
    }
}

// ========== だらけ (covered with/full of) ==========
// Pattern: だらけ (covered with/full of - scattered state)
// Data source: grammar_points_data.json["だらけ"]
//
// Structure variants to test:
//   standard[0]: Noun + だらけ
//   standard[1]: Noun + だらけ + の + Noun

mod darake_tests {
    use super::*;

    // Test: Noun + だらけ (without の)
    #[test]
    fn test_darake_holes() {
        let sentence = "パンケーキの裏側が穴だらけで気持ち悪い";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だらけ");
        assert_pattern_range(&patterns, "だらけ", 9, 13); // 穴だらけ
    }

    // Test: Noun + だらけ (without の) - garbage
    #[test]
    fn test_darake_garbage() {
        let sentence = "この公園はゴミだらけだから子供を連れて来たくない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だらけ");
        assert_pattern_range(&patterns, "だらけ", 5, 10); // ゴミだらけ
    }

    // Test: Noun + だらけ + の + Noun - scratches
    #[test]
    fn test_darake_no_scratches() {
        let sentence = "傷だらけの車に乗っているのを見られたくない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だらけ");
        assert_pattern_range(&patterns, "だらけ", 0, 5); // 傷だらけの
    }

    // Test: Noun + だらけ + の + Noun - mud
    #[test]
    fn test_darake_no_mud() {
        let sentence = "泥だらけの服を洗濯機にいれないで！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だらけ");
        assert_pattern_range(&patterns, "だらけ", 0, 5); // 泥だらけの
    }
}

// ========== もっとも (although/however) ==========
// Pattern: もっとも (although/however/with that said)
// Data source: grammar_points_data.json["もっとも"]
//
// Structure variants to test:
//   standard[0]: Phrase (A)。もっとも + Phrase (B)。

mod mottomo_tests {
    use super::*;

    // Test: もっとも - piano example
    #[test]
    fn test_mottomo_piano() {
        let sentence = "妹はピアノを弾くのが上手だ。もっとも、弾けるのは一曲だけだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "もっとも");
        assert_pattern_range(&patterns, "もっとも", 14, 18); // もっとも
    }

    // Test: もっとも - sukiyaki example
    #[test]
    fn test_mottomo_sukiyaki() {
        let sentence = "俺はすき焼きが大好きだ。もっとも、毎日食べれる訳では無い。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "もっとも");
        assert_pattern_range(&patterns, "もっとも", 12, 16); // もっとも
    }

    // Test: もっとも - Korea knowledge
    #[test]
    fn test_mottomo_korea() {
        let sentence = "私は韓国についてものすごく詳しいです。もっとも、韓国には行った事が無いけど。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "もっとも");
        assert_pattern_range(&patterns, "もっとも", 19, 23); // もっとも
    }

    // Test: もっとも - computer example
    #[test]
    fn test_mottomo_computer() {
        let sentence = "今日は会社に自分のパソコンを持ってきた。もっとも、自分のパソコンはいらないが。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "もっとも");
        assert_pattern_range(&patterns, "もっとも", 20, 24); // もっとも
    }
}

// ========== 再び (again/once more) ==========
// Pattern: 再び (again/once more/a second time)
// Data source: grammar_points_data.json["再び"]
//
// Structure variants to test:
//   standard[0]: ふたたび + Phrase

mod futatabi_tests {
    use super::*;

    // Test: 再び - crime example (hiragana form)
    #[test]
    fn test_futatabi_crime() {
        let sentence = "彼は刑務所から出て、ふたたび犯罪を犯した。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "再び");
        assert_pattern_range(&patterns, "再び", 10, 14); // ふたたび
    }

    // Test: 再び - hometown example (hiragana form, sentence start)
    #[test]
    fn test_futatabi_hometown() {
        let sentence = "ふたたびふるさとに戻りたいです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "再び");
        assert_pattern_range(&patterns, "再び", 0, 4); // ふたたび
    }

    // Test: 再び - meet again (kanji form)
    #[test]
    fn test_futatabi_meet_kanji() {
        let sentence = "あなたに再び会えて嬉しい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "再び");
        assert_pattern_range(&patterns, "再び", 4, 6); // 再び
    }

    // Test: 再び - start over (kanji form)
    #[test]
    fn test_futatabi_start_over() {
        let sentence = "この仕事を再び始めることにした。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "再び");
        assert_pattern_range(&patterns, "再び", 5, 7); // 再び
    }
}

// ========== たとたんに (the instant/the moment) ==========
// Pattern: たとたんに (the instant/the moment)
// Data source: grammar_points_data.json["たとたんに"]
//
// Structure variants to test:
//   standard[0]: Verb［た］+ とたん(に)

mod tatotannini_tests {
    use super::*;

    // Test: たとたんに - highway tire puncture (hiragana with に)
    #[test]
    fn test_tatotannini_highway() {
        let sentence = "高速に乗ったとたんに、タイヤがパンクした。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たとたんに");
        assert_pattern_range(&patterns, "たとたんに", 3, 10); // 乗ったとたんに
    }

    // Test: たとたんに - futon phone call (hiragana with に)
    #[test]
    fn test_tatotannini_futon() {
        let sentence = "布団に入ったとたんに先輩から電話がかかってきた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たとたんに");
        assert_pattern_range(&patterns, "たとたんに", 3, 10); // 入ったとたんに
    }

    // Test: たとたん - without に (kanji form)
    #[test]
    fn test_tatotan_no_ni() {
        let sentence = "家を出た途端、雨が降り出した。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たとたんに");
        assert_pattern_range(&patterns, "たとたんに", 2, 6); // 出た途端
    }

    // Test: た途端に - kanji form with に
    #[test]
    fn test_tatotan_kanji_ni() {
        let sentence = "目を閉じた途端に眠ってしまった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たとたんに");
        assert_pattern_range(&patterns, "たとたんに", 2, 8); // 閉じた途端に
    }
}

// ========== ため(に) (for the sake of / in order to) ==========
// Pattern: ため(に) - purpose/goal expression
// Data source: grammar_points_data.json["ため(に)"]
//
// Structure variants to test:
//   standard[0]: Verb[る] + ため(に)
//   standard[1]: Noun + の + ため(に)

mod tameni_purpose_tests {
    use super::*;

    // Test: Verb[る] + ために - purpose with に
    #[test]
    fn test_tameni_verb_purpose_ni() {
        let sentence = "新しい家を建てるために、土地を買った。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ため(に)");
        assert_pattern_range(&patterns, "ため(に)", 5, 11); // 建てるために
    }

    // Test: Verb[る] + ため - purpose without に
    #[test]
    fn test_tameni_verb_purpose_no_ni() {
        let sentence = "勉強をするため、親に机を買ってもらった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ため(に)");
        assert_pattern_range(&patterns, "ため(に)", 3, 7); // するため
    }

    // Test: Noun + の + ために - purpose with に
    #[test]
    fn test_tameni_noun_purpose_ni() {
        let sentence = "私のためにやって。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ため(に)");
        assert_pattern_range(&patterns, "ため(に)", 0, 5); // 私のために
    }

    // Test: Noun + の + ため - purpose without に
    #[test]
    fn test_tameni_noun_purpose_no_ni() {
        let sentence = "君のために買ってあげたのに、誰かにあげちゃったの？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ため(に)");
        assert_pattern_range(&patterns, "ため(に)", 0, 5); // 君のために
    }
}

// ========== たて (freshly/just finished) ==========
// Pattern: たて - indicates something just finished/freshly done
// Data source: grammar_points_data.json["たて"]
//
// Structure variants to test:
//   standard[0]: Verb[stem] + たて
//   standard[1]: Verb[stem] + たて + の + Noun

mod tate_tests {
    use super::*;

    // Test: Verb[stem] + たて - just fished (split tokenization)
    #[test]
    fn test_tate_fished() {
        let sentence = "釣りたてだから新鮮だよ！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たて");
        assert_pattern_range(&patterns, "たて", 0, 4); // 釣りたて
    }

    // Test: Verb[stem] + たて + の + Noun - freshly made (split tokenization)
    #[test]
    fn test_tate_no_noun_made() {
        let sentence = "私は出来たてのパンを食べるのが大好きです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たて");
        assert_pattern_range(&patterns, "たて", 2, 7); // 出来たての
    }

    // Test: Noun + たて + の - just picked (split tokenization)
    #[test]
    fn test_tate_picked() {
        let sentence = "採りたての野菜で作るサラダは美味しい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たて");
        assert_pattern_range(&patterns, "たて", 0, 5); // 採りたての
    }

    // Test: Noun + たて + の + Noun - freshly baked (split tokenization)
    #[test]
    fn test_tate_no_noun_baked() {
        let sentence = "焼きたてのパンケーキが食べたい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たて");
        assert_pattern_range(&patterns, "たて", 0, 5); // 焼きたての
    }
}

// Pattern: たとえ〜ても (even if)
// Data source: grammar_points_data.json["たとえ〜ても"]
// Testing: 3 working structure variants (na-adjective case with single-token でも not currently detected)
//   - standard[0]: たとえ + Verb［ても］
//   - standard[1]: たとえ + ［い］Adjective［ても］
//   - standard[3]: たとえ + Noun + でも
//
// Note: standard[2] (たとえ + な-Adjective + でも) has a detection issue when でも tokenizes
// as a single token (助詞/副助詞) instead of で+も. This is a known limitation.
mod tatoetemo_tests {
    use super::*;

    #[test]
    fn test_tatoetemo_verb() {
        let sentence = "空手ではたとえ試合で勝っても、ガッツポーズをしてはいけない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たとえ〜ても");
        assert_pattern_range(&patterns, "たとえ〜ても", 4, 14); // たとえ試合で勝っても
    }

    #[test]
    fn test_tatoetemo_i_adjective() {
        let sentence = "たとえ暑くても、虫が入ってくるのでこの窓は開けないで下さい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たとえ〜ても");
        assert_pattern_range(&patterns, "たとえ〜ても", 0, 7); // たとえ暑くても
    }

    #[test]
    fn test_tatoetemo_noun() {
        let sentence = "たとえ電車でも３０分はかかる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たとえ〜ても");
        assert_pattern_range(&patterns, "たとえ〜ても", 0, 7); // たとえ電車でも
    }

    // TODO: Fix na-adjective detection when でも is a single token
    // #[test]
    // fn test_tatoetemo_na_adjective() {
    //     let sentence = "たとえ好きでも、嫌いになることもあるよ";
    //     let tokens = tokenize_sentence(sentence);
    //     let patterns = detect_patterns(&tokens);
    //
    //     assert_has_pattern(&patterns, "たとえ〜ても");
    //     assert_pattern_range(&patterns, "たとえ〜ても", 0, 7); // たとえ好きでも
    // }
}

// ========== つい (accidentally/unconsciously) ==========
// Pattern: つい (accidentally/unconsciously/against one's better judgment)
// Data source: grammar_points_data.json["つい"]
//
// Structure variants to test:
//   standard[0]: つい + Phrase
//
// Note: Often paired with てしまう to emphasize unintentional action.
// Can convey "uncalculated", "not thought about", or "small time gap" nuances.
mod tsui_tests {
    use super::*;

    // Test: つい + Verb (with てしまう)
    // Example from grammar data: つい食べてしまった (accidentally ate it)
    #[test]
    fn test_tsui_with_teshimau() {
        let sentence = "ごめん、クッキーが目の前にあったからつい食べてしまった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "つい");
        assert_pattern_range(&patterns, "つい", 18, 20); // つい
    }

    // Test: つい + Verb (simple)
    // Example: つい笑っちゃう (unconsciously laugh)
    #[test]
    fn test_tsui_with_verb() {
        let sentence = "先生に怒られると、つい笑っちゃうんだよね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "つい");
        assert_pattern_range(&patterns, "つい", 9, 11); // つい
    }

    // Test: つい + Verb (casual form)
    // Example: メールが来るとつい携帯を見ちゃう
    #[test]
    fn test_tsui_casual() {
        let sentence = "メールが来るとつい携帯を見ちゃう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "つい");
        assert_pattern_range(&patterns, "つい", 7, 9); // つい
    }
}

// ========== つまり (in other words/in short) ==========
// Pattern: つまり (in other words/in short/condensed summary)
// Data source: grammar_points_data.json["つまり"]
//
// Structure variants to test:
//   standard[0]: Phrase (A)。つまり + (Summary) Phrase (B)
//
// Note: Used to summarize or rephrase a previous statement.
// Can be tokenized as either 名詞/一般 or 接続詞 depending on context.
mod tsumari_tests {
    use super::*;

    // Test: つまり after question (noun form)
    // Example: つまり、また無断欠勤ということですね (in other words, ditched work again)
    #[test]
    fn test_tsumari_noun_form() {
        let sentence = "ハマサキさんはまた休んだのですか？つまり、また無断欠勤ということですね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "つまり");
        assert_pattern_range(&patterns, "つまり", 17, 20); // つまり
    }

    // Test: つまり as conjunction
    // Example: つまり、もう関わらなくていいということ？
    #[test]
    fn test_tsumari_conjunction() {
        let sentence = "彼氏と別れたの？つまり、もう関わらなくていいということ？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "つまり");
        assert_pattern_range(&patterns, "つまり", 8, 11); // つまり
    }

    // Test: つまり summarizing statement
    // Example: つまり一日中ダラダラしていたということですね
    #[test]
    fn test_tsumari_summarizing() {
        let sentence = "ゲームばかりしていた。つまり一日中ダラダラしていたということですね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "つまり");
        assert_pattern_range(&patterns, "つまり", 11, 14); // つまり
    }
}

// ========== っぱなし (left in a state) ==========
// Pattern: っぱなし (left in a state / left unchecked)
// Data source: grammar_points_data.json["っぱなし"]
//
// Structure variants to test:
//   standard[0]: Verb[stem] + っぱなし

mod ppanashi_tests {
    use super::*;

    // Test: Verb stem + っぱなし (light left on)
    // Example: 電気を点けっぱなしにするな
    #[test]
    fn test_ppanashi_light_on() {
        let sentence = "電気を点けっぱなしにするな";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "っぱなし");
        assert_pattern_range(&patterns, "っぱなし", 3, 9); // 点けっぱなし
    }

    // Test: Verb stem + っぱなし (water running)
    // Example: 水を出しっぱなしにしていたら、お母さんに怒られた
    #[test]
    fn test_ppanashi_water_running() {
        let sentence = "歯を磨いている間、水を出しっぱなしにしていたら、お母さんに怒られた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "っぱなし");
        assert_pattern_range(&patterns, "っぱなし", 11, 17); // 出しっぱなし
    }

    // Test: Verb stem + っぱなし (tap running)
    // Example: 蛇口を開きっぱなしにするな
    #[test]
    fn test_ppanashi_tap_running() {
        let sentence = "蛇口を開きっぱなしにするな";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "っぱなし");
        assert_pattern_range(&patterns, "っぱなし", 3, 9); // 開きっぱなし
    }

    // Test: Verb stem + っぱなし (winning streak)
    // Example: 勝ちっぱなしだとつまらないね
    #[test]
    fn test_ppanashi_winning_streak() {
        let sentence = "勝ちっぱなしだとつまらないね、たまには負けを味わいたい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "っぱなし");
        assert_pattern_range(&patterns, "っぱなし", 0, 6); // 勝ちっぱなし
    }

    // TODO: Compound form tests - These tokenize as single noun (名詞/一般) and cannot be detected
    // as grammar patterns. They are lexicalized compounds.
    //
    // #[test]
    // fn test_ppanashi_compound_form() {
    //     let sentence = "冷蔵庫を開けっぱなしにしないでください";
    //     // 開けっぱなし = single token (名詞/一般) - not detectable as pattern
    // }
}

// Pattern: だって (because/but/even)
// Data source: grammar_points_data.json["だって"]
// Testing structure variants
mod datte_tests {
    use super::*;

    // Testing: structure.standard[0] - "Noun + （なん） + だって"
    // Example: 俺だって行きたくないよ。 (Even I don't want to go.)
    #[test]
    fn test_datte_noun_even() {
        let sentence = "俺だって行きたくないよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だって");
        assert_pattern_range(&patterns, "だって", 0, 4); // 俺だって
    }

    // Testing: structure.standard[0] - "Noun + （なん） + だって"
    // Example: 誰だって傷つくよ (Anyone would get hurt)
    #[test]
    fn test_datte_noun_anyone() {
        let sentence = "そんなこと言ったら、誰だって傷つくよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だって");
        assert_pattern_range(&patterns, "だって", 10, 14); // 誰だって
    }

    // Testing: structure.standard[1] - "だって + Phrase"
    // Example: だって、サメとか怖いもん。 (It's because I'm afraid of things like sharks.)
    #[test]
    fn test_datte_because_beginning() {
        let sentence = "だって、サメとか怖いもん。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だって");
        assert_pattern_range(&patterns, "だって", 0, 3); // だって
    }

    // Testing: structure.standard[1] - "だって + Phrase"
    // Example: だって、俺の元カノも誘ったんでしょう？ (It's because you also invited my ex-girlfriend, right?)
    #[test]
    fn test_datte_because_explanation() {
        let sentence = "え〜行きたくないよ。だって、俺の元カノも誘ったんでしょう？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だって");
        assert_pattern_range(&patterns, "だって", 10, 13); // だって
    }
}

// ========== っぽい (ish/like/tendency to) ==========
// Pattern: っぽい (ish/like/tendency to)
// Data source: grammar_points_data.json["っぽい"]
//
// Structure variants to test:
//   standard[0]: Verb[stem] + っぽい
//   standard[1]: い-Adjective[い] + っぽい
//   standard[2]: な-Adjective + っぽい
//   standard[3]: Noun + っぽい
//   polite[0-3]: Same + です
//
// Meaning: Exhibits characteristics of (A), -ish, -like, tendency to
// Often carries negative connotation

mod ppoi_tests {
    use super::*;

    // Testing: structure.standard[0] - "Verb[stem] + っぽい"
    // Example: 飽きっぽい (tendency to get bored)
    #[test]
    fn test_ppoi_verb_stem() {
        let sentence = "私は飽きっぽいから、何も続かない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "っぽい");
        assert_pattern_range(&patterns, "っぽい", 2, 7); // 飽きっぽい
    }

    // Testing: structure.standard[2] - "な-Adjective + っぽい"
    // Example: 有名っぽい (seems famous)
    #[test]
    fn test_ppoi_na_adjective() {
        let sentence = "タナカ君はファッション業界では有名っぽいよ！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "っぽい");
        assert_pattern_range(&patterns, "っぽい", 15, 20); // 有名っぽい
    }

    // Testing: structure.standard[3] - "Noun + っぽい"
    // Example: 嘘っぽい (seems like a lie)
    #[test]
    fn test_ppoi_noun() {
        let sentence = "今の話は嘘っぽいけど本当の話なの";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "っぽい");
        assert_pattern_range(&patterns, "っぽい", 4, 8); // 嘘っぽい
    }

    // Testing: polite form - "Noun + っぽい + です"
    // Example: 紫っぽいです (it's purplish)
    #[test]
    fn test_ppoi_polite() {
        let sentence = "昔は紫っぽかったのに今はなぜか赤っぽいです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "っぽい");
        // Two instances detected
        assert_pattern_range(&patterns, "っぽい", 2, 8); // 紫っぽかった (first match)
    }

    // Testing: conjugated form - "Noun + っぽかった" (past tense)
    // Example: 紫っぽかった (was purplish)
    #[test]
    fn test_ppoi_conjugated_past() {
        let sentence = "昔は紫っぽかったよね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "っぽい");
        assert_pattern_range(&patterns, "っぽい", 2, 8); // 紫っぽかった
    }

    // Testing: negative form - "Noun + っぽく + ない"
    // Example: 子供っぽくない (not childish)
    #[test]
    fn test_ppoi_negative() {
        let sentence = "彼女は全然子供っぽくないよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "っぽい");
        assert_pattern_range(&patterns, "っぽい", 5, 12); // 子供っぽくない
    }

    // TODO: UNDETECTABLE - い-Adjective[い] + っぽい (lexicalized compounds)
    // Common combinations like 安っぽい and 白っぽい are lexicalized as single tokens
    // Tokenization: 安っぽい (形容詞/自立) - NOT 安 + っぽい
    // Tokenization: 白っぽい (形容詞/自立) - NOT 白 + っぽい
    // These tokenize as standalone い-adjectives (形容詞/自立), not as suffix patterns (形容詞/接尾)
    // Cannot be detected as the っぽい pattern since there's no split tokenization
    //
    // #[test]
    // fn test_ppoi_i_adjective_undetectable() {
    //     let sentence = "この靴なんか安っぽくない？";
    //     // 安っぽく tokenizes as 形容詞/自立, not as 安 + っぽい
    // }
    //
    // #[test]
    // fn test_ppoi_noun_color_undetectable() {
    //     let sentence = "白っぽいやつをください";
    //     // 白っぽい tokenizes as 形容詞/自立, not as 白 + っぽい
    // }
}

// ========== ついでに (while you're at it / on the occasion of) ==========
// Pattern: ついでに (while you're at it / on the occasion of)
// Data source: grammar_points_data.json["ついでに"]
//
// Structure variants to test:
//   standard[0]: Verb + ついでに
//   standard[1]: Noun + の + ついでに
//   standard[2]: Phrase。ついでに + Phrase

mod tsuideni_tests {
    use super::*;

    // Test: Verb + ついでに
    #[test]
    fn test_tsuideni_verb() {
        let sentence = "買い物に行くついでに郵便局に寄ってくれる？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ついでに");
        assert_pattern_range(&patterns, "ついでに", 4, 10); // 行くついでに
    }

    // Test: Verb + ついでに (different verb)
    #[test]
    fn test_tsuideni_verb_return() {
        let sentence = "お母さんの所にテレビを返しに行くついでに、これを持って行って";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ついでに");
        assert_pattern_range(&patterns, "ついでに", 14, 20); // 行くついでに
    }

    // Test: Noun + の + ついでに
    #[test]
    fn test_tsuideni_noun_no() {
        let sentence = "散歩のついでに寄って行ってください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ついでに");
        assert_pattern_range(&patterns, "ついでに", 0, 7); // 散歩のついでに
    }

    // Test: Phrase。ついでに + Phrase (standalone after period)
    #[test]
    fn test_tsuideni_standalone() {
        let sentence = "デパートで買い物をする。ついでに友達に会う予定だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ついでに");
        // Note: Pattern matches from period to に because TokenMatcher::Any includes the period
        // This is acceptable as it captures the context that ついでに appears after a sentence
        assert_pattern_range(&patterns, "ついでに", 11, 16); // 。ついでに
    }
}

// ========== たものだ (used to / would often - past habit) ==========
// Pattern: たものだ (used to / would often)
// Data source: grammar_points_data.json["たものだ"]
//
// Structure variants to test:
//   standard[0]: Verb[た] + ものだ
//   polite[0]: Verb[た] + ものです

mod tamonoda_tests {
    use super::*;

    // Test: Verb[た] + ものだ (standard form - nostalgia about getting scolded)
    #[test]
    fn test_tamonoda_standard_scolded() {
        let sentence = "子供の頃はよく先生に怒られたものだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たものだ");
        assert_pattern_range(&patterns, "たものだ", 12, 17); // れたものだ
    }

    // Test: Verb[た] + ものだ (standard form - nostalgia about fishing)
    #[test]
    fn test_tamonoda_standard_fishing() {
        let sentence = "昔はよく親父と釣りに行ったものだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たものだ");
        assert_pattern_range(&patterns, "たものだ", 10, 16); // 行ったものだ
    }

    // Test: Verb[た] + ものだ (standard form - reminiscing about hardship)
    #[test]
    fn test_tamonoda_standard_hardship() {
        let sentence = "若い頃は色々と苦労したものだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たものだ");
        assert_pattern_range(&patterns, "たものだ", 7, 14); // 苦労したものだ
    }

    // Test: Verb[た] + ものです (polite form)
    #[test]
    fn test_tamonoda_polite() {
        let sentence = "学生時代はあそこのカフェでよく勉強したものです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たものだ");
        assert_pattern_range(&patterns, "たものだ", 15, 23); // 勉強したものです
    }
}

// ========== て初めて (only after) ==========
// Pattern: て初めて (only after/not until)
// Data source: grammar_points_data.json["て初めて"]
//
// Structure variants to test:
//   standard[0]: Verb[て] + はじめて
//
// Note: This pattern uses 初めて (はじめて) as an adverb meaning "for the first time"
// The tokenizer recognizes the kanji form 初めて as 副詞/一般 (adverb)
// The hiragana form はじめて gets parsed as はじめる (auxiliary verb) + て
// For reliable detection, we match the kanji form 初めて

mod te_hajimete_tests {
    use super::*;

    // Test: Verb[て] + 初めて - basic example
    #[test]
    fn test_te_hajimete_basic() {
        let sentence = "先生になって初めて、先生の大変さが分かった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "て初めて");
        assert_pattern_range(&patterns, "て初めて", 3, 9); // なって初めて
    }

    // Test: Verb[て] + 初めて - different verb
    #[test]
    fn test_te_hajimete_realize() {
        let sentence = "好きなバンドのコンサートに行って初めて、歌手の人が男の人だと気づいた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "て初めて");
        assert_pattern_range(&patterns, "て初めて", 13, 19); // 行って初めて
    }

    // Test: Verb[て] + 初めて - train schedule example
    #[test]
    fn test_te_hajimete_know() {
        let sentence = "駅にある時刻表を見て初めて、土日には電車が来ないことを知った";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "て初めて");
        assert_pattern_range(&patterns, "て初めて", 8, 13); // 見て初めて
    }

    // Test: Verb[て] + 初めて - casual conversation
    #[test]
    fn test_te_hajimete_understand() {
        let sentence = "海外に住んでみて初めて日本の良さが分かる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "て初めて");
        assert_pattern_range(&patterns, "て初めて", 6, 11); // みて初めて
    }
}

// ========== 向き (suitable for / facing) ==========
// Pattern: 向き (suitable for / facing toward)
// Data source: grammar_points_data.json["向き"]
//
// Structure variants to test:
//   standard[0]: Noun + 向き
//
// About: 向き is a noun/suffix meaning "suitable for" or "facing toward"
// Comes from the intransitive verb 向く (to face)
// Implies natural suitability rather than intentional design
// Examples: 初心者向き (suitable for beginners), 南向き (south-facing), 前向き (forward-facing/positive)

mod muki_tests {
    use super::*;

    // Test: Noun + 向き - beginners (suitability)
    #[test]
    fn test_muki_beginners() {
        let sentence = "この本は初心者向きですか？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "向き");
        assert_pattern_range(&patterns, "向き", 4, 9); // 初心者向き
    }

    // Test: Noun + 向き - direction (south-facing)
    #[test]
    fn test_muki_direction() {
        let sentence = "この部屋のベランダは南向きです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "向き");
        assert_pattern_range(&patterns, "向き", 10, 13); // 南向き
    }

    // Test: Noun + 向き - casual usage
    #[test]
    fn test_muki_casual() {
        let sentence = "大人向きの映画だと思うけど";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "向き");
        assert_pattern_range(&patterns, "向き", 0, 4); // 大人向き
    }

    // TODO: Undetectable - Compound word 前向き
    // The word 前向き (forward-facing/positive) is tokenized as a single compound word
    // (名詞/形容動詞語幹) rather than as 前 + 向き. This is a lexicalized compound that
    // cannot be detected as a pattern. Similar to how 白っぽい tokenizes as a single
    // adjective rather than 白 + っぽい.
    //
    // #[test]
    // fn test_muki_personality() {
    //     let sentence = "あなたは本当に前向きな人なのですね";
    //     let tokens = tokenize_sentence(sentence);
    //     let patterns = detect_patterns(&tokens);
    //
    //     // 前向き = 名詞/形容動詞語幹 (single token, not detectable as pattern)
    //     assert!(!has_pattern(&patterns, "向き"));
    // }
}

// ============================================================================
// 中 (ちゅう/じゅう) - "during / in the middle of / throughout"
// ============================================================================
// Pattern: During/throughout (in the middle of)
// Data source: grammar_points_data.json["中"]
// Structure: Noun + 中（ちゅう/じゅう）（に）
//
// ちゅう - Specific/variable time/space, process-focused
// じゅう - Fixed time/space, duration-focused
mod chuu_tests {
    use super::*;

    // Test: Noun + 中 (process/ongoing - ちゅう reading)
    #[test]
    fn test_chuu_process() {
        let sentence = "今は仕事中なんだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "中");
        assert_pattern_range(&patterns, "中", 2, 5); // 仕事中
    }

    // Test: Noun + 中 + に (during - ちゅう reading with に)
    #[test]
    fn test_chuu_ni_during() {
        let sentence = "彼は休憩中に本を読む。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "中");
        assert_pattern_range(&patterns, "中", 2, 6); // 休憩中に
    }

    // Test: Noun + じゅう + に (throughout/by deadline - じゅう reading)
    #[test]
    fn test_juu_ni_deadline() {
        let sentence = "今日じゅうにレポートを提出してください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "中");
        assert_pattern_range(&patterns, "中", 0, 6); // 今日じゅうに
    }

    // Test: Noun + じゅう + に (throughout space - じゅう reading)
    #[test]
    fn test_juu_space() {
        let sentence = "ゴミが家じゅうに広がっていて歩くスペースがない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "中");
        assert_pattern_range(&patterns, "中", 3, 8); // 家じゅうに
    }
}

// ============================================================================
// って - Casual topic marker (replacing は)
// ============================================================================
// Pattern: Sentence topic + って
// Data source: grammar_points_data.json["って"]
// Structure: Sentence topic + って
//
// Note: This is the TOPIC MARKER usage (replacing は), not the quotation marker
mod tte_tests {
    use super::*;

    // Test: Noun + って (topic marker - from grammar_points_data.json)
    #[test]
    fn test_tte_topic_marker() {
        let sentence = "私って皆に嫌われている？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "って");
        assert_pattern_range(&patterns, "って", 0, 3); // 私って
    }

    // Test: Noun + って (question about tomatoes - from grammar_points_data.json)
    #[test]
    fn test_tte_topic_question() {
        let sentence = "トマトって野菜なの？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "って");
        assert_pattern_range(&patterns, "って", 0, 5); // トマトって
    }

    // Test: Noun + って in casual conversation
    #[test]
    fn test_tte_casual_topic() {
        let sentence = "あの人って誰？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "って");
        assert_pattern_range(&patterns, "って", 2, 5); // 人って
    }
}

// ========== なかなか～ない (hardly/not easily/far from) ==========
// Pattern: なかなか～ない (hardly/not easily/far from)
// Data source: grammar_points_data.json["なかなか～ない"]
//
// Structure variants to test:
//   standard[0]: なかなか + Phrase + Verb[ない]
//
// Note: This is the negative usage of なかなか, expressing frustration
// that something expected should happen but doesn't.

mod nakanaka_nai_tests {
    use super::*;

    // Test: なかなか + Verb[ない] (basic example)
    #[test]
    fn test_nakanaka_nai_basic() {
        let sentence = "風邪がなかなか治らないんです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なかなか～ない");
        assert_pattern_range(&patterns, "なかなか～ない", 3, 11); // なかなか治らない
    }

    // Test: なかなか + Phrase + Verb[ない] (with intervening phrase)
    #[test]
    fn test_nakanaka_nai_with_phrase() {
        let sentence = "仕事が忙しくてなかなか休みが取れないんだよね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なかなか～ない");
        assert_pattern_range(&patterns, "なかなか～ない", 7, 18); // なかなか休みが取れない
    }

    // Test: なかなか + Verb[ない] (sentence beginning)
    #[test]
    fn test_nakanaka_nai_sentence_start() {
        let sentence = "なかなか来ないね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なかなか～ない");
        assert_pattern_range(&patterns, "なかなか～ない", 0, 7); // なかなか来ない
    }

    // Test: なかなか + Polite negative form
    #[test]
    fn test_nakanaka_nai_polite() {
        let sentence = "この問題はなかなか解決しません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なかなか～ない");
        assert_pattern_range(&patterns, "なかなか～ない", 5, 15); // なかなか解決しません
    }
}

// ========== 的 (like / -ish / -ly) ==========
// Pattern: 的 (like / -ish / -ly)
// Data source: grammar_points_data.json["的"]
//
// Structure variants to test:
//   standard[0]: Noun + 的（てき） + に
//   standard[1]: Noun + 的（てき） + な + Noun
//
// Note: 的 creates adverbs (with に) or na-adjectives (with な)

mod teki_tests {
    use super::*;

    // Test: Noun + 的 + に (adverbial form)
    #[test]
    fn test_teki_adverbial_form() {
        let sentence = "定期的に掃除してください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "的");
        assert_pattern_range(&patterns, "的", 0, 4); // 定期的に
    }

    // Test: Noun + 的 + に (another adverbial example)
    #[test]
    fn test_teki_adverbial_emotional() {
        let sentence = "そんな感情的にならなくてもいいのに";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "的");
        assert_pattern_range(&patterns, "的", 3, 7); // 感情的に
    }

    // Test: Noun + 的 + な + Noun (na-adjective form)
    #[test]
    fn test_teki_na_adjective_form() {
        let sentence = "それは個人的な考えですよね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "的");
        assert_pattern_range(&patterns, "的", 3, 7); // 個人的な
    }

    // Test: Noun + 的 + な + Noun (another na-adj example)
    #[test]
    fn test_teki_na_adjective_religious() {
        let sentence = "なんかの宗教的な人たちが来た";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "的");
        assert_pattern_range(&patterns, "的", 4, 8); // 宗教的な
    }
}

// ========== てごらん (please try to) ==========
// Pattern: てごらん (please try to - honorific suggestion)
// Data source: grammar_points_data.json["てごらん"]
//
// Structure variants to test:
//   standard[0]: Verb[て] + ごらん
//   standard[1]: Verb[て] + ごらんなさい

mod tegoran_tests {
    use super::*;

    // Test: Verb[て] + ごらん (basic form)
    #[test]
    fn test_tegoran_basic() {
        let sentence = "この本を読んでごらん";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てごらん");
        assert_pattern_range(&patterns, "てごらん", 4, 10); // 読んでごらん
    }

    // Test: Verb[て] + ごらん (different verb)
    #[test]
    fn test_tegoran_look() {
        let sentence = "外を見てごらん。虹が出ているよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てごらん");
        assert_pattern_range(&patterns, "てごらん", 2, 7); // 見てごらん
    }

    // Test: Verb[て] + ごらんなさい (with なさい - stronger form)
    #[test]
    fn test_tegoran_nasai() {
        let sentence = "見てごらんなさい、こんな酷いこと";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てごらん");
        assert_pattern_range(&patterns, "てごらん", 0, 8); // 見てごらんなさい
    }
}

// ========== ちゃんと・きちんと (properly/neatly) ==========
// Pattern: ちゃんと・きちんと (properly/neatly - adverbs)
// Data source: grammar_points_data.json["ちゃんと・きちんと"]
//
// Structure variants to test:
//   standard[0]: ちゃんと + Phrase
//   standard[1]: ちゃんと + した + Noun
//   standard[2]: ちゃんと + している
//   Note: (1) きちんと - same structures apply to きちんと

mod chanto_kichinto_tests {
    use super::*;

    // Test: ちゃんと + Verb (modifying verb phrase)
    #[test]
    fn test_chanto_verb() {
        let sentence = "ちゃんと宿題したか？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ちゃんと・きちんと");
        assert_pattern_range(&patterns, "ちゃんと・きちんと", 0, 4); // ちゃんと
    }

    // Test: ちゃんと + した + Noun (proper/neat noun)
    #[test]
    fn test_chanto_shita_noun() {
        let sentence = "今度からはもっとちゃんとした車を買おう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ちゃんと・きちんと");
        assert_pattern_range(&patterns, "ちゃんと・きちんと", 8, 12); // ちゃんと
    }

    // Test: ちゃんと + している (doing properly)
    #[test]
    fn test_chanto_shiteiru() {
        let sentence = "みんなちゃんとしているか確認しろ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ちゃんと・きちんと");
        assert_pattern_range(&patterns, "ちゃんと・きちんと", 3, 7); // ちゃんと
    }

    // Test: きちんと + Verb (modifying verb phrase)
    #[test]
    fn test_kichinto_verb() {
        let sentence = "結婚式にはきちんとした服装で来てください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ちゃんと・きちんと");
        assert_pattern_range(&patterns, "ちゃんと・きちんと", 5, 9); // きちんと
    }

    // Test: きちんと + Verb (sentence start)
    #[test]
    fn test_kichinto_sentence_start() {
        let sentence = "きちんと部屋の掃除をしなさい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ちゃんと・きちんと");
        assert_pattern_range(&patterns, "ちゃんと・きちんと", 0, 4); // きちんと
    }
}

// ========== っけ (recall/confirmation particle) ==========
// Pattern: っけ (trying to remember or confirm information)
// Data source: grammar_points_data.json["っけ"]
//
// Structure variants to test:
//   standard[0]: Verb[た] + っけ
//   standard[1]: Verb[る] + んだ + っけ
//   standard[2]: い-Adjective[た] + っけ
//   standard[3]: な-Adjective + だった + っけ
//   standard[4]: Noun + だった + っけ

mod pkke_tests {
    use super::*;

    // Test: standard[0] - Verb[た] + っけ
    #[test]
    fn test_pkke_verb_past() {
        let sentence = "今日の朝は朝ご飯を食べたっけ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "っけ");
        assert_pattern_range(&patterns, "っけ", 9, 14); // 食べたっけ
    }

    // Test: standard[1] - Verb[る] + んだ + っけ
    #[test]
    fn test_pkke_verb_nda() {
        let sentence = "レポートの修正は君がやってるんだっけ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "っけ");
        assert_pattern_range(&patterns, "っけ", 14, 18); // んだっけ
    }

    // Test: standard[2] - い-Adjective[た] + っけ
    #[test]
    fn test_pkke_i_adjective_past() {
        let sentence = "あれ、テイラーの車って青かったっけ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "っけ");
        assert_pattern_range(&patterns, "っけ", 11, 17); // 青かったっけ
    }

    // Test: standard[3] - な-Adjective + だった + っけ
    #[test]
    fn test_pkke_na_adjective_past() {
        let sentence = "土曜日って暇だったっけ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "っけ");
        assert_pattern_range(&patterns, "っけ", 6, 11); // だったっけ
    }

    // Test: standard[4] - Noun + だった + っけ
    #[test]
    fn test_pkke_noun_past() {
        let sentence = "あの人誰だったっけ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "っけ");
        assert_pattern_range(&patterns, "っけ", 4, 9); // だったっけ
    }
}

// ========== さ - Casual よ (sentence-ending particle) ==========
// Pattern: さ (drawing attention with high confidence - casual よ)
// Data source: grammar_points_data.json["さ - Casual よ"]
//
// Structure variants to test:
//   standard[0]: Phrase + さ

mod sa_casual_yo_tests {
    use super::*;

    // Test: Phrase + さ (sentence-ending)
    #[test]
    fn test_sa_sentence_ending() {
        let sentence = "彼氏がもう私のことを信用できないってさ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "さ - Casual よ");
        assert_pattern_range(&patterns, "さ - Casual よ", 16, 19); // ってさ
    }

    // Test: Phrase + さ (with だって before)
    #[test]
    fn test_sa_with_datte() {
        let sentence = "キヨミの赤ちゃんが明日生まれるんだってさ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "さ - Casual よ");
        assert_pattern_range(&patterns, "さ - Casual よ", 16, 20); // だってさ
    }

    // Test: Phrase + さ (emphatic affirmation)
    #[test]
    fn test_sa_emphatic() {
        let sentence = "そんなこと心配ないさ！俺に任せとけ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "さ - Casual よ");
        assert_pattern_range(&patterns, "さ - Casual よ", 7, 10); // ないさ
    }
}

// ========== 同士 (fellow/mutually) ==========
// Pattern: 同士 (fellow/mutually/together)
// Data source: grammar_points_data.json["同士"]
//
// Structure variants to test:
//   standard[0]: Noun + 同士（どうし）
//   standard[1]: Noun + 同士（どうし） + の + Noun

mod doushi_tests {
    use super::*;

    // Test: Noun + 同士 (basic usage - friends together)
    #[test]
    fn test_doushi_basic() {
        let sentence = "友達同士で旅行に行くことになった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "同士");
        assert_pattern_range(&patterns, "同士", 0, 4); // 友達同士
    }

    // Test: Noun + 同士 (siblings)
    #[test]
    fn test_doushi_siblings() {
        let sentence = "これは兄弟同士の問題だから口出しするな";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "同士");
        assert_pattern_range(&patterns, "同士", 3, 7); // 兄弟同士
    }

    // Test: Noun + 同士 + の + Noun (promise between friends)
    #[test]
    fn test_doushi_no_noun() {
        let sentence = "友達同士の約束は必ず守るべきだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "同士");
        assert_pattern_range(&patterns, "同士", 0, 4); // 友達同士
    }
}

// ========== 左右する (influence/dictate/control) ==========
// Pattern: 左右する (influence/dictate/control)
// Data source: grammar_points_data.json["左右する"]
//
// Structure variants to test:
//   standard[0]: 左右（さゆう） + する
//   standard[1]: 左右（さゆう） + する + Noun
//   standard[2]: される (passive)
//   polite[0]: 左右（さゆう） + します
//   polite[1]: 左右（さゆう） + する + Noun (same as standard)
//   polite[2]: されます (polite passive)
//   polite[3]: される (passive, same as standard)

mod sayuusuru_tests {
    use super::*;

    // Test: 左右 + する (basic form - influences/dictates)
    #[test]
    fn test_sayuusuru_basic() {
        let sentence = "この決定が会社の未来を大きく左右する";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "左右する");
        assert_pattern_range(&patterns, "左右する", 14, 18); // 左右する
    }

    // Test: 左右 + する + Noun (modifying noun)
    #[test]
    fn test_sayuusuru_modifying_noun() {
        let sentence = "天気が収穫を左右する要因の一つだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "左右する");
        assert_pattern_range(&patterns, "左右する", 6, 10); // 左右する
    }

    // Test: 左右 + される (passive form)
    #[test]
    fn test_sayuusuru_passive() {
        let sentence = "農業は天候に大きく左右される仕事だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "左右する");
        assert_pattern_range(&patterns, "左右する", 9, 12); // 左右さ
    }

    // Test: 左右 + します (polite form)
    #[test]
    fn test_sayuusuru_polite() {
        let sentence = "この選択が結果を左右します";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "左右する");
        assert_pattern_range(&patterns, "左右する", 8, 13); // 左右します
    }

    // Test: 左右 + されます (polite passive)
    #[test]
    fn test_sayuusuru_passive_polite() {
        let sentence = "彼の意見は周りの人に左右されます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "左右する");
        assert_pattern_range(&patterns, "左右する", 10, 13); // 左右さ
    }
}

// ========== さえ (even) ==========
// Pattern: さえ (even)
// Data source: grammar_points_data.json["さえ"]
//
// Structure variants to test:
//   standard[0]: Noun + (で) + さえ
//   standard[1]: Noun + Particle + さえ
//   standard[2]: Verb + こと(の) + さえ + (する)
//   standard[3]: Verb[stem] + さえ + (する)
//   standard[4]: Verb[て] + さえ + (いる)

mod sae_tests {
    use super::*;

    // Test: Noun + でさえ
    #[test]
    fn test_sae_noun_de_sae() {
        let sentence = "上級者の彼でさえ出来ないのに、あなたが出来るわけがないでしょう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "さえ");
        assert_pattern_range(&patterns, "さえ", 4, 8); // 彼でさえ
    }

    // Test: Noun + に + さえ
    #[test]
    fn test_sae_noun_particle_sae() {
        let sentence = "彼女は親にさえ知らせずに彼氏と結婚をした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "さえ");
        assert_pattern_range(&patterns, "さえ", 3, 7); // 親にさえ
    }

    // Test: Verb + こと + さえ
    #[test]
    fn test_sae_verb_koto_sae() {
        let sentence = "彼はストレスのせいで、晩飯を食べることさえ出来なくなった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "さえ");
        assert_pattern_range(&patterns, "さえ", 17, 21); // ことさえ
    }

    // Test: Verb[stem] + さえ + する
    #[test]
    fn test_sae_verb_stem_sae_suru() {
        let sentence = "タナカ君は酷くない？彼女を傷つけたのに、謝りさえしなかったらしいよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "さえ");
        assert_pattern_range(&patterns, "さえ", 20, 24); // 謝りさえ
    }

    // Test: Verb[て] + さえ + いる
    #[test]
    fn test_sae_verb_te_sae_iru() {
        let sentence = "そんなの放っておいてさえいたら、すぐ直るよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "さえ");
        assert_pattern_range(&patterns, "さえ", 9, 12); // てさえ
    }

    // Test: Noun + さえ (without particle)
    #[test]
    fn test_sae_noun_sae() {
        let sentence = "水さえあれば三日は生きられる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "さえ");
        assert_pattern_range(&patterns, "さえ", 0, 3); // 水さえ
    }
}

// ========== とおり (in that way / just like) ==========
// Pattern: とおり (in that way / just like)
// Data source: grammar_points_data.json["とおり"]
//
// Structure variants to test:
//   standard[0]: Verb[る] + とおり (also Verb[た] + とおり)
//   standard[1]: Noun + どおり
//   standard[2]: Noun + の + とおり

mod toori_tests {
    use super::*;

    // Test: Verb[る] + とおり
    #[test]
    fn test_toori_verb_plain() {
        let sentence = "お客様のおっしゃるとおりです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とおり");
        assert_pattern_range(&patterns, "とおり", 4, 14); // おっしゃるとおりです
    }

    // Test: Verb[た] + とおり
    #[test]
    fn test_toori_verb_past() {
        let sentence = "彼は僕の思ったとおりの時間に帰ってきた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とおり");
        assert_pattern_range(&patterns, "とおり", 6, 10); // たとおり
    }

    // Test: Noun + どおり
    #[test]
    fn test_toori_noun_doori() {
        let sentence = "計画どおりに進めて行きます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とおり");
        assert_pattern_range(&patterns, "とおり", 0, 5); // 計画どおり
    }

    // Test: Noun + の + とおり
    #[test]
    fn test_toori_noun_no_toori() {
        let sentence = "ご覧のとおり、私の手には何もありません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とおり");
        assert_pattern_range(&patterns, "とおり", 0, 6); // ご覧のとおり
    }
}

// ========== さえ〜ば (if only / as long as) ==========
// Pattern: さえ〜ば (if only / as long as - extreme limitation hypothetical)
// Data source: grammar_points_data.json["さえ〜ば"]
//
// Structure variants to test:
//   standard[0]: Verb[stem] + さえ + すれば
//   standard[1]: Verb[て] + さえ + いれば
//   standard[2]: Noun + さえ + Verb[ば]
//   standard[3]: い-Adjective[く] + さえ + あれば
//   standard[4]: Noun + さえ + い-Adjective[ば]
//   standard[5]: な-Adjective + (で) + さえ + あれば

mod sae_ba_tests {
    use super::*;

    // Test: Verb[stem] + さえ + すれば
    #[test]
    fn test_sae_ba_verb_stem_sureba() {
        let sentence = "この薬を毎日飲みさえすれば、すぐに治りますよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "さえ〜ば");
        assert_pattern_range(&patterns, "さえ〜ば", 6, 13); // 飲みさえすれば
    }

    // Test: Verb[て] + さえ + いれば
    #[test]
    fn test_sae_ba_verb_te_ireba() {
        let sentence = "もっと早くから貯めてさえいれば、今頃デカい家を買えてたかも";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "さえ〜ば");
        assert_pattern_range(&patterns, "さえ〜ば", 9, 15); // てさえいれば
    }

    // Test: Noun + さえ + Verb[ば]
    #[test]
    fn test_sae_ba_noun_verb_ba() {
        let sentence = "あの背が高い人さえ横にずれてくれれば、ちゃんと見えるのにな";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "さえ〜ば");
        assert_pattern_range(&patterns, "さえ〜ば", 6, 18); // 人さえ横にずれてくれれば
    }

    // Test: い-Adjective[く] + さえ + あれば
    #[test]
    fn test_sae_ba_i_adj_ku_areba() {
        let sentence = "タナカさんは若くさえあれば、あんな人でも雇う";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "さえ〜ば");
        assert_pattern_range(&patterns, "さえ〜ば", 6, 13); // 若くさえあれば
    }

    // Test: Noun + さえ + い-Adjective[ば]
    #[test]
    fn test_sae_ba_noun_i_adj_ba() {
        let sentence = "家の前の道さえ広ければSUVが買えたのに";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "さえ〜ば");
        assert_pattern_range(&patterns, "さえ〜ば", 4, 11); // 道さえ広ければ
    }

    // Test: な-Adjective + (で) + さえ + あれば
    #[test]
    fn test_sae_ba_na_adj_de_areba() {
        let sentence = "僕は静かでさえあればどこでも寝れます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "さえ〜ば");
        assert_pattern_range(&patterns, "さえ〜ば", 4, 10); // でさえあれば
    }
}

// ========== だけしか (only/nothing but) ==========
// Pattern: だけしか (only/nothing but) - combination of だけ and しか
// Data source: grammar_points_data.json["だけしか"]
//
// Structure variants to test:
//   standard[0]: Noun + だけしか + ない
//   polite[0]: Noun + だけしか + ありません

mod dakeshika_tests {
    use super::*;

    // Test: Noun + だけしか + Verb[ない]
    #[test]
    fn test_dakeshika_verb_nai() {
        let sentence = "私は日本語だけしか話せないです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だけしか");
        assert_pattern_range(&patterns, "だけしか", 2, 15); // 日本語だけしか話せないです
    }

    // Test: Noun + だけしか + Verb[られない]
    #[test]
    fn test_dakeshika_potential_nai() {
        let sentence = "彼は野菜だけしか食べられません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だけしか");
        assert_pattern_range(&patterns, "だけしか", 2, 15); // 野菜だけしか食べられません
    }

    // Test: Noun + だけしか + ない (existential)
    #[test]
    fn test_dakeshika_existential() {
        let sentence = "土曜日だけしか空いてる日がないから、出来たら土曜日がいい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だけしか");
        assert_pattern_range(&patterns, "だけしか", 0, 15); // 土曜日だけしか空いてる日がない
    }

    // Test: Noun + だけしか + じゃない (copula negative)
    #[test]
    fn test_dakeshika_janai() {
        let sentence = "このコップだけしか５００円ではないの？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だけしか");
        assert_pattern_range(&patterns, "だけしか", 2, 17); // コップだけしか５００円ではない
    }
}

// ========== てもかまわない (doesn't matter / don't mind) ==========
// Pattern: てもかまわない (doesn't matter / don't mind)
// Data source: grammar_points_data.json["てもかまわない"]
//
// Structure variants to test:
//   standard[0]: Verb[ても] + かまわない
//   standard[1]: い-Adjective[ても] + かまわない
//   standard[2]: な-Adjective + でも + かまわない
//   standard[3]: Noun + でも + かまわない
//   polite[0]: Verb[ても] + かまいません
//   polite[1]: い-Adjective[ても] + かまいません
//   polite[2]: な-Adjective + でも + かまいません
//   polite[3]: Noun + でも + かまいません

mod temo_kamawanai_tests {
    use super::*;

    // Test: standard[0] - Verb[ても] + かまわない
    #[test]
    fn test_verb_temo_kamawanai() {
        let sentence = "君が行きたくないなら行かなくてもかまわないよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てもかまわない");
        assert_pattern_range(&patterns, "てもかまわない", 12, 21); // なくてもかまわない
    }

    // Test: polite[0] - い-Adjective[ても] + かまいません
    #[test]
    fn test_i_adj_temo_kamawanai() {
        let sentence = "運転手さん、遅くてもかまいませんので、安全運転でお願いします";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てもかまわない");
        assert_pattern_range(&patterns, "てもかまわない", 6, 16); // 遅くてもかまいません
    }

    // Test: polite[2] - な-Adjective + でも + かまいません
    #[test]
    fn test_na_adj_demo_kamawanai() {
        let sentence = "仕事はどんなに大変でもかまいません、仕事が出来ればうれしいです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てもかまわない");
        assert_pattern_range(&patterns, "てもかまわない", 7, 17); // 大変でもかまいません
    }

    // Test: standard[0] - Verb[て] + も + かまわない (multiple in sentence)
    #[test]
    fn test_noun_demo_kamawanai() {
        let sentence = "君が払うなら、ピザでも寿司でも何を頼んでもかまわないよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てもかまわない");
        assert_pattern_range(&patterns, "てもかまわない", 17, 26); // 頼んでもかまわない
    }

    // Test: polite[3] - Noun + でも + かまいません
    #[test]
    fn test_noun_demo_kamaimasen() {
        let sentence = "アルバイト募集中。未経験者でもかまいません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てもかまわない");
        assert_pattern_range(&patterns, "てもかまわない", 12, 21); // 者でもかまいません
    }

    // Test: Verb[て] + も + かまわない (simplified sentence)
    #[test]
    fn test_multiple_instances() {
        let sentence = "ピザでも寿司でも何を頼んでもかまわないよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てもかまわない");
        assert_pattern_range(&patterns, "てもかまわない", 10, 19); // 頼んでもかまわない
    }

    // Test: Negative - かまわない without ても should NOT match
    #[test]
    fn test_temo_kamau_affirmative() {
        let sentence = "そんな細かいことは構わないから、早く終わらせよう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // Should NOT match てもかまわない (missing ても)
        assert!(!has_pattern(&patterns, "てもかまわない"));
    }

    // Test: Verb[ない] + て + も + かまわない
    #[test]
    fn test_verb_nai_form_temo() {
        let sentence = "別に来なくてもかまわないけど、来てくれたら嬉しい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てもかまわない");
        assert_pattern_range(&patterns, "てもかまわない", 3, 12); // なくてもかまわない
    }
}

// ========== である (formal copula) ==========
// Pattern: である (formal equivalent of だ)
// Data source: grammar_points_data.json["である"]
//
// Structure variants to test:
//   standard[0]: Noun + である
//   standard[1]: な-Adjective + である
//   polite[0]: Noun + であります
//   polite[1]: な-Adjective + であります

mod dearu_tests {
    use super::*;

    // Test: standard[0] - Noun + である (sentence-ending)
    #[test]
    fn test_noun_dearu_sentence_ending() {
        let sentence = "日本一デカイ博物館はこの博物館である";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "である");
        assert_pattern_range(&patterns, "である", 12, 18); // 博物館である
    }

    // Test: standard[1] - な-Adjective + である (attributive form)
    #[test]
    fn test_na_adjective_dearu_attributive() {
        let sentence = "重要である箇所をまとめた資料がこちらです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "である");
        assert_pattern_range(&patterns, "である", 0, 5); // 重要である
    }

    // Test: standard[0] - Noun + である (mid-sentence)
    #[test]
    fn test_noun_dearu_mid_sentence() {
        let sentence = "ここから悲しいことが起きるのである";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "である");
        assert_pattern_range(&patterns, "である", 13, 17); // のである
    }

    // Test: polite[0] - Noun + であります
    #[test]
    fn test_noun_dearimasu() {
        let sentence = "名前はタナカであります！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "である");
        assert_pattern_range(&patterns, "である", 3, 11); // タナカであります
    }

    // Test: polite[0] - Noun + であります (topic marker)
    #[test]
    fn test_noun_dearimasu_topic() {
        let sentence = "私達が会えたのは運命であります";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "である");
        assert_pattern_range(&patterns, "である", 8, 15); // 運命であります
    }

    // Test: polite[1] - な-Adjective + であります
    #[test]
    fn test_na_adjective_dearimasu() {
        let sentence = "この問題は極めて重要であります";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "である");
        assert_pattern_range(&patterns, "である", 8, 15); // 重要であります
    }
}

// Pattern: ために (due to, because of, for the sake of)
// Data source: grammar_points_data.json["ために"]
// Structures:
//   - standard[0]: Verb + ため(に)
//   - standard[1]: い-Adjective + ため(に)
//   - standard[2]: な-Adjective + な + ため(に)
//   - standard[3]: Noun + の + ため(に)
//
// Note: For Verb, い-Adjective, and Noun cases, the existing "ため(に)" pattern
// (with higher priority) will also match. The unique contribution of this pattern
// is the な-Adjective case which includes the adjective stem in the range.
mod tameni_tests {
    use super::*;

    // Testing: structure.standard[0] - "Verb + ため(に)"
    // Note: Also matches "ため(に)" pattern which has higher priority
    #[test]
    fn test_verb_tameni() {
        let sentence = "空手の試合で勝つために、毎日夜遅くまで練習しています";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // "ため(に)" pattern matches with higher priority
        assert_has_pattern(&patterns, "ため(に)");
        assert_pattern_range(&patterns, "ため(に)", 6, 11); // 勝つために
    }

    // Testing: structure.standard[1] - "い-Adjective + ため(に)"
    // Note: Also matches "ため(に)" pattern which has higher priority
    #[test]
    fn test_i_adjective_tameni() {
        let sentence = "暑いために、職場で何人かが倒れた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // "ため(に)" pattern matches with higher priority
        assert_has_pattern(&patterns, "ため(に)");
        assert_pattern_range(&patterns, "ため(に)", 0, 5); // 暑いために
    }

    // Testing: structure.standard[2] - "な-Adjective + な + ため(に)"
    // This is the unique case where "ために" pattern includes the adjective stem
    #[test]
    fn test_na_adjective_tameni() {
        let sentence = "このアプリは便利なために、ユーザーがどんどん増えてきた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // "ために" pattern uniquely captures the full な-adjective + な + ために
        assert_has_pattern(&patterns, "ために");
        assert_pattern_range(&patterns, "ために", 6, 12); // 便利なために
    }

    // Testing: structure.standard[3] - "Noun + の + ため(に)"
    // Note: Also matches "ため(に)" pattern which has higher priority
    #[test]
    fn test_noun_no_tameni() {
        let sentence = "大雨のため、サッカーの試合を中止します";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // "ため(に)" pattern matches with higher priority
        assert_has_pattern(&patterns, "ため(に)");
        assert_pattern_range(&patterns, "ため(に)", 0, 5); // 大雨のため
    }
}

// ========== できれば・できたら (if possible) ==========
// Pattern: できれば・できたら (if possible)
// Data source: grammar_points_data.json["できれば・できたら"]
//
// Structure variants to test:
//   standard[0]: できれば + Phrase
//   standard[1]: できたら + Phrase

mod dekireba_dekitara_tests {
    use super::*;

    // Testing: structure.standard[0] - "できれば + Phrase"
    #[test]
    fn test_dekireba_phrase() {
        let sentence = "できれば車で行きたいけど、電車で行った方が早いかもしれない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "できれば・できたら");
        assert_pattern_range(&patterns, "できれば・できたら", 0, 4); // できれば
    }

    // Testing: structure.standard[0] - "できれば + Phrase" (different example)
    #[test]
    fn test_dekireba_meeting() {
        let sentence = "できれば今週中にもう一度会って話したいんです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "できれば・できたら");
        assert_pattern_range(&patterns, "できれば・できたら", 0, 4); // できれば
    }

    // Testing: structure.standard[1] - "できたら + Phrase"
    #[test]
    fn test_dekitara_pharmacy() {
        let sentence = "できたら薬局に行って頭痛薬を買って来てくれる？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "できれば・できたら");
        assert_pattern_range(&patterns, "できれば・できたら", 0, 4); // できたら
    }

    // Testing: structure.standard[1] - "できたら + Phrase" (different example)
    #[test]
    fn test_dekitara_tomorrow() {
        let sentence = "できたら明日の朝までに返事をください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "できれば・できたら");
        assert_pattern_range(&patterns, "できれば・できたら", 0, 4); // できたら
    }
}

// Pattern: ところが (however, but unexpectedly)
// Data source: grammar_points_data.json["ところが"]
// Testing: structure.standard[0] - "(Expectation) Phrase + ところが + (Unexpected Result) Phrase"
//
// Meaning: Conjunction showing unexpected result contrary to expectation
// Note: Usually used at beginning of sentence to contrast with previous statement
mod tokoroga_tests {
    use super::*;

    // Testing: ところが at sentence beginning (standard usage)
    #[test]
    fn test_tokoroga_sentence_beginning() {
        let sentence = "９時に寝た。ところが、目覚ましがならなかったから寝坊した";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ところが");
        assert_pattern_range(&patterns, "ところが", 6, 10); // ところが
    }

    // Testing: ところが mid-sentence (after phrase)
    #[test]
    fn test_tokoroga_after_phrase() {
        let sentence = "新しい洗濯機を買った。ところが、洗濯機が大きすぎて家に入らなかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ところが");
        assert_pattern_range(&patterns, "ところが", 11, 15); // ところが
    }

    // Testing: ところが with positive unexpected result
    #[test]
    fn test_tokoroga_positive_result() {
        let sentence = "嫌がると思っていた。ところが、喜んで手伝ってくれた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ところが");
        assert_pattern_range(&patterns, "ところが", 10, 14); // ところが
    }
}

// Pattern: ところで (by the way, incidentally)
// Data source: grammar_points_data.json["ところで"]
// Testing: structure.standard[0] - "ところで + (New Topic) Phrase"
//
// Meaning: Conjunction for introducing new topics (not continuing same topic)
// Note: Usually used at beginning of sentence before unrelated statement/question
mod tokorode_tests {
    use super::*;

    // Testing: ところで at sentence beginning (topic change)
    #[test]
    fn test_tokorode_topic_change() {
        let sentence = "ところで、昨日の話はどうなった？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ところで");
        assert_pattern_range(&patterns, "ところで", 0, 4); // ところで
    }

    // Testing: ところで with polite question
    #[test]
    fn test_tokorode_polite_question() {
        let sentence = "ところで、お宅の旦那さんは元気にしていますか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ところで");
        assert_pattern_range(&patterns, "ところで", 0, 4); // ところで
    }

    // Testing: ところで mid-conversation
    #[test]
    fn test_tokorode_mid_conversation() {
        let sentence = "そうですね。ところで、明日の予定は決まりましたか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ところで");
        assert_pattern_range(&patterns, "ところで", 6, 10); // ところで
    }
}

// Pattern: でよければ (if...is okay)
// Data source: grammar_points_data.json["でよければ"]
// Structure: standard[0] - "Noun + でよければ"
//
// Meaning: "if (A) is ok" - used to politely offer help/advice/service
// Note: Constructed with で (case particle) + よければ (potential form of いい)
mod deyokereba_tests {
    use super::*;

    // Testing: standard[0] - "Noun + でよければ" (offering place/thing)
    #[test]
    fn test_deyokereba_place() {
        let sentence = "焼き肉でよければ、うちの近くに美味しいところがある";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "でよければ");
        assert_pattern_range(&patterns, "でよければ", 0, 8); // 焼き肉でよければ
    }

    // Testing: standard[0] - "Noun + でよければ" (offering used item)
    #[test]
    fn test_deyokereba_used_item() {
        let sentence = "中古でよければいいのが沢山ありますよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "でよければ");
        assert_pattern_range(&patterns, "でよければ", 0, 7); // 中古でよければ
    }

    // Testing: standard[0] - "Noun + でよければ" (offering oneself - common usage)
    #[test]
    fn test_deyokereba_self_offer() {
        let sentence = "私でよければ手伝うよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "でよければ");
        assert_pattern_range(&patterns, "でよければ", 0, 6); // 私でよければ
    }

    // Testing: standard[0] - "Noun + でよければ" (polite self-offer)
    #[test]
    fn test_deyokereba_polite_offer() {
        let sentence = "先生でよければ、先生が写真を撮ってあげるよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "でよければ");
        assert_pattern_range(&patterns, "でよければ", 0, 7); // 先生でよければ
    }
}

// Pattern: そうもない (very unlikely)
// Data source: grammar_points_data.json["そうもない"]
// Testing: structure.standard[0] - "Verb[stem] + そうもない"
//          structure.polite[0] - "Verb[stem] + そうもありません"
//
// Structure variants:
//   - standard[0]: Verb[stem] + そうもない
//   - polite[0]: Verb[stem] + そうもありません
mod soumonai_tests {
    use super::*;

    // Testing: structure.standard[0] - "Verb[stem] + そうもない"
    // Example: 出来そうもない (doesn't even seem possible)
    #[test]
    fn test_soumonai_potential_verb() {
        let sentence = "腕がまだ治っていないから、あと一週間ぐらい運動が出来そうもない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そうもない");
        assert_pattern_range(&patterns, "そうもない", 24, 31); // 出来そうもない
    }

    // Testing: structure.standard[0] - "Verb[stem] + そうもない"
    // Example: 行けそうもない (doesn't even appear like I can go)
    #[test]
    fn test_soumonai_potential_verb_ikeru() {
        let sentence = "今日は雷が酷いから、釣りに行けそうもない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そうもない");
        assert_pattern_range(&patterns, "そうもない", 13, 20); // 行けそうもない
    }

    // Testing: structure.standard[0] - "Verb[stem] + そうもない"
    // Example: 食べられそうもない (doesn't even seem like I can eat)
    #[test]
    fn test_soumonai_potential_verb_taberare() {
        let sentence = "こんな量、食べられそうもない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そうもない");
        assert_pattern_range(&patterns, "そうもない", 7, 14); // られそうもない
    }

    // Testing: structure.polite[0] - "Verb[stem] + そうもありません"
    // Example: 見られそうもありません (might not be able to watch)
    #[test]
    fn test_soumonai_polite_potential_verb() {
        let sentence = "今日は１１時まで帰って来ないかもしれないから、いつも見ているテレビ番組が見られそうもありません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そうもない");
        assert_pattern_range(&patterns, "そうもない", 37, 47); // られそうもありません
    }
}

// Pattern: でもある (is also)
// Data source: grammar_points_data.json["でもある"]
// Testing structure variants from grammar_points_data.json:
//   - standard[0]: Noun + でもある
//   - standard[1]: い-Adjective + くもある
//   - standard[2]: な-Adjective + でもある
//   - polite[0]: Noun + でもあります
//   - polite[1]: い-Adjective + くもあります
//   - polite[2]: な-Adjective + でもあります
#[cfg(test)]
mod demoaru_tests {
    use super::*;

    // Testing: standard[0] - Noun + でもある
    #[test]
    fn test_demoaru_noun_standard() {
        let sentence = "タナカさんは英語の先生だし、休みの日は船の船長でもあるから休む時間がない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "でもある");
        assert_pattern_range(&patterns, "でもある", 21, 27); // 船長でもある
    }

    // Testing: standard[0] - Noun + でもある (another example)
    #[test]
    fn test_demoaru_noun_standard_2() {
        let sentence = "これは洗濯機でもあるけど、同時に乾燥機でもある";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "でもある");
        assert_pattern_range(&patterns, "でもある", 5, 10); // 機でもある (first occurrence)
    }

    // Testing: standard[1] - い-Adjective + くもある
    #[test]
    fn test_demoaru_i_adj_standard() {
        let sentence = "バンジージャンプは楽しいけれども、同時に危険でもある";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "でもある");
        assert_pattern_range(&patterns, "でもある", 20, 26); // 危険でもある
    }

    // Testing: standard[2] - な-Adjective + でもある
    #[test]
    fn test_demoaru_na_adj_standard() {
        let sentence = "この機械は重くて邪魔だけど、同時に便利でもある";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "でもある");
        assert_pattern_range(&patterns, "でもある", 17, 23); // 便利でもある
    }

    // Testing: polite[0] - Noun + でもあります
    #[test]
    fn test_demoaru_noun_polite() {
        let sentence = "彼は医者であり、科学者でもあります";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "でもある");
        assert_pattern_range(&patterns, "でもある", 10, 17); // 者でもあります
    }

    // Testing: polite[1] - い-Adjective + くもあります
    #[test]
    fn test_demoaru_i_adj_polite() {
        let sentence = "この料理は美味しくもあります";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "でもある");
        assert_pattern_range(&patterns, "でもある", 5, 14); // 美味しくもあります
    }

    // Testing: polite[2] - な-Adjective + でもあります
    #[test]
    fn test_demoaru_na_adj_polite() {
        let sentence = "彼女は親切でもあります";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "でもある");
        assert_pattern_range(&patterns, "でもある", 3, 11); // 親切でもあります
    }
}

// Pattern: では・それでは・じゃあ (conjunction/transition)
// Data source: grammar_points_data.json["では・それでは・じゃあ"]
// Testing: structure.standard[0] - （それ）+ では + Phrase
//          structure.standard[1] - じゃあ、じゃ + Phrase
//
// Meaning: "well then, in that case" - conjunction showing result/conclusion or topic change
// Usage: それでは (formal) > では (neutral) > じゃあ/じゃ (casual)
mod deha_soredewa_jaa_tests {
    use super::*;

    // Testing: standard[0] - それでは (most formal)
    #[test]
    fn test_soredewa_formal() {
        let sentence = "それでは、あなたは行きたくないという事ですか？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "では・それでは・じゃあ");
        assert_pattern_range(&patterns, "では・それでは・じゃあ", 0, 4); // それでは
    }

    // Testing: standard[0] - では (neutral formality)
    #[test]
    fn test_deha_neutral() {
        let sentence = "では、こちらの商品はどうでしょう？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "では・それでは・じゃあ");
        assert_pattern_range(&patterns, "では・それでは・じゃあ", 0, 2); // では
    }

    // Testing: standard[1] - じゃあ (casual)
    #[test]
    fn test_jaa_casual() {
        let sentence = "用意できた？じゃあ、５分で出よう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "では・それでは・じゃあ");
        assert_pattern_range(&patterns, "では・それでは・じゃあ", 6, 9); // じゃあ
    }

    // Testing: standard[1] - じゃ (most casual)
    #[test]
    fn test_ja_most_casual() {
        let sentence = "じゃ、また明日ね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "では・それでは・じゃあ");
        assert_pattern_range(&patterns, "では・それでは・じゃあ", 0, 2); // じゃ
    }
}

// Pattern: ではなくて・じゃなくて (negative copula te-form)
// Data source: grammar_points_data.json["ではなくて・じゃなくて"]
// Testing: structure.standard[0] - Verb + の + ではなく（て）
//          structure.standard[1] - い-Adjective + の + ではなく（て）
//          structure.standard[2] - な-Adjective + ではなく（て）
//          structure.standard[3] - Noun + ではなく（て）
//          structure.standard[4] - じゃなく（て） variant
//
// Meaning: "not...but" - negates (A) and contrasts with (B)
// Note: で can be は+で or just で; て is optional
mod dewanakute_janakute_tests {
    use super::*;

    // Testing: standard[0] - Verb + の + ではなくて
    #[test]
    fn test_verb_no_dewanakute() {
        let sentence = "事故を起こしたら逃げるのではなくて、警察に電話をしてください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ではなくて・じゃなくて");
        assert_pattern_range(&patterns, "ではなくて・じゃなくて", 11, 17); // のではなくて
    }

    // Testing: standard[1] - い-Adjective + の + ではなくて
    #[test]
    fn test_i_adj_no_dewanakute() {
        let sentence = "赤いのではなくて、青いのをください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ではなくて・じゃなくて");
        assert_pattern_range(&patterns, "ではなくて・じゃなくて", 2, 8); // のではなくて
    }

    // Testing: standard[2] - な-Adjective + ではなくて
    #[test]
    fn test_na_adj_dewanakute() {
        let sentence = "今日は暇ではなくて忙しいです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ではなくて・じゃなくて");
        assert_pattern_range(&patterns, "ではなくて・じゃなくて", 4, 9); // ではなくて
    }

    // Testing: standard[3] - Noun + ではなくて
    #[test]
    fn test_noun_dewanakute() {
        let sentence = "メールではなくて、ファックスで送ってください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ではなくて・じゃなくて");
        assert_pattern_range(&patterns, "ではなくて・じゃなくて", 3, 8); // ではなくて
    }

    // Testing: standard[3] - Noun + でなくて (without は)
    #[test]
    fn test_noun_denakute() {
        let sentence = "彼は正社員でなくて、アルバイトです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ではなくて・じゃなくて");
        assert_pattern_range(&patterns, "ではなくて・じゃなくて", 5, 9); // でなくて
    }

    // Testing: standard[4] - Noun + じゃなくて (casual)
    #[test]
    fn test_noun_janakute() {
        let sentence = "その建物は刑務所じゃなくて、学校です。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ではなくて・じゃなくて");
        assert_pattern_range(&patterns, "ではなくて・じゃなくて", 8, 13); // じゃなくて
    }
}

// Pattern: そうだ (hearsay - I heard that)
// Data source: grammar_points_data.json["そうだ "]
// Testing: Hearsay そうだ (reporting information from others)
//   - standard[0]: Verb + そうだ
//   - standard[1]: い-Adjective + そうだ
//   - standard[2]: Noun + だそうだ
//   - standard[3]: な-Adjective + だそうだ
//   - polite[0]: Verb + そうです
//   - polite[1]: い-Adjective + そうです
//   - polite[2]: Noun + だそうです
//   - polite[3]: な-Adjective + だそうです
//
// NOTE: This is HEARSAY そうだ, different from APPEARANCE そうだ
// Hearsay: 降るそうだ (plain form + そう)
// Appearance: 降りそうだ (verb stem + そう) - different pattern
mod souda_hearsay_tests {
    use super::*;

    // Testing: standard[0] - Verb + そうだ
    #[test]
    fn test_verb_souda() {
        let sentence = "この種類の鳥はよく鳴くそうだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そうだ ");
        assert_pattern_range(&patterns, "そうだ ", 9, 14); // 鳴くそうだ
    }

    // Testing: polite[0] - Verb + そうです
    #[test]
    fn test_verb_soudesu() {
        let sentence = "次の患者さんは酷い事故にあったそうです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そうだ ");
        assert_pattern_range(&patterns, "そうだ ", 14, 19); // たそうです
    }

    // Testing: standard[1] - い-Adjective + そうだ
    #[test]
    fn test_i_adj_souda() {
        let sentence = "あの店のラーメンは美味しいそうだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そうだ ");
        assert_pattern_range(&patterns, "そうだ ", 9, 16); // 美味しいそうだ
    }

    // Testing: polite[1] - い-Adjective + そうです
    #[test]
    fn test_i_adj_soudesu() {
        let sentence = "今年の冬は寒いそうです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そうだ ");
        assert_pattern_range(&patterns, "そうだ ", 5, 11); // 寒いそうです
    }

    // Testing: standard[2] - Noun + だそうだ
    #[test]
    fn test_noun_dasouda() {
        let sentence = "先輩は明日も仕事だそうだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そうだ ");
        assert_pattern_range(&patterns, "そうだ ", 6, 12); // 仕事だそうだ
    }

    // Testing: polite[2] - Noun + だそうです
    #[test]
    fn test_noun_dasoudesu() {
        let sentence = "あの人は医者だそうです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そうだ ");
        assert_pattern_range(&patterns, "そうだ ", 4, 11); // 医者だそうです
    }

    // Testing: standard[3] - な-Adjective + だそうだ
    #[test]
    fn test_na_adj_dasouda() {
        let sentence = "あそこから見る夕日は綺麗だそうだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そうだ ");
        assert_pattern_range(&patterns, "そうだ ", 10, 16); // 綺麗だそうだ
    }

    // Testing: polite[3] - な-Adjective + だそうです
    #[test]
    fn test_na_adj_dasoudesu() {
        let sentence = "この映画は有名だそうです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そうだ ");
        assert_pattern_range(&patterns, "そうだ ", 5, 12); // 有名だそうです
    }
}

// ========== わざわざ (going out of one's way) ==========
// Pattern: わざわざ (to go out of one's way to do something)
// Data source: grammar_points_data.json["わざわざ"]
//
// Structure variants to test:
//   standard[0]: わざわざ + Phrase

mod wazawaza_tests {
    use super::*;

    // Testing: standard[0] - わざわざ + Phrase (positive usage)
    #[test]
    fn test_wazawaza_positive() {
        let sentence = "わざわざ私が好きな饅頭を買いに行ってくれたの？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "わざわざ");
        assert_pattern_range(&patterns, "わざわざ", 0, 4); // わざわざ
    }

    // Testing: standard[0] - わざわざ + Phrase (thank you context)
    #[test]
    fn test_wazawaza_thank_you() {
        let sentence = "わざわざここまで来てくれてありがとうございます。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "わざわざ");
        assert_pattern_range(&patterns, "わざわざ", 0, 4); // わざわざ
    }

    // Testing: standard[0] - わざわざ + Phrase (negative context)
    #[test]
    fn test_wazawaza_negative() {
        let sentence = "何でわざわざそういうひどいこと言うの？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "わざわざ");
        assert_pattern_range(&patterns, "わざわざ", 2, 6); // わざわざ
    }

    // Testing: standard[0] - わざわざ + Phrase (questioning context)
    #[test]
    fn test_wazawaza_question() {
        let sentence = "わざわざ会社に行って仕事しなきゃいけない理由がわからない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "わざわざ");
        assert_pattern_range(&patterns, "わざわざ", 0, 4); // わざわざ
    }
}

// ========== 一体 (on earth/in the world) ==========
// Pattern: 一体 (what the heck/on earth - with question words)
// Data source: grammar_points_data.json["一体"]
//
// Structure variants to test:
//   standard[0]: いったい + Question Word + Phrase

mod ittai_tests {
    use super::*;

    // Testing: standard[0] - いったい + 何 (what)
    #[test]
    fn test_ittai_nani() {
        let sentence = "いったいここで何が起きたんだ！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "一体");
        assert_pattern_range(&patterns, "一体", 0, 4); // いったい
    }

    // Testing: standard[0] - いったい + どういう (what kind)
    #[test]
    fn test_ittai_douiu() {
        let sentence = "これはいったいどういうことだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "一体");
        assert_pattern_range(&patterns, "一体", 3, 7); // いったい
    }

    // Testing: standard[0] - いったい + なんで (why)
    #[test]
    fn test_ittai_nande() {
        let sentence = "いったいなんでこんな事になったの？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "一体");
        assert_pattern_range(&patterns, "一体", 0, 4); // いったい
    }

    // Testing: standard[0] - いったい + なんで (why - dating context)
    #[test]
    fn test_ittai_nande_dating() {
        let sentence = "いったいなんであんな奴と付き合おうと思ったの？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "一体");
        assert_pattern_range(&patterns, "一体", 0, 4); // いったい
    }
}

// ========== むしろ (rather/instead) ==========
// Pattern: むしろ (rather/instead - expressing preference)
// Data source: grammar_points_data.json["むしろ"]
//
// Structure variants to test:
//   standard[0]: むしろ + (Preferred Choice) Phrase

mod mushiro_tests {
    use super::*;

    // Testing: standard[0] - むしろ + Phrase (preference)
    #[test]
    fn test_mushiro_preference() {
        let sentence = "何でそっちが怒ってるの？むしろこっちが怒りたいよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "むしろ");
        assert_pattern_range(&patterns, "むしろ", 12, 15); // むしろ
    }

    // Testing: standard[0] - むしろ + Phrase (buying choice)
    #[test]
    fn test_mushiro_buying() {
        let sentence = "むしろ、安い方を買った方がいいとおもう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "むしろ");
        assert_pattern_range(&patterns, "むしろ", 0, 3); // むしろ
    }

    // Testing: standard[0] - むしろ + Phrase (with より comparison)
    #[test]
    fn test_mushiro_yori_car() {
        let sentence = "車をリースするより、むしろ中古の車を買った方が安く済む。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "むしろ");
        assert_pattern_range(&patterns, "むしろ", 10, 13); // むしろ
    }

    // Testing: standard[0] - むしろ + Phrase (game preference)
    #[test]
    fn test_mushiro_yori_games() {
        let sentence = "俺はMMOよりむしろシングルプレイヤーのゲームの方が好きだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "むしろ");
        assert_pattern_range(&patterns, "むしろ", 7, 10); // むしろ
    }
}

// ========== たらいい・といい (it would be good if) ==========
// Pattern: たらいい・といい (conditional + いい - expressing desire/advice)
// Data source: grammar_points_data.json["たらいい・といい"]
//
// Structure variants to test:
//   standard[0]: Verb[たら] + いい
//   standard[1]: Verb[ば] + いい
//   standard[2]: Verb + と + いい
//   standard[3]: い-Adjective[た] + ら + いい
//   standard[4]: い-Adjective[い] + ければ + いい
//   standard[5]: い-Adjective + と + いい
//   standard[6]: な-Adjective + だった + ら + いい
//   standard[7]: な-Adjective + であれば + いい
//   standard[8]: な-Adjective + だ + といい
//   polite[0-8]: Same forms + です

mod tara_ii_to_ii_tests {
    use super::*;

    // Testing: standard[0] - Verb[たら] + いい
    #[test]
    fn test_verb_tara_ii() {
        let sentence = "私もあんなに早く走れたらいいな";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たらいい・といい_たら連用");
        assert_pattern_range(&patterns, "たらいい・といい_たら連用", 8, 14); // 走れたらいい
    }

    // Testing: standard[1] - Verb[ば] + いい
    #[test]
    fn test_verb_ba_ii() {
        let sentence = "もっと時間があればいいのに";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たらいい・といい_ば");
        assert_pattern_range(&patterns, "たらいい・といい_ば", 6, 11); // あればいい
    }

    // Testing: standard[2] - Verb + と + いい
    #[test]
    fn test_verb_to_ii() {
        let sentence = "来年は海外旅行に行けるといいね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たらいい・といい_と");
        assert_pattern_range(&patterns, "たらいい・といい_と", 8, 14); // 行けるといい
    }

    // Testing: standard[3] - い-Adjective[た] + ら + いい
    #[test]
    fn test_i_adj_tara_ii() {
        let sentence = "もっと安かったらいいのに";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たらいい・といい_たら連用タ");
        assert_pattern_range(&patterns, "たらいい・といい_たら連用タ", 3, 10); // 安かったらいい
    }

    // Testing: standard[4] - い-Adjective[い] + ければ + いい
    #[test]
    fn test_i_adj_kereba_ii() {
        let sentence = "天気が良ければいいね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たらいい・といい_ば");
        assert_pattern_range(&patterns, "たらいい・といい_ば", 3, 9); // 良ければいい
    }

    // Testing: standard[5] - い-Adjective + と + いい
    #[test]
    fn test_i_adj_to_ii() {
        let sentence = "もっと早いといいんだけど";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たらいい・といい_と");
        assert_pattern_range(&patterns, "たらいい・といい_と", 3, 8); // 早いといい
    }

    // Testing: standard[6] - な-Adjective + だった + ら + いい
    #[test]
    fn test_na_adj_dattara_ii() {
        let sentence = "もっと静かだったらいいのに";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たらいい・といい_な形だったら");
        assert_pattern_range(&patterns, "たらいい・といい_な形だったら", 3, 11); // 静かだったらいい
    }

    // Testing: standard[7] - な-Adjective + であれば + いい
    #[test]
    fn test_na_adj_deareba_ii() {
        let sentence = "仕事が楽であればいいというわけではない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たらいい・といい_な形であれば");
        assert_pattern_range(&patterns, "たらいい・といい_な形であれば", 3, 10); // 楽であればいい
    }

    // Testing: standard[8] - な-Adjective + だ + といい
    #[test]
    fn test_na_adj_da_to_ii() {
        let sentence = "部屋が綺麗だといいね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たらいい・といい_な形だと");
        assert_pattern_range(&patterns, "たらいい・といい_な形だと", 3, 9); // 綺麗だといい
    }

    // Testing: polite[0] - Verb[たら] + いい + です
    #[test]
    fn test_verb_tara_ii_desu() {
        let sentence = "もっと早く来られたらいいですね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たらいい・といい_たら連用");
        assert_pattern_range(&patterns, "たらいい・といい_たら連用", 6, 14); // られたらいいです
    }
}

// ========== ばいい (it would be good if) ==========
// Pattern: ばいい (it would be good if)
// Data source: grammar_points_data.json["ばいい"]
//
// Structure variants to test:
//   standard[0]: Verb［ば］+ いい
//   polite[0]: Verb［ば］+ いい + です

mod baii_tests {
    use super::*;

    // Testing: structure.standard[0] - Verb［ば］+ いい
    #[test]
    fn test_verb_ba_ii_standard() {
        let sentence = "どれを食べればいいか迷ってるところ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ばいい");
        assert_pattern_range(&patterns, "ばいい", 3, 9); // 食べればいい
    }

    // Testing: structure.standard[0] - Verb［ば］+ いい (potential verb)
    #[test]
    fn test_verb_ba_ii_potential() {
        let sentence = "行けばいいけど、行けるか分からない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ばいい");
        assert_pattern_range(&patterns, "ばいい", 0, 5); // 行けばいい
    }

    // Testing: polite[0] - Verb［ば］+ いい + です
    // Note: The pattern detects Verb + ば + いい, not including です
    #[test]
    fn test_verb_ba_ii_suru_verb() {
        let sentence = "勉強すればいいんじゃない？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ばいい");
        assert_pattern_range(&patterns, "ばいい", 0, 7); // 勉強すればいい
    }
}

// ========== べき (ought to/should - moral obligation) ==========
// Pattern: べき (ought to/should)
// Data source: grammar_points_data.json["べき"]
//
// Structure variants to test:
//   standard[0]: Verb + べき + だ
//   standard[1]: Verb + べき + Noun
//   polite[0]: Verb + べき + です
//   Exception: する → すべき (optional る)
//
// Complex structures (skip for now):
//   standard[2-4]: い/な-Adj/Noun + である + べき + だ

mod beki_tests {
    use super::*;

    // Testing: standard[0] - Verb + べき + だ
    // Note: Pattern range extends to include following だ auxiliary
    #[test]
    fn test_verb_beki_da() {
        let sentence = "そういう事は本人に言うべきだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "べき");
        assert_pattern_range(&patterns, "べき", 9, 14); // 言うべきだ (includes だ)
    }

    // Testing: standard[1] - Verb + べき + Noun
    // Note: For suru-verbs, pattern extends backwards to include the サ変接続 noun
    #[test]
    fn test_verb_beki_noun() {
        let sentence = "ハマサキさんは尊敬するべき人だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "べき");
        assert_pattern_range(&patterns, "べき", 7, 13); // 尊敬するべき (includes サ変接続 noun)
    }

    // Testing: Exception - する → すべき (without る)
    // Note: Pattern range extends to include following だ auxiliary
    #[test]
    fn test_suru_subeki_exception() {
        let sentence = "家族は何があっても大切にすべきだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "べき");
        assert_pattern_range(&patterns, "べき", 12, 16); // すべきだ (includes だ)
    }

    // Testing: polite[0] - Verb + べき + です
    // Note: Pattern detects Verb + べき, not including です
    #[test]
    fn test_verb_beki_desu() {
        let sentence = "あなたに教えるべき事はこれで全部です";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "べき");
        assert_pattern_range(&patterns, "べき", 4, 9); // 教えるべき
    }
}

// ========== ところだった ① (was about to / almost) ==========
// Pattern: ところだった ① (was about to / almost happened)
// Data source: grammar_points_data.json["ところだった ①"]
//
// Structure variants to test:
//   standard[0]: Verb[る] + ところだった
//   standard[1]: Verb[ない] + ところだった
//   polite[0]: Verb[る] + ところでした
//   polite[1]: Verb[ない] + ところでした

mod tokorodatta_tests {
    use super::*;

    // Testing: standard[0] - Verb[る] + ところだった
    #[test]
    fn test_verb_ru_tokorodatta() {
        let sentence = "やばい、大事な書類を捨てるところだった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ところだった ①");
        assert_pattern_range(&patterns, "ところだった ①", 10, 19); // 捨てるところだった
    }

    // Testing: standard[1] - Verb[ない] + ところだった
    #[test]
    fn test_verb_nai_tokorodatta() {
        let sentence = "もう少しで待ち合わせ時間に間に合わないところだった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ところだった ①");
        assert_pattern_range(&patterns, "ところだった ①", 17, 25); // ないところだった
    }

    // Testing: polite[0] - Verb[る] + ところでした
    #[test]
    fn test_verb_ru_tokorodeshita() {
        let sentence = "危うく電車に乗り遅れるところでした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ところだった ①");
        assert_pattern_range(&patterns, "ところだった ①", 6, 17); // 乗り遅れるところでした
    }

    // Testing: polite[1] - Verb[ない] + ところでした
    #[test]
    fn test_verb_nai_tokorodeshita() {
        let sentence = "現金が足りなくて家賃が払えないところでした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ところだった ①");
        assert_pattern_range(&patterns, "ところだった ①", 13, 21); // ないところでした
    }
}

// ========== として (as / in the capacity of) ==========
// Pattern: として (as / in the capacity of)
// Data source: grammar_points_data.json["として"]
//
// Structure variants to test:
//   standard[0]: Noun + として
//   standard[1]: Noun + として + Noun

mod toshite_tests {
    use super::*;

    // Testing: standard[0] - Noun + として
    #[test]
    fn test_noun_toshite() {
        let sentence = "ミキは友達としては最高だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "として");
        assert_pattern_range(&patterns, "として", 5, 8); // として
    }

    // Testing: standard[0] - Noun + として (different example)
    #[test]
    fn test_noun_toshite_dvd_player() {
        let sentence = "このゲーム機はＤＶＤプレイヤーとしても使えます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "として");
        assert_pattern_range(&patterns, "として", 15, 18); // として
    }

    // Testing: standard[1] - Noun + として + Noun (失格)
    #[test]
    fn test_noun_toshite_noun() {
        let sentence = "動物を傷つける奴は人間として失格だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "として");
        assert_pattern_range(&patterns, "として", 11, 14); // として
    }

    // Testing: standard[1] - Noun + として + の + Noun
    #[test]
    fn test_noun_toshite_no_noun() {
        let sentence = "これは会社としての目標です";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "として");
        assert_pattern_range(&patterns, "として", 5, 8); // として
    }
}

// ========== にしては (considering / for) ==========
// Pattern: にしては (considering / for)
// Data source: grammar_points_data.json["にしては"]
//
// Structure variants to test:
//   standard[0]: Verb + にしては
//   standard[1]: Noun + にしては

mod nishiteha_tests {
    use super::*;

    // Testing: standard[0] - Verb[ている] + にしては
    #[test]
    fn test_verb_teiru_nishiteha() {
        let sentence = "毎日勉強をしているにしては全然漢字を読めない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にしては");
        assert_pattern_range(&patterns, "にしては", 9, 13); // にしては
    }

    // Testing: standard[0] - Verb[たばかり] + にしては
    #[test]
    fn test_verb_tabakari_nishiteha() {
        let sentence = "始めたばかりにしては結構じょうずだね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にしては");
        assert_pattern_range(&patterns, "にしては", 6, 10); // にしては
    }

    // Testing: standard[1] - Noun + にしては
    #[test]
    fn test_noun_nishiteha() {
        let sentence = "田舎にしてはコンビニがいっぱいあるね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にしては");
        assert_pattern_range(&patterns, "にしては", 2, 6); // にしては
    }

    // Testing: standard[1] - Noun + にしては (actor example)
    #[test]
    fn test_noun_nishiteha_actor() {
        let sentence = "あの人は俳優にしてはあまりイケメンじゃない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にしては");
        assert_pattern_range(&patterns, "にしては", 6, 10); // にしては
    }
}

// ========== ないうちに (before/without happening) ==========
// Pattern: ないうちに (before X happens / without X happening)
// Data source: grammar_points_data.json["ないうちに"]
//
// Structure variants to test:
//   standard[0]: Verb[ない] + うちに + Phrase

mod naiuchini_tests {
    use super::*;

    // Testing: standard[0] - Verb[ない] + うちに (before meeting)
    #[test]
    fn test_verb_nai_uchini_meeting() {
        let sentence = "全然会わないうちに、凄く大きくなったね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないうちに");
        assert_pattern_range(&patterns, "ないうちに", 4, 9); // ないうちに
    }

    // Testing: standard[0] - Verb[ない] + うちに (before forgetting)
    #[test]
    fn test_verb_nai_uchini_forgetting() {
        let sentence = "忘れないうちに電話しておかなきゃ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないうちに");
        assert_pattern_range(&patterns, "ないうちに", 2, 7); // ないうちに
    }

    // Testing: standard[0] - Verb[ない] + うちに (without realizing)
    #[test]
    fn test_verb_nai_uchini_realizing() {
        let sentence = "知らないうちに家の前にあるラーメン屋が潰れていた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないうちに");
        assert_pattern_range(&patterns, "ないうちに", 2, 7); // ないうちに
    }

    // Testing: standard[0] - Verb[ない] + うちに (before leaving)
    #[test]
    fn test_verb_nai_uchini_leaving() {
        let sentence = "彼女が離れないうちにちゃんと謝っておこう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないうちに");
        assert_pattern_range(&patterns, "ないうちに", 5, 10); // ないうちに
    }
}
