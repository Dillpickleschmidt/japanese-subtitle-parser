// Internal implementation modules
mod matchers;
mod pattern_matcher;
mod pattern_registry;
mod patterns;

// Public API
pub mod types;

// Re-export the main entry point
pub use patterns::create_pattern_matcher;

// Re-export types needed by consumers
pub use pattern_matcher::{PatternCategory, PatternMatch, PatternMatcher};
pub use pattern_registry::get_jlpt_level;
