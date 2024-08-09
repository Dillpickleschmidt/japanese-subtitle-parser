use crate::db::DbHandler;
use crate::error::{Error, Result};
use crate::index::ReverseIndex;
use serde_json::{json, Value as JsonValue};

pub struct ComplexSearch {
    db: DbHandler,
    reverse_index: ReverseIndex,
}

impl ComplexSearch {
    pub fn new(db: DbHandler) -> Self {
        let reverse_index = ReverseIndex::new(db.clone());
        ComplexSearch { db, reverse_index }
    }

    pub fn search(&self, keyword: &str) -> Result<JsonValue> {
        let transcript_ids = self.reverse_index.search(keyword)?;
        let mut results = Vec::new();

        for transcript_id in transcript_ids {
            let transcript = self.db.get_transcript_by_id(transcript_id)?;
            let episode = self.db.get_episode_by_id(transcript.episode_id)?;
            let show = self.db.get_show_by_id(episode.show_id)?;

            let context = self.get_context(&transcript, 5)?;

            results.push(json!({
                "show_name": show.name,
                "season": episode.season,
                "episode_number": episode.episode_number,
                "transcript_id": transcript_id,
                "context": context
            }));
        }

        Ok(json!(results))
    }

    fn get_context(
        &self,
        transcript: &crate::db::transcript::Transcript,
        context_lines: i32,
    ) -> Result<Vec<JsonValue>> {
        let conn = self.db.get_connection();
        let mut stmt = conn.prepare(
            "
            SELECT 
                id,
                line_id - ?1 AS ts_num,
                text
            FROM 
                transcripts
            WHERE 
                episode_id = ?2 AND
                line_id BETWEEN ?3 - ?4 AND ?3 + ?4
            ORDER BY 
                line_id
        ",
        )?;

        let context_iter = stmt.query_map(
            rusqlite::params![
                transcript.line_id,
                transcript.episode_id,
                transcript.line_id,
                context_lines
            ],
            |row| {
                Ok(json!({
                    "id": row.get::<_, i64>(0)?,
                    "ts_num": row.get::<_, i32>(1)?,
                    "text": row.get::<_, String>(2)?
                }))
            },
        )?;

        context_iter
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Error::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    fn create_test_db() -> (NamedTempFile, DbHandler) {
        let file = NamedTempFile::new().unwrap();
        let handler = DbHandler::new(file.path().to_str().unwrap()).unwrap();
        handler.create_tables().unwrap();
        (file, handler)
    }

    #[test]
    fn test_complex_search() {
        let (_file, db) = create_test_db();

        // Insert test data
        db.get_connection().execute_batch("
            INSERT INTO shows (id, name, show_type) VALUES (1, 'Test Show', 'Anime');
            INSERT INTO episodes (id, show_id, name, season, episode_number) VALUES (1, 1, 'Test Episode', 1, 1);
            INSERT INTO transcripts (id, episode_id, line_id, time_start, time_end, text) VALUES 
                (1, 1, 1, '00:00:01', '00:00:05', 'The quick brown fox'),
                (2, 1, 2, '00:00:06', '00:00:10', 'jumps over the lazy dog'),
                (3, 1, 3, '00:00:11', '00:00:15', 'The dog is very lazy');
        ").unwrap();

        // Build the reverse index
        let reverse_index = ReverseIndex::new(db.clone());
        reverse_index.build().unwrap();

        let complex_search = ComplexSearch::new(db);

        // Test the search
        let results = complex_search.search("lazy dog").unwrap();
        let results = results.as_array().unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0]["transcript_id"], 2);
        assert_eq!(results[1]["transcript_id"], 3);

        // Check context
        let context = results[0]["context"].as_array().unwrap();
        assert_eq!(context.len(), 3);
        assert_eq!(context[1]["text"], "jumps over the lazy dog");
    }
}
