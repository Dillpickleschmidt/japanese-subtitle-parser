use crate::db::episode::Episode;
use crate::db::reverse_index;
use crate::db::search;
use crate::db::show::Show;
use crate::db::transcript::Transcript;
use crate::error::Error;
use rusqlite::Connection;
use serde_json::Value as JsonValue;

/// DbHandler struct that wraps a SQLite connection
pub struct DbHandler {
    pub conn: Connection,
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

    pub fn get_show_id_name_pairs(&mut self) -> Result<Vec<(i32, String)>, Error> {
        let shows = Show::get_all(&self.conn)?;
        let show_id_name_pairs = shows
            .iter()
            .filter_map(|show| show.id.map(|id| (id, show.name.clone())))
            .collect();
        Ok(show_id_name_pairs)
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
        episodes: &[(i32, String, i32, i32)],
    ) -> Result<Vec<i32>, Error> {
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
        transcripts: &[(i32, i32, String, String, String)],
    ) -> Result<Vec<i32>, Error> {
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

        let output_csv = true; // hard coded for now
        if output_csv {
            let mut count = 0;
            let mut wtr = csv::Writer::from_path("transcripts.csv")?;
            for &(_, _, _, _, ref text) in transcripts {
                count += 1;
                for line in text.split('\n') {
                    wtr.write_record(&[count.to_string(), line.to_string()])?;
                }
            }
            wtr.flush()?;
        }

        tx.commit()?;
        Ok(inserted_ids)
    }

    /// Creates a reverse index from a CSV file using the JMDict database for lookups
    pub fn create_reverse_index(
        &mut self,
        csv_path: &str,
        jmdict_db: &mut DbHandler,
    ) -> Result<(), Error> {
        reverse_index::create_reverse_index(&mut self.conn, csv_path, &mut jmdict_db.conn)
    }

    /// Performs a search for transcripts containing a specific keyword with context, filtered by shows
    pub fn search_word_with_context(
        &self,
        keyword: &str,
        shows: &[i32],
    ) -> Result<JsonValue, Error> {
        search::search_word_with_context(&self.conn, keyword, shows)
    }

    // Print the contents of an episode
    pub fn print_episode_contents(&self, show_id: i32) -> Result<(), Error> {
        search::print_episode_contents(&self.conn, show_id)
    }
}

impl Default for DbHandler {
    fn default() -> Self {
        DbHandler {
            conn: Connection::open("example.db").unwrap(),
        }
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
            let count: i32 = self
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
