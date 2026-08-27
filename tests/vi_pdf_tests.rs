//! Vietnamese PDF text normalization integration tests.
//!
//! Reproduces real-world PDF text scenarios (economics, science, citations,
//! contact info) to verify TN produces correct spoken Vietnamese for TTS.
//! Tests use the default max_span (16) since multi-token expressions like
//! "750 ml" and "1900 1560" require a window larger than 1.

use text_processing_rs::tn_normalize_sentence_lang;

#[test]
fn test_pdf_economics_banking() {
    let input = "Tính đến ngày 31/12/2025, tổng tài sản tăng 12.5 %";
    let result = tn_normalize_sentence_lang(input, "vi");
    assert!(
        result.contains("ba mươi mốt tháng mười hai"),
        "date 31/12 should normalize: got {}",
        result
    );
    assert!(
        result.contains("mười hai phẩy năm phần trăm"),
        "12.5 % should normalize to decimal + percent: got {}",
        result
    );
}

#[test]
fn test_pdf_currency_vnd() {
    // VNĐ currency code should normalize to "đồng".
    assert_eq!(
        tn_normalize_sentence_lang("112.500.000.000 VNĐ", "vi"),
        "một trăm mười hai tỷ năm trăm triệu đồng"
    );
}

#[test]
fn test_pdf_dollar_amount() {
    // $4.500.000 (dot-separated thousands) should normalize correctly.
    let result = tn_normalize_sentence_lang("$4.500.000", "vi");
    assert!(
        result.contains("bốn triệu năm trăm nghìn đô la"),
        "$4.500.000 should normalize: got {}",
        result
    );
}

#[test]
fn test_pdf_negative_percentage() {
    // -0.5 % should normalize with "âm" and "phần trăm".
    let result = tn_normalize_sentence_lang("-0.5 %", "vi");
    assert!(
        result.contains("âm không phẩy năm phần trăm"),
        "-0.5 % should normalize: got {}",
        result
    );
}

#[test]
fn test_pdf_scientific_temperature() {
    // 25.5°C should preserve the decimal, not flatten to 255.
    let result = tn_normalize_sentence_lang("25.5°C", "vi");
    assert_eq!(result, "hai mươi lăm phẩy năm độ Xê");
}

#[test]
fn test_pdf_fraction_with_unit() {
    // 3/4 should be a fraction ("ba phần tư"), not a date ("ba tháng tư").
    let result = tn_normalize_sentence_lang("3/4 lít", "vi");
    assert!(
        result.contains("ba phần tư"),
        "3/4 should be a fraction: got {}",
        result
    );
}

#[test]
fn test_pdf_measure_ml() {
    // 750 ml should normalize to "bảy trăm năm mươi mi li lít".
    assert_eq!(
        tn_normalize_sentence_lang("750 ml", "vi"),
        "bảy trăm năm mươi mi li lít"
    );
}

#[test]
fn test_pdf_roman_numeral_chapter() {
    // Roman numeral II in chapter heading should convert to "hai".
    let result = tn_normalize_sentence_lang("chương II", "vi");
    assert!(
        result.contains("chương hai"),
        "Roman II should normalize to 'hai': got {}",
        result
    );
}

#[test]
fn test_pdf_year_single_digit_trailing() {
    // Year 2002 should not be ambiguous — "hai nghìn không trăm linh hai".
    let result = tn_normalize_sentence_lang("năm 2002", "vi");
    assert!(
        result.contains("hai nghìn không trăm linh hai"),
        "2002 should disambiguate: got {}",
        result
    );
}

#[test]
fn test_pdf_hotline_number() {
    // 1900 1560 (hotline) should be read digit-by-digit, not as a cardinal.
    let result = tn_normalize_sentence_lang("1900 1560", "vi");
    assert!(
        result.contains("một chín không không"),
        "1900 should be digit-by-digit: got {}",
        result
    );
}

#[test]
fn test_pdf_phone_with_dots() {
    // 0988.123.456 (phone with dot separators) should be read digit-by-digit.
    let result = tn_normalize_sentence_lang("0988.123.456", "vi");
    assert!(
        result.contains("không chín tám tám"),
        "phone with dots should be digit-by-digit: got {}",
        result
    );
}

#[test]
fn test_pdf_time_with_modifier() {
    // 08:30 sáng should normalize with "sáng" AM modifier.
    let result = tn_normalize_sentence_lang("08:30 sáng", "vi");
    assert!(
        result.contains("tám giờ ba mươi"),
        "08:30 should normalize: got {}",
        result
    );
}

#[test]
fn test_pdf_full_passage_1() {
    // Full passage from the user's test — verify key parts normalize correctly.
    let input = "Tính đến ngày 31/12/2025, tổng tài sản tăng 12.5 %, đạt khoảng $4.500.000 (tương đương 112.500.000.000 VNĐ). Mức lạm phát trung bình duy trì ở mức -0.5 %.";
    let result = tn_normalize_sentence_lang(input, "vi");

    // Verify all key transformations:
    assert!(
        result.contains("ba mươi mốt tháng mười hai"),
        "date: {}",
        result
    );
    assert!(
        result.contains("mười hai phẩy năm phần trăm"),
        "12.5%: {}",
        result
    );
    assert!(
        result.contains("bốn triệu năm trăm nghìn đô la"),
        "$4.5M: {}",
        result
    );
    assert!(
        result.contains("một trăm mười hai tỷ năm trăm triệu đồng"),
        "VNĐ: {}",
        result
    );
    assert!(
        result.contains("âm không phẩy năm phần trăm"),
        "-0.5%: {}",
        result
    );
}

#[test]
fn test_pdf_full_passage_2() {
    let input = "Nhiệt độ phòng thí nghiệm duy trì ở 25.5°C với áp suất 1.2 bar. Lượng dung dịch còn lại là 3/4 lít, tương đương 750 ml.";
    let result = tn_normalize_sentence_lang(input, "vi");

    assert!(
        result.contains("hai mươi lăm phẩy năm độ Xê"),
        "25.5°C: {}",
        result
    );
    assert!(result.contains("ba phần tư"), "3/4 fraction: {}", result);
    assert!(
        result.contains("bảy trăm năm mươi mi li lít"),
        "750 ml: {}",
        result
    );
}

#[test]
fn test_pdf_full_passage_3() {
    let input = "Theo chương II, trang 145 của cuốn sách xuất bản năm 1998, cuộc họp diễn ra vào lúc 08:30 sáng ngày 15/04/2002.";
    let result = tn_normalize_sentence_lang(input, "vi");

    assert!(result.contains("chương hai"), "Roman II: {}", result);
    assert!(
        result.contains("một nghìn chín trăm chín mươi tám"),
        "1998: {}",
        result
    );
    assert!(
        result.contains("hai nghìn không trăm linh hai"),
        "2002: {}",
        result
    );
}
