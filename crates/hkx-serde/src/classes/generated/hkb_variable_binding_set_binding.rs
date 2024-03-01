//! A Rust structure that implements a serializer/deserializer corresponding to `hkbVariableBindingSetBinding`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// In XML, it is enclosed in a `hkobject` tag
/// and the `class` attribute contains the C++ class nam
///
/// # Information on the original C++ class
/// -    size: 32
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbVariableBindingSetBinding<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbVariableBindingSetBinding"`: Name of this class.
    #[serde(default = "HkbVariableBindingSetBinding::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x4d592f72`: Unique value of this class.
    #[serde(default = "HkbVariableBindingSetBinding::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbVariableBindingSetBindingHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbVariableBindingSetBindingHkParam<'a>>
}

impl HkbVariableBindingSetBinding<'_> {
    /// Return `"hkbVariableBindingSetBinding"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbVariableBindingSetBinding".into()
    }

    /// Return `"0x4d592f72"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x4d592f72".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbVariableBindingSetBindingHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"memberPath"`
    /// -   type: `hkStringPtr`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "memberPath")]
    MemberPath(String),
    /// # Information on fields in the original C++ class
    /// -   name:`"memberClass"`
    /// -   type: `void*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memberClass", skip_serializing)]
    MemberClass(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"offsetInObjectPlusOne"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "offsetInObjectPlusOne", skip_serializing)]
    OffsetInObjectPlusOne(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"offsetInArrayPlusOne"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "offsetInArrayPlusOne", skip_serializing)]
    OffsetInArrayPlusOne(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"rootVariableIndex"`
    /// -   type: `hkInt32`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "rootVariableIndex", skip_serializing)]
    RootVariableIndex(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"variableIndex"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variableIndex")]
    VariableIndex(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"bitIndex"`
    /// -   type: `hkInt8`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bitIndex")]
    BitIndex(i8),
    /// # Information on fields in the original C++ class
    /// -   name:`"bindingType"`
    /// -   type: `enum BindingType`
    /// - offset: 25
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bindingType")]
    BindingType(BindingType),
    /// # Information on fields in the original C++ class
    /// -   name:`"memberType"`
    /// -   type: `enum unknown`
    /// - offset: 26
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memberType", skip_serializing)]
    MemberType(Unknown),
    /// # Information on fields in the original C++ class
    /// -   name:`"variableType"`
    /// -   type: `hkInt8`
    /// - offset: 27
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "variableType", skip_serializing)]
    VariableType(i8),
    /// # Information on fields in the original C++ class
    /// -   name:`"flags"`
    /// -   type: `flags unknown`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "flags", skip_serializing)]
    Flags(Unknown),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbVariableBindingSetBindingHkParam<'de>, "@name",
    ("memberPath" => MemberPath(String)),
    ("memberClass" => MemberClass(())),
    ("offsetInObjectPlusOne" => OffsetInObjectPlusOne(i32)),
    ("offsetInArrayPlusOne" => OffsetInArrayPlusOne(i32)),
    ("rootVariableIndex" => RootVariableIndex(i32)),
    ("variableIndex" => VariableIndex(i32)),
    ("bitIndex" => BitIndex(i8)),
    ("bindingType" => BindingType(BindingType)),
    ("memberType" => MemberType(Unknown)),
    ("variableType" => VariableType(i8)),
    ("flags" => Flags(Unknown)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum BindingType {
    #[serde(rename = "BINDING_TYPE_VARIABLE")]
    BindingTypeVariable = 0,
    #[serde(rename = "BINDING_TYPE_CHARACTER_PROPERTY")]
    BindingTypeCharacterProperty = 1,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InternalBindingFlags {
    #[serde(rename = "FLAG_NONE")]
    FlagNone = 0,
    #[serde(rename = "FLAG_OUTPUT")]
    FlagOutput = 1,
}
