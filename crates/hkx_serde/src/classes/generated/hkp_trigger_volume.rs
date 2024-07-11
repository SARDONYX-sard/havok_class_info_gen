//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpTriggerVolume`
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

/// `hkpTriggerVolume`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 52
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xa29a8d1a`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpTriggerVolume<'a> {
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
    /// -   name:`"overlappingBodies"`
    /// -   type: `hkArray<hkpRigidBody*>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub overlapping_bodies: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"eventQueue"`
    /// -   type: `hkArray<struct hkpTriggerVolumeEventInfo>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub event_queue: HkArrayClass<HkpTriggerVolumeEventInfo<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"triggerBody"`
    /// -   type: `struct hkpRigidBody*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub trigger_body: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"sequenceNumber"`
    /// -   type: `hkUint32`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub sequence_number: u32,
}

impl Serialize for HkpTriggerVolume<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpTriggerVolumeVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpTriggerVolume<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpTriggerVolumeVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpTriggerVolumeVisitor<'a>>> for HkpTriggerVolume<'a> {
    fn from(_values: Vec<HkpTriggerVolumeVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut overlapping_bodies = None;
            let mut event_queue = None;
            let mut trigger_body = None;
            let mut sequence_number = None;


        for _value in _values {
            match _value {
                HkpTriggerVolumeVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpTriggerVolumeVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpTriggerVolumeVisitor::OverlappingBodies(m) => overlapping_bodies = Some(m),
                HkpTriggerVolumeVisitor::EventQueue(m) => event_queue = Some(m),
                HkpTriggerVolumeVisitor::TriggerBody(m) => trigger_body = Some(m),
                HkpTriggerVolumeVisitor::SequenceNumber(m) => sequence_number = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            overlapping_bodies: overlapping_bodies.unwrap_or_default(),
            event_queue: event_queue.unwrap_or_default(),
            trigger_body: trigger_body.unwrap_or_default().into_inner(),
            sequence_number: sequence_number.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpTriggerVolume<'a>> for Vec<HkpTriggerVolumeVisitor<'a>> {
    fn from(data: &HkpTriggerVolume<'a>) -> Self {
        vec![
            HkpTriggerVolumeVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpTriggerVolumeVisitor::ReferenceCount(data.reference_count.into()),
            HkpTriggerVolumeVisitor::OverlappingBodies(data.overlapping_bodies.clone()),
            HkpTriggerVolumeVisitor::EventQueue(data.event_queue.clone()),
            HkpTriggerVolumeVisitor::TriggerBody(data.trigger_body.clone().into()),
            HkpTriggerVolumeVisitor::SequenceNumber(data.sequence_number.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpTriggerVolume<'de> {
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
enum HkpTriggerVolumeVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "overlappingBodies")]
    OverlappingBodies(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "eventQueue")]
    EventQueue(HkArrayClass<HkpTriggerVolumeEventInfo<'a>>),
    /// Visitor fields
    #[serde(rename = "triggerBody")]
    TriggerBody(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "sequenceNumber")]
    SequenceNumber(Primitive<u32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpTriggerVolumeVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("overlappingBodies" => OverlappingBodies(HkArrayRef<Cow<'de, str>>)),
    ("eventQueue" => EventQueue(HkArrayClass<HkpTriggerVolumeEventInfo<'de>>)),
    ("triggerBody" => TriggerBody(Primitive<Cow<'de, str>>)),
    ("sequenceNumber" => SequenceNumber(Primitive<u32>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum EventType {
    #[serde(rename = "ENTERED_EVENT")]
    #[default]
    EnteredEvent = 1,
    #[serde(rename = "LEFT_EVENT")]
    LeftEvent = 2,
    #[serde(rename = "ENTERED_AND_LEFT_EVENT")]
    EnteredAndLeftEvent = 3,
    #[serde(rename = "TRIGGER_BODY_LEFT_EVENT")]
    TriggerBodyLeftEvent = 6,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum Operation {
    #[serde(rename = "ADDED_OP")]
    #[default]
    AddedOp = 0,
    #[serde(rename = "REMOVED_OP")]
    RemovedOp = 1,
    #[serde(rename = "CONTACT_OP")]
    ContactOp = 2,
    #[serde(rename = "TOI_OP")]
    ToiOp = 3,
}
