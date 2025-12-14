use super::*;

// Pattern: 〜ようではないか (why don't we, let's)
// Data source: grammar_points_data.json["〜ようではないか"]
// Testing: structure.standard[0] - "Verb［おう］ + ではないか"
// Testing: structure.standard[1] - "Verb［おう］ + じゃないか"
//
// Structure variants:
//   - standard[0]: Verb［おう］ + ではないか (formal)
//   - standard[1]: Verb［おう］ + じゃないか (casual/abbreviated)

mod youdehanaika_tests {
    use super::*;

    #[test]
    fn test_ou_dehanaika() {
        let sentence = "正々堂々と戦おうではないか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "〜ようではないか");
        assert_pattern_range(&patterns, "〜ようではないか", 5, 13); // 戦おうではないか
    }

    #[test]
    fn test_you_dehanaika() {
        let sentence = "この問題をどう解決するかみんなで考えようではないか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "〜ようではないか");
        assert_pattern_range(&patterns, "〜ようではないか", 16, 25); // 考えようではないか
    }

    #[test]
    fn test_ou_janai() {
        let sentence = "こういう時こそお互い助け合おうじゃないか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "〜ようではないか");
        assert_pattern_range(&patterns, "〜ようではないか", 10, 20); // 助け合おうじゃないか
    }

    #[test]
    fn test_you_janai() {
        let sentence = "食べれるだけの日本食を食べようじゃないか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "〜ようではないか");
        assert_pattern_range(&patterns, "〜ようではないか", 11, 20); // 食べようじゃないか
    }
}

// Pattern: いきなり (suddenly, all of a sudden)
// Data source: grammar_points_data.json["いきなり"]
// Testing: structure.standard[0] - "いきなり + (Action) Phrase"
//
// Structure variants:
//   - standard[0]: いきなり + (Action) Phrase
//   - No polite forms listed

mod ikinari_tests {
    use super::*;

    #[test]
    fn test_ikinari_before_verb() {
        let sentence = "いきなり電話してごめん";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いきなり");
        assert_pattern_range(&patterns, "いきなり", 0, 4); // いきなり
    }

    #[test]
    fn test_ikinari_mid_sentence() {
        let sentence = "ペットのワンちゃんがいきなり吠え出したからびっくりして起きた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いきなり");
        assert_pattern_range(&patterns, "いきなり", 10, 14); // いきなり
    }
}

// Pattern: いよいよ (finally, at last, more and more)
// Data source: grammar_points_data.json["いよいよ"]
// Testing: structure.standard[0] - "いよいよ + Phrase"
//
// Structure variants:
//   - standard[0]: いよいよ + Phrase
//   - No polite forms listed

mod iyoiyo_tests {
    use super::*;

    #[test]
    fn test_iyoiyo_finally() {
        let sentence = "いよいよ明日で卒業か";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いよいよ");
        assert_pattern_range(&patterns, "いよいよ", 0, 4); // いよいよ
    }

    #[test]
    fn test_iyoiyo_more_and_more() {
        let sentence = "彼の話を聞いていると彼がいよいよ怪しくなってきた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いよいよ");
        assert_pattern_range(&patterns, "いよいよ", 12, 16); // いよいよ
    }
}

// Pattern: おそらく (probably, perhaps)
// Data source: grammar_points_data.json["おそらく"]
// Testing: structure.standard[0] - "おそらく + Phrase"
//
// Structure variants:
//   - standard[0]: おそらく + Phrase
//   - No polite forms listed

mod osoraku_tests {
    use super::*;

    #[test]
    fn test_osoraku_with_conjecture() {
        let sentence = "あいつの不自然な行動からして、おそらくあいつが犯人だろう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "おそらく");
        assert_pattern_range(&patterns, "おそらく", 15, 19); // おそらく
    }

    #[test]
    fn test_osoraku_at_start() {
        let sentence = "おそらく明日は雨なので、ピクニックは中止しましょう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "おそらく");
        assert_pattern_range(&patterns, "おそらく", 0, 4); // おそらく
    }
}

// Pattern: いわゆる (so-called, what is called)
// Data source: grammar_points_data.json["いわゆる"]
// Testing: structure.standard[0] - "いわゆる + Noun"
//
// Structure variants:
//   - standard[0]: いわゆる + Noun

mod iwayuru_tests {
    use super::*;

    #[test]
    fn test_iwayuru_before_noun() {
        let sentence = "ハンバーガーやフライドポテトはいわゆるジャンクフードだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いわゆる");
        assert_pattern_range(&patterns, "いわゆる", 15, 19); // いわゆる
    }

    #[test]
    fn test_iwayuru_conspiracy_theory() {
        let sentence = "それはいわゆる陰謀論というものですね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いわゆる");
        assert_pattern_range(&patterns, "いわゆる", 3, 7); // いわゆる
    }
}

// Pattern: おおよそ (approximately, roughly)
// Data source: grammar_points_data.json["おおよそ"]
// Testing: structure.standard[0] - "おおよそ + Noun"
// Testing: structure.standard[1] - "およそ + Noun"
//
// Structure variants:
//   - standard[0]: おおよそ + Noun
//   - standard[1]: およそ + Noun (alternative form)

mod ooyoso_tests {
    use super::*;

    #[test]
    fn test_ooyoso_full_form() {
        let sentence = "私はおおよそ理解したけど、周りの子達はポカーンとした表情で先生を見ていた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "おおよそ");
        assert_pattern_range(&patterns, "おおよそ", 2, 6); // おおよそ
    }

    #[test]
    fn test_oyoso_abbreviated_form() {
        let sentence = "この物件から最寄りの駅までは徒歩でおよそ１０分かかります";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "おおよそ");
        assert_pattern_range(&patterns, "おおよそ", 17, 20); // およそ
    }
}

// Pattern: おまけに (besides, in addition, to make matters worse)
// Data source: grammar_points_data.json["おまけに"]
// Testing: structure.standard[0] - "おまけに + Phrase"
//
// Structure variants:
//   - standard[0]: おまけに + Phrase

mod omakeni_tests {
    use super::*;

    #[test]
    fn test_omakeni_negative_addition() {
        let sentence = "今日は仕事に遅刻して部長に怒られたし、おまけに取引先の人も怒らせちゃった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "おまけに");
        assert_pattern_range(&patterns, "おまけに", 19, 23); // おまけに
    }

    #[test]
    fn test_omakeni_positive_addition() {
        let sentence = "昨日は彼氏に美味しいご飯をご馳走してもらって、おまけにプレゼントまでもらった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "おまけに");
        assert_pattern_range(&patterns, "おまけに", 23, 27); // おまけに
    }
}

// および (and, as well as)
// Data source: grammar_points_data.json["および"]
// Testing: structure.standard[0] - "Noun + および"
mod oyobi_tests {
    use super::*;

    #[test]
    fn test_oyobi_noun_conjunction() {
        let sentence = "免許証および印鑑を持ってきてください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "および");
        assert_pattern_range(&patterns, "および", 2, 6); // 証および
    }

    #[test]
    fn test_oyobi_sentence_start() {
        let sentence = "学校説明会に参加されたい方、および体験入学をされたい方は電話をください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "および");
        assert_pattern_range(&patterns, "および", 12, 17); // 方、および
    }
}

// いつの間にか (before one knows it, suddenly)
// Data source: grammar_points_data.json["いつの間にか"]
// Testing: structure.standard[0] - "いつのまにか + Phrase"
mod itsunomanika_tests {
    use super::*;

    #[test]
    fn test_itsunomanika_gradual_change() {
        let sentence = "３年間日本人の友達と毎日日本語で話していたら、いつのまにか日本語が上手になっていた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いつの間にか");
        assert_pattern_range(&patterns, "いつの間にか", 23, 29); // いつのまにか
    }

    #[test]
    fn test_itsunomanika_sudden_realization() {
        let sentence = "気づいたら、いつのまにか冬になっていた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いつの間にか");
        assert_pattern_range(&patterns, "いつの間にか", 6, 12); // いつのまにか
    }
}

// あげく (in the end, after all)
// Data source: grammar_points_data.json["あげく"]
// Testing multiple structure variants
mod ageku_tests {
    use super::*;

    // Testing: structure.standard[0] - "Verb［た］+ あげく + (に)"
    #[test]
    fn test_ageku_verb_past() {
        let sentence = "私は夫と話し合いをしたあげく、離婚することに決めた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あげく");
        assert_pattern_range(&patterns, "あげく", 10, 14); // たあげく
    }

    // Testing: structure.standard[1] - "Verb［た］+ あげく + の + Noun"
    #[test]
    fn test_ageku_no_noun() {
        let sentence = "長時間の議論のあげくの結論がこれですか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あげく");
        assert_pattern_range(&patterns, "あげく", 4, 10); // 議論のあげく
    }

    // Testing: structure.standard[2] - "Noun + の + あげく + (に)"
    #[test]
    fn test_ageku_noun_no() {
        let sentence = "田中さんと中田さんは口論のあげく、喧嘩になってしまった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あげく");
        assert_pattern_range(&patterns, "あげく", 10, 16); // 口論のあげく
    }
}

// Pattern: ～を～に任せる (entrust X to Y)
// Data source: grammar_points_data.json["～を～に任せる"]
// Testing: structure.standard[0] - "(Task) Nounを + (Target) Nounに + 任せる"
// Testing: structure.standard[1] - "(Target) Nounに + (Task) Nounを + 任せる"
//
// Structure variants:
//   - standard[0]: (Task) Nounを + (Target) Nounに + 任せる (task-target order)
//   - standard[1]: (Target) Nounに + (Task) Nounを + 任せる (target-task order)
//   - polite[0-1]: Same with 任せます

mod wo_ni_makaseru_tests {
    use super::*;

    // Testing: structure.standard[0] - "(Task) Nounを + (Target) Nounに + 任せる"
    #[test]
    fn test_task_wo_target_ni() {
        let sentence = "私は忙しいから、ペットの世話をあなたに任せてもいい？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～を～に任せる");
        assert_pattern_range(&patterns, "～を～に任せる", 14, 21); // をあなたに任せ
    }

    // Testing: structure.standard[1] - "(Target) Nounに + (Task) Nounを + 任せる"
    #[test]
    fn test_target_ni_task_wo() {
        let sentence = "新人君にあの重要なプレゼンを任せたの？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～を～に任せる");
        assert_pattern_range(&patterns, "～を～に任せる", 3, 17); // にあの重要なプレゼンを任せた
    }

    // Testing: polite form - "Nounを + Nounに + 任せます"
    #[test]
    fn test_polite_form() {
        let sentence = "この計画を部長に任せます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～を～に任せる");
        assert_pattern_range(&patterns, "～を～に任せる", 4, 12); // を部長に任せます
    }

    // Testing: with fate/luck as target
    #[test]
    fn test_luck_target() {
        let sentence = "合否を運に任せる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～を～に任せる");
        assert_pattern_range(&patterns, "～を～に任せる", 2, 8); // を運に任せる
    }
}

// Pattern: 〜得ない (cannot, impossible)
// Data source: grammar_points_data.json["〜得ない"]
// Testing structure variants:
//   - standard[0]: Verb[stem] + えない
//   - polite[0]: Verb[stem] + えません
#[cfg(test)]
mod enai_tests {
    use super::*;

    // Testing: structure.standard[0] - Verb[stem] + えない
    #[test]
    fn test_verb_stem_enai() {
        let sentence = "本人以外は知りえない情報だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "〜得ない");
        assert_pattern_range(&patterns, "〜得ない", 5, 10); // 知りえない
    }

    // Testing: structure.polite[0] - Verb[stem] + えません
    #[test]
    fn test_verb_stem_emasen() {
        let sentence = "10年後の自分なんて想像しえません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "〜得ない");
        assert_pattern_range(&patterns, "〜得ない", 10, 17); // 想像しえません
    }

    // Testing: common set phrase ありえない (impossible)
    #[test]
    fn test_arieru_common_phrase() {
        let sentence = "田中さんが不合格なんて絶対ありえない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "〜得ない");
        assert_pattern_range(&patterns, "〜得ない", 13, 18); // ありえない
    }

    // Testing: another realistic example
    #[test]
    fn test_okori_enai() {
        let sentence = "そんな大きな災害は起こりえないと思います";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "〜得ない");
        assert_pattern_range(&patterns, "〜得ない", 9, 15); // 起こりえない
    }
}

// Pattern: かねる (cannot, difficult to do)
// Data source: grammar_points_data.json["かねる"]
// Testing structure variants:
//   - standard[0]: Verb[stem] + かねる
//   - polite[0]: Verb[stem] + かねます
#[cfg(test)]
mod kaneru_tests {
    use super::*;

    // Testing: structure.standard[0] - Verb[stem] + かねる
    #[test]
    fn test_verb_stem_kaneru() {
        let sentence = "こういう場合でも返金はできかねます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かねる");
        assert_pattern_range(&patterns, "かねる", 11, 17); // できかねます
    }

    // Testing: structure.polite[0] - Verb[stem] + かねます
    #[test]
    fn test_verb_stem_kanemasu() {
        let sentence = "それはお答えしかねます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かねる");
        assert_pattern_range(&patterns, "かねる", 3, 11); // お答えしかねます
    }

    // Testing: another realistic example
    #[test]
    fn test_uketsuke_kanemasu() {
        let sentence = "お電話での予約受付は受けかねます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かねる");
        assert_pattern_range(&patterns, "かねる", 10, 16); // 受けかねます
    }
}

// Pattern: かねない (might, capable of)
// Data source: grammar_points_data.json["かねない"]
// Testing structure variants:
//   - standard[0]: Verb[stem] + かねない
//   - standard[1]: Noun + に + かねない
//   - polite[0]: Verb[stem] + かねません
//   - polite[1]: Noun + に + かねません
#[cfg(test)]
mod kanenai_tests {
    use super::*;

    // Testing: structure.standard[0] - Verb[stem] + かねない
    #[test]
    fn test_verb_stem_kanenai() {
        let sentence = "それは命を落としかねない感染症らしい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かねない");
        assert_pattern_range(&patterns, "かねない", 5, 12); // 落としかねない
    }

    // Testing: structure.standard[1] - Noun + に + かねない (via になる verb)
    #[test]
    fn test_noun_ni_kanenai() {
        let sentence = "仕事のやりすぎは鬱の原因になりかねない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かねない");
        assert_pattern_range(&patterns, "かねない", 13, 19); // なりかねない
    }

    // Testing: structure.polite[0] - Verb[stem] + かねません
    #[test]
    fn test_verb_stem_kanemasen() {
        let sentence = "地震の後には津波が起こりかねません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かねない");
        assert_pattern_range(&patterns, "かねない", 9, 17); // 起こりかねません
    }

    // Testing: with conditional phrase
    #[test]
    fn test_conditional_kanenai() {
        let sentence = "操作を間違えれば怪我人が出かねない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かねない");
        assert_pattern_range(&patterns, "かねない", 12, 17); // 出かねない
    }
}

// Pattern: からには (as long as, since, given that)
// Data source: grammar_points_data.json["からには"]
// Testing: structure.standard[0] - "Verb[る] + からには"
//
// Other structures tested:
//   - standard[1]: Verb[た] + からには
//   - standard[2]: い-Adjective + からには
//   - standard[3]: な-Adjective + である + からには
//   - standard[4]: Noun + である + からには
//
// Note: Pattern matches the immediately preceding token + からには
// For verb+た forms, the pattern matches た+からには (not full verb phrase)
mod karaniha_tests {
    use super::*;

    #[test]
    fn test_verb_ta_form() {
        let sentence = "ここまで来たからには、最後まで頑張りたい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "からには");
        assert_pattern_range(&patterns, "からには", 5, 10); // たからには
    }

    #[test]
    fn test_verb_ta_form_2() {
        let sentence = "約束したからには、必ず守らなければならない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "からには");
        assert_pattern_range(&patterns, "からには", 3, 8); // たからには
    }

    #[test]
    fn test_i_adjective() {
        let sentence = "高いからには、それなりの品質を期待している";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "からには");
        assert_pattern_range(&patterns, "からには", 0, 6); // 高いからには
    }

    #[test]
    fn test_na_adjective_dearu() {
        let sentence = "親として有名であるからには、責任がある";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "からには");
        assert_pattern_range(&patterns, "からには", 7, 13); // あるからには
    }

    #[test]
    fn test_noun_dearu() {
        let sentence = "教師であるからには、生徒の手本となるべきだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "からには");
        assert_pattern_range(&patterns, "からには", 3, 9); // あるからには
    }
}
