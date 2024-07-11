//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbStateMachineInternalState`
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

/// `hkbStateMachineInternalState`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xbd1a7502`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbStateMachineInternalState<'a> {
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
    /// -   name:`"activeTransitions"`
    /// -   type: `hkArray<struct hkbStateMachineActiveTransitionInfo>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub active_transitions: HkArrayClass<HkbStateMachineActiveTransitionInfo<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"transitionFlags"`
    /// -   type: `hkArray<hkUint8>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub transition_flags: HkArrayNum<u8>,
    /// # C++ Class Fields Info
    /// -   name:`"wildcardTransitionFlags"`
    /// -   type: `hkArray<hkUint8>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub wildcard_transition_flags: HkArrayNum<u8>,
    /// # C++ Class Fields Info
    /// -   name:`"delayedTransitions"`
    /// -   type: `hkArray<struct hkbStateMachineDelayedTransitionInfo>`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub delayed_transitions: HkArrayClass<HkbStateMachineDelayedTransitionInfo>,
    /// # C++ Class Fields Info
    /// -   name:`"timeInState"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub time_in_state: f32,
    /// # C++ Class Fields Info
    /// -   name:`"lastLocalTime"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub last_local_time: f32,
    /// # C++ Class Fields Info
    /// -   name:`"currentStateId"`
    /// -   type: `hkInt32`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub current_state_id: i32,
    /// # C++ Class Fields Info
    /// -   name:`"previousStateId"`
    /// -   type: `hkInt32`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub previous_state_id: i32,
    /// # C++ Class Fields Info
    /// -   name:`"nextStartStateIndexOverride"`
    /// -   type: `hkInt32`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    pub next_start_state_index_override: i32,
    /// # C++ Class Fields Info
    /// -   name:`"stateOrTransitionChanged"`
    /// -   type: `hkBool`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    pub state_or_transition_changed: bool,
    /// # C++ Class Fields Info
    /// -   name:`"echoNextUpdate"`
    /// -   type: `hkBool`
    /// - offset: 77
    /// -  flags: `FLAGS_NONE`
    pub echo_next_update: bool,
}

impl Serialize for HkbStateMachineInternalState<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbStateMachineInternalStateVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbStateMachineInternalState<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbStateMachineInternalStateVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbStateMachineInternalStateVisitor<'a>>> for HkbStateMachineInternalState<'a> {
    fn from(_values: Vec<HkbStateMachineInternalStateVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut active_transitions = None;
            let mut transition_flags = None;
            let mut wildcard_transition_flags = None;
            let mut delayed_transitions = None;
            let mut time_in_state = None;
            let mut last_local_time = None;
            let mut current_state_id = None;
            let mut previous_state_id = None;
            let mut next_start_state_index_override = None;
            let mut state_or_transition_changed = None;
            let mut echo_next_update = None;


        for _value in _values {
            match _value {
                HkbStateMachineInternalStateVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbStateMachineInternalStateVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbStateMachineInternalStateVisitor::ActiveTransitions(m) => active_transitions = Some(m),
                HkbStateMachineInternalStateVisitor::TransitionFlags(m) => transition_flags = Some(m),
                HkbStateMachineInternalStateVisitor::WildcardTransitionFlags(m) => wildcard_transition_flags = Some(m),
                HkbStateMachineInternalStateVisitor::DelayedTransitions(m) => delayed_transitions = Some(m),
                HkbStateMachineInternalStateVisitor::TimeInState(m) => time_in_state = Some(m),
                HkbStateMachineInternalStateVisitor::LastLocalTime(m) => last_local_time = Some(m),
                HkbStateMachineInternalStateVisitor::CurrentStateId(m) => current_state_id = Some(m),
                HkbStateMachineInternalStateVisitor::PreviousStateId(m) => previous_state_id = Some(m),
                HkbStateMachineInternalStateVisitor::NextStartStateIndexOverride(m) => next_start_state_index_override = Some(m),
                HkbStateMachineInternalStateVisitor::StateOrTransitionChanged(m) => state_or_transition_changed = Some(m),
                HkbStateMachineInternalStateVisitor::EchoNextUpdate(m) => echo_next_update = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            active_transitions: active_transitions.unwrap_or_default(),
            transition_flags: transition_flags.unwrap_or_default(),
            wildcard_transition_flags: wildcard_transition_flags.unwrap_or_default(),
            delayed_transitions: delayed_transitions.unwrap_or_default(),
            time_in_state: time_in_state.unwrap_or_default().into_inner(),
            last_local_time: last_local_time.unwrap_or_default().into_inner(),
            current_state_id: current_state_id.unwrap_or_default().into_inner(),
            previous_state_id: previous_state_id.unwrap_or_default().into_inner(),
            next_start_state_index_override: next_start_state_index_override.unwrap_or_default().into_inner(),
            state_or_transition_changed: state_or_transition_changed.unwrap_or_default().into_inner(),
            echo_next_update: echo_next_update.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbStateMachineInternalState<'a>> for Vec<HkbStateMachineInternalStateVisitor<'a>> {
    fn from(data: &HkbStateMachineInternalState<'a>) -> Self {
        vec![
            HkbStateMachineInternalStateVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbStateMachineInternalStateVisitor::ReferenceCount(data.reference_count.into()),
            HkbStateMachineInternalStateVisitor::ActiveTransitions(data.active_transitions.clone()),
            HkbStateMachineInternalStateVisitor::TransitionFlags(data.transition_flags.clone()),
            HkbStateMachineInternalStateVisitor::WildcardTransitionFlags(data.wildcard_transition_flags.clone()),
            HkbStateMachineInternalStateVisitor::DelayedTransitions(data.delayed_transitions.clone()),
            HkbStateMachineInternalStateVisitor::TimeInState(data.time_in_state.into()),
            HkbStateMachineInternalStateVisitor::LastLocalTime(data.last_local_time.into()),
            HkbStateMachineInternalStateVisitor::CurrentStateId(data.current_state_id.into()),
            HkbStateMachineInternalStateVisitor::PreviousStateId(data.previous_state_id.into()),
            HkbStateMachineInternalStateVisitor::NextStartStateIndexOverride(data.next_start_state_index_override.into()),
            HkbStateMachineInternalStateVisitor::StateOrTransitionChanged(data.state_or_transition_changed.into()),
            HkbStateMachineInternalStateVisitor::EchoNextUpdate(data.echo_next_update.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbStateMachineInternalState<'de> {
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
enum HkbStateMachineInternalStateVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "activeTransitions")]
    ActiveTransitions(HkArrayClass<HkbStateMachineActiveTransitionInfo<'a>>),
    /// Visitor fields
    #[serde(rename = "transitionFlags")]
    TransitionFlags(HkArrayNum<u8>),
    /// Visitor fields
    #[serde(rename = "wildcardTransitionFlags")]
    WildcardTransitionFlags(HkArrayNum<u8>),
    /// Visitor fields
    #[serde(rename = "delayedTransitions")]
    DelayedTransitions(HkArrayClass<HkbStateMachineDelayedTransitionInfo>),
    /// Visitor fields
    #[serde(rename = "timeInState")]
    TimeInState(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "lastLocalTime")]
    LastLocalTime(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "currentStateId")]
    CurrentStateId(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "previousStateId")]
    PreviousStateId(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "nextStartStateIndexOverride")]
    NextStartStateIndexOverride(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "stateOrTransitionChanged")]
    StateOrTransitionChanged(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "echoNextUpdate")]
    EchoNextUpdate(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbStateMachineInternalStateVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("activeTransitions" => ActiveTransitions(HkArrayClass<HkbStateMachineActiveTransitionInfo<'de>>)),
    ("transitionFlags" => TransitionFlags(HkArrayNum<u8>)),
    ("wildcardTransitionFlags" => WildcardTransitionFlags(HkArrayNum<u8>)),
    ("delayedTransitions" => DelayedTransitions(HkArrayClass<HkbStateMachineDelayedTransitionInfo>)),
    ("timeInState" => TimeInState(Primitive<f32>)),
    ("lastLocalTime" => LastLocalTime(Primitive<f32>)),
    ("currentStateId" => CurrentStateId(Primitive<i32>)),
    ("previousStateId" => PreviousStateId(Primitive<i32>)),
    ("nextStartStateIndexOverride" => NextStartStateIndexOverride(Primitive<i32>)),
    ("stateOrTransitionChanged" => StateOrTransitionChanged(Primitive<bool>)),
    ("echoNextUpdate" => EchoNextUpdate(Primitive<bool>)),
}
