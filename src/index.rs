// src/index.rs

mod reverse_index;

pub use reverse_index::ReverseIndex;

use crate::db::DbHandler;
use crate::error::Result;

/// IndexBuilder provides a way to construct and manage different types of indexes.
pub struct IndexBuilder {
    db: DbHandler,
}

impl IndexBuilder {
    /// Create a new IndexBuilder
    pub fn new(db: DbHandler) -> Self {
        IndexBuilder { db }
    }

    /// Build a reverse index
    pub fn build_reverse_index(&self) -> Result<()> {
        let reverse_index = ReverseIndex::new(self.db.clone());
        reverse_index.build()
    }

    /// Get a ReverseIndex instance
    pub fn get_reverse_index(&self) -> ReverseIndex {
        ReverseIndex::new(self.db.clone())
    }

    // In the future, you might add methods for other types of indexes here
    // For example:
    // pub fn build_full_text_index(&self) -> Result<()> { ... }
    // pub fn get_full_text_index(&self) -> FullTextIndex { ... }
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
    fn test_index_builder() {
        let (_file, db) = create_test_db();

        // Insert test data
        db.get_connection().execute_batch("
            INSERT INTO shows (id, name, show_type) VALUES (1, 'Test Show', 'Anime');
            INSERT INTO episodes (id, show_id, name, season, episode_number) VALUES (1, 1, 'Test Episode', 1, 1);
            INSERT INTO transcripts (id, episode_id, line_id, time_start, time_end, text) VALUES 
                (1, 1, 1, '00:00:01', '00:00:05', 'The quick brown fox'),
                (2, 1, 2, '00:00:06', '00:00:10', 'jumps over the lazy dog');
        ").unwrap();

        let index_builder = IndexBuilder::new(db);

        // Build the reverse index
        index_builder.build_reverse_index().unwrap();

        // Test the reverse index
        let reverse_index = index_builder.get_reverse_index();
        let results = reverse_index.search("quick brown").unwrap();
        assert_eq!(results, vec![1]);

        let results = reverse_index.search("lazy dog").unwrap();
        assert_eq!(results, vec![2]);
    }
}
