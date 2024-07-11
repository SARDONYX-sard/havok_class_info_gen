//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BSIStateManagerModifierBSiStateData`
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

/// `BSIStateManagerModifierBSiStateData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0x6b8a15fc`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BsiStateManagerModifierBSiStateData<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"pStateMachine"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub p_state_machine: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"StateID"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub state_id: i32,
    /// # C++ Class Fields Info
    /// -   name:`"iStateToSetAs"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub i_state_to_set_as: i32,
}

impl Serialize for BsiStateManagerModifierBSiStateData<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<BsiStateManagerModifierBSiStateDataVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BsiStateManagerModifierBSiStateData<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<BsiStateManagerModifierBSiStateDataVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<BsiStateManagerModifierBSiStateDataVisitor<'a>>> for BsiStateManagerModifierBSiStateData<'a> {
    fn from(_values: Vec<BsiStateManagerModifierBSiStateDataVisitor<'a>>) -> Self {
            let mut p_state_machine = None;
            let mut state_id = None;
            let mut i_state_to_set_as = None;


        for _value in _values {
            match _value {
                BsiStateManagerModifierBSiStateDataVisitor::PStateMachine(m) => p_state_machine = Some(m),
                BsiStateManagerModifierBSiStateDataVisitor::StateId(m) => state_id = Some(m),
                BsiStateManagerModifierBSiStateDataVisitor::IStateToSetAs(m) => i_state_to_set_as = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            p_state_machine: p_state_machine.unwrap_or_default().into_inner(),
            state_id: state_id.unwrap_or_default().into_inner(),
            i_state_to_set_as: i_state_to_set_as.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&BsiStateManagerModifierBSiStateData<'a>> for Vec<BsiStateManagerModifierBSiStateDataVisitor<'a>> {
    fn from(data: &BsiStateManagerModifierBSiStateData<'a>) -> Self {
        vec![
            BsiStateManagerModifierBSiStateDataVisitor::PStateMachine(data.p_state_machine.clone().into()),
            BsiStateManagerModifierBSiStateDataVisitor::StateId(data.state_id.into()),
            BsiStateManagerModifierBSiStateDataVisitor::IStateToSetAs(data.i_state_to_set_as.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for BsiStateManagerModifierBSiStateData<'de> {
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
enum BsiStateManagerModifierBSiStateDataVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "pStateMachine")]
    PStateMachine(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "StateID")]
    StateId(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "iStateToSetAs")]
    IStateToSetAs(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsiStateManagerModifierBSiStateDataVisitor<'de>, "@name",
    ("pStateMachine" => PStateMachine(Primitive<Cow<'de, str>>)),
    ("StateID" => StateId(Primitive<i32>)),
    ("iStateToSetAs" => IStateToSetAs(Primitive<i32>)),
}
