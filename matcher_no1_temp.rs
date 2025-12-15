// Pattern: の1 (nominalization - that which / the one who)
// Structures: Verb + の + は/が
//
// Meaning: Nominalizes verb clauses to use them as nouns
// Examples: 食べるのは (the one that eats), 乗るのが (that which is riding)
//
// Key distinction from N5 の:
// - N5 の: Pronoun replacement (私の "mine") - can stand alone
// - N4 の1: Nominalization (食べるのは) - MUST be followed by は or が
pub fn no1() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match の as dependent noun (nominalizer)
    #[derive(Debug)]
    struct NoNominalizerMatcher;
    impl super::Matcher for NoNominalizerMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "の"
                && token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.pos.get(1).is_some_and(|pos| pos == "非自立")
        }
    }

    // Match は or が following の
    #[derive(Debug)]
    struct WaGaMatcher;
    impl super::Matcher for WaGaMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            (token.surface == "は" || token.surface == "が")
                && token.pos.first().is_some_and(|pos| pos == "助詞")
        }
    }

    vec![
        TokenMatcher::Any, // Match verb, adjective, or auxiliary before の
        TokenMatcher::Custom(Arc::new(NoNominalizerMatcher)),
        TokenMatcher::Custom(Arc::new(WaGaMatcher)),
    ]
}
