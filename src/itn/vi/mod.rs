//! Inverse Text Normalization taggers for Vietnamese.
//!
//! Converts spoken-form Vietnamese to written form:
//! - "một trăm hai mươi ba" → "123"
//! - "ba phẩy một bốn" → "3.14"
//! - "mười bốn giờ ba mươi" → "14:30"
//! - "năm nghìn đồng" → "5000 ₫"

pub mod cardinal;
pub mod date;
pub mod decimal;
pub mod electronic;
pub mod fraction;
pub mod measure;
pub mod money;
pub mod ordinal;
pub mod punctuation;
pub mod telephone;
pub mod time;
pub mod whitelist;
pub mod word;
