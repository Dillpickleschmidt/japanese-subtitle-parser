// src/search.rs

mod complex_search;

pub use complex_search::ComplexSearch;

use crate::db::DbHandler;
use crate::error::Result;
use serde_json::Value as JsonValue;

pub struct SearchEngine {
    complex_search: ComplexSearch,
}

impl SearchEngine {
    pub fn new(db: DbHandler) -> Self {
        let complex_search = ComplexSearch::new(db);
        SearchEngine { complex_search }
    }

    pub fn complex_search(&self, keyword: &str) -> Result<JsonValue> {
        self.complex_search.search(keyword)
    }

    // You can add more search methods here in the future
    // For example:
    // pub fn simple_search(&self, keyword: &str) -> Result<Vec<Transcript>> { ... }
    // pub fn fuzzy_search(&self, keyword: &str) -> Result<JsonValue> { ... }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::index::ReverseIndex;
    use tempfile::NamedTempFile;

    fn create_test_db() -> (NamedTempFile, DbHandler) {
        let file = NamedTempFile::new().unwrap();
        let handler = DbHandler::new(file.path().to_str().unwrap()).unwrap();
        handler.create_tables().unwrap();
        (file, handler)
    }

    #[test]
    fn test_search_engine() {
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

        let search_engine = SearchEngine::new(db);

        // Test the complex search
        let results = search_engine.complex_search("lazy dog").unwrap();
        let results = results.as_array().unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0]["transcript_id"], 2);
        assert_eq!(results[1]["transcript_id"], 3);
    }
}
