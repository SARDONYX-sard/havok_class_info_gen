//! A Rust structure that implements a serializer/deserializer corresponding to `hkpTriangleShape`, a class defined in C++
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
/// -  parent: hkpConvexShape/`f8f74f85`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpTriangleShape<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpTriangleShape"`: Name of this class.
    #[serde(default = "HkpTriangleShape::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x95ad1a25`: Unique value of this class.
    #[serde(default = "HkpTriangleShape::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpTriangleShapeHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpTriangleShapeHkParam<'a>>
}

impl HkpTriangleShape<'_> {
    /// Return `"hkpTriangleShape"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpTriangleShape".into()
    }

    /// Return `"0x95ad1a25"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x95ad1a25".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpTriangleShapeHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"weldingInfo"`
    /// -   type: `hkUint16`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "weldingInfo")]
    WeldingInfo(u16),
    /// # Information on fields in the original C++ class
    /// -   name:`"weldingType"`
    /// -   type: `enum WeldingType`
    /// - offset: 22
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "weldingType")]
    WeldingType(WeldingType),
    /// # Information on fields in the original C++ class
    /// -   name:`"isExtruded"`
    /// -   type: `hkUint8`
    /// - offset: 23
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isExtruded")]
    IsExtruded(u8),
    /// # Information on fields in the original C++ class
    /// -   name:`"vertexA"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexA")]
    VertexA(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"vertexB"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexB")]
    VertexB(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"vertexC"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexC")]
    VertexC(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"extrusion"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extrusion")]
    Extrusion(cgmath::Vector4<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpTriangleShapeHkParam<'de>, "@name",
    ("weldingInfo" => WeldingInfo(u16)),
    ("weldingType" => WeldingType(WeldingType)),
    ("isExtruded" => IsExtruded(u8)),
    ("vertexA" => VertexA(cgmath::Vector4<f32>)),
    ("vertexB" => VertexB(cgmath::Vector4<f32>)),
    ("vertexC" => VertexC(cgmath::Vector4<f32>)),
    ("extrusion" => Extrusion(cgmath::Vector4<f32>)),
}
