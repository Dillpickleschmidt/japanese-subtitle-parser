use crate::subtitle_importer::show_configs;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use std::sync::OnceLock;

// Define a type alias for our extraction functions
pub type Extractor = Box<dyn Fn(&str) -> String>;

// Struct to hold the extraction method for each show
pub struct ShowConfig {
    pub episode_number_extractor: Extractor,
    pub season_number_extractor: Extractor,
}

// Static regex patterns compiled once
static PARENTHESES_REGEX: OnceLock<Regex> = OnceLock::new();
static SEASON_SXX_EXX_REGEX: OnceLock<Regex> = OnceLock::new();
static EPISODE_SXX_EXX_REGEX: OnceLock<Regex> = OnceLock::new();

fn get_parentheses_regex() -> &'static Regex {
    PARENTHESES_REGEX.get_or_init(|| Regex::new(r"\((\d+)\)").expect("Invalid regex"))
}

fn get_season_sxx_exx_regex() -> &'static Regex {
    SEASON_SXX_EXX_REGEX.get_or_init(|| Regex::new(r"S(\d+)E\d+").expect("Invalid regex"))
}

fn get_episode_sxx_exx_regex() -> &'static Regex {
    EPISODE_SXX_EXX_REGEX.get_or_init(|| Regex::new(r"S\d+E(\d+)").expect("Invalid regex"))
}


// Specific extractors
pub mod extractors {
    use super::*;

    // Hunter x Hunter (1)
    pub fn extract_from_parentheses() -> Extractor {
        Box::new(|input: &str| {
            get_parentheses_regex()
                .captures(input)
                .and_then(|cap| cap.get(1))
                .map(|m| m.as_str().to_string())
                .expect("Failed to extract episode number")
        })
    }

    // S01E001
    pub fn extract_season_from_sxx_exx() -> Extractor {
        Box::new(|input: &str| {
            get_season_sxx_exx_regex()
                .captures(input)
                .and_then(|cap| cap.get(1))
                .map(|m| m.as_str().to_string())
                .expect("Failed to extract season number")
        })
    }

    // S01E001
    pub fn extract_episode_from_sxx_exx() -> Extractor {
        Box::new(|input: &str| {
            get_episode_sxx_exx_regex()
                .captures(input)
                .and_then(|cap| cap.get(1))
                .map(|m| m.as_str().to_string())
                .expect("Failed to extract episode number")
        })
    }

    // // Episode_42
    // pub fn extract_last_number() -> Extractor {
    //     regex_extractor(r"(\d+)(?:[^0-9]*$)", 1)
    // }
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

// Function to extract season number using the configuration
pub fn get_season_number(file_path: &Path, configs: &HashMap<String, ShowConfig>) -> i32 {
    // Try to get the show-specific config, or fall back to the default
    let config = configs
        .get(&get_show_name(file_path))
        .unwrap_or_else(|| configs.get("").expect("Default configuration not found"));

    let file_name = file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_else(|| panic!("Failed to get file name for {:?}", file_path));

    let number_str = (config.season_number_extractor)(file_name);

    number_str.parse().unwrap_or_else(|_| {
        panic!(
            "Failed to parse season number: {} for show: {}",
            number_str,
            get_show_name(file_path)
        )
    })
}

// Function to extract episode number using the configuration
pub fn get_episode_number(
    show_name: &str,
    file_path: &Path,
    configs: &HashMap<String, ShowConfig>,
) -> i32 {
    // Try to get the show-specific config, or fall back to the default
    let config = configs
        .get(show_name)
        .unwrap_or_else(|| configs.get("").expect("Default configuration not found"));

    let file_name = file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_else(|| panic!("Failed to get file name for {:?}", file_path));

    let number_str = (config.episode_number_extractor)(file_name);

    number_str.parse().unwrap_or_else(|_| {
        panic!(
            "Failed to parse episode number: {} for show: {}",
            number_str, show_name
        )
    })
}
