use crate::{
    cpp_type_parser::cpp_type_parser_for_struct::parse_cpp_type_to_non_wrapper,
    generators::{
        aliases::{FieldMap, LifeTimeMap},
        lifetime_manager::{add_lifetime_to_array, get_type_with_lifetime},
    },
    hkxcmd_parser::MemberInfo,
};
use convert_case::{Case, Casing};
use indexmap::IndexMap;

/// Generates C++ fields to Rust enum visitor
///
/// # Returns
/// `(generated code, IndexMap<"C++ field name", ("rust enum tag name", "rust type name")>)`
///
/// # Note
/// - This function is used in two patterns
/// 1. extraction of fields of the parent class
/// 2. extraction of fields of its own class
pub fn generate_struct_fields<'a>(
    members: &'a [MemberInfo],
    life_time_map: Option<&LifeTimeMap>,
) -> (String, FieldMap<'a>) {
    let mut fields = IndexMap::new();
    let mut rust_code = String::new();

    for member in members {
        let MemberInfo {
            name: member_name,
            type_name,
            offset_x86: offset,
            flags,
            ..
        } = member;

        // Rust Field type is non wrapper type
        // e.g. `i16`
        let (_, rust_type) = parse_cpp_type_to_non_wrapper(type_name).unwrap();

        let rust_type = match type_name.starts_with("hkArray") || type_name.starts_with("struct") {
            true => add_lifetime_to_array(&rust_type, life_time_map),
            false => get_type_with_lifetime(&rust_type, life_time_map)
                .unwrap_or(rust_type.to_string())
                .to_string(),
        };

        // These used for documentation purposes only
        let type_name = type_name.replace("&lt;", "<").replace("&gt;", ">");

        let tag_name = member_name.to_case(Case::Snake);
        // Enum tag name(If the first letter is a number, escape it with `_`.)
        let tag_name = match member_name.chars().next().map_or(false, |c| c.is_numeric())
            || matches!(member_name.as_str(), "type" | "enum")
        {
            true => format!("_{tag_name}"),
            false => tag_name,
        };
        rust_code.push_str(&format!(
            r#"    /// # C++ Class Fields Info
    /// -   name:`"{member_name}"`
    /// -   type: `{type_name}`
    /// - offset: {offset}
    /// -  flags: `{flags}`
    pub {tag_name}: {rust_type},
"#
        ));
        fields.insert(member_name, (tag_name, rust_type.into()));
    }

    (rust_code, fields)
}
