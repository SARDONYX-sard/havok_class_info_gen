//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbMoveCharacterModifierInternalState`
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

/// `hkbMoveCharacterModifierInternalState`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x28f67ba0`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbMoveCharacterModifierInternalState {
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
    /// -   name:`"timeSinceLastModify"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub time_since_last_modify: f32,
}

impl Serialize for HkbMoveCharacterModifierInternalState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbMoveCharacterModifierInternalStateVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbMoveCharacterModifierInternalState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbMoveCharacterModifierInternalStateVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbMoveCharacterModifierInternalStateVisitor>> for HkbMoveCharacterModifierInternalState {
    fn from(_values: Vec<HkbMoveCharacterModifierInternalStateVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut time_since_last_modify = None;


        for _value in _values {
            match _value {
                HkbMoveCharacterModifierInternalStateVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbMoveCharacterModifierInternalStateVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbMoveCharacterModifierInternalStateVisitor::TimeSinceLastModify(m) => time_since_last_modify = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            time_since_last_modify: time_since_last_modify.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbMoveCharacterModifierInternalState> for Vec<HkbMoveCharacterModifierInternalStateVisitor> {
    fn from(data: &HkbMoveCharacterModifierInternalState) -> Self {
        vec![
            HkbMoveCharacterModifierInternalStateVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbMoveCharacterModifierInternalStateVisitor::ReferenceCount(data.reference_count.into()),
            HkbMoveCharacterModifierInternalStateVisitor::TimeSinceLastModify(data.time_since_last_modify.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbMoveCharacterModifierInternalState {
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
enum HkbMoveCharacterModifierInternalStateVisitor {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "timeSinceLastModify")]
    TimeSinceLastModify(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbMoveCharacterModifierInternalStateVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("timeSinceLastModify" => TimeSinceLastModify(Primitive<f32>)),
}
