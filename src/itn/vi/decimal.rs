//! Decimal number tagger for Vietnamese.
//!
//! Converts spoken Vietnamese decimals to written form:
//! - "ba phẩy một bốn" → "3.14"
//! - "không phẩy năm" → "0.5"
//! - "âm ba phẩy một bốn" → "-3.14"
//! - "một phẩy năm tỷ" → "1.5 tỷ"
//!
//! Vietnamese uses "phẩy" as the decimal marker. The integer part is read as a
//! whole number; each fractional digit is read individually. The Vietnamese
//! decimal separator in writing is `.`.
//!
//! Scale preservation: only "triệu" (10⁶) and "tỷ" (10⁹) are preserved as
//! written scale suffixes ("một phẩy năm tỷ" → "1.5 tỷ"). Smaller scales like
//! "nghìn" (10³) are folded into digits by the cardinal tagger ("một nghìn" →
//! "1000"), mirroring the French decimal/cardinal convention.

use super::cardinal::words_to_number;

/// Scales preserved as written suffixes by the decimal tagger. "nghìn"/"ngàn"
/// (10³) is intentionally excluded — plain thousands are the cardinal tagger's
/// responsibility ("một nghìn" → "1000", not "1 nghìn").
const PRESERVED_SCALES: &[&str] = &["tỷ", "triệu"];

/// Parse a spoken Vietnamese decimal expression to written form.
pub fn parse(input: &str) -> Option<String> {
    let original = input.trim();
    let input_lower = original.to_lowercase();

    // Scale-suffixed decimals ("một phẩy năm tỷ") are handled first so the
    // scale word is preserved verbatim from the original input.
    if let Some(result) = parse_with_scale(original, &input_lower) {
        return Some(result);
    }

    parse_phay_decimal(&input_lower)
}

/// Parse numbers ending in a preserved scale word (tỷ / triệu).
fn parse_with_scale(original: &str, input_lower: &str) -> Option<String> {
    for &scale in PRESERVED_SCALES {
        if let Some(num_part) = input_lower.strip_suffix(scale) {
            let num_part = num_part.trim();
            // Preserve the original-case scale word for the output.
            let orig_scale = &original[original.len() - scale.len()..];

            if num_part.contains("phẩy") {
                let decimal = parse_phay_decimal(num_part)?;
                return Some(format!("{} {}", decimal, orig_scale));
            }

            // Plain integer + preserved scale ("hai mươi tỷ" → "20 tỷ").
            let n = parse_integer(num_part)?;
            return Some(format!("{} {}", n, orig_scale));
        }
    }

    None
}

/// Parse "X phẩy Y" decimal pattern.
fn parse_phay_decimal(input: &str) -> Option<String> {
    let (is_negative, rest) = if let Some(stripped) = input.strip_prefix("âm ") {
        (true, stripped)
    } else {
        (false, input)
    };

    let (integer_str, decimal_str) = if let Some(stripped) = rest.strip_prefix("phẩy ") {
        // "phẩy X" with no integer part → ",X"
        ("", stripped)
    } else if let Some((lhs, rhs)) = rest.split_once(" phẩy ") {
        (lhs, rhs)
    } else {
        return None;
    };

    let integer_part = if integer_str.is_empty() {
        String::new()
    } else {
        let n = parse_integer(integer_str)?;
        n.to_string()
    };

    let decimal_digits = parse_decimal_digits(decimal_str)?;
    let sign = if is_negative { "-" } else { "" };

    if integer_part.is_empty() {
        Some(format!("{}.{}", sign, decimal_digits))
    } else {
        Some(format!("{}{}.{}", sign, integer_part, decimal_digits))
    }
}

/// Parse the integer part (a Vietnamese number phrase) into an i64.
fn parse_integer(input: &str) -> Option<i64> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }
    words_to_number(trimmed).map(|n| n as i64)
}

/// Parse the fractional part into a digit string. Each token is a digit word;
/// "mốt" / "lăm" / "tư" are accepted as positional variants of 1 / 5 / 4.
fn parse_decimal_digits(input: &str) -> Option<String> {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    let mut result = String::new();

    for token in &tokens {
        if let Some(digit) = digit_word_to_char(token) {
            result.push(digit);
        } else {
            // A compound like "hai mươi" inside the fractional part is unusual
            // but tolerated — emit its decimal digits verbatim.
            let num = words_to_number(token)?;
            for c in num.to_string().chars() {
                result.push(c);
            }
        }
    }

    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}

/// Map a single Vietnamese digit word to its char.
fn digit_word_to_char(word: &str) -> Option<char> {
    match word {
        "không" => Some('0'),
        "một" | "mốt" => Some('1'),
        "hai" => Some('2'),
        "ba" => Some('3'),
        "bốn" | "tư" => Some('4'),
        "năm" | "lăm" => Some('5'),
        "sáu" => Some('6'),
        "bảy" | "bẩy" => Some('7'),
        "tám" => Some('8'),
        "chín" => Some('9'),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!(parse("ba phẩy một bốn"), Some("3.14".to_string()));
        assert_eq!(parse("không phẩy năm"), Some("0.5".to_string()));
        assert_eq!(parse("không phẩy hai sáu"), Some("0.26".to_string()));
    }

    #[test]
    fn test_negative() {
        assert_eq!(parse("âm ba phẩy một bốn"), Some("-3.14".to_string()));
    }

    #[test]
    fn test_with_scale() {
        assert_eq!(parse("một phẩy năm tỷ"), Some("1.5 tỷ".to_string()));
        assert_eq!(parse("hai mươi tỷ"), Some("20 tỷ".to_string()));
    }

    #[test]
    fn test_invalid() {
        assert_eq!(parse("mười"), None);
        assert_eq!(parse("hello"), None);
        assert_eq!(parse(""), None);
    }
}
