//! Text Normalization taggers for Vietnamese.
//!
//! Converts written-form text to spoken Vietnamese:
//! - "123" → "một trăm hai mươi ba"
//! - "15" → "mười lăm"
//! - "21" → "hai mươi mốt"
//! - "3.14" → "ba phẩy một bốn"
//! - "14:30" → "mười bốn giờ ba mươi"
//!
//! Vietnamese number rules:
//! - 1 in the ones place of a ≥20 number becomes "mốt" (21 = "hai mươi mốt"),
//!   but stays "một" in the teens (11 = "mười một").
//! - 5 in any compound number becomes "lăm" (15 = "mười lăm", 25 = "hai mươi lăm"),
//!   but stays "năm" when standalone (5 = "năm").
//! - 0 between hundreds and ones uses the linker "linh" (105 = "một trăm linh năm"),
//!   with "lẻ" as an accepted regional variant.
//! - Thousands scale is "nghìn" (Northern, the NeMo default) — TN emits "nghìn".

pub mod cardinal;
pub mod date;
pub mod decimal;
pub mod electronic;
pub mod fraction;
pub mod measure;
pub mod money;
pub mod ordinal;
pub mod roman;
pub mod telephone;
pub mod time;
pub mod whitelist;

/// Standalone digit words for 0-9.
///
/// Note that 5 is "năm" standalone but becomes "lăm" inside compound numbers
/// (handled by [`chunk_to_words`]), and 1 becomes "mốt" in the ones place of
/// a ≥20 number (handled by [`tens_ones_word`]).
const DIGITS: [&str; 10] = [
    "không", "một", "hai", "ba", "bốn", "năm", "sáu", "bảy", "tám", "chín",
];

/// Convert an integer to Vietnamese words.
///
/// Examples:
/// - `0` → `"không"`
/// - `5` → `"năm"`
/// - `10` → `"mười"`
/// - `15` → `"mười lăm"`
/// - `21` → `"hai mươi mốt"`
/// - `25` → `"hai mươi lăm"`
/// - `105` → `"một trăm linh năm"`
/// - `123` → `"một trăm hai mươi ba"`
/// - `1000` → `"một nghìn"`
/// - `2025` → `"hai nghìn không trăm hai mươi lăm"`
/// - `-42` → `"âm bốn mươi hai"`
pub fn number_to_words(n: i64) -> String {
    if n == 0 {
        return "không".to_string();
    }
    if n < 0 {
        return format!("âm {}", unsigned_to_words(n.unsigned_abs()));
    }
    unsigned_to_words(n as u64)
}

/// Convert a non-negative integer to Vietnamese words. `n == 0` yields the empty
/// string so callers can compose it inside larger phrases (e.g. negatives).
fn unsigned_to_words(n: u64) -> String {
    if n == 0 {
        return String::new();
    }

    // Vietnamese groups digits in threes and uses the scales:
    // nghìn (10^3), triệu (10^6), tỷ (10^9). For 10^12 and above, Vietnamese
    // composes scale names: "nghìn tỷ" (10^12), "triệu tỷ" (10^15), "tỷ tỷ" (10^18).
    let scales: &[(u64, &str)] = &[
        (1_000_000_000_000_000_000, "tỷ tỷ"),
        (1_000_000_000_000_000, "triệu tỷ"),
        (1_000_000_000_000, "nghìn tỷ"),
        (1_000_000_000, "tỷ"),
        (1_000_000, "triệu"),
        (1_000, "nghìn"),
    ];

    let mut parts: Vec<String> = Vec::new();
    let mut remaining = n;

    for &(scale_value, scale_name) in scales {
        if remaining >= scale_value {
            let chunk = (remaining / scale_value) as u32;
            remaining %= scale_value;
            // The chunk is always 1..=999 here because we process larger scales first.
            // Non-trailing chunks (multiplied by a scale) never force the hundreds
            // digit: "hai mươi nghìn" (20 000) is correct, not "không trăm hai mươi nghìn".
            parts.push(format!("{} {}", chunk_to_words(chunk, false), scale_name));
        }
    }

    if remaining > 0 {
        // Trailing chunk: when higher-scale parts are present and the trailing
        // chunk is in 1..=99 (zero hundreds), emit "không trăm" explicitly so
        // the listener doesn't lose the hundreds place. This disambiguates:
        // - 2025 → "hai nghìn không trăm hai mươi lăm" (not "hai nghìn hai mươi lăm")
        // - 2002 → "hai nghìn không trăm linh hai" (not "hai nghìn hai" = 22?)
        // - 1005 → "một nghìn không trăm linh năm" (not "một nghìn năm" = 15?)
        // Chunks ≥ 100 (hundreds digit > 0) don't need forcing.
        let force_hundreds = !parts.is_empty() && remaining < 100;
        parts.push(chunk_to_words(remaining as u32, force_hundreds));
    }

    parts.join(" ")
}

/// Convert a value in `1..=999` to Vietnamese words.
///
/// `force_hundreds` emits "không trăm" when the hundreds digit is zero, used
/// for trailing chunks in multi-scale numbers (e.g. 2025) where the zero
/// hundreds place must be made explicit.
fn chunk_to_words(n: u32, force_hundreds: bool) -> String {
    debug_assert!(n > 0 && n < 1000, "chunk_to_words expects 1..=999, got {n}");

    let hundreds = n / 100;
    let rest = n % 100;

    let mut result = String::new();

    if hundreds > 0 {
        result.push_str(DIGITS[hundreds as usize]);
        result.push_str(" trăm");
    } else if force_hundreds {
        result.push_str("không trăm");
    }

    if rest > 0 {
        if !result.is_empty() {
            result.push(' ');
        }
        // "linh" is required when there is a hundreds digit (real or forced)
        // and the rest is a single digit (1..9).
        let has_hundreds = hundreds > 0 || force_hundreds;
        if rest < 10 {
            if has_hundreds {
                result.push_str("linh ");
            }
            result.push_str(DIGITS[rest as usize]);
        } else if rest < 20 {
            // 10-19: "mười X" with "lăm" for 5.
            result.push_str("mười");
            let ones = rest - 10;
            if ones > 0 {
                result.push(' ');
                result.push_str(teen_ones_word(ones));
            }
        } else {
            // 20-99: "X mươi Y" with "mốt" for 1 and "lăm" for 5.
            let tens = rest / 10;
            let ones = rest % 10;
            result.push_str(DIGITS[tens as usize]);
            result.push_str(" mươi");
            if ones > 0 {
                result.push(' ');
                result.push_str(tens_ones_word(ones));
            }
        }
    }

    result
}

/// Ones-place word for 11-19 (after "mười"). Only 5 changes form ("lăm");
/// every other digit keeps its standalone word.
fn teen_ones_word(d: u32) -> &'static str {
    match d {
        1 => "một",
        2 => "hai",
        3 => "ba",
        4 => "bốn",
        5 => "lăm",
        6 => "sáu",
        7 => "bảy",
        8 => "tám",
        9 => "chín",
        _ => "",
    }
}

/// Ones-place word for the X0-X9 range where X ≥ 2 (after "mươi").
/// 1 becomes "mốt" (e.g. 21 = "hai mươi mốt") and 5 becomes "lăm".
fn tens_ones_word(d: u32) -> &'static str {
    match d {
        1 => "mốt",
        5 => "lăm",
        _ => teen_ones_word(d),
    }
}

/// Spell each digit of a string individually in Vietnamese.
///
/// Used for decimal fractions and telephone numbers where each digit is read
/// independently. Non-digit characters are silently skipped.
pub fn spell_digits(s: &str) -> String {
    s.chars()
        .filter_map(|c| c.to_digit(10).map(|d| DIGITS[d as usize]))
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(number_to_words(0), "không");
        assert_eq!(number_to_words(1), "một");
        assert_eq!(number_to_words(5), "năm");
        assert_eq!(number_to_words(10), "mười");
        assert_eq!(number_to_words(11), "mười một");
        assert_eq!(number_to_words(15), "mười lăm");
    }

    #[test]
    fn test_twenties() {
        assert_eq!(number_to_words(20), "hai mươi");
        assert_eq!(number_to_words(21), "hai mươi mốt");
        assert_eq!(number_to_words(25), "hai mươi lăm");
        assert_eq!(number_to_words(29), "hai mươi chín");
    }

    #[test]
    fn test_hundreds() {
        assert_eq!(number_to_words(100), "một trăm");
        assert_eq!(number_to_words(105), "một trăm linh năm");
        assert_eq!(number_to_words(123), "một trăm hai mươi ba");
        assert_eq!(number_to_words(999), "chín trăm chín mươi chín");
    }

    #[test]
    fn test_thousands() {
        assert_eq!(number_to_words(1000), "một nghìn");
        assert_eq!(number_to_words(2000), "hai nghìn");
        assert_eq!(number_to_words(2025), "hai nghìn không trăm hai mươi lăm");
        assert_eq!(number_to_words(1_000_000), "một triệu");
        assert_eq!(number_to_words(1_000_000_000), "một tỷ");
    }

    #[test]
    fn test_large() {
        // Trailing single-digit chunk "5" forces "không trăm linh" to disambiguate
        // from "hai tỷ ba triệu bốn nghìn năm" which could be misheard as ...15.
        assert_eq!(
            number_to_words(2_003_004_005),
            "hai tỷ ba triệu bốn nghìn không trăm linh năm"
        );
    }

    #[test]
    fn test_negative() {
        assert_eq!(number_to_words(-42), "âm bốn mươi hai");
        assert_eq!(number_to_words(-1), "âm một");
    }

    #[test]
    fn test_spell_digits() {
        assert_eq!(spell_digits("0"), "không");
        assert_eq!(spell_digits("123"), "một hai ba");
        assert_eq!(spell_digits("905"), "chín không năm");
    }
}
