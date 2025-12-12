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
