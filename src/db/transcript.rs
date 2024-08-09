use crate::db::episode::Episode;
use crate::db::DbHandler;
use crate::error::Error;
use rusqlite::params;

/// Represents a transcript line in the database
#[derive(Debug)]
pub struct Transcript {
    pub id: Option<i64>,
    pub episode_id: i64,
    pub line_id: i32,
    pub time_start: String,
    pub time_end: String,
    pub text: String,
}

impl Transcript {
    /// Creates a new Transcript instance
    pub fn new(
        episode_id: i64,
        line_id: i32,
        time_start: String,
        time_end: String,
        text: String,
    ) -> Self {
        Transcript {
            id: None,
            episode_id,
            line_id,
            time_start,
            time_end,
            text,
        }
    }

    /// Inserts the transcript into the database
    pub fn insert(&mut self, db: &DbHandler) -> Result<(), Error> {
        let conn = db.get_connection();
        conn.execute(
            "INSERT INTO transcripts (episode_id, line_id, time_start, time_end, text) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![self.episode_id, self.line_id, self.time_start, self.time_end, self.text],
        )?;
        self.id = Some(conn.last_insert_rowid());
        Ok(())
    }

    /// Updates the transcript in the database
    pub fn update(&self, db: &DbHandler) -> Result<(), Error> {
        let conn = db.get_connection();
        conn.execute(
            "UPDATE transcripts SET episode_id = ?1, line_id = ?2, time_start = ?3, time_end = ?4, text = ?5 WHERE id = ?6",
            params![self.episode_id, self.line_id, self.time_start, self.time_end, self.text, self.id],
        )?;
        Ok(())
    }

    /// Deletes the transcript from the database
    pub fn delete(&self, db: &DbHandler) -> Result<(), Error> {
        let conn = db.get_connection();
        conn.execute("DELETE FROM transcripts WHERE id = ?1", params![self.id])?;
        Ok(())
    }

    /// Retrieves a transcript from the database by ID
    pub fn get_by_id(db: &DbHandler, id: i64) -> Result<Transcript, Error> {
        let conn = db.get_connection();
        let mut stmt = conn.prepare("SELECT id, episode_id, line_id, time_start, time_end, text FROM transcripts WHERE id = ?1")?;
        let transcript = stmt.query_row(params![id], |row| {
            Ok(Transcript {
                id: Some(row.get(0)?),
                episode_id: row.get(1)?,
                line_id: row.get(2)?,
                time_start: row.get(3)?,
                time_end: row.get(4)?,
                text: row.get(5)?,
            })
        })?;
        Ok(transcript)
    }

    /// Retrieves all transcripts for a specific episode
    pub fn get_all_for_episode(db: &DbHandler, episode_id: i64) -> Result<Vec<Transcript>, Error> {
        let conn = db.get_connection();
        let mut stmt = conn.prepare("SELECT id, episode_id, line_id, time_start, time_end, text FROM transcripts WHERE episode_id = ?1 ORDER BY line_id")?;
        let transcripts_iter = stmt.query_map(params![episode_id], |row| {
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

    /// Searches for transcripts by text content
    pub fn search_by_text(db: &DbHandler, search_term: &str) -> Result<Vec<Transcript>, Error> {
        let conn = db.get_connection();
        let mut stmt = conn.prepare("SELECT id, episode_id, line_id, time_start, time_end, text FROM transcripts WHERE text LIKE ?1")?;
        let transcripts_iter = stmt.query_map(params![format!("%{}%", search_term)], |row| {
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

    /// Gets the associated Episode for this Transcript
    pub fn get_episode(&self, db: &DbHandler) -> Result<Episode, Error> {
        Episode::get_by_id(db, self.episode_id)
    }

    /// Gets surrounding context for a transcript line
    pub fn get_context(
        db: &DbHandler,
        episode_id: i64,
        line_id: i32,
        context_lines: i32,
    ) -> Result<Vec<Transcript>, Error> {
        let conn = db.get_connection();
        let mut stmt = conn.prepare(
            "SELECT id, episode_id, line_id, time_start, time_end, text 
             FROM transcripts 
             WHERE episode_id = ?1 
             AND line_id BETWEEN ?2 AND ?3 
             ORDER BY line_id",
        )?;

        let start_line = line_id - context_lines;
        let end_line = line_id + context_lines;

        let transcripts_iter =
            stmt.query_map(params![episode_id, start_line, end_line], |row| {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::episode::Episode;
    use crate::db::show::Show;
    use tempfile::NamedTempFile;

    fn create_test_db() -> (NamedTempFile, DbHandler) {
        let file = NamedTempFile::new().unwrap();
        let handler = DbHandler::new(file.path().to_str().unwrap()).unwrap();
        handler.create_tables().unwrap();
        (file, handler)
    }

    fn create_test_show_and_episode(db: &DbHandler) -> Episode {
        let mut show = Show::new("Test Show".to_string(), "Anime".to_string());
        show.insert(db).unwrap();
        let mut episode = Episode::new(show.id.unwrap(), "Test Episode".to_string(), 1, 1);
        episode.insert(db).unwrap();
        episode
    }

    #[test]
    fn test_insert_and_get_transcript() {
        let (_file, db) = create_test_db();
        let episode = create_test_show_and_episode(&db);

        let mut transcript = Transcript::new(
            episode.id.unwrap(),
            1,
            "00:00:01,000".to_string(),
            "00:00:05,000".to_string(),
            "Hello, world!".to_string(),
        );
        transcript.insert(&db).unwrap();

        assert!(transcript.id.is_some());

        let retrieved_transcript = Transcript::get_by_id(&db, transcript.id.unwrap()).unwrap();
        assert_eq!(retrieved_transcript.text, "Hello, world!");
        assert_eq!(retrieved_transcript.line_id, 1);
    }

    #[test]
    fn test_update_transcript() {
        let (_file, db) = create_test_db();
        let episode = create_test_show_and_episode(&db);

        let mut transcript = Transcript::new(
            episode.id.unwrap(),
            1,
            "00:00:01,000".to_string(),
            "00:00:05,000".to_string(),
            "Hello, world!".to_string(),
        );
        transcript.insert(&db).unwrap();

        transcript.text = "Updated text".to_string();
        transcript.update(&db).unwrap();

        let updated_transcript = Transcript::get_by_id(&db, transcript.id.unwrap()).unwrap();
        assert_eq!(updated_transcript.text, "Updated text");
    }

    #[test]
    fn test_delete_transcript() {
        let (_file, db) = create_test_db();
        let episode = create_test_show_and_episode(&db);

        let mut transcript = Transcript::new(
            episode.id.unwrap(),
            1,
            "00:00:01,000".to_string(),
            "00:00:05,000".to_string(),
            "Hello, world!".to_string(),
        );
        transcript.insert(&db).unwrap();

        transcript.delete(&db).unwrap();

        let result = Transcript::get_by_id(&db, transcript.id.unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn test_get_all_for_episode() {
        let (_file, db) = create_test_db();
        let episode = create_test_show_and_episode(&db);

        let transcripts = vec![
            Transcript::new(
                episode.id.unwrap(),
                1,
                "00:00:01,000".to_string(),
                "00:00:05,000".to_string(),
                "Line 1".to_string(),
            ),
            Transcript::new(
                episode.id.unwrap(),
                2,
                "00:00:06,000".to_string(),
                "00:00:10,000".to_string(),
                "Line 2".to_string(),
            ),
            Transcript::new(
                episode.id.unwrap(),
                3,
                "00:00:11,000".to_string(),
                "00:00:15,000".to_string(),
                "Line 3".to_string(),
            ),
        ];

        for mut transcript in transcripts {
            transcript.insert(&db).unwrap();
        }

        let retrieved_transcripts =
            Transcript::get_all_for_episode(&db, episode.id.unwrap()).unwrap();
        assert_eq!(retrieved_transcripts.len(), 3);
        assert_eq!(retrieved_transcripts[0].line_id, 1);
        assert_eq!(retrieved_transcripts[1].line_id, 2);
        assert_eq!(retrieved_transcripts[2].line_id, 3);
    }

    #[test]
    fn test_search_transcripts() {
        let (_file, db) = create_test_db();
        let episode = create_test_show_and_episode(&db);

        let transcripts = vec![
            Transcript::new(
                episode.id.unwrap(),
                1,
                "00:00:01,000".to_string(),
                "00:00:05,000".to_string(),
                "Hello, world!".to_string(),
            ),
            Transcript::new(
                episode.id.unwrap(),
                2,
                "00:00:06,000".to_string(),
                "00:00:10,000".to_string(),
                "This is a test".to_string(),
            ),
            Transcript::new(
                episode.id.unwrap(),
                3,
                "00:00:11,000".to_string(),
                "00:00:15,000".to_string(),
                "Goodbye, world!".to_string(),
            ),
        ];

        for mut transcript in transcripts {
            transcript.insert(&db).unwrap();
        }

        let search_results = Transcript::search_by_text(&db, "world").unwrap();
        assert_eq!(search_results.len(), 2);
        assert!(search_results.iter().any(|t| t.text == "Hello, world!"));
        assert!(search_results.iter().any(|t| t.text == "Goodbye, world!"));
    }

    #[test]
    fn test_get_episode_for_transcript() {
        let (_file, db) = create_test_db();
        let episode = create_test_show_and_episode(&db);

        let mut transcript = Transcript::new(
            episode.id.unwrap(),
            1,
            "00:00:01,000".to_string(),
            "00:00:05,000".to_string(),
            "Hello, world!".to_string(),
        );
        transcript.insert(&db).unwrap();

        let retrieved_episode = transcript.get_episode(&db).unwrap();
        assert_eq!(retrieved_episode.id, episode.id);
        assert_eq!(retrieved_episode.name, episode.name);
    }

    #[test]
    fn test_get_context() {
        let (_file, db) = create_test_db();
        let episode = create_test_show_and_episode(&db);

        let transcripts = vec![
            Transcript::new(
                episode.id.unwrap(),
                1,
                "00:00:01,000".to_string(),
                "00:00:05,000".to_string(),
                "Line 1".to_string(),
            ),
            Transcript::new(
                episode.id.unwrap(),
                2,
                "00:00:06,000".to_string(),
                "00:00:10,000".to_string(),
                "Line 2".to_string(),
            ),
            Transcript::new(
                episode.id.unwrap(),
                3,
                "00:00:11,000".to_string(),
                "00:00:15,000".to_string(),
                "Line 3".to_string(),
            ),
            Transcript::new(
                episode.id.unwrap(),
                4,
                "00:00:16,000".to_string(),
                "00:00:20,000".to_string(),
                "Line 4".to_string(),
            ),
            Transcript::new(
                episode.id.unwrap(),
                5,
                "00:00:21,000".to_string(),
                "00:00:25,000".to_string(),
                "Line 5".to_string(),
            ),
        ];

        for mut transcript in transcripts {
            transcript.insert(&db).unwrap();
        }

        let context = Transcript::get_context(&db, episode.id.unwrap(), 3, 1).unwrap();
        assert_eq!(context.len(), 3);
        assert_eq!(context[0].line_id, 2);
        assert_eq!(context[1].line_id, 3);
        assert_eq!(context[2].line_id, 4);
    }
}
