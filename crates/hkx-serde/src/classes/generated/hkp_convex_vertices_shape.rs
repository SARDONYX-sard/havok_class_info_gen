//! A Rust structure that implements a serializer/deserializer corresponding to `hkpConvexVerticesShape`, a class defined in C++
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
/// -    size: 112
/// -  vtable: true
/// -  parent: hkpConvexShape/`f8f74f85`(Non prefix hex signature)
/// - version: 3
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpConvexVerticesShape<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpConvexVerticesShape"`: The original C++ class name.
    #[serde(default = "HkpConvexVerticesShape::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x28726ad8`: Unique value of this class.
    #[serde(default = "HkpConvexVerticesShape::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpConvexVerticesShapeHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpConvexVerticesShapeHkParam<'a>>
}

impl HkpConvexVerticesShape<'_> {
    /// Return `"hkpConvexVerticesShape"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpConvexVerticesShape".into()
    }

    /// Return `"0x28726ad8"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x28726ad8".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpConvexVerticesShapeHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"aabbHalfExtents"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "aabbHalfExtents")]
    AabbHalfExtents(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"aabbCenter"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "aabbCenter")]
    AabbCenter(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"rotatedVertices"`
    /// -   type: `hkArray&lt;struct hkpConvexVerticesShapeFourVectors&gt;`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotatedVertices")]
    RotatedVertices(Vec<HkpConvexVerticesShapeFourVectors>),
    /// # Field information in the original C++ class
    /// -   name:`"numVertices"`
    /// -   type: `hkInt32`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numVertices")]
    NumVertices(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"externalObject"`
    /// -   type: `void*`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "externalObject", skip_serializing)]
    ExternalObject(()),
    /// # Field information in the original C++ class
    /// -   name:`"getFaceNormals"`
    /// -   type: `void*`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "getFaceNormals", skip_serializing)]
    GetFaceNormals(()),
    /// # Field information in the original C++ class
    /// -   name:`"planeEquations"`
    /// -   type: `hkArray&lt;hkVector4&gt;`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "planeEquations")]
    PlaneEquations(Vec<Vector4<f32>>),
    /// # Field information in the original C++ class
    /// -   name:`"connectivity"`
    /// -   type: `struct hkpConvexVerticesConnectivity*`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "connectivity")]
    Connectivity(Cow<'a, str>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpConvexVerticesShapeHkParam<'de>, "@name",
    ("aabbHalfExtents" => AabbHalfExtents(Vector4<f32>)),
    ("aabbCenter" => AabbCenter(Vector4<f32>)),
    ("rotatedVertices" => RotatedVertices(Vec<HkpConvexVerticesShapeFourVectors>)),
    ("numVertices" => NumVertices(Primitive<i32>)),
    ("externalObject" => ExternalObject(())),
    ("getFaceNormals" => GetFaceNormals(())),
    ("planeEquations" => PlaneEquations(Vec<Vector4<f32>>)),
    ("connectivity" => Connectivity(Cow<'a, str>)),
}
