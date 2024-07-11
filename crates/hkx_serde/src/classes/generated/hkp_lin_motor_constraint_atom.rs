//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpLinMotorConstraintAtom`
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

/// `hkpLinMotorConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0x10312464`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpLinMotorConstraintAtom<'a> {
    /// # C++ Parent class(`hkpConstraintAtom` => parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub _type: AtomType,

    /// # C++ Class Fields Info
    /// -   name:`"isEnabled"`
    /// -   type: `hkBool`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    pub is_enabled: bool,
    /// # C++ Class Fields Info
    /// -   name:`"motorAxis"`
    /// -   type: `hkUint8`
    /// - offset: 3
    /// -  flags: `FLAGS_NONE`
    pub motor_axis: u8,
    /// # C++ Class Fields Info
    /// -   name:`"initializedOffset"`
    /// -   type: `hkInt16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub initialized_offset: i16,
    /// # C++ Class Fields Info
    /// -   name:`"previousTargetPositionOffset"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    pub previous_target_position_offset: i16,
    /// # C++ Class Fields Info
    /// -   name:`"targetPosition"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub target_position: f32,
    /// # C++ Class Fields Info
    /// -   name:`"motor"`
    /// -   type: `struct hkpConstraintMotor*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub motor: Cow<'a, str>,
}

impl Serialize for HkpLinMotorConstraintAtom<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpLinMotorConstraintAtomVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpLinMotorConstraintAtom<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpLinMotorConstraintAtomVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpLinMotorConstraintAtomVisitor<'a>>> for HkpLinMotorConstraintAtom<'a> {
    fn from(_values: Vec<HkpLinMotorConstraintAtomVisitor<'a>>) -> Self {
            let mut _type = None;
            let mut is_enabled = None;
            let mut motor_axis = None;
            let mut initialized_offset = None;
            let mut previous_target_position_offset = None;
            let mut target_position = None;
            let mut motor = None;


        for _value in _values {
            match _value {
                HkpLinMotorConstraintAtomVisitor::Type(m) => _type = Some(m),
                HkpLinMotorConstraintAtomVisitor::IsEnabled(m) => is_enabled = Some(m),
                HkpLinMotorConstraintAtomVisitor::MotorAxis(m) => motor_axis = Some(m),
                HkpLinMotorConstraintAtomVisitor::InitializedOffset(m) => initialized_offset = Some(m),
                HkpLinMotorConstraintAtomVisitor::PreviousTargetPositionOffset(m) => previous_target_position_offset = Some(m),
                HkpLinMotorConstraintAtomVisitor::TargetPosition(m) => target_position = Some(m),
                HkpLinMotorConstraintAtomVisitor::Motor(m) => motor = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            _type: _type.unwrap_or_default().into_inner(),
            is_enabled: is_enabled.unwrap_or_default().into_inner(),
            motor_axis: motor_axis.unwrap_or_default().into_inner(),
            initialized_offset: initialized_offset.unwrap_or_default().into_inner(),
            previous_target_position_offset: previous_target_position_offset.unwrap_or_default().into_inner(),
            target_position: target_position.unwrap_or_default().into_inner(),
            motor: motor.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpLinMotorConstraintAtom<'a>> for Vec<HkpLinMotorConstraintAtomVisitor<'a>> {
    fn from(data: &HkpLinMotorConstraintAtom<'a>) -> Self {
        vec![
            HkpLinMotorConstraintAtomVisitor::Type(data._type.clone().into()),
            HkpLinMotorConstraintAtomVisitor::IsEnabled(data.is_enabled.into()),
            HkpLinMotorConstraintAtomVisitor::MotorAxis(data.motor_axis.into()),
            HkpLinMotorConstraintAtomVisitor::InitializedOffset(data.initialized_offset.into()),
            HkpLinMotorConstraintAtomVisitor::PreviousTargetPositionOffset(data.previous_target_position_offset.into()),
            HkpLinMotorConstraintAtomVisitor::TargetPosition(data.target_position.into()),
            HkpLinMotorConstraintAtomVisitor::Motor(data.motor.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpLinMotorConstraintAtom<'de> {
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
enum HkpLinMotorConstraintAtomVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<AtomType>),

    /// Visitor fields
    #[serde(rename = "isEnabled")]
    IsEnabled(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "motorAxis")]
    MotorAxis(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "initializedOffset")]
    InitializedOffset(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "previousTargetPositionOffset")]
    PreviousTargetPositionOffset(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "targetPosition")]
    TargetPosition(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "motor")]
    Motor(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpLinMotorConstraintAtomVisitor<'de>, "@name",
    ("type" => Type(Primitive<AtomType>)),
    ("isEnabled" => IsEnabled(Primitive<bool>)),
    ("motorAxis" => MotorAxis(Primitive<u8>)),
    ("initializedOffset" => InitializedOffset(Primitive<i16>)),
    ("previousTargetPositionOffset" => PreviousTargetPositionOffset(Primitive<i16>)),
    ("targetPosition" => TargetPosition(Primitive<f32>)),
    ("motor" => Motor(Primitive<Cow<'de, str>>)),
}
