//! Ordinal tagger for Vietnamese.
//!
//! Converts spoken Vietnamese ordinals to written form:
//! - "thứ nhất" → "1"
//! - "thứ hai" → "2"
//! - "thứ ba" → "3"
//! - "thứ tư" → "4"
//! - "thứ năm" → "5"
//! - "thứ mười" → "10"
//! - "thứ hai mươi mốt" → "21"
//!
//! Vietnamese ordinals are prefixed with "thứ". 1-4 use unique words (nhất,
//! hai, ba, tư); 5 and above reuse the cardinal form. The written output is a
//! bare digit (Vietnamese does not use English suffixes like "st"/"nd").

use super::cardinal::words_to_number;

/// Parse a spoken Vietnamese ordinal expression to written form (a digit string).
pub fn parse(input: &str) -> Option<String> {
    let input_lower = input.trim().to_lowercase();

    let body = input_lower.strip_prefix("thứ ")?.trim();
    if body.is_empty() {
        return None;
    }

    // Reject positional variants "mốt" and "lăm" as ordinal bodies — they are
    // only valid after "mươi" in cardinal context, never as standalone ordinals.
    // "thứ mốt" / "thứ lăm" are not valid Vietnamese.
    if body == "mốt" || body == "lăm" {
        return None;
    }

    // Special small ordinals 1-4: "nhất" → 1, "hai" → 2, "ba" → 3, "tư" → 4.
    let n: i64 = match body {
        "nhất" => 1,
        "hai" => 2,
        "ba" => 3,
        "tư" => 4,
        // 5 and above reuse the cardinal words.
        other => words_to_number(other)? as i64,
    };

    if n < 1 {
        return None;
    }
    Some(n.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small() {
        assert_eq!(parse("thứ nhất"), Some("1".to_string()));
        assert_eq!(parse("thứ hai"), Some("2".to_string()));
        assert_eq!(parse("thứ ba"), Some("3".to_string()));
        assert_eq!(parse("thứ tư"), Some("4".to_string()));
    }

    #[test]
    fn test_compound() {
        assert_eq!(parse("thứ năm"), Some("5".to_string()));
        assert_eq!(parse("thứ mười"), Some("10".to_string()));
        assert_eq!(parse("thứ mười lăm"), Some("15".to_string()));
        assert_eq!(parse("thứ hai mươi mốt"), Some("21".to_string()));
    }

    #[test]
    fn test_invalid() {
        assert_eq!(parse("mười"), None); // missing "thứ "
        assert_eq!(parse("thứ"), None);
        assert_eq!(parse("hello"), None);
        assert_eq!(parse(""), None);
    }
}
