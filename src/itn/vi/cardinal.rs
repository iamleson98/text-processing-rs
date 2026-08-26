//! Cardinal number tagger for Vietnamese.
//!
//! Converts spoken Vietnamese number words to digits:
//! - "không" → "0" (only as part of a larger expression; bare "không" returns None)
//! - "mười" → "10"
//! - "mười lăm" → "15"
//! - "hai mươi mốt" → "21"
//! - "một trăm hai mươi ba" → "123"
//! - "hai nghìn không trăm hai mươi lăm" → "2025"
//! - "âm bốn mươi hai" → "-42"
//!
//! Vietnamese number grammar:
//! - Standalone digit words 0-9 ("không".."chín"), with "mốt" and "lăm" as
//!   positional variants of 1 and 5 in the ones place of a compound number.
//! - "mười" is the literal 10 (and the tens marker for 11-19).
//! - "mươi" is a suffix that promotes the preceding digit to the tens place
//!   (e.g. "hai mươi" = 20).
//! - "trăm" is a suffix that promotes the preceding digit to the hundreds place.
//! - "linh" / "lẻ" is the zero-tens linker ("một trăm linh năm" = 105).
//! - Scales: "nghìn"/"ngàn" (10^3), "triệu" (10^6), "tỷ" (10^9), and the
//!   composed scales "nghìn tỷ", "triệu tỷ", "tỷ tỷ".

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    /// Digit words → value, including the positional variants "mốt" (1 after
    /// "mươi") and "lăm" (5 after "mười"/"mươi").
    static ref DIGIT_WORDS: HashMap<&'static str, i64> = {
        let mut m = HashMap::new();
        m.insert("không", 0);
        m.insert("một", 1);
        m.insert("mốt", 1);
        m.insert("hai", 2);
        m.insert("ba", 3);
        m.insert("bốn", 4);
        m.insert("tư", 4);
        m.insert("năm", 5);
        m.insert("lăm", 5);
        m.insert("sáu", 6);
        m.insert("bảy", 7);
        m.insert("bẩy", 7); // older spelling
        m.insert("tám", 8);
        m.insert("chín", 9);
        m
    };

    /// Scale words → multiplier. The composed scales ("nghìn tỷ", "triệu tỷ",
    /// "tỷ tỷ") are handled separately because they are multi-token.
    static ref SCALES: HashMap<&'static str, i128> = {
        let mut m = HashMap::new();
        m.insert("nghìn", 1_000);
        m.insert("ngàn", 1_000); // Southern variant
        m.insert("triệu", 1_000_000);
        m.insert("tỷ", 1_000_000_000);
        m
    };
}

/// Parse a spoken Vietnamese cardinal expression to its digit string.
///
/// Bare single digits 0-9 return `None` to avoid over-matching inside
/// sentences (the sentence-mode dispatcher should not turn every spoken "một"
/// into "1"). Compounds like "mười", "mười một", "một trăm", "hai nghìn" all
/// parse to digits.
pub fn parse(input: &str) -> Option<String> {
    let input_lower = input.trim().to_lowercase();

    if input_lower.is_empty() {
        return None;
    }

    // Bare standalone digits 0-9 are refused so the sentence-mode scanner
    // doesn't replace every "một" with "1".
    if is_bare_single_digit(&input_lower) {
        return None;
    }

    // Reject space-separated phrases that lack any positional marker or scale
    // word. Without this guard, "hai ba" (two unrelated digit words) would be
    // summed to 5 instead of being left untouched. A leading "âm " is allowed
    // for the bare negative case ("âm một" → -1).
    if input_lower.contains(' ') && !contains_marker_or_scale(&input_lower) {
        let is_bare_negative =
            input_lower.starts_with("âm ") && input_lower.matches(' ').count() == 1;
        if !is_bare_negative {
            return None;
        }
    }

    // Split off a leading "âm " (negative).
    let (is_negative, rest) = if let Some(stripped) = input_lower.strip_prefix("âm ") {
        (true, stripped)
    } else {
        (false, input_lower.as_str())
    };

    let num = words_to_number(rest)?;
    if num == 0 {
        // Reject bare "không"; a real "0" should come from a decimal context.
        return None;
    }

    if is_negative {
        Some(format!("-{}", num))
    } else {
        Some(num.to_string())
    }
}

/// True if the input is exactly one of the standalone digit words (0-9).
fn is_bare_single_digit(input: &str) -> bool {
    matches!(
        input,
        "không"
            | "một"
            | "hai"
            | "ba"
            | "bốn"
            | "tư"
            | "năm"
            | "lăm"
            | "sáu"
            | "bảy"
            | "bẩy"
            | "tám"
            | "chín"
            | "mốt"
    )
}

/// True if the phrase contains a positional marker (`mười`, `mươi`, `trăm`,
/// `linh`, `lẻ`) or a scale word (`nghìn`, `ngàn`, `triệu`, `tỷ`). Phrases
/// that lack both are not valid Vietnamese number compounds and should not be
/// parsed by [`parse`].
fn contains_marker_or_scale(input: &str) -> bool {
    const MARKERS: &[&str] = &[
        "mười", "mươi", "trăm", "linh", "lẻ", "nghìn", "ngàn", "triệu", "tỷ",
    ];
    input.split_whitespace().any(|t| MARKERS.contains(&t))
}

/// Convert spoken Vietnamese number words to an integer.
///
/// Recognises the scales `nghìn`/`ngàn` (10³), `triệu` (10⁶), `tỷ` (10⁹) and
/// the composed scales `nghìn tỷ` (10¹²), `triệu tỷ` (10¹⁵), `tỷ tỷ` (10¹⁸).
/// Positional markers `trăm`, `mươi`, `mười`, and the zero-tens linker
/// `linh`/`lẻ` are interpreted positionally against the running `current`
/// chunk.
pub fn words_to_number(input: &str) -> Option<i128> {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.is_empty() {
        return None;
    }

    let mut result: i128 = 0;
    let mut current: i128 = 0;
    // The last digit word seen but not yet committed to `current`. We hold it
    // back so that a following `trăm` / `mươi` can promote it to the hundreds
    // or tens place.
    let mut pending: Option<i64> = None;

    let flush_pending = |pending: &mut Option<i64>, current: &mut i128| {
        if let Some(d) = pending.take() {
            *current += d as i128;
        }
    };

    let mut i = 0;
    while i < tokens.len() {
        let token = tokens[i];

        // Composed scales: "nghìn tỷ" / "ngàn tỷ", "triệu tỷ", "tỷ tỷ".
        if (token == "nghìn" || token == "ngàn") && i + 1 < tokens.len() && tokens[i + 1] == "tỷ"
        {
            flush_pending(&mut pending, &mut current);
            if current == 0 {
                current = 1;
            }
            result += current * 1_000_000_000_000;
            current = 0;
            i += 2;
            continue;
        }
        if token == "triệu" && i + 1 < tokens.len() && tokens[i + 1] == "tỷ" {
            flush_pending(&mut pending, &mut current);
            if current == 0 {
                current = 1;
            }
            result += current * 1_000_000_000_000_000;
            current = 0;
            i += 2;
            continue;
        }
        if token == "tỷ" && i + 1 < tokens.len() && tokens[i + 1] == "tỷ" {
            flush_pending(&mut pending, &mut current);
            if current == 0 {
                current = 1;
            }
            result += current * 1_000_000_000_000_000_000;
            current = 0;
            i += 2;
            continue;
        }

        // Single-token scales: nghìn / ngàn / triệu / tỷ.
        if let Some(&scale) = SCALES.get(token) {
            flush_pending(&mut pending, &mut current);
            if current == 0 {
                current = 1; // "nghìn" alone = 1000
            }
            result += current * scale;
            current = 0;
            i += 1;
            continue;
        }

        // "trăm" — promote the pending digit to the hundreds place. If there is
        // no pending digit, assume 1 ("trăm" alone = 100, though rare).
        if token == "trăm" {
            let hundreds_digit = pending.take().unwrap_or(1);
            current += (hundreds_digit as i128) * 100;
            i += 1;
            continue;
        }

        // "mươi" — promote the pending digit to the tens place.
        if token == "mươi" {
            let tens_digit = pending.take().unwrap_or(1);
            current += (tens_digit as i128) * 10;
            i += 1;
            continue;
        }

        // "mười" — literal 10 (tens place = 1) and start of the 11-19 range.
        if token == "mười" {
            // Any pending digit is committed as a ones-place value first; this
            // shouldn't normally happen in well-formed input but keeps the
            // parser robust.
            flush_pending(&mut pending, &mut current);
            current += 10;
            i += 1;
            continue;
        }

        // Zero-tens linker: "linh" / "lẻ" — commits any pending digit and is
        // otherwise a no-op (the next digit will be the ones place).
        if token == "linh" || token == "lẻ" {
            flush_pending(&mut pending, &mut current);
            i += 1;
            continue;
        }

        // Plain digit word — stash it as pending so a following `trăm`/`mươi`
        // can promote it, otherwise it will be committed as a ones-place digit.
        if let Some(&d) = DIGIT_WORDS.get(token) {
            // If we already have a pending digit, commit it as a ones-place
            // digit before stashing the new one. This handles sequences like
            // "hai ba" (which is not standard but should not silently corrupt).
            if pending.is_some() {
                flush_pending(&mut pending, &mut current);
            }
            pending = Some(d);
            i += 1;
            continue;
        }

        // Unknown token — give up.
        return None;
    }

    flush_pending(&mut pending, &mut current);
    result += current;

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(parse("mười"), Some("10".to_string()));
        assert_eq!(parse("mười lăm"), Some("15".to_string()));
        assert_eq!(parse("hai mươi"), Some("20".to_string()));
        assert_eq!(parse("hai mươi mốt"), Some("21".to_string()));
        assert_eq!(parse("hai mươi lăm"), Some("25".to_string()));
    }

    #[test]
    fn test_hundreds() {
        assert_eq!(parse("một trăm"), Some("100".to_string()));
        assert_eq!(parse("một trăm linh năm"), Some("105".to_string()));
        assert_eq!(parse("một trăm lẻ năm"), Some("105".to_string()));
        assert_eq!(parse("một trăm hai mươi ba"), Some("123".to_string()));
    }

    #[test]
    fn test_thousands() {
        assert_eq!(parse("một nghìn"), Some("1000".to_string()));
        assert_eq!(parse("một ngàn"), Some("1000".to_string()));
        assert_eq!(
            parse("hai nghìn không trăm hai mươi lăm"),
            Some("2025".to_string())
        );
    }

    #[test]
    fn test_large() {
        assert_eq!(parse("một triệu"), Some("1000000".to_string()));
        assert_eq!(parse("một tỷ"), Some("1000000000".to_string()));
        assert_eq!(parse("một nghìn tỷ"), Some("1000000000000".to_string()));
        assert_eq!(
            parse("hai tỷ ba triệu bốn nghìn năm"),
            Some("2003004005".to_string())
        );
    }

    #[test]
    fn test_negative() {
        assert_eq!(parse("âm bốn mươi hai"), Some("-42".to_string()));
        assert_eq!(parse("âm một"), Some("-1".to_string()));
    }

    #[test]
    fn test_bare_digits_rejected() {
        // Single digits 0-9 are refused to avoid over-matching in sentences.
        assert_eq!(parse("không"), None);
        assert_eq!(parse("một"), None);
        assert_eq!(parse("hai"), None);
        assert_eq!(parse("chín"), None);
        // Positional variants are also rejected bare.
        assert_eq!(parse("mốt"), None);
        assert_eq!(parse("lăm"), None);
    }

    #[test]
    fn test_invalid() {
        assert_eq!(parse("hello"), None);
        assert_eq!(parse(""), None);
        assert_eq!(parse("một trăm xyz"), None);
    }

    #[test]
    fn test_words_to_number_directly() {
        assert_eq!(words_to_number("không"), Some(0));
        assert_eq!(words_to_number("mười"), Some(10));
        assert_eq!(words_to_number("chín trăm chín mươi chín"), Some(999));
        assert_eq!(
            words_to_number("một tỷ tỷ"),
            Some(1_000_000_000_000_000_000)
        );
    }
}
