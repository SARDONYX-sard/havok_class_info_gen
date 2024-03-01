//! A Rust structure that implements a serializer/deserializer corresponding to `hkMultipleVertexBufferVertexBufferInfo`, a class defined in C++
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
/// -    size: 12
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkMultipleVertexBufferVertexBufferInfo<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkMultipleVertexBufferVertexBufferInfo"`: Name of this class.
    #[serde(default = "HkMultipleVertexBufferVertexBufferInfo::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xdafbe0e6`: Unique value of this class.
    #[serde(default = "HkMultipleVertexBufferVertexBufferInfo::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkMultipleVertexBufferVertexBufferInfoHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkMultipleVertexBufferVertexBufferInfoHkParam<'a>>
}

impl HkMultipleVertexBufferVertexBufferInfo<'_> {
    /// Return `"hkMultipleVertexBufferVertexBufferInfo"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkMultipleVertexBufferVertexBufferInfo".into()
    }

    /// Return `"0xdafbe0e6"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xdafbe0e6".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMultipleVertexBufferVertexBufferInfoHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"vertexBuffer"`
    /// -   type: `struct hkMeshVertexBuffer*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexBuffer")]
    VertexBuffer(Box<HkMeshVertexBuffer>),
    /// # Information on fields in the original C++ class
    /// -   name:`"lockedVertices"`
    /// -   type: `void*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "lockedVertices", skip_serializing)]
    LockedVertices(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"isLocked"`
    /// -   type: `hkBool`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isLocked")]
    IsLocked(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkMultipleVertexBufferVertexBufferInfoHkParam<'de>, "@name",
    ("vertexBuffer" => VertexBuffer(Box<HkMeshVertexBuffer>)),
    ("lockedVertices" => LockedVertices(())),
    ("isLocked" => IsLocked(bool)),
}
