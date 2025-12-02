use crate::PatternMatch;

/// Convert character position to byte position in a UTF-8 string.
/// Useful for extracting substrings when you have character offsets.
pub fn char_pos_to_byte_pos(s: &str, char_pos: usize) -> usize {
    s.char_indices()
        .nth(char_pos)
        .map(|(byte_pos, _)| byte_pos)
        .unwrap_or(s.len())
}

/// Extract the text span covered by a pattern match.
pub fn pattern_text(sentence: &str, pattern: &PatternMatch) -> String {
    let start_byte = char_pos_to_byte_pos(sentence, pattern.start_char as usize);
    let end_byte = char_pos_to_byte_pos(sentence, pattern.end_char as usize);
    sentence[start_byte..end_byte].to_string()
}
