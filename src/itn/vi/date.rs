//! Date tagger for Vietnamese.
//!
//! Converts spoken Vietnamese date expressions to written form:
//! - "năm tháng một hai nghìn không trăm hai mươi lăm" → "5/1/2025"
//! - "mười bốn tháng bảy" → "14/7"
//! - "mùng một tháng một" → "1/1" (colloquial "mùng" for day-of-month)
//! - "mười bốn tháng bảy năm hai nghìn mười ba" → "14/7/2013" (explicit "năm")
//!
//! Vietnamese dates spoken form: "[ngày/mùng] <day> tháng <month> [năm <year>]"
//! Written form: "DD/MM/YYYY" (or "DD/MM" if no year). The year separator
//! "năm" is optional when the year is unambiguous (e.g. "hai nghìn...").

use super::cardinal::words_to_number;

/// Minimum value accepted as a year when no explicit "năm" separator is
/// present. Without this threshold, "mười tháng mười ba" would split as
/// month=10 / year=3 instead of being rejected for the invalid month 13.
const MIN_IMPLICIT_YEAR: i64 = 100;

/// Parse a spoken Vietnamese date expression to written form.
pub fn parse(input: &str) -> Option<String> {
    let input_lower = input.trim().to_lowercase();

    // Must contain "tháng" to be a date.
    let month_pos = input_lower.find("tháng ")?;
    let day_part = input_lower[..month_pos].trim();
    let after_month = input_lower[month_pos + "tháng ".len()..].trim();

    // Day may be prefixed with "ngày " or "mùng " (colloquial); strip both.
    let day_str = day_part
        .strip_prefix("ngày ")
        .or_else(|| day_part.strip_prefix("mùng "))
        .unwrap_or(day_part);

    let day = words_to_number(day_str)? as i64;
    if !(1..=31).contains(&day) {
        return None;
    }

    let (month, year) = split_month_year(after_month)?;
    if !(1..=12).contains(&month) {
        return None;
    }

    match year {
        Some(y) if y > 0 => Some(format!("{}/{}/{}", day, month, y)),
        _ => Some(format!("{}/{}", day, month)),
    }
}

/// Split the "month [year]" portion of a spoken date.
///
/// Tries the explicit "năm" separator first; falls back to scanning token
/// split points for a valid (month, year) pair where month ∈ 1..=12 and the
/// year (if present) is ≥ [`MIN_IMPLICIT_YEAR`].
fn split_month_year(after_month: &str) -> Option<(i64, Option<i64>)> {
    // Explicit "năm" separator: "<month> năm <year>".
    if let Some(year_pos) = after_month.find(" năm ") {
        let m_str = after_month[..year_pos].trim();
        let y_str = after_month[year_pos + " năm ".len()..].trim();
        let m = words_to_number(m_str)? as i64;
        let y = words_to_number(y_str)? as i64;
        return Some((m, Some(y)));
    }

    // No explicit separator — try every possible split point, shortest month
    // first. The month must parse to 1..=12; the year (if any) must parse to
    // a value ≥ MIN_IMPLICIT_YEAR so we don't grab unrelated trailing tokens.
    let tokens: Vec<&str> = after_month.split_whitespace().collect();
    if tokens.is_empty() {
        return None;
    }

    for split in 1..=tokens.len() {
        let m_str = tokens[..split].join(" ");
        let y_str = tokens[split..].join(" ");
        let m = words_to_number(&m_str)? as i64;
        if !(1..=12).contains(&m) {
            continue;
        }
        if y_str.is_empty() {
            return Some((m, None));
        }
        if let Some(y) = words_to_number(&y_str) {
            let y = y as i64;
            if y >= MIN_IMPLICIT_YEAR {
                return Some((m, Some(y)));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_month_year() {
        assert_eq!(
            parse("năm tháng một hai nghìn không trăm hai mươi lăm"),
            Some("5/1/2025".to_string())
        );
        assert_eq!(
            parse("mười bốn tháng bảy hai nghìn mười ba"),
            Some("14/7/2013".to_string())
        );
        assert_eq!(
            parse("mười bốn tháng bảy năm hai nghìn mười ba"),
            Some("14/7/2013".to_string())
        );
    }

    #[test]
    fn test_day_month() {
        assert_eq!(parse("mười bốn tháng bảy"), Some("14/7".to_string()));
        assert_eq!(parse("mùng một tháng một"), Some("1/1".to_string()));
        assert_eq!(parse("ngày mười tháng mười"), Some("10/10".to_string()));
        assert_eq!(parse("mười một tháng mười hai"), Some("11/12".to_string()));
    }

    #[test]
    fn test_invalid() {
        assert_eq!(parse("hello"), None);
        assert_eq!(parse("mười bốn"), None); // no "tháng"
        assert_eq!(parse("bốn mươi tháng một"), None); // bad day
        assert_eq!(parse("mười tháng mười ba"), None); // bad month (13)
    }
}
