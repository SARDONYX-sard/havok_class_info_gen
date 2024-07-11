//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbStateMachineActiveTransitionInfo`
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

/// `hkbStateMachineActiveTransitionInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: false
/// - signature: `0xbb90d54f`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbStateMachineActiveTransitionInfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"transitionEffect"`
    /// -   type: `void*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub transition_effect: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"transitionEffectInternalStateInfo"`
    /// -   type: `struct hkbNodeInternalStateInfo*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub transition_effect_internal_state_info: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"transitionInfoReference"`
    /// -   type: `struct hkbStateMachineTransitionInfoReference`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub transition_info_reference: SingleClass<HkbStateMachineTransitionInfoReference>,
    /// # C++ Class Fields Info
    /// -   name:`"transitionInfoReferenceForTE"`
    /// -   type: `struct hkbStateMachineTransitionInfoReference`
    /// - offset: 14
    /// -  flags: `FLAGS_NONE`
    pub transition_info_reference_for_te: SingleClass<HkbStateMachineTransitionInfoReference>,
    /// # C++ Class Fields Info
    /// -   name:`"fromStateId"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub from_state_id: i32,
    /// # C++ Class Fields Info
    /// -   name:`"toStateId"`
    /// -   type: `hkInt32`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub to_state_id: i32,
    /// # C++ Class Fields Info
    /// -   name:`"isReturnToPreviousState"`
    /// -   type: `hkBool`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub is_return_to_previous_state: bool,
}

impl Serialize for HkbStateMachineActiveTransitionInfo<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbStateMachineActiveTransitionInfoVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbStateMachineActiveTransitionInfo<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbStateMachineActiveTransitionInfoVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbStateMachineActiveTransitionInfoVisitor<'a>>> for HkbStateMachineActiveTransitionInfo<'a> {
    fn from(_values: Vec<HkbStateMachineActiveTransitionInfoVisitor<'a>>) -> Self {
            let mut transition_effect = None;
            let mut transition_effect_internal_state_info = None;
            let mut transition_info_reference = None;
            let mut transition_info_reference_for_te = None;
            let mut from_state_id = None;
            let mut to_state_id = None;
            let mut is_return_to_previous_state = None;


        for _value in _values {
            match _value {
                HkbStateMachineActiveTransitionInfoVisitor::TransitionEffect(m) => transition_effect = Some(m),
                HkbStateMachineActiveTransitionInfoVisitor::TransitionEffectInternalStateInfo(m) => transition_effect_internal_state_info = Some(m),
                HkbStateMachineActiveTransitionInfoVisitor::TransitionInfoReference(m) => transition_info_reference = Some(m),
                HkbStateMachineActiveTransitionInfoVisitor::TransitionInfoReferenceForTe(m) => transition_info_reference_for_te = Some(m),
                HkbStateMachineActiveTransitionInfoVisitor::FromStateId(m) => from_state_id = Some(m),
                HkbStateMachineActiveTransitionInfoVisitor::ToStateId(m) => to_state_id = Some(m),
                HkbStateMachineActiveTransitionInfoVisitor::IsReturnToPreviousState(m) => is_return_to_previous_state = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            transition_effect: transition_effect.unwrap_or_default().into_inner(),
            transition_effect_internal_state_info: transition_effect_internal_state_info.unwrap_or_default().into_inner(),
            transition_info_reference: transition_info_reference.unwrap_or_default(),
            transition_info_reference_for_te: transition_info_reference_for_te.unwrap_or_default(),
            from_state_id: from_state_id.unwrap_or_default().into_inner(),
            to_state_id: to_state_id.unwrap_or_default().into_inner(),
            is_return_to_previous_state: is_return_to_previous_state.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbStateMachineActiveTransitionInfo<'a>> for Vec<HkbStateMachineActiveTransitionInfoVisitor<'a>> {
    fn from(data: &HkbStateMachineActiveTransitionInfo<'a>) -> Self {
        vec![
            HkbStateMachineActiveTransitionInfoVisitor::TransitionEffect(data.transition_effect.clone().into()),
            HkbStateMachineActiveTransitionInfoVisitor::TransitionEffectInternalStateInfo(data.transition_effect_internal_state_info.clone().into()),
            HkbStateMachineActiveTransitionInfoVisitor::TransitionInfoReference(data.transition_info_reference.clone()),
            HkbStateMachineActiveTransitionInfoVisitor::TransitionInfoReferenceForTe(data.transition_info_reference_for_te.clone()),
            HkbStateMachineActiveTransitionInfoVisitor::FromStateId(data.from_state_id.into()),
            HkbStateMachineActiveTransitionInfoVisitor::ToStateId(data.to_state_id.into()),
            HkbStateMachineActiveTransitionInfoVisitor::IsReturnToPreviousState(data.is_return_to_previous_state.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbStateMachineActiveTransitionInfo<'de> {
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
enum HkbStateMachineActiveTransitionInfoVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "transitionEffect", skip_serializing)]
    TransitionEffect(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "transitionEffectInternalStateInfo")]
    TransitionEffectInternalStateInfo(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "transitionInfoReference")]
    TransitionInfoReference(SingleClass<HkbStateMachineTransitionInfoReference>),
    /// Visitor fields
    #[serde(rename = "transitionInfoReferenceForTE")]
    TransitionInfoReferenceForTe(SingleClass<HkbStateMachineTransitionInfoReference>),
    /// Visitor fields
    #[serde(rename = "fromStateId")]
    FromStateId(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "toStateId")]
    ToStateId(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "isReturnToPreviousState")]
    IsReturnToPreviousState(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbStateMachineActiveTransitionInfoVisitor<'de>, "@name",
    ("transitionEffect" => TransitionEffect(Primitive<Cow<'de, str>>)),
    ("transitionEffectInternalStateInfo" => TransitionEffectInternalStateInfo(Primitive<Cow<'de, str>>)),
    ("transitionInfoReference" => TransitionInfoReference(SingleClass<HkbStateMachineTransitionInfoReference>)),
    ("transitionInfoReferenceForTE" => TransitionInfoReferenceForTe(SingleClass<HkbStateMachineTransitionInfoReference>)),
    ("fromStateId" => FromStateId(Primitive<i32>)),
    ("toStateId" => ToStateId(Primitive<i32>)),
    ("isReturnToPreviousState" => IsReturnToPreviousState(Primitive<bool>)),
}
