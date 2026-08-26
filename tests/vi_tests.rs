//! Vietnamese inverse text normalization tests.
//!
//! Test cases mirror the NeMo-format `spoken~written` convention used by the
//! other per-language ITN suites (see `tests/fr_tests.rs` for the canonical
//! layout). Each test reads `tests/data/vi/<category>.txt` and runs every case
//! through [`text_processing_rs::normalize_with_lang`] with `lang = "vi"`.

mod common;

use std::path::Path;
use text_processing_rs::normalize_with_lang;

fn normalize_vi(input: &str) -> String {
    normalize_with_lang(input, "vi")
}

fn print_failures(results: &common::TestResults) {
    for f in &results.failures {
        println!(
            "  FAIL: '{}' => '{}' (expected '{}')",
            f.input, f.got, f.expected
        );
    }
}

#[test]
fn test_cardinal() {
    let results = common::run_test_file(Path::new("tests/data/vi/cardinal.txt"), normalize_vi);
    println!(
        "cardinal: {}/{} passed ({} failures)",
        results.passed,
        results.total,
        results.failures.len()
    );
    print_failures(&results);
    assert!(
        results.failures.is_empty(),
        "{} cardinal ITN tests failed",
        results.failures.len()
    );
}

#[test]
fn test_decimal() {
    let results = common::run_test_file(Path::new("tests/data/vi/decimal.txt"), normalize_vi);
    println!(
        "decimal: {}/{} passed ({} failures)",
        results.passed,
        results.total,
        results.failures.len()
    );
    print_failures(&results);
    assert!(
        results.failures.is_empty(),
        "{} decimal ITN tests failed",
        results.failures.len()
    );
}

#[test]
fn test_time() {
    let results = common::run_test_file(Path::new("tests/data/vi/time.txt"), normalize_vi);
    println!(
        "time: {}/{} passed ({} failures)",
        results.passed,
        results.total,
        results.failures.len()
    );
    print_failures(&results);
    assert!(
        results.failures.is_empty(),
        "{} time ITN tests failed",
        results.failures.len()
    );
}

#[test]
fn test_date() {
    let results = common::run_test_file(Path::new("tests/data/vi/date.txt"), normalize_vi);
    println!(
        "date: {}/{} passed ({} failures)",
        results.passed,
        results.total,
        results.failures.len()
    );
    print_failures(&results);
    assert!(
        results.failures.is_empty(),
        "{} date ITN tests failed",
        results.failures.len()
    );
}

#[test]
fn test_money() {
    let results = common::run_test_file(Path::new("tests/data/vi/money.txt"), normalize_vi);
    println!(
        "money: {}/{} passed ({} failures)",
        results.passed,
        results.total,
        results.failures.len()
    );
    print_failures(&results);
    assert!(
        results.failures.is_empty(),
        "{} money ITN tests failed",
        results.failures.len()
    );
}

#[test]
fn test_measure() {
    let results = common::run_test_file(Path::new("tests/data/vi/measure.txt"), normalize_vi);
    println!(
        "measure: {}/{} passed ({} failures)",
        results.passed,
        results.total,
        results.failures.len()
    );
    print_failures(&results);
    assert!(
        results.failures.is_empty(),
        "{} measure ITN tests failed",
        results.failures.len()
    );
}

#[test]
fn test_telephone() {
    let results = common::run_test_file(Path::new("tests/data/vi/telephone.txt"), normalize_vi);
    println!(
        "telephone: {}/{} passed ({} failures)",
        results.passed,
        results.total,
        results.failures.len()
    );
    print_failures(&results);
    assert!(
        results.failures.is_empty(),
        "{} telephone ITN tests failed",
        results.failures.len()
    );
}

#[test]
fn test_electronic() {
    let results = common::run_test_file(Path::new("tests/data/vi/electronic.txt"), normalize_vi);
    println!(
        "electronic: {}/{} passed ({} failures)",
        results.passed,
        results.total,
        results.failures.len()
    );
    print_failures(&results);
    assert!(
        results.failures.is_empty(),
        "{} electronic ITN tests failed",
        results.failures.len()
    );
}

#[test]
fn test_whitelist() {
    let results = common::run_test_file(Path::new("tests/data/vi/whitelist.txt"), normalize_vi);
    println!(
        "whitelist: {}/{} passed ({} failures)",
        results.passed,
        results.total,
        results.failures.len()
    );
    print_failures(&results);
    assert!(
        results.failures.is_empty(),
        "{} whitelist ITN tests failed",
        results.failures.len()
    );
}

#[test]
fn test_word() {
    let results = common::run_test_file(Path::new("tests/data/vi/word.txt"), normalize_vi);
    println!(
        "word: {}/{} passed ({} failures)",
        results.passed,
        results.total,
        results.failures.len()
    );
    print_failures(&results);
}

#[test]
fn test_fraction() {
    let results = common::run_test_file(Path::new("tests/data/vi/fraction.txt"), normalize_vi);
    println!(
        "fraction: {}/{} passed ({} failures)",
        results.passed,
        results.total,
        results.failures.len()
    );
    print_failures(&results);
    assert!(
        results.failures.is_empty(),
        "{} fraction ITN tests failed",
        results.failures.len()
    );
}

#[test]
fn test_ordinal() {
    let results = common::run_test_file(Path::new("tests/data/vi/ordinal.txt"), normalize_vi);
    println!(
        "ordinal: {}/{} passed ({} failures)",
        results.passed,
        results.total,
        results.failures.len()
    );
    print_failures(&results);
    assert!(
        results.failures.is_empty(),
        "{} ordinal ITN tests failed",
        results.failures.len()
    );
}
