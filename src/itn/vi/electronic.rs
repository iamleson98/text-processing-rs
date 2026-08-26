//! Electronic tagger for Vietnamese.
//!
//! Converts spoken Vietnamese electronic addresses to written form:
//! - "t e s t a còng g m a i l chấm c o m" → "test@gmail.com"
//! - "a một b hai a còng e x a m p l e chấm c o m" → "a1b2@example.com"
//!
//! Vietnamese spoken symbols:
//! - "a còng" / "a cồng" → @
//! - "chấm" → .
//! - "dấu hai chấm" → :
//! - "gạch chéo" → /
//! - "gạch ngang" → -
//! - "gạch dưới" → _
//! - "dấu thăng" → #

use super::cardinal::words_to_number;

/// Multi-word symbols (longest first) and the single-character symbols that
/// follow. Order matters because "a còng" must be matched before bare "a".
const MULTI_WORD_SYMBOLS: &[(&str, char)] = &[
    ("a còng", '@'),
    ("a cồng", '@'),
    ("dấu hai chấm", ':'),
    ("gạch chéo", '/'),
    ("gạch ngang", '-'),
    ("gạch dưới", '_'),
    ("dấu thăng", '#'),
    ("dấu ngã", '~'),
];

/// Single-character symbol words. Note: "phẩy" (comma) is intentionally
/// excluded — it is the Vietnamese decimal separator and is handled by the
/// decimal tagger. Including it here would cause the electronic tagger to
/// hijack every decimal expression.
const SINGLE_SYMBOLS: &[(&str, char)] = &[("chấm", '.'), ("chấm hỏi", '?'), ("chấm than", '!')];

/// Parse a spoken Vietnamese electronic address to written form.
///
/// Only fires when the input contains at least one electronic-specific marker
/// (`chấm`, `a còng`, `dấu hai chấm`, `gạch chéo`, etc.). Without this guard,
/// the tagger would greedily consume any digit-word sequence (`"mốt"` → `"1"`).
pub fn parse(input: &str) -> Option<String> {
    let input_lower = input.trim().to_lowercase();
    if input_lower.is_empty() {
        return None;
    }
    if !contains_electronic_marker(&input_lower) {
        return None;
    }

    let tokens: Vec<&str> = input_lower.split_whitespace().collect();
    let mut result = String::new();
    let mut i = 0;

    while i < tokens.len() {
        // Try multi-word symbols first.
        let mut matched = false;
        for &(phrase, sym) in MULTI_WORD_SYMBOLS {
            let phrase_tokens: Vec<&str> = phrase.split_whitespace().collect();
            let n = phrase_tokens.len();
            if i + n <= tokens.len() && tokens[i..i + n] == phrase_tokens[..] {
                result.push(sym);
                i += n;
                matched = true;
                break;
            }
        }
        if matched {
            continue;
        }

        // Single-token symbol.
        let token = tokens[i];
        if let Some(&(_, sym)) = SINGLE_SYMBOLS.iter().find(|(w, _)| *w == token) {
            result.push(sym);
            i += 1;
            continue;
        }

        // Spelled letter (single ASCII alphabetic char).
        let chars: Vec<char> = token.chars().collect();
        if chars.len() == 1 && chars[0].is_ascii_alphabetic() {
            result.push(chars[0]);
            i += 1;
            continue;
        }

        // Digit word.
        if let Some(d) = digit_word_to_char(token) {
            result.push(d);
            i += 1;
            continue;
        }

        // Compound number word ("mười", "hai mươi") — append its digits. We
        // only consume one token here to avoid greedily combining groups.
        if let Some(num) = words_to_number(token) {
            if (0..=99).contains(&num) {
                for c in num.to_string().chars() {
                    result.push(c);
                }
                i += 1;
                continue;
            }
        }

        // Unknown token — give up.
        return None;
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

/// True if the input contains at least one electronic-specific marker word.
/// Without this guard, the tagger would greedily consume any digit-word
/// sequence as if it were an email address.
fn contains_electronic_marker(input: &str) -> bool {
    // Multi-word markers (substring match).
    const MULTI_WORD_MARKERS: &[&str] = &[
        "a còng",
        "a cồng",
        "dấu hai chấm",
        "gạch chéo",
        "gạch ngang",
        "gạch dưới",
        "dấu thăng",
        "dấu ngã",
    ];
    if MULTI_WORD_MARKERS.iter().any(|m| input.contains(m)) {
        return true;
    }
    // Single-token markers (exact token match). "phẩy" is excluded — it is
    // the Vietnamese decimal separator, not an electronic symbol.
    const SINGLE_MARKERS: &[&str] = &["chấm", "chấm hỏi", "chấm than"];
    input
        .split_whitespace()
        .any(|t| SINGLE_MARKERS.contains(&t))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email() {
        assert_eq!(
            parse("t e s t a còng g m a i l chấm c o m"),
            Some("test@gmail.com".to_string())
        );
    }

    #[test]
    fn test_with_digits() {
        assert_eq!(
            parse("a một b hai a còng e x a m p l e chấm c o m"),
            Some("a1b2@example.com".to_string())
        );
    }

    #[test]
    fn test_url_symbols() {
        assert_eq!(
            parse("h t t p s dấu hai chấm gạch chéo gạch chéo e x a m p l e chấm c o m"),
            Some("https://example.com".to_string())
        );
    }

    #[test]
    fn test_invalid() {
        assert_eq!(parse("hello"), None);
        assert_eq!(parse(""), None);
    }
}
