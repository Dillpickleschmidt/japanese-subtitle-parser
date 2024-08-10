use crate::srt_parser::show_configs;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;

// Define a type alias for our extraction functions
pub type Extractor = Box<dyn Fn(&str) -> String>;

// Struct to hold the extraction method for each show
pub struct ShowConfig {
    pub number_extractor: Extractor,
}

// Helper function to create regex-based extractors
pub fn regex_extractor(pattern: &str, group: usize) -> Extractor {
    let re = Regex::new(pattern).expect("Invalid regex pattern");
    Box::new(move |input: &str| {
        re.captures(input)
            .and_then(|cap| cap.get(group))
            .map(|m| m.as_str().to_string())
            .expect("Failed to extract episode number")
    })
}

// Specific extractors
pub mod extractors {
    use super::*;

    // Hunter x Hunter (1)
    pub fn extract_from_parentheses() -> Extractor {
        regex_extractor(r"\((\d+)\)", 1)
    }

    // S01E001
    pub fn extract_after_s_e() -> Extractor {
        regex_extractor(r"S\d+E(\d+)", 1)
    }

    // Episode_42
    pub fn extract_last_number() -> Extractor {
        regex_extractor(r"(\d+)(?:[^0-9]*$)", 1)
    }
}

// Function to create show configurations
pub fn create_show_configs() -> HashMap<String, ShowConfig> {
    show_configs::create_show_configs()
}

// Function to get show name from the immediate parent folder
pub fn get_show_name(file_path: &Path) -> String {
    file_path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|name| name.to_str())
        .map(String::from)
        .expect("Failed to extract show name from parent folder")
}

// Function to extract episode number using the configuration
pub fn get_episode_number(
    show_name: &str,
    file_path: &Path,
    configs: &HashMap<String, ShowConfig>,
) -> i32 {
    let config = configs
        .get(show_name)
        .expect(&format!("No configuration found for show: {}", show_name));
    let file_name = file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .expect("Failed to get file name");
    let number_str = (config.number_extractor)(file_name);
    number_str
        .parse()
        .expect(&format!("Failed to parse episode number: {}", number_str))
}
