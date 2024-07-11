//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpAngMotorConstraintAtom`
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

/// `hkpAngMotorConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 20
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0x81f087ff`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpAngMotorConstraintAtom<'a> {
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
    /// -   name:`"previousTargetAngleOffset"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    pub previous_target_angle_offset: i16,
    /// # C++ Class Fields Info
    /// -   name:`"correspondingAngLimitSolverResultOffset"`
    /// -   type: `hkInt16`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub corresponding_ang_limit_solver_result_offset: i16,
    /// # C++ Class Fields Info
    /// -   name:`"targetAngle"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub target_angle: f32,
    /// # C++ Class Fields Info
    /// -   name:`"motor"`
    /// -   type: `struct hkpConstraintMotor*`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub motor: Cow<'a, str>,
}

impl Serialize for HkpAngMotorConstraintAtom<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpAngMotorConstraintAtomVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpAngMotorConstraintAtom<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpAngMotorConstraintAtomVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpAngMotorConstraintAtomVisitor<'a>>> for HkpAngMotorConstraintAtom<'a> {
    fn from(_values: Vec<HkpAngMotorConstraintAtomVisitor<'a>>) -> Self {
            let mut _type = None;
            let mut is_enabled = None;
            let mut motor_axis = None;
            let mut initialized_offset = None;
            let mut previous_target_angle_offset = None;
            let mut corresponding_ang_limit_solver_result_offset = None;
            let mut target_angle = None;
            let mut motor = None;


        for _value in _values {
            match _value {
                HkpAngMotorConstraintAtomVisitor::Type(m) => _type = Some(m),
                HkpAngMotorConstraintAtomVisitor::IsEnabled(m) => is_enabled = Some(m),
                HkpAngMotorConstraintAtomVisitor::MotorAxis(m) => motor_axis = Some(m),
                HkpAngMotorConstraintAtomVisitor::InitializedOffset(m) => initialized_offset = Some(m),
                HkpAngMotorConstraintAtomVisitor::PreviousTargetAngleOffset(m) => previous_target_angle_offset = Some(m),
                HkpAngMotorConstraintAtomVisitor::CorrespondingAngLimitSolverResultOffset(m) => corresponding_ang_limit_solver_result_offset = Some(m),
                HkpAngMotorConstraintAtomVisitor::TargetAngle(m) => target_angle = Some(m),
                HkpAngMotorConstraintAtomVisitor::Motor(m) => motor = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            _type: _type.unwrap_or_default().into_inner(),
            is_enabled: is_enabled.unwrap_or_default().into_inner(),
            motor_axis: motor_axis.unwrap_or_default().into_inner(),
            initialized_offset: initialized_offset.unwrap_or_default().into_inner(),
            previous_target_angle_offset: previous_target_angle_offset.unwrap_or_default().into_inner(),
            corresponding_ang_limit_solver_result_offset: corresponding_ang_limit_solver_result_offset.unwrap_or_default().into_inner(),
            target_angle: target_angle.unwrap_or_default().into_inner(),
            motor: motor.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpAngMotorConstraintAtom<'a>> for Vec<HkpAngMotorConstraintAtomVisitor<'a>> {
    fn from(data: &HkpAngMotorConstraintAtom<'a>) -> Self {
        vec![
            HkpAngMotorConstraintAtomVisitor::Type(data._type.clone().into()),
            HkpAngMotorConstraintAtomVisitor::IsEnabled(data.is_enabled.into()),
            HkpAngMotorConstraintAtomVisitor::MotorAxis(data.motor_axis.into()),
            HkpAngMotorConstraintAtomVisitor::InitializedOffset(data.initialized_offset.into()),
            HkpAngMotorConstraintAtomVisitor::PreviousTargetAngleOffset(data.previous_target_angle_offset.into()),
            HkpAngMotorConstraintAtomVisitor::CorrespondingAngLimitSolverResultOffset(data.corresponding_ang_limit_solver_result_offset.into()),
            HkpAngMotorConstraintAtomVisitor::TargetAngle(data.target_angle.into()),
            HkpAngMotorConstraintAtomVisitor::Motor(data.motor.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpAngMotorConstraintAtom<'de> {
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
enum HkpAngMotorConstraintAtomVisitor<'a> {
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
    #[serde(rename = "previousTargetAngleOffset")]
    PreviousTargetAngleOffset(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "correspondingAngLimitSolverResultOffset")]
    CorrespondingAngLimitSolverResultOffset(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "targetAngle")]
    TargetAngle(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "motor")]
    Motor(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpAngMotorConstraintAtomVisitor<'de>, "@name",
    ("type" => Type(Primitive<AtomType>)),
    ("isEnabled" => IsEnabled(Primitive<bool>)),
    ("motorAxis" => MotorAxis(Primitive<u8>)),
    ("initializedOffset" => InitializedOffset(Primitive<i16>)),
    ("previousTargetAngleOffset" => PreviousTargetAngleOffset(Primitive<i16>)),
    ("correspondingAngLimitSolverResultOffset" => CorrespondingAngLimitSolverResultOffset(Primitive<i16>)),
    ("targetAngle" => TargetAngle(Primitive<f32>)),
    ("motor" => Motor(Primitive<Cow<'de, str>>)),
}
