//! A Rust structure that implements a serializer/deserializer corresponding to `hkMeshVertexBuffer`, a class defined in C++
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
/// -    size: 8
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkMeshVertexBuffer<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkMeshVertexBuffer"`: The original C++ class name.
    #[serde(default = "HkMeshVertexBuffer::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x534b08c8`: Unique value of this class.
    #[serde(default = "HkMeshVertexBuffer::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkMeshVertexBufferHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkMeshVertexBufferHkParam<'a>>
}

impl HkMeshVertexBuffer<'_> {
    /// Return `"hkMeshVertexBuffer"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkMeshVertexBuffer".into()
    }

    /// Return `"0x534b08c8"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x534b08c8".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMeshVertexBufferHkParam<'a> {
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkMeshVertexBufferHkParam<'de>, "@name",
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Flags {
    #[serde(rename = "ACCESS_READ")]
    AccessRead = 1,
    #[serde(rename = "ACCESS_WRITE")]
    AccessWrite = 2,
    #[serde(rename = "ACCESS_READ_WRITE")]
    AccessReadWrite = 3,
    #[serde(rename = "ACCESS_WRITE_DISCARD")]
    AccessWriteDiscard = 4,
    #[serde(rename = "ACCESS_ELEMENT_ARRAY")]
    AccessElementArray = 8,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum LockResult {
    #[serde(rename = "RESULT_FAILURE")]
    ResultFailure = 0,
    #[serde(rename = "RESULT_SUCCESS")]
    ResultSuccess = 1,
    #[serde(rename = "RESULT_IN_USE")]
    ResultInUse = 2,
}
