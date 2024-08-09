use crate::db::transcript::Transcript;
use crate::db::DbHandler;
use crate::error::Error;
use rusqlite::params;

/// Represents a word in the database
#[derive(Debug)]
pub struct Word {
    pub id: Option<i64>,
    pub word: String,
    pub reading: Option<String>,
}

/// Represents a word occurrence in a transcript
#[derive(Debug)]
pub struct WordOccurrence {
    pub word_id: i64,
    pub transcript_id: i64,
}

impl Word {
    /// Creates a new Word instance
    pub fn new(word: String, reading: Option<String>) -> Self {
        Word {
            id: None,
            word,
            reading,
        }
    }

    /// Inserts the word into the database
    pub fn insert(&mut self, db: &DbHandler) -> Result<(), Error> {
        let conn = db.get_connection();
        conn.execute(
            "INSERT INTO words (word, reading) VALUES (?1, ?2)",
            params![self.word, self.reading],
        )?;
        self.id = Some(conn.last_insert_rowid());
        Ok(())
    }

    /// Updates the word in the database
    pub fn update(&self, db: &DbHandler) -> Result<(), Error> {
        let conn = db.get_connection();
        conn.execute(
            "UPDATE words SET word = ?1, reading = ?2 WHERE id = ?3",
            params![self.word, self.reading, self.id],
        )?;
        Ok(())
    }

    /// Deletes the word from the database
    pub fn delete(&self, db: &DbHandler) -> Result<(), Error> {
        let conn = db.get_connection();
        conn.execute("DELETE FROM words WHERE id = ?1", params![self.id])?;
        Ok(())
    }

    /// Retrieves a word from the database by ID
    pub fn get_by_id(db: &DbHandler, id: i64) -> Result<Word, Error> {
        let conn = db.get_connection();
        let mut stmt = conn.prepare("SELECT id, word, reading FROM words WHERE id = ?1")?;
        let word = stmt.query_row(params![id], |row| {
            Ok(Word {
                id: Some(row.get(0)?),
                word: row.get(1)?,
                reading: row.get(2)?,
            })
        })?;
        Ok(word)
    }

    /// Retrieves a word from the database by its text
    pub fn get_by_word(db: &DbHandler, word_text: &str) -> Result<Word, Error> {
        let conn = db.get_connection();
        let mut stmt = conn.prepare("SELECT id, word, reading FROM words WHERE word = ?1")?;
        let word = stmt.query_row(params![word_text], |row| {
            Ok(Word {
                id: Some(row.get(0)?),
                word: row.get(1)?,
                reading: row.get(2)?,
            })
        })?;
        Ok(word)
    }

    /// Searches for words by text
    pub fn search_by_text(db: &DbHandler, search_term: &str) -> Result<Vec<Word>, Error> {
        let conn = db.get_connection();
        let mut stmt = conn.prepare("SELECT id, word, reading FROM words WHERE word LIKE ?1")?;
        let words_iter = stmt.query_map(params![format!("%{}%", search_term)], |row| {
            Ok(Word {
                id: Some(row.get(0)?),
                word: row.get(1)?,
                reading: row.get(2)?,
            })
        })?;

        let mut words = Vec::new();
        for word in words_iter {
            words.push(word?);
        }
        Ok(words)
    }

    /// Gets all transcripts containing this word
    pub fn get_transcripts(&self, db: &DbHandler) -> Result<Vec<Transcript>, Error> {
        let conn = db.get_connection();
        let mut stmt = conn.prepare(
            "
            SELECT t.id, t.episode_id, t.line_id, t.time_start, t.time_end, t.text
            FROM transcripts t
            JOIN word_occurrences wo ON t.id = wo.transcript_id
            WHERE wo.word_id = ?1
        ",
        )?;
        let transcripts_iter = stmt.query_map(params![self.id], |row| {
            Ok(Transcript {
                id: Some(row.get(0)?),
                episode_id: row.get(1)?,
                line_id: row.get(2)?,
                time_start: row.get(3)?,
                time_end: row.get(4)?,
                text: row.get(5)?,
            })
        })?;

        let mut transcripts = Vec::new();
        for transcript in transcripts_iter {
            transcripts.push(transcript?);
        }
        Ok(transcripts)
    }
}

impl WordOccurrence {
    /// Creates a new WordOccurrence instance
    pub fn new(word_id: i64, transcript_id: i64) -> Self {
        WordOccurrence {
            word_id,
            transcript_id,
        }
    }

    /// Inserts the word occurrence into the database
    pub fn insert(&self, db: &DbHandler) -> Result<(), Error> {
        let conn = db.get_connection();
        conn.execute(
            "INSERT OR IGNORE INTO word_occurrences (word_id, transcript_id) VALUES (?1, ?2)",
            params![self.word_id, self.transcript_id],
        )?;
        Ok(())
    }

    /// Deletes the word occurrence from the database
    pub fn delete(&self, db: &DbHandler) -> Result<(), Error> {
        let conn = db.get_connection();
        conn.execute(
            "DELETE FROM word_occurrences WHERE word_id = ?1 AND transcript_id = ?2",
            params![self.word_id, self.transcript_id],
        )?;
        Ok(())
    }

    /// Retrieves all word occurrences for a specific word
    pub fn get_by_word_id(db: &DbHandler, word_id: i64) -> Result<Vec<WordOccurrence>, Error> {
        let conn = db.get_connection();
        let mut stmt =
            conn.prepare("SELECT word_id, transcript_id FROM word_occurrences WHERE word_id = ?1")?;
        let occurrences_iter = stmt.query_map(params![word_id], |row| {
            Ok(WordOccurrence {
                word_id: row.get(0)?,
                transcript_id: row.get(1)?,
            })
        })?;

        let mut occurrences = Vec::new();
        for occurrence in occurrences_iter {
            occurrences.push(occurrence?);
        }
        Ok(occurrences)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::episode::Episode;
    use crate::db::show::Show;
    use crate::db::transcript::Transcript;
    use tempfile::NamedTempFile;

    fn create_test_db() -> (NamedTempFile, DbHandler) {
        let file = NamedTempFile::new().unwrap();
        let handler = DbHandler::new(file.path().to_str().unwrap()).unwrap();
        handler.create_tables().unwrap();
        (file, handler)
    }

    fn create_test_transcript(db: &DbHandler) -> Transcript {
        let mut show = Show::new("Test Show".to_string(), "Anime".to_string());
        show.insert(db).unwrap();
        let mut episode = Episode::new(show.id.unwrap(), "Test Episode".to_string(), 1, 1);
        episode.insert(db).unwrap();
        let mut transcript = Transcript::new(
            episode.id.unwrap(),
            1,
            "00:00:01,000".to_string(),
            "00:00:05,000".to_string(),
            "Hello, world!".to_string(),
        );
        transcript.insert(db).unwrap();
        transcript
    }

    #[test]
    fn test_insert_and_get_word() {
        let (_file, db) = create_test_db();

        let mut word = Word::new("hello".to_string(), Some("こんにちは".to_string()));
        word.insert(&db).unwrap();

        assert!(word.id.is_some());

        let retrieved_word = Word::get_by_id(&db, word.id.unwrap()).unwrap();
        assert_eq!(retrieved_word.word, "hello");
        assert_eq!(retrieved_word.reading, Some("こんにちは".to_string()));
    }

    #[test]
    fn test_update_word() {
        let (_file, db) = create_test_db();

        let mut word = Word::new("hello".to_string(), Some("こんにちは".to_string()));
        word.insert(&db).unwrap();

        word.reading = Some("ハロー".to_string());
        word.update(&db).unwrap();

        let updated_word = Word::get_by_id(&db, word.id.unwrap()).unwrap();
        assert_eq!(updated_word.reading, Some("ハロー".to_string()));
    }

    #[test]
    fn test_delete_word() {
        let (_file, db) = create_test_db();

        let mut word = Word::new("hello".to_string(), None);
        word.insert(&db).unwrap();

        word.delete(&db).unwrap();

        let result = Word::get_by_id(&db, word.id.unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn test_search_words() {
        let (_file, db) = create_test_db();

        let words = vec![
            Word::new("hello".to_string(), None),
            Word::new("world".to_string(), None),
            Word::new("help".to_string(), None),
        ];

        for mut word in words {
            word.insert(&db).unwrap();
        }

        let search_results = Word::search_by_text(&db, "hel").unwrap();
        assert_eq!(search_results.len(), 2);
        assert!(search_results.iter().any(|w| w.word == "hello"));
        assert!(search_results.iter().any(|w| w.word == "help"));
    }

    #[test]
    fn test_word_occurrences() {
        let (_file, db) = create_test_db();
        let transcript = create_test_transcript(&db);

        let mut word = Word::new("hello".to_string(), None);
        word.insert(&db).unwrap();

        let occurrence = WordOccurrence::new(word.id.unwrap(), transcript.id.unwrap());
        occurrence.insert(&db).unwrap();

        let occurrences = WordOccurrence::get_by_word_id(&db, word.id.unwrap()).unwrap();
        assert_eq!(occurrences.len(), 1);
        assert_eq!(occurrences[0].transcript_id, transcript.id.unwrap());

        let transcripts = word.get_transcripts(&db).unwrap();
        assert_eq!(transcripts.len(), 1);
        assert_eq!(transcripts[0].id, transcript.id);
    }
}
