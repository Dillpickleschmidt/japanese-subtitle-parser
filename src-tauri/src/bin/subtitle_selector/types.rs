use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JimakuFile {
    pub name: String,
    pub url: String,
    pub size: u64,
    pub last_modified: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SourceType {
    BD = 100,
    FanRetime = 90,
    OtherWeb = 80,
    DVD = 70,
    StreamDeprio = 50,
}

#[derive(Debug, Clone)]
pub struct SubtitleCandidate {
    pub file_info: Rc<JimakuFile>,
    pub episode_numbers: Vec<i32>,
    pub source_type: SourceType,
    pub is_cc: bool,
    pub format: String,
    pub is_zip: bool,
    pub source_zip_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CurrentChoice {
    pub original_source: String,
    pub source_type: SourceType,
    pub format: String,
    pub is_zip: bool,
}

pub type SelectionMap = HashMap<i32, SubtitleCandidate>;
pub type CurrentChoices = HashMap<i32, CurrentChoice>;

pub fn format_source_type(source_type: &SourceType) -> &'static str {
    match source_type {
        SourceType::BD => "BD/BluRay",
        SourceType::FanRetime => "Fan Retime",
        SourceType::OtherWeb => "Other Web",
        SourceType::DVD => "DVD",
        SourceType::StreamDeprio => "Streaming (Deprio)",
    }
}

pub fn format_priority(format: &str) -> i32 {
    match format {
        "srt" => 100,
        "unknown" => 50,
        "ass" => 10,
        _ => 0,
    }
}