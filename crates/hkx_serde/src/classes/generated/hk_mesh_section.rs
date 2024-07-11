//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMeshSection`
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

/// `hkMeshSection`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 40
/// -    vtable: false
/// - signature: `0x1893c365`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkMeshSection<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"primitiveType"`
    /// -   type: `enum PrimitiveType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub primitive_type: PrimitiveType,
    /// # C++ Class Fields Info
    /// -   name:`"numPrimitives"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub num_primitives: i32,
    /// # C++ Class Fields Info
    /// -   name:`"numIndices"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub num_indices: i32,
    /// # C++ Class Fields Info
    /// -   name:`"vertexStartIndex"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub vertex_start_index: i32,
    /// # C++ Class Fields Info
    /// -   name:`"transformIndex"`
    /// -   type: `hkInt32`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub transform_index: i32,
    /// # C++ Class Fields Info
    /// -   name:`"indexType"`
    /// -   type: `enum MeshSectionIndexType`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub index_type: MeshSectionIndexType,
    /// # C++ Class Fields Info
    /// -   name:`"indices"`
    /// -   type: `void*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub indices: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"vertexBuffer"`
    /// -   type: `struct hkMeshVertexBuffer*`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub vertex_buffer: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"material"`
    /// -   type: `struct hkMeshMaterial*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub material: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"sectionIndex"`
    /// -   type: `hkInt32`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub section_index: i32,
}

impl Serialize for HkMeshSection<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkMeshSectionVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkMeshSection<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkMeshSectionVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkMeshSectionVisitor<'a>>> for HkMeshSection<'a> {
    fn from(_values: Vec<HkMeshSectionVisitor<'a>>) -> Self {
            let mut primitive_type = None;
            let mut num_primitives = None;
            let mut num_indices = None;
            let mut vertex_start_index = None;
            let mut transform_index = None;
            let mut index_type = None;
            let mut indices = None;
            let mut vertex_buffer = None;
            let mut material = None;
            let mut section_index = None;


        for _value in _values {
            match _value {
                HkMeshSectionVisitor::PrimitiveType(m) => primitive_type = Some(m),
                HkMeshSectionVisitor::NumPrimitives(m) => num_primitives = Some(m),
                HkMeshSectionVisitor::NumIndices(m) => num_indices = Some(m),
                HkMeshSectionVisitor::VertexStartIndex(m) => vertex_start_index = Some(m),
                HkMeshSectionVisitor::TransformIndex(m) => transform_index = Some(m),
                HkMeshSectionVisitor::IndexType(m) => index_type = Some(m),
                HkMeshSectionVisitor::Indices(m) => indices = Some(m),
                HkMeshSectionVisitor::VertexBuffer(m) => vertex_buffer = Some(m),
                HkMeshSectionVisitor::Material(m) => material = Some(m),
                HkMeshSectionVisitor::SectionIndex(m) => section_index = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            primitive_type: primitive_type.unwrap_or_default().into_inner(),
            num_primitives: num_primitives.unwrap_or_default().into_inner(),
            num_indices: num_indices.unwrap_or_default().into_inner(),
            vertex_start_index: vertex_start_index.unwrap_or_default().into_inner(),
            transform_index: transform_index.unwrap_or_default().into_inner(),
            index_type: index_type.unwrap_or_default().into_inner(),
            indices: indices.unwrap_or_default().into_inner(),
            vertex_buffer: vertex_buffer.unwrap_or_default().into_inner(),
            material: material.unwrap_or_default().into_inner(),
            section_index: section_index.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkMeshSection<'a>> for Vec<HkMeshSectionVisitor<'a>> {
    fn from(data: &HkMeshSection<'a>) -> Self {
        vec![
            HkMeshSectionVisitor::PrimitiveType(data.primitive_type.clone().into()),
            HkMeshSectionVisitor::NumPrimitives(data.num_primitives.into()),
            HkMeshSectionVisitor::NumIndices(data.num_indices.into()),
            HkMeshSectionVisitor::VertexStartIndex(data.vertex_start_index.into()),
            HkMeshSectionVisitor::TransformIndex(data.transform_index.into()),
            HkMeshSectionVisitor::IndexType(data.index_type.clone().into()),
            HkMeshSectionVisitor::Indices(data.indices.clone().into()),
            HkMeshSectionVisitor::VertexBuffer(data.vertex_buffer.clone().into()),
            HkMeshSectionVisitor::Material(data.material.clone().into()),
            HkMeshSectionVisitor::SectionIndex(data.section_index.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkMeshSection<'de> {
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
enum HkMeshSectionVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "primitiveType")]
    PrimitiveType(Primitive<PrimitiveType>),
    /// Visitor fields
    #[serde(rename = "numPrimitives")]
    NumPrimitives(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "numIndices")]
    NumIndices(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "vertexStartIndex")]
    VertexStartIndex(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "transformIndex")]
    TransformIndex(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "indexType")]
    IndexType(Primitive<MeshSectionIndexType>),
    /// Visitor fields
    #[serde(rename = "indices", skip_serializing)]
    Indices(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "vertexBuffer")]
    VertexBuffer(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "material")]
    Material(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "sectionIndex")]
    SectionIndex(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMeshSectionVisitor<'de>, "@name",
    ("primitiveType" => PrimitiveType(Primitive<PrimitiveType>)),
    ("numPrimitives" => NumPrimitives(Primitive<i32>)),
    ("numIndices" => NumIndices(Primitive<i32>)),
    ("vertexStartIndex" => VertexStartIndex(Primitive<i32>)),
    ("transformIndex" => TransformIndex(Primitive<i32>)),
    ("indexType" => IndexType(Primitive<MeshSectionIndexType>)),
    ("indices" => Indices(Primitive<Cow<'de, str>>)),
    ("vertexBuffer" => VertexBuffer(Primitive<Cow<'de, str>>)),
    ("material" => Material(Primitive<Cow<'de, str>>)),
    ("sectionIndex" => SectionIndex(Primitive<i32>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum MeshSectionIndexType {
    #[serde(rename = "INDEX_TYPE_NONE")]
    #[default]
    IndexTypeNone = 0,
    #[serde(rename = "INDEX_TYPE_UINT16")]
    IndexTypeUint16 = 1,
    #[serde(rename = "INDEX_TYPE_UINT32")]
    IndexTypeUint32 = 2,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum PrimitiveType {
    #[serde(rename = "PRIMITIVE_TYPE_UNKNOWN")]
    #[default]
    PrimitiveTypeUnknown = 0,
    #[serde(rename = "PRIMITIVE_TYPE_POINT_LIST")]
    PrimitiveTypePointList = 1,
    #[serde(rename = "PRIMITIVE_TYPE_LINE_LIST")]
    PrimitiveTypeLineList = 2,
    #[serde(rename = "PRIMITIVE_TYPE_TRIANGLE_LIST")]
    PrimitiveTypeTriangleList = 3,
    #[serde(rename = "PRIMITIVE_TYPE_TRIANGLE_STRIP")]
    PrimitiveTypeTriangleStrip = 4,
}
