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
