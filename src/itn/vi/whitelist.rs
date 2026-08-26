//! Whitelist tagger for Vietnamese.
//!
//! Converts specific Vietnamese spoken titles and abbreviations to their
//! written abbreviated forms:
//! - "tiến sĩ" → "TS."
//! - "giáo sư" → "GS."
//! - "kỹ sư" → "KS."
//! - "thạc sĩ" → "ThS."
//! - "bác sĩ" → "BS."
//! - "cử nhân" → "CN."
//! - "đại học" → "ĐH"
//! - "trung học phổ thông" → "THPT"
//! - "ủy ban nhân dân" → "UBND"

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    /// Mapping of Vietnamese spoken forms to their written abbreviations.
    static ref WHITELIST: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        // Academic / professional titles.
        m.insert("tiến sĩ", "TS.");
        m.insert("giáo sư", "GS.");
        m.insert("phó giáo sư", "PGS.");
        m.insert("kỹ sư", "KS.");
        m.insert("thạc sĩ", "ThS.");
        m.insert("bác sĩ", "BS.");
        m.insert("dược sĩ", "DS.");
        m.insert("cử nhân", "CN.");
        // Educational institutions.
        m.insert("đại học", "ĐH");
        m.insert("trung học phổ thông", "THPT");
        m.insert("trung học cơ sở", "THCS");
        m.insert("mầm non", "MN");
        // Common institutional abbreviations.
        m.insert("ủy ban nhân dân", "UBND");
        m.insert("trung ương", "TW");
        m.insert("thành phố", "TP.");
        m.insert("luật sư", "Luật sư");
        m
    };
}

/// Convert a whitelisted Vietnamese spoken form to its written abbreviation.
pub fn parse(input: &str) -> Option<String> {
    let input_lower = input.trim().to_lowercase();
    WHITELIST.get(input_lower.as_str()).map(|&s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_titles() {
        assert_eq!(parse("tiến sĩ"), Some("TS.".to_string()));
        assert_eq!(parse("giáo sư"), Some("GS.".to_string()));
        assert_eq!(parse("kỹ sư"), Some("KS.".to_string()));
        assert_eq!(parse("thạc sĩ"), Some("ThS.".to_string()));
        assert_eq!(parse("bác sĩ"), Some("BS.".to_string()));
    }

    #[test]
    fn test_institutions() {
        assert_eq!(parse("đại học"), Some("ĐH".to_string()));
        assert_eq!(parse("trung học phổ thông"), Some("THPT".to_string()));
        assert_eq!(parse("ủy ban nhân dân"), Some("UBND".to_string()));
    }

    #[test]
    fn test_not_whitelisted() {
        assert_eq!(parse("hello"), None);
        assert_eq!(parse(""), None);
    }
}
