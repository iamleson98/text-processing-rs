//! Decimal TN tagger for Vietnamese.
//!
//! Converts written decimal numbers to spoken Vietnamese:
//! - "3.14" → "ba phẩy một bốn"
//! - "3,14" → "ba phẩy một bốn" (comma also accepted as a decimal separator)
//! - "0.5" → "không phẩy năm"
//! - "-3.14" → "âm ba phẩy một bốn"
//!
//! Vietnamese reads the integer part as a whole number and each fractional digit
//! individually, joined by the decimal marker "phẩy".

use super::{number_to_words, spell_digits};

/// Vietnamese quantity suffixes recognized after a decimal number.
const QUANTITY_SUFFIXES: &[&str] = &[
    "tỷ tỷ",
    "triệu tỷ",
    "nghìn tỷ",
    "tỷ",
    "triệu",
    "nghìn",
    "ngàn",
];

/// Parse a written decimal number to spoken Vietnamese.
pub fn parse(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    let (number_part, suffix) = extract_suffix(trimmed);

    // Vietnamese uses `.` as the decimal separator. `,` is also accepted so the
    // tagger is tolerant of either convention. Thousands-grouped numbers
    // ("1.000", "1,000", "1.000.000") are rejected so the cardinal tagger can
    // handle them.
    let sep = if number_part.contains('.') && !is_thousands_grouped(number_part) {
        '.'
    } else if number_part.contains(',') && !is_thousands_grouped(number_part) {
        ','
    } else {
        return None;
    };

    let parts: Vec<&str> = number_part.splitn(2, sep).collect();
    if parts.len() != 2 {
        return None;
    }

    let int_str = parts[0];
    let frac_str = parts[1];

    let (is_negative, int_digits) = if let Some(rest) = int_str.strip_prefix('-') {
        (true, rest)
    } else {
        (false, int_str)
    };

    if !int_digits.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    if frac_str.is_empty() || !frac_str.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    let int_val: i64 = if int_digits.is_empty() {
        0
    } else {
        int_digits.parse().ok()?
    };

    let int_words = number_to_words(int_val);
    let frac_words = spell_digits(frac_str);

    let mut result = if is_negative {
        format!("âm {} phẩy {}", int_words, frac_words)
    } else {
        format!("{} phẩy {}", int_words, frac_words)
    };

    if let Some(suf) = suffix {
        result.push(' ');
        result.push_str(suf);
    }

    Some(result)
}

/// True if the number uses `.` or `,` as thousands separators (every group
/// after the first is exactly 3 digits). Such numbers are cardinals, not
/// decimals, and should be rejected so the cardinal tagger handles them.
///
/// Examples:
/// - `"1.000"` → true (groups [1, 000])
/// - `"1,000"` → true (groups [1, 000])
/// - `"1.000.000"` → true (groups [1, 000, 000])
/// - `"3.14"` → false (last group is 2 digits)
/// - `"100.05"` → false (last group is 2 digits)
/// - `"3.14159"` → false (last group is 5 digits)
fn is_thousands_grouped(s: &str) -> bool {
    let groups: Vec<&str> = s.split(['.', ',']).collect();
    if groups.len() < 2 {
        return false;
    }
    // The first group can be 1-3 digits (e.g. "1" or "123").
    // Every subsequent group must be exactly 3 digits.
    groups[1..]
        .iter()
        .all(|g| g.len() == 3 && g.chars().all(|c| c.is_ascii_digit()))
}

/// Extract a quantity suffix from the end if present.
fn extract_suffix(input: &str) -> (&str, Option<&str>) {
    for &suf in QUANTITY_SUFFIXES {
        if let Some(before) = input.strip_suffix(suf) {
            let before = before.trim_end();
            if !before.is_empty() {
                return (before, Some(suf));
            }
        }
    }
    (input, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot_decimal() {
        assert_eq!(parse("3.14"), Some("ba phẩy một bốn".to_string()));
        assert_eq!(parse("0.5"), Some("không phẩy năm".to_string()));
        assert_eq!(parse("0.26"), Some("không phẩy hai sáu".to_string()));
    }

    #[test]
    fn test_comma_decimal() {
        assert_eq!(parse("3,14"), Some("ba phẩy một bốn".to_string()));
    }

    #[test]
    fn test_negative() {
        assert_eq!(parse("-3.14"), Some("âm ba phẩy một bốn".to_string()));
    }

    #[test]
    fn test_with_quantity() {
        assert_eq!(parse("1.5 tỷ"), Some("một phẩy năm tỷ".to_string()));
        assert_eq!(
            parse("4.85 triệu"),
            Some("bốn phẩy tám năm triệu".to_string())
        );
    }

    #[test]
    fn test_non_decimal() {
        // Pure integers fall through to the cardinal tagger.
        assert_eq!(parse("123"), None);
        assert_eq!(parse("hello"), None);
        // "1.000" is a thousands-grouped integer, not a decimal.
        assert_eq!(parse("1.000"), None);
    }

    #[test]
    fn test_thousands_grouped_rejected() {
        // Thousands-grouped numbers (dot or comma, 3-digit groups) are cardinals.
        assert_eq!(parse("1.000"), None);
        assert_eq!(parse("1,000"), None);
        assert_eq!(parse("1.000.000"), None);
        assert_eq!(parse("1,000,000"), None);
        assert_eq!(parse("999.000"), None);
        // But genuine decimals are accepted:
        assert_eq!(parse("3.14"), Some("ba phẩy một bốn".to_string()));
        assert_eq!(parse("3,14"), Some("ba phẩy một bốn".to_string()));
        assert_eq!(parse("100.05"), Some("một trăm phẩy không năm".to_string()));
    }
}
