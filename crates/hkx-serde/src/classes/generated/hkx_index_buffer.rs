//! A Rust structure that implements a serializer/deserializer corresponding to `hkxIndexBuffer`, a class defined in C++
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
/// -    size: 44
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkxIndexBuffer<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkxIndexBuffer"`: Name of this class.
    #[serde(default = "HkxIndexBuffer::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xc12c8197`: Unique value of this class.
    #[serde(default = "HkxIndexBuffer::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkxIndexBufferHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkxIndexBufferHkParam<'a>>
}

impl HkxIndexBuffer<'_> {
    /// Return `"hkxIndexBuffer"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkxIndexBuffer".into()
    }

    /// Return `"0xc12c8197"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xc12c8197".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxIndexBufferHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"indexType"`
    /// -   type: `enum IndexType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indexType")]
    IndexType(IndexType),
    /// # Information on fields in the original C++ class
    /// -   name:`"indices16"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indices16")]
    Indices16(Vec<u16>),
    /// # Information on fields in the original C++ class
    /// -   name:`"indices32"`
    /// -   type: `hkArray&lt;hkUint32&gt;`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indices32")]
    Indices32(Vec<u32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"vertexBaseOffset"`
    /// -   type: `hkUint32`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexBaseOffset")]
    VertexBaseOffset(u32),
    /// # Information on fields in the original C++ class
    /// -   name:`"length"`
    /// -   type: `hkUint32`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "length")]
    Length(u32),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkxIndexBufferHkParam<'de>, "@name",
    ("indexType" => IndexType(IndexType)),
    ("indices16" => Indices16(Vec<u16>)),
    ("indices32" => Indices32(Vec<u32>)),
    ("vertexBaseOffset" => VertexBaseOffset(u32)),
    ("length" => Length(u32)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum IndexType {
    #[serde(rename = "INDEX_TYPE_INVALID")]
    IndexTypeInvalid = 0,
    #[serde(rename = "INDEX_TYPE_TRI_LIST")]
    IndexTypeTriList = 1,
    #[serde(rename = "INDEX_TYPE_TRI_STRIP")]
    IndexTypeTriStrip = 2,
    #[serde(rename = "INDEX_TYPE_TRI_FAN")]
    IndexTypeTriFan = 3,
    #[serde(rename = "INDEX_TYPE_MAX_ID")]
    IndexTypeMaxId = 4,
}
