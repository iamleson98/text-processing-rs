//! Fraction TN tagger for Vietnamese.
//!
//! Converts written fractions to spoken Vietnamese:
//! - "3/4" → "ba phần tư"
//! - "1/2" → "một phần hai"
//! - "2/3" → "hai phần ba"
//! - "1/100" → "một phần một trăm"
//!
//! Vietnamese fractions use "phần" (part) as the separator: "tử số phần mẫu số".
//! The denominator 4 has a special ordinal form "tư" (like months).
//!
//! This tagger runs BEFORE the date tagger to prevent dates from hijacking
//! fractions. It only matches `N/M` (exactly one slash, two positive integers)
//! where the pattern is a common fraction, not a date. The heuristic:
//! - If M > 31 (can't be a day-of-month), it's a fraction.
//! - If M is a common fraction denominator (2, 3, 4, 5, 6, 8, 10, 16, 20, 100,
//!   1000), it's a fraction.
//! - Otherwise (M in 7, 9, 11, 12, 13..31), defer to the date tagger.

use super::number_to_words;

/// Common fraction denominators that take priority over date interpretation.
/// Months 1-12 that are NOT in this list (7, 9, 11, 12) defer to the date
/// tagger when the numerator is a valid day (1-31).
const FRACTION_DENOMINATORS: &[u32] = &[2, 3, 4, 5, 6, 8, 10, 16, 20, 100, 1000];

/// Parse a written fraction `N/M` to spoken Vietnamese.
pub fn parse(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    // Must be exactly "N/M" (one slash, two parts, no extra content).
    let parts: Vec<&str> = trimmed.split('/').collect();
    if parts.len() != 2 {
        return None;
    }

    let numer: u32 = parts[0].trim().parse().ok()?;
    let denom: u32 = parts[1].trim().parse().ok()?;

    if numer == 0 || denom == 0 {
        return None;
    }

    // Determine if this is a fraction (not a date):
    // - M > 31: can't be a day-of-month → fraction
    // - M in FRACTION_DENOMINATORS: common fraction → fraction
    // - Otherwise: defer to date tagger (M in 7, 9, 11-31 could be a date)
    let is_fraction = denom > 31 || FRACTION_DENOMINATORS.contains(&denom);
    if !is_fraction {
        return None;
    }

    let numer_words = number_to_words(numer as i64);
    // Denominator 4 uses ordinal "tư" (like tháng tư = April).
    let denom_words = if denom == 4 {
        "tư".to_string()
    } else {
        number_to_words(denom as i64)
    };

    Some(format!("{} phần {}", numer_words, denom_words))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_fractions() {
        assert_eq!(parse("1/2"), Some("một phần hai".to_string()));
        assert_eq!(parse("1/3"), Some("một phần ba".to_string()));
        assert_eq!(parse("3/4"), Some("ba phần tư".to_string()));
        assert_eq!(parse("2/5"), Some("hai phần năm".to_string()));
        assert_eq!(parse("1/8"), Some("một phần tám".to_string()));
        assert_eq!(parse("1/100"), Some("một phần một trăm".to_string()));
    }

    #[test]
    fn test_improper_fractions() {
        assert_eq!(parse("4/3"), Some("bốn phần ba".to_string()));
        assert_eq!(parse("5/2"), Some("năm phần hai".to_string()));
    }

    #[test]
    fn test_large_denominator() {
        // M > 31 → fraction (can't be a date).
        assert_eq!(parse("1/50"), Some("một phần năm mươi".to_string()));
        assert_eq!(parse("3/100"), Some("ba phần một trăm".to_string()));
    }

    #[test]
    fn test_date_like_not_fraction() {
        // M in 7, 9, 11, 12, 13-31 with valid day → defer to date tagger.
        assert_eq!(parse("3/7"), None); // could be 3 July
        assert_eq!(parse("15/9"), None); // could be 15 Sep
        assert_eq!(parse("31/12"), None); // could be 31 Dec
        assert_eq!(parse("1/11"), None); // could be 1 Nov
    }

    #[test]
    fn test_invalid() {
        assert_eq!(parse("0/4"), None); // zero numerator
        assert_eq!(parse("3/0"), None); // zero denominator
        assert_eq!(parse("3"), None); // no slash
        assert_eq!(parse("3/4/5"), None); // two slashes
        assert_eq!(parse("hello"), None);
        assert_eq!(parse(""), None);
    }
}
