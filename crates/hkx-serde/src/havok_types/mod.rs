pub mod float;
pub mod hk_array;
pub mod vector4;

pub use vector4::Vector4;

pub fn string(text: &str) -> Option<&str> {
    const NULL_CHAR: &str = "\u{2400}";
    match text == NULL_CHAR {
        true => None,
        false => Some(str::trim(text)),
    }
}
