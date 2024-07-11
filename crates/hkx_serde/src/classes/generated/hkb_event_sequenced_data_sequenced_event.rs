//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbEventSequencedDataSequencedEvent`
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

/// `hkbEventSequencedDataSequencedEvent`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0x9139b821`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbEventSequencedDataSequencedEvent<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"event"`
    /// -   type: `struct hkbEvent`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub event: SingleClass<HkbEvent<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"time"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub time: f32,
}

impl Serialize for HkbEventSequencedDataSequencedEvent<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbEventSequencedDataSequencedEventVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbEventSequencedDataSequencedEvent<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbEventSequencedDataSequencedEventVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbEventSequencedDataSequencedEventVisitor<'a>>> for HkbEventSequencedDataSequencedEvent<'a> {
    fn from(_values: Vec<HkbEventSequencedDataSequencedEventVisitor<'a>>) -> Self {
            let mut event = None;
            let mut time = None;


        for _value in _values {
            match _value {
                HkbEventSequencedDataSequencedEventVisitor::Event(m) => event = Some(m),
                HkbEventSequencedDataSequencedEventVisitor::Time(m) => time = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            event: event.unwrap_or_default(),
            time: time.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbEventSequencedDataSequencedEvent<'a>> for Vec<HkbEventSequencedDataSequencedEventVisitor<'a>> {
    fn from(data: &HkbEventSequencedDataSequencedEvent<'a>) -> Self {
        vec![
            HkbEventSequencedDataSequencedEventVisitor::Event(data.event.clone()),
            HkbEventSequencedDataSequencedEventVisitor::Time(data.time.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbEventSequencedDataSequencedEvent<'de> {
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
enum HkbEventSequencedDataSequencedEventVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "event")]
    Event(SingleClass<HkbEvent<'a>>),
    /// Visitor fields
    #[serde(rename = "time")]
    Time(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbEventSequencedDataSequencedEventVisitor<'de>, "@name",
    ("event" => Event(SingleClass<HkbEvent<'de>>)),
    ("time" => Time(Primitive<f32>)),
}
