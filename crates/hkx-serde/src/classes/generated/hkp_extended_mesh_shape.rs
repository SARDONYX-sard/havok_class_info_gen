//! A Rust structure that implements a serializer/deserializer corresponding to `hkpExtendedMeshShape`, a class defined in C++
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
/// -    size: 240
/// -  vtable: true
/// -  parent: hkpShapeCollection/`e8c3991d`(Non prefix hex signature)
/// - version: 3
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpExtendedMeshShape<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpExtendedMeshShape"`: The original C++ class name.
    #[serde(default = "HkpExtendedMeshShape::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x177114a2`: Unique value of this class.
    #[serde(default = "HkpExtendedMeshShape::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpExtendedMeshShapeHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpExtendedMeshShapeHkParam<'a>>
}

impl HkpExtendedMeshShape<'_> {
    /// Return `"hkpExtendedMeshShape"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpExtendedMeshShape".into()
    }

    /// Return `"0x177114a2"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x177114a2".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpExtendedMeshShapeHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"embeddedTrianglesSubpart"`
    /// -   type: `struct hkpExtendedMeshShapeTrianglesSubpart`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "embeddedTrianglesSubpart")]
    EmbeddedTrianglesSubpart(HkpExtendedMeshShapeTrianglesSubpart),
    /// # Field information in the original C++ class
    /// -   name:`"aabbHalfExtents"`
    /// -   type: `hkVector4`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "aabbHalfExtents")]
    AabbHalfExtents(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"aabbCenter"`
    /// -   type: `hkVector4`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "aabbCenter")]
    AabbCenter(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"materialClass"`
    /// -   type: `void*`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "materialClass", skip_serializing)]
    MaterialClass(()),
    /// # Field information in the original C++ class
    /// -   name:`"numBitsForSubpartIndex"`
    /// -   type: `hkInt32`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numBitsForSubpartIndex")]
    NumBitsForSubpartIndex(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"trianglesSubparts"`
    /// -   type: `hkArray&lt;struct hkpExtendedMeshShapeTrianglesSubpart&gt;`
    /// - offset: 184
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "trianglesSubparts")]
    TrianglesSubparts(Vec<HkpExtendedMeshShapeTrianglesSubpart>),
    /// # Field information in the original C++ class
    /// -   name:`"shapesSubparts"`
    /// -   type: `hkArray&lt;struct hkpExtendedMeshShapeShapesSubpart&gt;`
    /// - offset: 196
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shapesSubparts")]
    ShapesSubparts(Vec<HkpExtendedMeshShapeShapesSubpart>),
    /// # Field information in the original C++ class
    /// -   name:`"weldingInfo"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 208
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "weldingInfo")]
    WeldingInfo(Vec<Primitive<u16>>),
    /// # Field information in the original C++ class
    /// -   name:`"weldingType"`
    /// -   type: `enum WeldingType`
    /// - offset: 220
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "weldingType")]
    WeldingType(WeldingType),
    /// # Field information in the original C++ class
    /// -   name:`"defaultCollisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 224
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "defaultCollisionFilterInfo")]
    DefaultCollisionFilterInfo(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"cachedNumChildShapes"`
    /// -   type: `hkInt32`
    /// - offset: 228
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "cachedNumChildShapes")]
    CachedNumChildShapes(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"triangleRadius"`
    /// -   type: `hkReal`
    /// - offset: 232
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "triangleRadius")]
    TriangleRadius(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"padding"`
    /// -   type: `hkInt32`
    /// - offset: 236
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "padding", skip_serializing)]
    Padding(Primitive<i32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpExtendedMeshShapeHkParam<'de>, "@name",
    ("embeddedTrianglesSubpart" => EmbeddedTrianglesSubpart(HkpExtendedMeshShapeTrianglesSubpart)),
    ("aabbHalfExtents" => AabbHalfExtents(Vector4<f32>)),
    ("aabbCenter" => AabbCenter(Vector4<f32>)),
    ("materialClass" => MaterialClass(())),
    ("numBitsForSubpartIndex" => NumBitsForSubpartIndex(Primitive<i32>)),
    ("trianglesSubparts" => TrianglesSubparts(Vec<HkpExtendedMeshShapeTrianglesSubpart>)),
    ("shapesSubparts" => ShapesSubparts(Vec<HkpExtendedMeshShapeShapesSubpart>)),
    ("weldingInfo" => WeldingInfo(Vec<Primitive<u16>>)),
    ("weldingType" => WeldingType(WeldingType)),
    ("defaultCollisionFilterInfo" => DefaultCollisionFilterInfo(Primitive<u32>)),
    ("cachedNumChildShapes" => CachedNumChildShapes(Primitive<i32>)),
    ("triangleRadius" => TriangleRadius(Primitive<f32>)),
    ("padding" => Padding(Primitive<i32>)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum IndexStridingType {
    #[serde(rename = "INDICES_INVALID")]
    IndicesInvalid = 0,
    #[serde(rename = "INDICES_INT8")]
    IndicesInt8 = 1,
    #[serde(rename = "INDICES_INT16")]
    IndicesInt16 = 2,
    #[serde(rename = "INDICES_INT32")]
    IndicesInt32 = 3,
    #[serde(rename = "INDICES_MAX_ID")]
    IndicesMaxId = 4,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MaterialIndexStridingType {
    #[serde(rename = "MATERIAL_INDICES_INVALID")]
    MaterialIndicesInvalid = 0,
    #[serde(rename = "MATERIAL_INDICES_INT8")]
    MaterialIndicesInt8 = 1,
    #[serde(rename = "MATERIAL_INDICES_INT16")]
    MaterialIndicesInt16 = 2,
    #[serde(rename = "MATERIAL_INDICES_MAX_ID")]
    MaterialIndicesMaxId = 3,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SubpartType {
    #[serde(rename = "SUBPART_TRIANGLES")]
    SubpartTriangles = 0,
    #[serde(rename = "SUBPART_SHAPE")]
    SubpartShape = 1,
}
