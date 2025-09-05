use crate::types::{CurrentChoice, CurrentChoices, SourceType, SubtitleCandidate, format_priority};
use csv::{Reader, Writer};
use std::collections::HashMap;
use std::path::Path;

pub fn read_existing_mapping<P: AsRef<Path>>(csv_path: P) -> Result<CurrentChoices, Box<dyn std::error::Error>> {
    let mut choices = HashMap::new();

    if !csv_path.as_ref().exists() {
        return Ok(choices);
    }

    let mut rdr = Reader::from_path(csv_path)?;
    for result in rdr.records() {
        let record = result?;
        if record.len() < 5 {
            continue;
        }

        let filename = record[0].to_string();
        let original_source = record[1].to_string();
        let source_type = parse_source_type(&record[2]);
        let format = record[3].to_string();
        let is_zip = record[4].parse::<bool>().unwrap_or(false);

        // Extract episode number from filename (S01E42.srt -> 42)
        if let Some(episode) = extract_episode_from_filename(&filename) {
            let choice = CurrentChoice {
                original_source,
                source_type,
                format,
                is_zip,
            };
            choices.insert(episode, choice);
        }
    }

    Ok(choices)
}

pub fn should_download(
    _episode: i32,
    current: Option<&CurrentChoice>,
    new_candidate: &SubtitleCandidate,
) -> bool {
    let Some(current) = current else {
        // No existing choice, download the new candidate
        return true;
    };

    // Compare priority: higher priority number wins
    let current_priority = calculate_priority(
        &current.source_type,
        &current.format,
        current.is_zip,
        false, // Assume existing files are non-CC (we don't track this)
        1000,  // Default size for comparison
    );

    let new_priority = calculate_priority(
        &new_candidate.source_type,
        &new_candidate.format,
        new_candidate.is_zip,
        new_candidate.is_cc,
        new_candidate.file_info.size,
    );

    new_priority > current_priority
}

pub fn write_mapping_csv<P: AsRef<Path>>(
    csv_path: P,
    selections: &HashMap<i32, SubtitleCandidate>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut wtr = Writer::from_path(csv_path)?;

    // Write header
    wtr.write_record(&[
        "renamed_file",
        "original_source",
        "source_type",
        "format",
        "is_zip",
    ])?;

    // Sort episodes for consistent output
    let mut episodes: Vec<_> = selections.keys().collect();
    episodes.sort();

    for &episode in episodes {
        let candidate = &selections[&episode];
        let season = 1; // Default to season 1
        let filename = format!("S{:02}E{:02}.srt", season, episode);
        
        let source_type_str = match candidate.source_type {
            SourceType::BD => "BD",
            SourceType::FanRetime => "FanRetime",
            SourceType::OtherWeb => "OtherWeb",
            SourceType::DVD => "DVD",
            SourceType::StreamDeprio => "StreamDeprio",
        };

        wtr.write_record(&[
            &filename,
            &candidate.file_info.name,
            source_type_str,
            &candidate.format,
            &candidate.is_zip.to_string(),
        ])?;
    }

    wtr.flush()?;
    Ok(())
}

fn extract_episode_from_filename(filename: &str) -> Option<i32> {
    use regex::Regex;
    let re = Regex::new(r"S\d+E(\d+)\.srt").ok()?;
    let caps = re.captures(filename)?;
    caps[1].parse().ok()
}

fn parse_source_type(s: &str) -> SourceType {
    match s {
        "BD" => SourceType::BD,
        "FanRetime" => SourceType::FanRetime,
        "OtherWeb" => SourceType::OtherWeb,
        "DVD" => SourceType::DVD,
        "StreamDeprio" => SourceType::StreamDeprio,
        _ => SourceType::OtherWeb,
    }
}

fn calculate_priority(
    source_type: &SourceType,
    format: &str,
    is_zip: bool,
    is_cc: bool,
    size: u64,
) -> i64 {
    let source_priority = source_type.clone() as i64 * 1000000;
    let format_priority = format_priority(format) as i64 * 10000;
    let zip_penalty = if is_zip { -1000 } else { 0 };
    let cc_penalty = if is_cc { -100 } else { 0 };
    let size_bonus = (size / 1000) as i64; // Size in KB as tiebreaker

    source_priority + format_priority + zip_penalty + cc_penalty + size_bonus
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{JimakuFile, SourceType, SubtitleCandidate};
    use std::rc::Rc;
    use tempfile::NamedTempFile;

    fn create_test_candidate(
        name: &str,
        episode: i32,
        source_type: SourceType,
        format: &str,
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
            is_cc: false,
            format: format.to_string(),
            is_zip,
            source_zip_url: None,
        }
    }

    #[test]
    fn test_extract_episode_from_filename() {
        assert_eq!(extract_episode_from_filename("S01E42.srt"), Some(42));
        assert_eq!(extract_episode_from_filename("S02E01.srt"), Some(1));
        assert_eq!(extract_episode_from_filename("invalid.srt"), None);
    }

    #[test]
    fn test_should_download_no_existing() {
        let new_candidate = create_test_candidate("test.srt", 1, SourceType::OtherWeb, "srt", false, 1000);
        assert!(should_download(1, None, &new_candidate));
    }

    #[test]
    fn test_should_download_better_source() {
        let current = CurrentChoice {
            original_source: "DVD.srt".to_string(),
            source_type: SourceType::DVD,
            format: "srt".to_string(),
            is_zip: false,
        };

        let new_candidate = create_test_candidate("BD.srt", 1, SourceType::BD, "srt", false, 1000);
        assert!(should_download(1, Some(&current), &new_candidate));
    }

    #[test]
    fn test_should_download_same_quality() {
        let current = CurrentChoice {
            original_source: "BD.srt".to_string(),
            source_type: SourceType::BD,
            format: "srt".to_string(),
            is_zip: false,
        };

        let new_candidate = create_test_candidate("BD2.srt", 1, SourceType::BD, "srt", false, 1000);
        assert!(!should_download(1, Some(&current), &new_candidate));
    }

    #[test]
    fn test_csv_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        let temp_file = NamedTempFile::new()?;
        let temp_path = temp_file.path();

        // Create test selections
        let mut selections = HashMap::new();
        selections.insert(
            1,
            create_test_candidate("Test S01E01.srt", 1, SourceType::BD, "srt", false, 1000),
        );

        // Write CSV
        write_mapping_csv(temp_path, &selections)?;

        // Read CSV back
        let choices = read_existing_mapping(temp_path)?;

        assert_eq!(choices.len(), 1);
        let choice = choices.get(&1).unwrap();
        assert_eq!(choice.source_type, SourceType::BD);
        assert_eq!(choice.format, "srt");
        assert!(!choice.is_zip);

        Ok(())
    }
}