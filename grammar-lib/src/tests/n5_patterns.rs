use super::*;

// ========== たい (Want to do) ==========
// Pattern: たい (desire/want to do)
// Data source: grammar_points_data.json["たい"]
//
// Structure variants to test:
//   standard[0]: Verb[stem] + たい
//   standard[1]: Verb[stem] + たくない (negative)
//   standard[2]: Verb[stem] + たかった (past)
//   standard[3]: Verb[stem] + たくなかった (past negative)
//   polite[0-5]: Same forms + です variants

mod tai_form_tests {
    use super::*;

    #[test]
    fn test_tai_form_non_past() {
        let sentence = "あの映画、絶対に見たいんだけど";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たい");
        assert_pattern_range(&patterns, "たい", 8, 11); // 見たい
    }

    #[test]
    fn test_tai_form_negative_polite() {
        let sentence = "虫は食べたくないです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たい");
        assert_pattern_range(&patterns, "たい", 2, 10); // 食べたくないです (extended with です)
    }

    #[test]
    fn test_tai_form_past() {
        let sentence = "警察官になりたかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たい");
        assert_pattern_range(&patterns, "たい", 4, 10); // なりたかった
    }

    #[test]
    fn test_tai_form_past_negative() {
        let sentence = "昔は先生になりたくなかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たい");
        assert_pattern_range(&patterns, "たい", 5, 13); // なりたくなかった
    }
}

// ========== ている① (Progressive/Resultative) ==========
// Pattern: ている① (progressive/resultative state)
// Data source: grammar_points_data.json["ている①"]
//
// Structure variants to test:
//   standard[0]: Verb[て] + いる
//   standard[1]: Verb[て] + る (contracted)
//   polite[0]: Verb[て] + います
//   polite[1]: Verb[て] + ます (contracted)

mod te_iru_tests {
    use super::*;

    #[test]
    fn test_te_iru_progressive() {
        let sentence = "最近ずっとあのことを考えている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ている①");
        assert_pattern_range(&patterns, "ている①", 10, 15); // 考えている
    }

    #[test]
    fn test_te_iru_contracted() {
        let sentence = "あいつ、どこ行ってるんだよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ている①");
        assert_pattern_range(&patterns, "ている①", 6, 10); // 行ってる
    }

    #[test]
    fn test_te_imasu_polite() {
        let sentence = "今ラーメンを食べています";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ている①");
        assert_pattern_range(&patterns, "ている①", 6, 12); // 食べています
    }

    #[test]
    fn test_te_masu_contracted_polite() {
        let sentence = "彼は何してます？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ている①");
        assert_pattern_range(&patterns, "ている①", 3, 7); // してます
    }
}

// ========== も (Also/Too) ==========
// Pattern: も (also/too/even)
// Data source: grammar_points_data.json["も"]
//
// Structure variants to test:
//   standard[0]: Noun + も
//
// Note: This pattern has only one structure variant but multiple contexts
// Test different contexts: pronoun + も, noun + particle + も
// NEGATIVE tests: Exclude 誰も/何も (different grammar - "nobody/nothing")

mod mo_also_tests {
    use super::*;

    #[test]
    fn test_pronoun_also() {
        let sentence = "俺もそう思ってたんだよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "も");
        assert_pattern_range(&patterns, "も", 0, 2); // 俺も
    }

    #[test]
    fn test_noun_particle_also() {
        let sentence = "こんな場所にも来たことあるの？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "も");
        assert_pattern_range(&patterns, "も", 3, 7); // 場所にも
    }

    // NEGATIVE TEST - should NOT match (different grammar)
    #[test]
    fn test_question_dare_mo() {
        let sentence = "誰も来ないなんて寂しいな";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert!(!has_pattern(&patterns, "も")); // 誰も is different grammar
    }
}

// ========== たことがある (Experience) ==========
// Pattern: たことがある (have experience of)
// Data source: grammar_points_data.json["たことがある"]
//
// Structure variants to test:
//   standard[0]: Verb[た] + こと + が + ある
//   standard[1]: Verb[た] + こと + が + ない
//   polite[0]: Verb[た] + こと + が + あります
//   polite[1]: Verb[た] + こと + が + ありません

mod ta_koto_ga_aru_tests {
    use super::*;

    #[test]
    fn test_affirmative_standard() {
        let sentence = "あいつとは前に会ったことがあるはずだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たことがある");
        assert_pattern_range(&patterns, "たことがある", 7, 15); // 会ったことがある
    }

    #[test]
    fn test_negative_standard() {
        let sentence = "こんな料理は食べたことがない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たことがある");
        assert_pattern_range(&patterns, "たことがある", 6, 14); // 食べたことがない
    }

    #[test]
    fn test_affirmative_polite() {
        let sentence = "京都に行ったことがあります";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たことがある");
        assert_pattern_range(&patterns, "たことがある", 3, 13); // 行ったことがあります
    }

    #[test]
    fn test_negative_polite() {
        let sentence = "まだ一度も使ったことがありません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たことがある");
        assert_pattern_range(&patterns, "たことがある", 5, 16); // 使ったことがありません
    }
}

// ========== から (From - Starting Point) ==========
// Pattern: から (from a starting point)
// Data source: grammar_points_data.json["から"]
//
// Structure variants to test:
//   standard[0]: Starting Point + から
//
// Note: This is the "from" usage (モロッコから, 空港から)
// Not the "because" usage (which requires だ before から)

mod kara_from_tests {
    use super::*;

    #[test]
    fn test_location_from() {
        let sentence = "彼女はモロッコから来ました";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "から");
        assert_pattern_range(&patterns, "から", 3, 9); // モロッコから
    }

    #[test]
    fn test_place_from() {
        let sentence = "空港から車で来ました";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "から");
        assert_pattern_range(&patterns, "から", 0, 4); // 空港から
    }

    #[test]
    fn test_time_from() {
        let sentence = "今日から新しい仕事を始めます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "から");
        assert_pattern_range(&patterns, "から", 0, 4); // 今日から
    }
}

// ========== ので (Because/Since) ==========
// Pattern: ので (because, since - objective reasoning)
// Data source: grammar_points_data.json["ので"]
//
// Structure variants to test:
//   standard[0]: Verb + ので
//   standard[1]: い-Adjective + ので
//   standard[2]: な-Adjective + な + ので
//   standard[3]: Noun + な + ので
//
// Note: More formal/objective than から
// Can also appear as んで (casual)

mod node_tests {
    use super::*;

    #[test]
    fn test_verb_node() {
        let sentence = "友達が家に来るので、部屋の掃除をする";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ので");
        assert_pattern_range(&patterns, "ので", 5, 9); // 来るので
    }

    #[test]
    fn test_i_adjective_node() {
        let sentence = "今日は寒いので、コートを着ます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ので");
        assert_pattern_range(&patterns, "ので", 3, 7); // 寒いので
    }

    #[test]
    fn test_na_adjective_node() {
        let sentence = "景色が綺麗なので、写真を撮ります";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ので");
        assert_pattern_range(&patterns, "ので", 3, 8); // 綺麗なので
    }

    #[test]
    fn test_noun_node() {
        let sentence = "今日は日曜日なので休みです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ので");
        assert_pattern_range(&patterns, "ので", 3, 9); // 日曜日なので
    }
}

// ========== まで (Until/To) ==========
// Pattern: まで (until/to - ending point)
// Data source: grammar_points_data.json["Noun + まで"]
//
// Structure variants to test:
//   standard[0]: Ending Point + まで
//   standard[1]: Noun + まで
// (These are effectively the same - test different contexts)

mod made_until_tests {
    use super::*;

    #[test]
    fn test_place_made() {
        let sentence = "駅まで送るよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Noun + まで");
        assert_pattern_range(&patterns, "Noun + まで", 0, 3); // 駅まで
    }

    #[test]
    fn test_time_made() {
        let sentence = "９時まで勉強する";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Noun + まで");
        assert_pattern_range(&patterns, "Noun + まで", 1, 4); // 時まで (９ is separate token)
    }

    #[test]
    fn test_location_made() {
        let sentence = "空港まで行く";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Noun + まで");
        assert_pattern_range(&patterns, "Noun + まで", 0, 4); // 空港まで
    }
}

// ========== てもいい (It's okay to) ==========
// Pattern: てもいい (permission/it's okay to do)
// Data source: grammar_points_data.json["Verb + てもいい"]
//
// Structure variants to test:
//   standard[0]: Verb[て] + も + いい
//   polite[0]: Verb[て] + も + いい + です

mod temo_ii_tests {
    use super::*;

    #[test]
    fn test_temo_ii_standard() {
        let sentence = "その靴を買ってもいいよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb + てもいい");
        assert_pattern_range(&patterns, "Verb + てもいい", 4, 10); // 買ってもいい
    }

    #[test]
    fn test_temo_ii_polite() {
        let sentence = "この肉は食べてもいいです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb + てもいい");
        assert_pattern_range(&patterns, "Verb + てもいい", 4, 12); // 食べてもいいです (extended with です)
    }

    #[test]
    fn test_demo_ii_de_particle() {
        let sentence = "ここで遊んでもいいですか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb + てもいい");
        assert_pattern_range(&patterns, "Verb + てもいい", 3, 11); // 遊んでもいいです (で form, extended with です)
    }
}

// ========== にいく (Go to do) ==========
// Pattern: にいく (go to do something)
// Data source: grammar_points_data.json["Verb + にいく"]
//
// Structure variants to test:
//   standard[0]: Verb[stem] + に + 行く
//   polite[0]: Verb[stem] + に + 行きます

mod ni_iku_tests {
    use super::*;

    #[test]
    fn test_ni_iku_standard() {
        let sentence = "釣りにいく";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb + にいく");
        assert_pattern_range(&patterns, "Verb + にいく", 0, 5); // 釣りにいく
    }

    #[test]
    fn test_ni_iku_stem_form() {
        let sentence = "今からトレーニングをしにいきます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb + にいく");
        assert_pattern_range(&patterns, "Verb + にいく", 10, 16); // しにいきます (extended with ます)
    }

    #[test]
    fn test_ni_iku_eating() {
        let sentence = "昼ご飯を食べに行こう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb + にいく");
        assert_pattern_range(&patterns, "Verb + にいく", 4, 10); // 食べに行こう (extended with う)
    }
}

// ========== Verb + まで (Until) ==========
// Pattern: Verb + まで (until [verb] happens)
// Data source: grammar_points_data.json["Verb + まで"]
//
// Structure variants to test:
//   standard[0]: Verb + まで

mod verb_made_tests {
    use super::*;

    #[test]
    fn test_verb_made_until() {
        let sentence = "友達が来るまで、駅をウロウロした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb + まで");
        assert_pattern_range(&patterns, "Verb + まで", 3, 7); // 来るまで
    }

    #[test]
    fn test_verb_made_polite() {
        let sentence = "携帯は壊れるまで、使います";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb + まで");
        assert_pattern_range(&patterns, "Verb + まで", 3, 8); // 壊れるまで
    }

    #[test]
    fn test_verb_made_negative_context() {
        let sentence = "授業が始まるまで、先生が来なかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb + まで");
        assert_pattern_range(&patterns, "Verb + まで", 3, 8); // 始まるまで
    }
}

// ========== あそこ (That place over there) ==========
// Pattern: あそこ (that place over there - distant from both speaker and listener)
// Data source: grammar_points_data.json["あそこ"]
//
// Structure variants to test:
//   standard[0]: あそこ (as demonstrative pronoun)

mod asoko_tests {
    use super::*;

    #[test]
    fn test_asoko_with_particle() {
        let sentence = "あそこの店はいい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あそこ");
        assert_pattern_range(&patterns, "あそこ", 0, 3); // あそこ
    }

    #[test]
    fn test_asoko_as_subject() {
        let sentence = "あそこは嫌だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あそこ");
        assert_pattern_range(&patterns, "あそこ", 0, 3); // あそこ
    }

    #[test]
    fn test_asoko_with_mo() {
        let sentence = "あそこも暑い";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あそこ");
        assert_pattern_range(&patterns, "あそこ", 0, 3); // あそこ
    }
}

// ========== ここ (Here / This place) ==========
// Pattern: ここ (this place - near the speaker)
// Data source: grammar_points_data.json["ここ"]
//
// Structure variants to test:
//   standard[0]: ここ (as demonstrative pronoun)

mod koko_tests {
    use super::*;

    #[test]
    fn test_koko_with_particle() {
        let sentence = "ここのパンは美味しい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ここ");
        assert_pattern_range(&patterns, "ここ", 0, 2); // ここ
    }

    #[test]
    fn test_koko_as_subject() {
        let sentence = "先生、ここが痛いです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ここ");
        assert_pattern_range(&patterns, "ここ", 3, 5); // ここ
    }
}

// ========== そこ (There / That place) ==========
// Pattern: そこ (that place - near the listener or previously mentioned)
// Data source: grammar_points_data.json["そこ"]
//
// Structure variants to test:
//   standard[0]: そこ (as demonstrative pronoun)

mod soko_tests {
    use super::*;

    #[test]
    fn test_soko_with_particle() {
        let sentence = "そこの犬は危ないです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そこ");
        assert_pattern_range(&patterns, "そこ", 0, 2); // そこ
    }

    #[test]
    fn test_soko_as_subject() {
        let sentence = "そこは綺麗？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そこ");
        assert_pattern_range(&patterns, "そこ", 0, 2); // そこ
    }
}

// ========== これ (This) ==========
// Pattern: これ (this - demonstrative pronoun for things near speaker)
// Data source: grammar_points_data.json["これ"]
//
// Structure variants to test:
//   standard[0]: これ (as demonstrative pronoun)

mod kore_tests {
    use super::*;

    #[test]
    fn test_kore_with_wa() {
        let sentence = "これは美味しいです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "これ");
        assert_pattern_range(&patterns, "これ", 0, 2); // これ
    }

    #[test]
    fn test_kore_with_mo() {
        let sentence = "これも欲しい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "これ");
        assert_pattern_range(&patterns, "これ", 0, 2); // これ
    }
}

// ========== それ (That) ==========
// Pattern: それ (that - demonstrative pronoun for things near listener)
// Data source: grammar_points_data.json["それ"]
//
// Structure variants to test:
//   standard[0]: それ (as demonstrative pronoun)

mod sore_tests {
    use super::*;

    #[test]
    fn test_sore_with_wa() {
        let sentence = "それは大変だね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "それ");
        assert_pattern_range(&patterns, "それ", 0, 2); // それ
    }

    #[test]
    fn test_sore_with_da() {
        let sentence = "それだ！絶対それだよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "それ");
        assert_pattern_range(&patterns, "それ", 0, 2); // First それ
    }
}

// ========== あれ (That over there) ==========
// Pattern: あれ (that - demonstrative pronoun for things away from both speaker and listener)
// Data source: grammar_points_data.json["あれ"]
//
// Structure variants to test:
//   standard[0]: あれ (as demonstrative pronoun)

mod are_tests {
    use super::*;

    #[test]
    fn test_are_with_wa() {
        let sentence = "あれは病院です";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あれ");
        assert_pattern_range(&patterns, "あれ", 0, 2); // あれ
    }

    #[test]
    fn test_are_with_ga() {
        let sentence = "あれがバス停です";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あれ");
        assert_pattern_range(&patterns, "あれ", 0, 2); // あれ
    }
}

// ========== この (This ~) ==========
// Pattern: この (this ~ - demonstrative determiner for things near speaker)
// Data source: grammar_points_data.json["この"]
//
// Structure variants to test:
//   standard[0]: この + Noun

mod kono_tests {
    use super::*;

    #[test]
    fn test_kono_with_noun() {
        let sentence = "このスーツは高い";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "この");
        assert_pattern_range(&patterns, "この", 0, 2); // この
    }

    #[test]
    fn test_kono_with_work() {
        let sentence = "この仕事は大変だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "この");
        assert_pattern_range(&patterns, "この", 0, 2); // この
    }
}

// ========== その (That ~) ==========
// Pattern: その (that ~ - demonstrative determiner for things near listener)
// Data source: grammar_points_data.json["その"]
//
// Structure variants to test:
//   standard[0]: その + Noun

mod sono_tests {
    use super::*;

    #[test]
    fn test_sono_with_noun() {
        let sentence = "その店は美味しいよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "その");
        assert_pattern_range(&patterns, "その", 0, 2); // その
    }

    #[test]
    fn test_sono_with_person() {
        let sentence = "その人は誰ですか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "その");
        assert_pattern_range(&patterns, "その", 0, 2); // その
    }
}

// ========== あの (That ~ over there) ==========
// Pattern: あの (that ~ over there - demonstrative determiner for things away from both)
// Data source: grammar_points_data.json["あの"]
//
// Structure variants to test:
//   standard[0]: あの + Noun

mod ano_tests {
    use super::*;

    #[test]
    fn test_ano_with_noun() {
        let sentence = "あの映画は面白かった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あの");
        assert_pattern_range(&patterns, "あの", 0, 2); // あの
    }

    #[test]
    fn test_ano_with_place() {
        let sentence = "あの建物は何ですか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あの");
        assert_pattern_range(&patterns, "あの", 0, 2); // あの
    }
}

// ========== Adjective + の(は) ==========
// Pattern: Adjective + の(は) (nominalizer - "the one that")
// Data source: grammar_points_data.json["Adjective + の(は)"]
//
// Structure variants to test:
//   standard[0]: な-Adjective + な + の + は/が/も
//   standard[1]: い-Adjective + の + は/が/も

mod adjective_no_wa_tests {
    use super::*;

    #[test]
    fn test_i_adjective_no_wa() {
        let sentence = "可愛いのは私の犬です";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Adjective + の(は)");
        assert_pattern_range(&patterns, "Adjective + の(は)", 0, 5); // 可愛いのは
    }

    #[test]
    fn test_na_adjective_no_wa() {
        let sentence = "心配なのはあなたです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Adjective + の(は)");
        assert_pattern_range(&patterns, "Adjective + の(は)", 0, 5); // 心配なのは
    }

    #[test]
    fn test_i_adjective_no_ga() {
        let sentence = "熱いのがこれです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Adjective + の(は)");
        assert_pattern_range(&patterns, "Adjective + の(は)", 0, 4); // 熱いのが
    }

    #[test]
    fn test_i_adjective_no_mo() {
        let sentence = "冷たいのも好きです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Adjective + の(は)");
        assert_pattern_range(&patterns, "Adjective + の(は)", 0, 5); // 冷たいのも
    }
}

// ========== けど・だけど (But) ==========
// Pattern: けど・だけど (but/however - casual)
// Data source: grammar_points_data.json["けど・だけど"]
//
// Structure variants to test:
//   standard[0]: Verb + けど
//   standard[1]: い-Adjective + けど
//   standard[2]: な-Adjective + だ + けど
//   standard[3]: Noun + だ + けど

mod kedo_dakedo_tests {
    use super::*;

    #[test]
    fn test_verb_kedo() {
        let sentence = "毎日泳ぐけど、今日は泳がない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "けど・だけど");
        assert_pattern_range(&patterns, "けど・だけど", 2, 6); // 泳ぐけど
    }

    #[test]
    fn test_i_adjective_kedo() {
        let sentence = "北海道は寒いけど、綺麗だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "けど・だけど");
        assert_pattern_range(&patterns, "けど・だけど", 4, 8); // 寒いけど
    }

    #[test]
    fn test_na_adjective_dakedo() {
        let sentence = "田舎は静かだけど、不便だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "けど・だけど");
        assert_pattern_range(&patterns, "けど・だけど", 3, 8); // 静かだけど
    }

    #[test]
    fn test_noun_dakedo() {
        let sentence = "これはステーキだけど、冷たい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "けど・だけど");
        assert_pattern_range(&patterns, "けど・だけど", 3, 10); // ステーキだけど
    }
}

// ========== どこ (Where) ==========
// Pattern: どこ (where - question word for place)
// Data source: grammar_points_data.json["どこ"]
//
// Structure: Demonstrative (single token)

mod doko_tests {
    use super::*;

    #[test]
    fn test_doko_with_particle() {
        let sentence = "明日はどこに行く？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "どこ");
        assert_pattern_range(&patterns, "どこ", 3, 5); // どこ
    }

    #[test]
    fn test_doko_as_subject() {
        let sentence = "ここはどこですか？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "どこ");
        assert_pattern_range(&patterns, "どこ", 3, 5); // どこ
    }
}

// ========== どれ (Which) ==========
// Pattern: どれ (which - question word for things)
// Data source: grammar_points_data.json["どれ"]
//
// Structure: Demonstrative (single token)

mod dore_tests {
    use super::*;

    #[test]
    fn test_dore_with_ga() {
        let sentence = "どれが飛びますか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "どれ");
        assert_pattern_range(&patterns, "どれ", 0, 2); // どれ
    }

    #[test]
    fn test_dore_after_topic() {
        let sentence = "あなたの車はどれ？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "どれ");
        assert_pattern_range(&patterns, "どれ", 6, 8); // どれ
    }
}

// ========== どの (Which ~) ==========
// Pattern: どの (which ~ - pre-noun adjectival)
// Data source: grammar_points_data.json["どの"]
//
// Structure: Demonstrative + Noun (pre-noun adjectival)

mod dono_tests {
    use super::*;

    #[test]
    fn test_dono_with_noun() {
        let sentence = "どの飲み物がいい？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "どの");
        assert_pattern_range(&patterns, "どの", 0, 2); // どの
    }

    #[test]
    fn test_dono_whatever() {
        let sentence = "どの色でもいいよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "どの");
        assert_pattern_range(&patterns, "どの", 0, 2); // どの
    }
}

// ========== 誰 (Who) ==========
// Pattern: 誰 (who - question word for person)
// Data source: grammar_points_data.json["誰"]
//
// Structure: Demonstrative (single token)

mod dare_tests {
    use super::*;

    #[test]
    fn test_dare_subject() {
        let sentence = "誰が来る？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "誰");
        assert_pattern_range(&patterns, "誰", 0, 1); // 誰
    }

    #[test]
    fn test_dare_with_particle() {
        let sentence = "お前は誰と行く？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "誰");
        assert_pattern_range(&patterns, "誰", 3, 4); // 誰
    }
}

// ========== Adjective + て + B ==========
// Pattern: Adjective + て + B (linking adjectives to phrases)
// Data source: grammar_points_data.json["Adjective + て + B"]
//
// Structure variants to test:
//   standard[0]: い-Adjective[い] + く + て + Phrase
//   standard[1]: な-Adjective + で + Phrase
//   standard[2]: Noun + で + Phrase
//   polite: None listed

mod adjective_te_b_tests {
    use super::*;

    #[test]
    fn test_i_adjective_te() {
        let sentence = "田中さんの犬は大きくて遊ぶのが好きです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Adjective + て + B");
        assert_pattern_range(&patterns, "Adjective + て + B", 7, 11); // 大きくて
    }

    #[test]
    fn test_na_adjective_de() {
        let sentence = "マサミは綺麗で水泳が趣味です";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Adjective + て + B");
        assert_pattern_range(&patterns, "Adjective + て + B", 4, 7); // 綺麗で
    }

    #[test]
    fn test_noun_de_copula() {
        let sentence = "彼女は医者で優しい人です";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Adjective + て + B");
        assert_pattern_range(&patterns, "Adjective + て + B", 3, 6); // 医者で
    }
}

// ========== Adjective + て・Noun + で ==========
// Pattern: Adjective + て・Noun + で (linking adjectives - listing qualities)
// Data source: grammar_points_data.json["Adjective + て・Noun + で"]
//
// Structure variants to test:
//   standard[0]: い-Adjective[い] + く + て
//   standard[3]: な-Adjective + で
//   standard[6]: Noun + で
//   special: いい→よくて (exception)
//   polite: None listed

mod adjective_te_noun_de_tests {
    use super::*;

    #[test]
    fn test_i_adj_linking() {
        let sentence = "このパソコンは新しくて早い";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Adjective + て・Noun + で");
        assert_pattern_range(&patterns, "Adjective + て・Noun + で", 7, 11); // 新しくて
    }

    #[test]
    fn test_na_adj_linking() {
        let sentence = "あの自転車は便利で軽い";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Adjective + て・Noun + で");
        assert_pattern_range(&patterns, "Adjective + て・Noun + で", 6, 9); // 便利で
    }

    #[test]
    fn test_noun_linking() {
        let sentence = "音楽を作るのは仕事で趣味だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Adjective + て・Noun + で");
        assert_pattern_range(&patterns, "Adjective + て・Noun + で", 7, 10); // 仕事で
    }

    #[test]
    fn test_ii_exception() {
        let sentence = "彼はいい人でよくて優しい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Adjective + て・Noun + で");
        // Should match both 人で and よくて
        // Testing よくて (the いい→よく exception case)
        assert_pattern_range(&patterns, "Adjective + て・Noun + で", 6, 9); // よくて
    }
}

// ========== い-Adjective (Past) ==========
// Pattern: い-Adjective past tense
// Data source: grammar_points_data.json["い-Adjective (Past)"]
//
// Structure variants to test:
//   standard[0]: い-Adjective[い] + かった
//   polite[0]: い-Adjective[い] + かった + です

mod i_adjective_past_tests {
    use super::*;

    #[test]
    fn test_i_adjective_past_standard() {
        let sentence = "昨日の映画は面白かったね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "い-Adjective (Past)");
        assert_pattern_range(&patterns, "い-Adjective (Past)", 6, 11); // 面白かった
    }

    #[test]
    fn test_i_adjective_past_polite() {
        let sentence = "お風呂が温かかったです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "い-Adjective (Past)");
        assert_pattern_range(&patterns, "い-Adjective (Past)", 4, 11); // 温かかったです
    }

    #[test]
    fn test_i_adjective_past_cold() {
        let sentence = "昨日の夜は寒かった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "い-Adjective (Past)");
        assert_pattern_range(&patterns, "い-Adjective (Past)", 5, 9); // 寒かった
    }

    #[test]
    fn test_i_adjective_past_sweet() {
        let sentence = "このケーキは甘かったです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "い-Adjective (Past)");
        assert_pattern_range(&patterns, "い-Adjective (Past)", 6, 12); // 甘かったです
    }
}

// ========== い-Adjective くなかった (Negative Past) ==========
// Pattern: い-Adjective negative past tense
// Data source: grammar_points_data.json["い-Adjective くなかった"]
//
// Structure variants to test:
//   standard[0]: い-Adjective[く] + なかった
//   polite[0]: い-Adjective[く] + なかった + です
//   polite[1]: い-Adjective[く] + ありませんでした

mod i_adjective_kunakatta_tests {
    use super::*;

    #[test]
    fn test_kunakatta_standard() {
        let sentence = "私は太くなかったよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "い-Adjective くなかった");
        assert_pattern_range(&patterns, "い-Adjective くなかった", 2, 8); // 太くなかった
    }

    #[test]
    fn test_kunakatta_semi_polite() {
        let sentence = "この車は高くなかったです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "い-Adjective くなかった");
        assert_pattern_range(&patterns, "い-Adjective くなかった", 4, 12); // 高くなかったです
    }

    #[test]
    fn test_kunakatta_polite() {
        let sentence = "北海道は暑くありませんでした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "い-Adjective くなかった");
        assert_pattern_range(&patterns, "い-Adjective くなかった", 4, 14); // 暑くありませんでした
    }

    #[test]
    fn test_kunakatta_difficult() {
        let sentence = "テストは難しくなかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "い-Adjective くなかった");
        assert_pattern_range(&patterns, "い-Adjective くなかった", 4, 11); // 難しくなかった
    }
}

// ========== いい (Good - Irregular) ==========
// Pattern: いい (good - irregular i-adjective, affirmative form only)
// Data source: grammar_points_data.json["いい"]
//
// Structure variants to test:
//   standard[0]: いい (non-past affirmative)
//   polite[0]: いい + です (polite affirmative)
//
// Other forms handled by different patterns:
//   よくない → い-Adjectives くない (not yet implemented)
//   よかった → い-Adjective (Past)
//   よくなかった → い-Adjective くなかった

mod ii_tests {
    use super::*;

    #[test]
    fn test_ii_standard() {
        let sentence = "これはいい映画だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いい");
        assert_pattern_range(&patterns, "いい", 3, 5); // いい
    }

    #[test]
    fn test_ii_polite() {
        let sentence = "このレストランはいいです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いい");
        assert_pattern_range(&patterns, "いい", 8, 12); // いいです (extends with です)
    }

    #[test]
    fn test_ii_predicate() {
        let sentence = "いい天気ですね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いい");
        assert_pattern_range(&patterns, "いい", 0, 2); // いい
    }

    // Verify that past forms are handled by other patterns
    #[test]
    fn test_yoi_past_handled_by_other_pattern() {
        let sentence = "昨日の天気はよかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // Should be detected by い-Adjective (Past), not いい
        assert_has_pattern(&patterns, "い-Adjective (Past)");
        assert_pattern_range(&patterns, "い-Adjective (Past)", 6, 10); // よかった
    }

    #[test]
    fn test_yoi_past_negative_handled_by_other_pattern() {
        let sentence = "昨日の天気はよくなかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // Should be detected by い-Adjective くなかった, not いい
        assert_has_pattern(&patterns, "い-Adjective くなかった");
        assert_pattern_range(&patterns, "い-Adjective くなかった", 6, 12); // よくなかった
    }
}

// ========== が (But/However) ==========
// Pattern: が (but/however - sentence connector showing contrast)
// Data source: grammar_points_data.json["が"]
//
// Structure variants to test:
//   standard[0]: Verb + が
//   standard[1]: い-Adjective + が
//   standard[2]: な-Adjective + だ + が
//   standard[3]: Noun + だ + が
//   polite[0]: Verb (Polite) + が
//   polite[1]: い-Adjective + です + が
//   polite[2]: な-Adjective + です + が
//   polite[3]: Noun + です + が

mod ga_but_tests {
    use super::*;

    #[test]
    fn test_verb_ga() {
        let sentence = "毎日走るが、運動は嫌いです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "が");
        assert_pattern_range(&patterns, "が", 2, 5); // 走るが
    }

    #[test]
    fn test_i_adjective_ga() {
        let sentence = "このカレーは辛いが、美味しい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "が");
        assert_pattern_range(&patterns, "が", 6, 9); // 辛いが
    }

    #[test]
    fn test_na_adjective_da_ga() {
        let sentence = "車は便利だが、危ない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "が");
        assert_pattern_range(&patterns, "が", 4, 6); // だが
    }

    #[test]
    fn test_noun_da_ga() {
        let sentence = "お金は大切だが、時間も大切だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "が");
        assert_pattern_range(&patterns, "が", 5, 7); // だが
    }

    #[test]
    fn test_verb_polite_ga() {
        let sentence = "行きますが、遅れるかもしれません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "が");
        assert_pattern_range(&patterns, "が", 2, 5); // ますが
    }

    #[test]
    fn test_i_adjective_desu_ga() {
        let sentence = "高いですが、品質がいいです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "が");
        assert_pattern_range(&patterns, "が", 2, 5); // ですが
    }

    #[test]
    fn test_na_adjective_desu_ga() {
        let sentence = "綺麗ですが、高すぎます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "が");
        assert_pattern_range(&patterns, "が", 2, 5); // ですが
    }

    #[test]
    fn test_noun_desu_ga() {
        let sentence = "学生ですが、働いています";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "が");
        assert_pattern_range(&patterns, "が", 2, 5); // ですが
    }
}

// ========== い-Adjectives くない (Negative) ==========
// Pattern: い-Adjectives くない (negative present)
// Data source: grammar_points_data.json["い-Adjectives くない"]
//
// Structure variants to test:
//   standard[0]: い-Adjective[く] + ない
//   polite[0]: い-Adjective[く] + ない + です (semi-polite)
//   polite[1]: い-Adjective[く] + ありません (polite)

mod i_adjective_kunai_tests {
    use super::*;

    #[test]
    fn test_kunai_standard() {
        let sentence = "この料理は美味しくないよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "い-Adjectives くない");
        assert_pattern_range(&patterns, "い-Adjectives くない", 5, 11); // 美味しくない
    }

    #[test]
    fn test_kunai_semi_polite() {
        let sentence = "今日はあまり寒くないです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "い-Adjectives くない");
        assert_pattern_range(&patterns, "い-Adjectives くない", 6, 12); // 寒くないです
    }

    #[test]
    fn test_kunai_polite() {
        // Polite form (くありません) is handled by い-Adjective くなかった pattern
        let sentence = "このテストは難しくありません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // Verify that くなかった pattern handles this
        assert_has_pattern(&patterns, "い-Adjective くなかった");
        assert_pattern_range(&patterns, "い-Adjective くなかった", 6, 14); // 難しくありません
    }

    #[test]
    fn test_yokunai_exception() {
        let sentence = "天気はよくないね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "い-Adjectives くない");
        assert_pattern_range(&patterns, "い-Adjectives くない", 3, 7); // よくない
    }
}

// ========== よ (Sentence-ending particle) ==========
// Pattern: よ (emphasis/new information)
// Data source: grammar_points_data.json["よ"]
//
// Structure variants to test:
//   standard[0]: Sentence + よ

mod yo_tests {
    use super::*;

    #[test]
    fn test_yo_with_verb() {
        let sentence = "これは美味しいよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "よ");
        assert_pattern_range(&patterns, "よ", 7, 8); // よ
    }

    #[test]
    fn test_yo_with_noun() {
        let sentence = "明日は休みだよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "よ");
        assert_pattern_range(&patterns, "よ", 6, 7); // よ
    }

    #[test]
    fn test_yo_polite() {
        let sentence = "気をつけてくださいよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "よ");
        assert_pattern_range(&patterns, "よ", 9, 10); // よ
    }
}

// ========== きらい (Dislike) ==========
// Pattern: きらい (dislike/hate - na-adjective)
// Data source: grammar_points_data.json["きらい"]
//
// Structure variants to test:
//   standard[0]: Noun + が + 嫌い (predicate usage)
//   standard[1]: 嫌い + な + Noun (adjectival usage)

mod kirai_tests {
    use super::*;

    #[test]
    fn test_kirai_predicate() {
        let sentence = "私はスポーツが嫌いです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "きらい");
        assert_pattern_range(&patterns, "きらい", 6, 11); // が嫌いです
    }

    #[test]
    fn test_kirai_adjectival() {
        let sentence = "彼の嫌いな食べ物はピザです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "きらい");
        assert_pattern_range(&patterns, "きらい", 2, 5); // 嫌いな
    }

    #[test]
    fn test_kirai_daisuki() {
        let sentence = "大嫌いな先輩が来る";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "きらい");
        assert_pattern_range(&patterns, "きらい", 0, 4); // 大嫌いな
    }
}

// ========== くらい ① (About/Approximately) ==========
// Pattern: くらい ① (about/approximately with numbers)
// Data source: grammar_points_data.json["くらい ①"]
//
// Structure: Number/counter + くらい or ぐらい

mod kurai_tests {
    use super::*;

    #[test]
    fn test_kurai_with_time() {
        let sentence = "後３分くらいで着く";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "くらい ①");
        assert_pattern_range(&patterns, "くらい ①", 1, 6); // ３分くらい
    }

    #[test]
    fn test_gurai_with_counter() {
        let sentence = "４個ぐらい欲しいな";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "くらい ①");
        assert_pattern_range(&patterns, "くらい ①", 0, 5); // ４個ぐらい
    }

    #[test]
    fn test_kurai_with_question() {
        let sentence = "どのくらいで着きますか？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "くらい ①");
        assert_pattern_range(&patterns, "くらい ①", 0, 5); // どのくらい
    }
}
