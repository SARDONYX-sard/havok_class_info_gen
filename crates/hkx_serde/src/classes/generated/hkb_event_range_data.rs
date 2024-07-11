//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbEventRangeData`
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

/// `hkbEventRangeData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0x6cb92c76`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbEventRangeData<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"upperBound"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub upper_bound: f32,
    /// # C++ Class Fields Info
    /// -   name:`"event"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub event: SingleClass<HkbEventProperty<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"eventMode"`
    /// -   type: `enum EventRangeMode`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub event_mode: EventRangeMode,
}

impl Serialize for HkbEventRangeData<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbEventRangeDataVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbEventRangeData<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbEventRangeDataVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbEventRangeDataVisitor<'a>>> for HkbEventRangeData<'a> {
    fn from(_values: Vec<HkbEventRangeDataVisitor<'a>>) -> Self {
            let mut upper_bound = None;
            let mut event = None;
            let mut event_mode = None;


        for _value in _values {
            match _value {
                HkbEventRangeDataVisitor::UpperBound(m) => upper_bound = Some(m),
                HkbEventRangeDataVisitor::Event(m) => event = Some(m),
                HkbEventRangeDataVisitor::EventMode(m) => event_mode = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            upper_bound: upper_bound.unwrap_or_default().into_inner(),
            event: event.unwrap_or_default(),
            event_mode: event_mode.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbEventRangeData<'a>> for Vec<HkbEventRangeDataVisitor<'a>> {
    fn from(data: &HkbEventRangeData<'a>) -> Self {
        vec![
            HkbEventRangeDataVisitor::UpperBound(data.upper_bound.into()),
            HkbEventRangeDataVisitor::Event(data.event.clone()),
            HkbEventRangeDataVisitor::EventMode(data.event_mode.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbEventRangeData<'de> {
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
enum HkbEventRangeDataVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "upperBound")]
    UpperBound(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "event")]
    Event(SingleClass<HkbEventProperty<'a>>),
    /// Visitor fields
    #[serde(rename = "eventMode")]
    EventMode(Primitive<EventRangeMode>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbEventRangeDataVisitor<'de>, "@name",
    ("upperBound" => UpperBound(Primitive<f32>)),
    ("event" => Event(SingleClass<HkbEventProperty<'de>>)),
    ("eventMode" => EventMode(Primitive<EventRangeMode>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum EventRangeMode {
    #[serde(rename = "EVENT_MODE_SEND_ON_ENTER_RANGE")]
    #[default]
    EventModeSendOnEnterRange = 0,
    #[serde(rename = "EVENT_MODE_SEND_WHEN_IN_RANGE")]
    EventModeSendWhenInRange = 1,
}
