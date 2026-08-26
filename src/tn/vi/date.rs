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
    if !(1..=31).contains(&day) || !(1..=12).contains(&month) {
        return None;
    }

    let day_words = number_to_words(day as i64);
    let month_words = number_to_words(month as i64);

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
}
