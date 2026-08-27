//! Vietnamese (vi) text normalization via NeMo's compiled grammars.
//!
//! Byte-exact with NeMo's `Normalizer(lang="vi", deterministic=True)`.
//! Vietnamese is space-delimited, so tokens join with `" "`. Vietnamese
//! requires a post-processing FST (for measure unit spacing and other
//! cleanup), unlike fr/de/es which pass `None`.
//!
//! Grammars in `grammars/vi/` are exported from NeMo-text-processing
//! (Apache-2.0, NeMo 1.2.0).

use super::{driver, load_gz};
use lazy_static::lazy_static;
use rustfst::prelude::*;

lazy_static! {
    static ref CLASSIFY: VectorFst<TropicalWeight> =
        load_gz(include_bytes!("../../grammars/vi/classify.fst.gz"));
    static ref VERBALIZE: VectorFst<TropicalWeight> =
        load_gz(include_bytes!("../../grammars/vi/verbalize.fst.gz"));
    static ref POSTPROCESS: VectorFst<TropicalWeight> =
        load_gz(include_bytes!("../../grammars/vi/postprocess.fst.gz"));
}

/// Normalize written-form Vietnamese (vi) to spoken form.
///
/// ```
/// # #[cfg(feature = "fst-engine")]
/// # {
/// use text_processing_rs::fst::vi;
/// assert_eq!(vi::normalize("50"), "năm mươi");
/// # }
/// ```
pub fn normalize(input: &str) -> String {
    driver::normalize_no_punct_post(&CLASSIFY, &VERBALIZE, Some(&POSTPROCESS), input, " ")
}
