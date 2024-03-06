mod hk_array_class;
mod hk_array_ref;
mod hk_array_vector;

use crate::havok_types::float::cpp_to_rust_float_str;
pub use hk_array_class::{HkArrayClass, HkArrayClassParam};
pub use hk_array_ref::HkArrayRef;
pub use hk_array_vector::HkArrayVector;
use std::borrow::Cow;

/// Separate vectors classified by `()`, `,` space, etc.
///
/// # Note
/// C++ non-numbers and infinities are converted to Rust numeric strings.
pub fn normalize(str: &str) -> Vec<Cow<'_, str>> {
    str.split(&['(', ')', ',', ' ', '\n', '\r', '\t'][..])
        .filter(|&x| !x.is_empty())
        .map(|s| cpp_to_rust_float_str(s).into())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_normalize() {
        let input = "1.0 (2.0, 3.0) -1.#IND00";
        assert_eq!(normalize(input), vec!["1.0", "2.0", "3.0", "0.0"]);
    }
}
