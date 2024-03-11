//! A Rust structure that implements a serializer/deserializer corresponding to `hkxVertexDescriptionElementDecl`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// In XML, it is enclosed in a `hkobject` tag
/// and the `class` attribute contains the C++ class nam
///
/// # Information on the original C++ class
/// -    size: 16
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 2
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkxVertexDescriptionElementDecl<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkxVertexDescriptionElementDecl"`: The original C++ class name.
    #[serde(default = "HkxVertexDescriptionElementDecl::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x483a429b`: Unique value of this class.
    #[serde(default = "HkxVertexDescriptionElementDecl::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkxVertexDescriptionElementDeclHkParam>,
}

impl HkxVertexDescriptionElementDecl<'_> {
    /// Return `"hkxVertexDescriptionElementDecl"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkxVertexDescriptionElementDecl".into()
    }

    /// Return `"0x483a429b"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x483a429b".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxVertexDescriptionElementDeclHkParam {
    /// # Field information in the original C++ class
    /// -   name:`"byteOffset"`
    /// -   type: `hkUint32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "byteOffset")]
    ByteOffset(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"type"`
    /// -   type: `enum DataType`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(DataType),
    /// # Field information in the original C++ class
    /// -   name:`"usage"`
    /// -   type: `enum DataUsage`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "usage")]
    Usage(DataUsage),
    /// # Field information in the original C++ class
    /// -   name:`"byteStride"`
    /// -   type: `hkUint32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "byteStride")]
    ByteStride(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"numElements"`
    /// -   type: `hkUint8`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numElements")]
    NumElements(Primitive<u8>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkxVertexDescriptionElementDeclHkParam, "@name",
    ("byteOffset" => ByteOffset(Primitive<u32>)),
    ("type" => Type(DataType)),
    ("usage" => Usage(DataUsage)),
    ("byteStride" => ByteStride(Primitive<u32>)),
    ("numElements" => NumElements(Primitive<u8>)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DataType {
    #[serde(rename = "HKX_DT_NONE")]
    HkxDtNone = 0,
    #[serde(rename = "HKX_DT_UINT8")]
    HkxDtUint8 = 1,
    #[serde(rename = "HKX_DT_INT16")]
    HkxDtInt16 = 2,
    #[serde(rename = "HKX_DT_UINT32")]
    HkxDtUint32 = 3,
    #[serde(rename = "HKX_DT_FLOAT")]
    HkxDtFloat = 4,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DataUsage {
    #[serde(rename = "HKX_DU_NONE")]
    None = 0,
    #[serde(rename = "HKX_DU_POSITION")]
    Position = 1,
    #[serde(rename = "HKX_DU_COLOR")]
    Color = 2,
    #[serde(rename = "HKX_DU_NORMAL")]
    Normal = 4,
    #[serde(rename = "HKX_DU_TANGENT")]
    Tangent = 8,
    #[serde(rename = "HKX_DU_BINORMAL")]
    BiNormal = 16,
    #[serde(rename = "HKX_DU_TEXCOORD")]
    TexCoord = 32,
    #[serde(rename = "HKX_DU_BLENDWEIGHTS")]
    BlendWeights = 64,
    #[serde(rename = "HKX_DU_BLENDINDICES")]
    BlendIndices = 128,
    #[serde(rename = "HKX_DU_USERDATA")]
    UserData = 256,
}
