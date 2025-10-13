pub mod matchers;
pub mod pattern_matcher;
pub mod pattern_registry;
pub mod patterns;
pub mod types;

// Most pattern_matcher types are now only used internally
pub use patterns::create_pattern_matcher;

#[cfg(test)]
mod tests;
