use crate::subtitle_importer::episode_info::extractors::*;
use crate::subtitle_importer::episode_info::ShowConfig;
use std::collections::HashMap;

// Function to create show configurations
pub fn create_show_configs() -> HashMap<String, ShowConfig> {
    let mut configs = HashMap::new();

    // Add default configuration for EXX format
    configs.insert(
        "".to_string(),
        ShowConfig {
            episode_number_extractor: extract_episode_from_exx(),
        },
    );

    configs.insert(
        "Hunter x Hunter".to_string(),
        ShowConfig {
            episode_number_extractor: extract_from_parentheses(),
        },
    );

    configs
}
