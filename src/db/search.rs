use crate::db::episode::Episode;
use crate::db::show::Show;
use crate::db::transcript::Transcript;
use crate::db::word::Word;
use crate::error::Error;
use rusqlite::Connection;
use serde_json::{json, Value as JsonValue};

pub fn find_transcripts_with_word(conn: &Connection, word: &str) -> Result<Vec<String>, Error> {
    let word_entry = Word::get_by_word(conn, word)?;
    let transcripts = word_entry.get_transcripts(conn)?;
    Ok(transcripts.into_iter().map(|t| t.text).collect())
}

pub fn search_word_with_context(conn: &Connection, keyword: &str) -> Result<JsonValue, Error> {
    let word_entry = Word::get_by_word(conn, keyword)?;
    let transcripts = word_entry.get_transcripts(conn)?;

    let mut results = Vec::new();

    for transcript in transcripts {
        let episode = Episode::get_by_id(conn, transcript.episode_id)?;
        let show = Show::get_by_id(conn, episode.show_id)?;
        let context = get_context(
            conn,
            &show.name,
            episode.season,
            episode.episode_number,
            transcript.line_id,
        )?;

        results.push(json!({
            "show_name": show.name,
            "season": episode.season,
            "episode_number": episode.episode_number,
            "transcript_id": transcript.id,
            "context": context
        }));
    }

    Ok(json!(results))
}

fn get_context(
    conn: &Connection,
    show_name: &str,
    season: i32,
    episode_number: i32,
    line_id: i32,
) -> Result<Vec<JsonValue>, Error> {
    let show = Show::get_by_name(conn, show_name)?;
    let episode =
        Episode::get_by_show_season_episode(conn, show.id.unwrap(), season, episode_number)?;
    let transcripts = Transcript::get_context(conn, episode.id.unwrap(), line_id, 5)?;

    Ok(transcripts
        .into_iter()
        .map(|t| {
            json!({
                "id": t.id,
                "ts_num": t.line_id - line_id,
                "text": t.text
            })
        })
        .collect())
}
