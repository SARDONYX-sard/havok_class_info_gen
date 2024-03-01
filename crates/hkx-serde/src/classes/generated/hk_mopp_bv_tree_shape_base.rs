//! A Rust structure that implements a serializer/deserializer corresponding to `hkMoppBvTreeShapeBase`, a class defined in C++
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
/// -    size: 48
/// -  vtable: true
/// -  parent: hkpBvTreeShape/`a823d623`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkMoppBvTreeShapeBase<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkMoppBvTreeShapeBase"`: Name of this class.
    #[serde(default = "HkMoppBvTreeShapeBase::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x7c338c66`: Unique value of this class.
    #[serde(default = "HkMoppBvTreeShapeBase::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkMoppBvTreeShapeBaseHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkMoppBvTreeShapeBaseHkParam<'a>>
}

impl HkMoppBvTreeShapeBase<'_> {
    /// Return `"hkMoppBvTreeShapeBase"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkMoppBvTreeShapeBase".into()
    }

    /// Return `"0x7c338c66"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x7c338c66".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMoppBvTreeShapeBaseHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"code"`
    /// -   type: `struct hkpMoppCode*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "code")]
    Code(Box<HkpMoppCode>),
    /// # Information on fields in the original C++ class
    /// -   name:`"moppData"`
    /// -   type: `void*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "moppData", skip_serializing)]
    MoppData(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"moppDataSize"`
    /// -   type: `hkUint32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "moppDataSize", skip_serializing)]
    MoppDataSize(u32),
    /// # Information on fields in the original C++ class
    /// -   name:`"codeInfoCopy"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "codeInfoCopy", skip_serializing)]
    CodeInfoCopy(cgmath::Vector4<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkMoppBvTreeShapeBaseHkParam<'de>, "@name",
    ("code" => Code(Box<HkpMoppCode>)),
    ("moppData" => MoppData(())),
    ("moppDataSize" => MoppDataSize(u32)),
    ("codeInfoCopy" => CodeInfoCopy(cgmath::Vector4<f32>)),
}
