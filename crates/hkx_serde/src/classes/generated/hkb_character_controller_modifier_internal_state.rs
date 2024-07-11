//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbCharacterControllerModifierInternalState`
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

/// `hkbCharacterControllerModifierInternalState`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xf8dfec0d`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbCharacterControllerModifierInternalState {
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
    /// -   name:`"gravity"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub gravity: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"timestep"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub timestep: f32,
    /// # C++ Class Fields Info
    /// -   name:`"isInitialVelocityAdded"`
    /// -   type: `hkBool`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub is_initial_velocity_added: bool,
    /// # C++ Class Fields Info
    /// -   name:`"isTouchingGround"`
    /// -   type: `hkBool`
    /// - offset: 37
    /// -  flags: `FLAGS_NONE`
    pub is_touching_ground: bool,
}

impl Serialize for HkbCharacterControllerModifierInternalState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbCharacterControllerModifierInternalStateVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbCharacterControllerModifierInternalState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbCharacterControllerModifierInternalStateVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbCharacterControllerModifierInternalStateVisitor>> for HkbCharacterControllerModifierInternalState {
    fn from(_values: Vec<HkbCharacterControllerModifierInternalStateVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut gravity = None;
            let mut timestep = None;
            let mut is_initial_velocity_added = None;
            let mut is_touching_ground = None;


        for _value in _values {
            match _value {
                HkbCharacterControllerModifierInternalStateVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbCharacterControllerModifierInternalStateVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbCharacterControllerModifierInternalStateVisitor::Gravity(m) => gravity = Some(m),
                HkbCharacterControllerModifierInternalStateVisitor::Timestep(m) => timestep = Some(m),
                HkbCharacterControllerModifierInternalStateVisitor::IsInitialVelocityAdded(m) => is_initial_velocity_added = Some(m),
                HkbCharacterControllerModifierInternalStateVisitor::IsTouchingGround(m) => is_touching_ground = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            gravity: gravity.unwrap_or_default().into_inner(),
            timestep: timestep.unwrap_or_default().into_inner(),
            is_initial_velocity_added: is_initial_velocity_added.unwrap_or_default().into_inner(),
            is_touching_ground: is_touching_ground.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbCharacterControllerModifierInternalState> for Vec<HkbCharacterControllerModifierInternalStateVisitor> {
    fn from(data: &HkbCharacterControllerModifierInternalState) -> Self {
        vec![
            HkbCharacterControllerModifierInternalStateVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbCharacterControllerModifierInternalStateVisitor::ReferenceCount(data.reference_count.into()),
            HkbCharacterControllerModifierInternalStateVisitor::Gravity(data.gravity.into()),
            HkbCharacterControllerModifierInternalStateVisitor::Timestep(data.timestep.into()),
            HkbCharacterControllerModifierInternalStateVisitor::IsInitialVelocityAdded(data.is_initial_velocity_added.into()),
            HkbCharacterControllerModifierInternalStateVisitor::IsTouchingGround(data.is_touching_ground.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbCharacterControllerModifierInternalState {
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
enum HkbCharacterControllerModifierInternalStateVisitor {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "gravity")]
    Gravity(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "timestep")]
    Timestep(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "isInitialVelocityAdded")]
    IsInitialVelocityAdded(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "isTouchingGround")]
    IsTouchingGround(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacterControllerModifierInternalStateVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("gravity" => Gravity(Primitive<Vector4<f32>>)),
    ("timestep" => Timestep(Primitive<f32>)),
    ("isInitialVelocityAdded" => IsInitialVelocityAdded(Primitive<bool>)),
    ("isTouchingGround" => IsTouchingGround(Primitive<bool>)),
}
