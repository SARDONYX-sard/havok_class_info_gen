//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbStateMachineTransitionInfoReference`
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

/// `hkbStateMachineTransitionInfoReference`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 6
/// -    vtable: false
/// - signature: `0x9810c2d0`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbStateMachineTransitionInfoReference {
    /// # C++ Class Fields Info
    /// -   name:`"fromStateIndex"`
    /// -   type: `hkInt16`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub from_state_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"transitionIndex"`
    /// -   type: `hkInt16`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    pub transition_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"stateMachineId"`
    /// -   type: `hkInt16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub state_machine_id: i16,
}

impl Serialize for HkbStateMachineTransitionInfoReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbStateMachineTransitionInfoReferenceVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbStateMachineTransitionInfoReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbStateMachineTransitionInfoReferenceVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbStateMachineTransitionInfoReferenceVisitor>> for HkbStateMachineTransitionInfoReference {
    fn from(_values: Vec<HkbStateMachineTransitionInfoReferenceVisitor>) -> Self {
            let mut from_state_index = None;
            let mut transition_index = None;
            let mut state_machine_id = None;


        for _value in _values {
            match _value {
                HkbStateMachineTransitionInfoReferenceVisitor::FromStateIndex(m) => from_state_index = Some(m),
                HkbStateMachineTransitionInfoReferenceVisitor::TransitionIndex(m) => transition_index = Some(m),
                HkbStateMachineTransitionInfoReferenceVisitor::StateMachineId(m) => state_machine_id = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            from_state_index: from_state_index.unwrap_or_default().into_inner(),
            transition_index: transition_index.unwrap_or_default().into_inner(),
            state_machine_id: state_machine_id.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbStateMachineTransitionInfoReference> for Vec<HkbStateMachineTransitionInfoReferenceVisitor> {
    fn from(data: &HkbStateMachineTransitionInfoReference) -> Self {
        vec![
            HkbStateMachineTransitionInfoReferenceVisitor::FromStateIndex(data.from_state_index.into()),
            HkbStateMachineTransitionInfoReferenceVisitor::TransitionIndex(data.transition_index.into()),
            HkbStateMachineTransitionInfoReferenceVisitor::StateMachineId(data.state_machine_id.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbStateMachineTransitionInfoReference {
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
enum HkbStateMachineTransitionInfoReferenceVisitor {
    /// Visitor fields
    #[serde(rename = "fromStateIndex")]
    FromStateIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "transitionIndex")]
    TransitionIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "stateMachineId")]
    StateMachineId(Primitive<i16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbStateMachineTransitionInfoReferenceVisitor, "@name",
    ("fromStateIndex" => FromStateIndex(Primitive<i16>)),
    ("transitionIndex" => TransitionIndex(Primitive<i16>)),
    ("stateMachineId" => StateMachineId(Primitive<i16>)),
}
