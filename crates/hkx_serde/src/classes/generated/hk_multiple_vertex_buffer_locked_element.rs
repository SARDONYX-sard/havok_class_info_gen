//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMultipleVertexBufferLockedElement`
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

/// `hkMultipleVertexBufferLockedElement`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 7
/// -    vtable: false
/// - signature: `0xa0e22afc`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkMultipleVertexBufferLockedElement {
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
    /// # C++ Class Fields Info
    /// -   name:`"lockedBufferIndex"`
    /// -   type: `hkUint8`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    pub locked_buffer_index: u8,
    /// # C++ Class Fields Info
    /// -   name:`"vertexFormatIndex"`
    /// -   type: `hkUint8`
    /// - offset: 3
    /// -  flags: `FLAGS_NONE`
    pub vertex_format_index: u8,
    /// # C++ Class Fields Info
    /// -   name:`"lockFlags"`
    /// -   type: `hkUint8`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub lock_flags: u8,
    /// # C++ Class Fields Info
    /// -   name:`"outputBufferIndex"`
    /// -   type: `hkUint8`
    /// - offset: 5
    /// -  flags: `FLAGS_NONE`
    pub output_buffer_index: u8,
    /// # C++ Class Fields Info
    /// -   name:`"emulatedIndex"`
    /// -   type: `hkInt8`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    pub emulated_index: i8,
}

impl Serialize for HkMultipleVertexBufferLockedElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkMultipleVertexBufferLockedElementVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkMultipleVertexBufferLockedElement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkMultipleVertexBufferLockedElementVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkMultipleVertexBufferLockedElementVisitor>> for HkMultipleVertexBufferLockedElement {
    fn from(_values: Vec<HkMultipleVertexBufferLockedElementVisitor>) -> Self {
            let mut vertex_buffer_index = None;
            let mut element_index = None;
            let mut locked_buffer_index = None;
            let mut vertex_format_index = None;
            let mut lock_flags = None;
            let mut output_buffer_index = None;
            let mut emulated_index = None;


        for _value in _values {
            match _value {
                HkMultipleVertexBufferLockedElementVisitor::VertexBufferIndex(m) => vertex_buffer_index = Some(m),
                HkMultipleVertexBufferLockedElementVisitor::ElementIndex(m) => element_index = Some(m),
                HkMultipleVertexBufferLockedElementVisitor::LockedBufferIndex(m) => locked_buffer_index = Some(m),
                HkMultipleVertexBufferLockedElementVisitor::VertexFormatIndex(m) => vertex_format_index = Some(m),
                HkMultipleVertexBufferLockedElementVisitor::LockFlags(m) => lock_flags = Some(m),
                HkMultipleVertexBufferLockedElementVisitor::OutputBufferIndex(m) => output_buffer_index = Some(m),
                HkMultipleVertexBufferLockedElementVisitor::EmulatedIndex(m) => emulated_index = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            vertex_buffer_index: vertex_buffer_index.unwrap_or_default().into_inner(),
            element_index: element_index.unwrap_or_default().into_inner(),
            locked_buffer_index: locked_buffer_index.unwrap_or_default().into_inner(),
            vertex_format_index: vertex_format_index.unwrap_or_default().into_inner(),
            lock_flags: lock_flags.unwrap_or_default().into_inner(),
            output_buffer_index: output_buffer_index.unwrap_or_default().into_inner(),
            emulated_index: emulated_index.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkMultipleVertexBufferLockedElement> for Vec<HkMultipleVertexBufferLockedElementVisitor> {
    fn from(data: &HkMultipleVertexBufferLockedElement) -> Self {
        vec![
            HkMultipleVertexBufferLockedElementVisitor::VertexBufferIndex(data.vertex_buffer_index.into()),
            HkMultipleVertexBufferLockedElementVisitor::ElementIndex(data.element_index.into()),
            HkMultipleVertexBufferLockedElementVisitor::LockedBufferIndex(data.locked_buffer_index.into()),
            HkMultipleVertexBufferLockedElementVisitor::VertexFormatIndex(data.vertex_format_index.into()),
            HkMultipleVertexBufferLockedElementVisitor::LockFlags(data.lock_flags.into()),
            HkMultipleVertexBufferLockedElementVisitor::OutputBufferIndex(data.output_buffer_index.into()),
            HkMultipleVertexBufferLockedElementVisitor::EmulatedIndex(data.emulated_index.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkMultipleVertexBufferLockedElement {
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
enum HkMultipleVertexBufferLockedElementVisitor {
    /// Visitor fields
    #[serde(rename = "vertexBufferIndex")]
    VertexBufferIndex(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "elementIndex")]
    ElementIndex(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "lockedBufferIndex")]
    LockedBufferIndex(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "vertexFormatIndex")]
    VertexFormatIndex(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "lockFlags")]
    LockFlags(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "outputBufferIndex")]
    OutputBufferIndex(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "emulatedIndex")]
    EmulatedIndex(Primitive<i8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMultipleVertexBufferLockedElementVisitor, "@name",
    ("vertexBufferIndex" => VertexBufferIndex(Primitive<u8>)),
    ("elementIndex" => ElementIndex(Primitive<u8>)),
    ("lockedBufferIndex" => LockedBufferIndex(Primitive<u8>)),
    ("vertexFormatIndex" => VertexFormatIndex(Primitive<u8>)),
    ("lockFlags" => LockFlags(Primitive<u8>)),
    ("outputBufferIndex" => OutputBufferIndex(Primitive<u8>)),
    ("emulatedIndex" => EmulatedIndex(Primitive<i8>)),
}
