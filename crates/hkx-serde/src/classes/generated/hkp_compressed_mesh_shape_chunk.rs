//! A Rust structure that implements a serializer/deserializer corresponding to `hkpCompressedMeshShapeChunk`, a class defined in C++
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

    /// `"hkpCompressedMeshShapeChunk"`: Name of this class.
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
    /// Return `"hkpCompressedMeshShapeChunk"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpCompressedMeshShapeChunk".into()
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
    /// # Information on fields in the original C++ class
    /// -   name:`"offset"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "offset")]
    Offset(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"vertices"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertices")]
    Vertices(Vec<u16>),
    /// # Information on fields in the original C++ class
    /// -   name:`"indices"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indices")]
    Indices(Vec<u16>),
    /// # Information on fields in the original C++ class
    /// -   name:`"stripLengths"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "stripLengths")]
    StripLengths(Vec<u16>),
    /// # Information on fields in the original C++ class
    /// -   name:`"weldingInfo"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "weldingInfo")]
    WeldingInfo(Vec<u16>),
    /// # Information on fields in the original C++ class
    /// -   name:`"materialInfo"`
    /// -   type: `hkUint32`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialInfo")]
    MaterialInfo(u32),
    /// # Information on fields in the original C++ class
    /// -   name:`"reference"`
    /// -   type: `hkUint16`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "reference")]
    Reference(u16),
    /// # Information on fields in the original C++ class
    /// -   name:`"transformIndex"`
    /// -   type: `hkUint16`
    /// - offset: 70
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformIndex")]
    TransformIndex(u16),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpCompressedMeshShapeChunkHkParam<'de>, "@name",
    ("offset" => Offset(cgmath::Vector4<f32>)),
    ("vertices" => Vertices(Vec<u16>)),
    ("indices" => Indices(Vec<u16>)),
    ("stripLengths" => StripLengths(Vec<u16>)),
    ("weldingInfo" => WeldingInfo(Vec<u16>)),
    ("materialInfo" => MaterialInfo(u32)),
    ("reference" => Reference(u16)),
    ("transformIndex" => TransformIndex(u16)),
}
