use crate::subtitle_importer::show_configs;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use std::sync::OnceLock;

// Define a type alias for our extraction functions
pub type Extractor = Box<dyn Fn(&str) -> Option<String>>;

// Struct to hold the extraction method for each show
pub struct ShowConfig {
    pub episode_number_extractor: Extractor,
}

// Static regex patterns compiled once
static PARENTHESES_REGEX: OnceLock<Regex> = OnceLock::new();
static EPISODE_EXX_REGEX: OnceLock<Regex> = OnceLock::new();

fn get_parentheses_regex() -> &'static Regex {
    PARENTHESES_REGEX.get_or_init(|| Regex::new(r"\((\d+)\)").expect("Invalid regex"))
}

fn get_episode_exx_regex() -> &'static Regex {
    EPISODE_EXX_REGEX.get_or_init(|| Regex::new(r"E(\d+)").expect("Invalid regex"))
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
        })
    }

    // E001
    pub fn extract_episode_from_exx() -> Extractor {
        Box::new(|input: &str| {
            get_episode_exx_regex()
                .captures(input)
                .and_then(|cap| cap.get(1))
                .map(|m| m.as_str().to_string())
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

// Function to extract episode number using the configuration
pub fn get_episode_number(
    show_name: &str,
    file_path: &Path,
    configs: &HashMap<String, ShowConfig>,
) -> Option<i32> {
    // Try to get the show-specific config, or fall back to the default
    let config = configs
        .get(show_name)
        .unwrap_or_else(|| configs.get("").expect("Default configuration not found"));

    let file_name = file_path.file_stem().and_then(|s| s.to_str())?;

    // Try to extract episode number, return None if extraction fails
    let number_str = (config.episode_number_extractor)(file_name)?;

    // Try to parse the extracted number, return None if parsing fails
    number_str.parse().ok()
}
