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
fn looks_electronic(input: &str) -> bool {
    input.contains('@')
        || input.contains("://")
        || (input.contains("www.") && input.contains('.'))
        || input.contains('.')
        || input.contains('/')
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
}
