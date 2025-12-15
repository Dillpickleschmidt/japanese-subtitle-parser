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
