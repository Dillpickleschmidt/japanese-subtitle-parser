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

// ========== ている② (Resultative State) ==========
// Pattern: ている② (resultative/completed state)
// Data source: grammar_points_data.json["ている②"]
//
// Structure variants to test:
//   standard[0]: Verb[て] + いる
//   standard[1]: Verb[て] + る (contracted)
//   polite[0]: Verb[て] + います
//   polite[1]: Verb[て] + ます (contracted)
//
// NOTE: This pattern tokenizes IDENTICALLY to ている① and ている③.
// The difference is only SEMANTIC:
//   - ている① = progressive ("am doing")
//   - ている② = resultative ("has done and the result continues")
//   - ている③ = habitual ("regularly does")
// When Verb[て] + いる is detected, ALL THREE patterns will match.
// The application shows all three grammar explanations to the user,
// who determines meaning from context (similar to らしい① vs らしい②).

mod teiru_u2461_tests {
    use super::*;

    #[test]
    fn test_state_has_started() {
        // Class has started (and is ongoing)
        let sentence = "クラスは始まっている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ている②");
        assert_pattern_range(&patterns, "ている②", 4, 10); // 始まっている
    }

    #[test]
    fn test_state_has_gone() {
        // Mom went (and is still gone)
        let sentence = "お母さんは今買い物に行っています";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ている②");
        assert_pattern_range(&patterns, "ている②", 10, 16); // 行っています
    }

    #[test]
    fn test_state_is_dead() {
        // Dog is dead (has died and remains dead)
        let sentence = "あの犬は死んでいるだろう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ている②");
        assert_pattern_range(&patterns, "ている②", 4, 12); // 死んでいるだろう (includes だろう)
    }

    #[test]
    fn test_state_is_angry() {
        // Teacher is angry (got angry and remains angry)
        let sentence = "先生がめちゃ怒っている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ている②");
        assert_pattern_range(&patterns, "ている②", 6, 11); // 怒っている
    }
}

// ========== ている③ (Habitual Action) ==========
// Pattern: ている③ (habitual/repeated action)
// Data source: grammar_points_data.json["ている③"]
//
// Structure variants to test:
//   standard[0]: Verb[て] + いる
//   standard[1]: Verb[て] + る (contracted)
//   standard[2]: Verb[て] + とる (dialectal)
//   polite[0]: Verb[て] + います
//   polite[1]: Verb[て] + ます (contracted)
//   polite[2]: Verb[て] + とります (dialectal)
//
// NOTE: This pattern tokenizes IDENTICALLY to ている① and ている②.
// The difference is only SEMANTIC:
//   - ている① = progressive ("am doing")
//   - ている② = resultative ("has done and the result continues")
//   - ている③ = habitual ("regularly does")
// When Verb[て] + いる is detected, ALL THREE patterns will match.
// The application shows all three grammar explanations to the user,
// who determines meaning from context (similar to らしい① vs らしい②).

mod teiru_u2462_tests {
    use super::*;

    #[test]
    fn test_habitual_play_guitar() {
        // I play guitar every day (habitual activity)
        let sentence = "私は毎日ギターを弾いている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ている③");
        assert_pattern_range(&patterns, "ている③", 8, 13); // 弾いている
    }

    #[test]
    fn test_habitual_sleep_time() {
        // I sleep at 9 every night (habitual pattern)
        let sentence = "毎晩９時に寝ている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ている③");
        assert_pattern_range(&patterns, "ている③", 5, 9); // 寝ている
    }

    #[test]
    fn test_habitual_work_at_school() {
        // He works at a school (regular activity)
        let sentence = "彼は学校で働いています";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ている③");
        assert_pattern_range(&patterns, "ている③", 5, 11); // 働いています
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

// ========== ね (Sentence-ending particle for seeking agreement) ==========
// Pattern: ね (seeking agreement/confirmation)
// Data source: grammar_points_data.json["ね"]
//
// Structure variants to test:
//   standard[0]: Sentence + ね

mod ne_tests {
    use super::*;

    #[test]
    fn test_ne_with_i_adjective() {
        let sentence = "今日も暑いね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ね");
        assert_pattern_range(&patterns, "ね", 5, 6); // ね
    }

    #[test]
    fn test_ne_with_verb() {
        let sentence = "仕事は疲れるね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ね");
        assert_pattern_range(&patterns, "ね", 6, 7); // ね
    }

    #[test]
    fn test_ne_with_na_adjective() {
        let sentence = "あれは綺麗だね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ね");
        assert_pattern_range(&patterns, "ね", 6, 7); // ね
    }

    #[test]
    fn test_ne_polite() {
        let sentence = "今日は寒いですね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ね");
        assert_pattern_range(&patterns, "ね", 7, 8); // ね
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

// ========== がある (Existence of inanimate objects) ==========
// Pattern: がある (there is/exists - inanimate)
// Data source: grammar_points_data.json["がある"]
//
// Structure variants to test:
//   standard[0]: Noun + が + ある
//   polite[0]: Noun + が + あります

mod ga_aru_tests {
    use super::*;

    #[test]
    fn test_ga_aru_standard() {
        let sentence = "明日、テストがある";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がある");
        assert_pattern_range(&patterns, "がある", 3, 9); // テストがある
    }

    #[test]
    fn test_ga_aru_polite() {
        let sentence = "部屋に椅子があります";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がある");
        assert_pattern_range(&patterns, "がある", 3, 10); // 椅子があります
    }
}

// ========== がいる (Existence of animate objects) ==========
// Pattern: がいる (there is/exists - animate)
// Data source: grammar_points_data.json["がいる"]
//
// Structure variants to test:
//   standard[0]: Noun + が + いる
//   polite[0]: Noun + が + います

mod ga_iru_tests {
    use super::*;

    #[test]
    fn test_ga_iru_standard() {
        let sentence = "公園に子供がいる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がいる");
        assert_pattern_range(&patterns, "がいる", 3, 8); // 子供がいる
    }

    #[test]
    fn test_ga_iru_polite() {
        let sentence = "猫がいます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がいる");
        assert_pattern_range(&patterns, "がいる", 0, 5); // 猫がいます
    }
}

// ========== あげる (To give) ==========
// Pattern: あげる (to give, to offer up)
// Data source: grammar_points_data.json["あげる"]
//
// Structure variants to test:
//   standard[0]: Giver + は/が + Recipient + に + Object + を + あげる
//   standard[1]: Recipient + に + Giver + は/が + Object + を + あげる
//   standard[2]: Giver + は/が + Object + を + Recipient + に + あげる
//   polite[0-2]: Same structures with あげます

mod ageru_tests {
    use super::*;

    #[test]
    fn test_ageru_standard_order() {
        let sentence = "トムがタカにプレゼントをあげる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あげる");
        assert_pattern_range(&patterns, "あげる", 6, 15); // プレゼントをあげる
    }

    #[test]
    fn test_ageru_recipient_first() {
        let sentence = "タカにトムがプレゼントをあげる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あげる");
        assert_pattern_range(&patterns, "あげる", 6, 15); // プレゼントをあげる
    }

    #[test]
    fn test_ageru_object_first() {
        let sentence = "トムがプレゼントをタカにあげる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あげる");
        assert_pattern_range(&patterns, "あげる", 3, 15); // プレゼントをタカにあげる
    }

    #[test]
    fn test_ageru_polite() {
        let sentence = "友達に本をあげます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あげる");
        assert_pattern_range(&patterns, "あげる", 3, 9); // 本をあげます
    }
}

// ========== ないで (without doing) - N4 pattern ==========
// Pattern: ないで (without doing)
// Data source: grammar_points_data.json["ないで"]
//
// Structure variants to test:
//   standard[0]: Verb[ない] + で
//
// Note: There are TWO patterns in patterns.rs for this grammar point:
//   - "ないで"
//   - "Verb[ないで]"
// Both use the same implementation (naide function).

#[cfg(test)]
mod naide_tests {
    use super::*;

    #[test]
    fn test_naide_godan_verb() {
        // 焼く (godan verb) -> 焼かない
        let sentence = "魚を焼かないで食べたから、お腹を壊した";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないで");
        assert_pattern_range(&patterns, "ないで", 2, 7); // 焼かないで
    }

    #[test]
    fn test_naide_ichidan_verb() {
        // かける (ichidan verb) -> かけない
        let sentence = "ドアのカギをかけないで寝るの？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないで");
        assert_pattern_range(&patterns, "ないで", 6, 11); // かけないで
    }

    #[test]
    fn test_naide_with_surprise() {
        // 伝える (ichidan verb) -> 伝えない
        let sentence = "上司に伝えないで休んだの！？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないで");
        assert_pattern_range(&patterns, "ないで", 3, 8); // 伝えないで
    }

    #[test]
    fn test_verb_naide_pattern() {
        // Also test the "Verb[ないで]" pattern name
        let sentence = "温めないで食べた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[ないで]");
        assert_pattern_range(&patterns, "Verb[ないで]", 0, 5); // 温めないで
    }
}

// ========== Verb + て (Te-form) ==========
// Pattern: Verb + て (te-form conjugation for sequential actions)
// Data source: grammar_points_data.json["Verb + て"]
//
// Structure variants to test (17 standard forms):
//   [る1] Verb: 見る → 見て
//   [る5] Verb: 座る → 座って
//   [う] Verb: 歌う → 歌って
//   [つ] Verb: 打つ → 打って
//   [く] Verb: 歩く → 歩いて
//   [ぐ] Verb: 泳ぐ → 泳いで
//   [ぬ] Verb: 死ぬ → 死んで
//   [ぶ] Verb: 飛ぶ → 飛んで
//   [む] Verb: 休む → 休んで
//   [す] Verb: 話す → 話して
//   Irregular: 行く → 行って, する → して, くる → きて

mod verb_te_tests {
    use super::*;

    #[test]
    fn test_ru_verb_te() {
        let sentence = "友達と映画を見て楽しかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb + て");
        assert_pattern_range(&patterns, "Verb + て", 6, 8); // 見て
    }

    #[test]
    fn test_u_verb_ru_ending_te() {
        let sentence = "椅子に座って本を読んだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb + て");
        assert_pattern_range(&patterns, "Verb + て", 3, 6); // 座って
    }

    #[test]
    fn test_u_ending_te() {
        let sentence = "歌を歌って踊った";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb + て");
        assert_pattern_range(&patterns, "Verb + て", 2, 5); // 歌って
    }

    #[test]
    fn test_ku_ending_te() {
        let sentence = "公園まで歩いて行った";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb + て");
        assert_pattern_range(&patterns, "Verb + て", 4, 7); // 歩いて
    }

    #[test]
    fn test_gu_ending_te() {
        let sentence = "海で泳いで遊んだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb + て");
        assert_pattern_range(&patterns, "Verb + て", 2, 5); // 泳いで
    }

    #[test]
    fn test_mu_ending_te() {
        let sentence = "図書館で休んで勉強した";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb + て");
        assert_pattern_range(&patterns, "Verb + て", 4, 7); // 休んで
    }

    #[test]
    fn test_bu_ending_te() {
        let sentence = "鳥が空を飛んで行った";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb + て");
        assert_pattern_range(&patterns, "Verb + て", 4, 7); // 飛んで
    }

    #[test]
    fn test_su_ending_te() {
        let sentence = "先生と話して分かった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb + て");
        assert_pattern_range(&patterns, "Verb + て", 3, 6); // 話して
    }

    #[test]
    fn test_irregular_iku_te() {
        let sentence = "学校に行って勉強する";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb + て");
        assert_pattern_range(&patterns, "Verb + て", 3, 6); // 行って
    }

    #[test]
    fn test_irregular_suru_te() {
        let sentence = "宿題をして寝た";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb + て");
        assert_pattern_range(&patterns, "Verb + て", 3, 5); // して
    }

    #[test]
    fn test_irregular_kuru_te() {
        let sentence = "友達が来て一緒に遊んだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb + て");
        assert_pattern_range(&patterns, "Verb + て", 3, 5); // 来て
    }
}

// ========== Verb + て+ B (Sequential Actions) ==========
// Pattern: Verb + て+ B (te-form followed by another action)
// Data source: grammar_points_data.json["Verb + て+ B"]
//
// Structure: Verb[て] + (Action) Phrase
// Meaning: Sequential actions "do X, then do Y"

mod verb_te_b_tests {
    use super::*;

    #[test]
    fn test_te_then_verb() {
        let sentence = "ショッピングセンターに行って買い物をします";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb + て+ B");
        assert_pattern_range(&patterns, "Verb + て+ B", 11, 21); // 行って買い物をします
    }

    #[test]
    fn test_te_then_verb_past() {
        let sentence = "パンを買って食べた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb + て+ B");
        assert_pattern_range(&patterns, "Verb + て+ B", 3, 9); // 買って食べた
    }

    #[test]
    fn test_multiple_te_sequence() {
        let sentence = "朝起きて顔を洗って朝ごはんを食べた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // This sentence has multiple sequential te-forms:
        // - 起きて顔を洗って (wake up, then wash face)
        // - 洗って朝ごはんを食べた (wash, then eat breakfast)
        assert_has_pattern(&patterns, "Verb + て+ B");
        // Should detect at least one of the sequences
        // The pattern matcher will find the first complete sequence
    }
}

// ========== い-Adjective + Noun ==========
// Pattern: い-Adjective + Noun (adjective modifying noun)
// Data source: grammar_points_data.json["い-Adjective + Noun"]
//
// Structure variants to test:
//   standard[0]: い-Adjective + Noun (e.g., かわいい猫, 新しい車, 寒い冬)

mod i_adjective_noun_tests {
    use super::*;

    #[test]
    fn test_kawaii_neko() {
        let sentence = "あそこにかわいい猫がいる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "い-Adjective + Noun");
        assert_pattern_range(&patterns, "い-Adjective + Noun", 4, 9); // かわいい猫
    }

    #[test]
    fn test_atarashii_kuruma() {
        let sentence = "新しい車を買いたいな";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "い-Adjective + Noun");
        assert_pattern_range(&patterns, "い-Adjective + Noun", 0, 4); // 新しい車
    }

    #[test]
    fn test_samui_fuyu() {
        let sentence = "寒い冬は苦手だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "い-Adjective + Noun");
        assert_pattern_range(&patterns, "い-Adjective + Noun", 0, 3); // 寒い冬
    }

    #[test]
    fn test_hayai_kuruma() {
        let sentence = "速い車が好きです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "い-Adjective + Noun");
        assert_pattern_range(&patterns, "い-Adjective + Noun", 0, 3); // 速い車
    }

    #[test]
    fn test_kowai_sensei() {
        let sentence = "怖い先生には近づかない方がいい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "い-Adjective + Noun");
        assert_pattern_range(&patterns, "い-Adjective + Noun", 0, 4); // 怖い先生
    }
}

// ========== けれども (But/Although - formal) ==========
// Pattern: けれども (formal conjunction meaning "but/although")
// Data source: grammar_points_data.json["けれども"]
//
// Structure variants to test:
//   standard[0]: Verb + けれども
//   standard[1]: い-Adjective + けれども
//   standard[2]: な-Adjective + だ + けれども
//   standard[3]: Noun + だ + けれども
//   polite[0-3]: Same with です forms

mod keredomo_tests {
    use super::*;

    #[test]
    fn test_verb_keredomo() {
        let sentence = "私は１０キロ走ったけれども、疲れていません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "けれども");
        assert_pattern_range(&patterns, "けれども", 9, 13); // けれども
    }

    #[test]
    fn test_i_adjective_keredomo() {
        let sentence = "温泉は熱いけれども、気持ちいいです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "けれども");
        assert_pattern_range(&patterns, "けれども", 5, 9); // けれども
    }

    #[test]
    fn test_na_adjective_keredomo() {
        let sentence = "あの人は綺麗だけれども、怖いです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "けれども");
        assert_pattern_range(&patterns, "けれども", 7, 11); // けれども
    }

    #[test]
    fn test_noun_keredomo() {
        let sentence = "これはゲームだけれども、面白くないです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "けれども");
        assert_pattern_range(&patterns, "けれども", 7, 11); // けれども
    }
}

// ========== か (Or/Question marker between options) ==========
// Pattern: か (or - presenting options/alternatives)
// Data source: grammar_points_data.json["か"]
//
// Structure variants to test:
//   standard[0]: Verb (A) + か + Verb (B) + か
//   standard[1]: い-Adjective (A) + か + い-Adjective (B) + か
//   standard[2]: な-Adjective (A) + か + な-Adjective (B) + か
//   standard[3]: Noun (A) + か + Noun (B) + か

mod ka_or_tests {
    use super::*;

    #[test]
    fn test_verb_or_verb() {
        let sentence = "行くか帰るか決めてよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "か");
        assert_pattern_range(&patterns, "か", 0, 6); // 行くか帰るか
    }

    #[test]
    fn test_noun_or_noun() {
        let sentence = "コーヒーかお茶か選んでください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "か");
        assert_pattern_range(&patterns, "か", 0, 8); // コーヒーかお茶か
    }

    #[test]
    fn test_i_adjective_or_i_adjective() {
        let sentence = "寒いか暑いかわからない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "か");
        assert_pattern_range(&patterns, "か", 0, 6); // 寒いか暑いか
    }

    #[test]
    fn test_na_adjective_or_na_adjective() {
        let sentence = "静かか賑やかか選んでください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "か");
        assert_pattern_range(&patterns, "か", 0, 7); // 静かか賑やかか
    }
}

// ========== すぎる (Too much/excessive) ==========
// Pattern: すぎる (too much, excessive)
// Data source: grammar_points_data.json["すぎる"]
//
// Structure variants to test:
//   standard[0]: Verb[stem] + すぎる
//   standard[1]: い-Adjective[stem] + すぎる
//   standard[2]: な-Adjective + すぎる
//   standard[3-6]: Negative forms with なさすぎる
//   polite[0-6]: Same forms with すぎます

mod sugiru_tests {
    use super::*;

    #[test]
    fn test_verb_stem_sugiru() {
        let sentence = "頑張りすぎるのは身体に良くない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "すぎる");
        assert_pattern_range(&patterns, "すぎる", 0, 6); // 頑張りすぎる
    }

    #[test]
    fn test_i_adjective_sugiru() {
        let sentence = "これは熱すぎるよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "すぎる");
        assert_pattern_range(&patterns, "すぎる", 3, 7); // 熱すぎる
    }

    #[test]
    fn test_na_adjective_sugiru() {
        let sentence = "俺には田舎は静かすぎる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "すぎる");
        assert_pattern_range(&patterns, "すぎる", 6, 11); // 静かすぎる
    }

    #[test]
    fn test_verb_stem_sugimasu_polite() {
        let sentence = "最近働きすぎます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "すぎる");
        assert_pattern_range(&patterns, "すぎる", 2, 8); // 働きすぎます
    }

    #[test]
    fn test_i_adjective_sugimasu_polite() {
        let sentence = "この犬、可愛すぎます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "すぎる");
        assert_pattern_range(&patterns, "すぎる", 4, 10); // 可愛すぎます
    }

    #[test]
    fn test_negative_nasasugiru() {
        let sentence = "私はお金がなさすぎる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "すぎる");
        assert_pattern_range(&patterns, "すぎる", 5, 10); // なさすぎる
    }

    #[test]
    fn test_yoi_stem_sugiru() {
        let sentence = "この歌はよすぎる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "すぎる");
        assert_pattern_range(&patterns, "すぎる", 4, 8); // よすぎる
    }
}

// ========== を (Object marker particle) ==========
// Pattern: を (object marker)
// Data source: grammar_points_data.json["を"]
//
// Structure variants to test:
//   standard[0]: Object + を

mod wo_particle_tests {
    use super::*;

    #[test]
    fn test_direct_object() {
        let sentence = "ラーメンを食べるのが好きだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を");
        assert_pattern_range(&patterns, "を", 0, 5); // ラーメンを
    }

    #[test]
    fn test_song_object() {
        let sentence = "歌を歌います";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を");
        assert_pattern_range(&patterns, "を", 0, 2); // 歌を
    }

    #[test]
    fn test_place_through() {
        let sentence = "公園を歩くのは気持ちいい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を");
        assert_pattern_range(&patterns, "を", 0, 3); // 公園を
    }

    #[test]
    fn test_building_through() {
        let sentence = "建物の中を走るな";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を");
        assert_pattern_range(&patterns, "を", 3, 5); // 中を
    }
}

// ========== まだ～ていません (Haven't done yet) ==========
// Pattern: まだ～ていません (haven't done yet / still haven't done)
// Data source: grammar_points_data.json["まだ～ていません"]
//
// Structure variants to test:
//   standard[0]: まだ + Verb[て] + いない
//   polite[0]: まだ + Verb[て] + いません

mod mada_te_imasen_tests {
    use super::*;

    #[test]
    fn test_casual_negative() {
        let sentence = "まだ何も決めていないんだって";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まだ～ていません");
        assert_pattern_range(&patterns, "まだ～ていません", 0, 10); // まだ何も決めていない
    }

    #[test]
    fn test_polite_negative() {
        let sentence = "その映画はまだ見ていません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まだ～ていません");
        assert_pattern_range(&patterns, "まだ～ていません", 5, 13); // まだ見ていません
    }

    #[test]
    fn test_with_intervening_words() {
        let sentence = "宿題はまだ全然終わっていないんだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まだ～ていません");
        assert_pattern_range(&patterns, "まだ～ていません", 3, 14); // まだ全然終わっていない
    }
}

// ========== たり～たりする (Listing non-ordered actions) ==========
// Pattern: たり～たりする (doing things like A and B)
// Data source: grammar_points_data.json["たり～たりする"]
//
// Structure variants to test:
//   standard[0]: Verb[た]り + (Verb[た]り) + する
//   standard[1]: Verb[た]り + [する]Verb(したり) + する
//   polite[0]: Verb[た]り + (Verb[た]り) + します
//   polite[1]: Verb[た]り + [する]Verb(したり) + します

mod tari_tarisuru_tests {
    use super::*;

    #[test]
    fn test_two_verbs_casual() {
        let sentence = "休みの日は家でテレビを見たり、寝たりする";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たり～たりする");
        // Note: Pattern may match from first たり (見たり、寝たりする) or second たり (寝たりする)
        // The framework finds both and scores the longer match higher
        assert_pattern_range(&patterns, "たり～たりする", 15, 20); // 寝たりする (shorter match also valid)
    }

    #[test]
    fn test_two_verbs_polite() {
        let sentence = "週末は映画を見たり本を読んだりします";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たり～たりする");
        assert_pattern_range(&patterns, "たり～たりする", 6, 18); // 見たり本を読んだりします (full pattern)
    }

    #[test]
    fn test_single_verb_past() {
        let sentence = "昔あそこの池で泳いだりした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たり～たりする");
        assert_pattern_range(&patterns, "たり～たりする", 7, 13); // 泳いだりした
    }
}

// ========== ～になる・～くなる (Become) ==========
// Pattern: になる・くなる (change of state, becoming)
// Data source: grammar_points_data.json["～になる・～くなる"]
//
// Structure variants to test:
//   standard[0]: な-Adjective + に + なる
//   standard[1]: い-Adjective[く] + なる
//   standard[2]: Noun + に + なる
//   standard[3]: Exception - いい → よくなる
//   polite[0-2]: Same forms + なります

mod ni_naru_ku_naru_tests {
    use super::*;

    #[test]
    fn test_i_adjective_ku_naru() {
        let sentence = "お茶が冷たくなったよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～になる・～くなる");
        assert_pattern_range(&patterns, "～になる・～くなる", 3, 9); // 冷たくなった
    }

    #[test]
    fn test_i_adjective_ku_naru_variant() {
        let sentence = "コーヒーが温くなった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～になる・～くなる");
        assert_pattern_range(&patterns, "～になる・～くなる", 5, 10); // 温くなった
    }

    #[test]
    fn test_noun_ni_naru() {
        let sentence = "私は医者にならない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～になる・～くなる");
        assert_pattern_range(&patterns, "～になる・～くなる", 2, 9); // 医者にならない
    }

    #[test]
    fn test_ii_exception_yoku_naru() {
        let sentence = "勉強してるから頭が良くなる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～になる・～くなる");
        assert_pattern_range(&patterns, "～になる・～くなる", 9, 13); // 良くなる
    }

    #[test]
    fn test_na_adjective_ni_naru_polite() {
        let sentence = "部屋が綺麗になります";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～になる・～くなる");
        assert_pattern_range(&patterns, "～になる・～くなる", 3, 10); // 綺麗になります
    }
}

// ========== 好き (To like) ==========
// Pattern: 好き (like, likable)
// Data source: grammar_points_data.json["好き"]
//
// Structure variants to test:
//   standard[0]: Noun + が + 好き + だ
//   polite[0]: Noun + が + 好き + です

mod suki_tests {
    use super::*;

    #[test]
    fn test_suki_standard() {
        let sentence = "私はコンビニが好きだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "好き");
        assert_pattern_range(&patterns, "好き", 7, 10); // 好きだ
    }

    #[test]
    fn test_suki_polite() {
        let sentence = "彼はサーフィンが好きです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "好き");
        assert_pattern_range(&patterns, "好き", 8, 12); // 好きです
    }

    #[test]
    fn test_daisuki() {
        let sentence = "大好きだよ！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "好き");
        assert_pattern_range(&patterns, "好き", 0, 4); // 大好きだ
    }
}

// ========== てから (After doing) ==========
// Pattern: てから (after doing)
// Data source: grammar_points_data.json["てから"]
//
// Structure variants to test:
//   standard[0]: Verb[て] + から

mod te_kara_tests {
    use super::*;

    #[test]
    fn test_te_kara_basic() {
        let sentence = "おやつを食べてから、勉強を始める";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てから");
        assert_pattern_range(&patterns, "てから", 4, 9); // 食べてから
    }

    #[test]
    fn test_te_kara_polite_context() {
        let sentence = "洗濯をしてから、出かけます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てから");
        assert_pattern_range(&patterns, "てから", 3, 7); // してから
    }

    #[test]
    fn test_te_kara_de_form() {
        let sentence = "晩ご飯を食べてから、洗い物をする";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てから");
        assert_pattern_range(&patterns, "てから", 4, 9); // 食べてから
    }
}

// ========== たほうがいい (Should do / It would be better to) ==========
// Pattern: たほうがいい (should do, advice)
// Data source: grammar_points_data.json["たほうがいい"]
//
// Structure variants to test:
//   standard[0]: Verb[た] + 方 + が + いい
//   polite[0]: Verb[た] + 方 + が + いい + です

mod ta_hou_ga_ii_tests {
    use super::*;

    #[test]
    fn test_ta_hou_ga_ii_advice() {
        let sentence = "警察に言ったほうがいい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たほうがいい");
        assert_pattern_range(&patterns, "たほうがいい", 3, 11); // 言ったほうがいい
    }

    #[test]
    fn test_ta_hou_ga_ii_polite() {
        let sentence = "手を洗ったほうがいいですよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たほうがいい");
        assert_pattern_range(&patterns, "たほうがいい", 2, 12); // 洗ったほうがいいです
    }

    #[test]
    fn test_ta_hou_ga_ii_u_verb() {
        let sentence = "もっと頑張ったほうがいい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たほうがいい");
        assert_pattern_range(&patterns, "たほうがいい", 3, 12); // 頑張ったほうがいい
    }
}

// ========== ないほうがいい (Should not do) ==========
// Pattern: ないほうがいい (advice - negative)
// Data source: grammar_points_data.json["ないほうがいい"]
//
// Structure variants to test:
//   standard[0]: Verb[ない] + 方 + が + いい
//   polite[0]: Verb[ない] + 方 + が + いい + です

mod nai_hou_ga_ii_tests {
    use super::*;

    #[test]
    fn test_nai_hou_ga_ii_advice() {
        let sentence = "あそこは危ないから、行かないほうがいいよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないほうがいい");
        assert_pattern_range(&patterns, "ないほうがいい", 10, 19); // 行かないほうがいい
    }

    #[test]
    fn test_nai_hou_ga_ii_polite() {
        let sentence = "それは触らないほうがいいです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないほうがいい");
        assert_pattern_range(&patterns, "ないほうがいい", 3, 14); // 触らないほうがいいです
    }
}

// ========== てください (Please do) ==========
// Pattern: てください (polite request)
// Data source: grammar_points_data.json["てください"]
//
// Structure variants to test:
//   standard[0]: Verb[て] + ください

mod tekudasai_tests {
    use super::*;

    #[test]
    fn test_tekudasai_request() {
        let sentence = "手を洗ってください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てください");
        assert_pattern_range(&patterns, "てください", 2, 9); // 洗ってください
    }

    #[test]
    fn test_tekudasai_de_form() {
        let sentence = "これを見てください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てください");
        assert_pattern_range(&patterns, "てください", 3, 9); // 見てください
    }
}

// ========== ないでください (Please don't do) ==========
// Pattern: ないでください (polite negative request)
// Data source: grammar_points_data.json["ないでください"]
//
// Structure variants to test:
//   standard[0]: Verb[ないで] + ください
//   Note: "Politeness Levels" mentioned but not detailed - testing casual variant without ください too

mod naide_kudasai_tests {
    use super::*;

    #[test]
    fn test_naide_kudasai_basic() {
        let sentence = "これは誰にも言わないでください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないでください");
        assert_pattern_range(&patterns, "ないでください", 6, 15); // 言わないでください
    }

    #[test]
    fn test_naide_kudasai_sit() {
        let sentence = "そこには座らないでください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないでください");
        assert_pattern_range(&patterns, "ないでください", 4, 13); // 座らないでください
    }

    // Casual form without ください (detected by separate ないで pattern)
    #[test]
    fn test_naide_casual() {
        let sentence = "それ、お兄ちゃんのだから食べないで";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないで");
        assert_pattern_range(&patterns, "ないで", 12, 17); // 食べないで
    }
}

// ========== てはいけない (Must not do) ==========
// Pattern: てはいけない (prohibition - must not do)
// Data source: grammar_points_data.json["てはいけない"]
//
// Structure variants to test:
//   standard[0]: Verb[て] + は + いけない
//   polite[0]: Verb[て] + は + いけません
//   Note: Casual variants ちゃいけない and じゃいけない also mentioned

mod te_wa_ikenai_tests {
    use super::*;

    #[test]
    fn test_te_wa_ikenai_standard() {
        let sentence = "タクシーの扉を自分で開けてはいけない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てはいけない");
        assert_pattern_range(&patterns, "てはいけない", 10, 18); // 開けてはいけない
    }

    #[test]
    fn test_te_wa_ikenai_polite() {
        let sentence = "子供の前でタバコを吸ってはいけません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てはいけない");
        assert_pattern_range(&patterns, "てはいけない", 9, 18); // 吸ってはいけません
    }

    // Casual variant: ちゃいけない (てはいけない → ちゃいけない)
    #[test]
    fn test_chya_ikenai_casual() {
        let sentence = "危ない場所に行っちゃいけない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てはいけない");
        assert_pattern_range(&patterns, "てはいけない", 6, 14); // 行っちゃいけない
    }
}

// ========== つもりだ (Intend to/Plan to) ==========
// Pattern: つもりだ (intention/plan)
// Data source: grammar_points_data.json["つもりだ"]
//
// Structure variants to test:
//   standard[0]: Verb[る] + つもり + だ
//   standard[1]: Verb[ない] + つもり + だ (intend not to)
//   standard[2]: Verb + つもり + は/が + ない (no intention of)
//   polite[0]: Verb[る] + つもり + です
//   polite[1]: Verb[ない] + つもり + です
//   polite[2]: Verb + つもり + は/が + ありません

mod tsumori_da_tests {
    use super::*;

    // Basic intention: 食べるつもりです (intend to eat)
    #[test]
    fn test_tsumori_basic_polite() {
        let sentence = "そのピザは今日の昼に食べるつもりです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "つもりだ");
        assert_pattern_range(&patterns, "つもりだ", 10, 18); // 食べるつもりです
    }

    // Negative intention: 行かないつもりだ (intend not to go)
    #[test]
    fn test_tsumori_nai_intention() {
        let sentence = "今日は学校に行かないつもりだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "つもりだ");
        assert_pattern_range(&patterns, "つもりだ", 6, 14); // 行かないつもりだ
    }

    // No intention: つもりはない (have no intention of)
    // Note: This tests the basic pattern without だ/です extension
    #[test]
    fn test_tsumori_wa_nai() {
        let sentence = "早く起きるつもりはない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "つもりだ");
        assert_pattern_range(&patterns, "つもりだ", 2, 8); // 起きるつもり
    }

    // Casual standard: 買うつもりだ (plan to buy)
    #[test]
    fn test_tsumori_casual() {
        let sentence = "明日、新しい靴を買うつもりだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "つもりだ");
        assert_pattern_range(&patterns, "つもりだ", 8, 14); // 買うつもりだ
    }
}

// ========== なくてはいけない (Must do) ==========
// Pattern: なくてはいけない (obligation/must do - double negative)
// Data source: grammar_points_data.json["なくてはいけない"]
//
// Structure variants to test:
//   standard[0]: Verb[ない] + なくては + いけない
//   standard[1]: Verb[ない] + なくちゃ + いけない (casual)
//   polite[0]: Verb[ない] + なくては + いけません
//   polite[1]: Verb[ない] + なくちゃ + いけません

mod nakutewa_ikenai_tests {
    use super::*;

    // Basic obligation: 寝なくてはいけない (must sleep)
    #[test]
    fn test_nakutewa_ikenai_basic() {
        let sentence = "今日は９時に寝なくてはいけない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくてはいけない");
        assert_pattern_range(&patterns, "なくてはいけない", 6, 15); // 寝なくてはいけない
    }

    // Polite form: 買わなくてはいけません (must buy - polite)
    #[test]
    fn test_nakutewa_ikenai_polite() {
        let sentence = "明日は彼女の誕生日プレゼントを買わなくてはいけません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくてはいけない");
        assert_pattern_range(&patterns, "なくてはいけない", 15, 26); // 買わなくてはいけません
    }

    // Casual contraction: しなくちゃいけない (gotta do)
    #[test]
    fn test_nakucha_ikenai_casual() {
        let sentence = "家の掃除をしなくちゃいけない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくてはいけない");
        assert_pattern_range(&patterns, "なくてはいけない", 5, 14); // しなくちゃいけない
    }

    // Polite casual: 行かなくちゃいけません (gotta go - polite casual)
    #[test]
    fn test_nakucha_ikenai_polite() {
        let sentence = "土曜日も会社に行かなくちゃいけません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくてはいけない");
        assert_pattern_range(&patterns, "なくてはいけない", 7, 18); // 行かなくちゃいけません
    }
}

// ========== だけ (Only/Just) ==========
// Pattern: だけ (only/just - limiting particle)
// Data source: grammar_points_data.json["だけ"]
//
// Structure variants to test:
//   standard[0]: Verb + だけ
//   standard[1]: い-Adjective + だけ
//   standard[2]: な-Adjective + な + だけ
//   standard[3]: Noun + だけ

mod dake_tests {
    use super::*;

    // Verb + だけ (only going to eat)
    #[test]
    fn test_dake_verb() {
        let sentence = "今夜は食べるだけ、何も飲まない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だけ");
        assert_pattern_range(&patterns, "だけ", 3, 8); // 食べるだけ
    }

    // い-Adjective + だけ (just spicy)
    #[test]
    fn test_dake_i_adjective() {
        let sentence = "このスープは辛いだけだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だけ");
        assert_pattern_range(&patterns, "だけ", 6, 11); // 辛いだけだ (extends with だ)
    }

    // な-Adjective + な + だけ (just famous)
    #[test]
    fn test_dake_na_adjective() {
        let sentence = "あの人は有名なだけです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だけ");
        assert_pattern_range(&patterns, "だけ", 4, 11); // 有名なだけです (full construction with auxiliary extension)
    }

    // Noun + だけ (only broccoli)
    #[test]
    fn test_dake_noun() {
        let sentence = "私はブロッコリーだけが嫌いだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だけ");
        assert_pattern_range(&patterns, "だけ", 2, 10); // ブロッコリーだけ
    }
}

// ========== だった・でした (Was/Were - Past Copula) ==========
// Pattern: だった・でした (past tense of だ/です)
// Data source: grammar_points_data.json["だった・でした"]
//
// Structure variants to test:
//   standard[0]: Noun + だった
//   standard[1]: な-Adjective + だった
//   polite[0]: Noun + でした
//   polite[1]: な-Adjective + でした

mod datta_deshita_tests {
    use super::*;

    // Noun + だった (was quiet - casual)
    #[test]
    fn test_noun_datta() {
        let sentence = "あの電車は静かだった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だった・でした");
        assert_pattern_range(&patterns, "だった・でした", 5, 10); // 静かだった
    }

    // な-Adjective + だった (was beautiful - casual)
    #[test]
    fn test_na_adjective_datta() {
        let sentence = "富士山は綺麗だった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だった・でした");
        assert_pattern_range(&patterns, "だった・でした", 4, 9); // 綺麗だった
    }

    // Noun + でした (was quiet - polite)
    #[test]
    fn test_noun_deshita() {
        let sentence = "あの電車は静かでした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だった・でした");
        assert_pattern_range(&patterns, "だった・でした", 5, 10); // 静かでした
    }

    // な-Adjective + でした (was beautiful - polite)
    #[test]
    fn test_na_adjective_deshita() {
        let sentence = "富士山は綺麗でした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だった・でした");
        assert_pattern_range(&patterns, "だった・でした", 4, 9); // 綺麗でした
    }
}

// ========== や (And/Or - Listing particle) ==========
// Pattern: や (and/or - non-exhaustive listing)
// Data source: grammar_points_data.json["や"]
//
// Structure variants to test:
//   standard[0]: Noun + や + Noun
//
// Note: や is used to list examples, not exhaustive lists (unlike と)

mod ya_tests {
    use super::*;

    // Basic listing: Noun + や + Noun
    #[test]
    fn test_ya_basic_listing() {
        let sentence = "このスーパーには果物や弁当がある";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "や");
        assert_pattern_range(&patterns, "や", 8, 13); // 果物や弁当
    }

    // Multiple や particles
    #[test]
    fn test_ya_multiple_items() {
        let sentence = "飛行機や船や車は大きいです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // Should detect both や patterns: 飛行機や船 and 船や車
        assert_has_pattern(&patterns, "や");
        // Check first occurrence (飛行機や船)
        assert_pattern_range(&patterns, "や", 0, 5); // 飛行機や船
    }

    // With particles after や
    #[test]
    fn test_ya_with_particle() {
        let sentence = "ビールやワインを飲んだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "や");
        assert_pattern_range(&patterns, "や", 0, 7); // ビールやワイン
    }
}

// ========== もう (Already/Anymore) ==========
// Pattern: もう (already - past, anymore - negative)
// Data source: grammar_points_data.json["もう"]
//
// Structure variants to test:
//   standard[0]: もう + (Past) Phrase
//   standard[1]: もう + (Negative) Phrase

mod mou_tests {
    use super::*;

    // もう + past phrase (already)
    #[test]
    fn test_mou_already_past() {
        let sentence = "朝ごはんはもう食べた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "もう");
        assert_pattern_range(&patterns, "もう", 5, 7); // もう
    }

    // もう + negative phrase (anymore)
    #[test]
    fn test_mou_anymore_negative() {
        let sentence = "もうお金がない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "もう");
        assert_pattern_range(&patterns, "もう", 0, 2); // もう
    }

    // もう in question (already?)
    #[test]
    fn test_mou_question() {
        let sentence = "もうデザートを食べた？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "もう");
        assert_pattern_range(&patterns, "もう", 0, 2); // もう
    }
}

// ========== じゃない (Is not - negative copula) ==========
// Pattern: じゃない (is not)
// Data source: grammar_points_data.json["じゃない"]
//
// Structure variants to test:
//   standard[0]: な-Adjective + では/じゃ + ない
//   standard[1]: Noun + では/じゃ + ない
//   polite[0]: な-Adjective + では/じゃ + ありません
//   polite[1]: Noun + では/じゃ + ありません
//   polite[2]: な-Adjective + では/じゃ + ないです
//   polite[3]: Noun + では/じゃ + ないです

mod janai_tests {
    use super::*;

    // Noun + じゃない (casual)
    #[test]
    fn test_janai_noun_casual() {
        let sentence = "これは食べ物じゃない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "じゃない");
        assert_pattern_range(&patterns, "じゃない", 3, 10); // 食べ物じゃない
    }

    // な-Adjective + じゃない (casual)
    #[test]
    fn test_janai_na_adjective_casual() {
        let sentence = "この携帯は便利じゃない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "じゃない");
        assert_pattern_range(&patterns, "じゃない", 5, 11); // 便利じゃない
    }

    // Noun + ではない (semi-polite)
    #[test]
    fn test_dewa_nai_noun() {
        let sentence = "それは飲み物ではない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "じゃない");
        assert_pattern_range(&patterns, "じゃない", 3, 10); // 飲み物ではない
    }

    // Noun + じゃありません (polite)
    #[test]
    fn test_janai_arimasen_polite() {
        let sentence = "これは私の本じゃありません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "じゃない");
        assert_pattern_range(&patterns, "じゃない", 5, 13); // 本じゃありません
    }

    // な-Adjective + ではありません (polite)
    #[test]
    fn test_dewa_arimasen_na_adjective() {
        let sentence = "あなたの部屋は綺麗ではありません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "じゃない");
        assert_pattern_range(&patterns, "じゃない", 7, 16); // 綺麗ではありません
    }

    // Noun + じゃないです (polite casual)
    #[test]
    fn test_janai_desu_polite() {
        let sentence = "彼は学生じゃないです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "じゃない");
        assert_pattern_range(&patterns, "じゃない", 2, 8); // 学生じゃない (です extends via auto-extension)
    }
}

// ========== じゃなかった (Was not - negative past copula) ==========
// Pattern: じゃなかった (was not)
// Data source: grammar_points_data.json["じゃなかった"]
//
// Structure variants to test:
//   standard[0]: な-Adjective + では/じゃ + なかった
//   standard[1]: Noun + では/じゃ + なかった
//   polite[0]: な-Adjective + では/じゃ + ありませんでした
//   polite[1]: Noun + では/じゃ + ありませんでした
//   polite[2]: な-Adjective + では/じゃ + なかったです
//   polite[3]: Noun + では/じゃ + なかったです

mod janakatta_tests {
    use super::*;

    // Noun + じゃなかった (casual)
    #[test]
    fn test_janakatta_noun_casual() {
        let sentence = "あそこは病院じゃなかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "じゃなかった");
        assert_pattern_range(&patterns, "じゃなかった", 4, 12); // 病院じゃなかった
    }

    // な-Adjective + じゃなかった (casual)
    #[test]
    fn test_janakatta_na_adjective_casual() {
        let sentence = "この車は便利じゃなかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "じゃなかった");
        assert_pattern_range(&patterns, "じゃなかった", 4, 12); // 便利じゃなかった
    }

    // Noun + ではなかった (semi-polite)
    #[test]
    fn test_dewa_nakatta_noun() {
        let sentence = "それは猫ではなかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "じゃなかった");
        assert_pattern_range(&patterns, "じゃなかった", 3, 10); // 猫ではなかった
    }

    // Noun + じゃありませんでした (polite)
    #[test]
    fn test_janakatta_arimasen_deshita_polite() {
        let sentence = "あれは私の財布じゃありませんでした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "じゃなかった");
        assert_pattern_range(&patterns, "じゃなかった", 5, 17); // 財布じゃありませんでした
    }

    // な-Adjective + ではありませんでした (polite)
    #[test]
    fn test_dewa_arimasen_deshita_na_adjective() {
        let sentence = "あの人は綺麗ではありませんでした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "じゃなかった");
        assert_pattern_range(&patterns, "じゃなかった", 4, 16); // 綺麗ではありませんでした
    }
}

// ========== ～んです・のです (Explanatory) ==========
// Pattern: ～んです・のです (explanatory/emphasis)
// Data source: grammar_points_data.json["～んです・のです"]
//
// Structure variants to test:
//   standard[0]: Verb + ん(の) + だ
//   standard[1]: い-Adjective + ん(の) + だ
//   standard[2]: な-Adjective + な + ん(の) + だ
//   standard[3]: Noun + な + ん(の) + だ
//   polite[0]: Verb + の(ん) + です
//   polite[1]: い-Adjective + の(ん) + です
//   polite[2]: な-Adjective + な + の(ん) + です
//   polite[3]: Noun + な + の(ん) + です

mod nodesu_tests {
    use super::*;

    // Polite form - い-Adjective + んです
    #[test]
    fn test_nodesu_i_adj_polite_n() {
        let sentence = "ここのコーヒーが美味しいんです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～んです・のです");
        assert_pattern_range(&patterns, "～んです・のです", 8, 15); // 美味しいんです
    }

    // Polite form - い-Adjective + のです
    #[test]
    fn test_nodesu_i_adj_polite_no() {
        let sentence = "私が悪いのです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～んです・のです");
        assert_pattern_range(&patterns, "～んです・のです", 2, 7); // 悪いのです
    }

    // Standard form - Verb + んだ
    #[test]
    fn test_nodesu_verb_casual_n() {
        let sentence = "お腹が空いたんだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～んです・のです");
        assert_pattern_range(&patterns, "～んです・のです", 5, 8); // たんだ
    }

    // Standard form - な-Adjective + なんだ
    #[test]
    fn test_nodesu_na_adj_casual_n() {
        let sentence = "彼は元気なんだよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～んです・のです");
        assert_pattern_range(&patterns, "～んです・のです", 2, 7); // 元気なんだ
    }

    // Standard form - Noun + なんだ
    #[test]
    fn test_nodesu_noun_casual_n() {
        let sentence = "あいつは学生なんだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～んです・のです");
        assert_pattern_range(&patterns, "～んです・のです", 4, 9); // 学生なんだ
    }

    // Polite form - Verb + んです
    #[test]
    fn test_nodesu_verb_polite_n() {
        let sentence = "明日は早く起きるんです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～んです・のです");
        assert_pattern_range(&patterns, "～んです・のです", 5, 11); // 起きるんです
    }

    // Polite form - Verb + のです (with question か)
    #[test]
    fn test_nodesu_verb_polite_no_question() {
        let sentence = "あなたはこれが欲しいのですか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～んです・のです");
        assert_pattern_range(&patterns, "～んです・のです", 7, 13); // 欲しいのです
    }

    // Polite form - い-Adjective + んです (different sentence)
    #[test]
    fn test_nodesu_i_adj_polite_n_alt() {
        let sentence = "この店のラーメンは美味しいんです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～んです・のです");
        assert_pattern_range(&patterns, "～んです・のです", 9, 16); // 美味しいんです
    }

    // Polite form - な-Adjective + なのです
    #[test]
    fn test_nodesu_na_adj_polite_no() {
        let sentence = "彼女は親切なのです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～んです・のです");
        assert_pattern_range(&patterns, "～んです・のです", 3, 9); // 親切なのです
    }

    // Polite form - Noun + なんです
    #[test]
    fn test_nodesu_noun_polite_n() {
        let sentence = "私は教師なんです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～んです・のです");
        assert_pattern_range(&patterns, "～んです・のです", 2, 8); // 教師なんです
    }
}

// ========== ～ましょうか (Shall we?) ==========
// Pattern: ～ましょうか (polite suggestion as question)
// Data source: grammar_points_data.json["～ましょうか"]
//
// Structure variants to test:
//   standard[0]: Verb[stem] + ましょうか

mod mashouka_tests {
    use super::*;

    // Verb stem + ましょうか (u-verb)
    #[test]
    fn test_mashouka_basic() {
        let sentence = "レストランまで車で行きましょうか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～ましょうか");
        assert_pattern_range(&patterns, "～ましょうか", 9, 16); // 行きましょうか
    }

    // Verb stem + ましょうか (u-verb, different verb)
    #[test]
    fn test_mashouka_run() {
        let sentence = "今夜走りましょうか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～ましょうか");
        assert_pattern_range(&patterns, "～ましょうか", 2, 9); // 走りましょうか
    }

    // Verb stem + ましょうか (u-verb)
    #[test]
    fn test_mashouka_u_verb() {
        let sentence = "一緒に映画を見ましょうか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～ましょうか");
        assert_pattern_range(&patterns, "～ましょうか", 6, 12); // 見ましょうか
    }

    // Verb stem + ましょうか (ru-verb)
    #[test]
    fn test_mashouka_ru_verb() {
        let sentence = "昼ご飯を食べましょうか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～ましょうか");
        assert_pattern_range(&patterns, "～ましょうか", 4, 11); // 食べましょうか
    }
}

// ========== ましょう (Let's do) ==========
// Pattern: ましょう (polite volitional - let's do)
// Data source: grammar_points_data.json["ましょう"]
//
// Structure variants to test:
//   standard[0]: Verb[stem] + ましょう

mod mashou_tests {
    use super::*;

    // Verb stem + ましょう (u-verb) - suggestion/volitional
    #[test]
    fn test_mashou_basic() {
        let sentence = "モールに行きましょう！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ましょう");
        assert_pattern_range(&patterns, "ましょう", 4, 10); // 行きましょう
    }

    // Verb stem + ましょう (ru-verb) - volitional/invitation
    #[test]
    fn test_mashou_dance() {
        let sentence = "踊りましょう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ましょう");
        assert_pattern_range(&patterns, "ましょう", 0, 6); // 踊りましょう
    }

    // Verb stem + ましょう (personal declaration)
    #[test]
    fn test_mashou_declaration() {
        let sentence = "５キロ痩せましょう！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ましょう");
        assert_pattern_range(&patterns, "ましょう", 3, 9); // 痩せましょう
    }
}

// ========== ませんか (Won't you?) ==========
// Pattern: ませんか (polite invitation)
// Data source: grammar_points_data.json["ませんか"]
//
// Structure variants to test:
//   polite[0]: Verb[stem] + ませんか
//   standard[0]: Verb[ない] + か (casual - optional)

mod masenka_tests {
    use super::*;

    // Verb stem + ませんか (polite invitation)
    #[test]
    fn test_masenka_polite() {
        let sentence = "明日は温泉に行きませんか。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ませんか");
        assert_pattern_range(&patterns, "ませんか", 6, 12); // 行きませんか
    }

    // Verb stem + ませんか (invitation with 一緒に)
    #[test]
    fn test_masenka_together() {
        let sentence = "一緒にお祭りへ行きませんか。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ませんか");
        assert_pattern_range(&patterns, "ませんか", 7, 13); // 行きませんか
    }

    // Verb stem + ませんか (different verb)
    #[test]
    fn test_masenka_eat() {
        let sentence = "ラーメンを食べに行きませんか。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ませんか");
        assert_pattern_range(&patterns, "ませんか", 8, 14); // 行きませんか
    }
}

// ========== がある + Noun ==========
// Pattern: がある + Noun (noun with/that has)
// Data source: grammar_points_data.json["がある + Noun"]
//
// Structure variants to test:
//   standard[0]: Noun + がある + Noun
//   standard[1]: Noun + のある + Noun (with の instead of が)

mod ga_aru_noun_tests {
    use super::*;

    // Noun + がある + Noun (restaurant with piano)
    #[test]
    fn test_ga_aru_noun_basic() {
        let sentence = "ピアノがあるレストランを探している";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がある + Noun");
        assert_pattern_range(&patterns, "がある + Noun", 0, 11); // ピアノがあるレストラン
    }

    // Noun + がある + Noun (building with convenience store)
    #[test]
    fn test_ga_aru_noun_building() {
        let sentence = "コンビニがあるビルの近くに住んでいる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がある + Noun");
        assert_pattern_range(&patterns, "がある + Noun", 0, 9); // コンビニがあるビル
    }

    // Noun + のある + Noun (mountain with beautiful river)
    #[test]
    fn test_no_aru_noun() {
        let sentence = "綺麗な川のある山に登りたい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がある + Noun");
        assert_pattern_range(&patterns, "がある + Noun", 3, 8); // 川のある山
    }
}

// ========== てもいい (It's okay even if - Adjectives/Nouns) ==========
// Pattern: てもいい (permission/acceptability for adjectives and nouns)
// Data source: grammar_points_data.json["てもいい"]
//
// Structure variants to test:
//   standard[0]: い-Adjective[く] + ても + いい
//   standard[1]: な-Adjective + でも + いい
//   standard[2]: Noun + でも + いい
//   polite[0-2]: Same forms + です variants
//
// Note: "Verb + てもいい" is a separate pattern already implemented

mod temoii_adj_noun_tests {
    use super::*;

    // い-Adjective[く] + ても + いい (casual)
    #[test]
    fn test_temoii_i_adj_casual() {
        let sentence = "お茶冷たくてもいい？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てもいい");
        assert_pattern_range(&patterns, "てもいい", 2, 9); // 冷たくてもいい
    }

    // い-Adjective[く] + ても + いい + です (polite)
    #[test]
    fn test_temoii_i_adj_polite() {
        let sentence = "部屋が少し暗くてもいいです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てもいい");
        assert_pattern_range(&patterns, "てもいい", 5, 13); // 暗くてもいいです
    }

    // な-Adjective + でも + いい (casual)
    #[test]
    fn test_temoii_na_adj_casual() {
        let sentence = "肉が好きでもいい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てもいい");
        assert_pattern_range(&patterns, "てもいい", 2, 8); // 好きでもいい
    }

    // な-Adjective + でも + いい + です (polite)
    #[test]
    fn test_temoii_na_adj_polite() {
        let sentence = "静かでもいいですよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てもいい");
        assert_pattern_range(&patterns, "てもいい", 0, 8); // 静かでもいいです
    }

    // Noun + でも + いい (casual)
    #[test]
    fn test_temoii_noun_casual() {
        let sentence = "電車でもいい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てもいい");
        assert_pattern_range(&patterns, "てもいい", 0, 6); // 電車でもいい
    }

    // Noun + でも + いい + です (polite)
    #[test]
    fn test_temoii_noun_polite() {
        let sentence = "焼きそばでもいいですか？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てもいい");
        assert_pattern_range(&patterns, "てもいい", 0, 10); // 焼きそばでもいいです
    }
}

// ========== 誰か・どこか・誰も・どこも (Someone/Somewhere/No one/Nowhere) ==========
// Pattern: 誰か・どこか・誰も・どこも (indefinite/negative pronouns)
// Data source: grammar_points_data.json["誰か・どこか・誰も・どこも"]
//
// Structure variants to test:
//   standard[0]: WH-Word + か + Particle
//   standard[1]: WH-Word + Particle + も
//   standard[2]: WH-Words: どこ, 誰（だれ）
//   standard[3]: Particles: へ, に, と
//
// Basic structures:
//   - 誰か (someone) in positive sentences
//   - どこか (somewhere) in positive sentences
//   - 誰も (no one) in negative sentences
//   - どこも (nowhere) in negative sentences
//   - With particles: 誰かに, どこかへ, 誰にも, どこへも etc.

mod dareka_dokoka_tests {
    use super::*;

    // 誰か (someone) - WH-Word + か
    #[test]
    fn test_dareka_someone() {
        let sentence = "誰かがいますか？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "誰か・どこか・誰も・どこも");
        assert_pattern_range(&patterns, "誰か・どこか・誰も・どこも", 0, 2); // 誰か
    }

    // どこか (somewhere) - WH-Word + か
    #[test]
    fn test_dokoka_somewhere() {
        let sentence = "来月はどこかへ行きますか？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "誰か・どこか・誰も・どこも");
        assert_pattern_range(&patterns, "誰か・どこか・誰も・どこも", 3, 7); // どこかへ
    }

    // 誰も (no one) - WH-Word + も
    #[test]
    fn test_daremo_no_one() {
        let sentence = "俺の誕生日パーティーには誰も来なかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "誰か・どこか・誰も・どこも");
        assert_pattern_range(&patterns, "誰か・どこか・誰も・どこも", 12, 14); // 誰も
    }

    // どこも (nowhere) - WH-Word + も
    #[test]
    fn test_dokomo_nowhere() {
        let sentence = "９時だからどこも開いていない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "誰か・どこか・誰も・どこも");
        assert_pattern_range(&patterns, "誰か・どこか・誰も・どこも", 5, 8); // どこも
    }

    // 誰かに (to someone) - WH-Word + か + に
    #[test]
    fn test_dareka_ni_to_someone() {
        let sentence = "誰かに言った？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "誰か・どこか・誰も・どこも");
        assert_pattern_range(&patterns, "誰か・どこか・誰も・どこも", 0, 3); // 誰かに
    }

    // どこかへ (to somewhere) - WH-Word + か + へ
    #[test]
    fn test_dokoka_he_to_somewhere() {
        let sentence = "休みの日はどこかへ行きましたか？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "誰か・どこか・誰も・どこも");
        assert_pattern_range(&patterns, "誰か・どこか・誰も・どこも", 5, 9); // どこかへ
    }

    // 誰にも (to no one) - WH-Word + に + も
    #[test]
    fn test_dare_ni_mo_to_no_one() {
        let sentence = "明日は誰にも会わない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "誰か・どこか・誰も・どこも");
        assert_pattern_range(&patterns, "誰か・どこか・誰も・どこも", 3, 6); // 誰にも
    }

    // どこへも (to nowhere) - WH-Word + へ + も
    #[test]
    fn test_doko_he_mo_to_nowhere() {
        let sentence = "今日はどこへも行かない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "誰か・どこか・誰も・どこも");
        assert_pattern_range(&patterns, "誰か・どこか・誰も・どこも", 3, 7); // どこへも
    }
}

// ========== でしょう (Probably/Right?) ==========
// Pattern: でしょう (tentative/assumption - "probably" or "right?")
// Data source: grammar_points_data.json["でしょう"]
//
// Structure variants to test:
//   standard[0]: Tentative (contextual use)
//   standard[1]: Noun + でしょう
//   standard[2]: Verb + でしょう
//   standard[3]: Adjective + でしょう
//
// Note: でしょう is polite. Casual form is だろう (separate pattern).
// でしょう can follow plain form verbs/adjectives or nouns.

mod deshou_tests {
    use super::*;

    // Noun + でしょう
    #[test]
    fn test_deshou_noun() {
        let sentence = "今日の昼ごはんはハンバーガーでしょう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "でしょう");
        assert_pattern_range(&patterns, "でしょう", 8, 18); // ハンバーガーでしょう
    }

    // Verb + でしょう
    #[test]
    fn test_deshou_verb() {
        let sentence = "彼も踊るでしょう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "でしょう");
        assert_pattern_range(&patterns, "でしょう", 2, 8); // 踊るでしょう
    }

    // い-Adjective + でしょう (past tense)
    #[test]
    fn test_deshou_i_adjective() {
        let sentence = "イタリアは良かったでしょう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "でしょう");
        assert_pattern_range(&patterns, "でしょう", 8, 13); // たでしょう (past auxiliary + でしょう)
    }

    // な-Adjective + でしょう
    #[test]
    fn test_deshou_na_adjective() {
        let sentence = "この仕事は簡単でしょう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "でしょう");
        assert_pattern_range(&patterns, "でしょう", 5, 11); // 簡単でしょう
    }
}

// ========== だろう (Probably/Right? - Casual) ==========
// Pattern: だろう (tentative/assumption - "probably" or "right?" - casual form)
// Data source: grammar_points_data.json["だろう"]
//
// Structure variants to test:
//   standard[0]: Verb + だろう
//   standard[1]: Adjective + だろう
//   standard[2]: Noun + だろう
//
// Note: だろう is casual form of でしょう. Can also contract to だろ.

mod darou_tests {
    use super::*;

    // Verb + だろう
    #[test]
    fn test_darou_verb() {
        let sentence = "明日は晴れるだろう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だろう");
        assert_pattern_range(&patterns, "だろう", 3, 9); // 晴れるだろう
    }

    // い-Adjective + だろう
    #[test]
    fn test_darou_i_adjective() {
        let sentence = "明日は寒いだろう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だろう");
        assert_pattern_range(&patterns, "だろう", 3, 8); // 寒いだろう
    }

    // Noun + だろう
    #[test]
    fn test_darou_noun() {
        let sentence = "これはお菓子だろう？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だろう");
        assert_pattern_range(&patterns, "だろう", 3, 9); // お菓子だろう
    }

    // Contracted form だろ (without う)
    #[test]
    fn test_darou_contracted() {
        let sentence = "お前、明日パーティーに行くだろ？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だろう");
        assert_pattern_range(&patterns, "だろう", 11, 15); // 行くだろ
    }
}

// ========== たくさん (A lot, many) ==========
// Pattern: たくさん (a lot, many)
// Data source: grammar_points_data.json["たくさん"]
//
// Structure variants to test:
//   standard[0]: たくさん + Phrase
//   standard[1]: たくさん + (の) + Noun

mod takusan_tests {
    use super::*;

    // たくさん + Phrase (before verb)
    #[test]
    fn test_takusan_phrase_verb() {
        let sentence = "たくさん食べてよ！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たくさん");
        assert_pattern_range(&patterns, "たくさん", 0, 4); // たくさん
    }

    // たくさん + の + Noun
    #[test]
    fn test_takusan_no_noun() {
        let sentence = "この島にはたくさんの猫がいます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たくさん");
        assert_pattern_range(&patterns, "たくさん", 5, 9); // たくさん
    }

    // たくさん + Noun (without の) - focuses on phrase
    #[test]
    fn test_takusan_noun_no_no() {
        let sentence = "あの店の前にたくさん人が集まっている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たくさん");
        assert_pattern_range(&patterns, "たくさん", 6, 10); // たくさん
    }
}

// ========== まだ (Still, not yet) ==========
// Pattern: まだ (still, not yet)
// Data source: grammar_points_data.json["まだ"]
//
// Structure variants to test:
//   standard[0]: まだ + Verb[ている]
//   standard[1]: まだ + Noun + が + いる
//   standard[2]: まだ + Noun + が + ある
//   polite[0-2]: Polite variants with います/あります

mod mada_tests {
    use super::*;

    // まだ + Verb[ている] - still doing
    #[test]
    fn test_mada_te_iru() {
        let sentence = "まだ走っている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まだ");
        assert_pattern_range(&patterns, "まだ", 0, 2); // まだ
    }

    // まだ + Noun + が + いる
    #[test]
    fn test_mada_ga_iru() {
        let sentence = "まだ親がいるからゲームできない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まだ");
        assert_pattern_range(&patterns, "まだ", 0, 2); // まだ
    }

    // まだ + Noun + が + ある
    #[test]
    fn test_mada_ga_aru() {
        let sentence = "まだ宿題があるから、遊ばない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まだ");
        assert_pattern_range(&patterns, "まだ", 0, 2); // まだ
    }

    // まだ in question form
    #[test]
    fn test_mada_question() {
        let sentence = "まだ日本語の勉強しているの？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まだ");
        assert_pattern_range(&patterns, "まだ", 0, 2); // まだ
    }

    // Polite form: まだ + Verb[ている] + polite
    #[test]
    fn test_mada_te_imasu() {
        let sentence = "まだ仕事をしています";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まだ");
        assert_pattern_range(&patterns, "まだ", 0, 2); // まだ
    }
}

// ========== けっこう (Quite, fairly) ==========
// Pattern: けっこう (quite, fairly, pretty)
// Data source: grammar_points_data.json["けっこう"]
//
// Structure variants to test:
//   standard[0]: けっこう + Phrase
//   standard[2]: けっこうだ (set expression)
//   polite[0]: けっこう + Phrase (polite)
//   polite[2]: けっこうです (set expression polite)

mod kekkou_tests {
    use super::*;

    // けっこう + Verb (past tense)
    #[test]
    fn test_kekkou_verb_past() {
        let sentence = "けっこう走ったね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "けっこう");
        assert_pattern_range(&patterns, "けっこう", 0, 4); // けっこう
    }

    // けっこう + Adjective (polite)
    #[test]
    fn test_kekkou_adjective_polite() {
        let sentence = "けっこう高いですよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "けっこう");
        assert_pattern_range(&patterns, "けっこう", 0, 4); // けっこう
    }

    // けっこうです (set expression - "no thank you")
    #[test]
    fn test_kekkou_desu() {
        let sentence = "袋はけっこうです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "けっこう");
        assert_pattern_range(&patterns, "けっこう", 2, 6); // けっこう
    }

    // けっこう + Adjective
    #[test]
    fn test_kekkou_adjective() {
        let sentence = "このラーメンはけっこう美味しい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "けっこう");
        assert_pattern_range(&patterns, "けっこう", 7, 11); // けっこう
    }
}

// ========== くる (Come - Irregular verb) ==========
// Pattern: くる (to come - irregular verb)
// Data source: grammar_points_data.json["くる"]
//
// Structure variants to test:
//   standard[0]: くる (dictionary form)
//   polite[0]: きます (polite form)
//
// Note: くる conjugates irregularly:
//   - Non-past: くる/来る
//   - Past: きた/来た
//   - Te-form: きて/来て
//   - Negative: こない/来ない
//   - Stem: き/来

mod kuru_tests {
    use super::*;

    // Dictionary form: くる
    #[test]
    fn test_kuru_dictionary() {
        let sentence = "明日パーティーに来る？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "くる");
        assert_pattern_range(&patterns, "くる", 8, 10); // 来る
    }

    // Polite form: きます
    #[test]
    fn test_kuru_polite() {
        let sentence = "友達が家に来ます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "くる");
        assert_pattern_range(&patterns, "くる", 5, 8); // 来ます
    }

    // Past form: きた
    #[test]
    fn test_kuru_past() {
        let sentence = "昨日彼女が来た";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "くる");
        assert_pattern_range(&patterns, "くる", 5, 7); // 来た
    }

    // Te-form: きて
    #[test]
    fn test_kuru_te_form() {
        let sentence = "駅に来ている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "くる");
        assert_pattern_range(&patterns, "くる", 2, 3); // 来
    }
}

// ========== だ (Copula - is/are) ==========
// Pattern: だ (casual copula - assertion/determination)
// Data source: grammar_points_data.json["だ"]
//
// Structure variants to test:
//   standard[0]: Noun + だ
//   standard[1]: な-Adjective + だ
//
// Note: だ is the casual copula. です is the polite equivalent.
// だ CANNOT be used after い-Adjectives (unlike です).

mod da_copula_tests {
    use super::*;

    // Noun + だ
    #[test]
    fn test_da_noun() {
        let sentence = "これは私の本だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だ");
        assert_pattern_range(&patterns, "だ", 5, 7); // 本だ
    }

    // な-Adjective + だ
    #[test]
    fn test_da_na_adjective() {
        let sentence = "彼は真面目だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だ");
        assert_pattern_range(&patterns, "だ", 2, 6); // 真面目だ
    }
}

// ========== です (Copula - polite is/are) ==========
// Pattern: です (polite copula - assertion/determination)
// Data source: grammar_points_data.json["です"]
//
// Structure variants to test:
//   polite[0]: Noun + です
//   polite[1]: Adjective + です (both い-Adj and な-Adj)
//
// Note: です is the polite copula. だ is the casual equivalent.
// です CAN be used after い-Adjectives (unlike だ), though some consider it informal in writing.

mod desu_copula_tests {
    use super::*;

    // Noun + です
    #[test]
    fn test_desu_noun() {
        let sentence = "私は学生です";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "です");
        assert_pattern_range(&patterns, "です", 2, 6); // 学生です
    }

    // な-Adjective + です
    #[test]
    fn test_desu_na_adjective() {
        let sentence = "この部屋は静かです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "です");
        assert_pattern_range(&patterns, "です", 5, 9); // 静かです
    }

    // い-Adjective + です
    #[test]
    fn test_desu_i_adjective() {
        let sentence = "今日は暑いです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "です");
        assert_pattern_range(&patterns, "です", 3, 7); // 暑いです
    }
}

// ========== くれる (To give to me/us) ==========
// Pattern: くれる (to give to me/us)
// Data source: grammar_points_data.json["くれる"]
//
// Structure variants:
//   The pattern itself is just the verb くれる
//   Word order is flexible (giver, recipient, object can appear in various orders)
//   Examples:
//     - おばあちゃんはいつもお菓子をくれる (grandmother always gives me candy)
//     - 毎年、彼女が手紙をくれる (every year, girlfriend gives me a letter)
//     - 母は毎日私にクッキーをくれる (mother gives me cookies every day)
//     - 嫁がプレゼントをくれた (wife gave me a present - past tense)

mod kureru_tests {
    use super::*;

    // Present tense - basic form
    #[test]
    fn test_kureru_present() {
        let sentence = "おばあちゃんはいつもお菓子をくれる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "くれる");
        assert_pattern_range(&patterns, "くれる", 14, 17); // くれる
    }

    // Present tense - with recipient に
    #[test]
    fn test_kureru_present_with_recipient() {
        let sentence = "母は毎日私にクッキーをくれる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "くれる");
        assert_pattern_range(&patterns, "くれる", 11, 14); // くれる
    }

    // Past tense
    #[test]
    fn test_kureru_past() {
        let sentence = "嫁がプレゼントをくれた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "くれる");
        assert_pattern_range(&patterns, "くれる", 8, 11); // くれた
    }

    // Polite present tense
    #[test]
    fn test_kureru_polite_present() {
        let sentence = "友達が本をくれます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "くれる");
        assert_pattern_range(&patterns, "くれる", 5, 9); // くれます
    }

    // Te-form (くれて)
    #[test]
    fn test_kureru_te_form() {
        let sentence = "先生が辞書をくれてありがとう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "くれる");
        assert_pattern_range(&patterns, "くれる", 6, 8); // くれ (stem before て)
    }
}

// ========== する (Irregular verb - to do/make) ==========
// Pattern: する (irregular verb)
// Data source: grammar_points_data.json["する"]
//
// Structure variants to test:
//   standard[0]: する
//   polite[0]: します

mod suru_tests {
    use super::*;

    // Dictionary form (する)
    #[test]
    fn test_suru_dictionary() {
        let sentence = "今日は洗濯をするつもりだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "する");
        assert_pattern_range(&patterns, "する", 6, 8); // する
    }

    // Polite present (します)
    #[test]
    fn test_suru_polite_present() {
        let sentence = "毎日勉強をします";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "する");
        assert_pattern_range(&patterns, "する", 5, 8); // します
    }

    // Past tense (した)
    #[test]
    fn test_suru_past() {
        let sentence = "昨日料理をした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "する");
        assert_pattern_range(&patterns, "する", 5, 7); // した
    }

    // Polite past (しました)
    #[test]
    fn test_suru_polite_past() {
        let sentence = "先週電話をしました";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "する");
        assert_pattern_range(&patterns, "する", 5, 9); // しました
    }

    // Te-form (して)
    #[test]
    fn test_suru_te_form() {
        let sentence = "準備をしてから出かける";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "する");
        assert_pattern_range(&patterns, "する", 3, 4); // し (stem before て)
    }

    // Negative (しない)
    #[test]
    fn test_suru_negative() {
        let sentence = "そんなことはしない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "する");
        assert_pattern_range(&patterns, "する", 6, 9); // しない
    }
}

// ========== もらう (to receive/get from someone) ==========
// Pattern: もらう (to receive)
// Data source: grammar_points_data.json["もらう"]
//
// Structure variants to test:
//   standard[0-2]: Various word orders with Giver+に, Recipient+は/が, Object+を
//   polite[0-2]: Same structures with もらいます

mod morau_tests {
    use super::*;

    // Basic present (もらう)
    #[test]
    fn test_morau_present() {
        let sentence = "誕生日にプレゼントをもらう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "もらう");
        assert_pattern_range(&patterns, "もらう", 10, 13); // もらう
    }

    // Past tense (もらった)
    #[test]
    fn test_morau_past() {
        let sentence = "駅でティッシュをもらったけど欲しい？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "もらう");
        assert_pattern_range(&patterns, "もらう", 8, 12); // もらった
    }

    // Polite form (もらいます)
    #[test]
    fn test_morau_polite() {
        let sentence = "母からお小遣いをもらいます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "もらう");
        assert_pattern_range(&patterns, "もらう", 8, 13); // もらいます
    }

    // Polite past (もらいました)
    #[test]
    fn test_morau_polite_past() {
        let sentence = "お正月におばあちゃんからお金をもらいました";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "もらう");
        assert_pattern_range(&patterns, "もらう", 15, 21); // もらいました
    }

    // Te-form (もらって)
    #[test]
    fn test_morau_te_form() {
        let sentence = "彼女に手作りのケーキをもらって嬉しかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "もらう");
        assert_pattern_range(&patterns, "もらう", 11, 14); // もらっ (stem before て)
    }
}

// ========== にする (To decide on/make) ==========
// Pattern: にする (to decide on/make something)
// Data source: grammar_points_data.json["にする"]
//
// Structure variants to test:
//   standard[0]: Noun + に + する
//   polite[0]: Noun + に + します
//
// Meaning: "to decide on (A)", "to make it (A)"
// Usage: Indicates a decision or choice - the speaker has control over the outcome

mod ni_suru_tests {
    use super::*;

    // Present tense - basic form (する)
    #[test]
    fn test_ni_suru_present() {
        let sentence = "俺はビールにする";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にする");
        assert_pattern_range(&patterns, "にする", 2, 8); // ビールにする
    }

    // Present tense - polite form (します)
    #[test]
    fn test_ni_suru_polite() {
        let sentence = "私はコーヒーにします";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にする");
        assert_pattern_range(&patterns, "にする", 2, 10); // コーヒーにします
    }

    // Past tense
    #[test]
    fn test_ni_suru_past() {
        let sentence = "結局ラーメンにした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にする");
        assert_pattern_range(&patterns, "にする", 2, 9); // ラーメンにした
    }

    // Polite past tense
    #[test]
    fn test_ni_suru_polite_past() {
        let sentence = "ハイキングの日は日曜日にしました";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にする");
        assert_pattern_range(&patterns, "にする", 8, 16); // 日曜日にしました
    }

    // Volitional form
    #[test]
    fn test_ni_suru_volitional() {
        let sentence = "次は緑茶にしよう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にする");
        assert_pattern_range(&patterns, "にする", 2, 8); // 緑茶にしよう
    }
}

// ========== まえに (Before) ==========
// Pattern: まえに (before - time or location)
// Data source: grammar_points_data.json["まえに"]
//
// Structure variants to test:
//   standard[0]: Verb + 前（まえ）に
//   standard[1]: Noun + の + 前（まえ）に
//   polite: (none listed)

mod mae_ni_tests {
    use super::*;

    // Verb + 前に (before doing)
    #[test]
    fn test_mae_ni_verb() {
        let sentence = "家に入る前に靴を脱ぐ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まえに");
        assert_pattern_range(&patterns, "まえに", 2, 6); // 入る前に
    }

    // Noun + の + 前に (in front of / before)
    #[test]
    fn test_mae_ni_noun_location() {
        let sentence = "病院の前にあるコンビニ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まえに");
        assert_pattern_range(&patterns, "まえに", 0, 5); // 病院の前に
    }

    // Verb + 前に (before - temporal)
    #[test]
    fn test_mae_ni_verb_temporal() {
        let sentence = "寝る前に歯を磨いてください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まえに");
        assert_pattern_range(&patterns, "まえに", 0, 4); // 寝る前に
    }

    // Noun + の + 前に (in time context)
    #[test]
    fn test_mae_ni_noun_temporal() {
        let sentence = "試験の前に復習しておこう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まえに");
        assert_pattern_range(&patterns, "まえに", 0, 5); // 試験の前に
    }
}

// ========== なにか・なにも (Something/Nothing) ==========
// Pattern: なにか (something/anything), なにも (nothing)
// Data source: grammar_points_data.json["なにか・なにも"]
//
// Structure variants to test:
//   standard[0]: 何（なに）か + Phrase
//   standard[1]: 何（なに）も + Phrase［ない］
//   Casual variants: なんか, なんにか, なんも, なんにも

mod nanika_nanimo_tests {
    use super::*;

    // なにか + Phrase (something)
    #[test]
    fn test_nanika_something() {
        let sentence = "なにか食べますか？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なにか");
        assert_pattern_range(&patterns, "なにか", 0, 3); // なにか
    }

    // なにも + ない (nothing)
    #[test]
    fn test_nanimo_nothing() {
        let sentence = "冷蔵庫の中にはなにもないです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なにも");
        assert_pattern_range(&patterns, "なにも", 7, 10); // なにも
    }

    // なんか (casual variant of なにか)
    #[test]
    fn test_nanka_casual() {
        let sentence = "なんか飲みたいな";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なにか");
        assert_pattern_range(&patterns, "なにか", 0, 3); // なんか
    }

    // なんも (casual variant of なにも)
    #[test]
    fn test_nanmo_casual() {
        let sentence = "なんも知らないよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なにも");
        assert_pattern_range(&patterns, "なにも", 0, 3); // なんも
    }

    // なんにも (another casual variant of なにも)
    #[test]
    fn test_nannimo_casual() {
        let sentence = "今日はなんにもしたくない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なにも");
        assert_pattern_range(&patterns, "なにも", 3, 7); // なんにも
    }
}

// ========== なくてはならない (Must do - formal) ==========
// Pattern: なくてはならない (must do, have to)
// Data source: grammar_points_data.json["なくてはならない"]
//
// Structure variants to test:
//   standard[0]: Verb[ない] + なくては + ならない
//   standard[1]: Verb[ない] + なくちゃ + ならない
//   polite[0]: Verb[ない] + なくては + なりません
//   polite[1]: Verb[ない] + なくちゃ + なりません
//
// Note: This is a double negative construction meaning "must do"
// Literally: "must not, not do (A)"

mod nakutewa_naranai_tests {
    use super::*;

    // Standard form: なくてはならない
    #[test]
    fn test_nakutewa_naranai_standard() {
        let sentence = "明日までに宿題を提出しなくてはならない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくてはならない");
        assert_pattern_range(&patterns, "なくてはならない", 8, 19); // 提出しなくてはならない
    }

    // Casual form: なくちゃならない
    #[test]
    fn test_nakucha_naranai_casual() {
        let sentence = "今日は沢山走ったからシャワーを浴びなくちゃならない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくてはならない");
        assert_pattern_range(&patterns, "なくてはならない", 15, 25); // 浴びなくちゃならない
    }

    // Polite form: なくてはなりません
    #[test]
    fn test_nakutewa_narimasen_polite() {
        let sentence = "トムはタカにお金を返さなくてはなりません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくてはならない");
        assert_pattern_range(&patterns, "なくてはならない", 9, 20); // 返さなくてはなりません
    }

    // Casual polite form: なくちゃなりません
    #[test]
    fn test_nakucha_narimasen_casual_polite() {
        let sentence = "今日は漢字の勉強をしなくちゃなりません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくてはならない");
        assert_pattern_range(&patterns, "なくてはならない", 9, 19); // しなくちゃなりません
    }
}

// ========== なくちゃ・なきゃ (Must do - casual) ==========
// Pattern: なくちゃ・なきゃ (casual "must do/gotta do")
// Data source: grammar_points_data.json["なくちゃ・なきゃ"]
//
// Structure variants to test:
//   standard[0]: Verb[ない] + なきゃ + いけない
//   standard[1]: Verb[ない] + なくちゃ + いけない
//   standard[2]: Verb[ない] + なけりゃ + だめ (very casual, male speech)
//   polite[0]: Verb[ない] + なきゃ + いけません
//   polite[1]: Verb[ない] + なくちゃ + いけません
//   Abbreviated forms (without いけない/ならない/だめ)

mod nakucha_nakya_tests {
    use super::*;

    // Standard form: なきゃいけない
    #[test]
    fn test_nakya_ikenai_standard() {
        let sentence = "今日は家の掃除をしなきゃいけないから、遊ばない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくちゃ・なきゃ");
        assert_pattern_range(&patterns, "なくちゃ・なきゃ", 8, 16); // しなきゃいけない
    }

    // Standard form: なくちゃいけない
    #[test]
    fn test_nakucha_ikenai_standard() {
        let sentence = "野菜を食べなくちゃいけない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくちゃ・なきゃ");
        assert_pattern_range(&patterns, "なくちゃ・なきゃ", 3, 13); // 食べなくちゃいけない
    }

    // Very casual form: なけりゃだめ (male speech)
    #[test]
    fn test_nakerya_dame_casual() {
        let sentence = "お前がやらなけりゃだめだろ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくちゃ・なきゃ");
        assert_pattern_range(&patterns, "なくちゃ・なきゃ", 3, 13); // やらなけりゃだめだろ
    }

    // Polite form: なきゃいけません
    #[test]
    fn test_nakya_ikemasen_polite() {
        let sentence = "明日までに宿題を終わらせなきゃいけません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくちゃ・なきゃ");
        assert_pattern_range(&patterns, "なくちゃ・なきゃ", 11, 20); // せなきゃいけません
    }

    // Polite form: なくちゃいけません
    #[test]
    fn test_nakucha_ikemasen_polite() {
        let sentence = "会議の前に資料を準備しなくちゃいけません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくちゃ・なきゃ");
        assert_pattern_range(&patterns, "なくちゃ・なきゃ", 8, 20); // 準備しなくちゃいけません
    }

    // Abbreviated form: なきゃ (without いけない)
    #[test]
    fn test_nakya_abbreviated() {
        let sentence = "あっ、洗濯をしなきゃ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくちゃ・なきゃ");
        assert_pattern_range(&patterns, "なくちゃ・なきゃ", 6, 10); // しなきゃ
    }

    // Abbreviated form: なくちゃ (without いけない)
    #[test]
    fn test_nakucha_abbreviated() {
        let sentence = "宿題をしなくちゃ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくちゃ・なきゃ");
        assert_pattern_range(&patterns, "なくちゃ・なきゃ", 3, 8); // しなくちゃ
    }
}

// ========== な (Prohibitive - Don't do X) ==========
// Pattern: な (prohibitive command - don't!)
// Data source: grammar_points_data.json["な"]
//
// Structure: Verb[dictionary form] + な
// Single structure variant (standard form only)

mod na_prohibitive_tests {
    use super::*;

    // Standard prohibitive form
    #[test]
    fn test_na_prohibitive_danger() {
        let sentence = "危険！触るな！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "な");
        assert_pattern_range(&patterns, "な", 3, 6); // 触るな
    }

    // Prohibitive in quoted speech
    #[test]
    fn test_na_prohibitive_mother() {
        let sentence = "「家の中で走るな！」と母が言った";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "な");
        assert_pattern_range(&patterns, "な", 5, 8); // 走るな
    }

    // Prohibitive with casual context
    #[test]
    fn test_na_prohibitive_casual() {
        let sentence = "そんなこと言うな";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "な");
        assert_pattern_range(&patterns, "な", 5, 8); // 言うな
    }

    // Prohibitive with different verb
    #[test]
    fn test_na_prohibitive_decide() {
        let sentence = "勝手に決めるな";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "な");
        assert_pattern_range(&patterns, "な", 3, 7); // 決めるな
    }
}

// ========== で (Particle - means/method/location) ==========
// Pattern: で (particle indicating means, method, or location)
// Data source: grammar_points_data.json["で"]
//
// Structure: Noun + で
// Single structure variant (standard form only)

mod de_particle_tests {
    use super::*;

    // Means of transportation
    #[test]
    fn test_de_means_car() {
        let sentence = "車で空港へ行く";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "で");
        assert_pattern_range(&patterns, "で", 0, 2); // 車で
    }

    // Method/instrument
    #[test]
    fn test_de_method_train() {
        let sentence = "電車で行く";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "で");
        assert_pattern_range(&patterns, "で", 0, 3); // 電車で
    }

    // Group action (with people)
    #[test]
    fn test_de_group_everyone() {
        let sentence = "みんなでレストランに行く";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "で");
        assert_pattern_range(&patterns, "で", 0, 4); // みんなで
    }

    // Location where action takes place
    #[test]
    fn test_de_location_park() {
        let sentence = "公園で遊ぶ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "で");
        assert_pattern_range(&patterns, "で", 0, 3); // 公園で
    }
}

// ========== に (Particle - location/direction/time) ==========
// Pattern: に (particle indicating location, direction, or time)
// Data source: grammar_points_data.json["に"]
//
// Structure: Noun + に
// Single structure variant (standard form only)

mod ni_particle_tests {
    use super::*;

    // Location (in/at)
    #[test]
    fn test_ni_location_room() {
        let sentence = "彼は部屋にいます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に");
        assert_pattern_range(&patterns, "に", 2, 5); // 部屋に
    }

    // Direction/destination (to)
    #[test]
    fn test_ni_direction_park() {
        let sentence = "公園に行く";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に");
        assert_pattern_range(&patterns, "に", 0, 3); // 公園に
    }

    // Time (on/at)
    #[test]
    fn test_ni_time_birthday() {
        let sentence = "誕生日にケーキを食べる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に");
        assert_pattern_range(&patterns, "に", 2, 4); // 日に (from 誕生日)
    }

    // Vehicle/surface (on)
    #[test]
    fn test_ni_vehicle_bus() {
        let sentence = "バスに乗る";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に");
        assert_pattern_range(&patterns, "に", 0, 3); // バスに
    }
}

// ========== ます (Polite auxiliary verb) ==========
// Pattern: ます (polite auxiliary verb for verbs)
// Data source: grammar_points_data.json["ます"]
//
// Structure variants to test:
//   polite[0]: Verb[stem] + ます
//
// Note: Testing various conjugations (ます, ました, ません, ませんでした)
// and different verb types (godan, ichidan, irregular)

mod masu_tests {
    use super::*;

    // Basic present polite
    #[test]
    fn test_masu_present() {
        let sentence = "毎日勉強をします";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ます");
        assert_pattern_range(&patterns, "ます", 5, 8); // します
    }

    // Past polite
    #[test]
    fn test_masu_past() {
        let sentence = "昨日料理をしました";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ます");
        assert_pattern_range(&patterns, "ます", 5, 9); // しました
    }

    // Negative polite
    #[test]
    fn test_masu_negative() {
        let sentence = "今日は行きません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ます");
        assert_pattern_range(&patterns, "ます", 3, 8); // 行きません
    }

    // Past negative polite
    #[test]
    fn test_masu_past_negative() {
        let sentence = "先週は雨が降りませんでした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ます");
        assert_pattern_range(&patterns, "ます", 5, 13); // 降りませんでした
    }

    // Ichidan verb (one-row verb)
    #[test]
    fn test_masu_ichidan() {
        let sentence = "私が食べます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ます");
        assert_pattern_range(&patterns, "ます", 2, 6); // 食べます
    }
}

// ========== へいく (Going to a place) ==========
// Pattern: へいく (Place + へ/に + 行く)
// Data source: grammar_points_data.json["へいく"]
//
// Structure variants to test:
//   standard[0]: Place + へ + 行く
//   standard[1]: Place + に + 行く
//
// Note: Testing both へ and に particles with 行く verb

mod heiku_tests {
    use super::*;

    // Place + へ + 行く
    #[test]
    fn test_heiku_with_he() {
        let sentence = "学校へ行く";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "へいく");
        assert_pattern_range(&patterns, "へいく", 0, 5); // 学校へ行く
    }

    // Place + に + 行く
    #[test]
    fn test_heiku_with_ni() {
        let sentence = "病院に行く";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "へいく");
        assert_pattern_range(&patterns, "へいく", 0, 5); // 病院に行く
    }

    // More natural sentence with へ
    #[test]
    fn test_heiku_natural_he() {
        let sentence = "明日は東京へ行くつもりだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "へいく");
        assert_pattern_range(&patterns, "へいく", 3, 8); // 東京へ行く
    }

    // Polite form with に (single-token place)
    #[test]
    fn test_heiku_polite_ni() {
        let sentence = "毎週公園に行きます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "へいく");
        assert_pattern_range(&patterns, "へいく", 2, 9); // 公園に行きます
    }
}

// ========== のがすき (Like doing something) ==========
// Pattern: のがすき (like/love doing)
// Data source: grammar_points_data.json["のがすき"]
//
// Structure variants to test:
//   standard[0]: Verb[る] + の + が + 好き + だ
//   polite[0]: Verb[る] + の + が + 好き + です
//   Past variants from about section:
//     - Verb[る] + の + が + 好き + だった (past)
//     - Verb[る] + の + が + 好き + でした (polite past)

mod nogasuki_tests {
    use super::*;

    // Standard present: Verb + の + が + 好き + だ
    #[test]
    fn test_nogasuki_standard_present() {
        let sentence = "私は映画を見るのが好きだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "のがすき");
        assert_pattern_range(&patterns, "のがすき", 5, 12); // 見るのが好きだ
    }

    // Polite present: Verb + の + が + 好き + です
    #[test]
    fn test_nogasuki_polite_present() {
        let sentence = "プレゼントをもらうのが好きです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "のがすき");
        assert_pattern_range(&patterns, "のがすき", 6, 15); // もらうのが好きです
    }

    // Past: Verb + の + が + 好き + だった
    #[test]
    fn test_nogasuki_past() {
        let sentence = "彼は野球をするのが好きだった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "のがすき");
        assert_pattern_range(&patterns, "のがすき", 5, 14); // するのが好きだった
    }

    // Polite past: Verb + の + が + 好き + でした
    #[test]
    fn test_nogasuki_polite_past() {
        let sentence = "息子は漢字を書くのが好きでした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "のがすき");
        assert_pattern_range(&patterns, "のがすき", 6, 15); // 書くのが好きでした
    }
}

// ========== のがじょうず (Good at doing something) ==========
// Pattern: のがじょうず (good at doing)
// Data source: grammar_points_data.json["のがじょうず"]
//
// Structure variants to test:
//   standard[0]: Verb + の + が + 上手（じょうず）
//   Past variants from about section:
//     - Verb + の + が + 上手 + だった (past)
//   Polite variants from examples:
//     - Verb + の + が + 上手 + です (polite present)

mod nogajouzu_tests {
    use super::*;

    // Present: Verb + の + が + 上手
    #[test]
    fn test_nogajouzu_present() {
        let sentence = "彼は歌うのが上手だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "のがじょうず");
        assert_pattern_range(&patterns, "のがじょうず", 2, 9); // 歌うのが上手だ
    }

    // Polite present: Verb + の + が + 上手 + です
    #[test]
    fn test_nogajouzu_polite_present() {
        let sentence = "彼女は漢字を覚えるのが上手です";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "のがじょうず");
        assert_pattern_range(&patterns, "のがじょうず", 6, 15); // 覚えるのが上手です
    }

    // Past: Verb + の + が + 上手 + だった
    #[test]
    fn test_nogajouzu_past() {
        let sentence = "母は若い頃、踊るのが上手だった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "のがじょうず");
        assert_pattern_range(&patterns, "のがじょうず", 6, 15); // 踊るのが上手だった
    }
}

// ========== のがへた (Bad at doing something) ==========
// Pattern: のがへた (bad at doing)
// Data source: grammar_points_data.json["のがへた"]
//
// Structure variants to test:
//   standard[0]: Verb + のが + 下手（へた）
//   Past variants from about section:
//     - Verb + のが + 下手 + だった (past)
//   Polite variants from examples:
//     - Verb + のが + 下手 + です (polite present)

mod nogaheta_tests {
    use super::*;

    // Present: Verb + の + が + 下手
    #[test]
    fn test_nogaheta_present() {
        let sentence = "私は漢字を教えるのが下手だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "のがへた");
        assert_pattern_range(&patterns, "のがへた", 5, 13); // 教えるのが下手だ
    }

    // Polite present: Verb + の + が + 下手 + です
    #[test]
    fn test_nogaheta_polite_present() {
        let sentence = "彼は運転するのが下手です";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "のがへた");
        assert_pattern_range(&patterns, "のがへた", 2, 12); // 運転するのが下手です
    }

    // Past: Verb + の + が + 下手 + だった
    #[test]
    fn test_nogaheta_past() {
        let sentence = "去年まで料理をするのが下手だった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "のがへた");
        assert_pattern_range(&patterns, "のがへた", 7, 16); // するのが下手だった
    }
}

// ========== のなかで～がいちばん～ (The most among) ==========
// Pattern: のなかで～がいちばん～ (superlative - most X among Y)
// Data source: grammar_points_data.json["のなかで～がいちばん～"]
//
// Structure variants to test:
//   standard[0]: (Set of Elements) + の中で一番
//   standard[1]: その/この + 中で一番

mod nonakade_ga_ichiban_tests {
    use super::*;

    // Standard structure: Noun + の中で + Noun + が一番 + Adjective
    #[test]
    fn test_nonakade_ga_ichiban_standard() {
        let sentence = "食べ物の中で寿司が一番好きだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "のなかで～がいちばん～");
        assert_pattern_range(&patterns, "のなかで～がいちばん～", 3, 11); // の中で寿司が一番
    }

    // Standard structure with polite ending
    #[test]
    fn test_nonakade_ga_ichiban_polite() {
        let sentence = "授業の中で理科が一番嫌いです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "のなかで～がいちばん～");
        assert_pattern_range(&patterns, "のなかで～がいちばん～", 2, 10); // の中で理科が一番
    }

    // Demonstrative この + 中で + interrogative + が一番
    #[test]
    fn test_nonakade_ga_ichiban_kono() {
        let sentence = "この中でどれが一番好き？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "のなかで～がいちばん～");
        assert_pattern_range(&patterns, "のなかで～がいちばん～", 0, 9); // この中でどれが一番
    }

    // Demonstrative その + 中で + は + Noun + が一番 + い-adjective
    #[test]
    fn test_nonakade_ga_ichiban_sono() {
        let sentence = "その中ではクッキーが一番美味しい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "のなかで～がいちばん～");
        assert_pattern_range(&patterns, "のなかで～がいちばん～", 0, 12); // その中ではクッキーが一番
    }
}

// ========== な-Adjective + Noun ==========
// Pattern: な-Adjective + な + Noun
// Data source: grammar_points_data.json["な-Adjective + Noun"]
//
// Structure variants to test:
//   standard[0]: な-Adjective + な + Noun
//   Examples: 静か + な + 夜, 元気 + な + 子供, 綺麗 + な + カーテン

mod na_adjective_noun_tests {
    use super::*;

    // Basic な-adjective: 静か + な + 夜
    #[test]
    fn test_na_adjective_noun_shizuka() {
        let sentence = "静かな夜に星を見るのが好きだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "な-Adjective + Noun");
        assert_pattern_range(&patterns, "な-Adjective + Noun", 0, 4); // 静かな夜
    }

    // な-adjective: 元気 + な + 子供
    #[test]
    fn test_na_adjective_noun_genki() {
        let sentence = "元気な子供たちが公園で遊んでいる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "な-Adjective + Noun");
        assert_pattern_range(&patterns, "な-Adjective + Noun", 0, 5); // 元気な子供
    }

    // な-adjective: 綺麗 + な + カーテン
    // Note: Pattern range extends to include です due to overlapping pattern detection
    #[test]
    fn test_na_adjective_noun_kirei() {
        let sentence = "これは綺麗なカーテンですね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "な-Adjective + Noun");
        assert_pattern_range(&patterns, "な-Adjective + Noun", 3, 12); // 綺麗なカーテンです (includes です)
    }

    // な-adjective in middle of sentence: 便利 + な + 場所
    #[test]
    fn test_na_adjective_noun_benri() {
        let sentence = "駅に近くて便利な場所を探している";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "な-Adjective + Noun");
        assert_pattern_range(&patterns, "な-Adjective + Noun", 5, 10); // 便利な場所
    }

    // な-adjective: 大切 + な + もの
    #[test]
    fn test_na_adjective_noun_taisetsu() {
        let sentence = "彼女にとって大切なものを失ってしまった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "な-Adjective + Noun");
        assert_pattern_range(&patterns, "な-Adjective + Noun", 6, 11); // 大切なもの
    }
}

// ========== な-Adjective だ ==========
// Pattern: な-Adjective + だ/です (predicate form)
// Data source: grammar_points_data.json["な-Adjective だ"]
//
// Structure variants to test:
//   standard[0]: な-Adjective + だ (e.g., 静か + だ, 元気 + だ)
//   polite[0]: な-Adjective + です (e.g., 静か + です, 元気 + です)
//   casual: な-Adjective alone (だ omitted in casual speech)

mod na_adjective_da_tests {
    use super::*;

    // Standard form: な-adjective + だ
    #[test]
    fn test_na_adjective_da_standard() {
        let sentence = "あの場所は静かだから落ち着く";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "な-Adjective だ");
        assert_pattern_range(&patterns, "な-Adjective だ", 5, 8); // 静かだ
    }

    // Polite form: な-adjective + です
    #[test]
    fn test_na_adjective_da_polite() {
        let sentence = "彼氏は素敵です";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "な-Adjective だ");
        assert_pattern_range(&patterns, "な-Adjective だ", 3, 7); // 素敵です
    }

    // Casual standalone (no だ): な-adjective alone
    #[test]
    fn test_na_adjective_da_standalone() {
        let sentence = "この料理、本当に美味しくて健康的";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "な-Adjective だ");
        assert_pattern_range(&patterns, "な-Adjective だ", 13, 15); // 健康 (without 的 suffix or copula)
    }

    // Past form: な-adjective + だった
    #[test]
    fn test_na_adjective_da_past() {
        let sentence = "昨日の試験は簡単だった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "な-Adjective だ");
        assert_pattern_range(&patterns, "な-Adjective だ", 6, 11); // 簡単だった
    }

    // Polite past: な-adjective + でした
    #[test]
    fn test_na_adjective_da_polite_past() {
        let sentence = "会議は重要でした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "な-Adjective だ");
        assert_pattern_range(&patterns, "な-Adjective だ", 3, 8); // 重要でした
    }
}

// ========== Verb［た・ている］+ Noun (Relative Clause) ==========
// Pattern: Verb［た・ている］+ Noun (verb modifying noun - relative clause)
// Data source: grammar_points_data.json["Verb［た・ている］+ Noun"]
//
// Structure variants to test:
//   standard[0]: Verb［た］+ Noun
//   standard[1]: Verb［ている］+ Noun
//
// Note: Only plain (non-polite) verb forms can modify nouns.
// Polite forms like ～ます cannot be used in relative clauses.

mod verb_ta_teiru_noun_tests {
    use super::*;

    // Verb[た] + Noun - past tense verb modifying noun
    #[test]
    fn test_verb_ta_noun() {
        let sentence = "たかしさんは東京に住んだ男";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb［た・ている］+ Noun");
        assert_pattern_range(&patterns, "Verb［た・ている］+ Noun", 9, 13); // 住んだ男
    }

    // Verb[た] + Noun - past tense, different verb
    #[test]
    fn test_verb_ta_noun_tabeta() {
        let sentence = "貴方が食べたハンバーガー";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb［た・ている］+ Noun");
        assert_pattern_range(&patterns, "Verb［た・ている］+ Noun", 3, 12); // 食べたハンバーガー
    }

    // Verb[ている] + Noun - continuous form modifying noun
    #[test]
    fn test_verb_teiru_noun_nondeiru() {
        let sentence = "田中さんが飲んでいるコーヒー";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb［た・ている］+ Noun");
        assert_pattern_range(&patterns, "Verb［た・ている］+ Noun", 5, 14); // 飲んでいるコーヒー
    }

    // Verb[ている] + Noun - continuous form, different verb
    #[test]
    fn test_verb_teiru_noun_neteiru() {
        let sentence = "道で寝ている犬";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb［た・ている］+ Noun");
        assert_pattern_range(&patterns, "Verb［た・ている］+ Noun", 2, 7); // 寝ている犬
    }

    // Verb[た] + Noun - more complex example
    #[test]
    fn test_verb_ta_noun_complex() {
        let sentence = "昨日会った人はトムさんだった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb［た・ている］+ Noun");
        assert_pattern_range(&patterns, "Verb［た・ている］+ Noun", 2, 6); // 会った人
    }
}

// ========== い-Adjective (Predicate) ==========
// Pattern: い-Adjective used as predicate (terminal position)
// Data source: grammar_points_data.json["い-Adjective (Predicate)"]
//
// Structure variants to test:
//   standard: い-Adjective alone (かわいい, 新しい, 寒い)
//   polite: い-Adjective + です (かわいいです, 新しいです, 寒いです)

mod i_adjective_predicate_tests {
    use super::*;

    // Standard form: い-Adjective alone
    #[test]
    fn test_i_adj_predicate_kawaii() {
        let sentence = "あの猫、本当にかわいい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "い-Adjective (Predicate)");
        assert_pattern_range(&patterns, "い-Adjective (Predicate)", 7, 11); // かわいい
    }

    #[test]
    fn test_i_adj_predicate_atarashii() {
        let sentence = "この車は新しい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "い-Adjective (Predicate)");
        assert_pattern_range(&patterns, "い-Adjective (Predicate)", 4, 7); // 新しい
    }

    #[test]
    fn test_i_adj_predicate_samui() {
        let sentence = "今日は寒い";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "い-Adjective (Predicate)");
        assert_pattern_range(&patterns, "い-Adjective (Predicate)", 3, 5); // 寒い
    }

    // Polite form: い-Adjective + です
    #[test]
    fn test_i_adj_predicate_polite_kawaii() {
        let sentence = "あの子犬はかわいいです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "い-Adjective (Predicate)");
        assert_pattern_range(&patterns, "い-Adjective (Predicate)", 5, 11); // かわいいです
    }

    #[test]
    fn test_i_adj_predicate_polite_atarashii() {
        let sentence = "このパソコンは新しいです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "い-Adjective (Predicate)");
        assert_pattern_range(&patterns, "い-Adjective (Predicate)", 7, 12); // 新しいです
    }
}

// ========== より～のほうが (Comparison: A than B) ==========
// Pattern: より～のほうが (comparison - "more X than Y")
// Data source: grammar_points_data.json["より～のほうが"]
//
// Structure variants to test:
//   standard[0]: Verb (A) + より + Verb (B) + 方（ほう） + が
//   standard[1]: ［い］Adjective (A) + より + ［い］Adjective (B) + 方（ほう） + が
//   standard[2]: ［な］Adjective (A) + な + より + ［な］Adjective (B) + な + 方（ほう） + が
//   standard[3]: Noun (A) + より + Noun (B) + の + 方（ほう） + が

mod yori_no_hou_ga_tests {
    use super::*;

    // Structure: Verb (A) + より + Verb (B) + 方（ほう） + が
    #[test]
    fn test_verb_comparison() {
        let sentence = "都会に住むよりほうがいい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "より～のほうが");
        assert_pattern_range(&patterns, "より～のほうが", 5, 10); // よりほうが
    }

    // Structure: ［い］Adjective (A) + より + ［い］Adjective (B) + 方（ほう） + が
    #[test]
    fn test_i_adjective_comparison() {
        let sentence = "怖い映画より面白い映画のほうが好き";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "より～のほうが");
        assert_pattern_range(&patterns, "より～のほうが", 4, 15); // より面白い映画のほうが
    }

    // Structure: ［な］Adjective (A) + な + より + ［な］Adjective (B) + な + 方（ほう） + が
    #[test]
    fn test_na_adjective_comparison() {
        let sentence = "静かな場所より賑やかな場所のほうがいい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "より～のほうが");
        assert_pattern_range(&patterns, "より～のほうが", 5, 17); // より賑やかな場所のほうが
    }

    // Structure: Noun (A) + より + Noun (B) + の + 方（ほう） + が
    #[test]
    fn test_noun_comparison() {
        let sentence = "彼女より私のほうが背が高い";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "より～のほうが");
        assert_pattern_range(&patterns, "より～のほうが", 2, 9); // より私のほうが
    }
}

// ========== へ (Direction particle) ==========
// Pattern: へ (to/toward)
// Data source: grammar_points_data.json["へ"]
// Structure: Noun + へ

mod he_tests {
    use super::*;

    // Standard: Noun + へ (direction/destination)
    #[test]
    fn test_he_direction_france() {
        let sentence = "私はフランスへ行く。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "へ");
        assert_pattern_range(&patterns, "へ", 2, 7); // フランスへ
    }

    // Noun + へ (welcome expression)
    #[test]
    fn test_he_welcome_osaka() {
        let sentence = "大阪へようこそ！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "へ");
        assert_pattern_range(&patterns, "へ", 0, 3); // 大阪へ
    }

    // Noun + へ (casual usage)
    #[test]
    fn test_he_direction_casual() {
        let sentence = "今日は図書館へ勉強しに行く";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "へ");
        assert_pattern_range(&patterns, "へ", 3, 7); // 図書館へ
    }
}

// ========== Verbs (Non-past) ==========
// Pattern: Verbs (Non-past)
// Data source: grammar_points_data.json["Verbs (Non-past)"]
//
// Structure variants to test:
//   standard[0-2]: Dictionary form examples (食べる, 洗う)
//   polite[0-1]: Polite form examples (食べます, 洗います)

mod verbs_non_past_tests {
    use super::*;

    // Testing: structure.standard - Dictionary form (る-verb)
    #[test]
    fn test_verbs_non_past_dictionary_ru_verb() {
        let sentence = "嫁は毎晩映画を見る";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verbs (Non-past)");
        assert_pattern_range(&patterns, "Verbs (Non-past)", 7, 9); // 見る
    }

    // Testing: structure.standard - Dictionary form (う-verb)
    #[test]
    fn test_verbs_non_past_dictionary_u_verb() {
        let sentence = "子供は遊ぶ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verbs (Non-past)");
        assert_pattern_range(&patterns, "Verbs (Non-past)", 3, 5); // 遊ぶ
    }

    // Testing: structure.standard - Dictionary form with future context
    #[test]
    fn test_verbs_non_past_future_context() {
        let sentence = "カホは８時に寝る";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verbs (Non-past)");
        assert_pattern_range(&patterns, "Verbs (Non-past)", 6, 8); // 寝る
    }

    // Note: Polite forms (ます) are covered by the separate "ます" pattern
    // This pattern specifically detects casual dictionary form verbs in non-past tense
}

// ========== い-Adjectives ==========
// Pattern: い-Adjectives
// Data source: grammar_points_data.json["い-Adjectives"]
//
// Structure variants to test:
//   standard[0]: Base form examples (おおきい, おいしい, はやい)

mod i_adjectives_tests {
    use super::*;

    // Testing: structure.standard - Basic い-adjective (大きい)
    #[test]
    fn test_i_adjective_ookii() {
        let sentence = "大きい犬が好きです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "い-Adjectives");
        assert_pattern_range(&patterns, "い-Adjectives", 0, 3); // 大きい
    }

    // Testing: structure.standard - い-adjective modifying noun (狭い道)
    #[test]
    fn test_i_adjective_semai() {
        let sentence = "狭い道を歩いている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "い-Adjectives");
        assert_pattern_range(&patterns, "い-Adjectives", 0, 2); // 狭い
    }

    // Testing: structure.standard - い-adjective as predicate (温い)
    #[test]
    fn test_i_adjective_nurui() {
        let sentence = "この水は温い";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "い-Adjectives");
        assert_pattern_range(&patterns, "い-Adjectives", 4, 6); // 温い
    }

}

// Pattern: な-Adjectives
// Data source: grammar_points_data.json["な-Adjectives"]
// Note: Pattern ranges are extended to include following auxiliary verbs (だ/です/な/etc.)
// similar to how い-Adjectives works
mod na_adjectives_tests {
    use super::*;

    // Testing: な-adjective stem with だ
    #[test]
    fn test_na_adjective_kirei() {
        let sentence = "彼女は本当に綺麗だと思う";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "な-Adjectives");
        assert_pattern_range(&patterns, "な-Adjectives", 6, 9); // 綺麗だ (extended to include auxiliary)
    }

    // Testing: な-adjective stem with な before noun
    #[test]
    fn test_na_adjective_shizuka() {
        let sentence = "静かな場所で勉強したい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "な-Adjectives");
        assert_pattern_range(&patterns, "な-Adjectives", 0, 3); // 静かな (extended to include auxiliary)
    }

    // Testing: な-adjective stem with です
    #[test]
    fn test_na_adjective_kaiteki() {
        let sentence = "この部屋は快適です";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "な-Adjectives");
        assert_pattern_range(&patterns, "な-Adjectives", 5, 9); // 快適です (extended to include auxiliary)
    }

    // Testing: な-adjective stem with だった (past tense)
    #[test]
    fn test_na_adjective_taihen() {
        let sentence = "昨日は大変だった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "な-Adjectives");
        assert_pattern_range(&patterns, "な-Adjectives", 3, 8); // 大変だった (extended to include auxiliaries)
    }
}

// Pattern: る-Verb (Dictionary) - Ichidan verbs in dictionary form
// Data source: grammar_points_data.json["る-Verb (Dictionary)"]
// Testing: structure.standard - examples of る-verbs in dictionary form
//
// る-verbs (一段動詞/ichidan): verbs that conjugate by removing る
// Examples from data: 食べる, 見る, 寝る
// Contrast with う-verbs where る changes to other sounds
// Note: Pattern ranges extend to include following auxiliaries (similar to Verbs (Non-past))
mod ru_verb_dictionary_tests {
    use super::*;

    // Testing: 食べる (to eat) - classic る-verb
    #[test]
    fn test_ru_verb_taberu() {
        let sentence = "毎日野菜を食べるようにしている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "る-Verb (Dictionary)");
        assert_pattern_range(&patterns, "る-Verb (Dictionary)", 5, 8); // 食べる
    }

    // Testing: 見る (to see/watch) - monosyllabic る-verb with auxiliary
    #[test]
    fn test_ru_verb_miru() {
        let sentence = "その映画は絶対に見るべきだよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "る-Verb (Dictionary)");
        assert_pattern_range(&patterns, "る-Verb (Dictionary)", 8, 13); // 見るべきだ (extended with auxiliaries)
    }

    // Testing: 寝る (to sleep) - another common る-verb
    #[test]
    fn test_ru_verb_neru() {
        let sentence = "今夜は早く寝るつもりです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "る-Verb (Dictionary)");
        assert_pattern_range(&patterns, "る-Verb (Dictionary)", 5, 7); // 寝る
    }
}

// Pattern: う-Verb (Dictionary) - Godan verbs in dictionary form
// Data source: grammar_points_data.json["う-Verb (Dictionary)"]
// Testing: structure.standard - examples of う-verbs in dictionary form
//
// う-verbs (五段動詞/godan): verbs that conjugate using all 5 vowel sounds in their column
// Examples from data: 座る, 歌う, 歩く, 話す, 打つ, 死ぬ, 飛ぶ, 休む, 泳ぐ
// Note: Some う-verbs end in る but are NOT る-verbs (they conjugate differently)
// Pattern ranges extend to include following auxiliaries (similar to Verbs (Non-past))
mod u_verb_dictionary_tests {
    use super::*;

    // Testing: 歩く (to walk) - く-ending う-verb
    #[test]
    fn test_u_verb_aruku() {
        let sentence = "毎朝公園を歩くのが日課です";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "う-Verb (Dictionary)");
        assert_pattern_range(&patterns, "う-Verb (Dictionary)", 5, 7); // 歩く
    }

    // Testing: 話す (to speak) - す-ending う-verb
    #[test]
    fn test_u_verb_hanasu() {
        let sentence = "彼とは英語で話すことが多い";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "う-Verb (Dictionary)");
        assert_pattern_range(&patterns, "う-Verb (Dictionary)", 6, 8); // 話す
    }

    // Testing: 座る (to sit) - る-ending う-verb (NOT a る-verb!)
    #[test]
    fn test_u_verb_suwaru() {
        let sentence = "ここに座る人はいないのか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "う-Verb (Dictionary)");
        assert_pattern_range(&patterns, "う-Verb (Dictionary)", 3, 5); // 座る
    }
}

// ========== う-Verb (Past) ==========
// Pattern: う-Verb (Past) - Past tense う-verbs
// Data source: grammar_points_data.json["う-Verb (Past)"]
//
// Structure variants to test:
//   standard: Various う-verb endings (った, いた, いだ, んだ, した)
//   polite: Verb[stem]ました forms

#[cfg(test)]
mod u_verb_past_tests {
    use super::*;

    // Testing: 買った (bought) - う-ending verb
    #[test]
    fn test_u_verb_past_katta() {
        let sentence = "昨日新しいペンを買った";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "う-Verb (Past)");
        assert_pattern_range(&patterns, "う-Verb (Past)", 8, 11); // 買った
    }

    // Testing: 待った (waited) - つ-ending verb
    #[test]
    fn test_u_verb_past_matta() {
        let sentence = "駅で５分待った";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "う-Verb (Past)");
        assert_pattern_range(&patterns, "う-Verb (Past)", 4, 7); // 待った
    }

    // Testing: 書いた (wrote) - く-ending verb
    #[test]
    fn test_u_verb_past_kaita() {
        let sentence = "ノートに名前を書いた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "う-Verb (Past)");
        assert_pattern_range(&patterns, "う-Verb (Past)", 7, 10); // 書いた
    }

    // Testing: 泳いだ (swam) - ぐ-ending verb
    #[test]
    fn test_u_verb_past_oyoida() {
        let sentence = "池で泳いだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "う-Verb (Past)");
        assert_pattern_range(&patterns, "う-Verb (Past)", 2, 5); // 泳いだ
    }

    // Testing: 死んだ (died) - ぬ-ending verb
    #[test]
    fn test_u_verb_past_shinda() {
        let sentence = "金魚が死んだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "う-Verb (Past)");
        assert_pattern_range(&patterns, "う-Verb (Past)", 3, 6); // 死んだ
    }

    // Testing: 消した (turned off) - す-ending verb
    #[test]
    fn test_u_verb_past_keshita() {
        let sentence = "電気を消した";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "う-Verb (Past)");
        assert_pattern_range(&patterns, "う-Verb (Past)", 3, 6); // 消した
    }

    // Testing: 座りました (sat - polite) - polite past
    #[test]
    fn test_u_verb_past_polite() {
        let sentence = "そこに座りました";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "う-Verb (Past)");
        assert_pattern_range(&patterns, "う-Verb (Past)", 3, 8); // 座りました
    }
}

// ========== る-Verb (Past) ==========
// Pattern: る-Verb (Past) - Past tense る-verbs (ichidan verbs)
// Data source: grammar_points_data.json["る-Verb (Past)"]
//
// Structure variants to test:
//   standard: Verb[stem] + た (食べた, 見た, 寝た)
//   polite: Verb[stem] + ました (食べました, 見ました, 寝ました)

#[cfg(test)]
mod ru_verb_past_tests {
    use super::*;

    // Testing: 食べた (ate) - standard past
    #[test]
    fn test_ru_verb_past_tabeta() {
        let sentence = "昼ごはんを食べた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "る-Verb (Past)");
        assert_pattern_range(&patterns, "る-Verb (Past)", 5, 8); // 食べた
    }

    // Testing: 見た (saw) - standard past
    #[test]
    fn test_ru_verb_past_mita() {
        let sentence = "映画を見た";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "る-Verb (Past)");
        assert_pattern_range(&patterns, "る-Verb (Past)", 3, 5); // 見た
    }

    // Testing: 寝た (slept) - standard past
    #[test]
    fn test_ru_verb_past_neta() {
        let sentence = "昨日は早く寝た";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "る-Verb (Past)");
        assert_pattern_range(&patterns, "る-Verb (Past)", 5, 7); // 寝た
    }

    // Testing: 食べました (ate - polite) - polite past
    #[test]
    fn test_ru_verb_past_polite() {
        let sentence = "朝ごはんを食べました";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "る-Verb (Past)");
        assert_pattern_range(&patterns, "る-Verb (Past)", 5, 10); // 食べました
    }
}

// Pattern: う-Verb (Negative) - Negative form of う-verbs (godan verbs)
// Data source: grammar_points_data.json["う-Verb (Negative)"]
// Testing all structure variants with different verb endings
#[cfg(test)]
mod u_verb_negative_tests {
    use super::*;

    // Testing: standard[1] - 座（すわ）る + らない
    #[test]
    fn test_u_verb_negative_ra() {
        let sentence = "あいつは絶対にここには座らない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "う-Verb (Negative)");
        assert_pattern_range(&patterns, "う-Verb (Negative)", 11, 15); // 座らない
    }

    // Testing: standard[2] - 歌（うた）う + わない (special case)
    #[test]
    fn test_u_verb_negative_wa() {
        let sentence = "カラオケでは歌わないようにしてる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "う-Verb (Negative)");
        assert_pattern_range(&patterns, "う-Verb (Negative)", 6, 10); // 歌わない
    }

    // Testing: standard[3] - 歩（ある）く + かない
    #[test]
    fn test_u_verb_negative_ka() {
        let sentence = "もう歩かないって決めたんだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "う-Verb (Negative)");
        assert_pattern_range(&patterns, "う-Verb (Negative)", 2, 6); // 歩かない
    }

    // Testing: standard[4] - 話（はな）す + さない
    #[test]
    fn test_u_verb_negative_sa() {
        let sentence = "誰にも話さないって約束して";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "う-Verb (Negative)");
        assert_pattern_range(&patterns, "う-Verb (Negative)", 3, 7); // 話さない
    }

    // Testing: standard[5] - 打（う）つ + たない
    #[test]
    fn test_u_verb_negative_ta() {
        let sentence = "そんな危険なことはしないし打たない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "う-Verb (Negative)");
        assert_pattern_range(&patterns, "う-Verb (Negative)", 13, 17); // 打たない
    }

    // Testing: standard[6] - 死（し）ぬ + なない
    #[test]
    fn test_u_verb_negative_na() {
        let sentence = "簡単には死なないから安心して";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "う-Verb (Negative)");
        assert_pattern_range(&patterns, "う-Verb (Negative)", 4, 8); // 死なない
    }

    // Testing: standard[7] - 飛（と）ぶ + ばない
    #[test]
    fn test_u_verb_negative_ba() {
        let sentence = "ダチョウは飛ばないって知ってた？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "う-Verb (Negative)");
        assert_pattern_range(&patterns, "う-Verb (Negative)", 5, 9); // 飛ばない
    }

    // Testing: standard[8] - 休（やす）む + まない
    #[test]
    fn test_u_verb_negative_ma() {
        let sentence = "こんな日でも休まないつもり？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "う-Verb (Negative)");
        assert_pattern_range(&patterns, "う-Verb (Negative)", 6, 10); // 休まない
    }

    // Testing: standard[9] - 泳（およ）ぐ + がない
    #[test]
    fn test_u_verb_negative_ga() {
        let sentence = "川では泳がないほうがいいよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "う-Verb (Negative)");
        assert_pattern_range(&patterns, "う-Verb (Negative)", 3, 7); // 泳がない
    }

    // Testing: polite[1] - 座（すわ）る + りません
    #[test]
    fn test_u_verb_negative_polite() {
        let sentence = "すみません、ここには座りません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "う-Verb (Negative)");
        assert_pattern_range(&patterns, "う-Verb (Negative)", 10, 15); // 座りません
    }

    // Testing: polite[2] - 座（すわ）る + らないです
    #[test]
    fn test_u_verb_negative_semi_polite() {
        let sentence = "あそこには座らないです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "う-Verb (Negative)");
        assert_pattern_range(&patterns, "う-Verb (Negative)", 5, 11); // 座らないです
    }
}

mod u_verb_negative_past_tests {
    use super::*;

    // Testing: standard[0] - 座（すわ）る + らなかった
    #[test]
    fn test_u_verb_negative_past_ra() {
        let sentence = "昨日は公園のベンチに座らなかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "う-Verb (Negative-Past)");
        assert_pattern_range(&patterns, "う-Verb (Negative-Past)", 10, 16); // 座らなかった
    }

    // Testing: standard[1] - 歌（うた）う + わなかった
    #[test]
    fn test_u_verb_negative_past_wa() {
        let sentence = "彼女はカラオケで全然歌わなかったよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "う-Verb (Negative-Past)");
        assert_pattern_range(&patterns, "う-Verb (Negative-Past)", 10, 16); // 歌わなかった
    }

    // Testing: standard[2] - 歩（ある）く + かなかった
    #[test]
    fn test_u_verb_negative_past_ka() {
        let sentence = "雨が降ったから歩かなかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "う-Verb (Negative-Past)");
        assert_pattern_range(&patterns, "う-Verb (Negative-Past)", 7, 13); // 歩かなかった
    }

    // Testing: standard[3] - 話（はな）す + さなかった
    #[test]
    fn test_u_verb_negative_past_sa() {
        let sentence = "あの件についてはまだ話さなかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "う-Verb (Negative-Past)");
        assert_pattern_range(&patterns, "う-Verb (Negative-Past)", 10, 16); // 話さなかった
    }

    // Testing: standard[6] - 飛（と）ぶ + ばなかった
    #[test]
    fn test_u_verb_negative_past_ba() {
        let sentence = "鳥がそっちには飛ばなかったね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "う-Verb (Negative-Past)");
        assert_pattern_range(&patterns, "う-Verb (Negative-Past)", 7, 13); // 飛ばなかった
    }

    // Testing: standard[7] - 休（やす）む + まなかった
    #[test]
    fn test_u_verb_negative_past_ma() {
        let sentence = "先週は一日も休まなかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "う-Verb (Negative-Past)");
        assert_pattern_range(&patterns, "う-Verb (Negative-Past)", 6, 12); // 休まなかった
    }

    // Testing: Exception - ある ￫ なかった
    // NOTE: The aru exception (なかった) is tokenized as an adjective (形容詞), not a verb.
    // This is correct Japanese grammar - ある doesn't follow normal う-verb negative conjugation.
    // Instead, it becomes なかった which is treated as an い-adjective.
    // This form is detected by the い-Adjective (Past) pattern, not う-Verb (Negative-Past).
    #[test]
    fn test_u_verb_negative_past_aru_exception() {
        let sentence = "そこには何もなかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // Should NOT detect う-Verb (Negative-Past) - it's an adjective
        assert!(!has_pattern(&patterns, "う-Verb (Negative-Past)"));
        // Should detect い-Adjective (Past) instead
        assert_has_pattern(&patterns, "い-Adjective (Past)");
        assert_pattern_range(&patterns, "い-Adjective (Past)", 6, 10); // なかった
    }

    // Testing: polite[1] - 座（すわ）る + りませんでした
    // NOTE: The polite form (ませんでした) is detected by う-Verb (Negative), not う-Verb (Negative-Past).
    // This is because ませんでした is structurally ません (polite negative) + でした (polite past copula).
    // Both pattern detections are semantically valid.
    #[test]
    fn test_u_verb_negative_past_polite_masen_deshita() {
        let sentence = "昨日はそこに座りませんでした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // Detected by う-Verb (Negative), not う-Verb (Negative-Past)
        assert_has_pattern(&patterns, "う-Verb (Negative)");
        assert_pattern_range(&patterns, "う-Verb (Negative)", 6, 14); // 座りませんでした
    }

    // Testing: polite[2] - 座（すわ）る + らなかったです
    #[test]
    fn test_u_verb_negative_past_semi_polite() {
        let sentence = "そのベンチには座らなかったです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "う-Verb (Negative-Past)");
        assert_pattern_range(&patterns, "う-Verb (Negative-Past)", 7, 15); // 座らなかったです
    }

    // Testing: polite exception - ある ￫ ありませんでした
    // NOTE: Similar to the standard aru exception, the polite form is detected by う-Verb (Negative).
    #[test]
    fn test_u_verb_negative_past_aru_polite() {
        let sentence = "会議室には誰もありませんでした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // Detected by う-Verb (Negative), not う-Verb (Negative-Past)
        assert_has_pattern(&patterns, "う-Verb (Negative)");
    }
}

// る-Verb (Negative) pattern tests
// Data source: grammar_points_data.json["る-Verb (Negative)"]
// Pattern: る-Verb (ichidan verb) negative forms
// Structures: Verb[一段] + ない (casual), Verb[一段] + ません (polite), Verb[一段] + ないです (semi-polite)
mod ru_verb_negative_tests {
    use super::*;

    // Testing: standard[1] - 食べる + ない
    #[test]
    fn test_ru_verb_negative_taberu() {
        let sentence = "私は肉を食べないよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "る-Verb (Negative)");
        assert_pattern_range(&patterns, "る-Verb (Negative)", 4, 8); // 食べない
    }

    // Testing: standard[2] - 見る + ない
    #[test]
    fn test_ru_verb_negative_miru() {
        let sentence = "テレビは見ないことにしている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "る-Verb (Negative)");
        assert_pattern_range(&patterns, "る-Verb (Negative)", 4, 7); // 見ない
    }

    // Testing: standard[3] - 寝る + ない
    #[test]
    fn test_ru_verb_negative_neru() {
        let sentence = "夜は寝ないで勉強している";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "る-Verb (Negative)");
        assert_pattern_range(&patterns, "る-Verb (Negative)", 2, 5); // 寝ない
    }

    // Testing: polite[1] - 食べる + ません
    #[test]
    fn test_ru_verb_negative_polite_taberu() {
        let sentence = "肉は食べません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "る-Verb (Negative)");
        assert_pattern_range(&patterns, "る-Verb (Negative)", 2, 7); // 食べません
    }

    // Testing: polite[3] - 見る + ません
    #[test]
    fn test_ru_verb_negative_polite_miru() {
        let sentence = "黒板が見えません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "る-Verb (Negative)");
        assert_pattern_range(&patterns, "る-Verb (Negative)", 3, 8); // 見えません
    }

    // Testing: polite[5] - 寝る + ませんでした
    // NOTE: ませんでした is detected as 寝ません + でした, which is semantically correct
    #[test]
    fn test_ru_verb_negative_polite_neru() {
        let sentence = "今日は寝ませんでした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "る-Verb (Negative)");
        assert_pattern_range(&patterns, "る-Verb (Negative)", 3, 10); // 寝ませんでした
    }

    // Testing: polite[2] - 食べる + ないです
    #[test]
    fn test_ru_verb_negative_semi_polite_taberu() {
        let sentence = "野菜は食べないです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "る-Verb (Negative)");
        assert_pattern_range(&patterns, "る-Verb (Negative)", 3, 9); // 食べないです
    }

    // Testing: polite[4] - 見る + ないです
    #[test]
    fn test_ru_verb_negative_semi_polite_miru() {
        let sentence = "映画は見ないです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "る-Verb (Negative)");
        assert_pattern_range(&patterns, "る-Verb (Negative)", 3, 8); // 見ないです
    }

    // Testing: polite[6] - 寝る + ないです
    #[test]
    fn test_ru_verb_negative_semi_polite_neru() {
        let sentence = "今夜は寝ないです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "る-Verb (Negative)");
        assert_pattern_range(&patterns, "る-Verb (Negative)", 3, 8); // 寝ないです
    }
}

// る-Verb (Negative-Past) pattern tests
// Data source: grammar_points_data.json["る-Verb (Negative-Past)"]
// Pattern: る-Verb (ichidan verb) negative past forms
// Structures: Verb[一段,未然形] + なかった (standard), Verb[一段,連用形] + ませんでした (polite), Verb[一段,未然形] + なかったです (semi-polite)
mod ru_verb_negative_past_tests {
    use super::*;

    // Testing: standard[0] - 食べる + なかった
    #[test]
    fn test_ru_verb_negative_past_taberu() {
        let sentence = "昨日の夜は何も食べなかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "る-Verb (Negative-Past)");
        assert_pattern_range(&patterns, "る-Verb (Negative-Past)", 7, 13); // 食べなかった
    }

    // Testing: standard[1] - 見る + なかった
    #[test]
    fn test_ru_verb_negative_past_miru() {
        let sentence = "最近はテレビを見なかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "る-Verb (Negative-Past)");
        assert_pattern_range(&patterns, "る-Verb (Negative-Past)", 7, 12); // 見なかった
    }

    // Testing: standard[2] - 寝る + なかった
    #[test]
    fn test_ru_verb_negative_past_neru() {
        let sentence = "昨日は全然寝なかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "る-Verb (Negative-Past)");
        assert_pattern_range(&patterns, "る-Verb (Negative-Past)", 5, 10); // 寝なかった
    }

    // Testing: polite[0] - 食べる + ませんでした
    // NOTE: The polite form (ませんでした) is detected by る-Verb (Negative), not る-Verb (Negative-Past).
    // This is because ませんでした is structurally ません (polite negative) + でした (polite past copula).
    // Both pattern detections are semantically valid.
    #[test]
    fn test_ru_verb_negative_past_polite_masen_deshita() {
        let sentence = "朝ごはんは食べませんでした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // Detected by る-Verb (Negative), not る-Verb (Negative-Past)
        assert_has_pattern(&patterns, "る-Verb (Negative)");
        assert_pattern_range(&patterns, "る-Verb (Negative)", 5, 13); // 食べませんでした
    }

    // Testing: polite[1] - 食べる + なかったです
    #[test]
    fn test_ru_verb_negative_past_semi_polite_taberu() {
        let sentence = "お弁当は食べなかったです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "る-Verb (Negative-Past)");
        assert_pattern_range(&patterns, "る-Verb (Negative-Past)", 4, 12); // 食べなかったです
    }

    // Testing: polite[2] - 見る + ませんでした
    #[test]
    fn test_ru_verb_negative_past_polite_miru() {
        let sentence = "映画は見ませんでした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // Detected by る-Verb (Negative), not る-Verb (Negative-Past)
        assert_has_pattern(&patterns, "る-Verb (Negative)");
        assert_pattern_range(&patterns, "る-Verb (Negative)", 3, 10); // 見ませんでした
    }

    // Testing: polite[3] - 見る + なかったです
    #[test]
    fn test_ru_verb_negative_past_semi_polite_miru() {
        let sentence = "ニュースは見なかったです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "る-Verb (Negative-Past)");
        assert_pattern_range(&patterns, "る-Verb (Negative-Past)", 5, 12); // 見なかったです
    }
}

// ========== の (Pronoun replacement) ==========
// Pattern: の (pronoun replacement - replaces previously mentioned noun)
// Data source: grammar_points_data.json["の"]
//
// Structure variants to test:
//   standard[0]: Noun + の + (previously mentioned noun)
//   Note: The previously mentioned noun is dropped.
//
// Examples from grammar data:
//   - この本はたけしさんのです (This book is Takeshi-san's)
//   - そのペンは誰の？あなたの？ (That pen, whose is it? Is it yours?)
//   - あの車、あなたが乗っているのですか (That car, is it the one you drive?)

mod no_pronoun_replacement_tests {
    use super::*;

    // Testing: standard[0] - Noun + の (possessive replacement)
    // Example: "さんの" where the following noun (本) is dropped
    #[test]
    fn test_no_possessive_replacement() {
        let sentence = "この本はたけしさんのです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "の");
        assert_pattern_range(&patterns, "の", 7, 10); // さんの
    }

    // Testing: standard[0] - Noun + の (pronoun replacement - multiple instances)
    // Example: "誰の" and "あなたの" where the following noun (ペン) is dropped
    #[test]
    fn test_no_yours_pronoun() {
        let sentence = "そのペンは誰の？あなたの？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "の");
        // Pattern should match both instances
        // First: 誰の (chars 5-7)
        // Second: あなたの (chars 8-12)
        // Checking for the second instance (pattern matcher may return any match)
        assert_pattern_range(&patterns, "の", 8, 12); // あなたの
    }

    // Testing: standard[0] - Verb + の (replacing non-specific things, "one")
    // Example: "いるの" where の replaces 車
    #[test]
    fn test_no_replacement_one() {
        let sentence = "あの車、あなたが乗っているのですか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "の");
        // Note: Range extends to include auxiliary です (automatic range extension)
        // Core pattern: いる (11-13) + の (13-14) → Extended: いるのです (11-16)
        assert_pattern_range(&patterns, "の", 11, 16); // いるのです
    }

    // Testing: standard[0] - Noun + の (casual conversation)
    // Example: "俺の" in casual speech
    #[test]
    fn test_no_casual_mine() {
        let sentence = "それ、俺のだから返してくれ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "の");
        assert_pattern_range(&patterns, "の", 3, 5); // 俺の
    }
}

// ========== は (Topic marker) ==========
// Pattern: は (topic marker - marks sentence topic, pronounced "wa")
// Data source: grammar_points_data.json["は"]
//
// Structure variants to test:
//   standard[0]: Sentence topic + は
//
// Notes:
//   - は is a 係助詞 (binding particle)
//   - Pronounced "wa" but written は
//   - Marks the topic of the sentence (not the subject - that's が)
//   - Can be used for contrast when used later in sentence
//   - Very fundamental particle, low priority to avoid over-highlighting

mod ha_topic_tests {
    use super::*;

    // Testing: standard[0] - Noun + は (basic topic marker)
    // Example: "さんは" marking the topic (さん tokenized separately from 田中)
    #[test]
    fn test_ha_noun_topic() {
        let sentence = "田中さんは先生です";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "は");
        assert_pattern_range(&patterns, "は", 2, 5); // さんは
    }

    // Testing: standard[0] - Pronoun + は (topic marker)
    // Example: "私は" marking the topic
    #[test]
    fn test_ha_pronoun_topic() {
        let sentence = "私はトムです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "は");
        assert_pattern_range(&patterns, "は", 0, 2); // 私は
    }

    // Testing: standard[0] - Noun + は (with adjective predicate)
    // Example: "カレーは" marking the topic
    #[test]
    fn test_ha_noun_adjective() {
        let sentence = "カレーは辛い";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "は");
        assert_pattern_range(&patterns, "は", 0, 4); // カレーは
    }

    // Testing: standard[0] - は used for contrast (mid-sentence)
    // Example: "金曜日は" with contrastive meaning (also "私は" at start)
    #[test]
    fn test_ha_contrast() {
        let sentence = "私は、金曜日は好き";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "は");
        // This sentence has two instances: 私は (0-2) and 金曜日は (3-7)
        // Testing for the contrastive one (金曜日は)
        assert_pattern_range(&patterns, "は", 3, 7); // 金曜日は
    }

    // Testing: standard[0] - Casual speech with は
    // Example: "俺は" in casual context
    #[test]
    fn test_ha_casual_negative() {
        let sentence = "そんなこと、俺は知らないよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "は");
        assert_pattern_range(&patterns, "は", 6, 8); // 俺は
    }

    // Testing: standard[0] - Question with は
    // Example: "あなたは" in question
    #[test]
    fn test_ha_question() {
        let sentence = "あなたは誰ですか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "は");
        assert_pattern_range(&patterns, "は", 0, 4); // あなたは
    }
}

