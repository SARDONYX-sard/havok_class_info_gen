//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbStateMachineTimeInterval`
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

/// `hkbStateMachineTimeInterval`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0x60a881e5`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbStateMachineTimeInterval {
    /// # C++ Class Fields Info
    /// -   name:`"enterEventId"`
    /// -   type: `hkInt32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub enter_event_id: i32,
    /// # C++ Class Fields Info
    /// -   name:`"exitEventId"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub exit_event_id: i32,
    /// # C++ Class Fields Info
    /// -   name:`"enterTime"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub enter_time: f32,
    /// # C++ Class Fields Info
    /// -   name:`"exitTime"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub exit_time: f32,
}

impl Serialize for HkbStateMachineTimeInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbStateMachineTimeIntervalVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbStateMachineTimeInterval {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbStateMachineTimeIntervalVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbStateMachineTimeIntervalVisitor>> for HkbStateMachineTimeInterval {
    fn from(_values: Vec<HkbStateMachineTimeIntervalVisitor>) -> Self {
            let mut enter_event_id = None;
            let mut exit_event_id = None;
            let mut enter_time = None;
            let mut exit_time = None;


        for _value in _values {
            match _value {
                HkbStateMachineTimeIntervalVisitor::EnterEventId(m) => enter_event_id = Some(m),
                HkbStateMachineTimeIntervalVisitor::ExitEventId(m) => exit_event_id = Some(m),
                HkbStateMachineTimeIntervalVisitor::EnterTime(m) => enter_time = Some(m),
                HkbStateMachineTimeIntervalVisitor::ExitTime(m) => exit_time = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            enter_event_id: enter_event_id.unwrap_or_default().into_inner(),
            exit_event_id: exit_event_id.unwrap_or_default().into_inner(),
            enter_time: enter_time.unwrap_or_default().into_inner(),
            exit_time: exit_time.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbStateMachineTimeInterval> for Vec<HkbStateMachineTimeIntervalVisitor> {
    fn from(data: &HkbStateMachineTimeInterval) -> Self {
        vec![
            HkbStateMachineTimeIntervalVisitor::EnterEventId(data.enter_event_id.into()),
            HkbStateMachineTimeIntervalVisitor::ExitEventId(data.exit_event_id.into()),
            HkbStateMachineTimeIntervalVisitor::EnterTime(data.enter_time.into()),
            HkbStateMachineTimeIntervalVisitor::ExitTime(data.exit_time.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbStateMachineTimeInterval {
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
enum HkbStateMachineTimeIntervalVisitor {
    /// Visitor fields
    #[serde(rename = "enterEventId")]
    EnterEventId(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "exitEventId")]
    ExitEventId(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "enterTime")]
    EnterTime(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "exitTime")]
    ExitTime(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbStateMachineTimeIntervalVisitor, "@name",
    ("enterEventId" => EnterEventId(Primitive<i32>)),
    ("exitEventId" => ExitEventId(Primitive<i32>)),
    ("enterTime" => EnterTime(Primitive<f32>)),
    ("exitTime" => ExitTime(Primitive<f32>)),
}
