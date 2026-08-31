//! Multi-language inverse text normalization integration tests.
//!
//! Tests the ITN language dispatch API: normalize_sentence_lang(). Ensures all
//! 7 languages are wired up and producing correct output through the public
//! sentence-level API (the ITN counterpart of tn_normalize_sentence_lang).
//!
//! Two tagger architectures are exercised:
//! - en/fr/de/es: expression-level taggers scanned with the sliding window.
//! - hi/ja/zh: whole-sentence scanning taggers (sentence == single-expression).

use text_processing_rs::{normalize_sentence_lang, normalize_sentence_with_max_span_lang};

// ── French (sliding-window taggers) ──────────────────────────────────

#[test]
fn test_fr_sentence_single_expression() {
    assert_eq!(
        normalize_sentence_lang("j'ai vingt et un ans", "fr"),
        "j'ai 21 ans"
    );
}

#[test]
fn test_fr_sentence_multiple_expressions() {
    // Two separate number spans in one sentence — the whole-input
    // normalize_with_lang would leave this untouched; the sliding window
    // catches each span.
    assert_eq!(
        normalize_sentence_lang("j'ai vingt et un ans et trente chiens", "fr"),
        "j'ai 21 ans et 30 chiens"
    );
}

#[test]
fn test_fr_sentence_passthrough() {
    assert_eq!(
        normalize_sentence_lang("bonjour le monde", "fr"),
        "bonjour le monde"
    );
}

// ── German (sliding-window taggers) ──────────────────────────────────

#[test]
fn test_de_sentence_single_expression() {
    assert_eq!(
        normalize_sentence_lang("ich habe einundzwanzig Katzen", "de"),
        "ich habe 21 Katzen"
    );
}

#[test]
fn test_de_sentence_multiple_expressions() {
    assert_eq!(
        normalize_sentence_lang("ich habe dreiundzwanzig Äpfel und zehn Birnen", "de"),
        "ich habe 23 Äpfel und 10 Birnen"
    );
}

// ── Spanish (sliding-window taggers) ─────────────────────────────────

#[test]
fn test_es_sentence_single_expression() {
    assert_eq!(
        normalize_sentence_lang("tengo veintiuno años", "es"),
        "tengo 21 años"
    );
}

// ── Vietnamese (sliding-window taggers) ──────────────────────────────

#[test]
fn test_vi_sentence_single_expression() {
    assert_eq!(
        normalize_sentence_lang("tôi có hai mươi mốt quả táo", "vi"),
        "tôi có 21 quả táo"
    );
}

#[test]
fn test_vi_sentence_multiple_expressions() {
    assert_eq!(
        normalize_sentence_lang("tôi có hai mươi mốt quả táo và ba mươi quả cam", "vi"),
        "tôi có 21 quả táo và 30 quả cam"
    );
}

#[test]
fn test_vi_sentence_passthrough() {
    assert_eq!(
        normalize_sentence_lang("xin chào thế giới", "vi"),
        "xin chào thế giới"
    );
}

#[test]
fn test_vi_sentence_money() {
    // Money expression inside a sentence.
    assert_eq!(
        normalize_sentence_lang("giá là năm nghìn đồng", "vi"),
        "giá là 5000 ₫"
    );
}

#[test]
fn test_vi_sentence_time() {
    // Time expression inside a sentence.
    assert_eq!(
        normalize_sentence_lang("hẹn gặp lúc hai giờ chiều", "vi"),
        "hẹn gặp lúc 14:00"
    );
}

#[test]
fn test_vi_sentence_mixed_types() {
    // Multiple expression types in one sentence.
    assert_eq!(
        normalize_sentence_lang("tôi có hai mươi mốt quả táo và năm nghìn đồng", "vi"),
        "tôi có 21 quả táo và 5000 ₫"
    );
}

#[test]
fn test_vi_sentence_with_punctuation() {
    // Trailing punctuation is preserved and glued (Vietnamese does not add a
    // space before terminal punctuation).
    assert_eq!(
        normalize_sentence_lang("tôi có hai mươi mốt quả táo!", "vi"),
        "tôi có 21 quả táo!"
    );
}

// ── Chinese (whole-sentence scanning) ────────────────────────────────

#[test]
fn test_zh_sentence() {
    assert_eq!(
        normalize_sentence_lang("我有二十一个苹果", "zh"),
        "我有21个苹果"
    );
}

// ── Japanese (whole-sentence scanning) ───────────────────────────────

#[test]
fn test_ja_sentence() {
    assert_eq!(
        normalize_sentence_lang("私は二十一個のりんごを持っています", "ja"),
        "私は21個のりんごを持っています"
    );
}

// ── Hindi (whole-sentence scanning, Devanagari digits) ───────────────

#[test]
fn test_hi_sentence() {
    assert_eq!(
        normalize_sentence_lang("मेरे पास इक्कीस सेब हैं", "hi"),
        "मेरे पास २१ सेब हैं"
    );
}

// ── English + fallback ───────────────────────────────────────────────

#[test]
fn test_en_sentence() {
    assert_eq!(
        normalize_sentence_lang("I have twenty one apples", "en"),
        "I have 21 apples"
    );
}

#[test]
fn test_unknown_lang_falls_back_to_english() {
    assert_eq!(
        normalize_sentence_lang("I have twenty one apples", "xx"),
        "I have 21 apples"
    );
    // Empty language code also routes through English sentence mode.
    assert_eq!(
        normalize_sentence_lang("I have twenty one apples", ""),
        "I have 21 apples"
    );
}

// ── Edge cases ───────────────────────────────────────────────────────

#[test]
fn test_empty_and_whitespace() {
    for lang in ["en", "fr", "de", "es", "hi", "ja", "zh", "vi"] {
        assert_eq!(normalize_sentence_lang("", lang), "");
        assert_eq!(normalize_sentence_lang("   ", lang), "");
    }
}

#[test]
fn test_max_span_zero_uses_library_default() {
    // max_span_tokens == 0 selects the library default window (16), matching
    // the documented FFI/WASM/Swift convention. Multi-token spans coalesce
    // normally: French cardinals ("vingt et un") and English money+scale
    // ("$84.5 billion") are read as a whole instead of degrading to
    // one-token spans ("vingt" / "eighty four dollars fifty cents").
    let out = normalize_sentence_with_max_span_lang("j'ai vingt et un ans", "fr", 0);
    assert_eq!(out, "j'ai 21 ans");
    // Scanning languages ignore max_span entirely.
    assert_eq!(
        normalize_sentence_with_max_span_lang("我有二十一个苹果", "zh", 0),
        "我有21个苹果"
    );
}
