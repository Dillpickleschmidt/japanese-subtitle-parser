// Consolidated matcher modules organized by JLPT level
mod common_matchers;
mod n2_matchers;
mod n3_matchers;
mod n4_matchers;
mod n5_matchers;

// Export all matchers from consolidated modules
pub use common_matchers::*;
pub use n2_matchers::*;
pub use n3_matchers::*;
pub use n4_matchers::*;
pub use n5_matchers::*;

use crate::types::KagomeToken;
use std::fmt::Debug;

/// Trait for token matching logic
///
/// Each custom matcher implements this trait to provide matching logic for a single token
pub trait TokenMatcherLogic: Debug + Send + Sync {
    /// Check if the token matches this matcher's criteria
    fn matches(&self, token: &KagomeToken) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_interface() {
        // Verify trait can be used as trait object
        let _matcher: &dyn TokenMatcherLogic = &TaiFormMatcher;
    }
}
