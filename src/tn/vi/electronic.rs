//! Electronic TN tagger for Vietnamese.
//!
//! Converts emails and URLs to spoken Vietnamese:
//! - "test@gmail.com" → "t e s t a còng g m a i l chấm c o m"
//! - "https://example.com" → "h t t p s dấu hai chấm gạch chéo gạch chéo e x a m p l e chấm c o m"
//!
//! Vietnamese spoken symbols:
//! - `@` → "a còng"
//! - `.` → "chấm"
//! - `:` → "dấu hai chấm"
//! - `/` → "gạch chéo"
//! - `-` → "gạch ngang"
//! - `_` → "gạch dưới"

/// Spoken Vietnamese name for a single ASCII character used in electronic
/// addresses. Letters are spelled as themselves (lowercase), digits are
/// spelled via the cardinal digit words, and symbols get their Vietnamese name.
fn spell_char(c: char) -> String {
    match c {
        '@' => "a còng".to_string(),
        '.' => "chấm".to_string(),
        ':' => "dấu hai chấm".to_string(),
        '/' => "gạch chéo".to_string(),
        '-' => "gạch ngang".to_string(),
        '_' => "gạch dưới".to_string(),
        '~' => "ngã".to_string(),
        '#' => "dấu thăng".to_string(),
        ' ' => "cách".to_string(),
        c if c.is_ascii_alphabetic() => c.to_ascii_lowercase().to_string(),
        c if c.is_ascii_digit() => super::spell_digits(&c.to_string()),
        // Any other character is spelled verbatim (lowercased).
        c => c.to_lowercase().to_string(),
    }
}

/// True when the input looks like an email, URL, or other electronic address.
///
/// A bare `.` or `/` is NOT enough — those appear in decimal numbers ("3.14")
/// and dates ("5/1/2025"). We require an unambiguous electronic marker: `@`,
/// `://`, `www.`, or a dot/slash combined with at least one alphabetic character
/// on each side (so `test.com` matches but `3.14` does not).
fn looks_electronic(input: &str) -> bool {
    if input.contains('@') || input.contains("://") || input.contains("www.") {
        return true;
    }
    // "word.word" pattern (e.g. "example.com", "test.org") — but NOT "3.14"
    // or "1.000". Require at least one alpha on each side of a dot.
    if input.contains('.') && has_alpha_dot_alpha(input) {
        return true;
    }
    // "word/word" pattern (e.g. "example/path") — but NOT "5/1/2025".
    if input.contains('/') && has_alpha_slash_alpha(input) {
        return true;
    }
    false
}

/// True if the input contains a dot with at least one alphabetic character on
/// each side (e.g. "test.com" → true, "3.14" → false, "1.000" → false).
fn has_alpha_dot_alpha(input: &str) -> bool {
    let bytes = input.as_bytes();
    for i in 1..bytes.len().saturating_sub(1) {
        if bytes[i] == b'.'
            && bytes[i - 1].is_ascii_alphabetic()
            && bytes[i + 1].is_ascii_alphabetic()
        {
            return true;
        }
    }
    false
}

/// True if the input contains a slash with at least one alphabetic character on
/// each side (e.g. "example/path" → true, "5/1/2025" → false).
fn has_alpha_slash_alpha(input: &str) -> bool {
    let bytes = input.as_bytes();
    for i in 1..bytes.len().saturating_sub(1) {
        if bytes[i] == b'/'
            && bytes[i - 1].is_ascii_alphabetic()
            && bytes[i + 1].is_ascii_alphabetic()
        {
            return true;
        }
    }
    false
}

/// Parse an electronic address into spoken Vietnamese.
pub fn parse(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() || !looks_electronic(trimmed) {
        return None;
    }

    let spelled: Vec<String> = trimmed.chars().map(spell_char).collect();
    Some(spelled.join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email() {
        assert_eq!(
            parse("test@gmail.com"),
            Some("t e s t a còng g m a i l chấm c o m".to_string())
        );
    }

    #[test]
    fn test_url() {
        assert_eq!(
            parse("https://example.com"),
            Some("h t t p s dấu hai chấm gạch chéo gạch chéo e x a m p l e chấm c o m".to_string())
        );
    }

    #[test]
    fn test_with_digits() {
        assert_eq!(
            parse("a1b2@example.com"),
            Some("a một b hai a còng e x a m p l e chấm c o m".to_string())
        );
    }

    #[test]
    fn test_invalid() {
        assert_eq!(parse("hello"), None);
        assert_eq!(parse(""), None);
    }

    #[test]
    fn test_not_hijacking_decimals_and_dates() {
        // Pure decimals and dates must NOT be matched by electronic — the
        // decimal and date taggers own these patterns.
        assert_eq!(parse("3.14"), None);
        assert_eq!(parse("-3.14"), None);
        assert_eq!(parse("1.000"), None);
        assert_eq!(parse("5/1/2025"), None);
        assert_eq!(parse("14/7"), None);
    }
}
