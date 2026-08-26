//! Punctuation tagger for Vietnamese.
//!
//! Converts spoken Vietnamese punctuation words to their written symbols:
//! - "chấm" → "."
//! - "phẩy" → ","
//! - "chấm hỏi" → "?"
//! - "chấm than" → "!"
//! - "dấu hai chấm" → ":"
//! - "chấm phẩy" → ";"
//! - "gạch ngang" → "-"
//! - "gạch chéo" → "/"

use lazy_static::lazy_static;

lazy_static! {
    /// Spoken Vietnamese punctuation → written symbol mappings. Multi-word
    /// patterns are listed first so exact-match comparison finds them.
    static ref PUNCTUATION: Vec<(&'static str, &'static str)> = vec![
        // Multi-word patterns first.
        ("chấm hỏi", "?"),
        ("chấm than", "!"),
        ("dấu hai chấm", ":"),
        ("chấm phẩy", ";"),
        ("gạch ngang", "-"),
        ("gạch chéo", "/"),
        ("gạch dưới", "_"),
        ("dấu thăng", "#"),
        ("dấu ngã", "~"),
        ("ngoặc mở", "("),
        ("ngoặc đóng", ")"),
        ("ngoặc vuông mở", "["),
        ("ngoặc vuông đóng", "]"),
        ("ngoặc nhọn mở", "{"),
        ("ngoặc nhọn đóng", "}"),
        ("a còng", "@"),
        ("a cồng", "@"),
        // Single-word patterns.
        ("chấm", "."),
        ("phẩy", ","),
        ("gạch", "-"),
        ("phần trăm", "%"),
        ("phần trăm", "%"),
    ];
}

/// Try to parse spoken Vietnamese punctuation into its written symbol.
pub fn parse(input: &str) -> Option<String> {
    let input_lower = input.trim().to_lowercase();

    for (pattern, symbol) in PUNCTUATION.iter() {
        if input_lower == *pattern {
            return Some(symbol.to_string());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(parse("chấm"), Some(".".to_string()));
        assert_eq!(parse("phẩy"), Some(",".to_string()));
        assert_eq!(parse("dấu hai chấm"), Some(":".to_string()));
        assert_eq!(parse("chấm phẩy"), Some(";".to_string()));
    }

    #[test]
    fn test_multi_word() {
        assert_eq!(parse("chấm hỏi"), Some("?".to_string()));
        assert_eq!(parse("chấm than"), Some("!".to_string()));
        assert_eq!(parse("ngoặc mở"), Some("(".to_string()));
    }

    #[test]
    fn test_symbols() {
        assert_eq!(parse("gạch ngang"), Some("-".to_string()));
        assert_eq!(parse("gạch chéo"), Some("/".to_string()));
        assert_eq!(parse("a còng"), Some("@".to_string()));
        assert_eq!(parse("dấu thăng"), Some("#".to_string()));
    }

    #[test]
    fn test_invalid() {
        assert_eq!(parse("hello"), None);
        assert_eq!(parse(""), None);
    }
}
