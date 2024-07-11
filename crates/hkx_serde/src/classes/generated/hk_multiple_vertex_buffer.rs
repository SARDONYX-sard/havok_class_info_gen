//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMultipleVertexBuffer`
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

/// `hkMultipleVertexBuffer`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 324
/// -    vtable: true
/// -    parent: `hkMeshVertexBuffer`/`0x534b08c8`
/// - signature: `0xde3ab602`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkMultipleVertexBuffer<'a> {
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
    /// -   name:`"vertexFormat"`
    /// -   type: `struct hkVertexFormat`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub vertex_format: SingleClass<HkVertexFormat>,
    /// # C++ Class Fields Info
    /// -   name:`"lockedElements"`
    /// -   type: `hkArray<struct hkMultipleVertexBufferLockedElement>`
    /// - offset: 268
    /// -  flags: `FLAGS_NONE`
    pub locked_elements: HkArrayClass<HkMultipleVertexBufferLockedElement>,
    /// # C++ Class Fields Info
    /// -   name:`"lockedBuffer"`
    /// -   type: `struct hkMemoryMeshVertexBuffer*`
    /// - offset: 280
    /// -  flags: `FLAGS_NONE`
    pub locked_buffer: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"elementInfos"`
    /// -   type: `hkArray<struct hkMultipleVertexBufferElementInfo>`
    /// - offset: 284
    /// -  flags: `FLAGS_NONE`
    pub element_infos: HkArrayClass<HkMultipleVertexBufferElementInfo>,
    /// # C++ Class Fields Info
    /// -   name:`"vertexBufferInfos"`
    /// -   type: `hkArray<struct hkMultipleVertexBufferVertexBufferInfo>`
    /// - offset: 296
    /// -  flags: `FLAGS_NONE`
    pub vertex_buffer_infos: HkArrayClass<HkMultipleVertexBufferVertexBufferInfo<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"numVertices"`
    /// -   type: `hkInt32`
    /// - offset: 308
    /// -  flags: `FLAGS_NONE`
    pub num_vertices: i32,
    /// # C++ Class Fields Info
    /// -   name:`"isLocked"`
    /// -   type: `hkBool`
    /// - offset: 312
    /// -  flags: `FLAGS_NONE`
    pub is_locked: bool,
    /// # C++ Class Fields Info
    /// -   name:`"updateCount"`
    /// -   type: `hkUint32`
    /// - offset: 316
    /// -  flags: `FLAGS_NONE`
    pub update_count: u32,
    /// # C++ Class Fields Info
    /// -   name:`"writeLock"`
    /// -   type: `hkBool`
    /// - offset: 320
    /// -  flags: `FLAGS_NONE`
    pub write_lock: bool,
    /// # C++ Class Fields Info
    /// -   name:`"isSharable"`
    /// -   type: `hkBool`
    /// - offset: 321
    /// -  flags: `FLAGS_NONE`
    pub is_sharable: bool,
    /// # C++ Class Fields Info
    /// -   name:`"constructionComplete"`
    /// -   type: `hkBool`
    /// - offset: 322
    /// -  flags: `FLAGS_NONE`
    pub construction_complete: bool,
}

impl Serialize for HkMultipleVertexBuffer<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkMultipleVertexBufferVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkMultipleVertexBuffer<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkMultipleVertexBufferVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkMultipleVertexBufferVisitor<'a>>> for HkMultipleVertexBuffer<'a> {
    fn from(_values: Vec<HkMultipleVertexBufferVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut vertex_format = None;
            let mut locked_elements = None;
            let mut locked_buffer = None;
            let mut element_infos = None;
            let mut vertex_buffer_infos = None;
            let mut num_vertices = None;
            let mut is_locked = None;
            let mut update_count = None;
            let mut write_lock = None;
            let mut is_sharable = None;
            let mut construction_complete = None;


        for _value in _values {
            match _value {
                HkMultipleVertexBufferVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkMultipleVertexBufferVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkMultipleVertexBufferVisitor::VertexFormat(m) => vertex_format = Some(m),
                HkMultipleVertexBufferVisitor::LockedElements(m) => locked_elements = Some(m),
                HkMultipleVertexBufferVisitor::LockedBuffer(m) => locked_buffer = Some(m),
                HkMultipleVertexBufferVisitor::ElementInfos(m) => element_infos = Some(m),
                HkMultipleVertexBufferVisitor::VertexBufferInfos(m) => vertex_buffer_infos = Some(m),
                HkMultipleVertexBufferVisitor::NumVertices(m) => num_vertices = Some(m),
                HkMultipleVertexBufferVisitor::IsLocked(m) => is_locked = Some(m),
                HkMultipleVertexBufferVisitor::UpdateCount(m) => update_count = Some(m),
                HkMultipleVertexBufferVisitor::WriteLock(m) => write_lock = Some(m),
                HkMultipleVertexBufferVisitor::IsSharable(m) => is_sharable = Some(m),
                HkMultipleVertexBufferVisitor::ConstructionComplete(m) => construction_complete = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            vertex_format: vertex_format.unwrap_or_default(),
            locked_elements: locked_elements.unwrap_or_default(),
            locked_buffer: locked_buffer.unwrap_or_default().into_inner(),
            element_infos: element_infos.unwrap_or_default(),
            vertex_buffer_infos: vertex_buffer_infos.unwrap_or_default(),
            num_vertices: num_vertices.unwrap_or_default().into_inner(),
            is_locked: is_locked.unwrap_or_default().into_inner(),
            update_count: update_count.unwrap_or_default().into_inner(),
            write_lock: write_lock.unwrap_or_default().into_inner(),
            is_sharable: is_sharable.unwrap_or_default().into_inner(),
            construction_complete: construction_complete.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkMultipleVertexBuffer<'a>> for Vec<HkMultipleVertexBufferVisitor<'a>> {
    fn from(data: &HkMultipleVertexBuffer<'a>) -> Self {
        vec![
            HkMultipleVertexBufferVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkMultipleVertexBufferVisitor::ReferenceCount(data.reference_count.into()),
            HkMultipleVertexBufferVisitor::VertexFormat(data.vertex_format.clone()),
            HkMultipleVertexBufferVisitor::LockedElements(data.locked_elements.clone()),
            HkMultipleVertexBufferVisitor::LockedBuffer(data.locked_buffer.clone().into()),
            HkMultipleVertexBufferVisitor::ElementInfos(data.element_infos.clone()),
            HkMultipleVertexBufferVisitor::VertexBufferInfos(data.vertex_buffer_infos.clone()),
            HkMultipleVertexBufferVisitor::NumVertices(data.num_vertices.into()),
            HkMultipleVertexBufferVisitor::IsLocked(data.is_locked.into()),
            HkMultipleVertexBufferVisitor::UpdateCount(data.update_count.into()),
            HkMultipleVertexBufferVisitor::WriteLock(data.write_lock.into()),
            HkMultipleVertexBufferVisitor::IsSharable(data.is_sharable.into()),
            HkMultipleVertexBufferVisitor::ConstructionComplete(data.construction_complete.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkMultipleVertexBuffer<'de> {
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
enum HkMultipleVertexBufferVisitor<'a> {
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
    #[serde(rename = "vertexFormat")]
    VertexFormat(SingleClass<HkVertexFormat>),
    /// Visitor fields
    #[serde(rename = "lockedElements")]
    LockedElements(HkArrayClass<HkMultipleVertexBufferLockedElement>),
    /// Visitor fields
    #[serde(rename = "lockedBuffer")]
    LockedBuffer(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "elementInfos")]
    ElementInfos(HkArrayClass<HkMultipleVertexBufferElementInfo>),
    /// Visitor fields
    #[serde(rename = "vertexBufferInfos")]
    VertexBufferInfos(HkArrayClass<HkMultipleVertexBufferVertexBufferInfo<'a>>),
    /// Visitor fields
    #[serde(rename = "numVertices")]
    NumVertices(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "isLocked")]
    IsLocked(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "updateCount")]
    UpdateCount(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "writeLock")]
    WriteLock(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "isSharable")]
    IsSharable(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "constructionComplete")]
    ConstructionComplete(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMultipleVertexBufferVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("vertexFormat" => VertexFormat(SingleClass<HkVertexFormat>)),
    ("lockedElements" => LockedElements(HkArrayClass<HkMultipleVertexBufferLockedElement>)),
    ("lockedBuffer" => LockedBuffer(Primitive<Cow<'de, str>>)),
    ("elementInfos" => ElementInfos(HkArrayClass<HkMultipleVertexBufferElementInfo>)),
    ("vertexBufferInfos" => VertexBufferInfos(HkArrayClass<HkMultipleVertexBufferVertexBufferInfo<'de>>)),
    ("numVertices" => NumVertices(Primitive<i32>)),
    ("isLocked" => IsLocked(Primitive<bool>)),
    ("updateCount" => UpdateCount(Primitive<u32>)),
    ("writeLock" => WriteLock(Primitive<bool>)),
    ("isSharable" => IsSharable(Primitive<bool>)),
    ("constructionComplete" => ConstructionComplete(Primitive<bool>)),
}
