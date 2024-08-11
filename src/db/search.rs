use crate::db::episode::Episode;
use crate::db::show::Show;
use crate::db::transcript::Transcript;
use crate::db::word::Word;
use crate::error::Error;
use rusqlite::Connection;
use serde_json::{json, Value as JsonValue};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Write},
};

pub fn find_transcripts_with_word(conn: &Connection, word: &str) -> Result<Vec<String>, Error> {
    let word_entry = Word::get_by_word(conn, word)?;
    let transcripts = word_entry.get_transcripts(conn)?;
    Ok(transcripts.into_iter().map(|t| t.text).collect())
}

pub fn search_word_with_context(conn: &Connection, keyword: &str) -> Result<JsonValue, Error> {
    let word_entry = Word::get_by_word(conn, keyword)?;
    let transcripts = word_entry.get_transcripts(conn)?;

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

        let context = get_context(conn, &episode, transcript.line_id)?;

        let instances = show_entry["instances"].as_array_mut().unwrap();
        let instance_entry = instances
            .iter_mut()
            .find(|i| i["episode"] == episode.episode_number);

        if let Some(instance_entry) = instance_entry {
            instance_entry["lines"]
                .as_array_mut()
                .unwrap()
                .extend(context);
        } else {
            instances.push(json!({
                "episode": episode.episode_number,
                "lines": context
            }));
        }
    }

    let results: Vec<JsonValue> = show_map.into_values().collect();

    let final_json = json!({
        "keyword": keyword,
        "results": results
    });

    // Write to file
    let file = File::create("search_results.json")?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &final_json)?;
    writer.flush()?;

    Ok(final_json)
}

fn get_context(
    conn: &Connection,
    episode: &Episode,
    line_id: i32,
) -> Result<Vec<JsonValue>, Error> {
    let episode_id = episode.id.unwrap();
    let transcripts = Transcript::get_context(conn, episode_id, line_id, 2)?;

    Ok(transcripts.iter().map(transcript_to_json).collect())
}

fn transcript_to_json(t: &Transcript) -> JsonValue {
    json!({
        "id": t.line_id,
        "text": t.text
    })
}

// Print the contents of an episode using Transcript::get_by_episode_id
pub fn print_episode_contents(conn: &Connection, episode_id: i32) -> Result<(), Error> {
    let transcripts = Transcript::get_by_episode_id(conn, episode_id)?;
    for t in transcripts {
        println!("{}: {}", t.time_start, t.text);
    }
    let episode = Episode::get_by_id(conn, episode_id)?;
    println!("Episode name: {}", episode.name);
    Ok(())
}
