use super::*;

// ========== Phase 1: Adverbs & Standalone Expressions (18 patterns) ==========

#[test]
fn test_toutei() {
    let sentence = "とうてい無理だ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "toutei"),
        "Expected toutei pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_yoppodo() {
    let sentence = "よっぽど疲れた";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "yoppodo"),
        "Expected yoppodo pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_iyoiyo() {
    let sentence = "いよいよ始まる";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "iyoiyo"),
        "Expected iyoiyo pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_sekkaku() {
    let sentence = "せっかく来たのに";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "sekkaku"),
        "Expected sekkaku pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_yamuoezu() {
    let sentence = "やむをえず断った";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "yamuoezu_verb"),
        "Expected yamuoezu pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_yappari() {
    let sentence = "やっぱり無理だった";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "yappari"),
        "Expected yappari pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_narubeku() {
    let sentence = "なるべく早く来てください";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "narubeku"),
        "Expected narubeku pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_tashika() {
    let sentence = "たしか彼は来ると言った";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "tashika"),
        "Expected tashika pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_man_ichi_kanji() {
    let sentence = "万一の事態に備える";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "man_ichi"),
        "Expected man_ichi pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_nanishiro_kanji() {
    let sentence = "何しろ難しい問題だ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "nanishiro_kanji"),
        "Expected nanishiro_kanji pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_sorenishitemo() {
    let sentence = "それにしても良い天気だ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "sorenishitemo"),
        "Expected sorenishitemo pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_tachimachi() {
    let sentence = "たちまち有名になった";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "tachimachi"),
        "Expected tachimachi pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_sasugani() {
    let sentence = "さすがに疲れた";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "sasugani_split"),
        "Expected sasugani pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_itsunomanika_kana() {
    let sentence = "いつのまにか寝ていた";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "itsunomanika"),
        "Expected itsunomanika pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_itsunomanika_kanji() {
    let sentence = "いつの間にか春になった";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "itsunomanika_split"),
        "Expected itsunomanika_split pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_aete() {
    let sentence = "あえて困難な道を選んだ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "aete"),
        "Expected aete pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_semete() {
    let sentence = "せめて一度は行きたい";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "semete"),
        "Expected semete pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_nantoittemo_kanji() {
    let sentence = "何といっても健康が一番だ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "nantoittemo_split_kanji"),
        "Expected nantoittemo pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_nantoittemo_kana() {
    let sentence = "なんといっても家族が大切だ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "nantoittemo_adverb_split"),
        "Expected nantoittemo_kana pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_rou_ni() {
    let sentence = "ろくに勉強しない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "rou_ni"),
        "Expected rou_ni pattern not detected in '{}'",
        sentence
    );
}

// ========== Phase 2: Verb Suffixes & Auxiliaries (11 patterns) ==========

#[test]
fn test_kaneru() {
    let sentence = "賛成しかねる";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "kaneru"),
        "Expected kaneru pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_kanenai() {
    let sentence = "失敗しかねない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "kanenai"),
        "Expected kanenai pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_tamaranai_verb() {
    let sentence = "暑くてたまらない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "tamaranai"),
        "Expected tamaranai pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_naide_sumu() {
    let sentence = "謝らないで済む";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "naide_sumu_split"),
        "Expected naide_sumu pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_kara_naru_noun() {
    let sentence = "五人からなるチーム";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "kara_naru"),
        "Expected kara_naru pattern not detected in '{}' (noun)",
        sentence
    );
}

#[test]
fn test_yori_shikata_ganai_kanji() {
    let sentence = "待つより仕方がない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "yori_shikata_ganai"),
        "Expected yori_shikata_ganai pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_yori_shikata_ganai_kana() {
    let sentence = "行くよりしかたがない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "yori_shikata_ganai_kana"),
        "Expected yori_shikata_ganai_kana pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ta_ue_de() {
    let sentence = "考えた上で決める";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ta_ue_de"),
        "Expected ta_ue_de pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ta_ue_de_negative() {
    // Test that physical location "on" doesn't match
    let sentence = "体の上で戦ってる";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        !has_pattern(&patterns, "ta_ue_de"),
        "ta_ue_de pattern should not match '{}' (physical location, not 'after doing')",
        sentence
    );
}

#[test]
fn test_ni_ataru_kana() {
    let sentence = "社長にあたる人物";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ni_ataru_compound"),
        "Expected ni_ataru pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ni_ataru_kanji() {
    let sentence = "百万円に当たる金額";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ni_ataru_compound_kanji"),
        "Expected ni_ataru_kanji pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_gotoshi_kanji() {
    let sentence = "夢の如し";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "gotoshi_kanji"),
        "Expected gotoshi_kanji pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_tsuujite_kanji() {
    let sentence = "一年通じて暖かい";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "tsuujite_verb_kanji"),
        "Expected tsuujite_kanji pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_noboru() {
    let sentence = "百万人にのぼる被害者";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "noboru"),
        "Expected noboru pattern not detected in '{}'",
        sentence
    );
}

// ========== Phase 3: Particle Patterns & Conjunctions (11 patterns) ==========

#[test]
fn test_gatera_verb() {
    let sentence = "散歩がてら買い物する";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "gatera_split"),
        "Expected gatera pattern not detected in '{}' (verb)",
        sentence
    );
}

#[test]
fn test_oyobi_kanji() {
    let sentence = "日本及び中国";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "oyobi"),
        "Expected oyobi pattern not detected in '{}' (kanji)",
        sentence
    );
}

#[test]
fn test_oyobi_hiragana() {
    let sentence = "販売および賭博";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "oyobi"),
        "Expected oyobi pattern not detected in '{}' (hiragana)",
        sentence
    );
}

#[test]
fn test_oyobi_negative_verb() {
    // Test that verb form 及ぶ does NOT match
    let sentence = "礼には及びません";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        !has_pattern(&patterns, "oyobi"),
        "oyobi pattern should not match '{}' (verb form 及ぶ, not conjunction)",
        sentence
    );
}

#[test]
fn test_sei_ka() {
    let sentence = "疲れたせいか眠い";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "sei_ka"),
        "Expected sei_ka pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_yueni() {
    let sentence = "それゆえに失敗した";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "yueni_split"),
        "Expected yueni pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ippou_dewa() {
    let sentence = "一方では良い結果だ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ippou_dewa_split"),
        "Expected ippou_dewa pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_mono_no_compound() {
    // Test: Verb + ものの (Kagome treats as single particle token)
    let sentence = "頑張ったものの失敗した";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "mono_no"),
        "Expected mono_no pattern not detected in '{}' (compound particle)",
        sentence
    );
}

#[test]
fn test_mono_no_split() {
    // Test: Adjective/Noun + もの + の (Kagome splits into two tokens)
    let sentence = "高いものの品質が良い";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "mono_no_split"),
        "Expected mono_no_split pattern not detected in '{}' (split tokens)",
        sentence
    );
}

#[test]
fn test_kuse_ni() {
    let sentence = "知っているくせに言わない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "kuse_ni_split"),
        "Expected kuse_ni pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_kaketeha() {
    let sentence = "料理にかけては彼が一番だ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "kaketeha_compound"),
        "Expected kaketeha pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_itaru_made_kana() {
    let sentence = "子供からお年寄りにいたるまで";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ni_itaru_made"),
        "Expected itaru_made pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_itaru_made_kanji() {
    let sentence = "老若男女に至るまで";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ni_itaru_made_kanji"),
        "Expected itaru_made_kanji pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_igai_no() {
    let sentence = "日曜日以外の日";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "igai_no"),
        "Expected igai_no pattern not detected in '{}'",
        sentence
    );
}

// ========== Phase 4: Fixed Expressions (20 patterns) ==========

#[test]
fn test_ba_ii_noni() {
    let sentence = "来ればいいのに";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ba_ii_noni"),
        "Expected ba_ii_noni pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ba_ii_noni_kanji() {
    let sentence = "勉強すれば良いのに";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ba_yoi_noni_split"),
        "Expected ba_ii_noni pattern not detected in '{}' (良い)",
        sentence
    );
}

#[test]
fn test_wake_desu() {
    let sentence = "つまりそういうわけです";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "wake_desu"),
        "Expected wake_desu pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_wake_da() {
    let sentence = "そういうわけだ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "wake_da"),
        "Expected wake_da pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_you_na_ki_ga_suru() {
    let sentence = "雨が降るような気がする";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "you_na_ki_ga_suru"),
        "Expected you_na_ki_ga_suru pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ni_ki_wo_tsukeru() {
    let sentence = "健康に気をつける";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ni_ki_wo_tsukeru"),
        "Expected ni_ki_wo_tsukeru pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_betsuni_nai() {
    let sentence = "別に問題ない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "betsuni_nai_split"),
        "Expected betsuni_nai pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_wake_niwa_ikanai() {
    let sentence = "行くわけにはいかない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "wake_niwa_ikanai_short"),
        "Expected wake_niwa_ikanai pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_dewa_nai_darou_ka() {
    let sentence = "これは間違いではないだろうか";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "dewa_nai_darou_ka_full_split"),
        "Expected dewa_nai_darou_ka pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_to_iu_wake_dewa_nai() {
    let sentence = "嫌いというわけではない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "to_iu_wake_dewa_nai_compound"),
        "Expected to_iu_wake_dewa_nai pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ni_koshita_koto_wa_nai() {
    let sentence = "行くに越したことはない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ni_koshita_koto_wa_nai"),
        "Expected ni_koshita_koto_wa_nai pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_sashitsukaenai_kanji() {
    let sentence = "差し支えない範囲で";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "sashitsukaenai_kanji"),
        "Expected sashitsukaenai_kanji pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_nai_wake_niwa_ikanai() {
    let sentence = "行かないわけにはいかない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "nai_wake_niwa_ikanai_short"),
        "Expected nai_wake_niwa_ikanai pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_to_ittemo_kanji() {
    let sentence = "簡単と言っても難しい";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "to_ittemo"),
        "Expected to_ittemo pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ga_ki_ni_naru() {
    let sentence = "結果が気になる";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ga_ki_ni_naru"),
        "Expected ga_ki_ni_naru pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_omou_you_ni() {
    let sentence = "思うようにいかない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "omou_you_ni"),
        "Expected omou_you_ni pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_mono_desukara() {
    let sentence = "初めてのものですから";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "mono_desukara"),
        "Expected mono_desukara pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_mono_dakara() {
    let sentence = "子供のものだから";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "mono_dakara"),
        "Expected mono_dakara pattern not detected in '{}'",
        sentence
    );
}

// ========== Negative Tests (Edge Cases) ==========

#[test]
fn test_rou_ni_negative() {
    // Test that ろ in other contexts doesn't match
    let sentence = "白い";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        !has_pattern(&patterns, "rou_ni"),
        "rou_ni pattern should not match '{}' (白い adjective)",
        sentence
    );
}

#[test]
fn test_wake_desu_negative() {
    // Test that わけ in contexts like 訳す doesn't match
    let sentence = "訳す仕事";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        !has_pattern(&patterns, "wake_desu"),
        "wake_desu pattern should not match '{}' (訳す verb)",
        sentence
    );
}

#[test]
fn test_mono_no_negative_mononoke() {
    // Test that もののけ (monster) doesn't match as ものの grammar pattern
    let sentence = "もののけが現れた";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    // Kagome should tokenize もののけ as a different token, not splitting it
    assert!(
        !has_pattern(&patterns, "mono_no") && !has_pattern(&patterns, "mono_no_split"),
        "mono_no patterns should not match '{}' (compound word もののけ)",
        sentence
    );
}
