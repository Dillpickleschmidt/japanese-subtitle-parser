use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Debug, Clone)]
pub struct PatternMetadata {
    pub jlpt_level: &'static str,
}

#[derive(Debug)]
pub struct PatternRegistry {
    metadata: HashMap<&'static str, PatternMetadata>,
}

impl PatternRegistry {
    /// Build the pattern registry from all JLPT level pattern definitions
    fn build() -> Self {
        let mut metadata = HashMap::new();

        let all_patterns = crate::patterns::get_all_patterns();

        for (grammar_pattern, _conjugation_pattern, jlpt_level) in all_patterns {
            metadata.insert(grammar_pattern.name, PatternMetadata { jlpt_level });
        }

        Self { metadata }
    }

    /// Get JLPT level for a pattern by name
    pub fn get_jlpt_level(&self, pattern_name: &str) -> &'static str {
        self.metadata
            .get(pattern_name)
            .map(|m| m.jlpt_level)
            .unwrap_or("n5") // Default to n5 if not found
    }
}

pub static PATTERN_REGISTRY: LazyLock<PatternRegistry> = LazyLock::new(PatternRegistry::build);

/// Convenience function to get JLPT level for a pattern name
pub fn get_jlpt_level(pattern_name: &str) -> &'static str {
    PATTERN_REGISTRY.get_jlpt_level(pattern_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_initialization() {
        let _registry = &*PATTERN_REGISTRY;
        // Just verify it initializes without panicking
    }

    #[test]
    fn test_get_jlpt_level_known_patterns() {
        // Test N5 patterns
        assert_eq!(get_jlpt_level("te_iru"), "n5");
        assert_eq!(get_jlpt_level("tai_form"), "n5");
        assert_eq!(get_jlpt_level("masu_form"), "n5");

        // Test N4 patterns
        assert_eq!(get_jlpt_level("potential"), "n4");
        assert_eq!(get_jlpt_level("causative"), "n4");

        // Test N3 patterns
        assert_eq!(get_jlpt_level("rashii"), "n3");
        assert_eq!(get_jlpt_level("you_ni_naru"), "n3");
    }
}
