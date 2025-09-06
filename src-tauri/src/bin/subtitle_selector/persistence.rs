use crate::types::{CurrentChoice, CurrentChoices, SubtitleCandidate};
use csv::{Reader, Writer};
use std::collections::HashMap;
use std::path::Path;

pub fn read_existing_mapping<P: AsRef<Path>>(
    csv_path: P,
) -> Result<CurrentChoices, Box<dyn std::error::Error>> {
    let mut choices = HashMap::new();

    if !csv_path.as_ref().exists() {
        return Ok(choices);
    }

    let mut rdr = Reader::from_path(csv_path)?;
    for result in rdr.records() {
        let record = result?;
        if record.len() < 3 {
            continue;
        }

        let filename = record[0].to_string();
        let original_source = record[1].to_string();
        let format = record[2].to_string();

        // Extract episode number from filename (S01E42.srt -> 42)
        if let Some(episode) = extract_episode_from_filename(&filename) {
            let choice = CurrentChoice {
                original_source,
                format,
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

    // If we already have this exact file, don't download again
    if current.original_source == new_candidate.file_info.name {
        return false;
    }

    // Compare priority: higher priority number wins
    let current_priority = calculate_priority_from_format(&current.format);
    let new_priority = calculate_priority_from_format(if new_candidate.is_zip {
        "zip"
    } else {
        &new_candidate.format
    });

    new_priority > current_priority
}

pub fn write_mapping_csv<P: AsRef<Path>>(
    csv_path: P,
    selections: &HashMap<i32, SubtitleCandidate>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut wtr = Writer::from_path(csv_path)?;

    // Write header
    wtr.write_record(&["renamed_file", "original_source", "format"])?;

    // Sort episodes for consistent output
    let mut episodes: Vec<_> = selections.keys().collect();
    episodes.sort();

    for &episode in episodes {
        let candidate = &selections[&episode];
        let season = 1; // Default to season 1
        let filename = format!("S{:02}E{:02}.srt", season, episode);

        // Set format based on delivery method: "zip" for ZIP files, file extension for others
        let format_str = if candidate.is_zip {
            "zip"
        } else {
            // Use actual file extension (srt or ass)
            &candidate.format
        };

        wtr.write_record(&[&filename, &candidate.file_info.name, format_str])?;
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

fn calculate_priority_from_format(format: &str) -> i32 {
    match format {
        "zip" => 1000, // ZIP files are highest priority (contain multiple episodes)
        "srt" => 100,  // SRT preferred over ASS
        "ass" => 50,   // ASS acceptable
        _ => 0,        // Unknown formats lowest priority
    }
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
        let new_candidate =
            create_test_candidate("test.srt", 1, SourceType::OtherWeb, "srt", false, 1000);
        assert!(should_download(1, None, &new_candidate));
    }

    #[test]
    fn test_should_download_same_file() {
        let current = CurrentChoice {
            original_source: "test.zip".to_string(),
            format: "zip".to_string(),
        };

        let new_candidate = create_test_candidate("test.zip", 1, SourceType::BD, "srt", true, 1000);
        assert!(!should_download(1, Some(&current), &new_candidate));
    }

    #[test]
    fn test_should_download_better_format() {
        let current = CurrentChoice {
            original_source: "test.ass".to_string(),
            format: "ass".to_string(),
        };

        let new_candidate =
            create_test_candidate("test.srt", 1, SourceType::BD, "srt", false, 1000);
        assert!(should_download(1, Some(&current), &new_candidate));
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
        assert_eq!(choice.original_source, "Test S01E01.srt");
        assert_eq!(choice.format, "srt");

        Ok(())
    }
}

