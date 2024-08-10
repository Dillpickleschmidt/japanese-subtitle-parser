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

    let mut results = Vec::new();
    // Decided to use a HashMap since ordering doesn't really matter in the end
    let mut show_map: HashMap<i32, JsonValue> = HashMap::new();

    for transcript in transcripts {
        let episode = Episode::get_by_id(conn, transcript.episode_id)?;
        let show = Show::get_by_id(conn, episode.show_id)?;

        let show_entry = show_map.entry(show.id.unwrap()).or_insert_with(|| {
            json!({
                "show_name": show.name,
                "show_type": "Anime",
                "episodes": []
            })
        });

        let context = get_context(conn, &episode, transcript.line_id)?;

        let episodes = show_entry["episodes"].as_array_mut().unwrap();
        let episode_entry = episodes
            .iter_mut()
            .find(|e| e["episode_number"] == episode.episode_number);

        if let Some(episode_entry) = episode_entry {
            episode_entry["keyword_instances"]
                .as_array_mut()
                .unwrap()
                .push(context);
        } else {
            episodes.push(json!({
                "episode_name": format!("Episode {:03}", episode.episode_number),
                "season": episode.season,
                "episode_number": episode.episode_number,
                "keyword_instances": [context]
            }));
        }
    }

    results.extend(show_map.into_values());

    let final_json = json!({
        "keyword": keyword,
        "results": results
    });

    // Write to file
    let file = File::create("search_results.json")?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, &final_json)?;
    writer.flush()?;

    Ok(final_json)
}

fn get_context(conn: &Connection, episode: &Episode, line_id: i32) -> Result<JsonValue, Error> {
    let episode_id = episode.id.unwrap();
    let transcripts = Transcript::get_context(conn, episode_id, line_id, 2)?;

    let target_index = transcripts
        .iter()
        .position(|t| t.line_id == line_id)
        .unwrap();

    let context_before = transcripts[target_index.saturating_sub(2)..target_index]
        .iter()
        .map(transcript_to_json)
        .collect::<Vec<JsonValue>>();

    let target_line = transcript_to_json(&transcripts[target_index]);

    let context_after = transcripts
        [target_index + 1..std::cmp::min(target_index + 3, transcripts.len())]
        .iter()
        .map(transcript_to_json)
        .collect::<Vec<JsonValue>>();

    Ok(json!({
        "context_before": context_before,
        "target_line": target_line,
        "context_after": context_after
    }))
}

fn transcript_to_json(t: &Transcript) -> JsonValue {
    json!({
        "line_id": t.line_id,
        "text": t.text,
        "time_start": t.time_start,
        "time_end": t.time_end
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
