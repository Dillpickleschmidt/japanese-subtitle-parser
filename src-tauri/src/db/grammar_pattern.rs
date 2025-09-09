use crate::error::Error;
use rusqlite::Connection;

#[derive(Debug, Clone)]
pub struct GrammarPattern {
    #[allow(dead_code)] // Used in database operations
    pub id: Option<i32>,
    #[allow(dead_code)] // Used in database operations
    pub pattern_name: String, // 'te_form', 'past_tense', etc.
}

#[derive(Debug, Clone)]
pub struct GrammarPatternOccurrence {
    pub pattern_id: i32,
    pub transcript_id: i64,
    pub confidence: f64,
}

impl GrammarPattern {
    #[cfg(test)]
    pub fn new(pattern_name: String) -> Self {
        Self {
            id: None,
            pattern_name,
        }
    }

    #[cfg(test)]
    pub fn insert(&mut self, conn: &Connection) -> Result<(), Error> {
        let mut stmt = conn
            .prepare_cached("INSERT OR IGNORE INTO grammar_patterns (pattern_name) VALUES (?)")?;

        stmt.execute([&self.pattern_name])?;

        // Get the pattern ID (whether it was just inserted or already existed)
        self.id = Some(Self::get_or_create_pattern_id(conn, &self.pattern_name)?);
        Ok(())
    }

    /// Get existing pattern ID or create new pattern and return its ID
    pub fn get_or_create_pattern_id(conn: &Connection, pattern_name: &str) -> Result<i32, Error> {
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
            "INSERT INTO grammar_patterns (pattern_name) VALUES (?)",
            [pattern_name],
        )?;
        Ok(conn.last_insert_rowid() as i32)
    }
}

impl GrammarPatternOccurrence {
    pub fn new(pattern_id: i32, transcript_id: i64, confidence: f64) -> Self {
        Self {
            pattern_id,
            transcript_id,
            confidence,
        }
    }

    /// Batch insert occurrences efficiently
    pub fn batch_insert(
        occurrences: &[GrammarPatternOccurrence],
        conn: &Connection,
    ) -> Result<(), Error> {
        let mut stmt = conn.prepare_cached(
            "INSERT OR IGNORE INTO grammar_pattern_occurrences (pattern_id, transcript_id, confidence) VALUES (?, ?, ?)"
        )?;

        for occurrence in occurrences {
            stmt.execute([
                &occurrence.pattern_id.to_string(),
                &occurrence.transcript_id.to_string(),
                &occurrence.confidence.to_string(),
            ])?;
        }

        Ok(())
    }
}

/// Helper struct to collect grammar pattern occurrences during analysis
#[derive(Debug)]
pub struct GrammarPatternCollector {
    occurrences: Vec<(String, i64, f64)>, // (pattern_name, transcript_id, confidence)
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

    /// Convert collected data into GrammarPatternOccurrence structs
    pub fn into_occurrences(
        self,
        conn: &Connection,
    ) -> Result<Vec<GrammarPatternOccurrence>, Error> {
        let mut result = Vec::new();

        for (pattern_name, transcript_id, confidence) in self.occurrences {
            // Get or create pattern ID
            let pattern_id = GrammarPattern::get_or_create_pattern_id(conn, &pattern_name)?;
            result.push(GrammarPatternOccurrence::new(
                pattern_id,
                transcript_id,
                confidence,
            ));
        }

        Ok(result)
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
                pattern_name TEXT NOT NULL UNIQUE
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

        let mut pattern = GrammarPattern::new("te_form".to_string());
        pattern.insert(&conn).unwrap();

        assert!(pattern.id.is_some());

        // Test that we can get the pattern ID
        let id = GrammarPattern::get_or_create_pattern_id(&conn, "te_form").unwrap();
        assert_eq!(pattern.id.unwrap(), id);
    }

    #[test]
    fn test_pattern_collector() {
        let conn = create_test_db();

        let mut collector = GrammarPatternCollector::new();
        collector.add_pattern("te_form".to_string(), 1, 8.0);
        collector.add_pattern("te_form".to_string(), 2, 7.0);

        let occurrences = collector.into_occurrences(&conn).unwrap();
        assert_eq!(occurrences.len(), 2);
        assert_eq!(occurrences[0].transcript_id, 1);
        assert_eq!(occurrences[1].transcript_id, 2);
    }
}
