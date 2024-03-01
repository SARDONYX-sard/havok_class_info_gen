//! A Rust structure that implements a serializer/deserializer corresponding to `hkpGenericConstraintDataScheme`, a class defined in C++
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
/// -    size: 64
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpGenericConstraintDataScheme<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpGenericConstraintDataScheme"`: Name of this class.
    #[serde(default = "HkpGenericConstraintDataScheme::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x11fd6f6c`: Unique value of this class.
    #[serde(default = "HkpGenericConstraintDataScheme::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpGenericConstraintDataSchemeHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpGenericConstraintDataSchemeHkParam<'a>>
}

impl HkpGenericConstraintDataScheme<'_> {
    /// Return `"hkpGenericConstraintDataScheme"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpGenericConstraintDataScheme".into()
    }

    /// Return `"0x11fd6f6c"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x11fd6f6c".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpGenericConstraintDataSchemeHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"info"`
    /// -   type: `struct hkpGenericConstraintDataSchemeConstraintInfo`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "info", skip_serializing)]
    Info(HkpGenericConstraintDataSchemeConstraintInfo),
    /// # Information on fields in the original C++ class
    /// -   name:`"data"`
    /// -   type: `hkArray&lt;hkVector4&gt;`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(Vec<cgmath::Vector4<f32>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"commands"`
    /// -   type: `hkArray&lt;hkInt32&gt;`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "commands")]
    Commands(Vec<i32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"modifiers"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "modifiers", skip_serializing)]
    Modifiers(Vec<()>),
    /// # Information on fields in the original C++ class
    /// -   name:`"motors"`
    /// -   type: `hkArray&lt;hkpConstraintMotor*&gt;`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "motors")]
    Motors(Vec<Box<HkpConstraintMotor>>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpGenericConstraintDataSchemeHkParam<'de>, "@name",
    ("info" => Info(HkpGenericConstraintDataSchemeConstraintInfo)),
    ("data" => Data(Vec<cgmath::Vector4<f32>>)),
    ("commands" => Commands(Vec<i32>)),
    ("modifiers" => Modifiers(Vec<()>)),
    ("motors" => Motors(Vec<Box<HkpConstraintMotor>>)),
}