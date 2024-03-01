//! A Rust structure that implements a serializer/deserializer corresponding to `hkbVariableInfo`, a class defined in C++
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
/// -    size: 6
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbVariableInfo<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbVariableInfo"`: Name of this class.
    #[serde(default = "HkbVariableInfo::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x9e746ba2`: Unique value of this class.
    #[serde(default = "HkbVariableInfo::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbVariableInfoHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbVariableInfoHkParam<'a>>
}

impl HkbVariableInfo<'_> {
    /// Return `"hkbVariableInfo"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbVariableInfo".into()
    }

    /// Return `"0x9e746ba2"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x9e746ba2".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbVariableInfoHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"role"`
    /// -   type: `struct hkbRoleAttribute`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "role")]
    Role(HkbRoleAttribute),
    /// # Information on fields in the original C++ class
    /// -   name:`"type"`
    /// -   type: `enum VariableType`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(VariableType),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbVariableInfoHkParam<'de>, "@name",
    ("role" => Role(HkbRoleAttribute)),
    ("type" => Type(VariableType)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum VariableType {
    #[serde(rename = "VARIABLE_TYPE_INVALID")]
    VariableTypeInvalid = -1,
    #[serde(rename = "VARIABLE_TYPE_BOOL")]
    VariableTypeBool = 0,
    #[serde(rename = "VARIABLE_TYPE_INT8")]
    VariableTypeInt8 = 1,
    #[serde(rename = "VARIABLE_TYPE_INT16")]
    VariableTypeInt16 = 2,
    #[serde(rename = "VARIABLE_TYPE_INT32")]
    VariableTypeInt32 = 3,
    #[serde(rename = "VARIABLE_TYPE_REAL")]
    VariableTypeReal = 4,
    #[serde(rename = "VARIABLE_TYPE_POINTER")]
    VariableTypePointer = 5,
    #[serde(rename = "VARIABLE_TYPE_VECTOR3")]
    VariableTypeVector3 = 6,
    #[serde(rename = "VARIABLE_TYPE_VECTOR4")]
    VariableTypeVector4 = 7,
    #[serde(rename = "VARIABLE_TYPE_QUATERNION")]
    VariableTypeQuaternion = 8,
}