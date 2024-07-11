//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpCompressedMeshShapeConvexPiece`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#![allow(
  clippy::clone_on_copy,
  clippy::unit_arg
)]

#[allow(unused)]
use super::*;
#[allow(unused)]
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpCompressedMeshShapeConvexPiece`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: false
/// - signature: `0x385bb842`
/// -   version: 3
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpCompressedMeshShapeConvexPiece {
    /// # C++ Class Fields Info
    /// -   name:`"offset"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub offset: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"vertices"`
    /// -   type: `hkArray<hkUint16>`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub vertices: HkArrayNum<u16>,
    /// # C++ Class Fields Info
    /// -   name:`"faceVertices"`
    /// -   type: `hkArray<hkUint8>`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub face_vertices: HkArrayNum<u8>,
    /// # C++ Class Fields Info
    /// -   name:`"faceOffsets"`
    /// -   type: `hkArray<hkUint16>`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub face_offsets: HkArrayNum<u16>,
    /// # C++ Class Fields Info
    /// -   name:`"reference"`
    /// -   type: `hkUint16`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub reference: u16,
    /// # C++ Class Fields Info
    /// -   name:`"transformIndex"`
    /// -   type: `hkUint16`
    /// - offset: 54
    /// -  flags: `FLAGS_NONE`
    pub transform_index: u16,
}

impl Serialize for HkpCompressedMeshShapeConvexPiece {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpCompressedMeshShapeConvexPieceVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpCompressedMeshShapeConvexPiece {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpCompressedMeshShapeConvexPieceVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpCompressedMeshShapeConvexPieceVisitor>> for HkpCompressedMeshShapeConvexPiece {
    fn from(_values: Vec<HkpCompressedMeshShapeConvexPieceVisitor>) -> Self {
            let mut offset = None;
            let mut vertices = None;
            let mut face_vertices = None;
            let mut face_offsets = None;
            let mut reference = None;
            let mut transform_index = None;


        for _value in _values {
            match _value {
                HkpCompressedMeshShapeConvexPieceVisitor::Offset(m) => offset = Some(m),
                HkpCompressedMeshShapeConvexPieceVisitor::Vertices(m) => vertices = Some(m),
                HkpCompressedMeshShapeConvexPieceVisitor::FaceVertices(m) => face_vertices = Some(m),
                HkpCompressedMeshShapeConvexPieceVisitor::FaceOffsets(m) => face_offsets = Some(m),
                HkpCompressedMeshShapeConvexPieceVisitor::Reference(m) => reference = Some(m),
                HkpCompressedMeshShapeConvexPieceVisitor::TransformIndex(m) => transform_index = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            offset: offset.unwrap_or_default().into_inner(),
            vertices: vertices.unwrap_or_default(),
            face_vertices: face_vertices.unwrap_or_default(),
            face_offsets: face_offsets.unwrap_or_default(),
            reference: reference.unwrap_or_default().into_inner(),
            transform_index: transform_index.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpCompressedMeshShapeConvexPiece> for Vec<HkpCompressedMeshShapeConvexPieceVisitor> {
    fn from(data: &HkpCompressedMeshShapeConvexPiece) -> Self {
        vec![
            HkpCompressedMeshShapeConvexPieceVisitor::Offset(data.offset.into()),
            HkpCompressedMeshShapeConvexPieceVisitor::Vertices(data.vertices.clone()),
            HkpCompressedMeshShapeConvexPieceVisitor::FaceVertices(data.face_vertices.clone()),
            HkpCompressedMeshShapeConvexPieceVisitor::FaceOffsets(data.face_offsets.clone()),
            HkpCompressedMeshShapeConvexPieceVisitor::Reference(data.reference.into()),
            HkpCompressedMeshShapeConvexPieceVisitor::TransformIndex(data.transform_index.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpCompressedMeshShapeConvexPiece {
    fn from_bytes<B>(
        _bytes: &'bytes [u8],
        _de: &mut PackFileDeserializer,
    ) -> Result<Self>
    where
        B: ByteOrder,
        Self: Sized + 'de
    {
        todo!()
    }
}


/// # Why use Visitor pattern?
/// Since the C++ field must be deserialized from the `name` attribute name of the `hkparam` in the XML,
/// this is accomplished by having the Visitor process the internally tagged enum and convert it.
/// Leakage of field items may occur if Vec<enum> is left as it is.
///
/// struct -> (De)serialize by visitor -> struct
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
enum HkpCompressedMeshShapeConvexPieceVisitor {
    /// Visitor fields
    #[serde(rename = "offset")]
    Offset(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "vertices")]
    Vertices(HkArrayNum<u16>),
    /// Visitor fields
    #[serde(rename = "faceVertices")]
    FaceVertices(HkArrayNum<u8>),
    /// Visitor fields
    #[serde(rename = "faceOffsets")]
    FaceOffsets(HkArrayNum<u16>),
    /// Visitor fields
    #[serde(rename = "reference")]
    Reference(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "transformIndex")]
    TransformIndex(Primitive<u16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpCompressedMeshShapeConvexPieceVisitor, "@name",
    ("offset" => Offset(Primitive<Vector4<f32>>)),
    ("vertices" => Vertices(HkArrayNum<u16>)),
    ("faceVertices" => FaceVertices(HkArrayNum<u8>)),
    ("faceOffsets" => FaceOffsets(HkArrayNum<u16>)),
    ("reference" => Reference(Primitive<u16>)),
    ("transformIndex" => TransformIndex(Primitive<u16>)),
}
