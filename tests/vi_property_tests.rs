//! Property-based tests for Vietnamese TN/ITN using proptest.
//!
//! These tests verify that TN and ITN are correct across the entire input
//! space, not just hand-picked cases. The key property tested is round-trip
//! consistency: TN(d) → ITN(TN(d)) should recover d for all valid digit
//! strings in the supported range.

use proptest::prelude::*;
use text_processing_rs::{normalize_with_lang, tn_normalize_lang};

/// Strategy: generate integers in 10..=99_999 (the round-trippable range).
/// 0-9 are excluded (ITN cardinal rejects bare single digits to avoid
/// over-matching). 100000+ are excluded because single-scale millions
/// preserve the scale word ("1 triệu") rather than folding to digits.
fn small_integer() -> impl Strategy<Value = i64> {
    (10i64..=99_999).prop_map(|n| n)
}

proptest! {
    /// TN(n) → ITN(TN(n)) should recover n for all n in 10..=99_999.
    #[test]
    fn prop_cardinal_roundtrip(n in small_integer()) {
        let digits = n.to_string();
        let spoken = tn_normalize_lang(&digits, "vi");
        let recovered = normalize_with_lang(&spoken, "vi");
        prop_assert_eq!(recovered.as_str(), digits.as_str(), "round-trip failed for {}", n);
    }

    /// TN(n) should always produce a non-empty string (no None passthrough
    /// for valid digit inputs).
    #[test]
    fn prop_tn_never_empty(n in small_integer()) {
        let digits = n.to_string();
        let spoken = tn_normalize_lang(&digits, "vi");
        prop_assert!(!spoken.is_empty(), "TN produced empty output for {}", n);
    }

    /// TN(n) should not contain any ASCII digits (all digits should be
    /// converted to Vietnamese words).
    #[test]
    fn prop_tn_no_digits(n in small_integer()) {
        let digits = n.to_string();
        let spoken = tn_normalize_lang(&digits, "vi");
        prop_assert!(
            !spoken.chars().any(|c| c.is_ascii_digit()),
            "TN output contains digits for {}: {}",
            n, spoken
        );
    }

    /// Negative round-trip: TN(-n) → ITN should recover -n.
    #[test]
    fn prop_negative_roundtrip(n in 10i64..=99_999) {
        let digits = format!("-{}", n);
        let spoken = tn_normalize_lang(&digits, "vi");
        let recovered = normalize_with_lang(&spoken, "vi");
        prop_assert_eq!(recovered.as_str(), digits.as_str(), "negative round-trip failed for {}", digits);
    }
}

/// Strategy: generate valid times HH:MM (00:00..=23:59).
fn valid_time() -> impl Strategy<Value = String> {
    (0u32..=23, 0u32..=59).prop_map(|(h, m)| format!("{}:{:02}", h, m))
}

proptest! {
    /// TN(time) → ITN should recover the time (except for 0:00 → "nửa đêm"
    /// and 12:00 → "trưa" which are special-cased).
    #[test]
    fn prop_time_roundtrip(time in valid_time()) {
        let spoken = tn_normalize_lang(&time, "vi");
        let recovered = normalize_with_lang(&spoken, "vi");
        // 0:00 → "nửa đêm" → "0:00", 12:00 → "trưa" → "12:00"
        prop_assert_eq!(recovered.as_str(), time.as_str(), "time round-trip failed for {}", time);
    }
}

/// Strategy: generate valid dates DD/MM/YYYY (non-leap-year-safe).
fn valid_date() -> impl Strategy<Value = String> {
    (1u32..=28, 1u32..=12, 1900u32..=2100).prop_map(|(d, m, y)| format!("{}/{}/{}", d, m, y))
}

proptest! {
    /// TN(date) should produce a non-empty spoken form for all valid dates.
    #[test]
    fn prop_date_nonempty(date in valid_date()) {
        let spoken = tn_normalize_lang(&date, "vi");
        prop_assert!(!spoken.is_empty(), "TN date produced empty for {}", date);
        prop_assert!(
            !spoken.contains('/'),
            "TN date still contains '/' for {}: {}",
            date, spoken
        );
    }
}

/// Strategy: generate decimal numbers with fractional parts that are NOT
/// 3-digit groups (to avoid being mistaken for thousands separators).
/// Uses 1, 2, 4, or 5 fractional digits, and a non-zero integer part.
fn decimal_number() -> impl Strategy<Value = String> {
    (1i64..=99999, 1u32..=5).prop_map(|(int_part, frac_len)| {
        // Avoid frac_len == 3 (would look like thousands grouping).
        let frac_len = if frac_len == 3 { 4 } else { frac_len };
        let frac: String = (0..frac_len)
            .map(|i| char::from(b'0' + ((int_part as u32 + i) % 10) as u8))
            .collect();
        format!("{}.{}", int_part, frac)
    })
}

proptest! {
    /// TN(decimal) should produce a spoken form containing "phẩy".
    #[test]
    fn prop_decimal_has_phay(decimal in decimal_number()) {
        let spoken = tn_normalize_lang(&decimal, "vi");
        prop_assert!(
            spoken.contains("phẩy"),
            "TN decimal missing 'phẩy' for {}: {}",
            decimal, spoken
        );
    }
}
