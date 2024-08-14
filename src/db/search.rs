use crate::db::episode::Episode;
use crate::db::show::Show;
use crate::db::transcript::Transcript;
use crate::db::word::Word;
use crate::db::DbHandler;
use crate::error::Error;
use rusqlite::Connection;
use serde_json::{json, Value as JsonValue};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;

pub fn search_word_with_context(
    conn: &Connection,
    keyword: &str,
    jmdict_db: DbHandler,
) -> Result<JsonValue, Error> {
    let transcripts = get_transcripts_for_word(conn, keyword)?;
    let results = build_results(conn, &transcripts)?;
    let final_json = create_final_json(keyword, results);
    write_json_to_file(&final_json, "search_results.json")?;
    Ok(final_json)
}

fn get_transcripts_for_word(conn: &Connection, word: &str) -> Result<Vec<Transcript>, Error> {
    let word_entry = Word::get_by_word(conn, word)?;
    word_entry.get_transcripts(conn)
}

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

fn add_instance_to_show(
    show_entry: &mut JsonValue,
    conn: &Connection,
    episode: &Episode,
    line_id: i32,
) -> Result<(), Error> {
    let context = get_context(conn, episode, line_id)?;
    let instances = show_entry["instances"].as_array_mut().unwrap();

    if let Some(instance_entry) = instances
        .iter_mut()
        .find(|i| i["episode"] == episode.episode_number)
    {
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

    Ok(())
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

fn create_final_json(keyword: &str, results: Vec<JsonValue>) -> JsonValue {
    json!({
        "keyword": keyword,
        "results": results
    })
}

fn write_json_to_file(json: &JsonValue, filename: &str) -> Result<(), Error> {
    let file = File::create(filename)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, json)?;
    Ok(())
}

pub fn find_transcripts_with_word(conn: &Connection, word: &str) -> Result<Vec<String>, Error> {
    let transcripts = get_transcripts_for_word(conn, word)?;
    Ok(transcripts.into_iter().map(|t| t.text).collect())
}

pub fn print_episode_contents(conn: &Connection, episode_id: i32) -> Result<(), Error> {
    let transcripts = Transcript::get_by_episode_id(conn, episode_id)?;
    for t in transcripts {
        println!("{}: {}", t.time_start, t.text);
    }
    let episode = Episode::get_by_id(conn, episode_id)?;
    println!("Episode name: {}", episode.name);
    Ok(())
}
