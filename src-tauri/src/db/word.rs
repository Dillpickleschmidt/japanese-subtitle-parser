use crate::db::transcript::Transcript;
use crate::error::Error;
use rusqlite::{params, Connection};

#[derive(Debug)]
pub struct Word {
    pub id: Option<i32>,
    pub word: String,
    pub reading: Option<String>,
}

#[derive(Debug)]
pub struct WordOccurrence {
    pub word_id: i32,
    pub transcript_id: i32,
}

#[allow(dead_code)]
impl Word {
    pub fn new(word: String, reading: Option<String>) -> Self {
        Word {
            id: None,
            word,
            reading,
        }
    }

    pub fn insert(&mut self, conn: &Connection) -> Result<(), Error> {
        conn.execute(
            "INSERT INTO words (word, reading) VALUES (?1, ?2)",
            params![self.word, self.reading],
        )?;
        // Convert the last inserted row id to i32 and assign it to the word's id field
        self.id = Some(conn.last_insert_rowid().try_into().unwrap());
        Ok(())
    }

    pub fn update(&self, conn: &Connection) -> Result<(), Error> {
        conn.execute(
            "UPDATE words SET word = ?1, reading = ?2 WHERE id = ?3",
            params![self.word, self.reading, self.id],
        )?;
        Ok(())
    }

    pub fn delete(&self, conn: &Connection) -> Result<(), Error> {
        conn.execute("DELETE FROM words WHERE id = ?1", params![self.id])?;
        Ok(())
    }

    pub fn get_by_id(conn: &Connection, id: i32) -> Result<Word, Error> {
        let mut stmt = conn.prepare("SELECT id, word, reading FROM words WHERE id = ?1")?;
        stmt.query_row(params![id], |row| {
            Ok(Word {
                id: Some(row.get(0)?),
                word: row.get(1)?,
                reading: row.get(2)?,
            })
        })
        .map_err(Error::from)
    }

    pub fn get_by_word(conn: &Connection, word_text: &str) -> Result<Word, Error> {
        let mut stmt = conn.prepare("SELECT id, word, reading FROM words WHERE word = ?1")?;
        stmt.query_row(params![word_text], |row| {
            Ok(Word {
                id: Some(row.get(0)?),
                word: row.get(1)?,
                reading: row.get(2)?,
            })
        })
        .map_err(Error::from)
    }

    pub fn search_by_text(conn: &Connection, search_term: &str) -> Result<Vec<Word>, Error> {
        let mut stmt = conn.prepare("SELECT id, word, reading FROM words WHERE word LIKE ?1")?;
        let words_iter = stmt.query_map(params![format!("%{}%", search_term)], |row| {
            Ok(Word {
                id: Some(row.get(0)?),
                word: row.get(1)?,
                reading: row.get(2)?,
            })
        })?;

        words_iter
            .collect::<Result<Vec<_>, _>>()
            .map_err(Error::from)
    }

    pub fn get_transcripts(
        &self,
        conn: &Connection,
        show_ids: &[i32],
    ) -> Result<Vec<Transcript>, Error> {
        if show_ids.is_empty() {
            return Ok(Vec::new());
        }

        // For small number of show_ids, use direct IN clause for better performance
        if show_ids.len() <= 10 {
            let placeholders = show_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
            let query = format!(
                "SELECT DISTINCT t.id, t.episode_id, t.line_id, t.time_start, t.time_end, t.text
                FROM transcripts t
                JOIN word_occurrences wo ON t.id = wo.transcript_id
                JOIN episodes e ON t.episode_id = e.id
                WHERE wo.word_id = ?1 AND e.show_id IN ({})",
                placeholders
            );

            let mut stmt = conn.prepare(&query)?;
            let mut params: Vec<&dyn rusqlite::ToSql> = vec![&self.id];
            params.extend(show_ids.iter().map(|id| id as &dyn rusqlite::ToSql));

            let transcripts_iter = stmt.query_map(rusqlite::params_from_iter(params), |row| {
                Ok(Transcript {
                    id: Some(row.get(0)?),
                    episode_id: row.get(1)?,
                    line_id: row.get(2)?,
                    time_start: row.get(3)?,
                    time_end: row.get(4)?,
                    text: row.get(5)?,
                })
            })?;

            transcripts_iter
                .collect::<Result<Vec<_>, _>>()
                .map_err(Error::from)
        } else {
            // For large number of show_ids, use a temporary table approach
            self.get_transcripts_large_set(conn, show_ids)
        }
    }

    fn get_transcripts_large_set(
        &self,
        conn: &Connection,
        show_ids: &[i32],
    ) -> Result<Vec<Transcript>, Error> {
        let tx = conn.unchecked_transaction()?;

        // Create temporary table for show_ids
        tx.execute(
            "CREATE TEMP TABLE temp_show_ids (show_id INTEGER PRIMARY KEY)",
            [],
        )?;

        // Batch insert show_ids
        let mut stmt = tx.prepare("INSERT INTO temp_show_ids (show_id) VALUES (?)")?;
        for &show_id in show_ids {
            stmt.execute([show_id])?;
        }
        drop(stmt);

        // Execute optimized query
        let mut stmt = tx.prepare(
            "SELECT DISTINCT t.id, t.episode_id, t.line_id, t.time_start, t.time_end, t.text
            FROM transcripts t
            JOIN word_occurrences wo ON t.id = wo.transcript_id
            JOIN episodes e ON t.episode_id = e.id
            JOIN temp_show_ids tsi ON e.show_id = tsi.show_id
            WHERE wo.word_id = ?1",
        )?;

        let transcripts_iter = stmt.query_map([&self.id], |row| {
            Ok(Transcript {
                id: Some(row.get(0)?),
                episode_id: row.get(1)?,
                line_id: row.get(2)?,
                time_start: row.get(3)?,
                time_end: row.get(4)?,
                text: row.get(5)?,
            })
        })?;

        let results = transcripts_iter.collect::<Result<Vec<_>, _>>()?;
        drop(stmt); // Explicitly drop the statement before committing

        // Clean up temp table
        tx.execute("DROP TABLE temp_show_ids", [])?;
        tx.commit()?;

        Ok(results)
    }
}

#[allow(dead_code)]
impl WordOccurrence {
    pub fn new(word_id: i32, transcript_id: i32) -> Self {
        WordOccurrence {
            word_id,
            transcript_id,
        }
    }

    pub fn insert(&self, conn: &Connection) -> Result<(), Error> {
        conn.execute(
            "INSERT OR IGNORE INTO word_occurrences (word_id, transcript_id) VALUES (?1, ?2)",
            params![self.word_id, self.transcript_id],
        )?;
        Ok(())
    }

    pub fn delete(&self, conn: &Connection) -> Result<(), Error> {
        conn.execute(
            "DELETE FROM word_occurrences WHERE word_id = ?1 AND transcript_id = ?2",
            params![self.word_id, self.transcript_id],
        )?;
        Ok(())
    }

    pub fn get_by_word_id(conn: &Connection, word_id: i32) -> Result<Vec<WordOccurrence>, Error> {
        let mut stmt =
            conn.prepare("SELECT word_id, transcript_id FROM word_occurrences WHERE word_id = ?1")?;
        let occurrences_iter = stmt.query_map(params![word_id], |row| {
            Ok(WordOccurrence {
                word_id: row.get(0)?,
                transcript_id: row.get(1)?,
            })
        })?;

        occurrences_iter
            .collect::<Result<Vec<_>, _>>()
            .map_err(Error::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::episode::Episode;
    use crate::db::show::Show;
    use crate::db::transcript::Transcript;
    use rusqlite::Connection;
    use tempfile::NamedTempFile;

    fn create_test_db() -> (NamedTempFile, Connection) {
        let file = NamedTempFile::new().unwrap();
        let conn = Connection::open(file.path()).unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS words (
                id INTEGER PRIMARY KEY,
                word TEXT NOT NULL,
                reading TEXT
            )",
            [],
        )
        .unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS word_occurrences (
                word_id INTEGER NOT NULL,
                transcript_id INTEGER NOT NULL,
                PRIMARY KEY (word_id, transcript_id),
                FOREIGN KEY (word_id) REFERENCES words (id),
                FOREIGN KEY (transcript_id) REFERENCES transcripts (id)
            )",
            [],
        )
        .unwrap();

        // Create other necessary tables (shows, episodes, transcripts) here...

        (file, conn)
    }

    fn create_test_transcript(conn: &Connection) -> Transcript {
        let mut show = Show::new("Test Show".to_string(), "Anime".to_string());
        show.insert(conn).unwrap();
        let mut episode = Episode::new(show.id.unwrap(), "Test Episode".to_string(), 1, 1);
        episode.insert(conn).unwrap();
        let mut transcript = Transcript::new(
            episode.id.unwrap(),
            1,
            "00:00:01,000".to_string(),
            "00:00:05,000".to_string(),
            "Hello, world!".to_string(),
        );
        transcript.insert(conn).unwrap();
        transcript
    }

    #[test]
    fn test_insert_and_get_word() {
        let (_file, conn) = create_test_db();

        let mut word = Word::new("hello".to_string(), Some("こんにちは".to_string()));
        word.insert(&conn).unwrap();

        assert!(word.id.is_some());

        let retrieved_word = Word::get_by_id(&conn, word.id.unwrap()).unwrap();
        assert_eq!(retrieved_word.word, "hello");
        assert_eq!(retrieved_word.reading, Some("こんにちは".to_string()));
    }

    #[test]
    fn test_update_word() {
        let (_file, conn) = create_test_db();

        let mut word = Word::new("hello".to_string(), Some("こんにちは".to_string()));
        word.insert(&conn).unwrap();

        word.reading = Some("ハロー".to_string());
        word.update(&conn).unwrap();

        let updated_word = Word::get_by_id(&conn, word.id.unwrap()).unwrap();
        assert_eq!(updated_word.reading, Some("ハロー".to_string()));
    }

    #[test]
    fn test_delete_word() {
        let (_file, conn) = create_test_db();

        let mut word = Word::new("hello".to_string(), None);
        word.insert(&conn).unwrap();

        word.delete(&conn).unwrap();

        let result = Word::get_by_id(&conn, word.id.unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn test_search_words() {
        let (_file, conn) = create_test_db();

        let words = vec![
            Word::new("hello".to_string(), None),
            Word::new("world".to_string(), None),
            Word::new("help".to_string(), None),
        ];

        for mut word in words {
            word.insert(&conn).unwrap();
        }

        let search_results = Word::search_by_text(&conn, "hel").unwrap();
        assert_eq!(search_results.len(), 2);
        assert!(search_results.iter().any(|w| w.word == "hello"));
        assert!(search_results.iter().any(|w| w.word == "help"));
    }

    #[test]
    fn test_word_occurrences() {
        let (_file, conn) = create_test_db();
        let transcript = create_test_transcript(&conn);

        let mut word = Word::new("hello".to_string(), None);
        word.insert(&conn).unwrap();

        let occurrence = WordOccurrence::new(word.id.unwrap(), transcript.id.unwrap());
        occurrence.insert(&conn).unwrap();

        let occurrences = WordOccurrence::get_by_word_id(&conn, word.id.unwrap()).unwrap();
        assert_eq!(occurrences.len(), 1);
        assert_eq!(occurrences[0].transcript_id, transcript.id.unwrap());

        // Get the show ID from the transcript's episode
        let episode = Episode::get_by_id(&conn, transcript.episode_id).unwrap();
        let show_id = episode.show_id;

        // Use the show ID in the get_transcripts call
        let transcripts = word.get_transcripts(&conn, &[show_id]).unwrap();
        assert_eq!(transcripts.len(), 1);
        assert_eq!(transcripts[0].id, transcript.id);
    }
}
