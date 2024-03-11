//! A Rust structure that implements a serializer/deserializer corresponding to `hkpCylinderShape`, a class defined in C++
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
/// -    size: 96
/// -  vtable: true
/// -  parent: hkpConvexShape/`f8f74f85`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpCylinderShape<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpCylinderShape"`: The original C++ class name.
    #[serde(default = "HkpCylinderShape::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x3e463c3a`: Unique value of this class.
    #[serde(default = "HkpCylinderShape::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpCylinderShapeHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpCylinderShapeHkParam<'a>>
}

impl HkpCylinderShape<'_> {
    /// Return `"hkpCylinderShape"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpCylinderShape".into()
    }

    /// Return `"0x3e463c3a"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x3e463c3a".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpCylinderShapeHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"cylRadius"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "cylRadius")]
    CylRadius(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"cylBaseRadiusFactorForHeightFieldCollisions"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "cylBaseRadiusFactorForHeightFieldCollisions")]
    CylBaseRadiusFactorForHeightFieldCollisions(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"vertexA"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexA")]
    VertexA(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"vertexB"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexB")]
    VertexB(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"perpendicular1"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "perpendicular1")]
    Perpendicular1(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"perpendicular2"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "perpendicular2")]
    Perpendicular2(Vector4<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpCylinderShapeHkParam<'de>, "@name",
    ("cylRadius" => CylRadius(Primitive<f32>)),
    ("cylBaseRadiusFactorForHeightFieldCollisions" => CylBaseRadiusFactorForHeightFieldCollisions(Primitive<f32>)),
    ("vertexA" => VertexA(Vector4<f32>)),
    ("vertexB" => VertexB(Vector4<f32>)),
    ("perpendicular1" => Perpendicular1(Vector4<f32>)),
    ("perpendicular2" => Perpendicular2(Vector4<f32>)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum VertexIdEncoding {
    #[serde(rename = "VERTEX_ID_ENCODING_IS_BASE_A_SHIFT")]
    VertexIdEncodingIsBaseAShift = 7,
    #[serde(rename = "VERTEX_ID_ENCODING_SIN_SIGN_SHIFT")]
    VertexIdEncodingSinSignShift = 6,
    #[serde(rename = "VERTEX_ID_ENCODING_COS_SIGN_SHIFT")]
    VertexIdEncodingCosSignShift = 5,
    #[serde(rename = "VERTEX_ID_ENCODING_IS_SIN_LESSER_SHIFT")]
    VertexIdEncodingIsSinLesserShift = 4,
    #[serde(rename = "VERTEX_ID_ENCODING_VALUE_MASK")]
    VertexIdEncodingValueMask = 15,
}
