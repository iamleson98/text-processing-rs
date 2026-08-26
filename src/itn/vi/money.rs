//! Money tagger for Vietnamese.
//!
//! Converts spoken Vietnamese currency expressions to written form:
//! - "năm nghìn đồng" → "5000 ₫"
//! - "một trăm đồng" → "100 ₫"
//! - "năm đô la" → "5 $"
//! - "mười ơ rô" → "10 €"
//! - "một tỷ đồng" → "1000000000 ₫"
//! - "hai mươi tỷ năm mươi triệu đồng" → "20050000000 ₫"

use super::cardinal::words_to_number;

/// A currency definition: spoken Vietnamese unit words and the written symbol.
struct Currency {
    /// Spoken Vietnamese unit words (longest first for matching).
    words: &'static [&'static str],
    /// Written symbol.
    symbol: &'static str,
}

const CURRENCIES: &[Currency] = &[
    Currency {
        words: &["đồng"],
        symbol: "₫",
    },
    Currency {
        words: &["đô la", "đô"],
        symbol: "$",
    },
    Currency {
        words: &["ơ rô"],
        symbol: "€",
    },
    Currency {
        words: &["bảng anh", "bảng"],
        symbol: "£",
    },
    Currency {
        words: &["yên nhật", "yên"],
        symbol: "¥",
    },
    Currency {
        words: &["nhân dân tệ", "nhân tệ", "tệ"],
        symbol: "¥",
    },
    Currency {
        words: &["won"],
        symbol: "₩",
    },
];

/// Parse a spoken Vietnamese money expression to written form.
pub fn parse(input: &str) -> Option<String> {
    let input_lower = input.trim().to_lowercase();

    for currency in CURRENCIES {
        for &word in currency.words {
            // Pattern: "X WORD" → "N SYMBOL".
            let suffix = format!(" {}", word);
            if let Some(num_part) = input_lower.strip_suffix(&suffix) {
                if let Some(num) = parse_money_number(num_part) {
                    return Some(format!("{} {}", num, currency.symbol));
                }
            }
            // Whole input is just the currency word → refuse (need an amount).
        }
    }

    None
}

/// Parse the numeric portion of a money expression.
///
/// Handles:
/// - Bare amounts: "năm nghìn" → "5000"
/// - Scale-prefixed: "một tỷ" → "1000000000"
/// - Decimal amounts: "năm phẩy hai" → "5.2"
/// - Decimal + scale: "một phẩy năm triệu" → "1500000"
fn parse_money_number(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    let lower = trimmed.to_lowercase();

    // Decimal amount: "năm phẩy hai" → "5.2", "một phẩy năm triệu" → "1500000".
    if lower.contains("phẩy") {
        // Try decimal + scale: "một phẩy năm triệu" → 1.5 * 1_000_000 = 1500000.
        if let Some(result) = parse_decimal_with_scale(&lower) {
            return Some(result);
        }
        // Plain decimal: "năm phẩy hai" → "5.2".
        return super::decimal::parse(&lower);
    }

    // "một tỷ", "hai triệu", etc. — the scale word is part of the amount.
    if let Some(n) = words_to_number(&lower) {
        return Some(n.to_string());
    }

    None
}

/// Parse a decimal amount followed by a scale word.
/// "một phẩy năm triệu" → 1.5 × 1_000_000 = "1500000".
fn parse_decimal_with_scale(input: &str) -> Option<String> {
    const SCALES: &[(&str, i128)] = &[
        ("tỷ", 1_000_000_000),
        ("triệu", 1_000_000),
        ("nghìn", 1_000),
        ("ngàn", 1_000),
    ];

    for &(scale_word, scale_val) in SCALES {
        if input.ends_with(scale_word) {
            let num_part = input.strip_suffix(scale_word)?.trim();
            if num_part.contains("phẩy") {
                let decimal_str = super::decimal::parse(num_part)?;
                // Parse the decimal string back to f64, multiply by scale,
                // and format as integer (Vietnamese money doesn't use
                // fractional scale suffixes in written form).
                let val: f64 = decimal_str.parse().ok()?;
                let result = (val * scale_val as f64).round() as i128;
                return Some(result.to_string());
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dong() {
        assert_eq!(parse("năm nghìn đồng"), Some("5000 ₫".to_string()));
        assert_eq!(parse("một trăm đồng"), Some("100 ₫".to_string()));
        assert_eq!(parse("mười triệu đồng"), Some("10000000 ₫".to_string()));
    }

    #[test]
    fn test_dollar() {
        assert_eq!(parse("năm đô la"), Some("5 $".to_string()));
        assert_eq!(parse("mười đô"), Some("10 $".to_string()));
    }

    #[test]
    fn test_large() {
        assert_eq!(parse("một tỷ đồng"), Some("1000000000 ₫".to_string()));
        assert_eq!(
            parse("hai mươi tỷ năm mươi triệu đồng"),
            Some("20050000000 ₫".to_string())
        );
    }

    #[test]
    fn test_other_currencies() {
        assert_eq!(parse("mười ơ rô"), Some("10 €".to_string()));
        assert_eq!(parse("năm mươi bảng"), Some("50 £".to_string()));
        assert_eq!(parse("một trăm yên"), Some("100 ¥".to_string()));
        assert_eq!(parse("một nghìn tệ"), Some("1000 ¥".to_string()));
    }

    #[test]
    fn test_decimal_amounts() {
        assert_eq!(parse("năm phẩy hai đô la"), Some("5.2 $".to_string()));
        assert_eq!(
            parse("một phẩy năm triệu đồng"),
            Some("1500000 ₫".to_string())
        );
    }

    #[test]
    fn test_invalid() {
        assert_eq!(parse("hello"), None);
        assert_eq!(parse("đồng"), None); // no amount
        assert_eq!(parse(""), None);
    }
}
