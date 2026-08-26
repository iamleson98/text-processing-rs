//! Whitelist TN tagger for Vietnamese.
//!
//! Converts specific Vietnamese written abbreviations to their spoken forms:
//! - "TS." → "tiến sĩ"
//! - "KS." → "kỹ sư"
//! - "GS." → "giáo sư"
//! - "ThS." → "thạc sĩ"
//! - "CN." → "cử nhân"
//! - "ĐH" → "đại học"
//! - "THPT" → "trung học phổ thông"
//!
//! The lookup is case-sensitive to avoid over-firing on common substrings.

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    /// Mapping of Vietnamese written abbreviations to their spoken forms.
    static ref WHITELIST: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        // Academic / professional titles.
        m.insert("TS.", "tiến sĩ");
        m.insert("GS.", "giáo sư");
        m.insert("KS.", "kỹ sư");
        m.insert("ThS.", "thạc sĩ");
        m.insert("CN.", "cử nhân");
        m.insert("PGS.", "phó giáo sư");
        m.insert("BS.", "bác sĩ");
        m.insert("DS.", "dược sĩ");
        m.insert("Luật sư", "luật sư");
        // Educational institutions.
        m.insert("ĐH", "đại học");
        m.insert("THPT", "trung học phổ thông");
        m.insert("THCS", "trung học cơ sở");
        m.insert("MN", "mầm non");
        // Common institutional abbreviations.
        m.insert("UBND", "ủy ban nhân dân");
        m.insert("TW", "trung ương");
        m.insert("TP.", "thành phố");
        m.insert("TP", "thành phố");
        m.insert("P.", "phường");
        m.insert("Q.", "quận");
        m.insert("TT.", "thị trấn");
        m.insert("X.", "xã");
        m.insert("T.", "tỉnh");
        m
    };
}

/// Convert a whitelisted Vietnamese abbreviation to its spoken form.
pub fn parse(input: &str) -> Option<String> {
    let trimmed = input.trim();
    WHITELIST.get(trimmed).map(|&s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_titles() {
        assert_eq!(parse("TS."), Some("tiến sĩ".to_string()));
        assert_eq!(parse("GS."), Some("giáo sư".to_string()));
        assert_eq!(parse("KS."), Some("kỹ sư".to_string()));
        assert_eq!(parse("ThS."), Some("thạc sĩ".to_string()));
        assert_eq!(parse("BS."), Some("bác sĩ".to_string()));
    }

    #[test]
    fn test_institutions() {
        assert_eq!(parse("ĐH"), Some("đại học".to_string()));
        assert_eq!(parse("THPT"), Some("trung học phổ thông".to_string()));
        assert_eq!(parse("UBND"), Some("ủy ban nhân dân".to_string()));
    }

    #[test]
    fn test_not_whitelisted() {
        assert_eq!(parse("hello"), None);
        assert_eq!(parse("ts."), None); // case-sensitive: lowercase "ts." is not whitelisted
        assert_eq!(parse(""), None);
    }
}
