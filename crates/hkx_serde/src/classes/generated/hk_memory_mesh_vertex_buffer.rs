//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMemoryMeshVertexBuffer`
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

/// `hkMemoryMeshVertexBuffer`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 424
/// -    vtable: true
/// -    parent: `hkMeshVertexBuffer`/`0x534b08c8`
/// - signature: `0xa2e50753`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkMemoryMeshVertexBuffer {
    // C++ Parent class(`hkMeshVertexBuffer` => parent: `hkReferencedObject`) has no fields
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
    /// -   name:`"format"`
    /// -   type: `struct hkVertexFormat`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub format: SingleClass<HkVertexFormat>,
    /// # C++ Class Fields Info
    /// -   name:`"elementOffsets"`
    /// -   type: `hkInt32[32]`
    /// - offset: 268
    /// -  flags: `FLAGS_NONE`
    pub element_offsets: CStyleArray<[i32; 32]>,
    /// # C++ Class Fields Info
    /// -   name:`"memory"`
    /// -   type: `hkArray<hkUint8>`
    /// - offset: 396
    /// -  flags: `FLAGS_NONE`
    pub memory: HkArrayNum<u8>,
    /// # C++ Class Fields Info
    /// -   name:`"vertexStride"`
    /// -   type: `hkInt32`
    /// - offset: 408
    /// -  flags: `FLAGS_NONE`
    pub vertex_stride: i32,
    /// # C++ Class Fields Info
    /// -   name:`"locked"`
    /// -   type: `hkBool`
    /// - offset: 412
    /// -  flags: `FLAGS_NONE`
    pub locked: bool,
    /// # C++ Class Fields Info
    /// -   name:`"numVertices"`
    /// -   type: `hkInt32`
    /// - offset: 416
    /// -  flags: `FLAGS_NONE`
    pub num_vertices: i32,
    /// # C++ Class Fields Info
    /// -   name:`"isBigEndian"`
    /// -   type: `hkBool`
    /// - offset: 420
    /// -  flags: `FLAGS_NONE`
    pub is_big_endian: bool,
    /// # C++ Class Fields Info
    /// -   name:`"isSharable"`
    /// -   type: `hkBool`
    /// - offset: 421
    /// -  flags: `FLAGS_NONE`
    pub is_sharable: bool,
}

impl Serialize for HkMemoryMeshVertexBuffer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkMemoryMeshVertexBufferVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkMemoryMeshVertexBuffer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkMemoryMeshVertexBufferVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkMemoryMeshVertexBufferVisitor>> for HkMemoryMeshVertexBuffer {
    fn from(_values: Vec<HkMemoryMeshVertexBufferVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut format = None;
            let mut element_offsets = None;
            let mut memory = None;
            let mut vertex_stride = None;
            let mut locked = None;
            let mut num_vertices = None;
            let mut is_big_endian = None;
            let mut is_sharable = None;


        for _value in _values {
            match _value {
                HkMemoryMeshVertexBufferVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkMemoryMeshVertexBufferVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkMemoryMeshVertexBufferVisitor::Format(m) => format = Some(m),
                HkMemoryMeshVertexBufferVisitor::ElementOffsets(m) => element_offsets = Some(m),
                HkMemoryMeshVertexBufferVisitor::Memory(m) => memory = Some(m),
                HkMemoryMeshVertexBufferVisitor::VertexStride(m) => vertex_stride = Some(m),
                HkMemoryMeshVertexBufferVisitor::Locked(m) => locked = Some(m),
                HkMemoryMeshVertexBufferVisitor::NumVertices(m) => num_vertices = Some(m),
                HkMemoryMeshVertexBufferVisitor::IsBigEndian(m) => is_big_endian = Some(m),
                HkMemoryMeshVertexBufferVisitor::IsSharable(m) => is_sharable = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            format: format.unwrap_or_default(),
            element_offsets: element_offsets.unwrap_or_default(),
            memory: memory.unwrap_or_default(),
            vertex_stride: vertex_stride.unwrap_or_default().into_inner(),
            locked: locked.unwrap_or_default().into_inner(),
            num_vertices: num_vertices.unwrap_or_default().into_inner(),
            is_big_endian: is_big_endian.unwrap_or_default().into_inner(),
            is_sharable: is_sharable.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkMemoryMeshVertexBuffer> for Vec<HkMemoryMeshVertexBufferVisitor> {
    fn from(data: &HkMemoryMeshVertexBuffer) -> Self {
        vec![
            HkMemoryMeshVertexBufferVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkMemoryMeshVertexBufferVisitor::ReferenceCount(data.reference_count.into()),
            HkMemoryMeshVertexBufferVisitor::Format(data.format.clone()),
            HkMemoryMeshVertexBufferVisitor::ElementOffsets(data.element_offsets.clone()),
            HkMemoryMeshVertexBufferVisitor::Memory(data.memory.clone()),
            HkMemoryMeshVertexBufferVisitor::VertexStride(data.vertex_stride.into()),
            HkMemoryMeshVertexBufferVisitor::Locked(data.locked.into()),
            HkMemoryMeshVertexBufferVisitor::NumVertices(data.num_vertices.into()),
            HkMemoryMeshVertexBufferVisitor::IsBigEndian(data.is_big_endian.into()),
            HkMemoryMeshVertexBufferVisitor::IsSharable(data.is_sharable.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkMemoryMeshVertexBuffer {
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
enum HkMemoryMeshVertexBufferVisitor {
    // C++ Parent class(`hkMeshVertexBuffer` => parent: `hkReferencedObject`) has no fields
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
    #[serde(rename = "format")]
    Format(SingleClass<HkVertexFormat>),
    /// Visitor fields
    #[serde(rename = "elementOffsets")]
    ElementOffsets(CStyleArray<[i32; 32]>),
    /// Visitor fields
    #[serde(rename = "memory")]
    Memory(HkArrayNum<u8>),
    /// Visitor fields
    #[serde(rename = "vertexStride")]
    VertexStride(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "locked")]
    Locked(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "numVertices")]
    NumVertices(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "isBigEndian")]
    IsBigEndian(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "isSharable")]
    IsSharable(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMemoryMeshVertexBufferVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("format" => Format(SingleClass<HkVertexFormat>)),
    ("elementOffsets" => ElementOffsets(CStyleArray<[i32; 32]>)),
    ("memory" => Memory(HkArrayNum<u8>)),
    ("vertexStride" => VertexStride(Primitive<i32>)),
    ("locked" => Locked(Primitive<bool>)),
    ("numVertices" => NumVertices(Primitive<i32>)),
    ("isBigEndian" => IsBigEndian(Primitive<bool>)),
    ("isSharable" => IsSharable(Primitive<bool>)),
}
