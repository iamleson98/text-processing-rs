//! Vietnamese round-trip tests: TN → ITN should recover the original digits.
//!
//! For each test case, we apply TN to convert a written number to spoken
//! Vietnamese, then apply ITN to convert the spoken form back to written form.
//! The round-trip should recover the original digit string (or a semantically
//! equivalent form).
//!
//! Known asymmetries (by design, not bugs):
//! - Bare 0: ITN cardinal rejects "không" to avoid over-matching in sentences.
//! - Single-scale millions/billions: ITN preserves the scale word ("1 triệu",
//!   "1 tỷ") rather than folding to digits, matching the French convention.
//!   The round-trip for these yields "N triệu"/"N tỷ" instead of full digits.
//! - Multi-scale compounds ("2 tỷ 3 triệu 4 nghìn 5") round-trip to full digits.

mod common;

use text_processing_rs::{normalize_with_lang, tn_normalize_lang};

/// Round-trip a cardinal: digit string → TN → spoken → ITN → digit string.
fn roundtrip_cardinal(digits: &str) -> String {
    let spoken = tn_normalize_lang(digits, "vi");
    normalize_with_lang(&spoken, "vi")
}

/// Round-trip a time: "H:M" → TN → spoken → ITN → "H:M".
fn roundtrip_time(time: &str) -> String {
    let spoken = tn_normalize_lang(time, "vi");
    normalize_with_lang(&spoken, "vi")
}

// ── Cardinal round-trips ─────────────────────────────────────────────

#[test]
fn test_cardinal_roundtrip_small() {
    // 10..=99 (skip 0-9: bare single-digit words are rejected by ITN cardinal
    // to avoid over-matching — "một" could be the article "a/an", not the number 1).
    for n in 10..=99 {
        let digits = n.to_string();
        let result = roundtrip_cardinal(&digits);
        assert_eq!(
            result, digits,
            "cardinal round-trip failed for {}: TN→ITN yielded {}",
            digits, result
        );
    }
}

#[test]
fn test_cardinal_roundtrip_hundreds() {
    let cases = [100, 105, 110, 123, 200, 505, 999];
    for n in cases {
        let digits = n.to_string();
        let result = roundtrip_cardinal(&digits);
        assert_eq!(
            result, digits,
            "hundreds round-trip failed for {}: got {}",
            digits, result
        );
    }
}

#[test]
fn test_cardinal_roundtrip_thousands() {
    // Skip 1000000 (single-scale million: round-trips to "1 triệu" not "1000000").
    let cases = [1000, 1005, 2025, 10000, 999000];
    for n in cases {
        let digits = n.to_string();
        let result = roundtrip_cardinal(&digits);
        assert_eq!(
            result, digits,
            "thousands round-trip failed for {}: got {}",
            digits, result
        );
    }
}

#[test]
fn test_cardinal_roundtrip_large_multiscale() {
    // Multi-scale compounds fold to full digits (cardinal handles them).
    // Single-scale (1e9, 1e12) are excluded — they preserve the scale word.
    let cases = [
        2_003_004_005i64, // 2 tỷ 3 triệu 4 nghìn 5
        123_045_000_000,  // 123 tỷ 45 triệu
    ];
    for n in cases {
        let digits = n.to_string();
        let result = roundtrip_cardinal(&digits);
        assert_eq!(
            result, digits,
            "multi-scale round-trip failed for {}: got {}",
            digits, result
        );
    }
}

#[test]
fn test_cardinal_roundtrip_single_scale_preserved() {
    // Single-scale millions/billions preserve the scale word (by design,
    // matching the French "18 milliards" convention).
    assert_eq!(roundtrip_cardinal("1000000"), "1 triệu");
    assert_eq!(roundtrip_cardinal("1000000000"), "1 tỷ");
}

// ── Time round-trips ─────────────────────────────────────────────────

#[test]
fn test_time_roundtrip() {
    let cases = [
        "0:00",  // nửa đêm — ITN yields "0:00"
        "8:00",  // tám giờ
        "9:05",  // chín giờ linh năm
        "14:30", // mười bốn giờ ba mươi
        "23:59", // hai mươi ba giờ năm mươi chín
    ];
    for time in &cases {
        let result = roundtrip_time(time);
        assert_eq!(
            result, *time,
            "time round-trip failed for {}: got {}",
            time, result
        );
    }
}

// ── Negative cardinal round-trips ────────────────────────────────────

#[test]
fn test_negative_roundtrip() {
    let cases = ["-1", "-10", "-42", "-100", "-1000"];
    for digits in &cases {
        let spoken = tn_normalize_lang(digits, "vi");
        let result = normalize_with_lang(&spoken, "vi");
        assert_eq!(
            result, *digits,
            "negative round-trip failed for {}: TN→ITN yielded {}",
            digits, result
        );
    }
}
