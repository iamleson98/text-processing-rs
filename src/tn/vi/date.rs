//! Date TN tagger for Vietnamese.
//!
//! Converts written Vietnamese dates to spoken form:
//! - "5/1/2025" → "năm tháng một hai nghìn không trăm hai mươi lăm"
//! - "5-1-2025" → "năm tháng một hai nghìn không trăm hai mươi lăm"
//! - "31/12" → "ba mươi mốt tháng mười hai"
//!
//! Vietnamese dates are typically written `DD/MM/YYYY` (day-month-year) with
//! `/` or `-` separators. The spoken form interleaves "tháng" between the day
//! and month, and "năm" before the year.

use super::number_to_words;

/// Parse a written date (`D/M/Y` or `D/M`) into spoken Vietnamese.
pub fn parse(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    // Accept either `/` or `-` as separators, but not a mix.
    let sep = if trimmed.contains('/') && !trimmed.contains('-') {
        '/'
    } else if trimmed.contains('-') && !trimmed.contains('/') {
        '-'
    } else {
        return None;
    };

    let parts: Vec<&str> = trimmed.split(sep).collect();
    if parts.len() != 2 && parts.len() != 3 {
        return None;
    }

    let day: u32 = parts[0].parse().ok()?;
    let month: u32 = parts[1].parse().ok()?;
    if !(1..=12).contains(&month) {
        return None;
    }

    // Validate day against the month, accounting for leap years in February.
    let year_for_validation: u32 = if parts.len() == 3 {
        parts[2].parse().ok().unwrap_or(0)
    } else {
        0
    };
    if !is_valid_day_for_month(day, month, year_for_validation) {
        return None;
    }

    let day_words = number_to_words(day as i64);
    // Vietnamese months use ordinal "tư" for April (month 4) instead of the
    // cardinal "bốn". All other months use the cardinal form.
    let month_words = if month == 4 {
        "tư".to_string()
    } else {
        number_to_words(month as i64)
    };

    if parts.len() == 3 {
        let year: u32 = parts[2].parse().ok()?;
        if year == 0 {
            return None;
        }
        let year_words = number_to_words(year as i64);
        Some(format!(
            "{} tháng {} {}",
            day_words, month_words, year_words
        ))
    } else {
        Some(format!("{} tháng {}", day_words, month_words))
    }
}

/// Maximum valid day for a given month/year combination.
///
/// When `year == 0` (no year provided in the date), February is allowed up to
/// 29 days so that "29/2" is accepted without knowing the year.
fn max_day_for_month(month: u32, year: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            // Unknown year (year == 0) allows 29; otherwise check leap year.
            if year == 0 || is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

/// True if a Gregorian leap year.
fn is_leap_year(year: u32) -> bool {
    year.is_multiple_of(4) && !year.is_multiple_of(100) || year.is_multiple_of(400)
}

/// True if `day` is valid for `month`/`year`.
fn is_valid_day_for_month(day: u32, month: u32, year: u32) -> bool {
    day >= 1 && day <= max_day_for_month(month, year)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_month_year() {
        assert_eq!(
            parse("5/1/2025"),
            Some("năm tháng một hai nghìn không trăm hai mươi lăm".to_string())
        );
        assert_eq!(
            parse("1/1/2000"),
            Some("một tháng một hai nghìn".to_string())
        );
    }

    #[test]
    fn test_day_month() {
        assert_eq!(
            parse("31/12"),
            Some("ba mươi mốt tháng mười hai".to_string())
        );
    }

    #[test]
    fn test_dash_separator() {
        assert_eq!(
            parse("5-1-2025"),
            Some("năm tháng một hai nghìn không trăm hai mươi lăm".to_string())
        );
    }

    #[test]
    fn test_invalid() {
        assert_eq!(parse("32/1/2025"), None); // bad day
        assert_eq!(parse("5/13/2025"), None); // bad month
        assert_eq!(parse("hello"), None);
        assert_eq!(parse(""), None);
    }

    #[test]
    fn test_invalid_day_for_month() {
        // Feb 31 is never valid.
        assert_eq!(parse("31/02/2025"), None);
        // Feb 30 is never valid.
        assert_eq!(parse("30/02/2025"), None);
        // Feb 29 is invalid in non-leap years (2025 is not a leap year).
        assert_eq!(parse("29/02/2025"), None);
        // Apr 31 is invalid (April has 30 days).
        assert_eq!(parse("31/04/2025"), None);
        // Apr 30 is valid.
        assert_eq!(
            parse("30/04/2025"),
            Some("ba mươi tháng tư hai nghìn không trăm hai mươi lăm".to_string())
        );
    }

    #[test]
    fn test_leap_year() {
        // Feb 29 is valid in leap years.
        assert_eq!(
            parse("29/02/2024"),
            Some("hai mươi chín tháng hai hai nghìn không trăm hai mươi bốn".to_string())
        );
        // Feb 29 is valid in 2000 (divisible by 400).
        assert_eq!(
            parse("29/02/2000"),
            Some("hai mươi chín tháng hai hai nghìn".to_string())
        );
        // Feb 29 is invalid in 1900 (divisible by 100 but not 400).
        assert_eq!(parse("29/02/1900"), None);
    }

    #[test]
    fn test_month_4_uses_tu() {
        // Month 4 uses ordinal "tư" instead of cardinal "bốn".
        assert_eq!(
            parse("1/4/2025"),
            Some("một tháng tư hai nghìn không trăm hai mươi lăm".to_string())
        );
        assert_eq!(parse("15/4"), Some("mười lăm tháng tư".to_string()));
    }
}
