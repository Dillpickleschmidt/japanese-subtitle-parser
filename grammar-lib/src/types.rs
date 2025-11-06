use serde::{Deserialize, Serialize};

/// Kagome token structure from morphological analysis
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KagomeToken {
    #[serde(default)]
    pub id: u32,
    #[serde(default)]
    pub start: u32,
    #[serde(default)]
    pub end: u32,
    #[serde(default)]
    pub surface: String,
    #[serde(default)]
    pub class: String,
    pub pos: Vec<String>,
    pub base_form: String,
    pub reading: String,
    #[serde(default)]
    pub pronunciation: String,
    #[serde(default)]
    pub features: Vec<String>,
}
