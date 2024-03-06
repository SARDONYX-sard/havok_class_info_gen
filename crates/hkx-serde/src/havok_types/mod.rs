pub mod array;
pub mod float;

fn string(text: &str) -> Option<&str> {
    const NULL_CHAR: &str = "\u{2400}";
    match text == NULL_CHAR {
        true => None,
        false => Some(str::trim(text)),
    }
}
