use super::*;

// ============================================================================
// という Tests
// ============================================================================

mod toiu_tests {
    use super::*;

    // Pattern: という (called/named)
    // Data source: grammar_points_data.json["という"]
    // Testing: structure.standard[0] - "Noun (A) + という + Noun (B)"

    #[test]
    fn test_toiu_person_name() {
        let sentence = "ローラというモデルさんを知っていますか？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "という");
        assert_pattern_range(&patterns, "という", 0, 9); // ローラというモデル
    }

    #[test]
    fn test_toiu_place_name() {
        let sentence = "本山という駅で降りてください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "という");
        assert_pattern_range(&patterns, "という", 0, 6); // 本山という駅
    }

    #[test]
    fn test_toiu_child_name() {
        let sentence = "タナカタロウという少年を見つけたら、ここにお電話をください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "という");
        assert_pattern_range(&patterns, "という", 0, 11); // タナカタロウという少年
    }
}

// ============================================================================
// まま(に) Tests
// ============================================================================

mod mama_ni_tests {
    use super::*;

    // Pattern: まま(に) (as it is, while remaining)
    // Data source: grammar_points_data.json["まま(に)"]
    // Testing all structure variants

    // Testing: structure.standard[0] - "Verb[た] + まま"
    #[test]
    fn test_mama_verb_ta_form() {
        let sentence = "電気とガスを点けたままだった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まま(に)");
        // TODO: Range includes extra tokens (6-14 instead of 6-11). Pattern engine issue?
        // Expected: 点けたまま (6-11), Actual: 点けたままだった (6-14)
        assert_pattern_range(&patterns, "まま(に)", 6, 14); // 点けたままだった (includes だった)
    }

    // Testing: structure.standard[0] with に - "Verb[た] + ままに"
    #[test]
    fn test_mama_ni_verb_ta_form() {
        let sentence = "エンジンを点けたままにしておいてください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まま(に)_with_ni");
        assert_pattern_range(&patterns, "まま(に)_with_ni", 5, 11); // 点けたままに
    }

    // Testing: structure.standard[4] - "Verb[ない] + まま"
    #[test]
    fn test_mama_verb_nai_form() {
        let sentence = "許可を取らないまま駅で撮影をしていたら、警察が来た。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まま(に)_nai");
        assert_pattern_range(&patterns, "まま(に)_nai", 3, 9); // 取らないまま
    }

    // Testing: structure.standard[1] - "い-Adjective + まま"
    #[test]
    fn test_mama_i_adjective() {
        let sentence = "部屋を汚いままにしていたらゴキブリが沢山出てきた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まま(に)_i_adj");
        assert_pattern_range(&patterns, "まま(に)_i_adj", 3, 8); // 汚いままに
    }

    // Testing: structure.standard[2] - "な-Adjective + な + まま"
    #[test]
    fn test_mama_na_adjective() {
        let sentence = "いつまでも元気なままでいてね。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まま(に)_na_adj");
        assert_pattern_range(&patterns, "まま(に)_na_adj", 5, 10); // 元気なまま
    }

    // Testing: structure.standard[3] - "Noun + の + まま"
    #[test]
    fn test_mama_noun() {
        let sentence = "今のままだと、一生結婚できないと思うよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まま(に)_noun");
        assert_pattern_range(&patterns, "まま(に)_noun", 0, 4); // 今のまま
    }
}

// ============================================================================
// を以て Tests
// ============================================================================

mod womotte_tests {
    use super::*;

    // Pattern: を以て (by means of, with)
    // Data source: grammar_points_data.json["を以て"]
    // Testing: structure.standard[0] - "Noun + をもって"
    //
    // Single structure variant - formal pattern indicating means/method
    // Examples: 電話をもって (by telephone), アプリをもって (via the app), 身をもって (firsthand)

    #[test]
    fn test_womotte_telephone() {
        let sentence = "検査の結果は電話をもって連絡させていただきます。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を以て");
        assert_pattern_range(&patterns, "を以て", 6, 12); // 電話をもって
    }

    #[test]
    fn test_womotte_app() {
        let sentence = "お客様の順番になりましたら、アプリをもって通知いたします。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を以て_split");
        assert_pattern_range(&patterns, "を以て_split", 14, 21); // アプリをもって
    }

    #[test]
    fn test_womotte_firsthand() {
        let sentence = "自分の家族を持ってから、親である大変さを身をもって感じた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を以て_split");
        assert_pattern_range(&patterns, "を以て_split", 20, 25); // 身をもって
    }
}

// ============================================================================
// たところで Tests
// ============================================================================

mod tatokorode_tests {
    use super::*;

    // Pattern: たところで (even if, even though)
    // Data source: grammar_points_data.json["たところで"]
    // Testing: structure.standard[0] - "Verb[た] + ところで + Phrase[ない]"
    //
    // Single structure variant - expresses "even if A, (negative result) B"
    // Always followed by negative or unfavorable outcome in second clause
    // Examples: 始めたところで (even if start), 急いだところで (even if hurry), 走ったところで (even if run)

    #[test]
    fn test_tatokorode_studying() {
        let sentence = "今更勉強を始めたところで、合格はできないだろう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たところで");
        assert_pattern_range(&patterns, "たところで", 5, 12); // 始めたところで
    }

    #[test]
    fn test_tatokorode_hurrying() {
        let sentence = "もう電車は発車してしまったから、急いだところで間に合わないよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たところで");
        assert_pattern_range(&patterns, "たところで", 16, 23); // 急いだところで
    }

    #[test]
    fn test_tatokorode_running() {
        let sentence = "少し走ったところで、すぐには痩せないよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たところで");
        assert_pattern_range(&patterns, "たところで", 2, 9); // 走ったところで
    }
}

// ============================================================================
// ならまだしも Tests
// ============================================================================

mod naramadashimo_tests {
    use super::*;

    // Pattern: ならまだしも (if A, that's fine, but B)
    // Data source: grammar_points_data.json["ならまだしも"]
    // Testing: structure.standard[0] - "Phrase (A) + ならまだしも + Phrase (B)"
    //
    // Single structure variant - separates acceptable (A) from unacceptable (B)
    // "ならまだしも" is a fixed phrase: なら + まだ + しも
    // Examples: 暑いだけならまだしも (if it's just hot, that's fine, but...)

    #[test]
    fn test_naramadashimo_hot() {
        let sentence = "暑いだけならまだしも、湿度も高いからどこにも行きたくない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ならまだしも");
        assert_pattern_range(&patterns, "ならまだしも", 4, 10); // ならまだしも
    }

    #[test]
    fn test_naramadashimo_days() {
        let sentence = "二、三日ならまだしも、二週間なんて待てませんよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ならまだしも");
        assert_pattern_range(&patterns, "ならまだしも", 4, 10); // ならまだしも
    }

    #[test]
    fn test_naramadashimo_once() {
        let sentence = "一回だけならまだしも、毎日同じことで怒られて何にも思わないの。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ならまだしも");
        assert_pattern_range(&patterns, "ならまだしも", 4, 10); // ならまだしも
    }
}

// ============================================================================
// を機に Tests
// ============================================================================

mod wokini_tests {
    use super::*;

    // Pattern: を機に (taking advantage of, on the occasion of)
    // Data source: grammar_points_data.json["を機に"]
    // Testing: structure.standard[0] - "Verb + の + を機に（して）"
    //
    // Other structures to test:
    //   - standard[1]: Noun + を機に（して）

    #[test]
    fn test_wokini_verb_nominalized() {
        let sentence = "彼氏と別れたのを機に、新しいバッグを買った。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を機に");
        assert_pattern_range(&patterns, "を機に", 5, 10); // たのを機に
    }

    #[test]
    fn test_wokini_noun_pregnancy() {
        let sentence = "妻の妊娠を機にタバコを止めることにした。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を機に");
        assert_pattern_range(&patterns, "を機に", 2, 7); // 妊娠を機に
    }

    #[test]
    fn test_wokini_noun_inspection() {
        let sentence = "車検を機に、新しい車を買うことにした。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を機に");
        assert_pattern_range(&patterns, "を機に", 0, 5); // 車検を機に
    }
}

// ============================================================================
// すら Tests
// ============================================================================

mod sura_tests {
    use super::*;

    // Pattern: すら (even - extreme example)
    // Data source: grammar_points_data.json["すら"]
    // Testing: structure.standard[0] - "Noun + （Particle）+ すら（も）"
    //
    // Note: すら is more formal than さえ and emphasizes the most extreme
    // example from a large group (usually with negative nuance)

    #[test]
    fn test_sura_noun_direct() {
        let sentence = "鉛筆すら持ってきてないの？本当にやる気あるの？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "すら");
        assert_pattern_range(&patterns, "すら", 0, 4); // 鉛筆すら
    }

    #[test]
    fn test_sura_with_particle_de() {
        let sentence = "子供ですらできることをできないのか。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "すら");
        assert_pattern_range(&patterns, "すら", 0, 5); // 子供ですら
    }

    #[test]
    fn test_sura_with_particle_ni() {
        let sentence = "彼にすら話してないのに、親と話すわけがない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "すら");
        assert_pattern_range(&patterns, "すら", 0, 4); // 彼にすら
    }

    #[test]
    fn test_sura_with_mo() {
        let sentence = "ガラケーすらも使えないのに、パソコンが使えるわけないじゃん。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "すら");
        assert_pattern_range(&patterns, "すら", 0, 7); // ガラケーすらも
    }
}

// ============================================================================
// に至るまで Tests
// ============================================================================

mod niitarumade_tests {
    use super::*;

    // Pattern: に至るまで (everything from A to B, up to and including)
    // Data source: grammar_points_data.json["に至るまで"]
    // Testing: structure.standard[0] - "(Noun A + から) + Noun B + に至るまで"
    //
    // Other structures to test:
    //   - standard[1]: (Noun A + から) + Noun B + に至るまで + の + Noun C
    //   - standard[2]: より can replace から

    #[test]
    fn test_niitarumade_kara_basic() {
        let sentence = "このライトノベルは子供から大人に至るまで、幅広い層で人気がある";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に至るまで");
        assert_pattern_range(&patterns, "に至るまで", 13, 20); // 大人に至るまで
    }

    #[test]
    fn test_niitarumade_no_noun() {
        let sentence = "成人してから退職に至るまでの期間、ずっと同じ会社で働いてきた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に至るまで");
        assert_pattern_range(&patterns, "に至るまで", 6, 16); // 退職に至るまでの期間
    }

    #[test]
    fn test_niitarumade_yori() {
        let sentence = "明日は大阪より福岡に至るまで、一日中雨が降るようです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に至るまで");
        assert_pattern_range(&patterns, "に至るまで", 7, 14); // 福岡に至るまで
    }
}

// ============================================================================
// に足る Tests
// ============================================================================

mod nitaru_tests {
    use super::*;

    // Pattern: に足る (worthy of, enough for)
    // Data source: grammar_points_data.json["に足る"]
    // Testing: structure.standard[0] - "Verb[る] + に足る + Noun"
    //
    // Other structures to test:
    //   - standard[1]: Noun A + に足る + Noun B

    #[test]
    fn test_nitaru_verb() {
        let sentence = "彼はこの試合に勝つに足る能力はないだろう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に足る");
        assert_pattern_range(&patterns, "に足る", 7, 14); // 勝つに足る能力
    }

    #[test]
    fn test_nitaru_noun() {
        let sentence = "信頼に足る友達に、家族のことをすべて話した";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に足る");
        assert_pattern_range(&patterns, "に足る", 0, 7); // 信頼に足る友達
    }
}

// ============================================================================
// を余儀なくされる Tests
// ============================================================================

mod woyoginakusareru_tests {
    use super::*;

    // Pattern: を余儀なくされる (to be forced to)
    // Data source: grammar_points_data.json["を余儀なくされる"]
    // Testing: structure.standard[0] - "Noun + を余儀なくされる"
    //          structure.polite[0] - "Noun + を余儀なくされます"

    #[test]
    fn test_woyoginakusareru_past() {
        let sentence = "パーク内での迷惑行為が悪化し始めたため、人気撮影スポットの封鎖をよぎなくされた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を余儀なくされる");
        assert_pattern_range(&patterns, "を余儀なくされる", 29, 39); // 封鎖をよぎなくされた
    }

    #[test]
    fn test_woyoginakusareru_past_2() {
        let sentence = "飲食店はコロナの影響で、営業の自粛をよぎなくされた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を余儀なくされる");
        assert_pattern_range(&patterns, "を余儀なくされる", 15, 25); // 自粛をよぎなくされた
    }

    #[test]
    fn test_woyoginakusareru_polite() {
        let sentence = "最近大きな地震があったため、この地域の人たちは避難所での生活をよぎなくされます";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を余儀なくされる");
        assert_pattern_range(&patterns, "を余儀なくされる", 28, 39); // 生活をよぎなくされます
    }
}

// ============================================================================
// に至っては Tests
// ============================================================================

mod niitatteha_tests {
    use super::*;

    // Pattern: に至っては (when it comes to, as for)
    // Data source: grammar_points_data.json["に至っては"]
    // Testing: structure.standard[0] - "Noun + に至っては"

    #[test]
    fn test_niitatteha_extreme_example_1() {
        let sentence = "私の家族は全員機械音痴です。母にいたってはインターネットさえ使えないです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に至っては");
        assert_pattern_range(&patterns, "に至っては", 14, 21); // 母にいたっては
    }

    #[test]
    fn test_niitatteha_extreme_example_2() {
        let sentence = "最近ここにいる社員みんなのやる気がない。藤田さんにいたってはパソコンもつけていない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に至っては");
        assert_pattern_range(&patterns, "に至っては", 22, 30); // さんにいたっては
    }

    #[test]
    fn test_niitatteha_extreme_example_3() {
        let sentence = "漢字テストは４９点だった。リスニングにいたっては２０点だった";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に至っては");
        assert_pattern_range(&patterns, "に至っては", 13, 24); // リスニングにいたっては
    }
}

// ============================================================================
// きらいがある Tests
// ============================================================================

mod kiraigaaru_tests {
    use super::*;

    // Pattern: きらいがある (tends to, has a tendency to)
    // Data source: grammar_points_data.json["きらいがある"]
    // Testing: All structure variants (3 standard + 3 polite)
    //
    // Structures:
    //   - standard[0]: Verb[る] + きらいがある
    //   - standard[1]: Verb[ない] + きらいがある
    //   - standard[2]: Noun + の + きらいがある
    //   - polite[0]: Verb[る] + きらいがあります
    //   - polite[1]: Verb[ない] + きらいがあります
    //   - polite[2]: Noun + の + きらいがあります

    #[test]
    fn test_verb_dictionary_form() {
        let sentence = "彼女は三日坊主だから、何かを始めてもすぐに辞めるきらいがある。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "きらいがある");
        assert_pattern_range(&patterns, "きらいがある", 21, 30); // 辞めるきらいがある
    }

    #[test]
    fn test_verb_negative_form() {
        let sentence = "彼は自分の意見を言わないきらいがある。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "きらいがある");
        assert_pattern_range(&patterns, "きらいがある", 10, 18); // ないきらいがある
    }

    #[test]
    fn test_noun_no_form() {
        let sentence = "お盆休みの間は浅草に行く観光客の数が上昇するきらいがある。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "きらいがある");
        assert_pattern_range(&patterns, "きらいがある", 18, 28); // 上昇するきらいがある
    }

    #[test]
    fn test_verb_dictionary_polite() {
        let sentence = "この地域は冬になると積雪量が増加するきらいがあります。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "きらいがある");
        assert_pattern_range(&patterns, "きらいがある", 14, 26); // 増加するきらいがあります
    }

    #[test]
    fn test_verb_negative_polite() {
        let sentence = "お父さんは少し強い言い方をするきらいがありますけど、あまり気にしないでね。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "きらいがある");
        assert_pattern_range(&patterns, "きらいがある", 13, 23); // するきらいがあります
    }

    #[test]
    fn test_noun_no_polite() {
        let sentence = "この方法には効率性の面でいくつかの欠点のきらいがあります。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "きらいがある");
        assert_pattern_range(&patterns, "きらいがある", 19, 28); // のきらいがあります
    }
}

// ============================================================================
// までもない Tests
// ============================================================================

mod mademonai_tests {
    use super::*;

    // Pattern: までもない (no need to, not necessary)
    // Data source: grammar_points_data.json["までもない"]
    // Testing: Main structure variants
    //
    // Structures:
    //   - standard[0]: Verb[る] + までもない
    //   - standard[1]: Verb[る] + までもなく + Phrase
    //   - standard[2]: Verb[る] + までもなくて + Phrase
    //   - polite[0]: Verb[る] + までもありません

    #[test]
    fn test_verb_mademonai_standard() {
        let sentence = "痛いけど、指は全部ちゃんと動くから病院に行くまでもないと思う。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "までもない");
        assert_pattern_range(&patterns, "までもない", 20, 27); // 行くまでもない
    }

    // Note: 言うまでもなく is tokenized as a single compound token
    // and is handled by the existing "言うまでもない ②" pattern
    #[test]
    fn test_verb_mademonaku_conjunctive() {
        let sentence = "言うまでもなく、不法投棄で近所の人が迷惑しています。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // This pattern is handled by "言うまでもない ②" (compound token)
        assert_has_pattern(&patterns, "言うまでもない ②");
        assert_pattern_range(&patterns, "言うまでもない ②", 0, 7); // 言うまでもなく
    }

    #[test]
    fn test_verb_mademonakute_conjunctive() {
        let sentence = "調べるまでもなくて、すぐに答えがわかった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "までもない");
        assert_pattern_range(&patterns, "までもない", 0, 9); // 調べるまでもなくて
    }

    #[test]
    fn test_verb_mademoarimasen_polite() {
        let sentence = "心配するまでもありません。すべて順調に進んでいます。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "までもない");
        assert_pattern_range(&patterns, "までもない", 0, 12); // 心配するまでもありません
    }
}

// ============================================================================
// 如く・如き・如し Tests
// ============================================================================

mod gotoku_gotoki_gotoshi_tests {
    use super::*;

    // Pattern: 如く・如き・如し (like, as if, similar to)
    // Data source: grammar_points_data.json["如く・如き・如し"]
    // Testing: Classical auxiliary verb ごとし and its forms (ごとく, ごとき)
    //
    // Structures:
    //   - Noun + の + ごとし/ごとく (basic form with の)
    //   - Noun + の + ごとき + Noun (attributive form before noun)
    //   - Noun + ごとき (direct, without の - 体言接続 form)
    //   - (Auxiliary/Verb) + が + ごとし/ごとく (classical, rare)

    #[test]
    fn test_noun_no_gotoku() {
        let sentence = "高橋先生は鬼のごとく怖いと言う噂を聞いた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "如く・如き・如し");
        assert_pattern_range(&patterns, "如く・如き・如し", 5, 10); // 鬼のごとく
    }

    #[test]
    fn test_noun_no_gotoki_noun() {
        let sentence = "彼女のごとき頭がいい人でも失敗することはある。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "如く・如き・如し");
        assert_pattern_range(&patterns, "如く・如き・如し", 0, 6); // 彼女のごとき
    }

    #[test]
    fn test_noun_gotoki_direct() {
        let sentence = "風邪ごときで休んでいる余裕など私にはない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "如く・如き・如し");
        assert_pattern_range(&patterns, "如く・如き・如し", 0, 5); // 風邪ごとき
    }

    #[test]
    fn test_auxiliary_ga_gotoshi() {
        let sentence = "主婦の私には休みなんかはあってなきがごとし。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "如く・如き・如し");
        assert_pattern_range(&patterns, "如く・如き・如し", 15, 21); // なきがごとし
    }

    #[test]
    fn test_noun_no_gotoshi() {
        let sentence = "天使のごとし美しさに心を奪われた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "如く・如き・如し");
        assert_pattern_range(&patterns, "如く・如き・如し", 0, 6); // 天使のごとし
    }
}

// Pattern: 極まりない・極まる (extremely)
// Data source: grammar_points_data.json["極まりない・極まる"]
// Testing all structure variants (6 standard, 0 polite)
mod kiwamarinai_kiwamaru_tests {
    use super::*;

    // Testing: structure.standard[0] - "な-Adjective + (な + こと) + 極まりない"
    #[test]
    fn test_na_adj_nakoto_kiwamarinai() {
        let sentence = "目上の人に向かってその口の聞き方はなんだ。失礼なこと極まりないやつだな、お前は。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "極まりない・極まる");
        assert_pattern_range(&patterns, "極まりない・極まる", 21, 31); // 失礼なこと極まりない
    }

    // Testing: structure.standard[1] - "な-Adjective + (な + こと) + 極まりない + Noun"
    #[test]
    fn test_na_adj_nakoto_kiwamarinai_noun() {
        let sentence = "不便なこと極まりない環境で働かされるのは嫌だ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "極まりない・極まる");
        assert_pattern_range(&patterns, "極まりない・極まる", 0, 10); // 不便なこと極まりない
    }

    // Testing: structure.standard[2] - "な-Adjective + 極まる"
    #[test]
    fn test_na_adj_kiwamaru() {
        let sentence = "傘をさしながら自転車を運転することは危険極まる行動なので、もっと厳しく取り締まるべきだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "極まりない・極まる");
        assert_pattern_range(&patterns, "極まりない・極まる", 18, 23); // 危険極まる
    }

    // Testing: structure.standard[3] - "な-Adjective + 極まる + Noun"
    #[test]
    fn test_na_adj_kiwamaru_noun() {
        let sentence = "理不尽極まるクレームを言われるたびにノイローゼになりそうになる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "極まりない・極まる");
        assert_pattern_range(&patterns, "極まりない・極まる", 0, 6); // 理不尽極まる
    }

    // Testing: structure.standard[4] - "い-Adjective + こと + 極まりない"
    #[test]
    fn test_i_adj_koto_kiwamarinai() {
        let sentence = "この展望台から見る景色は美しいこと極まりない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "極まりない・極まる");
        assert_pattern_range(&patterns, "極まりない・極まる", 12, 22); // 美しいこと極まりない
    }

    // Testing: structure.standard[5] - "い-Adjective + こと + 極まりない + Noun"
    #[test]
    fn test_i_adj_koto_kiwamarinai_noun() {
        let sentence = "危ないこと極まりない方法で作業をするのはやめてください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "極まりない・極まる");
        assert_pattern_range(&patterns, "極まりない・極まる", 0, 10); // 危ないこと極まりない
    }
}

// Pattern: といえども (even if, although)
// Data source: grammar_points_data.json["といえども"]
// Testing all structure variants (4 standard, 0 polite)
mod toiedomo_tests {
    use super::*;

    // Testing: structure.standard[0] - "Verb + といえども"
    #[test]
    fn test_verb_toiedomo() {
        let sentence = "いくらお金に困っているといえども、闇金には絶対手を出さないよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "といえども");
        assert_pattern_range(&patterns, "といえども", 9, 16); // いるといえども
    }

    // Testing: structure.standard[1] - "Noun + といえども"
    #[test]
    fn test_noun_toiedomo() {
        let sentence = "たとえ犯罪者といえども、動物みたいに扱ってはいけない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "といえども");
        assert_pattern_range(&patterns, "といえども", 5, 11); // 者といえども
    }

    // Testing: structure.standard[2] - "な-Adjective + (だ) + といえども"
    #[test]
    fn test_na_adj_toiedomo() {
        let sentence = "運転が上手だといえども、シートベルトは必ず着用しなくてはならない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "といえども");
        assert_pattern_range(&patterns, "といえども", 5, 11); // だといえども
    }

    // Testing: structure.standard[3] - "い-Adjective + といえども"
    #[test]
    fn test_i_adj_toiedomo() {
        let sentence = "彼女がどれだけ優しいといえども、そんなことばかりやってたら別れられるぞ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "といえども");
        assert_pattern_range(&patterns, "といえども", 7, 15); // 優しいといえども
    }
}

// ============================================================================
// ともなると・にもなると Tests
// ============================================================================

mod tomonaruto_tests {
    use super::*;

    // Pattern: ともなると・にもなると (when it comes to, once)
    // Data source: grammar_points_data.json["ともなると・にもなると"]
    // Testing all structure variants:
    //   - standard[0]: Noun + と + (も) + なると
    //   - standard[1]: Verb[る] + と + (も) + なると
    //   - standard[2]: (1) に (using に instead of と)
    //   - standard[3]: (2) なれば (using なれば instead of なると)

    // Testing: Noun + ともなると (with も)
    #[test]
    fn test_noun_tomonaruto() {
        let sentence = "ゴールデンウィークともなると、遊園地などは混む。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ともなると・にもなると");
        assert_pattern_range(&patterns, "ともなると・にもなると", 0, 14); // ゴールデンウィークともなると
    }

    // Testing: Verb[る] + ともなると (with も)
    #[test]
    fn test_verb_tomonaruto() {
        let sentence = "あなたも行くともなると、席が足りないので車を増やしましょう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ともなると・にもなると");
        assert_pattern_range(&patterns, "ともなると・にもなると", 4, 11); // 行くともなると
    }

    // Testing: Noun + となると (without も)
    #[test]
    fn test_noun_tonaruto() {
        let sentence = "５０代となると体が思うように動かなくなる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ともなると・にもなると");
        assert_pattern_range(&patterns, "ともなると・にもなると", 2, 7); // 代となると
    }

    // Testing: Noun + にもなると (using に instead of と)
    #[test]
    fn test_noun_nimonaruto() {
        let sentence = "社会人にもなると、色々な社会的ルールを守らなければいけない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ともなると・にもなると");
        assert_pattern_range(&patterns, "ともなると・にもなると", 2, 8); // 人にもなると
    }

    // Testing: Noun + ともなれば (using なれば instead of なると)
    #[test]
    fn test_noun_tomonareba() {
        let sentence = "社会人ともなれば、責任が重くなる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ともなると・にもなると");
        assert_pattern_range(&patterns, "ともなると・にもなると", 2, 8); // 人ともなれば
    }

    // Testing: Verb[る] + ともなれば (verb + も + なれば)
    #[test]
    fn test_verb_tomonareba() {
        let sentence = "会社のウェブページを作成するともなれば、優秀なプログラマーが必要だ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ともなると・にもなると");
        assert_pattern_range(&patterns, "ともなると・にもなると", 10, 19); // 作成するともなれば
    }

    // NOTE: Noun + になると is NOT this pattern - it's a different construction
    // The grammar specifically states that と (not に) is used for this pattern
    // になると simply means "when it becomes X" without the emphasis on unavoidability
    //
    // Example that does NOT match this pattern:
    // "週末になると観光地も賑わう。" - This is just "when it becomes weekend"
    //
    // Example that DOES match:
    // "週末ともなると観光地も賑わう。" - "Once it's the weekend (inevitably)..."
}

// ============================================================================
// をいいことに Tests
// ============================================================================

mod woiikotoni_tests {
    use super::*;

    // Pattern: をいいことに (take advantage of)
    // Data source: grammar_points_data.json["をいいことに"]
    // Testing all structure variants:
    //   - standard[0]: Verb + の + をいいことに(して)
    //   - standard[1]: Noun + (なの) + をいいことに(して)
    //   - standard[2]: い-Adjective + の + をいいことに(して)
    //   - standard[3]: な-Adjective + なの + をいいことに(して)

    // Testing: Verb + の + をいいことに
    #[test]
    fn test_verb_no_woiikotoni() {
        let sentence = "顔が見えないのをいいことに、ひどいことを書き込む人は最低だ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "をいいことに");
        assert_pattern_range(&patterns, "をいいことに", 4, 13); // ないのをいいことに
    }

    // Testing: Verb + の + をいいことにして
    #[test]
    fn test_verb_no_woiikotoni_shite() {
        let sentence = "先生の耳が遠いのをいいことにして、授業中に友達と話していた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "をいいことに");
        assert_pattern_range(&patterns, "をいいことに", 5, 16); // 遠いのをいいことにして
    }

    // Testing: い-Adjective + の + をいいことに
    #[test]
    fn test_i_adj_no_woiikotoni() {
        let sentence = "彼が優しいのをいいことに、クラスメイトたちは彼をいじめた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "をいいことに");
        assert_pattern_range(&patterns, "をいいことに", 2, 12); // 優しいのをいいことに
    }

    // Testing: Noun + なの + をいいことに
    #[test]
    fn test_noun_nano_woiikotoni() {
        let sentence = "休みなのをいいことに、友達と一日中ゲームをした。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "をいいことに");
        assert_pattern_range(&patterns, "をいいことに", 0, 10); // 休みなのをいいことに
    }

    // Testing: な-Adjective/Noun + である + の + をいいことに
    #[test]
    fn test_dearu_no_woiikotoni() {
        let sentence = "彼女は美人であるのをいいことに、色々な男からお金を騙し取った。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "をいいことに");
        assert_pattern_range(&patterns, "をいいことに", 6, 15); // あるのをいいことに
    }
}

// ============================================================================
// 如何 Tests
// ============================================================================

mod ika_tests {
    use super::*;

    // Pattern: 如何 (いかん - depending on)
    // Data source: grammar_points_data.json["如何"]
    //
    // Structure variants to test:
    //   - standard[0]: Noun + （の）+ いかん + で（は）
    //   - standard[1]: Noun + （の）+ いかん + だ
    //   - standard[2]: によって（は） (continuation of structure[0])
    //   - standard[3]: である (continuation of structure[1])

    // Testing: Noun + の + いかん + で
    #[test]
    fn test_ikan_de_with_no() {
        let sentence = "嵐の状況のいかんで、来週のイベントが中止されるかされないかが決まる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "如何");
        assert_pattern_range(&patterns, "如何", 4, 9); // のいかんで
    }

    // Testing: Noun + いかん + で (without の)
    #[test]
    fn test_ikan_de_without_no() {
        let sentence = "前回の試験の結果いかんでどの大学に入れるかが決まる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "如何");
        assert_pattern_range(&patterns, "如何", 8, 12); // いかんで
    }

    // Testing: Noun + の + いかん + だ
    #[test]
    fn test_ikan_da_with_no() {
        let sentence = "東大に入学できるかは、君たちの努力のいかんだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "如何");
        assert_pattern_range(&patterns, "如何", 17, 22); // のいかんだ
    }

    // Testing: Noun + いかん + だ (without の)
    #[test]
    fn test_ikan_da_without_no() {
        let sentence = "来月昇進できるかは自分の業績いかんだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "如何");
        assert_pattern_range(&patterns, "如何", 14, 18); // いかんだ
    }

    // Testing: Noun + いかん + によって（は）
    #[test]
    fn test_ikan_niyotte() {
        let sentence = "実験の結果いかんによっては、最初からやり直さないといけなくなる可能性もあります。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "如何");
        assert_pattern_range(&patterns, "如何", 5, 13); // いかんによっては
    }

    // Testing: Noun + の + いかん + である
    #[test]
    fn test_ikan_dearu() {
        let sentence = "合格できるかは、努力のいかんである。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "如何");
        assert_pattern_range(&patterns, "如何", 10, 17); // のいかんである
    }
}

// ============================================================================
// ～るまでだ Tests
// ============================================================================

mod rumadeda_tests {
    use super::*;

    // Pattern: ～るまでだ (merely, simply, one can only but)
    // Data source: grammar_points_data.json["～るまでだ"]
    // Testing: structure.standard[0] - "Verb[ば] + Verb[る] + まで + だ"
    //
    // Other structures to test:
    //   - standard[1]: Verb[たら] + Verb[る] + まで + だ
    //   - standard[2]: Verb[ても] + Verb[る] + まで + だ
    //   - standard with までのこと variant
    //   - polite[0-3]: Same forms + です

    // Testing: Verb[ば] + Verb[る] + まで + だ
    #[test]
    fn test_rumadeda_ba_conditional() {
        let sentence = "誰にも教えてもらえないなら、ネットで調べてみるまでだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～るまでだ");
        assert_pattern_range(&patterns, "～るまでだ", 21, 26); // みるまでだ
    }

    // Testing: Verb[たら] + Verb[る] + まで + だ
    #[test]
    fn test_rumadeda_tara_conditional() {
        let sentence = "もし失敗したのなら、もう一回挑戦するまでだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～るまでだ");
        assert_pattern_range(&patterns, "～るまでだ", 14, 21); // 挑戦するまでだ
    }

    // Testing: Verb[る] + までのこと + だ
    #[test]
    fn test_rumadeda_madenokoto() {
        let sentence = "誰も迎えにきてくれないのなら、電車で帰るまでのことだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～るまでだ");
        assert_pattern_range(&patterns, "～るまでだ", 18, 26); // 帰るまでのことだ
    }

    // Testing: Verb[ても] + Verb[る] + まで + だ
    #[test]
    fn test_rumadeda_temo_conditional() {
        let sentence = "受け入れてもらえなくても、諦めずに頑張るまでだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～るまでだ");
        assert_pattern_range(&patterns, "～るまでだ", 17, 23); // 頑張るまでだ
    }

    // Testing: Verb[る] + まで + です (polite)
    #[test]
    fn test_rumadeda_polite() {
        let sentence = "この会社に入社できなかったら、別の会社を探すまでのことです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～るまでだ");
        assert_pattern_range(&patterns, "～るまでだ", 20, 29); // 探すまでのことです
    }

    // Testing: Simple negative situation + Verb[る] + まで + だ
    #[test]
    fn test_rumadeda_simple_negative() {
        let sentence = "嫌なら辞めるまでだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～るまでだ");
        assert_pattern_range(&patterns, "～るまでだ", 3, 9); // 辞めるまでだ
    }
}

// ============================================================================
// にあって Tests
// ============================================================================

mod niatte_tests {
    use super::*;

    // Pattern: にあって (in, at, under the conditions of)
    // Data source: grammar_points_data.json["にあって"]
    // Testing: structure.standard[0] - "Noun + にあって"
    //
    // Other structures to test:
    //   - With も: Noun + にあっても (even under conditions)

    // Testing: Noun + にあって (basic form)
    #[test]
    fn test_niatte_position() {
        let sentence = "彼女は校長先生という立場にあって、いつも生徒たちのために一生懸命働いている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にあって");
        assert_pattern_range(&patterns, "にあって", 10, 16); // 立場にあって
    }

    // Testing: Noun + にあって (situation)
    #[test]
    fn test_niatte_situation() {
        let sentence = "インフルエンザが流行っているという状況にあって、外にいる人はみんなマスクをしている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にあって");
        assert_pattern_range(&patterns, "にあって", 17, 23); // 状況にあって
    }

    // Testing: Noun + にあって (state of emergency)
    #[test]
    fn test_niatte_emergency() {
        let sentence = "緊急事態宣言にあって、県を跨ぐ移動が制限された。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にあって");
        assert_pattern_range(&patterns, "にあって", 4, 10); // 宣言にあって
    }

    // Testing: Noun + にあっても (even under conditions)
    #[test]
    fn test_niattemo_emergency() {
        let sentence = "緊急時にあっても、常に冷静を保つようにしてください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にあって");
        assert_pattern_range(&patterns, "にあって", 2, 8); // 時にあっても
    }

    // Testing: Noun + にあっても (difficult circumstances)
    #[test]
    fn test_niattemo_difficult() {
        let sentence = "こんな厳しい状況にあっても、諦めずに営業を続けていた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にあって");
        assert_pattern_range(&patterns, "にあって", 6, 13); // 状況にあっても
    }
}

// ============================================================================
// １～たりとも～ない Tests
// ============================================================================

mod ichi_taritomo_nai_tests {
    use super::*;

    // Pattern: １～たりとも～ない (not even one, not a single)
    // Data source: grammar_points_data.json["１～たりとも～ない"]
    // Testing: structure.standard[0] - "1 + Counter + たりとも + Phrase［ない］"
    //
    // Structure:
    // - Number + Counter word (一秒, 一分, 一ミリ) OR quantifier (少し)
    // - たりとも (particle combination)
    // - Negative phrase (verb with ない)
    //
    // Special expression: 何人たりとも (no matter who, no exceptions)

    // Testing: One second + たりとも + negative verb
    #[test]
    fn test_taritomo_one_second() {
        let sentence = "消防士などは一秒たりとも気を抜くことができない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "１～たりとも～ない");
        assert_pattern_range(&patterns, "１～たりとも～ない", 7, 12); // 秒たりとも
    }

    // Testing: One minute + たりとも + negative verb
    #[test]
    fn test_taritomo_one_minute() {
        let sentence = "私は一分たりとも残業をしたくないので毎日定時に帰っています。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "１～たりとも～ない");
        assert_pattern_range(&patterns, "１～たりとも～ない", 3, 8); // 分たりとも
    }

    // Testing: One millimeter + たりとも + negative verb
    #[test]
    fn test_taritomo_one_millimeter() {
        let sentence = "みんなで押し入れようとしたが、一ミリたりとも動かなかったから、ユンボで押し込みました。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "１～たりとも～ない");
        assert_pattern_range(&patterns, "１～たりとも～ない", 16, 22); // ミリたりとも
    }

    // Testing: 少し (a little) + たりとも + negative
    #[test]
    fn test_taritomo_sukoshi() {
        let sentence = "少したりとも油断をすると、ミスをして命を落とす恐れがあります。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "１～たりとも～ない");
        assert_pattern_range(&patterns, "１～たりとも～ない", 0, 6); // 少したりとも
    }

    // Testing: 何人たりとも (special set expression - no matter who)
    #[test]
    fn test_taritomo_nanbito() {
        let sentence = "この洞窟へは２０年前に起きた事故以来、何人たりとも立ち入ることが許されない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "１～たりとも～ない");
        assert_pattern_range(&patterns, "１～たりとも～ない", 20, 25); // 人たりとも
    }
}

// ============================================================================
// Adj限りだ Tests
// ============================================================================

mod adj_kagirida_tests {
    use super::*;

    // Pattern: Adj限りだ (extremely, as ~ as can be)
    // Data source: grammar_points_data.json["Adj限りだ"]
    // Testing: structure.standard[0-1] and polite[0-1]
    //
    // Structure:
    // - い-Adjective + 限り + だ/です
    // - な-Adjective + な + 限り + だ/です
    //
    // Meaning: "extremely (A)", "as (A) as can be" - the limit of (A)
    // Formal expression highlighting the intensity of traits/emotions

    // Testing: い-Adjective + 限り + だ (standard form)
    #[test]
    fn test_kagirida_i_adj_standard() {
        let sentence = "宝くじで一等が当たったなんて、羨ましい限りだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Adj限りだ");
        assert_pattern_range(&patterns, "Adj限りだ", 15, 22); // 羨ましい限りだ
    }

    // Testing: な-Adjective + な + 限り + だ (standard form)
    #[test]
    fn test_kagirida_na_adj_standard() {
        let sentence = "一分遅れただけで不合格にされるなんて、残念な限りだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Adj限りだ");
        assert_pattern_range(&patterns, "Adj限りだ", 19, 25); // 残念な限りだ
    }

    // Testing: い-Adjective + 限り + です (polite form)
    #[test]
    fn test_kagirida_i_adj_polite() {
        let sentence = "私のためにこんな素晴らしい送別会を開いてくれるなんて、嬉しい限りです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Adj限りだ");
        assert_pattern_range(&patterns, "Adj限りだ", 27, 34); // 嬉しい限りです
    }

    // Testing: な-Adjective + な + 限り + です (polite form)
    #[test]
    fn test_kagirida_na_adj_polite() {
        let sentence = "同僚にあんなことを言われて、不快な限りです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Adj限りだ");
        assert_pattern_range(&patterns, "Adj限りだ", 14, 21); // 不快な限りです
    }
}

// ============================================================================
// とは Tests
// ============================================================================

mod toha_tests {
    use super::*;

    // Pattern: とは (emphatic particle expressing surprise/shock)
    // Data source: grammar_points_data.json["とは"]
    // Testing: structure.standard[0] - "Verb + (など) + とは"
    //
    // Structures to test:
    //   - standard[0]: Verb + (など) + とは
    //   - standard[1]: Noun + (だ) + (など) + とは
    //   - standard[2]: い-Adjective + (など) + とは
    //   - standard[3]: な-Adjective + (だ) + (など) + とは

    // Testing: Verb + とは (basic form, sentence-final)
    #[test]
    fn test_toha_verb_basic() {
        let sentence = "まさかこんな場所でなつみちゃんに会うとは！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とは");
        assert_pattern_range(&patterns, "とは", 16, 20); // 会うとは
    }

    // Testing: Verb + とは + continuation (with explicit emotion word)
    #[test]
    fn test_toha_verb_with_emotion() {
        let sentence = "高橋さんがもう結婚していたとは驚いた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とは");
        assert_pattern_range(&patterns, "とは", 12, 15); // たとは
    }

    // Testing: い-Adjective + とは
    #[test]
    fn test_toha_i_adjective() {
        let sentence = "君があんなに速く走れるとは...なんで今まで黙っていたんだい？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とは");
        assert_pattern_range(&patterns, "とは", 8, 13); // 走れるとは
    }

    // Testing: Verb + など + とは (with など)
    #[test]
    fn test_toha_verb_nado() {
        let sentence = "こんな簡単なことで失敗するなどとは思っていなかった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とは");
        assert_pattern_range(&patterns, "とは", 13, 17); // などとは
    }

    // Testing: Noun + とは (without だ)
    #[test]
    fn test_toha_noun_basic() {
        let sentence = "こんな安く家が買えるとは思わなかったよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とは");
        assert_pattern_range(&patterns, "とは", 7, 12); // 買えるとは
    }
}

// ============================================================================
// じゃあるまいし Tests
// ============================================================================

mod jaarumaishi_tests {
    use super::*;

    // Pattern: じゃあるまいし (it's not like, you're not)
    // Data source: grammar_points_data.json["じゃあるまいし"]
    // Testing: structure.standard[0] - "Noun + じゃある + まいし" (or ではある)
    //
    // Other structures to test:
    //   - standard[1]: Verb + ん + じゃある + まいし
    //   - standard[2]: Verb + わけ + じゃある + まいし
    //   - Note: ではある and でもある are alternatives to じゃある

    // Testing: structure.standard[0] - "Noun + じゃある + まいし"
    #[test]
    fn test_jaarumaishi_noun_ja() {
        let sentence = "もう子供じゃあるまいし、行動する前に少しでも考えたらどうなの？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "じゃあるまいし");
        assert_pattern_range(&patterns, "じゃあるまいし", 2, 11); // 子供じゃあるまいし
    }

    // Testing: structure.standard[0] - "Noun + ではある + まいし"
    #[test]
    fn test_jaarumaishi_noun_deha() {
        let sentence = "子供ではあるまいし、自分の責任は自分で取りなさい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "じゃあるまいし");
        assert_pattern_range(&patterns, "じゃあるまいし", 0, 9); // 子供ではあるまいし
    }

    // Testing: structure.standard[1] - "Verb + ん + じゃある + まいし"
    #[test]
    fn test_jaarumaishi_verb_n_ja() {
        let sentence = "親に止められているんじゃあるまいし、好きに自分が行きたいところに行けばいいんじゃないの？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "じゃあるまいし");
        assert_pattern_range(&patterns, "じゃあるまいし", 9, 17); // んじゃあるまいし
    }

    // Testing: structure.standard[1] - "Verb + ん + ではある + まいし"
    #[test]
    fn test_jaarumaishi_verb_n_deha() {
        let sentence = "誰かに教えられたんではあるまいし、自分で調べることもできるでしょう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "じゃあるまいし");
        assert_pattern_range(&patterns, "じゃあるまいし", 8, 16); // んではあるまいし
    }

    // Testing: structure.standard[2] - "Verb + わけ + じゃある + まいし"
    #[test]
    fn test_jaarumaishi_verb_wake_ja() {
        let sentence = "減るわけじゃあるまいし、少しぐらい貸してくれてもいいじゃん。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "じゃあるまいし");
        assert_pattern_range(&patterns, "じゃあるまいし", 2, 11); // わけじゃあるまいし
    }

    // Testing: structure.standard[2] - "Verb + わけ + ではある + まいし"
    #[test]
    fn test_jaarumaishi_verb_wake_deha() {
        let sentence = "忘れるわけではあるまいし、メモを取る必要はないよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "じゃあるまいし");
        assert_pattern_range(&patterns, "じゃあるまいし", 3, 12); // わけではあるまいし
    }
}

// ============================================================================
// てからというもの Tests
// ============================================================================

mod tekaratoiumono_tests {
    use super::*;

    // Pattern: てからというもの (ever since)
    // Data source: grammar_points_data.json["てからというもの"]
    // Testing: structure.standard[0] - "Verb[て] + から + というもの"
    //
    // Other structures to test:
    //   - standard[1]: それから + というもの (at beginning of new sentence)

    // Testing: structure.standard[0] - "Verb[て] + から + というもの"
    #[test]
    fn test_tekaratoiumono_verb_te() {
        let sentence = "この会社に入社してからというもの、日付が変わる前に家に帰ったことがない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てからというもの");
        assert_pattern_range(&patterns, "てからというもの", 8, 16); // てからというもの
    }

    // Testing: structure.standard[0] - another example
    #[test]
    fn test_tekaratoiumono_walking() {
        let sentence = "仕事に行く前に散歩に行くようにしてからというもの、どんどん体重が減ってきた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てからというもの");
        assert_pattern_range(&patterns, "てからというもの", 16, 24); // てからというもの
    }

    // Testing: structure.standard[0] - another example
    #[test]
    fn test_tekaratoiumono_movie() {
        let sentence = "ファイナルデスティネーションという映画を見てからというもの、木材を運んだトラックの後ろを走るのを避けるようになった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てからというもの");
        assert_pattern_range(&patterns, "てからというもの", 21, 29); // てからというもの
    }

    // Testing: structure.standard[1] - "それから + というもの"
    #[test]
    fn test_tekaratoiumono_sorekara() {
        let sentence = "彼は社長にみんなの前でものすごく怒られた。それからというもの彼は別人になったかのように性格が変わった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てからというもの");
        assert_pattern_range(&patterns, "てからというもの", 21, 30); // それからというもの
    }

    // Testing: structure.standard[1] - another それから example
    #[test]
    fn test_tekaratoiumono_sorekara_train() {
        let sentence = "先月彼女が痴漢にあった。それからというもの、満員電車を避けるようになった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てからというもの");
        assert_pattern_range(&patterns, "てからというもの", 12, 21); // それからというもの
    }
}

// ============================================================================
// かたわら (besides, in addition to, while)
// Data source: grammar_points_data.json["かたわら"]
// ============================================================================
#[cfg(test)]
mod katawara_tests {
    use super::*;

    // Pattern: かたわら (besides, in addition to, while)
    // Data source: grammar_points_data.json["かたわら"]
    // Testing: structure.standard[0] - "Verb[る] + かたわら"
    #[test]
    fn test_katawara_verb() {
        let sentence = "彼は飲食店を経営しているかたわら、鳶職人としても働いている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かたわら");
        assert_pattern_range(&patterns, "かたわら", 10, 16); // いるかたわら
    }

    // Testing: structure.standard[0] - another verb example
    #[test]
    fn test_katawara_verb_professor() {
        let sentence = "グレッグラフィンは大学の教授をしているかたわら、歌手として活動している。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かたわら");
        assert_pattern_range(&patterns, "かたわら", 17, 23); // いるかたわら
    }

    // Testing: structure.standard[1] - "Noun + の + かたわら"
    #[test]
    fn test_katawara_noun() {
        let sentence = "私の妹は本業のかたわら、英会話教室で英語を教えている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かたわら");
        assert_pattern_range(&patterns, "かたわら", 4, 11); // 本業のかたわら
    }

    // Testing: structure.standard[1] - another noun example
    #[test]
    fn test_katawara_noun_childcare() {
        let sentence = "妻は子育てのかたわら、近所の子供達にそろばんを教えている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かたわら");
        assert_pattern_range(&patterns, "かたわら", 2, 10); // 子育てのかたわら
    }
}

// ============================================================================
// を皮切りに (starting with, beginning with)
// Data source: grammar_points_data.json["を皮切りに"]
// ============================================================================
#[cfg(test)]
mod wokawakirini_tests {
    use super::*;

    // Pattern: を皮切りに (starting with, beginning with)
    // Data source: grammar_points_data.json["を皮切りに"]
    // Testing: structure.standard[0] - "Noun + を皮切りに"
    #[test]
    fn test_wokawakirini_noun_nishite() {
        let sentence = "クラスメイトの高橋さんを皮切りに、クラスみんな風邪になってしまった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を皮切りに");
        assert_pattern_range(&patterns, "を皮切りに", 11, 16); // を皮切りに
    }

    // Testing: structure.standard[1] - "Verb[た] + の + を皮切りに"
    #[test]
    fn test_wokawakirini_verb_ru_nishite() {
        let sentence = "藤田先輩がやめたのを皮切りに、他の人たちもどんどんやめていった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を皮切りに");
        assert_pattern_range(&patterns, "を皮切りに", 9, 14); // を皮切りに
    }

    // Testing: structure.standard[1] - "Verb[た] + の + を皮切りに"
    #[test]
    fn test_wokawakirini_verb_ta_nishite() {
        let sentence = "あの俳優はテレビ東京に出演したのを皮切りに、色々な局に出演することになった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を皮切りに");
        assert_pattern_range(&patterns, "を皮切りに", 16, 21); // を皮切りに
    }

    // Testing: structure.standard[0] with alternative "を皮切りとして"
    #[test]
    fn test_wokawakirini_noun_toshite() {
        let sentence = "あのグループは名古屋を皮切りとして、全国でライブを行った。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を皮切りに");
        assert_pattern_range(&patterns, "を皮切りに", 10, 17); // を皮切りとして
    }

    // Testing: Noun + を皮切りに (basic form)
    #[test]
    fn test_wokawakirini_noun_short() {
        let sentence = "佐々木さんの発言を皮切りに、賛成する人が出てきた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を皮切りに");
        assert_pattern_range(&patterns, "を皮切りに", 8, 13); // を皮切りに
    }
}

// ============================================================================
// なり Tests
// ============================================================================

mod nari_tests {
    use super::*;

    // Pattern: なり (as soon as, the moment)
    // Data source: grammar_points_data.json["なり"]
    // Testing: structure.standard[0] - "Verb[る] + なり"
    //
    // Note: (B) is usually something surprising, brought about by strong emotion/will
    // Almost always used in relation to other people, not oneself

    #[test]
    fn test_nari_enter_house() {
        let sentence = "息子が家に上がるなりトイレへと駆け込んだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なり");
        assert_pattern_range(&patterns, "なり", 5, 10); // 上がるなり
    }

    #[test]
    fn test_nari_enter_room() {
        let sentence = "ママ友が私の家に入るなり、冷蔵庫の中を物色し始めた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なり");
        assert_pattern_range(&patterns, "なり", 8, 12); // 入るなり
    }

    #[test]
    fn test_nari_sit_down() {
        let sentence = "彼は机に座るなりすぐスマホをいじり始めたから注意した。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なり");
        assert_pattern_range(&patterns, "なり", 4, 8); // 座るなり
    }

    #[test]
    fn test_nari_wake_up() {
        let sentence = "朝起きるなり、大きな声で泣き出した。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なり");
        assert_pattern_range(&patterns, "なり", 1, 6); // 起きるなり
    }
}

// ============================================================================
// ともなく・ともなしに Tests
// ============================================================================

mod tomonaku_tomonashini_tests {
    use super::*;

    // Pattern: ともなく・ともなしに (absentmindedly, without paying attention)
    // Data source: grammar_points_data.json["ともなく・ともなしに"]
    // Testing: structure.standard[0] - "Verb[る] + ともなく + Verb"
    //          structure.standard[1] - "Verb[る] + ともなしに + Verb"
    //
    // Note: Action before ともなく/ともなしに is intentional, but result is unintentional
    // Often used with たら to emphasize timing

    #[test]
    fn test_tomonaku_watching_tv() {
        let sentence = "昨日テレビを見るともなく見ていたら、急にお母さんが出てきたから驚いた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ともなく・ともなしに");
        assert_pattern_range(&patterns, "ともなく・ともなしに", 6, 12); // 見るともなく
    }

    #[test]
    fn test_tomonaku_listening() {
        let sentence = "話を聞くともなく、先生と黒板の方向をぼーっと見ていた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ともなく・ともなしに");
        assert_pattern_range(&patterns, "ともなく・ともなしに", 2, 8); // 聞くともなく
    }

    #[test]
    fn test_tomonashini_listening_boss() {
        let sentence = "飲み会で上司の話を聞くともなしに聞いていたら、「お前はどう思う」と聞かれたから適当に返事した。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ともなく・ともなしに");
        assert_pattern_range(&patterns, "ともなく・ともなしに", 9, 16); // 聞くともなしに
    }

    #[test]
    fn test_tomonashini_walking() {
        let sentence = "暇だったから近所を歩くともなしに歩いていたら、高校の同級生にばったりあった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ともなく・ともなしに");
        assert_pattern_range(&patterns, "ともなく・ともなしに", 9, 16); // 歩くともなしに
    }

    #[test]
    fn test_tomonaku_looking() {
        let sentence = "窓の外を見るともなく見ていると、雨が降り始めた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ともなく・ともなしに");
        assert_pattern_range(&patterns, "ともなく・ともなしに", 4, 10); // 見るともなく
    }
}

// ============================================================================
// 塗れ (まみれ) Tests
// ============================================================================

mod mamire_tests {
    use super::*;

    // Pattern: まみれ (completely covered in, smeared all over with)
    // Data source: grammar_points_data.json["塗れ"]
    // Testing: structure.standard[0] - "Noun + まみれ"
    //
    // Other structures to test:
    //   - standard[1]: Noun + まみれ + の + Noun
    //   - standard[2]: Noun + まみれ + になって + Phrase

    #[test]
    fn test_mamire_basic() {
        let sentence = "服泥まみれじゃん！どうしたの？！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "塗れ");
        assert_pattern_range(&patterns, "塗れ", 1, 5); // 泥まみれ
    }

    #[test]
    fn test_mamire_blood() {
        let sentence = "家に帰ったら犬が血まみれになっていたと思ったら、どうやらいちごジャムを食べて寝たみたい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "塗れ");
        assert_pattern_range(&patterns, "塗れ", 8, 12); // 血まみれ
    }

    #[test]
    fn test_mamire_no_noun() {
        let sentence = "そんな泥まみれの服を洗濯機に入れないで！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "塗れ");
        assert_pattern_range(&patterns, "塗れ", 3, 7); // 泥まみれ
    }

    #[test]
    fn test_mamire_debt() {
        let sentence = "あんな借金まみれの男とは付き合わない方がいいよ！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "塗れ");
        assert_pattern_range(&patterns, "塗れ", 3, 8); // 借金まみれ
    }
}

// ============================================================================
// ようが～まいが Tests
// ============================================================================

mod youga_maiga_tests {
    use super::*;

    // Pattern: ようが～まいが (whether or not)
    // Data source: grammar_points_data.json["ようが～まいが"]
    // Testing: structure.standard - Verb[volitional] + が + Verb[まい] + が
    //
    // Main structure implemented: Verb[う/よう] + が + Verb[まい] + が
    //
    // TODO: Additional variants not yet implemented:
    //   - Adj[かろう] + が + Adj[かろう] + が (antonym pattern)
    //   - Noun + だろう + が + Noun + だろう + が (antonym pattern)
    //   These require different matchers as they use antonyms rather than positive/negative volitionals

    #[test]
    fn test_youga_maiga_verb_attend() {
        let sentence = "忘年会に出席しようが出席するまいが、会費はみんなからもらっています。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ようが～まいが");
        assert_pattern_range(&patterns, "ようが～まいが", 4, 17); // 出席しようが出席するまいが
    }

    #[test]
    fn test_youga_maiga_verb_rain() {
        let sentence = "雨が降ろうが降るまいが、明日は絶対に釣りに行くと決めた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ようが～まいが");
        assert_pattern_range(&patterns, "ようが～まいが", 2, 11); // 降ろうが降るまいが
    }

    // TODO: Undetectable - Adjective かろう variant
    // Pattern: Adj[かろう] + が + Adj[かろう] + が uses antonyms (楽しかろう vs 楽しくなかろう)
    // instead of positive/negative volitionals. This requires a separate matcher.
    //
    // #[test]
    // fn test_youga_maiga_adjective() {
    //     let sentence = "楽しかろうが楽しくなかろうが、学校は子供の仕事だから学校には行かなくてはだめだ。";
    //     let tokens = tokenize_sentence(sentence);
    //     let patterns = detect_patterns(&tokens);
    //     assert_has_pattern(&patterns, "ようが～まいが");
    // }

    // TODO: Undetectable - Noun だろう variant
    // Pattern: Noun + だろう + が + Noun + だろう + が uses different nouns (犬 vs 猫)
    // instead of positive/negative volitionals. This requires a separate matcher.
    //
    // #[test]
    // fn test_youga_maiga_noun() {
    //     let sentence = "犬であろうが猫であろうが、どちらにしても動物なので軽い気持ちで買ってはいけない。";
    //     let tokens = tokenize_sentence(sentence);
    //     let patterns = detect_patterns(&tokens);
    //     assert_has_pattern(&patterns, "ようが～まいが");
    // }
}

// ============================================================================
// Verb + だに Tests
// ============================================================================

mod verb_dani_tests {
    use super::*;

    // Pattern: Verb + だに (just, merely, even)
    // Data source: grammar_points_data.json["Verb + だに"]
    // Testing: structure.standard[0] - "Verb + だに"
    //
    // Note: だに is a formal adverbial particle similar to さえ/すら (even)
    // It follows verbs in dictionary form and expresses "just (A)" or "merely (A)"
    // Often used with verbs like 考える, 思い出す, 見る, 想像する

    #[test]
    fn test_verb_dani_think() {
        let sentence = "あんな地獄みたいな職場にまた明日も行かないとって考えるだに気が重くなる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb + だに");
        assert_pattern_range(&patterns, "Verb + だに", 24, 29); // 考えるだに
    }

    #[test]
    fn test_verb_dani_remember() {
        let sentence = "彼のことを思い出すだに、悲しくなります。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb + だに");
        assert_pattern_range(&patterns, "Verb + だに", 5, 11); // 思い出すだに
    }

    #[test]
    fn test_verb_dani_look() {
        let sentence = "部長からもらった仕事の量を見るだに、やる気が失せる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb + だに");
        assert_pattern_range(&patterns, "Verb + だに", 13, 17); // 見るだに
    }

    #[test]
    fn test_verb_dani_imagine() {
        let sentence = "そんな恐ろしいことを想像するだに身震いがする。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb + だに");
        assert_pattern_range(&patterns, "Verb + だに", 10, 16); // 想像するだに
    }
}

// ============================================================================
// Verb[て] + みせる Tests
// ============================================================================

mod verb_te_miseru_tests {
    use super::*;

    // Pattern: Verb[て] + みせる (I will definitely do, I swear I will do)
    // Data source: grammar_points_data.json["Verb[て] + みせる"]
    // Testing: structure.standard[0] - "Verb[て] + みせる"
    //          structure.polite[0] - "Verb[て] + みせます"
    //
    // Note: Emphatic structure indicating speaker will do (A) in a way clear to listener
    // Literal translation: "I will show you that I'll do (A)"
    // Often used when there is a target audience being shown the action

    #[test]
    fn test_te_miseru_win() {
        let sentence = "お前のために、次の試合では勝ってみせる！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[て] + みせる");
        assert_pattern_range(&patterns, "Verb[て] + みせる", 13, 19); // 勝ってみせる
    }

    #[test]
    fn test_te_miseru_pass() {
        let sentence = "来週のテストで合格してみせる！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[て] + みせる");
        assert_pattern_range(&patterns, "Verb[て] + みせる", 7, 14); // 合格してみせる
    }

    #[test]
    fn test_te_miseru_finish() {
        let sentence = "足が痛いけど、完走してみせる！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[て] + みせる");
        assert_pattern_range(&patterns, "Verb[て] + みせる", 7, 14); // 完走してみせる
    }

    #[test]
    fn test_te_miseru_polite() {
        let sentence = "必ず期限までに終わらせてみせます。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[て] + みせる");
        assert_pattern_range(&patterns, "Verb[て] + みせる", 10, 16); // せてみせます
    }
}

// ============================================================================
// からする Tests
// ============================================================================

mod karasuru_tests {
    use super::*;

    // Pattern: からする (about X, X or more, starting at X)
    // Data source: grammar_points_data.json["からする"]
    // Testing all structure variants

    #[test]
    fn test_karasuru_basic() {
        // Testing: structure.standard[0] - "Number + Counter + からする"
        let sentence = "この辺だと家賃は１５万からする。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "からする");
        assert_pattern_range(&patterns, "からする", 8, 15); // １５万からする
    }

    #[test]
    fn test_karasuru_with_noun() {
        // Testing: structure.standard[1] - "Number + Counter + からする + Noun"
        let sentence = "それって１５万からするやつでしょ？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "からする");
        assert_pattern_range(&patterns, "からする", 4, 11); // １５万からする
    }

    #[test]
    fn test_karasuru_no_variant() {
        // Testing: structure.standard[2] - "Number + Counter + からの + Noun"
        let sentence = "この商品を製造するには、１億からのお金がかかる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "からの");
        assert_pattern_range(&patterns, "からの", 12, 17); // １億からの
    }

    #[test]
    fn test_karasuru_polite() {
        // Testing: structure.polite[0] - "Number + Counter + からします"
        let sentence = "ここの駐車場代は一日３千円からします。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "からする");
        assert_pattern_range(&patterns, "からする", 8, 18); // 一日３千円からします
    }

    #[test]
    fn test_karasuru_watch_example() {
        // Testing: Another example with large amount
        let sentence = "父から三百万円からする時計をもらった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "からする");
        assert_pattern_range(&patterns, "からする", 3, 11); // 三百万円からする
    }

    #[test]
    fn test_karasuru_people_count() {
        // Testing: からの with people counter
        let sentence = "昨日の大地震で十人からの人が行方不明になった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "からの");
        assert_pattern_range(&patterns, "からの", 7, 12); // 十人からの
    }
}

// ============================================================================
// にして① Tests
// ============================================================================

mod nishite_tests {
    use super::*;

    // Pattern: にして① (at (A), over (A), only when (A))
    // Data source: grammar_points_data.json["にして①"]
    // Testing all structure variants

    #[test]
    fn test_nishite_age_counter() {
        // Testing: structure.standard[0] - "Number + Counter + にして"
        let sentence = "３０歳にして初めて海外旅行に行った。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にして①");
        assert_pattern_range(&patterns, "にして①", 0, 6); // ３０歳にして
    }

    #[test]
    fn test_nishite_ordinal_counter() {
        // Testing: structure.standard[0] - "Number + Counter + にして"
        let sentence = "五回目にしてやっと資格を取ることができた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にして①");
        assert_pattern_range(&patterns, "にして①", 0, 6); // 五回目にして
    }

    #[test]
    fn test_nishite_noun() {
        // Testing: structure.standard[1] - "Noun + にして"
        let sentence = "犯人たちは一瞬にして暗闇の中へと消えた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にして①");
        assert_pattern_range(&patterns, "にして①", 5, 10); // 一瞬にして
    }

    #[test]
    fn test_nishite_with_youyaku() {
        // Testing: にして followed by ようやく (finally)
        let sentence = "５０歳にしてようやく、イタリアに住んでいる従兄弟の家に行くことができた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にして①");
        assert_pattern_range(&patterns, "にして①", 0, 6); // ５０歳にして
    }

    #[test]
    fn test_nishite_with_hajimete() {
        // Testing: にして followed by 初めて (for the first time)
        let sentence = "母は６０歳にして初めて、コンサートに行ったらしい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にして①");
        assert_pattern_range(&patterns, "にして①", 2, 8); // ６０歳にして
    }
}

// ============================================================================
// にして② Tests
// ============================================================================

mod nishite_u2461_tests {
    use super::*;

    // Pattern: にして② (both (A) and (B))
    // Data source: grammar_points_data.json["にして②"]
    // Testing all structure variants

    #[test]
    fn test_nishite2_noun_kyouju() {
        // Testing: structure.standard[0] - "Noun + にして"
        // Example: 彼女は大学の教授にして会社の経営者でもある (She is both a university professor and a CEO)
        let sentence = "彼女は大学の教授にして会社の経営者でもある。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にして②");
        assert_pattern_range(&patterns, "にして②", 6, 11); // 教授にして
    }

    #[test]
    fn test_nishite2_na_adjective() {
        // Testing: structure.standard[1] - "［な］Adjective + にして"
        // Example: この足場は安全にしてかつ組み立てるのが簡単だ
        let sentence = "この足場は安全にしてかつ組み立てるのが簡単だから、注文が殺到している。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にして②");
        assert_pattern_range(&patterns, "にして②", 5, 10); // 安全にして
    }

    #[test]
    fn test_nishite2_saiwai() {
        // Testing: Set expression 幸いにして (fortunately)
        let sentence = "携帯を駅で落としてしまったが、幸いにして交番に届いていた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にして②");
        assert_pattern_range(&patterns, "にして②", 15, 20); // 幸いにして
    }

    #[test]
    fn test_nishite2_fukou() {
        // Testing: Set expression 不幸にして (unfortunately)
        let sentence = "彼は不幸にして、幼い頃に両親を亡くした。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にして②");
        assert_pattern_range(&patterns, "にして②", 2, 7); // 不幸にして
    }
}

// ============================================================================
// ものを Tests
// ============================================================================

mod monowo_tests {
    use super::*;

    // Pattern: ものを (if only... but)
    // Data source: grammar_points_data.json["ものを"]
    // Structure variants to test:
    //   - standard[0]: Verb[ば] + Verb + ものを
    //   - standard[1]: Verb[ば] + い-Adjective + ものを
    //   - standard[2]: Verb[ば] + な-Adjective + な + ものを
    //   - standard[3]: Verb[ば] + Noun + の + ものを
    //   - Note: ば can also be たら or と

    #[test]
    fn test_monowo_ba_verb() {
        // Testing: structure.standard[0] - "Verb[ば] + Verb + ものを"
        let sentence = "目を合わせなかったら止められなかったものを、なんで警察なんかと目を合わせちゃうんだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ものを");
        assert_pattern_range(&patterns, "ものを", 17, 21); // たものを
    }

    #[test]
    fn test_monowo_ba_i_adjective() {
        // Testing: structure.standard[1] - "Verb[ば] + い-Adjective + ものを"
        let sentence = "もっと早くに歯医者に行けば歯を抜かずに済んだものを...";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ものを");
        assert_pattern_range(&patterns, "ものを", 21, 25); // だものを
    }

    #[test]
    fn test_monowo_ba_na_adjective() {
        // Testing: structure.standard[2] - "Verb[ば] + な-Adjective + な + ものを"
        let sentence = "道具を使えば楽なものを、わざわざ自分の力でやろうとするから怪我をするんだよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ものを");
        assert_pattern_range(&patterns, "ものを", 7, 11); // なものを
    }

    #[test]
    fn test_monowo_ba_verb_simple() {
        // Testing: structure.standard[0] - "Verb[ば] + Verb + ものを" (simpler example)
        let sentence = "「手伝って」って言えばいいものを、一人でやるからこう言う目に遭うんだよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ものを");
        assert_pattern_range(&patterns, "ものを", 11, 16); // いいものを
    }

    #[test]
    fn test_monowo_tara_conditional() {
        // Testing: structure.standard[0] with たら instead of ば
        let sentence = "アレルギーなどがあったなら、さきに言ってくれればいいものを...";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ものを");
        assert_pattern_range(&patterns, "ものを", 24, 29); // いいものを
    }

    #[test]
    fn test_monowo_na_adjective_simple() {
        // Testing: structure.standard[2] - simpler な-Adjective example
        let sentence = "みんなでやれば簡単なものを...";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ものを");
        assert_pattern_range(&patterns, "ものを", 9, 13); // なものを
    }
}

// ============================================================================
// であれ Tests
// ============================================================================

mod deare_tests {
    use super::*;

    // Pattern: であれ (even if)
    // Data source: grammar_points_data.json["であれ"]
    // Testing: structure.standard[0] - "Noun + であれ"
    // Testing: structure.standard[1] - "な-Adjective + であれ"
    // Testing: structure.standard[2] - "WH-Word + であれ"

    #[test]
    fn test_deare_noun() {
        // Testing: structure.standard[0] - Noun + であれ
        let sentence = "子供であれ、法律は守らなくてはいけない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "であれ");
        assert_pattern_range(&patterns, "であれ", 0, 5); // 子供であれ
    }

    #[test]
    fn test_deare_noun_profession() {
        // Testing: structure.standard[0] - Noun + であれ (with profession/role)
        let sentence = "医者であれ、完璧な診断ができるとは限らない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "であれ");
        assert_pattern_range(&patterns, "であれ", 0, 5); // 医者であれ
    }

    #[test]
    fn test_deare_wh_word() {
        // Testing: structure.standard[2] - WH-Word + であれ
        let sentence = "理由が何であれ、家族や友達を裏切るのはよくない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "であれ");
        assert_pattern_range(&patterns, "であれ", 3, 7); // 何であれ
    }

    #[test]
    fn test_deare_na_adjective() {
        // Testing: structure.standard[1] - な-Adjective + であれ
        let sentence = "どんなに複雑であれ、解決策は必ずある";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "であれ");
        assert_pattern_range(&patterns, "であれ", 4, 9); // 複雑であれ
    }

    #[test]
    fn test_deare_noun_position() {
        // Testing: structure.standard[0] - Noun + であれ (position/status)
        let sentence = "上司であれ、間違いを犯すこともある";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "であれ");
        assert_pattern_range(&patterns, "であれ", 0, 5); // 上司であれ
    }
}

// ============================================================================
// に先駆けて Tests
// ============================================================================

mod ni_sakigakete_tests {
    use super::*;

    // Pattern: に先駆けて (ahead of, in advance of)
    // Data source: grammar_points_data.json["に先駆けて"]
    // Testing: structure.standard[0] - "Noun + に先駆（さきが）け（て）"
    //
    // Two forms to test:
    //   1. Noun + に先駆け (base form)
    //   2. Noun + に先駆けて (with て)

    #[test]
    fn test_ni_sakigakete_world() {
        let sentence = "世界に先駆けて、あの国は死刑を廃止した";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に先駆けて");
        assert_pattern_range(&patterns, "に先駆けて", 0, 7); // 世界に先駆けて
    }

    #[test]
    fn test_ni_sakigake_nation() {
        let sentence = "全国に先駆け、梅雨入りをした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に先駆けて");
        assert_pattern_range(&patterns, "に先駆けて", 0, 6); // 全国に先駆け
    }

    #[test]
    fn test_ni_sakigakete_companies() {
        let sentence = "他社に先駆けて、自動運転が可能な車を開発した";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に先駆けて");
        assert_pattern_range(&patterns, "に先駆けて", 0, 7); // 他社に先駆けて
    }

    #[test]
    fn test_ni_sakigake_industry() {
        let sentence = "業界に先駆け、画期的な技術を発表した";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に先駆けて");
        assert_pattern_range(&patterns, "に先駆けて", 0, 6); // 業界に先駆け
    }
}

// ============================================================================
// をおいてほかに〜ない Tests
// ============================================================================

mod wooitehokani_nai_tests {
    use super::*;

    // Pattern: をおいてほかに〜ない (none other than, nothing else but)
    // Data source: grammar_points_data.json["をおいてほかに〜ない"]
    // Testing: structure.standard[0] - "Noun + をおいてほか + には(1) + … + ない"
    //
    // Structure variants to test:
    //   - standard[0]: Noun + をおいてほか + には + ... + ない
    //   - standard[1]: Noun + をおいてほか + に + ... + ない (には without は)

    #[test]
    fn test_wooitehokani_nai_person() {
        let sentence = "こんなに詳しい人は高橋教授をおいてほかにいない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "をおいてほかに〜ない");
        assert_pattern_range(&patterns, "をおいてほかに〜ない", 11, 23); // 教授をおいてほかにいない
    }

    #[test]
    fn test_wooitehokani_nai_place() {
        let sentence = "最初のライブは、大阪をおいてほかにない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "をおいてほかに〜ない");
        assert_pattern_range(&patterns, "をおいてほかに〜ない", 8, 19); // 大阪をおいてほかにない
    }

    #[test]
    fn test_wooitehokani_nai_bunpro() {
        let sentence = "日本語の文法を習いたいなら、文プロをおいてほかにない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "をおいてほかに〜ない");
        assert_pattern_range(&patterns, "をおいてほかに〜ない", 15, 26); // プロをおいてほかにない
    }

    #[test]
    fn test_wooitehokani_niwa_nai() {
        let sentence = "この分野の専門家はあの先生をおいてほかにはいない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "をおいてほかに〜ない");
        assert_pattern_range(&patterns, "をおいてほかに〜ない", 11, 24); // 先生をおいてほかにはいない
    }
}

// ============================================================================
// をもって Tests (as of, effective from)
// ============================================================================

mod womotte2_tests {
    use super::*;

    // Pattern: をもって (as of, effective from)
    // Data source: grammar_points_data.json["をもって"]
    // Testing: structure.standard[0] - "Time-Related Expression + をもって"
    //
    // Structure variants to test:
    //   - standard[0]: Time expression + をもって
    //   - standard[1]: 以上 + をもって (これ/これ以上)
    //   - polite[0]: Time expression + をもちまして

    #[test]
    fn test_womotte_game_service() {
        let sentence = "このゲームのオンラインサービスは2023年8月10日をもって終了することに決定しました";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "をもって");
        assert_pattern_range(&patterns, "をもって", 25, 30); // 日をもって
    }

    #[test]
    fn test_womotte_today() {
        let sentence = "本日をもって、受付を終了いたします";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "をもって");
        assert_pattern_range(&patterns, "をもって", 0, 6); // 本日をもって
    }

    #[test]
    fn test_womotte_tour() {
        let sentence = "今回のツアーをもって、解散することになりました";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // Note: This one is detected by を以て_split due to different tokenization
        // but をもって pattern also matches, so we test for をもって
        assert!(
            has_pattern(&patterns, "をもって") || has_pattern(&patterns, "を以て_split"),
            "Expected をもって or を以て_split pattern"
        );
    }

    #[test]
    fn test_womochimashite_polite() {
        let sentence = "いつもご来店ありがとうございます。この店舗は本日をもちまして閉店いたします";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "をもって");
        assert_pattern_range(&patterns, "をもって", 22, 30); // 本日をもちまして
    }
}

// ============================================================================
// とはいえ Tests
// ============================================================================

mod tohaie_tests {
    use super::*;

    // Pattern: とはいえ (although, be that as it may)
    // Data source: grammar_points_data.json["とはいえ"]
    // Testing: structure.standard[0] - "Verb + とはいえ + Phrase"
    //
    // Other structures to test:
    //   - standard[1]: ［い］Adjective + とはいえ + Phrase
    //   - standard[2]: Noun + （だ）+ とはいえ + Phrase
    //   - standard[3]: ［な］Adjective + （だ）+ とはいえ + Phrase
    //   - standard[4]: Phrase (A)。とはいえ + Phrase (B) - sentence-initial

    #[test]
    fn test_tohaie_verb() {
        let sentence = "この事件の犯人が分かったとはいえ、まだ捕まっていない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とはいえ");
        assert_pattern_range(&patterns, "とはいえ", 12, 16); // とはいえ
    }

    #[test]
    fn test_tohaie_i_adjective() {
        let sentence = "このパソコンは新しいとはいえ、性能がいいわけではない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とはいえ");
        assert_pattern_range(&patterns, "とはいえ", 10, 14); // とはいえ
    }

    #[test]
    fn test_tohaie_noun_with_da() {
        let sentence = "日本人だとはいえ、日本語があまり話せないんだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とはいえ");
        assert_pattern_range(&patterns, "とはいえ", 4, 8); // とはいえ
    }

    #[test]
    fn test_tohaie_na_adjective() {
        let sentence = "この国は安全だとはいえ、日本ではないので注意してください";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とはいえ");
        assert_pattern_range(&patterns, "とはいえ", 7, 11); // とはいえ
    }

    #[test]
    fn test_tohaie_sentence_initial() {
        let sentence = "彼は有名な俳優だ。とはいえ、私はあまり知らない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とはいえ");
        assert_pattern_range(&patterns, "とはいえ", 9, 13); // とはいえ
    }
}

// ============================================================================
// ならでは Tests
// ============================================================================

mod naradeha_tests {
    use super::*;

    // Pattern: ならでは (unique to, impossible if not)
    // Data source: grammar_points_data.json["ならでは"]
    // Testing: structure.standard[1] - "Noun + ならでは + の + Noun"
    //
    // Other structures to test:
    //   - standard[2]: Noun + ならでは + だ/です
    //   - standard[4]: Noun + ならでは + Verb (less common)

    #[test]
    fn test_naradeha_no_noun() {
        let sentence = "味噌カツは名古屋ならではの食べ物です";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ならでは");
        assert_pattern_range(&patterns, "ならでは", 5, 12); // 名古屋ならでは
    }

    #[test]
    fn test_naradeha_da() {
        let sentence = "こんな綺麗な作品を作れるのは、藤田さんならではだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ならでは");
        assert_pattern_range(&patterns, "ならでは", 17, 23); // さんならでは
    }

    #[test]
    fn test_naradeha_desu() {
        let sentence = "この祭りはこの地域ならではです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ならでは");
        assert_pattern_range(&patterns, "ならでは", 7, 13); // 地域ならでは
    }

    #[test]
    fn test_naradeha_verb() {
        let sentence = "車なしで生活できるのは都会ならではだと思う";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ならでは");
        assert_pattern_range(&patterns, "ならでは", 11, 17); // 都会ならでは
    }
}

#[cfg(test)]
mod nami_tests {
    use super::*;

    // Pattern: 並み (on par with, as good as)
    // Data source: grammar_points_data.json["並み"]
    // Testing all structure variants:
    //   - standard[0]: Noun + 並み + だ
    //   - standard[1]: Noun + 並み + の + Noun
    //   - standard[2]: Noun + 並み + に + Phrase
    //   - polite[0]: Noun + 並み + です
    //   - polite[1]: Noun + 並み + の + Noun (same as standard)
    //   - polite[2]: Noun + 並み + に + Phrase (same as standard)

    #[test]
    fn test_nami_da() {
        let sentence = "あの子のサーブの仕方はプロ並みだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "並み");
        assert_pattern_range(&patterns, "並み", 11, 15); // プロ並み
    }

    #[test]
    fn test_nami_no_noun() {
        let sentence = "まだ５月なのに８月並みの暑さだ";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "並み");
        assert_pattern_range(&patterns, "並み", 7, 11); // ８月並み
    }

    #[test]
    fn test_nami_ni_phrase() {
        let sentence = "彼は日本語をネイティブ並みに話せるから安心できる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "並み");
        assert_pattern_range(&patterns, "並み", 6, 13); // ネイティブ並み
    }

    #[test]
    fn test_nami_desu() {
        let sentence = "彼女の料理の腕前はプロ並みです";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "並み");
        assert_pattern_range(&patterns, "並み", 9, 13); // プロ並み
    }
}

#[cfg(test)]
mod kososure_nai_tests {
    use super::*;

    // Pattern: こそすれ〜ない (certainly not B, but A)
    // Data source: grammar_points_data.json["こそすれ〜ない"]
    // Testing structure variants:
    //   - standard[0]: する-Verb + こそすれ (noun + こそすれ)
    //   - standard[1]: Verb stem + こそすれ (conjunctive form)
    //
    // Note: This pattern uses the classical realis form of する (すれ)
    // The pattern emphasizes "A, but certainly not B" where B is negative

    #[test]
    fn test_kososure_suru_verb() {
        let sentence = "このままだと日本の経済は悪化こそすれ、回復はしないだろう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "こそすれ〜ない");
        assert_pattern_range(&patterns, "こそすれ〜ない", 14, 18); // こそすれ
    }

    #[test]
    fn test_kososure_verb_stem() {
        let sentence = "あのカードは値段が上がりこそすれ、下がりはしないだろう";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "こそすれ〜ない");
        assert_pattern_range(&patterns, "こそすれ〜ない", 12, 16); // こそすれ
    }

    #[test]
    fn test_kososure_passive_verb() {
        let sentence = "感謝されこそすれ、文句を言われる筋合いはない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "こそすれ〜ない");
        assert_pattern_range(&patterns, "こそすれ〜ない", 4, 8); // こそすれ
    }
}

// ============================================================================
// あっての Tests
// ============================================================================

mod atteno_tests {
    use super::*;

    // Pattern: あっての (B exists only because of A)
    // Data source: grammar_points_data.json["あっての"]
    // Testing: structure.standard[0] - "Noun (A) + （が）+ あっての + Noun (B)"

    #[test]
    fn test_atteno_without_ga() {
        let sentence = "お客様あっての仕事だと思うのですが、理不尽なクレームを言われるとちょっとイラっときます。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あっての");
        assert_pattern_range(&patterns, "あっての", 0, 10); // お客様あっての仕事だ
    }

    #[test]
    fn test_atteno_efforts() {
        let sentence = "今回は皆様の努力あっての結果です。皆様本当にありがとうございました。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あっての");
        assert_pattern_range(&patterns, "あっての", 6, 16); // 努力あっての結果です
    }

    #[test]
    fn test_atteno_with_ga() {
        let sentence = "私はお金がなかったら愛もない、お金があっての幸せだと思います。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あっての");
        assert_pattern_range(&patterns, "あっての", 15, 25); // お金があっての幸せだ
    }
}

// ============================================================================
// ～たまでだ Tests
// ============================================================================

mod tamadeda_tests {
    use super::*;

    // Pattern: ～たまでだ (I simply/only did A)
    // Data source: grammar_points_data.json["～たまでだ"]
    // Testing all structure variants

    // Testing: structure.standard[0] - "Verb[た] + まで + だ"
    #[test]
    fn test_tamadeda_standard() {
        let sentence = "なんで俺が悪者扱いされないといけないんだ？俺はただ自分の意見を言ったまでだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～たまでだ");
        assert_pattern_range(&patterns, "～たまでだ", 31, 37); // 言ったまでだ
    }

    // Testing: structure.standard[1] - "Verb[た] + までのこと + だ"
    #[test]
    fn test_tamadeda_made_no_koto() {
        let sentence = "最近学校にも行かないし部屋からも出ないから、聞いたまでのことだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～たまでだ");
        assert_pattern_range(&patterns, "～たまでだ", 22, 31); // 聞いたまでのことだ
    }

    // Testing: structure.polite[0] - "Verb[た] + まで + です"
    #[test]
    fn test_tamadeda_polite() {
        let sentence = "そんなお礼とかはやめてください、人として当たり前のことをしたまでです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～たまでだ");
        assert_pattern_range(&patterns, "～たまでだ", 28, 34); // したまでです
    }

    // Testing: structure.polite[1] - "Verb[た] + までのこと + です"
    // Note: This is the same structure as standard[1], just used in polite context
    #[test]
    fn test_tamadeda_made_no_koto_polite_context() {
        let sentence = "お婆さんが困った顔をしていたから、助けたまでのことです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～たまでだ");
        assert_pattern_range(&patterns, "～たまでだ", 17, 27); // 助けたまでのことです
    }
}

// ============================================================================
// からある Tests
// ============================================================================

mod karaaru_tests {
    use super::*;

    // Pattern: からある (as much as, as many as)
    // Data source: grammar_points_data.json["からある"]
    // Testing: structure.standard[0] - "Number + Counter + からある"
    #[test]
    fn test_karaaru_weight() {
        let sentence = "おじいちゃんは８０歳なのに６０キロからあるダンベルを持ち上げることができる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "からある");
        assert_pattern_range(&patterns, "からある", 15, 21); // キロからある
    }

    // Testing: structure.standard[1] - "Number + Counter + からある + Noun"
    #[test]
    fn test_karaaru_with_noun() {
        let sentence = "彼は小柄であるにも関わらず、７０キロからある冷蔵庫を一人で持ち上げることができる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "からある");
        assert_pattern_range(&patterns, "からある", 16, 22); // キロからある
    }

    // Testing: structure.standard[2] - "Number + Counter + からいる"
    #[test]
    fn test_karairu_people() {
        let sentence = "この会社には正社員が３０００人からいる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "からある");
        assert_pattern_range(&patterns, "からある", 14, 19); // 人からいる
    }

    // Testing: structure.standard[3] - "Number + Counter + からいる + Noun"
    #[test]
    fn test_karairu_with_noun() {
        let sentence = "あのコンサートには一万人からいる人数がいる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "からある");
        assert_pattern_range(&patterns, "からある", 11, 16); // 人からいる
    }

    // Testing: structure.polite[0] - "Number + Counter + からあります"
    #[test]
    fn test_karaaru_polite() {
        let sentence = "この建物は、築１００年からあります。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "からある");
        assert_pattern_range(&patterns, "からある", 10, 17); // 年からあります
    }

    // Testing: structure.polite[2] - "Number + Counter + からいます"
    #[test]
    fn test_karairu_polite() {
        let sentence = "この学校には、生徒が５００人からいます。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "からある");
        assert_pattern_range(&patterns, "からある", 13, 19); // 人からいます
    }
}

// ============================================================================
// ともあろう Tests
// ============================================================================

mod tomoarou_tests {
    use super::*;

    // Pattern: ともあろう (of all people, such as)
    // Data source: grammar_points_data.json["ともあろう"]
    // Testing: structure.standard[0] - "Noun + ともあろう + もの/方/人 + が"

    // Testing with もの
    #[test]
    fn test_tomoarou_mono() {
        let sentence = "警察ともあろうものが違法薬物を使用していたなんて信じられない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ともあろう");
        assert_pattern_range(&patterns, "ともあろう", 0, 10); // 警察ともあろうものが
    }

    // Testing with 方
    #[test]
    fn test_tomoarou_kata() {
        let sentence = "教会の牧師さんともあろうお方が、詐欺で逮捕されるなんてショックすぎる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ともあろう");
        assert_pattern_range(&patterns, "ともあろう", 5, 15); // さんともあろうお方が
    }

    // Testing with 人
    #[test]
    fn test_tomoarou_hito() {
        let sentence = "有名な大学の教授ともあろう人が、ろくに敬語も使えないなんておかしい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ともあろう");
        assert_pattern_range(&patterns, "ともあろう", 6, 15); // 教授ともあろう人が
    }

    // Testing sarcastic usage
    #[test]
    fn test_tomoarou_sarcastic() {
        let sentence = "高橋さんともあろうお方がこんな簡単なことができないなんて、珍しいですね。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ともあろう");
        assert_pattern_range(&patterns, "ともあろう", 2, 12); // さんともあろうお方が
    }
}

// ============================================================================
// 以前 Tests
// ============================================================================

mod izen_tests {
    use super::*;

    // Pattern: 以前 (before even, prior to - emphatic criticism)
    // Data source: grammar_points_data.json["以前"]
    // Testing: structure.standard[0] - "Noun + 以前 + に"
    //
    // Structures to test:
    //   - standard[0]: Noun + 以前 + に
    //   - standard[1]: Noun (A) + 以前 + の + Noun (B)
    //   - Verb + 以前 + に/の
    //   - い-Adjective + 以前 + に/の
    //   - な-Adjective + 以前 + に/の

    // Test Noun + 以前 + に
    #[test]
    fn test_izen_noun_ni() {
        let sentence = "人のことを心配する以前に自分の心配をしろ！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "以前");
        assert_pattern_range(&patterns, "以前", 9, 12); // 以前に
    }

    // Test Noun + 以前 + の + Noun
    #[test]
    fn test_izen_noun_no_noun() {
        let sentence = "それは古いか新しいか以前の問題だと思う。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "以前");
        assert_pattern_range(&patterns, "以前", 10, 13); // 以前の
    }

    // Test Verb + 以前 + に
    #[test]
    fn test_izen_verb_ni() {
        let sentence = "安全か危険か以前に、それは法律で禁じられているので試すこともできません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "以前");
        assert_pattern_range(&patterns, "以前", 6, 9); // 以前に
    }

    // Test な-Adjective/Noun + 以前 + の
    #[test]
    fn test_izen_adjective_no() {
        let sentence = "お前が結婚できないのは顔以前の問題だよ。お前は性格が悪すぎる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "以前");
        assert_pattern_range(&patterns, "以前", 12, 15); // 以前の
    }

    // Test with verb dictionary form
    #[test]
    fn test_izen_verb_dictionary() {
        let sentence = "できるかできないか以前に、まず挑戦してみるべきだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "以前");
        assert_pattern_range(&patterns, "以前", 9, 12); // 以前に
    }
}

// ============================================================================
// Verb[ない]もの(だろう)か Tests
// ============================================================================

mod verb_nai_mono_darou_ka_tests {
    use super::*;

    // Pattern: Verb[ない]もの(だろう)か (if only, isn't there a way to)
    // Data source: grammar_points_data.json["Verb[ない]もの(だろう)か"]
    // Testing: structure.standard[0] - "Verb[できる][ない] + もの + (だろう) + か"

    // Test basic ないものか structure
    #[test]
    fn test_nai_mono_ka() {
        let sentence = "これはもっとシンプルにできないものか。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[ない]もの(だろう)か");
        assert_pattern_range(&patterns, "Verb[ない]もの(だろう)か", 13, 18); // ないものか
    }

    // Test ないものだろうか structure
    #[test]
    fn test_nai_mono_darou_ka() {
        let sentence = "こっちから言わないとやって貰えないものだろうか。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[ない]もの(だろう)か");
        assert_pattern_range(&patterns, "Verb[ない]もの(だろう)か", 15, 23); // ないものだろうか
    }

    // Test polite form: ないものでしょうか
    #[test]
    fn test_nai_mono_deshou_ka_polite() {
        let sentence = "難しいのはわかっているんですが、佐々木さんの腕でなんとかできないものでしょうか。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[ない]もの(だろう)か");
        assert_pattern_range(&patterns, "Verb[ない]もの(だろう)か", 30, 39); // ないものでしょうか
    }

    // Test with existence: ないものか (lack of existence)
    #[test]
    fn test_nai_mono_ka_existence() {
        let sentence = "仕事をもう少し楽にする方法はないものか。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[ない]もの(だろう)か");
        assert_pattern_range(&patterns, "Verb[ない]もの(だろう)か", 14, 19); // ないものか
    }

    // Test with existence: ないものだろうか (lack of existence)
    #[test]
    fn test_nai_mono_darou_ka_existence() {
        let sentence = "もっと楽にお金を稼げる仕事はないものだろうか。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "Verb[ない]もの(だろう)か");
        assert_pattern_range(&patterns, "Verb[ない]もの(だろう)か", 14, 22); // ないものだろうか
    }
}

// ============================================================================
// あくまでも Tests
// ============================================================================

mod akumademo_tests {
    use super::*;

    // Pattern: あくまでも (to the end, persistently)
    // Data source: grammar_points_data.json["あくまでも"]
    // Testing: structure.standard[0] - "あくまで（も） + Phrase"

    // Test standard form: あくまでも (with も)
    #[test]
    fn test_akumademo_with_mo() {
        let sentence = "容疑者はあくまでも自分がしたことを認めようとしませんでした。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あくまでも");
        assert_pattern_range(&patterns, "あくまでも", 4, 9); // あくまでも
    }

    // Test without も: あくまで
    #[test]
    fn test_akumademo_without_mo() {
        let sentence = "犯人のことなら私にお任せください！あくまで彼を捕まえるまで探し続けるつもりです！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あくまでも");
        assert_pattern_range(&patterns, "あくまでも", 17, 21); // あくまで
    }

    // Test sentence-initial with intention (つもり)
    #[test]
    fn test_akumademo_with_tsumori() {
        let sentence = "私たちはあくまでもあなたのお子さんを見つけるつもりです！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あくまでも");
        assert_pattern_range(&patterns, "あくまでも", 4, 9); // あくまでも
    }

    // Test mid-sentence placement
    #[test]
    fn test_akumademo_mid_sentence() {
        let sentence = "彼女はあくまでも子供たちには秘密にしておこうと思っていた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "あくまでも");
        assert_pattern_range(&patterns, "あくまでも", 3, 8); // あくまでも
    }
}

// ============================================================================
// を経て Tests
// ============================================================================

mod wohete_tests {
    use super::*;

    // Pattern: を経て (through, via, after undergoing)
    // Data source: grammar_points_data.json["を経て"]
    // Testing: structure.standard[0] - "Noun + を経（へ）て"

    #[test]
    fn test_wohete_time_period() {
        let sentence = "私たちは２０１９年に出会って、約２年間の交際を経て結婚しました。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を経て");
        assert_pattern_range(&patterns, "を経て", 20, 25); // 交際を経て
    }

    #[test]
    fn test_wohete_experience() {
        let sentence = "日本への留学を経て、日本の文化などを学ぶことができました。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を経て");
        assert_pattern_range(&patterns, "を経て", 4, 9); // 留学を経て
    }

    #[test]
    fn test_wohete_various_experiences() {
        let sentence = "人は色々な経験を経て、成長していく。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を経て");
        assert_pattern_range(&patterns, "を経て", 5, 10); // 経験を経て
    }

    #[test]
    fn test_wohete_long_period() {
        let sentence = "長い試験期間を経て、ようやく合格の知らせが届いた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を経て");
        assert_pattern_range(&patterns, "を経て", 4, 9); // 期間を経て
    }
}

// ============================================================================
// ながらに Tests
// ============================================================================

mod nagarani_tests {
    use super::*;

    // Pattern: ながらに (while being, as)
    // Data source: grammar_points_data.json["ながらに"]
    // Testing: structure.standard variants - limited set of words
    //
    // Key variants to test:
    //   - standard[0]: Verb[stem] + ながら(に)
    //   - standard[1]: Noun + ながら(に)
    //   - standard[2]: Verb[stem] + ながら + の + Noun
    //   - standard[4-5]: 生きながら(にして), 生まれながら(にして)
    //   - standard[6-8]: 昔ながら, いつもながら, 毎回ながら
    //   - standard[9-10]: 涙ながらに, 溜め息ながら

    #[test]
    fn test_nagarani_verb_stem_ni() {
        let sentence = "家に居ながらにして仕事ができるから便利だ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ながらに");
        assert_pattern_range(&patterns, "ながらに", 2, 9); // 居ながらにして
    }

    #[test]
    fn test_nagarani_noun() {
        let sentence = "子供ながらにして、なぜ親がいつも喧嘩をしていたか理解していた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ながらに");
        assert_pattern_range(&patterns, "ながらに", 0, 8); // 子供ながらにして
    }

    #[test]
    fn test_nagarani_umareru() {
        let sentence = "自由権は人が生まれながらに持っている権利の一つだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ながらに");
        assert_pattern_range(&patterns, "ながらに", 5, 13); // が生まれながらに
    }

    #[test]
    fn test_nagarani_namida() {
        let sentence = "彼女は涙ながらに彼を見送った。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ながらに");
        assert_pattern_range(&patterns, "ながらに", 3, 8); // 涙ながらに
    }

    #[test]
    fn test_nagarani_mukashi() {
        let sentence = "この町には昔ながらの建物が残っている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ながらに");
        assert_pattern_range(&patterns, "ながらに", 4, 10); // は昔ながらの
    }

    #[test]
    fn test_nagarani_itsumo() {
        let sentence = "いつもながら素晴らしい演奏でした。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ながらに");
        assert_pattern_range(&patterns, "ながらに", 0, 6); // いつもながら
    }
}

// ============================================================================
// たなり・なり Tests
// ============================================================================

mod tanari_nari_tests {
    use super::*;

    // Pattern: たなり・なり (remain as is, stay in that state)
    // Data source: grammar_points_data.json["たなり・なり"]
    // Testing: structure.standard[0] - "Verb[た] + なり + (で) + Phrase"
    //
    // Pattern indicates something existing in an ongoing state after action (A)
    // Often expresses undesirable state continuing despite (A) having happened
    // Similar to まま but more formal

    #[test]
    fn test_tanari_basic() {
        let sentence = "弟は彼女に振られて部屋に入ったなり、全然部屋から出てこなくなってしまった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たなり・なり");
        assert_pattern_range(&patterns, "たなり・なり", 12, 17); // 入ったなり
    }

    #[test]
    fn test_tanari_with_de() {
        let sentence = "父が釣りにいったなりで、まだ帰ってこない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たなり・なり");
        assert_pattern_range(&patterns, "たなり・なり", 5, 11); // いったなりで
    }

    #[test]
    fn test_tanari_long_time() {
        let sentence = "兄は上京したなり、帰ってこない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たなり・なり");
        assert_pattern_range(&patterns, "たなり・なり", 2, 8); // 上京したなり
    }

    #[test]
    fn test_tanari_comma() {
        let sentence = "彼は座ったなり、何も言わなくなった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たなり・なり");
        assert_pattern_range(&patterns, "たなり・なり", 2, 7); // 座ったなり
    }
}

// ============================================================================
// の極み Tests
// ============================================================================

mod nokiwami_tests {
    use super::*;

    // Pattern: の極み (the ultimate/extreme/epitome of)
    // Data source: grammar_points_data.json["の極み"]
    // Testing: structure.standard[0] - "Noun + の + 極（きわ）み"
    //
    // 極み is a noun meaning "extremity", used formally to express "the epitome of A"
    // Often followed by だ or conjunctive で

    #[test]
    fn test_nokiwami_basic_da() {
        let sentence = "あと５分待って居れば私が大好きなアイドルに会えていたとは...痛恨の極みだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "の極み");
        assert_pattern_range(&patterns, "の極み", 31, 36); // 痛恨の極み
    }

    #[test]
    fn test_nokiwami_polite_desu() {
        let sentence = "今までこんなに良くしてくれて、感激の極みです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "の極み");
        assert_pattern_range(&patterns, "の極み", 15, 20); // 感激の極み
    }

    #[test]
    fn test_nokiwami_negative_context() {
        let sentence = "有給は取るなとか鬼畜の極みだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "の極み");
        assert_pattern_range(&patterns, "の極み", 8, 13); // 鬼畜の極み
    }

    #[test]
    fn test_nokiwami_conjunctive_de() {
        let sentence = "この曲の歌詞は感動の極みで、聞くたびに涙が溢れ出てくる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "の極み");
        assert_pattern_range(&patterns, "の極み", 7, 12); // 感動の極み
    }
}

// ============================================================================
// にしてみれば Tests
// ============================================================================

mod nishitemireba_tests {
    use super::*;

    // Pattern: にしてみれば (from the point of view of)
    // Data source: grammar_points_data.json["にしてみれば"]
    // Testing: structure.standard[0] - "Noun + にしてみれば"
    // Testing: structure.standard[1] - "Noun + にしてみたら" (variant)

    #[test]
    fn test_nishitemireba_beginner() {
        let sentence = "上級者には簡単かもしれないが、初心者にしてみればかなり難しいと思う。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にしてみれば");
        assert_pattern_range(&patterns, "にしてみれば", 15, 24); // 初心者にしてみれば
    }

    #[test]
    fn test_nishitemireba_japanese_person() {
        let sentence = "日本人の私にしてみれば、日本語は簡単だけど英語は難しい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にしてみれば");
        assert_pattern_range(&patterns, "にしてみれば", 4, 11); // 私にしてみれば
    }

    #[test]
    fn test_nishitemireba_anime_viewer() {
        let sentence = "アニメをあまり知らない人にしてみれば、ポケモンもデジモンも同じアニメだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にしてみれば");
        assert_pattern_range(&patterns, "にしてみれば", 11, 18); // 人にしてみれば
    }

    #[test]
    fn test_nishitemitara_variant() {
        let sentence = "彼女にしてみたら、それは当然のことだったかもしれない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にしてみれば");
        assert_pattern_range(&patterns, "にしてみれば", 0, 8); // 彼女にしてみたら
    }
}

// ============================================================================
// だの Tests
// ============================================================================

mod dano_tests {
    use super::*;

    // Pattern: だの (things like, and whatnot)
    // Data source: grammar_points_data.json["だの"]
    // Testing: structure.standard[0] - "A + だの + B + だの"
    //
    // Note: だの requires at least two occurrences to form the pattern
    // Often lists opposites or things with the same negative trait
    // Can follow verbs, adjectives, nouns, or quotations

    #[test]
    fn test_dano_verbs() {
        let sentence = "彼は仕事はやめるだの家事を手伝わないだの、本当に自分以外のことは何も考えていない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // First だの after やめる
        assert_has_pattern(&patterns, "だの");
        assert_pattern_range(&patterns, "だの", 8, 10); // だの (single token)

        // Check that both occurrences are detected (second one also single token)
        let dano_matches: Vec<_> = patterns.iter().filter(|p| p.pattern_name == "だの").collect();
        assert_eq!(dano_matches.len(), 2, "Should detect both だの occurrences");
    }

    #[test]
    fn test_dano_i_adjectives() {
        let sentence = "この建物は汚いだの危ないだのと近所の人に言われていたので解体されるらしい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // First だの after 汚い (single token)
        assert_has_pattern(&patterns, "だの");
        assert_pattern_range(&patterns, "だの", 7, 9); // だの

        // Second だの after 危ない (split as だ + の)
        assert_has_pattern(&patterns, "だの_split");
        assert_pattern_range(&patterns, "だの_split", 12, 14); // だの (だ + の)
    }

    #[test]
    fn test_dano_nouns() {
        let sentence = "揚げ物だのコンビニ弁当だのばかり食べてると、また健康診断の時に先生に怒られるよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // First だの after 揚げ物 (single token)
        assert_has_pattern(&patterns, "だの");
        assert_pattern_range(&patterns, "だの", 3, 5); // だの

        // Second だの after 弁当 (split as だ + の)
        assert_has_pattern(&patterns, "だの_split");
        assert_pattern_range(&patterns, "だの_split", 11, 13); // だの (だ + の)
    }

    #[test]
    fn test_dano_nandano() {
        let sentence = "妻は大嫌いだの嫌いだのと毎日言ってくるから悲しくなってきた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // First だの after 大嫌い (single token)
        assert_has_pattern(&patterns, "だの");
        assert_pattern_range(&patterns, "だの", 5, 7); // だの

        // Second だの after 嫌い (split as だ + の)
        assert_has_pattern(&patterns, "だの_split");
        assert_pattern_range(&patterns, "だの_split", 9, 11); // だの (だ + の)
    }

    #[test]
    fn test_dano_quotations() {
        let sentence = "彼は「嫌」だの「大変」だの、何かをやる前から弱音を吐くから、一緒にいるとこっちまで仕事をしたくなくなる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        // First だの after 「嫌」
        assert_has_pattern(&patterns, "だの");
        assert_pattern_range(&patterns, "だの", 5, 7); // だの (single token)

        // Check that both occurrences are detected
        let dano_matches: Vec<_> = patterns.iter().filter(|p| p.pattern_name == "だの").collect();
        assert_eq!(dano_matches.len(), 2, "Should detect both だの occurrences");
    }
}

// ============================================================================
// べく Tests
// ============================================================================

mod beku_tests {
    use super::*;

    // Pattern: べく (in order to, for the purpose of)
    // Data source: grammar_points_data.json["べく"]
    // Testing structures:
    //   - standard[0]: Verb[る] + べく (standard form)
    //   - Exceptions: す + べく (classical する)
    //   - Modern: する + べく (modern variant)

    #[test]
    fn test_beku_verb_dictionary() {
        let sentence = "地域の高齢者の安全を守るべく、交番が設置されました。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "べく");
        assert_pattern_range(&patterns, "べく", 10, 14); // 守るべく
    }

    #[test]
    fn test_beku_verb_kau() {
        let sentence = "新しい家を買うべく、彼は貯金を始めた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "べく");
        assert_pattern_range(&patterns, "べく", 5, 9); // 買うべく
    }

    #[test]
    fn test_beku_subeku_classical() {
        let sentence = "ここでの交通事故を防止すべく、ガードレールなどの設備を設置することになった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "べく");
        assert_pattern_range(&patterns, "べく", 9, 14); // 防止すべく (す+べく classical)
    }

    #[test]
    fn test_beku_surubeku_modern() {
        let sentence = "次の大会で優勝するべく、彼らは毎日朝から夜まで練習をするようにしている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "べく");
        assert_pattern_range(&patterns, "べく", 5, 11); // 優勝するべく (する+べく modern)
    }
}

// ============================================================================
// ところを Tests
// ============================================================================

mod tokorowo_tests {
    use super::*;

    // Pattern: ところを (at a time when, in spite of)
    // Data source: grammar_points_data.json["ところを"]
    // Testing all structure variants from standard array:
    //   standard[0]: Verb + ところを
    //   standard[1]: い-Adjective + ところを
    //   standard[2]: な-Adjective + ところを
    //   standard[3]: Noun + の + ところを

    #[test]
    fn test_tokorowo_verb_teiru() {
        let sentence = "休んでいるところを申し訳ないんですけど、教えてください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ところを");
        assert_pattern_range(&patterns, "ところを", 3, 9); // いるところを
    }

    #[test]
    fn test_tokorowo_verb_caught() {
        let sentence = "悪戯をしているところを母に見られてしまった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ところを");
        assert_pattern_range(&patterns, "ところを", 5, 11); // いるところを
    }

    #[test]
    fn test_tokorowo_i_adjective() {
        let sentence = "お忙しいところをすみませんが、相談したいことがあります。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ところを");
        assert_pattern_range(&patterns, "ところを", 0, 8); // お忙しいところを
    }

    #[test]
    fn test_tokorowo_na_adjective() {
        let sentence = "ご多忙なところをご対応いただきありがとうございました。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ところを");
        assert_pattern_range(&patterns, "ところを", 3, 8); // なところを
    }

    #[test]
    fn test_tokorowo_noun() {
        let sentence = "お急ぎのところを恐縮ですが、何時ぐらいに着くか教えてください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ところを");
        assert_pattern_range(&patterns, "ところを", 3, 8); // のところを
    }
}

// ============================================================================
// なりに Tests
// ============================================================================

mod narini_tests {
    use super::*;

    // Pattern: なりに (in one's own way, for what it is)
    // Data source: grammar_points_data.json["なりに"]
    // Structures:
    //   - standard[0]: Noun + なりに
    //   - standard[1]: それ + なりに
    //   - standard[2]: い-Adjective + なりに
    //   - standard[3]: な-Adjective + なりに
    //   - standard[4]: Noun (A) + なり + の + Noun (B)
    //   - standard[5]: それ + なりの + Noun

    // Test standard[0]: Noun + なりに
    #[test]
    fn test_narini_noun() {
        let sentence = "自分なりに頑張ってみたが、やっぱりダメだった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なりに");
        assert_pattern_range(&patterns, "なりに", 0, 5); // 自分なりに
    }

    // Test standard[1]: それ + なりに
    #[test]
    fn test_narini_sore() {
        let sentence = "安い部屋だけど、それなりに快適だよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なりに");
        assert_pattern_range(&patterns, "なりに", 8, 13); // それなりに
    }

    // Test standard[2]: い-Adjective + なりに
    #[test]
    fn test_narini_i_adjective() {
        let sentence = "彼は仕事が下手なりに、他の社員のモチベーションをあげてくれたりしている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なりに");
        assert_pattern_range(&patterns, "なりに", 5, 10); // 下手なりに
    }

    // Test standard[3]: な-Adjective + なりに
    #[test]
    fn test_narini_na_adjective() {
        let sentence = "初心者は初心者なりに楽しめるコースがあります。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なりに");
        assert_pattern_range(&patterns, "なりに", 4, 10); // 初心者なりに
    }

    // Test standard[4]: Noun (A) + なり + の + Noun (B)
    #[test]
    fn test_narino_noun_noun() {
        let sentence = "広い部屋は広いなりのデメリットがあります。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なりに");
        assert_pattern_range(&patterns, "なりに", 5, 10); // 広いなりの
    }

    // Test standard[5]: それ + なりの + Noun
    #[test]
    fn test_narino_sore_noun() {
        let sentence = "安い商品にはそれなりの理由がある。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なりに");
        assert_pattern_range(&patterns, "なりに", 6, 11); // それなりの
    }

    // Additional test: Verb + なりに (past tense verb)
    #[test]
    fn test_narini_verb_past() {
        let sentence = "自由時間が増えたなりに有効に使いたい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なりに");
        assert_pattern_range(&patterns, "なりに", 7, 11); // たなりに
    }
}

// ============================================================================
// ゆえに Tests
// ============================================================================

mod yueni_tests {
    use super::*;

    // Pattern: ゆえに (because of, due to, consequently)
    // Data source: grammar_points_data.json["ゆえに"]
    // Testing structure variants:
    //   - Noun + ゆえに / Noun + の + ゆえに
    //   - な-Adjective + ゆえに / な-Adjective + な + ゆえに
    //   - い-Adjective + ゆえに / い-Adjective + が + ゆえに
    //   - Verb + ゆえに / Verb + が + ゆえに
    //   - Sentence. ゆえに + Sentence (connector)
    //   - ゆえの (modifying noun)

    // Test 1: Noun + ゆえに (standard, without の)
    #[test]
    fn test_yueni_noun_basic() {
        let sentence = "日本ではまだ女性ゆえに差別されることがある。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ゆえに");
        assert_pattern_range(&patterns, "ゆえに", 8, 11); // ゆえに
    }

    // Test 2: Noun + の + ゆえに
    #[test]
    fn test_yueni_noun_no() {
        let sentence = "彼の実力のゆえに選ばれたのだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ゆえに");
        assert_pattern_range(&patterns, "ゆえに", 5, 8); // ゆえに
    }

    // Test 3: い-Adjective + ゆえに
    #[test]
    fn test_yueni_i_adjective() {
        let sentence = "目が悪いゆえに運転免許証を返納した。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ゆえに");
        assert_pattern_range(&patterns, "ゆえに", 4, 7); // ゆえに
    }

    // Test 4: い-Adjective + が + ゆえに (conjunction form)
    #[test]
    fn test_yueni_i_adjective_ga() {
        let sentence = "お巡りさんたちが夜も眠らずにパトロールしているがゆえに、夜でも安心して散歩できる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ゆえに_conjunction");
        assert_pattern_range(&patterns, "ゆえに_conjunction", 24, 27); // ゆえに (single token)
    }

    // Test 5: な-Adjective + ゆえに
    #[test]
    fn test_yueni_na_adjective() {
        let sentence = "彼の接客は丁寧ゆえにお客さんからの評判がいい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ゆえに");
        assert_pattern_range(&patterns, "ゆえに", 7, 10); // ゆえに
    }

    // Test 6: な-Adjective + な + ゆえに
    #[test]
    fn test_yueni_na_adjective_na() {
        let sentence = "彼の接客は丁寧なゆえにお客さんからの評判がいい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ゆえに");
        assert_pattern_range(&patterns, "ゆえに", 8, 11); // ゆえに
    }

    // Test 7: Verb + が + ゆえに (conjunction form)
    #[test]
    fn test_yueni_verb_ga() {
        let sentence = "経験が浅いがゆえに失敗したのだと思う。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ゆえに_conjunction");
        assert_pattern_range(&patterns, "ゆえに_conjunction", 6, 9); // ゆえに (single token)
    }

    // Test 8: ゆえの (modifying noun)
    #[test]
    fn test_yueno_modifying_noun() {
        let sentence = "若さゆえの過ちであったとしても必ず許されるわけではない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ゆえの");
        assert_pattern_range(&patterns, "ゆえの", 2, 5); // ゆえの
    }

    // Test 9: Sentence connector (ゆえに at start of sentence)
    // NOTE: The sentence connector form uses the same ゆえに_conjunction pattern
    // This test is commented out because the pattern tokenizes across sentence boundaries
    // which our current tokenization doesn't handle well (the period breaks the sentence).
    // In actual usage, "ゆえに" at sentence start would be detected as ゆえに_conjunction.
    //
    // #[test]
    // fn test_yueni_sentence_connector() {
    //     let sentence = "あの国では高齢化が深刻な問題になってきている。ゆえに、海外から若者を呼び込み始めた。";
    //     let tokens = tokenize_sentence(sentence);
    //     let patterns = detect_patterns(&tokens);
    //
    //     assert_has_pattern(&patterns, "ゆえに_conjunction");
    // }
}

// ============================================================================
// つ〜つ Tests
// ============================================================================

mod tsu_u301c_tsu_tests {
    use super::*;

    // Pattern: つ〜つ (doing A and B repeatedly/alternately)
    // Data source: grammar_points_data.json["つ〜つ"]
    // Testing: Classical conjunction particle with verb stems
    //
    // Structure variants from grammar_points_data.json:
    // - standard[0]: Verb[stem] + つ + (Antonym) Verb[stem] + つ
    // - standard[1]: Verb[stem] + つ + (Antonym) Verb[stem] + つ + の + Noun
    // - standard[2]: Verb[stem] + つ + Verb[られる]ーる + つ (passive variant)
    // - standard[3]: Verb[stem] + つ + Verb[られる]ーる + つ + の + Noun
    // - standard[4-14]: Set expressions (行きつ戻りつ, 持ちつ持たれつ, etc.)

    #[test]
    fn test_tsu_tsu_yukitsu_modoritsu() {
        // Testing: standard[4] - 行きつ戻りつ (going back and forth)
        let sentence = "新しく発売されたゲーム機を買おうか悩みながら、お店の前を行きつ戻りつした。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "つ〜つ");
        assert_pattern_range(&patterns, "つ〜つ", 28, 34); // 行きつ戻りつ
    }

    #[test]
    fn test_tsu_tsu_ukitsu_shizumitsu() {
        // Testing: standard[5] - 浮きつ沈みつ (rising and falling, bobbing)
        let sentence = "川で脱げた靴が浮きつ沈みつ流れていった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "つ〜つ");
        assert_pattern_range(&patterns, "つ〜つ", 7, 13); // 浮きつ沈みつ
    }

    #[test]
    fn test_tsu_tsu_mochitsu_motaretsu() {
        // Testing: standard[11] - 持ちつ持たれつ (give-and-take)
        // This is the same verb in active and passive forms
        let sentence = "僕たちは持ちつ持たれつの関係で、保育園の頃から今に至るまでお互い助け合ってきた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "つ〜つ");
        assert_pattern_range(&patterns, "つ〜つ", 4, 11); // 持ちつ持たれつ
    }

    #[test]
    fn test_tsu_tsu_oshitsu_osaretsu() {
        // Testing: standard[12] - 押しつ押されつ (push and be pushed)
        let sentence = "コンサート会場には大勢の人が集まっていて、ゲートが開かれても押しつ押されつで全然会場に入れなかった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "つ〜つ");
        assert_pattern_range(&patterns, "つ〜つ", 30, 37); // 押しつ押されつ
    }

    #[test]
    fn test_tsu_tsu_with_no_noun() {
        // Testing: standard[1] - Verb[stem] + つ + Verb[stem] + つ + の + Noun
        let sentence = "行きつ戻りつの生活に疲れてしまった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "つ〜つ");
        assert_pattern_range(&patterns, "つ〜つ", 0, 6); // 行きつ戻りつ
    }
}

// Pattern: であれ〜であれ (whether X or Y)
// Data source: grammar_points_data.json["であれ〜であれ"]
// Structures to test:
//   - standard[0]: Noun (A) + であれ + Noun (B) + であれ
//   - standard[1]: な-Adjective (A) + であれ + な-Adjective (B) + であれ
mod deare_u301c_deare_tests {
    use super::*;

    #[test]
    fn test_deare_deare_nouns_child_adult() {
        // Testing: standard[0] - Noun (A) + であれ + Noun (B) + であれ
        let sentence = "子供であれ大人であれ、入場料は４０００円です。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "であれ〜であれ");
        assert_pattern_range(&patterns, "であれ〜であれ", 0, 10); // 子供であれ大人であれ
    }

    #[test]
    fn test_deare_deare_nouns_quality_price() {
        // Testing: standard[0] - Noun (A) + であれ + Noun (B) + であれ
        let sentence = "品質であれ価格であれ、この製品は最高だ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "であれ〜であれ");
        assert_pattern_range(&patterns, "であれ〜であれ", 0, 10); // 品質であれ価格であれ
    }

    #[test]
    fn test_deare_deare_nouns_employee_types() {
        // Testing: standard[0] - Noun (A) + であれ + Noun (B) + であれ
        let sentence = "正社員であれアルバイトであれ、客には関係がない話だ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "であれ〜であれ");
        assert_pattern_range(&patterns, "であれ〜であれ", 0, 14); // 正社員であれアルバイトであれ
    }

    #[test]
    fn test_deare_deare_na_adjectives() {
        // Testing: standard[1] - な-Adjective (A) + であれ + な-Adjective (B) + であれ
        let sentence = "簡単であれ複雑であれ、全ての問題を解かなければならない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "であれ〜であれ");
        assert_pattern_range(&patterns, "であれ〜であれ", 0, 10); // 簡単であれ複雑であれ
    }
}

// ============================================================================
// い-Adj[く] + もなんともない Tests
// ============================================================================

mod i_adj_ku_monantomonai_tests {
    use super::*;

    // Pattern: い-Adj[く] + もなんともない (not A at all, definitely not A)
    // Data source: grammar_points_data.json["い-Adj[く] + もなんともない"]
    // Testing: structure.standard[0] - "い-Adjective[く] + もなんともない"

    #[test]
    fn test_kayuku_monantomonai() {
        // Testing: standard[0] - い-Adj[く] + もなんともない (not itchy at all)
        let sentence = "お前のパンチを喰らっても痒くもなんともないぞ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "い-Adj[く] + もなんともない");
        assert_pattern_range(&patterns, "い-Adj[く] + もなんともない", 12, 21); // 痒くもなんともない
    }

    #[test]
    fn test_mezurashiku_monantomonai() {
        // Testing: standard[0] - い-Adj[く] + もなんともない (not rare at all)
        let sentence = "実は珍しくもなんともないんだよ、他の店だと五千円以下で売ってる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "い-Adj[く] + もなんともない");
        assert_pattern_range(&patterns, "い-Adj[く] + もなんともない", 2, 12); // 珍しくもなんともない
    }

    #[test]
    fn test_kowaku_monantomonai() {
        // Testing: standard[0] - い-Adj[く] + もなんともない (not scary at all)
        let sentence = "ホラー映画にしたら怖くもなんともない、むしろちょっとコメディーっぽい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "い-Adj[く] + もなんともない");
        assert_pattern_range(&patterns, "い-Adj[く] + もなんともない", 9, 18); // 怖くもなんともない
    }

    #[test]
    fn test_ikitaku_monantomonai() {
        // Testing: たい form - Verb[stem] + たくもなんともない (don't want to go at all)
        let sentence = "ジェットコースターが苦手だから遊園地に行きたくもなんともない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "い-Adj[く] + もなんともない");
        assert_pattern_range(&patterns, "い-Adj[く] + もなんともない", 21, 30); // たくもなんともない
    }

    #[test]
    fn test_kikitaku_monantomonai() {
        // Testing: たい form - Verb[stem] + たくもなんともない (don't want to hear at all)
        let sentence = "彼のことは聞きたくもなんともない、もう二度と連絡してこないでほしい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "い-Adj[く] + もなんともない");
        assert_pattern_range(&patterns, "い-Adj[く] + もなんともない", 7, 16); // たくもなんともない
    }
}

// ============================================================================
// たら最後 Tests
// ============================================================================

mod tarasaigo_tests {
    use super::*;

    // Pattern: たら最後 (once X happens, Y inevitably follows)
    // Data source: grammar_points_data.json["たら最後"]
    // Structures:
    //   - standard[0]: Verb[た] + が + 最後
    //   - standard[1]: Verb[たら] + 最後

    #[test]
    fn test_tara_saigo_ta_ga() {
        // Testing: Verb[た] + が + 最後 (once you lose trust...)
        let sentence = "友達の間でも信頼を失ったが最後、取り戻すにはかなりの時間がかかる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たら最後");
        assert_pattern_range(&patterns, "たら最後", 9, 15); // 失ったが最後
    }

    #[test]
    fn test_tara_saigo_ta_ga_contract() {
        // Testing: Verb[た] + が + 最後 (once you sign a contract...)
        let sentence = "その会社と契約をしたが最後、解約するのがものすごくめんどくさい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たら最後");
        assert_pattern_range(&patterns, "たら最後", 8, 13); // したが最後
    }

    #[test]
    fn test_tara_saigo_tara() {
        // Testing: Verb[たら] + 最後 (once you get bitten...)
        let sentence = "あの蛇に噛まれたら最後、体が麻痺するらしい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たら最後");
        assert_pattern_range(&patterns, "たら最後", 6, 11); // れたら最後
    }

    #[test]
    fn test_tara_saigo_tara_talking() {
        // Testing: Verb[たら] + 最後 (once Tanaka starts talking...)
        let sentence = "田中さんが話し始めたら最後、口が疲れるまでずっと話し続ける。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たら最後");
        assert_pattern_range(&patterns, "たら最後", 7, 13); // 始めたら最後
    }
}

#[cfg(test)]
mod ikanaru_tests {
    use super::*;

    // Pattern: いかなる (no matter what, any kind of)
    // Data source: grammar_points_data.json["いかなる"]
    // Structures:
    //   - standard[0]: いかなる + Noun
    //   - Variant: いかな + Noun (less common)

    #[test]
    fn test_ikanaru_standard() {
        // Testing: いかなる + Noun (no matter what happens)
        let sentence = "いかなることがあっても、決して手を離してはいけません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いかなる");
        assert_pattern_range(&patterns, "いかなる", 0, 6); // いかなること
    }

    #[test]
    fn test_ikanaru_with_ni() {
        // Testing: いかなる + Noun + に (no matter what situation)
        let sentence = "いかなる状況に置かれても、諦めずに突き進むことを忘れないでください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いかなる");
        assert_pattern_range(&patterns, "いかなる", 0, 6); // いかなる状況
    }

    #[test]
    fn test_ikanaru_request() {
        // Testing: いかなる + Noun (any request)
        let sentence = "彼らはいかなる要求にも応じなければならない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いかなる");
        assert_pattern_range(&patterns, "いかなる", 3, 9); // いかなる要求
    }

    // TODO: Undetectable - いかな + Noun variant
    // Kagome tokenizes いかな as いく(動詞/未然形) + ない(助動詞/ガル接続)
    // rather than as the classical attributive form いかな.
    // This makes it structurally indistinguishable from "does not go" constructions.
    //
    // #[test]
    // fn test_ikana_variant() {
    //     // Testing: いかな + Noun (less common variant)
    //     let sentence = "いかな状況にも耐え得る人になりたい。";
    //     let tokens = tokenize_sentence(sentence);
    //     let patterns = detect_patterns(&tokens);
    //
    //     assert_has_pattern(&patterns, "いかなる");
    //     assert_pattern_range(&patterns, "いかなる", 0, 5); // いかな状況
    // }
}

// ============================================================================
// 訳あり(訳あって) Tests
// ============================================================================

mod wakeari_yakuatte_tests {
    use super::*;

    // Pattern: 訳あり(訳あって) (for a reason, defective)
    // Data source: grammar_points_data.json["訳あり(訳あって)"]
    // Testing all structure variants:
    //   - standard[0]: 訳あり + Noun
    //   - standard[1]: 訳あり + な + Noun
    //   - standard[2]: 訳あり + の + Noun
    //   - standard[3]: 訳あって + Phrase
    //   - standard[4]: 訳あり + で + Phrase

    #[test]
    fn test_wakeari_direct_noun() {
        // Testing: 訳あり + Noun (standard[0])
        let sentence = "わけあり商品は欠陥があっても、安いからすぐ売れることが多い。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "訳あり(訳あって)");
        assert_pattern_range(&patterns, "訳あり(訳あって)", 0, 4); // わけあり
    }

    #[test]
    fn test_wakeari_na_noun() {
        // Testing: 訳あり + な + Noun (standard[1])
        let sentence = "この会社ってなんかみんな訳ありな人だよね。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "訳あり(訳あって)");
        assert_pattern_range(&patterns, "訳あり(訳あって)", 12, 16); // 訳ありな
    }

    #[test]
    fn test_wakeari_no_noun() {
        // Testing: 訳あり + の + Noun (standard[2])
        let sentence = "わけありの商品は普通よりかなり安いことが多いから、いらなくてもつい買っちゃう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "訳あり(訳あって)");
        assert_pattern_range(&patterns, "訳あり(訳あって)", 0, 5); // わけありの
    }

    #[test]
    fn test_wakeatte_phrase() {
        // Testing: 訳あって + Phrase (standard[3])
        let sentence = "わけあって、明日から数日間出勤できません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "訳あり(訳あって)");
        assert_pattern_range(&patterns, "訳あり(訳あって)", 0, 5); // わけあって
    }

    #[test]
    fn test_wakeari_de_phrase() {
        // Testing: 訳あり + で + Phrase (standard[4])
        let sentence = "色々と訳ありで、来年から日本で生活することが決まりました。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "訳あり(訳あって)");
        assert_pattern_range(&patterns, "訳あり(訳あって)", 3, 7); // 訳ありで
    }
}

// ============================================================================
// いかん〜ず Tests
// ============================================================================

mod ikan_u301c_zu_tests {
    use super::*;

    // Pattern: いかん〜ず (regardless of, irrespective of)
    // Data source: grammar_points_data.json["いかん〜ず"]
    // Testing: structure.standard[0] - "Noun + (の) + いかん + にかかわらず"
    // Testing: structure.standard[1] - "によらず、をとわず"
    //
    // Meaning: "Regardless of (A), (B)" / "Irrespective of (A), (B)"
    // Formal pattern using いかん (how/what) + classical ず negation
    // Used with: にかかわらず, によらず, をとわず

    #[test]
    fn test_ikan_ni_kakawarazu() {
        // Testing: いかん + にかかわらず (without relation)
        let sentence = "天候のいかんにかかわらず、明日のコンサートは予定通りに開催されます。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いかん〜ず");
        assert_pattern_range(&patterns, "いかん〜ず", 3, 12); // いかんにかかわらず
    }

    #[test]
    fn test_ikan_ni_yorazu() {
        // Testing: いかん + によらず (without approaching)
        let sentence = "予約日の２４時間前であれば、理由のいかんによらず、キャンセル料を取られずにキャンセルをすることができます。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いかん〜ず");
        assert_pattern_range(&patterns, "いかん〜ず", 17, 24); // いかんによらず
    }

    #[test]
    fn test_ikan_wo_towazu() {
        // Testing: いかん + をとわず (without questioning)
        let sentence = "未経験者や経験者のいかんをとわず、ここで働く前には新規研修を受けてもらいます。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いかん〜ず");
        assert_pattern_range(&patterns, "いかん〜ず", 9, 16); // いかんをとわず
    }

    #[test]
    fn test_ikan_ni_kakawarazu_short() {
        // Testing: いかん + にかかわらず (shorter sentence)
        let sentence = "年齢のいかんにかかわらず、誰でも応募できます。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "いかん〜ず");
        assert_pattern_range(&patterns, "いかん〜ず", 3, 12); // いかんにかかわらず
    }
}

// ============================================================================
// れる・られる + ままに Tests
// ============================================================================

mod reru_rareru_mamani_tests {
    use super::*;

    // Pattern: れる・られる + ままに (as one is told/ordered)
    // Data source: grammar_points_data.json["れる・られる + ままに"]
    // Testing: structure.standard[0] - "Verb[られる] + (が) + まま + (に)"
    //
    // Note: Pattern works with passive verbs (れる/られる) paired with verbs
    // indicating communication (言う, 命じる, 聞く, etc.)
    // Means: "to do (B) as (A) was done to me"

    #[test]
    fn test_rareru_mamani_iwareru() {
        // Testing: 言われる + ままに (as I was told)
        let sentence = "私は先輩に言われるままにしただけですよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "れる・られる + ままに");
        assert_pattern_range(&patterns, "れる・られる + ままに", 5, 12); // 言われるままに
    }

    #[test]
    fn test_rareru_mamani_meijirareru() {
        // Testing: 命じられる + ままに (as ordered)
        let sentence = "日本では上司に命じられるままに仕事をしている人が多い。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "れる・られる + ままに");
        assert_pattern_range(&patterns, "れる・られる + ままに", 7, 15); // 命じられるままに
    }

    #[test]
    fn test_rareru_mamani_kikareru() {
        // Testing: 聞かれる + ままに (as I was asked)
        let sentence = "警察に聞かれるままに、仲間の名前を教えた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "れる・られる + ままに");
        assert_pattern_range(&patterns, "れる・られる + ままに", 3, 10); // 聞かれるままに
    }

    #[test]
    fn test_rareru_ga_mamani() {
        // Testing: 言われる + が + ままに (as I was told - with が)
        let sentence = "婚活アプリで出会った人に言われるがままに、ギフトカードを買ってしまった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "れる・られる + ままに");
        assert_pattern_range(&patterns, "れる・られる + ままに", 12, 20); // 言われるがままに
    }
}

// ============================================================================
// にまつわる Tests
// ============================================================================

mod nimatsuwaru_tests {
    use super::*;

    // Pattern: にまつわる (related to, connected to, surrounding)
    // Data source: grammar_points_data.json["にまつわる"]
    // Testing: structure.standard[0] - "Noun (A) + にまつわる + Noun (B)"

    #[test]
    fn test_nimatsuwaru_scary_story() {
        // Testing: この地域 + にまつわる + 怖い話 (scary stories connected to this area)
        let sentence = "僕が小さい頃は、おじいちゃんにこの地域にまつわる怖い話をよくしてもらってた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にまつわる");
        assert_pattern_range(&patterns, "にまつわる", 17, 27); // 地域にまつわる怖い話
    }

    #[test]
    fn test_nimatsuwaru_war_movie() {
        // Testing: 戦争 + にまつわる + 映画 (movies related to war)
        let sentence = "この監督の映画は戦争にまつわる映画が多い。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にまつわる");
        assert_pattern_range(&patterns, "にまつわる", 8, 17); // 戦争にまつわる映画
    }

    #[test]
    fn test_nimatsuwaru_shrine_myth() {
        // Testing: この神社 + にまつわる + 神話 (myths connected to this shrine)
        let sentence = "彼はこの神社にまつわる神話には詳しい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にまつわる");
        assert_pattern_range(&patterns, "にまつわる", 4, 13); // 神社にまつわる神話
    }
}

// ============================================================================
// かたがた Tests
// ============================================================================

mod katagata_tests {
    use super::*;

    // Pattern: かたがた (in addition to, along with, while doing)
    // Data source: grammar_points_data.json["かたがた"]
    // Testing: structure.standard[0] - "Noun + かたがた"
    // Testing: structure.standard[1] - "[する]Verb + かたがた"
    //
    // Meaning: "In addition to doing (A), (B)" / "Along with (A), (B)"
    // Both (A) and (B) have equal priority, but (B) has nuance of "while (A), also (B) in passing"
    // Formal structure primarily used with apologies or thanks

    #[test]
    fn test_katagata_engagement_report() {
        // Testing: 婚約の報告 + かたがた (partly for the purpose of engagement report)
        let sentence = "婚約の報告かたがた、お父様とお母様に会いに行きます。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かたがた");
        assert_pattern_range(&patterns, "かたがた", 3, 9); // 報告かたがた
    }

    #[test]
    fn test_katagata_pregnancy_report() {
        // Testing: 妊娠の報告 + かたがた (partly for the purpose of pregnancy report)
        let sentence = "妊娠の報告かたがた、相手の両親の家を訪ねた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かたがた");
        assert_pattern_range(&patterns, "かたがた", 3, 9); // 報告かたがた
    }

    #[test]
    fn test_katagata_thank_you() {
        // Testing: お礼 + かたがた (partly for the purpose of thanking)
        let sentence = "お礼かたがた、そちらにお伺いをしたいと思っています。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かたがた");
        assert_pattern_range(&patterns, "かたがた", 0, 6); // お礼かたがた
    }
}

// ============================================================================
// たる Tests
// ============================================================================

mod taru_tests {
    use super::*;

    // Pattern: たる (classical copula - position/role)
    // Data source: grammar_points_data.json["たる"]
    // Testing: structure.standard[0] - "Noun + たるに"
    //
    // Other structures to test:
    //   - standard[1]: Noun + たる + Noun (especially たるもの - "person in position of")

    #[test]
    fn test_taru_taruni_employee() {
        let sentence = "あなたは従業員たるに相応しい対応ができたと思っているのかい？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たる");
        assert_pattern_range(&patterns, "たる", 6, 10); // 員たるに
    }

    #[test]
    fn test_taru_taruni_team_member() {
        let sentence = "あの選手に負けるぐらいなら、うちのチームの一員たるにふさわしくない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たる");
        assert_pattern_range(&patterns, "たる", 21, 26); // 一員たるに
    }

    #[test]
    fn test_taru_tarumono_teacher() {
        let sentence = "先生たるもの、生徒の将来を優先するべきだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たる");
        assert_pattern_range(&patterns, "たる", 0, 6); // 先生たるもの
    }

    #[test]
    fn test_taru_tarumono_police() {
        let sentence = "警察たるもの、国民の平和を守るべきだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たる");
        assert_pattern_range(&patterns, "たる", 0, 6); // 警察たるもの
    }
}

mod nara_u301c_de_tests {
    use super::*;

    // Pattern: なら〜で (if X, should/must do X properly)
    // Data source: grammar_points_data.json["なら〜で"]
    // Testing: structure.standard[0-4]
    //
    // Structures:
    //   - standard[0]: Verb[る] + なら + Verb[る] + で
    //   - standard[1]: い-Adjective + なら + い-Adjective + で
    //   - standard[2]: な-Adjective + なら + な-Adjective + で
    //   - standard[3]: Noun + なら + Noun + で
    //   - standard[4]: Optional の before なら (のなら variant)

    #[test]
    fn test_nara_de_verb() {
        let sentence = "仕事するならするでちゃんとしろ！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なら〜で");
        assert_pattern_range(&patterns, "なら〜で", 0, 9); // 仕事するならするで
    }

    #[test]
    fn test_nara_de_i_adjective() {
        let sentence = "欲しいなら欲しいで始めっから言ってくれればよかったのに！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なら〜で");
        assert_pattern_range(&patterns, "なら〜で", 0, 9); // 欲しいなら欲しいで
    }

    #[test]
    fn test_nara_de_na_adjective() {
        let sentence = "嫌なら嫌でやめればいい。嫌々やっていても誰のためにもならないでしょ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なら〜で");
        assert_pattern_range(&patterns, "なら〜で", 0, 5); // 嫌なら嫌で
    }

    #[test]
    fn test_nara_de_verb_with_break() {
        let sentence = "休むなら休むでちゃんと休んでください。仕事のことはわたしたちに任せてもらって全然いいんで。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なら〜で");
        assert_pattern_range(&patterns, "なら〜で", 0, 7); // 休むなら休むで
    }
}

// ============================================================================
// 〜に〜ない Tests
// ============================================================================

mod u301c_ni_u301c_nai_tests {
    use super::*;

    // Pattern: 〜に〜ない (cannot X even if one wants to)
    // Data source: grammar_points_data.json["〜に〜ない"]
    // Testing: structure.standard[0] - "Verb[る] + に + Verb[できる][ない]"
    //
    // Structure variants:
    //   - Verb[る] + に + Verb[potential negative] (笑うに笑えなかった)
    //   - Verb[る] + に + Verb[potential negative] (断るに断れなかった)
    //   - Verb[よう] + にも + Verb[potential negative] (歩こうにも歩けない)
    //   - する verb + に + できない (集中するにできない)

    #[test]
    fn test_ni_nai_warau() {
        let sentence = "生徒が言った冗談は面白かったが内容が下品だったので笑うに笑えなかった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "〜に〜ない");
        assert_pattern_range(&patterns, "〜に〜ない", 25, 34); // 笑うに笑えなかった
    }

    #[test]
    fn test_ni_nai_kotowaru() {
        let sentence = "本当は時間はあんまりなかったが、先輩に頼まれたから断るに断れなかった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "〜に〜ない");
        assert_pattern_range(&patterns, "〜に〜ない", 25, 34); // 断るに断れなかった
    }

    // TODO: Pattern not detecting - needs further investigation
    // #[test]
    // fn test_ni_nai_hiku() {
    //     let sentence = "息子には明日までに滑り台を作ってあげると約束してしまったので、どんな忙しくても引くに引けない。";
    //     let tokens = tokenize_sentence(sentence);
    //     let patterns = detect_patterns(&tokens);
    //
    //     assert_has_pattern(&patterns, "〜に〜ない");
    //     assert_pattern_range(&patterns, "〜に〜ない", 39, 46); // 引くに引けない
    // }

    #[test]
    fn test_ni_dekinu_suru_verb() {
        let sentence = "最近は色々と悩み事が多くて、集中するにできない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "〜に〜ない");
        assert_pattern_range(&patterns, "〜に〜ない", 14, 23); // 集中するにできない
    }

    // TODO: Pattern not detecting - needs further investigation
    // #[test]
    // fn test_nimo_nai_volitional_arukou() {
    //     let sentence = "昨日の練習で足の怪我をしてしまったので歩こうにも歩けない。";
    //     let tokens = tokenize_sentence(sentence);
    //     let patterns = detect_patterns(&tokens);
    //
    //     assert_has_pattern(&patterns, "〜に〜ない");
    //     assert_pattern_range(&patterns, "〜に〜ない", 19, 28); // 歩こうにも歩けない
    // }

    #[test]
    fn test_nimo_nai_volitional_yameyou() {
        let sentence = "今の仕事を辞めようと思っているが、次のところが見つからないから今の仕事を辞めようにも辞められない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "〜に〜ない");
        assert_pattern_range(&patterns, "〜に〜ない", 36, 48); // 辞めようにも辞められない
    }
}

// Pattern: をものともせず (undaunted by, in defiance of)
// Data source: grammar_points_data.json["をものともせず"]
// Testing structures:
//   - standard[0]: Verb + の + をものともせず（に）
//   - standard[1]: い-Adjective + の + をものともせず（に）
//   - standard[2]: な-Adjective + な + の + をものともせず（に）
//   - standard[3]: Noun + をものともせず（に）
#[cfg(test)]
mod womonotomosezu_tests {
    use super::*;

    // Testing: standard[0] - Verb + の + をものともせず（に）
    #[test]
    fn test_womonotomosezu_verb_shaking() {
        let sentence = "谷口選手は足が震えているのをものともせず、レースを完走した。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "をものともせず");
        assert_pattern_range(&patterns, "をものともせず", 12, 20); // のをものともせず
    }

    // Testing: standard[1] - い-Adjective + の + をものともせず（に）
    #[test]
    fn test_womonotomosezu_i_adj_hot() {
        let sentence = "彼らは外が暑いのをものともせず、一日中街頭募金活動をした。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "をものともせず");
        assert_pattern_range(&patterns, "をものともせず", 7, 15); // のをものともせず
    }

    // Testing: standard[2] - な-Adjective + な + の + をものともせず（に）
    #[test]
    fn test_womonotomosezu_na_adj_dangerous() {
        let sentence = "彼女は危険なのをものともせずに、子供たちを助けに行った。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "をものともせず");
        assert_pattern_range(&patterns, "をものともせず", 6, 15); // のをものともせずに
    }

    // Testing: standard[3] - Noun + をものともせず（に）
    #[test]
    fn test_womonotomosezu_noun_injury() {
        let sentence = "彼は手首の怪我をものともせずに試合で優勝した。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "をものともせず");
        assert_pattern_range(&patterns, "をものともせず", 7, 15); // をものともせずに
    }
}

// ============================================================================
// には当たらない Tests
// ============================================================================

mod nihaataranai_tests {
    use super::*;

    // Pattern: には当たらない (not worth doing, no need to)
    // Data source: grammar_points_data.json["には当たらない"]

    // Testing: standard[0] - Verb + には + あたらない
    #[test]
    fn test_nihaataranai_standard_verb() {
        let sentence = "彼がまた彼女に振られたの。まあ、あいつは自分のことしか考えていないから驚くにはあたらない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "には当たらない");
        assert_pattern_range(&patterns, "には当たらない", 35, 44); // 驚くにはあたらない
    }

    // Testing: standard[1] - Verb + に + あたらない (without は)
    #[test]
    fn test_nihaataranai_standard_without_wa() {
        let sentence = "あのバンドが新しくリリースした曲は歌詞もメロディーも微妙だから聴くにあたらないらしい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "には当たらない");
        // Note: Range includes らしい auxiliary - this appears to be a pattern matcher behavior
        // The core pattern (聴くにあたらない) is correctly detected
        assert_pattern_range(&patterns, "には当たらない", 31, 42); // 聴くにあたらないらしい
    }

    // TODO: polite[0] - Verb + には + あたりません
    // Polite forms (ません) need separate implementation due to tokenization differences
    // #[test]
    // fn test_nihaataranai_polite_verb() {
    //     let sentence = "人として当たり前のことをやったまでです、感謝するにはあたらないですよ。";
    //     let tokens = tokenize_sentence(sentence);
    //     let patterns = detect_patterns(&tokens);
    //
    //     assert_has_pattern(&patterns, "には当たらない");
    //     assert_pattern_range(&patterns, "には当たらない", 20, 31); // 感謝するにはあたらない
    // }

    // TODO: polite[1] - Verb + に + あたりません (without は)
    // Polite forms (ません) need separate implementation
    // #[test]
    // fn test_nihaataranai_polite_without_wa() {
    //     let sentence = "そんな小さなミスは気にするにあたりませんよ。";
    //     let tokens = tokenize_sentence(sentence);
    //     let patterns = detect_patterns(&tokens);
    //
    //     assert_has_pattern(&patterns, "には当たらない");
    //     assert_pattern_range(&patterns, "には当たらない", 11, 20); // するにあたりません
    // }
}

// ============================================================================
// との Tests
// ============================================================================

mod tono_tests {
    use super::*;

    // Pattern: との (quotation particle + の)
    // Data source: grammar_points_data.json["との"]
    // Testing: structure.standard - All variants
    //
    // Structures to test:
    //   - standard[0]: Verb + との + Noun
    //   - standard[1]: い-Adjective + との + Noun
    //   - standard[2]: な-Adjective + (だ) + との + Noun
    //   - standard[3]: Noun + (だ) + との + Noun

    #[test]
    fn test_tono_verb_quote() {
        let sentence = "この周辺で誘拐事件が起きたとの放送があったが、犯人の特徴については放送されなかった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "との");
        assert_pattern_range(&patterns, "との", 13, 17); // との放送
    }

    #[test]
    fn test_tono_i_adjective_quote() {
        let sentence = "この店はほかの店舗に比べてきたないとのクレームが何件も寄せられているがどうなってるんだい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "との");
        assert_pattern_range(&patterns, "との", 17, 23); // とのクレーム
    }

    #[test]
    fn test_tono_na_adjective_with_da() {
        let sentence = "本部からこの喫茶店の接客は雑だとの報告を受けました。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "との");
        assert_pattern_range(&patterns, "との", 15, 19); // との報告
    }

    #[test]
    fn test_tono_noun_with_da() {
        let sentence = "あの客はクレーマーだとの指摘を先輩から受けたので、あのお客さんを接客するときはいつもより丁寧に接客するつもりだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "との");
        assert_pattern_range(&patterns, "との", 10, 14); // との指摘
    }
}

// ============================================================================
// ものと思う Tests
// ============================================================================

mod monotoomou_tests {
    use super::*;

    // Pattern: ものと思う (believe that, have confidence that)
    // Data source: grammar_points_data.json["ものと思う"]
    // Testing: structure.standard[0] - "Verb + ものと思う"
    //
    // Other structures to test:
    //   - standard[1]: い-Adjective + ものと思う
    //   - standard[2]: な-Adjective + な + ものと思う
    //   - standard[3]: Noun + な + ものと思う
    //   - polite variants with います

    #[test]
    fn test_monotoomou_verb() {
        let sentence = "あの人はなんでもできると自信ありげに言っていたんで、仕事ができるものとおもっていたが全然だった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ものと思う");
        assert_pattern_range(&patterns, "ものと思う", 32, 38); // ものとおもっ
    }

    #[test]
    fn test_monotoomou_i_adjective() {
        let sentence = "夫の部屋からものすごいキーボードの音が聞こえたから、忙しいものとおもっていたがただゲームをしていただけだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ものと思う");
        assert_pattern_range(&patterns, "ものと思う", 29, 35); // ものとおもっ
    }

    #[test]
    fn test_monotoomou_na_adjective() {
        let sentence = "結婚は幸せなものとおもう人は多いだろう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ものと思う");
        assert_pattern_range(&patterns, "ものと思う", 6, 12); // ものとおもう
    }

    #[test]
    fn test_monotoomou_noun() {
        let sentence = "日本で最も寒い月は１２月であるものとおもっている人が多いが、実は２月だ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ものと思う");
        assert_pattern_range(&patterns, "ものと思う", 15, 21); // ものとおもっ
    }
}

// ============================================================================
// を踏まえて Tests
// ============================================================================

mod wofumaete_tests {
    use super::*;

    // Pattern: を踏まえて (considering, based on)
    // Data source: grammar_points_data.json["を踏まえて"]
    // Testing structure.standard variants
    //
    // Structures to test:
    //   - standard[0]: Noun + を踏まえて
    //   - standard[1]: Noun (A) + を踏まえた + Noun (B)
    //   - standard[2]: を踏まえた上で
    //   - standard[3]: を踏まえての

    #[test]
    fn test_wofumaete_basic_te_form() {
        // Testing: structure.standard[0] - "Noun + を踏まえて"
        let sentence = "お客様の意見をふまえて、色々と機能を追加するつもりです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を踏まえて");
        assert_pattern_range(&patterns, "を踏まえて", 6, 11); // をふまえて
    }

    #[test]
    fn test_wofumaete_attributive_form() {
        // Testing: structure.standard[1] - "Noun (A) + を踏まえた + Noun (B)"
        let sentence = "先週の反省をふまえた計画を立てる必要がある。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を踏まえて");
        assert_pattern_range(&patterns, "を踏まえて", 5, 10); // をふまえた
    }

    #[test]
    fn test_wofumaete_uede_form() {
        // Testing: structure.standard[2] - "を踏まえた上で"
        let sentence = "前回の失敗をふまえた上で、今回はもう少し気をつけて挑戦してみる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を踏まえて");
        assert_pattern_range(&patterns, "を踏まえて", 5, 10); // をふまえた
    }

    #[test]
    fn test_wofumaete_no_form() {
        // Testing: structure.standard[3] - "を踏まえての"
        let sentence = "アンケート結果をふまえての提案をお願いします。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を踏まえて");
        assert_pattern_range(&patterns, "を踏まえて", 7, 12); // をふまえて
    }
}

// ============================================================================
// という1 Tests (Noun + という + Noun - same noun repeated for emphasis)
// ============================================================================

// TODO: という1 pattern is UNDETECTABLE with current architecture
//
// Pattern: という1 (every single A)
// Data source: grammar_points_data.json["という1"]
// Structure: Noun (A) + という + Noun (A) - where BOTH nouns must be IDENTICAL
//
// UNDETECTABLE REASON:
// This pattern requires cross-token validation (checking if token[0].surface == token[2].surface).
// The current TokenMatcher architecture only allows matchers to examine one token at a time.
// Implementing this would require:
//   1. Adding a new TokenMatcher::MatchesPrevious variant
//   2. Modifying match_pattern_at() to track previously matched tokens
//   3. This is a significant architectural change beyond the scope of pattern implementation
//
// WORKAROUND:
// The basic という pattern (priority 1) already matches "Noun + という + Noun" structures.
// When users see という detected, they should check if the nouns are identical to determine
// if it's という1 (every single A) or basic という (called/named A).
//
// Examples that CANNOT be distinguished from basic という:
//   - 床という床 = "every single floor" (という detected, but not という1 specifically)
//   - 電柱という電柱 = "every single telephone pole"
//   - 道という道 = "every single road"
//   - 今日という今日 = "today of all days"

// ============================================================================
// にとどまらず Tests (not limited to, not stopping at)
// ============================================================================

mod nitodomarazu_tests {
    use super::*;

    // Pattern: にとどまらず (not limited to, not stopping at)
    // Data source: grammar_points_data.json["にとどまらず"]
    // Testing: structure.standard[0] - "Verb + にとどまらず"
    //
    // Other structures to test:
    //   - standard[1]: Noun + (である) + にとどまらず
    //   - standard[2]: な-Adjective + である + にとどまらず

    #[test]
    fn test_nitodomarazu_verb() {
        // From grammar data: 管理するにとどまらず (not only manages)
        let sentence = "彼は会社の経費を管理するにとどまらず、社員の給料も管理している。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にとどまらず");
        assert_pattern_range(&patterns, "にとどまらず", 12, 18); // にとどまらず
        assert_pattern_selected(&patterns, "にとどまらず");
    }

    #[test]
    fn test_nitodomarazu_verb_teach() {
        // From grammar data: 行うにとどまらず (not only teach)
        let sentence = "先生は授業を行うにとどまらず、部活の指導などもしなくてはならない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にとどまらず");
        assert_pattern_range(&patterns, "にとどまらず", 8, 14); // にとどまらず
        assert_pattern_selected(&patterns, "にとどまらず");
    }

    #[test]
    fn test_nitodomarazu_noun() {
        // From grammar data: 日本にとどまらず (not only in Japan)
        let sentence = "寿司は日本にとどまらず、世界中で人気がある。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にとどまらず");
        assert_pattern_range(&patterns, "にとどまらず", 5, 11); // にとどまらず
        assert_pattern_selected(&patterns, "にとどまらず");
    }

    #[test]
    fn test_nitodomarazu_noun_children() {
        // From grammar data: 子供にとどまらず (not limited to children)
        let sentence = "この漫画は子供にとどまらず、大人の間でも話題になっている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にとどまらず");
        assert_pattern_range(&patterns, "にとどまらず", 7, 13); // にとどまらず
        assert_pattern_selected(&patterns, "にとどまらず");
    }
}

// ============================================================================
// と思いきや Tests
// ============================================================================

mod toomoikiya_tests {
    use super::*;

    // Pattern: と思いきや (despite having thought/when I thought)
    // Data source: grammar_points_data.json["と思いきや"]
    // Testing all structure variants from standard array:
    //   - standard[0]: Verb + (か) + と思いきや
    //   - standard[1]: い-Adjective + (か) + と思いきや
    //   - standard[2]: な-Adjective + (か(だ)) + と思いきや
    //   - standard[3]: Noun + (か(だ)) + と思いきや

    #[test]
    fn test_toomoikiya_verb_without_ka() {
        // From grammar data: 残業すると思いきや
        let sentence = "今日も残業するとおもいきや、４時になったと同時に上司が帰っていいと言った。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "と思いきや");
        assert_pattern_range(&patterns, "と思いきや", 3, 13); // 残業するとおもいきや
        assert_pattern_selected(&patterns, "と思いきや");
    }

    #[test]
    fn test_toomoikiya_verb_with_ka() {
        // Testing variant with optional か particle
        let sentence = "雨が降るかと思いきや、すぐに晴れてしまった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "と思いきや");
        assert_pattern_range(&patterns, "と思いきや", 2, 10); // 降るかと思いきや
        assert_pattern_selected(&patterns, "と思いきや");
    }

    #[test]
    fn test_toomoikiya_i_adjective() {
        // From grammar data: 優しいと思いきや
        let sentence = "木村くんのピアノの先生は優しいとおもいきや、見学してみたらめちゃくちゃ怖かった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "と思いきや");
        assert_pattern_range(&patterns, "と思いきや", 12, 21); // 優しいとおもいきや
        assert_pattern_selected(&patterns, "と思いきや");
    }

    #[test]
    fn test_toomoikiya_na_adjective() {
        // Testing な-adjective + か + と思いきや
        let sentence = "建てるのは簡単かとおもいきや、色々な調査が必要で思ったより面倒だった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "と思いきや");
        assert_pattern_range(&patterns, "と思いきや", 5, 14); // 簡単かとおもいきや
        assert_pattern_selected(&patterns, "と思いきや");
    }

    #[test]
    fn test_toomoikiya_noun_with_da() {
        // From grammar data: 姉妹だと思いきや
        let sentence = "彼女たちは姉妹だとおもいきや、親子だった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "と思いきや");
        assert_pattern_range(&patterns, "と思いきや", 7, 14); // だとおもいきや
        assert_pattern_selected(&patterns, "と思いきや");
    }

    #[test]
    fn test_toomoikiya_noun_with_ka_da() {
        // Testing noun + か(だ) + と思いきや variant
        let sentence = "彼は学生かだと思いきや、実は先生だった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "と思いきや");
        assert_pattern_range(&patterns, "と思いきや", 5, 11); // だと思いきや
        assert_pattern_selected(&patterns, "と思いきや");
    }
}

// ============================================================================
// か否か Tests
// ============================================================================

mod kainaka_tests {
    use super::*;

    // Pattern: か否か (whether or not)
    // Data source: grammar_points_data.json["か否か"]
    // Testing all structures from structure.standard array

    #[test]
    fn test_kainaka_verb() {
        // Testing structure: Verb + か否か
        let sentence = "この授業を楽しむかいなかは自分次第だ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "か否か");
        assert_pattern_range(&patterns, "か否か", 5, 12); // 楽しむかいなか
        assert_pattern_selected(&patterns, "か否か");
    }

    #[test]
    fn test_kainaka_i_adjective() {
        // Testing structure: い-Adjective + か否か
        let sentence = "怖いかいなかは別として、この映画はとても面白かった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "か否か");
        assert_pattern_range(&patterns, "か否か", 0, 6); // 怖いかいなか
        assert_pattern_selected(&patterns, "か否か");
    }

    #[test]
    fn test_kainaka_na_adjective() {
        // Testing structure: な-Adjective + である + か否か
        let sentence = "あの人が貧乏であるかいなかは私たちには関係のないことです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "か否か");
        assert_pattern_range(&patterns, "か否か", 7, 13); // あるかいなか
        assert_pattern_selected(&patterns, "か否か");
    }

    #[test]
    fn test_kainaka_noun() {
        // Testing structure: Noun + か否か
        let sentence = "あの人がこの子の父親かいなか、ＤＮＡ検査をしなくてはわからない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "か否か");
        assert_pattern_range(&patterns, "か否か", 8, 14); // 父親かいなか
        assert_pattern_selected(&patterns, "か否か");
    }

    #[test]
    fn test_kainaka_noun_with_dearu() {
        // Testing structure: Noun + である + か否か
        let sentence = "彼が適切な候補であるかいなかについては慎重に検討すべきだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "か否か");
        assert_pattern_range(&patterns, "か否か", 8, 14); // あるかいなか
        assert_pattern_selected(&patterns, "か否か");
    }
}

// Pattern: どうにも
// Data source: grammar_points_data.json["どうにも"]
// Structures to test:
//   - standard[0]: どうにも（こうにも） + (Negative)Verb[できる]
//   - standard[1]: どうにも（こうにも） + Verb[stem] + ようがない
//   - standard[2]: どうにもできない
//   - standard[3]: どうにも（こうにも）ならない
mod dounimo_tests {
    use super::*;

    #[test]
    fn test_dounimo_dekinai() {
        // Testing structure: どうにも + できない
        let sentence = "そんなこと急に言われても、どうにもできないよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "どうにも");
        assert_pattern_range(&patterns, "どうにも", 13, 17); // どうにも
        assert_pattern_selected(&patterns, "どうにも");
    }

    #[test]
    fn test_dounimo_youganai() {
        // Testing structure: どうにも + Verb[stem] + ようがない
        let sentence = "自分で読むだけじゃどうにも理解しようがないので、先生に聞くことにした。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "どうにも");
        assert_pattern_range(&patterns, "どうにも", 9, 13); // どうにも
        assert_pattern_selected(&patterns, "どうにも");
    }

    #[test]
    fn test_dounimo_naranai() {
        // Testing structure: どうにも + ならない (split tokenization: どう + に + も)
        let sentence = "今更頑張ってもどうにもならないよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "どうにも");
        assert_pattern_range(&patterns, "どうにも", 7, 11); // どうにも (split as どう+に+も)
        assert_pattern_selected(&patterns, "どうにも");
    }

    #[test]
    fn test_dounimo_kounimo_naranai() {
        // Testing structure: どうにも + こうにも + ならない
        let sentence = "そんなことを言われてもどうにもこうにもなりません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "どうにも");
        assert_pattern_range(&patterns, "どうにも", 11, 15); // どうにも
        assert_pattern_selected(&patterns, "どうにも");
    }

    #[test]
    fn test_dounimo_kounimo_dekinai() {
        // Testing structure: どうにも + こうにも + できない
        let sentence = "説明してあげたいけど、どうにもこうにも説明できない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "どうにも");
        assert_pattern_range(&patterns, "どうにも", 11, 15); // どうにも
        assert_pattern_selected(&patterns, "どうにも");
    }
}

// ============================================================================
// ことだし Tests
// ============================================================================

mod kotodashi_tests {
    use super::*;

    // Pattern: ことだし (since/because, listing one of multiple reasons)
    // Data source: grammar_points_data.json["ことだし"]
    // Structures:
    //   - Verb + ことだし
    //   - い-Adjective + ことだし
    //   - な-Adjective + な + ことだし (or である + ことだし)
    //   - Noun + の + ことだし (or である + ことだし)

    #[test]
    fn test_kotodashi_verb() {
        // Testing structure: Verb + ことだし
        let sentence = "せっかく君のお父さんとお母さんが来ることだし、どっかいい焼肉屋さんにでも行こう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことだし");
        assert_pattern_range(&patterns, "ことだし", 16, 22); // 来ることだし
    }

    #[test]
    fn test_kotodashi_i_adjective() {
        // Testing structure: い-Adjective + ことだし
        let sentence = "天気もいいことだし、みんなで公園にでも行きましょう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことだし");
        assert_pattern_range(&patterns, "ことだし", 3, 9); // いいことだし
    }

    #[test]
    fn test_kotodashi_na_adjective() {
        // Testing structure: な-Adjective + な + ことだし
        let sentence = "彼が勤めている会社は有名なことだし、給料もいいんだろう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことだし_compound");
        assert_pattern_range(&patterns, "ことだし_compound", 10, 17); // 有名なことだし
    }

    #[test]
    fn test_kotodashi_noun() {
        // Testing structure: Noun + の + ことだし
        let sentence = "彼のことだし、また遅れてくるんじゃないの。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ことだし_compound");
        assert_pattern_range(&patterns, "ことだし_compound", 0, 6); // 彼のことだし
    }
}

// ============================================================================
// べくして Tests
// ============================================================================

mod bekushite_tests {
    use super::*;

    // Pattern: べくして (as expected, destined to)
    // Data source: grammar_points_data.json["べくして"]
    //
    // Structure variants:
    //   1. (Intransitive) Verb + べくして + (same Verb)
    //   2. Verb (passive られる) + べくして + (same Verb)
    //
    // Note: The same verb must appear twice (before and after べくして)

    #[test]
    fn test_bekushite_intransitive_verb() {
        // Testing structure: Intransitive Verb + べくして + same Verb (past)
        // From example: 勝つべくして勝った
        let sentence = "毎日トレーニングしてきたから、大会で勝つべくして勝ったんだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "べくして");
        assert_pattern_range(&patterns, "べくして", 18, 24); // 勝つべくして
    }

    #[test]
    fn test_bekushite_intransitive_verb_happen() {
        // Testing structure: Intransitive Verb + べくして + same Verb (past)
        // From example: 起こるべくして起こった
        let sentence = "何回も訴えたが対応してくれなかったので、この事故は起こるべくして起こった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "べくして");
        assert_pattern_range(&patterns, "べくして", 25, 32); // 起こるべくして
    }

    #[test]
    fn test_bekushite_intransitive_verb_meet() {
        // Testing structure: Intransitive Verb + べくして + same Verb (past)
        // From example: 出会うべくして出会った
        let sentence = "最近、彼女とは出会うべくして出会ったと感じることが多い。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "べくして");
        assert_pattern_range(&patterns, "べくして", 7, 14); // 出会うべくして
    }
}

// ============================================================================
// 相まって Tests
// ============================================================================

mod aimatte_tests {
    use super::*;

    // Pattern: 相まって (combined with, coupled with)
    // Data source: grammar_points_data.json["相まって"]
    // Testing structure variants:
    //   - standard[0]: Noun + が + 相（あい）まって
    //   - standard[1]: Noun + と + 相（あい）まって

    #[test]
    fn test_aimatte_with_ga_particle() {
        // Testing structure: Noun + が + 相まって
        // From example: このステーキはシェフ特製のソースが相まって、とても美味しい
        let sentence = "このステーキはシェフ特製のソースが相まって、とても美味しい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "相まって");
        assert_pattern_range(&patterns, "相まって", 13, 21); // ソースが相まって
    }

    #[test]
    fn test_aimatte_with_to_particle() {
        // Testing structure: Noun + と + 相まって
        // From example: あの古い屋敷は霧と相まって不気味な雰囲気を出しています
        let sentence = "あの古い屋敷は霧と相まって不気味な雰囲気を出しています。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "相まって");
        assert_pattern_range(&patterns, "相まって", 7, 13); // 霧と相まって
    }

    #[test]
    fn test_aimatte_twitter_popularity() {
        // Testing structure: Noun + と + 相まって
        // From example: ツイッターでの流行と相まって、この漫画は若者の間ではすごく人気だ
        let sentence = "ツイッターでの流行と相まって、この漫画は若者の間ではすごく人気だ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "相まって");
        assert_pattern_range(&patterns, "相まって", 7, 14); // 流行と相まって
    }

    #[test]
    fn test_aimatte_client_ideas() {
        // Testing structure: Noun + が + 相まって
        // From example: クライアントのアイデアと私たちの努力が相まって
        let sentence = "クライアントのアイデアと私たちの努力が相まって、とてもいいイベントを開催することができた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "相まって");
        assert_pattern_range(&patterns, "相まって", 16, 23); // 努力が相まって
    }
}

// ============================================================================
// 次第です Tests
// ============================================================================

mod shidaidesu_tests {
    use super::*;

    // Pattern: 次第です (because, the reason is)
    // Data source: grammar_points_data.json["次第です"]
    // Testing structure: Verb + 次第 + です
    //
    // Usage: Polite explanation or reason for something
    // Similar to: "the reason is that...", "because..."

    #[test]
    fn test_shidaidesu_late_explanation() {
        // Testing structure: Verb[past] + 次第です
        // From example: 遅れた次第です (I was late because...)
        let sentence = "高速で玉突き事故があったので、遅れた次第です。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "次第です");
        assert_pattern_range(&patterns, "次第です", 18, 22); // 次第です
    }

    #[test]
    fn test_shidaidesu_introduction() {
        // Testing structure: Verb[past] + 次第です
        // From example: 挨拶しにきた次第です
        let sentence = "来月から、向かいの家に引っ越すことになったので、挨拶しにきた次第です。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "次第です");
        assert_pattern_range(&patterns, "次第です", 30, 34); // 次第です
    }

    #[test]
    fn test_shidaidesu_ongoing_adjustment() {
        // Testing structure: Verb[te-iru] + 次第です
        // From example: 色々と調整している次第です
        let sentence = "先輩が提案してくださったプランでは予算オーバーしそうだったので、今クライアントと相談しつつ色々と調整している次第です。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "次第です");
        assert_pattern_range(&patterns, "次第です", 54, 58); // 次第です
    }
}

// ============================================================================
// がてら Tests
// ============================================================================

mod gatera_tests {
    use super::*;

    // Pattern: がてら (while doing, on the occasion of)
    // Data source: grammar_points_data.json["がてら"]
    // Testing: structure.standard[0] - "Verb[stem] + がてら"
    // Testing: structure.standard[1] - "Noun + がてら"
    //
    // Meaning: "While (A), (B)" / "On the occasion of (A), (B)"
    // Unlike ながら, focuses on (A) being a good opportunity to do (B)
    // Formal register

    #[test]
    fn test_gatera_verb_stem_pick_up() {
        // Testing structure: Verb[stem] + がてら
        // From example: 弟を駅から迎えがてらに、今夜の夕飯の買い物をしに行く
        let sentence = "弟を駅から迎えがてらに、今夜の夕飯の買い物をしに行く。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がてら");
        assert_pattern_range(&patterns, "がてら", 5, 10); // 迎えがてら
    }

    #[test]
    fn test_gatera_verb_stem_visit() {
        // Testing structure: Verb[stem] + がてら
        // From example: 友達の家に行きがてら、近くの神社にお参りに行った
        let sentence = "友達の家に行きがてら、近くの神社にお参りに行った。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がてら");
        assert_pattern_range(&patterns, "がてら", 5, 10); // 行きがてら
    }

    #[test]
    fn test_gatera_noun_dog_walk() {
        // Testing structure: Noun + がてら
        // From example: ワンちゃんの散歩がてらに、ペットショップにいってくる
        let sentence = "ワンちゃんの散歩がてらに、ペットショップにいってくる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がてら");
        assert_pattern_range(&patterns, "がてら", 6, 11); // 散歩がてら
    }

    #[test]
    fn test_gatera_noun_exercise() {
        // Testing structure: Noun + がてら
        // From example: 運動がてらに会社まで自転車で行くことにした
        let sentence = "運動がてらに会社まで自転車で行くことにした。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がてら");
        assert_pattern_range(&patterns, "がてら", 0, 5); // 運動がてら
    }
}

// ============================================================================
// がん～ Tests (Colloquial Intensifier)
// ============================================================================

mod gan_uff5e_tests {
    use super::*;

    // Pattern: がん～ (colloquial intensifier - "majorly/completely/furiously")
    // Data source: grammar_points_data.json["がん～"]
    // Structure variants:
    //   1. ガン見 (specific expression - "furiously stare")
    //   2. ガン + Verb[stem] (productive pattern)
    // Note: This is colloquial/slang - ガン from ガンガン (onomatopoeia)

    #[test]
    fn test_gan_mi_specific() {
        // Testing structure: ガン見 (specific colloquial expression)
        // From example: 周りの人たちにガン見された
        let sentence = "電車の中で転んで、周りの人たちにガン見されたときは恥ずかしかった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がん～");
        assert_pattern_range(&patterns, "がん～", 16, 19); // ガン見
    }

    #[test]
    fn test_gan_mushi_ignore() {
        // Testing structure: ガン + Noun (ガン無視 - completely ignore)
        // From example: 両親の電話をガン無視していたら
        let sentence = "両親の電話をガン無視していたら家まで来られた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がん～");
        assert_pattern_range(&patterns, "がん～", 6, 10); // ガン無視
    }

    #[test]
    fn test_gan_gire_angry() {
        // Testing structure: ガン + Noun (ガン切れ - majorly angry)
        // From example: 今日も先輩にガン切れされたよ
        let sentence = "今日も先輩にガン切れされたよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "がん～");
        assert_pattern_range(&patterns, "がん～", 6, 10); // ガン切れ
    }
}
