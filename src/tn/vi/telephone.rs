//! Telephone TN tagger for Vietnamese.
//!
//! Converts written Vietnamese phone numbers to spoken form by spelling each
//! digit individually:
//! - "0901234567" → "không chín không một hai ba bốn năm sáu bảy"
//! - "+84901234567" → "tám tư chín không một hai ba bốn năm sáu bảy"
//! - "090-123-4567" → "không chín không một hai ba bốn năm sáu bảy"
//!
//! Vietnamese mobile numbers are 10 digits (leading 0) or 11 digits for some
//! older networks. Landlines are 9-11 digits with a regional prefix. To
//! distinguish phone numbers from large cardinals (e.g. "1000000000" = "một
//! tỷ"), the tagger requires either a visual separator (`-`, `.`, space,
//! parens), a leading `+` (country code), or a leading `0` (Vietnamese
//! domestic convention).

/// Parse a written telephone number to spoken Vietnamese (digit-by-digit).
pub fn parse(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    // Strip an optional leading "+" country-code marker; we still spell its digits.
    let body = trimmed.strip_prefix('+').unwrap_or(trimmed);

    // Accept digits and visual separators (space, dot, hyphen, parens).
    if !body
        .chars()
        .all(|c| c.is_ascii_digit() || c == ' ' || c == '.' || c == '-' || c == '(' || c == ')')
    {
        return None;
    }

    let digits: String = body.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() < 7 {
        return None;
    }

    // Distinguish phone numbers from large cardinals. Vietnamese phone
    // numbers use spaces and hyphens as visual separators (sometimes parens),
    // never dots or commas (those are thousands separators). A pure digit
    // string is a phone only if it starts with "0" (domestic) or the input had
    // a leading "+" (country code).
    let has_phone_separator = body
        .chars()
        .any(|c| c == ' ' || c == '-' || c == '(' || c == ')');
    let has_country_code = trimmed.starts_with('+');
    let starts_with_zero = digits.starts_with('0');
    if !has_phone_separator && !has_country_code && !starts_with_zero {
        return None;
    }

    Some(super::spell_digits(&digits))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mobile() {
        assert_eq!(
            parse("0901234567"),
            Some("không chín không một hai ba bốn năm sáu bảy".to_string())
        );
    }

    #[test]
    fn test_with_country_code() {
        assert_eq!(
            parse("+84901234567"),
            Some("tám bốn chín không một hai ba bốn năm sáu bảy".to_string())
        );
    }

    #[test]
    fn test_with_separators() {
        assert_eq!(
            parse("090-123-4567"),
            Some("không chín không một hai ba bốn năm sáu bảy".to_string())
        );
        assert_eq!(
            parse("090 123 4567"),
            Some("không chín không một hai ba bốn năm sáu bảy".to_string())
        );
    }

    #[test]
    fn test_large_cardinal_not_phone() {
        // Pure digit strings not starting with 0 are cardinals, not phone numbers.
        assert_eq!(parse("1000000000"), None);
        assert_eq!(parse("12345678"), None);
    }

    #[test]
    fn test_invalid() {
        assert_eq!(parse("12345"), None); // too short
        assert_eq!(parse("hello"), None);
        assert_eq!(parse(""), None);
    }
}
