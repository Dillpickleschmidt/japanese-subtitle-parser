// Pattern: と2 (quotation particle)
// Structures: Phrase + と + Verb (言う, 思う, 聞く, etc.)
//
// Meaning: Quotation/citation marker, shows what was said/thought/heard
// Examples: 「危ない！」と叫んだ (yelled "Watch out!"), 猫だと思う (think it's a cat)
//
// Note: と is tokenized as 助詞/格助詞/引用 (quotation particle)
// The verb following と is often omitted at sentence end
pub fn to2() -> Vec<TokenMatcher> {
    use std::sync::Arc;

    // Match と as quotation particle
    #[derive(Debug)]
    struct ToQuotationMatcher;
    impl super::Matcher for ToQuotationMatcher {
        fn matches(&self, token: &crate::KagomeToken) -> bool {
            token.surface == "と"
                && token.pos.first().is_some_and(|pos| pos == "助詞")
                && token.pos.get(1).is_some_and(|pos| pos == "格助詞")
                && token.pos.get(2).is_some_and(|pos| pos == "引用")
        }
    }

    vec![TokenMatcher::Custom(Arc::new(ToQuotationMatcher))]
}
