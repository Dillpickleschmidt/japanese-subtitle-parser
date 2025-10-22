use super::*;

// N4 conjugation patterns
#[test]
fn test_imperative_detection() {
    let sentence = "早く食べろ";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "imperative");
    assert_pattern_range(&patterns, "imperative", 2, 5); // 食べろ
    assert_pattern_selected(&patterns, &tokens, "imperative");
}

#[test]
fn test_nagara_detection() {
    let sentence = "テレビを見ながら夕食を食べる";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "nagara");
    assert_pattern_range(&patterns, "nagara", 4, 8); // 見ながら
    assert_pattern_selected(&patterns, &tokens, "nagara");
}

// N4 te-form patterns
#[test]
fn test_te_miru_detection() {
    let sentence = "新しい料理を食べてみる";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "te_miru");
    assert_pattern_range(&patterns, "te_miru", 6, 11); // 食べてみる
    assert_pattern_selected(&patterns, &tokens, "te_miru");
}

#[test]
fn test_te_shimau_detection() {
    let sentence = "子どもが全部食べてしまった";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    assert_has_pattern(&patterns, "te_shimau");
    assert_pattern_range(&patterns, "te_shimau", 6, 13); // 食べてしまった
    assert_pattern_selected(&patterns, &tokens, "te_shimau");
}

// Tari form patterns (lists non-sequential actions: V-tari V-tari ... suru)
mod tari_tests {
    use super::*;

    #[test]
    fn test_tari_form_single_past() {
        // Single tari with implied other activities (past tense)
        let sentence = "昔あそこの池で泳いだりした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        print_debug(sentence, &tokens, &patterns);

        assert_has_pattern(&patterns, "tari_suru_single");
        assert_pattern_range(&patterns, "tari_suru_single", 7, 13); // 泳いだりした (泳い starts at char 7)
        assert_pattern_selected(&patterns, &tokens, "tari_suru_single");
    }

    #[test]
    fn test_tari_form_double_past() {
        // Multiple tari construction (standard usage, past tense)
        let sentence = "飲み会で食べたり飲んだりした";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        print_debug(sentence, &tokens, &patterns);

        assert_has_pattern(&patterns, "tari_suru");
        assert_pattern_range(&patterns, "tari_suru", 4, 14); // 食べたり飲んだりした
        assert_pattern_selected(&patterns, &tokens, "tari_suru");
    }

    #[test]
    fn test_tari_form_triple() {
        // Three different actions with tari (showing variety)
        let sentence = "休みの日は家でテレビを見たり寝たり本を読んだりする";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        print_debug(sentence, &tokens, &patterns);

        assert_has_pattern(&patterns, "tari_suru");
        assert_pattern_range(&patterns, "tari_suru", 11, 25); // 見たり寝たり本を読んだりする
        assert_pattern_selected(&patterns, &tokens, "tari_suru");
    }

    #[test]
    fn test_tari_form_with_suru_verb() {
        // Suru verb as one of the tari items (勉強したり)
        let sentence = "勉強したり運動したりする";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        print_debug(sentence, &tokens, &patterns);

        assert_has_pattern(&patterns, "tari_suru");
        assert_pattern_range(&patterns, "tari_suru", 0, 12); // 勉強したり運動したりする
        assert_pattern_selected(&patterns, &tokens, "tari_suru");
    }

    #[test]
    fn test_tari_dari_variation() {
        // Using だり instead of たり (phonetic variant with certain verbs)
        let sentence = "本を読んだり映画を見たりする";
        let tokens = tokenize_sentence(sentence);
        let patterns = detect_patterns(&tokens);

        print_debug(sentence, &tokens, &patterns);

        assert_has_pattern(&patterns, "tari_suru");
        assert_pattern_range(&patterns, "tari_suru", 2, 14); // 読んだり映画を見たりする
        assert_pattern_selected(&patterns, &tokens, "tari_suru");
    }
}

// N4 conditional patterns
#[test]
fn test_ba_conditional_detection() {
    let sentence = "食べれば";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ba_conditional"),
        "Expected ba_conditional pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_tara_conditional_detection() {
    let sentence = "食べたら";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "tara_conditional"),
        "Expected tara_conditional pattern not detected in '{}'",
        sentence
    );
}

// N4 causative/passive patterns
#[test]
fn test_potential_detection() {
    let sentence = "食べられる";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "potential"),
        "Expected potential pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_passive_detection() {
    let sentence = "食べられる";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    // Note: られる can be both potential and passive
    // We expect at least one of them to be detected
    let has_potential_or_passive =
        has_pattern(&patterns, "potential") || has_pattern(&patterns, "passive");

    assert!(
        has_potential_or_passive,
        "Expected potential or passive pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_causative_detection() {
    let sentence = "食べさせる";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "causative"),
        "Expected causative pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_causative_passive_detection() {
    let sentence = "食べさせられる";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "causative_passive"),
        "Expected causative_passive pattern not detected in '{}'",
        sentence
    );
}

// N4 must patterns
#[test]
fn test_must_nakereba_detection() {
    let sentence = "食べなければならない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "must_nakereba"),
        "Expected must_nakereba pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_must_nakute_wa_detection() {
    let sentence = "食べなくてはいけない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "must_nakute_wa"),
        "Expected must_nakute_wa pattern not detected in '{}'",
        sentence
    );
}

// N4 te-form extensions
#[test]
fn test_te_aru_detection() {
    let sentence = "食べてある";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "te_aru"),
        "Expected te_aru pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_te_kureru_detection() {
    let sentence = "食べてくれる";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "te_kureru"),
        "Expected te_kureru pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_te_ageru_detection() {
    let sentence = "食べてあげる";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "te_ageru"),
        "Expected te_ageru pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_te_oku_detection() {
    let sentence = "食べておく";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "te_oku"),
        "Expected te_oku pattern not detected in '{}'",
        sentence
    );
}

// N4 auxiliary verbs
#[test]
fn test_yasui_detection() {
    let sentence = "食べやすい";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "yasui"),
        "Expected yasui pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_nikui_detection() {
    let sentence = "食べにくい";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "nikui"),
        "Expected nikui pattern not detected in '{}'",
        sentence
    );
}

// Additional N4 te-form patterns
#[test]
fn test_te_morau_detection() {
    let sentence = "食べてもらう";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "te_morau"),
        "Expected te_morau pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_te_sumimasen_detection() {
    let sentence = "食べてすみません";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "te_sumimasen"),
        "Expected te_sumimasen pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_te_kurete_arigatou_detection() {
    let sentence = "食べてくれてありがとう";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "te_kurete_arigatou"),
        "Expected te_kurete_arigatou pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_te_yokatta_detection() {
    let sentence = "食べてよかった";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "te_yokatta"),
        "Expected te_yokatta pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_te_mo_detection() {
    let sentence = "食べても";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "te_mo"),
        "Expected te_mo pattern not detected in '{}'",
        sentence
    );
}

// N4 nai-form patterns
#[test]
fn test_naide_detection() {
    let sentence = "食べないで";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "naide"),
        "Expected naide pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_naide_exception_verb_kureru() {
    let sentence = "くれないで"; // くれる is natural verb ending in れる, not potential
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "naide"),
        "Expected naide pattern not detected in '{}' (exception verb)",
        sentence
    );
}

#[test]
fn test_naide_potential_form_rejection() {
    let sentence = "帰れないで"; // 帰れる is potential form, should NOT match naide
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        !has_pattern(&patterns, "naide"),
        "naide should NOT match potential form '{}', but was detected",
        sentence
    );
}

#[test]
fn test_nakute_mo_ii_detection() {
    let sentence = "食べなくてもいい";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "nakute_mo_ii"),
        "Expected nakute_mo_ii pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ba_yokatta_detection() {
    let sentence = "食べればよかった";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ba_yokatta"),
        "Expected ba_yokatta pattern not detected in '{}'",
        sentence
    );
}

// N4 auxiliary/modal patterns
#[test]
fn test_nasai_detection() {
    let sentence = "食べなさい";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "nasai"),
        "Expected nasai pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_hazu_desu_detection() {
    let sentence = "食べるはずです";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "hazu_desu"),
        "Expected hazu_desu pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_tagaru_detection() {
    let sentence = "食べたがる";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "tagaru"),
        "Expected tagaru pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_te_itadakemasen_ka_detection() {
    let sentence = "食べていただく";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "te_itadakemasen_ka"),
        "Expected te_itadakemasen_ka pattern not detected in '{}'",
        sentence
    );
}

// N4 other common patterns
#[test]
fn test_tara_dou_detection() {
    let sentence = "食べたらどうですか";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "tara_dou"),
        "Expected tara_dou pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_to_ii_detection() {
    let sentence = "早く治るといいですね";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "to_ii"),
        "Expected to_ii pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ga_hoshii_detection() {
    let sentence = "食べ物がほしい";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ga_hoshii"),
        "Expected ga_hoshii pattern not detected in '{}'",
        sentence
    );
}

// Additional N4 patterns
#[test]
fn test_shika_nai_detection() {
    let sentence = "これしか食べない";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "shika_nai"),
        "Expected shika_nai pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_shika_nai_naru_rejection() {
    let sentence = "しかならない"; // なる doesn't make sense with しか, should NOT match
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        !has_pattern(&patterns, "shika_nai"),
        "shika_nai should NOT match '{}' (なる exclusion), but was detected",
        sentence
    );
}

#[test]
fn test_to_iu_detection() {
    let sentence = "太郎という人";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "to_iu"),
        "Expected to_iu pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_dictionary_to_detection() {
    let sentence = "食べると";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "dictionary_to"),
        "Expected dictionary_to pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_nara_detection() {
    let sentence = "食べるなら";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "nara"),
        "Expected nara pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_shi_detection() {
    let sentence = "美味しいし";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "shi"),
        "Expected shi pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_ka_dou_ka_detection() {
    let sentence = "食べるかどうか";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "ka_dou_ka"),
        "Expected ka_dou_ka pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_koto_ni_suru_detection() {
    let sentence = "食べることにする";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "koto_ni_suru"),
        "Expected koto_ni_suru pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_noni_detection() {
    let sentence = "食べるのに";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "noni"),
        "Expected noni pattern not detected in '{}'",
        sentence
    );
}

#[test]
fn test_koto_ni_naru_detection() {
    let sentence = "行くことになる";
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "koto_ni_naru"),
        "Expected koto_ni_naru pattern not detected in '{}'",
        sentence
    );
}

// Edge case tests for different verb types

#[test]
fn test_tari_form_godan() {
    let sentence = "飲んだりする"; // Godan verb: 飲む -> 飲んだり (uses 連用タ接続)
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "tari_suru_single"),
        "Expected tari_suru_single pattern not detected in '{}' (godan verb with 連用タ接続)",
        sentence
    );
}

#[test]
fn test_tara_dou_precedence() {
    let sentence = "飲んだらどうですか"; // Should match tara_dou (priority 12), not just tara_conditional (priority 7)
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    // Both should match, but tara_dou should be first due to higher priority
    assert!(
        has_pattern(&patterns, "tara_dou"),
        "Expected tara_dou pattern not detected in '{}'",
        sentence
    );

    // Verify tara_dou comes before tara_conditional in the results
    let tara_dou_pos = patterns.iter().position(|p| p.pattern_name == "tara_dou");
    let tara_cond_pos = patterns
        .iter()
        .position(|p| p.pattern_name == "tara_conditional");

    if let (Some(dou_pos), Some(cond_pos)) = (tara_dou_pos, tara_cond_pos) {
        assert!(
            dou_pos < cond_pos,
            "tara_dou (priority 12) should come before tara_conditional (priority 7) in results, but got positions {} and {}",
            dou_pos, cond_pos
        );
    }
}

#[test]
fn test_te_mo_without_ii() {
    let sentence = "飲んでも"; // Should match te_mo only, not te_mo_ii
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "te_mo"),
        "Expected te_mo pattern not detected in '{}'",
        sentence
    );

    // Should NOT match te_mo_ii since there's no いい
    assert!(
        !has_pattern(&patterns, "te_mo_ii"),
        "te_mo_ii should not match '{}' (missing いい)",
        sentence
    );
}

#[test]
fn test_te_mo_ii_precedence() {
    let sentence = "飲んでもいい"; // Should match both te_mo and te_mo_ii, but te_mo_ii should be first
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    // Both should match
    assert!(
        has_pattern(&patterns, "te_mo_ii"),
        "Expected te_mo_ii pattern not detected in '{}'",
        sentence
    );
    assert!(
        has_pattern(&patterns, "te_mo"),
        "Expected te_mo pattern not detected in '{}'",
        sentence
    );

    // Verify te_mo_ii comes before te_mo due to higher priority (11 vs 8)
    let te_mo_ii_pos = patterns.iter().position(|p| p.pattern_name == "te_mo_ii");
    let te_mo_pos = patterns.iter().position(|p| p.pattern_name == "te_mo");

    if let (Some(ii_pos), Some(mo_pos)) = (te_mo_ii_pos, te_mo_pos) {
        assert!(
            ii_pos < mo_pos,
            "te_mo_ii (priority 11) should come before te_mo (priority 8) in results, but got positions {} and {}",
            ii_pos, mo_pos
        );
    }
}

#[test]
fn test_te_miru_de_variation() {
    let sentence = "飲んでみる"; // Godan verb with んで
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "te_miru"),
        "Expected te_miru pattern not detected in '{}' (de-form variation)",
        sentence
    );
}

#[test]
fn test_te_oku_de_variation() {
    let sentence = "飲んでおく"; // Godan verb with んで
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "te_oku"),
        "Expected te_oku pattern not detected in '{}' (de-form variation)",
        sentence
    );
}

#[test]
fn test_te_yokatta_de_variation() {
    let sentence = "飲んでよかった"; // Godan verb with んで
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "te_yokatta"),
        "Expected te_yokatta pattern not detected in '{}' (de-form variation)",
        sentence
    );
}

#[test]
fn test_potential_passive_ambiguity() {
    let sentence = "食べられる"; // Ichidan: can be both potential AND passive
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    // Both patterns should be detected since られる is ambiguous for ichidan verbs
    let has_potential = has_pattern(&patterns, "potential");
    let has_passive = has_pattern(&patterns, "passive");

    assert!(
        has_potential && has_passive,
        "Expected BOTH potential and passive patterns to be detected in '{}' (ambiguous ichidan), but got potential={}, passive={}",
        sentence, has_potential, has_passive
    );
}

// Sou desu patterns (appearance and hearsay)
#[test]
fn test_sou_desu_appearance_verb() {
    let sentence = "食べそうです"; // Looks like they will eat (appearance)
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "sou_desu_appearance"),
        "Expected sou_desu_appearance pattern not detected in '{}' (verb)",
        sentence
    );
}

#[test]
fn test_sou_desu_appearance_i_adjective() {
    let sentence = "高そうです"; // Looks expensive (appearance, i-adjective)
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "sou_desu_appearance"),
        "Expected sou_desu_appearance pattern not detected in '{}' (i-adjective)",
        sentence
    );
}

#[test]
fn test_sou_desu_appearance_na_adjective() {
    let sentence = "静かそうです"; // Looks quiet (appearance, na-adjective)
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "sou_desu_appearance"),
        "Expected sou_desu_appearance pattern not detected in '{}' (na-adjective)",
        sentence
    );
}

#[test]
fn test_sou_desu_hearsay_verb() {
    let sentence = "食べるそうです"; // I heard they will eat (hearsay)
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "sou_desu_hearsay"),
        "Expected sou_desu_hearsay pattern not detected in '{}' (verb)",
        sentence
    );
}

#[test]
fn test_sou_desu_hearsay_i_adjective() {
    let sentence = "高いそうです"; // I heard it's expensive (hearsay, i-adjective)
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "sou_desu_hearsay"),
        "Expected sou_desu_hearsay pattern not detected in '{}' (i-adjective)",
        sentence
    );
}

#[test]
fn test_sou_desu_hearsay_na_adjective() {
    let sentence = "静かだそうです"; // I heard it's quiet (hearsay, na-adjective)
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    // Na-adjectives use a special pattern with だ between stem and そう
    assert!(
        has_pattern(&patterns, "sou_desu_hearsay_na"),
        "Expected sou_desu_hearsay_na pattern not detected in '{}' (na-adjective)",
        sentence
    );
}

// Kamo shirenai patterns (might/maybe)
#[test]
fn test_kamo_shirenai_verb() {
    let sentence = "食べるかもしれない"; // Might eat
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "kamo_shirenai"),
        "Expected kamo_shirenai pattern not detected in '{}' (verb)",
        sentence
    );
}

#[test]
fn test_kamo_shiremasen_verb() {
    let sentence = "食べるかもしれません"; // Might eat (polite)
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "kamo_shiremasen"),
        "Expected kamo_shiremasen pattern not detected in '{}' (verb, polite)",
        sentence
    );
}

// Kamo shirenai with adjectives/nouns
#[test]
fn test_kamo_shirenai_i_adjective() {
    let sentence = "高いかもしれない"; // Might be expensive (i-adjective)
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "kamo_shirenai_adj_noun"),
        "Expected kamo_shirenai_adj_noun pattern not detected in '{}' (i-adjective)",
        sentence
    );
}

#[test]
fn test_kamo_shirenai_na_adjective() {
    let sentence = "静かかもしれない"; // Might be quiet (na-adjective)
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "kamo_shirenai_adj_noun"),
        "Expected kamo_shirenai_adj_noun pattern not detected in '{}' (na-adjective)",
        sentence
    );
}

#[test]
fn test_kamo_shirenai_noun() {
    let sentence = "雨かもしれない"; // Might be rain (noun)
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "kamo_shirenai_adj_noun"),
        "Expected kamo_shirenai_adj_noun pattern not detected in '{}' (noun)",
        sentence
    );
}

// Mitai pattern (looks like/seems)
#[test]
fn test_mitai_verb() {
    let sentence = "食べるみたい"; // Looks like they'll eat
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "mitai"),
        "Expected mitai pattern not detected in '{}' (verb)",
        sentence
    );
}

#[test]
fn test_mitai_i_adjective() {
    let sentence = "高いみたい"; // Looks expensive (i-adjective)
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "mitai_adj_noun"),
        "Expected mitai_adj_noun pattern not detected in '{}' (i-adjective)",
        sentence
    );
}

#[test]
fn test_mitai_na_adjective() {
    let sentence = "静かみたい"; // Looks quiet (na-adjective)
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "mitai_adj_noun"),
        "Expected mitai_adj_noun pattern not detected in '{}' (na-adjective)",
        sentence
    );
}

#[test]
fn test_mitai_noun() {
    let sentence = "雨みたい"; // Looks like rain (noun)
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    assert!(
        has_pattern(&patterns, "mitai_adj_noun"),
        "Expected mitai_adj_noun pattern not detected in '{}' (noun)",
        sentence
    );
}

// Edge case tests: Verify sou desu patterns DON'T match agreement/confirmation uses
#[test]
fn test_sou_desu_agreement_rejection() {
    let sentence = "そうです"; // "That's right" (agreement, NOT grammar pattern)
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    // Should NOT match any sou_desu patterns - this is standalone agreement
    assert!(
        !has_pattern(&patterns, "sou_desu_appearance")
            && !has_pattern(&patterns, "sou_desu_hearsay")
            && !has_pattern(&patterns, "sou_desu_hearsay_na"),
        "sou_desu patterns should NOT match standalone agreement '{}', but detected: {:?}",
        sentence,
        patterns.iter().map(|p| p.pattern_name).collect::<Vec<_>>()
    );
}

#[test]
fn test_sou_desu_hai_rejection() {
    let sentence = "はい、そうです"; // "Yes, that's right" (agreement, NOT grammar pattern)
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    // Should NOT match any sou_desu patterns
    assert!(
        !has_pattern(&patterns, "sou_desu_appearance")
            && !has_pattern(&patterns, "sou_desu_hearsay")
            && !has_pattern(&patterns, "sou_desu_hearsay_na"),
        "sou_desu patterns should NOT match agreement '{}', but detected: {:?}",
        sentence,
        patterns.iter().map(|p| p.pattern_name).collect::<Vec<_>>()
    );
}

#[test]
fn test_sou_desu_ne_rejection() {
    let sentence = "そうですね"; // "That's right, isn't it" (agreement with ね)
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    // Should NOT match any sou_desu patterns - ends with ね, not です
    assert!(
        !has_pattern(&patterns, "sou_desu_appearance")
            && !has_pattern(&patterns, "sou_desu_hearsay")
            && !has_pattern(&patterns, "sou_desu_hearsay_na"),
        "sou_desu patterns should NOT match '{}' (ends with ね), but detected: {:?}",
        sentence,
        patterns.iter().map(|p| p.pattern_name).collect::<Vec<_>>()
    );
}

#[test]
fn test_sou_desu_ka_rejection() {
    let sentence = "そうですか"; // "Is that so?" (question)
    let tokens = tokenize_sentence(sentence);
    let patterns = detect_patterns(&tokens);

    print_debug(sentence, &tokens, &patterns);

    // Should NOT match any sou_desu patterns - ends with か, not です
    assert!(
        !has_pattern(&patterns, "sou_desu_appearance")
            && !has_pattern(&patterns, "sou_desu_hearsay")
            && !has_pattern(&patterns, "sou_desu_hearsay_na"),
        "sou_desu patterns should NOT match '{}' (question form), but detected: {:?}",
        sentence,
        patterns.iter().map(|p| p.pattern_name).collect::<Vec<_>>()
    );
}
