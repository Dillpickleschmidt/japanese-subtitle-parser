use crate::types::{format_priority, SelectionMap, SourceType, SubtitleCandidate};
use std::collections::HashMap;

pub fn select_best_subtitles(candidates: Vec<SubtitleCandidate>) -> SelectionMap {
    let mut best_per_episode = HashMap::new();


    // Group candidates by episode
    let mut episode_candidates: HashMap<i32, Vec<SubtitleCandidate>> = HashMap::new();
    for candidate in candidates {
        for &episode in &candidate.episode_numbers {
            episode_candidates
                .entry(episode)
                .or_insert_with(Vec::new)
                .push(candidate.clone());
        }
    }

    for (episode, mut candidates) in episode_candidates {
        if candidates.is_empty() {
            continue;
        }

        // Hard rule: BD always wins
        if candidates.iter().any(|c| c.source_type == SourceType::BD) {
            candidates.retain(|c| c.source_type == SourceType::BD);
        }

        // Hard rule: Within the same source type, drop .ass if .srt exists
        // But don't drop higher-quality sources (like ZIP files) for lower-quality .srt
        let best_source_type = candidates.iter().map(|c| c.source_type.clone()).max().unwrap();
        let has_srt_in_best = candidates.iter().any(|c| c.source_type == best_source_type && c.format == "srt");
        
        if has_srt_in_best {
            // Only filter within the best source type
            candidates.retain(|c| c.source_type != best_source_type || c.format == "srt");
        }

        // Sort by all priority rules to find the best candidate
        sort_candidates(&mut candidates);

        if let Some(best) = candidates.first() {
            best_per_episode.insert(episode, best.clone());
        }
    }

    best_per_episode
}

fn sort_candidates(candidates: &mut Vec<SubtitleCandidate>) {
    candidates.sort_by(|a, b| {
        // Primary: Source type priority (BD > Retime > Other > DVD > Stream)
        b.source_type
            .cmp(&a.source_type)
            // Format priority (srt > unknown > ass)
            .then(format_priority(&b.format).cmp(&format_priority(&a.format)))
            // Individual files preferred over ZIP (when formats are equal)
            .then(a.is_zip.cmp(&b.is_zip))
            // Non-CC preferred over CC
            .then(a.is_cc.cmp(&b.is_cc))
            // Larger file size preferred
            .then(b.file_info.size.cmp(&a.file_info.size))
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{JimakuFile, SourceType, SubtitleCandidate};
    use std::rc::Rc;

    fn create_test_candidate(
        name: &str,
        episode: i32,
        source_type: SourceType,
        format: &str,
        is_cc: bool,
        is_zip: bool,
        size: u64,
    ) -> SubtitleCandidate {
        SubtitleCandidate {
            file_info: Rc::new(JimakuFile {
                name: name.to_string(),
                url: format!("https://example.com/{}", name),
                size,
                last_modified: "2024-01-01T00:00:00Z".to_string(),
            }),
            episode_numbers: vec![episode],
            source_type,
            is_cc,
            format: format.to_string(),
            is_zip,
            source_zip_url: if is_zip {
                Some("https://example.com/test.zip".to_string())
            } else {
                None
            },
        }
    }

    #[test]
    fn test_selection_bd_priority() {
        let candidates = vec![
            create_test_candidate("Show S01E01 [Hi10].srt", 1, SourceType::OtherWeb, "srt", false, false, 1000),
            create_test_candidate("Show S01E01 [BD].srt", 1, SourceType::BD, "srt", false, false, 500),
        ];

        let selections = select_best_subtitles(candidates);
        assert_eq!(selections.get(&1).unwrap().source_type, SourceType::BD);
    }

    #[test]
    fn test_selection_srt_over_ass() {
        let candidates = vec![
            create_test_candidate("Show S01E01 [Hi10].ass", 1, SourceType::OtherWeb, "ass", false, false, 2000),
            create_test_candidate("Show S01E01 [Hi10].srt", 1, SourceType::OtherWeb, "srt", false, false, 1000),
        ];

        let selections = select_best_subtitles(candidates);
        assert_eq!(selections.get(&1).unwrap().format, "srt");
    }

    #[test]
    fn test_selection_individual_over_zip_same_format() {
        let candidates = vec![
            create_test_candidate("Show S01E01 [BD].srt", 1, SourceType::BD, "srt", false, false, 1000),
            create_test_candidate("Show.zip", 1, SourceType::BD, "srt", false, true, 1000),
        ];

        let selections = select_best_subtitles(candidates);
        assert!(!selections.get(&1).unwrap().is_zip);
    }

    #[test]
    fn test_selection_zip_unknown_over_individual_ass() {
        let candidates = vec![
            create_test_candidate("Show S01E01 [BD].ass", 1, SourceType::BD, "ass", false, false, 1000),
            create_test_candidate("Show.zip", 1, SourceType::BD, "unknown", false, true, 1000),
        ];

        let selections = select_best_subtitles(candidates);
        assert_eq!(selections.get(&1).unwrap().format, "unknown");
        assert!(selections.get(&1).unwrap().is_zip);
    }

    #[test]
    fn test_selection_non_cc_preferred() {
        let candidates = vec![
            create_test_candidate("Show S01E01 [CC].srt", 1, SourceType::OtherWeb, "srt", true, false, 1500),
            create_test_candidate("Show S01E01.srt", 1, SourceType::OtherWeb, "srt", false, false, 1000),
        ];

        let selections = select_best_subtitles(candidates);
        assert!(!selections.get(&1).unwrap().is_cc);
    }

    #[test]
    fn test_high_priority_zip_beats_low_priority_srt() {
        // This tests the bug we fixed: FanRetime ZIP ("unknown" format) should beat StreamDeprio SRT
        let candidates = vec![
            create_test_candidate("Netflix.S01E01.srt", 1, SourceType::StreamDeprio, "srt", false, false, 1000),
            create_test_candidate("HorribleSubs.zip", 1, SourceType::FanRetime, "unknown", false, true, 2000),
        ];

        let selections = select_best_subtitles(candidates);
        let winner = selections.get(&1).unwrap();
        assert_eq!(winner.source_type, SourceType::FanRetime);
        assert!(winner.is_zip);
    }

    #[test]
    fn test_format_filtering_respects_source_priority() {
        // Format filtering should only happen within the same source tier, not across tiers
        let candidates = vec![
            create_test_candidate("Low.S01E01.srt", 1, SourceType::StreamDeprio, "srt", false, false, 1000),
            create_test_candidate("High.S01E01.ass", 1, SourceType::FanRetime, "ass", false, false, 1000),
        ];

        let selections = select_best_subtitles(candidates);
        let winner = selections.get(&1).unwrap();
        // Higher source priority should win despite being ASS format
        assert_eq!(winner.source_type, SourceType::FanRetime);
        assert_eq!(winner.format, "ass");
    }

    #[test]
    fn test_zip_source_priority_ordering() {
        // Multiple ZIPs of different source types - highest priority should win
        let candidates = vec![
            create_test_candidate("OtherWeb.zip", 1, SourceType::OtherWeb, "unknown", false, true, 1000),
            create_test_candidate("FanRetime.zip", 1, SourceType::FanRetime, "unknown", false, true, 1000),
            create_test_candidate("BD.zip", 1, SourceType::BD, "unknown", false, true, 1000),
        ];

        let selections = select_best_subtitles(candidates);
        let winner = selections.get(&1).unwrap();
        assert_eq!(winner.source_type, SourceType::BD);
        assert_eq!(winner.file_info.name, "BD.zip");
    }
}