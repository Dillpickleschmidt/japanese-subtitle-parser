use crate::types::{JimakuFile, SourceType, SubtitleCandidate};
use regex::Regex;
use std::path::Path;
use std::rc::Rc;

/// Pre-compiled regex patterns for subtitle file parsing
struct RegexPatterns {
    range: Regex,
    single_episode: Regex,
    bd: Regex,
    retime: Regex,
    dvd: Regex,
    streaming: Regex,
    closed_caption: Regex,
}

impl RegexPatterns {
    fn new() -> Self {
        Self {
            range: Regex::new(r"0*(\d+)-(\d+)").unwrap(),
            single_episode: Regex::new(r"(?:S\d+E|EO?|Episode\s*|Ep\s*|[^\d])(\d+)(?:[^\d]|$)").unwrap(),
            bd: Regex::new(r"(?i)\b(bd|blu[\s_-]?ray?)\b").unwrap(),
            retime: Regex::new(r"(?i)\bretime[ds]?\b").unwrap(),
            dvd: Regex::new(r"(?i)\bdvd\b").unwrap(),
            streaming: Regex::new(r"(?i)\b(netflix|prime|amazon|disney)\b").unwrap(),
            closed_caption: Regex::new(r"(?i)\[?(cc|sdh)\]?").unwrap(),
        }
    }
}

/// Parse subtitle files and create candidates for selection.
/// Returns virtual candidates for ZIP files and individual candidates for regular files.
pub fn parse_candidates(files: Vec<JimakuFile>) -> Result<Vec<SubtitleCandidate>, Box<dyn std::error::Error>> {
    let patterns = RegexPatterns::new();
    let mut candidates = Vec::new();
    
    // Filter files: keep only the best ZIP of each source type + all individual files
    let (best_zips, individual_files) = filter_files(files, &patterns);
    
    // Create virtual candidates for ZIP files
    for zip_file in best_zips.into_values() {
        candidates.extend(parse_zip_candidate(zip_file, &patterns)?);
    }
    
    // Process individual subtitle files
    for file in individual_files {
        if let Some(candidate) = parse_individual_candidate(file, &patterns)? {
            candidates.push(candidate);
        }
    }

    Ok(candidates)
}

/// Filter input files to keep only the best ZIP of each source type and all individual files
fn filter_files(files: Vec<JimakuFile>, patterns: &RegexPatterns) -> (std::collections::HashMap<SourceType, JimakuFile>, Vec<JimakuFile>) {
    let mut best_zips = std::collections::HashMap::new();
    let mut individual_files = Vec::new();
    
    for file in files {
        let format = extract_format(&file.name);
        if format == "zip" {
            let source_type = detect_source_type(&file.name, patterns);
            let is_cc = is_closed_caption(&file.name, patterns);
            
            // Keep the best ZIP for each source type (prefer non-CC)
            match best_zips.get(&source_type) {
                None => {
                    best_zips.insert(source_type, file);
                }
                Some(existing) => {
                    let existing_cc = is_closed_caption(&existing.name, patterns);
                    if !is_cc && existing_cc {
                        best_zips.insert(source_type, file);
                    }
                }
            }
        } else if format == "srt" || format == "ass" {
            individual_files.push(file);
        }
    }
    
    (best_zips, individual_files)
}

/// Create virtual candidates for a ZIP file containing multiple episodes
fn parse_zip_candidate(file: JimakuFile, patterns: &RegexPatterns) -> Result<Vec<SubtitleCandidate>, Box<dyn std::error::Error>> {
    let episode_numbers = extract_episode_numbers(&file.name, patterns)?;
    if episode_numbers.is_empty() {
        return Ok(Vec::new());
    }

    let source_type = detect_source_type(&file.name, patterns);
    let is_cc = is_closed_caption(&file.name, patterns);
    let zip_url = file.url.clone();
    let file_rc = Rc::new(file);
    
    let candidates = episode_numbers
        .into_iter()
        .map(|episode| SubtitleCandidate {
            file_info: file_rc.clone(),
            episode_numbers: vec![episode],
            source_type: source_type.clone(),
            is_cc,
            format: "unknown".to_string(),
            is_zip: true,
            source_zip_url: Some(zip_url.clone()),
        })
        .collect();
    
    Ok(candidates)
}

/// Create a candidate for an individual subtitle file
fn parse_individual_candidate(file: JimakuFile, patterns: &RegexPatterns) -> Result<Option<SubtitleCandidate>, Box<dyn std::error::Error>> {
    let format = extract_format(&file.name);
    let episode_numbers = extract_episode_numbers(&file.name, patterns)?;
    
    if episode_numbers.is_empty() {
        return Ok(None);
    }

    let source_type = detect_source_type(&file.name, patterns);
    let is_cc = is_closed_caption(&file.name, patterns);

    let candidate = SubtitleCandidate {
        file_info: Rc::new(file),
        episode_numbers,
        source_type,
        is_cc,
        format,
        is_zip: false,
        source_zip_url: None,
    };
    
    Ok(Some(candidate))
}

/// Extract episode numbers from filename, handling both ranges (1-50) and single episodes
fn extract_episode_numbers(filename: &str, patterns: &RegexPatterns) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    // Check for episode ranges first (e.g., "001-167", "E01-50")
    if let Some(caps) = patterns.range.captures(filename) {
        let start: i32 = caps[1].parse()?;
        let end: i32 = caps[2].parse()?;
        return Ok((start..=end).collect());
    }

    // Look for single episode numbers
    if let Some(caps) = patterns.single_episode.captures(filename) {
        if let Ok(episode) = caps[1].parse::<i32>() {
            return Ok(vec![episode]);
        }
    }

    Ok(Vec::new())
}

/// Detect the source type of a subtitle file based on its filename
fn detect_source_type(filename: &str, patterns: &RegexPatterns) -> SourceType {
    let filename_lower = filename.to_lowercase();

    if patterns.bd.is_match(&filename_lower) {
        SourceType::BD
    } else if patterns.retime.is_match(&filename_lower) {
        SourceType::FanRetime
    } else if patterns.dvd.is_match(&filename_lower) {
        SourceType::DVD
    } else if patterns.streaming.is_match(&filename_lower) {
        SourceType::StreamDeprio
    } else {
        SourceType::OtherWeb
    }
}

/// Check if a subtitle file contains closed captions/SDH
fn is_closed_caption(filename: &str, patterns: &RegexPatterns) -> bool {
    let filename_lower = filename.to_lowercase();
    patterns.closed_caption.is_match(&filename_lower)
}

/// Extract file format from filename extension
pub fn extract_format(filename: &str) -> String {
    Path::new(filename)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_file(name: &str, size: u64) -> JimakuFile {
        JimakuFile {
            name: name.to_string(),
            url: format!("https://example.com/{}", name),
            size,
            last_modified: "2024-01-01T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn test_extract_episode_numbers_single() {
        let patterns = RegexPatterns::new();
        assert_eq!(extract_episode_numbers("Inuyasha - 053 [Hi10].srt", &patterns).unwrap(), vec![53]);
        assert_eq!(extract_episode_numbers("S01E42.srt", &patterns).unwrap(), vec![42]);
        assert_eq!(extract_episode_numbers("Episode 7.srt", &patterns).unwrap(), vec![7]);
        assert_eq!(extract_episode_numbers("EO01.srt", &patterns).unwrap(), vec![1]);
    }

    #[test]
    fn test_extract_episode_numbers_range() {
        let patterns = RegexPatterns::new();
        // Test various range formats
        assert_eq!(extract_episode_numbers("Episodes 147-148.srt", &patterns).unwrap(), vec![147, 148]);
        assert_eq!(extract_episode_numbers("1-3.srt", &patterns).unwrap(), vec![1, 2, 3]);
        assert_eq!(extract_episode_numbers("001-167.zip", &patterns).unwrap(), (1..=167).collect::<Vec<_>>());
        assert_eq!(extract_episode_numbers("[Group] Show EO01-167 (BD).zip", &patterns).unwrap(), (1..=167).collect::<Vec<_>>());
        assert_eq!(extract_episode_numbers("[Group] Show E01-50.zip", &patterns).unwrap(), (1..=50).collect::<Vec<_>>());
        assert_eq!(extract_episode_numbers("[DmonHiro] Inuyasha E001-167 (BD, 720p) (US BD).zip", &patterns).unwrap(), (1..=167).collect::<Vec<_>>());
        assert_eq!(extract_episode_numbers("Show 05-10.zip", &patterns).unwrap(), (5..=10).collect::<Vec<_>>());
    }

    #[test]
    fn test_detect_source_type() {
        let patterns = RegexPatterns::new();
        assert_eq!(detect_source_type("Show S01E01 [BD].srt", &patterns), SourceType::BD);
        assert_eq!(detect_source_type("Show S01E01 [BluRay].srt", &patterns), SourceType::BD);
        assert_eq!(detect_source_type("Show S01E01 [retime].srt", &patterns), SourceType::FanRetime);
        assert_eq!(detect_source_type("Show S01E01 [DVD].srt", &patterns), SourceType::DVD);
        assert_eq!(detect_source_type("Show S01E01 [Netflix].srt", &patterns), SourceType::StreamDeprio);
        assert_eq!(detect_source_type("Show S01E01 [Hi10].srt", &patterns), SourceType::OtherWeb);
    }

    #[test]
    fn test_detect_cc() {
        let patterns = RegexPatterns::new();
        assert!(is_closed_caption("Show S01E01 [CC].srt", &patterns));
        assert!(is_closed_caption("Show S01E01 [SDH].srt", &patterns));
        assert!(!is_closed_caption("Show S01E01.srt", &patterns));
    }

    #[test]
    fn test_zip_candidate_creation() {
        let patterns = RegexPatterns::new();
        let zip_file = create_test_file("[Group] Show EO01-3 (BD).zip", 10000);
        let candidates = parse_zip_candidate(zip_file, &patterns).unwrap();
        
        assert_eq!(candidates.len(), 3);
        assert!(candidates.iter().all(|c| c.is_zip));
        assert!(candidates.iter().all(|c| c.format == "unknown"));
        assert!(candidates.iter().all(|c| c.source_type == SourceType::BD));
        
        let episodes: Vec<i32> = candidates.iter().map(|c| c.episode_numbers[0]).collect();
        assert_eq!(episodes, vec![1, 2, 3]);
    }
}
