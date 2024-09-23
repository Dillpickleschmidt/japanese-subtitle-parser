use crate::srt_parser::episode_info::extractors::*;
use crate::srt_parser::episode_info::ShowConfig;
use std::collections::HashMap;

// Function to create show configurations
pub fn create_show_configs() -> HashMap<String, ShowConfig> {
    let mut configs = HashMap::new();

    // Add default configuration
    configs.insert(
        "".to_string(),
        ShowConfig {
            episode_number_extractor: extract_episode_from_sxx_exx(),
            season_number_extractor: extract_season_from_sxx_exx(),
        },
    );

    configs.insert(
        "Hunter x Hunter".to_string(),
        ShowConfig {
            episode_number_extractor: extract_from_parentheses(),
            // Set all to season 1
            season_number_extractor: Box::new(|_| "1".to_string()),
        },
    );

    configs
}
