//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMemoryMeshBody`
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

/// `hkMemoryMeshBody`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkMeshBody`/`0xd0be5d7d`
/// - signature: `0x94a620a8`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkMemoryMeshBody<'a> {
    // C++ Parent class(`hkMeshBody` => parent: `hkReferencedObject`) has no fields
    //
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub mem_size_and_flags: u16,
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub reference_count: i16,

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// # C++ Class Fields Info
    /// -   name:`"transform"`
    /// -   type: `hkMatrix4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub transform: Matrix4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"transformSet"`
    /// -   type: `struct hkIndexedTransformSet*`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub transform_set: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"shape"`
    /// -   type: `struct hkMeshShape*`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    pub shape: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"vertexBuffers"`
    /// -   type: `hkArray<hkMeshVertexBuffer*>`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    pub vertex_buffers: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    pub name: Cow<'a, str>,
}

impl Serialize for HkMemoryMeshBody<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkMemoryMeshBodyVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkMemoryMeshBody<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkMemoryMeshBodyVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkMemoryMeshBodyVisitor<'a>>> for HkMemoryMeshBody<'a> {
    fn from(_values: Vec<HkMemoryMeshBodyVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut transform = None;
            let mut transform_set = None;
            let mut shape = None;
            let mut vertex_buffers = None;
            let mut name = None;


        for _value in _values {
            match _value {
                HkMemoryMeshBodyVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkMemoryMeshBodyVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkMemoryMeshBodyVisitor::Transform(m) => transform = Some(m),
                HkMemoryMeshBodyVisitor::TransformSet(m) => transform_set = Some(m),
                HkMemoryMeshBodyVisitor::Shape(m) => shape = Some(m),
                HkMemoryMeshBodyVisitor::VertexBuffers(m) => vertex_buffers = Some(m),
                HkMemoryMeshBodyVisitor::Name(m) => name = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            transform: transform.unwrap_or_default().into_inner(),
            transform_set: transform_set.unwrap_or_default().into_inner(),
            shape: shape.unwrap_or_default().into_inner(),
            vertex_buffers: vertex_buffers.unwrap_or_default(),
            name: name.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkMemoryMeshBody<'a>> for Vec<HkMemoryMeshBodyVisitor<'a>> {
    fn from(data: &HkMemoryMeshBody<'a>) -> Self {
        vec![
            HkMemoryMeshBodyVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkMemoryMeshBodyVisitor::ReferenceCount(data.reference_count.into()),
            HkMemoryMeshBodyVisitor::Transform(data.transform.into()),
            HkMemoryMeshBodyVisitor::TransformSet(data.transform_set.clone().into()),
            HkMemoryMeshBodyVisitor::Shape(data.shape.clone().into()),
            HkMemoryMeshBodyVisitor::VertexBuffers(data.vertex_buffers.clone()),
            HkMemoryMeshBodyVisitor::Name(data.name.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkMemoryMeshBody<'de> {
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
enum HkMemoryMeshBodyVisitor<'a> {
    // C++ Parent class(`hkMeshBody` => parent: `hkReferencedObject`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "transform")]
    Transform(Primitive<Matrix4<f32>>),
    /// Visitor fields
    #[serde(rename = "transformSet")]
    TransformSet(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "shape")]
    Shape(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "vertexBuffers")]
    VertexBuffers(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMemoryMeshBodyVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("transform" => Transform(Primitive<Matrix4<f32>>)),
    ("transformSet" => TransformSet(Primitive<Cow<'de, str>>)),
    ("shape" => Shape(Primitive<Cow<'de, str>>)),
    ("vertexBuffers" => VertexBuffers(HkArrayRef<Cow<'de, str>>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
}
