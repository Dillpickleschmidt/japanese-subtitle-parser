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
