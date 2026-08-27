//! Electronic TN tagger for Vietnamese.
//!
//! Converts emails and URLs to spoken Vietnamese:
//! - "test@gmail.com" → "t e s t a còng g m a i l chấm c o m"
//! - "https://example.com" → "h t t p s dấu hai chấm gạch chéo gạch chéo e x a m p l e chấm c o m"
//!
//! Vietnamese spoken symbols:
//! - `@` → "a còng"
//! - `.` → "chấm"
//! - `:` → "dấu hai chấm"
//! - `/` → "gạch chéo"
//! - `-` → "gạch ngang"
//! - `_` → "gạch dưới"

/// Spoken Vietnamese name for a single ASCII character used in electronic
/// addresses. Letters are spelled as themselves (lowercase), digits are
/// spelled via the cardinal digit words, and symbols get their Vietnamese name.
fn spell_char(c: char) -> String {
    match c {
        '@' => "a còng".to_string(),
        '.' => "chấm".to_string(),
        ':' => "dấu hai chấm".to_string(),
        '/' => "gạch chéo".to_string(),
        '-' => "gạch ngang".to_string(),
        '_' => "gạch dưới".to_string(),
        '~' => "ngã".to_string(),
        '#' => "dấu thăng".to_string(),
        ' ' => "cách".to_string(),
        c if c.is_ascii_alphabetic() => c.to_ascii_lowercase().to_string(),
        c if c.is_ascii_digit() => super::spell_digits(&c.to_string()),
        // Any other character is spelled verbatim (lowercased).
        c => c.to_lowercase().to_string(),
    }
}

/// True when the input looks like an email, URL, or other electronic address.
///
/// Conservative rules to avoid hijacking proper nouns and name initials
/// ("L.Taylor", "Mr.John", "St.Louis"):
/// - `@` → email (unambiguous)
/// - `://` → URL scheme (unambiguous)
/// - `www.` → URL prefix (unambiguous)
/// - A dot followed by a known TLD as the last segment (`.com`, `.org`, `.vn`...)
///   → domain name. A bare "word.word" without a known TLD is NOT electronic
///   (it could be a name initial "L.Taylor" or an abbreviation "Mr.John").
fn looks_electronic(input: &str) -> bool {
    if input.contains('@') || input.contains("://") || input.contains("www.") {
        return true;
    }
    // Domain name: the last dot-separated segment must be a known TLD.
    // This matches "example.com" (TLD=com) but not "L.Taylor" (TLD=taylor, unknown).
    if input.contains('.') && has_known_tld(input) {
        return true;
    }
    // "word/word" pattern (e.g. "example/path") — but NOT "5/1/2025".
    // Require at least 2 alpha chars on the left so single letters don't trigger.
    if input.contains('/') && has_alpha_slash_alpha(input) {
        return true;
    }
    false
}

/// Common top-level domains. If the input ends with `.tld` where tld is in
/// this list, it's treated as a domain name. Includes both gTLDs and the
/// Vietnamese ccTLD.
const KNOWN_TLDS: &[&str] = &[
    "com", "org", "net", "edu", "gov", "mil", "int", "info", "biz", "name", "pro", "aero", "coop",
    "museum", "io", "co", "me", "tv", "cc", "ws", "us", "uk", "eu", "de", "fr", "jp", "kr", "cn",
    "ru", "br", "ca", "au", "in", "it", "es", "nl", "se", "no", "fi", "dk", "ch", "at", "be", "pl",
    "cz", "hu", "gr", "pt", "ie", "nz", "sg", "hk", "tw", "th", "my", "id", "ph", "vn", "tr", "sa",
    "ae", "il", "za", "mx", "ar", "cl", "pe", "ve", "ly",
];

/// True if the input ends with a dot followed by a known TLD (case-insensitive).
/// "example.com" → true (ends with ".com")
/// "L.Taylor" → false ("taylor" is not a known TLD)
/// "test@gmail.com" → true (but @ already matched)
fn has_known_tld(input: &str) -> bool {
    let lower = input.to_lowercase();
    // Find the last dot.
    let last_dot = match lower.rfind('.') {
        Some(pos) => pos,
        None => return false,
    };
    let suffix = &lower[last_dot + 1..];
    // The TLD is the text after the last dot, up to the first non-alpha char
    // (handles "example.com/path" → tld="com").
    let tld: String = suffix
        .chars()
        .take_while(|c| c.is_ascii_alphabetic())
        .collect();
    if tld.is_empty() {
        return false;
    }
    KNOWN_TLDS.contains(&tld.as_str())
}

/// True if the input contains a slash with at least one alphabetic character on
/// each side (e.g. "example/path" → true, "5/1/2025" → false).
fn has_alpha_slash_alpha(input: &str) -> bool {
    let bytes = input.as_bytes();
    for i in 1..bytes.len().saturating_sub(1) {
        if bytes[i] == b'/'
            && bytes[i - 1].is_ascii_alphabetic()
            && bytes[i + 1].is_ascii_alphabetic()
        {
            return true;
        }
    }
    false
}

/// Parse an electronic address into spoken Vietnamese.
pub fn parse(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() || !looks_electronic(trimmed) {
        return None;
    }

    let spelled: Vec<String> = trimmed.chars().map(spell_char).collect();
    Some(spelled.join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email() {
        assert_eq!(
            parse("test@gmail.com"),
            Some("t e s t a còng g m a i l chấm c o m".to_string())
        );
    }

    #[test]
    fn test_url() {
        assert_eq!(
            parse("https://example.com"),
            Some("h t t p s dấu hai chấm gạch chéo gạch chéo e x a m p l e chấm c o m".to_string())
        );
    }

    #[test]
    fn test_with_digits() {
        assert_eq!(
            parse("a1b2@example.com"),
            Some("a một b hai a còng e x a m p l e chấm c o m".to_string())
        );
    }

    #[test]
    fn test_invalid() {
        assert_eq!(parse("hello"), None);
        assert_eq!(parse(""), None);
    }

    #[test]
    fn test_not_hijacking_decimals_and_dates() {
        // Pure decimals and dates must NOT be matched by electronic — the
        // decimal and date taggers own these patterns.
        assert_eq!(parse("3.14"), None);
        assert_eq!(parse("-3.14"), None);
        assert_eq!(parse("1.000"), None);
        assert_eq!(parse("5/1/2025"), None);
        assert_eq!(parse("14/7"), None);
    }

    #[test]
    fn test_not_hijacking_name_initials_and_abbreviations() {
        // Name initials and abbreviations with dots must NOT be matched by
        // electronic — they are proper nouns, not domain names.
        assert_eq!(parse("L.Taylor"), None); // name initial
        assert_eq!(parse("Mr.John"), None); // title + name
        assert_eq!(parse("St.Louis"), None); // saint + place
        assert_eq!(parse("Dr.Smith"), None); // doctor + name
        assert_eq!(parse("Mt.Everest"), None); // mount + name
        assert_eq!(parse("Jr.Robert"), None); // junior + name
        assert_eq!(parse("A.B.C"), None); // initials
    }

    #[test]
    fn test_domain_with_known_tld() {
        // Domains ending in a known TLD ARE matched.
        assert!(parse("example.com").is_some());
        assert!(parse("google.org").is_some());
        assert!(parse("school.edu").is_some());
        assert!(parse("site.vn").is_some());
        assert!(parse("page.io").is_some());
    }
}
