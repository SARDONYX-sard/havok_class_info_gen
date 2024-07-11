//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbStateMachineDelayedTransitionInfo`
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

/// `hkbStateMachineDelayedTransitionInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: false
/// - signature: `0x26d5499`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbStateMachineDelayedTransitionInfo {
    /// # C++ Class Fields Info
    /// -   name:`"delayedTransition"`
    /// -   type: `struct hkbStateMachineProspectiveTransitionInfo`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub delayed_transition: SingleClass<HkbStateMachineProspectiveTransitionInfo>,
    /// # C++ Class Fields Info
    /// -   name:`"timeDelayed"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub time_delayed: f32,
    /// # C++ Class Fields Info
    /// -   name:`"isDelayedTransitionReturnToPreviousState"`
    /// -   type: `hkBool`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub is_delayed_transition_return_to_previous_state: bool,
    /// # C++ Class Fields Info
    /// -   name:`"wasInAbutRangeLastFrame"`
    /// -   type: `hkBool`
    /// - offset: 21
    /// -  flags: `FLAGS_NONE`
    pub was_in_abut_range_last_frame: bool,
}

impl Serialize for HkbStateMachineDelayedTransitionInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbStateMachineDelayedTransitionInfoVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbStateMachineDelayedTransitionInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbStateMachineDelayedTransitionInfoVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbStateMachineDelayedTransitionInfoVisitor>> for HkbStateMachineDelayedTransitionInfo {
    fn from(_values: Vec<HkbStateMachineDelayedTransitionInfoVisitor>) -> Self {
            let mut delayed_transition = None;
            let mut time_delayed = None;
            let mut is_delayed_transition_return_to_previous_state = None;
            let mut was_in_abut_range_last_frame = None;


        for _value in _values {
            match _value {
                HkbStateMachineDelayedTransitionInfoVisitor::DelayedTransition(m) => delayed_transition = Some(m),
                HkbStateMachineDelayedTransitionInfoVisitor::TimeDelayed(m) => time_delayed = Some(m),
                HkbStateMachineDelayedTransitionInfoVisitor::IsDelayedTransitionReturnToPreviousState(m) => is_delayed_transition_return_to_previous_state = Some(m),
                HkbStateMachineDelayedTransitionInfoVisitor::WasInAbutRangeLastFrame(m) => was_in_abut_range_last_frame = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            delayed_transition: delayed_transition.unwrap_or_default(),
            time_delayed: time_delayed.unwrap_or_default().into_inner(),
            is_delayed_transition_return_to_previous_state: is_delayed_transition_return_to_previous_state.unwrap_or_default().into_inner(),
            was_in_abut_range_last_frame: was_in_abut_range_last_frame.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbStateMachineDelayedTransitionInfo> for Vec<HkbStateMachineDelayedTransitionInfoVisitor> {
    fn from(data: &HkbStateMachineDelayedTransitionInfo) -> Self {
        vec![
            HkbStateMachineDelayedTransitionInfoVisitor::DelayedTransition(data.delayed_transition.clone()),
            HkbStateMachineDelayedTransitionInfoVisitor::TimeDelayed(data.time_delayed.into()),
            HkbStateMachineDelayedTransitionInfoVisitor::IsDelayedTransitionReturnToPreviousState(data.is_delayed_transition_return_to_previous_state.into()),
            HkbStateMachineDelayedTransitionInfoVisitor::WasInAbutRangeLastFrame(data.was_in_abut_range_last_frame.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbStateMachineDelayedTransitionInfo {
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
enum HkbStateMachineDelayedTransitionInfoVisitor {
    /// Visitor fields
    #[serde(rename = "delayedTransition")]
    DelayedTransition(SingleClass<HkbStateMachineProspectiveTransitionInfo>),
    /// Visitor fields
    #[serde(rename = "timeDelayed")]
    TimeDelayed(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "isDelayedTransitionReturnToPreviousState")]
    IsDelayedTransitionReturnToPreviousState(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "wasInAbutRangeLastFrame")]
    WasInAbutRangeLastFrame(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbStateMachineDelayedTransitionInfoVisitor, "@name",
    ("delayedTransition" => DelayedTransition(SingleClass<HkbStateMachineProspectiveTransitionInfo>)),
    ("timeDelayed" => TimeDelayed(Primitive<f32>)),
    ("isDelayedTransitionReturnToPreviousState" => IsDelayedTransitionReturnToPreviousState(Primitive<bool>)),
    ("wasInAbutRangeLastFrame" => WasInAbutRangeLastFrame(Primitive<bool>)),
}
