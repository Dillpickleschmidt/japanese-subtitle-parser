use crate::db::DbHandler;
use crate::error::{Error, Result};
use rusqlite::params;
use std::collections::HashMap;

pub struct ReverseIndex {
    db: DbHandler,
}

impl ReverseIndex {
    pub fn new(db: DbHandler) -> Self {
        ReverseIndex { db }
    }

    pub fn build(&self) -> Result<()> {
        let conn = self.db.get_connection();

        // Start a transaction for better performance
        let tx = conn.transaction()?;

        // Clear existing index
        tx.execute("DELETE FROM word_occurrences", [])?;
        tx.execute("DELETE FROM words", [])?;

        // Fetch all transcripts
        let mut stmt = tx.prepare("SELECT id, text FROM transcripts")?;
        let transcript_iter = stmt.query_map([], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
        })?;

        // Prepare statements for inserting words and occurrences
        let mut insert_word_stmt = tx.prepare("INSERT OR IGNORE INTO words (word) VALUES (?)")?;
        let mut insert_occurrence_stmt = tx.prepare(
            "INSERT OR IGNORE INTO word_occurrences (word_id, transcript_id) VALUES (?, ?)",
        )?;
        let mut get_word_id_stmt = tx.prepare("SELECT id FROM words WHERE word = ?")?;

        for transcript in transcript_iter {
            let (transcript_id, text) = transcript?;
            let words = Self::tokenize(&text);

            for word in words {
                // Insert or get word ID
                insert_word_stmt.execute(params![word])?;
                let word_id: i64 = get_word_id_stmt.query_row(params![word], |row| row.get(0))?;

                // Insert word occurrence
                insert_occurrence_stmt.execute(params![word_id, transcript_id])?;
            }
        }

        // Commit the transaction
        tx.commit()?;

        Ok(())
    }

    pub fn search(&self, query: &str) -> Result<Vec<i64>> {
        let conn = self.db.get_connection();
        let words = Self::tokenize(query);

        let mut transcript_counts = HashMap::new();

        for word in words {
            let mut stmt = conn.prepare(
                "
                SELECT DISTINCT wo.transcript_id
                FROM word_occurrences wo
                JOIN words w ON wo.word_id = w.id
                WHERE w.word = ?
            ",
            )?;

            let transcript_ids = stmt.query_map(params![word], |row| row.get::<_, i64>(0))?;

            for transcript_id in transcript_ids {
                *transcript_counts.entry(transcript_id?).or_insert(0) += 1;
            }
        }

        // Sort transcripts by the number of matching words, in descending order
        let mut results: Vec<_> = transcript_counts.into_iter().collect();
        results.sort_by(|a, b| b.1.cmp(&a.1));

        Ok(results.into_iter().map(|(id, _)| id).collect())
    }

    fn tokenize(text: &str) -> Vec<String> {
        text.split_whitespace()
            .map(|word| word.to_lowercase())
            .map(|word| word.trim_matches(|c: char| !c.is_alphanumeric()))
            .filter(|word| !word.is_empty())
            .map(String::from)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::DbHandler;
    use tempfile::NamedTempFile;

    fn create_test_db() -> (NamedTempFile, DbHandler) {
        let file = NamedTempFile::new().unwrap();
        let handler = DbHandler::new(file.path().to_str().unwrap()).unwrap();
        handler.create_tables().unwrap();
        (file, handler)
    }

    #[test]
    fn test_build_and_search_reverse_index() {
        let (_file, db) = create_test_db();

        // Insert test data
        db.get_connection().execute_batch("
            INSERT INTO shows (id, name, show_type) VALUES (1, 'Test Show', 'Anime');
            INSERT INTO episodes (id, show_id, name, season, episode_number) VALUES (1, 1, 'Test Episode', 1, 1);
            INSERT INTO transcripts (id, episode_id, line_id, time_start, time_end, text) VALUES 
                (1, 1, 1, '00:00:01', '00:00:05', 'The quick brown fox'),
                (2, 1, 2, '00:00:06', '00:00:10', 'jumps over the lazy dog');
        ").unwrap();

        let reverse_index = ReverseIndex::new(db);
        reverse_index.build().unwrap();

        // Test single word search
        let results = reverse_index.search("quick").unwrap();
        assert_eq!(results, vec![1]);

        // Test multiple word search
        let results = reverse_index.search("the lazy").unwrap();
        assert_eq!(results, vec![2]);

        // Test search with no results
        let results = reverse_index.search("nonexistent").unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_tokenize() {
        let text = "The quick, brown fox-jumps over the lazy dog!";
        let tokens = ReverseIndex::tokenize(text);
        assert_eq!(
            tokens,
            vec!["the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"]
        );
    }
}
