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
                word TEXT NOT NULL UNIQUE, 
                reading TEXT,
                pos TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS word_occurrences (
                word_id INTEGER, 
                transcript_id INTEGER,
                FOREIGN KEY(word_id) REFERENCES words(id),
                FOREIGN KEY(transcript_id) REFERENCES transcripts(id),
                UNIQUE(word_id, transcript_id)
            );
            CREATE TABLE IF NOT EXISTS jlpt_levels (
                word TEXT PRIMARY KEY,
                level INTEGER NOT NULL
            );
            CREATE TABLE IF NOT EXISTS episode_jlpt_stats (
                episode_id INTEGER PRIMARY KEY,
                n5_pct REAL,
                n4_pct REAL,
                n3_pct REAL,
                n2_pct REAL,
                n1_pct REAL,
                FOREIGN KEY(episode_id) REFERENCES episodes(id)
            );
            CREATE INDEX IF NOT EXISTS idx_n4_pct ON episode_jlpt_stats(n4_pct);
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


        tx.commit()?;
        Ok(inserted_ids)
    }

    /// Creates a reverse index using kagome for Japanese morphological analysis
    pub fn create_reverse_index(&mut self) -> Result<(), Error> {
        reverse_index::create_reverse_index(&mut self.conn)
    }

    /// Performs a search for transcripts containing a specific keyword with context, filtered by shows
    pub fn search_word_with_context(
        &self,
        keyword: &str,
        shows: &[i32],
    ) -> Result<JsonValue, Error> {
        search::search_word_with_context(&self.conn, keyword, shows)
    }


    /// Imports JLPT word levels from a CSV file
    pub fn import_jlpt_csv(&mut self, path: &str) -> Result<(), Error> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        
        let tx = self.conn.transaction()?;
        
        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() || line.starts_with("word,") {
                continue; // Skip empty lines and header
            }
            
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 2 {
                let word = parts[0].trim();
                let level: i32 = parts[1].trim().parse()
                    .map_err(|_| Error::Other(format!("Invalid level in line: {}", line)))?;
                
                tx.execute(
                    "INSERT OR REPLACE INTO jlpt_levels (word, level) VALUES (?, ?)",
                    [word, &level.to_string()],
                )?;
            }
        }
        
        tx.commit()?;
        Ok(())
    }

    /// Computes JLPT statistics for all episodes
    pub fn compute_jlpt_stats(&mut self) -> Result<(), Error> {
        let tx = self.conn.transaction()?;
        
        // Clear existing stats
        tx.execute("DELETE FROM episode_jlpt_stats", [])?;
        
        // Get all episodes
        let mut stmt = tx.prepare("SELECT id FROM episodes")?;
        let episode_rows = stmt.query_map([], |row| Ok(row.get::<_, i32>(0)?))?
            .collect::<Result<Vec<_>, _>>()?;
        drop(stmt);
        
        for episode_id in episode_rows {
            // Count words by JLPT level for this episode
            let mut level_counts = [0; 6]; // index 0 unused, 1-5 for N1-N5
            let mut total_words = 0;
            
            let mut word_stmt = tx.prepare("
                SELECT jl.level, COUNT(*) as count
                FROM transcripts t
                JOIN word_occurrences wo ON wo.transcript_id = t.id
                JOIN words w ON w.id = wo.word_id
                JOIN jlpt_levels jl ON jl.word = w.word
                WHERE t.episode_id = ?
                GROUP BY jl.level
            ")?;
            
            let word_rows = word_stmt.query_map([episode_id], |row| {
                Ok((row.get::<_, i32>(0)?, row.get::<_, i32>(1)?))
            })?.collect::<Result<Vec<_>, _>>()?;
            drop(word_stmt);
            
            for (level, count) in word_rows {
                if level >= 1 && level <= 5 {
                    level_counts[level as usize] = count;
                    total_words += count;
                }
            }
            
            if total_words > 0 {
                let n5_pct = (level_counts[5] + level_counts[4] + level_counts[3] + level_counts[2] + level_counts[1]) as f64 / total_words as f64 * 100.0;
                let n4_pct = (level_counts[4] + level_counts[3] + level_counts[2] + level_counts[1]) as f64 / total_words as f64 * 100.0;
                let n3_pct = (level_counts[3] + level_counts[2] + level_counts[1]) as f64 / total_words as f64 * 100.0;
                let n2_pct = (level_counts[2] + level_counts[1]) as f64 / total_words as f64 * 100.0;
                let n1_pct = level_counts[1] as f64 / total_words as f64 * 100.0;
                
                tx.execute(
                    "INSERT INTO episode_jlpt_stats (episode_id, n5_pct, n4_pct, n3_pct, n2_pct, n1_pct) VALUES (?, ?, ?, ?, ?, ?)",
                    [episode_id.to_string(), n5_pct.to_string(), n4_pct.to_string(), n3_pct.to_string(), n2_pct.to_string(), n1_pct.to_string()],
                )?;
            }
        }
        
        tx.commit()?;
        Ok(())
    }

    /// Gets episodes where at least min_pct% of words are at min_level or easier
    pub fn get_episodes_by_jlpt(&self, min_level: u8, min_pct: f64) -> Result<Vec<(i32, String, i32, i32, f64)>, Error> {
        let column = match min_level {
            5 => "n5_pct",
            4 => "n4_pct", 
            3 => "n3_pct",
            2 => "n2_pct",
            1 => "n1_pct",
            _ => return Err(Error::Other("Invalid JLPT level. Use 1-5.".to_string())),
        };
        
        let sql = format!(
            "SELECT e.id, e.name, e.season, e.episode_number, ejs.{}
             FROM episodes e
             JOIN episode_jlpt_stats ejs ON ejs.episode_id = e.id
             WHERE ejs.{} >= ?
             ORDER BY ejs.{} DESC",
            column, column, column
        );
        
        let mut stmt = self.conn.prepare(&sql)?;
        let rows = stmt.query_map([min_pct], |row| {
            Ok((
                row.get::<_, i32>(0)?,      // episode_id
                row.get::<_, String>(1)?,   // episode_name  
                row.get::<_, i32>(2)?,      // season
                row.get::<_, i32>(3)?,      // episode_number
                row.get::<_, f64>(4)?,      // percentage
            ))
        })?;
        
        rows.collect::<Result<Vec<_>, _>>().map_err(Error::from)
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
            "jlpt_levels",
            "episode_jlpt_stats",
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
