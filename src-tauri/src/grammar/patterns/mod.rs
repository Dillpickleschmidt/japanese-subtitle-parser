mod n1;
mod n2;
mod n3;
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

    // N3 patterns (intermediate-advanced)
    all_patterns.extend(n3::get_patterns());

    // N2 patterns (upper intermediate)
    all_patterns.extend(n2::get_patterns());

    // N1 patterns (advanced)
    all_patterns.extend(n1::get_patterns());

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

/// Get all patterns from all JLPT levels (for internal use by registry)
pub(crate) fn get_all_patterns() -> Vec<(
    crate::grammar::pattern_matcher::GrammarPattern,
    ConjugationPattern,
    &'static str,
)> {
    let mut all_patterns = Vec::new();
    all_patterns.extend(n5::get_patterns());
    all_patterns.extend(n4::get_patterns());
    all_patterns.extend(n3::get_patterns());
    all_patterns.extend(n2::get_patterns());
    all_patterns.extend(n1::get_patterns());
    all_patterns
}

/// Get the JLPT level for a given pattern name
pub fn get_jlpt_level(pattern_name: &str) -> &'static str {
    crate::grammar::pattern_registry::get_jlpt_level(pattern_name)
}
