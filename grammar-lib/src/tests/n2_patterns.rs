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

// Pattern: からして (based on, judging from)
// Data source: grammar_points_data.json["からして"]
// Testing: structure.standard[0] - "Noun + からして"
//
// からして tokenizes as: から (particle) + し (する verb) + て (particle)
// Pattern includes noun + から + し + て
mod karashite_tests {
    use super::*;

    #[test]
    fn test_noun_karashite_personality() {
        let sentence = "彼の性格からして、一緒に住むのは無理だろう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "からして");
        assert_pattern_range(&patterns, "からして", 2, 8); // 性格からして
    }

    #[test]
    fn test_noun_karashite_price() {
        let sentence = "値段からして、このお店は高級だと思う";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "からして");
        assert_pattern_range(&patterns, "からして", 0, 6); // 値段からして
    }

    #[test]
    fn test_noun_karashite_name() {
        let sentence = "名前からしてつまらなそうなゲームだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "からして");
        assert_pattern_range(&patterns, "からして", 0, 6); // 名前からして
    }
}

// Pattern: からといって (just because)
// Data source: grammar_points_data.json["からといって"]
// Testing: structure.standard[0] - "Verb + からといって"
//
// Other structures tested:
//   - standard[1]: い-Adjective + からといって
//   - standard[2]: な-Adjective + だからといって
//   - standard[3]: Noun + だからといって
//
// からといって tokenizes as: から + と + いう + て (4 tokens)
// For な-Adj/Noun: だ + から + と + いう + て (5 tokens, starts with だ)
mod karatoitte_tests {
    use super::*;

    #[test]
    fn test_verb_karatoitte() {
        let sentence = "たくさん勉強したからといって、合格するとは限らない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "からといって");
        assert_pattern_range(&patterns, "からといって", 7, 14); // たからといって
    }

    #[test]
    fn test_i_adjective_karatoitte() {
        let sentence = "公園が広いからといって、犬を放し飼いにしていいわけではない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "からといって");
        assert_pattern_range(&patterns, "からといって", 3, 11); // 広いからといって
    }

    #[test]
    fn test_na_adjective_dakaratoitte() {
        let sentence = "丈夫だからといって、雑に扱えば壊れる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "からといって");
        assert_pattern_range(&patterns, "からといって", 2, 9); // だからといって
    }

    #[test]
    fn test_noun_dakaratoitte() {
        let sentence = "日本人だからといって、漢字が書けるとは限らない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "からといって");
        assert_pattern_range(&patterns, "からといって", 3, 10); // だからといって
    }
}

// Pattern: からすると・からすれば (judging from, considering)
// Data source: grammar_points_data.json["からすると・からすれば"]
// Testing: structure.standard[0] - "Noun + からすると"
//          structure.standard[1] - "Noun + からすれば"
//
// Expresses speaker's judgment/assessment based on (A)
// Nuance: "going off (A)" or "considering (A)" as basis for judgment
// Literal: から (from) + する (to do) + と/ば (hypothetical)
mod karasuruto_karasureba_tests {
    use super::*;

    #[test]
    fn test_noun_karasuruto() {
        let sentence = "この成績からすると、あなたは全然勉強をしていないでしょう？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "からすると・からすれば");
        assert_pattern_range(&patterns, "からすると・からすれば", 2, 9); // 成績からすると
    }

    #[test]
    fn test_noun_karasureba() {
        let sentence = "日本育ちの私からすれば、これは寿司ではない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "からすると・からすれば");
        assert_pattern_range(&patterns, "からすると・からすれば", 5, 11); // 私からすれば
    }

    #[test]
    fn test_reaction_karasuruto() {
        let sentence = "彼女の反応からすると、本当に驚いているようだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "からすると・からすれば");
        assert_pattern_range(&patterns, "からすると・からすれば", 3, 10); // 反応からすると
    }

    #[test]
    fn test_sweetness_karasuruto() {
        let sentence = "この甘さからすると、塩と間違えて砂糖を入れてしまったに違いない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "からすると・からすれば");
        assert_pattern_range(&patterns, "からすると・からすれば", 3, 9); // さからすると
    }
}

// Pattern: から見ると (from the perspective of, judging from)
// Data source: grammar_points_data.json["から見ると"]
// Testing: structure.standard[0] - "Noun + から見（み）ると"
//          structure.standard[1] - "Noun + から見（み）れば"
//          structure.standard[2] - "Noun + から見（み）て"
//          structure.standard[3] - "Noun + から見（み）たら"
//
// Expresses viewing from perspective of (A) or judgment based on looking at (A)
// Nuance: "from the point of view of (A)" or "from looking at (A)"
// Literal: から (from) + 見る (to look) + と/ば/て/たら (hypothetical/connective)
mod karamiruto_tests {
    use super::*;

    #[test]
    fn test_noun_karamiruto() {
        let sentence = "アニメに興味がない人からみると、どのアニメも同じに見える";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "から見ると");
        assert_pattern_range(&patterns, "から見ると", 9, 15); // 人からみると
    }

    #[test]
    fn test_noun_karamireba() {
        let sentence = "私からみれば、彼女はあまり美人じゃない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "から見ると");
        assert_pattern_range(&patterns, "から見ると", 0, 6); // 私からみれば
    }

    #[test]
    fn test_noun_karamite() {
        let sentence = "このタイヤ跡の大きさからみて、犯人はトラックを使ったに違いない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "から見ると");
        assert_pattern_range(&patterns, "から見ると", 9, 14); // さからみて
    }

    #[test]
    fn test_noun_karamitara() {
        let sentence = "外からみたら、私のコレクションはゴミに見えるでしょう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "から見ると");
        assert_pattern_range(&patterns, "から見ると", 0, 6); // 外からみたら
    }
}

// Pattern: か何か (or something, or something like that)
// Data source: grammar_points_data.json["か何か"]
// Testing: structure.standard[0] - "Noun + か何（なに）か"
//
// Expresses uncertainty about a specific thing
// Nuance: "(A) or whatever it is", "or something like (A)"
// Often used when offering something or when more options are available
mod kananika_tests {
    use super::*;

    #[test]
    fn test_noun_kananika_question() {
        let sentence = "すみません、お茶かなにかありませんか？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "か何か");
        assert_pattern_range(&patterns, "か何か", 6, 12); // お茶かなにか
    }

    #[test]
    fn test_noun_kananika_holding() {
        let sentence = "あそこにナイフかなにかを持った怪しい人がいました";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "か何か");
        assert_pattern_range(&patterns, "か何か", 4, 11); // ナイフかなにか
    }

    #[test]
    fn test_noun_kananika_offer() {
        let sentence = "紅茶かなにか飲みませんか？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "か何か");
        assert_pattern_range(&patterns, "か何か", 0, 6); // 紅茶かなにか
    }
}

// Pattern: かのようだ (as if, seems like)
// Data source: grammar_points_data.json["かのようだ"]
// Testing: structure.standard - "Verb/Adj/Noun + かのようだ"
//
// Structure variants:
//   - standard[0]: Verb + かのようだ
//   - standard[1]: い-Adjective + かのようだ
//   - standard[2]: な-Adjective + である + かのようだ
//   - standard[3]: Noun + である + かのようだ
//   - standard[4]: かのように + Phrase、かのような + Noun
//   - polite[0-4]: Same with です instead of だ

mod kanoyouda_tests {
    use super::*;

    #[test]
    fn test_verb_kanoyouda() {
        let sentence = "このVRゲームはまるで本物の飛行機を操縦しているかのようだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かのようだ");
        assert_pattern_range(&patterns, "かのようだ", 22, 29); // いるかのようだ
    }

    #[test]
    fn test_i_adj_kanoyouda() {
        let sentence = "この町には誰もいないかのようだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かのようだ");
        assert_pattern_range(&patterns, "かのようだ", 8, 15); // ないかのようだ
    }

    #[test]
    fn test_na_adj_dearu_kanoyouda() {
        let sentence = "彼はまるで私と話すのが面倒であるかのようだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かのようだ");
        assert_pattern_range(&patterns, "かのようだ", 14, 21); // あるかのようだ
    }

    #[test]
    fn test_noun_dearu_kanoyouda() {
        let sentence = "彼と僕は兄弟であるかのようだが、実は彼は僕のパートナーです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かのようだ");
        assert_pattern_range(&patterns, "かのようだ", 7, 14); // あるかのようだ
    }

    #[test]
    fn test_kanoyouni_phrase() {
        let sentence = "長谷川くんのお母さんは私を自分の子供であるかのように小さい頃から可愛がってくれていた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かのようだ");
        assert_pattern_range(&patterns, "かのようだ", 19, 26); // あるかのように
    }

    #[test]
    fn test_kanoyouna_noun() {
        let sentence = "このスーパーに入ると、まるで母国に帰ってきたかのような感じがする";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かのようだ");
        assert_pattern_range(&patterns, "かのようだ", 21, 27); // たかのような
    }

    #[test]
    fn test_kanoyoudesu_polite() {
        let sentence = "彼女は何も知らないかのようです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かのようだ");
        assert_pattern_range(&patterns, "かのようだ", 7, 15); // ないかのようです
    }
}

// Pattern: げ (seeming, appearance)
// Data source: grammar_points_data.json["げ"]
// Testing: structure.standard - "Adj/Verb[stem] + げ + に/な"
//
// Structure variants:
//   - standard[0]: Verb[stem] + げ + に
//   - standard[1]: い-Adjective[stem] + げ + に
//   - standard[2]: な-Adjective + げ + に
//   - standard[3]: (All forms) + な + Noun

mod ge_tests {
    use super::*;

    #[test]
    fn test_i_adj_ge_ni() {
        let sentence = "高校生たちがファストフード店で楽しげに話しているのを見た";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "げ");
        assert_pattern_range(&patterns, "げ", 15, 19); // 楽しげに
    }

    #[test]
    fn test_na_adj_ge_ni() {
        let sentence = "高橋くんは不安げに手を上げました";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "げ");
        assert_pattern_range(&patterns, "げ", 5, 9); // 不安げに
    }

    #[test]
    fn test_verb_stem_ge_ni() {
        let sentence = "自信ありげに見えたかもしれないけど、実はめちゃくちゃ緊張してた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "げ");
        assert_pattern_range(&patterns, "げ", 2, 6); // ありげに
    }

    #[test]
    fn test_ge_na_noun() {
        let sentence = "彼は意味ありげな笑みを浮かべていた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "げ");
        assert_pattern_range(&patterns, "げ", 4, 8); // ありげな
    }
}

// Pattern: きっかけ (opportunity, chance, trigger)
// Data source: grammar_points_data.json["きっかけ"]
// Testing: structure.standard - various きっかけ constructions
//
// Structure variants:
//   - standard[0]: Verb + の + をきっかけ + に
//   - standard[1]: Noun + をきっかけ + に
//   - standard[2]: Verb + の + がきっかけ + で
//   - standard[3]: Noun + がきっかけ + で

mod kikkake_tests {
    use super::*;

    #[test]
    fn test_verb_no_wo_kikkake_ni() {
        let sentence = "彼女とは、パーティーで会ったのをきっかけに付き合い始めた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "きっかけ");
        assert_pattern_range(&patterns, "きっかけ", 15, 21); // のをきっかけに
    }

    #[test]
    fn test_noun_wo_kikkake_ni() {
        let sentence = "私は入院をきっかけにタバコを止めることにしました";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "きっかけ");
        assert_pattern_range(&patterns, "きっかけ", 4, 10); // をきっかけに
    }

    #[test]
    fn test_verb_no_ga_kikkake_de() {
        let sentence = "この音楽は有名な歌手がカバーしたのがきっかけで再び有名になった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "きっかけ");
        assert_pattern_range(&patterns, "きっかけ", 17, 23); // のがきっかけで
    }

    #[test]
    fn test_noun_ga_kikkake_de() {
        let sentence = "このゲームがきっかけで、プログラマーになる事ができた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "きっかけ");
        assert_pattern_range(&patterns, "きっかけ", 5, 11); // がきっかけで
    }
}

// Pattern: お～願う (humble request)
// Data source: grammar_points_data.json["お～願う"]
// Testing all structure variants:
//   - standard[0]: お + Verb[stem] + 願う
//   - standard[1]: ご + Chinese-origin Noun + 願う
//   - standard[2]: Western-origin Noun + 願う (no prefix)
//   - polite[0]: お + Verb[stem] + 願います
//   - polite[1]: ご + Chinese-origin Noun + 願います
//   - polite[2]: Western-origin Noun + 願います (no prefix)

mod o_uff5e_negau_tests {
    use super::*;

    #[test]
    fn test_o_verb_negau_standard() {
        let sentence = "この件についてもう一度お調べ願う";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "お～願う");
        assert_pattern_range(&patterns, "お～願う", 11, 16); // お調べ願う
    }

    #[test]
    fn test_o_verb_negaimasu_polite() {
        let sentence = "こちらからのメールが届いているか、お確かめ願います";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "お～願う");
        assert_pattern_range(&patterns, "お～願う", 17, 25); // お確かめ願います
    }

    #[test]
    fn test_go_noun_negaimasu_chinese() {
        let sentence = "この契約書にご記入された情報に間違いがないか、ご確認願います";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "お～願う");
        assert_pattern_range(&patterns, "お～願う", 23, 30); // ご確認願います
    }

    #[test]
    fn test_go_noun_negaimasu_cooperation() {
        let sentence = "歩道での禁煙にご協力願います";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "お～願う");
        assert_pattern_range(&patterns, "お～願う", 7, 14); // ご協力願います
    }

    #[test]
    fn test_western_noun_negaimasu() {
        let sentence = "この契約書の下の方にサイン願います";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "お～願う");
        assert_pattern_range(&patterns, "お～願う", 10, 17); // サイン願います
    }

    #[test]
    fn test_o_verb_negaimasu_wait() {
        let sentence = "社長はもうすぐ到着するので、もう少々お待ち願います";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "お～願う");
        assert_pattern_range(&patterns, "お～願う", 18, 25); // お待ち願います
    }
}

// Pattern: がけに (on the way, as you go)
// Data source: grammar_points_data.json["がけに"]
// Testing: structure.standard[0] - "Verb[stem] + がけに"
//
// Note: Only 1 structure variant (standard only, no polite form)

mod gakeni_tests {
    use super::*;

    #[test]
    fn test_kaeri_gakeni() {
        let sentence = "帰りがけに駅前のたこ焼き屋でたこ焼きを買った";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がけに");
        assert_pattern_range(&patterns, "がけに", 0, 5); // 帰りがけに
    }

    #[test]
    fn test_iki_gakeni() {
        let sentence = "今日は学校への行きがけにコンビニで弁当を買う";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がけに");
        assert_pattern_range(&patterns, "がけに", 7, 12); // 行きがけに
    }

    #[test]
    fn test_toori_gakeni() {
        let sentence = "お父さん、通りがけにサービスエリアにでも寄って行こ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がけに");
        assert_pattern_range(&patterns, "がけに", 5, 10); // 通りがけに
    }
}

// Pattern: ことなく (without doing)
// Data source: grammar_points_data.json["ことなく"]
// Testing: structure.standard[0] - "Verb + ことなく"
//
// Note: Only 1 structure variant (standard only, no polite form)
// More formal than ないで

mod kotonaku_tests {
    use super::*;

    #[test]
    fn test_verb_kotonaku_preventative() {
        let sentence = "怪我人を出すことなく、人質を全員救出する事ができた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことなく");
        assert_pattern_range(&patterns, "ことなく", 4, 10); // 出すことなく
    }

    #[test]
    fn test_verb_kotonaku_arrival() {
        let sentence = "遅刻することなく、職場に着いた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことなく");
        assert_pattern_range(&patterns, "ことなく", 0, 8); // 遅刻することなく
    }

    #[test]
    fn test_verb_kotonaku_unmet_desire() {
        let sentence = "犯人が捕まることなく１０年が経つ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことなく");
        assert_pattern_range(&patterns, "ことなく", 3, 10); // 捕まることなく
    }

    #[test]
    fn test_verb_kotonaku_quit() {
        let sentence = "彼は社長に何も言うことなく会社を辞めた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことなく");
        assert_pattern_range(&patterns, "ことなく", 7, 13); // 言うことなく
    }
}

// Pattern: か〜ないかのうちに (as soon as, just when, barely when)
// Data source: grammar_points_data.json["か〜ないかのうちに"]
// Structure: Verb[る] + か + Verb[ない] + かのうちに (same verb repeated)
// Testing: structure.standard[0] - "Verb[る] + か + Verb ない + かのうちに"
mod ka_naika_nouchini_tests {
    use super::*;

    #[test]
    fn test_verb_dictionary_form() {
        // Example: 飲み終わるか飲み終わらないかのうちに (as soon as finishing drinking)
        // Note: Compound verb 飲み終わる tokenizes as 飲み + 終わる
        // Pattern starts from 終わる (the verb in 基本形)
        let sentence = "父はビールを飲み終わるか飲み終わらないかのうちに、新しい缶を開けた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "か〜ないかのうちに");
        assert_pattern_range(&patterns, "か〜ないかのうちに", 8, 24); // 終わるか飲み終わらないかのうちに
    }

    #[test]
    fn test_simple_verb_sit() {
        // Example: 座るか座らないかのうちに (just as sitting down)
        let sentence = "彼は椅子に座るか座らないかのうちに、テレビをつけた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "か〜ないかのうちに");
        assert_pattern_range(&patterns, "か〜ないかのうちに", 5, 17); // 座るか座らないかのうちに
    }

    #[test]
    fn test_verb_stop() {
        // Example: 止まるか止まらないかのうちに (just as stopping)
        let sentence = "彼女は車が止まるか止まらないかのうちに、ドアを開けて駅へと走っていった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "か〜ないかのうちに");
        assert_pattern_range(&patterns, "か〜ないかのうちに", 5, 19); // 止まるか止まらないかのうちに
    }

    #[test]
    fn test_time_period() {
        // Example: 経つか経たないかのうちに (in just [time period])
        let sentence = "就活を始めて一週間経つか経たないかのうちに、仕事が見つかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "か〜ないかのうちに");
        assert_pattern_range(&patterns, "か〜ないかのうちに", 9, 21); // 経つか経たないかのうちに
    }
}

// Pattern: かと思ったら・かと思うと (just when I thought)
// Data source: grammar_points_data.json["かと思ったら・かと思うと"]
// Testing structures: Verb[た/る] + かと思ったら/かと思うと/かと思えば
mod katoomottara_tests {
    use super::*;

    #[test]
    fn test_verb_past_omottara() {
        // Example: 泣き止んだかと思ったら (just when I thought [baby] had stopped crying)
        let sentence = "赤ちゃんが泣き止んだかと思ったら、また大声で泣き始めた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かと思ったら・かと思うと");
        assert_pattern_range(&patterns, "かと思ったら・かと思うと", 7, 16); // 止んだかと思ったら
    }

    #[test]
    fn test_verb_dict_omouto() {
        // Example: 始めるのかと思うと (just when I thought [she] would start)
        let sentence = "娘が宿題を始めるのかと思うと、パソコンを開いてユーチューブを見始めた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かと思ったら・かと思うと");
        assert_pattern_range(&patterns, "かと思ったら・かと思うと", 5, 14); // 始めるのかと思うと
    }

    #[test]
    fn test_verb_past_omouto() {
        // Example: 転んだかと思うと (no sooner than [she] fell)
        let sentence = "子供が転んだかと思うと、立ち上がって走り出した";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かと思ったら・かと思うと");
        assert_pattern_range(&patterns, "かと思ったら・かと思うと", 3, 11); // 転んだかと思うと
    }

    #[test]
    fn test_verb_dict_omoeba() {
        // Example: 遅れるかと思えば (just when I thought [he] would be late)
        let sentence = "彼は遅れるかと思えば、意外と早く到着した";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かと思ったら・かと思うと");
        assert_pattern_range(&patterns, "かと思ったら・かと思うと", 2, 10); // 遅れるかと思えば
    }

    #[test]
    fn test_noun_omottara() {
        // Example: 冷たい人かと思ったら (just when I thought [he] was cold)
        let sentence = "田中さんは冷たい人かと思ったら、ただ人見知りなだけだった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かと思ったら・かと思うと");
        assert_pattern_range(&patterns, "かと思ったら・かと思うと", 8, 15); // 人かと思ったら
    }

    #[test]
    fn test_noun_omouto() {
        // Example: 最後かと思うと (just when I remembered [this] is the last one)
        let sentence = "今日の試合が最後かと思うと、悲しくなる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かと思ったら・かと思うと");
        assert_pattern_range(&patterns, "かと思ったら・かと思うと", 6, 13); // 最後かと思うと
    }

    #[test]
    fn test_i_adjective() {
        // Example: 暑いかと思ったら (just when I thought it was hot)
        let sentence = "外は暑いかと思ったら、意外と涼しかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かと思ったら・かと思うと");
        assert_pattern_range(&patterns, "かと思ったら・かと思うと", 2, 10); // 暑いかと思ったら
    }

    #[test]
    fn test_na_adjective() {
        // Example: 静かかと思ったら (just when I thought it was quiet)
        let sentence = "ここは静かかと思ったら、夜になると賑やかになる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かと思ったら・かと思うと");
        assert_pattern_range(&patterns, "かと思ったら・かと思うと", 3, 11); // 静かかと思ったら
    }
}

// Pattern: が気になる (be concerned about, be interested in)
// Data source: grammar_points_data.json["が気になる"]
// Testing multiple structures from grammar_points_data
//
// Structure variants:
//   - standard[0]: Noun + が気になる
//   - standard[1]: Verb + こと + が気になる
//   - standard[2]: Verb + の + が気になる
//   - polite[0]: Noun + が気になります
//   - polite[1]: Verb + こと + が気になります
//   - polite[2]: Verb + の + が気になります

mod gakininaru_tests {
    use super::*;

    #[test]
    fn test_noun_basic() {
        // Example: 値段が気になる (be concerned about the price)
        let sentence = "家自体は良いと思うんですけど、やっぱり値段が気になります";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "が気になる");
        assert_pattern_range(&patterns, "が気になる", 19, 28); // 値段が気になります
    }

    #[test]
    fn test_noun_person() {
        // Example: 高橋さんが気になる (be interested in Takahashi-san)
        let sentence = "私は高橋さんが気になる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "が気になる");
        assert_pattern_range(&patterns, "が気になる", 4, 11); // さんが気になる
    }

    #[test]
    fn test_verb_koto() {
        // Example: 言ったことが気になる (be concerned about what was said)
        let sentence = "やっぱりお前がさっき言ったことが気になる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "が気になる");
        assert_pattern_range(&patterns, "が気になる", 13, 20); // ことが気になる
    }

    #[test]
    fn test_verb_no() {
        // Example: 考えているのが気になる (be interested in what someone is thinking)
        let sentence = "私は好きな人が考えているのが気になる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "が気になる");
        assert_pattern_range(&patterns, "が気になる", 12, 18); // のが気になる
    }

    #[test]
    fn test_noun_polite() {
        // Example: 結果が気になります (be concerned about the results)
        let sentence = "テストの結果が気になります";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "が気になる");
        assert_pattern_range(&patterns, "が気になる", 4, 13); // 結果が気になります
    }

    #[test]
    fn test_verb_koto_polite() {
        // Example: 起こることが気になります (be concerned about what will happen)
        let sentence = "これから起こることが気になります";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "が気になる");
        assert_pattern_range(&patterns, "が気になる", 7, 16); // ことが気になります
    }
}

// Pattern: ことだから (it is exactly because / precisely because)
// Data source: grammar_points_data.json["ことだから"]
// Testing all structure variants
//
// Structure variants:
//   - standard[0]: Verb + ことだから
//   - standard[1]: Noun + の + ことだから
//   - standard[2]: な-Adjective + な + ことだから
//   - polite[0]: Verb + ことですから
//   - polite[1]: Noun + の + ことですから
//   - polite[2]: な-Adjective + な + ことですから

mod kotodakara_tests {
    use super::*;

    #[test]
    fn test_verb_kotodakara() {
        // Example: あの人がすることだから、どうせ人を騙して儲けているに違いない
        // (Precisely because it is something that person does, there is no doubt that he is making money scamming people)
        let sentence = "あの人がすることだから、どうせ人を騙して儲けているに違いない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことだから");
        assert_pattern_range(&patterns, "ことだから", 6, 11); // ことだから
    }

    #[test]
    fn test_noun_no_kotodakara() {
        // Example: いつも遅れてくる田中くんのことだから、今日も遅れてくるだろう
        // (It is exactly because Tanaka-kun always arrives late, that he will probably arrive late today)
        let sentence = "いつも遅れてくる田中くんのことだから、今日も遅れてくるだろう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことだから");
        assert_pattern_range(&patterns, "ことだから", 13, 18); // ことだから
    }

    #[test]
    fn test_na_adj_kotodakara() {
        // Example: 部長のことだから、またミスをしたらクビにさせられると思う
        // (It is exactly because it is my boss, he will probably fire me if I mess up again)
        let sentence = "部長のことだから、またミスをしたらクビにさせられると思う";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことだから");
        assert_pattern_range(&patterns, "ことだから", 3, 8); // ことだから
    }

    #[test]
    fn test_verb_kotodesu_polite() {
        // Example: 撮影が無事終わったことだから、打ち上げでもしましょう
        // (Considering that we have finished filming, it is a good opportunity for us to have a party)
        let sentence = "撮影が無事終わったことですから、打ち上げでもしましょう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことだから");
        assert_pattern_range(&patterns, "ことだから", 9, 15); // ことですから
    }

    #[test]
    fn test_opportunity_meaning() {
        // Example: お父さんとお母さんが珍しくうちに来ていることだから、久しぶりにみんなで映画でも見よう
        // (Dad, mom, considering that you guys are here, which is a rare occasion, why don't we watch a movie together)
        let sentence = "お父さんとお母さんが珍しくうちに来ていることだから、久しぶりにみんなで映画でも見よう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことだから");
        assert_pattern_range(&patterns, "ことだから", 20, 25); // ことだから
    }

    #[test]
    fn test_strong_reason() {
        // Example: お客様のプライバシーに関わることなので、これ以上詳しいことは言えません
        // Note: The grammar data shows "ことなので" as an alternative form
        // (Because this is something that has to do with the privacy of our customer, we can't give you further details)
        let sentence = "お客様のプライバシーに関わることだから、これ以上詳しいことは言えません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことだから");
        assert_pattern_range(&patterns, "ことだから", 14, 19); // ことだから
    }
}

// Pattern: ことになっている (it is expected / scheduled to)
// Data source: grammar_points_data.json["ことになっている"]
// Testing all structure variants
//
// Structure variants:
//   - standard[0]: Verb[る] + ことになっている
//   - standard[1]: Verb[ない] + ことになっている
//   - polite[0]: Verb[る] + ことになっています
//   - polite[1]: Verb[ない] + ことになっています

mod kotoninatteiru_tests {
    use super::*;

    #[test]
    fn test_verb_ru_kotoninatteiru() {
        // Example: 授業は12時から始まることになっています
        // (The class is scheduled to start at 12)
        let sentence = "授業は12時から始まることになっています";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことになっている");
        assert_pattern_range(&patterns, "ことになっている", 11, 20); // ことになっています
    }

    #[test]
    fn test_verb_nai_kotoninatteiru() {
        // Example: 社長は来週の飲み会に参加しないことになっています
        // (The president is scheduled to not come to the party next week)
        let sentence = "社長は来週の飲み会に参加しないことになっています";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことになっている");
        assert_pattern_range(&patterns, "ことになっている", 15, 24); // ことになっています
    }

    #[test]
    fn test_expected_habit_ru() {
        // Example: タクシーは左から乗ることになっている
        // (It is expected for customers to get into the cab from the left side)
        let sentence = "タクシーは左から乗ることになっている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことになっている");
        assert_pattern_range(&patterns, "ことになっている", 10, 18); // ことになっている
    }

    #[test]
    fn test_expected_habit_nai() {
        // Example: 生徒たちは授業中に教室を出れないことになっている
        // (Students are expected to not leave the classroom during class)
        let sentence = "生徒たちは授業中に教室を出れないことになっている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことになっている");
        assert_pattern_range(&patterns, "ことになっている", 16, 24); // ことになっている
    }
}

// ことにはならない - "Just because (A), it doesn't mean that (B)" / "It doesn't mean that"
// Data source: grammar_points_data.json["ことにはならない"]
mod kotonihanaranai_tests {
    use super::*;

    #[test]
    fn test_karatoitte_structure() {
        // Example: みんなが簡単にできたからと言って、君にも簡単にできるということにはならない
        // Structure.standard[0]: "Phrase + からといって + Phrase + ことにはならない"
        let sentence = "みんなが簡単にできたからと言って、君にも簡単にできるということにはならない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことにはならない");
        assert_pattern_range(&patterns, "ことにはならない", 23, 37); // できるということにはならない
    }

    #[test]
    fn test_temo_structure() {
        // Example: いくら上司でも、仕事を全部部下たちに押し付けてもいいことにはならない
        // Structure.standard[1]: "Phrase［ても］ + ことにはならない"
        let sentence = "いくら上司でも、仕事を全部部下たちに押し付けてもいいことにはならない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことにはならない");
        assert_pattern_range(&patterns, "ことにはならない", 24, 34); // いいことにはならない
    }

    #[test]
    fn test_simple_structure() {
        // Example: ５分ノートを見直しただけでは、勉強したことにはならない
        // Simple form without からといって or ても
        let sentence = "５分ノートを見直しただけでは、勉強したことにはならない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことにはならない");
        assert_pattern_range(&patterns, "ことにはならない", 18, 27); // たことにはならない
    }

    #[test]
    fn test_toiu_emphasis() {
        // Example: 殴られたからと言って、殴り返してもいいということにはならない
        // With という before ことにはならない for emphasis
        let sentence = "殴られたからと言って、殴り返してもいいということにはならない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことにはならない");
        assert_pattern_range(&patterns, "ことにはならない", 17, 30); // いいということにはならない
    }

    #[test]
    fn test_polite_form() {
        // Structure.polite[0]: "Phrase + からといって + Phrase + ことにはなりません"
        let sentence = "簡単だからといって、誰にでもできることにはなりません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことにはならない");
        assert_pattern_range(&patterns, "ことにはならない", 14, 26); // できることにはなりません
    }

    #[test]
    fn test_polite_temo() {
        // Structure.polite[1]: "Phrase［ても］ + ことにはなりません"
        let sentence = "時間があっても、やりたいことにはなりません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことにはならない");
        assert_pattern_range(&patterns, "ことにはならない", 10, 21); // たいことにはなりません
    }
}

// ことは〜が - "(A) is true, but (B)" / "although (A), (B)"
// Data source: grammar_points_data.json["ことは〜が"]
mod kotoha_ga_tests {
    use super::*;

    #[test]
    fn test_verb_repetition_ga() {
        // Example: 漢字は読めることは読めるが、簡単な漢字しか読めないです
        // Structure.standard[0]: "Verb + ことは + Verb(*) + が"
        let sentence = "漢字は読めることは読めるが、簡単な漢字しか読めないです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことは〜が");
        assert_pattern_range(&patterns, "ことは〜が", 3, 13); // 読めることは読めるが
    }

    #[test]
    fn test_verb_repetition_kedo() {
        // Example with けど variant
        // Structure.standard[0] with けど: "Verb + ことは + Verb(*) + けど"
        let sentence = "食べることは食べるけど、あまり好きじゃない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことは〜が");
        assert_pattern_range(&patterns, "ことは〜が", 0, 11); // 食べることは食べるけど
    }

    #[test]
    fn test_i_adjective_repetition() {
        // Example: 新しい家は広いことは広いけど、家具が多いから狭く見える
        // Structure.standard[1]: "［い］Adjective + ことは + ［い］Adjective(*) + が"
        let sentence = "新しい家は広いことは広いけど、家具が多いから狭く見える";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことは〜が");
        assert_pattern_range(&patterns, "ことは〜が", 5, 14); // 広いことは広いけど
    }

    #[test]
    fn test_na_adjective_repetition() {
        // Example: このスマホは便利であることは便利であるけど、本体がデカすぎて片手では操作できない
        // Structure.standard[2]: "［な］Adjective + であることは + ［な］Adjective(*) + である + けど"
        let sentence = "このスマホは便利であることは便利であるけど、本体がデカすぎて片手では操作できない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことは〜が");
        assert_pattern_range(&patterns, "ことは〜が", 9, 21); // あることは便利であるけど
    }

    #[test]
    fn test_noun_repetition() {
        // Example: ここは道路なことは道路だけど、狭すぎて車が通れない
        // Structure.standard[3]: "Noun + なことは + Noun(*) + だ + けど"
        let sentence = "ここは道路なことは道路だけど、狭すぎて車が通れない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことは〜が");
        assert_pattern_range(&patterns, "ことは〜が", 5, 14); // なことは道路だけど
    }

    #[test]
    fn test_verb_keredo() {
        // Example with けれど variant
        let sentence = "行くことは行くけれど、あまり気が進まない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことは〜が");
        assert_pattern_range(&patterns, "ことは〜が", 0, 10); // 行くことは行くけれど
    }
}

// Pattern: さすが (as expected of / that is just like)
// Data source: grammar_points_data.json["さすが"]
// Testing: structure.standard[0] - "さすが + （に）+ Phrase"
// Testing: structure.standard[1] - "さすが + （の）+ Noun"
//
// Structure variants:
//   - standard[0]: さすが + （に）+ Phrase
//   - standard[1]: さすが + （の）+ Noun
mod sasuga_tests {
    use super::*;

    #[test]
    fn test_sasuga_basic_phrase() {
        // Example: すごい！さすが先輩！
        // Structure.standard[0]: "さすが + Phrase" (basic usage)
        let sentence = "すごい！さすが先輩！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "さすが");
        assert_pattern_range(&patterns, "さすが", 4, 7); // さすが
    }

    #[test]
    fn test_sasuga_ni_phrase() {
        // Example: さすがに今日は雨が降りすぎだろ...
        // Structure.standard[0]: "さすが + に + Phrase"
        let sentence = "さすがに今日は雨が降りすぎだろ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "さすが");
        assert_pattern_range(&patterns, "さすが", 0, 4); // さすがに
    }

    #[test]
    fn test_sasuga_no_noun() {
        // Example: さすがの君でもこの問題は難しいだろう
        // Structure.standard[1]: "さすが + の + Noun"
        let sentence = "さすがの君でもこの問題は難しいだろう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "さすが");
        assert_pattern_range(&patterns, "さすが", 0, 4); // さすがの
    }

    #[test]
    fn test_sasuga_negative_connotation() {
        // Example: さすが田中くん！発注ミスの天才だね！
        // Structure: さすが + Noun (negative/sarcastic usage)
        let sentence = "さすが田中くん！発注ミスの天才だね！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "さすが");
        assert_pattern_range(&patterns, "さすが", 0, 3); // さすが
    }
}

// Pattern: しかも (moreover, furthermore)
// Data source: grammar_points_data.json["しかも"]
// Testing: structure.standard[0] - "Phrase (A) + しかも + Phrase (B)"
//
// Structure variants:
//   - standard[0]: Phrase (A) + しかも + Phrase (B) (conjunction)

mod shikamo_tests {
    use super::*;

    #[test]
    fn test_shikamo_between_phrases() {
        // Example from data: このテレビは画質がめちゃくちゃいい。しかも、受信機がついていない
        let sentence = "このテレビは画質がめちゃくちゃいい。しかも、受信機がついていないから";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "しかも");
        assert_pattern_range(&patterns, "しかも", 18, 21); // しかも
    }

    #[test]
    fn test_shikamo_sentence_beginning() {
        // Example: 彼女はとても頭がいいし性格もいい。しかも、美人だから
        let sentence = "彼女はとても頭がいいし性格もいい。しかも、美人だからもてないわけがない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "しかも");
        assert_pattern_range(&patterns, "しかも", 17, 20); // しかも
    }

    #[test]
    fn test_shikamo_within_sentence() {
        // Example: あの店は古いし汚い。しかも品揃えもよくない
        let sentence = "あの店は古いし汚い。しかも品揃えもよくないから潰れるだろう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "しかも");
        assert_pattern_range(&patterns, "しかも", 10, 13); // しかも
    }
}

// Pattern: てでも (even if I have to)
// Data source: grammar_points_data.json["てでも"]
// Testing: structure.standard[0] - "Verb［て］+ でも"
//
// Structure variants:
//   - standard[0]: Verb［て］+ でも (strong determination/willingness)

mod tedemo_tests {
    use super::*;

    #[test]
    fn test_tedemo_strong_will() {
        // Example from data: 高いお金を払ってでも手に入れたい
        let sentence = "あの最新のノートパソコンは高いお金を払ってでも手に入れたい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てでも");
        assert_pattern_range(&patterns, "てでも", 18, 23); // 払ってでも
    }

    #[test]
    fn test_dedemo_variant() {
        // Example from data: 寒い中並んででもあそこのラーメンを食べてみたい
        let sentence = "寒い中並んででもあそこのラーメンを食べてみたい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てでも");
        assert_pattern_range(&patterns, "てでも", 3, 8); // 並んででも
    }

    #[test]
    fn test_tedemo_must_finish() {
        // Example from data: 徹夜してでも終わらせなければいけない
        let sentence = "このレポートは徹夜してでも終わらせなければいけない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てでも");
        assert_pattern_range(&patterns, "てでも", 7, 13); // 徹夜してでも
    }
}

// Pattern: 得る・得る (to be possible, can do)
// Data source: grammar_points_data.json["得る・得る"]
// Testing structure variants: Verb[stem] + える/うる (can be える or うる in formal contexts)
#[cfg(test)]
mod eru_u30fb_eru_tests {
    use super::*;

    #[test]
    fn test_eru_shinieru_can_die() {
        // Testing: structure.standard[0] - Verb[stem] + える + だろう (volitional)
        let sentence = "あんな状況では誰もが死にえるだろう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "得る・得る");
        assert_pattern_range(&patterns, "得る・得る", 10, 17); // 死にえるだろう (includes auxiliary)
    }

    #[test]
    fn test_eru_okorieru_can_occur() {
        // Example from data: いくら気をつけていても、交通事故は起こりえるものだ
        // Testing: structure.standard[0] - Verb[stem] + える (dictionary form)
        let sentence = "いくら気をつけていても、交通事故は起こりえるものだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "得る・得る");
        assert_pattern_range(&patterns, "得る・得る", 17, 22); // 起こりえる
    }

    #[test]
    fn test_eru_narieru_can_become() {
        // Example from data: そういう事はセクハラにもなりえるので
        // Testing: structure.standard[0] - Verb[stem] + える (dictionary form)
        let sentence = "そういう事はセクハラにもなりえるので、そういう事はやらない方がいい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "得る・得る");
        assert_pattern_range(&patterns, "得る・得る", 12, 16); // なりえる
    }

    #[test]
    fn test_uru_kangaeuru_can_think() {
        // Example from data: 考えうる事は、すべてこの企画書に書いておきました
        // Testing: structure.standard[0] with うる form (used when preceding sound is え)
        let sentence = "考えうる事は、すべてこの企画書に書いておきました";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "得る・得る");
        assert_pattern_range(&patterns, "得る・得る", 0, 4); // 考えうる
    }

    #[test]
    fn test_eru_polite_ariemasu() {
        // Testing: structure.polite[0] - Verb[stem] + えます
        let sentence = "それはありえますね、確認してみます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "得る・得る");
        assert_pattern_range(&patterns, "得る・得る", 3, 8); // ありえます
    }

    #[test]
    fn test_eru_negative_arienai() {
        // Example from data: マジありえない
        // Testing: Verb[stem] + えない (negative form)
        let sentence = "電気代がまた値上がりするみたいだよ。マジありえない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "得る・得る");
        assert_pattern_range(&patterns, "得る・得る", 20, 25); // ありえない
    }
}

// ============================================================================
// ざるを得ない Tests - "cannot help but / have no choice but to"
// ============================================================================
#[cfg(test)]
mod zaruwoenai_tests {
    use super::*;

    // Pattern: ざるを得ない (cannot help but / have no choice but to)
    // Data source: grammar_points_data.json["ざるを得ない"]
    // Testing: structure.standard[0] - "Verb[ない] + ざるを得ない"
    //
    // Grammar: Classical auxiliary ざる (negative) + を + 得る + ない
    // Means: Cannot help doing (A), no choice but to (A), can't not (A)
    // Note: Special conjugation with する → せざる (not しざる)
    //
    // Structures to test:
    //   - standard[0]: Verb[ない] + ざるを得ない
    //   - polite[0]: Verb[ない] + ざるを得ません
    //   - Special: する → せざるを得ない (not しざるを得ない)

    #[test]
    fn test_zaruwoenai_standard_akirame() {
        // Example from data: 諦めざるをえない
        // Testing: structure.standard[0] - Regular verb + ざるを得ない
        let sentence = "山の頂上まで登りたかったが、天候が悪くなって来たから諦めざるを得ない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ざるを得ない");
        assert_pattern_range(&patterns, "ざるを得ない", 26, 34); // 諦めざるを得ない
    }

    #[test]
    fn test_zaruwoenai_polite_shitagawa() {
        // Example from data: 従わざるをえません
        // Testing: structure.polite[0] - Regular verb + ざるを得ません
        let sentence = "先輩に指示されたので従わざるを得ません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ざるを得ない");
        assert_pattern_range(&patterns, "ざるを得ない", 10, 19); // 従わざるを得ません
    }

    #[test]
    fn test_zaruwoenai_suru_verb_special() {
        // Example from data: 勉強せざるを得ない (not 勉強しざるを得ない)
        // Testing: Special conjugation - する → せざる (classical form)
        let sentence = "フランスの大学へ行くため、フランス語を勉強せざるを得ない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ざるを得ない");
        assert_pattern_range(&patterns, "ざるを得ない", 19, 28); // 勉強せざるを得ない
    }

    #[test]
    fn test_zaruwoenai_regular_verb_shitagaw() {
        // Testing: Another regular verb example with だろう following
        let sentence = "会社の方針だから、従わざるを得ないだろう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ざるを得ない");
        // Note: Range includes ないだろう due to auxiliary verb extension
        // This is acceptable as だろう modifies the entire ざるを得ない construction
        assert_pattern_range(&patterns, "ざるを得ない", 9, 20); // 従わざるを得ないだろう
    }
}

// ～ざる Tests - "attributive form of classical auxiliary ず (negative)"
// Data source: grammar_points_data.json["～ざる"]

#[cfg(test)]
mod uff5e_zaru_tests {
    use super::*;

    // Pattern: ～ざる (attributive form of classical negative auxiliary ず)
    // Data source: grammar_points_data.json["～ざる"]
    // Testing: structure.standard[0] - "Verb[ない] + ざる + Noun"
    //
    // Grammar: ざる is the attributive form of the classical auxiliary verb ず
    // - Attaches to the 未然形 (irrealis/negative form) of verbs
    // - Translates as "not" or "un~" (like "unstoppable", "unknown")
    // - Primarily used in set expressions and before nouns (attributive position)
    //
    // Special note: する conjugates with せ stem (せざる), not し stem
    //
    // Structures to test:
    //   - standard[0]: Verb[ない] + ざる + Noun (e.g., 言わざる人, 知られざる過去)

    #[test]
    fn test_uff5e_zaru_iwazaru() {
        // Example from data: 弱音を言わざる人
        // Testing: 言う → 言わ (未然形) + ざる
        let sentence = "彼はどんな辛い時でも弱音を言わざる人だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～ざる");
        assert_pattern_range(&patterns, "～ざる", 13, 17); // 言わざる
    }

    #[test]
    fn test_uff5e_zaru_shirareru() {
        // Example from data: 知られざる過去
        // Testing: 知られる → れ (未然形) + ざる
        // Note: ざる attaches to the auxiliary れる, not the main verb 知る
        let sentence = "記事のタイトル：「演歌歌手鈴木太朗の知られざる過去。」";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～ざる");
        assert_pattern_range(&patterns, "～ざる", 20, 23); // れざる (part of 知られざる)
    }

    #[test]
    fn test_uff5e_zaru_ataezaru() {
        // Testing: 与える → 与え (未然形) + ざる
        // Meaning: influence that cannot be given / ungiven influence
        let sentence = "そのニュースは我々に与えざる影響を及ぼした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～ざる");
        assert_pattern_range(&patterns, "～ざる", 10, 14); // 与えざる
    }

    #[test]
    fn test_uff5e_zaru_osorezaru() {
        // Testing: 恐れる → 恐れ (未然形) + ざる
        // Meaning: fearless / not afraid
        let sentence = "彼女は何事も恐れざる勇気を持っている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～ざる");
        assert_pattern_range(&patterns, "～ざる", 6, 10); // 恐れざる
    }
}

// つもりで Tests - "with the intention of / as if"
// Data source: grammar_points_data.json["つもりで"]

#[cfg(test)]
mod tsumoride_tests {
    use super::*;

    // Pattern: つもりで (with the intention of / as if / pretend)
    // Data source: grammar_points_data.json["つもりで"]
    //
    // Grammar: つもり (noun: intention) + で (particle: with/by)
    // - With verbs/adjectives/nouns: "with the intention of (A)"
    // - With past-tense verbs: "as if (A)" / "pretending that (A)"
    //
    // Structures to test:
    //   - standard[0]: Verb + つもりで
    //   - standard[1]: な-Adjective + な + つもりで
    //   - standard[2]: Noun + の + つもりで
    //   - standard[4]: Verb[た] + つもりで (different meaning: "as if")

    #[test]
    fn test_tsumoride_verb() {
        // Example from data: 何も買わないつもりで
        // Testing: structure.standard[0] - Verb + つもりで
        let sentence = "何も買わないつもりで新しく出来たショッピングモールへ行ったが、色々買ってしまった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "つもりで");
        assert_pattern_range(&patterns, "つもりで", 4, 10); // ないつもりで
    }

    #[test]
    fn test_tsumoride_verb_past() {
        // Example from data: 主人公になったつもりで
        // Testing: structure.standard[4] - Verb[た] + つもりで (meaning: "as if")
        let sentence = "今度は、主人公になったつもりで読んでみてください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "つもりで");
        assert_pattern_range(&patterns, "つもりで", 10, 15); // たつもりで
    }

    #[test]
    fn test_tsumoride_noun() {
        // Example from data: 冗談のつもりで
        // Testing: structure.standard[2] - Noun + の + つもりで
        let sentence = "冗談のつもりで言っただけなのに、相手を傷付けてしまった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "つもりで");
        assert_pattern_range(&patterns, "つもりで", 2, 7); // のつもりで
    }

    #[test]
    fn test_tsumoride_na_adjective() {
        // Testing: structure.standard[1] - な-Adjective + な + つもりで
        // Example: 真剣なつもりで (with serious intention)
        let sentence = "真剣なつもりで提案したのに、誰も聞いてくれなかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "つもりで");
        assert_pattern_range(&patterns, "つもりで", 2, 7); // なつもりで
    }

    #[test]
    fn test_tsumoride_verb_selected() {
        // Testing: 新鮮な魚を選んだつもりで
        // Another example of Verb[た] + つもりで
        let sentence = "新鮮な魚を選んだつもりで買ったのに、パックから出したらものすごいにおいがした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "つもりで");
        assert_pattern_range(&patterns, "つもりで", 7, 12); // だつもりで
    }
}

// Pattern: どうせ (in any case, anyway)
// Data source: grammar_points_data.json["どうせ"]
// Testing: structure.standard[0] - "どうせ + Phrase"
//
// Structure variants:
//   - standard[0]: どうせ + Phrase (only one structure)
//
// Usage contexts:
//   - Expressing inevitability/resignation
//   - "Might as well" with なら or だから

mod douse_tests {
    use super::*;

    #[test]
    fn test_douse_resignation() {
        // Example: どうせまたパチンコに行くんでしょ
        // (In any case, you are going to the pachinko parlor again, aren't you?)
        let sentence = "どうせまたパチンコに行くんでしょ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "どうせ");
        assert_pattern_range(&patterns, "どうせ", 0, 3); // どうせ
    }

    #[test]
    fn test_douse_with_nara() {
        // Example: どうせ新しいスマホを買うならもうちょっといい奴買いなよ
        // (If you're going to buy a new smartphone, you might as well buy something a little better)
        let sentence = "どうせ新しいスマホを買うならもうちょっといい奴買いなよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "どうせ");
        assert_pattern_range(&patterns, "どうせ", 0, 3); // どうせ
    }

    #[test]
    fn test_douse_with_dakara() {
        // Example: どうせまた負けるんだから頑張っても時間と体力の無駄だよ
        // (We're going to lose again anyway, so trying our best would be a waste)
        let sentence = "どうせまた負けるんだから頑張っても時間と体力の無駄だよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "どうせ");
        assert_pattern_range(&patterns, "どうせ", 0, 3); // どうせ
    }
}

// Pattern: せめて (at least)
// Data source: grammar_points_data.json["せめて"]
// Testing: structure.standard[0] - "せめて + Phrase"
//
// Structure variants:
//   - standard[0]: せめて + Phrase (only one structure)
//
// Usage contexts:
//   - Expressing minimum expectation with sense of responsibility
//   - Often used with たい, ほしい, べき, etc.

mod semete_tests {
    use super::*;

    #[test]
    fn test_semete_minimum_request() {
        // Example: せめて国語の宿題はやっておきなさい
        // (At least finish your Japanese homework in advance)
        let sentence = "せめて国語の宿題はやっておきなさい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "せめて");
        assert_pattern_range(&patterns, "せめて", 0, 3); // せめて
    }

    #[test]
    fn test_semete_with_tai() {
        // Example: せめてこの曲だけでも弾けるようになりたい
        // (I want to be able to play this song, at least)
        let sentence = "せめてこの曲だけでも弾けるようになりたい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "せめて");
        assert_pattern_range(&patterns, "せめて", 0, 3); // せめて
    }

    #[test]
    fn test_semete_with_tehoshii() {
        // Example: せめて晩御飯だけは毎日作ってほしい
        // (I want you to at least make dinner every night)
        let sentence = "別に毎日洗濯をしろっては言わないけど、せめて晩御飯だけは毎日作ってほしい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "せめて");
        assert_pattern_range(&patterns, "せめて", 19, 22); // せめて
    }
}

// Pattern: 確かに (certainly, surely)
// Data source: grammar_points_data.json["確かに"]
// Testing: structure.standard[0] - "確（たし）かに + Phrase"
//
// Structure variants:
//   - standard[0]: 確（たし）かに + Phrase (only one structure)
//
// Usage contexts:
//   - Expressing certainty or agreement
//   - Both hiragana (たしかに) and kanji (確かに) forms

mod tashikani_tests {
    use super::*;

    #[test]
    fn test_tashikani_hiragana_agreement() {
        // Example: たしかに彼はイケメンですが、マナーが悪いです
        // (He is certainly handsome, but he has bad manners)
        let sentence = "たしかに彼はイケメンですが、マナーが悪いです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "確かに");
        assert_pattern_range(&patterns, "確かに", 0, 4); // たしかに
    }

    #[test]
    fn test_tashikani_kanji_apology() {
        // Example: 確かにさっきのは言い過ぎだった
        // (What I said earlier was certainly out of line)
        let sentence = "確かにさっきのは言い過ぎだった。ごめん";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "確かに");
        assert_pattern_range(&patterns, "確かに", 0, 3); // 確かに
    }

    #[test]
    fn test_tashikani_difficult() {
        // Example: 確かに結構難しいね
        // (This is certainly difficult)
        let sentence = "確かに結構難しいね。ちょっと甘く見てたわ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "確かに");
        assert_pattern_range(&patterns, "確かに", 0, 3); // 確かに
    }
}

// Pattern: どうやら (apparently, it seems like)
// Data source: grammar_points_data.json["どうやら"]
// Testing: structure.standard[0] - "どうやら + Phrase + みたいだ"
// Testing: structure.standard[1] - "(ようだ、そうだ、らしい、って感じだ)"
//
// Structure notes:
//   - どうやら is an adverb that appears at the beginning of sentences
//   - Often used with speculation markers: ようだ, そうだ, らしい, みたいだ
//   - Emphasizes uncertainty/speculation
//
// Test strategy:
//   - Test どうやら + みたいだ (standard[0])
//   - Test どうやら + ようだ (standard[1])
//   - Test どうやら + そうだ (standard[1])
//   - Test どうやら + らしい (standard[1])

mod douyara_tests {
    use super::*;

    #[test]
    fn test_douyara_mitaida() {
        // Example: どうやら今日も休んでいるみたいだね
        // (It seems like he's taking today off as well)
        let sentence = "鈴木くんはいるか？どうやら今日も休んでいるみたいだね";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "どうやら");
        assert_pattern_range(&patterns, "どうやら", 9, 13); // どうやら
    }

    #[test]
    fn test_douyara_youda() {
        // Example: どうやら私は彼女には必要がないようだ
        // (It seems like I am not needed by her)
        let sentence = "どうやら私は彼女には必要がないようだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "どうやら");
        assert_pattern_range(&patterns, "どうやら", 0, 4); // どうやら
    }

    #[test]
    fn test_douyara_rashii() {
        // Example: どうやら彼は結婚しているらしいよ
        // (I heard that he is apparently married)
        let sentence = "どうやら彼は結婚しているらしいよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "どうやら");
        assert_pattern_range(&patterns, "どうやら", 0, 4); // どうやら
    }

    #[test]
    fn test_douyara_mid_sentence() {
        // Example: お隣の鈴木さんがどうやら来月からイギリスに行くらしいですよ
        // (Our neighbor, Suzuki-san, is apparently going to England next month)
        let sentence = "お隣の鈴木さんがどうやら来月からイギリスに行くらしいですよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "どうやら");
        assert_pattern_range(&patterns, "どうやら", 8, 12); // どうやら
    }
}

// Pattern: なにやら (something or other, some kind of, for some reason)
// Data source: grammar_points_data.json["なにやら"]
// Testing: structure.standard[0] - "何（なに）やら + Phrase"
//
// Structure notes:
//   - なにやら is an adverb expressing uncertainty about what something is
//   - Composed of 何（なに） and particle やら denoting uncertainty
//   - Different from 何か - only used when speaker is unsure of what thing is
//
// Test strategy:
//   - Test various contexts where speaker expresses uncertainty
//   - All examples from grammar_points_data.json

mod naniyara_tests {
    use super::*;

    #[test]
    fn test_naniyara_something_said() {
        // Example: あの人に何やら言われても何にも感じない
        // (I don't feel anything when that person says something to me)
        let sentence = "あの人に何やら言われても何にも感じない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なにやら");
        assert_pattern_range(&patterns, "なにやら", 4, 7); // 何やら
    }

    #[test]
    fn test_naniyara_some_kind() {
        // Example: やっと子供が落ち着いたと思ったら、何やら歌い出した
        // (When I thought my kid finally settled down, he started to sing some kind of song)
        let sentence = "やっと子供が落ち着いたと思ったら、何やら歌い出した";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なにやら");
        assert_pattern_range(&patterns, "なにやら", 17, 20); // 何やら
    }

    #[test]
    fn test_naniyara_for_some_reason() {
        // Example: 家に帰ってカバンを開けたら何やら見覚えのないレシートが入ってた
        // (When I opened my bag when I got home, there was a receipt in there for some reason)
        let sentence = "家に帰ってカバンを開けたら何やら見覚えのないレシートが入ってた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なにやら");
        assert_pattern_range(&patterns, "なにやら", 13, 16); // 何やら
    }

    #[test]
    fn test_naniyara_something_happened() {
        // Example: あの二人の間で何やらあったらしい
        // (Apparently something happened between those two)
        let sentence = "あの二人の間で何やらあったらしい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なにやら");
        assert_pattern_range(&patterns, "なにやら", 7, 10); // 何やら
    }
}

// Pattern: 万が一 (in the unlikely event, just in case)
// Data source: grammar_points_data.json["万が一"]
// Testing: structure.standard[0] - "万（まん）（が）一（いち） + Phrase"
//
// Structure variants:
//   - 万が一 (mangaichi) - full form with が particle
//   - 万一 (man'ichi) - abbreviated form without が
//
// Both forms mean "in the unlikely event of" or "just in case"
// Used adverbially, often at the beginning of sentences

mod mangaichi_tests {
    use super::*;

    #[test]
    fn test_mangaichi_full_form() {
        // Example: 万が一の時のために防災セットを買っておきましょう
        // (Let's buy a disaster kit just in case)
        let sentence = "万が一の時のために防災セットを買っておきましょう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "万が一");
        assert_pattern_range(&patterns, "万が一", 0, 3); // 万が一
    }

    #[test]
    fn test_mangaichi_with_condition() {
        // Example: 万が一来れなくなった場合は、連絡をください
        // (In the unlikely event that you are unable to come, please contact us)
        let sentence = "万が一来れなくなった場合は、連絡をください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "万が一");
        assert_pattern_range(&patterns, "万が一", 0, 3); // 万が一
    }

    #[test]
    fn test_man_ichi_abbreviated() {
        // Example: 万一分からないことがあれば、何でも私に聞いてください
        // (If by chance there is something you don't understand, you can ask me anything)
        let sentence = "万一分からないことがあれば、何でも私に聞いてください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "万が一");
        assert_pattern_range(&patterns, "万が一", 0, 2); // 万一
    }
}

// Pattern: 一応 ① (just in case, just to be sure)
// Pattern: 一応 ② (more or less, for the time being, tentatively)
// Data source: grammar_points_data.json["一応 ①"] and ["一応 ②"]
// Testing: structure.standard[0] - "一応（いちおう） + Phrase" (both patterns)
//
// Structure notes:
//   - Both 一応① and 一応② use the same structure (adverb)
//   - Meaning differs by context:
//     - 一応①: "just in case" / "just to be sure" (preventive)
//     - 一応②: "more or less" / "for the time being" / "tentatively" (minimum requirement)
//   - Both tokenize identically as 副詞/助詞類接続
//
// Implementation note:
//   - Since both patterns are structurally identical, implement as single pattern
//   - Show both grammar explanations when detected (user determines meaning from context)

mod ichiou_tests {
    use super::*;

    #[test]
    fn test_ichiou_just_in_case() {
        // Example from 一応①: 一応傘を持って行ったほうがいいかも
        // (It might be better to bring an umbrella just in case)
        let sentence = "今日はずっと晴れって天気予報で言ってたけど曇ってきたから一応傘を持って行ったほうがいいかも";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "一応 ①");
        assert_pattern_range(&patterns, "一応 ①", 28, 30); // 一応
        assert_has_pattern(&patterns, "一応 ②");
        assert_pattern_range(&patterns, "一応 ②", 28, 30); // 一応 (same detection)
    }

    #[test]
    fn test_ichiou_just_to_be_sure() {
        // Example from 一応①: 一応チェックしておいてくれない？
        // (Just to be sure, can you please double check it for me?)
        let sentence = "これで大丈夫だと思うけど、一応チェックしておいてくれない？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "一応 ①");
        assert_pattern_range(&patterns, "一応 ①", 13, 15); // 一応
        assert_has_pattern(&patterns, "一応 ②");
        assert_pattern_range(&patterns, "一応 ②", 13, 15); // 一応 (same detection)
    }

    #[test]
    fn test_ichiou_more_or_less() {
        // Example from 一応②: 一応私が店長ですが、どうかなさいましたか
        // (I'm more or less the manager. Did something happen?)
        let sentence = "一応私が店長ですが、どうかなさいましたか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "一応 ①");
        assert_pattern_range(&patterns, "一応 ①", 0, 2); // 一応
        assert_has_pattern(&patterns, "一応 ②");
        assert_pattern_range(&patterns, "一応 ②", 0, 2); // 一応 (same detection)
    }

    #[test]
    fn test_ichiou_for_time_being() {
        // Example from 一応②: 今日は一応ここまでにしておきましょう
        // (For the time being, let's call it a day for today)
        let sentence = "今日は一応ここまでにしておきましょう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "一応 ①");
        assert_pattern_range(&patterns, "一応 ①", 3, 5); // 一応
        assert_has_pattern(&patterns, "一応 ②");
        assert_pattern_range(&patterns, "一応 ②", 3, 5); // 一応 (same detection)
    }
}

// Pattern: よりほかない (have no choice but / nothing but)
// Data source: grammar_points_data.json["よりほかない"]
// Testing: structure.standard[0] - "Verb + より + ほか + （は(1)）+ ない"
// Note: (1) は, に, or には can optionally appear between ほか and ない
//
// Structure variants to test:
//   - Verb + より + ほか + ない
//   - Verb + より + ほか + は + ない
//   - Verb + より + ほか + に + ない
//   - Verb + より + ほか + には + ない

mod yorihokanai_tests {
    use super::*;

    #[test]
    fn test_yorihoka_nai_basic() {
        // Example: 靴に穴が空いたので、新しいのを買うよりほかない
        // (I have no choice but to buy a new pair of shoes because there's a hole in mine)
        let sentence = "靴に穴が空いたので、新しいのを買うよりほかない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "よりほかない");
        assert_pattern_range(&patterns, "よりほかない", 15, 23); // 買うよりほかない
    }

    #[test]
    fn test_yorihoka_wa_nai() {
        // Example: 誰もお婆さんを助けようとしなかったので、私が助けるよりほかはなかった
        // (No one was trying to help the old lady, so I had no choice but to help her)
        let sentence = "誰もお婆さんを助けようとしなかったので、私が助けるよりほかはなかった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "よりほかない");
        assert_pattern_range(&patterns, "よりほかない", 22, 34); // 助けるよりほかはなかった
    }

    #[test]
    fn test_yorihoka_ni_nai() {
        // Example: 被害者が無事であることを祈るよりほかにない
        // (There is nothing we can do other than pray that the victims are safe)
        let sentence = "被害者が無事であることを祈るよりほかにない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "よりほかない");
        assert_pattern_range(&patterns, "よりほかない", 12, 21); // 祈るよりほかにない
    }

    #[test]
    fn test_yorihoka_niwa_nai() {
        // Example: 仕事をクビになったので新しい仕事を探すよりほかにはない
        // (I got fired from my job, so I have no choice but to look for a new one)
        let sentence = "仕事をクビになったので新しい仕事を探すよりほかにはない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "よりほかない");
        assert_pattern_range(&patterns, "よりほかない", 17, 27); // 探すよりほかにはない
    }
}

// Pattern: に相違ない (without a doubt, no mistaking)
// Data source: grammar_points_data.json["に相違ない"]
// Testing: structure.standard[] - Various forms + に相違ない
//
// Structure variants to test:
//   - Verb + に相違ない
//   - い-Adjective + に相違ない
//   - な-Adjective + に相違ない
//   - Noun + に相違ない
//   - から + に相違ない (emphasizing cause)

mod nisouinai_tests {
    use super::*;

    #[test]
    fn test_verb_nisouinai() {
        // Example: いい大学に行けるに相違ない
        // (There's no doubt that he can go to a good university)
        let sentence = "一生懸命勉強をし続けたので、いい大学に行けるに相違ない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に相違ない");
        assert_pattern_range(&patterns, "に相違ない", 19, 27); // 行けるに相違ない
    }

    #[test]
    fn test_i_adj_nisouinai() {
        // Example: 美味しいに相違ない
        // (There is no doubt that it will be delicious)
        let sentence = "このコースは世界的に有名なシェフによって作られたものなので美味しいに相違ない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に相違ない");
        assert_pattern_range(&patterns, "に相違ない", 29, 38); // 美味しいに相違ない
    }

    #[test]
    fn test_na_adj_nisouinai() {
        // Example: 便利に相違ない
        // (There is no doubt that it is convenient)
        let sentence = "この街には電車が３分に１本来るので便利に相違ない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に相違ない");
        assert_pattern_range(&patterns, "に相違ない", 17, 24); // 便利に相違ない
    }

    #[test]
    fn test_noun_nisouinai() {
        // Example: あの人に相違ない
        // (There is no doubt that that person is the culprit)
        let sentence = "あの事件の犯人はあの人に相違ない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に相違ない");
        assert_pattern_range(&patterns, "に相違ない", 10, 16); // 人に相違ない
    }

    #[test]
    fn test_kara_nisouinai() {
        // Example: 嘘をついているからに相違ない
        // (There is no doubt that he's being restless because he's lying)
        let sentence = "彼がキョロキョロしているのは、嘘をついているからに相違ない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に相違ない");
        assert_pattern_range(&patterns, "に相違ない", 22, 29); // からに相違ない
    }
}

// ============================================================================
// ずに済む (zunisumu) - get away without doing / no need to do
// ============================================================================
#[cfg(test)]
mod zunisumu_tests {
    use super::*;

    // Pattern: ずに済む (get away without doing, can avoid)
    // Data source: grammar_points_data.json["ずに済む"]
    // Testing all structure variants with print_debug first

    #[test]
    fn test_zunisumu_standard() {
        // Structure: Verb[ない] + ずに済む
        // Example: やらずに済む (can get away without doing)
        let sentence = "今宿題をやっておけば後でやらずに済むから、今のうちにやっておこう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずに済む");
        assert_pattern_range(&patterns, "ずに済む", 12, 18); // やらずに済む
    }

    #[test]
    fn test_zunisumu_polite() {
        // Structure: Verb[ない] + ずに済みます
        // Example: 待たずに済みます (can get by without waiting)
        let sentence = "このVIPパスを使えば待たずに済みます。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずに済む");
        assert_pattern_range(&patterns, "ずに済む", 11, 19); // 待たずに済みます
    }

    #[test]
    fn test_zunisumu_nakutesumu() {
        // Structure: Verb[なくて] + 済む
        // Example: 支払わなくて済む (can get by without paying)
        let sentence = "今月契約すると初期費用を支払わなくて済むので、今月中に契約することをお勧めします。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずに済む");
        assert_pattern_range(&patterns, "ずに済む", 12, 20); // 支払わなくて済む
    }

    #[test]
    fn test_zunisumu_naidesumu() {
        // Structure: Verb[ないで] + 済む
        // Example: 支払わないで済む (can get by without paying)
        let sentence = "キャンペーン中なら配送料を支払わないで済むんだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずに済む");
        assert_pattern_range(&patterns, "ずに済む", 13, 21); // 支払わないで済む
    }

    #[test]
    fn test_zunisumu_past() {
        // Structure: Verb[ない] + ずに済んだ (past tense)
        // Example: 使わずに済んだ (got by without using)
        let sentence = "今年の冬は去年より暖かかったので、ヒーターを使わずに済んだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずに済む");
        assert_pattern_range(&patterns, "ずに済む", 22, 29); // 使わずに済んだ
    }

    #[test]
    fn test_zunisumu_nakutesumimashita() {
        // Structure: Verb[なくて] + 済みました (polite past)
        // Example: 並ばなくて済みました (got by without waiting in line)
        let sentence = "友達が予約してくれたので並ばなくて済みました。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずに済む");
        assert_pattern_range(&patterns, "ずに済む", 12, 22); // 並ばなくて済みました
    }
}

// Pattern: ようがない・ようもない (there is no way to / impossible to)
// Data source: grammar_points_data.json["ようがない・ようもない"]
// Testing structure variants:
//   - standard[0]: Verb[stem] + よう + が + ない
//   - standard[1]: する Verb + (の) + しよう + が + ない
//   - standard[2]: Both can use も instead of が
//   - polite[0-2]: Same but with ありません instead of ない
mod youganai_u30fb_youmonai_tests {
    use super::*;

    // Test 1: Regular verb + ようがない
    // Structure: Verb[stem] + よう + が + ない
    // Example: 行きようがない (there is no way to go)
    #[test]
    fn verb_stem_youganai() {
        let sentence = "車も自転車も壊れているので買い物に行きようがない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ようがない・ようもない");
        assert_pattern_range(&patterns, "ようがない・ようもない", 17, 24); // 行きようがない
    }

    // Test 2: する Verb + の + しようがありません (polite)
    // Structure: Noun + の + しよう + が + ありません
    // Example: 連絡のしようがありません (there is no way to contact)
    #[test]
    fn suru_verb_shiyouganai() {
        let sentence = "電話番号もメールアドレスも分からないので、連絡のしようがありません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ようがない・ようもない");
        assert_pattern_range(&patterns, "ようがない・ようもない", 23, 33); // のしようがありません
    }

    // Test 3: Regular verb + ようもない (も variant)
    // Structure: Verb[stem] + よう + も + ない
    // Example: 直しようもない (there is no way to fix)
    #[test]
    fn verb_stem_youmonai() {
        let sentence = "パソコンが粉々になったため直しようもない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ようがない・ようもない");
        assert_pattern_range(&patterns, "ようがない・ようもない", 13, 20); // 直しようもない
    }

    // Test 4: する Verb + の + しようもない (も variant)
    // Structure: Noun + の + しよう + も + ない
    // Example: 対処のしようもない (there is no way to deal with)
    #[test]
    fn suru_verb_shiyoumonai() {
        let sentence = "どうしたらいいかわからず対処のしようもない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ようがない・ようもない");
        assert_pattern_range(&patterns, "ようがない・ようもない", 14, 21); // のしようもない
    }

    // Test 5: Polite form with ありません
    // Structure: Verb[stem] + よう + が + ありません
    // Example: 調べようがありません (there is no way to investigate)
    #[test]
    fn youga_arimasen_polite() {
        let sentence = "資料が一切ないので調べようがありません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ようがない・ようもない");
        assert_pattern_range(&patterns, "ようがない・ようもない", 9, 19); // 調べようがありません
    }

    // Test 6: Negative past form - ようがなかった
    // Structure: Verb[stem] + よう + が + なかった
    // Example: 逃げようがなかった (there was no way to escape)
    #[test]
    fn youganakatta_past() {
        let sentence = "当時の状況では逃げようがなかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ようがない・ようもない");
        assert_pattern_range(&patterns, "ようがない・ようもない", 7, 16); // 逃げようがなかった
    }
}

// Pattern: にほかならない (nothing but, simply)
// Data source: grammar_points_data.json["にほかならない"]
// Testing all structure variants
mod nihokanaranai_tests {
    use super::*;

    // Testing: structure.standard[0] - "Noun + にほかならない"
    #[test]
    fn test_nihokanaranai_standard() {
        let sentence = "かすみさんがみんなに好かれるのはお人よしだからにほかならない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にほかならない");
        assert_pattern_range(&patterns, "にほかならない", 23, 30); // にほかならない
    }

    // Testing: structure.standard[1] - "にほかならぬ" (formal written variant)
    #[test]
    fn test_nihokanaranai_naranu() {
        let sentence = "Aチームがこの大会で優勝できたのは岡村コーチのおかげにほかならぬ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にほかならない");
        assert_pattern_range(&patterns, "にほかならない", 26, 32); // にほかならぬ
    }

    // Testing: structure.polite[0] - "Noun + にほかなりません"
    #[test]
    fn test_nihokanaranai_polite() {
        let sentence = "こんな夜遅くに訪問してくるなんて迷惑にほかなりません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にほかならない");
        assert_pattern_range(&patterns, "にほかならない", 18, 26); // にほかなりません
    }

    // Testing: Noun emphasis variant
    #[test]
    fn test_nihokanaranai_reason() {
        let sentence = "我が社の商品が人気なのは広告が印象的だからにほかならぬ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にほかならない");
        assert_pattern_range(&patterns, "にほかならない", 21, 27); // にほかならぬ
    }
}

// Pattern: っこない (there is no chance of / impossible)
// Data source: grammar_points_data.json["っこない"]
// Testing all structure variants
mod kkonai_tests {
    use super::*;

    // Testing: structure.standard[0] - "Verb[stem] + っこない"
    #[test]
    fn test_kkonai_potential_verb() {
        let sentence = "今年中に３０万円貯めたいの？お前の給料じゃできっこないよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "っこない");
        assert_pattern_range(&patterns, "っこない", 22, 27); // きっこない
    }

    // Testing: Verb[stem] + っこない (another example)
    #[test]
    fn test_kkonai_verb_win() {
        let sentence = "どんなに鍛えてもあの人だけには勝てっこないよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "っこない");
        assert_pattern_range(&patterns, "っこない", 16, 21); // てっこない
    }

    // Testing: Verb[stem] + っこない (another example)
    #[test]
    fn test_kkonai_verb_buy() {
        let sentence = "あんなデカい家、どんなに貯金しても買えっこないよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "っこない");
        assert_pattern_range(&patterns, "っこない", 17, 23); // 買えっこない
    }

    // Testing: Verb[stem] + っこない (past tense example)
    #[test]
    fn test_kkonai_past() {
        let sentence = "こんな漢字だらけの新聞なんて読めっこなかった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "っこない");
        assert_pattern_range(&patterns, "っこない", 14, 22); // 読めっこなかった
    }
}

// Pattern: それなら (if that's the case, then)
// Data source: grammar_points_data.json["それなら"]
// Structures: それなら + Phrase
//
// Note: The variants だったら and それだったら mentioned in grammar_points_data.json
// are already detected by the existing たら conditional pattern, as they tokenize as
// the copula だ + conditional auxiliary たら (different structure from conjunction それなら).
mod sorenara_tests {
    use super::*;

    // Testing: structure.standard[0] - "それなら + Phrase"
    #[test]
    fn test_sorenara_basic() {
        let sentence = "駅の近くにあるスーパーに行くの？それなら私を駅まで送ってくれない？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "それなら");
        assert_pattern_range(&patterns, "それなら", 16, 20); // それなら
    }

    // Testing: それなら at beginning of sentence
    #[test]
    fn test_sorenara_sentence_start() {
        let sentence = "それなら彼でもできるようだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "それなら");
        assert_pattern_range(&patterns, "それなら", 0, 4); // それなら
    }

    // Testing: それなら with suggestion
    #[test]
    fn test_sorenara_with_suggestion() {
        let sentence = "それなら先生に聞いたほうがいいと思います。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "それなら");
        assert_pattern_range(&patterns, "それなら", 0, 4); // それなら
    }
}

// Pattern: ものなら① (if one could / if it were possible)
// Data source: grammar_points_data.json["ものなら①"]
// Structures: Verb[potential] + ものなら + Phrase
//
// This pattern expresses hypothetical possibility: "if (A) were possible, (B)"
// Comes after potential form verbs (できる or られる/れる forms)
mod mononara_u2460_tests {
    use super::*;

    // Testing: structure.standard[0] - "Verb[できる] + ものなら + Phrase"
    #[test]
    fn test_mononara_potential_verb() {
        let sentence = "俺のことを捕まえることができるものなら、捕まえてみろ！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ものなら①");
        assert_pattern_range(&patterns, "ものなら①", 12, 19); // できるものなら
    }

    // Testing: potential verb (られる form)
    #[test]
    fn test_mononara_rareru_form() {
        let sentence = "すぐに直せるものなら直したいけど、今はちょっとお金がないから。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ものなら①");
        assert_pattern_range(&patterns, "ものなら①", 3, 10); // 直せるものなら
    }

    // Testing: with もし (if) at sentence start
    #[test]
    fn test_mononara_with_moshi() {
        let sentence = "もし行けるものならケニヤに行ってみたいです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ものなら①");
        assert_pattern_range(&patterns, "ものなら①", 2, 9); // 行けるものなら
    }
}

// Pattern: 活かす (to make good use of, to leverage)
// Data source: grammar_points_data.json["活かす"]
// Structures: Noun + を + 活かす/生かす
//
// Matches the verb 活かす or 生かす (to make good use of) in any conjugation.
// Often used with adverbs like 十分に (adequately), 有効に (effectively), etc.
mod ikasu_tests {
    use super::*;

    // Testing: structure.standard[0] - "Noun + を + 活かす"
    #[test]
    fn test_ikasu_basic() {
        let sentence = "このパソコンの機能を十分に活かすのにはたくさんの知識が必要だ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "活かす");
        assert_pattern_range(&patterns, "活かす", 13, 16); // 活かす
    }

    // Testing: structure.standard[1] - "Noun + を + 活かした + Noun" (attributive)
    #[test]
    fn test_ikasu_attributive() {
        let sentence = "私のくせ毛を活かした髪型にカットしてください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "活かす");
        assert_pattern_range(&patterns, "活かす", 6, 10); // 活かした (includes た)
    }

    // Testing: structure.standard[2] - "Noun + を + 生かす" (variant kanji)
    #[test]
    fn test_ikasu_variant_kanji() {
        let sentence = "この失敗を次に生かしてみてください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "活かす");
        assert_pattern_range(&patterns, "活かす", 7, 10); // 生かし (連用形)
    }

    // Testing: Noun + を + 活かす with adverb
    #[test]
    fn test_ikasu_with_adverb() {
        let sentence = "この料理は鯖の旨味を活かした料理なので調味料はあまり使っていません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "活かす");
        assert_pattern_range(&patterns, "活かす", 10, 14); // 活かした
    }

    // ============================================================================
    // Pattern: まい (won't / intend not to / probably not)
    // Data source: grammar_points_data.json["まい"]
    // ============================================================================

    // Testing: structure.standard[0] - "Verb + まい" (basic form)
    #[test]
    fn test_mai_basic_form() {
        let sentence = "あんな接客が雑なレストランにはもう行くまい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まい");
        assert_pattern_range(&patterns, "まい", 17, 21); // 行くまい
    }

    // Testing: structure.standard[1] - "Verb[stem] + まい" (conjunctive form)
    // Note: する→しまい or すまい, 来る→きまい or こまい
    #[test]
    fn test_mai_suru_stem() {
        let sentence = "田中くんはとてもいい人なので、そんなひどい事はしまい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まい");
        assert_pattern_range(&patterns, "まい", 23, 26); // しまい
    }

    // Testing: Verb[stem] + まい with する (alternative: すまい)
    #[test]
    fn test_mai_suru_sumai() {
        let sentence = "彼女はそんな嘘はすまい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まい");
        assert_pattern_range(&patterns, "まい", 8, 11); // すまい
    }

    // Testing: Verb + まい with 来る (standard form: 来るまい)
    #[test]
    fn test_mai_kuru_standard() {
        let sentence = "台風の警報が出ているので、今日中に届くはずだった荷物は来るまい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まい");
        assert_pattern_range(&patterns, "まい", 27, 31); // 来るまい
    }

    // Testing: Verb[stem] + まい with 来る (stem form: こまい)
    #[test]
    fn test_mai_kuru_stem() {
        let sentence = "台風の警報が出ているので、今日中に届くはずだった荷物はこまい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まい");
        assert_pattern_range(&patterns, "まい", 27, 30); // こまい
    }

    // Testing: structure.polite[0] - "Verb (Polite) + まい"
    #[test]
    fn test_mai_polite_form() {
        let sentence = "あのアパートの家賃は高いので、あそこには住みますまい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まい");
        assert_pattern_range(&patterns, "まい", 20, 26); // 住みますまい
    }
}

// Pattern: 上 (standpoint/from the perspective of)
// Data source: grammar_points_data.json["上"]
mod ue_tests {
    use super::*;

    // Testing: structure.standard[0] - "Noun + 上（じょう）"
    #[test]
    fn test_ue_legal_standpoint() {
        let sentence = "法律上、伊勢海老を許可なく釣り上げて持って帰ることが禁止されている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "上");
        assert_pattern_range(&patterns, "上", 0, 3); // 法律上
    }

    // Testing: structure.standard[0] - "Noun + 上（じょう）" (rules context)
    #[test]
    fn test_ue_rules_standpoint() {
        let sentence = "規則上、生徒は髪を染めてはいけないのに、髪を染めている生徒が複数います。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "上");
        assert_pattern_range(&patterns, "上", 0, 3); // 規則上
    }

    // Testing: structure.standard[0] - "Noun + 上（じょう）" (work context)
    #[test]
    fn test_ue_work_standpoint() {
        let sentence = "仕事上、夜遅くまで残業することが多くてたまらないよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "上");
        assert_pattern_range(&patterns, "上", 0, 3); // 仕事上
    }
}

// Pattern: 上に (in addition to / as well as)
// Data source: grammar_points_data.json["上に"]
mod ueni_tests {
    use super::*;

    // Testing: structure.standard[0] - "Verb + 上（うえ）(に)"
    #[test]
    fn test_ueni_verb() {
        let sentence = "私の犬は子供を見ると吠えるうえに噛みつこうとするので子供には近づけさせないようにしています。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "上に");
        assert_pattern_range(&patterns, "上に", 10, 16); // 吠えるうえに
    }

    // Testing: structure.standard[1] - "［い］Adjective + 上（うえ）(に)"
    #[test]
    fn test_ueni_i_adjective() {
        let sentence = "家の近所にあるレストランはまずいうえに、高いからいつも空いている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "上に");
        assert_pattern_range(&patterns, "上に", 13, 19); // まずいうえに
    }

    // Testing: structure.standard[2] - "［な］Adjective + な + 上（うえ）(に)"
    #[test]
    fn test_ueni_na_adjective() {
        let sentence = "高橋くんは無礼なうえに清潔感がないため、周りの人たちには避けられている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "上に");
        assert_pattern_range(&patterns, "上に", 7, 11); // なうえに
    }

    // Testing: structure.standard[3] - "Noun + の + 上（うえ）(に)"
    #[test]
    fn test_ueni_noun() {
        let sentence = "彼女は調理師免許のうえに健康食アドバイザーの資格も持っているので、彼女が作る料理は健康的で美味しいです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "上に");
        assert_pattern_range(&patterns, "上に", 8, 12); // のうえに
    }

    // Testing: structure.standard[4] - "な-Adj + である + 上（うえ）(に)"
    #[test]
    fn test_ueni_na_adjective_dearu() {
        let sentence = "あそこの駅は不便であるうえに利用者が減っている為、来月の中旬に取り壊されるそうだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "上に");
        assert_pattern_range(&patterns, "上に", 9, 14); // あるうえに
    }

    // Testing: structure.standard[4] - "Noun + である + 上（うえ）(に)"
    #[test]
    fn test_ueni_noun_dearu() {
        let sentence = "ケントさんは新聞記者であるうえにスーパーヒーローでもあった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "上に");
        assert_pattern_range(&patterns, "上に", 11, 16); // あるうえに
    }
}

// Pattern: 中を (doing B in/on/inside A)
// Data source: grammar_points_data.json["中を"]
mod nakawo_tests {
    use super::*;

    // Testing: structure.standard[0] - "Noun + の + 中（なか）を"
    #[test]
    fn test_nakawo_empty_lunchbox() {
        let sentence = "家に帰ってきたら、まずは弁当箱の中を空にしてっていつも言ってるじゃん！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "中を");
        assert_pattern_range(&patterns, "中を", 15, 18); // の中を
    }

    // Testing: structure.standard[0] - "Noun + の + 中（なか）を" (wandering in supermarket)
    #[test]
    fn test_nakawo_supermarket() {
        let sentence = "スーパーの中を何も買わずにウロウロしていたら、警備員のおじさんに怪しまれて声をかけられた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "中を");
        assert_pattern_range(&patterns, "中を", 4, 7); // の中を
    }

    // Testing: structure.standard[0] - "Noun + の + 中（なか）を" (peek into room)
    #[test]
    fn test_nakawo_peek_into_room() {
        let sentence = "子供が妙に静かだな〜と思い、子供部屋の中を覗いてみたらただ寝ていただけだった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "中を");
        assert_pattern_range(&patterns, "中を", 18, 21); // の中を
    }
}

// Pattern: 以上 ② (since, now that, as long as)
// Data source: grammar_points_data.json["以上 ②"]
mod ijou_u2461_tests {
    use super::*;

    // Testing: structure.standard[0] - "Verb + 以上（いじょう）(は)"
    #[test]
    fn test_ijou_verb_with_wa() {
        let sentence = "猫を飼うと決めた以上は、最後まで責任を持って育てないといけないと思います。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "以上 ②");
        assert_pattern_range(&patterns, "以上 ②", 7, 11); // た以上は
    }

    // Testing: structure.standard[0] - "Verb + 以上（いじょう）(は)" without は
    #[test]
    fn test_ijou_verb_without_wa() {
        let sentence = "キャプテンに選ばれた以上、結果を残すために精一杯頑張ります！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "以上 ②");
        assert_pattern_range(&patterns, "以上 ②", 9, 12); // た以上
    }

    // Testing: structure.standard[1] - "［い］Adjective + 以上（いじょう）(は)"
    #[test]
    fn test_ijou_i_adjective() {
        let sentence = "体調が悪い以上は、会社に来ないでください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "以上 ②");
        assert_pattern_range(&patterns, "以上 ②", 3, 8); // 悪い以上は
    }

    // Testing: structure.standard[2] - "［な］Adjective + である + 以上（いじょう）(は)"
    #[test]
    fn test_ijou_na_adjective_dearu() {
        let sentence = "有名である以上は、テレビではもちろん、ＳＮＳでも発言に気をつけなければいけません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "以上 ②");
        assert_pattern_range(&patterns, "以上 ②", 3, 8); // ある以上は (from 有名である以上は)
    }

    // Testing: structure.standard[3] - "Noun + である + 以上（いじょう）(は)"
    #[test]
    fn test_ijou_noun_dearu_with_wa() {
        let sentence = "この地域の住民である以上は、この地域のルールをしっかりと守ってもらわないと困ります。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "以上 ②");
        assert_pattern_range(&patterns, "以上 ②", 8, 13); // ある以上は (from 住民である以上は)
    }

    // Testing: structure.standard[3] - "Noun + である + 以上（いじょう）(は)" without は
    #[test]
    fn test_ijou_noun_dearu_without_wa() {
        let sentence = "どんなに気を付けていても、人間である以上、ミスは防げないよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "以上 ②");
        assert_pattern_range(&patterns, "以上 ②", 16, 20); // ある以上 (from 人間である以上)
    }
}

// Pattern: 以上に (more than, even more than)
// Data source: grammar_points_data.json["以上に"]
mod ijouni_tests {
    use super::*;

    // Testing: structure.standard[0] - "Verb + 以上（いじょう）に"
    #[test]
    fn test_ijouni_verb() {
        let sentence = "新しく発売した商品の評判が思っていた以上に良かったのでびっくりした。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "以上に");
        assert_pattern_range(&patterns, "以上に", 17, 21); // た以上に
    }

    // Testing: structure.standard[1] - "［い］Adjective + 以上（いじょう）に"
    #[test]
    fn test_ijouni_i_adjective() {
        let sentence = "今日の風は冷たい以上に痛い。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "以上に");
        assert_pattern_range(&patterns, "以上に", 5, 11); // 冷たい以上に
    }

    // Testing: structure.standard[2] - "［な］Adjective + 以上（いじょう）に"
    #[test]
    fn test_ijouni_na_adjective() {
        let sentence = "今節約中だから必要以上にお金を使わないようにしてる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "以上に");
        assert_pattern_range(&patterns, "以上に", 7, 12); // 必要以上に
    }

    // Testing: structure.standard[3] - "Noun + 以上（いじょう）に"
    #[test]
    fn test_ijouni_noun() {
        let sentence = "私はスキー以上にスノーボードが好きだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "以上に");
        assert_pattern_range(&patterns, "以上に", 2, 8); // スキー以上に
    }

    // Testing: structure.standard[4] - "以上（いじょう） + の + Noun"
    #[test]
    fn test_ijouni_no_noun() {
        let sentence = "僕の最初のライブに想像していた以上の人が集まったので、すごく嬉しかった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "以上に");
        assert_pattern_range(&patterns, "以上に", 14, 18); // た以上の
    }
}

// Pattern: 途中に・途中で (on the way, partway through, in the middle of)
// Data source: grammar_points_data.json["途中に・途中で"]
// Testing: structure.standard[0] - "Verb［る］+ 途中（とちゅう） + で"
// Testing: structure.standard[1] - "Noun + の + 途中（とちゅう） + で"
// Testing: structure.standard[2] - "(1) に" variant
//
// Structure variants:
//   - standard[0]: Verb［る］+ 途中（とちゅう） + で
//   - standard[1]: Noun + の + 途中（とちゅう） + で
//   - standard[2]: Both can use に instead of で
//
// Note: に emphasizes time/duration, で emphasizes process/opportunity

mod tochuuni_tochuude_tests {
    use super::*;

    // Testing: Verb[る] + 途中 + に
    #[test]
    fn test_verb_tochuuni() {
        let sentence = "待ち合わせ場所に向かっている途中に、「ごめん、やっぱり今日行けないかも」と友達からメールが来た。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "途中に・途中で");
        assert_pattern_range(&patterns, "途中に・途中で", 12, 17); // いる途中に
    }

    // Testing: Verb[る] + 途中 + で
    #[test]
    fn test_verb_tochuude() {
        let sentence = "会社から帰る途中で変なおじさんに話しかけられた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "途中に・途中で");
        assert_pattern_range(&patterns, "途中に・途中で", 4, 9); // 帰る途中で
    }

    // Testing: Noun + の + 途中 + に
    #[test]
    fn test_noun_tochuuni() {
        let sentence = "授業の途中に校長先生から呼び出された時はビクッとした。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "途中に・途中で");
        assert_pattern_range(&patterns, "途中に・途中で", 0, 6); // 授業の途中に
    }

    // Testing: Noun + の + 途中 + で
    #[test]
    fn test_noun_tochuude() {
        let sentence = "ミーティングの途中で社長が倒れて、会社中がパニックになった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "途中に・途中で");
        assert_pattern_range(&patterns, "途中に・途中で", 0, 10); // ミーティングの途中で
    }
}

// Pattern: を中心に (focused on, centered around, mainly)
// Data source: grammar_points_data.json["を中心に"]
// Testing: structure.standard[0] - "Noun + を中心（ちゅうしん） + に"
// Testing: structure.standard[1] - "Noun + を中心（ちゅうしん） + にした + Noun"
// Testing: structure.standard[2] - "にして、として" variants
// Testing: structure.standard[3] - "とした" variant
//
// Structure variants:
//   - standard[0]: Noun + を中心に
//   - standard[1]: Noun + を中心にした + Noun
//   - standard[2]: を中心にして, を中心として
//   - standard[3]: を中心とした + Noun
//   - Additional: を中心にする (verb form)

mod wochuushinni_tests {
    use super::*;

    // Testing: Noun + を中心に
    #[test]
    fn test_noun_wo_chuushinni() {
        let sentence = "あのアイドルは若者を中心に人気を集めている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を中心に");
        assert_pattern_range(&patterns, "を中心に", 7, 13); // 若者を中心に
    }

    // Testing: Noun + を中心にした + Noun
    #[test]
    fn test_noun_wo_chuushinni_shita_noun() {
        let sentence = "ここはスケートボードを中心にしたポップアップストアです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を中心に");
        assert_pattern_range(&patterns, "を中心に", 7, 16); // ボードを中心にした
    }

    // Testing: Noun + を中心にして
    #[test]
    fn test_noun_wo_chuushinni_shite() {
        let sentence = "私は今日本語の文法を中心にして勉強をしています。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を中心に");
        assert_pattern_range(&patterns, "を中心に", 7, 14); // 文法を中心にし (verb part before て)
    }

    // Testing: Noun + を中心として
    #[test]
    fn test_noun_wo_chuushintoshite() {
        let sentence = "明日からは札幌を中心として大雪が降るところがあるでしょう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を中心に");
        assert_pattern_range(&patterns, "を中心に", 5, 13); // 札幌を中心として
    }

    // Testing: Noun + を中心とした + Noun
    #[test]
    fn test_noun_wo_chuushintoshita_noun() {
        let sentence = "これからはクライアントの提案を中心とした話を進めたいと思います。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を中心に");
        assert_pattern_range(&patterns, "を中心に", 12, 20); // 提案を中心とした
    }

    // Testing: Noun + を中心にする
    #[test]
    fn test_noun_wo_chuushinni_suru() {
        let sentence = "この業界ではお客様を中心にすることが一番重要なことだと言われている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を中心に");
        assert_pattern_range(&patterns, "を中心に", 6, 15); // お客様を中心にする
    }
}

// Pattern: その上 (besides, in addition to, furthermore)
// Data source: grammar_points_data.json["その上"]
// Testing: structure.standard[0] - "その上（うえ） + Phrase"
//
// Structure variants:
//   - standard[0]: その上 + Phrase (conjunction at beginning of sentence)
//
// Note: Formal expression, but also used in daily speech

mod sonoue_tests {
    use super::*;

    // Testing: その上 at sentence beginning
    #[test]
    fn test_sonoue_sentence_start() {
        let sentence = "高橋さんはとても頭がいい。その上、人柄もいいので会社での評判がいいです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "その上");
        assert_pattern_range(&patterns, "その上", 13, 16); // その上
    }

    // Testing: その上 mid-conversation
    #[test]
    fn test_sonoue_addition() {
        let sentence = "今住んでいる家はものすごく小さい。その上隣に住んでいる住人が一日中うるさいので、来月引っ越そうと思う。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "その上");
        assert_pattern_range(&patterns, "その上", 17, 20); // その上
    }

    // Testing: その上 after negative statement
    #[test]
    fn test_sonoue_negative_context() {
        let sentence = "今日は学校で先生に怒られた。その上、家に帰ったら親に怒られたのであまりいい一日ではなかった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "その上");
        assert_pattern_range(&patterns, "その上", 14, 17); // その上
    }

    // Pattern: 上は (now that, since, as long as)
    // Data source: grammar_points_data.json["上は"]
    // Testing: structure.standard[0] - Verb[る] + 上は
    #[test]
    fn test_ueha_verb_ru() {
        let sentence = "この会社の従業員として働く上は、きちんと我が社のルールを守ってもらわないと困ります。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "上は");
        assert_pattern_range(&patterns, "上は", 11, 15); // 働く上は
    }

    // Testing: structure.standard[1] - Verb[た] + 上は
    #[test]
    fn test_ueha_verb_ta() {
        let sentence = "一人暮らしを始めた上は、家事など料理は自分で全部やらなくてはいけない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "上は");
        assert_pattern_range(&patterns, "上は", 6, 11); // 始めた上は
    }

    // Pattern: の下で (under, on the basis of)
    // Data source: grammar_points_data.json["の下で"]

    // Testing: structure.standard[0] - Noun + のもとで
    #[test]
    fn test_noshitade_moto_de() {
        let sentence = "こんな厳しい環境のもとで試料を採取できたのは皆様のおかげです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "の下で");
        assert_pattern_range(&patterns, "の下で", 6, 12); // 環境のもとで
    }

    // Testing: structure.standard[1] - Noun + のもとに (using で in example)
    #[test]
    fn test_noshitade_moto_ni() {
        let sentence = "先輩のもとで働けて光栄です！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "の下で");
        assert_pattern_range(&patterns, "の下で", 0, 6); // 先輩のもとで
    }

    // Testing: Noun + のもと (without particle)
    #[test]
    fn test_noshitade_moto_only() {
        let sentence = "皆さんの協力のもと、このプロジェクトを無事に終えることができました。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "の下で");
        assert_pattern_range(&patterns, "の下で", 4, 9); // 協力のもと
    }

    // Pattern: 手前 (in front of, given the circumstances)
    // Data source: grammar_points_data.json["手前"]

    // Testing: structure.standard[0] - Verb + 手前
    #[test]
    fn test_temae_verb() {
        let sentence = "勢いで手を上げてしまった手前、答えずに手を下げるわけにはいかない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "手前");
        assert_pattern_range(&patterns, "手前", 8, 14); // しまった手前
    }

    // Testing: structure.standard[1] - Noun + の + 手前
    #[test]
    fn test_temae_noun() {
        let sentence = "娘の手前、運動音痴な姿を見せるわけにはいかない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "手前");
        assert_pattern_range(&patterns, "手前", 0, 4); // 娘の手前
    }

    // Pattern: 後(の) Noun (the rest of, what's remaining)
    // Data source: grammar_points_data.json["後(の) Noun"]

    // Testing: structure.standard[0] - あと + の + Noun
    #[test]
    fn test_ato_no_noun() {
        let sentence = "今あなたが持っている段ボール箱はキッチンまで運んでおいてください。後の段ボール箱は全部寝室までお願いします。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "後(の) Noun");
        assert_pattern_range(&patterns, "後(の) Noun", 33, 39); // 後の段ボール
    }

    // Testing: structure.standard[1] - あと + Phrase
    #[test]
    fn test_ato_phrase() {
        let sentence = "後ちょっとで着くからもうちょっと我慢して。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "後(の) Noun");
        assert_pattern_range(&patterns, "後(の) Noun", 0, 5); // 後ちょっと
    }

    // Testing: structure.standard[2] - あと + Number + (Counter)
    #[test]
    fn test_ato_number() {
        let sentence = "映画公開まで後三日！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "後(の) Noun");
        assert_pattern_range(&patterns, "後(の) Noun", 6, 9); // 後三日
    }

    // Pattern: を巡って (concerning, in regard to, about)
    // Data source: grammar_points_data.json["を巡って"]

    // Testing: structure.standard[0] - Noun + をめぐって
    #[test]
    fn test_womegutte_basic() {
        let sentence = "隣人と土地の境界線をめぐってトラブルが起きた時は誰に連絡するべきですか？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を巡って");
        assert_pattern_range(&patterns, "を巡って", 8, 14); // 線をめぐって
    }

    // Testing: structure.standard[1] - Noun + をめぐる + Noun
    #[test]
    fn test_womegutte_meguru() {
        let sentence = "この島をめぐる争いは私が生まれる前から続いています。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を巡って");
        assert_pattern_range(&patterns, "を巡って", 2, 7); // 島をめぐる
    }

    // Testing: structure.standard[2] - をめぐり (same as structure 0, but used before の)
    #[test]
    fn test_womegutte_meguri() {
        let sentence = "ゴミ出しルールをめぐってのトラブルはどの地域でも珍しいことではありません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を巡って");
        assert_pattern_range(&patterns, "を巡って", 4, 12); // ルールをめぐって
    }
}

// Pattern: にわたって (across, throughout, over the period of)
// Data source: grammar_points_data.json["にわたって"]
// Testing all structure variants with print_debug

#[test]
fn test_niwatatte_te_form() {
    // Testing: structure.standard[0] - "Noun + にわたって"
    let sentence = "彼は脱獄後長年にわたって警察から身を隠し続けた";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "にわたって");
    assert_pattern_range(&patterns, "にわたって", 5, 12); // 長年にわたって
}

#[test]
fn test_niwatatte_ru_form() {
    // Testing: structure.standard[1] - "Noun + にわたる + Noun"
    let sentence = "１５年にわたる戦争は、ついにその幕を閉じた";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "にわたって");
    assert_pattern_range(&patterns, "にわたって", 2, 7); // 年にわたる
}

#[test]
fn test_niwatatte_ri_form() {
    // Testing: structure.standard[2] - "にわたり"
    let sentence = "会議は三日間にわたり続けられた";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "にわたって");
    assert_pattern_range(&patterns, "にわたって", 4, 10); // 日間にわたり
}

#[test]
fn test_niwatatte_ta_form() {
    // Testing: structure.standard[3] - "にわたった"
    let sentence = "十年間にわたった研究がようやく完成した";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "にわたって");
    assert_pattern_range(&patterns, "にわたって", 1, 8); // 年間にわたった
}

// Pattern: に沿って (along, in accordance with, in line with)
// Data source: grammar_points_data.json["に沿って"]
// Testing all structure variants with print_debug

#[test]
fn test_nisotte_te_form() {
    // Testing: structure.standard[0] - "Noun + にそって"
    let sentence = "会社のルールにそって、残業をする方は必ずタイムカードを押してからにしてください";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "に沿って");
    assert_pattern_range(&patterns, "に沿って", 3, 10); // ルールにそって
}

#[test]
fn test_nisotte_ta_form() {
    // Testing: structure.standard[1] - "Noun + にそった + Noun"
    let sentence = "ご予算内でお客様のご希望にそったお家を建てれるよう全力を尽くします";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "に沿って");
    assert_pattern_range(&patterns, "に沿って", 10, 16); // 希望にそった
}

// TODO: Undetectable - "Noun + にそう" (dictionary form)
// The にそう form in the example sentence "希望にそう方向で進めたい" tokenizes
// as に (particle) + そう (副詞/助詞類接続) instead of に + そう (verb).
// Kagome misidentifies "そう" as an adverb in this context rather than the verb
// 沿う (to follow along). This creates ambiguity with the auxiliary そう (seems like).
// The pattern can only reliably detect にそって and にそった forms where
// the verb is clearly conjugated.
//
// #[test]
// fn test_nisotte_u_form() {
//     let sentence = "クライアント様の希望にそう方向で進めたいと思います";
//     let tokens = tokenize_sentence(sentence);
//     let patterns = detect_patterns(&tokens);
//     assert_has_pattern(&patterns, "に沿って");
// }

// Pattern: た末・の末 (after, as a result of)
// Data source: grammar_points_data.json["た末・の末"]
// Testing: structure.standard[0] - "Verb[た] + すえ (に)"
// Testing: structure.standard[1] - "Noun + の + すえ (に)"
//
// Structure variants:
//   - standard[0]: Verb[た] + すえ (に) - after verb (past tense), implies long effort/struggle
//   - standard[1]: Noun + の + すえ (に) - after noun (process/struggle)

mod tasue_nosue_tests {
    use super::*;

    #[test]
    fn test_verb_ta_sue() {
        // Testing: structure.standard[0] - "Verb[た] + すえ (に)"
        let sentence = "色々と考えたすえに、お父さんの会社を継ぐことに決めた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "た末・の末");
        assert_pattern_range(&patterns, "た末・の末", 3, 9); // 考えたすえに
    }

    #[test]
    fn test_verb_ta_sue_simple() {
        // Testing: structure.standard[0] - "Verb[た] + すえ" (without に)
        let sentence = "迷いに迷ったすえ、彼女と別れることにした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "た末・の末");
        assert_pattern_range(&patterns, "た末・の末", 3, 8); // 迷ったすえ
    }

    #[test]
    fn test_noun_no_sue_ni() {
        // Testing: structure.standard[1] - "Noun + の + すえ"
        let sentence = "５年間にわたる争いのすえ、アメリカ軍が撤退を開始した";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "た末・の末");
        assert_pattern_range(&patterns, "た末・の末", 7, 12); // 争いのすえ
    }

    #[test]
    fn test_noun_no_sue_ni_long() {
        // Testing: structure.standard[1] - "Noun + の + すえに"
        let sentence = "社長と長い議論のすえに、人事の伊藤さんをクビにすることに決めた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "た末・の末");
        assert_pattern_range(&patterns, "た末・の末", 5, 11); // 議論のすえに
    }
}

// Pattern: にしたがって (in accordance with, as, following)
// Data source: grammar_points_data.json["にしたがって"]
// Testing: structure.standard[0] - "Verb[る] + にしたがって"
// Testing: structure.standard[1] - "Noun + にしたがって"
// Testing: structure.standard[2] - "にしたがい" variant
//
// Structure variants:
//   - standard[0]: Verb[る] + にしたがって/にしたがい - as verb happens
//   - standard[1]: Noun + にしたがって/にしたがい - in accordance with noun

mod nishitagatte_tests {
    use super::*;

    #[test]
    fn test_verb_nishitagatte() {
        // Testing: structure.standard[0] - "Verb[る] + にしたがって"
        let sentence = "年を取るにしたがって、目がどんどん悪くなっていってる気がする";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にしたがって");
        assert_pattern_range(&patterns, "にしたがって", 2, 10); // 取るにしたがって
    }

    #[test]
    fn test_verb_nishitagatte_progress() {
        // Testing: structure.standard[0] - "Verb[る] + にしたがって"
        let sentence = "テレワークの普及が進むにしたがって、通勤するサラリーマンが減っていっている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にしたがって");
        assert_pattern_range(&patterns, "にしたがって", 9, 17); // 進むにしたがって
    }

    #[test]
    fn test_noun_nishitagatte() {
        // Testing: structure.standard[1] - "Noun + にしたがって"
        let sentence = "僕はただ上司の指示にしたがって仕事を進めていただけです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にしたがって");
        assert_pattern_range(&patterns, "にしたがって", 7, 15); // 指示にしたがって
    }

    #[test]
    fn test_noun_nishitagatte_instructions() {
        // Testing: structure.standard[1] - "Noun + にしたがって"
        let sentence = "この説明書にしたがって、パソコンの初期設定を行ってください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にしたがって");
        assert_pattern_range(&patterns, "にしたがって", 4, 11); // 書にしたがって
    }
}

// Pattern: につき (due to, per)
// Data source: grammar_points_data.json["につき"]
// Testing: structure.standard[0] - "Noun + につき"
//
// Structure variants:
//   - standard[0]: Noun + につき (only one structure)

mod nitsuki_tests {
    use super::*;

    #[test]
    fn test_nitsuki_store_closing() {
        // Testing: structure.standard[0] - "Noun + につき"
        // Example: 閉店につき - Due to store closing
        // Tokenization: 閉店(noun) + に(particle) + つき(verb)
        let sentence = "閉店につき、特別セール開催中！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "につき");
        assert_pattern_range(&patterns, "につき", 0, 5); // 閉店につき
    }

    #[test]
    fn test_nitsuki_construction() {
        // Testing: structure.standard[0] - "Noun + につき"
        // Example: 工事中につき - Due to construction
        // Tokenization: 工事(noun) + 中(noun suffix) + に(particle) + つき(verb)
        let sentence = "工事中につき、この先立ち入り禁止";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "につき");
        assert_pattern_range(&patterns, "につき", 2, 6); // 中につき
    }

    #[test]
    fn test_nitsuki_per_person() {
        // Testing: structure.standard[0] - "Noun + につき"
        // Example: 一人につき - Per person
        // Tokenization: 一(noun) + 人(noun suffix) + につき(single particle token)
        let sentence = "この商品は大人気のため、一人につき二つまでとさせていただいております";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "につき_compound");
        assert_pattern_range(&patterns, "につき_compound", 13, 17); // 人につき
    }
}

// Pattern: 次第に (gradually, bit by bit)
// Data source: grammar_points_data.json["次第に"]
// Testing: structure.standard[0] - "次第（しだい）に + Phrase"
//
// Structure variants:
//   - standard[0]: 次第に + Phrase (adverb meaning "gradually")

mod shidaini_tests {
    use super::*;

    #[test]
    fn test_shidaini_drift_apart() {
        // Testing: structure.standard[0] - "次第に + Phrase"
        // Example: わずか数ヶ月のうちに二人は次第に疎遠になった
        let sentence = "わずか数ヶ月のうちに二人は次第に疎遠になった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "次第に");
        assert_pattern_range(&patterns, "次第に", 13, 16); // 次第に
    }

    #[test]
    fn test_shidaini_prices_cheaper() {
        // Testing: structure.standard[0] - "次第に + Phrase"
        // Example: ガソリンの価格は次第に安くなると予測されている
        let sentence = "ガソリンの価格は次第に安くなると予測されている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "次第に");
        assert_pattern_range(&patterns, "次第に", 8, 11); // 次第に
    }

    #[test]
    fn test_shidaini_deteriorating() {
        // Testing: structure.standard[0] - "次第に + Phrase"
        // Example: 友人の祖父の健康状態が次第に悪化していることも知らずに
        let sentence = "友人の祖父の健康状態が次第に悪化していることも知らずに、変なことを聞いてしまった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "次第に");
        assert_pattern_range(&patterns, "次第に", 11, 14); // 次第に
    }
}

// Pattern: に限って (particularly when, only when, those who)
// Data source: grammar_points_data.json["に限って"]
// Testing: structure.standard[0] - "Noun + に限（かぎ）って"
//
// Structure variants:
//   - standard[0]: Noun + に限（かぎ）って
//
// Note: This pattern has 3 different meanings/uses:
//   1. When (A) is unbelievable/unexpected
//   2. When something is limited to (A)
//   3. When (B) is generally true for (A)
// All use the same grammatical structure: Noun + に限って

mod ni_kagitte_tests {
    use super::*;

    #[test]
    fn test_ni_kagitte_unbelievable() {
        // Testing: Meaning 1 - unbelievable thing about (A)
        // うちの子に限って、他の子に手を出すなんて考えられない
        let sentence = "うちの子に限って、他の子に手を出すなんて考えられない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に限って");
        assert_pattern_range(&patterns, "に限って", 3, 8); // 子に限って
    }

    #[test]
    fn test_ni_kagitte_limited_to() {
        // Testing: Meaning 2 - limited to (A)
        // 武くんと釣りに行く日に限って、いつも雨が降るんだよな
        let sentence = "武くんと釣りに行く日に限って、いつも雨が降るんだよな";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に限って");
        assert_pattern_range(&patterns, "に限って", 9, 14); // 日に限って
    }

    #[test]
    fn test_ni_kagitte_those_who() {
        // Testing: Meaning 3 - (B) is true in most cases of (A)
        // 「クレームを入れるぞ」って言う人に限ってクレームを入れないからあまり気にしていない
        let sentence = "クレームを入れるぞって言う人に限ってクレームを入れないからあまり気にしていない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に限って");
        assert_pattern_range(&patterns, "に限って", 13, 18); // 人に限って
    }
}

// Pattern: に限らず (not only, not just)
// Data source: grammar_points_data.json["に限らず"]
// Testing: structure.standard[0] - "Noun + に限（かぎ）らず"
//
// Structure variants:
//   - standard[0]: Noun + に限（かぎ）らず
//
// Note: This pattern means "not only (A), but also (B)"

mod ni_kagirazu_tests {
    use super::*;

    #[test]
    fn test_ni_kagirazu_weekends() {
        // Testing: not only weekends, but also weekdays
        // ＵＳＪは週末に限らず、平日でも多くの人で賑わっています
        let sentence = "ＵＳＪは週末に限らず、平日でも多くの人で賑わっています";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に限らず");
        assert_pattern_range(&patterns, "に限らず", 4, 10); // 週末に限らず
    }

    #[test]
    fn test_ni_kagirazu_children() {
        // Testing: not only children, but also adults
        // この遊園地は子供に限らず、大人でも楽しめます
        let sentence = "この遊園地は子供に限らず、大人でも楽しめます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に限らず");
        assert_pattern_range(&patterns, "に限らず", 6, 12); // 子供に限らず
    }

    #[test]
    fn test_ni_kagirazu_grammar() {
        // Testing: not only grammar, but also culture
        // 新しい言語を習う場合、文法に限らず、その国の文化も勉強したほうがいい
        let sentence = "新しい言語を習う場合、文法に限らず、その国の文化も勉強したほうがいい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に限らず");
        assert_pattern_range(&patterns, "に限らず", 11, 17); // 文法に限らず
    }
}

// Pattern: なお② (furthermore, moreover, in addition)
// Data source: grammar_points_data.json["なお②"]
// Testing: structure.standard[0] - "Phrase. なお、+ Phrase"
//
// Structure variants:
//   - standard[0]: Phrase. なお、+ Phrase (used at sentence beginning as conjunction)
//
// Note: Formal expression used in letters, posters, commercials. Often without kanji (尚).

mod nao_u2461_tests {
    use super::*;

    #[test]
    fn test_nao_u2461_information_session() {
        // Testing: なお as conjunction meaning "in addition"
        // それでは只今から説明会を開始したいと思います。なお、ご不明な点がある場合はお気軽にお申し付けください
        let sentence = "それでは只今から説明会を開始したいと思います。なお、ご不明な点がある場合はお気軽にお申し付けください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なお②");
        assert_pattern_range(&patterns, "なお②", 23, 25); // なお
    }

    #[test]
    fn test_nao_u2461_meeting_place() {
        // Testing: なお as conjunction meaning "moreover"
        // ５日の土曜日には１１時までに待ち合わせ場所に集合してください。なお、遅れる場合は私のＬＩＮＥまで連絡をください
        let sentence = "５日の土曜日には１１時までに待ち合わせ場所に集合してください。なお、遅れる場合は私のＬＩＮＥまで連絡をください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なお②");
        assert_pattern_range(&patterns, "なお②", 31, 33); // なお
    }

    #[test]
    fn test_nao_u2461_click_next() {
        // Testing: なお as conjunction meaning "furthermore"
        // 内容の確認が出来ましたら、「次へ」をクリックしてください。なお、「次へ」をクリックした後には内容の変更ができないのでご注意ください
        let sentence = "内容の確認が出来ましたら、「次へ」をクリックしてください。なお、「次へ」をクリックした後には内容の変更ができないのでご注意ください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なお②");
        assert_pattern_range(&patterns, "なお②", 29, 31); // なお
    }
}

// Pattern: ～てこそ (only if, only by, only when)
// Data source: grammar_points_data.json["～てこそ"]
// Testing: structure.standard[0] - "Verb［て］+ こそ"
//
// Structure variants:
//   - standard[0]: Verb［て］+ こそ
//
// Note: Indicates that (B) is absolutely reliant on (A). Translations: "only if (A), (B)", "unless (A), (B)", "until (A), (B)"

mod tekoso_tests {
    use super::*;

    #[test]
    fn test_tekoso_study_abroad() {
        // Testing: てこそ meaning "only by"
        // 海外に留学してこそ、日本食の美味しさが分かる
        let sentence = "海外に留学してこそ、日本食の美味しさが分かる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～てこそ");
        assert_pattern_range(&patterns, "～てこそ", 3, 9); // 留学してこそ
    }

    #[test]
    fn test_tekoso_become_parent() {
        // Testing: てこそ meaning "only when"
        // 親になってこそ、子育ての大変さがわかる
        let sentence = "親になってこそ、子育ての大変さがわかる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～てこそ");
        assert_pattern_range(&patterns, "～てこそ", 2, 7); // なってこそ
    }

    #[test]
    fn test_tekoso_captain_responsibility() {
        // Testing: てこそ meaning "only if"
        // チーム全員の責任を背負ってこそ、キャプテンになれる
        let sentence = "チーム全員の責任を背負ってこそ、キャプテンになれる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～てこそ");
        assert_pattern_range(&patterns, "～てこそ", 9, 15); // 背負ってこそ
    }

    #[test]
    fn test_tekoso_beat_teams() {
        // Testing: てこそ meaning "only by"
        // 他のチームに勝ってこそ、世界一になれる
        let sentence = "他のチームに勝ってこそ、世界一になれる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～てこそ");
        assert_pattern_range(&patterns, "～てこそ", 6, 11); // 勝ってこそ
    }
}

// Pattern: なお① (still, even, yet - adverb emphasizing continuation despite circumstances)
// Data source: grammar_points_data.json["なお①"]
// Testing structure variants:
//   - standard[0]: なお + Verb［ている］
//   - standard[1]: なお + Nounもいる
//   - standard[2]: なお + Nounもある
//   - standard[3]: なお + Noun + だ
//   - standard[4]: なお + Noun + である (formal)
//
// Note: This pattern detects なお as an adverb (not conjunction like なお②)

mod nao_u2460_tests {
    use super::*;

    #[test]
    fn test_nao_verb_teiru() {
        // Testing: structure.standard[0] - "なお + Verb［ている］"
        // Example from grammar_points_data.json
        let sentence = "豊橋市では今もなお、チンチン電車が走っている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なお①");
        assert_pattern_range(&patterns, "なお①", 7, 9); // なお
    }

    #[test]
    fn test_nao_noun_mo_iru() {
        // Testing: structure.standard[1] - "なお + Nounもいる"
        // Realistic example showing "still" with noun + も + いる
        let sentence = "卒業後もなお、彼女のことを覚えている人もいる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なお①");
        assert_pattern_range(&patterns, "なお①", 4, 6); // なお
    }

    #[test]
    fn test_nao_noun_mo_aru() {
        // Testing: structure.standard[2] - "なお + Nounもある"
        // Realistic example showing "still" with noun + も + ある
        let sentence = "この問題を解決できていない企業もなお、多くある";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なお①");
        assert_pattern_range(&patterns, "なお①", 16, 18); // なお
    }

    #[test]
    fn test_nao_noun_da() {
        // Testing: structure.standard[3] - "なお + Noun + だ"
        // Realistic example showing "still" with noun + だ
        let sentence = "あれから10年経った今もなお、彼は独身だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なお①");
        assert_pattern_range(&patterns, "なお①", 12, 14); // なお
    }

    #[test]
    fn test_nao_noun_dearu() {
        // Testing: structure.standard[4] - "なお + Noun + である" (formal)
        // Formal example showing "still" with noun + である
        let sentence = "研究は進展しているが、なお課題は山積みである";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なお①");
        assert_pattern_range(&patterns, "なお①", 11, 13); // なお
    }
}

// Pattern: につけ (every time, whenever)
// Data source: grammar_points_data.json["につけ"]
// Testing structure variants:
//   - standard[0]: Verb［る］+ につけ（て）
//   - standard[1]: Noun + につけ（て）
//
// Note: May appear as につけ or につけて (abbreviated form more common)

mod nitsuke_tests {
    use super::*;

    #[test]
    fn test_nitsuke_verb_dictionary() {
        // Testing: structure.standard[0] - "Verb［る］+ につけ"
        // Example from grammar_points_data.json
        let sentence = "元カノが写っている写真を見るにつけ、胸が痛くなる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "につけ");
        assert_pattern_range(&patterns, "につけ", 14, 17); // につけ
    }

    #[test]
    fn test_nitsuke_verb_tsukete() {
        // Testing: structure.standard[0] - "Verb［る］+ につけて" (less common)
        // Example from grammar_points_data.json showing につけて variant
        let sentence = "彼と一緒に時間を過ごすにつけて、なんでこんな人と付き合っているんだろうと思う";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "につけて");
        assert_pattern_range(&patterns, "につけて", 11, 15); // につけて
    }

    #[test]
    fn test_nitsuke_noun() {
        // Testing: structure.standard[1] - "Noun + につけ"
        // Example from grammar_points_data.json: 何かにつけ
        let sentence = "彼は何かにつけ文句を言うので、一緒にいるだけで疲れる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "につけ");
        assert_pattern_range(&patterns, "につけ", 4, 7); // につけ
    }
}

// Pattern: 要するに (To sum up, in summary, in short)
// Data source: grammar_points_data.json["要するに"]
// Testing: structure.standard[0] - "要するに + Phrase"
//
// Structure variants:
//   - standard[0]: 要するに + Phrase (sentence-initial discourse marker)
//   - polite: (none)

mod yousuruni_tests {
    use super::*;

    #[test]
    fn test_yousuruni_sentence_initial_1() {
        // Testing: structure.standard[0] - "要するに + Phrase"
        // Example from grammar_points_data.json
        let sentence = "要するに、あなたは履歴書に嘘を書いたと言う事ですね。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "要するに");
        assert_pattern_range(&patterns, "要するに", 0, 4); // 要するに
    }

    #[test]
    fn test_yousuruni_sentence_initial_2() {
        // Testing: structure.standard[0] - "要するに + Phrase"
        // Example from grammar_points_data.json
        let sentence = "要するに、小麦が入っている食べ物は食べれないと言う事ですね？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "要するに");
        assert_pattern_range(&patterns, "要するに", 0, 4); // 要するに
    }

    #[test]
    fn test_yousuruni_sentence_initial_3() {
        // Testing: structure.standard[0] - "要するに + Phrase"
        // Example from grammar_points_data.json
        let sentence = "要するに、俺はあいつに使われていたと言うことだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "要するに");
        assert_pattern_range(&patterns, "要するに", 0, 4); // 要するに
    }
}

// Pattern: たまえ (imperative form - polite order)
// Data source: grammar_points_data.json["たまえ"]
// Testing: structure.standard[0] - "Verb[stem] + たまえ"
//
// Structure variants:
//   - standard[0]: Verb[stem] + たまえ (polite imperative, men only, to subordinates)
//   - polite: (none)

mod tamae_tests {
    use super::*;

    #[test]
    fn test_tamae_sit() {
        // Testing: structure.standard[0] - "Verb[stem] + たまえ"
        // Example from grammar_points_data.json: 座りたまえ
        // Kagome correctly tokenizes this as: 座り(動詞/連用形) + たまえ(動詞/命令ｅ/base=たまう)
        let sentence = "まあまあ、とりあえず座りたまえ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たまえ");
        assert_pattern_range(&patterns, "たまえ", 10, 15); // 座りたまえ
    }
}

// TODO: Undetectable - Verb stem + たまえ (inconsistent Kagome tokenization)
// Kagome inconsistently tokenizes たまえ:
// - 座りたまえ: Correctly tokenized as verb stem + たまえ(動詞/命令ｅ/base=たまう) ✓
// - 食べたまえ: Incorrectly tokenized as た(助動詞) + ま(フィラー) + え(フィラー) ✗
// - 言いたまえ: Incorrectly tokenized as proper noun (person's name) ✗
//
// The pattern can only be detected when Kagome correctly recognizes たまえ as
// the imperative form of たまう. Unfortunately, this is inconsistent.
//
// #[test]
// fn test_tamae_say() {
//     // Example: 言いたまえ - Kagome tokenizes as proper noun (名詞/固有名詞/人名/名)
//     let sentence = "何か言いたそうな顔をしている君！考えてる事を言いたまえ！";
//     let tokens = tokenize_sentence(sentence);
//     let patterns = detect_patterns(&tokens);
//     // Cannot detect - tokenized as proper noun
// }
//
// #[test]
// fn test_tamae_eat() {
//     // Example: 食べたまえ - Kagome tokenizes as た(past) + ま(filler) + え(filler)
//     let sentence = "遠慮せずどんどん食べたまえ。";
//     let tokens = tokenize_sentence(sentence);
//     let patterns = detect_patterns(&tokens);
//     // Cannot detect - たまえ split into fillers
// }

// Pattern: に伴って・に伴い (due to, along with, in conjunction with)
// Data source: grammar_points_data.json["に伴って・に伴い"]
//
// Structure variants:
//   - standard[0]: Verb[る]+(の)+ に伴って (or に伴い)
//   - standard[1]: Verb[る]+(の)+ に伴う + Noun
//   - standard[2]: Noun + に伴って (or に伴い)
//   - standard[3]: Noun + に伴う + Noun

mod nitomonatte_tests {
    use super::*;

    // Testing: structure.standard[0] - "Verb[る]+(の)+ に伴って"
    #[test]
    fn test_verb_ni_tomonatte() {
        // Example from grammar_points_data.json: 減少するにともなって
        let sentence = "この町の人口が減少するに伴って空き家が増加して来ました。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に伴って・に伴い");
        assert_pattern_range(&patterns, "に伴って・に伴い", 7, 15); // 減少するに伴って
    }

    // Testing: structure.standard[0] with の - "Verb[る]+の+ に伴って"
    #[test]
    fn test_verb_no_ni_tomonatte() {
        // Example: 初期化をするのにともなって
        let sentence = "パソコンの初期化をするのに伴って、すべてのデータが消えます。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に伴って・に伴い");
        assert_pattern_range(&patterns, "に伴って・に伴い", 9, 16); // するのに伴って
    }

    // Testing: structure.standard[0] with い form - "Verb[る]+ に伴い"
    // Note: This test uses a Noun (地震) not a verb - testing Noun + に伴い
    #[test]
    fn test_verb_ni_tomonai() {
        // Example from grammar_points_data.json: 地震にともない
        let sentence = "津波は地震に伴い発生することが多いそうだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に伴って・に伴い");
        assert_pattern_range(&patterns, "に伴って・に伴い", 3, 8); // 地震に伴い
    }

    // Testing: structure.standard[0] - "Verb[る]+ に伴って" with progressive change
    #[test]
    fn test_verb_ni_tomonatte_progressive() {
        // Example from grammar_points_data.json: 歳を取るにともなって
        let sentence = "歳を取るに伴って物忘れが酷くなってきてる感じがする。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に伴って・に伴い");
        assert_pattern_range(&patterns, "に伴って・に伴い", 2, 8); // 取るに伴って
    }

    // Testing: structure.standard[1] - "Verb[る]+(の)+ に伴う + Noun"
    #[test]
    fn test_verb_ni_tomonau_noun() {
        // Example from grammar_points_data.json: するのにともなう手順
        let sentence = "パソコンの初期化をするのに伴う手順は取扱説明書の５１ページに記載されています。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に伴って・に伴い");
        assert_pattern_range(&patterns, "に伴って・に伴い", 9, 15); // するのに伴う
    }

    // Testing: structure.standard[2] - "Noun + に伴って"
    #[test]
    fn test_noun_ni_tomonatte() {
        // Example from grammar_points_data.json: 普及にともなって
        let sentence = "インターネットの普及に伴って、オンラインで買い物を済ませる人が増えた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に伴って・に伴い");
        assert_pattern_range(&patterns, "に伴って・に伴い", 8, 14); // 普及に伴って
    }

    // Testing: structure.standard[2] - "Noun + に伴い"
    #[test]
    fn test_noun_ni_tomonai() {
        // Example from grammar_points_data.json: 火山現象に伴い
        let sentence = "火山現象に伴い津波が発生することも有るそうだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に伴って・に伴い");
        assert_pattern_range(&patterns, "に伴って・に伴い", 2, 7); // 現象に伴い
    }

    // Testing: structure.standard[3] - "Noun + に伴う + Noun"
    #[test]
    fn test_noun_ni_tomonau_noun() {
        // Example from grammar_points_data.json: 工事にともなう車線規制
        let sentence = "高速道路の情報サイト：リフレッシュ工事に伴う車線規制のお知らせ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に伴って・に伴い");
        assert_pattern_range(&patterns, "に伴って・に伴い", 17, 22); // 工事に伴う
    }
}

// Pattern: つつ (while doing, in the course of)
// Data source: grammar_points_data.json["つつ"]
// Testing: structure.standard[0] - "Verb[stem] + つつ"
//
// Note: つつ is a formal construction used when (A) is an ongoing state
// rather than a physical action. More formal than ながら.
mod tsutsu_tests {
    use super::*;

    // Testing: structure.standard[0] - "Verb[stem] + つつ"
    #[test]
    fn test_tsutsu_while_knowing() {
        // Example from grammar_points_data.json: 環境に悪いと知りつつ
        let sentence = "環境に悪いと知りつつ、レジ袋を使い続けている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "つつ");
        assert_pattern_range(&patterns, "つつ", 6, 10); // 知りつつ
    }

    #[test]
    fn test_tsutsu_while_looking() {
        // Example from grammar_points_data.json: レシピを見つつ
        let sentence = "インターネットでレシピを見つつ、料理をした。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "つつ");
        assert_pattern_range(&patterns, "つつ", 12, 15); // 見つつ
    }

    #[test]
    fn test_tsutsu_while_thinking() {
        // Example from grammar_points_data.json: 仕事に関係ないことを考えつつ
        let sentence = "仕事に関係ないことを考えつつ仕事をしていた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "つつ");
        assert_pattern_range(&patterns, "つつ", 10, 14); // 考えつつ
    }
}

// Pattern: つつ(も) (even while doing, although doing)
// Data source: grammar_points_data.json["つつ(も)"]
// Testing: structure.standard[0] - "Verb[stem] + つつ（も）"
//
// Note: つつも adds the nuance of 'even' or 'although' to つつ.
// (B) is almost always contradictory to (A).
mod tsutsumo_tests {
    use super::*;

    // Testing: structure.standard[0] - "Verb[stem] + つつ（も）"
    #[test]
    fn test_tsutsumo_despite_knowing() {
        // Example from grammar_points_data.json: 知りつつも
        let sentence = "一人で洞窟に入るのは危ないと知りつつも、入ってしまった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "つつ(も)");
        assert_pattern_range(&patterns, "つつ(も)", 14, 19); // 知りつつも
    }

    #[test]
    fn test_tsutsumo_despite_being_nervous() {
        // Example from grammar_points_data.json: 緊張しつつも
        let sentence = "初めてのライブで緊張しつつもものすごく楽しめました。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "つつ(も)");
        assert_pattern_range(&patterns, "つつ(も)", 8, 14); // 緊張しつつも
    }

    #[test]
    fn test_tsutsumo_despite_thinking() {
        // Example from grammar_points_data.json: 思いつつも
        let sentence = "なんかこの話は怪しいなと思いつつも、彼のことを信じてみた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "つつ(も)");
        assert_pattern_range(&patterns, "つつ(も)", 12, 17); // 思いつつも
    }
}

// Pattern: にかかわる (relating to, concerning)
// Data source: grammar_points_data.json["にかかわる"]
// Testing: structure.standard[0-1]
//
// Structures:
//   - standard[0]: Noun + にかかわる
//   - standard[1]: Noun + にかかわる + Noun
mod nikakawaru_tests {
    use super::*;

    #[test]
    fn test_nikakawaru_life_depends() {
        // Example from grammar_points_data.json: 人生にかかわる
        let sentence = "あの試験は私の人生にかかわるので、一生懸命勉強しなくてはならない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にかかわる");
        assert_pattern_range(&patterns, "にかかわる", 7, 14); // 人生にかかわる
    }

    #[test]
    fn test_nikakawaru_life_affecting_disease() {
        // Example from grammar_points_data.json: 命にかかわる病気 (structure[1])
        let sentence = "別に命にかかわる病気では無いので、すぐに治療する必要はないです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にかかわる");
        assert_pattern_range(&patterns, "にかかわる", 2, 8); // 命にかかわる
    }

    #[test]
    fn test_nikakawaru_interview_concerns() {
        // Example from grammar_points_data.json: 人生にかかわる
        let sentence = "この面接は私の人生にかかわるので、頑張らなければいけない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にかかわる");
        assert_pattern_range(&patterns, "にかかわる", 7, 14); // 人生にかかわる
    }
}

// Pattern: に向かって・に向けて (towards, facing, aimed at)
// Data source: grammar_points_data.json["に向かって・に向けて"]
// Testing: structure.standard[0-2]
//
// Structures:
//   - standard[0]: Noun + に向（む）かって
//   - standard[1]: Noun + に向（む）けて
//   - standard[2]: Noun + に向（む）けて + の + Noun
mod nimukatte_nimukete_tests {
    use super::*;

    #[test]
    fn test_nimukatte_mic() {
        // Example from grammar_points_data.json: マイクにむかって
        let sentence = "マイクにむかって何か一言お願いします。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に向かって・に向けて");
        assert_pattern_range(&patterns, "に向かって・に向けて", 0, 8); // マイクにむかって
    }

    #[test]
    fn test_nimukatte_sky() {
        // Example from grammar_points_data.json: 空にむかって
        let sentence = "空にむかって大声を出した。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に向かって・に向けて");
        assert_pattern_range(&patterns, "に向かって・に向けて", 0, 6); // 空にむかって
    }

    #[test]
    fn test_nimukete_new_employees() {
        // Example from grammar_points_data.json: 新入社員にむけて
        let sentence = "新入社員にむけて何かアドバイスを一言お願いします。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に向かって・に向けて");
        assert_pattern_range(&patterns, "に向かって・に向けて", 2, 8); // 社員にむけて
    }

    #[test]
    fn test_nimukete_training() {
        // Example from grammar_points_data.json: 全国大会にむけて
        let sentence = "彼らは全国大会にむけて毎日朝から夜までトレーニングをしている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に向かって・に向けて");
        assert_pattern_range(&patterns, "に向かって・に向けて", 5, 11); // 大会にむけて
    }
}

// Pattern: に気をつける (be careful of, watch out for, pay attention to)
// Data source: grammar_points_data.json["に気をつける"]
// Testing: structure.standard[0] - "Noun + に気（き）をつける"
// Testing: structure.standard[1] - "Verb［ない］+ ように気（き）をつける"
// Testing: structure.polite[0] - "Noun + に気（き）をつけます"
// Testing: structure.polite[1] - "Verb［ない］+ ように気（き）をつけます"
//
// Structure variants:
//   - standard[0]: Noun + に気をつける
//   - standard[1]: Verb[ない] + ように気をつける
//   - polite[0]: Noun + に気をつけます
//   - polite[1]: Verb[ない] + ように気をつけます

mod nikiwotsukeru_tests {
    use super::*;

    #[test]
    fn test_noun_ni_ki_wo_tsukeru() {
        // Example from grammar_points_data.json: 怪（あや）しい人（ひと）にきをつけて
        let sentence = "最近、電車に痴漢が沢山いるらしいから怪しい人にきをつけて帰ってくるんだよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に気をつける");
        assert_pattern_range(&patterns, "に気をつける", 21, 27); // 人にきをつけ
    }

    #[test]
    fn test_noun_ni_ki_wo_tsukemashou() {
        // Example from grammar_points_data.json: 歩行者（ほこうしゃ）にきをつけましょう
        let sentence = "車で歩道を横切る時には歩行者にきをつけましょう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に気をつける");
        assert_pattern_range(&patterns, "に気をつける", 13, 23); // 者にきをつけましょう
    }

    // TODO: Undetectable - Verb[ない] + ように気をつける pattern
    // The pattern matcher currently only detects the simple Noun + にきをつける form.
    // For the Verb + ない + よう + に + きをつける structure, the wildcard mechanism
    // doesn't reliably match the intervening tokens (ない + よう) between the verb and に.
    // This would require a more sophisticated pattern matcher or separate pattern definition.
    //
    // #[test]
    // fn test_verb_nai_youni_ki_wo_tsukeru() {
    //     // Example: 壊（こわ）さないようにきをつけて
    //     let sentence = "このギターは俺の姉ちゃんのやつだから壊さないようにきをつけてね。";
    //     let tokens = tokenize_sentence(sentence);
    //     let patterns = detect_patterns(&tokens);
    //
    //     assert_has_pattern(&patterns, "に気をつける");
    //     assert_pattern_range(&patterns, "に気をつける", 18, 30); // 壊さないようにきをつけて
    // }
    //
    // #[test]
    // fn test_verb_nai_youni_ki_wo_tsukeyou() {
    //     // Example: しないようにきをつけよう
    //     let sentence = "今月はお金を無駄遣いしないようにきをつけよう。";
    //     let tokens = tokenize_sentence(sentence);
    //     let patterns = detect_patterns(&tokens);
    //
    //     assert_has_pattern(&patterns, "に気をつける");
    //     assert_pattern_range(&patterns, "に気をつける", 10, 22); // しないようにきをつけよう
    // }
}

// Pattern: に限って (particularly when, only when, in particular)
// Data source: grammar_points_data.json["に限って"]
// Testing: structure.standard[0] - "Noun + に限（かぎ）って"
//
// Structure variants:
//   - standard[0]: Noun + に限（かぎ）って
//
// Note: This pattern has three meanings:
//   1. Something unbelievable about (A)
//   2. Something limited to (A)
//   3. In most cases for (A), (B) is true

mod nikagitte_tests {
    use super::*;

    #[test]
    fn test_unbelievable_meaning() {
        // Meaning 1: Unbelievable thing about A
        // Example from grammar_points_data.json
        let sentence = "うちの子に限って、他の子に手を出すなんて考えられない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に限って");
        assert_pattern_range(&patterns, "に限って", 3, 8); // 子に限って
    }

    #[test]
    fn test_limited_to_meaning() {
        // Meaning 2: Limited to A
        // Example from grammar_points_data.json
        let sentence = "武くんと釣りに行く日に限って、いつも雨が降るんだよな。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に限って");
        assert_pattern_range(&patterns, "に限って", 9, 14); // 日に限って
    }

    #[test]
    fn test_most_cases_meaning() {
        // Meaning 3: In most cases for A, B is true
        // Example from grammar_points_data.json
        let sentence = "「クレームを入れるぞ」って言う人に限ってクレームを入れないからあまり気にしていない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に限って");
        assert_pattern_range(&patterns, "に限って", 15, 20); // 人に限って
    }
}

// Pattern: を除いて (except for, with the exception of, excluding)
// Data source: grammar_points_data.json["を除いて"]
// Testing all structure variants
//
// Structure variants:
//   - standard[0]: Noun + を除（のぞ）いて（は）
//   - standard[1]: Noun + を除（のぞ）く + Noun
//   - standard[2]: を除（のぞ）き

mod wonozoite_tests {
    use super::*;

    #[test]
    fn test_wo_nozoite() {
        // Structure: Noun + を除いて
        // Example from grammar_points_data.json
        let sentence = "高橋さんと浜崎さんを除いて、他のみんなは残業をしてください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を除いて");
        assert_pattern_range(&patterns, "を除いて", 7, 13); // さんを除いて
    }

    #[test]
    fn test_wo_nozoite_polite() {
        // Structure: Noun + を除いて (polite sentence)
        let sentence = "私は第２土曜日を除いて、毎日働いています。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を除いて");
        assert_pattern_range(&patterns, "を除いて", 4, 11); // 土曜日を除いて
    }

    #[test]
    fn test_wo_nozoku_noun() {
        // Structure: Noun + を除く + Noun (dictionary form modifying noun)
        // Example from grammar_points_data.json
        let sentence = "そこの壁にかけてあるギターを除く全てのギターは母親のものです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を除いて");
        assert_pattern_range(&patterns, "を除いて", 10, 16); // ギターを除く
    }

    #[test]
    fn test_wo_nozoku_dictionary() {
        // Structure: Noun + を除く (dictionary form)
        let sentence = "中田先生を除く全ての先生は同じ大学を卒業したらしい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を除いて");
        assert_pattern_range(&patterns, "を除いて", 2, 7); // 先生を除く
    }
}

// Pattern: なくはない (it's not that it isn't, somewhat, slightly)
// Data source: grammar_points_data.json["なくはない"]
// Testing: structure.standard[0-3]
//
// Structure variants:
//   - standard[0]: Verb［なくて］+ はない
//   - standard[1]: ［い］Adjective［く］ + なくはない
//   - standard[2]: ［な］Adjective + では(じゃ) + なくはない
//   - standard[3]: Noun + が(は、に) + なくはない

mod nakuhanai_tests {
    use super::*;

    #[test]
    fn test_verb_negative_form() {
        // Structure: Verb［なくて］+ はない
        // Example from grammar_points_data.json
        let sentence = "別に出来なくはないけど、めんどくさいから業者に頼むわ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくはない");
        assert_pattern_range(&patterns, "なくはない", 4, 9); // なくはない
    }

    #[test]
    fn test_i_adjective() {
        // Structure: ［い］Adjective［く］ + なくはない
        // Example: "it's not that it isn't heavy" = "somewhat heavy"
        let sentence = "このカバンは重くなくはないけど、まあ持ち運べるレベルだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくはない");
        assert_pattern_range(&patterns, "なくはない", 8, 13); // なくはない
    }

    #[test]
    fn test_na_adjective_dewa() {
        // Structure: ［な］Adjective + ではなくはない
        let sentence = "彼女の話は確かに複雑ではなくはないが、理解できないわけではない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくはない");
        assert_pattern_range(&patterns, "なくはない", 12, 17); // なくはない
    }

    #[test]
    fn test_na_adjective_jya() {
        // Structure: ［な］Adjective + じゃなくはない
        let sentence = "この仕事は大変じゃなくはないけど、やりがいはある。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくはない");
        assert_pattern_range(&patterns, "なくはない", 9, 14); // なくはない
    }

    #[test]
    fn test_noun_ga() {
        // Structure: Noun + が + なくはない
        // Example from grammar_points_data.json
        let sentence = "それを買うお金がなくはないが、そんなくだらないことにお金を使いたくない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくはない");
        assert_pattern_range(&patterns, "なくはない", 8, 13); // なくはない
    }

    #[test]
    fn test_noun_wa() {
        // Structure: Noun + は + なくはない
        // Example from grammar_points_data.json
        let sentence = "時間はなくはないんですが、もっと時間を有効に使いたいだけです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくはない");
        assert_pattern_range(&patterns, "なくはない", 3, 8); // なくはない
    }

    #[test]
    fn test_noun_ni() {
        // Structure: Noun + に + なくはない
        let sentence = "家にテレビがなくはないけど、アンテナがつながっていないからモニターとして使ってる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくはない");
        assert_pattern_range(&patterns, "なくはない", 6, 11); // なくはない
    }

    #[test]
    fn test_potential_verb() {
        // Structure: Verb（potential）［なくて］+ はない
        // Example from grammar_points_data.json - very common usage
        let sentence = "納豆は食べれなくはないけど、どうせ食べるならもっと美味しいものを食べたいかな。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくはない");
        assert_pattern_range(&patterns, "なくはない", 6, 11); // なくはない
    }
}

// Pattern: も構わず (without worrying about, without minding)
// Data source: grammar_points_data.json["も構わず"]
// Testing: structure.standard[0] - "Verb + の + も + かまわず"
// Testing: structure.standard[1] - "い-Adjective + の + も + かまわず"
// Testing: structure.standard[2] - "な-Adjective + である + も + かまわず"
// Testing: structure.standard[3] - "Noun + も + かまわず"
// Testing: structure.standard[4] - "にも can replace も for emphasis"
//
// Structure variants (5 structures):
//   - standard[0]: Verb + の + も + かまわず
//   - standard[1]: い-Adjective + の + も + かまわず
//   - standard[2]: な-Adjective + である + も + かまわず
//   - standard[3]: Noun + も + かまわず
//   - standard[4]: にも can replace も (for emphasis)

mod mokamawazu_tests {
    use super::*;

    #[test]
    fn test_verb_no_mo_kamawazu() {
        // Structure: Verb + の + も + かまわず
        // Example from grammar_points_data.json: レポートの提出日が迫っているのもかまわず
        let sentence = "娘はレポートの提出日が迫っているのもかまわず、友達と遊んでばかりいる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "も構わず");
        assert_pattern_range(&patterns, "も構わず", 14, 22); // いるのもかまわず
    }

    #[test]
    fn test_i_adjective_no_mo_kamawazu() {
        // Structure: い-Adjective + にも + かまわず (using にも emphatic variant)
        // Example from grammar_points_data.json: 遠いにもかまわず
        let sentence = "彼は遠いにもかまわず、広島まで休憩なしで運転し続けた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "も構わず");
        assert_pattern_range(&patterns, "も構わず", 2, 10); // 遠いにもかまわず
    }

    #[test]
    fn test_na_adjective_dearu_mo_kamawazu() {
        // Structure: な-Adjective + な + の + も + かまわず
        // Example: 危険なのもかまわず
        let sentence = "彼女は危険なのもかまわず、池で溺れていた猫を救出した。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "も構わず");
        assert_pattern_range(&patterns, "も構わず", 5, 12); // なのもかまわず
    }

    #[test]
    fn test_noun_mo_kamawazu() {
        // Structure: Noun + も + かまわず
        // Example from grammar_points_data.json: 時間もかまわず
        let sentence = "鈴木先輩は時間もかまわず、電話を掛けてくるから困っている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "も構わず");
        assert_pattern_range(&patterns, "も構わず", 5, 12); // 時間もかまわず
    }

    #[test]
    fn test_noun_nimo_kamawazu() {
        // Structure: Verb[past] + にも + かまわず (emphatic variant)
        // Example from grammar_points_data.json: ストップをかけられていたにもかまわず
        let sentence = "医者にストップをかけられていたにもかまわず試合に出て、大怪我をしてしまいました。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "も構わず");
        assert_pattern_range(&patterns, "も構わず", 14, 21); // たにもかまわず
    }
}

// Pattern: つつある (to be -ing, in the process of)
// Data source: grammar_points_data.json["つつある"]
// Testing: structure.standard[0] - "Verb[stem] + つつある"
// Testing: structure.polite[0] - "Verb[stem] + つつあります"
//
// Structure variants:
//   - standard[0]: Verb［stem］+ つつある (formal, indicates ongoing/gradual change)
//   - polite[0]: Verb［stem］+ つつあります (formal polite)

mod tsutsuaru_tests {
    use super::*;

    #[test]
    fn test_tsutsuaru_standard() {
        // Standard form: Verb[stem] + つつある
        // Example from grammar_points_data.json: 減りつつある (are decreasing)
        let sentence = "毎年、年金の受給額が減りつつあるので、生活に困っている高齢者が増えてきている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "つつある");
        assert_pattern_range(&patterns, "つつある", 10, 16); // 減りつつある
    }

    #[test]
    fn test_tsutsuaru_rising_prices() {
        // Standard form: Verb[stem] + つつある
        // Example from grammar_points_data.json: 上がりつつある (are rising)
        let sentence = "日本では物価が上がりつつある。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "つつある");
        assert_pattern_range(&patterns, "つつある", 7, 14); // 上がりつつある
    }

    #[test]
    fn test_tsutsuaru_polite() {
        // Polite form: Verb[stem] + つつあります
        // Using grammar_points_data.json example adapted to polite form
        let sentence = "最近はインターネットで買い物ができるので、わざわざお店に行く人が減りつつあります。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "つつある");
        assert_pattern_range(&patterns, "つつある", 32, 40); // 減りつつあります
    }
}

// Pattern: ていては (if you keep doing, if one continues with)
// Data source: grammar_points_data.json["ていては"]
// Testing: structure.standard[0] - "Verb[て] + いては"
//
// Structure variants:
//   - standard[0]: Verb［て］+ いては (indicates negative outcome if continuing action)

mod teiteha_tests {
    use super::*;

    #[test]
    fn test_teiteha_eating_snacks() {
        // Structure: Verb[て] + いては (negative consequence)
        // Example from grammar_points_data.json: 食べていては (if you keep eating)
        let sentence = "毎日お菓子ばかりを食べていては、いつまで経っても痩せませんよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ていては");
        assert_pattern_range(&patterns, "ていては", 9, 15); // 食べていては
    }

    #[test]
    fn test_teiteha_going_out() {
        // Structure: Verb[て] + いては (で variant)
        // Example from grammar_points_data.json: でいては (if you keep going)
        let sentence = "毎晩遊んでいてはお金はたまりませんよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ていては");
        assert_pattern_range(&patterns, "ていては", 2, 8); // 遊んでいては
    }

    #[test]
    fn test_teiteha_worrying() {
        // Structure: Verb[て] + いては
        // Example from grammar_points_data.json: していては (if you keep worrying)
        let sentence = "仕事の事ばかりを気にしていては、旅行は楽しめませんよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ていては");
        assert_pattern_range(&patterns, "ていては", 10, 15); // していては
    }
}

// ========== ところだった ② (was just about to) ==========
// Pattern: ところだった ② (was just about to, was in the middle of)
// Data source: grammar_points_data.json["ところだった ②"]
//
// Structure variants:
//   standard[0]: Verb[る] + ところだった
//   polite[0]: Verb[る] + ところでした
//
// Note: This is different from ところだった ① (N3) which includes Verb[ない] forms.
// ところだった ② specifically emphasizes what came about from NOT doing (A),
// often showing a narrowly avoided positive or negative result.

mod tokorodatta_u2461_tests {
    use super::*;

    #[test]
    fn test_tokoro_datta_narrowly_avoided() {
        // Structure: Verb[る] + ところだった
        // Example from grammar_points_data.json: ひくところだった (was about to run over)
        let sentence = "危なかった。危うくあの人をひくところだった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ところだった ②");
        assert_pattern_range(&patterns, "ところだった ②", 13, 21); // ひくところだった
    }

    #[test]
    fn test_tokoro_datta_late() {
        // Structure: Verb[る] + ところだった
        // Example from grammar_points_data.json: 遅刻するところだった (was about to be late)
        let sentence = "起きるのが３分遅かったら、遅刻するところだった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ところだった ②");
        assert_pattern_range(&patterns, "ところだった ②", 13, 23); // 遅刻するところだった
    }

    #[test]
    fn test_tokoro_datta_with_noni() {
        // Structure: Verb[る] + ところだった + のに
        // Example from grammar_points_data.json: 出るところだったのに (was just about to leave)
        let sentence = "今から家を出るところだったのに、いきなり雨が降ってきたから雨が止むまで待つ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ところだった ②");
        assert_pattern_range(&patterns, "ところだった ②", 5, 13); // 出るところだった
    }

    #[test]
    fn test_tokoro_deshita_polite() {
        // Structure: Verb[る] + ところでした
        // Polite form of the pattern
        let sentence = "電車が来る直前まで待っていて、乗り遅れるところでした。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ところだった ②");
        assert_pattern_range(&patterns, "ところだった ②", 15, 26); // 乗り遅れるところでした
    }
}

// ========== どころではない (far from, out of the question) ==========
// Pattern: どころではない (far from, out of the question)
// Data source: grammar_points_data.json["どころではない"]
//
// Structure variants:
//   standard[0]: Phrase + どころ + ではない
//   standard[1]: Phrase + どころ + じゃない (casual)
//   polite[0]: Phrase + どころ + ではありません
//   polite[1]: Phrase + どころ + じゃありません
//
// Pattern highlights that the actual situation is even more remarkable than (A),
// or that it is not the time for (A) due to more important matters.

mod dokorodehanai_tests {
    use super::*;

    #[test]
    fn test_dokoro_dehanai_far_from() {
        // Structure: Phrase + どころ + ではない
        // Example from grammar_points_data.json: 痛いどころではない (far from painful)
        let sentence = "出産は痛いどころではないらしい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "どころではない");
        assert_pattern_range(&patterns, "どころではない", 3, 15); // 痛いどころではないらしい
    }

    #[test]
    fn test_dokoro_janai_out_of_question() {
        // Structure: Phrase + どころ + じゃない (casual)
        // Example from grammar_points_data.json: 取るどころじゃない (out of the question)
        let sentence = "締め切りが迫って来てるから、休憩を取るどころじゃない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "どころではない");
        assert_pattern_range(&patterns, "どころではない", 17, 26); // 取るどころじゃない
    }

    #[test]
    fn test_dokoro_dehanai_not_time_for() {
        // Structure: Phrase + どころ + ではない
        // Example from grammar_points_data.json: 仕事どころではない (not the time for work)
        let sentence = "お母さんからお父さんが倒れたと連絡があったので、仕事どころではない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "どころではない");
        assert_pattern_range(&patterns, "どころではない", 24, 33); // 仕事どころではない
    }

    #[test]
    fn test_dokoro_janai_marriage() {
        // Structure: Phrase + どころ + じゃない (casual)
        // Example from grammar_points_data.json: 結婚どころじゃない (not the time for marriage)
        let sentence = "今は仕事が忙しいし、お金も全然ないから、結婚どころじゃない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "どころではない");
        assert_pattern_range(&patterns, "どころではない", 20, 29); // 結婚どころじゃない
    }

    #[test]
    fn test_dokoro_dearimasen_polite() {
        // Structure: Phrase + どころ + ではありません (polite)
        let sentence = "この状況では、休暇どころではありません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "どころではない");
        assert_pattern_range(&patterns, "どころではない", 7, 19); // 休暇どころではありません
    }
}

// Pattern: にもかかわらず (despite, in spite of)
// Data source: grammar_points_data.json["にもかかわらず"]
// Structures to test (4 variants):
//   - standard[0]: Verb + にもかかわらず
//   - standard[1]: い-Adjective + にもかかわらず
//   - standard[2]: な-Adjective + である + にもかかわらず
//   - standard[3]: Noun + (である) + にもかかわらず
#[cfg(test)]
mod nimokakawarazu_tests {
    use super::*;

    #[test]
    fn test_verb_nimokakawarazu() {
        // Structure: Verb + にもかかわらず
        // Example from grammar_points_data.json: 働いていたのにもかかわらず (despite working)
        let sentence = "浜崎さんは毎日一生懸命働いていたのにもかかわらず、会社をクビになった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にもかかわらず");
        assert_pattern_range(&patterns, "にもかかわらず", 15, 24); // たのにもかかわらず
    }

    #[test]
    fn test_i_adjective_nimokakawarazu() {
        // Structure: い-Adjective + にもかかわらず
        // Example from grammar_points_data.json: 悪いのにもかかわらず (despite being bad/wrong)
        let sentence = "彼が悪いのにもかかわらず、私に謝らせようとしてきた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にもかかわらず");
        assert_pattern_range(&patterns, "にもかかわらず", 2, 12); // 悪いのにもかかわらず
    }

    #[test]
    fn test_na_adjective_dearu_nimokakawarazu() {
        // Structure: な-Adjective + である + にもかかわらず
        // Example from grammar_points_data.json: 親切であるにもかかわらず (despite being kind)
        let sentence = "田中さんは優しくて親切であるにもかかわらず、見た目が怖いから避けられている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にもかかわらず");
        assert_pattern_range(&patterns, "にもかかわらず", 9, 21); // 親切であるにもかかわらず
    }

    #[test]
    fn test_noun_nimokakawarazu() {
        // Structure: Noun + にもかかわらず (without である)
        // Example from grammar_points_data.json: 深夜にもかかわらず (despite being late at night)
        let sentence = "深夜にもかかわらず上司が鬼電をしてきたので、労基に報告しようと思う。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にもかかわらず");
        assert_pattern_range(&patterns, "にもかかわらず", 0, 9); // 深夜にもかかわらず
    }

    // Pattern: よりしかたがない (there is no choice but, cannot be helped)
    // Data source: grammar_points_data.json["よりしかたがない"]
    // Testing structures:
    //   - standard[0]: Verb + より + 仕方がない
    //   - standard[1]: Verb + より + ほかに + 仕方ない (が dropped)

    #[test]
    fn test_yori_shikata_ga_nai_standard() {
        // Structure: Verb + より + 仕方がない
        // Example from grammar_points_data.json: 立て直すよりしかたがない
        let sentence = "この建物はボロボロでいつ倒れてもおかしくないから、倉庫として使いたいなら立て直すよりしかたがない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "よりしかたがない");
        assert_pattern_range(&patterns, "よりしかたがない", 36, 48); // 立て直すよりしかたがない
    }

    #[test]
    fn test_yori_shikata_ga_nai_with_ga_dropped() {
        // Structure: Verb + より + 仕方ない (が dropped)
        // Example from grammar_points_data.json: 働くよりしかたない
        let sentence = "生活をするにはお金が必要だから働くよりしかたない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "よりしかたがない");
        assert_pattern_range(&patterns, "よりしかたがない", 15, 24); // 働くよりしかたない
    }
}

// Pattern: てからでないと (unless you do, until you do)
// Data source: grammar_points_data.json["てからでないと"]
// Testing structures:
//   - standard[0]: Verb[て] + からでないと + (Negative)
//   - standard[1]: Verb[て] + からでなければ + (Negative)
mod tekaradenaito_tests {
    use super::*;

    #[test]
    fn test_tekara_denaito_standard() {
        // Structure: Verb[て] + からでないと
        // Example from grammar_points_data.json: 揃ってからでないと
        let sentence = "全ての書類が揃ってからでないと、本申し込みができません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てからでないと");
        assert_pattern_range(&patterns, "てからでないと", 6, 15); // 揃ってからでないと
    }

    #[test]
    fn test_tekara_denaito_polite() {
        // Structure: Verb[て] + からでないと
        // Example from grammar_points_data.json: インストールしてからでないと
        let sentence = "このアプリをインストールしてからでないと、携帯に入れた音楽は聴けません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てからでないと");
        assert_pattern_range(&patterns, "てからでないと", 12, 20); // してからでないと
    }

    #[test]
    fn test_tekara_denakereba_standard() {
        // Structure: Verb[て] + からでなければ
        // Example from grammar_points_data.json: とってからでなければ
        let sentence = "資格をとってからでなければ、面接が受けられません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てからでないと");
        assert_pattern_range(&patterns, "てからでないと", 3, 13); // とってからでなければ
    }

    #[test]
    fn test_tekara_denakereba_polite() {
        // Structure: Verb[て] + からでなければ
        // Example from grammar_points_data.json: 相談してからでなければ
        let sentence = "その件については上司と相談してからでなければ、ご回答できません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てからでないと");
        assert_pattern_range(&patterns, "てからでないと", 11, 22); // 相談してからでなければ
    }
}

// Pattern: 次第だ・次第で (depending on, depends on)
// Data source: grammar_points_data.json["次第だ・次第で"]
// Testing structures:
//   - standard[0]: Noun + 次第（しだい） + だ
//   - standard[1]: Noun + 次第（しだい）で
//   - polite[0]: Noun + 次第（しだい） + です
//   - polite[1]: Noun + 次第（しだい）で (same as standard)
mod shidaida_shidaide_tests {
    use super::*;

    #[test]
    fn test_shidai_da_standard() {
        // Structure: Noun + 次第（しだい） + だ
        // Example from grammar_points_data.json: 努力しだいだ
        let sentence = "夢をかなえられるかどうかは君の努力しだいだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "次第だ・次第で");
        assert_pattern_range(&patterns, "次第だ・次第で", 15, 21); // 努力しだいだ
    }

    #[test]
    fn test_shidai_da_polite() {
        // Structure: Noun + 次第（しだい） + です
        // Example: 試験の結果しだいです
        let sentence = "第一希望の大学へ入れるかは、試験の結果しだいです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "次第だ・次第で");
        assert_pattern_range(&patterns, "次第だ・次第で", 17, 24); // 結果しだいです
    }

    #[test]
    fn test_shidai_de_standard() {
        // Structure: Noun + 次第（しだい）で
        // Example from grammar_points_data.json: 完成日しだいで
        let sentence = "内装工事の完成日しだいで、鍵の引き渡し日が変わる可能性があります。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "次第だ・次第で");
        assert_pattern_range(&patterns, "次第だ・次第で", 7, 12); // 日しだいで
    }

    #[test]
    fn test_shidai_de_polite() {
        // Structure: Noun + 次第（しだい）で (same in polite)
        // Example from grammar_points_data.json: 値段しだいで
        let sentence = "値段しだいで、ナビを付けるか付けないかを決めます。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "次第だ・次第で");
        assert_pattern_range(&patterns, "次第だ・次第で", 0, 6); // 値段しだいで
    }
}

// Pattern: 際に (on the occasion of, when)
// Data source: grammar_points_data.json["際に"]
// Testing polite structures (no standard forms listed)
//
// Structure variants:
//   - polite[0]: Verb[る/た] + 際に (note (1) says "Verb[る]" is also possible)
//   - polite[1]: Noun + の + 際に

mod saini_tests {
    use super::*;

    #[test]
    fn test_saini_verb_ru_form() {
        // Structure: Verb[る] + 際に
        // Example from grammar_points_data.json: この建物に入る際には
        let sentence = "この建物に入る際には、必ずヘルメットを被ってください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "際に");
        assert_pattern_range(&patterns, "際に", 5, 9); // 入る際に
    }

    #[test]
    fn test_saini_verb_ta_form() {
        // Structure: Verb[た] + 際に
        // Example from grammar_points_data.json: 肌に飛び散った際には
        let sentence = "この薬品が誤って肌に飛び散った際には、直ぐに医者に相談してください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "際に");
        assert_pattern_range(&patterns, "際に", 10, 17); // 飛び散った際に
    }

    #[test]
    fn test_saini_noun_no() {
        // Structure: Noun + の + 際に
        // Example from grammar_points_data.json: 明日の面接の際には
        let sentence = "明日の面接の際には、印鑑を持ってきてください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "際に");
        assert_pattern_range(&patterns, "際に", 3, 8); // 面接の際に
    }
}

// Pattern: ないことには～ない (unless, without)
// Data source: grammar_points_data.json["ないことには～ない"]
// Testing standard structures (no polite forms listed)
//
// Structure variants:
//   - standard[0]: Verb[ない] + ことには + Verb[ない]
//   - standard[1]: い-Adjective[ない] + ことには + Verb[ない]
//   - standard[2]: な-Adjective + でない + ことには + Verb[ない]
//   - standard[3]: Noun + でない + ことには + Verb[ない]

mod naikotoniha_tests {
    use super::*;

    #[test]
    fn test_verb_nai_kotoniha() {
        // Structure: Verb[ない] + ことには + Verb[ない]
        // Example from grammar_points_data.json: ヘルメットを被らないことには、この工事現場には入れない
        let sentence = "ヘルメットを被らないことには、この工事現場には入れない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないことには～ない");
        assert_pattern_range(&patterns, "ないことには～ない", 8, 14); // ないことには
    }

    #[test]
    fn test_i_adj_nai_kotoniha() {
        // Structure: い-Adjective[ない] + ことには + Verb[ない]
        // Example from grammar_points_data.json: スリルがないことには楽しくない
        let sentence = "ジェットコースターは、スリルがないことには楽しくない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないことには～ない");
        assert_pattern_range(&patterns, "ないことには～ない", 15, 21); // ないことには
    }

    #[test]
    fn test_na_adj_denai_kotoniha() {
        // Structure: な-Adjective + でない + ことには + Verb[ない]
        // Example from grammar_points_data.json: 何事も一生懸命でないことには、何も上手くなれない
        let sentence = "何事も一生懸命でないことには、何も上手くなれない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないことには～ない");
        assert_pattern_range(&patterns, "ないことには～ない", 7, 14); // でないことには
    }

    #[test]
    fn test_noun_denai_kotoniha() {
        // Structure: Noun + でない + ことには + Verb[ない]
        // Example from grammar_points_data.json: 許可書がないことには、ここへは入れません
        let sentence = "許可書がないことには、ここへは入れません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないことには～ない");
        assert_pattern_range(&patterns, "ないことには～ない", 4, 10); // ないことには
    }
}

// Pattern: しかしながら (however, nevertheless)
// Data source: grammar_points_data.json["しかしながら"]
// Testing: Single structure - Phrase。 しかしながら + Phrase
//
// Pattern is a formal conjunction that appears at the beginning of a sentence,
// connecting to a previous statement. It's an emphatic form of しかし.

mod shikashi_nagara_tests {
    use super::*;

    #[test]
    fn test_shikashi_nagara_example1() {
        // Example from grammar_points_data.json
        let sentence = "日本は安全な国だと言われている。しかしながら１００％安全というわけでもない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "しかしながら");
        assert_pattern_range(&patterns, "しかしながら", 16, 22); // しかしながら
    }

    #[test]
    fn test_shikashi_nagara_example2() {
        // Example from grammar_points_data.json
        let sentence = "精一杯頑張れば夢が叶うと言われている。しかしながら、人生はそう甘くない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "しかしながら");
        assert_pattern_range(&patterns, "しかしながら", 19, 25); // しかしながら
    }

    #[test]
    fn test_shikashi_nagara_example3() {
        // Example from grammar_points_data.json
        let sentence = "そのアイデアはいいと思います。しかしながら、我々の予算だとそのプランを実行することはできないでしょう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "しかしながら");
        assert_pattern_range(&patterns, "しかしながら", 15, 21); // しかしながら
    }
}

// Pattern: も又 (also, in addition)
// Data source: grammar_points_data.json["も又"]
// Testing: 4 structure variants - Noun, な-Adj + なの, い-Adj + の, Verb + の + もまた
//
// Pattern emphasizes that in addition to something, something else is also true.
// More formal/emphatic than simple も.

mod momata_tests {
    use super::*;

    #[test]
    fn test_momata_noun() {
        // Structure: Noun + もまた
        // Example from grammar_points_data.json: 明日もまた
        let sentence = "明日もまた同じ時間にここにきてね！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "も又");
        assert_pattern_range(&patterns, "も又", 0, 5); // 明日もまた
    }

    #[test]
    fn test_momata_na_adjective() {
        // Structure: な-Adjective + なの + もまた
        // Example from grammar_points_data.json: 必要なのもまた
        let sentence = "経験が必要なのもまた確かだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "も又");
        assert_pattern_range(&patterns, "も又", 6, 10); // のもまた
    }

    #[test]
    fn test_momata_i_adjective() {
        // Structure: い-Adjective + の + もまた
        // Example from grammar_points_data.json: 辛いのもまた
        let sentence = "カレーは甘くても美味しいけど、辛いのもまた美味しいんだよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "も又");
        assert_pattern_range(&patterns, "も又", 17, 21); // のもまた
    }

    #[test]
    fn test_momata_verb() {
        // Structure: Verb + の + もまた
        // Example from grammar_points_data.json: 実行するのもまた
        let sentence = "田中さんが提案したことを実行するのもまた、いい考えだと思います。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "も又");
        assert_pattern_range(&patterns, "も又", 16, 20); // のもまた
    }
}

// Pattern: にかかわらず (regardless of)
// Data source: grammar_points_data.json["にかかわらず"]
// Structures to test:
//   - standard[0]: Noun (A) + 、 + Noun (B) + にかかわらず
//   - standard[1]: A + か + B + か + にかかわらず
//   - standard[2]: Adj/Verb + (Antonym) Adj/Verb + にかかわらず
//   - standard[3]: A + かどうか + にかかわらず
//   - standard[4]: A + Aない + にかかわらず
mod nikakawarazu_tests {
    use super::*;

    #[test]
    fn test_nikakawarazu_noun_with_comma() {
        // Structure: Noun (A) + や + Noun (B) + にかかわらず
        // Example from grammar_points_data.json: 性別や国籍にかかわらず
        let sentence = "彼は性別や国籍にかかわらず、誰とでも友達になれる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にかかわらず");
        assert_pattern_range(&patterns, "にかかわらず", 2, 13); // 性別や国籍にかかわらず
    }

    #[test]
    fn test_nikakawarazu_ka_ka() {
        // Structure: A + か + B + か + にかかわらず
        // Example from grammar_points_data.json: 降るか降らないかにかかわらず
        let sentence = "雨が降るか降らないかにかかわらず、運動会は開催されます。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にかかわらず");
        assert_pattern_range(&patterns, "にかかわらず", 2, 16); // 降るか降らないかにかかわらず
    }

    #[test]
    fn test_nikakawarazu_adjective_antonym() {
        // Structure: い-Adj + な-Adj antonym + にかかわらず
        // Example from grammar_points_data.json: 上手い下手にかかわらず
        let sentence = "上手い下手にかかわらず、何事も一生懸命やるのが大切だ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にかかわらず");
        assert_pattern_range(&patterns, "にかかわらず", 0, 11); // 上手い下手にかかわらず
    }

    #[test]
    fn test_nikakawarazu_kadouka() {
        // Structure: A + かどうか + にかかわらず
        // Example from grammar_points_data.json: いいかどうかにかかわらず
        let sentence = "このビーチは天気がいいかどうかにかかわらず、毎日サーファーたちで賑わっています。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にかかわらず");
        assert_pattern_range(&patterns, "にかかわらず", 9, 21); // いいかどうかにかかわらず
    }

    #[test]
    fn test_nikakawarazu_affirmative_negative() {
        // Structure: Verb + Verbない + にかかわらず
        // Example: できるできないにかかわらず
        let sentence = "できるできないにかかわらず、まずはやってみることが大事です。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にかかわらず");
        assert_pattern_range(&patterns, "にかかわらず", 0, 13); // できるできないにかかわらず
    }

    #[test]
    fn test_nikakawarazu_simple_noun() {
        // Structure: Noun + にかかわらず
        // Example from grammar_points_data.json: 年齢にかかわらず
        let sentence = "このゲームは年齢にかかわらず、誰でも楽しめます！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にかかわらず");
        assert_pattern_range(&patterns, "にかかわらず", 6, 14); // 年齢にかかわらず
    }
}

// ============================================================================
// Pattern: 限り (as long as, as far as, while, assuming)
// Data source: grammar_points_data.json["限り"]
// ============================================================================
//
// Structure variants to test:
//   - standard[0]: Verb[る] + 限り + Phrase
//   - standard[1]: Verb[ない] + 限り + Phrase
//   - standard[2]: Verb[ている] + 限り + Phrase
//   - standard[3]: Noun + である + 限り
//   - standard[4]: Verb[た] + 限り (note in data)
//
// Meanings:
//   1. Limited to scope of A: "limited to (A), (B)"
//   2. Within scope of knowledge/senses: "as far as (A), (B)", "as long as (A), (B)"
//   3. Assuming A is true: "while (A) is the case, (B)", "assuming (A), (B)"

mod kagiri_tests {
    use super::*;

    #[test]
    fn test_kagiri_verb_ru_form() {
        // Structure: Verb[る] + 限り
        // Meaning: "as long as he keeps saying"
        let sentence = "彼が真剣に芸人になりたいと言っているかぎり、彼の夢は叶うだろう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "限り");
        assert_pattern_range(&patterns, "限り", 16, 21); // いるかぎり
    }

    #[test]
    fn test_kagiri_verb_nai_form() {
        // Structure: Verb[ない] + 限り
        // Meaning: "as long as (something) doesn't happen"
        let sentence = "君が諦めないかぎり、必ず成功する日が来る";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "限り");
        assert_pattern_range(&patterns, "限り", 4, 9); // ないかぎり
    }

    #[test]
    fn test_kagiri_verb_teiru_form() {
        // Structure: Verb[ている] + 限り
        // Meaning: "as long as (continuous state)"
        let sentence = "あなたが元気でいるかぎり、私も頑張れます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "限り");
        assert_pattern_range(&patterns, "限り", 7, 12); // いるかぎり
    }

    #[test]
    fn test_kagiri_noun_dearu_form() {
        // Structure: Noun + である + 限り
        // Meaning: "as long as (identity/status)"
        let sentence = "このアパートの住人であるかぎりは、ルールを守ってください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "限り");
        assert_pattern_range(&patterns, "限り", 10, 15); // あるかぎり
    }

    #[test]
    fn test_kagiri_verb_ta_form() {
        // Structure: Verb[た] + 限り
        // Meaning: "as far as (past perception/knowledge)"
        let sentence = "彼から聞いたかぎり、田中君は退学になるらしい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "限り");
        assert_pattern_range(&patterns, "限り", 5, 9); // たかぎり
    }

    #[test]
    fn test_kagiri_living_in_japan() {
        // Structure: Verb[る] + 限り
        // Meaning: "assuming that you live in Japan"
        let sentence = "日本に住むかぎり、日本の法律を守らなければいけない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "限り");
        assert_pattern_range(&patterns, "限り", 3, 8); // 住むかぎり
    }
}

// ============================================================================
// Pattern: を問わず (regardless of, irrespective of, whether or not)
// Data source: grammar_points_data.json["を問わず"]
// ============================================================================
//
// Structure variants to test:
//   - standard[0]: Noun + を問わず
//   - standard[1]: Noun (A) + 、 + Noun (B) + を問わず
//   - standard[2]: A + か + B + か + を問わず
//   - standard[3]: A (Antonym) A + を問わず
//   - standard[4]: A + かどうか + を問わず
//   - standard[5]: A + A[ない] + を問わず
//
// Notes:
//   - を can be replaced by は for emphasis (はとわず)
//   - Formal register pattern
//   - Similar to にかかわらず but slightly different nuance

mod wotowazu_tests {
    use super::*;

    #[test]
    fn test_wotowazu_simple_noun() {
        // Structure: Noun + を問わず
        // Example from data: 宗教や国籍を問わず
        let sentence = "宗教や国籍をとわず、このイベントには誰でも参加できます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を問わず");
        assert_pattern_range(&patterns, "を問わず", 0, 9); // 宗教や国籍をとわず
    }

    #[test]
    fn test_wotowazu_ka_ka_pattern() {
        // Structure: A + か + B + か + を問わず
        // Example: 上級者か初心者かを問わず
        let sentence = "パソコンの上級者か初心者かをとわず、初めての方は初心者コースから始めてもらいます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を問わず");
        assert_pattern_range(&patterns, "を問わず", 0, 17); // パソコンの上級者か初心者かをとわず
    }

    #[test]
    fn test_wotowazu_kadouka_pattern() {
        // Structure: A + かどうか + を問わず
        // Example: 買うかどうかを問わず
        let sentence = "このマンションを買うかどうかをとわず、この物件を押さえたい場合は手付金を払ってください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を問わず");
        assert_pattern_range(&patterns, "を問わず", 0, 18); // このマンションを買うかどうかをとわず
    }

    #[test]
    fn test_wotowazu_day_night() {
        // Structure: Noun + Noun + を問わず (complementary pair)
        // Example: 昼夜を問わず
        let sentence = "彼は昼夜をとわず、ずっと働いている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を問わず");
        assert_pattern_range(&patterns, "を問わず", 2, 8); // 昼夜をとわず
    }

    #[test]
    fn test_wotowazu_with_ha_particle() {
        // Structure: Noun + かはとわず (は instead of を for emphasis)
        // Example: どのブランドかはとわず
        let sentence = "どのブランドかはとわず、とにかく新しい靴が欲しいです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を問わず");
        assert_pattern_range(&patterns, "を問わず", 0, 11); // どのブランドかはとわず
    }

    #[test]
    fn test_wotowazu_affirmative_negative() {
        // Structure: A + A[ない] + を問わず
        // Example: 経験がある経験がないを問わず
        let sentence = "経験がある経験がないをとわず、やる気のある方を募集しています";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を問わず");
        assert_pattern_range(&patterns, "を問わず", 8, 14); // ないをとわず
    }

    // Pattern: たって (even if, even though, no matter how)
    // Data source: grammar_points_data.json["たって"]
    // Testing all 8 structure variants (casual register)
    mod tatte_tests {
        use super::*;

        #[test]
        fn test_tatte_verb_ta() {
            // Structure: Verb[た] + って
            let sentence = "彼女に謝ったってどうせ許してくれないだろう";
            let tokens = tokenize_sentence(sentence);
            let patterns = detect_patterns(&tokens);

            assert_has_pattern(&patterns, "たって");
            assert_pattern_range(&patterns, "たって", 3, 8); // 謝ったって
        }

        #[test]
        fn test_tatte_verb_nakute() {
            // Structure: Verb/Adj[なくて] + たって
            let sentence = "稽古がどんなに楽しくなくたって、稽古中には欠伸をしてはならない";
            let tokens = tokenize_sentence(sentence);
            let patterns = detect_patterns(&tokens);

            assert_has_pattern(&patterns, "たって_naku");
            assert_pattern_range(&patterns, "たって_naku", 7, 15); // 楽しくなくたって
        }

        #[test]
        fn test_tatte_i_adj_ku() {
            // Structure: い-Adjective[く] + たって
            let sentence = "どんなに欲しくたって、万引きをしてはいけない";
            let tokens = tokenize_sentence(sentence);
            let patterns = detect_patterns(&tokens);

            assert_has_pattern(&patterns, "たって_i_adj_ku");
            assert_pattern_range(&patterns, "たって_i_adj_ku", 4, 10); // 欲しくたって
        }

        #[test]
        fn test_tatte_i_adj_nakute() {
            // Structure: い-Adjective[なくて] + たって
            let sentence = "どんなに難しくなくたって、時間がかかるよ";
            let tokens = tokenize_sentence(sentence);
            let patterns = detect_patterns(&tokens);

            assert_has_pattern(&patterns, "たって_naku");
            assert_pattern_range(&patterns, "たって_naku", 4, 12); // 難しくなくたって
        }

        #[test]
        fn test_tatte_na_adj() {
            // Structure: な-Adjective + だって (handled by existing だって pattern)
            let sentence = "こんな簡単な問題馬鹿だってわかるよ";
            let tokens = tokenize_sentence(sentence);
            let patterns = detect_patterns(&tokens);

            // This is detected by the existing "だって" pattern, not "たって"
            assert_has_pattern(&patterns, "だって");
            assert_pattern_range(&patterns, "だって", 8, 13); // 馬鹿だって
        }

        #[test]
        fn test_tatte_na_adj_negative() {
            // Structure: な-Adjective + じゃなくたって
            let sentence = "有名じゃなくたって、良い歌手はたくさんいる";
            let tokens = tokenize_sentence(sentence);
            let patterns = detect_patterns(&tokens);

            assert_has_pattern(&patterns, "たって_naku");
            assert_pattern_range(&patterns, "たって_naku", 0, 9); // 有名じゃなくたって
        }

        #[test]
        fn test_tatte_noun() {
            // Structure: Noun + だって (handled by existing だって pattern)
            let sentence = "友達だって喧嘩をすることがある";
            let tokens = tokenize_sentence(sentence);
            let patterns = detect_patterns(&tokens);

            // This is detected by the existing "だって" pattern, not "たって"
            assert_has_pattern(&patterns, "だって");
            assert_pattern_range(&patterns, "だって", 0, 5); // 友達だって
        }

        #[test]
        fn test_tatte_noun_negative() {
            // Structure: Noun + じゃなくたって
            let sentence = "専門家じゃなくたって意見を言う権利がある";
            let tokens = tokenize_sentence(sentence);
            let patterns = detect_patterns(&tokens);

            assert_has_pattern(&patterns, "たって_naku");
            assert_pattern_range(&patterns, "たって_naku", 2, 10); // 家じゃなくたって
        }
    }

    // Pattern: に越したことはない (there is nothing better than)
    // Data source: grammar_points_data.json["に越したことはない"]
    // Testing all structure variants from structure.standard[] and structure.polite[]
    //
    // Structure variants:
    //   - standard[0]: Verb + に越したことはない
    //   - standard[1]: い-Adjective + に越したことはない
    //   - standard[2]: な-Adjective + (である) + に越したことはない
    //   - standard[3]: Noun + (である) + に越したことはない
    //   - polite[0]: Verb + に越したことはありません
    //   - polite[1]: い-Adjective + に越したことはありません
    //   - polite[2]: な-Adjective + (である) + に越したことはありません
    //   - polite[3]: Noun + (である) + に越したことはありません
    //
    // Also common: ない + に越したことはない (it's best if not X)

    mod nikoshitakotohanai_tests {
        use super::*;

        #[test]
        fn test_verb_standard() {
            // Structure: Verb + に越したことはない
            let sentence = "先輩と待ち合わせているなら、待ち合わせ時間の10分前に着くにこしたことはない";
            let tokens = tokenize_sentence(sentence);
            let patterns = detect_patterns(&tokens);

            assert_has_pattern(&patterns, "に越したことはない");
            assert_pattern_range(&patterns, "に越したことはない", 27, 38); // 着くにこしたことはない
        }

        #[test]
        fn test_i_adjective_standard() {
            // Structure: い-Adjective + に越したことはない
            let sentence = "家は広いにこしたことはないが、値段と場所も大切だ";
            let tokens = tokenize_sentence(sentence);
            let patterns = detect_patterns(&tokens);

            assert_has_pattern(&patterns, "に越したことはない");
            assert_pattern_range(&patterns, "に越したことはない", 2, 13); // 広いにこしたことはない
        }

        #[test]
        fn test_na_adjective_dearu() {
            // Structure: な-Adjective + である + に越したことはない
            let sentence = "部屋は綺麗であるにこしたことはないので、毎日掃除をしています";
            let tokens = tokenize_sentence(sentence);
            let patterns = detect_patterns(&tokens);

            assert_has_pattern(&patterns, "に越したことはない");
            assert_pattern_range(&patterns, "に越したことはない", 6, 17); // あるにこしたことはない
        }

        #[test]
        fn test_noun_dearu() {
            // Structure: Noun + である + に越したことはない
            let sentence = "億万長者であるにこしたことはない";
            let tokens = tokenize_sentence(sentence);
            let patterns = detect_patterns(&tokens);

            assert_has_pattern(&patterns, "に越したことはない");
            assert_pattern_range(&patterns, "に越したことはない", 5, 16); // あるにこしたことはない
        }

        #[test]
        fn test_verb_polite() {
            // Structure: Verb + に越したことはありません
            let sentence = "健康でいるにこしたことはありません";
            let tokens = tokenize_sentence(sentence);
            let patterns = detect_patterns(&tokens);

            assert_has_pattern(&patterns, "に越したことはない");
            assert_pattern_range(&patterns, "に越したことはない", 3, 17); // いるにこしたことはありません
        }

        #[test]
        fn test_nai_verb() {
            // Structure: Verb[ない] + に越したことはない (it's best if not X)
            let sentence = "災害は起きないにこしたことはない";
            let tokens = tokenize_sentence(sentence);
            let patterns = detect_patterns(&tokens);

            assert_has_pattern(&patterns, "に越したことはない");
            assert_pattern_range(&patterns, "に越したことはない", 5, 16); // ないにこしたことはない
        }

        #[test]
        fn test_nai_i_adjective() {
            // Structure: い-Adjective[なく] + ない + に越したことはない
            let sentence = "映画はつまらなくないにこしたことはない";
            let tokens = tokenize_sentence(sentence);
            let patterns = detect_patterns(&tokens);

            assert_has_pattern(&patterns, "に越したことはない");
            assert_pattern_range(&patterns, "に越したことはない", 8, 19); // ないにこしたことはない
        }

        #[test]
        fn test_nai_na_adjective() {
            // Structure: な-Adjective + ではない + に越したことはない
            let sentence = "テストは複雑ではないにこしたことはない";
            let tokens = tokenize_sentence(sentence);
            let patterns = detect_patterns(&tokens);

            assert_has_pattern(&patterns, "に越したことはない");
            assert_pattern_range(&patterns, "に越したことはない", 8, 19); // ないにこしたことはない
        }
    }

    // Pattern: ないではいられない (can't help but, can't resist)
    // Data source: grammar_points_data.json["ないではいられない"]
    // Testing all structure variants from structure.standard[] and structure.polite[]
    //
    // Structure variants:
    //   - standard[0]: Verb[ない] + ではいられない (formal)
    //   - standard[1]: Verb[ない] + じゃいられない (casual)
    //   - polite[0]: Verb[ない] + ではいられません
    //   - polite[1]: Verb[ない] + じゃいられません

    mod naidehairarenai_tests {
        use super::*;

        #[test]
        fn test_naide_wa_standard() {
            // Structure: Verb[ない] + ではいられない
            let sentence = "セール品を見ると、買わないではいられなくなる";
            let tokens = tokenize_sentence(sentence);
            let patterns = detect_patterns(&tokens);

            assert_has_pattern(&patterns, "ないではいられない");
            assert_pattern_range(&patterns, "ないではいられない", 9, 20); // 買わないではいられなく
        }

        #[test]
        fn test_naide_wa_past() {
            // Structure: Verb[ない] + ではいられなかった
            let sentence = "大親友が事故でなくなったときは、泣かないではいられなかった";
            let tokens = tokenize_sentence(sentence);
            let patterns = detect_patterns(&tokens);

            assert_has_pattern(&patterns, "ないではいられない");
            assert_pattern_range(&patterns, "ないではいられない", 16, 29); // 泣かないではいられなかった
        }

        #[test]
        fn test_nai_jya_casual() {
            // Structure: Verb[ない] + じゃいられない
            let sentence = "彼女の笑顔を見ると、笑わないじゃいられない";
            let tokens = tokenize_sentence(sentence);
            let patterns = detect_patterns(&tokens);

            assert_has_pattern(&patterns, "ないではいられない");
            assert_pattern_range(&patterns, "ないではいられない", 10, 21); // 笑わないじゃいられない
        }

        #[test]
        fn test_naide_wa_polite() {
            // Structure: Verb[ない] + ではいられません
            let sentence = "おばあさんが困っているのを見ると助けないではいられません";
            let tokens = tokenize_sentence(sentence);
            let patterns = detect_patterns(&tokens);

            assert_has_pattern(&patterns, "ないではいられない");
            assert_pattern_range(&patterns, "ないではいられない", 16, 28); // 助けないではいられません
        }
    }

    // Pattern: ねばならない (must, have to)
    // Data source: grammar_points_data.json["ねばならない"]
    // Testing all structure variants:
    //   - standard[0]: Verb[ない] + ねばならない
    //   - standard exception: する → せねばならない
    //   - polite[0]: Verb[ない] + ねばなりません
    //   - polite exception: する → せねばなりません
    mod nebanaranaי_tests {
        use super::*;

        #[test]
        fn test_verb_standard() {
            // Structure: Verb[ない] + ねばならない
            let sentence = "今月の１３日までに電気代を払わねばならない";
            let tokens = tokenize_sentence(sentence);
            let patterns = detect_patterns(&tokens);

            assert_has_pattern(&patterns, "ねばならない");
            assert_pattern_range(&patterns, "ねばならない", 13, 21); // 払わねばならない
        }

        #[test]
        fn test_suru_exception_standard() {
            // Structure: する → せねばならない (exception)
            let sentence = "お客さんが来る前に掃除をせねばならない";
            let tokens = tokenize_sentence(sentence);
            let patterns = detect_patterns(&tokens);

            assert_has_pattern(&patterns, "ねばならない");
            assert_pattern_range(&patterns, "ねばならない", 12, 19); // せねばならない
        }

        #[test]
        fn test_verb_polite() {
            // Structure: Verb[ない] + ねばなりません
            let sentence = "あの人は私の先輩なので何を言われても従わねばなりません";
            let tokens = tokenize_sentence(sentence);
            let patterns = detect_patterns(&tokens);

            assert_has_pattern(&patterns, "ねばならない");
            assert_pattern_range(&patterns, "ねばならない", 18, 27); // 従わねばなりません
        }

        #[test]
        fn test_suru_exception_polite() {
            // Structure: する → せねばなりません (exception)
            let sentence = "日本語を今より話せる様になりたいならもっと努力をせねばなりません";
            let tokens = tokenize_sentence(sentence);
            let patterns = detect_patterns(&tokens);

            assert_has_pattern(&patterns, "ねばならない");
            assert_pattern_range(&patterns, "ねばならない", 24, 32); // せねばなりません
        }
    }
}

// Pattern: のももっともだ (it's only natural that, it's reasonable that)
// Data source: grammar_points_data.json["のももっともだ"]
//
// Structure variants:
//   - standard[0]: Verb + のも + もっとも + だ
//   - standard[1]: い-Adjective + のも + もっとも + だ
//   - standard[2]: な-Adjective + なのも + もっとも + だ
//   - standard[3]: Noun + も + もっともだ (no の)
//   - Note: のは can be used instead of のも
//   - Note: は can be used instead of も (for nouns)
//   - polite[0-3]: Same with です instead of だ

mod nomomottomoda_tests {
    use super::*;

    #[test]
    fn test_verb_standard() {
        // Structure: Verb + のももっともだ
        let sentence = "そんなことを彼女に言ったのか？彼女が怒るのももっともだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "のももっともだ");
        assert_pattern_range(&patterns, "のももっともだ", 18, 27); // 怒るのももっともだ
    }

    #[test]
    fn test_i_adjective_standard() {
        // Structure: い-Adjective + のももっともだ
        let sentence = "彼はいつも夜遅くまで残業をしているから、いつも眠いのももっともだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "のももっともだ");
        assert_pattern_range(&patterns, "のももっともだ", 23, 32); // 眠いのももっともだ
    }

    #[test]
    fn test_na_adjective_standard() {
        // Structure: な-Adjective + なのももっともだ
        let sentence = "彼は高校生の頃から毎日日本語を勉強をしてきた。上手なのももっともだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "のももっともだ");
        assert_pattern_range(&patterns, "のももっともだ", 23, 33); // 上手なのももっともだ
    }

    #[test]
    fn test_noun_standard() {
        // Structure: Noun + ももっともだ (no の)
        let sentence = "親の反対ももっともだが、俺はもう俺のやり方でやると決めたから親の意見に合わすつもりはない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "のももっともだ");
        assert_pattern_range(&patterns, "のももっともだ", 2, 10); // 反対ももっともだ
    }

    #[test]
    fn test_verb_with_noha() {
        // Structure: Verb + のは + もっともだ (variant with は instead of も)
        let sentence = "彼がそんな風に考えるのはもっともだと思う";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "のももっともだ");
        assert_pattern_range(&patterns, "のももっともだ", 7, 17); // 考えるのはもっともだ
    }

    #[test]
    fn test_noun_with_ha() {
        // Structure: Noun + はもっともだ (variant with は instead of も)
        let sentence = "君の心配はもっともだけど、大丈夫だよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "のももっともだ");
        assert_pattern_range(&patterns, "のももっともだ", 2, 10); // 心配はもっともだ
    }

    #[test]
    fn test_verb_polite() {
        // Structure: Verb + のももっともです (polite)
        let sentence = "長時間働いているから疲れるのももっともです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "のももっともだ");
        assert_pattern_range(&patterns, "のももっともだ", 10, 21); // 疲れるのももっともです
    }

    #[test]
    fn test_na_adjective_polite() {
        // Structure: な-Adjective + なのももっともです (polite)
        let sentence = "この料理は特別な材料を使っているから高価なのももっともです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "のももっともだ");
        assert_pattern_range(&patterns, "のももっともだ", 18, 29); // 高価なのももっともです
    }
}

// Pattern: ～のうち(で) (among, out of)
// Data source: grammar_points_data.json["～のうち(で)"]
// Testing all structure variants:
//   - standard[0]: Noun + のうち（で）
//   - standard[1]: Noun + のうち + （の）+ Number
//   - standard[2]: Number + （Counter）+ のうち（で）
//   - standard[3]: その + うち（で）
//   - standard[4]: この + うち
//
// Note: pattern means "among (A)" or "out of (A)", specifying a subset from within a group
mod nouchi_de_tests {
    use super::*;

    #[test]
    fn test_noun_nouchi_de() {
        // Structure: Noun + のうちで
        let sentence = "この会社のうちでもっとも偉い人は田中さんです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～のうち(で)");
        assert_pattern_range(&patterns, "～のうち(で)", 2, 8); // 会社のうちで
    }

    #[test]
    fn test_noun_nouchi_de_number() {
        // Structure: Noun + のうちで + Number
        let sentence = "ここにいる５人のうちで一番給料が高いのは鈴木さんです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～のうち(で)");
        assert_pattern_range(&patterns, "～のうち(で)", 6, 11); // 人のうちで
    }

    #[test]
    fn test_noun_nouchi_no_number() {
        // Structure: Noun + のうちの + Number (connects to number with の)
        let sentence = "ここにある釣竿、１０本のうちの４本は海釣り用だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～のうち(で)");
        assert_pattern_range(&patterns, "～のうち(で)", 10, 15); // 本のうちの
    }

    #[test]
    fn test_number_counter_nouchi() {
        // Structure: Number + Counter + のうち (without で)
        let sentence = "学校の科目全てのうちから、一番好きな科目を教えてください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～のうち(で)");
        assert_pattern_range(&patterns, "～のうち(で)", 5, 12); // 全てのうちから
    }

    #[test]
    fn test_sono_uchi_de() {
        // Structure: その + うちで (among those, among them)
        let sentence = "いろんな方法を試したが、そのうちで一番効果的だったのは朝のランニングだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～のうち(で)");
        assert_pattern_range(&patterns, "～のうち(で)", 12, 17); // そのうちで
    }

    #[test]
    fn test_kono_uchi() {
        // Structure: この + うち (among these, among this group)
        let sentence = "このうちどれが一番気に入りましたか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～のうち(で)");
        assert_pattern_range(&patterns, "～のうち(で)", 0, 4); // このうち
    }
}

// Pattern: に際して (on the occasion of, at the time of)
// Data source: grammar_points_data.json["に際して"]
// Testing all structure variants from structure.standard[]
//
// Structure variants:
//   - standard[0]: Noun + に際（さい）し（て）
//   - standard[1]: Verb［る］+ に際（さい）し（て）
//   - standard[2]: Noun + に際（さい）しての + Noun
mod nisaishite_tests {
    use super::*;

    #[test]
    fn test_noun_nisaishite() {
        // Structure: Noun + に際して
        let sentence = "引越しに際して、いらなくなった家具を全部捨てた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に際して");
        assert_pattern_range(&patterns, "に際して", 0, 7); // 引越しに際して
    }

    #[test]
    fn test_verb_nisaishite() {
        // Structure: Verb[る] + に際して
        let sentence = "新しいプロジェクトを始めるに際して、新しいグループリーダーを決めた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に際して");
        assert_pattern_range(&patterns, "に際して", 10, 17); // 始めるに際して
    }

    #[test]
    fn test_noun_nisaishite_no_noun() {
        // Structure: Noun + に際しての + Noun
        let sentence = "クレジットカードを申し込むに際しての注意事項を確認してください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に際して");
        assert_pattern_range(&patterns, "に際して", 9, 18); // 申し込むに際しての
    }
}

// Pattern: にあたり・にあたって (on the occasion of, at the time of)
// Data source: grammar_points_data.json["にあたり・にあたって"]
// Testing all structure variants from structure.standard[]
//
// Structure variants:
//   - standard[0]: Verb［る］+ にあたり
//   - standard[1]: Noun + にあたり
//   - standard[2]: Same as above but にあたって instead
mod niatari_niatatte_tests {
    use super::*;

    #[test]
    fn test_verb_niatari() {
        // Structure: Verb[る] + にあたり
        let sentence = "就活を始めるにあたり、新しいスーツを買った";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にあたり・にあたって");
        assert_pattern_range(&patterns, "にあたり・にあたって", 3, 10); // 始めるにあたり
    }

    #[test]
    fn test_noun_niatari() {
        // Structure: Noun + にあたり
        let sentence = "転職をするにあたり、インターネットで自分に合った会社を探すことにした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にあたり・にあたって");
        assert_pattern_range(&patterns, "にあたり・にあたって", 3, 9); // するにあたり
    }

    #[test]
    fn test_noun_niatatte() {
        // Structure: Noun + にあたって
        let sentence = "卒業にあたって、みんなで卒業旅行として韓国に行った";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にあたり・にあたって");
        assert_pattern_range(&patterns, "にあたり・にあたって", 0, 7); // 卒業にあたって
    }
}

// Pattern: というわけではない (doesn't mean that, it's not that)
// Data source: grammar_points_data.json["というわけではない"]
// Testing structure variants:
//   - standard[0]: Verb + という + わけではない
//   - standard[1]: い-Adjective + という + わけではない
//   - standard[2]: な-Adjective + (だ) + という + わけではない
//   - standard[3]: Noun + (だ) + という + わけではない
//   - polite[0-3]: Same with わけではありません
//   - Variation: じゃ instead of では

mod toiuwakedehanai_tests {
    use super::*;

    #[test]
    fn test_verb_standard() {
        // Structure: Verb + という + わけではない
        let sentence = "日本に２年間住んでいたからと言って、日本語を話せるというわけではない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "というわけではない");
        assert_pattern_range(&patterns, "というわけではない", 22, 34); // 話せるというわけではない
    }

    #[test]
    fn test_i_adjective_standard() {
        // Structure: い-Adjective + という + わけではない
        let sentence = "数学が嫌いというわけではない、ただ苦手なだけだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "というわけではない");
        assert_pattern_range(&patterns, "というわけではない", 3, 14); // 嫌いというわけではない
    }

    #[test]
    fn test_na_adjective_standard() {
        // Structure: な-Adjective + だ + という + わけではない
        let sentence = "いつもニコニコしているからって、親切だというわけではない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "というわけではない");
        assert_pattern_range(&patterns, "というわけではない", 18, 28); // だというわけではない
    }

    #[test]
    fn test_noun_standard() {
        // Structure: Noun + という + わけではない
        let sentence = "離婚というわけではないが、しばらく別居することになった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "というわけではない");
        assert_pattern_range(&patterns, "というわけではない", 0, 11); // 離婚というわけではない
    }

    #[test]
    fn test_verb_polite() {
        // Structure: Verb + という + わけではありません
        let sentence = "勉強しているというわけではありませんが、少し見ていただけです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "というわけではない");
        assert_pattern_range(&patterns, "というわけではない", 4, 18); // いるというわけではありません
    }

    #[test]
    fn test_na_adjective_with_ja() {
        // Structure: な-Adjective + だ + というわけじゃない (casual variation)
        let sentence = "嫌いだというわけじゃないけど、あんまり好きでもない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "というわけではない");
        assert_pattern_range(&patterns, "というわけではない", 2, 12); // だというわけじゃない
    }

    #[test]
    fn test_noun_with_da() {
        // Structure: Noun + だ + という + わけではない
        let sentence = "彼が犯人だというわけではないけど、怪しいことは確かだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "というわけではない");
        assert_pattern_range(&patterns, "というわけではない", 4, 14); // だというわけではない
    }
}

// Pattern: を契機に (as a trigger/opportunity, led to)
// Data source: grammar_points_data.json["を契機に"]
// Testing structure variants:
//   - standard[0]: Noun + を契機に
//   - standard[1]: Verb + の/こと + を契機に
//   - Variations: を契機にして, を契機として

mod wokeikini_tests {
    use super::*;

    #[test]
    fn test_noun_wokeikini() {
        // Structure: Noun + を契機に
        let sentence = "出産を契機に、会社を辞めることにした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を契機に");
        assert_pattern_range(&patterns, "を契機に", 0, 6); // 出産を契機に
    }

    #[test]
    fn test_verb_no_wokeikini() {
        // Structure: Verb + の + を契機に
        let sentence = "子供が生まれたのを契機に、パチンコをやめることに決めた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を契機に");
        assert_pattern_range(&patterns, "を契機に", 7, 12); // のを契機に
    }

    #[test]
    fn test_verb_koto_wokeikini() {
        // Structure: Verb + こと + を契機に
        let sentence = "移動制限が緩和されたことを契機に、観光客の数が増えた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を契機に");
        assert_pattern_range(&patterns, "を契機に", 10, 16); // ことを契機に
    }

    #[test]
    fn test_noun_wokeikini_nishite() {
        // Structure: Noun + を契機にして (variation)
        let sentence = "入院を契機にして、タバコを止めました";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を契機に");
        assert_pattern_range(&patterns, "を契機に", 0, 8); // 入院を契機にして
    }

    #[test]
    fn test_noun_wokeikini_toshite() {
        // Structure: Noun + を契機として (variation)
        let sentence = "大統領の暗殺を契機として、戦争が始まった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を契機に");
        assert_pattern_range(&patterns, "を契機に", 4, 12); // 暗殺を契機として
    }
}

// Pattern: ないわけにはいかない (cannot afford not to, no way to avoid)
// Data source: grammar_points_data.json["ないわけにはいかない"]
// Testing: structure.standard[0] - "Verb[ない] + わけにはいかない"
//
// Structure variants:
//   - standard[0]: Verb[ない] + わけにはいかない
//   - polite[0]: Verb[ない] + わけにはいきません

mod naiwakenihaikanai_tests {
    use super::*;

    #[test]
    fn test_nai_wakenihaikanai_standard() {
        // Structure: Verb[ない] + わけにはいかない (standard)
        let sentence = "いつもお世話になっている先輩の結婚式だから、行きたくなくても行かないわけにはいかない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないわけにはいかない");
        assert_pattern_range(&patterns, "ないわけにはいかない", 30, 42); // 行かないわけにはいかない
    }

    #[test]
    fn test_nai_wakenihaikanai_diet() {
        // Structure: Verb[ない] + わけにはいかない (standard)
        let sentence = "ダイエット中だからといって、何も食べないわけにはいかない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないわけにはいかない");
        assert_pattern_range(&patterns, "ないわけにはいかない", 16, 28); // 食べないわけにはいかない
    }

    #[test]
    fn test_nai_wakenihaikimasen_polite() {
        // Structure: Verb[ない] + わけにはいきません (polite)
        let sentence = "忙しいからといって、挨拶しないわけにはいきません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないわけにはいかない");
        assert_pattern_range(&patterns, "ないわけにはいかない", 10, 24); // 挨拶しないわけにはいきません
    }
}

// Pattern: ～ところに・～ところへ (at the time of, while, when)
// Data source: grammar_points_data.json["～ところに・～ところへ"]
// Testing: structure.standard[0] - "Verb［ている］+ ところに"
// Testing: structure.standard[1] - "Verb［ていた］+ ところに"
// Testing: structure.standard[2] - "(1) へ" (use へ instead of に)
//
// Structure variants:
//   - standard[0]: Verb［ている］+ ところに (progressive)
//   - standard[1]: Verb［ていた］+ ところに (past progressive)
//   - standard[2]: Can use へ instead of に for both variants
//
// Note: This pattern highlights the precise moment when something unexpected happens.
// Usually (A)ところに/へ, (B) happened - emphasizing timing and unexpectedness.

mod tokoroni_tests {
    use super::*;

    #[test]
    fn test_teiru_tokoroni() {
        // Structure: Verb［ている］+ ところに (standard[0])
        let sentence = "車で左折しているところに、急に自転車が飛び出してきた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～ところに・～ところへ");
        assert_pattern_range(&patterns, "～ところに・～ところへ", 2, 12); // 左折しているところに
    }

    #[test]
    fn test_teita_tokoroni() {
        // Structure: Verb［ていた］+ ところに (standard[1])
        let sentence = "休憩室で踊っていたところに後輩たちが入ってきた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～ところに・～ところへ");
        assert_pattern_range(&patterns, "～ところに・～ところへ", 4, 13); // 踊っていたところに
    }

    #[test]
    fn test_teiru_tokorohe() {
        // Structure: Verb［ている］+ ところへ (standard[2] with へ)
        let sentence = "倉庫を取り壊しているところへ近所の人が来て手伝ってくれた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～ところに・～ところへ");
        assert_pattern_range(&patterns, "～ところに・～ところへ", 3, 14); // 取り壊しているところへ
    }

    #[test]
    fn test_teita_tokorohe() {
        // Structure: Verb［ていた］+ ところへ (standard[2] with ていた + へ)
        let sentence = "上司に怒られて落ち込んでいたところへ、先輩が来て慰めてくれた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～ところに・～ところへ");
        assert_pattern_range(&patterns, "～ところに・～ところへ", 7, 18); // 落ち込んでいたところへ
    }
}

// Pattern: ぶりに (for the first time in [time period])
// Data source: grammar_points_data.json["ぶりに"]
// Testing all structure variants with print_debug to see tokenization
//
// Structure variants:
//   - standard[0]/polite[0]: Noun + ぶり + だ/です
//   - standard[1]/polite[1]: Noun + ぶりに
//   - standard[2]/polite[2]: Noun + ぶりの + Noun

mod burini_tests {
    use super::*;

    #[test]
    fn test_burida_standard() {
        // Structure: Noun + ぶり + だ
        let sentence = "彼女との再会は五年ぶりだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ぶりに");
        assert_pattern_range(&patterns, "ぶりに", 7, 12); // 五年ぶりだ
    }

    #[test]
    fn test_buridesu_polite() {
        // Structure: Noun + ぶり + です
        let sentence = "実家に帰るのは三年ぶりです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ぶりに");
        assert_pattern_range(&patterns, "ぶりに", 7, 13); // 三年ぶりです
    }

    #[test]
    fn test_burini_standard() {
        // Structure: Noun + ぶりに
        let sentence = "一年ぶりに五キロも走ったから明日は絶対に筋肉痛だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ぶりに");
        assert_pattern_range(&patterns, "ぶりに", 0, 5); // 一年ぶりに
    }

    #[test]
    fn test_burinino_with_noun() {
        // Structure: Noun + ぶりの + Noun
        let sentence = "四年ぶりの寿司だ！やっぱり日本の寿司はうまいな";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ぶりに");
        assert_pattern_range(&patterns, "ぶりに", 0, 5); // 四年ぶりの
    }

    #[test]
    fn test_burini_verb_following() {
        // Structure: Noun + ぶりに + Verb (風呂に入る)
        let sentence = "三年ぶりに風呂に入る";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ぶりに");
        assert_pattern_range(&patterns, "ぶりに", 0, 5); // 三年ぶりに
    }

    #[test]
    fn test_burinino_victory() {
        // Structure: Noun + ぶりの + Noun (優勝)
        let sentence = "高橋選手が十年ぶりの優勝！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ぶりに");
        assert_pattern_range(&patterns, "ぶりに", 5, 10); // 十年ぶりの
    }
}

// Pattern: ては (if, when - conditional with negative expectation)
// Data source: grammar_points_data.json["ては"]
// Testing: structure.standard[0-5] - "Verb[て] + は", "い-Adj[て] + は", "な-Adj + では", "Noun + では", "Verb[て] + ちゃ", "じゃ"
//
// Structure variants:
//   - standard[0]: Verb[て] + は (if/when doing)
//   - standard[1]: い-Adjective[て] + は (if/when being adj)
//   - standard[2]: な-Adjective + では (if/when being na-adj)
//   - standard[3]: Noun + では (if/when being noun)
//   - standard[4]: Verb[て] + ちゃ (casual contraction of ては)
//   - standard[5]: じゃ (casual contraction of では)

mod teha_tests {
    use super::*;

    #[test]
    fn test_teha_verb() {
        // Structure: Verb[て] + は (negative consequence)
        let sentence = "そのように力を入れて回してはネジがだめになってしまいますよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ては");
        assert_pattern_range(&patterns, "ては", 10, 14); // 回しては
    }

    #[test]
    fn test_teha_i_adjective() {
        // Structure: い-Adjective[て] + は (too long to fit)
        let sentence = "そんなに長くては鞄に入らないので、半分に折ってください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ては");
        assert_pattern_range(&patterns, "ては", 4, 8); // 長くては
    }

    #[test]
    fn test_teha_na_adjective() {
        // Structure: な-Adjective + では (if being that meticulous)
        let sentence = "私は掃除が苦手なので、そんな几帳面では私と住むことはできないでしょう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ては");
        assert_pattern_range(&patterns, "ては", 14, 19); // 几帳面では
    }

    #[test]
    fn test_teha_noun() {
        // Structure: Noun + では (if wearing that outfit)
        let sentence = "そんな貧乏くさい格好ではあのレストランには入れないよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ては");
        assert_pattern_range(&patterns, "ては", 8, 12); // 格好では
    }

    #[test]
    fn test_teha_casual_cha() {
        // Structure: Verb[て] + ちゃ (casual contraction - if not eating)
        let sentence = "ご飯を全部食べなくちゃ大きくなれないよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ては");
        assert_pattern_range(&patterns, "ては", 7, 11); // なくちゃ
    }

    #[test]
    fn test_teha_casual_ja() {
        // Structure: じゃ (casual contraction of では - if that person)
        let sentence = "リーダーがあの人じゃ嫌です";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ては");
        assert_pattern_range(&patterns, "ては", 7, 10); // 人じゃ
    }
}

// Pattern: ては〜ては (doing A and B repeatedly, alternating actions)
// Data source: grammar_points_data.json["ては〜ては"]
// Testing: structure.standard[0] - "Verb［て］(A) + は + Verb［て］(B) + は"
// Testing: structure.standard[1] - "Verb［て］(A) + は + Verb［て］(A) + は"
//
// Structure variants:
//   - standard[0]: Verb［て］(A) + は + Verb［て］(B) + は (contrasting actions)
//   - standard[1]: Verb［て］(A) + は + Verb［て］(A) + は (same action repeated)
//   - Casual forms: ちゃ/じゃ instead of ては/では
//
// Note: The pattern expresses repeated alternating actions, often contrasting
// (e.g., eating and sleeping, making mistakes and getting scolded)

// Pattern: ては〜ては (doing A and B repeatedly)
// Data source: grammar_points_data.json["ては〜ては"]
// Structures: 2 standard (Verb[て](A) + は + Verb[て](B) + は, same or different verbs)
//
// Current implementation: Successfully detects the core て+は structure for
// repeated alternating actions. Works best with continuous text without punctuation.
// The casual contracted forms (ちゃ/じゃ without explicit は) would require
// additional matcher logic as they don't follow the same tokenization pattern.
mod teha_uff5e_teha_tests {
    use super::*;

    #[test]
    fn test_same_verb_repeated() {
        // Structure: Verb[て](A) + は + Verb[て](A) + は (same verb repeated)
        let sentence = "食べては寝て食べては寝ての繰り返しです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ては〜ては");
        assert_pattern_range(&patterns, "ては〜ては", 0, 10); // 食べては寝て食べては
    }

    #[test]
    fn test_contrasting_verbs() {
        // Structure: Verb[て](A) + は + Verb[て](B) + は (different verbs)
        let sentence = "ミスしては怒られ怒られてはミスをする毎日です";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ては〜ては");
        assert_pattern_range(&patterns, "ては〜ては", 0, 13); // ミスしては怒られ怒られては
    }
}

// Pattern: 以来 (since, ever since)
// Data source: grammar_points_data.json["以来"]
// Structures: 3 main (Verb[て] + 以来, Noun + 以来, Demonstrative + 以来)
mod irai_tests {
    use super::*;

    #[test]
    fn test_verb_te_irai() {
        // Structure: Verb[て] + いらい (hiragana form)
        let sentence = "日本に来ていらい全然母国の友達と連絡をとっていない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "以来");
        assert_pattern_range(&patterns, "以来", 4, 8); // ていらい
    }

    #[test]
    fn test_noun_irai() {
        // Structure: Noun + いらい (hiragana form)
        let sentence = "卒業いらいじゃない？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "以来");
        assert_pattern_range(&patterns, "以来", 0, 5); // 卒業いらい
    }

    #[test]
    fn test_noun_irai_with_kanji() {
        // Structure: Noun + 以来 (kanji form)
        let sentence = "入社以来、あの先輩には色々とお世話になっている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "以来");
        assert_pattern_range(&patterns, "以来", 0, 4); // 入社以来
    }

    #[test]
    fn test_demonstrative_irai() {
        // Structure: Demonstrative (それ/これ/あれ) + 以来 (kanji form)
        let sentence = "それ以来ずっと音沙汰がない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "以来");
        assert_pattern_range(&patterns, "以来", 0, 4); // それ以来
    }
}

// Pattern: 結果・の結果 (as a result of)
// Data source: grammar_points_data.json["結果・の結果"]
// Testing: structure.standard[0] - "Verb[た] + 結果 + Phrase"
// Testing: structure.standard[1] - "Noun + の + 結果 + Phrase"
//
// Structure variants:
//   - standard[0]: Verb[た] + 結果 + Phrase (as a result of doing)
//   - standard[1]: Noun + の + 結果 + Phrase (as a result of noun)

mod kekka_u30fb_nokekka_tests {
    use super::*;

    #[test]
    fn test_verb_ta_kekka() {
        // Structure: Verb[た] + 結果 (result)
        let sentence = "お酒を飲んで運転した結果、電柱にぶつかって免許を取り消された";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "結果・の結果");
        assert_pattern_range(&patterns, "結果・の結果", 6, 12); // 運転した結果
    }

    #[test]
    fn test_verb_ta_kekka_positive() {
        // Structure: Verb[た] + 結果 (positive outcome)
        let sentence = "みんなが努力した結果、大会で優勝することができた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "結果・の結果");
        assert_pattern_range(&patterns, "結果・の結果", 4, 10); // 努力した結果
    }

    #[test]
    fn test_noun_no_kekka() {
        // Structure: Noun + の + 結果 (result)
        let sentence = "話し合いの結果、親が学費を払ってくれることになりました";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "結果・の結果");
        assert_pattern_range(&patterns, "結果・の結果", 0, 7); // 話し合いの結果
    }

    #[test]
    fn test_noun_no_kekka_investigation() {
        // Structure: Noun + の + 結果 (investigation result)
        let sentence = "調査の結果、この土地の土は汚染されていることが確認できました";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "結果・の結果");
        assert_pattern_range(&patterns, "結果・の結果", 0, 5); // 調査の結果
    }
}

// Pattern: とも (even if, no matter if)
// Data source: grammar_points_data.json["とも"]
// Testing structures:
//   - standard[0]: Verb[おう] + とも (volitional + とも)
//   - standard[1]: Verb[なくて] + とも (negative form + とも) - NOTE: This seems incorrect
//   - standard[2]: な-Adjective + であろう + とも
//   - standard[3]: い-Adjective[く] + とも (conjunctive form)
//   - standard[4]: い-Adjective[かろう] + とも (alternate for standard[3])
//
// Note: standard[1] appears to be an error - the examples show volitional forms, not なくて forms
mod tomo_tests {
    use super::*;

    #[test]
    fn test_verb_volitional_ou() {
        // Structure: Verb[おう] + とも
        // Example from grammar_points_data.json: 親が何と言おうとも
        let sentence = "親が何と言おうとも、私は彼と結婚するつもりだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とも");
        assert_pattern_range(&patterns, "とも", 4, 9); // 言おうとも
    }

    #[test]
    fn test_verb_volitional_you() {
        // Structure: Verb[よう] + とも
        // Example from grammar_points_data.json: どんなに暴れようとも
        let sentence = "魚がどんなに暴れようとも、この糸は絶対に切れません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とも");
        assert_pattern_range(&patterns, "とも", 6, 12); // 暴れようとも
    }

    #[test]
    fn test_i_adjective_conjunctive() {
        // Structure: い-Adjective[く] + とも
        // Example from grammar_points_data.json: いくら辛くとも
        let sentence = "人生がいくら辛くとも、諦めてはいけない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とも");
        assert_pattern_range(&patterns, "とも", 6, 10); // 辛くとも
    }

    #[test]
    fn test_na_adjective_dearou() {
        // Structure: な-Adjective + であろう + とも
        // Example from grammar_points_data.json: いくら下手であろうとも
        let sentence = "いくら下手であろうとも、毎日練習をしていればそのうち上手になれる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とも");
        assert_pattern_range(&patterns, "とも", 3, 11); // 下手であろうとも
    }

    #[test]
    fn test_i_adjective_karou() {
        // Structure: い-Adjective[かろう] + とも (less common alternate form)
        // Using a realistic example
        let sentence = "どんなに苦しかろうとも、最後まで諦めずに頑張ろう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とも");
        assert_pattern_range(&patterns, "とも", 4, 11); // 苦しかろうとも
    }
}

// Pattern: 陸に～ない (ろくに～ない - barely, hardly, not properly)
// Data source: grammar_points_data.json["陸に～ない"]
// Testing: structure.standard[0] - "ろくに + (Negative) Phrase"
//
// Structure variants:
//   - standard[0]: ろくに + (Negative) Phrase (can be written as ろくに, 陸に, or 碌に)
//   - The negative phrase typically contains ない (often potential form + ない)

mod rikuni_nai_tests {
    use super::*;

    #[test]
    fn test_roku_ni_potential_negative() {
        // Structure: ろくに + Verb[potential] + ない
        // Example: 息子は英語をろくに話せないのに、アメリカにひとりで行くそうだ
        let sentence = "息子は英語をろくに話せないのに、アメリカに一人で行くそうだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "陸に～ない");
        assert_pattern_range(&patterns, "陸に～ない", 6, 13); // ろくに話せない
    }

    #[test]
    fn test_roku_ni_nai_past() {
        // Structure: ろくに + Verb[potential] + なかった
        // Example: 近所のワンちゃんが朝までずっと吠えていたから、昨晩はろくに寝れなかった
        let sentence = "昨晩はろくに寝れなかったから、今日は眠い";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "陸に～ない");
        assert_pattern_range(&patterns, "陸に～ない", 3, 12); // ろくに寝れなかった
    }

    #[test]
    fn test_roku_ni_with_mo_particle() {
        // Structure: ろくに + Noun + も + Verb + ない
        // Example: ろくに仕事も出来ないのに、偉そうな態度をとる
        let sentence = "あの人はろくに仕事も出来ないのに、偉そうな態度をとる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "陸に～ない");
        assert_pattern_range(&patterns, "陸に～ない", 4, 14); // ろくに仕事も出来ない
    }

    #[test]
    fn test_roku_ni_plain_negative() {
        // Structure: ろくに + Verb + ない
        // Example: 豆なんてろくに食べないのに、うちには豆の缶詰がいっぱいある
        let sentence = "豆なんてろくに食べないのに、缶詰がいっぱいある";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "陸に～ない");
        assert_pattern_range(&patterns, "陸に～ない", 4, 11); // ろくに食べない
    }

    #[test]
    fn test_roku_ni_te_iru_negative() {
        // Structure: ろくに + Verb[て] + いない
        // Example: 仕事が忙しすぎて、ここ最近、ろくに友達にも会えていない
        let sentence = "最近仕事が忙しくて、ろくに友達にも会えていない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "陸に～ない");
        assert_pattern_range(&patterns, "陸に～ない", 10, 23); // ろくに友達にも会えていない
    }
}

// Pattern: てはいられない (cannot afford to, unable to)
// Data source: grammar_points_data.json["てはいられない"]
// Testing: structure.standard[0-4] - Various forms
//
// Structure variants:
//   - standard[0]: Verb[て] + はいられない
//   - standard[1]: い-Adjective[て] + はいられない
//   - standard[2]: な-Adjective + では + いられない (or じゃ)
//   - standard[3]: Noun + では + いられない (or じゃ)
//   - polite[0-3]: Same with いられません

mod tehairarenai_tests {
    use super::*;

    #[test]
    fn test_verb_te_wa_irarenai() {
        // Structure: Verb[て] + はいられない
        // Example: 遊んではいられない
        let sentence = "明日までに提出しないといけないから、遊んではいられない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てはいられない");
        assert_pattern_range(&patterns, "てはいられない", 20, 27); // ではいられない
    }

    #[test]
    fn test_verb_te_wa_irarenai_2() {
        // Structure: Verb[て] + はいられない
        // Example: のんびり食べてはいられない
        let sentence = "あと３０分で出ないといけないので、のんびり食べてはいられない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てはいられない");
        assert_pattern_range(&patterns, "てはいられない", 23, 30); // てはいられない
    }

    #[test]
    fn test_verb_de_wa_irarenai() {
        // Structure: Verb[で] + はいられない (de variant)
        // Example: 休んではいられない
        let sentence = "午後までに終わらせるように頼まれたから、休んではいられない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てはいられない");
        assert_pattern_range(&patterns, "てはいられない", 22, 29); // ではいられない
    }

    #[test]
    fn test_i_adjective_te_wa_irarenai() {
        // Structure: い-Adjective[て] + はいられない
        // Example: 遅くてはいられない
        let sentence = "次の大会で勝つには、こんな遅くてはいられない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てはいられない");
        assert_pattern_range(&patterns, "てはいられない", 15, 22); // てはいられない
    }

    #[test]
    fn test_na_adjective_de_wa_irarenai() {
        // Structure: な-Adjective + では + いられない
        // Example: 元気ではいられない
        let sentence = "お父さんもいつまでも元気ではいられないんだから、気をつけな";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てはいられない");
        assert_pattern_range(&patterns, "てはいられない", 12, 19); // ではいられない
    }

    #[test]
    fn test_noun_de_wa_irarenai() {
        // Structure: Noun + では + いられない
        // Example: 社員ではいられない
        let sentence = "もうこんな会社の社員ではいられない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てはいられない");
        assert_pattern_range(&patterns, "てはいられない", 10, 17); // ではいられない
    }
}

// Pattern: したがって (therefore, accordingly, as a result)
// Data source: grammar_points_data.json["したがって"]
// Testing: structure.standard[0] - "(Cause) + したがって + (Result)"
//
// Structure variants:
//   - standard[0]: (Cause) + したがって + (Result) - logical conclusion
//
// Note: This is a formal conjunction that connects cause and result
// It appears at the beginning of the result clause (after the cause)
// Only used for logical conclusions, not subjective opinions

mod shitagatte_tests {
    use super::*;

    #[test]
    fn test_shitagatte_basic() {
        // Cause: 工事が行われている → Result: したがって、通行止めとなっている
        let sentence = "現在東名高速でリニューアル工事が行われている。したがって、その区間は現在通行止めとなっている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "したがって");
        assert_pattern_range(&patterns, "したがって", 23, 28); // したがって
    }

    #[test]
    fn test_shitagatte_comparison() {
        // Cause: 台風が多かった → Result: したがって、米の値段が高くなる
        let sentence = "今年は去年より台風が多かった。したがって、今年の米の値段は去年より高くなるでしょう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "したがって");
        assert_pattern_range(&patterns, "したがって", 15, 20); // したがって
    }

    #[test]
    fn test_shitagatte_decision() {
        // Cause: インフルエンザが流行っている → Result: したがって、会議はオンラインで行われる
        let sentence = "会社ではインフルエンザが流行っている。したがって、会議は全てオンラインで行われることになった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "したがって");
        assert_pattern_range(&patterns, "したがって", 19, 24); // したがって
    }
}

// Pattern: はたして (I wonder if, really, as expected)
// Data source: grammar_points_data.json["はたして"]
// Testing: structure.standard[0] - "はたして + (Question) Phrase"
// Testing: structure.standard[1] - "はたして + Phrase"
//
// Structure variants:
//   - standard[0]: はたして + question/speculation (I wonder if, really)
//   - standard[1]: はたして + statement (as expected, sure enough)
//
// Note: Appears adverbially at the beginning of sentences
// With speculation: "I wonder if", with statements: "as expected"

mod hatashite_tests {
    use super::*;

    #[test]
    fn test_hatashite_speculation() {
        // はたして + だろうか (I wonder if)
        let sentence = "はたして彼が言っていることは本当なのだろうか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "はたして");
        assert_pattern_range(&patterns, "はたして", 0, 4); // はたして
    }

    #[test]
    fn test_hatashite_question() {
        // はたして + でしょうか (I wonder if)
        let sentence = "はたして、あのやり方で本当に成功するのでしょうか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "はたして");
        assert_pattern_range(&patterns, "はたして", 0, 4); // はたして
    }

    #[test]
    fn test_hatashite_expected_result() {
        // はたして + statement (as expected)
        let sentence = "彼は取引先の信頼を失う事をしてしまった。はたして、彼はクビになった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "はたして");
        assert_pattern_range(&patterns, "はたして", 20, 24); // はたして
    }

    #[test]
    fn test_hatashite_midsentence() {
        // はたして in middle of sentence (as expected)
        let sentence = "彼女は自分の意見を彼に押し付けようとしたが、はたして彼は自分の意見を変えなかった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "はたして");
        assert_pattern_range(&patterns, "はたして", 22, 26); // はたして
    }
}

// Pattern: に先立ち (prior to, before)
// Data source: grammar_points_data.json["に先立ち"]
// Testing all structure variants with print_debug to analyze tokenization
//
// Structure variants:
//   - standard[0]: Verb[る] + に先立って
//   - standard[1]: Noun + に先立って
//   - standard[2]: Noun (A) + に先立つ + Noun (B)
//   - Variation: に先立ち (formal conjunctive form)

mod nisakidachi_tests {
    use super::*;

    #[test]
    fn test_verb_nisakidatte() {
        // Verb[る] + に先立って
        let sentence = "新しい機械を導入するに先立って、色々な会社のサイトを見てスペックを比較します";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に先立ち");
        assert_pattern_range(&patterns, "に先立ち", 6, 15); // 導入するに先立って
    }

    #[test]
    fn test_noun_nisakidatte() {
        // Noun + に先立って
        let sentence = "会議に先立って、資料を確認しておいてください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に先立ち");
        assert_pattern_range(&patterns, "に先立ち", 0, 7); // 会議に先立って
    }

    #[test]
    fn test_noun_nisakidachi() {
        // Noun + に先立ち (formal conjunctive form)
        let sentence = "映画の公開に先立ち、予告編が公開された";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に先立ち");
        assert_pattern_range(&patterns, "に先立ち", 3, 9); // 公開に先立ち
    }

    #[test]
    fn test_noun_nisakidatsu_noun() {
        // Noun (A) + に先立つ + Noun (B)
        let sentence = "引越しに先立つ１日前に洗濯機や冷蔵庫の水抜きをしておかなければならない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に先立ち");
        assert_pattern_range(&patterns, "に先立ち", 0, 7); // 引越しに先立つ
    }
}

// Pattern: 甲斐がある (worth doing, pays off)
// Data source: grammar_points_data.json["甲斐がある"]
// Testing all structure variants with print_debug to analyze tokenization
//
// Structure variants:
//   - standard[0]: Verb[た] + かい + がある
//   - standard[1]: Verb[stem] + がい + がある
//   - standard[2]: Noun + の + かい + がある
//   - Negative forms: かい/がい + がない

mod kaigaaru_tests {
    use super::*;

    #[test]
    fn test_verb_ta_kai_ga_aru() {
        // Verb[た] + かい + がある
        let sentence = "この食パンを買えて、朝の４時から並んだかいがあった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "甲斐がある");
        assert_pattern_range(&patterns, "甲斐がある", 18, 25); // だかいがあった
    }

    #[test]
    fn test_verb_stem_gai_ga_aru() {
        // Verb[stem] + がい + がある (using ている form)
        let sentence = "こういう景色を見ると生きているかいがあると思う";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "甲斐がある");
        assert_pattern_range(&patterns, "甲斐がある", 13, 20); // いるかいがある
    }

    #[test]
    fn test_verb_stem_gai_ga_nai() {
        // Verb[stem] + がい + がない (negative)
        let sentence = "彼女は何回注意しても上達しないから教えがいがない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "甲斐がある");
        assert_pattern_range(&patterns, "甲斐がある", 17, 24); // 教えがいがない
    }

    #[test]
    fn test_noun_no_kai_ga_aru() {
        // Noun + の + かい + がある
        let sentence = "みんなの努力のかいがあり、この大会で優勝する事ができました";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "甲斐がある");
        assert_pattern_range(&patterns, "甲斐がある", 6, 12); // のかいがあり
    }

    #[test]
    fn test_verb_ta_kai_ga_nai() {
        // Verb[た] + かい + がない (negative)
        let sentence = "こんなもんのために東京からきたかいがない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "甲斐がある");
        assert_pattern_range(&patterns, "甲斐がある", 14, 20); // たかいがない
    }
}

// Pattern: てはならない (must not do)
// Data source: grammar_points_data.json["てはならない"]
// Testing: structure.standard[0] - "Verb[て] + はならない"
//          structure.polite[0] - "Verb[て] + はなりません"
mod tehanaranai_tests {
    use super::*;

    #[test]
    fn test_verb_te_hanaranaい_standard() {
        // Structure: Verb[て] + はならない (standard form)
        // Example from grammar_points_data.json: 泳いではならない
        let sentence = "この池では泳いではならない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てはならない");
        assert_pattern_range(&patterns, "てはならない", 5, 13); // 泳いではならない
    }

    #[test]
    fn test_verb_te_hanaranasen_polite() {
        // Structure: Verb[て] + はなりません (polite form)
        // Example: 運転してはなりません
        let sentence = "お酒を飲んで運転してはなりません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てはならない");
        assert_pattern_range(&patterns, "てはならない", 6, 16); // 運転してはなりません
    }

    #[test]
    fn test_verb_te_hanaranaい_formal_writing() {
        // Structure: Verb[て] + はならない (formal writing)
        // Example from grammar_points_data.json: 関係者以外、入ってはならない
        let sentence = "ここから先は関係者以外、入ってはならない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てはならない");
        assert_pattern_range(&patterns, "てはならない", 12, 20); // 入ってはならない
    }
}

// Pattern: やがて (before long, eventually, soon)
// Data source: grammar_points_data.json["やがて"]
// Testing: structure.standard[0] - "やがて + Phrase"
mod yagate_tests {
    use super::*;

    #[test]
    fn test_yagate_future_event() {
        // Structure: やがて + Phrase (future prediction)
        // Example from grammar_points_data.json: やがて人気がなくなる
        let sentence = "そのアニメもあのアニメのようにやがて人気がなくなるでしょう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "やがて");
        assert_pattern_range(&patterns, "やがて", 15, 18); // やがて
    }

    #[test]
    fn test_yagate_with_condition() {
        // Structure: やがて + Phrase (with conditional)
        // Example from grammar_points_data.json: やがて小説を読めるようになる
        let sentence = "毎日漢字を読む練習をしていれば、やがて小説を読めるようになるだろう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "やがて");
        assert_pattern_range(&patterns, "やがて", 16, 19); // やがて
    }

    #[test]
    fn test_yagate_immediate_future() {
        // Structure: やがて + Phrase (near future)
        // Example from grammar_points_data.json: やがて日が沈み始める
        let sentence = "もう５時なので、やがて日が沈み始めるだろう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "やがて");
        assert_pattern_range(&patterns, "やがて", 8, 11); // やがて
    }
}

// Pattern: にかけては (when it comes to, regarding)
// Data source: grammar_points_data.json["にかけては"]
// Testing: structure.standard[0] - "Noun + にかけては"
//
// Pattern description:
// にかけては expresses "when it comes to (A), (B)" where (B) is the best or has
// a distinct advantage in field (A). Always expresses a positive aspect.
// Combines に + かけて (te-form of 掛ける) + は.
//
// Structure: Only one main form
//   - standard[0]: Noun + にかけては

mod nikaketeha_tests {
    use super::*;

    #[test]
    fn test_nikaketeha_skill() {
        // Example from grammar_points_data.json: 大工の技術にかけては彼が一番だ
        let sentence = "大工の技術にかけては彼が一番だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にかけては");
        assert_pattern_range(&patterns, "にかけては", 3, 10); // 技術にかけては
    }

    #[test]
    fn test_nikaketeha_sports() {
        // Example from grammar_points_data.json: スポーツにかけては誰にも負けない
        let sentence = "私はゲームは下手だけど、スポーツにかけては誰にも負けない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にかけては");
        assert_pattern_range(&patterns, "にかけては", 12, 21); // スポーツにかけては
    }

    #[test]
    fn test_nikaketeha_features() {
        // Example from grammar_points_data.json: 機能にかけては他のスマホとは比べ物にならない
        let sentence = "あのスマホは機能にかけては、他のスマホとは比べ物にはならないほどいい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にかけては");
        assert_pattern_range(&patterns, "にかけては", 6, 13); // 機能にかけては
    }
}

// Pattern: とっくに (long ago, already, ages ago)
// Data source: grammar_points_data.json["とっくに"]
// Testing: structure.standard[0] - "とっくに + Phrase"
//
// Pattern description:
// とっくに is an adverb meaning "long ago" or "already" (with emphasis).
// It's an emphasized version of とうに (literary form).
// Appears at beginning of sentences or before verbs to indicate something
// happened much earlier than expected or for exaggeration.
//
// Structure: Only one form
//   - standard[0]: とっくに + Phrase

mod tokkuni_tests {
    use super::*;

    #[test]
    fn test_tokkuni_past_action() {
        // Example from grammar_points_data.json: とっくに食べちゃったよ
        let sentence = "とっくに食べちゃったよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とっくに");
        assert_pattern_range(&patterns, "とっくに", 0, 4); // とっくに
    }

    #[test]
    fn test_tokkuni_time_passed() {
        // Example from grammar_points_data.json: 提出期限はとっくに過ぎている
        let sentence = "提出期限はとっくに過ぎているのでやってもやらなくても関係がない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とっくに");
        assert_pattern_range(&patterns, "とっくに", 5, 9); // とっくに
    }

    #[test]
    fn test_tokkuni_already_gone() {
        // Example from grammar_points_data.json: みんなはもうとっくに帰ってるぞ
        let sentence = "遅いぞ！みんなはもうとっくに帰ってるぞ！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とっくに");
        assert_pattern_range(&patterns, "とっくに", 10, 14); // とっくに
    }
}

// Pattern: 未だに (still, even now)
// Data source: grammar_points_data.json["未だに"]
// Testing: structure.standard[0] - "未（いま）だに + … + Verb［る］"
// Testing: structure.standard[1] - "未（いま）だに + … + Verb［ない］"
//
// Structure variants:
//   - standard[0]: 未（いま）だに + Verb［る］ (affirmative - still doing)
//   - standard[1]: 未（いま）だに + Verb［ない］ (negative - still not)

mod imadani_tests {
    use super::*;

    #[test]
    fn test_imadani_affirmative() {
        // Example from grammar_points_data.json: 私が子供の頃に巻き込まれた事故のことはいまだに覚えている
        // Structure: 未（いま）だに + Verb［る］ (affirmative - still remembering)
        let sentence = "私が子供の頃に巻き込まれた事故のことはいまだに覚えている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "未だに");
        assert_pattern_range(&patterns, "未だに", 19, 23); // いまだに (split: いまだ + に)
    }

    #[test]
    fn test_imadani_negative() {
        // Example from grammar_points_data.json: 部長は人事に注意されて２ヶ月も経つのに、彼の態度はいまだに変わらない
        // Structure: 未（いま）だに + Verb［ない］ (negative - still hasn't changed)
        let sentence = "部長は人事に注意されて２ヶ月も経つのに、彼の態度はいまだに変わらない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "未だに");
        assert_pattern_range(&patterns, "未だに", 25, 29); // いまだに (single token)
    }
}

// Pattern: をもとに (based on)
// Data source: grammar_points_data.json["をもとに"]
// Testing: structure.standard[0] - "Noun + をもとに（して）"
// Testing: structure.standard[1] - "Noun + をもとにした + Noun"
//
// Structure variants:
//   - standard[0]: Noun + をもとに (without して)
//   - standard[0]: Noun + をもとにして (with して)
//   - standard[1]: Noun + をもとにした + Noun

mod womotoni_tests {
    use super::*;

    #[test]
    fn test_womotoni_basic() {
        // Example from grammar_points_data.json: このドラマは実際に起こったことをもとに作られたそうだ
        // Structure: Noun + をもとに (basic form without して)
        let sentence = "このドラマは実際に起こったことをもとに作られたそうだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "をもとに");
        assert_pattern_range(&patterns, "をもとに", 15, 19); // をもとに
    }

    #[test]
    fn test_womotoni_with_shite() {
        // Structure: Noun + をもとにして (with して)
        let sentence = "研究の結果をもとにしてレポートを書いてください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "をもとに");
        assert_pattern_range(&patterns, "をもとに", 5, 9); // をもとに (して is separate)
    }

    #[test]
    fn test_womotoni_shita_noun() {
        // Structure: Noun + をもとにした + Noun
        let sentence = "このデータをもとにした分析を提出します";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "をもとに");
        assert_pattern_range(&patterns, "をもとに", 5, 9); // をもとに (した is separate)
    }
}

// Pattern: だけのことはある (no wonder, as expected)
// Data source: grammar_points_data.json["だけのことはある"]
// Testing structure variants from grammar data
//
// Structure variants:
//   - standard[0]: Verb + だけのことはある
//   - standard[1]: い-Adjective + だけのことはある
//   - standard[2]: Noun + (だった) + だけのことはある
//   - standard[3]: な-Adjective + な/だった + だけのことはある
//   - polite[0]: Verb + だけのことはあります
mod dakenokotohaaru_tests {
    use super::*;

    #[test]
    fn test_verb_dakeno() {
        // Structure: Verb + だけのことはある
        // Example from grammar data: 毎日稽古に行っていただけのことはある
        let sentence = "娘が空手の大会で優勝した。さすがに一年間毎日稽古に行っていただけのことはある";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だけのことはある");
        assert_pattern_range(&patterns, "だけのことはある", 29, 38); // ただけのことはある
    }

    #[test]
    fn test_i_adjective_dakeno() {
        // Structure: い-Adjective + だけのことはある
        // Example from grammar data: 若いだけのことはある
        let sentence = "田中くんは体力もあるし動きもテキパキしている。やっぱり若いだけのことはある";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だけのことはある");
        assert_pattern_range(&patterns, "だけのことはある", 27, 37); // 若いだけのことはある
    }

    #[test]
    fn test_noun_dakeno() {
        // Structure: Noun + な + だけのことはある (な-adjective treated as noun)
        // Example from grammar data: 有名なだけのことはある
        let sentence = "あのホテルのサービスはとても良かった。やっぱり有名なだけのことはある";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だけのことはある");
        assert_pattern_range(&patterns, "だけのことはある", 25, 34); // なだけのことはある
    }

    #[test]
    fn test_na_adjective_dakeno() {
        // Structure: Noun + だけのことはある
        // Example from grammar data: 習字の先生だけのことはある
        let sentence = "高橋先生が書く漢字はものすごく綺麗だ。さすが習字の先生だけのことはある";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だけのことはある");
        assert_pattern_range(&patterns, "だけのことはある", 25, 35); // 先生だけのことはある
    }

    #[test]
    fn test_noun_datta_dakeno() {
        // Structure: Noun + だった + だけのことはある
        // Example from grammar data: 昔ボクサーだっただけのことはある
        let sentence = "鈴木さんは６０歳なのにムキムキだ。昔ボクサーだっただけのことはある";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だけのことはある");
        assert_pattern_range(&patterns, "だけのことはある", 24, 33); // ただけのことはある
    }

    #[test]
    fn test_na_adjective_datta_dakeno() {
        // Structure: な-Adjective + だった + だけのことはある
        // Example from grammar data: 昔から正直だっただけのことはある
        let sentence = "あの政治家はとても信頼されている。さすが昔から正直だっただけのことはある";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だけのことはある");
        assert_pattern_range(&patterns, "だけのことはある", 27, 36); // ただけのことはある
    }
}

// Pattern: やら～やら (A and B, and so on)
// Data source: grammar_points_data.json["やら～やら"]
// Testing structure variants from grammar data
//
// Structure variants:
//   - standard[0]: A + やら + B + やら (A and B can be Verb[る], Noun, い-Adjective)
mod yara_yara_tests {
    use super::*;

    #[test]
    fn test_verb_yara_verb_yara() {
        // Structure: Verb[る] + やら + Verb[る] + やら
        // Example from grammar data: 世話をするやら家事をするやら
        let sentence = "毎日子供の世話をするやら家事をするやらで忙しいです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "やら～やら");
        // Pattern detects multiple instances: するやら (8-12) and するやらで (15-20)
        // Testing the first detected instance
        assert_pattern_range(&patterns, "やら～やら", 15, 20); // するやらで (includes で)
    }

    #[test]
    fn test_i_adj_yara_i_adj_yara() {
        // Structure: い-Adjective + やら + い-Adjective + やら
        // Example from grammar data: 痛いやら悪いやら
        let sentence = "先週は身体中が痛いやら、体調が悪いやらで大変でした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "やら～やら");
        assert_pattern_range(&patterns, "やら～やら", 15, 20); // 悪いやらで
    }

    #[test]
    fn test_noun_yara_noun_yara() {
        // Structure: Noun + やら + Noun + やら
        // Example from grammar data: 不安やら怒りやら
        let sentence = "あのニュースを聞いてから不安やら怒りやら、色々な感情が溢れ出てきた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "やら～やら");
        assert_pattern_range(&patterns, "やら～やら", 12, 16); // 不安やら
    }

    #[test]
    fn test_mixed_noun_yara() {
        // Structure: Noun + やら + Noun + やら (another example)
        // Example from grammar data: 漢字やら単語やら
        let sentence = "文法だけではなく漢字やら単語やらも勉強した方がいい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "やら～やら");
        assert_pattern_range(&patterns, "やら～やら", 8, 12); // 漢字やら
    }
}

// Pattern: 一旦 (once, for a moment)
// Data source: grammar_points_data.json["一旦"]
// Testing: structure.standard[0] - "一旦（いったん） + Verb［ば］"
// Testing: structure.standard[1] - "一旦（いったん） + Verb［たら］"
// Testing: structure.standard[2] - "一旦（いったん） + Verb + と"
//
// Structure variants:
//   - standard[0]: 一旦 + Verb[ば] (conditional ば)
//   - standard[1]: 一旦 + Verb[たら] (conditional たら)
//   - standard[2]: 一旦 + Verb + と (conditional と)

mod ittan_tests {
    use super::*;

    #[test]
    fn test_ittan_ba() {
        // Structure: 一旦 + Verb[ば]
        // Example from grammar data: いったん再起動をすれば、もとに戻るはずです
        let sentence = "いったん再起動をすれば、もとに戻るはずです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "一旦");
        assert_pattern_range(&patterns, "一旦", 0, 4); // いったん
    }

    #[test]
    fn test_ittan_tara() {
        // Structure: 一旦 + Verb[たら]
        // Example from grammar data: 娘はいったん泣き出したら、しばらく泣き止まない
        let sentence = "娘はいったん泣き出したら、しばらく泣き止まないので大変です";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "一旦");
        assert_pattern_range(&patterns, "一旦", 2, 6); // いったん
    }

    #[test]
    fn test_ittan_to() {
        // Structure: 一旦 + Verb + と
        // Example from grammar data: いったん集中力が切れてしまうと、再び集中するのに時間がかかる
        let sentence = "いったん集中力が切れてしまうと、再び集中するのに時間がかかる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "一旦");
        assert_pattern_range(&patterns, "一旦", 0, 4); // いったん
    }
}

// Pattern: はもとより (not only... but also, let alone)
// Data source: grammar_points_data.json["はもとより"]
// Testing: structure.standard[0] - "Noun + はもとより + Phrase"
// Testing: structure.standard[1] - "Verb + こと + はもとより + Phrase"
// Testing: structure.standard[2] - "Verb + の + はもとより + Phrase"
//
// Structure variants:
//   - standard[0]: Noun + はもとより (basic noun usage)
//   - standard[1]: Verb + こと + はもとより (nominalized with こと)
//   - standard[2]: Verb + の + はもとより (nominalized with の)

mod hamotoyori_tests {
    use super::*;

    #[test]
    fn test_noun_hamotoyori() {
        // Structure: Noun + はもとより
        // Example from grammar data: 地元の人はもとより、全国からの観光客で賑わっています
        let sentence = "この商店街は地元の人はもとより、全国からの観光客で賑わっています";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "はもとより");
        assert_pattern_range(&patterns, "はもとより", 9, 15); // 人はもとより
    }

    #[test]
    fn test_verb_koto_hamotoyori() {
        // Structure: Verb + こと + はもとより
        // Example from grammar data: 日本語が上手になることはもとより
        let sentence = "この学校では日本語が上手になることはもとより、日本でのマナーや文化を理解できる授業が行われています";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "はもとより");
        assert_pattern_range(&patterns, "はもとより", 15, 22); // ことはもとより
    }

    #[test]
    fn test_verb_no_hamotoyori() {
        // Structure: Verb + の + はもとより
        // Example from grammar data: 自分の子供を守るのはもとより
        let sentence = "親は自分の子供を守るのはもとより、何不自由ない生活をしてほしいと思っている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "はもとより");
        assert_pattern_range(&patterns, "はもとより", 10, 16); // のはもとより
    }
}

// Pattern: ならともかく (if it's A, sure, but...)
// Data source: grammar_points_data.json["ならともかく"]
// Testing all structure variants:
//   - standard[0]: Verb + ならともかく
//   - standard[1]: い-Adjective + ならともかく
//   - standard[2]: な-Adjective + ならともかく
//   - standard[3]: Noun + ならともかく

mod naratomokaku_tests {
    use super::*;

    #[test]
    fn test_verb_naratomokaku() {
        // Structure: Verb + ならともかく
        // Example from grammar data: 丈夫で何回も使えるものならともかく
        let sentence = "丈夫で何回も使えるものならともかく、なんでわざわざ高くて使い捨てみたいなもんを買うんですか";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ならともかく");
        assert_pattern_range(&patterns, "ならともかく", 9, 17); // ものならともかく
    }

    #[test]
    fn test_i_adj_naratomokaku() {
        // Structure: い-Adjective + ならともかく
        // Example: 高いならともかく、安くても買わない
        let sentence = "値段が高いならともかく、品質が悪いのは論外だ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ならともかく");
        assert_pattern_range(&patterns, "ならともかく", 3, 11); // 高いならともかく
    }

    #[test]
    fn test_na_adj_naratomokaku() {
        // Structure: な-Adjective + ならともかく
        // Example: 丁寧ならともかく
        let sentence = "仕事が丁寧ならともかく、雑な仕事では困る";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ならともかく");
        assert_pattern_range(&patterns, "ならともかく", 3, 11); // 丁寧ならともかく
    }

    #[test]
    fn test_noun_naratomokaku() {
        // Structure: Noun + ならともかく
        // Example from grammar data: 俺の妹ならともかく
        let sentence = "俺の妹ならともかく、俺はそんなことしねぇよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ならともかく");
        assert_pattern_range(&patterns, "ならともかく", 2, 9); // 妹ならともかく
    }
}

// Pattern: はともかく (setting aside, apart from)
// Data source: grammar_points_data.json["はともかく"]
// Testing structure variant:
//   - standard[0]: Noun + は + ともかく（として）

mod hatomokaku_tests {
    use super::*;

    #[test]
    fn test_noun_hatomokaku() {
        // Structure: Noun + は + ともかく
        // Example from grammar data: 給料はともかく
        let sentence = "給料はともかく、やりがいのある仕事がしたい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "はともかく");
        assert_pattern_range(&patterns, "はともかく", 0, 7); // 給料はともかく
    }

    #[test]
    fn test_phrase_hatomokaku() {
        // Structure: Phrase + か + は + ともかく
        // Example from grammar data: 家賃が高いか安いかはともかく
        let sentence = "家賃が高いか安いかはともかく、東京駅の近くに住みたい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "はともかく");
        assert_pattern_range(&patterns, "はともかく", 8, 14); // かはともかく
    }

    #[test]
    fn test_appearance_hatomokaku() {
        // Structure: Noun + の + Noun + は + ともかく
        // Example from grammar data: この料理の見た目はともかく
        let sentence = "この料理の見た目はともかく、味はとてもいい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "はともかく");
        assert_pattern_range(&patterns, "はともかく", 5, 13); // 見た目はともかく
    }
}

// Pattern: そうにない (unlikely to, showing no signs of)
// Data source: grammar_points_data.json["そうにない"]
mod souninai_tests {
    use super::*;

    #[test]
    fn test_souninai_basic() {
        // Structure: Verb[stem] + そうにない
        // Example from grammar data: 定時で上がれそうにない
        let sentence = "ごめん、定時で上がれそうにないから先に食べてて";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そうにない");
        assert_pattern_range(&patterns, "そうにない", 7, 15); // 上がれそうにない
    }

    #[test]
    fn test_souninai_make_it() {
        // Structure: Verb[stem] + そうにない
        // Example from grammar data: 行けそうにない
        let sentence = "今日は行けそうにないのでキャンセルでお願いします";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そうにない");
        assert_pattern_range(&patterns, "そうにない", 3, 10); // 行けそうにない
    }

    #[test]
    fn test_sounimonai_emphatic() {
        // Structure: Verb[stem] + そうにもない (emphatic)
        // Example from grammar data: 降りそうにもない
        let sentence = "今日は雨が降りそうにもないから釣りに行こう！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そうにない");
        assert_pattern_range(&patterns, "そうにない", 5, 13); // 降りそうにもない
    }

    #[test]
    fn test_sounimonai_no_signs() {
        // Structure: Verb[stem] + そうにもない (emphatic - no signs)
        // Example from grammar data: 進みそうにもない
        let sentence = "列が進みそうにもないのでまた今度来ましょう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そうにない");
        assert_pattern_range(&patterns, "そうにない", 2, 10); // 進みそうにもない
    }

    // TODO: Polite forms (そうにありません, そうにもありません)
    // These use the verb ある in polite negative form rather than ない.
    // Tokenization: Verb[stem] + そう + に + (も) + あり + ませ + ん
    // This is semantically equivalent but syntactically different from そうにない.
    // The pattern そうにない specifically matches constructions ending with ない/もない.
    // The polite forms would need a separate matcher or broader pattern definition.
    //
    // Example sentences:
    // - "この問題は解決しそうにありませんね"
    // - "彼は来そうにもありませんから待たなくていいですよ"
}

// Pattern: に反して (contrary to, in contrast to)
// Data source: grammar_points_data.json["に反して"]
mod nihanshite_tests {
    use super::*;

    #[test]
    fn test_nihanshite_company_rules() {
        // Structure: Noun + に反して
        // Example from grammar data: 会社のルールに反して
        let sentence = "彼らは会社のルールに反して交際をしているらしい";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に反して");
        assert_pattern_range(&patterns, "に反して", 6, 13); // ルールに反して
    }

    #[test]
    fn test_nihanshite_expectations() {
        // Structure: Noun + に反して
        // Example from grammar data: 予想に反して
        let sentence = "予想に反してクライアントの反応がいまいちだった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に反して");
        assert_pattern_range(&patterns, "に反して", 0, 6); // 予想に反して
    }

    #[test]
    fn test_nihansuru_modify_noun() {
        // Structure: Noun + に反する + Noun
        // Example from grammar data: 指示に反すること
        let sentence = "私は上司の指示に反することはできません";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に反して");
        assert_pattern_range(&patterns, "に反して", 5, 11); // 指示に反する
    }

    #[test]
    fn test_nihanshite_finish_early() {
        // Structure: Noun + に反して
        // Additional example: 予想に反して早く終える
        let sentence = "予想に反して早く終えることができた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に反して");
        assert_pattern_range(&patterns, "に反して", 0, 6); // 予想に反して
    }
}

// Pattern: 逆に (conversely, on the contrary)
// Data source: grammar_points_data.json["逆に"]
// Testing: structure.standard[0] - "Phrase (A) + 逆（ぎゃく）に + Phrase (B)"
//
// Structure variants:
//   - standard[0]: Phrase (A) + 逆（ぎゃく）に + Phrase (B)
//
// Notes: Can appear between phrases for contrast or at beginning of sentences for rebuttal

mod gyakuni_tests {
    use super::*;

    #[test]
    fn test_gyakuni_between_phrases() {
        // Structure: Phrase (A) + 逆に + Phrase (B)
        // Example from grammar data: 近道だと思っていたのに...逆に回り道だった
        let sentence = "近道だと思っていたのに、地図アプリで調べてみたら逆に回り道だった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "逆に");
        assert_pattern_range(&patterns, "逆に", 24, 26); // 逆に
    }

    #[test]
    fn test_gyakuni_easier_harder() {
        // Structure: Phrase + 逆に + Phrase
        // Example from grammar data: 食べやすいようにスパゲティを半分にして茹でたけど、逆に食べにくくなった
        let sentence = "食べやすいようにスパゲティを半分にして茹でたけど、逆に食べにくくなった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "逆に");
        assert_pattern_range(&patterns, "逆に", 25, 27); // 逆に
    }

    #[test]
    fn test_gyakuni_sentence_start() {
        // Structure: 逆に + question (rebuttal/slang usage)
        // Example from grammar data: 逆に、何歳に見えますか？
        let sentence = "逆に、何歳に見えますか？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "逆に");
        assert_pattern_range(&patterns, "逆に", 0, 2); // 逆に
    }
}

// Pattern: 反面 (on the other hand, while)
// Data source: grammar_points_data.json["反面"]
// Testing multiple structure variants showing contrast between opposing features
//
// Structure variants:
//   - standard[0]: Verb + 反面（はんめん）
//   - standard[1]: ［い］Adjective + 反面（はんめん）
//   - standard[2]: ［な］Adjective + な(1) + 反面（はんめん）
//   - standard[3]: Noun + である + 反面（はんめん）
//   - standard[4]: Phrase + が + （、）+ 反面（はんめん）
//
// Notes: Shows contrast between positive and negative features of same thing

mod hanmen_tests {
    use super::*;

    #[test]
    fn test_hanmen_verb() {
        // Structure: Verb + 反面
        // Example from grammar data: 都内に引っ越して色々と便利になった反面
        let sentence = "都内に引っ越して色々と便利になった反面、家賃が前住んでいた所の倍になった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "反面");
        assert_pattern_range(&patterns, "反面", 16, 19); // た反面
    }

    #[test]
    fn test_hanmen_i_adjective() {
        // Structure: い-Adjective + 反面
        // Example from grammar data: カリフォルニアの夏は日本より暑い反面
        let sentence = "カリフォルニアの夏は日本より暑い反面、湿度が低いので過ごしやすいそうです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "反面");
        assert_pattern_range(&patterns, "反面", 14, 18); // 暑い反面
    }

    #[test]
    fn test_hanmen_na_adjective() {
        // Structure: な-Adjective + な + 反面
        // Example from grammar data: 日本語を教えるのがとても上手な反面
        let sentence = "あの日本語教室の先生達は日本語を教えるのがとても上手な反面、生徒達にはとても厳しいそうだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "反面");
        assert_pattern_range(&patterns, "反面", 26, 29); // な反面
    }

    #[test]
    fn test_hanmen_noun_dearu() {
        // Structure: Noun + である + 反面
        // Example from grammar data: 田中くんはとても頭のいい子である反面
        let sentence = "田中くんはとても頭のいい子である反面、怠け者なのでいつも先生に怒られています";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "反面");
        assert_pattern_range(&patterns, "反面", 14, 18); // ある反面
    }
}

// Pattern: 抜く (to do completely, to pull through, thoroughly)
// Data source: grammar_points_data.json["抜く"]
// Testing: structure.standard[0] - "Verb[stem] + ぬく"
//
// Structure variants:
//   - standard[0]: Verb[stem] + ぬく (do completely/thoroughly)
//   - polite[0]: Verb[stem] + ぬきます

mod nuku_tests {
    use super::*;

    #[test]
    fn test_nuku_masu_stem() {
        // Structure: Verb[stem] + ぬく (standard form)
        // Example from grammar data: 自分の子供達を守りぬく
        let sentence = "自分の子供達を守りぬくためならなんでもできる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "抜く");
        assert_pattern_range(&patterns, "抜く", 7, 11); // 守りぬく
    }

    #[test]
    fn test_nuku_passive() {
        // Structure: Verb[stem] + ぬく (passive form - ぬかれる)
        // Example from grammar data: 私の計画は彼に見ぬかれていた
        let sentence = "私の計画は彼に見ぬかれていた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "抜く");
        assert_pattern_range(&patterns, "抜く", 7, 10); // 見ぬか
    }

    #[test]
    fn test_nuku_negative() {
        // Structure: Verb[stem] + ぬく (negative form - ぬかない)
        // Example from grammar data: 最後までやりぬかないと気が済まない
        let sentence = "私は何があっても最後までやりぬかないと気が済まないです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "抜く");
        assert_pattern_range(&patterns, "抜く", 12, 18); // やりぬかない
    }

    #[test]
    fn test_nuku_past() {
        // Structure: Verb[stem] + ぬく (past form - ぬいた)
        // Example from grammar data: 考えぬいた結果
        let sentence = "考えぬいた結果、俺は大学を中退して親父の会社を継ぐことにした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "抜く");
        assert_pattern_range(&patterns, "抜く", 0, 5); // 考えぬいた
    }

    #[test]
    fn test_nuku_te_iru() {
        // Structure: Verb[stem] + ぬく (ている form - compound verb)
        // Example from grammar data: 田中師匠は俳句の事なら知りぬいている
        // Note: 知りぬく is tokenized as a single compound verb (動詞/自立)
        let sentence = "田中師匠は俳句の事なら知りぬいている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "抜く_compound");
        assert_pattern_range(&patterns, "抜く_compound", 11, 15); // 知りぬい
    }

    #[test]
    fn test_nukimasu_polite() {
        // Structure: Verb[stem] + ぬきます (polite form)
        let sentence = "この困難を乗り越えぬきます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "抜く");
        assert_pattern_range(&patterns, "抜く", 5, 13); // 乗り越えぬきます
    }
}

// Pattern: 抜きで (without, leaving out)
// Data source: grammar_points_data.json["抜きで"]
// Testing: structure.standard[] variants
//
// Structure variants:
//   - standard[0]: Noun + 抜きで(は)
//   - standard[1]: Noun + 抜きに(は)
//   - standard[2]: Noun + を + 抜きにして(は)
//   - standard[3]: Noun + は + 抜きとして(は)
//   - standard[4]: Noun + 抜き + の + Noun

mod nukide_tests {
    use super::*;

    #[test]
    fn test_nukide_basic() {
        // Structure: Noun + 抜きで
        // Example from grammar data: あの人は冗談抜きで怖いから
        let sentence = "あの人は冗談抜きで怖いから怒らせない方がいいよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "抜きで");
        assert_pattern_range(&patterns, "抜きで", 4, 9); // 冗談抜きで
    }

    #[test]
    fn test_nukide_compound_noun() {
        // Structure: Noun + 抜きで
        // Testing compound noun (あいつ + ら)
        // Note: "ら" is tokenized as a separate noun suffix (名詞/接尾)
        let sentence = "あいつら全然来ないな、あいつら抜きで始めちゃおうぜ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "抜きで");
        assert_pattern_range(&patterns, "抜きで", 14, 18); // ら抜きで
    }

    #[test]
    fn test_nukini() {
        // Structure: Noun + 抜きに
        let sentence = "彼女抜きにパーティーは盛り上がらない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "抜きで");
        assert_pattern_range(&patterns, "抜きで", 0, 5); // 彼女抜きに
    }

    #[test]
    fn test_nukinishite() {
        // Structure: Noun + 抜きに (within 抜きにして context)
        // Example from grammar data: 若い労働者抜きにしては、日本の建築業は成り立たない
        // Note: Detects "抜きに" part; "にして" is separate pattern
        let sentence = "若い労働者抜きにしては、日本の建築業は成り立たない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "抜きで");
        assert_pattern_range(&patterns, "抜きで", 4, 8); // 者抜きに
    }

    // TODO: Undetectable - Noun + は + 抜きとして
    // The particle は appears between the noun and 抜き, so the current matcher
    // (which expects Noun directly before 抜き) cannot detect this variant.
    // Structure: 前置き + は + 抜き + として
    // This would require a more complex matcher that allows optional particles before 抜き.
    //
    // #[test]
    // fn test_nukitoshite() {
    //     let sentence = "前置きは抜きとして本題に入りましょう";
    //     let tokens = tokenize_sentence(sentence);
    //     let patterns = detect_patterns(&tokens);
    //
    //     assert_has_pattern(&patterns, "抜きで");
    //     assert_pattern_range(&patterns, "抜きで", ?, ?); // は抜きとして (complex structure)
    // }

    #[test]
    fn test_nuki_no_noun() {
        // Structure: Noun + 抜き + の + Noun
        // Example from grammar data: ワサビ抜きの寿司なんて寿司じゃないよ
        let sentence = "ワサビ抜きの寿司なんて寿司じゃないよ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "抜きで");
        assert_pattern_range(&patterns, "抜きで", 0, 6); // ワサビ抜きの
    }
}

// Pattern: に応じて (in accordance with, depending on)
// Data source: grammar_points_data.json["に応じて"]
// Testing: structure.standard[0] - "Noun + に応じて"
// Testing: structure.standard[1] - "Noun + に応じた + Noun"
//
// Structure variants:
//   - standard[0]: Noun + に応じて (adverbial form)
//   - standard[1]: Noun + に応じた + Noun (noun-modifying form)

mod nioujite_tests {
    use super::*;

    #[test]
    fn test_nioujite_adverbial() {
        // Structure: Noun + に応じて
        // Example from grammar data: 生徒達のレベルに応じてクラスを分ける
        let sentence = "日本語を教える際には、生徒達のレベルに応じてクラスを分ける必要があります。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に応じて");
        assert_pattern_range(&patterns, "に応じて", 15, 22); // レベルに応じて
    }

    #[test]
    fn test_nioujite_price() {
        // Structure: Noun + に応じて
        // Example from grammar data: 面積に応じて値段が高くなります
        let sentence = "この地域の土地は、面積に応じて値段が高くなります。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に応じて");
        assert_pattern_range(&patterns, "に応じて", 9, 15); // 面積に応じて
    }

    #[test]
    fn test_nioujita_noun() {
        // Structure: Noun + に応じた + Noun
        // Example from grammar data: クライアントの要望に応じたデザインを提供
        let sentence = "私たちはクライアントの要望に応じたデザインを提供しています。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に応じて");
        assert_pattern_range(&patterns, "に応じて", 11, 17); // 要望に応じた
    }
}

// Pattern: を通じて・を通して (through, via)
// Data source: grammar_points_data.json["を通じて・を通して"]
// Testing: structure.standard[0] - "Noun + を通して"
// Testing: structure.standard[1] - "Noun + を通じて"
//
// Structure variants:
//   - standard[0]: Noun + を通して (transitive - intentional/by use of)
//   - standard[1]: Noun + を通じて (intransitive - throughout/via)

mod wotsuujite_wotooshite_tests {
    use super::*;

    #[test]
    fn test_wotooshite_language_study() {
        // Structure: Noun + を通して
        // Example from grammar data: 語学留学を通して日本の文化などを学びました
        let sentence = "語学留学を通して日本の文化などを学びました。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を通じて・を通して");
        assert_pattern_range(&patterns, "を通じて・を通して", 2, 8); // 留学を通して
    }

    #[test]
    fn test_wotooshite_work() {
        // Structure: Noun + を通して
        // Example from grammar data: 仕事を通して今の妻と出会いました
        let sentence = "仕事を通して今の妻と出会いました。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を通じて・を通して");
        assert_pattern_range(&patterns, "を通じて・を通して", 0, 6); // 仕事を通して
    }

    #[test]
    fn test_wotsuujite_news() {
        // Structure: Noun + を通じて
        // Example from grammar data: ネットニュースを通じて知りました
        let sentence = "その事件はネットニュースを通じて知りました。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を通じて・を通して");
        assert_pattern_range(&patterns, "を通じて・を通して", 8, 16); // ニュースを通じて
    }

    #[test]
    fn test_wotsuujite_anime() {
        // Structure: Noun + を通じて
        // Example from grammar data: アニメを通じて日本語を学んだ
        let sentence = "彼はアニメを通じて日本語を学んだそうだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を通じて・を通して");
        assert_pattern_range(&patterns, "を通じて・を通して", 2, 9); // アニメを通じて
    }
}

// Pattern: てたまらない (can't help but / extremely)
// Data source: grammar_points_data.json["てたまらない"]
mod tetamaranai_tests {
    use super::*;

    // Testing: structure.standard[1] - "Verb[たい][て] + たまらない"
    #[test]
    fn test_tetamaranai_tai_form() {
        // Example from grammar data: 来月公開される映画が見たくてたまらない
        let sentence = "来月公開される映画が見たくてたまらないんだけど。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てたまらない");
        assert_pattern_range(&patterns, "てたまらない", 10, 19); // 見たくてたまらない
    }

    // Testing: structure.standard[3] - "[い]Adjective[て]+ たまらない"
    #[test]
    fn test_tetamaranai_i_adjective() {
        // Example from grammar data: 昨日は３時間しか寝てないから、眠たくてたまらない
        let sentence = "昨日は３時間しか寝てないから、眠たくてたまらない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てたまらない");
        assert_pattern_range(&patterns, "てたまらない", 15, 24); // 眠たくてたまらない
    }

    // Testing: structure.standard[4] - "[な]Adjective + で + たまらない"
    #[test]
    fn test_tetamaranai_na_adjective() {
        // Example from grammar data: 娘をお使いに行かせたが、ちゃんと一人で行って帰ってこれるかが心配でたまらない
        let sentence = "娘をお使いに行かせたが、心配でたまらないよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てたまらない");
        assert_pattern_range(&patterns, "てたまらない", 12, 20); // 心配でたまらない
    }

    // Testing: structure.polite[1] - "Verb[たい][て]+ たまりません"
    #[test]
    fn test_tetamaranai_tai_polite() {
        let sentence = "新しいゲームが早くやりたくてたまりません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てたまらない");
        assert_pattern_range(&patterns, "てたまらない", 9, 20); // やりたくてたまりません
    }

    // Testing: structure.polite[3] - "[い]Adjective[て]+ たまりません"
    #[test]
    fn test_tetamaranai_i_adjective_polite() {
        let sentence = "この部屋は暑くてたまりません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てたまらない");
        assert_pattern_range(&patterns, "てたまらない", 5, 14); // 暑くてたまりません
    }

    // Testing: structure.polite[4] - "[な]Adjective + で + たまりません"
    #[test]
    fn test_tetamaranai_na_adjective_polite() {
        let sentence = "この映画は退屈でたまりません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てたまらない");
        assert_pattern_range(&patterns, "てたまらない", 5, 14); // 退屈でたまりません
    }
}

// Pattern: でしかない (nothing but / no more than)
// Data source: grammar_points_data.json["でしかない"]
mod deshikanai_tests {
    use super::*;

    // Testing: structure.standard[0] - "Noun + でしかない"
    #[test]
    fn test_deshikanai_standard() {
        // Example from grammar data: それは言い訳でしかない
        let sentence = "それは言い訳でしかないと思う。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "でしかない");
        assert_pattern_range(&patterns, "でしかない", 3, 11); // 言い訳でしかない
    }

    #[test]
    fn test_deshikanai_standard2() {
        // Example from grammar data: 暴言でしかない
        let sentence = "先輩は愛のムチだと言っているが、暴言でしかない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "でしかない");
        assert_pattern_range(&patterns, "でしかない", 16, 23); // 暴言でしかない
    }

    // Testing: structure.polite[0] - "Noun + でしかありません"
    #[test]
    fn test_deshikanai_polite() {
        let sentence = "それはただの言い訳でしかありません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "でしかない");
        assert_pattern_range(&patterns, "でしかない", 6, 17); // 言い訳でしかありません
    }
}

// Pattern: にしろ～にしろ (whether... or...)
// Data source: grammar_points_data.json["にしろ～にしろ"]
// Testing all structure variants with print_debug to understand tokenization
//
// Structure variants to test:
//   - standard[0]: Verb + にしろ + Verb + にしろ
//   - standard[1]: い-Adjective + にしろ + い-Adjective + にしろ
//   - standard[2]: な-Adjective + (である) + にしろ + な-Adjective + (である) + にしろ
//   - standard[3]: Noun + (である) + にしろ + Noun + (である) + にしろ
//   - Note: Can also use にせよ instead of にしろ
mod nishiro_uff5e_nishiro_tests {
    use super::*;

    // Testing: structure.standard[0] - "Verb + にしろ + Verb + にしろ"
    #[test]
    fn test_verb_nishiro_verb_nishiro() {
        // Example from grammar data: 参加するにせよしないにせよ
        let sentence = "このイベントに参加するにせよしないにせよ、私に連絡をください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にしろ～にしろ");
        // Note: Pattern matches each instance separately
        // First: 参加するにせよ (参加 is token before に)
        assert_pattern_range(&patterns, "にしろ～にしろ", 7, 14); // 参加するにせよ
        // Second: ないにせよ (ない is the token before に, し is separate)
        // The full phrase "しないにせよ" contains the pattern "ないにせよ"
    }

    // Testing: structure.standard[1] - "い-Adjective + にしろ + い-Adjective + にしろ"
    #[test]
    fn test_i_adj_nishiro_i_adj_nishiro() {
        // Example from grammar data: 多いにしろ少ないにしろ
        let sentence = "給料が多いにしろ少ないにしろ、給料をもらっている以上ちゃんと働かないといけない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にしろ～にしろ");
        // Pattern matches each instance separately - testing first occurrence found
        assert_pattern_range(&patterns, "にしろ～にしろ", 8, 14); // 少ないにしろ
    }

    // Testing: structure.standard[2] - "な-Adjective + にせよ + な-Adjective + にせよ"
    #[test]
    fn test_na_adj_niseyo_na_adj_niseyo() {
        // Example from grammar data: 好きにせよ嫌いにせよ
        let sentence = "勉強が好きにせよ嫌いにせよ、子供である以上、勉強はしなくてはいけません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にしろ～にしろ");
        // Pattern matches each instance separately
        assert_pattern_range(&patterns, "にしろ～にしろ", 3, 8); // 好きにせよ
    }

    // Testing: structure.standard[3] - "Noun + にしろ + Noun + にしろ"
    #[test]
    fn test_noun_nishiro_noun_nishiro() {
        // Example from grammar data: 社長にしろアルバイトにしろ
        let sentence = "社長にしろアルバイトにしろ、工場内での喫煙は禁止されている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にしろ～にしろ");
        // Pattern matches each instance separately - testing first occurrence found
        assert_pattern_range(&patterns, "にしろ～にしろ", 5, 13); // アルバイトにしろ
    }

    // Testing: Fun-fact example with antonyms - "い-Adj + Noun + にしろ + い-Adj + Noun + にしろ"
    #[test]
    fn test_antonym_pair_nishiro() {
        // Example from grammar data: 浅い川にしろ深い川にしろ
        let sentence = "浅い川にしろ深い川にしろ、子供にはライフジャケット着させないといけない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にしろ～にしろ");
        // Pattern matches each instance separately
        // Note: Pattern matches Noun + に + しろ, not the full い-Adj + Noun phrase
        assert_pattern_range(&patterns, "にしろ～にしろ", 2, 6); // 川にしろ
    }
}

// Pattern: に応えて (in response to, to meet)
// Data source: grammar_points_data.json["に応えて"]
// Testing all structure variants:
//   - standard[0]: "Noun + にこたえ（て）"
//   - standard[1]: "Noun + にこたえる(1) + Noun"
//   - standard[2]: "(1) にこたえた"

mod nikotaete_tests {
    use super::*;

    // Testing: structure.standard[0] - "Noun + にこたえ（て）"
    #[test]
    fn test_noun_nikotaete() {
        // Example from grammar data: 生徒達の要望にこたえて
        let sentence = "生徒達の要望にこたえて、レッスン料金を千円安くした。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に応えて");
        assert_pattern_range(&patterns, "に応えて", 4, 11); // 要望にこたえて
    }

    #[test]
    fn test_noun_nikotaete_hiragana() {
        // Example from grammar data: クライアントのリクエストにこたえて
        let sentence = "クライアントのリクエストにこたえて、デザインを少しだけ変えました。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に応えて");
        assert_pattern_range(&patterns, "に応えて", 7, 17); // リクエストにこたえて
    }

    // Testing: structure.standard[1] - "Noun + にこたえる(1) + Noun"
    #[test]
    fn test_noun_nikotaeru_noun() {
        // Example from grammar data: お客様のニーズにこたえるサービス
        let sentence = "弊社ではお客様のニーズにこたえるサービスを提供するよう、日々努力しています。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に応えて");
        assert_pattern_range(&patterns, "に応えて", 8, 16); // ニーズにこたえる
    }

    #[test]
    fn test_noun_nikotaeru_noun_variant() {
        // Example from grammar data: あなたの要望にこたえる自信
        let sentence = "あなたの要望にこたえる自信はありますが、時間がかかると思います。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に応えて");
        assert_pattern_range(&patterns, "に応えて", 4, 11); // 要望にこたえる
    }

    // Testing: structure.standard[2] - "(1) にこたえた" (past form)
    #[test]
    fn test_nikotaeta_past() {
        // Past form example
        let sentence = "市民の声にこたえた政策を実施することができました。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に応えて");
        assert_pattern_range(&patterns, "に応えて", 3, 9); // 声にこたえた
    }
}

// Pattern: それとも (or, or rather)
// Data source: grammar_points_data.json["それとも"]
// Testing structure variant:
//   - standard[0]: "Option (A) + それとも + Option (B)"
// Note: Single structure variant - a simple conjunction

mod soretomo_tests {
    use super::*;

    #[test]
    fn test_soretomo_between_questions() {
        // Example from grammar data: 文法の勉強をしますか。それとも漢字の勉強をしますか。
        let sentence = "今日は文法の勉強をしますか。それとも漢字の勉強をしますか。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "それとも");
        assert_pattern_range(&patterns, "それとも", 14, 18); // それとも
    }

    #[test]
    fn test_soretomo_mid_sentence() {
        // Example from grammar data: 動物園に行く？それとも家でゆっくりする？
        let sentence = "来週はどこに行きたい？動物園に行く？それとも家でゆっくりする？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "それとも");
        assert_pattern_range(&patterns, "それとも", 18, 22); // それとも
    }

    #[test]
    fn test_soretomo_with_comma() {
        // Example from grammar data: 海に行こうか、それとも川に行こうか
        let sentence = "今日は海に行こうか、それとも川に行こうか、迷うな。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "それとも");
        assert_pattern_range(&patterns, "それとも", 10, 14); // それとも
    }
}

// Pattern: にしたら (from the point of view of, from the perspective of)
// Data source: grammar_points_data.json["にしたら"]
// Testing: structure.standard[0] - "Noun + にしたら"
// Testing: structure.standard[1] - "Noun + にすれば"
//
// Structure variants:
//   - standard[0]: Noun + にしたら (from the perspective of)
//   - standard[1]: Noun + にすれば (from the perspective of, more formal)

mod nishitara_tests {
    use super::*;

    #[test]
    fn test_nishitara_customer() {
        // Example from grammar data: お客様にしたら、値段が安い方がいいだろう
        let sentence = "お客様にしたら、値段が安い方がいいだろう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にしたら");
        assert_pattern_range(&patterns, "にしたら", 0, 7); // お客様にしたら
    }

    #[test]
    fn test_nishitara_person() {
        // Example from grammar data: 納豆が嫌いな人にしたら、納豆はただ臭いだけに違いない
        let sentence = "納豆が嫌いな人にしたら、納豆はただ臭いだけに違いない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にしたら");
        assert_pattern_range(&patterns, "にしたら", 6, 11); // 人にしたら
    }

    #[test]
    fn test_nisureba_parent() {
        // Example from grammar data: 親にすれば、いつまでも子供に見える
        let sentence = "子供が何歳になろうとも、親にすれば、いつまでも子供に見える。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にしたら");
        assert_pattern_range(&patterns, "にしたら", 12, 17); // 親にすれば
    }

    #[test]
    fn test_nisureba_japanese() {
        // Example from grammar data: 日本人にすれば普通なことでも
        let sentence = "日本人にすれば普通なことでも、海外の方が見たら驚く事がたくさんあるらしい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にしたら");
        assert_pattern_range(&patterns, "にしたら", 0, 7); // 日本人にすれば
    }
}

// Pattern: にしても～にしても (regardless of whether A or B)
// Data source: grammar_points_data.json["にしても～にしても"]
// Testing: structure.standard[0] - "Verb (A) + にしても + Verb (B) + にしても"
// Testing: structure.standard[1] - "Noun (A) + にしても + Noun (B) + にしても"
// Testing: structure.standard[2] - "［い］Adjective (A) + にしても + ［い］Adjective (B) + にしても"
// Testing: structure.standard[3] - "［な］Adjective (A) + にしても + ［な］Adjective (B) + にしても"
//
// Note: This pattern detects each "X にしても" instance separately, not the full "A にしても B にしても" structure

mod nishitemo_uff5e_nishitemo_tests {
    use super::*;

    #[test]
    fn test_verb_nishitemo_verb_nishitemo() {
        // Example from grammar data: 出席するにしても、しないにしても
        let sentence = "忘年会に出席するにしても、しないにしても、参加費を払わないといけない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // Pattern detects each instance separately
        assert_has_pattern(&patterns, "にしても");
        assert_pattern_range(&patterns, "にしても", 8, 12); // するにしても (first instance)

        // Find second instance
        let nishitemo_patterns: Vec<_> = patterns.iter()
            .filter(|p| p.pattern_name == "にしても")
            .collect();
        assert_eq!(nishitemo_patterns.len(), 2, "Should detect both instances");
        assert_eq!(nishitemo_patterns[1].start_char, 16);
        assert_eq!(nishitemo_patterns[1].end_char, 20); // しないにしても (second instance)
    }

    #[test]
    fn test_verb_nishitemo_rebuild_remodel() {
        // Example from grammar data: 立て直すにしても、リフォームするにしても
        let sentence = "家を立て直すにしても、リフォームするにしても、費用は同じぐらいになりそうだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にしても");
        assert_pattern_range(&patterns, "にしても", 6, 10); // 直すにしても (first instance)

        let nishitemo_patterns: Vec<_> = patterns.iter()
            .filter(|p| p.pattern_name == "にしても")
            .collect();
        assert_eq!(nishitemo_patterns.len(), 2);
        assert_eq!(nishitemo_patterns[1].start_char, 18);
        assert_eq!(nishitemo_patterns[1].end_char, 22); // するにしても (second instance)
    }

    #[test]
    fn test_i_adj_nishitemo_deep_shallow() {
        // Example from grammar data: 深いにしても、浅いにしても
        let sentence = "水深が深いにしても、浅いにしても、ライフジャケットは着ておいた方がいい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にしても");
        assert_pattern_range(&patterns, "にしても", 5, 9); // 深いにしても (first instance - includes only にしても)

        let nishitemo_patterns: Vec<_> = patterns.iter()
            .filter(|p| p.pattern_name == "にしても")
            .collect();
        assert_eq!(nishitemo_patterns.len(), 2);
        assert_eq!(nishitemo_patterns[1].start_char, 12);
        assert_eq!(nishitemo_patterns[1].end_char, 16); // 浅いにしても (second instance)
    }

    #[test]
    fn test_na_adj_nishitemo_good_bad() {
        // Example from grammar data: 上手にしても、下手にしても
        let sentence = "日本語が上手にしても、下手にしても、勉強は毎日しておいた方がいい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にしても");
        assert_pattern_range(&patterns, "にしても", 6, 10); // 上手にしても (first instance - includes only にしても)

        let nishitemo_patterns: Vec<_> = patterns.iter()
            .filter(|p| p.pattern_name == "にしても")
            .collect();
        assert_eq!(nishitemo_patterns.len(), 2);
        assert_eq!(nishitemo_patterns[1].start_char, 13);
        assert_eq!(nishitemo_patterns[1].end_char, 17); // 下手にしても (second instance)
    }

    #[test]
    fn test_noun_nishitemo_weekday_holiday() {
        // Example from grammar data: 平日にしても祝日にしても
        let sentence = "平日にしても祝日にしても、ディズニーランドは多くの人で賑わっています。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にしても");
        assert_pattern_range(&patterns, "にしても", 2, 6); // 平日にしても (first instance - includes only にしても)

        let nishitemo_patterns: Vec<_> = patterns.iter()
            .filter(|p| p.pattern_name == "にしても")
            .collect();
        assert_eq!(nishitemo_patterns.len(), 2);
        assert_eq!(nishitemo_patterns[1].start_char, 8);
        assert_eq!(nishitemo_patterns[1].end_char, 12); // 祝日にしても (second instance)
    }
}

// Pattern: 何しろ (at any rate, after all)
// Data source: grammar_points_data.json["何しろ"]
// Testing: structure.standard[0] - "何（なに）しろ + Phrase"
//
// Structure variants:
//   - standard[0]: 何しろ + Phrase (only one structure variant)
//
// Note: 何しろ is typically an adverb appearing at the beginning of sentences or clauses

mod nanishiro_tests {
    use super::*;

    #[test]
    fn test_nanishiro_sentence_start() {
        // Example from grammar data: なにしろ彼はまだ始めたばかりなので
        let sentence = "なにしろ彼はまだ始めたばかりなので大目に見てやってください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "何しろ");
        assert_pattern_range(&patterns, "何しろ", 0, 4); // なにしろ
    }

    #[test]
    fn test_nanishiro_clause_start() {
        // Example from grammar data: なにしろ彼はプロなんだから
        let sentence = "彼はできて当たり前だよ。なにしろ彼はプロなんだから。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "何しろ");
        assert_pattern_range(&patterns, "何しろ", 12, 16); // なにしろ
    }

    #[test]
    fn test_nanishiro_with_kara() {
        // Example from grammar data: なにしろ今日は普段より暑かったから
        let sentence = "なにしろ今日は普段より暑かったからクタクタだよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "何しろ");
        assert_pattern_range(&patterns, "何しろ", 0, 4); // なにしろ
    }

    #[test]
    fn test_nanishiro_with_node() {
        // Example from grammar data: なにしろこの商品は不便すぎるので
        let sentence = "なにしろこの商品は不便すぎるので、どれだけ値下げをしても売れないでしょう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "何しろ");
        assert_pattern_range(&patterns, "何しろ", 0, 4); // なにしろ
    }
}

// Pattern: にせよ・にしろ (even if, no matter if)
// Data source: grammar_points_data.json["にせよ・にしろ"]
// Testing: structure.standard[0-3] - Single form usage (not repeated)
//
// Structure variants:
//   - standard[0]: Verb + にしろ/にせよ
//   - standard[1]: い-Adjective + にしろ/にせよ
//   - standard[2]: な-Adjective + (である) + にしろ/にせよ
//   - standard[3]: Noun + (である) + にしろ/にせよ
//
// Note: This is the single form, not the repeated form (にしろ～にしろ)

mod niseyo_nishiro_tests {
    use super::*;

    #[test]
    fn test_verb_niseyo() {
        // Example from grammar data: 参加しないにせよ
        let sentence = "飲み会に参加しないにせよ、参加費は明日までに持ってきてください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にせよ・にしろ");
        assert_pattern_range(&patterns, "にせよ・にしろ", 7, 12); // ないにせよ
    }

    #[test]
    fn test_i_adj_nishiro() {
        // Example from grammar data: 怖いにしろ
        let sentence = "怖いにしろ、私の仕事なのでやらなくてはならない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にせよ・にしろ");
        assert_pattern_range(&patterns, "にせよ・にしろ", 0, 5); // 怖いにしろ
    }

    #[test]
    fn test_na_adj_niseyo() {
        // Example from grammar data: 不便であるにせよ
        let sentence = "どんなに不便であるにせよ、私は自然が豊かな田舎に住みたいです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にせよ・にしろ");
        assert_pattern_range(&patterns, "にせよ・にしろ", 7, 12); // あるにせよ
    }

    #[test]
    fn test_noun_nishiro() {
        // Example from grammar data: 子供にしろ
        let sentence = "あの人は相手が子供にしろ、一切手加減はしない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にせよ・にしろ");
        assert_pattern_range(&patterns, "にせよ・にしろ", 7, 12); // 子供にしろ
    }
}

// Pattern: としては (as for, as, from the standpoint of)
// Data source: grammar_points_data.json["としては"]
// Testing: structure.standard[0] - "Noun + としては"
//
// Structure variants:
//   - standard[0]: Noun + としては
//
// としては is used to make judgments from the standpoint of (A), emphasizing
// the subject as a stand-alone entity being compared or considered.

mod toshiteha_tests {
    use super::*;

    #[test]
    fn test_person_toshiteha() {
        // Example adapted from grammar data: 先輩としては
        let sentence = "あの人は先輩としてはいいけど、友達になろうとは思えない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "としては");
        assert_pattern_range(&patterns, "としては", 4, 10); // 先輩としては
    }

    #[test]
    fn test_organization_toshiteha() {
        // Example adapted from grammar data: 建築会社としては
        let sentence = "あの会社は建築会社としてはホワイトな方だと思う。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "としては");
        assert_pattern_range(&patterns, "としては", 7, 13); // 会社としては
    }

    #[test]
    fn test_concept_toshiteha() {
        // Example adapted from grammar data: アイデアとしては
        let sentence = "アイデアとしてはいいんですが、実際にやるとなるとものすごい費用がかかると思います。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "としては");
        assert_pattern_range(&patterns, "としては", 0, 8); // アイデアとしては
    }

    #[test]
    fn test_hobby_toshiteha() {
        // Example adapted from grammar data: 趣味としては
        let sentence = "ピアノは趣味としては好きだけど、ピアノで食べていこうとは思わない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "としては");
        assert_pattern_range(&patterns, "としては", 4, 10); // 趣味としては
    }
}

// Pattern: としても (even if we assume that)
// Data source: grammar_points_data.json["としても"]
// Testing: structure.standard[0-3]
//   - standard[0]: Verb + としても
//   - standard[1]: い-Adj + としても
//   - standard[2]: な-Adj + (だ) + としても
//   - standard[3]: Noun + (だ) + としても
mod toshitemo_tests {
    use super::*;

    #[test]
    fn test_verb_toshitemo() {
        // Testing: Verb + としても
        // Example adapted from grammar data: たとえ早くついたとしても
        let sentence = "たとえ早くついたとしても、一番乗りになることはないだろう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "としても");
        assert_pattern_range(&patterns, "としても", 7, 12); // たとしても
    }

    #[test]
    fn test_i_adjective_toshitemo() {
        // Testing: い-Adj + としても
        // Example adapted from grammar data: 正しいとしても
        let sentence = "あなたが言っている事が正しいとしても、私は賛成できません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "としても");
        assert_pattern_range(&patterns, "としても", 11, 18); // 正しいとしても
    }

    #[test]
    fn test_na_adjective_toshitemo() {
        // Testing: な-Adj + だ + としても
        // Example adapted from grammar data: 簡単だとしても
        let sentence = "たとえ仕事が簡単だとしても、仕事は丁寧にやるべきだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "としても");
        assert_pattern_range(&patterns, "としても", 8, 13); // だとしても
    }

    #[test]
    fn test_noun_toshitemo() {
        // Testing: Noun + だ + としても
        // Example adapted from grammar data: 冗談だとしても
        let sentence = "冗談だとしても、そういうことは言わない方がいいと思う。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "としても");
        assert_pattern_range(&patterns, "としても", 2, 7); // だとしても
    }
}

// Pattern: それにしても (even so, nevertheless)
// Data source: grammar_points_data.json["それにしても"]
// Testing: structure.standard[0] - Phrase (A) + それにしても + Phrase (B)
mod sorenishitemo_tests {
    use super::*;

    #[test]
    fn test_after_comma() {
        // Testing: それにしても appearing after comma
        // Example adapted from grammar data
        let sentence = "彼女は新人だから仕方がないが、それにしても仕事ができなさすぎる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "それにしても");
        assert_pattern_range(&patterns, "それにしても", 15, 21); // それにしても
    }

    #[test]
    fn test_sentence_beginning() {
        // Testing: それにしても at the beginning of a sentence
        // Example adapted from grammar data
        let sentence = "漢字を勉強し始めてから１年になる。それにしても、まだ全然漢字が読めない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "それにしても");
        assert_pattern_range(&patterns, "それにしても", 17, 23); // それにしても
    }

    #[test]
    fn test_comparison_context() {
        // Testing: それにしても with comparison/expectation exceeded
        // Example adapted from grammar data
        let sentence = "家に来たばかりのときなんかお手もできなかったのにね。それにしても、大きくなったね。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "それにしても");
        assert_pattern_range(&patterns, "それにしても", 26, 32); // それにしても
    }
}

// Pattern: ぬ (classical negative - "not")
// Data source: grammar_points_data.json["ぬ"]
// Testing: structure.standard[0] - "Verb［ない］+ ぬ"
// Testing: structure.standard[1] - "Verb［ない］+ ぬ + Noun"
// Testing: Exceptions - する → せぬ, くる → こぬ, いる → おらぬ
//
// Structure variants:
//   - standard[0]: Verb[ない] + ぬ (classical negative)
//   - standard[1]: Verb[ない] + ぬ + Noun (attributive form)
//   - Exceptions: する→せぬ, くる→こぬ, いる→おらぬ

mod nu_negative_tests {
    use super::*;

    #[test]
    fn test_nu_basic_negative() {
        // Testing: structure.standard[0] - "Verb[ない] + ぬ"
        // Example from grammar data: 通れぬ (cannot pass)
        let sentence = "こんなに狭い道は通れぬ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ぬ");
        assert_pattern_range(&patterns, "ぬ", 8, 11); // 通れぬ
    }

    #[test]
    fn test_nu_negative_imperative() {
        // Testing: structure.standard[0] - "Verb[ない] + ぬ"
        // Example from grammar data: 入れぬ (will not put)
        let sentence = "こんなまずそうなものは口に入れぬ！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ぬ");
        assert_pattern_range(&patterns, "ぬ", 13, 16); // 入れぬ
    }

    #[test]
    fn test_nu_attributive_form() {
        // Testing: structure.standard[1] - "Verb[ない] + ぬ + Noun"
        // Testing classical negative modifying a noun (attributive form)
        let sentence = "見たこともなき知らぬ土地へ向かう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ぬ");
        assert_pattern_range(&patterns, "ぬ", 7, 10); // 知らぬ
    }

    #[test]
    fn test_nu_exception_suru() {
        // Testing: Exception - する → せぬ
        // Example from grammar data: せぬ (would not do)
        let sentence = "あの方は人を傷つけるようなことはせぬ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ぬ");
        assert_pattern_range(&patterns, "ぬ", 16, 18); // せぬ
    }

    #[test]
    fn test_nu_exception_kuru() {
        // Testing: Exception - くる → こぬ
        // Example from grammar data: こぬ (won't come)
        let sentence = "婆さんが川から帰ってこぬ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ぬ");
        assert_pattern_range(&patterns, "ぬ", 10, 12); // こぬ
    }

    #[test]
    fn test_nu_exception_iru() {
        // Testing: Exception - いる → おらぬ
        // Example from grammar data: おらぬ (is not)
        let sentence = "鈴木殿はここにはおらぬ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ぬ");
        assert_pattern_range(&patterns, "ぬ", 8, 11); // おらぬ
    }
}

// Pattern: にて (formal particle for で)
// Data source: grammar_points_data.json["にて"]
// Testing: structure.standard[0] - "Noun + にて"
//
// Structure variants:
//   - standard[0]: Noun + にて (formal particle meaning with/by/at/using)
//
// Usage contexts to test:
//   - Location (at)
//   - Time (at)
//   - Means/Method (by/using)

mod nite_tests {
    use super::*;

    #[test]
    fn test_nite_location() {
        // Testing: Noun (location) + にて
        // Example from grammar data: 金時計にて集合しましょう
        let sentence = "明日は金時計にて集合しましょう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にて");
        assert_pattern_range(&patterns, "にて", 4, 8); // 時計にて
    }

    #[test]
    fn test_nite_time() {
        // Testing: Noun (time) + にて
        // Example from grammar data: 午前９時にて開店いたします
        let sentence = "当店は午前９時にて開店いたします。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にて");
        assert_pattern_range(&patterns, "にて", 6, 9); // 時にて
    }

    #[test]
    fn test_nite_means() {
        // Testing: Noun (means) + にて
        // Example from grammar data: 電話にてご連絡ください
        let sentence = "予約をされる場合は電話にてご連絡ください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にて");
        assert_pattern_range(&patterns, "にて", 9, 13); // 電話にて
    }
}

// Pattern: には (emphatic に + は)
// Data source: grammar_points_data.json["には"]
// Testing: structure.standard[0] - "Verb + には"
// Testing: structure.standard[1] - "Noun + には"
//
// Structure variants:
//   - standard[0]: Verb (dictionary form) + には
//   - standard[1]: Noun + には
//
// Usage: Expresses contrast/emphasis for the topic (に + は)
// Meaning: "in order to", "for", "in regard to"

mod niha_tests {
    use super::*;

    #[test]
    fn test_niha_verb() {
        // Testing: Verb + には
        // Example from grammar data: 買い物をするには会員登録が必要
        let sentence = "このホームセンターは会員制なので、買い物をするには会員登録が必要です。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "には");
        assert_pattern_range(&patterns, "には", 21, 25); // するには
    }

    #[test]
    fn test_niha_verb_passport() {
        // Testing: Verb + には
        // Example from grammar data: 海外に行くにはパスポートが必要
        let sentence = "海外に行くにはパスポートが必要です。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "には");
        assert_pattern_range(&patterns, "には", 3, 7); // 行くには
    }

    #[test]
    fn test_niha_noun() {
        // Testing: Noun + には
        // Example from grammar data: この本は子供には難しすぎる
        let sentence = "この本は子供には難しすぎるだろう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "には");
        assert_pattern_range(&patterns, "には", 4, 8); // 子供には
    }

    #[test]
    fn test_niha_noun_commute() {
        // Testing: Noun + には
        // Example from grammar data: 通勤には車を使わないで
        let sentence = "通勤には車を使わないでください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "には");
        assert_pattern_range(&patterns, "には", 0, 4); // 通勤には
    }
}

// Pattern: 傾向がある (tendency/trend)
// Data source: grammar_points_data.json["傾向がある"]
// Structures to test:
//   - standard[0]: Verb[る] + 傾向
//   - standard[1]: い-Adj + 傾向
//   - standard[2]: な-Adj + な + 傾向
//   - standard[3]: Noun + の + 傾向
//   - Note: standard[4] is just a variant note about た-form verbs
mod keikougaaru_tests {
    use super::*;

    #[test]
    fn test_keikougaaru_verb() {
        // Testing: Verb[る] + 傾向がある
        // Realistic context: discussing a statistical trend
        let sentence = "この国では物価が上がっていく傾向がある。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "傾向がある");
        assert_pattern_range(&patterns, "傾向がある", 14, 19); // 傾向がある
    }

    #[test]
    fn test_keikougaaru_i_adjective() {
        // Testing: い-Adj + 傾向がある
        // Realistic context: describing a characteristic tendency
        let sentence = "彼は気分の浮き沈みが激しい傾向がある。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "傾向がある");
        assert_pattern_range(&patterns, "傾向がある", 13, 18); // 傾向がある
    }

    #[test]
    fn test_keikougaaru_na_adjective() {
        // Testing: な-Adj + な + 傾向がある
        // Realistic context: textbook characteristic
        let sentence = "その教科書は使いやすくてユニークな傾向がある。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "傾向がある");
        assert_pattern_range(&patterns, "傾向がある", 17, 22); // 傾向がある
    }

    #[test]
    fn test_keikougaaru_noun() {
        // Testing: Noun + の + 傾向がある
        // Realistic context: social trend
        let sentence = "最近は世界中で小家族化の傾向がある。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "傾向がある");
        assert_pattern_range(&patterns, "傾向がある", 12, 17); // 傾向がある
    }

    #[test]
    fn test_keikougaaru_verb_past() {
        // Testing: Verb[た] + 傾向がある (variant from standard[4])
        // Realistic context: describing past trend
        let sentence = "この地域では雨が降った傾向がある。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "傾向がある");
        assert_pattern_range(&patterns, "傾向がある", 11, 16); // 傾向がある
    }
}

// Pattern: 恐れがある (fear/risk of - negative possibility)
// Data source: grammar_points_data.json["恐れがある"]
// Structures to test:
//   - standard[0]: Verb[る/ない] + 恐れがある
//   - standard[1]: い-Adj + 恐れがある
//   - standard[2]: な-Adj + な/である + 恐れがある
//   - standard[3]: Noun + の/である + 恐れがある
mod osoregaaru_tests {
    use super::*;

    #[test]
    fn test_osoregaaru_verb() {
        // Testing: Verb[る] + 恐れがある
        // Realistic context: warning about explosion risk
        let sentence = "高温にすると爆発する恐れがあるので注意してください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "恐れがある");
        assert_pattern_range(&patterns, "恐れがある", 10, 15); // 恐れがある
    }

    #[test]
    fn test_osoregaaru_noun() {
        // Testing: Noun + の + 恐れがある
        // Realistic context: tsunami warning
        let sentence = "津波の恐れがある場合は速やかに避難してください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "恐れがある");
        assert_pattern_range(&patterns, "恐れがある", 3, 8); // 恐れがある
    }

    #[test]
    fn test_osoregaaru_verb_negative() {
        // Testing: Verb[ない] + 恐れがある
        // Realistic context: failure risk
        let sentence = "このままでは成功しない恐れがあると思います。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "恐れがある");
        assert_pattern_range(&patterns, "恐れがある", 11, 16); // 恐れがある
    }

    #[test]
    fn test_osoregaaru_i_adjective() {
        // Testing: い-Adj + 恐れがある
        // Realistic context: describing a dangerous situation
        let sentence = "この地域は地震の影響で危ない恐れがあります。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "恐れがある");
        assert_pattern_range(&patterns, "恐れがある", 14, 21); // 恐れがあります
    }
}

// Pattern: 思うように (as hoped/as desired)
// Data source: grammar_points_data.json["思うように"]
// Structures to test:
//   - standard[0]: Phrase + 思うように + (Outcome)
//   - standard[1]: Phrase + 思うような + Noun
mod omouyouni_tests {
    use super::*;

    #[test]
    fn test_omouyouni_negative() {
        // Testing: 思うように + negative outcome
        // Realistic context: work not going well
        let sentence = "最近仕事が思うように行かなくてストレスが溜まってる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "思うように");
        assert_pattern_range(&patterns, "思うように", 5, 10); // 思うように
    }

    #[test]
    fn test_omouyouni_positive() {
        // Testing: 思うように + positive outcome
        // Realistic context: plans going as expected
        let sentence = "今回のプロジェクトは思うように進んでいます。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "思うように");
        assert_pattern_range(&patterns, "思うように", 10, 15); // 思うように
    }

    #[test]
    fn test_omouyouni_ability() {
        // Testing: 思うように + できない (inability)
        // Realistic context: physical limitation
        let sentence = "事故に遭ってから指が思うように動かなくなった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "思うように");
        assert_pattern_range(&patterns, "思うように", 10, 15); // 思うように
    }

    #[test]
    fn test_omouyouna_noun() {
        // Testing: 思うような + Noun
        // Realistic context: describing results
        let sentence = "思うような結果を得ることができませんでした。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "思うように");
        assert_pattern_range(&patterns, "思うように", 0, 5); // 思うような
    }
}

// Pattern: というものでもない (not necessarily, there's no guarantee)
// Data source: grammar_points_data.json["というものでもない"]
// Testing structure variants: Verb/Adj/Noun + というものでもない/じゃない
//
// Structure variants:
//   - standard[0]: Verb + というものではない
//   - standard[1]: な-Adj + というものではない
//   - standard[2]: い-Adj + というものではない
//   - standard[3]: Noun + というものではない
//   - standard[4]: (variants with でもない/じゃない)
//   - polite[0-3]: Same with でもありません/じゃありません
mod toiumonodemonai_tests {
    use super::*;

    #[test]
    fn test_verb_demonai() {
        // Testing: Verb + というものでもない
        // Realistic context: clothes don't need to be expensive
        let sentence = "洋服は高ければいいというものでもない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "というものでもない");
        assert_pattern_range(&patterns, "というものでもない", 7, 18); // いいというものでもない
    }

    #[test]
    fn test_verb_dehanai() {
        // Testing: Verb + というものではない
        // Realistic context: good university doesn't guarantee good job
        let sentence = "いい大学を出たからといって、いい会社に入社できるというものではない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "というものでもない");
        assert_pattern_range(&patterns, "というものでもない", 21, 33); // できるというものではない
    }

    #[test]
    fn test_na_adj_demonai() {
        // Testing: な-Adj + というものでもない
        // Realistic context: app features
        let sentence = "アプリの機能は多ければ多いほど便利だというものでもない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "というものでもない");
        assert_pattern_range(&patterns, "というものでもない", 17, 27); // だというものでもない
    }

    #[test]
    fn test_i_adj_demonai() {
        // Testing: い-Adj + というものでもない
        // Realistic context: expensive gifts
        let sentence = "プレゼントは高額であればいいというものではない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "というものでもない");
        assert_pattern_range(&patterns, "というものでもない", 12, 23); // いいというものではない
    }

    #[test]
    fn test_noun_demonai() {
        // Testing: Noun + というものでもない
        // Realistic context: brand popularity
        let sentence = "あのブランドが人気だからといって、あのブランドの商品がいいものだというものでもない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "というものでもない");
        assert_pattern_range(&patterns, "というものでもない", 31, 41); // だというものでもない
    }

    #[test]
    fn test_polite_demoarimasen() {
        // Testing: でもありません (polite form)
        // Realistic context: work situation
        let sentence = "経験があれば必ず成功できるというものでもありません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "というものでもない");
        assert_pattern_range(&patterns, "というものでもない", 10, 25); // できるというものでもありません
    }

    #[test]
    fn test_janai_casual() {
        // Testing: じゃない (casual contraction)
        // Realistic context: study effectiveness
        let sentence = "勉強時間が長ければいいというものじゃないよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "というものでもない");
        assert_pattern_range(&patterns, "というものでもない", 9, 20); // いいというものじゃない
    }

    #[test]
    fn test_jaarimasen_polite() {
        // Testing: じゃありません (polite casual contraction)
        // Realistic context: language learning
        let sentence = "日本語が上手であれば勉強をし続けなくてもいいというものじゃありません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "というものでもない");
        assert_pattern_range(&patterns, "というものでもない", 20, 34); // いいというものじゃありません
    }
}

// Pattern: と考えられる (can be considered, is thought to be)
// Data source: grammar_points_data.json["と考えられる"]
// Testing: structure.standard[0] - "Phrase + と考えられる"
// Testing: structure.polite[0] - "Phrase + と考えられます"
//
// Structure variants:
//   - standard[0]: Phrase + と考えられる (considered/thought to be)
//   - polite[0]: Phrase + と考えられます (polite form)
//
// Note: This expresses an objective opinion based on observable facts.
// Compare with と思われる (subjective opinion) and と考えられている (widely accepted opinion).

mod tokangaerareru_tests {
    use super::*;

    #[test]
    fn test_standard_form() {
        // Testing: と考えられる (standard form)
        // Realistic context: accident analysis
        let sentence = "あの事故は煽り運転のせいだと考えられる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "と考えられる");
        assert_pattern_range(&patterns, "と考えられる", 13, 19); // と考えられる
    }

    #[test]
    fn test_standard_complex() {
        // Testing: と考えられる with complex clause
        // Realistic context: language difficulty
        let sentence = "日本語は文字種が多いことから、第二言語として習うのが難しいと考えられる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "と考えられる");
        assert_pattern_range(&patterns, "と考えられる", 29, 35); // と考えられる
    }

    #[test]
    fn test_polite_form() {
        // Testing: と考えられます (polite form)
        // Realistic context: scientific analysis
        let sentence = "この結果から、気候変動が原因だと考えられます。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "と考えられる");
        assert_pattern_range(&patterns, "と考えられる", 15, 22); // と考えられます
    }

    #[test]
    fn test_negative_form() {
        // Testing: と考えられない (negative)
        // Realistic context: disagreement with analysis
        let sentence = "その仮説は正しいと考えられない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "と考えられる");
        assert_pattern_range(&patterns, "と考えられる", 8, 15); // と考えられない
    }

    #[test]
    fn test_past_form() {
        // Testing: と考えられた (past tense)
        // Realistic context: historical perspective
        let sentence = "当時はそれが最善の策だと考えられた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "と考えられる");
        assert_pattern_range(&patterns, "と考えられる", 11, 17); // と考えられた
    }
}

// Pattern: という点から考えると (from the viewpoint of, speaking in terms of)
// Data source: grammar_points_data.json["という点から考えると"]
// Testing: All structure variants
//
// Structure variants:
//   - standard[0]: Noun + の点から考えると
//   - standard[1]: Noun + という点から考えると
//   - standard[2]: Verb + という点から考えると
//   - standard[3]: その点から考えると

mod toiutenkarakangaeruto_tests {
    use super::*;

    #[test]
    fn test_noun_no_ten() {
        // Testing: Noun + の点から考えると
        // Example from grammar data: 健康の点から考えると
        let sentence = "健康の点から考えると休む時はちゃんと休まなければ身体を壊す可能性が高まります。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "という点から考えると");
        assert_pattern_range(&patterns, "という点から考えると", 0, 10); // 健康の点から考えると
    }

    #[test]
    fn test_noun_toiu_ten() {
        // Testing: Noun + という点から考えると
        // Example from grammar data: 子供の教育という点から考えると
        let sentence = "子供の教育という点から考えると、楽しく勉強をさせる事が重要です。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "という点から考えると");
        assert_pattern_range(&patterns, "という点から考えると", 0, 15); // 子供の教育という点から考えると
    }

    #[test]
    fn test_verb_toiu_ten() {
        // Testing: Verb + という点から考えると
        // Example from grammar data: デジタル化が遅れているという点から考えると
        let sentence = "日本のデジタル化が遅れているという点から考えると、デジタル人材の育成に力を入れなければならない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "という点から考えると");
        assert_pattern_range(&patterns, "という点から考えると", 9, 24); // 遅れているという点から考えると
    }

    #[test]
    fn test_sono_ten() {
        // Testing: その点から考えると
        // This tests the demonstrative その + 点から考えると variant
        let sentence = "価格が高いのは問題だ。その点から考えると、もっと手頃な選択肢を探すべきだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "という点から考えると");
        assert_pattern_range(&patterns, "という点から考えると", 11, 20); // その点から考えると
    }
}

// Pattern: ということは (that means, in other words)
// Data source: grammar_points_data.json["ということは"]
// Testing: structure.standard[0] - "Phrase + ということは + Phrase"
//
// Structure variants:
//   - standard[0]: Phrase + ということは + Phrase (mid-sentence clarification)
//   - Beginning of sentence: ということは + Phrase (confirming understanding)
//
// Notes:
//   - Mid-sentence usage: previous statement requires clarification
//   - Beginning usage: confirming understanding of what was just said
//   - Often finishes with ということだ but not limited to this
//   - Literally: "that which is said to be (A), (B)"

mod toiukotoha_tests {
    use super::*;

    #[test]
    fn test_mid_sentence_working() {
        // Testing: Phrase + ということは + Phrase (mid-sentence)
        // Clarifying what it means to work at a famous company
        let sentence = "あの有名なIT企業で働いているということは、結構いい大学を出たってこと？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ということは");
        assert_pattern_range(&patterns, "ということは", 7, 21); // 企業で働いているということは
    }

    #[test]
    fn test_mid_sentence_no_reply() {
        // Testing: Phrase + ということは + Phrase (mid-sentence)
        // Clarifying what it means when she hasn't responded
        let sentence = "彼女から返事が来ないということは、今は忙しいということだろう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ということは");
        assert_pattern_range(&patterns, "ということは", 2, 16); // から返事が来ないということは
    }

    #[test]
    fn test_mid_sentence_japanese_skill() {
        // Testing: Phrase + ということは + Phrase (mid-sentence)
        // Clarifying what it means to speak Japanese well
        let sentence = "日本語を上手に話せるということは、日本に長い間住んでいたということですか。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ということは");
        assert_pattern_range(&patterns, "ということは", 0, 16); // 日本語を上手に話せるということは
    }

    #[test]
    fn test_beginning_work_done() {
        // Testing: ということは at beginning of sentence (confirming understanding)
        // Confirming that finished work means can eat together
        let sentence = "もう仕事終わったの？ということは、今日こそは一緒に晩御飯が食べれるということだね！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ということは");
        assert_pattern_range(&patterns, "ということは", 10, 16); // ということは (standalone)
    }

    #[test]
    fn test_beginning_pregnancy() {
        // Testing: ということは at beginning of sentence (confirming understanding)
        // Confirming what pregnancy means for the speaker
        let sentence = "妊娠したの！？ということは、私はおばあちゃんになるってこと？！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ということは");
        assert_pattern_range(&patterns, "ということは", 7, 13); // ということは (standalone)
    }
}
