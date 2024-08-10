use super::episode_info::{create_show_configs, get_episode_number, get_show_name, ShowConfig};
use super::errors::ParsingError;
use super::types::{Subtitle, Subtitles, Timestamp};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;
use walkdir::WalkDir;

pub struct SrtEntry {
    pub show_name: String,
    pub episode_name: String,
    pub episode_number: i32,
    pub content: Subtitles,
}

pub struct ShowEntry {
    pub name: String,
    pub episodes: Vec<SrtEntry>,
}

pub fn process_srt_directory(root_dir: &Path) -> Vec<ShowEntry> {
    let mut show_entries: Vec<ShowEntry> = Vec::new();
    let configs = create_show_configs();

    // Use WalkDir with sorting enabled
    let walker = WalkDir::new(root_dir)
        .sort_by(|a, b| a.file_name().cmp(b.file_name()))
        .min_depth(1)
        .max_depth(2)
        .into_iter();

    for entry in walker.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "srt") {
            println!("Processing {:?}...", path.file_name().unwrap());
            match process_srt_file(path, &configs) {
                Ok(srt_entry) => {
                    if let Some(show) = show_entries
                        .iter_mut()
                        .find(|s| s.name == srt_entry.show_name)
                    {
                        show.episodes.push(srt_entry);
                    } else {
                        show_entries.push(ShowEntry {
                            name: srt_entry.show_name.clone(),
                            episodes: vec![srt_entry],
                        });
                    }
                }
                Err(e) => eprintln!("Error processing file {:?}: {}", path, e),
            }
        }
    }

    // Sort shows alphabetically
    show_entries.sort_by(|a, b| a.name.cmp(&b.name));

    /*
    Sort episodes for each show by episode number (episode number is not the same as the file order)
        File order:
            Hunter x Hunter (1)
            Hunter x Hunter (10)
            Hunter x Hunter (100)

        Episode number order:
            Hunter x Hunter (1)
            Hunter x Hunter (2)
            Hunter x Hunter (3)

        *Episode numbers were extracted from the file names in the process_srt_file function
     */
    for show in &mut show_entries {
        show.episodes.sort_by_key(|entry| entry.episode_number);
    }

    show_entries
}

pub fn process_srt_file(
    file_path: &Path,
    configs: &HashMap<String, ShowConfig>,
) -> Result<SrtEntry, ParsingError> {
    let show_name = get_show_name(file_path);
    let episode_number = get_episode_number(&show_name, file_path, configs);
    let episode_name = file_path
        .file_stem()
        .and_then(|name| name.to_str())
        .map(String::from)
        .unwrap_or_else(|| format!("Episode {}", episode_number));

    let content = Subtitles::parse_from_file(file_path)?;

    Ok(SrtEntry {
        show_name,
        episode_name,
        episode_number,
        content,
    })
}

impl Subtitles {
    /// Parses a string containing SRT formatted subtitles into a `Subtitles` struct.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice containing the SRT formatted subtitles
    ///
    /// # Returns
    ///
    /// * `Result<Self, ParsingError>` - Parsed subtitles or an error
    pub fn parse_from_str(input: &str) -> Result<Self, ParsingError> {
        // Remove BOM if present and normalize line endings
        let input = input.trim_start_matches('\u{feff}').replace('\r', "");

        // Define regex pattern for parsing SRT format
        // Detailed explanation of the regex pattern:
        // r"(\d+)\n                     - Group 1: Matches the subtitle number (one or more digits) followed by a newline
        //   (\d{2}:\d{2}:\d{2},\d{3})   - Group 2: Matches the start time (HH:MM:SS,mmm format)
        //   -->                         - Matches the arrow separator between timestamps
        //   (\d{2}:\d{2}:\d{2},\d{3})   - Group 3: Matches the end time (HH:MM:SS,mmm format)
        //   \n                          - Matches the newline after the timestamp line
        //   ((?s:.*?)                   - Group 4: Starts the subtitle text capture
        //     (?s:.*?)                    - Non-greedy match of any characters, including newlines (s flag)
        //   (?:\n\n|$))                 - End of Group 4: Matches either two newlines or the end of the string
        //                                 This allows for multi-line subtitles and handles the last subtitle"
        let re = Regex::new(
            r"(\d+)\n(\d{2}:\d{2}:\d{2},\d{3}) --> (\d{2}:\d{2}:\d{2},\d{3})\n((?s:.*?)(?:\n\n|$))",
        )
        .map_err(|_| ParsingError::MalformedSubtitle)?;

        let mut subtitles = Vec::new();

        // Iterate over each regex match in the input
        for cap in re.captures_iter(&input) {
            // Parse subtitle number (Group 1)
            let number = cap[1].parse().map_err(|_| ParsingError::InvalidNumber)?;

            // Parse start timestamp (Group 2)
            let start_time = Timestamp::from_str(&cap[2])?;

            // Parse end timestamp (Group 3)
            let end_time = Timestamp::from_str(&cap[3])?;

            // Extract and trim subtitle text (Group 4)
            let text = cap[4].trim().to_string();

            // Debug output - consider removing in production
            // println!("Number: {}", number);
            // println!("Start time: {:?}", start_time);
            // println!("End time: {:?}", end_time);
            // println!("Text: {}", text);

            // Create and add new Subtitle to the collection
            subtitles.push(Subtitle {
                number,
                start_time,
                end_time,
                text,
            });
        }

        // Check if any subtitles were parsed
        if subtitles.is_empty() {
            Err(ParsingError::MalformedSubtitle)
        } else {
            Ok(Subtitles(subtitles))
        }
    }

    pub fn parse_from_file(path: &Path) -> Result<Self, ParsingError> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        Self::parse_from_str(&content)
    }
}
