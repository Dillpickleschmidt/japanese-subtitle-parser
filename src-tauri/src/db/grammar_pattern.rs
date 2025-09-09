use crate::error::Error;
use rusqlite::Connection;
use serde_json::{json, Value as JsonValue};

#[derive(Debug, Clone)]
pub struct GrammarPattern {
    pub id: Option<i32>,
    pub pattern_name: String, // 'te_form', 'past_tense', etc.
}

#[derive(Debug, Clone)]
pub struct GrammarPatternOccurrence {
    pub pattern_id: i32,
    pub transcript_id: i64,
    pub confidence: f64,
}

#[allow(dead_code)]
impl GrammarPattern {
    pub fn new(pattern_name: String) -> Self {
        Self {
            id: None,
            pattern_name,
        }
    }

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

#[allow(dead_code)]
impl GrammarPatternOccurrence {
    pub fn new(pattern_id: i32, transcript_id: i64, confidence: f64) -> Self {
        Self {
            pattern_id,
            transcript_id,
            confidence,
        }
    }

    pub fn insert(&self, conn: &Connection) -> Result<(), Error> {
        conn.execute(
            "INSERT OR IGNORE INTO grammar_pattern_occurrences (pattern_id, transcript_id, confidence) VALUES (?, ?, ?)",
            [&self.pattern_id.to_string(), &self.transcript_id.to_string(), &self.confidence.to_string()]
        )?;
        Ok(())
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

    /// Get patterns for a specific episode with occurrence counts and examples
    pub fn get_by_episode(conn: &Connection, episode_id: i32) -> Result<JsonValue, Error> {
        let mut stmt = conn.prepare_cached(
            "SELECT gp.pattern_name, 
                    COUNT(gpo.transcript_id) as occurrence_count,
                    AVG(gpo.confidence) as avg_confidence
             FROM grammar_patterns gp
             JOIN grammar_pattern_occurrences gpo ON gp.id = gpo.pattern_id
             JOIN transcripts t ON gpo.transcript_id = t.id
             WHERE t.episode_id = ?
             GROUP BY gp.id, gp.pattern_name
             ORDER BY occurrence_count DESC",
        )?;

        let mut patterns = Vec::new();
        let rows = stmt.query_map([episode_id], |row| {
            Ok(json!({
                "pattern_name": row.get::<_, String>(0)?,
                "count": row.get::<_, i64>(1)?,
                "confidence": row.get::<_, f64>(2)?
            }))
        })?;

        for row in rows {
            patterns.push(row?);
        }

        Ok(json!({
            "episode_id": episode_id,
            "patterns": patterns
        }))
    }

    /// Get examples (transcript text) for a specific pattern in an episode
    pub fn get_examples(
        conn: &Connection,
        pattern_name: &str,
        episode_id: i32,
        limit: Option<usize>,
    ) -> Result<Vec<String>, Error> {
        let limit_clause = if let Some(limit) = limit {
            format!("LIMIT {}", limit)
        } else {
            String::new()
        };

        let query = format!(
            "SELECT t.text 
             FROM transcripts t
             JOIN grammar_pattern_occurrences gpo ON t.id = gpo.transcript_id
             JOIN grammar_patterns gp ON gpo.pattern_id = gp.id
             WHERE gp.pattern_name = ? AND t.episode_id = ?
             ORDER BY gpo.confidence DESC
             {}",
            limit_clause
        );

        let mut stmt = conn.prepare_cached(&query)?;
        let examples_iter = stmt.query_map([pattern_name, &episode_id.to_string()], |row| {
            Ok(row.get::<_, String>(0)?)
        })?;

        let mut examples = Vec::new();
        for example in examples_iter {
            examples.push(example?);
        }

        Ok(examples)
    }

    /// Delete occurrences for a specific episode (useful for reprocessing)
    pub fn delete_by_episode(conn: &Connection, episode_id: i32) -> Result<(), Error> {
        conn.execute(
            "DELETE FROM grammar_pattern_occurrences 
             WHERE transcript_id IN (SELECT id FROM transcripts WHERE episode_id = ?)",
            [episode_id],
        )?;
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

