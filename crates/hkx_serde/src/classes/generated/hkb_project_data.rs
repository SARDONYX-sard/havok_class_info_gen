//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbProjectData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#![allow(clippy::clone_on_copy, clippy::unit_arg)]

#[allow(unused)]
use super::*;
#[allow(unused)]
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkbProjectData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x13a39ba7`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbProjectData<'a> {
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
    /// -   name:`"worldUpWS"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub world_up_ws: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"stringData"`
    /// -   type: `struct hkbProjectStringData*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub string_data: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"defaultEventMode"`
    /// -   type: `enum EventMode`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub default_event_mode: EventMode,
}

impl Serialize for HkbProjectData<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbProjectDataVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbProjectData<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbProjectDataVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbProjectDataVisitor<'a>>> for HkbProjectData<'a> {
    fn from(_values: Vec<HkbProjectDataVisitor<'a>>) -> Self {
        let mut mem_size_and_flags = None;
        let mut reference_count = None;
        let mut world_up_ws = None;
        let mut string_data = None;
        let mut default_event_mode = None;

        for _value in _values {
            match _value {
                HkbProjectDataVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbProjectDataVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbProjectDataVisitor::WorldUpWs(m) => world_up_ws = Some(m),
                HkbProjectDataVisitor::StringData(m) => string_data = Some(m),
                HkbProjectDataVisitor::DefaultEventMode(m) => default_event_mode = Some(m),
            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            world_up_ws: world_up_ws.unwrap_or_default().into_inner(),
            string_data: string_data.unwrap_or_default().into_inner(),
            default_event_mode: default_event_mode.unwrap_or_default().into_inner(),
        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbProjectData<'a>> for Vec<HkbProjectDataVisitor<'a>> {
    fn from(data: &HkbProjectData<'a>) -> Self {
        vec![
            // HkbProjectDataVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            // HkbProjectDataVisitor::ReferenceCount(data.reference_count.into()),
            HkbProjectDataVisitor::WorldUpWs(data.world_up_ws.into()),
            HkbProjectDataVisitor::StringData(data.string_data.clone().into()),
            HkbProjectDataVisitor::DefaultEventMode(data.default_event_mode.clone().into()),
        ]
    }
}

impl<'de> ByteDeSerialize<'de> for HkbProjectData<'de> {
    fn from_bytes<D>(deserializer: &'de D, position: &mut u32) -> Result<Self>
    where
        D: ByteDeserializer,
        Self: Sized + 'de,
    {
        // `hkBaseObject`
        deserializer.read_usize(position)?; // offset: 0

        // `hkReferencedObject`
        let mem_size_and_flags = deserializer.read_u16(position)?; // offset: 8
        let reference_count = deserializer.read_i16(position)?; // offset: 10
        deserializer.align_usize(position); // offset: 12

        // `hkbProjectData`
        let world_up_ws = deserializer.read_vector4(position)?; // offset: 20
        let string_data = deserializer.read_class_ptr(position)?; // offset: 36
        let default_event_mode = EventMode::from_u8(deserializer.read_u8(position)?).unwrap(); // offset: 44
        *position += 7; // C ++ size is 48, current 45 = 3

        Ok(Self {
            mem_size_and_flags,
            reference_count,
            world_up_ws,
            string_data,
            default_event_mode,
        })
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
enum HkbProjectDataVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "worldUpWS")]
    WorldUpWs(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "stringData")]
    StringData(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "defaultEventMode")]
    DefaultEventMode(Primitive<EventMode>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbProjectDataVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("worldUpWS" => WorldUpWs(Primitive<Vector4<f32>>)),
    ("stringData" => StringData(Primitive<Cow<'de, str>>)),
    ("defaultEventMode" => DefaultEventMode(Primitive<EventMode>)),
}
