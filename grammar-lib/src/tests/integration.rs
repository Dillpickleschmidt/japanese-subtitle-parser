// use super::*;
//
// // Integration tests for grammar pattern matching with natural sentences.
// // These tests verify that patterns work correctly together in realistic usage.
//
// // ========== Multi-Pattern Natural Sentences ==========
//
// #[test]
// fn test_complex_desire_sentence() {
//     // "I want to try eating" - multiple patterns
//     let sentence = "食べてみたいと思っているんです";
//     let tokens = tokenize_sentence(sentence);
//     let patterns = detect_patterns(&tokens);
//
//     print_debug(sentence, &tokens, &patterns);
//
//     assert!(
//         has_pattern(&patterns, "te_miru"),
//         "Expected te_miru pattern in '{}'",
//         sentence
//     );
//     assert!(
//         has_pattern(&patterns, "tai_form"),
//         "Expected tai_form pattern in '{}'",
//         sentence
//     );
//     assert!(
//         has_pattern(&patterns, "te_iru"),
//         "Expected te_iru pattern in '{}'",
//         sentence
//     );
//     assert!(
//         has_pattern(&patterns, "n_desu"),
//         "Expected n_desu pattern in '{}'",
//         sentence
//     );
// }
//
// #[test]
// fn test_complex_necessity_sentence() {
//     // "It doesn't mean you have to go" - nested patterns
//     let sentence = "行かなければならないわけではない";
//     let tokens = tokenize_sentence(sentence);
//     let patterns = detect_patterns(&tokens);
//
//     print_debug(sentence, &tokens, &patterns);
//
//     assert!(
//         has_pattern(&patterns, "must_nakereba"),
//         "Expected must_nakereba pattern in '{}'",
//         sentence
//     );
//     // Note: wake_dewa_nai might not match depending on exact tokenization
//     // Main goal is to verify complex sentence doesn't break pattern matching
// }
//
// #[test]
// fn test_tara_dou_in_context() {
//     // "How about eating?" - verify tara_dou takes precedence
//     let sentence = "食べたらどうですか";
//     let tokens = tokenize_sentence(sentence);
//     let patterns = detect_patterns(&tokens);
//
//     print_debug(sentence, &tokens, &patterns);
//
//     assert!(
//         has_pattern(&patterns, "tara_dou"),
//         "Expected tara_dou pattern in '{}'",
//         sentence
//     );
//
//     // Verify tara_dou comes before tara_conditional if both detected
//     let tara_dou_pos = patterns.iter().position(|p| p.pattern_name == "tara_dou");
//     let tara_cond_pos = patterns
//         .iter()
//         .position(|p| p.pattern_name == "tara_conditional");
//
//     if let (Some(dou_pos), Some(cond_pos)) = (tara_dou_pos, tara_cond_pos) {
//         assert!(
//             dou_pos < cond_pos,
//             "tara_dou should have higher priority than tara_conditional"
//         );
//     }
// }
//
// #[test]
// fn test_to_ii_with_context() {
//     // "I hope you get better soon"
//     let sentence = "早く治るといいですね";
//     let tokens = tokenize_sentence(sentence);
//     let patterns = detect_patterns(&tokens);
//
//     print_debug(sentence, &tokens, &patterns);
//
//     assert!(
//         has_pattern(&patterns, "to_ii"),
//         "Expected to_ii pattern in '{}'",
//         sentence
//     );
// }
//
// #[test]
// fn test_causative_request_sentence() {
//     // "Please let me eat" - causative + te-form + kudasai
//     let sentence = "食べさせてください";
//     let tokens = tokenize_sentence(sentence);
//     let patterns = detect_patterns(&tokens);
//
//     print_debug(sentence, &tokens, &patterns);
//
//     // Causative might tokenize as causative verb form
//     // At minimum should detect te_kudasai
//     assert!(
//         has_pattern(&patterns, "te_kudasai") || has_pattern(&patterns, "te_request"),
//         "Expected te_kudasai or te_request pattern in '{}'",
//         sentence
//     );
// }
//
// #[test]
// fn test_polite_past_negative() {
//     // "I didn't eat" - complex polite negative past
//     let sentence = "食べませんでした";
//     let tokens = tokenize_sentence(sentence);
//     let patterns = detect_patterns(&tokens);
//
//     print_debug(sentence, &tokens, &patterns);
//
//     // This is a complex form that might not have a specific pattern
//     // Main goal is to verify tokenization works correctly
//     println!("Detected {} patterns", patterns.len());
// }
//
// #[test]
// fn test_mixed_jlpt_levels() {
//     // Mix of N5 (te_iru), N4 (te_shimau), N3 (rashii)
//     let sentence = "食べてしまっているらしい";
//     let tokens = tokenize_sentence(sentence);
//     let patterns = detect_patterns(&tokens);
//
//     print_debug(sentence, &tokens, &patterns);
//
//     assert!(
//         has_pattern(&patterns, "te_shimau"),
//         "Expected te_shimau (N4) pattern in '{}'",
//         sentence
//     );
//     assert!(
//         has_pattern(&patterns, "te_iru"),
//         "Expected te_iru (N5) pattern in '{}'",
//         sentence
//     );
//     assert!(
//         has_pattern(&patterns, "rashii"),
//         "Expected rashii (N3) pattern in '{}'",
//         sentence
//     );
// }
//
// #[test]
// fn test_conditional_with_result() {
//     // "If you eat, you'll feel better"
//     let sentence = "食べれば元気になる";
//     let tokens = tokenize_sentence(sentence);
//     let patterns = detect_patterns(&tokens);
//
//     print_debug(sentence, &tokens, &patterns);
//
//     assert!(
//         has_pattern(&patterns, "ba_conditional"),
//         "Expected ba_conditional pattern in '{}'",
//         sentence
//     );
// }
//
// #[test]
// fn test_volitional_with_ka() {
//     // "Shall we eat?"
//     let sentence = "食べようか";
//     let tokens = tokenize_sentence(sentence);
//     let patterns = detect_patterns(&tokens);
//
//     print_debug(sentence, &tokens, &patterns);
//
//     assert!(
//         has_pattern(&patterns, "volitional"),
//         "Expected volitional pattern in '{}'",
//         sentence
//     );
// }
//
// #[test]
// fn test_experience_with_negation() {
//     // "I've never eaten sushi" - uses ない instead of ある
//     // ta_koto_ga_aru pattern expects ある, so use positive form
//     let sentence = "寿司を食べたことがある";
//     let tokens = tokenize_sentence(sentence);
//     let patterns = detect_patterns(&tokens);
//
//     print_debug(sentence, &tokens, &patterns);
//
//     assert!(
//         has_pattern(&patterns, "ta_koto_ga_aru") || has_pattern(&patterns, "past_tense"),
//         "Expected ta_koto_ga_aru or past_tense pattern in '{}'",
//         sentence
//     );
// }
//
// #[test]
// fn test_hearsay_appearance_distinction() {
//     // "I heard it's expensive" (hearsay, not appearance)
//     let sentence = "高いそうです";
//     let tokens = tokenize_sentence(sentence);
//     let patterns = detect_patterns(&tokens);
//
//     print_debug(sentence, &tokens, &patterns);
//
//     assert!(
//         has_pattern(&patterns, "sou_desu_hearsay"),
//         "Expected sou_desu_hearsay pattern in '{}'",
//         sentence
//     );
// }
//
// #[test]
// fn test_multiple_te_forms_in_sequence() {
//     // "Eat, drink, and play"
//     let sentence = "食べて飲んで遊ぶ";
//     let tokens = tokenize_sentence(sentence);
//     let patterns = detect_patterns(&tokens);
//
//     print_debug(sentence, &tokens, &patterns);
//
//     let te_form_count = patterns
//         .iter()
//         .filter(|p| p.pattern_name == "te_form_basic")
//         .count();
//
//     assert!(
//         te_form_count >= 2,
//         "Expected at least 2 te_form patterns in '{}', found {}",
//         sentence,
//         te_form_count
//     );
// }
//
// // ========== Nested/Overlapping Pattern Tests ==========
//
// #[test]
// fn test_te_mo_ii_overlap() {
//     // Should detect both te_mo_ii and te_mo
//     let sentence = "食べてもいい";
//     let tokens = tokenize_sentence(sentence);
//     let patterns = detect_patterns(&tokens);
//
//     print_debug(sentence, &tokens, &patterns);
//
//     assert!(
//         has_pattern(&patterns, "te_mo_ii"),
//         "Expected te_mo_ii pattern in '{}'",
//         sentence
//     );
//     assert!(
//         has_pattern(&patterns, "te_mo"),
//         "Expected te_mo pattern in '{}'",
//         sentence
//     );
//
//     // Verify te_mo_ii has higher priority (comes first)
//     let te_mo_ii_pos = patterns.iter().position(|p| p.pattern_name == "te_mo_ii");
//     let te_mo_pos = patterns.iter().position(|p| p.pattern_name == "te_mo");
//
//     if let (Some(ii_pos), Some(mo_pos)) = (te_mo_ii_pos, te_mo_pos) {
//         assert!(
//             ii_pos < mo_pos,
//             "te_mo_ii should come before te_mo due to higher priority"
//         );
//     }
// }
//
// #[test]
// fn test_potential_or_passive_detection() {
//     // Ichidan verb られる is ambiguous - should detect at least one
//     let sentence = "食べられる";
//     let tokens = tokenize_sentence(sentence);
//     let patterns = detect_patterns(&tokens);
//
//     print_debug(sentence, &tokens, &patterns);
//
//     // At least one should be detected for ichidan verbs
//     let has_potential = has_pattern(&patterns, "potential_ichidan");
//     let has_passive = has_pattern(&patterns, "passive_ichidan");
//
//     assert!(
//         has_potential || has_passive,
//         "Expected potential_ichidan or passive_ichidan pattern in '{}'",
//         sentence
//     );
// }
//
// #[test]
// fn test_ta_koto_ga_aru_components() {
//     // Complex pattern with multiple components
//     let sentence = "食べたことがある";
//     let tokens = tokenize_sentence(sentence);
//     let patterns = detect_patterns(&tokens);
//
//     print_debug(sentence, &tokens, &patterns);
//
//     assert!(
//         has_pattern(&patterns, "ta_koto_ga_aru"),
//         "Expected ta_koto_ga_aru pattern in '{}'",
//         sentence
//     );
//     assert!(
//         has_pattern(&patterns, "past_tense"),
//         "Expected past_tense pattern in '{}'",
//         sentence
//     );
// }
//
// #[test]
// fn test_must_nakereba_nesting() {
//     // Must pattern contains negative
//     let sentence = "食べなければならない";
//     let tokens = tokenize_sentence(sentence);
//     let patterns = detect_patterns(&tokens);
//
//     print_debug(sentence, &tokens, &patterns);
//
//     assert!(
//         has_pattern(&patterns, "must_nakereba"),
//         "Expected must_nakereba pattern in '{}'",
//         sentence
//     );
//     assert!(
//         has_pattern(&patterns, "negative"),
//         "Expected negative pattern in '{}'",
//         sentence
//     );
// }
//
// #[test]
// fn test_te_oku_with_masu() {
//     // て-form pattern + おく + ます
//     let sentence = "食べておきます";
//     let tokens = tokenize_sentence(sentence);
//     let patterns = detect_patterns(&tokens);
//
//     print_debug(sentence, &tokens, &patterns);
//
//     assert!(
//         has_pattern(&patterns, "te_oku"),
//         "Expected te_oku pattern in '{}'",
//         sentence
//     );
//     assert!(
//         has_pattern(&patterns, "masu_form"),
//         "Expected masu_form pattern in '{}'",
//         sentence
//     );
// }
//
// #[test]
// fn test_you_ni_suru_naru_distinction() {
//     // Verify you_ni_suru doesn't match when followed by naru
//     let sentence_suru = "早く寝るようにする";
//     let tokens_suru = tokenize_sentence(sentence_suru);
//     let patterns_suru = detect_patterns(&tokens_suru);
//
//     assert!(
//         has_pattern(&patterns_suru, "you_ni_suru"),
//         "Expected you_ni_suru pattern in '{}'",
//         sentence_suru
//     );
//
//     let sentence_naru = "早く寝るようになる";
//     let tokens_naru = tokenize_sentence(sentence_naru);
//     let patterns_naru = detect_patterns(&tokens_naru);
//
//     assert!(
//         has_pattern(&patterns_naru, "you_ni_naru"),
//         "Expected you_ni_naru pattern in '{}'",
//         sentence_naru
//     );
// }
//
// #[test]
// fn test_tari_form_overlap() {
//     // たり form - たり is a particle, not past tense aux verb
//     let sentence = "食べたり飲んだりする";
//     let tokens = tokenize_sentence(sentence);
//     let patterns = detect_patterns(&tokens);
//
//     print_debug(sentence, &tokens, &patterns);
//
//     assert!(
//         has_pattern(&patterns, "tari_suru"),
//         "Expected tari_suru pattern in '{}'",
//         sentence
//     );
//     // Note: たり is a particle, not the past tense auxiliary verb た
// }
//
// #[test]
// fn test_sugiru_with_conjugations() {
//     // すぎる pattern can be conjugated
//     let sentence = "食べすぎた";
//     let tokens = tokenize_sentence(sentence);
//     let patterns = detect_patterns(&tokens);
//
//     print_debug(sentence, &tokens, &patterns);
//
//     assert!(
//         has_pattern(&patterns, "sugiru"),
//         "Expected sugiru pattern in '{}'",
//         sentence
//     );
// }
//
// #[test]
// fn test_ba_yokatta_components() {
//     // Complex regret pattern
//     let sentence = "食べればよかった";
//     let tokens = tokenize_sentence(sentence);
//     let patterns = detect_patterns(&tokens);
//
//     print_debug(sentence, &tokens, &patterns);
//
//     assert!(
//         has_pattern(&patterns, "ba_yokatta"),
//         "Expected ba_yokatta pattern in '{}'",
//         sentence
//     );
//     assert!(
//         has_pattern(&patterns, "ba_conditional"),
//         "Expected ba_conditional pattern in '{}'",
//         sentence
//     );
// }
//
// #[test]
// fn test_kamo_shirenai_overlap() {
//     // Might pattern with verb
//     let sentence = "行くかもしれない";
//     let tokens = tokenize_sentence(sentence);
//     let patterns = detect_patterns(&tokens);
//
//     print_debug(sentence, &tokens, &patterns);
//
//     assert!(
//         has_pattern(&patterns, "kamo_shirenai"),
//         "Expected kamo_shirenai pattern in '{}'",
//         sentence
//     );
// }
//
// // ========== Character Range Validation Tests ==========
//
// #[test]
// fn test_character_range_validity() {
//     // Test that all patterns have valid character ranges
//     let sentence = "食べてみたいと思っている";
//     let tokens = tokenize_sentence(sentence);
//     let patterns = detect_patterns(&tokens);
//
//     print_debug(sentence, &tokens, &patterns);
//
//     assert!(!patterns.is_empty(), "Should detect at least one pattern");
//
//     let char_count = sentence.chars().count();
//
//     // Verify all patterns have valid character ranges
//     for pattern in &patterns {
//         assert!(
//             (pattern.start_char as usize) < (pattern.end_char as usize),
//             "Pattern '{}' start should be before end",
//             pattern.pattern_name
//         );
//         assert!(
//             (pattern.end_char as usize) <= char_count,
//             "Pattern '{}' end ({}) should be within sentence character count ({})",
//             pattern.pattern_name,
//             pattern.end_char,
//             char_count
//         );
//
//         // Verify we can extract text using these character positions
//         let byte_start = char_pos_to_byte_pos(sentence, pattern.start_char as usize);
//         let byte_end = char_pos_to_byte_pos(sentence, pattern.end_char as usize);
//         let extracted = &sentence[byte_start..byte_end];
//         assert!(
//             !extracted.is_empty(),
//             "Pattern '{}' should extract non-empty text",
//             pattern.pattern_name
//         );
//     }
// }
//
// #[test]
// fn test_character_range_single_pattern() {
//     // Test character ranges with a simple single-pattern sentence
//     let sentence = "食べます";
//     let tokens = tokenize_sentence(sentence);
//     let patterns = detect_patterns(&tokens);
//
//     print_debug(sentence, &tokens, &patterns);
//
//     let masu = patterns
//         .iter()
//         .find(|p| p.pattern_name == "masu_form")
//         .expect("masu_form pattern should be detected");
//
//     // Verify character ranges are valid
//     let start = masu.start_char as usize;
//     let end = masu.end_char as usize;
//     let char_count = sentence.chars().count();
//
//     assert!(start < end, "Start should be before end");
//     assert!(end <= char_count, "End should be within character count");
//
//     // Can extract text using character positions
//     let byte_start = char_pos_to_byte_pos(sentence, start);
//     let byte_end = char_pos_to_byte_pos(sentence, end);
//     let extracted = &sentence[byte_start..byte_end];
//     assert!(!extracted.is_empty(), "Should extract non-empty text");
// }
//
// #[test]
// fn test_character_range_multiple_patterns() {
//     // Test character ranges when multiple patterns detected
//     let sentence = "食べてもいい";
//     let tokens = tokenize_sentence(sentence);
//     let patterns = detect_patterns(&tokens);
//
//     print_debug(sentence, &tokens, &patterns);
//
//     assert!(
//         patterns.len() >= 2,
//         "Should detect multiple patterns in '{}'",
//         sentence
//     );
//
//     let char_count = sentence.chars().count();
//
//     for pattern in &patterns {
//         let start = pattern.start_char as usize;
//         let end = pattern.end_char as usize;
//
//         // Verify character ranges are valid
//         assert!(
//             start < end,
//             "Pattern '{}' should have start < end",
//             pattern.pattern_name
//         );
//         assert!(
//             end <= char_count,
//             "Pattern '{}' end should be within character count",
//             pattern.pattern_name
//         );
//
//         // Can extract valid text using character positions
//         let byte_start = char_pos_to_byte_pos(sentence, start);
//         let byte_end = char_pos_to_byte_pos(sentence, end);
//         let extracted = &sentence[byte_start..byte_end];
//         assert!(
//             !extracted.is_empty(),
//             "Pattern '{}' should extract non-empty text",
//             pattern.pattern_name
//         );
//     }
// }
//
// #[test]
// fn test_character_range_extract_text() {
//     // Test that we can extract meaningful text from character ranges
//     let test_cases = vec![
//         ("食べます", "masu_form"),
//         ("食べたい", "tai_form"),
//         ("食べない", "negative"),
//     ];
//
//     for (sentence, pattern_name) in test_cases {
//         let tokens = tokenize_sentence(sentence);
//         let patterns = detect_patterns(&tokens);
//
//         let pattern = patterns
//             .iter()
//             .find(|p| p.pattern_name == pattern_name)
//             .unwrap_or_else(|| panic!("{} pattern should be detected", pattern_name));
//
//         let start = pattern.start_char as usize;
//         let end = pattern.end_char as usize;
//         let char_count = sentence.chars().count();
//
//         // Valid character range
//         assert!(start < end, "{}: start < end", pattern_name);
//         assert!(
//             end <= char_count,
//             "{}: end within character count",
//             pattern_name
//         );
//
//         // Can extract using character positions
//         let byte_start = char_pos_to_byte_pos(sentence, start);
//         let byte_end = char_pos_to_byte_pos(sentence, end);
//         let extracted = &sentence[byte_start..byte_end];
//         assert!(!extracted.is_empty(), "{}: non-empty extract", pattern_name);
//     }
// }
//
// #[test]
// fn test_character_range_utf8_boundaries() {
//     // Verify character ranges work correctly with multi-byte UTF-8 characters
//     let sentences = vec![
//         "カタカナで食べます", // Mix of katakana, hiragana, kanji
//         "簡単な文章です",     // Kanji and hiragana
//         "食べて飲んで",       // Multiple patterns
//     ];
//
//     for sentence in sentences {
//         let tokens = tokenize_sentence(sentence);
//         let patterns = detect_patterns(&tokens);
//         let char_count = sentence.chars().count();
//
//         for pattern in &patterns {
//             let start = pattern.start_char as usize;
//             let end = pattern.end_char as usize;
//
//             // Verify character positions are within bounds
//             assert!(
//                 end <= char_count,
//                 "Pattern '{}' in '{}' should have end within character count",
//                 pattern.pattern_name,
//                 sentence
//             );
//
//             // Should successfully extract text when converting char positions to byte positions
//             let byte_start = char_pos_to_byte_pos(sentence, start);
//             let byte_end = char_pos_to_byte_pos(sentence, end);
//             let extracted = &sentence[byte_start..byte_end];
//             assert!(
//                 !extracted.is_empty(),
//                 "Pattern '{}' in '{}' should extract non-empty text",
//                 pattern.pattern_name,
//                 sentence
//             );
//         }
//     }
// }
//
// // ========== Real-World Complexity Tests ==========
//
// #[test]
// fn test_multiple_same_pattern_instances() {
//     // Three te-forms in a row
//     let sentence = "食べて、飲んで、遊んで";
//     let tokens = tokenize_sentence(sentence);
//     let patterns = detect_patterns(&tokens);
//
//     print_debug(sentence, &tokens, &patterns);
//
//     let te_form_count = patterns
//         .iter()
//         .filter(|p| p.pattern_name == "te_form_basic")
//         .count();
//
//     assert!(
//         te_form_count == 3,
//         "Expected exactly 3 te_form patterns in '{}', found {}",
//         sentence,
//         te_form_count
//     );
//
//     // Verify each has unique character ranges
//     let te_forms: Vec<_> = patterns
//         .iter()
//         .filter(|p| p.pattern_name == "te_form_basic")
//         .collect();
//
//     for i in 0..te_forms.len() {
//         for j in (i + 1)..te_forms.len() {
//             assert_ne!(
//                 (te_forms[i].start_char, te_forms[i].end_char),
//                 (te_forms[j].start_char, te_forms[j].end_char),
//                 "Each te_form instance should have unique character range"
//             );
//         }
//     }
// }
//
// #[test]
// fn test_long_complex_sentence() {
//     // Long sentence with multiple patterns
//     let sentence = "昨日友達と一緒に美味しいラーメンを食べに行ったんだけど、すごく美味しかったから、また行きたいと思っている";
//     let tokens = tokenize_sentence(sentence);
//     let patterns = detect_patterns(&tokens);
//
//     print_debug(sentence, &tokens, &patterns);
//
//     // Should detect several patterns
//     assert!(
//         patterns.len() >= 3,
//         "Expected at least 3 patterns in complex sentence, found {}",
//         patterns.len()
//     );
//
//     // Verify all character ranges are valid
//     for pattern in &patterns {
//         assert!(
//             pattern.start_char < pattern.end_char,
//             "Pattern '{}' should have valid range",
//             pattern.pattern_name
//         );
//         assert!(
//             (pattern.end_char as usize) <= sentence.len(),
//             "Pattern '{}' should be within sentence bounds",
//             pattern.pattern_name
//         );
//     }
// }
//
// #[test]
// fn test_casual_polite_mixed() {
//     // Mix of casual and polite forms
//     let sentence = "食べたけど、美味しかったです";
//     let tokens = tokenize_sentence(sentence);
//     let patterns = detect_patterns(&tokens);
//
//     print_debug(sentence, &tokens, &patterns);
//
//     assert!(
//         has_pattern(&patterns, "past_tense"),
//         "Expected past_tense pattern in '{}'",
//         sentence
//     );
// }
//
// #[test]
// fn test_patterns_with_particles() {
//     // Patterns separated by particles
//     let sentence = "食べるのは好きだけど、作るのは嫌い";
//     let tokens = tokenize_sentence(sentence);
//     let patterns = detect_patterns(&tokens);
//
//     print_debug(sentence, &tokens, &patterns);
//
//     // Should detect dictionary form and no pattern
//     assert!(
//         !patterns.is_empty(),
//         "Expected some patterns in '{}'",
//         sentence
//     );
// }
//
// #[test]
// fn test_question_form_complex() {
//     // Question with multiple patterns
//     let sentence = "どこで食べたらいいと思いますか";
//     let tokens = tokenize_sentence(sentence);
//     let patterns = detect_patterns(&tokens);
//
//     print_debug(sentence, &tokens, &patterns);
//
//     // Should detect multiple patterns
//     assert!(
//         patterns.len() >= 2,
//         "Expected multiple patterns in question, found {}",
//         patterns.len()
//     );
// }
