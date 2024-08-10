use crate::srt_parser::episode_info::extractors::*;
use crate::srt_parser::episode_info::ShowConfig;
use std::collections::HashMap;

// Function to create show configurations
pub fn create_show_configs() -> HashMap<String, ShowConfig> {
    let mut configs = HashMap::new();

    configs.insert(
        "Inuyasha".to_string(),
        ShowConfig {
            number_extractor: extract_after_s_e(),
        },
    );

    configs.insert(
        "Hunter x Hunter".to_string(),
        ShowConfig {
            number_extractor: extract_from_parentheses(),
        },
    );

    configs
}
