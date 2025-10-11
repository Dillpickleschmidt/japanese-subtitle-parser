use crate::error::Error;
use rusqlite::Connection;

#[derive(Debug, Clone)]
pub struct GrammarPatternOccurrence {
    pub pattern_id: i32,
    pub transcript_id: i64,
    pub confidence: f64,
}

/// Get existing pattern ID or create new pattern and return its ID
pub fn get_or_create_pattern_id(
    conn: &Connection,
    pattern_name: &str,
    jlpt_level: &str,
) -> Result<i32, Error> {
    // Try to get existing pattern
    if let Ok(id) = conn.query_row(
        "SELECT id FROM grammar_patterns WHERE pattern_name = ?",
        [pattern_name],
        |row| row.get::<_, i32>(0),
    ) {
        return Ok(id);
    }

    // Create new pattern
    conn.execute(
        "INSERT INTO grammar_patterns (pattern_name, jlpt_level) VALUES (?, ?)",
        [pattern_name, jlpt_level],
    )?;
    Ok(conn.last_insert_rowid() as i32)
}

impl GrammarPatternOccurrence {
    pub fn new(pattern_id: i32, transcript_id: i64, confidence: f64) -> Self {
        Self {
            pattern_id,
            transcript_id,
            confidence,
        }
    }

    /// Optimized bulk insert using VALUES clauses (like word insertion)
    pub fn bulk_insert_optimized(
        occurrences: &[GrammarPatternOccurrence],
        conn: &Connection,
    ) -> Result<(), Error> {
        const CHUNK_SIZE: usize = 1000;

        for chunk in occurrences.chunks(CHUNK_SIZE) {
            let placeholders: Vec<String> = chunk.iter().map(|_| "(?, ?, ?)".to_string()).collect();
            let sql = format!(
                "INSERT OR IGNORE INTO grammar_pattern_occurrences (pattern_id, transcript_id, confidence) VALUES {}",
                placeholders.join(", ")
            );

            let mut params = Vec::with_capacity(chunk.len() * 3);
            for occurrence in chunk {
                params.push(occurrence.pattern_id.to_string());
                params.push(occurrence.transcript_id.to_string());
                params.push(occurrence.confidence.to_string());
            }

            conn.execute(&sql, rusqlite::params_from_iter(params))?;
        }

        Ok(())
    }
}

/// Helper struct to collect grammar pattern occurrences during analysis
#[derive(Debug)]
pub struct GrammarPatternCollector {
    pub occurrences: Vec<(String, i64, f64)>, // (pattern_name, transcript_id, confidence)
}

impl GrammarPatternCollector {
    pub fn new() -> Self {
        Self {
            occurrences: Vec::new(),
        }
    }

    pub fn add_pattern(&mut self, pattern_name: String, transcript_id: i64, confidence: f64) {
        self.occurrences
            .push((pattern_name, transcript_id, confidence));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn create_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();

        // Create tables
        conn.execute_batch(
            "
            CREATE TABLE grammar_patterns (
                id INTEGER PRIMARY KEY,
                pattern_name TEXT NOT NULL UNIQUE,
                jlpt_level TEXT NOT NULL
            );
            CREATE TABLE grammar_pattern_occurrences (
                pattern_id INTEGER,
                transcript_id INTEGER,
                confidence REAL,
                PRIMARY KEY (pattern_id, transcript_id),
                FOREIGN KEY (pattern_id) REFERENCES grammar_patterns(id)
            );
            CREATE TABLE transcripts (
                id INTEGER PRIMARY KEY,
                episode_id INTEGER NOT NULL,
                text TEXT NOT NULL
            );
            CREATE TABLE episodes (
                id INTEGER PRIMARY KEY
            );
        ",
        )
        .unwrap();

        conn
    }

    #[test]
    fn test_grammar_pattern_insert() {
        let conn = create_test_db();

        // Test creating a new pattern
        let id1 = get_or_create_pattern_id(&conn, "te_form", "n5").unwrap();
        assert!(id1 > 0);

        // Test that getting the same pattern returns the same ID
        let id2 = get_or_create_pattern_id(&conn, "te_form", "n5").unwrap();
        assert_eq!(id1, id2);

        // Test creating a different pattern
        let id3 = get_or_create_pattern_id(&conn, "past_tense", "n5").unwrap();
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_pattern_collector() {
        let mut collector = GrammarPatternCollector::new();
        collector.add_pattern("te_form".to_string(), 1, 8.0);
        collector.add_pattern("past_tense".to_string(), 2, 7.5);

        assert_eq!(collector.occurrences.len(), 2);
        assert_eq!(collector.occurrences[0], ("te_form".to_string(), 1, 8.0));
        assert_eq!(collector.occurrences[1], ("past_tense".to_string(), 2, 7.5));
    }
}
