mod n4;
mod n5;

use crate::grammar::pattern_matcher::PatternMatcher;
use crate::grammar::types::ConjugationPattern;

/// Create a pattern matcher configured with all JLPT grammar patterns
pub fn create_pattern_matcher() -> PatternMatcher<ConjugationPattern> {
    let mut matcher = PatternMatcher::new();

    // Combine patterns from all JLPT levels
    let mut all_patterns = Vec::new();

    // N5 patterns (fundamental)
    all_patterns.extend(n5::get_patterns());

    // N4 patterns (intermediate)
    all_patterns.extend(n4::get_patterns());

    // Convert to the format expected by PatternMatcher (without jlpt_level string)
    let matcher_patterns: Vec<_> = all_patterns
        .into_iter()
        .map(|(grammar_pattern, conjugation_pattern, _jlpt_level)| {
            (grammar_pattern, conjugation_pattern)
        })
        .collect();

    matcher.add_patterns(matcher_patterns);

    matcher
}

/// Get the JLPT level for a given pattern name
pub fn get_jlpt_level(pattern_name: &str) -> &'static str {
    // Combine all patterns to create a lookup
    let all_patterns: Vec<_> = n5::get_patterns()
        .into_iter()
        .chain(n4::get_patterns().into_iter())
        .collect();

    for (grammar_pattern, _conjugation_pattern, jlpt_level) in all_patterns {
        if grammar_pattern.name == pattern_name {
            return jlpt_level;
        }
    }

    // Default to n5 if not found (shouldn't happen)
    "n5"
}
