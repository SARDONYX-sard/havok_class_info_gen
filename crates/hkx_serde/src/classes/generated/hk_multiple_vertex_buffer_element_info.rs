//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMultipleVertexBufferElementInfo`
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

/// `hkMultipleVertexBufferElementInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 2
/// -    vtable: false
/// - signature: `0x4731fb1b`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkMultipleVertexBufferElementInfo {
    /// # C++ Class Fields Info
    /// -   name:`"vertexBufferIndex"`
    /// -   type: `hkUint8`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub vertex_buffer_index: u8,
    /// # C++ Class Fields Info
    /// -   name:`"elementIndex"`
    /// -   type: `hkUint8`
    /// - offset: 1
    /// -  flags: `FLAGS_NONE`
    pub element_index: u8,
}

impl Serialize for HkMultipleVertexBufferElementInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkMultipleVertexBufferElementInfoVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkMultipleVertexBufferElementInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkMultipleVertexBufferElementInfoVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkMultipleVertexBufferElementInfoVisitor>> for HkMultipleVertexBufferElementInfo {
    fn from(_values: Vec<HkMultipleVertexBufferElementInfoVisitor>) -> Self {
            let mut vertex_buffer_index = None;
            let mut element_index = None;


        for _value in _values {
            match _value {
                HkMultipleVertexBufferElementInfoVisitor::VertexBufferIndex(m) => vertex_buffer_index = Some(m),
                HkMultipleVertexBufferElementInfoVisitor::ElementIndex(m) => element_index = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            vertex_buffer_index: vertex_buffer_index.unwrap_or_default().into_inner(),
            element_index: element_index.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkMultipleVertexBufferElementInfo> for Vec<HkMultipleVertexBufferElementInfoVisitor> {
    fn from(data: &HkMultipleVertexBufferElementInfo) -> Self {
        vec![
            HkMultipleVertexBufferElementInfoVisitor::VertexBufferIndex(data.vertex_buffer_index.into()),
            HkMultipleVertexBufferElementInfoVisitor::ElementIndex(data.element_index.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkMultipleVertexBufferElementInfo {
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
enum HkMultipleVertexBufferElementInfoVisitor {
    /// Visitor fields
    #[serde(rename = "vertexBufferIndex")]
    VertexBufferIndex(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "elementIndex")]
    ElementIndex(Primitive<u8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMultipleVertexBufferElementInfoVisitor, "@name",
    ("vertexBufferIndex" => VertexBufferIndex(Primitive<u8>)),
    ("elementIndex" => ElementIndex(Primitive<u8>)),
}
