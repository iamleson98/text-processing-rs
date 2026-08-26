//! Money TN tagger for Vietnamese.
//!
//! Converts written Vietnamese currency expressions to spoken form:
//! - "5000 ₫" → "năm nghìn đồng"
//! - "5.000 ₫" → "năm nghìn đồng"
//! - "$5" → "năm đô la"
//! - "₫100" → "một trăm đồng"
//! - "20.000 VND" → "hai mươi nghìn đồng"

use super::number_to_words;

/// A currency definition: the symbols / code words that introduce it and the
/// spoken Vietnamese name to emit.
struct Currency {
    /// Symbols that prefix the amount (e.g. "$", "₫") or suffix codes ("VND").
    symbols: &'static [&'static str],
    /// Spoken Vietnamese unit name.
    name: &'static str,
}

const CURRENCIES: &[Currency] = &[
    Currency {
        symbols: &["₫", "VND", "vnd"],
        name: "đồng",
    },
    Currency {
        symbols: &["$"],
        name: "đô la",
    },
    Currency {
        symbols: &["€"],
        name: "ơ rô",
    },
    Currency {
        symbols: &["£"],
        name: "bảng",
    },
    Currency {
        symbols: &["¥"],
        name: "yên",
    },
];

/// Parse a written money expression to spoken Vietnamese.
pub fn parse(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    for currency in CURRENCIES {
        for &symbol in currency.symbols {
            // Prefix form: "$5", "₫100".
            if let Some(rest) = trimmed.strip_prefix(symbol) {
                if let Some(amount) = parse_amount(rest) {
                    return Some(format!("{} {}", amount, currency.name));
                }
            }
            // Suffix form: "5 ₫", "100 VND" — accept optional separating space.
            let suffix_no_space = symbol;
            let suffix_with_space = format!(" {}", symbol);

            if let Some(rest) = trimmed.strip_suffix(suffix_no_space) {
                if let Some(amount) = parse_amount(rest.trim_end()) {
                    return Some(format!("{} {}", amount, currency.name));
                }
            }
            if let Some(rest) = trimmed.strip_suffix(&suffix_with_space) {
                if let Some(amount) = parse_amount(rest.trim()) {
                    return Some(format!("{} {}", amount, currency.name));
                }
            }
        }
    }

    None
}

/// Parse a numeric amount (with optional thousands separators) into Vietnamese words.
fn parse_amount(s: &str) -> Option<String> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    if !s
        .chars()
        .all(|c| c.is_ascii_digit() || c == '.' || c == ',' || c == ' ' || c == '\u{a0}')
    {
        return None;
    }
    let clean: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.is_empty() {
        return None;
    }
    let n: i64 = clean.parse().ok()?;
    Some(number_to_words(n))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dong() {
        assert_eq!(parse("₫5000"), Some("năm nghìn đồng".to_string()));
        assert_eq!(parse("5000 ₫"), Some("năm nghìn đồng".to_string()));
        assert_eq!(parse("5.000 ₫"), Some("năm nghìn đồng".to_string()));
        assert_eq!(parse("100 VND"), Some("một trăm đồng".to_string()));
    }

    #[test]
    fn test_dollar() {
        assert_eq!(parse("$5"), Some("năm đô la".to_string()));
        assert_eq!(parse("$100"), Some("một trăm đô la".to_string()));
    }

    #[test]
    fn test_other_currencies() {
        assert_eq!(parse("€10"), Some("mười ơ rô".to_string()));
        assert_eq!(parse("£50"), Some("năm mươi bảng".to_string()));
        assert_eq!(parse("¥1000"), Some("một nghìn yên".to_string()));
    }

    #[test]
    fn test_invalid() {
        assert_eq!(parse("hello"), None);
        assert_eq!(parse(""), None);
        assert_eq!(parse("$"), None);
    }
}
