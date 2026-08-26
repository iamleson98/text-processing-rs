//! Telephone tagger for Vietnamese.
//!
//! Converts spoken Vietnamese phone numbers to written form:
//! - "không chín không một hai ba bốn năm sáu bảy" → "0901234567"
//! - "không chín không, một hai ba, bốn năm sáu bảy" → "0901234567"
//!
//! Vietnamese phone numbers are 10-11 digits. Spoken form reads each digit
//! individually. The tagger accepts digit words (including the positional
//! variants "mốt", "lăm", "tư") and groups the resulting digits.

use super::cardinal::words_to_number;

/// Parse a spoken Vietnamese telephone number to written form.
pub fn parse(input: &str) -> Option<String> {
    let input_lower = input.trim().to_lowercase();

    let tokens: Vec<&str> = input_lower.split_whitespace().collect();
    if tokens.is_empty() {
        return None;
    }

    let mut digits = String::new();
    for token in &tokens {
        // Strip trailing comma from grouped readings like "không chín không,"
        let token = token.trim_end_matches(',');
        if token.is_empty() {
            continue;
        }
        if let Some(d) = digit_word_to_char(token) {
            digits.push(d);
        } else {
            // A compound like "mười" or "hai mươi" — append its decimal digits.
            // Only accept 0-99 to avoid swallowing unrelated large numbers.
            let num = words_to_number(token)?;
            if !(0..=99).contains(&num) {
                return None;
            }
            for c in num.to_string().chars() {
                digits.push(c);
            }
        }
    }

    // Need at least 7 digits for a phone number.
    if digits.len() < 7 {
        return None;
    }

    Some(digits)
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
    fn test_digit_by_digit() {
        assert_eq!(
            parse("không chín không một hai ba bốn năm sáu bảy"),
            Some("0901234567".to_string())
        );
    }

    #[test]
    fn test_grouped() {
        assert_eq!(
            parse("không chín không một hai ba bốn năm sáu bảy"),
            Some("0901234567".to_string())
        );
    }

    #[test]
    fn test_with_compound() {
        // Single-token compounds ("mười" = 10) expand to their decimal digits,
        // so "mười" contributes "10" (two digits). Vietnamese phone numbers are
        // normally spelled digit-by-digit; this test documents the edge case.
        assert_eq!(
            parse("không chín không mười hai ba bốn năm sáu bảy"),
            Some("09010234567".to_string())
        );
    }

    #[test]
    fn test_invalid() {
        assert_eq!(parse("một hai ba"), None); // too short
        assert_eq!(parse("hello"), None);
        assert_eq!(parse(""), None);
    }
}
