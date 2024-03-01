//! A Rust structure that implements a serializer/deserializer corresponding to `hkpCompressedMeshShape`, a class defined in C++
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
/// -    size: 224
/// -  vtable: true
/// -  parent: hkpShapeCollection/`e8c3991d`(Non prefix hex signature)
/// - version: 9
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpCompressedMeshShape<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpCompressedMeshShape"`: Name of this class.
    #[serde(default = "HkpCompressedMeshShape::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xa62d5e6e`: Unique value of this class.
    #[serde(default = "HkpCompressedMeshShape::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpCompressedMeshShapeHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpCompressedMeshShapeHkParam<'a>>
}

impl HkpCompressedMeshShape<'_> {
    /// Return `"hkpCompressedMeshShape"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpCompressedMeshShape".into()
    }

    /// Return `"0xa62d5e6e"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xa62d5e6e".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpCompressedMeshShapeHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"bitsPerIndex"`
    /// -   type: `hkInt32`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bitsPerIndex")]
    BitsPerIndex(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"bitsPerWIndex"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bitsPerWIndex")]
    BitsPerWIndex(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"wIndexMask"`
    /// -   type: `hkInt32`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wIndexMask")]
    WIndexMask(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"indexMask"`
    /// -   type: `hkInt32`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indexMask")]
    IndexMask(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"radius"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "radius")]
    Radius(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"weldingType"`
    /// -   type: `enum WeldingType`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "weldingType")]
    WeldingType(WeldingType),
    /// # Information on fields in the original C++ class
    /// -   name:`"materialType"`
    /// -   type: `enum MaterialType`
    /// - offset: 45
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialType")]
    MaterialType(MaterialType),
    /// # Information on fields in the original C++ class
    /// -   name:`"materials"`
    /// -   type: `hkArray&lt;hkUint32&gt;`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materials")]
    Materials(Vec<u32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"materials16"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materials16")]
    Materials16(Vec<u16>),
    /// # Information on fields in the original C++ class
    /// -   name:`"materials8"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materials8")]
    Materials8(Vec<u8>),
    /// # Information on fields in the original C++ class
    /// -   name:`"transforms"`
    /// -   type: `hkArray&lt;hkQsTransform&gt;`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transforms")]
    Transforms(Vec<cgmath::Matrix4<f32>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"bigVertices"`
    /// -   type: `hkArray&lt;hkVector4&gt;`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bigVertices")]
    BigVertices(Vec<cgmath::Vector4<f32>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"bigTriangles"`
    /// -   type: `hkArray&lt;struct hkpCompressedMeshShapeBigTriangle&gt;`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bigTriangles")]
    BigTriangles(Vec<HkpCompressedMeshShapeBigTriangle>),
    /// # Information on fields in the original C++ class
    /// -   name:`"chunks"`
    /// -   type: `hkArray&lt;struct hkpCompressedMeshShapeChunk&gt;`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "chunks")]
    Chunks(Vec<HkpCompressedMeshShapeChunk>),
    /// # Information on fields in the original C++ class
    /// -   name:`"convexPieces"`
    /// -   type: `hkArray&lt;struct hkpCompressedMeshShapeConvexPiece&gt;`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "convexPieces")]
    ConvexPieces(Vec<HkpCompressedMeshShapeConvexPiece>),
    /// # Information on fields in the original C++ class
    /// -   name:`"error"`
    /// -   type: `hkReal`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "error")]
    Error(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"bounds"`
    /// -   type: `struct hkAabb`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bounds")]
    Bounds(HkAabb),
    /// # Information on fields in the original C++ class
    /// -   name:`"defaultCollisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "defaultCollisionFilterInfo")]
    DefaultCollisionFilterInfo(u32),
    /// # Information on fields in the original C++ class
    /// -   name:`"meshMaterials"`
    /// -   type: `void*`
    /// - offset: 196
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "meshMaterials", skip_serializing)]
    MeshMaterials(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"materialStriding"`
    /// -   type: `hkUint16`
    /// - offset: 200
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialStriding")]
    MaterialStriding(u16),
    /// # Information on fields in the original C++ class
    /// -   name:`"numMaterials"`
    /// -   type: `hkUint16`
    /// - offset: 202
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numMaterials")]
    NumMaterials(u16),
    /// # Information on fields in the original C++ class
    /// -   name:`"namedMaterials"`
    /// -   type: `hkArray&lt;struct hkpNamedMeshMaterial&gt;`
    /// - offset: 204
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "namedMaterials")]
    NamedMaterials(Vec<HkpNamedMeshMaterial>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpCompressedMeshShapeHkParam<'de>, "@name",
    ("bitsPerIndex" => BitsPerIndex(i32)),
    ("bitsPerWIndex" => BitsPerWIndex(i32)),
    ("wIndexMask" => WIndexMask(i32)),
    ("indexMask" => IndexMask(i32)),
    ("radius" => Radius(f64)),
    ("weldingType" => WeldingType(WeldingType)),
    ("materialType" => MaterialType(MaterialType)),
    ("materials" => Materials(Vec<u32>)),
    ("materials16" => Materials16(Vec<u16>)),
    ("materials8" => Materials8(Vec<u8>)),
    ("transforms" => Transforms(Vec<cgmath::Matrix4<f32>>)),
    ("bigVertices" => BigVertices(Vec<cgmath::Vector4<f32>>)),
    ("bigTriangles" => BigTriangles(Vec<HkpCompressedMeshShapeBigTriangle>)),
    ("chunks" => Chunks(Vec<HkpCompressedMeshShapeChunk>)),
    ("convexPieces" => ConvexPieces(Vec<HkpCompressedMeshShapeConvexPiece>)),
    ("error" => Error(f64)),
    ("bounds" => Bounds(HkAabb)),
    ("defaultCollisionFilterInfo" => DefaultCollisionFilterInfo(u32)),
    ("meshMaterials" => MeshMaterials(())),
    ("materialStriding" => MaterialStriding(u16)),
    ("numMaterials" => NumMaterials(u16)),
    ("namedMaterials" => NamedMaterials(Vec<HkpNamedMeshMaterial>)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MaterialType {
    #[serde(rename = "MATERIAL_NONE")]
    MaterialNone = 0,
    #[serde(rename = "MATERIAL_SINGLE_VALUE_PER_CHUNK")]
    MaterialSingleValuePerChunk = 1,
    #[serde(rename = "MATERIAL_ONE_BYTE_PER_TRIANGLE")]
    MaterialOneBytePerTriangle = 2,
    #[serde(rename = "MATERIAL_TWO_BYTES_PER_TRIANGLE")]
    MaterialTwoBytesPerTriangle = 3,
    #[serde(rename = "MATERIAL_FOUR_BYTES_PER_TRIANGLE")]
    MaterialFourBytesPerTriangle = 4,
}
