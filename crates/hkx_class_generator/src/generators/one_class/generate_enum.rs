use convert_case::{Case, Casing as _};

use crate::hkxcmd_parser::{Enum, EnumItem};

/// Generate flags and C++ enum(If exists)
pub fn generate_enums(enum_info: &Enum) -> String {
    let mut rust_code = String::new();

    let Enum {
        name: enum_name,
        enum_item: enum_pair_info,
        ..
    } = enum_info;

    let enum_name = enum_name.to_case(Case::Pascal);
    // Generate one enum template prefix
    rust_code.push_str(&format!(
        r#"
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum {enum_name} {{
"#
    ));

    // Generate tag & value pairs
    for (
        index,
        EnumItem {
            name: tag_name,
            value: enum_value,
            ..
        },
    ) in enum_pair_info.iter().enumerate()
    {
        let rust_enum_field_name = &tag_name.to_case(Case::Pascal);
        let default_attr = if index == 0 { "\n    #[default]" } else { "" };
        rust_code.push_str(&format!(
            r#"    #[serde(rename = "{tag_name}")]{default_attr}
    {rust_enum_field_name} = {enum_value},
"#
        ));
    }

    // Generate one enum template postfix
    rust_code.push_str("}\n");

    rust_code
}
