use super::episode_info::{create_show_configs, get_episode_number, get_show_name, ShowConfig};
use super::errors::ParsingError;
use super::types::{Subtitle, Subtitles, Timestamp};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;
use walkdir::WalkDir;

pub struct SrtEntry {
    pub show_name: String,
    pub episode_name: String,
    pub episode_number: Option<i32>,
    pub content: Subtitles,
}

pub struct ShowEntry {
    pub name: String,
    pub episodes: Vec<SrtEntry>,
}

pub fn process_srt_directory(root_dir: &Path) -> Vec<ShowEntry> {
    let mut show_entries: Vec<ShowEntry> = Vec::new();
    let mut show_name_to_index: HashMap<String, usize> = HashMap::new();
    let configs = create_show_configs();

    let walker = WalkDir::new(root_dir)
        .sort_by(|a, b| a.file_name().cmp(b.file_name()))
        .min_depth(1)
        .max_depth(2)
        .into_iter();

    for entry in walker.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "srt") {
            println!("Processing {:?}...", path.file_name().unwrap());
            match process_srt_file(path, &configs) {
                Ok(srt_entry) => {
                    if let Some(&show_index) = show_name_to_index.get(&srt_entry.show_name) {
                        show_entries[show_index].episodes.push(srt_entry);
                    } else {
                        let show_index = show_entries.len();
                        show_name_to_index.insert(srt_entry.show_name.clone(), show_index);
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
        show.episodes
            .sort_by_key(|entry| entry.episode_number.unwrap_or(i32::MAX));
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
        .unwrap_or_else(|| match episode_number {
            Some(num) => format!("Episode {}", num),
            None => "Movie".to_string(),
        });

    let content = Subtitles::parse_from_file(file_path)?;

    Ok(SrtEntry {
        show_name,
        episode_name,
        episode_number,
        content,
    })
}

impl Subtitles {
    pub fn parse_from_str(input: &str) -> Result<Self, ParsingError> {
        let input = input.trim_start_matches('\u{feff}');

        let input = if input.contains('\r') {
            std::borrow::Cow::Owned(input.replace('\r', ""))
        } else {
            std::borrow::Cow::Borrowed(input)
        };

        let estimated_capacity = input.len() / 90;
        let mut subtitles = Vec::with_capacity(estimated_capacity);

        #[derive(Debug)]
        enum ParseState {
            ExpectingNumber,
            ExpectingTimestamp,
            ReadingText,
        }

        let mut state = ParseState::ExpectingNumber;
        let mut current_number = 0;
        let mut current_start_time = None;
        let mut current_end_time = None;
        let mut text_lines = Vec::new();

        for line in input.lines() {
            let line = line.trim();

            match state {
                ParseState::ExpectingNumber => {
                    if line.is_empty() {
                        continue;
                    }
                    current_number = line.parse().map_err(|_| ParsingError::InvalidNumber)?;
                    state = ParseState::ExpectingTimestamp;
                }

                ParseState::ExpectingTimestamp => {
                    if let Some(arrow_pos) = line.find(" --> ") {
                        let start_str = line[..arrow_pos].trim();
                        let end_str = line[arrow_pos + 5..].trim();

                        current_start_time = Some(Timestamp::from_str(start_str)?);
                        current_end_time = Some(Timestamp::from_str(end_str)?);
                        state = ParseState::ReadingText;
                        text_lines.clear();
                    } else {
                        return Err(ParsingError::MalformedSubtitle);
                    }
                }

                ParseState::ReadingText => {
                    if line.is_empty() {
                        if let (Some(start_time), Some(end_time)) =
                            (current_start_time.take(), current_end_time.take())
                        {
                            let text = text_lines.join("\n");
                            subtitles.push(Subtitle {
                                number: current_number,
                                start_time,
                                end_time,
                                text,
                            });
                        }
                        state = ParseState::ExpectingNumber;
                    } else {
                        text_lines.push(line);
                    }
                }
            }
        }

        if matches!(state, ParseState::ReadingText) {
            if let (Some(start_time), Some(end_time)) = (current_start_time, current_end_time) {
                let text = text_lines.join("\n");
                subtitles.push(Subtitle {
                    number: current_number,
                    start_time,
                    end_time,
                    text,
                });
            }
        }

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
