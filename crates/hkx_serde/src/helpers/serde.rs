//! quick_xml convenience functions.

/// pretty formatted XML String
pub fn to_string_pretty_xml(
    serializer: &impl serde::Serialize,
    indent_char: char,
    indent_size: usize,
) -> Result<String, quick_xml::DeError> {
    let mut xml = String::new();
    let mut ser = quick_xml::se::Serializer::new(&mut xml);
    ser.indent(indent_char, indent_size);
    serializer.serialize(ser)?;
    Ok(xml)
}
