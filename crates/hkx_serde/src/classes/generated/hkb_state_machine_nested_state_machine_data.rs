//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbStateMachineNestedStateMachineData`
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

/// `hkbStateMachineNestedStateMachineData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// - signature: `0x7358f5da`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbStateMachineNestedStateMachineData<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"nestedStateMachine"`
    /// -   type: `void*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub nested_state_machine: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"eventIdMap"`
    /// -   type: `void*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub event_id_map: Cow<'a, str>,
}

impl Serialize for HkbStateMachineNestedStateMachineData<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbStateMachineNestedStateMachineDataVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbStateMachineNestedStateMachineData<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbStateMachineNestedStateMachineDataVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbStateMachineNestedStateMachineDataVisitor<'a>>> for HkbStateMachineNestedStateMachineData<'a> {
    fn from(_values: Vec<HkbStateMachineNestedStateMachineDataVisitor<'a>>) -> Self {
            let mut nested_state_machine = None;
            let mut event_id_map = None;


        for _value in _values {
            match _value {
                HkbStateMachineNestedStateMachineDataVisitor::NestedStateMachine(m) => nested_state_machine = Some(m),
                HkbStateMachineNestedStateMachineDataVisitor::EventIdMap(m) => event_id_map = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            nested_state_machine: nested_state_machine.unwrap_or_default().into_inner(),
            event_id_map: event_id_map.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbStateMachineNestedStateMachineData<'a>> for Vec<HkbStateMachineNestedStateMachineDataVisitor<'a>> {
    fn from(data: &HkbStateMachineNestedStateMachineData<'a>) -> Self {
        vec![
            HkbStateMachineNestedStateMachineDataVisitor::NestedStateMachine(data.nested_state_machine.clone().into()),
            HkbStateMachineNestedStateMachineDataVisitor::EventIdMap(data.event_id_map.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbStateMachineNestedStateMachineData<'de> {
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
enum HkbStateMachineNestedStateMachineDataVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "nestedStateMachine", skip_serializing)]
    NestedStateMachine(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "eventIdMap", skip_serializing)]
    EventIdMap(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbStateMachineNestedStateMachineDataVisitor<'de>, "@name",
    ("nestedStateMachine" => NestedStateMachine(Primitive<Cow<'de, str>>)),
    ("eventIdMap" => EventIdMap(Primitive<Cow<'de, str>>)),
}
