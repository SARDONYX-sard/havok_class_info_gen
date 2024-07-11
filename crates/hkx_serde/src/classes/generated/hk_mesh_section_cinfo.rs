//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMeshSectionCinfo`
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

/// `hkMeshSectionCinfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: false
/// - signature: `0x6075f3ff`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkMeshSectionCinfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"vertexBuffer"`
    /// -   type: `struct hkMeshVertexBuffer*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub vertex_buffer: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"material"`
    /// -   type: `struct hkMeshMaterial*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub material: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"primitiveType"`
    /// -   type: `enum PrimitiveType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub primitive_type: PrimitiveType,
    /// # C++ Class Fields Info
    /// -   name:`"numPrimitives"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub num_primitives: i32,
    /// # C++ Class Fields Info
    /// -   name:`"indexType"`
    /// -   type: `enum MeshSectionIndexType`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub index_type: MeshSectionIndexType,
    /// # C++ Class Fields Info
    /// -   name:`"indices"`
    /// -   type: `void*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub indices: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"vertexStartIndex"`
    /// -   type: `hkInt32`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub vertex_start_index: i32,
    /// # C++ Class Fields Info
    /// -   name:`"transformIndex"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub transform_index: i32,
}

impl Serialize for HkMeshSectionCinfo<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkMeshSectionCinfoVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkMeshSectionCinfo<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkMeshSectionCinfoVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkMeshSectionCinfoVisitor<'a>>> for HkMeshSectionCinfo<'a> {
    fn from(_values: Vec<HkMeshSectionCinfoVisitor<'a>>) -> Self {
            let mut vertex_buffer = None;
            let mut material = None;
            let mut primitive_type = None;
            let mut num_primitives = None;
            let mut index_type = None;
            let mut indices = None;
            let mut vertex_start_index = None;
            let mut transform_index = None;


        for _value in _values {
            match _value {
                HkMeshSectionCinfoVisitor::VertexBuffer(m) => vertex_buffer = Some(m),
                HkMeshSectionCinfoVisitor::Material(m) => material = Some(m),
                HkMeshSectionCinfoVisitor::PrimitiveType(m) => primitive_type = Some(m),
                HkMeshSectionCinfoVisitor::NumPrimitives(m) => num_primitives = Some(m),
                HkMeshSectionCinfoVisitor::IndexType(m) => index_type = Some(m),
                HkMeshSectionCinfoVisitor::Indices(m) => indices = Some(m),
                HkMeshSectionCinfoVisitor::VertexStartIndex(m) => vertex_start_index = Some(m),
                HkMeshSectionCinfoVisitor::TransformIndex(m) => transform_index = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            vertex_buffer: vertex_buffer.unwrap_or_default().into_inner(),
            material: material.unwrap_or_default().into_inner(),
            primitive_type: primitive_type.unwrap_or_default().into_inner(),
            num_primitives: num_primitives.unwrap_or_default().into_inner(),
            index_type: index_type.unwrap_or_default().into_inner(),
            indices: indices.unwrap_or_default().into_inner(),
            vertex_start_index: vertex_start_index.unwrap_or_default().into_inner(),
            transform_index: transform_index.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkMeshSectionCinfo<'a>> for Vec<HkMeshSectionCinfoVisitor<'a>> {
    fn from(data: &HkMeshSectionCinfo<'a>) -> Self {
        vec![
            HkMeshSectionCinfoVisitor::VertexBuffer(data.vertex_buffer.clone().into()),
            HkMeshSectionCinfoVisitor::Material(data.material.clone().into()),
            HkMeshSectionCinfoVisitor::PrimitiveType(data.primitive_type.clone().into()),
            HkMeshSectionCinfoVisitor::NumPrimitives(data.num_primitives.into()),
            HkMeshSectionCinfoVisitor::IndexType(data.index_type.clone().into()),
            HkMeshSectionCinfoVisitor::Indices(data.indices.clone().into()),
            HkMeshSectionCinfoVisitor::VertexStartIndex(data.vertex_start_index.into()),
            HkMeshSectionCinfoVisitor::TransformIndex(data.transform_index.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkMeshSectionCinfo<'de> {
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
enum HkMeshSectionCinfoVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "vertexBuffer")]
    VertexBuffer(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "material")]
    Material(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "primitiveType")]
    PrimitiveType(Primitive<PrimitiveType>),
    /// Visitor fields
    #[serde(rename = "numPrimitives")]
    NumPrimitives(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "indexType")]
    IndexType(Primitive<MeshSectionIndexType>),
    /// Visitor fields
    #[serde(rename = "indices", skip_serializing)]
    Indices(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "vertexStartIndex")]
    VertexStartIndex(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "transformIndex")]
    TransformIndex(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMeshSectionCinfoVisitor<'de>, "@name",
    ("vertexBuffer" => VertexBuffer(Primitive<Cow<'de, str>>)),
    ("material" => Material(Primitive<Cow<'de, str>>)),
    ("primitiveType" => PrimitiveType(Primitive<PrimitiveType>)),
    ("numPrimitives" => NumPrimitives(Primitive<i32>)),
    ("indexType" => IndexType(Primitive<MeshSectionIndexType>)),
    ("indices" => Indices(Primitive<Cow<'de, str>>)),
    ("vertexStartIndex" => VertexStartIndex(Primitive<i32>)),
    ("transformIndex" => TransformIndex(Primitive<i32>)),
}
