//! Benchmarks for Vietnamese TN/ITN hot paths.
//!
//! Run with: `cargo bench --bench vi_bench`
//!
//! Covers the most performance-critical paths:
//! - TN cardinal (digit → spoken): the inner `number_to_words` + `parse` chain
//! - ITN cardinal (spoken → digit): the `words_to_number` parser
//! - TN/ITN sentence mode: the sliding-window scanner
//! - Each benchmark is run on small (2-digit), medium (6-digit), and large
//!   (10+ digit) inputs to characterise scaling.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use text_processing_rs::{
    normalize_sentence_lang, normalize_with_lang, tn_normalize_lang, tn_normalize_sentence_lang,
};

/// TN cardinal: digit string → spoken Vietnamese.
fn bench_tn_cardinal(c: &mut Criterion) {
    let mut group = c.benchmark_group("tn_cardinal_vi");
    let cases: &[(&str, &str)] = &[
        ("small", "42"),
        ("medium", "123456"),
        ("large", "2003004005"),
        ("xlarge", "9223372036854775807"), // i64::MAX
    ];
    for (name, input) in cases {
        group.bench_with_input(BenchmarkId::from_parameter(name), input, |b, &inp| {
            b.iter(|| tn_normalize_lang(black_box(inp), "vi"));
        });
    }
    group.finish();
}

/// ITN cardinal: spoken Vietnamese → digit string.
fn bench_itn_cardinal(c: &mut Criterion) {
    let mut group = c.benchmark_group("itn_cardinal_vi");
    let cases: &[(&str, &str)] = &[
        ("small", "hai mươi mốt"),
        ("medium", "một trăm hai mươi ba nghìn bốn trăm năm mươi sáu"),
        ("large", "hai tỷ ba triệu bốn nghìn năm"),
    ];
    for (name, input) in cases {
        group.bench_with_input(BenchmarkId::from_parameter(name), input, |b, &inp| {
            b.iter(|| normalize_with_lang(black_box(inp), "vi"));
        });
    }
    group.finish();
}

/// TN sentence mode: scan a sentence for digit spans and convert to spoken.
fn bench_tn_sentence(c: &mut Criterion) {
    let mut group = c.benchmark_group("tn_sentence_vi");
    let cases: &[(&str, &str)] = &[
        ("short", "Tôi có 21 quả táo"),
        ("medium", "Tôi có 21 quả táo và 30 quả cam"),
        (
            "long",
            "Giá là 5000 ₫ cho 21 quả táo và 30 quả cam vào 14:30 ngày 5/1/2025",
        ),
    ];
    for (name, input) in cases {
        group.bench_with_input(BenchmarkId::from_parameter(name), input, |b, &inp| {
            b.iter(|| tn_normalize_sentence_lang(black_box(inp), "vi"));
        });
    }
    group.finish();
}

/// ITN sentence mode: scan a spoken sentence for number spans and convert to digits.
fn bench_itn_sentence(c: &mut Criterion) {
    let mut group = c.benchmark_group("itn_sentence_vi");
    let cases: &[(&str, &str)] = &[
        ("short", "tôi có hai mươi mốt quả táo"),
        ("medium", "tôi có hai mươi mốt quả táo và ba mươi quả cam"),
        (
            "long",
            "tôi có hai mươi mốt quả táo và ba mươi quả cam giá năm nghìn đồng lúc hai giờ chiều",
        ),
    ];
    for (name, input) in cases {
        group.bench_with_input(BenchmarkId::from_parameter(name), input, |b, &inp| {
            b.iter(|| normalize_sentence_lang(black_box(inp), "vi"));
        });
    }
    group.finish();
}

/// TN decimal: digit decimal → spoken.
fn bench_tn_decimal(c: &mut Criterion) {
    let mut group = c.benchmark_group("tn_decimal_vi");
    let cases: &[(&str, &str)] = &[
        ("short", "3.14"),
        ("medium", "123.456789"),
        ("long", "0.141592653589793"),
    ];
    for (name, input) in cases {
        group.bench_with_input(BenchmarkId::from_parameter(name), input, |b, &inp| {
            b.iter(|| tn_normalize_lang(black_box(inp), "vi"));
        });
    }
    group.finish();
}

/// ITN decimal: spoken decimal → digit.
fn bench_itn_decimal(c: &mut Criterion) {
    let mut group = c.benchmark_group("itn_decimal_vi");
    let cases: &[(&str, &str)] = &[
        ("short", "ba phẩy một bốn"),
        (
            "medium",
            "một trăm hai mươi ba phẩy bốn năm sáu bảy tám chín",
        ),
        (
            "long",
            "không phẩy một bốn một năm chín hai sáu năm ba năm tám chín bảy chín ba",
        ),
    ];
    for (name, input) in cases {
        group.bench_with_input(BenchmarkId::from_parameter(name), input, |b, &inp| {
            b.iter(|| normalize_with_lang(black_box(inp), "vi"));
        });
    }
    group.finish();
}

/// Passthrough baseline: non-normalizable text should be fast (no tagger
/// matches, early return).
fn bench_passthrough(c: &mut Criterion) {
    let mut group = c.benchmark_group("passthrough_vi");
    let cases: &[(&str, &str)] = &[
        ("short", "xin chào"),
        ("medium", "xin chào thế giới hôm nay trời đẹp"),
        (
            "long",
            "tôi đang đi dạo trong công viên và nghe thấy tiếng chim hót rất vui tai",
        ),
    ];
    for (name, input) in cases {
        group.bench_with_input(BenchmarkId::from_parameter(name), input, |b, &inp| {
            b.iter(|| normalize_sentence_lang(black_box(inp), "vi"));
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_tn_cardinal,
    bench_itn_cardinal,
    bench_tn_sentence,
    bench_itn_sentence,
    bench_tn_decimal,
    bench_itn_decimal,
    bench_passthrough,
);
criterion_main!(benches);
