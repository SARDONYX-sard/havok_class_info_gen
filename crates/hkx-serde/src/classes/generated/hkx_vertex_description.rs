//! A Rust structure that implements a serializer/deserializer corresponding to `hkxVertexDescription`, a class defined in C++
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
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkxVertexDescription<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkxVertexDescription"`: Name of this class.
    #[serde(default = "HkxVertexDescription::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x2df6313d`: Unique value of this class.
    #[serde(default = "HkxVertexDescription::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkxVertexDescriptionHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkxVertexDescriptionHkParam<'a>>
}

impl HkxVertexDescription<'_> {
    /// Return `"hkxVertexDescription"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkxVertexDescription".into()
    }

    /// Return `"0x2df6313d"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x2df6313d".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxVertexDescriptionHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"decls"`
    /// -   type: `hkArray&lt;struct hkxVertexDescriptionElementDecl&gt;`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "decls")]
    Decls(Vec<HkxVertexDescriptionElementDecl>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkxVertexDescriptionHkParam<'de>, "@name",
    ("decls" => Decls(Vec<HkxVertexDescriptionElementDecl>)),
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
    HkxDuNone = 0,
    #[serde(rename = "HKX_DU_POSITION")]
    HkxDuPosition = 1,
    #[serde(rename = "HKX_DU_COLOR")]
    HkxDuColor = 2,
    #[serde(rename = "HKX_DU_NORMAL")]
    HkxDuNormal = 4,
    #[serde(rename = "HKX_DU_TANGENT")]
    HkxDuTangent = 8,
    #[serde(rename = "HKX_DU_BINORMAL")]
    HkxDuBinormal = 16,
    #[serde(rename = "HKX_DU_TEXCOORD")]
    HkxDuTexcoord = 32,
    #[serde(rename = "HKX_DU_BLENDWEIGHTS")]
    HkxDuBlendweights = 64,
    #[serde(rename = "HKX_DU_BLENDINDICES")]
    HkxDuBlendindices = 128,
    #[serde(rename = "HKX_DU_USERDATA")]
    HkxDuUserdata = 256,
}
