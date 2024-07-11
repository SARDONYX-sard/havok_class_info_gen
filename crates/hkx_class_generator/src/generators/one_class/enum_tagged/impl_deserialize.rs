use crate::generators::aliases::FieldMap;

/// tagged enum(Returns code that implements [`serde::Deserializer`], which branches processing for each value of
/// the `name` attribute of XML, which is each field of the C++ class.
pub fn generate_impl_deserialize(
    rust_class_name_with_life_time: &str,
    fields: &FieldMap,
) -> String {
    let mut rust_code = String::new();

    let rust_class_name_with_life_time = rust_class_name_with_life_time.replace("'a", "'de");
    rust_code.push_str(&format!(
        r#"
// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {{
    {rust_class_name_with_life_time}, "@name",
"#
    ));

    for (member_name, (tag_name, rust_type)) in fields {
        let rust_type = rust_type.replace("'a", "'de");
        rust_code.push_str(&format!(
            r#"    ("{member_name}" => {tag_name}({rust_type})),
"#
        ));
    }

    rust_code.push_str("}\n");
    rust_code
}
