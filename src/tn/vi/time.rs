//! Time TN tagger for Vietnamese.
//!
//! Converts written times to spoken Vietnamese:
//! - "14:30" → "mười bốn giờ ba mươi"
//! - "8:00" → "tám giờ"
//! - "0:00" → "nửa đêm"
//! - "12:00" → "trưa"
//! - "9:05" → "chín giờ linh năm"
//! - "14:30:00" → "mười bốn giờ ba mươi phút"

use super::number_to_words;

/// Parse a written time (`H:M` or `H:M:S`) to spoken Vietnamese.
pub fn parse(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    // Split off a leading sign or country code is not expected here.
    let parts: Vec<&str> = trimmed.split(':').collect();
    if parts.len() < 2 || parts.len() > 3 {
        return None;
    }

    let hour: u32 = parts[0].parse().ok()?;
    let minute: u32 = parts[1].parse().ok()?;
    let second: u32 = if parts.len() == 3 {
        parts[2].parse().ok()?
    } else {
        0
    };

    if hour > 23 || minute > 59 || second > 59 {
        return None;
    }

    // Special cases: midnight (00:00) and noon (12:00).
    if hour == 0 && minute == 0 && second == 0 {
        return Some("nửa đêm".to_string());
    }
    if hour == 12 && minute == 0 && second == 0 {
        return Some("trưa".to_string());
    }

    let mut result = String::new();
    result.push_str(&number_to_words(hour as i64));
    result.push_str(" giờ");

    if minute > 0 {
        result.push(' ');
        if minute < 10 {
            // 9:05 → "chín giờ linh năm"
            result.push_str("linh ");
            result.push_str(&number_to_words(minute as i64));
        } else {
            result.push_str(&number_to_words(minute as i64));
        }
    }

    if second > 0 {
        result.push_str(" phút ");
        result.push_str(&number_to_words(second as i64));
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(parse("14:30"), Some("mười bốn giờ ba mươi".to_string()));
        assert_eq!(parse("8:00"), Some("tám giờ".to_string()));
        assert_eq!(parse("9:05"), Some("chín giờ linh năm".to_string()));
    }

    #[test]
    fn test_special() {
        assert_eq!(parse("0:00"), Some("nửa đêm".to_string()));
        assert_eq!(parse("12:00"), Some("trưa".to_string()));
    }

    #[test]
    fn test_with_seconds() {
        assert_eq!(
            parse("14:30:15"),
            Some("mười bốn giờ ba mươi phút mười lăm".to_string())
        );
    }

    #[test]
    fn test_invalid() {
        assert_eq!(parse("25:00"), None);
        assert_eq!(parse("12:60"), None);
        assert_eq!(parse("hello"), None);
        assert_eq!(parse(""), None);
    }
}
