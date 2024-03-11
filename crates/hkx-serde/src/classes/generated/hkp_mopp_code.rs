//! A Rust structure that implements a serializer/deserializer corresponding to `hkpMoppCode`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::hk_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// In XML, it is enclosed in a `hkobject` tag
/// and the `class` attribute contains the C++ class nam
///
/// # Information on the original C++ class
/// -    size: 48
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpMoppCode<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpMoppCode"`: The original C++ class name.
    #[serde(default = "HkpMoppCode::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x924c2661`: Unique value of this class.
    #[serde(default = "HkpMoppCode::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpMoppCodeHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpMoppCodeHkParam<'a>>
}

impl HkpMoppCode<'_> {
    /// Return `"hkpMoppCode"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpMoppCode".into()
    }

    /// Return `"0x924c2661"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x924c2661".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpMoppCodeHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"info"`
    /// -   type: `struct hkpMoppCodeCodeInfo`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "info")]
    Info(HkpMoppCodeCodeInfo),
    /// # Field information in the original C++ class
    /// -   name:`"data"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(Vec<Primitive<u8>>),
    /// # Field information in the original C++ class
    /// -   name:`"buildType"`
    /// -   type: `enum BuildType`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "buildType")]
    BuildType(BuildType),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpMoppCodeHkParam<'de>, "@name",
    ("info" => Info(HkpMoppCodeCodeInfo)),
    ("data" => Data(Vec<Primitive<u8>>)),
    ("buildType" => BuildType(BuildType)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum BuildType {
    #[serde(rename = "BUILT_WITH_CHUNK_SUBDIVISION")]
    BuiltWithChunkSubdivision = 0,
    #[serde(rename = "BUILT_WITHOUT_CHUNK_SUBDIVISION")]
    BuiltWithoutChunkSubdivision = 1,
    #[serde(rename = "BUILD_NOT_SET")]
    BuildNotSet = 2,
}
