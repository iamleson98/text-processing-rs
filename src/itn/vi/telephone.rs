//! Telephone tagger for Vietnamese.
//!
//! Converts spoken Vietnamese phone numbers to written form:
//! - "không chín không một hai ba bốn năm sáu bảy" → "0901234567"
//!
//! Vietnamese phone numbers are 10-11 digits, always spelled digit-by-digit.
//! The tagger accepts single digit words (0-9) including the positional
//! variants "mốt" (1), "lăm" (5), "tư" (4). Compound number words like
//! "mười" or "hai mươi" are rejected to avoid hijacking cardinal expressions.

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
        // Only accept single digit words. Compound numbers ("mười", "hai mươi")
        // are rejected to prevent the tagger from hijacking cardinal expressions
        // like "hai mươi mốt, ba mươi" (which is "21, 30", not a phone number).
        let d = digit_word_to_char(token)?;
        digits.push(d);
    }

    // Vietnamese phone numbers are 10-11 digits. Require at least 7 to allow
    // partial numbers, but cap at 11 to reject overly long digit sequences.
    if !(7..=11).contains(&digits.len()) {
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
    fn test_with_positional_variants() {
        // "mốt" (1), "lăm" (5), "tư" (4) are accepted as digit variants.
        assert_eq!(
            parse("không một mốt hai ba bốn lăm sáu bảy tám"),
            Some("0112345678".to_string())
        );
        assert_eq!(
            parse("không tư năm sáu bảy tám chín không một hai"),
            Some("0456789012".to_string())
        );
    }

    #[test]
    fn test_compound_numbers_rejected() {
        // Compound number words ("mười", "hai mươi") are rejected to prevent
        // hijacking cardinal expressions like "hai mươi mốt, ba mươi".
        assert_eq!(parse("hai mươi mốt, ba mươi"), None);
        assert_eq!(parse("không chín không mười hai ba bốn năm sáu bảy"), None);
    }

    #[test]
    fn test_too_short() {
        assert_eq!(parse("một hai ba"), None); // only 3 digits
    }

    #[test]
    fn test_too_long() {
        // 12 digits → rejected (Vietnamese phone numbers are max 11 digits).
        assert_eq!(
            parse("không một hai ba bốn năm sáu bảy tám chín không một"),
            None
        );
    }

    #[test]
    fn test_invalid() {
        assert_eq!(parse("hello"), None);
        assert_eq!(parse(""), None);
    }
}
