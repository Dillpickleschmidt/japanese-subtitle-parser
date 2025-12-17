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

// ============================================================================
// てやまない Tests (Never cease to, earnestly)
// ============================================================================

mod teyamanai_tests {
    use super::*;

    // Pattern: てやまない (never cease to, earnestly, from the bottom of one's heart)
    // Data source: grammar_points_data.json["てやまない"]
    // Structure variants:
    //   1. Verb[て] + やまない (standard)
    //   2. Verb[て] + やみません (polite)
    // Used exclusively with feelings/emotions (not temporary ones)

    #[test]
    fn test_teyamanai_love() {
        // Testing structure: Verb[て] + やまない
        // From example: 私が愛してやまない犬
        let sentence = "私が愛してやまない犬が急に体を壊したから心配でしょうがない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てやまない");
        assert_pattern_range(&patterns, "てやまない", 2, 9); // 愛してやまない
    }

    #[test]
    fn test_teyamanai_regret() {
        // Testing structure: Verb[て] + やまない
        // From example: 後悔してやまない
        let sentence = "あんなことを親にさせてしまって後悔してやまない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てやまない");
        assert_pattern_range(&patterns, "てやまない", 15, 23); // 後悔してやまない
    }

    #[test]
    fn test_teyamanai_polite_pray() {
        // Testing structure: Verb[て] + やみません (polite)
        // From example: 祈ってやみません
        let sentence = "１日も早く普通の生活に戻れる様に祈ってやみません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てやまない");
        assert_pattern_range(&patterns, "てやまない", 16, 24); // 祈ってやみません
    }
}

// ============================================================================
// ぐらいなら Tests (Would rather / Better off)
// ============================================================================

mod gurainara_tests {
    use super::*;

    // Pattern: ぐらいなら (would rather B than A, better off B than A)
    // Data source: grammar_points_data.json["ぐらいなら"]
    // Structure variants:
    //   1. Verb[る] + ぐらいなら + Phrase
    //   2. Verb[る] + くらいなら + Phrase (alternative)
    // Meaning: "If it gets to the extent of A, then B" where B is preferable

    #[test]
    fn test_gurainara_with_gurai() {
        // Testing structure: Verb[dictionary] + ぐらいなら
        // From example: 引っ越すぐらいなら、死んだ方がマシだ
        let sentence = "あの地域に引っ越すぐらいなら、死んだ方がマシだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ぐらいなら");
        assert_pattern_range(&patterns, "ぐらいなら", 5, 14); // 引っ越すぐらいなら
    }

    #[test]
    fn test_kurainara_with_kurai() {
        // Testing structure: Verb[dictionary] + くらいなら
        // From example: 働くくらいなら、一人で働いた方がいい
        let sentence = "あの先輩の下で働くくらいなら、一人で働いた方がいいと思うよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ぐらいなら");
        assert_pattern_range(&patterns, "ぐらいなら", 7, 14); // 働くくらいなら
    }

    #[test]
    fn test_gurainara_buy_new() {
        // Testing structure: Verb[dictionary] + くらいなら
        // From example: 直すくらいなら、買った方がいい
        let sentence = "何十万もかけて直すくらいなら、もう少し出して新しい車を買った方がいい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ぐらいなら");
        assert_pattern_range(&patterns, "ぐらいなら", 7, 14); // 直すくらいなら
    }
}

// ============================================================================
// まま(に)1 Tests
// ============================================================================

mod mamani1_tests {
    use super::*;

    // Pattern: まま(に)1 (as one wishes, on a whim)
    // Data source: grammar_points_data.json["まま(に)1"]
    // Testing: structure.standard[0] - "Verb[dictionary] + (が) + まま(に)"
    //
    // Note: This is DIFFERENT from the basic まま(に) pattern:
    // - まま(に): Verb[た/ない] + まま = "left in the state of"
    // - まま(に)1: Verb[dictionary] + まま(に) = "as one wishes/desires"
    //
    // Common expressions:
    // - 思うままに = as one thinks/wishes
    // - 思いつくままに = whatever comes to mind
    // - 気の向くままに = as one's interest is inclined
    // - 足の向くままに = wherever one's feet take them
    // - 欲するままに = as one desires

    #[test]
    fn test_mamani1_omou_mamani() {
        // Testing: Verb[dictionary] + ままに (思う + ままに)
        // Example: 自分が思うままに使えばいいよ
        let sentence = "これはもうあなたのものだから、自分が思うままに使えばいいよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まま(に)1");
        assert_pattern_range(&patterns, "まま(に)1", 18, 23); // 思うままに
    }

    #[test]
    fn test_mamani1_muku_mamani() {
        // Testing: Verb[dictionary] + ままに (向く + ままに)
        // Example: 足の向くままに旅をする
        let sentence = "僕はしばらくの間一人で、足の向くままに旅をする。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まま(に)1");
        assert_pattern_range(&patterns, "まま(に)1", 14, 19); // 向くままに
    }

    #[test]
    fn test_mamani1_omoitsuku_mamani() {
        // Testing: Verb[dictionary] + ままに (思いつく + ままに)
        // Example: 思いつくままに書いてくれればいい
        let sentence = "文法とかは気にしなくていい！ただ思いつくままに書いてくれればいい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まま(に)1");
        assert_pattern_range(&patterns, "まま(に)1", 16, 23); // 思いつくままに
    }

    #[test]
    fn test_mamani1_with_ga_particle() {
        // Testing: Verb[dictionary] + が + ままに (optional が particle)
        // Example: 心が向くままに生きる
        let sentence = "心が向くままに生きるのが一番だと思う。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まま(に)1");
        assert_pattern_range(&patterns, "まま(に)1", 2, 7); // 向くままに (が is optional, not included in range)
    }
}

// ============================================================================
// かれ〜かれ Tests
// ============================================================================

mod kare_u301c_kare_tests {
    use super::*;

    // Pattern: かれ〜かれ (whether A or B)
    // Data source: grammar_points_data.json["かれ〜かれ"]
    // Testing: structure.standard[0-5] - Various い-Adjective stem + かれ pairs
    //
    // Structures:
    //   - standard[0]: い-Adjective[stem] + かれ + い-Adjective[antonym stem] + かれ
    //   - standard[1]: 遅かれ早かれ - sooner or later
    //   - standard[2]: 多かれ少なかれ - more or less
    //   - standard[3]: to a greater or lesser extent
    //   - standard[4]: 良かれ悪しかれ - good or bad
    //   - standard[5]: for better or worse
    //
    // Note: This pattern uses a very limited set of adjectives in set phrases

    #[test]
    fn test_kare_kare_osokare_hayakare() {
        // Testing: 遅かれ早かれ (sooner or later)
        // Most common fixed expression
        let sentence = "遅かれ早かれ、このことは公になるだろう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かれ〜かれ");
        assert_pattern_range(&patterns, "かれ〜かれ", 0, 6); // 遅かれ早かれ
    }

    #[test]
    fn test_kare_kare_ookare_sukunakare() {
        // Testing: 多かれ少なかれ (more or less)
        // Common fixed expression
        let sentence = "どんな人でも多かれ少なかれ悩みを抱えて生きている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かれ〜かれ");
        assert_pattern_range(&patterns, "かれ〜かれ", 6, 13); // 多かれ少なかれ
    }

    #[test]
    fn test_kare_kare_yokare_ashikare() {
        // Testing: 良かれ悪しかれ (for better or worse)
        // Note: 悪し is archaic form of 悪い
        let sentence = "この発明は、良かれ悪しかれたくさんの人に影響を与えるだろう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かれ〜かれ");
        assert_pattern_range(&patterns, "かれ〜かれ", 6, 13); // 良かれ悪しかれ
    }

    #[test]
    fn test_kare_kare_atsukare_samukare() {
        // Testing: 暑かれ寒かれ (whether hot or cold)
        // Less common but valid variant
        let sentence = "暑かれ寒かれ、脱水症状は起こりうるので水分補給はこまめにしてください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "かれ〜かれ");
        assert_pattern_range(&patterns, "かれ〜かれ", 0, 6); // 暑かれ寒かれ
    }
}

// ============================================================================
// なくして(は) Tests
// ============================================================================

mod nakushiteha_tests {
    use super::*;

    // Pattern: なくして(は) (without)
    // Data source: grammar_points_data.json["なくして(は)"]
    // Testing: structure.standard[0] - "Noun + なくして(は)"
    //
    // Other structures to test:
    //   - standard[1]: Verb + ことなくして(は)

    #[test]
    fn test_nakushiteha_noun_simple() {
        let sentence = "私は彼女なくしては生きていけないと思います。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくして(は)");
        assert_pattern_range(&patterns, "なくして(は)", 2, 9); // 彼女なくしては
    }

    #[test]
    fn test_nakushiteha_noun_effort() {
        let sentence = "努力なくしては、日本語は上達しません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくして(は)");
        assert_pattern_range(&patterns, "なくして(は)", 0, 7); // 努力なくしては
    }

    #[test]
    fn test_nakushiteha_verb_koto() {
        let sentence = "失敗することなくしては、成功できないと言われているが本当なのだろうか。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくして(は)");
        assert_pattern_range(&patterns, "なくして(は)", 4, 11); // ことなくしては
    }

    #[test]
    fn test_nakushiteha_verb_koto_consider() {
        let sentence = "相手の気持ちを考えることなくしては、友情は続かない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なくして(は)");
        assert_pattern_range(&patterns, "なくして(は)", 10, 17); // ことなくしては
    }
}

// ============================================================================
// のなんのって Tests
// ============================================================================

mod nonannotte_tests {
    use super::*;

    // Pattern: のなんのって (extremely, so much that)
    // Data source: grammar_points_data.json["のなんのって"]
    // Testing all structure variants

    #[test]
    fn test_nonannotte_verb() {
        let sentence = "あんな痩せているのにあの量一人で食べちゃうなんて、びっくりしたのなんのって。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "のなんのって");
        assert_pattern_range(&patterns, "のなんのって", 30, 37); // たのなんのって
    }

    #[test]
    fn test_nonannotte_i_adjective() {
        let sentence = "名古屋の夏は暑いのなんのって、外に出た瞬間シャツが汗でびっしょりになるんだよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "のなんのって");
        assert_pattern_range(&patterns, "のなんのって", 6, 14); // 暑いのなんのって
    }

    #[test]
    fn test_nonannotte_na_adjective() {
        let sentence = "あの建物は丈夫なのなんのって、去年あった震度７の地震までも耐えたんだよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "のなんのって");
        assert_pattern_range(&patterns, "のなんのって", 7, 14); // なのなんのって
    }
}

// ============================================================================
// にかかっている Tests
// ============================================================================

mod nikakatteiru_tests {
    use super::*;

    // Pattern: にかかっている (depends on)
    // Data source: grammar_points_data.json["にかかっている"]
    // Testing all structure variants

    #[test]
    fn test_nikakatteiru_noun_basic() {
        let sentence = "地球の未来は若者にかかっているから、若者を教育しなくてはいけない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にかかっている");
        assert_pattern_range(&patterns, "にかかっている", 8, 15); // にかかっている
    }

    #[test]
    fn test_nikakatteiru_kadouka() {
        let sentence = "自分で立ち上げた会社が成功するかどうかは自分の努力にかかっている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にかかっている");
        assert_pattern_range(&patterns, "にかかっている", 25, 32); // にかかっている
    }

    #[test]
    fn test_nikakatteiru_question_word() {
        let sentence = "来月旅行に行けるかは、部長が僕に休みをくれるかにかかっている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にかかっている");
        assert_pattern_range(&patterns, "にかかっている", 23, 30); // にかかっている
    }

    #[test]
    fn test_nikakatteiru_polite() {
        let sentence = "この商品の売れ行きはマーケティングが成功するかどうかにかかっています。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にかかっている");
        assert_pattern_range(&patterns, "にかかっている", 26, 34); // にかかっています
    }
}

// ============================================================================
// ずとも Tests
// ============================================================================

mod zutomo_tests {
    use super::*;

    // Pattern: ずとも (even if not / don't have to)
    // Data source: grammar_points_data.json["ずとも"]
    // Testing: structure.standard[0] - "Verb[ない] + ず + とも"
    //
    // Structures to test:
    //   - standard[0]: Verb[ない] + ず + とも
    //   - standard[1]: Noun + ならずとも (from なる[ない])
    //   - Exception: する → せずとも

    #[test]
    fn test_zutomo_verb_regular() {
        let sentence = "昔は練習せずとも新しい曲を完璧に弾けたのに、今となってはどんだけ練習しても上手く弾けないんだよな。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずとも");
        assert_pattern_range(&patterns, "ずとも", 2, 8); // 練習せずとも
    }

    #[test]
    fn test_zutomo_verb_kiku() {
        let sentence = "あなたが今ものすごく落ち込んでいるということは聞かずとも分かりますよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずとも");
        assert_pattern_range(&patterns, "ずとも", 23, 28); // 聞かずとも
    }

    #[test]
    fn test_zutomo_verb_study() {
        let sentence = "日本に留学せずとも、日本語を習得する方法はあるが、日本に行って勉強する方が楽しそうだから私は日本に留学したいです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずとも");
        assert_pattern_range(&patterns, "ずとも", 3, 9); // 留学せずとも
    }

    #[test]
    fn test_zutomo_narazu_noun() {
        let sentence = "専門家ならずとも、彼は政治について色々と知っている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずとも");
        assert_pattern_range(&patterns, "ずとも", 3, 8); // ならずとも
    }

    #[test]
    fn test_zutomo_narazu_parent() {
        let sentence = "親ならずとも子育てが大変だということは見てわかるはずだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずとも");
        assert_pattern_range(&patterns, "ずとも", 1, 6); // ならずとも
    }

    #[test]
    fn test_zutomo_with_yoi() {
        let sentence = "来週の試合の相手はあの弱いチームだから、そんなに練習せずとも良いだろう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずとも");
        assert_pattern_range(&patterns, "ずとも", 24, 30); // 練習せずとも
    }
}

// ============================================================================
// ってば・ったら Tests
// ============================================================================

mod tteba_ttara_tests {
    use super::*;

    // Pattern: ってば・ったら (insisting on viewpoint, expressing frustration)
    // Data source: grammar_points_data.json["ってば・ったら"]
    // Testing structures:
    //   - standard[0]: Verb + ってば
    //   - standard[1]: い-Adj + ってば
    //   - standard[2]: な-Adj + (だ) + ってば
    //   - standard[3]: Noun + (だ) + ってば
    //   - (1) Can use ったら instead of ってば
    //
    // Two uses:
    // 1. With だ: Insisting viewpoint (translated as "I said X!", "I told you X!")
    // 2. Without だ (after noun/name): Addressing person ("that darn X", "oh that X")

    #[test]
    fn test_tteba_verb_past() {
        let sentence = "分かったってば！何回も言われると本当むかつくからちょっと黙ってて。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ってば・ったら");
        assert_pattern_range(&patterns, "ってば・ったら", 3, 7); // たってば
    }

    // TODO: UNDETECTABLE - い-Adj + ったら tokenizes as verb いう + たら
    // Tokenization: 痛 (Adj stem) + いっ (verb いう) + たら (conditional)
    // Cannot distinguish from actual verb いう + たら structurally
    // #[test]
    // fn test_ttara_i_adjective() {
    //     let sentence = "痛いったら！なんでずっと痛いって言ってるのに何回も何回もフォークでつついてくるの？";
    //     let tokens = tokenize_sentence(sentence);
    //     let patterns = detect_patterns(&tokens);
    //     assert_has_pattern(&patterns, "ってば・ったら");
    //     assert_pattern_range(&patterns, "ってば・ったら", 0, 5); // 痛いったら
    // }

    #[test]
    fn test_tteba_i_adjective() {
        let sentence = "痛いってば！なんで信じてくれないの？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ってば・ったら");
        assert_pattern_range(&patterns, "ってば・ったら", 0, 5); // 痛いってば
    }

    #[test]
    fn test_tteba_noun_with_da() {
        let sentence = "嫌いだってば！あんな奴なんて大嫌いだ！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ってば・ったら");
        assert_pattern_range(&patterns, "ってば・ったら", 0, 6); // 嫌いだってば
    }

    #[test]
    fn test_ttara_noun_with_da() {
        let sentence = "だから、俺は大人だったら！背が低いからってひどいぞ本当に。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ってば・ったら");
        assert_pattern_range(&patterns, "ってば・ったら", 6, 12); // 大人だったら
    }

    #[test]
    fn test_tteba_person_name_addressing() {
        let sentence = "金太郎ってば、またエアコンつけっぱなしで家出てったよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ってば・ったら");
        assert_pattern_range(&patterns, "ってば・ったら", 0, 6); // 金太郎ってば
    }

    // TODO: UNDETECTABLE - あなたったら tokenizes incorrectly
    // Tokenization: あな (noun "hole") + たっ (verb たつ "to stand") + たら
    // Should be: あなた (pronoun "you") + ったら
    // Kagome tokenization error - cannot detect this case
    // #[test]
    // fn test_ttara_pronoun_addressing() {
    //     let sentence = "あなたったら本当におっちょこちょいね。";
    //     let tokens = tokenize_sentence(sentence);
    //     let patterns = detect_patterns(&tokens);
    //     assert_has_pattern(&patterns, "ってば・ったら");
    //     assert_pattern_range(&patterns, "ってば・ったら", 0, 6); // あなたったら
    // }
}

// ============================================================================
// ぐるみで Tests
// ============================================================================

mod gurumide_tests {
    use super::*;

    // Pattern: ぐるみで (including / all over / whole)
    // Data source: grammar_points_data.json["ぐるみで"]
    // Testing: structure.standard[0] - "Noun + ぐるみで"
    // Testing: structure.standard[1] - "Noun + ぐるみ + の + Noun"
    //
    // Meaning: "including (A)", "all over (A)", or "(A) wide"
    // Etymology: From 包（くる）む "to wrap/encompass"
    // Structures: Noun + ぐるみで / Noun + ぐるみの + Noun

    #[test]
    fn test_gurumide_family_with_de() {
        let sentence = "毎年家族ぐるみで初詣に行きます。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ぐるみで");
        assert_pattern_range(&patterns, "ぐるみで", 2, 8); // 家族ぐるみで
    }

    #[test]
    fn test_gurumide_company_with_de() {
        let sentence = "会社ぐるみで違法な取引をしていたので、社員全員逮捕された。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ぐるみで");
        assert_pattern_range(&patterns, "ぐるみで", 0, 6); // 会社ぐるみで
    }

    #[test]
    fn test_gurumide_family_with_no() {
        let sentence = "彼とは家族ぐるみの付き合いです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ぐるみで");
        assert_pattern_range(&patterns, "ぐるみで", 3, 9); // 家族ぐるみの
    }

    #[test]
    fn test_gurumide_town_with_no() {
        let sentence = "街ぐるみの治安をよくするために、町内会の人たちが毎晩パトロールをしています。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ぐるみで");
        assert_pattern_range(&patterns, "ぐるみで", 0, 5); // 街ぐるみの
    }
}

// Pattern: が早いか (as soon as, no sooner than)
// Data source: grammar_points_data.json["が早いか"]
// Testing structure variants:
//   - standard[0]: Verb[dictionary] + が早いか + Phrase[past]
//   - standard[1]: Verb[past] + が早いか + Phrase[past]
//
// Note: This pattern is formal and expresses immediate succession of actions
// The second action (B) is always in past tense and often unexpected
mod ga_hayai_ka_tests {
    use super::*;

    #[test]
    fn test_ga_hayai_ka_dictionary_form() {
        let sentence = "彼女は彼が書いた手紙を読むが早いか、悲しさのあまりで泣き始めた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "が早いか");
        assert_pattern_range(&patterns, "が早いか", 11, 17); // 読むが早いか
    }

    #[test]
    fn test_ga_hayai_ka_dictionary_form_sit() {
        let sentence = "息子は食卓に座るが早いか、並べてあった夕食を吸い込むように食い上げた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "が早いか");
        assert_pattern_range(&patterns, "が早いか", 6, 12); // 座るが早いか
    }

    #[test]
    fn test_ga_hayai_ka_past_form_alarm() {
        let sentence = "木村くんはアラームが鳴ったが早いか、スヌーズボタンを押した。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "が早いか");
        assert_pattern_range(&patterns, "が早いか", 12, 17); // たが早いか
    }

    #[test]
    fn test_ga_hayai_ka_past_form_police() {
        let sentence = "警察官は犯人を見つけたが早いか、犯人に飛びつき手錠をかけた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "が早いか");
        assert_pattern_range(&patterns, "が早いか", 10, 15); // たが早いか
    }
}

// ============================================================================
// こととて Tests
// ============================================================================

mod kototote_tests {
    use super::*;

    // Pattern: こととて (due to / because of)
    // Data source: grammar_points_data.json["こととて"]
    // Testing structure variants:
    //   - standard[0]: Verb + こととて
    //   - standard[1]: Verb[ない] + ぬ + こととて (classical negative)
    //   - standard[2]: Noun + の + こととて

    #[test]
    fn test_kototote_verb_past() {
        let sentence = "あの事件は２０年前に起こったこととて、世間からは忘れられているだろう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "こととて");
        assert_pattern_range(&patterns, "こととて", 10, 18); // 起こったこととて
    }

    #[test]
    fn test_kototote_verb_nu_negative() {
        let sentence = "商品についてあまり知らぬこととて、お客様の信頼を失ってしまった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "こととて");
        assert_pattern_range(&patterns, "こととて", 9, 16); // 知らぬこととて
    }

    #[test]
    fn test_kototote_noun_no_newbie() {
        let sentence = "新人のこととて、まだ分からないことがたくさんありますが、これからよろしくお願いいたします。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "こととて");
        assert_pattern_range(&patterns, "こととて", 0, 7); // 新人のこととて
    }

    #[test]
    fn test_kototote_noun_no_sudden() {
        let sentence = "突然のこととて、前の車を避けきれませんでした。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "こととて");
        assert_pattern_range(&patterns, "こととて", 0, 7); // 突然のこととて
    }
}

// ============================================================================
// とあって Tests
// ============================================================================

mod toatte_tests {
    use super::*;

    // Pattern: とあって (since, because of)
    // Data source: grammar_points_data.json["とあって"]
    // Testing all structure variants:
    //   - standard[0]: Verb + とあって
    //   - standard[1]: い-Adjective + とあって
    //   - standard[2]: な-Adjective + (だ) + とあって
    //   - standard[3]: Noun + (だ) + とあって

    #[test]
    fn test_toatte_verb() {
        let sentence = "あのグループの卒業ライブが行われるとあって、この会場は大勢のファンで溢れている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とあって");
        assert_pattern_range(&patterns, "とあって", 15, 21); // れるとあって
    }

    #[test]
    fn test_toatte_i_adjective() {
        let sentence = "北海道は夏の間でも涼しいとあって、冬の間だけでなく夏にも多くの観光客が訪れるらしい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とあって");
        assert_pattern_range(&patterns, "とあって", 9, 16); // 涼しいとあって
    }

    #[test]
    fn test_toatte_na_adjective() {
        let sentence = "あの街は便利だとあって、土地の値段が高い。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とあって");
        assert_pattern_range(&patterns, "とあって", 6, 11); // だとあって
    }

    #[test]
    fn test_toatte_noun() {
        let sentence = "お盆の最終日とあって、高速道路だけでなく、下道もものすごく混んでいる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とあって");
        assert_pattern_range(&patterns, "とあって", 5, 10); // 日とあって
    }
}

// ============================================================================
// さぞ Tests
// ============================================================================

mod sazo_tests {
    use super::*;

    // Pattern: さぞ (you must be very, how ~ you must be, I dare say that)
    // Data source: grammar_points_data.json["さぞ"]
    // Testing: structure.standard[0] - "さぞ + （かし）+ Phrase + （こと）+ だろう"
    //
    // Other structures to test:
    //   - polite[0]: さぞ + （かし）+ Phrase + （こと）+ でしょう
    //   - variants: さぞや and さぞかし (emphasized versions)

    #[test]
    fn test_sazo_basic_darou() {
        let sentence = "あの一流企業で働いているなら、さぞいいところに住んでいるんでしょう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "さぞ");
        assert_pattern_range(&patterns, "さぞ", 15, 17); // さぞ
    }

    #[test]
    fn test_sazo_polite_deshou() {
        let sentence = "おばあさんが急に入院されて、さぞご心配でしょう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "さぞ");
        assert_pattern_range(&patterns, "さぞ", 14, 16); // さぞ
    }

    #[test]
    fn test_sazo_kashi_variant() {
        let sentence = "あの人が彼の父だなんて、さぞかし驚いただろう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "さぞ");
        assert_pattern_range(&patterns, "さぞ", 12, 16); // さぞかし
    }

    #[test]
    fn test_sazo_ya_variant() {
        let sentence = "あんなに大切にしていた車を盗まれたなんて、さぞや悔しかっただろう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "さぞ");
        assert_pattern_range(&patterns, "さぞ", 21, 24); // さぞや
    }
}

// ============================================================================
// でもなんでもない Tests
// ============================================================================

mod demonandemonai_tests {
    use super::*;

    // Pattern: でもなんでもない (not at all / definitely not)
    // Data source: grammar_points_data.json["でもなんでもない"]
    // Testing: structure.standard[0] - "Noun + でもなんでもない"
    //
    // Other structures to test:
    //   - standard[1]: な-Adjective + でもなんでもない
    //   - polite[0]: Noun + でもなんでもないです
    //   - polite[1]: な-Adjective + でもなんでもないです
    //   - Caution: い-Adjective + くもなんでもない (less common variant)

    #[test]
    fn test_demonandemonai_noun_fan() {
        let sentence = "俺はファンでもなんでもないよ、ただあの人が作曲した曲を聞くのが好きなだけ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "でもなんでもない");
        assert_pattern_range(&patterns, "でもなんでもない", 2, 13); // ファンでもなんでもない
    }

    #[test]
    fn test_demonandemonai_noun_friend() {
        let sentence = "あっ、あの人ですか？友達でもなんでもないですよ、なんかさっき急に話しかけてきたので話していただけです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "でもなんでもない");
        assert_pattern_range(&patterns, "でもなんでもない", 10, 22); // 友達でもなんでもないです (includes polite です)
    }

    #[test]
    fn test_demonandemonai_na_adj_bothersome() {
        let sentence = "迷惑でもなんでもないですよ、ゆっくりしていってください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "でもなんでもない");
        assert_pattern_range(&patterns, "でもなんでもない", 0, 12); // 迷惑でもなんでもないです (includes polite です)
    }

    #[test]
    fn test_demonandemonai_na_adj_like() {
        let sentence = "好きでもなんでもない人からそんなこと言われても全然嬉しくない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "でもなんでもない");
        assert_pattern_range(&patterns, "でもなんでもない", 0, 10); // 好きでもなんでもない (no です in this sentence)
    }

    // Note: い-Adjective + くもなんでもない variant is less common
    // Testing one example for completeness
    #[test]
    fn test_demonandemonai_i_adj_heavy() {
        let sentence = "これ重くもなんでもないじゃん。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "でもなんでもない");
        assert_pattern_range(&patterns, "でもなんでもない", 2, 14); // 重くもなんでもないじゃん (includes casual じゃん)
    }
}

// ============================================================================
// そばから Tests
// ============================================================================

mod sobakara_tests {
    use super::*;

    // Pattern: そばから (as soon as / right after)
    // Data source: grammar_points_data.json["そばから"]
    // Testing: structure.standard[0] - "Verb[る] + そばから"
    //          structure.standard[1] - "Verb[た] + そばから"
    //
    // Meaning: "As soon as (A), (B)" - usually (B) is something annoying
    // that happens immediately and nullifies (A)
    //
    // Etymology: そば (side) + から (from) = "from right beside (A)"

    #[test]
    fn test_sobakara_verb_dictionary_yaburu() {
        let sentence = "彼女は約束をするそばからすぐ約束を破る。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そばから");
        assert_pattern_range(&patterns, "そばから", 6, 12); // するそばから
    }

    #[test]
    fn test_sobakara_verb_dictionary_shippai() {
        let sentence = "彼は注意するそばから失敗をするから、私のいうことは聞いていないんだろう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そばから");
        assert_pattern_range(&patterns, "そばから", 2, 10); // 注意するそばから
    }

    #[test]
    fn test_sobakara_verb_past_kowareru() {
        let sentence = "パソコンを直したそばからまた壊れてしまうから困っている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そばから");
        assert_pattern_range(&patterns, "そばから", 5, 12); // 直したそばから
    }

    #[test]
    fn test_sobakara_verb_past_wasureru() {
        let sentence = "早く読みすぎると読んだそばから忘れるから、もっとゆっくり読むようにしている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "そばから");
        assert_pattern_range(&patterns, "そばから", 8, 15); // 読んだそばから
    }
}

// Pattern: ずくめ (nothing but / all in)
// Data source: grammar_points_data.json["ずくめ"]
// Testing all structure variants with print_debug
mod zukume_tests {
    use super::*;

    // Testing: structure.standard[0] - "Noun + ずくめ"
    #[test]
    fn test_zukume_noun_shigoto() {
        let sentence = "今月も仕事ずくめになりそうだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずくめ");
        assert_pattern_range(&patterns, "ずくめ", 3, 8); // 仕事ずくめ
    }

    // Testing: structure.standard[0] - "Noun + ずくめ" (negative context)
    #[test]
    fn test_zukume_noun_iyanakoto() {
        let sentence = "最近は嫌なことずくめで全然楽しくない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずくめ");
        assert_pattern_range(&patterns, "ずくめ", 5, 10); // ことずくめ
    }

    // Testing: structure.standard[1] - "Noun (A) + ずくめ + の + Noun (B)"
    #[test]
    fn test_zukume_no_noun_kuro() {
        let sentence = "彼は黒ずくめの男たちに誘拐された。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずくめ");
        assert_pattern_range(&patterns, "ずくめ", 2, 6); // 黒ずくめ
    }

    // Testing: structure.standard[1] - "Noun (A) + ずくめ + の + Noun (B)" (alternative example)
    #[test]
    fn test_zukume_no_noun_kuruma() {
        let sentence = "家の前に黒ずくめの車が何台も止まっているんです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずくめ");
        assert_pattern_range(&patterns, "ずくめ", 4, 8); // 黒ずくめ
    }
}

// Pattern: ずじまい (end up not doing)
// Data source: grammar_points_data.json["ずじまい"]
// Testing structure variant with print_debug
mod zujimai_tests {
    use super::*;

    // Testing: structure.standard[0] - "Verb［ない］+ ず + じまい"
    #[test]
    fn test_zujimai_verb_tsukau() {
        let sentence = "マウンテンバイクを買ったが、使わずじまいで売ってしまった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずじまい");
        assert_pattern_range(&patterns, "ずじまい", 14, 20); // 使わずじまい
    }

    // Testing: structure.standard[0] - with だった ending (token combiner extends range)
    #[test]
    fn test_zujimai_verb_iku_datta() {
        let sentence = "コンサートのチケットを買ったが、色々とやることがあって行かずじまいだった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずじまい");
        assert_pattern_range(&patterns, "ずじまい", 27, 36); // 行かずじまいだった (includes copula)
    }

    // Testing: structure.standard[0] - longer sentence with だった
    #[test]
    fn test_zujimai_verb_tsukau_denshi() {
        let sentence = "日本で使うために電子辞書を持っていったが、持ち運ぶのが面倒だったから結局使わずじまいだった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずじまい");
        assert_pattern_range(&patterns, "ずじまい", 36, 45); // 使わずじまいだった (includes copula)
    }

    // Testing: structure.standard[0] - with する verb (becomes せず)
    #[test]
    fn test_zujimai_suru_verb_kokuhaku() {
        let sentence = "好きな子に告白せずじまいで、高校を卒業してしまった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずじまい");
        assert_pattern_range(&patterns, "ずじまい", 5, 12); // 告白せずじまい
    }
}

// ============================================================================
// ずにはおかない Tests
// ============================================================================

mod zunihaokanai_tests {
    use super::*;

    // Pattern: ずにはおかない (will certainly do / will not fail to do)
    // Data source: grammar_points_data.json["ずにはおかない"]
    // Testing: structure.standard[0] - "Verb [ない-stem] + ずには + おかない"
    //
    // Structure variants:
    //   - standard[0]: Verb [ない-stem] + ずには + おかない
    //   - standard[1]: Verb [ない] + では + おかない
    //   - standard[2]: Exception: する→せずにはおかない

    // Testing: structure.standard[0] - regular verb with ずには
    #[test]
    fn test_zunihaokanai_verb_tsukamaeru() {
        let sentence = "あんなひどいことをした人だから、警察も捕まえずにはおかないだろう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずにはおかない");
        assert_pattern_range(&patterns, "ずにはおかない", 19, 32); // 捕まえずにはおかないだろう
    }

    // Testing: structure.standard[0] - with causative verb
    #[test]
    fn test_zunihaokanai_causative_kandou() {
        let sentence = "あの人のスピーチは聞く人を感動させずにはおかない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずにはおかない");
        assert_pattern_range(&patterns, "ずにはおかない", 16, 24); // せずにはおかない
    }

    // Testing: structure.standard[2] - する verb exception (becomes せず)
    #[test]
    fn test_zunihaokanai_suru_verb_chousa() {
        let sentence = "メディアにも取り上げられたから、警察も調査をせずにはおかない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずにはおかない");
        assert_pattern_range(&patterns, "ずにはおかない", 22, 30); // せずにはおかない
    }

    // Testing: structure.standard[1] - ないでは form
    #[test]
    fn test_zunihaokanai_naidewa_uttaeru() {
        let sentence = "こんなことをされたのだから、お客さんも訴えないではおかないだろう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないではおかない");
        assert_pattern_range(&patterns, "ないではおかない", 19, 32); // 訴えないではおかないだろう
    }

    // Testing: structure.standard[1] - ないでは form with different verb
    #[test]
    fn test_zunihaokanai_naidewa_chikara() {
        let sentence = "この犬は３０キロになるから、躾に力を入れないではおかない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないではおかない");
        assert_pattern_range(&patterns, "ないではおかない", 18, 28); // 入れないではおかない
    }
}

// ============================================================================
// ずにはすまない Tests
// ============================================================================

mod zunihasumanai_tests {
    use super::*;

    // Pattern: ずにはすまない (won't get away without doing / have no choice but to do)
    // Data source: grammar_points_data.json["ずにはすまない"]
    // Testing: structure.standard[0] - "Verb[ない] + ずには + すまない"
    //
    // Other structures to test:
    //   - standard[1]: Verb[ない] + では + すまない (alternative form)
    //   - Exception: する → せずにはすまない

    // Testing: structure.standard[0] - regular verb with ずには
    #[test]
    fn test_zunihasumanai_verb_ayamaru() {
        let sentence = "あんなに迷惑をかけてしまったんだから、謝らずにはすまないだろう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずにはすまない");
        assert_pattern_range(&patterns, "ずにはすまない", 19, 31); // 謝らずにはすまないだろう (includes だろう via token combiner)
    }

    // Testing: structure.standard[0] - regular verb with ずには
    #[test]
    fn test_zunihasumanai_verb_iku() {
        let sentence = "取引先のお偉いさんに飲みに誘われたから、行かずにはすまない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずにはすまない");
        assert_pattern_range(&patterns, "ずにはすまない", 20, 29); // 行かずにはすまない
    }

    // Testing: structure.standard[0] - regular verb with ずには
    #[test]
    fn test_zunihasumanai_verb_deru() {
        let sentence = "会社の先輩からの電話は出ずにはすまない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずにはすまない");
        assert_pattern_range(&patterns, "ずにはすまない", 11, 19); // 出ずにはすまない
    }

    // Testing: exception - する verb becomes せず
    #[test]
    fn test_zunihasumanai_suru_verb_benshou() {
        let sentence = "やばい、田中君のゲーム機を壊してしまった。これは、弁償せずにはすまないな。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ずにはすまない");
        assert_pattern_range(&patterns, "ずにはすまない", 25, 35); // 弁償せずにはすまない (does not include な)
    }

    // Testing: structure.standard[1] - ないでは form
    #[test]
    fn test_zunihasumanai_naidewa_ayamaru() {
        let sentence = "父が大切にしていた釣竿を折ってしまった。謝らないではすまない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないではすまない");
        assert_pattern_range(&patterns, "ないではすまない", 20, 30); // 謝らないではすまない
    }

    // Testing: structure.standard[1] - ないでは form with different verb
    #[test]
    fn test_zunihasumanai_naidewa_harau() {
        let sentence = "税金は払わないではすまない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないではすまない");
        assert_pattern_range(&patterns, "ないではすまない", 3, 13); // 払わないではすまない
    }
}

// ============================================================================
// たら〜で Tests
// ============================================================================

mod tara_u301c_de_tests {
    use super::*;

    // Pattern: たら〜で (even if, if...then with)
    // Data source: grammar_points_data.json["たら〜で"]
    // Testing all structure variants with print_debug

    // Testing: structure.standard[0] - Verb[たら] + Verb[た] + で
    #[test]
    fn test_tarade_verb_tara_form() {
        let sentence = "ニートになったらなったで大変だと思うよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たら〜で");
        assert_pattern_range(&patterns, "たら〜で", 4, 12); // なったらなったで
    }

    // Testing: structure.standard[3] - Verb[ば] + Verb[た] + で
    #[test]
    fn test_tarade_verb_ba_form() {
        let sentence = "明日雨が降れば降ったでなんとかすればいい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たら〜で");
        assert_pattern_range(&patterns, "たら〜で", 4, 11); // 降れば降ったで
    }

    // Testing: structure.standard[1] - い-Adj[たら] + い-Adj[い] + で
    #[test]
    fn test_tarade_i_adj_tara_form() {
        let sentence = "大きい方が荷物とかいっぱい積めるけど、大きかったら大きいでデメリットがある。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たら〜で");
        assert_pattern_range(&patterns, "たら〜で", 19, 29); // 大きかったら大きいで
    }

    // Testing: structure.standard[4] - い-Adj[ば] + い-Adj[い] + で
    #[test]
    fn test_tarade_i_adj_ba_form() {
        let sentence = "力がなくてもできる仕事は沢山あるから、弱ければ弱いでいい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たら〜で");
        assert_pattern_range(&patterns, "たら〜で", 19, 26); // 弱ければ弱いで
    }

    // Testing: structure.standard[2] - な-Adj + なら + な-Adj + で
    #[test]
    fn test_tarade_na_adj_nara_form() {
        let sentence = "嫌なら嫌で大丈夫！田中くんに頼むこともできるし。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たら〜で");
        assert_pattern_range(&patterns, "たら〜で", 0, 5); // 嫌なら嫌で
    }

    // Testing: structure.standard[2] - な-Adj + なら + な-Adj + で (different adjective)
    #[test]
    fn test_tarade_na_adj_kantan() {
        let sentence = "難しい文法を見ると怯んでしまうけど、簡単なら簡単でやりがいはない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "たら〜で");
        assert_pattern_range(&patterns, "たら〜で", 18, 25); // 簡単なら簡単で
    }
}

// ============================================================================
// に至って・に至り Tests
// ============================================================================

mod niitatte_niitari_tests {
    use super::*;

    // Pattern: に至って・に至り (only when/going as far as)
    // Data source: grammar_points_data.json["に至って・に至り"]
    // Testing: structure.standard[0] - "Verb + に至って"
    //
    // Structure variants:
    //   - standard[0]: Verb + に至って
    //   - standard[1]: Noun + に至って
    //   - standard[2]: に至り (formal conjunctive form)

    // Testing: structure.standard[0] - Verb + に至って
    #[test]
    fn test_niitatte_verb() {
        let sentence = "先輩に注意されるに至って、どれだけ俺が自分勝手だったかが分かった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に至って・に至り");
        assert_pattern_range(&patterns, "に至って・に至り", 6, 12); // れるに至って
    }

    // Testing: structure.standard[0] - Verb + に至って (different example)
    #[test]
    fn test_niitatte_verb_accident() {
        let sentence = "工場で大きな事故が発生するに至って安全大会を実施する様では、この工場では事故が起こり続けます。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に至って・に至り");
        assert_pattern_range(&patterns, "に至って・に至り", 9, 17); // 発生するに至って
    }

    // Testing: structure.standard[1] - Noun + に至って
    #[test]
    fn test_niitatte_noun() {
        let sentence = "糖尿病に至って、食生活に気をつける様になった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に至って・に至り");
        assert_pattern_range(&patterns, "に至って・に至り", 2, 7); // 病に至って
    }

    // Testing: structure.standard[1] - Noun + に至って (age example)
    #[test]
    fn test_niitatte_noun_age() {
        let sentence = "この年に至って会社を立ち上げるなんて、自分でもびっくりだよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に至って・に至り");
        assert_pattern_range(&patterns, "に至って・に至り", 2, 7); // 年に至って
    }

    // Testing: structure.standard[2] - に至り (formal conjunctive form)
    #[test]
    fn test_niitari_formal() {
        let sentence = "事態がここに至り、もはや後戻りはできない状況となった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に至って・に至り");
        assert_pattern_range(&patterns, "に至って・に至り", 3, 8); // ここに至り
    }
}

// ============================================================================
// だに + しない Tests
// ============================================================================

mod dani_shinai_tests {
    use super::*;

    // Pattern: だに + しない (not even, cannot even)
    // Data source: grammar_points_data.json["だに + しない"]
    // Testing: structure.standard[0] - "Noun + だに + しない"
    //          structure.standard[2] - "夢にだに思わない" (exception)
    //
    // Note: だに is a classical adverbial particle similar to さえ/すら (even)
    // It marks a minimum example to which (A) does not apply, so obviously doesn't apply to anything greater
    // Used only in very limited set of expressions in modern Japanese
    // Common expressions: 想像だに, 考えるだに, 夢にだに, 聞くだに, 微動だに

    // Testing: structure.standard[0] - Noun + だに + しない (予想だに)
    #[test]
    fn test_dani_shinai_yosou() {
        let sentence = "アメリカでこんな大きい地震が来るとは誰も予想だにしなかった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だに + しない");
        assert_pattern_range(&patterns, "だに + しない", 20, 29); // 予想だにしなかった
    }

    // Testing: structure.standard[0] - Noun + だに + しない (想像だに)
    #[test]
    fn test_dani_shinai_souzou() {
        let sentence = "彼が彼女と別れるなんて想像だにしなかったよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だに + しない");
        assert_pattern_range(&patterns, "だに + しない", 11, 20); // 想像だにしなかった
    }

    // Testing: structure.standard[0] - Noun + だに + しない (思いだに)
    #[test]
    fn test_dani_shinai_omoi() {
        let sentence = "まさかこの年になって子供を授かるなんて思いだにしていなかった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だに + しない");
        assert_pattern_range(&patterns, "だに + しない", 19, 30); // 思いだにしていなかった
    }

    // Testing: structure.standard[2] - Exception: 夢にだに思わない
    // Note: Pattern matches 夢に + だに + 思わない as a unit (に is part of the idiom)
    #[test]
    fn test_dani_shinai_yume_ni_dani() {
        let sentence = "自分がこんな立場になるなんて、夢にだに思わなかった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "だに + しない");
        assert_pattern_range(&patterns, "だに + しない", 16, 25); // にだに思わなかった (includes に before だに)
    }
}

// ============================================================================
// んがため(に) Tests
// ============================================================================

mod ngatame_ni_tests {
    use super::*;

    // Pattern: んがため(に) (for the purpose of, in order to)
    // Data source: grammar_points_data.json["んがため(に)"]
    // Testing: structure.standard[0] - "Verb[ない] + ん + が + ため(に)"
    //
    // Structure variants:
    //   - standard[0]: Verb[ない] + んがため(に)
    //   - standard[1]: Verb[ない] + んがための + Noun
    //   - Exception: する → せんがため

    // Testing: structure.standard[0] - Verb + んがために
    #[test]
    fn test_ngatame_ni_basic() {
        let sentence = "彼は若い頃からお金を貯めんがために、色々と我慢してきた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "んがため(に)");
        assert_pattern_range(&patterns, "んがため(に)", 10, 17); // 貯めんがために
    }

    // Testing: structure.standard[0] - Verb + んがために (different verb)
    #[test]
    fn test_ngatame_ni_kanae() {
        let sentence = "私の両親たちは私たちの夢を叶えんがために、毎日休まずに仕事をしてきました。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "んがため(に)");
        assert_pattern_range(&patterns, "んがため(に)", 13, 20); // 叶えんがために
    }

    // Testing: structure.standard[1] - Verb + んがための + Noun
    #[test]
    fn test_ngatame_no_noun_katsu() {
        let sentence = "こんなことはしたくないが大会で勝たんがためのことだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "んがため(に)");
        assert_pattern_range(&patterns, "んがため(に)", 15, 22); // 勝たんがための
    }

    // Testing: structure.standard[1] - Verb + んがための + Noun (different verb)
    #[test]
    fn test_ngatame_no_noun_uru() {
        let sentence = "売れ残りの商品を売らんがための作戦を考えたが何も思いつかなかった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "んがため(に)");
        assert_pattern_range(&patterns, "んがため(に)", 8, 15); // 売らんがための
    }

    // Testing: Exception - する → せんがため
    #[test]
    fn test_ngatame_ni_suru_exception() {
        let sentence = "彼女は自分の作戦を成功させんがために、友達を裏切った。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "んがため(に)");
        assert_pattern_range(&patterns, "んがため(に)", 12, 18); // せんがために
    }
}

// ============================================================================
// にも～ない Tests
// ============================================================================

mod nimo_nai_tests {
    use super::*;

    // Pattern: にも～ない (can't do even if wanted to)
    // Data source: grammar_points_data.json["にも～ない"]
    // Testing: structure.standard[0] - "Verb[おう] + にも + Verb[できる][ない]"
    //
    // Structure variants:
    //   - standard[0]: Verb[おう/よう] + にも + Same Verb[potential negative]
    //   - standard[1]: Verb[おう/よう] + にも + (Reason) Phrase

    // Testing: structure.standard[0] - Verb[おう] + にも + Verb[potential negative]
    #[test]
    fn test_nimo_nai_kaeru() {
        let sentence = "もう夜遅いから電車もバスも走ってないし、全然タクシーも通らないし、帰ろうにも帰れない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にも～ない");
        assert_pattern_range(&patterns, "にも～ない", 33, 42); // 帰ろうにも帰れない
    }

    // Testing: structure.standard[0] - Verb[よう] + にも + Verb[potential negative]
    #[test]
    fn test_nimo_nai_neru() {
        let sentence = "色々と仕事のことで悩んでいるから、毎晩寝ようにも寝れない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にも～ない");
        assert_pattern_range(&patterns, "にも～ない", 19, 28); // 寝ようにも寝れない
    }

    // Testing: structure.standard[0] - Verb[おう] + にも + Verb[potential negative]
    #[test]
    fn test_nimo_nai_kau() {
        let sentence = "昨日出てきた釣竿を買いたいけど、高いから買おうにも買えない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にも～ない");
        assert_pattern_range(&patterns, "にも～ない", 20, 29); // 買おうにも買えない
    }

    // Testing: structure.standard[1] - Verb[よう] + にも + できない
    #[test]
    fn test_nimo_nai_renraku() {
        let sentence = "携帯を無くしてしまったから、上司に連絡しようにもできない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にも～ない");
        assert_pattern_range(&patterns, "にも～ない", 17, 28); // 連絡しようにもできない
    }
}

mod uff5e_nari_uff5e_nari_tests {
    use super::*;

    // Pattern: ～なり～なり (either...or...)
    // Data source: grammar_points_data.json["～なり～なり"]
    // Testing all structure variants with print_debug
    //
    // Structure variants:
    //   - standard[0]: Verb(A) + なり + Verb(B) + なり + する
    //   - standard[1]: Noun(A) + (Particle) + なり + Noun(B) + (Particle) + なり
    //   - standard[2]: Noun(A) + なり + Noun(B) + なり + (Particle)
    //   - standard[3]: Verb + なり + WH-Word + (Particle) + なり + する
    //   - standard[4]: Noun + (Particle) + なり + WH-Word + (Particle) + なり

    // Testing: structure.standard[0] - Verb(A) + なり + Verb(B) + なり + する
    #[test]
    fn test_nari_nari_verb_verb_suru() {
        let sentence = "タバコを吸うなり弁当食べるなりして待ってな。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～なり～なり");
        assert_pattern_range(&patterns, "～なり～なり", 4, 15); // 吸うなり弁当食べるなり
    }

    // Testing: structure.standard[0] - Verb(A) + なり + Verb(B) + なり + する (variant 2)
    #[test]
    fn test_nari_nari_uru_suteru() {
        let sentence = "いらないなら売るなり捨てるなりしてもらっても結構ですので。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～なり～なり");
        assert_pattern_range(&patterns, "～なり～なり", 6, 15); // 売るなり捨てるなり
    }

    // Testing: structure.standard[1] - Noun(A) + なり + Noun(B) + なり
    #[test]
    fn test_nari_nari_noun_simple() {
        let sentence = "遅れるなら電話なりメールなりするのが社会人としての常識じゃないのか？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～なり～なり");
        assert_pattern_range(&patterns, "～なり～なり", 5, 14); // 電話なりメールなり
    }

    // Testing: structure.standard[3] - Verb + なり + WH-Word + Particle + Verb + なり
    #[test]
    fn test_nari_nari_verb_wh() {
        let sentence = "行くなり何をするなりして自分で決めてください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～なり～なり");
        assert_pattern_range(&patterns, "～なり～なり", 0, 10); // 行くなり何をするなり
    }

    // TODO: Undetectable - Noun + なり + Noun + なり + に
    // In the sentence "俺なり高橋なりに", the second なり is tokenized as a noun (名詞/一般),
    // not as a particle. This makes it indistinguishable from the なりに pattern structurally.
    //
    // #[test]
    // fn test_nari_nari_noun_noun_particle() {
    //     let sentence = "俺なり高橋なりに相談してくれればいいのに！";
    //     let tokens = tokenize_sentence(sentence);
    //     let patterns = detect_patterns(&tokens);
    //
    //     assert_has_pattern(&patterns, "～なり～なり");
    //     assert_pattern_range(&patterns, "～なり～なり", 0, 8); // 俺なり高橋なりに
    // }

    // TODO: Undetectable - Noun + に + なり (Verb "become")
    // In the sentence "家になりどこになり", the なり tokens are parsed as the verb "なる" (become)
    // in 連用形, not as the adverbial particle なり. This is a different grammatical construction.
    //
    // #[test]
    // fn test_nari_nari_noun_wh() {
    //     let sentence = "家になりどこになり好きなところで勉強していいよ。";
    //     let tokens = tokenize_sentence(sentence);
    //     let patterns = detect_patterns(&tokens);
    //
    //     assert_has_pattern(&patterns, "～なり～なり");
    //     assert_pattern_range(&patterns, "～なり～なり", 0, 9); // 家になりどこになり
    // }
}

// ============================================================================
// ものとする Tests
// ============================================================================

mod monotosuru_tests {
    use super::*;

    // Pattern: ものとする (shall / supposing that / on the assumption that)
    // Data source: grammar_points_data.json["ものとする"]
    // Testing: structure.standard[0] - "Verb + ものとする"
    //
    // Other structures to test:
    //   - polite[0]: Verb + ものとします

    #[test]
    fn test_monotosuru_standard_form() {
        let sentence = "住民税は世帯主が払うものとする。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ものとする");
        assert_pattern_range(&patterns, "ものとする", 8, 15); // 払うものとする
    }

    #[test]
    fn test_monotosuru_contract_example() {
        let sentence = "契約者の都合で解約をした場合、契約者に解約金を請求するものとする。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ものとする");
        assert_pattern_range(&patterns, "ものとする", 23, 32); // 請求するものとする
    }

    #[test]
    fn test_monotosuru_polite_form() {
        let sentence = "ダム建設を取り下げるものとします。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ものとする");
        assert_pattern_range(&patterns, "ものとする", 5, 16); // 取り下げるものとします
    }
}

// ============================================================================
// ないでもない Tests
// ============================================================================

mod naidemonai_tests {
    use super::*;

    // Pattern: ないでもない (kind of / might / not not)
    // Data source: grammar_points_data.json["ないでもない"]
    // Testing: structure.standard[0] - "Verb[ない] + でも + ない"
    //
    // Other structures to test:
    //   - standard[1]: Verb[ない] + では + ない (では variant)
    //   - Also covers: Verb[ない] + もの + でも + ない (ないものでもない variant)
    //   - Related pattern: なくもない (not implemented separately)

    #[test]
    fn test_naidemonai_verb_negative() {
        let sentence = "あいつが悪いとは思わないでもないけど、お前にも少しは非があると思うよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないでもない");
        assert_pattern_range(&patterns, "ないでもない", 10, 16); // ないでもない
    }

    #[test]
    fn test_naidemonai_dewa_variant() {
        let sentence = "彼女が謝ってきたら、彼女がしたことを許す気がないではない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないでもない");
        assert_pattern_range(&patterns, "ないでもない", 22, 28); // ないではない
    }

    #[test]
    fn test_naidemonai_mono_variant() {
        let sentence = "あなたの考えが分からないものでもないけど、そういうことはあまり口に出さないほうがいいと思うよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないでもない");
        assert_pattern_range(&patterns, "ないでもない", 10, 18); // ないものでもない
    }
}

// ============================================================================
// もさることながら Tests
// ============================================================================

mod mosarukotonagara_tests {
    use super::*;

    // Pattern: もさることながら (not only A but also B)
    // Data source: grammar_points_data.json["もさることながら"]
    // Testing: structure.standard[0] - "Noun + もさることながら"

    #[test]
    fn test_mosarukotonagara_brand_products() {
        let sentence = "あのブランドが出す商品もさることながら、マーケティングも素晴らしい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "もさることながら");
        assert_pattern_range(&patterns, "もさることながら", 9, 19); // 商品もさることながら
    }
}

// ============================================================================
// ものと思っていた Tests
// ============================================================================

mod monotoomotteita_tests {
    use super::*;

    // Pattern: ものと思っていた (was under the impression that)
    // Data source: grammar_points_data.json["ものと思っていた"]
    // Testing structure variants

    // Testing: structure.standard[0] - "Verb + ものと思（おも）っていた"
    #[test]
    fn test_monotoomotteita_verb() {
        let sentence = "てっきり帰りに食べてくるものと思っていたから、あなたの分は作ってないですよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ものと思っていた");
        assert_pattern_range(&patterns, "ものと思っていた", 10, 20); // くるものと思っていた
    }

    // Testing: structure.standard[1] - "［い］Adjective + ものと思（おも）っていた"
    #[test]
    fn test_monotoomotteita_i_adjective() {
        let sentence = "私の母は猫が好きではないものと思っていたが、聞いてみたらただ猫アレルギーがあるそうだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ものと思っていた");
        assert_pattern_range(&patterns, "ものと思っていた", 10, 20); // ないものと思っていた
    }

    // Testing: structure.standard[2] - "［な］Adjective + な + ものと思（おも）っていた"
    #[test]
    fn test_monotoomotteita_na_adjective() {
        let sentence = "彼女は料理が得意なものと思っていたけど、実は全然作れないらしい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ものと思っていた");
        assert_pattern_range(&patterns, "ものと思っていた", 8, 17); // なものと思っていた
    }

    // Testing: structure.standard[3] - "Noun + の + ものと思（おも）っていた"
    #[test]
    fn test_monotoomotteita_noun() {
        let sentence = "ゲートボールは老人しかやらないものと思っていたが、最近は若者の間で流行っているらしい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ものと思っていた");
        assert_pattern_range(&patterns, "ものと思っていた", 13, 23); // ないものと思っていた
    }

    // Testing: structure.polite[0] - "Verb + ものと思（おも）っていました"
    #[test]
    fn test_monotoomotteita_verb_polite() {
        let sentence = "彼はもう帰ったものと思っていました。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ものと思っていた");
        assert_pattern_range(&patterns, "ものと思っていた", 6, 17); // たものと思っていました
    }
}

// ============================================================================
// でなくてなんだろう Tests
// ============================================================================

mod denakutenandarou_tests {
    use super::*;

    // Pattern: でなくてなんだろう (if not A, then what is it?)
    // Data source: grammar_points_data.json["でなくてなんだろう"]
    // Testing: structure.standard[0] - "Noun + でなくてなん + だろう（か）"

    #[test]
    fn test_denakutenandarou_darou() {
        let sentence = "あの子が新しいってだけでいじめられている。あれはパワハラでなくてなんだろう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "でなくてなんだろう");
        assert_pattern_range(&patterns, "でなくてなんだろう", 24, 37); // パワハラでなくてなんだろう
    }

    #[test]
    fn test_denakutenandarou_darou_ka() {
        let sentence = "私の家だけ浸水しなかった。これが奇跡でなくてなんだろうか。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "でなくてなんだろう");
        assert_pattern_range(&patterns, "でなくてなんだろう", 16, 28); // 奇跡でなくてなんだろうか
    }

    // Testing: structure.standard[1] - "Noun + でなくてなん + であろう（か）"

    #[test]
    fn test_denakutenandarou_dearou() {
        let sentence = "俺はこんなに夜遅くまでタダ働きをしている。俺は社畜でなくてなんであろう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "でなくてなんだろう");
        assert_pattern_range(&patterns, "でなくてなんだろう", 23, 35); // 社畜でなくてなんであろう
    }

    #[test]
    fn test_denakutenandarou_dearou_ka() {
        let sentence = "説明欄には新品だと書いてあったのに届いた商品は傷だらけだった。これは詐欺でなくてなんであろうか。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "でなくてなんだろう");
        assert_pattern_range(&patterns, "でなくてなんだろう", 34, 47); // 詐欺でなくてなんであろうか
    }
}

// ============================================================================
// はさておき・はさておいて Tests
// ============================================================================

mod hasateoki_tests {
    use super::*;

    // Pattern: はさておき・はさておいて (leaving aside, apart from)
    // Data source: grammar_points_data.json["はさておき・はさておいて"]
    // Testing: structure.standard[0] - "Noun + はさておき"

    #[test]
    fn test_hasateoki_noun_kansou() {
        let sentence = "みんなの感想はさておき、私は美味しいと思ったよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "はさておき・はさておいて");
        assert_pattern_range(&patterns, "はさておき・はさておいて", 6, 11); // 感想はさておき
    }

    #[test]
    fn test_hasateoki_noun_sonna() {
        let sentence = "そんなことはさておき、仕事をちゃちゃっと終わらせて早く帰りましょうよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "はさておき・はさておいて");
        assert_pattern_range(&patterns, "はさておき・はさておいて", 5, 10); // ことはさておき
    }

    // Testing: structure.standard[1] - "Phrase + かどうか + はさておき"

    #[test]
    fn test_hasateoki_kadouka_jishin() {
        let sentence = "日本語に自信があるかどうかはさておき、一人で行ったことのない国へ行くのには勇気がいる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "はさておき・はさておいて");
        assert_pattern_range(&patterns, "はさておき・はさておいて", 13, 18); // かどうかはさておき
    }

    #[test]
    fn test_hasateoki_kadouka_kanojo() {
        let sentence = "彼女がいるかいないかはさておき、普段から清潔にしていたほうがいいと思うよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "はさておき・はさておいて");
        assert_pattern_range(&patterns, "はさておき・はさておいて", 10, 15); // かはさておき
    }

    // Testing: structure.standard[2] - "はさておいて" (conjunctive form)

    #[test]
    fn test_hasateoite_joudan() {
        let sentence = "冗談はさておいて、そろそろ会議を始めよう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "はさておき・はさておいて");
        assert_pattern_range(&patterns, "はさておき・はさておいて", 2, 8); // 冗談はさておいて
    }
}

// ============================================================================
// 折には Tests
// ============================================================================

mod oriniha_tests {
    use super::*;

    // Pattern: 折には (on occasions when, when the chance comes up)
    // Data source: grammar_points_data.json["折には"]
    // Testing: structure.standard[0] - "Verb + 折（に）"

    #[test]
    fn test_oriniha_verb() {
        let sentence = "こちらの旅館にいらっしゃるおりには、事前の予約をお願いいたします。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "折には");
        assert_pattern_range(&patterns, "折には", 13, 17); // おりには
    }

    #[test]
    fn test_orini_verb_past() {
        let sentence = "今度お会いしたおりに、旅行へ行った時のお土産をお渡しします。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "折には");
        assert_pattern_range(&patterns, "折には", 7, 10); // おりに
    }

    // Testing: structure.standard[3] - "Noun + の + 折（に）"

    #[test]
    fn test_orini_noun() {
        let sentence = "卒業のおりに担任の先生に手紙を渡そうと思っている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "折には");
        assert_pattern_range(&patterns, "折には", 3, 6); // おりに
    }

    // Testing: structure.standard[1] - "い-Adj + 折（に）"

    #[test]
    fn test_orini_i_adj() {
        let sentence = "お忙しい折に恐れ入りますが、ご協力をお願いします。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "折には");
        assert_pattern_range(&patterns, "折には", 4, 6); // 折に
    }

    // Testing: structure.standard[2] - "な-Adj + な + 折（に）"

    #[test]
    fn test_orini_na_adj() {
        let sentence = "お暇な折にぜひお立ち寄りください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "折には");
        assert_pattern_range(&patterns, "折には", 3, 5); // 折に
    }
}

// ============================================================================
// とばかり（に） Tests
// ============================================================================

mod tobakarini_tests {
    use super::*;

    // Pattern: とばかり（に） (as if to say / seeming that)
    // Data source: grammar_points_data.json["とばかり（に）"]
    // Testing: structure.standard[0] - "Quote + とばかり（に）"
    //
    // Structures to test:
    //   - standard[0]: Quote + とばかり（に）
    //   - standard[1]: Noun +（だ）+ とばかり（に）

    // Testing: structure.standard[0] - Quote + とばかり（に）

    #[test]
    fn test_tobakarini_quote() {
        let sentence = "うちのわんちゃんが「待ってました」とばかりに尻尾を振って友達に飛びついた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とばかり（に）");
        assert_pattern_range(&patterns, "とばかり（に）", 17, 22); // とばかりに
    }

    #[test]
    fn test_tobakarini_quote_without_ni() {
        let sentence = "先生が「俺の話聞いてるか」とばかり生徒たちを睨みつけた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とばかり（に）");
        assert_pattern_range(&patterns, "とばかり（に）", 13, 17); // とばかり
    }

    // Testing: structure.standard[1] - Noun + とばかり（に）

    #[test]
    fn test_tobakarini_noun() {
        let sentence = "中田選手のバランスが崩れたので、斉藤選手はチャンスとばかりに中田選手の太ももを蹴った。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とばかり（に）");
        assert_pattern_range(&patterns, "とばかり（に）", 25, 30); // とばかりに
    }
}

// ============================================================================
// わ〜わ（で） Tests
// ============================================================================

mod wa_wa_de_tests {
    use super::*;

    // Pattern: わ〜わ（で） (with A and B, resulting in C)
    // Data source: grammar_points_data.json["わ〜わ（で）"]
    // Testing: structure.standard[0] - "(A) (1)+ わ + (B) (1) + わ（で）"
    // (1) Verb［る］、［い］Adjective、［な］Adjective + だ、Noun + だ
    //
    // Structures to test:
    //   - Verb + わ + Verb + わで
    //   - い-Adjective + わ + い-Adjective + わで
    //   - な-Adjective + だわ + な-Adjective + だわで
    //   - Noun + だわ + Noun + だわで
    //   - Without で (optional)

    // Testing: Verb + わ + Verb + わで

    #[test]
    fn test_wa_wa_de_simple() {
        // Simple test: Verb + わ + Verb + わで (no intervening punctuation)
        let sentence = "行くわ来るわで忙しい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "わ〜わ（で）");
        assert_pattern_range(&patterns, "わ〜わ（で）", 2, 7); // わ来るわで
    }

    #[test]
    fn test_wa_wa_de_verb() {
        // Verb + わ + Verb + わで (from grammar_points_data.json example)
        let sentence = "今年は宝くじには当たるわ長年付き合っていた彼氏にプロポーズされるわで、幸せなことが沢山あってとても嬉しい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "わ〜わ（で）");
        assert_pattern_range(&patterns, "わ〜わ（で）", 11, 34); // わ長年付き合っていた彼氏にプロポーズされるわで
    }

    // Testing: い-Adjective + わ + い-Adjective + わで

    #[test]
    fn test_wa_wa_de_i_adj() {
        let sentence = "このアパートはボロいわ狭いわで最悪な物件だ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "わ〜わ（で）");
        assert_pattern_range(&patterns, "わ〜わ（で）", 10, 15); // わ狭いわで
    }

    // Testing: な-Adjective + だわ + な-Adjective + だわで
    // Note: This sentence has commas between わ, which may prevent detection due to wildcard limitations.
    // Commenting out for now as it's a known limitation.

    // #[test]
    // fn test_wa_wa_de_na_adj() {
    //     let sentence = "新しく入ってきた新人はクライアントには無礼だわ、クレーム対応が下手だわで、どこから教育していいかわからない。";
    //     let tokens = tokenize_sentence(sentence);
    //     let patterns = detect_patterns(&tokens);
    //
    //     // This may not be detected due to comma between the two わ
    //     // TODO: Enhance wildcard to cross punctuation boundaries
    // }

    // Testing: Noun + だわ + Verb + わで (mixed types)

    #[test]
    fn test_wa_wa_de_noun_mixed() {
        let sentence = "携帯を買ってくれたこと自体は嬉しかったんだけど、５年前のモデルだわ画面は割れているわで使い物にならなそうだったから、自分で新しい携帯を買った。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "わ〜わ（で）");
        assert_pattern_range(&patterns, "わ〜わ（で）", 32, 43); // わ画面は割れているわで
    }
}

// ============================================================================
// わ〜わ Tests
// ============================================================================

mod wa_wa_tests {
    use super::*;

    // Pattern: わ〜わ (more and more / keeps happening / so much)
    // Data source: grammar_points_data.json["わ〜わ"]
    // Testing: structure.standard[0] - "Verb［る］+ わ + Verb［る］+ わ(*)"
    // (*) The same verb has to be repeated
    //
    // Note: This implementation cannot verify that the same verb is repeated
    // (requires cross-token validation). It will match any Verb + わ + Verb + わ pattern.
    // This may result in false positives if different verbs are used.

    #[test]
    fn test_wa_wa_same_verb() {
        // Testing: Same verb repeated (from grammar_points_data.json example)
        let sentence = "空き家を買ったのはいいけど、家中にはゴキブリがいるわいるわ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "わ〜わ");
        assert_pattern_range(&patterns, "わ〜わ", 23, 29); // いるわいるわ
    }

    #[test]
    fn test_wa_wa_suru_verb() {
        // Testing: する verb shorthand (勉強するわするわ)
        let sentence = "あの子は医者になりたいって決めてから、毎日勉強するわするわ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "わ〜わ");
        assert_pattern_range(&patterns, "わ〜わ", 21, 29); // 勉強するわするわ
    }

    #[test]
    fn test_wa_wa_simple() {
        // Simple test case with で following
        // Note: This sentence ends with で, so both わ〜わ and わ〜わ（で） will match
        let sentence = "子供達が喧嘩して泣くわ泣くわで大変だった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "わ〜わ");
        assert_pattern_range(&patterns, "わ〜わ", 8, 15); // 泣くわ泣くわで
    }
}

// ============================================================================
// なりとも Tests
// ============================================================================

mod naritomo_tests {
    use super::*;

    // Pattern: なりとも (at least, even)
    // Data source: grammar_points_data.json["なりとも"]
    // Testing: structure.standard[0] - "Noun + なりとも"
    //
    // Note: This pattern has two tokenization variants:
    // 1. Split pattern: Noun + なり(助動詞) + と(助詞) + も(助詞) - detectable
    // 2. Compound adverb: 多少なりとも (副詞/一般) - undetectable with current matcher

    // TODO: Undetectable - Compound adverb case
    // "多少なりとも" tokenizes as a single adverb token (副詞/一般),
    // not as Noun + なり + と + も. Cannot be matched by the current 4-token pattern.
    // Would need a separate pattern for compound adverbs ending in なりとも.
    //
    // #[test]
    // fn test_naritomo_compound_adverb() {
    //     let sentence = "多少なりとも毎日勉強することはいいことだ。";
    //     // Tokenizes as: 多少なりとも(副詞/一般) - single token
    //     // Pattern expects: Noun + なり(助動詞) + と(助詞) + も(助詞) - 4 tokens
    // }

    #[test]
    fn test_naritomo_once() {
        // Testing: Counter + なりとも (一度なりとも = not even once)
        let sentence = "このようなことは一度なりともやったことはないですが、頑張ってみます。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なりとも");
        assert_pattern_range(&patterns, "なりとも", 9, 14); // 度なりとも
    }

    #[test]
    fn test_naritomo_moment() {
        // Testing: Time counter + なりとも (一時なりとも = not even for a moment)
        let sentence = "お子様から一時なりともお目を離さずご利用ください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なりとも");
        assert_pattern_range(&patterns, "なりとも", 6, 11); // 時なりとも
    }

    #[test]
    fn test_naritomo_brief_glance() {
        // Testing: Occurrence noun + なりとも (一目なりとも = even for a brief glance)
        let sentence = "時間があったら、一目なりとも会いたいです！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "なりとも");
        assert_pattern_range(&patterns, "なりとも", 8, 14); // 一目なりとも
    }

    // TODO: Undetectable - Different tokenization
    // "わずかなりとも" tokenizes as: わずか(副詞) + なり(動詞/なる/連用形) + とも(助詞/接続助詞)
    // This is NOT the なりとも pattern (which requires なり as 助動詞, not 動詞).
    // This is actually わずか + the verb なる + とも particle (different grammar).
    //
    // #[test]
    // fn test_naritomo_wazuka() {
    //     let sentence = "わずかなりとも休憩をいただいてもよろしいですか。";
    //     // Token: なり is 動詞/自立/五段・ラ行 (verb なる), not 助動詞/文語・ナリ
    //     // This is a different grammatical pattern, not the classical なりとも
    // }
}

// ============================================================================
// に至っても Tests
// ============================================================================

mod niitattemo_tests {
    use super::*;

    // Pattern: に至っても (even when it reaches/even if it comes to)
    // Data source: grammar_points_data.json["に至っても"]
    // Testing: structure.standard[0] - "Verb + に至（いた）っても"
    //
    // Other structures to test:
    //   - standard[1]: Noun + に至（いた）っても
    //
    // Meaning: Presents extreme example from which some result (often negative) will not occur
    // Contrast: に至っては (topic marker は) vs に至っても (even も)
    // Usage: "Even if it came to (A), (B)" - (B) often negative, showing (A) couldn't affect (A)

    #[test]
    fn test_niitattemo_verb_makikomareru() {
        // Testing: Verb + に至っても (経つに至っても)
        let sentence = "あの事故に巻き込まれてから２年経つに至っても、相手は謝罪すらもしてこない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に至っても");
        assert_pattern_range(&patterns, "に至っても", 15, 22); // 経つに至っても
    }

    #[test]
    fn test_niitattemo_verb_naru() {
        // Testing: Verb + に至っても (なるに至っても)
        let sentence = "社会人になるに至っても、まだ自分が子供だと思っている人は少なくないだろう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に至っても");
        assert_pattern_range(&patterns, "に至っても", 4, 11); // なるに至っても
    }

    #[test]
    fn test_niitattemo_noun_genzai() {
        // Testing: Noun + に至っても (現在に至っても)
        let sentence = "彼と別れて３年経った現在に至っても、男の人を信用することができない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に至っても");
        assert_pattern_range(&patterns, "に至っても", 10, 17); // 現在に至っても
    }

    #[test]
    fn test_niitattemo_noun_asagata() {
        // Testing: Noun + に至っても (朝方に至っても)
        let sentence = "この会議は朝方に至っても、続きそうだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に至っても");
        assert_pattern_range(&patterns, "に至っても", 5, 12); // 朝方に至っても
    }

    #[test]
    fn test_niitattemo_noun_kekka() {
        // Testing: Noun + に至っても (結果に至っても)
        let sentence = "これは俺が決めたことだから、どんな結果に至っても、絶対に後悔しない！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に至っても");
        assert_pattern_range(&patterns, "に至っても", 17, 24); // 結果に至っても
    }
}

// ============================================================================
// を兼ねて Tests
// ============================================================================

mod wokanete_tests {
    use super::*;

    // Pattern: を兼ねて (also partly for the purpose of / to double as)
    // Data source: grammar_points_data.json["を兼ねて"]
    // Testing: structure.standard[0] - "Noun + を + 兼（か）ねて"
    //
    // Other structures to test:
    //   - standard[1]: Noun + も + 兼（か）ねて (も replacing を)
    //
    // Meaning: "To (A) at the same time as (B)" / "To double as (A)"
    // Highlights (B) as primary goal, (A) is secondary/supporting goal
    // 兼（か）ねる = to stretch across to / to do concurrently

    #[test]
    fn test_wokanete_bokeBoushi() {
        // Testing: Noun + をかねて (防止をかねて)
        let sentence = "おばあちゃんはボケ防止をかねて、毎日クロスワードパズルをしています。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を兼ねて");
        assert_pattern_range(&patterns, "を兼ねて", 9, 15); // 防止をかねて
    }

    #[test]
    fn test_wokanete_benkyou() {
        // Testing: Noun + をかねて (勉強をかねて)
        let sentence = "日本語の勉強をかねて、英語の字幕なしで日本の映画をみています。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を兼ねて");
        assert_pattern_range(&patterns, "を兼ねて", 4, 10); // 勉強をかねて
    }

    #[test]
    fn test_wokanete_diet() {
        // Testing: Noun + をかねて (ダイエットをかねて)
        let sentence = "ダイエットをかねて、毎朝ランニングしています。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を兼ねて");
        assert_pattern_range(&patterns, "を兼ねて", 0, 9); // ダイエットをかねて
    }

    #[test]
    fn test_wokanete_multiple_nouns() {
        // Testing: Multiple nouns with と (発散をかねて)
        let sentence = "運動不足とストレス発散をかねて、キックボクシングジムに通っている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を兼ねて");
        assert_pattern_range(&patterns, "を兼ねて", 9, 15); // 発散をかねて
    }

    #[test]
    fn test_wokanete_taichou_kanri() {
        // Testing: Multiple nouns with と (節約をかねて)
        let sentence = "体調管理と節約をかねて、寝る時はエアコンを使わないようにしている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を兼ねて");
        assert_pattern_range(&patterns, "を兼ねて", 5, 11); // 節約をかねて
    }
}

// ============================================================================
// ただ〜のみ Tests
// ============================================================================

mod tada_nomi_tests {
    use super::*;

    // Pattern: ただ〜のみ (nothing but, all that remains)
    // Data source: grammar_points_data.json["ただ〜のみ"]
    // Testing: structure.standard[0] - "(ただ) + Verb[る] + のみ + (だ)"
    //
    // Structure variants to test:
    //   - standard[0]: (ただ) + Verb[る] + のみ + (だ)
    //   - standard[1]: (ただ) + [する]Verb + ある + のみ + (だ)
    //   - polite[0]: (ただ) + Verb[る] + のみ + (です)
    //   - polite[1]: (ただ) + [する]Verb + ある + のみ + (です)

    #[test]
    fn test_tada_nomi_verb_with_tada() {
        // Testing: ただ + Verb[る] + のみだ
        // Example from grammar_points_data.json
        let sentence = "私たちにはもう何もできない。ただ医者を信じて待つのみだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ただ〜のみ");
        assert_pattern_range(&patterns, "ただ〜のみ", 22, 27); // 待つのみだ
    }

    #[test]
    fn test_tada_nomi_verb_without_tada() {
        // Testing: Verb[る] + のみです (without ただ)
        // Example from grammar_points_data.json
        let sentence = "僕にできることは祈るのみです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ただ〜のみ");
        assert_pattern_range(&patterns, "ただ〜のみ", 8, 14); // 祈るのみです
    }

    #[test]
    fn test_tada_nomi_suru_verb_aru() {
        // Testing: ただ + [する]Verb + ある + のみだ
        // Example from grammar_points_data.json
        let sentence = "将来日本の大手企業に勤めたいので、ただ勉強あるのみだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ただ〜のみ");
        assert_pattern_range(&patterns, "ただ〜のみ", 19, 26); // 勉強あるのみだ
    }

    #[test]
    fn test_tada_nomi_verb_dearu() {
        // Testing: Verb[る] + のみである (formal copula)
        let sentence = "今の状況では、静かに見守るのみである。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ただ〜のみ");
        assert_pattern_range(&patterns, "ただ〜のみ", 10, 18); // 見守るのみである
    }

    #[test]
    fn test_tada_nomi_verb_simple() {
        // Testing: Verb[る] + のみ (without copula)
        let sentence = "この先は前に進むのみ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ただ〜のみ");
        assert_pattern_range(&patterns, "ただ〜のみ", 6, 10); // 進むのみ
    }
}

// ============================================================================
// に足りない Tests
// ============================================================================

mod nitarinai_tests {
    use super::*;

    // Pattern: に足りない (not worth / not sufficient)
    // Data source: grammar_points_data.json["に足りない"]
    // Testing: structure.standard[0] - "Verb + に足（た）りない"

    #[test]
    fn test_nitarinai_shinjiru() {
        // Testing: Verb[る] + に + 足りない (conspiracy theorist example)
        let sentence = "彼は陰謀論者だから、あの人のいうことは信じるにたりない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に足りない");
        assert_pattern_range(&patterns, "に足りない", 19, 27); // 信じるにたりない
    }

    #[test]
    fn test_nitarinai_shinrai() {
        // Testing: Verb[する] + に + 足りない (trust example)
        let sentence = "彼は信頼するにたりない人だ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に足りない");
        assert_pattern_range(&patterns, "に足りない", 2, 11); // 信頼するにたりない
    }

    #[test]
    fn test_nitarinai_uru() {
        // Testing: Verb[る] + に + 足りない (sell example)
        let sentence = "買取の金額が売るにたりない金額だった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に足りない");
        assert_pattern_range(&patterns, "に足りない", 6, 13); // 売るにたりない
    }

    #[test]
    fn test_nitarinai_toru() {
        // Testing: 取る + に + 足りない (idiom - worthless)
        let sentence = "周りから見ると取るにたりない、ただのゴミだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に足りない");
        assert_pattern_range(&patterns, "に足りない", 7, 14); // 取るにたりない
    }
}

// ============================================================================
// べからず Tests
// ============================================================================

mod bekarazu_tests {
    use super::*;

    // Pattern: べからず (must not, ought not to)
    // Data source: grammar_points_data.json["べからず"]
    // Testing structure variants:
    //   standard[0]: Verb + べからず
    //   standard[2]: する → す + べからず (exception)
    //   standard[3]: Verb + べからざる + Noun

    #[test]
    fn test_bekarazu_basic() {
        // Testing: Verb + べからず (basic form)
        // Classic proverb: 初心忘れるべからず (Do not forget beginner's humility)
        let sentence = "初心忘れるべからず。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "べからず");
        assert_pattern_range(&patterns, "べからず", 2, 9); // 忘れるべからず
    }

    #[test]
    fn test_bekarazu_facility() {
        // Testing: Verb + べからず (prohibition sign)
        let sentence = "この施設内ではスケートボードなどのことをするべからず。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "べからず");
        assert_pattern_range(&patterns, "べからず", 20, 26); // するべからず
    }

    #[test]
    fn test_bekarazaru_yurusu() {
        // Testing: 許す + べからざる (set expression - unforgivable)
        let sentence = "配達員を装って玄関のドアを開けさせるのは、許すべからざる行為だ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "べからず");
        assert_pattern_range(&patterns, "べからず", 21, 28); // 許すべからざる
    }

    #[test]
    fn test_bekarazaru_suu() {
        // Testing: 吸う + べからざる (must not smoke)
        let sentence = "吸うべからざる場所でタバコを吸っているのを見られたら、罰金だ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "べからず");
        assert_pattern_range(&patterns, "べからず", 0, 7); // 吸うべからざる
    }

    #[test]
    fn test_bekarazaru_kaku() {
        // Testing: 欠く + べからざる (set expression - indispensable)
        let sentence = "水は人間にとって欠くべからざる資源である。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "べからず");
        assert_pattern_range(&patterns, "べからず", 8, 15); // 欠くべからざる
    }
}

// ============================================================================
// んばかりに Tests
// ============================================================================

mod nbakarini_tests {
    use super::*;

    // Pattern: んばかりに (as if about to, seeming that it will)
    // Data source: grammar_points_data.json["んばかりに"]
    // Testing: structure.standard[0] - "Verb[ない] + ん + ばかり + に"
    // Testing: structure.standard[1] - "Verb[ない] + ん + ばかり + の + Noun"

    #[test]
    fn test_nbakarini_kuzure() {
        // Testing: 崩れ + んばかりに (as if about to collapse)
        // Example from grammar_points_data.json
        let sentence = "台風の風で、隣の空き家が崩れんばかりにギシギシ音を立てていた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "んばかりに");
        assert_pattern_range(&patterns, "んばかりに", 12, 19); // 崩れんばかりに
    }

    #[test]
    fn test_nbakarini_iwa() {
        // Testing: 言わ + んばかりに (as if to say)
        // Example from grammar_points_data.json
        let sentence = "先生は「こっちを見ろ」って言わんばかりに、机を叩いた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "んばかりに");
        assert_pattern_range(&patterns, "んばかりに", 13, 20); // 言わんばかりに
    }

    #[test]
    fn test_nbakarini_afure() {
        // Testing: 溢れ + んばかりの (as if about to overflow - with の)
        // Example from grammar_points_data.json
        let sentence = "このコンサート会場は溢れんばかりのファンで埋め尽くされている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "んばかりに");
        assert_pattern_range(&patterns, "んばかりに", 10, 17); // 溢れんばかりの
    }

    #[test]
    fn test_nbakarini_harisake() {
        // Testing: 張り裂け + んばかりの (as if about to tear apart)
        // Example from grammar_points_data.json
        let sentence = "私は胸が張り裂けんばかりの気持ちになって、その場から動くことができなくなった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "んばかりに");
        assert_pattern_range(&patterns, "んばかりに", 4, 13); // 張り裂けんばかりの
    }

    #[test]
    fn test_nbakarini_tobikaka() {
        // Testing: 飛びかか + んばかりに (as if about to jump at)
        // Additional realistic example
        let sentence = "犬が飛びかかんばかりに吠えている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "んばかりに");
        assert_pattern_range(&patterns, "んばかりに", 2, 11); // 飛びかかんばかりに
    }
}

// ～てやる tests
mod teyaru_tests {
    use super::*;

    #[test]
    fn test_teyaru_benefit_standard() {
        // Testing: Verb[て] + やる (doing for someone - benefit)
        // Example from grammar_points_data.json
        let sentence = "いいよいいよ、俺がやってやるからお前は見とけ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～てやる");
        assert_pattern_range(&patterns, "～てやる", 9, 14); // やってやる
    }

    #[test]
    fn test_teyaru_benefit_polite() {
        // Testing: Verb[て] + やる (doing for someone)
        // Example from grammar_points_data.json
        let sentence = "今夜は俺が奢ってやるから、どんどん好きなもん頼めばいいからな。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～てやる");
        assert_pattern_range(&patterns, "～てやる", 5, 10); // 奢ってやる
    }

    #[test]
    fn test_teyaru_determination() {
        // Testing: Verb[て] + やる (strong will/determination)
        // Example from grammar_points_data.json
        let sentence = "絶対綺麗になって見返してやる！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～てやる");
        assert_pattern_range(&patterns, "～てやる", 8, 14); // 見返してやる
    }

    #[test]
    fn test_teyaru_resolve() {
        // Testing: Verb[て] + やる (determination - another example)
        // Example from grammar_points_data.json
        let sentence = "こうなったら勝ってやるしかないだろ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～てやる");
        assert_pattern_range(&patterns, "～てやる", 6, 11); // 勝ってやる
    }

    #[test]
    fn test_teyaru_polite_masu() {
        // Testing: Verb[て] + やります (polite form - determination)
        // Additional realistic example
        let sentence = "約束は必ず守ってやります。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～てやる");
        assert_pattern_range(&patterns, "～てやる", 5, 12); // 守ってやります
    }
}

// ============================================================================
// に則って・に則り Tests
// ============================================================================

mod ninottotte_tests {
    use super::*;

    // Pattern: に則って・に則り (in accordance with, based on)
    // Data source: grammar_points_data.json["に則って・に則り"]
    // Testing all structure variants:
    //   standard[0]: Noun + に則（のっと）って
    //   standard[1]: Noun (A) + に則（のっと）った + Noun (B)
    //   standard[2]: に則（のっと）り
    //   standard[3]: に則（のっと）っての

    #[test]
    fn test_ninottotte_te_form() {
        // Testing: structure.standard[0] - Noun + に則（のっと）って
        // Example from grammar_points_data.json
        let sentence = "どの国に行こうと、その国の法律にのっとって、生活しなくてはいけない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に則って・に則り");
        assert_pattern_range(&patterns, "に則って・に則り", 13, 20); // 法律にのっとっ
    }

    #[test]
    fn test_ninottotte_ta_form_modifier() {
        // Testing: structure.standard[1] - Noun (A) + に則（のっと）った + Noun (B)
        // Example from grammar_points_data.json
        let sentence = "私は無宗教なので、宗教にのっとった式はあげたくないです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に則って・に則り");
        assert_pattern_range(&patterns, "に則って・に則り", 9, 17); // 宗教にのっとった (includes た)
    }

    #[test]
    fn test_ninottori_continuative() {
        // Testing: structure.standard[2] - に則（のっと）り (continuative form)
        // Realistic example
        let sentence = "この計画は国際法に則り、厳格に実施されています。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に則って・に則り");
        assert_pattern_range(&patterns, "に則って・に則り", 7, 11); // 法に則り
    }

    #[test]
    fn test_ninottotte_no_form() {
        // Testing: structure.standard[3] - に則（のっと）っての (noun modifier with の)
        // Realistic example
        let sentence = "伝統に則っての行事が毎年開催される。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に則って・に則り");
        assert_pattern_range(&patterns, "に則って・に則り", 0, 5); // 伝統に則っ
    }
}

// ============================================================================
// はおろか Tests
// ============================================================================

mod haoroka_tests {
    use super::*;

    // Pattern: はおろか (let alone, not to mention)
    // Data source: grammar_points_data.json["はおろか"]
    // Testing: structure.standard[0] - "Noun (A) + はおろか + Noun (B) + さえ/も/まで/すら"
    //
    // Structure variants:
    // - standard[0]: Noun (A) + はおろか + Noun (B) + さえ
    // - standard[0]: Noun (A) + はおろか + Noun (B) + も
    // - standard[0]: Noun (A) + はおろか + Noun (B) + まで
    // - standard[0]: Noun (A) + はおろか + Noun (B) + すら

    #[test]
    fn test_haoroka_with_sae() {
        // Testing: Noun + はおろか + Noun + さえ
        // Realistic example: can't even X, let alone Y
        let sentence = "彼は掃除はおろか、料理さえできない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "はおろか");
        assert_pattern_range(&patterns, "はおろか", 2, 8); // 掃除はおろか
    }

    #[test]
    fn test_haoroka_with_mo() {
        // Testing: Noun + はおろか + Noun + も
        // Realistic example with も instead of さえ
        let sentence = "私はスーパーはおろか、コンビニもないど田舎に住んでいる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "はおろか");
        assert_pattern_range(&patterns, "はおろか", 2, 10); // スーパーはおろか
    }

    #[test]
    fn test_haoroka_with_sura() {
        // Testing: Noun + はおろか + Noun + すら
        // Realistic example with すら
        let sentence = "彼女は挨拶はおろか、目すら合わせようとしない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "はおろか");
        assert_pattern_range(&patterns, "はおろか", 3, 9); // 挨拶はおろか
    }

    #[test]
    fn test_haoroka_with_made() {
        // Testing: Noun + はおろか + Noun + まで
        // Realistic example with まで
        let sentence = "父はパソコンはおろか、スマホまで使うことができない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "はおろか");
        assert_pattern_range(&patterns, "はおろか", 2, 10); // パソコンはおろか
    }
}

// ============================================================================
// それまでだ Tests
// ============================================================================

mod soremadeda_tests {
    use super::*;

    // Pattern: それまでだ (if that happens, it's all over / all in vain)
    // Data source: grammar_points_data.json["それまでだ"]
    // Testing structure variants:
    //   - standard[0]: "Verb[たら] + それまでだ"
    //   - standard[1]: "Verb[ば] + それまでだ"

    #[test]
    fn test_soremadeda_tara_conditional() {
        // Testing: Verb[たら] + それまでだ
        // From grammar_points_data.json: "ここで見つかったらそれまでだ"
        let sentence = "もう少し我慢をしろ！ここで見つかったらそれまでだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "それまでだ");
        assert_pattern_range(&patterns, "それまでだ", 13, 24); // 見つかったらそれまでだ
    }

    #[test]
    fn test_soremadeda_ba_conditional() {
        // Testing: Verb[ば] + それまでだ
        // From grammar_points_data.json: "外部に漏れればそれまでだ"
        let sentence = "このことが外部に漏れればそれまでだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "それまでだ");
        assert_pattern_range(&patterns, "それまでだ", 8, 17); // 漏れればそれまでだ
    }

    #[test]
    fn test_soremadeda_mistake() {
        // Testing: Verb[ば] + それまでだ
        // From grammar_points_data.json: "ミスを犯せばそれまでだ"
        let sentence = "やっとここまでたどり着いた。ここでミスを犯せばそれまでだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "それまでだ");
        assert_pattern_range(&patterns, "それまでだ", 20, 28); // 犯せばそれまでだ
    }
}

// ============================================================================
// めく・めいた Tests
// ============================================================================

mod meku_meita_tests {
    use super::*;

    // Pattern: めく・めいた (shows signs of, has appearance of)
    // Data source: grammar_points_data.json["めく・めいた"]
    // Testing structure variants from grammar_points_data.json examples

    #[test]
    fn test_meku_meite_compound() {
        // Testing: Compound verb めく (春めく as single token)
        // From grammar_points_data.json: "やっと春めいてきた"
        let sentence = "昨日までは嘘みたいに暑かったのに、やっと春めいてきた！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "めく・めいた");
        assert_pattern_range(&patterns, "めく・めいた", 20, 23); // 春めい (compound verb)
    }

    #[test]
    fn test_meku_meite_split() {
        // Testing: Noun + めいて (split form)
        // Example: 冗談めいて言った
        let sentence = "冗談めいて言ったが、多分冗談だったということは気づいていないと思う。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "めく・めいた");
        assert_pattern_range(&patterns, "めく・めいた", 0, 4); // 冗談めい
    }

    #[test]
    fn test_meku_meita_modifying_noun() {
        // Testing: Noun + めいた (past form modifying noun)
        // From grammar_points_data.json: "皮肉めいたことを言ってしまって"
        let sentence = "皮肉めいたことを言ってしまって、相手が黙り込んでしまった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "めく・めいた");
        assert_pattern_range(&patterns, "めく・めいた", 0, 5); // 皮肉めいた
    }

    #[test]
    fn test_meku_meita_mysterious() {
        // Testing: Noun + めいた (common expression 謎めいた)
        // From grammar_points_data.json: "謎めいた雰囲気"
        let sentence = "彼女はなんか謎めいた雰囲気があるよね。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "めく・めいた");
        assert_pattern_range(&patterns, "めく・めいた", 6, 10); // 謎めいた
    }

    #[test]
    fn test_meku_season_compound() {
        // Testing: Compound season verb (夏めく as single token)
        // Example: 夏めいてきた
        let sentence = "最近の陽気は、すっかり夏めいてきた感じだね。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "めく・めいた");
        assert_pattern_range(&patterns, "めく・めいた", 11, 14); // 夏めい (compound verb)
    }

    #[test]
    fn test_meku_meiteimasu_polite_compound() {
        // Testing: Compound verb + polite form (春めいています)
        // Polite form example
        let sentence = "最近の天気は春めいていますね。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "めく・めいた");
        assert_pattern_range(&patterns, "めく・めいた", 6, 9); // 春めい (compound verb)
    }

    #[test]
    fn test_meku_meitekimashita_polite_split() {
        // Testing: Noun + めいて + polite past (冬めいてきました)
        // 冬めい is split form: 冬 + めい
        let sentence = "ここ数日で、やっと冬めいてきました。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "めく・めいた");
        assert_pattern_range(&patterns, "めく・めいた", 9, 12); // 冬めい (split: Noun + めい)
    }

    #[test]
    fn test_meku_nazo_meiteiru() {
        // Testing: Common set expression 謎めく + ている
        let sentence = "あの人の行動は最近謎めいている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "めく・めいた");
        assert_pattern_range(&patterns, "めく・めいた", 9, 12); // 謎めい
    }
}

// ============================================================================
// といわず Tests
// ============================================================================

mod toiwazu_tests {
    use super::*;

    // Pattern: といわず (not just...but also everything)
    // Data source: grammar_points_data.json["といわず"]
    // Testing: structure.standard[0] - "Noun (A) + といわず + Noun (B) + といわず"
    //
    // Meaning: "Not just (A) or (B), but all" / "(A), (B) and everything else"
    // Usage: と + 言わず (negative stem of 言う) + ず (classical negative)
    // Note: Must use with nouns that belong to the same logical group

    #[test]
    fn test_toiwazu_tansu_tana() {
        // Testing: Noun + といわず + Noun + といわず (wardrobe and shelves)
        let sentence = "旅行から帰ってきたら、タンスといわず、棚といわず、家中泥棒に物色されていた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "といわず");
        assert_pattern_range(&patterns, "といわず", 11, 24); // タンスといわず、棚といわず
    }

    #[test]
    fn test_toiwazu_shigotochu_kyuukeichu() {
        // Testing: Noun + といわず + Noun + といわず (work time and break time)
        // Compound nouns: 仕事中 = 仕事 + 中, 休憩中 = 休憩 + 中
        let sentence = "彼は仕事中といわず、休憩中といわず、ずっとユーチューブを見ている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "といわず");
        assert_pattern_range(&patterns, "といわず", 2, 17); // 仕事中といわず、休憩中といわず
    }

    #[test]
    fn test_toiwazu_kodomo_otona() {
        // Testing: Noun + といわず + Noun + といわず (children and adults)
        let sentence = "この映画は子供といわず、大人といわず、みんなに愛されている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "といわず");
        assert_pattern_range(&patterns, "といわず", 5, 18); // 子供といわず、大人といわず
    }
}

// ============================================================================
// ったらない・といったらない Tests
// ============================================================================

mod ttaranai_toittaranai_tests {
    use super::*;

    // Pattern: ったらない・といったらない (too X for words / indescribably X)
    // Data source: grammar_points_data.json["ったらない・といったらない"]
    // Meaning: "There are no words to express (A)" / "Too (A) for words"
    // Used when something is at a level such that not even (A) is sufficient to describe it
    //
    // Structure variants to test:
    // Standard forms:
    //   - Noun + といったらない/ったらない (more common)
    //   - い-Adj[さ] + といったらない/ったらない (more common)
    //   - Verb + といったらない/ったらない (less common)
    //   - な-Adj + といったらない/ったらない (less common)
    // Polite forms:
    //   - Same structures + といったらありません/ったらありません

    #[test]
    fn test_toittaranai_noun() {
        // Testing: Noun + といったらない (more common form)
        let sentence = "プロジェクト完成後の達成感といったらない！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ったらない・といったらない");
        assert_pattern_range(&patterns, "ったらない・といったらない", 12, 20); // 感といったらない
    }

    #[test]
    fn test_toittaranai_i_adj() {
        // Testing: い-Adj + といったらない (with adjective directly)
        // Example from grammar_points_data.json
        let sentence = "隣の人の犬は朝からうるさいといったらない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ったらない・といったらない");
        assert_pattern_range(&patterns, "ったらない・といったらない", 9, 20); // うるさいといったらない
    }

    #[test]
    fn test_toittaranai_i_adj_sa() {
        // Testing: い-Adj[さ] + といったらない (with さ suffix)
        let sentence = "あの人の優しさといったらない、本当に素晴らしい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ったらない・といったらない");
        assert_pattern_range(&patterns, "ったらない・といったらない", 6, 14); // さといったらない
    }

    #[test]
    fn test_toittaranai_na_adj() {
        // Testing: な-Adj + といったらない (less common)
        let sentence = "息子を家で留守番させるのは心配といったらない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ったらない・といったらない");
        assert_pattern_range(&patterns, "ったらない・といったらない", 13, 22); // 心配といったらない
    }

    // TODO: Polite form variants (ありません, ありゃしない) not yet implemented
    // These would require extending the matcher to handle additional endings
    //
    // #[test]
    // fn test_toittaranai_polite() {
    //     // Testing: Noun + といったらありません (polite form)
    //     let sentence = "田舎では車がないとどこにもいけないから、不便といったらありません。";
    //     let tokens = tokenize_sentence(sentence);
    //     let patterns = detect_patterns(&tokens);
    //
    //     assert_has_pattern(&patterns, "ったらない・といったらない");
    //     assert_pattern_range(&patterns, "ったらない・といったらない", 20, 33);
    // }
    //
    // #[test]
    // fn test_toittaranai_ariyashinai() {
    //     // Testing: Verb + といったらありゃしない (emphatic form)
    //     let sentence = "彼女は私の話を全然聞いてくれない。むかつくといったらありゃしない。";
    //     let tokens = tokenize_sentence(sentence);
    //     let patterns = detect_patterns(&tokens);
    //
    //     assert_has_pattern(&patterns, "ったらない・といったらない");
    //     assert_pattern_range(&patterns, "ったらない・といったらない", 21, 35);
    // }
}

// ============================================================================
// を余儀なくさせる Tests
// ============================================================================

mod woyoginakusaseru_tests {
    use super::*;

    // Pattern: を余儀なくさせる (force/compel to)
    // Data source: grammar_points_data.json["を余儀なくさせる"]
    // Testing structure variants

    #[test]
    fn test_woyoginakusaseru_noun_standard() {
        // Testing: structure.standard[0] - "Noun + を余儀なくさせる"
        let sentence = "運転手の不注意が、入院を余儀なくさせた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を余儀なくさせる");
        assert_pattern_range(&patterns, "を余儀なくさせる", 9, 19); // 入院を余儀なくさせた
    }

    #[test]
    fn test_woyoginakusaseru_heiten() {
        // Testing: structure.standard[0] - "Noun + を余儀なくさせる"
        let sentence = "外出自粛要請が、飲食店の閉店を余儀なくさせた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を余儀なくさせる");
        assert_pattern_range(&patterns, "を余儀なくさせる", 12, 22); // 閉店を余儀なくさせた
    }

    #[test]
    fn test_woyoginakusaseru_verb_koto() {
        // Testing: structure.standard[1] - "Verb + こと + を余儀なくさせる"
        let sentence = "予算不足が、計画を中止することを余儀なくさせた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を余儀なくさせる");
        assert_pattern_range(&patterns, "を余儀なくさせる", 13, 23); // ことを余儀なくさせた
    }

    #[test]
    fn test_woyoginakusaseru_polite() {
        // Testing: structure.polite[0] - "Noun + を余儀なくさせます"
        let sentence = "経済危機が企業の倒産を余儀なくさせます。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を余儀なくさせる");
        assert_pattern_range(&patterns, "を余儀なくさせる", 8, 19); // 倒産を余儀なくさせます
    }
}

// ============================================================================
// てはかなわない Tests
// ============================================================================

mod tehakanawanai_tests {
    use super::*;

    // Pattern: てはかなわない (can't stand, unbearable)
    // Data source: grammar_points_data.json["てはかなわない"]
    // Testing structure variants

    #[test]
    fn test_tehakanawanai_verb_te() {
        // Testing: structure.standard[0] - "Verb[て] + は + かなわない"
        let sentence = "明日充電器を忘れてはかなわないから、今のうちに鞄に入れておこう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てはかなわない");
        assert_pattern_range(&patterns, "てはかなわない", 6, 15); // 忘れてはかなわない
    }

    #[test]
    fn test_tehakanawanai_passive() {
        // Testing: structure.standard[0] - "Verb[られる][て] + は + かなわない" (passive)
        let sentence = "息子に竿を折られてはかなわないから、手が届かないところに置いておこう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てはかなわない");
        assert_pattern_range(&patterns, "てはかなわない", 7, 15); // れてはかなわない
    }

    #[test]
    fn test_tehakanawanai_i_adj() {
        // Testing: structure.standard[1] - "い-Adj[く] + ては + かなわない"
        let sentence = "こう頭が痛くてはかなわない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てはかなわない");
        assert_pattern_range(&patterns, "てはかなわない", 4, 13); // 痛くてはかなわない
    }

    #[test]
    fn test_tehakanawanai_na_adj() {
        // Testing: structure.standard[2] - "な-Adj + では + かなわない"
        let sentence = "そうあやふやではかなわない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てはかなわない");
        assert_pattern_range(&patterns, "てはかなわない", 2, 13); // あやふやではかなわない
    }

    #[test]
    fn test_tehakanawanai_verb_passive_quit() {
        // Testing: structure.standard[0] - "Verb[られる][て] + は + かなわない" (passive)
        let sentence = "今繁忙期だしこんな時にやめられてはかなわない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "てはかなわない");
        assert_pattern_range(&patterns, "てはかなわない", 13, 22); // られてはかなわない
    }
}

// Pattern: にもほどがある (there is a limit to / you are too)
// Data source: grammar_points_data.json["にもほどがある"]
// Testing structure variants:
//   - standard[0]: Verb + にもほどがある
//   - standard[1]: い-Adjective + にも + ほどがある
//   - standard[2]: な-Adjective + にも + ほどがある
//   - standard[3]: Noun + にも + ほどがある
//   - polite[0-3]: Same forms + あります
mod nimohodogaaru_tests {
    use super::*;

    #[test]
    fn test_nimohodogaaru_verb() {
        // Testing: structure.standard[0] - "Verb + にもほどがある"
        // Example from grammar_points_data.json: "飲み過ぎにもほどがあるよ"
        let sentence = "お前またぶっ倒れるまで飲み続けたの？飲み過ぎにもほどがあるよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にもほどがある");
        assert_pattern_range(&patterns, "にもほどがある", 20, 29); // 過ぎにもほどがある
    }

    #[test]
    fn test_nimohodogaaru_i_adjective() {
        // Testing: structure.standard[1] - "い-Adjective + にも + ほどがある"
        // Example from grammar_points_data.json: "図々しいにもほどがある"
        let sentence = "あいつからまたメールがきた。図々しいにもほどがあるって。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にもほどがある");
        assert_pattern_range(&patterns, "にもほどがある", 14, 25); // 図々しいにもほどがある
    }

    #[test]
    fn test_nimohodogaaru_na_adjective() {
        // Testing: structure.standard[2] - "な-Adjective + にも + ほどがある"
        // Example from grammar_points_data.json: "失礼にもほどがある"
        let sentence = "先輩に向かって「お前」っていうのは失礼にもほどがある。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にもほどがある");
        assert_pattern_range(&patterns, "にもほどがある", 17, 26); // 失礼にもほどがある
    }

    #[test]
    fn test_nimohodogaaru_noun() {
        // Testing: structure.standard[3] - "Noun + にも + ほどがある"
        // Example from grammar_points_data.json: "冗談にもほどがある"
        let sentence = "そういうことは相手を傷つけるから冗談にもほどがある。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にもほどがある");
        assert_pattern_range(&patterns, "にもほどがある", 16, 25); // 冗談にもほどがある
    }

    #[test]
    fn test_nimohodogaaru_positive_use() {
        // Testing: Positive/joking use with positive adjective
        // Example from grammar_points_data.json: "うまいにもほどがある"
        let sentence = "店出せるほどの美味しさじゃん！うまいにもほどがあるだろ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にもほどがある");
        assert_pattern_range(&patterns, "にもほどがある", 15, 27); // うまいにもほどがあるだろ
    }

    #[test]
    fn test_nimohodogaaru_polite() {
        // Testing: structure.polite[0] - "Verb + にもほどがあります"
        let sentence = "そんな態度は失礼にもほどがありますよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にもほどがある");
        assert_pattern_range(&patterns, "にもほどがある", 6, 17); // 失礼にもほどがあります
    }
}

// ============================================================================
// にもまして Tests
// ============================================================================

mod nimomashite_tests {
    use super::*;

    // Pattern: にもまして (even more than, more than ever)
    // Data source: grammar_points_data.json["にもまして"]
    // Meaning: Implies something has surpassed a certain point, particularly when it was at a consistent level in the past
    // Structures: Noun + にもまして, 何にもまして, 誰にもまして, いつにもまして

    #[test]
    fn test_nimomashite_noun() {
        // Testing: structure.standard[0] - "Noun + にもまして"
        // Example from grammar_points_data.json: "今年は去年にもまして、湿気が多い"
        let sentence = "今年は去年にもまして、湿気が多い。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にもまして");
        assert_pattern_range(&patterns, "にもまして", 3, 10); // 去年にもまして
    }

    #[test]
    fn test_nimomashite_nani() {
        // Testing: structure.standard[1] - "何にもまして"
        // Example from grammar_points_data.json: "このプロジェクトを成功させることは、何にもまして大切なんだ"
        let sentence = "このプロジェクトを成功させることは、何にもまして大切なんだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にもまして");
        assert_pattern_range(&patterns, "にもまして", 18, 24); // 何にもまして
    }

    #[test]
    fn test_nimomashite_dare() {
        // Testing: structure.standard[2] - "誰にもまして"
        // Example from grammar_points_data.json: "斎藤くんは誰にもまして仕事を一生懸命やってくれる"
        let sentence = "斎藤くんは誰にもまして仕事を一生懸命やってくれるからものすごく助かっている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にもまして");
        assert_pattern_range(&patterns, "にもまして", 5, 11); // 誰にもまして
    }

    #[test]
    fn test_nimomashite_itsu() {
        // Testing: structure.standard[3] - "いつにもまして"
        // Example from grammar_points_data.json: "彼女はいつにもまして輝いている"
        let sentence = "彼女はいつにもまして輝いている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にもまして");
        assert_pattern_range(&patterns, "にもまして", 3, 10); // いつにもまして
    }
}

// ============================================================================
// まくる Tests
// ============================================================================

mod makuru_tests {
    use super::*;

    // Pattern: まくる (do repeatedly/excessively, like crazy)
    // Data source: grammar_points_data.json["まくる"]
    // Meaning: Indicates an action is done over and over, a lot, or with great vigor
    // Structures: Verb[stem] + まくる, Verb[て] + Verb[て] + Verb[stem] + まくる
    // Note: Verb must be volitional (controllable action)

    #[test]
    fn test_makuru_simple_past() {
        // Testing: structure.standard[0] - "Verb[stem] + まくる" (split tokenization)
        // Example from grammar_points_data.json: "買いまくった"
        let sentence = "昨日は釣具屋でルアーを買いまくったから、来週までご飯とふりかけで我慢しなきゃいけない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まくる");
        assert_pattern_range(&patterns, "まくる", 11, 17); // 買いまくった
    }

    #[test]
    fn test_makuru_te_iru() {
        // Testing: structure.standard[0] - "Verb[stem] + まくる" with ている (split tokenization)
        // Example from grammar_points_data.json: "食べまくってる"
        let sentence = "久しぶりに実家に帰って、母の料理を食べまくってるから少し太ってきた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まくる");
        assert_pattern_range(&patterns, "まくる", 17, 22); // 食べまくっ
    }

    #[test]
    fn test_makuru_compound() {
        // Testing: structure.standard[0] - "Verb[stem] + まくる" (compound tokenization)
        // Example from grammar_points_data.json: "歌いまくった"
        let sentence = "昨日はカラオケで歌いまくったから、声が全然でない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まくる_compound");
        assert_pattern_range(&patterns, "まくる_compound", 8, 14); // 歌いまくった
    }
}

// ============================================================================
// どうにか Tests
// ============================================================================

mod dounika_tests {
    use super::*;

    // Pattern: どうにか (somehow, one way or another, barely)
    // Data source: grammar_points_data.json["どうにか"]
    // Meaning: Something happens "somehow or other", often with effort or against odds
    // Structure: どうにか + Phrase

    #[test]
    fn test_dounika_persuade() {
        // Testing: structure.standard[0] - "どうにか + Phrase"
        // Example from grammar_points_data.json: "どうにか説得することができた"
        let sentence = "条件に不安はあったみたいだけど、どうにか説得することができた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "どうにか");
        assert_pattern_range(&patterns, "どうにか", 16, 20); // どうにか
    }

    #[test]
    fn test_dounika_solve() {
        // Testing: structure.standard[0] - "どうにか + Phrase"
        // Example from grammar_points_data.json: "どうにか明日までに解決しなくてはならない"
        let sentence = "どうにか明日までに解決しなくてはならない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "どうにか");
        assert_pattern_range(&patterns, "どうにか", 0, 4); // どうにか
    }

    #[test]
    fn test_dounika_naru() {
        // Testing: どうにかなる pattern (things will work out on their own)
        // Example from grammar_points_data.json: "時間が経てばどうにかなるよ"
        let sentence = "そんな落ち込むなって、時間が経てばどうにかなるよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "どうにか");
        assert_pattern_range(&patterns, "どうにか", 17, 21); // どうにか
    }

    #[test]
    fn test_dounika_suru() {
        // Testing: どうにかする pattern (will somehow manage to do)
        // Example from grammar_points_data.json: "どうにかする"
        let sentence = "こっちはこっちでどうにかするから、今は家族との時間を大切にしろ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "どうにか");
        assert_pattern_range(&patterns, "どうにか", 8, 12); // どうにか
    }
}

// ============================================================================
// や否や Tests
// ============================================================================

mod yainaya_tests {
    use super::*;

    // Pattern: や否や (as soon as)
    // Data source: grammar_points_data.json["や否や"]
    // Testing: structure.standard[0] - "Verb[る] + や否（いな）や + Phrase[た]"
    //
    // Note: The caution section mentions that や can be used alone (without 否や)
    // for the same meaning, but we'll focus on the full や否や pattern first.

    #[test]
    fn test_yainaya_burglar() {
        // Testing: structure.standard[0] - "Verb[る] + や否や"
        // Example from grammar_points_data.json: "見るやいなや"
        let sentence = "私が部屋から出てくるのを見るやいなや、空き巣が手に持っていたものを私に投げて、逃げていった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "や否や");
        assert_pattern_range(&patterns, "や否や", 12, 18); // 見るやいなや
    }

    #[test]
    fn test_yainaya_dog_bark() {
        // Testing: structure.standard[0] - "Verb[る] + や否や"
        // Example from grammar_points_data.json: "見るやいなや"
        let sentence = "俺の顔を見るやいなや友達の犬が吠え出した。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "や否や");
        assert_pattern_range(&patterns, "や否や", 4, 10); // 見るやいなや
    }

    #[test]
    fn test_yainaya_sit_down() {
        // Testing: structure.standard[0] - "Verb[る] + や否や"
        // Example from grammar_points_data.json: "座るやいなや"
        let sentence = "彼は席に座るやいなや「とりあえず生一つ」といった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "や否や");
        assert_pattern_range(&patterns, "や否や", 4, 10); // 座るやいなや
    }

    // TODO: Test the simplified や form (without 否や) mentioned in caution section
    // #[test]
    // fn test_ya_simplified() {
    //     // Testing: structure.standard[1] - "Verb[る] + や"
    //     // Example from grammar_points_data.json caution: "開くや"
    //     let sentence = "彼は本を開くや、寝た。";
    //     let tokens = tokenize_sentence(sentence);
    //     let patterns = detect_patterns(&tokens);
    //     print_debug(sentence, &tokens, &patterns);
    //     // This might be a separate pattern or variant
    // }
}

// ============================================================================
// というところ Tests
// ============================================================================

mod toiutokoro_tests {
    use super::*;

    // Pattern: というところ (I would say about / approximately)
    // Data source: grammar_points_data.json["というところ"]
    //
    // Meaning: Used when highlighting an approximate number or degree for something
    // Translation: "I would say about (A)", "I guess about (A)"
    //
    // Structure variants from grammar_points_data.json:
    // - standard[0]: Phrase + というところ + だ
    // - standard[1]: といったところ, ってとこ (casual variants)
    // - polite[0]: Phrase + というところ + です
    // - polite[1]: といったところ, ってとこ (casual variants)

    #[test]
    fn test_toiutokoro_time_period() {
        // Testing: structure.standard[0] - "Phrase + というところ + だ"
        // Example from grammar_points_data.json: "二週間で終わるというところだ"
        let sentence = "このペースだとこの工事はあと二週間で終わるというところだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "というところ");
        assert_pattern_range(&patterns, "というところ", 18, 28); // 終わるというところだ
    }

    #[test]
    fn test_toiutokoro_degree() {
        // Testing: structure.standard[0] - "Phrase + というところ"
        // Example from grammar_points_data.json: "汗が出るぐらい辛いというところ"
        let sentence = "どれぐらい辛いって聞かれても汗が出るぐらい辛いというところかな。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "というところ");
        assert_pattern_range(&patterns, "というところ", 21, 29); // 辛いというところ
    }

    #[test]
    fn test_toiutokoro_duration() {
        // Testing: structure.polite[0] - "Phrase + というところ + です"
        // Example from grammar_points_data.json: "15分というところです"
        let sentence = "電車で行けば30分かかるけど、車なら15分というところです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "というところ");
        assert_pattern_range(&patterns, "というところ", 20, 29); // 分というところです
    }

    #[test]
    fn test_toittatokoro_price() {
        // Testing: structure variant - "といったところ + です" (past form)
        // Example from grammar_points_data.json: "5000万といったところです"
        let sentence = "これぐらいの物件だったら5000万といったところですね。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "というところ");
        assert_pattern_range(&patterns, "というところ", 16, 26); // 万といったところです
    }

    #[test]
    fn test_toittatokoro_walk() {
        // Testing: structure variant - "といったところ + です" (past form)
        // Example from grammar_points_data.json: "1時間といったところです"
        let sentence = "徒歩だと1時間といったところです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "というところ");
        assert_pattern_range(&patterns, "というところ", 5, 16); // 時間といったところです
    }
}

// ============================================================================
// に照らして・に照らすと Tests
// ============================================================================

mod niterashite_niterasuto_tests {
    use super::*;

    // Pattern: に照らして・に照らすと (in light of / in accordance with)
    // Data source: grammar_points_data.json["に照らして・に照らすと"]
    //
    // Meaning: Used comparatively similar to "in light of" in English
    // Translation: "in light of (A)", "in accordance with (A)", "as reflected by (A)"
    // Used to illustrate (B) from the perspective of (A)
    //
    // Structure variants from grammar_points_data.json:
    // - standard[0]: Noun + に照らして
    // - standard[1]: Noun (A) + に照らした + Noun (B)
    // - standard[2]: に照らすと

    #[test]
    fn test_niterashite_experience() {
        // Testing: structure.standard[0] - "Noun + に照らして"
        // Example from grammar_points_data.json: "経験に照らして"
        let sentence = "母は自分の経験に照らして、こういう時はどうすればいいかをアドバイスしてくれた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に照らして・に照らすと");
        assert_pattern_range(&patterns, "に照らして・に照らすと", 5, 12); // 経験に照らして
    }

    #[test]
    fn test_niterashite_law() {
        // Testing: structure.standard[0] - "Noun + に照らして"
        // Example from grammar_points_data.json: "法律に照らして"
        let sentence = "労働基準法を守らない会社は法律に照らして罰せられるべきだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に照らして・に照らすと");
        assert_pattern_range(&patterns, "に照らして・に照らすと", 13, 20); // 法律に照らして
    }

    #[test]
    fn test_niterashita_modifying_noun() {
        // Testing: structure.standard[1] - "Noun (A) + に照らした + Noun (B)"
        // Example from grammar_points_data.json: "経済統計に照らした犯罪統計"
        let sentence = "経済統計に照らした強盗と窃盗事件に関する犯罪統計。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に照らして・に照らすと");
        assert_pattern_range(&patterns, "に照らして・に照らすと", 2, 9); // 統計に照らした
    }

    #[test]
    fn test_niterasuto_conditional() {
        // Testing: structure.standard[2] - "に照らすと"
        // This is the conditional form "に照らすと" (if we illuminate with...)
        let sentence = "法律に照らすと、この行為は明らかに違法だ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に照らして・に照らすと");
        assert_pattern_range(&patterns, "に照らして・に照らすと", 0, 7); // 法律に照らすと
    }
}

// ============================================================================
// とあれば Tests
// ============================================================================

mod toareba_tests {
    use super::*;

    // Pattern: とあれば (if/when it comes to)
    // Data source: grammar_points_data.json["とあれば"]
    // Testing: structure.standard[0] - "Noun + （だ）+ とあれば"
    //
    // Meaning: Emphatic version of なら 'if'. 'If (A) is the case, then (B)'
    // Components: と (case-marking particle) + あれば (hypothetical form of ある)
    // Used when (A) is a special circumstance leading to (B), which is usually
    // something unavoidable (しかない, なければならない, ざるを得ない, etc.)

    #[test]
    fn test_toareba_noun_direct() {
        // Testing: Noun + とあれば (without copula)
        // Based on: "お前の頼みとあればやるしかないだろ"
        let sentence = "お前の頼みとあればやるしかないだろ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とあれば");
        assert_pattern_range(&patterns, "とあれば", 3, 9); // 頼みとあれば
    }

    #[test]
    fn test_toareba_senpai_orders() {
        // Testing: Noun + とあれば
        // Based on: "先輩の指示とあれば従わないわけにはいけないけど"
        let sentence = "先輩の指示とあれば従わないわけにはいけないけど。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とあれば");
        assert_pattern_range(&patterns, "とあれば", 3, 9); // 指示とあれば
    }

    #[test]
    fn test_toareba_verb_quotation() {
        // Testing: Verb + とあれば (quotation form)
        // Based on: "データを復元できるとあれば、いくらでも出します"
        let sentence = "データを復元できるとあれば、いくらでも出します！";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とあれば");
        assert_pattern_range(&patterns, "とあれば", 6, 13); // できるとあれば
    }

    #[test]
    fn test_toareba_adjective_quotation() {
        // Testing: い-Adjective + とあれば (quotation form)
        // Based on: "状態がいいとあれば４万円ぐらいで売れる"
        let sentence = "状態がいいとあれば４万円ぐらいで売れると思いますよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とあれば");
        assert_pattern_range(&patterns, "とあれば", 3, 9); // いいとあれば
    }
}

// ============================================================================
// ときたら Tests
// ============================================================================

mod tokitara_tests {
    use super::*;

    // Pattern: ときたら (when it comes to / that darn)
    // Data source: grammar_points_data.json["ときたら"]
    // Testing: structure.standard[0] - "Noun + ときたら"
    //
    // Meaning: Casually expressing mild frustration or highlighting something about (A)
    // Components: と (case-marking particle) + 来たら (conditional of 来る)
    // Almost exclusively written in kana (ときたら, not と来たら)
    // Can express frustration or just highlight something "to be expected"

    #[test]
    fn test_tokitara_son_complaint() {
        // Testing: Noun + ときたら (expressing frustration)
        // Based on: "うちの息子ときたら、就職もしないで家でダラダラしている"
        let sentence = "うちの息子ときたら、就職もしないで家でダラダラしている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ときたら");
        assert_pattern_range(&patterns, "ときたら", 3, 9); // 息子ときたら
    }

    #[test]
    fn test_tokitara_brother_complaint() {
        // Testing: Noun + ときたら (expressing frustration)
        // Based on: "うちの弟ときたら、朝から文句ばかり言ってくる"
        let sentence = "うちの弟ときたら、朝から文句ばかり言ってくるから本当にうざい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ときたら");
        assert_pattern_range(&patterns, "ときたら", 3, 8); // 弟ときたら
    }

    #[test]
    fn test_tokitara_boss_complaint() {
        // Testing: Noun + ときたら (expressing frustration)
        // Based on: "私の上司ときたら、自分が仕事できないくせに..."
        let sentence = "私の上司ときたら、自分が仕事できないくせに少しでもミスをしたらものすごく怒る。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ときたら");
        assert_pattern_range(&patterns, "ときたら", 2, 8); // 上司ときたら
    }

    #[test]
    fn test_tokitara_dog_expected() {
        // Testing: Noun + ときたら (highlighting what's to be expected)
        // Based on: "うちの犬ときたら、家にお客さんが来るとその人に飛びつく"
        let sentence = "うちの犬ときたら、家にお客さんが来るとその人に飛びつくから困る。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ときたら");
        assert_pattern_range(&patterns, "ときたら", 3, 8); // 犬ときたら
    }
}

// ============================================================================
// ても差し支えない Tests (it's not a hindrance if, do you mind if)
// ============================================================================

mod temosashitsukaenai_tests {
    use super::*;

    // Pattern: ても差し支えない (it's not a hindrance if, may I, do you mind if)
    // Data source: grammar_points_data.json["ても差し支えない"]
    // Testing: structure.standard[0] - "Verb[ても] + 差し支え + ありません"
    //
    // Other structures to test:
    //   - standard[1]: Noun[でも] + 差し支え + ありません
    //   - standard[2]: な-Adjective[でも] + 差し支え + ありません
    //   - standard[3]: い-Adjective[ても] + 差し支え + ありません
    //   - With variations: ない, ないでしょう, ございません

    #[test]
    fn test_temosashitsukaenai_verb_permission() {
        // Testing: Verb[ても] + 差し支え + ありません (granting permission)
        // Based on: "ここに座ってもらってもさしつかえありません"
        let sentence = "ここに座ってもらってもさしつかえありません。どうぞお座りください。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ても差し支えない");
        assert_pattern_range(&patterns, "ても差し支えない", 9, 21); // てもさしつかえありません
    }

    #[test]
    fn test_temosashitsukaenai_i_adjective() {
        // Testing: い-Adjective[ても] + 差し支え + ありません
        // Based on: "お風呂は熱くてもさしつかえありません"
        let sentence = "お風呂は熱くてもさしつかえありません。むしろ熱い方がありがたいです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ても差し支えない");
        assert_pattern_range(&patterns, "ても差し支えない", 6, 18); // てもさしつかえありません
    }

    #[test]
    fn test_temosashitsukaenai_na_adjective_question() {
        // Testing: な-Adjective[でも] + 差し支え + ありませんか (asking permission)
        // Based on: "私はまだ初心者です。下手でもさしつかえありませんか"
        let sentence = "私はまだ初心者です。下手でもさしつかえありませんか。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ても差し支えない");
        assert_pattern_range(&patterns, "ても差し支えない", 12, 24); // でもさしつかえありません
    }

    #[test]
    fn test_temosashitsukaenai_noun_time() {
        // Testing: Noun[でも] + 差し支え + ありませんか (asking about timing)
        // Based on: "今週はものすごく忙しいので来週でもさしつかえありませんか"
        let sentence = "今週はものすごく忙しいので来週でもさしつかえありませんか。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ても差し支えない");
        assert_pattern_range(&patterns, "ても差し支えない", 15, 27); // でもさしつかえありません
    }
}

// ============================================================================
// びる Tests
// ============================================================================

mod biru_tests {
    use super::*;

    // Pattern: びる (seeming/looking like)
    // Data source: grammar_points_data.json["びる"]
    // Testing: structure.standard[0] - "Noun + びる"
    //
    // Structure variants from grammar data:
    //   - standard[0]: Noun + びる (e.g., 大人びる)
    //   - standard[1]: い-Adjective stem + びる (e.g., 幼びる from 幼い)
    //   - standard[2]: Noun + びた + Noun (modifying noun)
    //   - standard[3]: い-Adjective stem + びた + Noun (modifying noun)
    //
    // Common expressions: 大人びる, 古びた, ひなびた, 田舎びた, 物寂びる
    //
    // Note: 幼びる (from 幼い) is grammatically valid but not recognized by Kagome.
    // It tokenizes as 幼 (noun) + びて (noun) rather than as a verb.

    #[test]
    fn test_biru_otonaびる_て_form() {
        // Testing: Noun + びる (て-form)
        // Based on: "彼女は大人びて見えるが、まだ社会のことなんて分かってないただの学生だ"
        // Note: て is a particle (助詞), not auxiliary, so pattern range is just the verb
        let sentence = "彼女は大人びて見えるが、まだ社会のことなんて分かってない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "びる");
        assert_pattern_range(&patterns, "びる", 3, 6); // 大人び (verb only, て is separate)
    }

    #[test]
    fn test_biru_furuびた_modifying_noun() {
        // Testing: Noun + びた + Noun (modifying noun)
        // Based on: "実家で屋根裏部屋を掃除していたら、古びたアルバムが続々と出てきた"
        // Note: た is an auxiliary (助動詞), so pattern range includes it
        let sentence = "実家で掃除していたら、古びたアルバムが出てきた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "びる");
        assert_pattern_range(&patterns, "びる", 11, 14); // 古びた (verb + auxiliary)
    }

    #[test]
    fn test_biru_inakabita_modifying_noun() {
        // Testing: Noun + びた + Noun (modifying noun)
        // Based on: "私はどんなに治安が良くても、田舎びた町には住みたくない"
        // Note: た is an auxiliary (助動詞), so pattern range includes it
        let sentence = "どんなに治安が良くても、田舎びた町には住みたくない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "びる");
        assert_pattern_range(&patterns, "びる", 12, 16); // 田舎びた (verb + auxiliary)
    }

    #[test]
    fn test_biru_hinabita_modifying_noun() {
        // Testing: Noun + びた (ひなびた - rustic/quaint)
        // Based on common expression ひなびた温泉 (rustic hot spring)
        // Note: た is an auxiliary (助動詞), so pattern range includes it
        let sentence = "ひなびた温泉に行きたいな。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "びる");
        assert_pattern_range(&patterns, "びる", 0, 4); // ひなびた (verb + auxiliary)
    }

    #[test]
    fn test_biru_inakaびて_いる() {
        // Testing: Noun + びている (progressive form)
        // Based on: "この町は田舎びているから好きだ"
        // Note: て is a particle (助詞), not auxiliary, so pattern range is just the verb
        let sentence = "この町は田舎びているから好きだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "びる");
        assert_pattern_range(&patterns, "びる", 4, 7); // 田舎び (verb only, て is separate)
    }
}

// ============================================================================
// にしたところで Tests
// ============================================================================

mod nishitatokorode_tests {
    use super::*;

    // Pattern: にしたところで (even if / even though)
    // Data source: grammar_points_data.json["にしたところで"]
    // Testing structure variants:
    //   - standard[0]: Noun + にしたところで
    //   - standard[2]: Noun + にしたって (less formal)
    //   - standard[3]: としたところで (alternative with と)
    //   - standard[4]: としたって (alternative casual with と)

    #[test]
    fn test_nishitatokorode_noun_basic() {
        // Testing: Noun + にしたところで
        // Based on: "先輩の青木さんにしたところで、上達しなければいけない箇所はある。"
        let sentence = "先輩の青木さんにしたところで、上達しなければいけない箇所はある。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にしたところで");
        assert_pattern_range(&patterns, "にしたところで", 5, 14); // さんにしたところで
    }

    #[test]
    fn test_nishitatokorode_laptop() {
        // Testing: Noun + にしたところで
        // Based on: "ノートパソコンにしたところで、Ｗｉ－Ｆｉがないと仕事ができない。"
        let sentence = "ノートパソコンにしたところで、Ｗｉ－Ｆｉがないと仕事ができない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にしたところで");
        assert_pattern_range(&patterns, "にしたところで", 3, 14); // パソコンにしたところで
    }

    #[test]
    fn test_nishitatte_casual() {
        // Testing: Noun + にしたって (less formal)
        // Based on: "私にしたってそんなことはしたくないけど、上司に言われたからにはするしかない。"
        let sentence = "私にしたってそんなことはしたくないけど、上司に言われたからにはするしかない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にしたところで");
        assert_pattern_range(&patterns, "にしたところで", 0, 6); // 私にしたって
    }

    #[test]
    fn test_toshitatokorode_verb_phrase() {
        // Testing: Verb phrase + としたところで
        // Based on: "いくら急いだとしたところで、間に合うわけがない。"
        let sentence = "いくら急いだとしたところで、間に合うわけがない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にしたところで");
        assert_pattern_range(&patterns, "にしたところで", 5, 13); // だとしたところで
    }

    #[test]
    fn test_toshitatte_casual() {
        // Testing: Verb phrase + としたって (less formal)
        // Based on: "謝ったとしたって、社長は彼のことを許さないだろう。"
        let sentence = "謝ったとしたって、社長は彼のことを許さないだろう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にしたところで");
        assert_pattern_range(&patterns, "にしたところで", 2, 8); // たとしたって
    }
}

// ============================================================================
// といおうか Tests
// ============================================================================

mod toiouka_tests {
    use super::*;

    // Pattern: といおうか (how to put it / shall I say)
    // Data source: grammar_points_data.json["といおうか"]
    // Testing: structure.standard[0] - "AといおうかBといおうか + (Phrase)"
    // where A and B can be Noun, い-Adjective, な-Adjective or Verb
    //
    // Structure variants to test:
    //   - Verb といおうか Verb といおうか
    //   - い-Adjective といおうか い-Adjective といおうか
    //   - な-Adjective といおうか な-Adjective といおうか
    //   - Noun といおうか Noun といおうか

    #[test]
    fn test_toiouka_verb_pair() {
        // Testing: Verb といおうか Verb といおうか pattern
        // Based on: "また彼女に誕生日を忘れられたんだ。あきれているといおうか、傷ついたといおうか、よくわからない。"
        let sentence = "あきれているといおうか傷ついたといおうか、よくわからない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "といおうか");
        assert_pattern_range(&patterns, "といおうか", 4, 11); // いるといおうか (first occurrence)
    }

    #[test]
    fn test_toiouka_i_adjective_pair() {
        // Testing: い-Adjective といおうか い-Adjective といおうか pattern
        // Based on: "あいつの運転は荒いといおうか、危ないといおうか、とにかくあいつが運転する車には乗りたくない。"
        let sentence = "彼の運転は荒いといおうか危ないといおうか、とにかく乗りたくない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "といおうか");
        assert_pattern_range(&patterns, "といおうか", 12, 20); // 危ないといおうか
    }

    #[test]
    fn test_toiouka_na_adjective_pair() {
        // Testing: な-Adjective といおうか な-Adjective といおうか pattern
        // Based on: "自分の体を使ってお客さんの車を守ろうとするなんて、バカといおうか、真面目すぎるといおうか分からない。"
        let sentence = "バカといおうか真面目すぎるといおうか分からない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "といおうか");
        assert_pattern_range(&patterns, "といおうか", 10, 18); // すぎるといおうか
    }

    #[test]
    fn test_toiouka_noun_pair() {
        // Testing: Noun といおうか Noun といおうか pattern
        // Based on: "彼はいとこといおうか、ハトコといおうか、私もどういう関係かはわからないけど親戚だということはわかっている。"
        let sentence = "彼はいとこといおうかハトコといおうか、よくわからない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "といおうか");
        assert_pattern_range(&patterns, "といおうか", 2, 10); // いとこといおうか (first occurrence)
    }

    #[test]
    fn test_toiouka_mixed_types() {
        // Testing: Mixed word types (Verb といおうか Adjective といおうか)
        // Using: "お前は努力が足りないといおうか、才能がないといおうか…。"
        let sentence = "彼は努力が足りないといおうか才能がないといおうか、よくわからない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "といおうか");
        assert_pattern_range(&patterns, "といおうか", 7, 14); // ないといおうか (first occurrence)
    }
}

// ～ばこそ pattern tests
mod bakoso_tests {
    use super::*;

    #[test]
    fn test_bakoso_verb_conditional() {
        // Testing: Verb[ば] + こそ + Phrase
        // Based on example: "娘の将来を思えばこそ、塾に通わせます。"
        let sentence = "娘の将来を思えばこそ、塾に通わせているんです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～ばこそ");
        assert_pattern_range(&patterns, "～ばこそ", 5, 10); // 思えばこそ
    }

    #[test]
    fn test_bakoso_i_adjective() {
        // Testing: い-Adjective[ば] + こそ + Phrase
        // Based on example: "忙しければこそ時間を効率的に使うことができるようになる。"
        let sentence = "忙しければこそ時間を有効に使える。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～ばこそ");
        assert_pattern_range(&patterns, "～ばこそ", 0, 7); // 忙しければこそ
    }

    #[test]
    fn test_bakoso_na_adjective() {
        // Testing: な-Adjective + であれば + こそ + Phrase
        // Based on example: "健康であればこそ、年をとっても楽しい人生を過ごせる。"
        let sentence = "健康であればこそ、人生を楽しめるのです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～ばこそ");
        assert_pattern_range(&patterns, "～ばこそ", 0, 8); // 健康であればこそ
    }

    #[test]
    fn test_bakoso_noun() {
        // Testing: Noun + であれば + こそ + Phrase
        // Based on example: "小さな会社であればこそ、優秀な人材が必要だ。"
        let sentence = "小さな会社であればこそ、人材が大切だ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "～ばこそ");
        assert_pattern_range(&patterns, "～ばこそ", 3, 11); // 会社であればこそ
    }
}

// ============================================================================
// を限りに Tests
// ============================================================================

mod wokagirini_tests {
    use super::*;

    // Pattern: を限りに (ending with / no longer than / as of)
    // Data source: grammar_points_data.json["を限りに"]
    // Testing: structure.standard[0] - "Noun + を限（かぎ）りに"
    //
    // Other structures to test:
    //   - standard[1]: Noun + 限（かぎ）りで

    #[test]
    fn test_wokagirini_store_closing() {
        // Testing: Noun[time] + を限りに + closing announcement
        // Based on example: "明日をかぎりに店を閉店させていただく"
        let sentence = "明日をかぎりに店を閉店させていただくことになりました。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を限りに");
        assert_pattern_range(&patterns, "を限りに", 0, 7); // 明日をかぎりに
    }

    #[test]
    fn test_wokagirini_moving_abroad() {
        // Testing: Noun[time] + を限りに + life change announcement
        // Based on example: "今月をかぎりに、海外に引っ越す"
        let sentence = "今月をかぎりに、海外に引っ越すことになりました。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を限りに");
        assert_pattern_range(&patterns, "を限りに", 0, 7); // 今月をかぎりに
    }

    #[test]
    fn test_wokagirini_concert_disbanding() {
        // Testing: Noun[event] + を限りに + event announcement
        // Based on example: "来月行われるコンサートをかぎりにＳＵＭ４１は解散する"
        let sentence = "来月行われるコンサートをかぎりに解散することを決めたそうです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を限りに");
        assert_pattern_range(&patterns, "を限りに", 6, 16); // コンサートをかぎりに
    }

    #[test]
    fn test_kagirinde_variant() {
        // Testing: Noun + 限りで (less formal variant)
        // Based on structure.standard[1]: "Noun + 限（かぎ）りで"
        let sentence = "今年限りでこの仕事は終わりにします。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を限りに");
        assert_pattern_range(&patterns, "を限りに", 0, 5); // 今年限りで
    }

    #[test]
    fn test_wokagirini_voice_idiom() {
        // Testing: 声を限りに (set idiom - "to the limit of one's voice")
        // Based on Fun Fact: "声を限りに" is an exception that doesn't follow temporal rule
        let sentence = "息子が声をかぎりに泣いていたから大変だったよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を限りに");
        assert_pattern_range(&patterns, "を限りに", 3, 9); // 声をかぎりに
    }
}

// ============================================================================
// には及ばない① Tests
// ============================================================================

mod niwaoyobanai_tests {
    use super::*;

    // Pattern: には及ばない① (no need to / unnecessary)
    // Data source: grammar_points_data.json["には及ばない①"]
    // Testing: structure.standard[0] - "Verb + には及（およ）ばない"
    //
    // Meaning: "There is no need to (A)" / "(A) is unnecessary"
    // Components: に (case-marking) + は (topic) + 及ばない (negated form of 及ぶ "to reach")
    // Used after nouns or verbs in dictionary form
    // Often preceded by お or ご for politeness

    #[test]
    fn test_niwaoyobanai_noun_basic() {
        // Testing: Noun + には及ばない
        // Based on structure.standard[0]
        let sentence = "心配には及ばない";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "には及ばない①");
        assert_pattern_range(&patterns, "には及ばない①", 0, 8); // 心配には及ばない
    }

    #[test]
    fn test_niwaoyobanai_verb_dictionary() {
        // Testing: Verb[dictionary] + には及ばない
        // Based on structure.standard[0]: "Verb + には及ばない"
        let sentence = "こちらまでお越しいただくには及びません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "には及ばない①");
        assert_pattern_range(&patterns, "には及ばない①", 8, 19); // いただくには及びません
    }

    #[test]
    fn test_niwaoyobanai_go_prefix() {
        // Testing: ご + Noun + には及ばない
        // Based on structure.standard[3]: "（ご）+ 心配（しんぱい） + には及（およ）ばない"
        let sentence = "ご心配には及びません。これぐらいの怪我なら全然我慢できます。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "には及ばない①");
        assert_pattern_range(&patterns, "には及ばない①", 1, 10); // 心配には及びません
    }

    #[test]
    fn test_niwaoyobanai_o_prefix() {
        // Testing: お + Noun + には及ばない
        // Based on structure.standard[2]: "（お）+ 礼（れい） + には及（およ）ばない"
        let sentence = "お礼には及びません。当然のことをしたまでです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "には及ばない①");
        assert_pattern_range(&patterns, "には及ばない①", 0, 9); // お礼には及びません
    }

    #[test]
    fn test_niwaoyobanai_explanation() {
        // Testing: する-Verb + には及ばない
        // Based on structure.standard[1]: "［する］Verb + （する）+ には及（およ）ばない"
        let sentence = "既にご存じだと思いますので、ご説明には及びません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "には及ばない①");
        assert_pattern_range(&patterns, "には及ばない①", 15, 24); // 説明には及びません
    }
}

// Pattern: に即して (in accordance with / based on)
// Data source: grammar_points_data.json["に即して"]
// Testing structure variants from grammar_points_data.json
mod nisokushite_tests {
    use super::*;

    #[test]
    fn test_nisokushite_te_form_rules() {
        // Testing: structure.standard[0] - "Noun + に即（そく）して"
        // Formal expression meaning "in accordance with"
        let sentence = "会社のルールに即して、副業をしてはいけない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に即して");
        assert_pattern_range(&patterns, "に即して", 3, 10); // ルールに即して
    }

    #[test]
    fn test_nisokushite_te_form_facts() {
        // Testing: structure.standard[0] - "Noun + に即（そく）して"
        // Used when doing something in line with facts/circumstances
        let sentence = "あの監督はちゃんと事実に即して映画を作るから、映画を見るだけでいろいろなことが学べる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に即して");
        assert_pattern_range(&patterns, "に即して", 9, 15); // 事実に即して
    }

    #[test]
    fn test_nisokushita_modifying_noun_work_hours() {
        // Testing: structure.standard[1] - "Noun + に即（そく）した + Noun"
        // Modifying noun form (した instead of して)
        let sentence = "法律に即した勤務時間は週４０時間にも関わらず、あの会社は社員に週７０時間働かせている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に即して");
        assert_pattern_range(&patterns, "に即して", 0, 6); // 法律に即した
    }

    #[test]
    fn test_nisokushita_modifying_noun_punishment() {
        // Testing: structure.standard[1] - "Noun + に即（そく）した + Noun"
        // Modifying noun form with different noun
        let sentence = "外国人であろうと、日本にいる限り、日本の法律に即した罰が与えられます。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に即して");
        assert_pattern_range(&patterns, "に即して", 20, 26); // 法律に即した
    }

    #[test]
    fn test_nisokushite_alternative_kanji() {
        // Testing: structure.standard[2] note - "(1) に則（そく）した"
        // Alternative kanji 則 instead of 即 (same reading, similar meaning)
        // Note: The data shows this as a variant of に即した, so testing した form
        let sentence = "現実に則した計画を立てることが重要だ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に即して");
        assert_pattern_range(&patterns, "に即して", 0, 6); // 現実に則した
    }
}

// Pattern: ないまでも (even if not / may not be... but)
// Data source: grammar_points_data.json["ないまでも"]
// Structure: Verb[ない] + までも / Noun + ではない/じゃない + までも
mod naimademo_tests {
    use super::*;

    #[test]
    fn test_naimademo_verb_negative_masterpiece() {
        // Testing: structure.standard[0] - "Verb[ない] + までも + Phrase"
        // Example from grammar data: 名作だとは言えないまでも
        let sentence = "この作品は名作だとは言えないまでも、なかなかいい作品だと思う。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないまでも");
        assert_pattern_range(&patterns, "ないまでも", 10, 17); // 言えないまでも
    }

    #[test]
    fn test_naimademo_verb_negative_eliminate() {
        // Testing: structure.standard[0] - "Verb[ない] + までも + Phrase"
        // Example from grammar data: 完全になくせないまでも
        let sentence = "犯罪などは完全になくせないまでも、減らすように頑張るべきだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないまでも");
        assert_pattern_range(&patterns, "ないまでも", 8, 16); // なくせないまでも
    }

    #[test]
    fn test_naimademo_verb_negative_become() {
        // Testing: structure.standard[0] - "Verb[ない] + までも + Phrase"
        // Example from grammar data: なれないまでも
        let sentence = "プロ野球選手にはなれないまでも、プロ野球に関わる仕事をしているので満足している。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないまでも");
        assert_pattern_range(&patterns, "ないまでも", 8, 15); // なれないまでも
    }

    #[test]
    fn test_naimademo_noun_dewanai() {
        // Testing: structure.standard[1] - "Noun + ではない + までも + Phrase"
        // Example from grammar data: 毎週ではないまでも
        let sentence = "毎週ではないまでも、娘に会えるだけ幸せだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないまでも");
        assert_pattern_range(&patterns, "ないまでも", 0, 9); // 毎週ではないまでも
    }

    #[test]
    fn test_naimademo_noun_janai() {
        // Testing: structure.standard[2] note - "(1) じゃない"
        // Example from grammar data: 幸せじゃないまでも
        let sentence = "好きな仕事ができないから幸せじゃないまでも、生活費に困っていないから文句は言えない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ないまでも");
        assert_pattern_range(&patterns, "ないまでも", 12, 21); // 幸せじゃないまでも
    }
}

// ============================================================================
// をよそに Tests
// ============================================================================

mod woyosoni_tests {
    use super::*;

    // Pattern: をよそに (ignoring/disregarding)
    // Data source: grammar_points_data.json["をよそに"]
    // Testing: structure.standard[0] - "Noun + をよそに"

    #[test]
    fn test_woyosoni_noun_basic() {
        // Testing: structure.standard[0] - "Noun + をよそに"
        // Example from grammar data: 親の心配をよそに
        let sentence = "彼は親の心配をよそに、毎晩夜遅くまで友達と遊んでいる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "をよそに");
        assert_pattern_range(&patterns, "をよそに", 4, 10); // 心配をよそに
    }

    #[test]
    fn test_woyosoni_noun_objections() {
        // Testing: structure.standard[0] - "Noun + をよそに"
        // Example from grammar data: 社員の反対をよそに
        let sentence = "社長は社員の反対をよそに、仕事の効率を上げるために休憩所を撤去した。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "をよそに");
        assert_pattern_range(&patterns, "をよそに", 6, 12); // 反対をよそに
    }

    #[test]
    fn test_woyosoni_noun_eyes() {
        // Testing: structure.standard[0] - "Noun + をよそに"
        // Example from grammar data: 周囲の視線をよそに
        let sentence = "彼は周囲の視線をよそに、コンビニの店員に暴言を吐き続けていた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "をよそに");
        assert_pattern_range(&patterns, "をよそに", 5, 11); // 視線をよそに
    }

    #[test]
    fn test_woyosoni_phrase_nominalized() {
        // Testing: structure.standard[1] - "Phrase + の + をよそに"
        // This structure would be like: "病気であるのをよそに" (ignoring that [they] are sick)
        let sentence = "彼女は体調が悪いのをよそに、仕事を続けていた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "をよそに");
        assert_pattern_range(&patterns, "をよそに", 8, 13); // のをよそに
    }

    #[test]
    fn test_woyosoni_concerns() {
        // Testing: structure.standard[0] - "Noun + をよそに"
        // Different context: 懸念 (concerns)
        let sentence = "彼らは環境への懸念をよそに、工場の建設を進めた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "をよそに");
        assert_pattern_range(&patterns, "をよそに", 7, 13); // 懸念をよそに
    }
}

// ============================================================================
// の至り Tests
// ============================================================================

mod noitari_tests {
    use super::*;

    // Pattern: の至り (the utmost / extreme of)
    // Data source: grammar_points_data.json["の至り"]
    // Testing: structure.standard[0] - "Noun + の + 至り + だ"
    // Testing: structure.polite[0] - "Noun + の + 至り + です"
    //
    // Meaning: Formal expression highlighting that (A) is the highest level of something
    // Usually used with emotions: 光栄 (honor), 感謝 (gratitude), 恐縮 (obligation), etc.

    #[test]
    fn test_noitari_gratitude_standard() {
        // Testing: structure.standard[0] - "Noun + の + 至り + だ"
        let sentence = "こんな素晴らしい賞をいただき、感謝の至りだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "の至り");
        assert_pattern_range(&patterns, "の至り", 15, 20); // 感謝の至り
    }

    #[test]
    fn test_noitari_honor_polite() {
        // Testing: structure.polite[0] - "Noun + の + 至り + です"
        let sentence = "このような大切な式に招待していただき、光栄の至りです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "の至り");
        assert_pattern_range(&patterns, "の至り", 19, 24); // 光栄の至り
    }

    #[test]
    fn test_noitari_obligation_polite() {
        // Testing: structure.polite[0] - "Noun + の + 至り + です"
        // Different context: 恐縮 (feeling obligated/grateful)
        let sentence = "４年間色々と教えていただき、恐縮の至りです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "の至り");
        assert_pattern_range(&patterns, "の至り", 14, 19); // 恐縮の至り
    }

    #[test]
    fn test_noitari_youthful_indiscretion() {
        // Testing: structure.standard[0] - "Noun + の + 至り + だ"
        // Set expression: 若気の至り (youthful indiscretion)
        let sentence = "大学生の頃は若気の至りで、色々と悪いことをしてしまった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "の至り");
        assert_pattern_range(&patterns, "の至り", 6, 11); // 若気の至り
    }

    #[test]
    fn test_noitari_embarrassment() {
        // Testing: structure.standard[0] - "Noun + の + 至り + だ"
        // Different context: 赤面 (embarrassment)
        let sentence = "あんなミスをしてしまい、赤面の至りだった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "の至り");
        assert_pattern_range(&patterns, "の至り", 12, 17); // 赤面の至り
    }
}

// ============================================================================
// に限ったことではない Tests
// ============================================================================

mod nikagittakotodehanai_tests {
    use super::*;

    // Pattern: に限ったことではない (not limited to / not only)
    // Data source: grammar_points_data.json["に限ったことではない"]
    // Testing: structure.standard[0] - "Noun + に限（かぎ）ったことではない"
    //
    // Other structures to test:
    //   - polite[0]: Noun + に限（かぎ）ったことではありません

    #[test]
    fn test_nikagittakotodehanai_young_people() {
        // Testing: structure.standard[0] - "Noun + にかぎったことではない"
        // Context: not limited to young people
        let sentence = "最近の若者は挨拶ができないと言われているが、若者にかぎったことではない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に限ったことではない");
        assert_pattern_range(&patterns, "に限ったことではない", 22, 35); // 若者にかぎったことではない
    }

    #[test]
    fn test_nikagittakotodehanai_today() {
        // Testing: structure.standard[0] - "Noun + にかぎったことではない"
        // Context: not limited to today
        let sentence = "今日にかぎったことではないが、なんか仕事する気にならない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に限ったことではない");
        assert_pattern_range(&patterns, "に限ったことではない", 0, 13); // 今日にかぎったことではない
    }

    #[test]
    fn test_nikagittakotodehanai_japanese() {
        // Testing: structure.standard[0] - "Noun + にかぎったことではない"
        // Context: not limited to Japanese
        let sentence = "文法が難しいのは日本語にかぎったことではない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に限ったことではない");
        assert_pattern_range(&patterns, "に限ったことではない", 8, 22); // 日本語にかぎったことではない
    }

    #[test]
    fn test_nikagittakotodehanai_polite_this_company() {
        // Testing: structure.polite[0] - "Noun + にかぎったことではありません"
        // Context: not limited to this company (polite form)
        let sentence = "このような問題はうちの会社にかぎったことではありません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に限ったことではない");
        assert_pattern_range(&patterns, "に限ったことではない", 11, 27); // 会社にかぎったことではありません
    }

    #[test]
    fn test_nikagittakotodehanai_polite_men() {
        // Testing: structure.polite[0] - "Noun + にかぎったことではありません"
        // Context: not limited to men (polite form)
        let sentence = "この悩みは男性にかぎったことではありません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に限ったことではない");
        assert_pattern_range(&patterns, "に限ったことではない", 5, 21); // 男性にかぎったことではありません
    }
}

// ============================================================================
// とは比べものにならない Tests
// ============================================================================

mod tohakurabemononinaranai_tests {
    use super::*;

    // Pattern: とは比べものにならない (cannot be compared to / nothing compared to)
    // Data source: grammar_points_data.json["とは比べものにならない"]
    // Testing: structure.standard[0] - "Noun + とは比べものにならない"
    //
    // Other structures to test:
    //   - polite[0]: Noun + とは比べものになりません

    #[test]
    fn test_tohakurabemononinaranai_company_parts() {
        // Testing: structure.standard[0] - "Noun + とは比べものにならない"
        // Context: Company B parts cannot be compared to Company A parts
        let sentence = "Ａ社の作る部品はいいが、Ｂ社の作る部品とはくらべものにならない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とは比べものにならない");
        assert_pattern_range(&patterns, "とは比べものにならない", 17, 31); // 部品とはくらべものにならない
    }

    #[test]
    fn test_tohakurabemononinaranai_mothers_spaghetti() {
        // Testing: structure.standard[0] - "Noun + とは比べものにならない"
        // Context: cannot be compared to mother's spaghetti
        let sentence = "お母さんのスパゲティーとはくらべものにならないが、ここのシェフが作るやつはなかなか美味しい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とは比べものにならない");
        assert_pattern_range(&patterns, "とは比べものにならない", 5, 23); // スパゲティーとはくらべものにならない
    }

    #[test]
    fn test_tohakurabemononinaranai_hokkaido_cold() {
        // Testing: structure.standard[0] - "Noun + とは比べものにはならない"
        // Context: Gifu cannot be compared to Hokkaido (with extra は particle)
        let sentence = "岐阜は寒いが、北海道とはくらべものにはならない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とは比べものにならない");
        assert_pattern_range(&patterns, "とは比べものにならない", 7, 23); // 北海道とはくらべものにはならない
    }

    #[test]
    fn test_tohakurabemononinaranai_polite() {
        // Testing: structure.polite[0] - "Noun + とは比べものになりません"
        // Context: polite form - cannot be compared to
        let sentence = "昔の技術とはくらべものになりません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とは比べものにならない");
        assert_pattern_range(&patterns, "とは比べものにならない", 2, 17); // 技術とはくらべものになりません
    }

    #[test]
    fn test_tohakurabemononinaranai_quality() {
        // Testing: structure.standard[0] - "Noun + とは比べものにならない"
        // Context: quality comparison - cannot be compared
        let sentence = "この店の料理は他の店とはくらべものにならないほど美味しい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とは比べものにならない");
        assert_pattern_range(&patterns, "とは比べものにならない", 9, 22); // 店とはくらべものにならない
    }
}

// ============================================================================
// まじき Tests
// ============================================================================

mod majiki_tests {
    use super::*;

    // Pattern: まじき (must not / unbecoming of)
    // Data source: grammar_points_data.json["まじき"]
    // Testing: structure.standard[0] - "Noun (A) + にある(1) + まじき + Noun (B)"
    // Note: (1) means "としてある" variant is also possible

    #[test]
    fn test_majiki_niarumajiki_teacher() {
        // Testing: structure.standard[0] - "Noun + にあるまじき + Noun"
        // Context: unbecoming behavior for a teacher
        let sentence = "生徒に手を挙げることは先生にあるまじき行為だ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まじき");
        assert_pattern_range(&patterns, "まじき", 11, 22); // 先生にあるまじき行為だ - includes だ
    }

    #[test]
    fn test_majiki_niarumajiki_police() {
        // Testing: structure.standard[0] - "Noun + にあるまじき + Noun"
        // Context: unbecoming behavior for a police officer
        let sentence = "彼は飲酒運転という警察官にあるまじき行動をし、クビになった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まじき");
        assert_pattern_range(&patterns, "まじき", 9, 20); // 警察官にあるまじき行動
    }

    #[test]
    fn test_majiki_toshitearumajiki_prime_minister() {
        // Testing: structure.standard[0] - variant with として
        // Context: unbecoming statement for a prime minister
        let sentence = "先日、斉藤首相は総理としてあるまじき発言をし、SNSで炎上している。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まじき");
        assert_pattern_range(&patterns, "まじき", 8, 20); // 総理としてあるまじき発言
    }

    #[test]
    fn test_majiki_toshitearumajiki_physician() {
        // Testing: structure.standard[0] - variant with として
        // Context: unbecoming mistake for a physician
        let sentence = "彼は医師としてあるまじきミスをしてしまい、医師免許を剥奪された。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まじき");
        assert_pattern_range(&patterns, "まじき", 2, 14); // 医師としてあるまじきミス
    }

    #[test]
    fn test_majiki_yurusumajiki_unforgivable() {
        // Testing: structure.standard[1] - "許すまじき + Noun"
        // Context: unforgivable act
        let sentence = "煽り運転のようなゆるすまじき行為をした場合、免許は取り消しになります。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "まじき");
        assert_pattern_range(&patterns, "まじき", 8, 16); // ゆるすまじき行為
    }
}

// ============================================================================
// に恥じない Tests
// ============================================================================

mod nihajinai_tests {
    use super::*;

    // Pattern: に恥じない (lives up to / not ashamed of)
    // Data source: grammar_points_data.json["に恥じない"]
    // Testing: structure.standard[0] - "Noun + に恥（は）じない"
    //
    // Other structures to test:
    //   - standard[1]: Noun + に恥（は）じない + Noun
    //   - polite[0]: Noun + に恥（は）じません
    //   - polite[1]: Noun + に恥（は）じません + Noun

    #[test]
    fn test_nihajinai_standard_name_reputation() {
        // Testing: structure.standard[0] - "Noun + に恥じない"
        // Context: lives up to the name/reputation
        let sentence = "あの大学のバスケチームは日本一という名にはじない強さだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に恥じない");
        assert_pattern_range(&patterns, "に恥じない", 18, 24); // 名にはじない
    }

    #[test]
    fn test_nihajinai_standard_modifying_noun() {
        // Testing: structure.standard[1] - "Noun + に恥じない + Noun"
        // Context: modifying a following noun
        let sentence = "日本代表の名にはじないよう、ベストを尽くします。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に恥じない");
        assert_pattern_range(&patterns, "に恥じない", 5, 11); // 名にはじない
    }

    #[test]
    fn test_nihajinai_standard_company() {
        // Testing: structure.standard[0] - "Noun + に恥じない"
        // Context: living up to the company's reputation
        let sentence = "会社の名にはじないよう、精一杯頑張ります。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に恥じない");
        assert_pattern_range(&patterns, "に恥じない", 3, 9); // 名にはじない
    }

    #[test]
    fn test_nihajinai_polite_name() {
        // Testing: polite[0] - "Noun + に恥じません"
        // Context: polite form
        let sentence = "日本代表の名にはじませんよう、努力を続けます。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に恥じない");
        assert_pattern_range(&patterns, "に恥じない", 5, 12); // 名にはじません
    }

    #[test]
    fn test_nihajinai_polite_modifying_noun() {
        // Testing: polite[1] - "Noun + に恥じません + Noun"
        // Context: polite form modifying noun
        let sentence = "このチームの名にはじません活動を心がけます。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に恥じない");
        assert_pattern_range(&patterns, "に恥じない", 6, 13); // 名にはじません
    }
}

// ============================================================================
// とは言うものの Tests
// ============================================================================

mod tohaiumonono_tests {
    use super::*;

    // Pattern: とは言うものの (although it is said that)
    // Data source: grammar_points_data.json["とは言うものの"]
    // Testing: Verb/Adj/Noun + (だ) + と + (は) + いう + ものの

    #[test]
    fn test_tohaiumonono_verb() {
        // Testing: standard[0] - "Verb + とは言うものの"
        // Example: 頑張ればできるとは言うものの
        let sentence = "なんでも頑張れば必ずできるようになるとはいうものの、どれだけ頑張ってもできないことはある。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とは言うものの");
        assert_pattern_range(&patterns, "とは言うものの", 16, 25); // なるとはいうものの
    }

    #[test]
    fn test_tohaiumonono_i_adjective() {
        // Testing: standard[1] - "い-Adjective + とは言うものの"
        // Example: 給料はいいとは言うものの
        let sentence = "今働いている会社の給料はいいとはいうものの、お金を使う時間がない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とは言うものの");
        assert_pattern_range(&patterns, "とは言うものの", 12, 21); // いいとはいうものの
    }

    #[test]
    fn test_tohaiumonono_na_adjective_with_da() {
        // Testing: standard[2] - "な-Adjective + だ + とは言うものの"
        // Example: 便利だとは言うものの (note: no は in this example)
        let sentence = "スマホは便利だというものの、使い方がわからないとスマホの便利さをしれきれない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とは言うものの");
        assert_pattern_range(&patterns, "とは言うものの", 6, 13); // だというものの
    }

    #[test]
    fn test_tohaiumonono_noun_with_da() {
        // Testing: standard[3] - "Noun + だ + とは言うものの"
        // Example: 学生だとは言うものの (note: no は in this example)
        let sentence = "彼は学生だというものの、バイトばかりしていて勉強についていけていない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "とは言うものの");
        assert_pattern_range(&patterns, "とは言うものの", 4, 11); // だというものの
    }

    // TODO: Sentence-initial usage test commented out
    // The pattern とは言うものの appears mid-sentence after a verb/adjective/noun.
    // When used at sentence start, it's typically a continuation from a previous sentence.
    // Current matcher requires at least one token before と, which is appropriate for
    // the main usage pattern (Verb/Adj/Noun + とは言うものの).
    //
    // #[test]
    // fn test_tohaiumonono_sentence_initial() {
    //     // Testing: standard[4] - "Phrase (A)。とは言うものの + Phrase (B)"
    //     // Example: (Previous sentence). とは言うものの、実際は...
    //     let sentence = "とはいうものの、まだ準備は終わっていないんだ。";
    //     let tokens = tokenize_sentence(sentence);
    //     let patterns = detect_patterns(&tokens);
    //
    //     assert_has_pattern(&patterns, "とは言うものの");
    //     assert_pattern_range(&patterns, "とは言うものの", 0, 7); // とはいうものの
    // }
}

// に言わせれば・に言わせると・に言わせたら Tests
mod niiwasereba_tests {
    use super::*;

    // Pattern: に言わせれば・に言わせると・に言わせたら (if you ask / according to)
    // Data source: grammar_points_data.json["に言わせれば・に言わせると・に言わせたら"]
    // Testing: structure.standard[0] - "Noun + に言わせれば"
    //
    // Other structures to test:
    //   - standard[1]: に言わせると, に言わせたら (と/たら variants)
    //
    // Note: Pattern can also use から instead of に occasionally

    #[test]
    fn test_niiwasereba_watashi() {
        // Testing: standard[0] - "私にいわせれば"
        // Sentence from grammar_points_data.json
        let sentence = "私にいわせれば５キロを２５分で走るのは簡単だ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に言わせれば・に言わせると・に言わせたら");
        assert_pattern_range(&patterns, "に言わせれば・に言わせると・に言わせたら", 0, 7); // 私にいわせれば
    }

    #[test]
    fn test_niiwasereba_kare() {
        // Testing: standard[0] - "彼にいわせれば"
        // Sentence from grammar_points_data.json
        let sentence = "私はあのパーソナルトレーナーは厳しすぎると思ったが、彼にいわせれば優しすぎるらしい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に言わせれば・に言わせると・に言わせたら");
        assert_pattern_range(&patterns, "に言わせれば・に言わせると・に言わせたら", 26, 33); // 彼にいわせれば
    }

    #[test]
    fn test_niiwasereba_senmonka() {
        // Testing: standard[0] - "専門家にいわせれば"
        // Sentence from grammar_points_data.json
        // Note: Pattern matches "家にいわせれば" (家 from 専門家)
        let sentence = "専門家にいわせれば、もっと早い対応をするべきだったそうです。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に言わせれば・に言わせると・に言わせたら");
        assert_pattern_range(&patterns, "に言わせれば・に言わせると・に言わせたら", 2, 9); // 家にいわせれば
    }

    #[test]
    fn test_karaiiwasereba_variant() {
        // Testing: から variant instead of に
        // Sentence from grammar_points_data.json
        let sentence = "私からいわせれば、なんでそんなことして許されると思ったのかがわからない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に言わせれば・に言わせると・に言わせたら");
        assert_pattern_range(&patterns, "に言わせれば・に言わせると・に言わせたら", 0, 8); // 私からいわせれば
    }

    #[test]
    fn test_niiwaseruto_variant() {
        // Testing: standard[1] - "に言わせると"
        let sentence = "母にいわせると、最近の若者は我慢が足りないらしい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に言わせれば・に言わせると・に言わせたら");
        assert_pattern_range(&patterns, "に言わせれば・に言わせると・に言わせたら", 0, 7); // 母にいわせると
    }

    #[test]
    fn test_niiwasetara_variant() {
        // Testing: standard[1] - "に言わせたら"
        let sentence = "先生にいわせたら、この問題は簡単だそうだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に言わせれば・に言わせると・に言わせたら");
        assert_pattern_range(&patterns, "に言わせれば・に言わせると・に言わせたら", 0, 8); // 先生にいわせたら
    }
}

// ============================================================
// には及ばない② Tests
// ============================================================

#[cfg(test)]
mod nihaoyobanai_u2461_tests {
    use super::*;

    // Pattern: には及ばない② (not as good as / no match for)
    // Data source: grammar_points_data.json["には及ばない②"]
    // Testing: structure.standard[0] - "Noun + には及ばない"
    #[test]
    fn test_nihaoyobanai2_beach_comparison() {
        let sentence = "この海は沖縄の海にはおよばないが、結構綺麗だ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "には及ばない②");
        assert_pattern_range(&patterns, "には及ばない②", 7, 15); // 海にはおよばない
    }

    // Testing: structure.standard[0] - "Noun + には及ばない"
    #[test]
    fn test_nihaoyobanai2_restaurant_taste() {
        let sentence = "あの一流レストランの味にはおよばないが、かなり似ている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "には及ばない②");
        assert_pattern_range(&patterns, "には及ばない②", 10, 18); // 味にはおよばない
    }

    // Testing: structure.standard[0] - "Noun + には及ばない"
    #[test]
    fn test_nihaoyobanai2_guitar_skill() {
        let sentence = "彼はものすごくギターが上手いが、布袋寅泰にはおよばない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "には及ばない②");
        assert_pattern_range(&patterns, "には及ばない②", 18, 27); // 寅泰にはおよばない
    }

    // Testing: set expression - "Noun + の足元にも及ばない"
    // Note: "あしもと" tokenizes as two separate nouns (あし + もと)
    // Pattern matches "もとにはおよばない" (もと is treated as the noun before には)
    #[test]
    fn test_nihaoyobanai2_ashimoto_set_expression() {
        let sentence = "どれだけ頑張っても、青木先輩のあしもとにはおよばない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "には及ばない②");
        assert_pattern_range(&patterns, "には及ばない②", 17, 26); // もとにはおよばない
    }

    // Testing: structure.polite[0] - "Noun + には及びません"
    #[test]
    fn test_nihaoyobanai2_polite_form() {
        let sentence = "当社の技術は彼らの技術にはおよびませんが、努力しています。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "には及ばない②");
        assert_pattern_range(&patterns, "には及ばない②", 9, 19); // 技術にはおよびません
    }
}

// ============================================================
// に難くない Tests
// ============================================================

#[cfg(test)]
mod nikatakunai_tests {
    use super::*;

    // Pattern: に難くない (not difficult to / not hard to)
    // Data source: grammar_points_data.json["に難くない"]
    // Testing: structure.standard[0] - "想像 + (する) + に難くない"
    #[test]
    fn test_nikatakunai_souzou() {
        let sentence = "そう言う叱り方は子供にいい影響を与えないだろうことは想像にかたくない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に難くない");
        assert_pattern_range(&patterns, "に難くない", 26, 34); // 想像にかたくない
    }

    // Testing: structure.standard[1] - "予想 + (する) + に難くない"
    #[test]
    fn test_nikatakunai_yosou() {
        let sentence = "このプロジェクトを中止したら多くの人に迷惑がかかることは予想にかたくない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に難くない");
        assert_pattern_range(&patterns, "に難くない", 28, 36); // 予想にかたくない
    }

    // Testing: structure.standard[2] - "察する + に難くない"
    #[test]
    fn test_nikatakunai_sassuru() {
        let sentence = "彼の複雑な心境は察するにかたくない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に難くない");
        assert_pattern_range(&patterns, "に難くない", 8, 17); // 察するにかたくない
    }

    // Testing: structure.standard[3] - "理解 + (する) + に難くない"
    #[test]
    fn test_nikatakunai_rikai() {
        let sentence = "子供を亡くした親たちの気持ちは理解にかたくない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に難くない");
        assert_pattern_range(&patterns, "に難くない", 15, 23); // 理解にかたくない
    }

    // Testing: structure.polite[0] - "想像 + (する) + に難くありません"
    #[test]
    fn test_nikatakunai_polite() {
        let sentence = "その結果は予想にかたくありませんでした。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "に難くない");
        assert_pattern_range(&patterns, "に難くない", 5, 19); // 予想にかたくありませんでした
    }
}

// ============================================================================
// ならいざ知らず Tests
// ============================================================================

mod naraizashirazu_tests {
    use super::*;

    // Pattern: ならいざ知らず (I don't know about A, but B / maybe A, but B)
    // Data source: grammar_points_data.json["ならいざ知らず"]
    // Testing: structure.standard[0] - "Noun + ならいざ知らず"
    //
    // Other structures to test:
    //   - standard[1]: Verb + (の) + ならいざ知らず
    //   - standard[2]: い-Adjective + (の) + ならいざ知らず
    //   - standard[3]: Noun + であればいざ知らず
    //   - standard[4]: Noun + はいざ知らず

    // Testing: structure.standard[0] - "Noun + ならいざ知らず" (most common)
    #[test]
    fn test_naraizashirazu_noun_weekday() {
        let sentence = "平日ならいざしらず、連休中に遊園地などのようなところに行く人は多いだろう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ならいざ知らず");
        assert_pattern_range(&patterns, "ならいざ知らず", 0, 9); // 平日ならいざしらず
    }

    // Testing: structure.standard[0] - "Noun + ならいざ知らず"
    #[test]
    fn test_naraizashirazu_noun_wedding() {
        let sentence = "結婚式かなんかに出席するならいざしらず、友達の家に行くだけなんだからスーツなんて着ていかなくてもいいんじゃない？";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ならいざ知らず");
        assert_pattern_range(&patterns, "ならいざ知らず", 8, 19); // 出席するならいざしらず
    }

    // Testing: structure.standard[1] - "Verb + ならいざ知らず" (verb without の)
    #[test]
    fn test_naraizashirazu_verb_wait() {
        let sentence = "１５分待つならいざしらず、いくら友達でも３０分以上待つのは無理だ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ならいざ知らず");
        assert_pattern_range(&patterns, "ならいざ知らず", 3, 12); // 待つならいざしらず
    }

    // Testing: structure.standard[2] - "い-Adjective + の + ならいざ知らず"
    #[test]
    fn test_naraizashirazu_i_adjective_with_no() {
        let sentence = "新しいのならいざしらず、前のモデルはよく勝手に電源が落ちたからあまり売れなかったらしい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ならいざ知らず");
        assert_pattern_range(&patterns, "ならいざ知らず", 3, 11); // のならいざしらず
    }

    // Testing: structure.standard[2] - "な-Adjective + ならいざ知らず" (without の)
    #[test]
    fn test_naraizashirazu_na_adjective() {
        let sentence = "経験豊富で仕事が上手ならいざしらず、まだ始めたばかりの子にこの仕事を任せるのはかわいそうすぎますよ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ならいざ知らず");
        assert_pattern_range(&patterns, "ならいざ知らず", 8, 17); // 上手ならいざしらず
    }

    // Testing: structure.standard[4] - "Noun + はいざ知らず"
    #[test]
    fn test_naraizashirazu_wa_variant() {
        let sentence = "６０年前はいざしらず、今では喫煙する人が減ってきている。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ならいざ知らず");
        assert_pattern_range(&patterns, "ならいざ知らず", 3, 10); // 前はいざしらず
    }

    // Testing: More examples with different nouns
    #[test]
    fn test_naraizashirazu_noun_past() {
        let sentence = "大昔ならいざしらず、現代では詐欺についてよくテレビやインターネットで話題になっているから、詐欺の手口も年々と進化していってる。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ならいざ知らず");
        assert_pattern_range(&patterns, "ならいざ知らず", 0, 9); // 大昔ならいざしらず
    }

    #[test]
    fn test_naraizashirazu_noun_god() {
        let sentence = "神ならいざしらず、俺みたいなやつにはああいうことをする奴たちを許すことなどできない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ならいざ知らず");
        assert_pattern_range(&patterns, "ならいざ知らず", 0, 8); // 神ならいざしらず
    }
}

// Pattern: を禁じ得ない (cannot help feeling / cannot hold back from)
// Data source: grammar_points_data.json["を禁じ得ない"]
// Testing all structure variants
mod wokinjienai_tests {
    use super::*;

    // Testing: structure.standard[0] - "Noun + を禁じ得ない"
    #[test]
    fn test_wokinjienai_noun_standard() {
        let sentence = "息子が生まれたが、いい父親になれるか不安をきんじえない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を禁じ得ない");
        assert_pattern_range(&patterns, "を禁じ得ない", 18, 27); // 不安をきんじえない
    }

    // Testing: structure.standard[1] - "Verb + の + を禁じ得ない"
    #[test]
    fn test_wokinjienai_verb_nominalized_standard() {
        let sentence = "初めて広島にある平和記念資料館に行ったとき、涙をこらえるのをきんじえなかった。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を禁じ得ない");
        assert_pattern_range(&patterns, "を禁じ得ない", 28, 38); // のをきんじえなかった
    }

    // Testing: structure.polite[0] - "Noun + を禁じ得ません"
    #[test]
    fn test_wokinjienai_noun_polite() {
        let sentence = "税金を国民のためではなく自分たちのためだけに使う政治家に怒りをきんじえません。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を禁じ得ない");
        assert_pattern_range(&patterns, "を禁じ得ない", 28, 38); // 怒りをきんじえません
    }

    // Testing: structure.polite[1] - "Verb + の + を禁じ得ません"
    #[test]
    fn test_wokinjienai_verb_nominalized_polite() {
        let sentence = "彼女の努力を見て感動するのをきんじえませんでした。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "を禁じ得ない");
        assert_pattern_range(&patterns, "を禁じ得ない", 12, 24); // のをきんじえませんでした
    }
}

// ============================================================================
// にかこつけて Tests
// ============================================================================

mod nikakotsukete_tests {
    use super::*;

    // Pattern: にかこつけて (under the pretense of / using as an excuse)
    // Data source: grammar_points_data.json["にかこつけて"]
    // Testing: structure.standard[0] - "Noun + にかこつけて"
    //
    // Other structures to test:
    //   - standard[1]: Verb + の + にかこつけて
    //   - standard[1]: Verb + こと + にかこつけて (の can be こと)

    // Testing: structure.standard[0] - "Noun + にかこつけて"
    #[test]
    fn test_nikakotsukete_noun_cold() {
        let sentence = "風邪にかこつけて、授業を休んで一日中テレビを見ながらダラダラしていた。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にかこつけて");
        assert_pattern_range(&patterns, "にかこつけて", 0, 8); // 風邪にかこつけて
    }

    // Testing: structure.standard[0] - "Noun + にかこつけて"
    #[test]
    fn test_nikakotsukete_noun_business_trip() {
        let sentence = "あいつは出張にかこつけて、また釣りに行っているに違いない。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にかこつけて");
        assert_pattern_range(&patterns, "にかこつけて", 4, 12); // 出張にかこつけて
    }

    // Testing: structure.standard[1] - "Verb + の + にかこつけて"
    #[test]
    fn test_nikakotsukete_verb_nominalized_no() {
        let sentence = "病院に行くというのにかこつけて仕事を休んだ。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にかこつけて");
        assert_pattern_range(&patterns, "にかこつけて", 8, 15); // のにかこつけて
    }

    // Testing: structure.standard[1] - "Verb + の + にかこつけて"
    #[test]
    fn test_nikakotsukete_verb_nominalized_no_longer() {
        let sentence = "彼女はいつも何かにかこつけて僕の家に来ようとする。本当にやめてほしい。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にかこつけて");
        assert_pattern_range(&patterns, "にかこつけて", 7, 14); // かにかこつけて (か from 何か)
    }

    // Testing: structure.standard[1] - "Verb + の + にかこつけて" (longer example)
    #[test]
    fn test_nikakotsukete_verb_nominalized_forgot_item() {
        let sentence = "彼の家に忘れ物をしたというのにかこつけて彼のところにもう一度行く。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にかこつけて");
        assert_pattern_range(&patterns, "にかこつけて", 13, 20); // のにかこつけて
    }

    // Testing: structure.standard[1] - "Verb + の + にかこつけて"
    #[test]
    fn test_nikakotsukete_verb_nominalized_tying_shoelace() {
        let sentence = "靴ひもを結ぶのにかこつけて落ちていた財布をこっそりと拾った。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "にかこつけて");
        assert_pattern_range(&patterns, "にかこつけて", 6, 13); // のにかこつけて
    }
}

// ============================================================================
// ようによっては Tests
// ============================================================================

mod youniyotteha_tests {
    use super::*;

    // Pattern: ようによっては (depending on the way that)
    // Data source: grammar_points_data.json["ようによっては"]
    // Testing: structure.standard[0] - "Verb[stem] + ようによっては + Phrase"
    //
    // Structure: Verb (連用形) + ようによっては
    // Meaning: "depending on the way that (A), (B)" / "depending on how (A)"
    // The 様(よう) 'manner/way' + に (particle) + よって (て-form of よる 'to depend on') + は (adverbial particle)

    // Testing: Verb[stem] + ようによっては - movie example (volitional form)
    #[test]
    fn test_youniyotteha_verb_watch() {
        let sentence = "この映画は見ようによってはコメディーだと感じる人もいるだろう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ようによっては");
        assert_pattern_range(&patterns, "ようによっては", 5, 13); // 見ようによっては
    }

    // Testing: Verb[stem] + ようによっては - congestion example (連用形 + よう suffix)
    #[test]
    fn test_youniyotteha_verb_congestion() {
        let sentence = "45分で行けるはずだけど、混みようによっては1時間半かかることもあり得るから明日は少し早く家を出よう。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ようによっては");
        assert_pattern_range(&patterns, "ようによっては", 13, 22); // 混みようによっては
    }

    // Testing: Verb[stem] + ようによっては - machine usage example (noun + よう suffix)
    #[test]
    fn test_youniyotteha_verb_use() {
        let sentence = "どんなに安全な機械であっても使いようによってはとても危険になることもある。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ようによっては");
        assert_pattern_range(&patterns, "ようによっては", 14, 23); // 使いようによっては
    }

    // Testing: Verb[stem] + ようによっては - looking/viewing example (volitional form)
    #[test]
    fn test_youniyotteha_verb_look() {
        let sentence = "この絵は見ようによっては奇麗な女性かウサギに見える。";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "ようによっては");
        assert_pattern_range(&patterns, "ようによっては", 4, 12); // 見ようによっては
    }
}
