//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbStateMachineProspectiveTransitionInfo`
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

/// `hkbStateMachineProspectiveTransitionInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0x3ab09a2e`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbStateMachineProspectiveTransitionInfo {
    /// # C++ Class Fields Info
    /// -   name:`"transitionInfoReference"`
    /// -   type: `struct hkbStateMachineTransitionInfoReference`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub transition_info_reference: SingleClass<HkbStateMachineTransitionInfoReference>,
    /// # C++ Class Fields Info
    /// -   name:`"transitionInfoReferenceForTE"`
    /// -   type: `struct hkbStateMachineTransitionInfoReference`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    pub transition_info_reference_for_te: SingleClass<HkbStateMachineTransitionInfoReference>,
    /// # C++ Class Fields Info
    /// -   name:`"toStateId"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub to_state_id: i32,
}

impl Serialize for HkbStateMachineProspectiveTransitionInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbStateMachineProspectiveTransitionInfoVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbStateMachineProspectiveTransitionInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbStateMachineProspectiveTransitionInfoVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbStateMachineProspectiveTransitionInfoVisitor>> for HkbStateMachineProspectiveTransitionInfo {
    fn from(_values: Vec<HkbStateMachineProspectiveTransitionInfoVisitor>) -> Self {
            let mut transition_info_reference = None;
            let mut transition_info_reference_for_te = None;
            let mut to_state_id = None;


        for _value in _values {
            match _value {
                HkbStateMachineProspectiveTransitionInfoVisitor::TransitionInfoReference(m) => transition_info_reference = Some(m),
                HkbStateMachineProspectiveTransitionInfoVisitor::TransitionInfoReferenceForTe(m) => transition_info_reference_for_te = Some(m),
                HkbStateMachineProspectiveTransitionInfoVisitor::ToStateId(m) => to_state_id = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            transition_info_reference: transition_info_reference.unwrap_or_default(),
            transition_info_reference_for_te: transition_info_reference_for_te.unwrap_or_default(),
            to_state_id: to_state_id.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbStateMachineProspectiveTransitionInfo> for Vec<HkbStateMachineProspectiveTransitionInfoVisitor> {
    fn from(data: &HkbStateMachineProspectiveTransitionInfo) -> Self {
        vec![
            HkbStateMachineProspectiveTransitionInfoVisitor::TransitionInfoReference(data.transition_info_reference.clone()),
            HkbStateMachineProspectiveTransitionInfoVisitor::TransitionInfoReferenceForTe(data.transition_info_reference_for_te.clone()),
            HkbStateMachineProspectiveTransitionInfoVisitor::ToStateId(data.to_state_id.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbStateMachineProspectiveTransitionInfo {
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
enum HkbStateMachineProspectiveTransitionInfoVisitor {
    /// Visitor fields
    #[serde(rename = "transitionInfoReference")]
    TransitionInfoReference(SingleClass<HkbStateMachineTransitionInfoReference>),
    /// Visitor fields
    #[serde(rename = "transitionInfoReferenceForTE")]
    TransitionInfoReferenceForTe(SingleClass<HkbStateMachineTransitionInfoReference>),
    /// Visitor fields
    #[serde(rename = "toStateId")]
    ToStateId(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbStateMachineProspectiveTransitionInfoVisitor, "@name",
    ("transitionInfoReference" => TransitionInfoReference(SingleClass<HkbStateMachineTransitionInfoReference>)),
    ("transitionInfoReferenceForTE" => TransitionInfoReferenceForTe(SingleClass<HkbStateMachineTransitionInfoReference>)),
    ("toStateId" => ToStateId(Primitive<i32>)),
}
