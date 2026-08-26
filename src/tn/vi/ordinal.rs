//! Ordinal TN tagger for Vietnamese.
//!
//! Converts written Vietnamese ordinals to spoken form:
//! - "1st" → "thứ nhất"
//! - "2nd" → "thứ hai"
//! - "3rd" → "thứ ba"
//! - "4th" → "thứ tư"
//! - "10th" → "thứ mười"
//! - "21st" → "thứ hai mươi mốt"
//!
//! Vietnamese ordinals are special: 1-4 have unique words (nhất, hai, ba, tư)
//! while 5 and above reuse the cardinal form. The written English suffixes
//! (`st`/`nd`/`rd`/`th`) are required — bare digits are handled by the
//! cardinal tagger (`"123"` → `"một trăm hai mươi ba"`, not an ordinal).

use super::number_to_words;

/// Vietnamese ordinal words for 1-4 (the four that diverge from cardinals).
/// 4 is "tư" in ordinal context (vs. "bốn" as a cardinal).
fn small_ordinal(n: i64) -> Option<&'static str> {
    match n {
        1 => Some("nhất"),
        2 => Some("hai"),
        3 => Some("ba"),
        4 => Some("tư"),
        _ => None,
    }
}

/// Parse a written ordinal (`Nst`/`Nnd`/`Nrd`/`Nth`) to spoken Vietnamese.
///
/// Returns `None` for bare digit strings so the cardinal tagger can handle
/// them — Vietnamese written form does not distinguish cardinal from ordinal
/// without an explicit suffix, so we require one.
pub fn parse(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    // Require an explicit English ordinal suffix. Without one, this is a bare
    // cardinal and the cardinal tagger should handle it.
    let digits = trimmed
        .strip_suffix("th")
        .or_else(|| trimmed.strip_suffix("st"))
        .or_else(|| trimmed.strip_suffix("nd"))
        .or_else(|| trimmed.strip_suffix("rd"))?;

    if digits.is_empty() || !digits.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let n: i64 = digits.parse().ok()?;
    if n < 1 {
        return None;
    }

    let body = if let Some(small) = small_ordinal(n) {
        small.to_string()
    } else {
        number_to_words(n)
    };

    Some(format!("thứ {}", body))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small() {
        assert_eq!(parse("1st"), Some("thứ nhất".to_string()));
        assert_eq!(parse("2nd"), Some("thứ hai".to_string()));
        assert_eq!(parse("3rd"), Some("thứ ba".to_string()));
        assert_eq!(parse("4th"), Some("thứ tư".to_string()));
    }

    #[test]
    fn test_compound() {
        assert_eq!(parse("5th"), Some("thứ năm".to_string()));
        assert_eq!(parse("10th"), Some("thứ mười".to_string()));
        assert_eq!(parse("15th"), Some("thứ mười lăm".to_string()));
        assert_eq!(parse("21st"), Some("thứ hai mươi mốt".to_string()));
        assert_eq!(parse("25th"), Some("thứ hai mươi lăm".to_string()));
    }

    #[test]
    fn test_bare_digits_rejected() {
        // Bare digit strings are cardinals, not ordinals — defer to cardinal tagger.
        assert_eq!(parse("1"), None);
        assert_eq!(parse("9"), None);
        assert_eq!(parse("123"), None);
    }

    #[test]
    fn test_invalid() {
        assert_eq!(parse("0th"), None);
        assert_eq!(parse("hello"), None);
        assert_eq!(parse(""), None);
    }
}
