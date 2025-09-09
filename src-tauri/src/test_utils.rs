//! Centralized test utilities for consistent database setup and test data creation

use crate::db::{
    episode::Episode, show::Show, transcript::Transcript, transcript_database::DbHandler,
};
use tempfile::NamedTempFile;

/// Creates a test database with all tables properly initialized
/// Returns both the temp file (to keep it alive) and the DbHandler
pub fn create_test_db() -> (NamedTempFile, DbHandler) {
    let file = NamedTempFile::new().unwrap();
    let handler = DbHandler::new(file.path().to_str().unwrap()).unwrap();
    handler.create_tables().unwrap();
    (file, handler)
}

/// Creates a test show and inserts it into the database
pub fn create_test_show(handler: &DbHandler, name: &str, show_type: &str) -> Show {
    let mut show = Show::new(name.to_string(), show_type.to_string());
    show.insert(&handler.conn).unwrap();
    show
}

/// Creates a test episode and inserts it into the database
pub fn create_test_episode(
    handler: &DbHandler,
    show: &Show,
    name: &str,
    season: i32,
    episode_number: i32,
) -> Episode {
    let mut episode = Episode::new(show.id.unwrap(), name.to_string(), season, episode_number);
    episode.insert(&handler.conn).unwrap();
    episode
}

/// Creates a test transcript and inserts it into the database
pub fn create_test_transcript(
    handler: &DbHandler,
    episode: &Episode,
    line_id: i32,
    time_start: &str,
    time_end: &str,
    text: &str,
) -> Transcript {
    let mut transcript = Transcript::new(
        episode.id.unwrap(),
        line_id,
        time_start.to_string(),
        time_end.to_string(),
        text.to_string(),
    );
    transcript.insert(&handler.conn).unwrap();
    transcript
}

/// Creates a complete test data hierarchy: show -> episode -> transcript
pub fn create_test_hierarchy(handler: &DbHandler) -> (Show, Episode, Transcript) {
    let show = create_test_show(handler, "Test Show", "Anime");
    let episode = create_test_episode(handler, &show, "Test Episode", 1, 1);
    let transcript = create_test_transcript(
        handler,
        &episode,
        1,
        "00:00:01,000",
        "00:00:05,000",
        "Hello, world!",
    );
    (show, episode, transcript)
}
