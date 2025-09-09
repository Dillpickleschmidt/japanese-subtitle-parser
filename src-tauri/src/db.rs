// db.rs
pub mod episode;
pub mod grammar_pattern;
pub mod model;
mod search;
pub mod show;
pub mod transcript;
pub mod transcript_database;
pub mod word;
pub use self::transcript_database::DbHandler;
