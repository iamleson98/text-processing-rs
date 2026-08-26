//! Cardinal TN tagger for Vietnamese.
//!
//! Converts written cardinal numbers to spoken Vietnamese:
//! - "123" → "một trăm hai mươi ba"
//! - "-42" → "âm bốn mươi hai"
//! - "1.000" → "một nghìn" (dot is the Vietnamese thousands separator)
//! - "1,000" → "một nghìn" (comma also accepted as a thousands separator)

use super::number_to_words;

/// Parse a written cardinal number to spoken Vietnamese words.
///
/// Accepts optional leading `-` for negatives and tolerates space / dot / comma
/// as thousands separators (Vietnamese conventionally uses `.`). Returns `None`
/// for any non-digit input so the dispatcher can fall through to other taggers.
pub fn parse(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    let (is_negative, digits_part) = if let Some(rest) = trimmed.strip_prefix('-') {
        (true, rest)
    } else {
        (false, trimmed)
    };

    // Digits with optional thousands separators (space, dot, comma, NBSP).
    if !digits_part
        .chars()
        .all(|c| c.is_ascii_digit() || c == ',' || c == '.' || c == ' ' || c == '\u{a0}')
    {
        return None;
    }
    if !digits_part.chars().any(|c| c.is_ascii_digit()) {
        return None;
    }

    let clean: String = digits_part.chars().filter(|c| c.is_ascii_digit()).collect();
    let n: i64 = clean.parse().ok()?;

    if is_negative {
        Some(format!("âm {}", number_to_words(n)))
    } else {
        Some(number_to_words(n))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(parse("0"), Some("không".to_string()));
        assert_eq!(parse("1"), Some("một".to_string()));
        assert_eq!(parse("5"), Some("năm".to_string()));
        assert_eq!(parse("10"), Some("mười".to_string()));
        assert_eq!(parse("15"), Some("mười lăm".to_string()));
        assert_eq!(parse("21"), Some("hai mươi mốt".to_string()));
        assert_eq!(parse("25"), Some("hai mươi lăm".to_string()));
        assert_eq!(parse("123"), Some("một trăm hai mươi ba".to_string()));
    }

    #[test]
    fn test_thousands_separators() {
        assert_eq!(parse("1.000"), Some("một nghìn".to_string()));
        assert_eq!(parse("1,000"), Some("một nghìn".to_string()));
        assert_eq!(parse("1 000"), Some("một nghìn".to_string()));
        assert_eq!(parse("1.000.000"), Some("một triệu".to_string()));
        assert_eq!(
            parse("2.025"),
            Some("hai nghìn không trăm hai mươi lăm".to_string())
        );
    }

    #[test]
    fn test_negative() {
        assert_eq!(parse("-42"), Some("âm bốn mươi hai".to_string()));
        assert_eq!(parse("-1"), Some("âm một".to_string()));
        assert_eq!(parse("-1000"), Some("âm một nghìn".to_string()));
    }

    #[test]
    fn test_non_numbers() {
        assert_eq!(parse("hello"), None);
        assert_eq!(parse("12abc"), None);
        assert_eq!(parse(""), None);
        assert_eq!(parse("-"), None);
    }
}
