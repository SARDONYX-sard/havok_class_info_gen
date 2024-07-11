//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxIndexBuffer`
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

/// `hkxIndexBuffer`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 44
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xc12c8197`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkxIndexBuffer {
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
    /// -   name:`"indexType"`
    /// -   type: `enum IndexType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub index_type: IndexType,
    /// # C++ Class Fields Info
    /// -   name:`"indices16"`
    /// -   type: `hkArray<hkUint16>`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub indices_16: HkArrayNum<u16>,
    /// # C++ Class Fields Info
    /// -   name:`"indices32"`
    /// -   type: `hkArray<hkUint32>`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub indices_32: HkArrayNum<u32>,
    /// # C++ Class Fields Info
    /// -   name:`"vertexBaseOffset"`
    /// -   type: `hkUint32`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub vertex_base_offset: u32,
    /// # C++ Class Fields Info
    /// -   name:`"length"`
    /// -   type: `hkUint32`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub length: u32,
}

impl Serialize for HkxIndexBuffer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkxIndexBufferVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkxIndexBuffer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkxIndexBufferVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkxIndexBufferVisitor>> for HkxIndexBuffer {
    fn from(_values: Vec<HkxIndexBufferVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut index_type = None;
            let mut indices_16 = None;
            let mut indices_32 = None;
            let mut vertex_base_offset = None;
            let mut length = None;


        for _value in _values {
            match _value {
                HkxIndexBufferVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkxIndexBufferVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkxIndexBufferVisitor::IndexType(m) => index_type = Some(m),
                HkxIndexBufferVisitor::Indices16(m) => indices_16 = Some(m),
                HkxIndexBufferVisitor::Indices32(m) => indices_32 = Some(m),
                HkxIndexBufferVisitor::VertexBaseOffset(m) => vertex_base_offset = Some(m),
                HkxIndexBufferVisitor::Length(m) => length = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            index_type: index_type.unwrap_or_default().into_inner(),
            indices_16: indices_16.unwrap_or_default(),
            indices_32: indices_32.unwrap_or_default(),
            vertex_base_offset: vertex_base_offset.unwrap_or_default().into_inner(),
            length: length.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkxIndexBuffer> for Vec<HkxIndexBufferVisitor> {
    fn from(data: &HkxIndexBuffer) -> Self {
        vec![
            HkxIndexBufferVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkxIndexBufferVisitor::ReferenceCount(data.reference_count.into()),
            HkxIndexBufferVisitor::IndexType(data.index_type.clone().into()),
            HkxIndexBufferVisitor::Indices16(data.indices_16.clone()),
            HkxIndexBufferVisitor::Indices32(data.indices_32.clone()),
            HkxIndexBufferVisitor::VertexBaseOffset(data.vertex_base_offset.into()),
            HkxIndexBufferVisitor::Length(data.length.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkxIndexBuffer {
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
enum HkxIndexBufferVisitor {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "indexType")]
    IndexType(Primitive<IndexType>),
    /// Visitor fields
    #[serde(rename = "indices16")]
    Indices16(HkArrayNum<u16>),
    /// Visitor fields
    #[serde(rename = "indices32")]
    Indices32(HkArrayNum<u32>),
    /// Visitor fields
    #[serde(rename = "vertexBaseOffset")]
    VertexBaseOffset(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "length")]
    Length(Primitive<u32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxIndexBufferVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("indexType" => IndexType(Primitive<IndexType>)),
    ("indices16" => Indices16(HkArrayNum<u16>)),
    ("indices32" => Indices32(HkArrayNum<u32>)),
    ("vertexBaseOffset" => VertexBaseOffset(Primitive<u32>)),
    ("length" => Length(Primitive<u32>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum IndexType {
    #[serde(rename = "INDEX_TYPE_INVALID")]
    #[default]
    IndexTypeInvalid = 0,
    #[serde(rename = "INDEX_TYPE_TRI_LIST")]
    IndexTypeTriList = 1,
    #[serde(rename = "INDEX_TYPE_TRI_STRIP")]
    IndexTypeTriStrip = 2,
    #[serde(rename = "INDEX_TYPE_TRI_FAN")]
    IndexTypeTriFan = 3,
    #[serde(rename = "INDEX_TYPE_MAX_ID")]
    IndexTypeMaxId = 4,
}
