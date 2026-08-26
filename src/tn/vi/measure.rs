//! Measure TN tagger for Vietnamese.
//!
//! Converts written measurements to spoken Vietnamese:
//! - "200 km" → "hai trăm ki lô mét"
//! - "200 km/h" → "hai trăm ki lô mét trên giờ"
//! - "50 kg" → "năm mươi ki lô gam"
//! - "25 °C" → "hai mươi lăm độ Xê"
//! - "2 m³" → "hai mét khối"

use super::number_to_words;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    /// Vietnamese spoken unit names indexed by their written symbol.
    /// Order of insertion matters for matching: longer symbols (e.g. "km/h")
    /// must be tried before shorter ones ("km").
    static ref UNITS: Vec<(&'static str, &'static str)> = vec![
        // Compound rate units first.
        ("km/h", "ki lô mét trên giờ"),
        ("m/s", "mét trên giây"),
        ("kg/m³", "ki lô gam trên mét khối"),
        // Plain units, longest first.
        ("km", "ki lô mét"),
        ("cm", "xăng ti mét"),
        ("mm", "mi li mét"),
        ("kg", "ki lô gam"),
        ("mg", "mi li gam"),
        ("g", "gam"),
        ("ml", "mi li lít"),
        ("cl", "xen ti lít"),
        ("dl", "đê xi lít"),
        ("l", "lít"),
        ("m³", "mét khối"),
        ("m²", "mét vuông"),
        ("cm³", "xăng ti mét khối"),
        ("cm²", "xăng ti mét vuông"),
        ("km²", "ki lô mét vuông"),
        ("m", "mét"),
        ("°C", "độ Xê"),
        ("°F", "độ Pha"),
        ("°", "độ"),
        ("min", "phút"),
        ("s", "giây"),
        ("h", "giờ"),
        ("Hz", "hét"),
        ("kHz", "ki lô hét"),
        ("MHz", "mê ga hét"),
        ("GHz", "gi ga hét"),
        ("W", "oát"),
        ("kW", "ki lô oát"),
        ("MW", "mê ga oát"),
        ("V", "vôn"),
        ("kV", "ki lô vôn"),
        ("A", "am pe"),
        ("mA", "mi li am pe"),
        ("Ω", "ôm"),
        ("J", "jun"),
        ("kJ", "ki lô jun"),
        ("cal", "ca lo"),
        ("kcal", "ki lô ca lo"),
        ("Pa", "pascal"),
        ("kPa", "ki lô pascal"),
        ("MPa", "mê ga pascal"),
        ("bar", "ba"),
        ("dB", "đê xi bén"),
        ("px", "pí xơ"),
        ("dpi", "đi pí ai"),
    ];
    /// Lookup map from symbol → spoken form (used after the longest symbol match
    /// has been resolved, so duplicates from the ordered Vec are not an issue).
    static ref UNIT_MAP: HashMap<&'static str, &'static str> = {
        UNITS.iter().cloned().collect()
    };
}

/// Parse a written measurement expression into spoken Vietnamese.
///
/// Recognised shapes: `"N unit"`, `"Nunit"`, `"unit N"` (rare). The numeric
/// amount is read with [`number_to_words`]; the unit is mapped through [`UNITS`].
pub fn parse(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    // Try each unit symbol (longest first) and look for amount + unit boundary.
    for &(symbol, spoken) in UNITS.iter() {
        // Suffix form: "200 km" or "200km".
        if let Some(rest) = strip_suffix_ci(trimmed, symbol) {
            if let Some(amount) = parse_amount(rest) {
                return Some(format!("{} {}", amount, spoken));
            }
        }
        // Prefix form: "°C 25" (uncommon but tolerated).
        if let Some(rest) = strip_prefix_ci(trimmed, symbol) {
            if let Some(amount) = parse_amount(rest) {
                return Some(format!("{} {}", amount, spoken));
            }
        }
    }

    None
}

/// Strip a unit suffix (case-sensitive where the symbol is case-sensitive).
fn strip_suffix_ci<'a>(haystack: &'a str, needle: &str) -> Option<&'a str> {
    // The unit list mixes case-sensitive symbols ("m" vs "M"); require exact
    // case match to avoid mis-parsing "M" as "m".
    let trimmed = haystack.trim_end();
    if let Some(rest) = trimmed.strip_suffix(needle) {
        return Some(rest);
    }
    // Tolerate a single space between number and unit when the symbol is alphabetic.
    let with_space = format!(" {}", needle);
    trimmed.strip_suffix(&with_space)
}

fn strip_prefix_ci<'a>(haystack: &'a str, needle: &str) -> Option<&'a str> {
    let trimmed = haystack.trim_start();
    if let Some(rest) = trimmed.strip_prefix(needle) {
        return Some(rest);
    }
    let with_space = format!("{} ", needle);
    trimmed.strip_prefix(&with_space)
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
    fn test_length() {
        assert_eq!(parse("200 km"), Some("hai trăm ki lô mét".to_string()));
        assert_eq!(parse("5 m"), Some("năm mét".to_string()));
        assert_eq!(parse("10 cm"), Some("mười xăng ti mét".to_string()));
    }

    #[test]
    fn test_rate() {
        assert_eq!(
            parse("200 km/h"),
            Some("hai trăm ki lô mét trên giờ".to_string())
        );
        assert_eq!(parse("5 m/s"), Some("năm mét trên giây".to_string()));
    }

    #[test]
    fn test_weight() {
        assert_eq!(parse("50 kg"), Some("năm mươi ki lô gam".to_string()));
        assert_eq!(parse("90 g"), Some("chín mươi gam".to_string()));
    }

    #[test]
    fn test_temperature() {
        assert_eq!(parse("25 °C"), Some("hai mươi lăm độ Xê".to_string()));
        assert_eq!(parse("100 °"), Some("một trăm độ".to_string()));
    }

    #[test]
    fn test_volume() {
        assert_eq!(parse("2 m³"), Some("hai mét khối".to_string()));
        assert_eq!(parse("5 m²"), Some("năm mét vuông".to_string()));
    }

    #[test]
    fn test_invalid() {
        assert_eq!(parse("hello"), None);
        assert_eq!(parse(""), None);
    }
}
