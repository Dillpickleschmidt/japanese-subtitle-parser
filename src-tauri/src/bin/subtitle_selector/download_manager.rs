use crate::types::SelectionMap;
use reqwest::Client;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// Custom error type for subtitle processing
#[derive(Debug)]
pub enum SubtitleError {
    Io(std::io::Error),
    Network(reqwest::Error),
    Other(String),
}

impl std::fmt::Display for SubtitleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubtitleError::Io(err) => write!(f, "IO error: {}", err),
            SubtitleError::Network(err) => write!(f, "Network error: {}", err),
            SubtitleError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for SubtitleError {}

impl From<std::io::Error> for SubtitleError {
    fn from(err: std::io::Error) -> Self {
        SubtitleError::Io(err)
    }
}

impl From<reqwest::Error> for SubtitleError {
    fn from(err: reqwest::Error) -> Self {
        SubtitleError::Network(err)
    }
}

impl From<Box<dyn std::error::Error>> for SubtitleError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        SubtitleError::Other(err.to_string())
    }
}

pub async fn download_and_process_files(
    selections: SelectionMap,
    output_dir: &Path,
    client: &Client,
) -> Result<(), SubtitleError> {
    fs::create_dir_all(output_dir)?;

    // 1. Process existing matching subfolders first
    process_matching_subfolders(output_dir, &selections).await?;

    // 2. Determine what files should exist after processing
    let target_files = determine_target_files(&selections);

    // 3. Scan directory and collect files to keep
    let files_to_keep = collect_files_to_keep(output_dir, &target_files)?;

    // 4. Delete everything else (except CSV and files to keep)
    cleanup_unwanted_files(output_dir, &files_to_keep)?;

    // 5. Download/extract missing files
    let download_count = download_missing_files(&selections, output_dir, client).await?;

    // Summary message
    if download_count > 0 {
        println!("Downloaded {} new episodes", download_count);
    } else {
        println!("No downloads needed - all episodes processed from local files");
    }

    Ok(())
}

/// Process existing subfolders that match ZIP selections
async fn process_matching_subfolders(
    output_dir: &Path,
    selections: &SelectionMap,
) -> Result<(), SubtitleError> {
    // Scan for subfolders
    let mut processed_episodes = Vec::new();

    if !output_dir.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(output_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let folder_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

            // Check if this subfolder matches any ZIP selection
            let matching_zip = selections.values().find(|candidate| {
                if candidate.is_zip {
                    // Extract ZIP filename and compare with folder name
                    let zip_name = &candidate.file_info.name;
                    if let Some(zip_base) = zip_name.strip_suffix(".zip") {
                        return zip_base == folder_name;
                    }
                }
                false
            });

            if matching_zip.is_some() {
                println!(
                    "Found matching local subfolder (skipping ZIP download): {}",
                    folder_name
                );

                // Find subtitle files in this subfolder and move them
                for sub_entry in fs::read_dir(&path)? {
                    let sub_entry = sub_entry?;
                    let file_path = sub_entry.path();

                    // Quick filter: skip non-files and non-subtitle files early
                    if !file_path.is_file() || !is_subtitle_file(&file_path) {
                        continue;
                    }

                    // Extract episode numbers from filename
                    if let Some(episode_numbers) = extract_episode_numbers_from_filename(&file_path)
                    {
                        for episode in episode_numbers {
                            if selections.contains_key(&episode) {
                                // Create target filename
                                let season = 1;
                                let target_filename = format!("S{:02}E{:02}.srt", season, episode);
                                let target_path = output_dir.join(&target_filename);

                                // Move and possibly convert file
                                let content = fs::read(&file_path)?;
                                let file_ext = file_path
                                    .extension()
                                    .and_then(|ext| ext.to_str())
                                    .unwrap_or("srt");

                                write_subtitle_content(&content, &target_path, file_ext).await?;
                                println!(
                                    "  {} -> {}",
                                    file_path.file_name().unwrap().to_string_lossy(),
                                    target_filename
                                );

                                processed_episodes.push(episode);
                            }
                        }
                    }
                }
            }
        }
    }

    if !processed_episodes.is_empty() {
        println!(
            "✅ Processed {} episodes from local subfolders (no downloads needed)",
            processed_episodes.len()
        );
    }

    Ok(())
}

/// Check if file is a subtitle file
fn is_subtitle_file(path: &Path) -> bool {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("srt") | Some("ass") | Some("vtt") => true,
        _ => false,
    }
}

/// Cached regex patterns for episode extraction
struct EpisodeRegexes {
    range: regex::Regex,
    episode_prefix: regex::Regex,
    three_digits: regex::Regex,
    two_digits: regex::Regex,
    any_digits: regex::Regex,
}

static EPISODE_REGEXES: OnceLock<EpisodeRegexes> = OnceLock::new();

fn get_episode_regexes() -> &'static EpisodeRegexes {
    EPISODE_REGEXES.get_or_init(|| EpisodeRegexes {
        range: regex::Regex::new(r"(\d+)-(\d+)").unwrap(),
        episode_prefix: regex::Regex::new(r"(?:Episode\s*|Ep?\s*|E)(\d+)").unwrap(),
        three_digits: regex::Regex::new(r"(\d{3})").unwrap(),
        two_digits: regex::Regex::new(r"(\d{2})").unwrap(),
        any_digits: regex::Regex::new(r"(\d+)").unwrap(),
    })
}

/// Extract episode numbers from filename using cached patterns
fn extract_episode_numbers_from_filename(path: &Path) -> Option<Vec<i32>> {
    let filename = path.file_stem()?.to_str()?;
    let regexes = get_episode_regexes();

    // Try range pattern first (e.g., "133-134" or "021-022")
    if let Some(caps) = regexes.range.captures(filename) {
        let start: i32 = caps[1].parse().ok()?;
        let end: i32 = caps[2].parse().ok()?;
        return Some((start..=end).collect());
    }

    // Try single episode patterns in priority order
    let patterns = [
        &regexes.episode_prefix, // Episode 1, Ep1, E1 - highest priority
        &regexes.three_digits,   // 001, 002, etc.
        &regexes.two_digits,     // 01, 02, etc.
        &regexes.any_digits,     // Any number - lowest priority
    ];

    for pattern in &patterns {
        if let Some(caps) = pattern.captures(filename) {
            if let Ok(episode) = caps[1].parse::<i32>() {
                return Some(vec![episode]);
            }
        }
    }

    None
}

/// Determine what files should exist in the directory after processing
fn determine_target_files(selections: &SelectionMap) -> HashSet<PathBuf> {
    let mut target_files = HashSet::new();

    // Always keep the CSV file
    target_files.insert(PathBuf::from("source_mapping.csv"));

    // Add all expected episode files
    for &episode in selections.keys() {
        let season = 1; // Default to season 1
        let filename = format!("S{:02}E{:02}.srt", season, episode);
        target_files.insert(PathBuf::from(filename));
    }

    target_files
}

/// Scan directory and collect files that should be kept
fn collect_files_to_keep(
    output_dir: &Path,
    target_files: &HashSet<PathBuf>,
) -> Result<HashSet<PathBuf>, SubtitleError> {
    let mut files_to_keep = HashSet::new();

    // Scan all files in directory (recursively for subfolders)
    let csv_filename = std::ffi::OsStr::new("source_mapping.csv");
    scan_files_recursively(output_dir, &mut |file_path| {
        let relative_path = file_path.strip_prefix(output_dir).unwrap_or(file_path);

        // Keep files that match our target files
        if target_files.contains(relative_path) {
            files_to_keep.insert(file_path.to_path_buf());
            return; // Early return to avoid checking CSV name
        }

        // Keep source_mapping.csv regardless of location
        if let Some(filename) = file_path.file_name() {
            if filename == csv_filename {
                files_to_keep.insert(file_path.to_path_buf());
            }
        }
    })?;

    Ok(files_to_keep)
}

/// Recursively scan files and call callback for each file
fn scan_files_recursively<F>(dir: &Path, callback: &mut F) -> Result<(), SubtitleError>
where
    F: FnMut(&Path),
{
    if !dir.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            callback(&path);
        } else if path.is_dir() {
            scan_files_recursively(&path, callback)?;
        }
    }

    Ok(())
}

/// Delete everything except files in the keep list
fn cleanup_unwanted_files(
    output_dir: &Path,
    files_to_keep: &HashSet<PathBuf>,
) -> Result<(), SubtitleError> {
    let mut files_to_delete = Vec::new();
    let mut dirs_to_delete = Vec::new();

    // Collect all files and directories
    scan_directory_recursive(output_dir, &mut files_to_delete, &mut dirs_to_delete)?;

    // Delete unwanted files
    let mut delete_count = 0;
    for file_path in files_to_delete {
        if !files_to_keep.contains(&file_path) {
            fs::remove_file(&file_path)?;
            delete_count += 1;
        }
    }

    // Delete empty directories (in reverse order - deepest first)
    dirs_to_delete.sort_by_key(|p| std::cmp::Reverse(p.components().count()));
    for dir_path in dirs_to_delete {
        if dir_path != output_dir && is_empty_directory(&dir_path)? {
            fs::remove_dir(&dir_path)?;
            println!("Removed empty directory: {}", dir_path.display());
            delete_count += 1;
        }
    }

    if delete_count > 0 {
        println!(
            "🧹 Cleaned up {} unwanted files and empty directories",
            delete_count
        );
    }

    Ok(())
}

/// Recursively scan directory collecting files and directories
fn scan_directory_recursive(
    dir: &Path,
    files: &mut Vec<PathBuf>,
    dirs: &mut Vec<PathBuf>,
) -> Result<(), SubtitleError> {
    if !dir.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            files.push(path);
        } else if path.is_dir() {
            dirs.push(path.clone());
            scan_directory_recursive(&path, files, dirs)?;
        }
    }

    Ok(())
}

/// Check if directory is empty
fn is_empty_directory(dir: &Path) -> Result<bool, SubtitleError> {
    let mut entries = fs::read_dir(dir)?;
    Ok(entries.next().is_none())
}

/// Download and extract missing files
async fn download_missing_files(
    selections: &SelectionMap,
    output_dir: &Path,
    client: &Client,
) -> Result<usize, SubtitleError> {
    let mut download_count = 0;

    // Check what files are missing and download them
    for (&episode, candidate) in selections {
        let season = 1; // Default to season 1
        let target_path = output_dir.join(format!("S{:02}E{:02}.srt", season, episode));

        if !target_path.exists() {
            download_count += 1;
            if candidate.is_zip {
                // Handle ZIP extraction - for now, just print what we'd do
                println!(
                    "Would extract episode {} from ZIP: {}",
                    episode, candidate.file_info.name
                );
            } else {
                // Download individual file
                println!(
                    "Downloading episode {}: {}",
                    episode, candidate.file_info.name
                );
                let response = client.get(&candidate.file_info.url).send().await?;
                let content = response.bytes().await?;
                write_subtitle_content(&content, &target_path, &candidate.format).await?;
            }
        }
    }

    Ok(download_count)
}

/// Write content to file with format conversion if needed
async fn write_subtitle_content(
    content: &[u8],
    dest: &Path,
    format: &str,
) -> Result<(), SubtitleError> {
    match format {
        "ass" => {
            // Convert ASS to SRT using ffmpeg
            convert_ass_to_srt(content, dest).await
        }
        _ => {
            // Write directly for SRT and other formats
            fs::write(dest, content).map_err(SubtitleError::from)
        }
    }
}

async fn convert_ass_to_srt(ass_content: &[u8], output_path: &Path) -> Result<(), SubtitleError> {
    use tempfile::TempDir;
    use tokio::process::Command;

    let temp_dir = TempDir::new()?;
    let temp_ass = temp_dir.path().join("input.ass");
    let temp_srt = temp_dir.path().join("output.srt");

    // Write ASS content to temp file
    fs::write(&temp_ass, ass_content)?;

    // Use ffmpeg to convert ASS to SRT
    let output = Command::new("ffmpeg")
        .args([
            "-i",
            temp_ass.to_str().unwrap(),
            "-f",
            "srt",
            "-y", // Overwrite output file
            temp_srt.to_str().unwrap(),
        ])
        .output()
        .await?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(SubtitleError::Other(format!(
            "ffmpeg conversion failed: {}",
            error_msg
        )));
    }

    // Copy converted SRT to final location
    fs::copy(&temp_srt, output_path)?;

    Ok(())
}
