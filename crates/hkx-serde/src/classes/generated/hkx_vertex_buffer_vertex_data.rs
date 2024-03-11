//! A Rust structure that implements a serializer/deserializer corresponding to `hkxVertexBufferVertexData`, a class defined in C++
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
/// -    size: 84
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkxVertexBufferVertexData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkxVertexBufferVertexData"`: The original C++ class name.
    #[serde(default = "HkxVertexBufferVertexData::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xd72b6fd0`: Unique value of this class.
    #[serde(default = "HkxVertexBufferVertexData::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkxVertexBufferVertexDataHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkxVertexBufferVertexDataHkParam<'a>>
}

impl HkxVertexBufferVertexData<'_> {
    /// Return `"hkxVertexBufferVertexData"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkxVertexBufferVertexData".into()
    }

    /// Return `"0xd72b6fd0"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xd72b6fd0".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxVertexBufferVertexDataHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"vectorData"`
    /// -   type: `hkArray&lt;hkVector4&gt;`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vectorData")]
    VectorData(Vec<Vector4<f32>>),
    /// # Field information in the original C++ class
    /// -   name:`"floatData"`
    /// -   type: `hkArray&lt;hkReal&gt;`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "floatData")]
    FloatData(Vec<Primitive<f32>>),
    /// # Field information in the original C++ class
    /// -   name:`"uint32Data"`
    /// -   type: `hkArray&lt;hkUint32&gt;`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "uint32Data")]
    Uint32Data(Vec<Primitive<u32>>),
    /// # Field information in the original C++ class
    /// -   name:`"uint16Data"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "uint16Data")]
    Uint16Data(Vec<Primitive<u16>>),
    /// # Field information in the original C++ class
    /// -   name:`"uint8Data"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "uint8Data")]
    Uint8Data(Vec<Primitive<u8>>),
    /// # Field information in the original C++ class
    /// -   name:`"numVerts"`
    /// -   type: `hkUint32`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numVerts")]
    NumVerts(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"vectorStride"`
    /// -   type: `hkUint32`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vectorStride")]
    VectorStride(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"floatStride"`
    /// -   type: `hkUint32`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "floatStride")]
    FloatStride(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"uint32Stride"`
    /// -   type: `hkUint32`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "uint32Stride")]
    Uint32Stride(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"uint16Stride"`
    /// -   type: `hkUint32`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "uint16Stride")]
    Uint16Stride(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"uint8Stride"`
    /// -   type: `hkUint32`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "uint8Stride")]
    Uint8Stride(Primitive<u32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkxVertexBufferVertexDataHkParam<'de>, "@name",
    ("vectorData" => VectorData(Vec<Vector4<f32>>)),
    ("floatData" => FloatData(Vec<Primitive<f32>>)),
    ("uint32Data" => Uint32Data(Vec<Primitive<u32>>)),
    ("uint16Data" => Uint16Data(Vec<Primitive<u16>>)),
    ("uint8Data" => Uint8Data(Vec<Primitive<u8>>)),
    ("numVerts" => NumVerts(Primitive<u32>)),
    ("vectorStride" => VectorStride(Primitive<u32>)),
    ("floatStride" => FloatStride(Primitive<u32>)),
    ("uint32Stride" => Uint32Stride(Primitive<u32>)),
    ("uint16Stride" => Uint16Stride(Primitive<u32>)),
    ("uint8Stride" => Uint8Stride(Primitive<u32>)),
}
