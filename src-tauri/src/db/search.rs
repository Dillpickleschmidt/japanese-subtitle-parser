use crate::db::episode::Episode;
use crate::db::show::Show;
use crate::db::transcript::Transcript;
use crate::db::word::Word;
use crate::error::Error;
use rusqlite::Connection;
use serde_json::{json, Value as JsonValue};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;

/// Searches for a keyword in the transcripts of specified shows and returns the results with surrounding transcripts for context
pub fn search_word_with_context(
    conn: &Connection,
    keyword: &str,
    shows: &[i32],
) -> Result<JsonValue, Error> {
    let transcripts = get_transcripts_for_word(conn, keyword, shows)?;
    let results = build_results(conn, &transcripts)?;
    let final_json = create_final_json(keyword, results);
    write_json_to_file(&final_json, "search_results.json")?;
    Ok(final_json)
}

/// Retrieves all transcripts that contain the given word, filtered by an array of show IDs
fn get_transcripts_for_word(
    conn: &Connection,
    word: &str,
    show_ids: &[i32],
) -> Result<Vec<Transcript>, Error> {
    let word_entry = Word::get_by_word(conn, word)?;
    word_entry.get_transcripts(conn, show_ids)
}

/// Builds a structured JSON result from the transcripts, grouped by show and episode
fn build_results(conn: &Connection, transcripts: &[Transcript]) -> Result<Vec<JsonValue>, Error> {
    let mut show_map: HashMap<i32, JsonValue> = HashMap::new();

    for transcript in transcripts {
        let episode = Episode::get_by_id(conn, transcript.episode_id)?;
        let show = Show::get_by_id(conn, episode.show_id)?;

        let show_entry = show_map.entry(show.id.unwrap()).or_insert_with(|| {
            json!({
                "show": show.name,
                "instances": []
            })
        });

        add_instance_to_show(show_entry, conn, &episode, transcript.line_id)?;
    }

    Ok(show_map.into_values().collect())
}

/// Adds a new instance of the word occurrence to the JSON show entry, including context and season information
fn add_instance_to_show(
    show_entry: &mut JsonValue,
    conn: &Connection,
    episode: &Episode,
    line_id: i32,
) -> Result<(), Error> {
    let context = get_context(conn, episode, line_id)?;
    let instances = show_entry["instances"].as_array_mut().unwrap();

    // Check if an entry for this episode already exists
    if let Some(instance_entry) = instances
        .iter_mut()
        .find(|i| i["season"] == episode.season && i["episode"] == episode.episode_number)
    {
        // If the episode entry exists, extend its "lines" array with the new context
        let lines = instance_entry["lines"].as_array_mut().unwrap();

        // Add new lines from context, avoiding duplicates
        for line in context {
            if !lines
                .iter()
                .any(|existing_line| existing_line["id"] == line["id"])
            {
                lines.push(line);
            }
        }
    } else {
        // If no entry for this episode exists, create a new one
        instances.push(json!({
            "season": episode.season,
            "episode": episode.episode_number,
            "lines": context
        }));
    }

    // Sort lines by id to maintain order
    if let Some(instance_entry) = instances
        .iter_mut()
        .find(|i| i["season"] == episode.season && i["episode"] == episode.episode_number)
    {
        let lines = instance_entry["lines"].as_array_mut().unwrap();
        lines.sort_by(|a, b| a["id"].as_i64().unwrap().cmp(&b["id"].as_i64().unwrap()));
    }

    Ok(())
}

/// Retrieves the context (surrounding lines) for a given line in an episode
fn get_context(
    conn: &Connection,
    episode: &Episode,
    line_id: i32,
) -> Result<Vec<JsonValue>, Error> {
    let episode_id = episode.id.unwrap();
    let transcripts = Transcript::get_context(conn, episode_id, line_id, 2)?;
    Ok(transcripts.iter().map(transcript_to_json).collect())
}

/// Converts a Transcript object to a JSON representation
fn transcript_to_json(t: &Transcript) -> JsonValue {
    json!({
        "id": t.id,
        "text": t.text
    })
}

/// Creates the final JSON structure with the keyword and search results
fn create_final_json(keyword: &str, results: Vec<JsonValue>) -> JsonValue {
    json!({
        "keyword": keyword,
        "results": results
    })
}

/// Writes the JSON search results to a file
fn write_json_to_file(json: &JsonValue, filename: &str) -> Result<(), Error> {
    let file = File::create(filename)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, json)?;
    Ok(())
}

/// Prints the contents of a specific episode, including all transcript lines
pub fn print_episode_contents(conn: &Connection, episode_id: i32) -> Result<(), Error> {
    let transcripts = Transcript::get_by_episode_id(conn, episode_id)?;
    for t in transcripts {
        println!("{}: {}", t.time_start, t.text);
    }
    let episode = Episode::get_by_id(conn, episode_id)?;
    println!("Episode name: {}", episode.name);
    Ok(())
}
