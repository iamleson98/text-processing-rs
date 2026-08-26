//! Fraction tagger for Vietnamese.
//!
//! Converts spoken Vietnamese fractions to written form:
//! - "một phần ba" → "1/3"
//! - "hai phần chín" → "2/9"
//! - "một rưỡi" → "1 1/2" (mixed: "một rưỡi" = "1 and a half")
//! - "nửa" → "1/2" (bare "half")
//!
//! Vietnamese fractions use "phần" (part) as the separator: "tử số phần mẫu số".
//! "rưỡi" is the special word for "half" in the ones place (e.g. "hai rưỡi" = 2½).

use super::cardinal;

/// Parse a spoken Vietnamese fraction to written form.
pub fn parse(input: &str) -> Option<String> {
    let lower = input.trim().to_lowercase();

    // Bare "nửa" (half) → 1/2.
    if lower == "nửa" {
        return Some("1/2".to_string());
    }

    // Mixed fraction: "X rưỡi" → "X 1/2".
    if let Some(whole_part) = lower.strip_suffix(" rưỡi") {
        if let Some(whole) = word_to_int(whole_part) {
            return Some(format!("{} 1/2", whole));
        }
    }

    // "X phần Y" → "X/Y".
    if let Some((lhs, rhs)) = lower.split_once(" phần ") {
        let numer = word_to_int(lhs)?;
        let denom = word_to_int(rhs)?;
        if denom == 0 {
            return None;
        }
        return Some(format!("{}/{}", numer, denom));
    }

    None
}

/// Vietnamese number phrase → integer, treating bare digit words as their value
/// (the cardinal tagger refuses bare single digits to avoid over-matching, but
/// a fraction's numerator or denominator can legitimately be "một", "hai", ...).
fn word_to_int(input: &str) -> Option<i128> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }
    cardinal::words_to_number(trimmed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!(parse("một phần ba"), Some("1/3".to_string()));
        assert_eq!(parse("một phần bốn"), Some("1/4".to_string()));
        assert_eq!(parse("hai phần chín"), Some("2/9".to_string()));
    }

    #[test]
    fn test_bare_half() {
        assert_eq!(parse("nửa"), Some("1/2".to_string()));
    }

    #[test]
    fn test_mixed_half() {
        assert_eq!(parse("một rưỡi"), Some("1 1/2".to_string()));
        assert_eq!(parse("hai rưỡi"), Some("2 1/2".to_string()));
    }

    #[test]
    fn test_invalid() {
        assert_eq!(parse("hello"), None);
        assert_eq!(parse("một"), None);
        assert_eq!(parse("một phần không"), None); // division by zero
        assert_eq!(parse(""), None);
    }
}
