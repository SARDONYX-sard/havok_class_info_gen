//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMultipleVertexBufferVertexBufferInfo`
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

/// `hkMultipleVertexBufferVertexBufferInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0xdafbe0e6`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkMultipleVertexBufferVertexBufferInfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"vertexBuffer"`
    /// -   type: `struct hkMeshVertexBuffer*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub vertex_buffer: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"lockedVertices"`
    /// -   type: `void*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub locked_vertices: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"isLocked"`
    /// -   type: `hkBool`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub is_locked: bool,
}

impl Serialize for HkMultipleVertexBufferVertexBufferInfo<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkMultipleVertexBufferVertexBufferInfoVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkMultipleVertexBufferVertexBufferInfo<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkMultipleVertexBufferVertexBufferInfoVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkMultipleVertexBufferVertexBufferInfoVisitor<'a>>> for HkMultipleVertexBufferVertexBufferInfo<'a> {
    fn from(_values: Vec<HkMultipleVertexBufferVertexBufferInfoVisitor<'a>>) -> Self {
            let mut vertex_buffer = None;
            let mut locked_vertices = None;
            let mut is_locked = None;


        for _value in _values {
            match _value {
                HkMultipleVertexBufferVertexBufferInfoVisitor::VertexBuffer(m) => vertex_buffer = Some(m),
                HkMultipleVertexBufferVertexBufferInfoVisitor::LockedVertices(m) => locked_vertices = Some(m),
                HkMultipleVertexBufferVertexBufferInfoVisitor::IsLocked(m) => is_locked = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            vertex_buffer: vertex_buffer.unwrap_or_default().into_inner(),
            locked_vertices: locked_vertices.unwrap_or_default().into_inner(),
            is_locked: is_locked.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkMultipleVertexBufferVertexBufferInfo<'a>> for Vec<HkMultipleVertexBufferVertexBufferInfoVisitor<'a>> {
    fn from(data: &HkMultipleVertexBufferVertexBufferInfo<'a>) -> Self {
        vec![
            HkMultipleVertexBufferVertexBufferInfoVisitor::VertexBuffer(data.vertex_buffer.clone().into()),
            HkMultipleVertexBufferVertexBufferInfoVisitor::LockedVertices(data.locked_vertices.clone().into()),
            HkMultipleVertexBufferVertexBufferInfoVisitor::IsLocked(data.is_locked.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkMultipleVertexBufferVertexBufferInfo<'de> {
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
enum HkMultipleVertexBufferVertexBufferInfoVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "vertexBuffer")]
    VertexBuffer(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "lockedVertices", skip_serializing)]
    LockedVertices(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "isLocked")]
    IsLocked(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMultipleVertexBufferVertexBufferInfoVisitor<'de>, "@name",
    ("vertexBuffer" => VertexBuffer(Primitive<Cow<'de, str>>)),
    ("lockedVertices" => LockedVertices(Primitive<Cow<'de, str>>)),
    ("isLocked" => IsLocked(Primitive<bool>)),
}
