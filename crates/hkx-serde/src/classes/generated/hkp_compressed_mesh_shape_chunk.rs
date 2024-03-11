//! A Rust structure that implements a serializer/deserializer corresponding to `hkpCompressedMeshShapeChunk`, a class defined in C++
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
/// -    size: 80
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 4
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpCompressedMeshShapeChunk<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpCompressedMeshShapeChunk"`: The original C++ class name.
    #[serde(default = "HkpCompressedMeshShapeChunk::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x5d0d67bd`: Unique value of this class.
    #[serde(default = "HkpCompressedMeshShapeChunk::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpCompressedMeshShapeChunkHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpCompressedMeshShapeChunkHkParam<'a>>
}

impl HkpCompressedMeshShapeChunk<'_> {
    /// Return `"hkpCompressedMeshShapeChunk"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpCompressedMeshShapeChunk".into()
    }

    /// Return `"0x5d0d67bd"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x5d0d67bd".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpCompressedMeshShapeChunkHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"offset"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "offset")]
    Offset(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"vertices"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertices")]
    Vertices(Vec<Primitive<u16>>),
    /// # Field information in the original C++ class
    /// -   name:`"indices"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indices")]
    Indices(Vec<Primitive<u16>>),
    /// # Field information in the original C++ class
    /// -   name:`"stripLengths"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "stripLengths")]
    StripLengths(Vec<Primitive<u16>>),
    /// # Field information in the original C++ class
    /// -   name:`"weldingInfo"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "weldingInfo")]
    WeldingInfo(Vec<Primitive<u16>>),
    /// # Field information in the original C++ class
    /// -   name:`"materialInfo"`
    /// -   type: `hkUint32`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialInfo")]
    MaterialInfo(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"reference"`
    /// -   type: `hkUint16`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "reference")]
    Reference(Primitive<u16>),
    /// # Field information in the original C++ class
    /// -   name:`"transformIndex"`
    /// -   type: `hkUint16`
    /// - offset: 70
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformIndex")]
    TransformIndex(Primitive<u16>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpCompressedMeshShapeChunkHkParam<'de>, "@name",
    ("offset" => Offset(Vector4<f32>)),
    ("vertices" => Vertices(Vec<Primitive<u16>>)),
    ("indices" => Indices(Vec<Primitive<u16>>)),
    ("stripLengths" => StripLengths(Vec<Primitive<u16>>)),
    ("weldingInfo" => WeldingInfo(Vec<Primitive<u16>>)),
    ("materialInfo" => MaterialInfo(Primitive<u32>)),
    ("reference" => Reference(Primitive<u16>)),
    ("transformIndex" => TransformIndex(Primitive<u16>)),
}
