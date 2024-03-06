use core::fmt;
use ordered_float::FloatCore;
use std::borrow::Cow;

/// The XML representation of Not a number, infinity is different from the Rust representation.
pub fn cpp_to_rust_float_str(s: &str) -> &str {
    match s {
        "1.#IND" | "1.#IND0" | "1.#IND00" | "-1.#IND" | "-1.#IND0" | "-1.#IND00" => "nan",
        "1.#INF" | "1.#INF0" | "1.#INF00" => "inf",
        "-1.#INF" | "-1.#INF0" | "-1.#INF00" => "-inf",
        s => s,
    }
}

/// The XML representation of Not a number, infinity is different from the Rust representation.
/// This function fills in the difference, since the information is lost in `NAN` in Rust, it is now displayed as a float(f32).
pub fn rust_to_cpp_float_str<'a>(n: impl FloatCore + fmt::Display) -> Cow<'a, str> {
    match n {
        n if n.is_nan() => "-1.#IND00".into(),
        n if n.is_sign_positive() && n.is_infinite() => "1.#INF00".into(),
        n if n.is_sign_negative() && n.is_infinite() => "-1.#INF00".into(),
        n => format!("{n:.6}").into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_cpp_to_rust_float_str() {
        assert_eq!(cpp_to_rust_float_str("1.#IND"), "nan");
        assert_eq!(cpp_to_rust_float_str("1.#INF00"), "inf");
        assert_eq!(cpp_to_rust_float_str("-1.#INF"), "-inf");
        assert_eq!(cpp_to_rust_float_str("42.0"), "42.0");
    }

    #[test]
    fn test_rust_to_cpp_float_str() {
        assert_eq!(rust_to_cpp_float_str(f32::NAN), "-1.#IND00");
        assert_eq!(rust_to_cpp_float_str(f32::INFINITY), "1.#INF00");
        assert_eq!(rust_to_cpp_float_str(f32::NEG_INFINITY), "-1.#INF00");
        assert_eq!(rust_to_cpp_float_str(42.0), "42");
    }
}
