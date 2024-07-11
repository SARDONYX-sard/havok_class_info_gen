//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpCompressedMeshShapeChunk`
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

/// `hkpCompressedMeshShapeChunk`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: false
/// - signature: `0x5d0d67bd`
/// -   version: 4
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpCompressedMeshShapeChunk {
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
    /// -   name:`"indices"`
    /// -   type: `hkArray<hkUint16>`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub indices: HkArrayNum<u16>,
    /// # C++ Class Fields Info
    /// -   name:`"stripLengths"`
    /// -   type: `hkArray<hkUint16>`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub strip_lengths: HkArrayNum<u16>,
    /// # C++ Class Fields Info
    /// -   name:`"weldingInfo"`
    /// -   type: `hkArray<hkUint16>`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub welding_info: HkArrayNum<u16>,
    /// # C++ Class Fields Info
    /// -   name:`"materialInfo"`
    /// -   type: `hkUint32`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub material_info: u32,
    /// # C++ Class Fields Info
    /// -   name:`"reference"`
    /// -   type: `hkUint16`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub reference: u16,
    /// # C++ Class Fields Info
    /// -   name:`"transformIndex"`
    /// -   type: `hkUint16`
    /// - offset: 70
    /// -  flags: `FLAGS_NONE`
    pub transform_index: u16,
}

impl Serialize for HkpCompressedMeshShapeChunk {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpCompressedMeshShapeChunkVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpCompressedMeshShapeChunk {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpCompressedMeshShapeChunkVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpCompressedMeshShapeChunkVisitor>> for HkpCompressedMeshShapeChunk {
    fn from(_values: Vec<HkpCompressedMeshShapeChunkVisitor>) -> Self {
            let mut offset = None;
            let mut vertices = None;
            let mut indices = None;
            let mut strip_lengths = None;
            let mut welding_info = None;
            let mut material_info = None;
            let mut reference = None;
            let mut transform_index = None;


        for _value in _values {
            match _value {
                HkpCompressedMeshShapeChunkVisitor::Offset(m) => offset = Some(m),
                HkpCompressedMeshShapeChunkVisitor::Vertices(m) => vertices = Some(m),
                HkpCompressedMeshShapeChunkVisitor::Indices(m) => indices = Some(m),
                HkpCompressedMeshShapeChunkVisitor::StripLengths(m) => strip_lengths = Some(m),
                HkpCompressedMeshShapeChunkVisitor::WeldingInfo(m) => welding_info = Some(m),
                HkpCompressedMeshShapeChunkVisitor::MaterialInfo(m) => material_info = Some(m),
                HkpCompressedMeshShapeChunkVisitor::Reference(m) => reference = Some(m),
                HkpCompressedMeshShapeChunkVisitor::TransformIndex(m) => transform_index = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            offset: offset.unwrap_or_default().into_inner(),
            vertices: vertices.unwrap_or_default(),
            indices: indices.unwrap_or_default(),
            strip_lengths: strip_lengths.unwrap_or_default(),
            welding_info: welding_info.unwrap_or_default(),
            material_info: material_info.unwrap_or_default().into_inner(),
            reference: reference.unwrap_or_default().into_inner(),
            transform_index: transform_index.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpCompressedMeshShapeChunk> for Vec<HkpCompressedMeshShapeChunkVisitor> {
    fn from(data: &HkpCompressedMeshShapeChunk) -> Self {
        vec![
            HkpCompressedMeshShapeChunkVisitor::Offset(data.offset.into()),
            HkpCompressedMeshShapeChunkVisitor::Vertices(data.vertices.clone()),
            HkpCompressedMeshShapeChunkVisitor::Indices(data.indices.clone()),
            HkpCompressedMeshShapeChunkVisitor::StripLengths(data.strip_lengths.clone()),
            HkpCompressedMeshShapeChunkVisitor::WeldingInfo(data.welding_info.clone()),
            HkpCompressedMeshShapeChunkVisitor::MaterialInfo(data.material_info.into()),
            HkpCompressedMeshShapeChunkVisitor::Reference(data.reference.into()),
            HkpCompressedMeshShapeChunkVisitor::TransformIndex(data.transform_index.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpCompressedMeshShapeChunk {
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
enum HkpCompressedMeshShapeChunkVisitor {
    /// Visitor fields
    #[serde(rename = "offset")]
    Offset(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "vertices")]
    Vertices(HkArrayNum<u16>),
    /// Visitor fields
    #[serde(rename = "indices")]
    Indices(HkArrayNum<u16>),
    /// Visitor fields
    #[serde(rename = "stripLengths")]
    StripLengths(HkArrayNum<u16>),
    /// Visitor fields
    #[serde(rename = "weldingInfo")]
    WeldingInfo(HkArrayNum<u16>),
    /// Visitor fields
    #[serde(rename = "materialInfo")]
    MaterialInfo(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "reference")]
    Reference(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "transformIndex")]
    TransformIndex(Primitive<u16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpCompressedMeshShapeChunkVisitor, "@name",
    ("offset" => Offset(Primitive<Vector4<f32>>)),
    ("vertices" => Vertices(HkArrayNum<u16>)),
    ("indices" => Indices(HkArrayNum<u16>)),
    ("stripLengths" => StripLengths(HkArrayNum<u16>)),
    ("weldingInfo" => WeldingInfo(HkArrayNum<u16>)),
    ("materialInfo" => MaterialInfo(Primitive<u32>)),
    ("reference" => Reference(Primitive<u16>)),
    ("transformIndex" => TransformIndex(Primitive<u16>)),
}
