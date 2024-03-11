//! A Rust structure that implements a serializer/deserializer corresponding to `hkMemoryMeshVertexBuffer`, a class defined in C++
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
/// -    size: 424
/// -  vtable: true
/// -  parent: hkMeshVertexBuffer/`534b08c8`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkMemoryMeshVertexBuffer<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkMemoryMeshVertexBuffer"`: The original C++ class name.
    #[serde(default = "HkMemoryMeshVertexBuffer::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xa2e50753`: Unique value of this class.
    #[serde(default = "HkMemoryMeshVertexBuffer::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkMemoryMeshVertexBufferHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkMemoryMeshVertexBufferHkParam<'a>>
}

impl HkMemoryMeshVertexBuffer<'_> {
    /// Return `"hkMemoryMeshVertexBuffer"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkMemoryMeshVertexBuffer".into()
    }

    /// Return `"0xa2e50753"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xa2e50753".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMemoryMeshVertexBufferHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"format"`
    /// -   type: `struct hkVertexFormat`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "format")]
    Format(HkVertexFormat),
    /// # Field information in the original C++ class
    /// -   name:`"elementOffsets"`
    /// -   type: `hkInt32[32]`
    /// - offset: 268
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "elementOffsets")]
    ElementOffsets([Primitive<i32>; 32]),
    /// # Field information in the original C++ class
    /// -   name:`"memory"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 396
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "memory")]
    Memory(Vec<Primitive<u8>>),
    /// # Field information in the original C++ class
    /// -   name:`"vertexStride"`
    /// -   type: `hkInt32`
    /// - offset: 408
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexStride")]
    VertexStride(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"locked"`
    /// -   type: `hkBool`
    /// - offset: 412
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "locked")]
    Locked(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"numVertices"`
    /// -   type: `hkInt32`
    /// - offset: 416
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numVertices")]
    NumVertices(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"isBigEndian"`
    /// -   type: `hkBool`
    /// - offset: 420
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isBigEndian")]
    IsBigEndian(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"isSharable"`
    /// -   type: `hkBool`
    /// - offset: 421
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isSharable")]
    IsSharable(Primitive<bool>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkMemoryMeshVertexBufferHkParam<'de>, "@name",
    ("format" => Format(HkVertexFormat)),
    ("elementOffsets" => ElementOffsets([Primitive<i32>; 32])),
    ("memory" => Memory(Vec<Primitive<u8>>)),
    ("vertexStride" => VertexStride(Primitive<i32>)),
    ("locked" => Locked(Primitive<bool>)),
    ("numVertices" => NumVertices(Primitive<i32>)),
    ("isBigEndian" => IsBigEndian(Primitive<bool>)),
    ("isSharable" => IsSharable(Primitive<bool>)),
}
