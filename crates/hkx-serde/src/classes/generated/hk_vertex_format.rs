//! A Rust structure that implements a serializer/deserializer corresponding to `hkVertexFormat`, a class defined in C++
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
/// -    size: 260
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkVertexFormat<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkVertexFormat"`: The original C++ class name.
    #[serde(default = "HkVertexFormat::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xf11e3ff7`: Unique value of this class.
    #[serde(default = "HkVertexFormat::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkVertexFormatHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkVertexFormatHkParam<'a>>
}

impl HkVertexFormat<'_> {
    /// Return `"hkVertexFormat"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkVertexFormat".into()
    }

    /// Return `"0xf11e3ff7"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xf11e3ff7".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkVertexFormatHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"elements"`
    /// -   type: `struct hkVertexFormatElement[32]`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "elements")]
    Elements([HkVertexFormatElement; 32]),
    /// # Field information in the original C++ class
    /// -   name:`"numElements"`
    /// -   type: `hkInt32`
    /// - offset: 256
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numElements")]
    NumElements(Primitive<i32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkVertexFormatHkParam<'de>, "@name",
    ("elements" => Elements([HkVertexFormatElement; 32])),
    ("numElements" => NumElements(Primitive<i32>)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ComponentType {
    #[serde(rename = "TYPE_NONE")]
    TypeNone = 0,
    #[serde(rename = "TYPE_INT8")]
    TypeInt8 = 1,
    #[serde(rename = "TYPE_UINT8")]
    TypeUint8 = 2,
    #[serde(rename = "TYPE_INT16")]
    TypeInt16 = 3,
    #[serde(rename = "TYPE_UINT16")]
    TypeUint16 = 4,
    #[serde(rename = "TYPE_INT32")]
    TypeInt32 = 5,
    #[serde(rename = "TYPE_UINT32")]
    TypeUint32 = 6,
    #[serde(rename = "TYPE_UINT8_DWORD")]
    TypeUint8Dword = 7,
    #[serde(rename = "TYPE_ARGB32")]
    TypeArgb32 = 8,
    #[serde(rename = "TYPE_FLOAT16")]
    TypeFloat16 = 9,
    #[serde(rename = "TYPE_FLOAT32")]
    TypeFloat32 = 10,
    #[serde(rename = "TYPE_VECTOR4")]
    TypeVector4 = 11,
    #[serde(rename = "TYPE_LAST")]
    TypeLast = 12,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ComponentUsage {
    #[serde(rename = "USAGE_NONE")]
    UsageNone = 0,
    #[serde(rename = "USAGE_POSITION")]
    UsagePosition = 1,
    #[serde(rename = "USAGE_NORMAL")]
    UsageNormal = 2,
    #[serde(rename = "USAGE_COLOR")]
    UsageColor = 3,
    #[serde(rename = "USAGE_TANGENT")]
    UsageTangent = 4,
    #[serde(rename = "USAGE_BINORMAL")]
    UsageBinormal = 5,
    #[serde(rename = "USAGE_BLEND_MATRIX_INDEX")]
    UsageBlendMatrixIndex = 6,
    #[serde(rename = "USAGE_BLEND_WEIGHTS")]
    UsageBlendWeights = 7,
    #[serde(rename = "USAGE_BLEND_WEIGHTS_LAST_IMPLIED")]
    UsageBlendWeightsLastImplied = 8,
    #[serde(rename = "USAGE_TEX_COORD")]
    UsageTexCoord = 9,
    #[serde(rename = "USAGE_POINT_SIZE")]
    UsagePointSize = 10,
    #[serde(rename = "USAGE_USER")]
    UsageUser = 11,
    #[serde(rename = "USAGE_LAST")]
    UsageLast = 12,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum HintFlags {
    #[serde(rename = "FLAG_READ")]
    FlagRead = 1,
    #[serde(rename = "FLAG_WRITE")]
    FlagWrite = 2,
    #[serde(rename = "FLAG_DYNAMIC")]
    FlagDynamic = 4,
    #[serde(rename = "FLAG_NOT_SHARED")]
    FlagNotShared = 8,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SharingType {
    #[serde(rename = "SHARING_ALL_SHARED")]
    SharingAllShared = 0,
    #[serde(rename = "SHARING_ALL_NOT_SHARED")]
    SharingAllNotShared = 1,
    #[serde(rename = "SHARING_MIXTURE")]
    SharingMixture = 2,
}
