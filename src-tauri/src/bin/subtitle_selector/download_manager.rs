use crate::types::{SelectionMap, SubtitleCandidate};
use reqwest::Client;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;
use zip::ZipArchive;

pub async fn download_and_process_files(
    selections: SelectionMap,
    output_dir: &Path,
    client: &Client,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(output_dir)?;

    // Scan existing directory for subtitle files and subfolders
    let (existing_files, existing_subfolders) = scan_directory_for_subtitles(output_dir)?;
    println!("Found {} existing files and {} subfolders in directory", existing_files.len(), existing_subfolders.len());

    // Process matching subfolders first (move contents up and clean up)
    let processed_selections = process_matching_subfolders(&existing_subfolders, &selections, output_dir).await?;

    // Match remaining existing files against remaining selections
    let (files_to_keep, files_to_download, files_to_delete) = match_existing_files(&existing_files, &processed_selections)?;
    
    // Clean up files that don't match any selection
    for file_path in files_to_delete {
        println!("Removing outdated file: {}", file_path.file_name().unwrap().to_string_lossy());
        fs::remove_file(&file_path)?;
    }
    
    // Rename kept files to standardized format if needed
    for file_path in files_to_keep {
        if let Some(episode_num) = extract_episode_from_existing_file(&file_path) {
            let season = 1; // Default to season 1
            let standard_filename = format!("S{:02}E{:02}.srt", season, episode_num);
            let standard_path = output_dir.join(&standard_filename);
            
            if file_path != standard_path {
                println!("Renaming {} to {}", 
                    file_path.file_name().unwrap().to_string_lossy(), 
                    standard_filename);
                fs::rename(&file_path, &standard_path)?;
            }
        }
    }
    
    // Group remaining downloads by ZIP file to avoid multiple downloads
    let mut zip_groups: HashMap<String, Vec<(i32, SubtitleCandidate)>> = HashMap::new();
    let mut individual_files = Vec::new();

    for (episode, candidate) in files_to_download {
        if candidate.is_zip {
            let zip_url = candidate.source_zip_url.as_ref().unwrap().clone();
            zip_groups.entry(zip_url).or_insert_with(Vec::new).push((episode, candidate));
        } else {
            individual_files.push((episode, candidate));
        }
    }

    // Process ZIP files
    for (zip_url, episodes) in zip_groups {
        process_zip_file(zip_url, episodes, output_dir, client).await?;
    }

    // Process individual files
    for (episode, candidate) in individual_files {
        process_individual_file(episode, candidate, output_dir, client).await?;
    }

    Ok(())
}

async fn process_individual_file(
    episode: i32,
    candidate: SubtitleCandidate,
    output_dir: &Path,
    client: &Client,
) -> Result<(), Box<dyn std::error::Error>> {
    let season = 1; // Default to season 1
    let output_filename = format!("S{:02}E{:02}.srt", season, episode);
    let output_path = output_dir.join(&output_filename);

    println!("Downloading episode {}: {} -> {}", 
        episode, candidate.file_info.name, output_filename);

    let response = client.get(&candidate.file_info.url).send().await?;
    let content = response.bytes().await?;

    if candidate.format == "ass" {
        // Convert ASS to SRT using ffmpeg
        convert_ass_to_srt(&content, &output_path).await?;
    } else {
        // Direct write for SRT files
        fs::write(&output_path, content)?;
    }

    Ok(())
}

async fn process_zip_file(
    zip_url: String,
    episodes: Vec<(i32, SubtitleCandidate)>,
    output_dir: &Path,
    client: &Client,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Processing ZIP file: {} for {} episodes", zip_url, episodes.len());

    // Download ZIP to temporary location
    let response = client.get(&zip_url).send().await?;
    let zip_content = response.bytes().await?;

    // Extract ZIP contents
    let temp_dir = TempDir::new()?;
    let extracted_files = extract_zip_contents(&zip_content, temp_dir.path())?;

    // Process each requested episode
    for (episode, _candidate) in episodes {
        if let Some(subtitle_file) = find_subtitle_for_episode(episode, &extracted_files) {
            let season = 1; // Default to season 1
            let output_filename = format!("S{:02}E{:02}.srt", season, episode);
            let output_path = output_dir.join(&output_filename);

            println!("  Episode {}: {} -> {}", episode, subtitle_file.file_name().unwrap().to_string_lossy(), output_filename);

            let content = fs::read(&subtitle_file)?;
            
            // Determine format by extension
            let format = subtitle_file.extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("srt")
                .to_lowercase();

            if format == "ass" {
                convert_ass_to_srt(&content, &output_path).await?;
            } else {
                fs::write(&output_path, content)?;
            }
        } else {
            println!("  Warning: Could not find subtitle for episode {} in ZIP", episode);
        }
    }

    Ok(())
}

fn extract_zip_contents(zip_data: &[u8], extract_to: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let cursor = std::io::Cursor::new(zip_data);
    let mut archive = ZipArchive::new(cursor)?;
    let mut extracted_files = Vec::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_path = extract_to.join(file.name());

        if file.is_dir() {
            fs::create_dir_all(&file_path)?;
        } else {
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent)?;
            }

            let mut output_file = fs::File::create(&file_path)?;
            std::io::copy(&mut file, &mut output_file)?;
            
            // Only track subtitle files
            if is_subtitle_file(&file_path) {
                extracted_files.push(file_path);
            }
        }
    }

    Ok(extracted_files)
}

fn find_subtitle_for_episode(episode: i32, extracted_files: &[PathBuf]) -> Option<&PathBuf> {
    use regex::Regex;

    // Try to match by episode number in filename
    let episode_patterns = [
        format!("E{:02}", episode),        // E01, E02, etc.
        format!("E{:03}", episode),        // E001, E002, etc.
        format!("EP{:02}", episode),       // EP01, EP02, etc.
        format!("Episode.{}", episode),    // Episode.1, Episode.2, etc.
        format!("{:02}", episode),         // Just the number: 01, 02, etc.
        format!("{:03}", episode),         // Just the number: 001, 002, etc.
    ];

    for file_path in extracted_files {
        let filename = file_path.file_name().unwrap().to_string_lossy();
        
        for pattern in &episode_patterns {
            if filename.contains(pattern) {
                return Some(file_path);
            }
        }
    }

    // Fallback: try regex-based matching
    for file_path in extracted_files {
        let filename = file_path.file_name().unwrap().to_string_lossy();
        if let Ok(re) = Regex::new(&format!(r"(?i)(?:E|EP|Episode).*?{}", episode)) {
            if re.is_match(&filename) {
                return Some(file_path);
            }
        }
    }

    // Last resort: if episode numbers match the file order and we have enough files
    if episode > 0 && (episode as usize) <= extracted_files.len() {
        // Sort files by name for consistent ordering
        let mut sorted_files: Vec<_> = extracted_files.iter().collect();
        sorted_files.sort_by(|a, b| {
            a.file_name().unwrap().to_string_lossy()
                .cmp(&b.file_name().unwrap().to_string_lossy())
        });
        
        if let Some(file) = sorted_files.get((episode - 1) as usize) {
            return Some(*file);
        }
    }

    None
}

fn is_subtitle_file(path: &Path) -> bool {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("srt") | Some("ass") | Some("vtt") => true,
        _ => false,
    }
}

async fn convert_ass_to_srt(ass_content: &[u8], output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let temp_ass = temp_dir.path().join("input.ass");
    let temp_srt = temp_dir.path().join("output.srt");

    // Write ASS content to temp file
    fs::write(&temp_ass, ass_content)?;

    // Use ffmpeg to convert ASS to SRT
    let output = Command::new("ffmpeg")
        .args([
            "-i", temp_ass.to_str().unwrap(),
            "-f", "srt",
            "-y", // Overwrite output file
            temp_srt.to_str().unwrap(),
        ])
        .output()?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("ffmpeg conversion failed: {}", error_msg).into());
    }

    // Copy converted SRT to final location
    fs::copy(&temp_srt, output_path)?;

    Ok(())
}

/// Scan directory for all existing subtitle files (including subfolders)
fn scan_directory_for_subtitles(dir: &Path) -> Result<(Vec<PathBuf>, Vec<PathBuf>), Box<dyn std::error::Error>> {
    let mut subtitle_files = Vec::new();
    let mut subfolders = Vec::new();
    
    if !dir.exists() {
        return Ok((subtitle_files, subfolders));
    }
    
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() && is_subtitle_file(&path) {
            subtitle_files.push(path);
        } else if path.is_dir() {
            subfolders.push(path.clone());
            
            // Scan subfolder for subtitle files
            for sub_entry in fs::read_dir(&path)? {
                let sub_entry = sub_entry?;
                let sub_path = sub_entry.path();
                
                if sub_path.is_file() && is_subtitle_file(&sub_path) {
                    subtitle_files.push(sub_path);
                }
            }
        }
    }
    
    Ok((subtitle_files, subfolders))
}

/// Match existing files against selections to determine what to keep, download, and delete
fn match_existing_files(
    existing_files: &[PathBuf],
    selections: &SelectionMap,
) -> Result<(Vec<PathBuf>, SelectionMap, Vec<PathBuf>), Box<dyn std::error::Error>> {
    let mut files_to_keep = Vec::new();
    let mut files_to_download = SelectionMap::new();
    let mut files_to_delete = Vec::new();
    
    // Create a set of all episodes we need
    let needed_episodes: std::collections::HashSet<i32> = selections.keys().copied().collect();
    
    // Check each existing file
    for existing_file in existing_files {
        let mut matches_any_episode = false;
        
        // Try to extract episode number from filename
        if let Some(episode_num) = extract_episode_from_existing_file(existing_file) {
            if needed_episodes.contains(&episode_num) {
                // This file corresponds to an episode we need
                let selection_candidate = &selections[&episode_num];
                
                // Check if existing file matches the selection
                if file_matches_selection(existing_file, selection_candidate)? {
                    // Keep this file (it's already what we want)
                    files_to_keep.push(existing_file.clone());
                    matches_any_episode = true;
                } else {
                    // File is for the right episode but wrong quality - we'll download better
                    files_to_download.insert(episode_num, selection_candidate.clone());
                }
            }
        }
        
        if !matches_any_episode {
            files_to_delete.push(existing_file.clone());
        }
    }
    
    // Add episodes that don't have any existing files
    for (&episode, candidate) in selections {
        if !files_to_keep.iter().any(|f| extract_episode_from_existing_file(f) == Some(episode)) {
            files_to_download.insert(episode, candidate.clone());
        }
    }
    
    Ok((files_to_keep, files_to_download, files_to_delete))
}

/// Extract episode number from existing filename (S01E01.srt -> 1)
fn extract_episode_from_existing_file(path: &Path) -> Option<i32> {
    let filename = path.file_name()?.to_str()?;
    
    // Try standard format first (S01E01.srt)
    if let Ok(re) = regex::Regex::new(r"S\d+E(\d+)\.") {
        if let Some(caps) = re.captures(filename) {
            return caps[1].parse().ok();
        }
    }
    
    // Try other common patterns (Episode 1, E01, etc.)
    if let Ok(re) = regex::Regex::new(r"(?:Episode\s*|Ep?\s*|E)(\d+)") {
        if let Some(caps) = re.captures(filename) {
            return caps[1].parse().ok();
        }
    }
    
    None
}

/// Check if an existing file matches what we want to select
fn file_matches_selection(existing_file: &Path, selection: &SubtitleCandidate) -> Result<bool, Box<dyn std::error::Error>> {
    if selection.is_zip {
        // For ZIP selections, check if the existing file came from the same ZIP
        // This is complex to determine, so for now assume no match (always re-extract from ZIP)
        Ok(false)
    } else {
        // For individual files, check if the filename matches the selection
        let existing_name = existing_file.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        
        // Simple comparison - if the original filename matches, keep it
        Ok(existing_name == selection.file_info.name)
    }
}

/// Process subfolders that match ZIP selections, moving their contents up to main directory
async fn process_matching_subfolders(
    subfolders: &[PathBuf],
    selections: &SelectionMap,
    output_dir: &Path,
) -> Result<SelectionMap, Box<dyn std::error::Error>> {
    let mut remaining_selections = selections.clone();
    
    for subfolder in subfolders {
        let folder_name = subfolder.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        
        // Check if any ZIP selection matches this folder name
        if let Some(matching_zip_name) = find_matching_zip_for_folder(folder_name, selections) {
            println!("Found matching subfolder '{}' for ZIP '{}'", folder_name, matching_zip_name);
            
            // Move files from subfolder to main directory
            move_subfolder_contents_to_main(subfolder, output_dir).await?;
            
            // Remove episodes from remaining selections that were satisfied by this subfolder
            let episodes_in_subfolder = get_episodes_from_subfolder(subfolder)?;
            for episode in episodes_in_subfolder {
                remaining_selections.remove(&episode);
            }
            
            // Remove empty subfolder
            if subfolder.read_dir()?.next().is_none() {
                fs::remove_dir(subfolder)?;
                println!("Removed empty subfolder: {}", folder_name);
            }
        }
    }
    
    Ok(remaining_selections)
}

/// Find ZIP selection that matches the given folder name
fn find_matching_zip_for_folder(folder_name: &str, selections: &SelectionMap) -> Option<String> {
    for candidate in selections.values() {
        if candidate.is_zip {
            let zip_name = &candidate.file_info.name;
            // Remove .zip extension and compare
            if let Some(zip_base) = zip_name.strip_suffix(".zip") {
                if zip_base == folder_name {
                    return Some(zip_name.clone());
                }
            }
        }
    }
    None
}

/// Move files from subfolder to main directory with standardized naming
async fn move_subfolder_contents_to_main(
    subfolder: &Path,
    output_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(subfolder)? {
        let entry = entry?;
        let file_path = entry.path();
        
        if file_path.is_file() && is_subtitle_file(&file_path) {
            if let Some(episode_num) = extract_episode_from_existing_file(&file_path) {
                let season = 1; // Default to season 1
                let standard_filename = format!("S{:02}E{:02}.srt", season, episode_num);
                let dest_path = output_dir.join(&standard_filename);
                
                println!("Moving {} to {}", 
                    file_path.file_name().unwrap().to_string_lossy(),
                    standard_filename);
                
                // Convert format if needed (ASS -> SRT)
                let content = fs::read(&file_path)?;
                let file_extension = file_path.extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("srt")
                    .to_lowercase();
                
                if file_extension == "ass" {
                    convert_ass_to_srt(&content, &dest_path).await?;
                } else {
                    fs::write(&dest_path, content)?;
                }
                
                // Remove original file
                fs::remove_file(&file_path)?;
            }
        }
    }
    
    Ok(())
}

/// Get list of episode numbers found in subfolder
fn get_episodes_from_subfolder(subfolder: &Path) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut episodes = Vec::new();
    
    for entry in fs::read_dir(subfolder)? {
        let entry = entry?;
        let file_path = entry.path();
        
        if file_path.is_file() && is_subtitle_file(&file_path) {
            if let Some(episode_num) = extract_episode_from_existing_file(&file_path) {
                episodes.push(episode_num);
            }
        }
    }
    
    Ok(episodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_subtitle_file() {
        assert!(is_subtitle_file(Path::new("test.srt")));
        assert!(is_subtitle_file(Path::new("test.ass")));
        assert!(is_subtitle_file(Path::new("test.vtt")));
        assert!(!is_subtitle_file(Path::new("test.txt")));
        assert!(!is_subtitle_file(Path::new("test.mp4")));
    }

    #[test]
    fn test_find_subtitle_for_episode() {
        let files = vec![
            PathBuf::from("Show E01.srt"),
            PathBuf::from("Show E02.srt"),
            PathBuf::from("Show E10.srt"),
        ];

        assert!(find_subtitle_for_episode(1, &files).is_some());
        assert!(find_subtitle_for_episode(2, &files).is_some());
        assert!(find_subtitle_for_episode(10, &files).is_some());
        assert!(find_subtitle_for_episode(99, &files).is_none());
    }

    #[test]
    fn test_find_subtitle_fallback_ordering() {
        let files = vec![
            PathBuf::from("a_subtitle.srt"),
            PathBuf::from("b_subtitle.srt"),
            PathBuf::from("c_subtitle.srt"),
        ];

        // Should pick first file for episode 1
        let result = find_subtitle_for_episode(1, &files);
        assert!(result.is_some());
        assert!(result.unwrap().file_name().unwrap().to_string_lossy().starts_with('a'));
    }

    #[test]
    fn test_extract_episode_from_existing_file() {
        assert_eq!(extract_episode_from_existing_file(Path::new("S01E01.srt")), Some(1));
        assert_eq!(extract_episode_from_existing_file(Path::new("S01E42.srt")), Some(42));
        assert_eq!(extract_episode_from_existing_file(Path::new("Episode 7.srt")), Some(7));
        assert_eq!(extract_episode_from_existing_file(Path::new("E123.srt")), Some(123));
        assert_eq!(extract_episode_from_existing_file(Path::new("random.srt")), None);
    }

    #[test]
    fn test_scan_directory_for_subtitles() {
        use tempfile::TempDir;
        use std::fs;

        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        
        // Create test files
        fs::write(temp_path.join("S01E01.srt"), "test").unwrap();
        fs::write(temp_path.join("S01E02.ass"), "test").unwrap();
        fs::write(temp_path.join("random.txt"), "test").unwrap();
        
        // Create a test subfolder with subtitle files
        fs::create_dir(temp_path.join("subfolder")).unwrap();
        fs::write(temp_path.join("subfolder/S01E03.srt"), "test").unwrap();
        
        let (subtitle_files, subfolders) = scan_directory_for_subtitles(temp_path).unwrap();
        assert_eq!(subtitle_files.len(), 3); // .srt and .ass files + subfolder file
        assert_eq!(subfolders.len(), 1); // One subfolder
        
        let filenames: Vec<String> = subtitle_files
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
            .collect();
        
        assert!(filenames.contains(&"S01E01.srt".to_string()));
        assert!(filenames.contains(&"S01E02.ass".to_string()));
        assert!(filenames.contains(&"S01E03.srt".to_string()));
        assert!(!filenames.contains(&"random.txt".to_string()));
    }

    #[test]
    fn test_find_matching_zip_for_folder() {
        use crate::types::{JimakuFile, SourceType, SubtitleCandidate};
        use std::collections::HashMap;
        use std::rc::Rc;

        let mut selections = HashMap::new();
        
        // Create a ZIP selection
        let zip_candidate = SubtitleCandidate {
            file_info: Rc::new(JimakuFile {
                name: "[HorribleSubs] Hunter x Hunter.zip".to_string(),
                url: "https://example.com/test.zip".to_string(),
                size: 1000,
                last_modified: "2024-01-01T00:00:00Z".to_string(),
            }),
            episode_numbers: vec![1],
            source_type: SourceType::FanRetime,
            is_cc: false,
            format: "unknown".to_string(),
            is_zip: true,
            source_zip_url: Some("https://example.com/test.zip".to_string()),
        };
        
        selections.insert(1, zip_candidate);
        
        // Test matching
        assert_eq!(
            find_matching_zip_for_folder("[HorribleSubs] Hunter x Hunter", &selections),
            Some("[HorribleSubs] Hunter x Hunter.zip".to_string())
        );
        
        assert_eq!(
            find_matching_zip_for_folder("Different Folder Name", &selections),
            None
        );
    }
}