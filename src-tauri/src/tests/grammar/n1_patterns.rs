use super::*;

// ========== Phase 1: Suffix Patterns (9 patterns, ~18 tests) ==========

mod meku_tests {
    use super::*;

    #[test]
    fn basic_form() {
        let sentence = "皮肉めく発言をした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "meku");
        assert_pattern_range(&patterns, "meku", 0, 4);
    }

    #[test]
    fn te_form() {
        let sentence = "冗談めいて言った";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "meku");
    }

    #[test]
    fn ta_form_modifying_noun() {
        let sentence = "謎めいた雰囲気がある";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "meku");
    }

    #[test]
    fn progressive_form() {
        let sentence = "冬めいている";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "meku");
    }

    #[test]
    fn te_kita_form() {
        let sentence = "春めいてきた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "meku_compound");
    }
}

mod mamire_tests {
    use super::*;

    #[test]
    fn basic_noun_form() {
        let sentence = "服泥まみれじゃん";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "mamire_compound");
    }

    #[test]
    fn with_no_particle() {
        let sentence = "泥まみれの服を洗濯機に入れる";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "mamire_compound");
    }

    #[test]
    fn with_ninatte_form() {
        let sentence = "犬が血まみれになっていた";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        assert_has_pattern(&patterns, "mamire_compound");
    }
}

#[test]
fn test_zukume() {
    let sentence = "黒ずくめの男が立っていた";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "zukume"),
        "Expected zukume pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ppanashi() {
    let sentence = "電気を付けっぱなしにした";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ppanashi"),
        "Expected ppanashi pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ppanashi_false_positive() {
    // Negative test: standalone っぱなし shouldn't match randomly
    let sentence = "放しっぱなしはダメだ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    // Should match since it's verb stem + っぱなし
    assert!(
        has_pattern(&patterns, "ppanashi"),
        "Expected ppanashi to match in '{}'",
        sentence
    );
}

#[test]
fn test_kiwamaru() {
    let sentence = "失礼極まる態度だ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "kiwamaru"),
        "Expected kiwamaru pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_kiwamarinai() {
    let sentence = "危険極まりない行動だ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "kiwamarinai"),
        "Expected kiwamarinai pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_kiwamaru_false_positive() {
    // Negative test: transitive 極める vs intransitive grammatical 極まる
    let sentence = "技術を極める";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    // Should NOT match - this is the transitive verb 極める (master), not the pattern
    assert!(
        !has_pattern(&patterns, "kiwamaru"),
        "kiwamaru pattern should not match transitive 極める in '{}'",
        sentence
    );
}

#[test]
fn test_beku() {
    let sentence = "見るべく行った";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "beku"),
        "Expected beku pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_bekarazu() {
    let sentence = "入るべからず";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "bekarazu"),
        "Expected bekarazu pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_majiki() {
    let sentence = "あるまじき行為だ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "majiki"),
        "Expected majiki pattern not detected in '{}'",
        sentence
    );
}

// ========== Phase 2: Simple Fixed Expressions (20 patterns) ==========

#[test]
fn test_nari() {
    let sentence = "家に帰るなり寝てしまった";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "nari"),
        "Expected nari pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ya_inaya() {
    let sentence = "聞くや否や駆けつけた";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ya_inaya"),
        "Expected ya_inaya pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ya_inaya_kana() {
    let sentence = "見るやいなや飛び出した";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ya_inaya_single"),
        "Expected ya_inaya_single pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ga_hayai_ka() {
    let sentence = "着くが早いか電話をかけた";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ga_hayai_ka"),
        "Expected ga_hayai_ka pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ga_saigo() {
    let sentence = "始めるが最後やめられない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ga_saigo"),
        "Expected ga_saigo pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_gotoki() {
    let sentence = "彼ごとき人間には負けない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "gotoki"),
        "Expected gotoki pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_wo_kawakiri_ni() {
    let sentence = "東京を皮切りに全国で開催する";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "wo_kawakiri_ni"),
        "Expected wo_kawakiri_ni pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_wo_motte() {
    let sentence = "本日をもって閉店します";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "wo_motte"),
        "Expected wo_motte pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_nakushiteha() {
    let sentence = "努力なくしては成功しない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "nakushiteha"),
        "Expected nakushiteha pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_nashini() {
    let sentence = "予告なしに訪問した";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "nashini"),
        "Expected nashini pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_naradewa() {
    let sentence = "彼ならではの発想だ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "naradewa"),
        "Expected naradewa pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ni_taru() {
    let sentence = "信頼に足る人物だ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ni_taru"),
        "Expected ni_taru pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_toatte() {
    let sentence = "有名人とあって多くの人が集まった";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "toatte"),
        "Expected toatte pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_katagata() {
    let sentence = "散歩かたがた買い物に行く";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "katagata"),
        "Expected katagata pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_wo_kagiri_ni() {
    let sentence = "今日を限りに辞める";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "wo_kagiri_ni"),
        "Expected wo_kagiri_ni pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_wo_hete() {
    let sentence = "長い年月を経て完成した";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "wo_hete"),
        "Expected wo_hete pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_wo_oshite() {
    let sentence = "病をおして出席した";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "wo_oshite"),
        "Expected wo_oshite pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_wo_fumaete() {
    let sentence = "前回の結果を踏まえて改善する";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "wo_fumaete"),
        "Expected wo_fumaete pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_te_yamanai() {
    let sentence = "心から願ってやまない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "te_yamanai"),
        "Expected te_yamanai pattern not detected in '{}'",
        sentence
    );
}

// ========== Phase 3: Conditional/Concessive Patterns (12 patterns) ==========

#[test]
fn test_to_omoikiya() {
    let sentence = "勝つと思いきや負けてしまった";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "to_omoikiya"),
        "Expected to_omoikiya pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_to_areba() {
    let sentence = "君のためとあれば何でもする";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "to_areba"),
        "Expected to_areba pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ta_tokoro_de() {
    let sentence = "謝ったところで許されない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ta_tokoro_de"),
        "Expected ta_tokoro_de pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_de_are() {
    let sentence = "誰であれ平等に扱う";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "de_are"),
        "Expected de_are pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_to_wa_ie() {
    let sentence = "子供とはいえ分かるだろう";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "to_wa_ie"),
        "Expected to_wa_ie pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_you_ga() {
    let sentence = "雨が降ろうが行く";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "you_ga"),
        "Expected you_ga pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_nai_made_mo() {
    let sentence = "完璧でないまでも十分だ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "nai_made_mo"),
        "Expected nai_made_mo pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_nagara_mo() {
    let sentence = "知りながらも黙っていた";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "nagara_mo"),
        "Expected nagara_mo pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_dewa_arumaishi() {
    let sentence = "子供ではあるまいし分かるでしょ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "dewa_arumaishi"),
        "Expected dewa_arumaishi pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_to_shita_tokoro_de() {
    let sentence = "今更謝るとしたところで遅い";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "to_shita_tokoro_de"),
        "Expected to_shita_tokoro_de pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_to_iedomo() {
    let sentence = "天才といえども努力が必要だ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "to_iedomo"),
        "Expected to_iedomo pattern not detected in '{}'",
        sentence
    );
}

// ========== Phase 4: Complex Multi-Word Expressions (12 patterns) ==========

#[test]
fn test_tomo_naruto() {
    let sentence = "プロともなると違う";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "tomo_naruto"),
        "Expected tomo_naruto pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ni_taenai() {
    let sentence = "見るに堪えない光景だった";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ni_taenai"),
        "Expected ni_taenai pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ni_sokushite() {
    let sentence = "規則に即して行動する";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ni_sokushite"),
        "Expected ni_sokushite pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_to_aimatte() {
    let sentence = "努力と相まって成功した";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "to_aimatte"),
        "Expected to_aimatte pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_wo_yosoni() {
    let sentence = "反対をよそに実行した";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "wo_yosoni"),
        "Expected wo_yosoni pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_temo_sashitsukaenai() {
    let sentence = "使ってもさしつかえない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "temo_sashitsukaenai"),
        "Expected temo_sashitsukaenai pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_wo_kinjienai() {
    let sentence = "感動を禁じ得ない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "wo_kinjienai"),
        "Expected wo_kinjienai pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_wo_yoginakusareru() {
    let sentence = "中止を余儀なくされる";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "wo_yoginakusareru"),
        "Expected wo_yoginakusareru pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_te_karatoiumono() {
    let sentence = "会ってからというもの幸せだ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "te_karatoiumono"),
        "Expected te_karatoiumono pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_nimo_mashite() {
    let sentence = "前にもまして元気だ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "nimo_mashite"),
        "Expected nimo_mashite pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ni_hikikae() {
    let sentence = "兄にひきかえ弟は静かだ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ni_hikikae"),
        "Expected ni_hikikae pattern not detected in '{}'",
        sentence
    );
}

// ========== Phase 5: Evaluative/Emphatic Patterns (12 patterns, 13 tests) ==========

#[test]
fn test_ikan_de() {
    // Test: Noun + いかん + で
    let sentence = "結果いかんで決まる";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ikan_de"),
        "Expected ikan_de pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ikan_no_da() {
    // Test: Noun + の + いかん + だ (sentence-final)
    let sentence = "君の努力のいかんだ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ikan_no_da"),
        "Expected ikan_no_da pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ikan_niyotte() {
    // Test: Noun + いかん + によって
    let sentence = "態度いかんによって変わる";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ikan_niyotte"),
        "Expected ikan_niyotte pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ikan_shidai() {
    // Test: Noun + いかん + 次第
    let sentence = "気分いかん次第で決める";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ikan_shidai"),
        "Expected ikan_shidai pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ikan_false_positive_prohibition() {
    // Negative test: ちゃいかん (must not) should NOT match ikan patterns
    let sentence = "入っちゃいかん";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    // Should NOT match any ikan patterns (they all require specific particles after いかん)
    assert!(
        !has_pattern(&patterns, "ikan_de")
            && !has_pattern(&patterns, "ikan_da")
            && !has_pattern(&patterns, "ikan_niyotte")
            && !has_pattern(&patterns, "ikan_shidai"),
        "ikan pattern should not match prohibition 'ちゃいかん' in '{}'",
        sentence
    );
}

#[test]
fn test_ikan_false_positive_exclamation() {
    // Negative test: Exclamation いかん! should NOT match
    let sentence = "いかん！逃げろ！";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    // Should NOT match any ikan patterns
    assert!(
        !has_pattern(&patterns, "ikan_de")
            && !has_pattern(&patterns, "ikan_da")
            && !has_pattern(&patterns, "ikan_niyotte")
            && !has_pattern(&patterns, "ikan_shidai"),
        "ikan pattern should not match exclamation 'いかん!' in '{}'",
        sentence
    );
}

#[test]
fn test_taritomo() {
    let sentence = "一人たりとも許さない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "taritomo"),
        "Expected taritomo pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_kirai_ga_aru() {
    let sentence = "遅れるきらいがある";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "kirai_ga_aru"),
        "Expected kirai_ga_aru pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_shimatsu_da() {
    let sentence = "泣く始末だった";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "shimatsu_da") || has_pattern(&patterns, "shimatsu_datta"),
        "Expected shimatsu_da pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_warini() {
    let sentence = "安い割りに良い";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "warini") || has_pattern(&patterns, "wariniha"),
        "Expected warini pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_wariniha() {
    let sentence = "値段の割りには美味しい";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "wariniha") || has_pattern(&patterns, "warini"),
        "Expected wariniha pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_kai_mo_naku() {
    let sentence = "努力の甲斐もなく失敗した";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "kai_mo_naku"),
        "Expected kai_mo_naku pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_dake_mashi() {
    let sentence = "怪我がないだけましだ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "dake_mashi"),
        "Expected dake_mashi pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_naide_wa_sumanai() {
    let sentence = "謝らないではすまない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "naide_wa_sumanai"),
        "Expected naide_wa_sumanai pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_koto_nashini() {
    let sentence = "休むことなしに働く";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "koto_nashini"),
        "Expected koto_nashini pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_de_sura() {
    let sentence = "子供ですら分かる";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "de_sura") || has_pattern(&patterns, "sura"),
        "Expected de_sura pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_sura() {
    let sentence = "彼すら知らない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "sura") || has_pattern(&patterns, "de_sura"),
        "Expected sura pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_nagarani_umare() {
    // Note: This sentence uses にして form, so will match nagarani_umare_shite
    let sentence = "生まれながらにして才能がある";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "nagarani_umare_shite"),
        "Expected nagarani_umare_shite pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_nagarani_umare_simple() {
    let sentence = "彼は生まれながらに美しい";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "nagarani_umare"),
        "Expected nagarani_umare pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_nagarani_split_namida() {
    let sentence = "涙ながらに語った";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "nagarani_split"),
        "Expected nagarani_split pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_nagarani_shite_kodomo() {
    let sentence = "子供ながらにして理解していた";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "nagarani_shite"),
        "Expected nagarani_shite pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_nagarani_shite_verb() {
    let sentence = "居ながらにして仕事ができる";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "nagarani_shite"),
        "Expected nagarani_shite pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ha_oroka() {
    let sentence = "英語はおろか日本語も話せない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ha_oroka"),
        "Expected ha_oroka pattern not detected in '{}'",
        sentence
    );
}
