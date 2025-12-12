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
