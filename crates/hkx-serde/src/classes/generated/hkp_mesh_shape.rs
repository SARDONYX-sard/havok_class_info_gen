//! A Rust structure that implements a serializer/deserializer corresponding to `hkpMeshShape`, a class defined in C++
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
/// -    size: 96
/// -  vtable: true
/// -  parent: hkpShapeCollection/`e8c3991d`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpMeshShape<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpMeshShape"`: Name of this class.
    #[serde(default = "HkpMeshShape::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x3bf12c0f`: Unique value of this class.
    #[serde(default = "HkpMeshShape::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpMeshShapeHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpMeshShapeHkParam<'a>>
}

impl HkpMeshShape<'_> {
    /// Return `"hkpMeshShape"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpMeshShape".into()
    }

    /// Return `"0x3bf12c0f"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x3bf12c0f".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpMeshShapeHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"scaling"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "scaling")]
    Scaling(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"numBitsForSubpartIndex"`
    /// -   type: `hkInt32`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numBitsForSubpartIndex")]
    NumBitsForSubpartIndex(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"subparts"`
    /// -   type: `hkArray&lt;struct hkpMeshShapeSubpart&gt;`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "subparts")]
    Subparts(Vec<HkpMeshShapeSubpart>),
    /// # Information on fields in the original C++ class
    /// -   name:`"weldingInfo"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "weldingInfo")]
    WeldingInfo(Vec<u16>),
    /// # Information on fields in the original C++ class
    /// -   name:`"weldingType"`
    /// -   type: `enum WeldingType`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "weldingType")]
    WeldingType(WeldingType),
    /// # Information on fields in the original C++ class
    /// -   name:`"radius"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "radius")]
    Radius(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"pad"`
    /// -   type: `hkInt32[3]`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pad")]
    Pad([i32; 3]),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpMeshShapeHkParam<'de>, "@name",
    ("scaling" => Scaling(cgmath::Vector4<f32>)),
    ("numBitsForSubpartIndex" => NumBitsForSubpartIndex(i32)),
    ("subparts" => Subparts(Vec<HkpMeshShapeSubpart>)),
    ("weldingInfo" => WeldingInfo(Vec<u16>)),
    ("weldingType" => WeldingType(WeldingType)),
    ("radius" => Radius(f64)),
    ("pad" => Pad([i32; 3])),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MeshShapeIndexStridingType {
    #[serde(rename = "INDICES_INVALID")]
    IndicesInvalid = 0,
    #[serde(rename = "INDICES_INT16")]
    IndicesInt16 = 1,
    #[serde(rename = "INDICES_INT32")]
    IndicesInt32 = 2,
    #[serde(rename = "INDICES_MAX_ID")]
    IndicesMaxId = 3,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MeshShapeMaterialIndexStridingType {
    #[serde(rename = "MATERIAL_INDICES_INVALID")]
    MaterialIndicesInvalid = 0,
    #[serde(rename = "MATERIAL_INDICES_INT8")]
    MaterialIndicesInt8 = 1,
    #[serde(rename = "MATERIAL_INDICES_INT16")]
    MaterialIndicesInt16 = 2,
    #[serde(rename = "MATERIAL_INDICES_MAX_ID")]
    MaterialIndicesMaxId = 3,
}
