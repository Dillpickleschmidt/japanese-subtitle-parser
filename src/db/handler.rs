use crate::db::episode::Episode;
use crate::db::show::Show;
use crate::db::transcript::Transcript;
use crate::db::word::Word;
use crate::error::Error;
use csv;
use rusqlite::{params, Connection, Transaction};
use serde_json::{json, Value as JsonValue};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

/// DbHandler struct that wraps a SQLite connection
pub struct DbHandler {
    conn: Connection,
}

impl DbHandler {
    /// Creates a new DbHandler with a connection to the specified database file
    pub fn new(path: &str) -> Result<Self, Error> {
        let conn = Connection::open(path)?;
        Ok(Self { conn })
    }

    /// Creates the necessary tables in the database if they don't already exist
    pub fn create_tables(&self) -> Result<(), Error> {
        let sql = "
            CREATE TABLE IF NOT EXISTS shows (
                id INTEGER PRIMARY KEY, 
                name TEXT NOT NULL UNIQUE, 
                show_type TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS episodes (
                id INTEGER PRIMARY KEY, 
                show_id INTEGER,
                name TEXT NOT NULL, 
                season INTEGER, 
                episode_number INTEGER,
                UNIQUE(show_id, season, episode_number),
                FOREIGN KEY(show_id) REFERENCES shows(id)
            );
            CREATE TABLE IF NOT EXISTS transcripts (
                id INTEGER PRIMARY KEY, 
                episode_id INTEGER,
                line_id INTEGER, 
                time_start TEXT, 
                time_end TEXT, 
                text TEXT NOT NULL,
                UNIQUE(episode_id, line_id, time_start, time_end, text),
                FOREIGN KEY(episode_id) REFERENCES episodes(id)
            );
            CREATE TABLE IF NOT EXISTS words (
                id INTEGER PRIMARY KEY, 
                word TEXT NOT NULL, 
                reading TEXT,
                UNIQUE(word, reading)
            );
            CREATE TABLE IF NOT EXISTS word_occurrences (
                word_id INTEGER, 
                transcript_id INTEGER,
                FOREIGN KEY(word_id) REFERENCES words(id),
                FOREIGN KEY(transcript_id) REFERENCES transcripts(id),
                UNIQUE(word_id, transcript_id)
            );
        ";
        self.conn.execute_batch(sql)?;
        Ok(())
    }

    /// Inserts a new show into the database
    pub fn insert_shows(&mut self, shows: &[(String, String)]) -> Result<(), Error> {
        let tx = self.conn.transaction()?;
        for (name, show_type) in shows {
            let mut show = Show::new(name.clone(), show_type.clone());
            show.insert(&tx)?;
        }
        tx.commit()?;
        Ok(())
    }

    /// Inserts multiple episodes into the database
    pub fn insert_episodes(
        &mut self,
        episodes: &[(i64, String, i32, i32)],
    ) -> Result<Vec<i64>, Error> {
        let tx = self.conn.transaction()?;
        let mut inserted_ids = Vec::with_capacity(episodes.len());

        for &(show_id, ref name, season, episode_number) in episodes {
            let mut episode = Episode::new(show_id, name.clone(), season, episode_number);
            episode.insert(&tx)?;
            inserted_ids.push(episode.id.unwrap());
        }

        tx.commit()?;
        Ok(inserted_ids)
    }

    /// Inserts multiple transcripts into the database
    pub fn insert_transcripts(
        &mut self,
        transcripts: &[(i64, i32, String, String, String)],
    ) -> Result<Vec<i64>, Error> {
        let tx = self.conn.transaction()?;
        let mut inserted_ids = Vec::with_capacity(transcripts.len());

        for &(episode_id, line_id, ref time_start, ref time_end, ref text) in transcripts {
            let mut transcript = Transcript::new(
                episode_id,
                line_id,
                time_start.clone(),
                time_end.clone(),
                text.clone(),
            );
            transcript.insert(&tx)?;
            inserted_ids.push(transcript.id.unwrap());
        }

        tx.commit()?;
        Ok(inserted_ids)
    }
    /// Creates a reverse index from a CSV file
    pub fn create_reverse_index(&mut self, csv_path: &str) -> Result<(), Error> {
        // Open the CSV file
        let file = File::open(csv_path)?;
        let reader = BufReader::new(file);

        // Create a CSV reader with headers
        let mut csv_reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(reader);

        // Initialize a HashMap to store words and their occurrences
        // Key: (word, reading), Value: Vec of transcript IDs
        let mut word_map: HashMap<(String, String), Vec<i64>> = HashMap::new();

        // Iterate over each record in the CSV file
        for (index, record) in csv_reader.records().enumerate() {
            // Handle potential CSV parsing errors
            let record = record.map_err(|e| Error::InvalidInput(format!("CSV error: {}", e)))?;

            // Ensure each record has at least 3 fields
            if record.len() < 3 {
                return Err(Error::InvalidInput(format!(
                    "Invalid record at line {}: insufficient fields",
                    index + 2 // Add 2 to account for 0-indexing and header row
                )));
            }

            // Parse the transcript ID, handling potential parsing errors
            let transcript_id: i64 = record.get(0).unwrap_or("").parse().map_err(|e| {
                Error::InvalidInput(format!(
                    "Invalid transcript_id at line {}: {}",
                    index + 2,
                    e
                ))
            })?;

            // Insert the word into the hash map, or update the existing entry
            // The key is a tuple of (word, reading)
            // The value is a Vec of transcript IDs where this word appears
            word_map
                .entry((
                    record.get(1).unwrap_or("").to_string(), // word
                    record.get(2).unwrap_or("").to_string(), // reading
                ))
                .or_insert_with(Vec::new)
                .push(transcript_id);
        }

        // Start a database transaction
        let tx = self.conn.transaction()?;

        // Insert words and their occurrences into the database
        Self::insert_words_and_occurrences(&tx, word_map)?;

        // Commit the transaction
        tx.commit()?;

        Ok(())
    }

    /// Helper function to insert words and their occurrences into the database
    fn insert_words_and_occurrences(
        tx: &Transaction,
        word_map: HashMap<(String, String), Vec<i64>>,
    ) -> Result<(), Error> {
        let mut stmt_word =
            tx.prepare("INSERT OR IGNORE INTO words (word, reading) VALUES (?1, ?2)")?;
        let mut stmt_get_word_id =
            tx.prepare("SELECT id FROM words WHERE word = ?1 AND reading = ?2")?;
        let mut stmt_occurrence = tx.prepare(
            "INSERT OR IGNORE INTO word_occurrences (word_id, transcript_id) VALUES (?1, ?2)",
        )?;

        for ((word, reading), transcript_ids) in word_map {
            // Try to insert the word
            stmt_word.execute(params![&word, &reading])?;

            // Fetch the word_id (whether it was just inserted or already existed)
            let word_id: i64 =
                stmt_get_word_id.query_row(params![&word, &reading], |row| row.get(0))?;

            for &transcript_id in &transcript_ids {
                stmt_occurrence.execute(params![word_id, transcript_id])?;
            }
        }

        Ok(())
    }

    /// Finds transcripts containing a specific word
    pub fn find_transcripts_with_word(&self, word: &str) -> Result<Vec<String>, Error> {
        let conn = self.get_connection();
        let word_entry = Word::get_by_word(&conn, word)?;
        let transcripts = word_entry.get_transcripts(&conn)?;
        Ok(transcripts.into_iter().map(|t| t.text).collect())
    }

    /// Performs a complex search for transcripts containing a specific keyword
    pub fn complex_search(&self, keyword: &str) -> Result<JsonValue, Error> {
        let conn = self.get_connection();
        let word_entry = Word::get_by_word(&conn, keyword)?;
        let transcripts = word_entry.get_transcripts(&conn)?;

        let mut results = Vec::new();

        for transcript in transcripts {
            let episode = Episode::get_by_id(&conn, transcript.episode_id)?;
            let show = Show::get_by_id(&conn, episode.show_id)?;
            let context = self.get_context(
                &conn,
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

    /// Helper function to get the context (surrounding lines) for a specific line in a transcript
    fn get_context(
        &self,
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

    pub fn get_connection(&self) -> &Connection {
        &self.conn
    }
    pub fn get_connection_mut(&mut self) -> &mut Connection {
        &mut self.conn
    }
}

// Implement Display trait for DbHandler to allow easy printing of database status
impl std::fmt::Display for DbHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Database Status:")?;
        let tables = [
            "shows",
            "episodes",
            "transcripts",
            "words",
            "word_occurrences",
        ];
        for table in &tables {
            let count: i64 = self
                .conn
                .query_row(&format!("SELECT COUNT(*) FROM {}", table), [], |row| {
                    row.get(0)
                })
                .unwrap_or(0);
            writeln!(f, "  {}: {} rows", table, count)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    fn create_test_db() -> (NamedTempFile, DbHandler) {
        let file = NamedTempFile::new().unwrap();
        let mut handler = DbHandler::new(file.path().to_str().unwrap()).unwrap();
        handler.create_tables().unwrap();
        (file, handler)
    }
}
