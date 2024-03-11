use super::cpp_type_parser::parse_cpp_type;
use crate::{
    flag_values::FlagValues,
    parse_rpt::{ClassInfo, MemberInfo},
};
use convert_case::{Case, Casing};
use indexmap::IndexMap;

pub fn generate_code(class: &ClassInfo) -> String {
    let mut rust_code = String::new();
    let ClassInfo {
        name: class_name,
        signature,
        size,
        vtable,
        parent,
        version,
        ..
    } = &class;
    let rust_struct_name = class_name.to_case(Case::Pascal);

    // Parent class information
    let parent_default = ("None".into(), 0);
    let (parent_name, parent_signature) = (parent.as_ref()).unwrap_or(&parent_default);

    // As long as everything is exported when mod.rs is automatically generated,
    // each structure and enum must be given a unique name.
    let hkparam_name = format!("{rust_struct_name}HkParam");

    // Struct definition
    rust_code.push_str(&format!(
        r#"//! A Rust structure that implements a serializer/deserializer corresponding to `{class_name}`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{{Deserialize, Serialize}};
use std::borrow::Cow;

/// In XML, it is enclosed in a `hkobject` tag
/// and the `class` attribute contains the C++ class nam
///
/// # Information on the original C++ class
/// -    size: {size}
/// -  vtable: {vtable}
/// -  parent: {parent_name}/`{parent_signature:x}`(Non prefix hex signature)
/// - version: {version}
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct {rust_struct_name}<'a> {{
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"{class_name}"`: The original C++ class name.
    #[serde(default = "{rust_struct_name}::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x{signature:x}`: Unique value of this class.
    #[serde(default = "{rust_struct_name}::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<{hkparam_name}<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<{hkparam_name}<'a>>
}}
"#,
    ));

    rust_code.push_str(&format!(
        r#"
impl {rust_struct_name}<'_> {{
    /// Return `"{class_name}"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {{
        "{class_name}".into()
    }}

    /// Return `"0x{signature:x}"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {{
        "0x{signature:x}".into()
    }}
}}
"#
    ));

    rust_code.push_str(&format!(
        r#"
/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum {hkparam_name}<'a> {{
"#
    ));

    let mut field = IndexMap::new();
    // Member definitions
    for member in &class.members {
        let mut skip_serializing_attr = String::new();
        if member.flags.contains(FlagValues::SERIALIZE_IGNORED) {
            skip_serializing_attr.push_str(", skip_serializing")
        }
        let flags = member.flags.human_readable();

        let MemberInfo {
            name: member_name,
            type_name,
            offset,
            ..
        } = &member;
        let (_, rust_type) = parse_cpp_type(type_name).unwrap();

        // Enum tag name
        let tag_name = member_name.to_case(Case::Pascal);
        rust_code.push_str(&format!(
            r#"    /// # Field information in the original C++ class
    /// -   name:`"{member_name}"`
    /// -   type: `{type_name}`
    /// - offset: {offset}
    /// -  flags: `{flags}`
    #[serde(rename = "{member_name}"{skip_serializing_attr})]
    {tag_name}({rust_type}),
"#
        ));
        field.insert(member_name, (tag_name, rust_type));
    }

    rust_code.push_str(&format!(
        r#"}}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {{
    {hkparam_name}<'de>, "@name",
"#
    )); // Env C++ field enum
    for (member_name, (tag_name, rust_type)) in field {
        rust_code.push_str(&format!(
            r#"    ("{member_name}" => {tag_name}({rust_type})),
"#
        ));
    }
    rust_code.push_str("}\n");

    // field Type Enum definitions(If exists)
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

        let generated_code = generate_code(&class);

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

        assert_eq!(generated_code, expected_code);
    }
}
