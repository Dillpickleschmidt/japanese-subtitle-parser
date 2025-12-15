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
