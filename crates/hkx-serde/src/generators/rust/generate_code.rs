use super::cpp_type_parser::parse_cpp_type;
use crate::{
    flag_values::FlagValues,
    parse_rpt::{ClassInfo, MemberInfo},
};
use convert_case::{Case, Casing};
use std::collections::HashMap;

pub fn generate_code(class: &ClassInfo) -> String {
    let mut rust_code = String::new();
    let ClassInfo {
        name: class_name,
        signature,
        ..
    } = &class;
    let rust_struct_name = class_name.to_case(Case::Pascal);

    // Struct definition
    rust_code.push_str(&format!(
        r#"use super::*;
use serde::{{Deserialize, Serialize}};
use std::borrow::Cow;

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct {rust_struct_name}<'a> {{
    #[serde(borrow)]
    #[serde(rename = "@name")]
    /// #0106, others
    pub name: Cow<'a, str>,
    #[serde(borrow)]
    #[serde(default = "{rust_struct_name}::class_name")]
    #[serde(rename = "@class")]
    /// "{class_name}"
    pub class: Cow<'a, str>,
    #[serde(borrow)]
    #[serde(default = "{rust_struct_name}::signature")]
    #[serde(rename = "@signature")]
    /// Fixed value unique to each class: `{signature}`
    pub signature: Cow<'a, str>,

"#,
    ));

    let mut field = HashMap::new();
    // Member definitions
    for member in &class.members {
        if member.flags.contains(FlagValues::SERIALIZE_IGNORED) {
            continue;
        }
        let MemberInfo {
            name: member_name,
            type_name,
            offset,
            ..
        } = &member;
        let (_, rust_type) = parse_cpp_type(type_name).unwrap();
        field.insert(type_name, (member_name, rust_type));

        rust_code.push_str(
            r#"}}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkparam")]
pub struct HkParam<'a> {{
    #[serde(rename = "@name")]
    /// class fields
    pub name: Field,
    #[serde(rename = "@numelements")]
    /// `self.hkcstrings.len()`
    pub numelements: usize,
    #[serde(borrow)]
    #[serde(default)]
    #[serde(rename = "hkcstring")]
    pub hkcstrings: Vec<Cow<'a, str>>,
}}
"#,
        );

        let tag_name = member_name.to_case(Case::Pascal);
        rust_code.push_str(&format!(
            r#"


                /// # C++ Info
    /// - field name:`"{member_name}"`
    /// -     offset: {offset}`
    #[serde(rename = "{member_name}")]
    {tag_name},
"#
        ));
    }

    rust_code.push_str(&format!(
        r#"}}

impl {rust_struct_name}<'_> {{
    #[inline]
    pub fn class_name() -> Cow<'static, str> {{
        "{rust_struct_name}".into()
    }}

    #[inline]
    pub fn signature() -> Cow<'static, str> {{
        "0x{signature:x}".into()
    }}
}}
"#
    ));

    // Enum definitions
    for (enum_name, enum_info) in &class.enums {
        rust_code.push_str(&format!(
            r#"
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum {enum_name} {{
"#
        ));

        for (tag_name, enum_value) in enum_info {
            let rust_enum_field_name = &tag_name.to_case(Case::Pascal);
            rust_code.push_str(&format!(
                r#"    #[serde(rename = "{tag_name}")]
    {rust_enum_field_name} = {enum_value},
"#
            ));
        }

        rust_code.push_str("}\n");
    }

    rust_code
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hk_types::Type;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_generate_rust_code() {
        // Create a mock class
        let members = vec![
            MemberInfo {
                name: "eventNames".into(),
                type_name: "Vec<Cow<'a, str>>".into(),
                flags: FlagValues::FLAGS_NONE,
                ..Default::default()
            },
            MemberInfo {
                name: "attributeNames".into(),
                type_name: "Vec<Cow<'a, str>>".into(),
                flags: FlagValues::FLAGS_NONE,
                ..Default::default()
            },
            MemberInfo {
                name: "variableNames".into(),
                type_name: "Vec<Cow<'a, str>>".into(),
                flags: FlagValues::FLAGS_NONE,
                ..Default::default()
            },
            MemberInfo {
                name: "characterPropertyNames".into(),
                hk_type: Type::Void,
                flags: FlagValues::SERIALIZE_IGNORED,
                ..Default::default()
            },
        ];
        let enums = vec![
            ("Enum1", vec![("Value1".into(), 0), ("Value2".into(), 1)]),
            ("Enum2", vec![("Value3".into(), 2)]),
        ];
        let class = ClassInfo {
            name: "TestClass".into(),
            members,
            enums,
            signature: 0xc713064e,
            ..Default::default()
        };

        // Generate Rust code
        let generated_code = generate_code(&class);

        // Expected Rust code
        let expected_code = r#"use super::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbBehaviorGraphStringData<'a> {
    #[serde(borrow)]
    #[serde(rename = "@name")]
    /// #0106
    pub name: Cow<'a, str>,
    #[serde(borrow)]
    #[serde(default = "HkbBehaviorGraphStringData::class_name")]
    #[serde(rename = "@class")]
    /// "hkbBehaviorGraphStringData"
    pub class: Cow<'a, str>,
    #[serde(borrow)]
    #[serde(default = "HkbBehaviorGraphStringData::signature")]
    #[serde(rename = "@signature")]
    /// Fixed value unique to each class: `0xc713064e`
    pub signature: Cow<'a, str>,
    /// The `"hkparam"` field containing the hkcstring vector
    pub hkparam: Vec<HkParam<'a>>,
}

impl HkbBehaviorGraphStringData<'_> {
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbBehaviorGraphStringData".into()
    }

    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xc713064e".into()
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Names {
    #[default]
    EventNames,
    AttributeNames,
    VariableNames,
    CharacterPropertyNames,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkparam")]
pub struct HkParam<'a> {
    #[serde(rename = "@name")]
    /// `"eventNames"` | `"attributeNames"` | `"variableNames"` | `"characterPropertyNames"`
    pub name: Names,
    #[serde(rename = "@numelements")]
    /// `self.hkcstrings.len()`
    pub numelements: usize,
    #[serde(borrow)]
    #[serde(default)]
    #[serde(rename = "hkcstring")]
    pub hkcstrings: Vec<Cow<'a, str>>,
}
"#;

        // Assert equality
        assert_eq!(generated_code, expected_code);
    }
}
