//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVelocityConstraintMotor`
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

/// `hkpVelocityConstraintMotor`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: true
/// -    parent: `hkpLimitedForceConstraintMotor`/`0x3377b0b0`
/// - signature: `0xfca2fcc3`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpVelocityConstraintMotor {
    /// # C++ Parent class(`hkpLimitedForceConstraintMotor` => parent: `hkpConstraintMotor`) field Info
    /// -   name:`"minForce"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub min_force: f32,
    /// # C++ Parent class(`hkpLimitedForceConstraintMotor` => parent: `hkpConstraintMotor`) field Info
    /// -   name:`"maxForce"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub max_force: f32,

    /// # C++ Parent class(`hkpConstraintMotor` => parent: `hkReferencedObject`) field Info
    /// -   name:`"type"`
    /// -   type: `enum MotorType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub _type: MotorType,

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
    /// -   name:`"tau"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub tau: f32,
    /// # C++ Class Fields Info
    /// -   name:`"velocityTarget"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub velocity_target: f32,
    /// # C++ Class Fields Info
    /// -   name:`"useVelocityTargetFromConstraintTargets"`
    /// -   type: `hkBool`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub use_velocity_target_from_constraint_targets: bool,
}

impl Serialize for HkpVelocityConstraintMotor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpVelocityConstraintMotorVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpVelocityConstraintMotor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpVelocityConstraintMotorVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpVelocityConstraintMotorVisitor>> for HkpVelocityConstraintMotor {
    fn from(_values: Vec<HkpVelocityConstraintMotorVisitor>) -> Self {
            let mut min_force = None;
            let mut max_force = None;
            let mut _type = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut tau = None;
            let mut velocity_target = None;
            let mut use_velocity_target_from_constraint_targets = None;


        for _value in _values {
            match _value {
                HkpVelocityConstraintMotorVisitor::MinForce(m) => min_force = Some(m),
                HkpVelocityConstraintMotorVisitor::MaxForce(m) => max_force = Some(m),
                HkpVelocityConstraintMotorVisitor::Type(m) => _type = Some(m),
                HkpVelocityConstraintMotorVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpVelocityConstraintMotorVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpVelocityConstraintMotorVisitor::Tau(m) => tau = Some(m),
                HkpVelocityConstraintMotorVisitor::VelocityTarget(m) => velocity_target = Some(m),
                HkpVelocityConstraintMotorVisitor::UseVelocityTargetFromConstraintTargets(m) => use_velocity_target_from_constraint_targets = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            min_force: min_force.unwrap_or_default().into_inner(),
            max_force: max_force.unwrap_or_default().into_inner(),
            _type: _type.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            tau: tau.unwrap_or_default().into_inner(),
            velocity_target: velocity_target.unwrap_or_default().into_inner(),
            use_velocity_target_from_constraint_targets: use_velocity_target_from_constraint_targets.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpVelocityConstraintMotor> for Vec<HkpVelocityConstraintMotorVisitor> {
    fn from(data: &HkpVelocityConstraintMotor) -> Self {
        vec![
            HkpVelocityConstraintMotorVisitor::MinForce(data.min_force.into()),
            HkpVelocityConstraintMotorVisitor::MaxForce(data.max_force.into()),
            HkpVelocityConstraintMotorVisitor::Type(data._type.clone().into()),
            HkpVelocityConstraintMotorVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpVelocityConstraintMotorVisitor::ReferenceCount(data.reference_count.into()),
            HkpVelocityConstraintMotorVisitor::Tau(data.tau.into()),
            HkpVelocityConstraintMotorVisitor::VelocityTarget(data.velocity_target.into()),
            HkpVelocityConstraintMotorVisitor::UseVelocityTargetFromConstraintTargets(data.use_velocity_target_from_constraint_targets.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpVelocityConstraintMotor {
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
enum HkpVelocityConstraintMotorVisitor {
    /// Visitor fields
    #[serde(rename = "minForce")]
    MinForce(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxForce")]
    MaxForce(Primitive<f32>),

    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<MotorType>),

    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "tau")]
    Tau(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "velocityTarget")]
    VelocityTarget(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "useVelocityTargetFromConstraintTargets")]
    UseVelocityTargetFromConstraintTargets(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVelocityConstraintMotorVisitor, "@name",
    ("minForce" => MinForce(Primitive<f32>)),
    ("maxForce" => MaxForce(Primitive<f32>)),
    ("type" => Type(Primitive<MotorType>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("tau" => Tau(Primitive<f32>)),
    ("velocityTarget" => VelocityTarget(Primitive<f32>)),
    ("useVelocityTargetFromConstraintTargets" => UseVelocityTargetFromConstraintTargets(Primitive<bool>)),
}
