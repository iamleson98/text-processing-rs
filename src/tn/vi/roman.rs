//! Roman numeral TN tagger for Vietnamese.
//!
//! Converts Roman numerals to spoken Vietnamese cardinal words:
//! - "I" → "một"
//! - "II" → "hai"
//! - "IV" → "bốn"
//! - "IX" → "chín"
//! - "X" → "mười"
//! - "XX" → "hai mươi"
//! - "XL" → "bốn mươi"
//! - "C" → "một trăm"
//! - "M" → "một nghìn"
//!
//! Vietnamese uses Roman numerals for chapter/section numbers (chương II,
//! mục III), centuries (thế kỷ XX), and formal outlines. This tagger only
//! matches all-uppercase Roman numeral strings (I, V, X, L, C, D, M) to avoid
//! false positives on regular words.

use super::number_to_words;

/// Parse a Roman numeral string to spoken Vietnamese.
///
/// Only matches strings consisting entirely of valid Roman numeral characters
/// (I, V, X, L, C, D, M) in uppercase. This avoids false positives on regular
/// words like "I" (English pronoun, unlikely in Vietnamese text) or "mix".
pub fn parse(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    // Must be all uppercase ASCII letters (no mixed case, no digits).
    if !trimmed.chars().all(|c| c.is_ascii_uppercase()) {
        return None;
    }

    // Must contain only valid Roman numeral characters.
    if !trimmed
        .chars()
        .all(|c| matches!(c, 'I' | 'V' | 'X' | 'L' | 'C' | 'D' | 'M'))
    {
        return None;
    }

    let n = roman_to_int(trimmed)?;
    if n == 0 || n > 3999 {
        return None;
    }

    Some(number_to_words(n as i64))
}

/// Convert a Roman numeral string to an integer. Returns None for invalid
/// patterns (e.g., "IIII", "VV", "IC", "XM").
fn roman_to_int(s: &str) -> Option<u32> {
    let values: [(char, u32); 7] = [
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ];

    let char_value =
        |c: char| -> Option<u32> { values.iter().find(|(ch, _)| *ch == c).map(|(_, v)| *v) };

    let chars: Vec<char> = s.chars().collect();
    let mut total: u32 = 0;
    let mut prev: u32 = 0;
    let mut repeat_count: u32 = 1; // count consecutive same-value chars

    // Process right-to-left: if current < prev, it's subtractive (IV=4, IX=9...).
    for &c in chars.iter().rev() {
        let val = char_value(c)?;

        if val == prev {
            // Consecutive same-value chars. V, L, D never repeat.
            if matches!(c, 'V' | 'L' | 'D') {
                return None;
            }
            // I, X, C, M can repeat up to 3 times.
            repeat_count += 1;
            if repeat_count > 3 {
                return None;
            }
        } else {
            repeat_count = 1;
        }

        if val < prev {
            // Validate subtractive pair: only I before V/X, X before L/C, C before D/M.
            let valid_subtraction = match (c, prev) {
                ('I', 5) | ('I', 10) => true,     // IV, IX
                ('X', 50) | ('X', 100) => true,   // XL, XC
                ('C', 500) | ('C', 1000) => true, // CD, CM
                _ => false,
            };
            if !valid_subtraction {
                return None;
            }
            total -= val;
        } else {
            total += val;
        }
        prev = val;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(parse("I"), Some("một".to_string()));
        assert_eq!(parse("II"), Some("hai".to_string()));
        assert_eq!(parse("III"), Some("ba".to_string()));
        assert_eq!(parse("IV"), Some("bốn".to_string()));
        assert_eq!(parse("V"), Some("năm".to_string()));
        assert_eq!(parse("VI"), Some("sáu".to_string()));
        assert_eq!(parse("IX"), Some("chín".to_string()));
        assert_eq!(parse("X"), Some("mười".to_string()));
    }

    #[test]
    fn test_tens() {
        assert_eq!(parse("X"), Some("mười".to_string()));
        assert_eq!(parse("XI"), Some("mười một".to_string()));
        assert_eq!(parse("XIV"), Some("mười bốn".to_string()));
        assert_eq!(parse("XIX"), Some("mười chín".to_string()));
        assert_eq!(parse("XX"), Some("hai mươi".to_string()));
        assert_eq!(parse("XL"), Some("bốn mươi".to_string()));
        assert_eq!(parse("L"), Some("năm mươi".to_string()));
        assert_eq!(parse("XC"), Some("chín mươi".to_string()));
        assert_eq!(parse("C"), Some("một trăm".to_string()));
    }

    #[test]
    fn test_hundreds_thousands() {
        assert_eq!(parse("D"), Some("năm trăm".to_string()));
        assert_eq!(parse("CM"), Some("chín trăm".to_string()));
        assert_eq!(parse("M"), Some("một nghìn".to_string()));
        assert_eq!(parse("MM"), Some("hai nghìn".to_string()));
        assert_eq!(
            parse("MMXXV"),
            Some("hai nghìn không trăm hai mươi lăm".to_string())
        );
        assert_eq!(
            parse("MCMXCVIII"),
            Some("một nghìn chín trăm chín mươi tám".to_string())
        );
    }

    #[test]
    fn test_invalid() {
        assert_eq!(parse(""), None);
        assert_eq!(parse("ABC"), None); // not Roman numeral chars
        assert_eq!(parse("ii"), None); // lowercase
        assert_eq!(parse("Ii"), None); // mixed case
        assert_eq!(parse("IIII"), None); // invalid (should be IV)
        assert_eq!(parse("VV"), None); // invalid (V never repeats)
        assert_eq!(parse("IC"), None); // invalid subtractive
        assert_eq!(parse("123"), None); // digits
        assert_eq!(parse("hello"), None); // word
    }
}
