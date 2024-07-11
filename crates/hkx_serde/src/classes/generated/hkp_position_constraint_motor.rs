//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpPositionConstraintMotor`
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

/// `hkpPositionConstraintMotor`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 36
/// -    vtable: true
/// -    parent: `hkpLimitedForceConstraintMotor`/`0x3377b0b0`
/// - signature: `0x748fb303`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpPositionConstraintMotor {
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
    /// -   name:`"damping"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub damping: f32,
    /// # C++ Class Fields Info
    /// -   name:`"proportionalRecoveryVelocity"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub proportional_recovery_velocity: f32,
    /// # C++ Class Fields Info
    /// -   name:`"constantRecoveryVelocity"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub constant_recovery_velocity: f32,
}

impl Serialize for HkpPositionConstraintMotor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpPositionConstraintMotorVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpPositionConstraintMotor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpPositionConstraintMotorVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpPositionConstraintMotorVisitor>> for HkpPositionConstraintMotor {
    fn from(_values: Vec<HkpPositionConstraintMotorVisitor>) -> Self {
            let mut min_force = None;
            let mut max_force = None;
            let mut _type = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut tau = None;
            let mut damping = None;
            let mut proportional_recovery_velocity = None;
            let mut constant_recovery_velocity = None;


        for _value in _values {
            match _value {
                HkpPositionConstraintMotorVisitor::MinForce(m) => min_force = Some(m),
                HkpPositionConstraintMotorVisitor::MaxForce(m) => max_force = Some(m),
                HkpPositionConstraintMotorVisitor::Type(m) => _type = Some(m),
                HkpPositionConstraintMotorVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpPositionConstraintMotorVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpPositionConstraintMotorVisitor::Tau(m) => tau = Some(m),
                HkpPositionConstraintMotorVisitor::Damping(m) => damping = Some(m),
                HkpPositionConstraintMotorVisitor::ProportionalRecoveryVelocity(m) => proportional_recovery_velocity = Some(m),
                HkpPositionConstraintMotorVisitor::ConstantRecoveryVelocity(m) => constant_recovery_velocity = Some(m),

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
            damping: damping.unwrap_or_default().into_inner(),
            proportional_recovery_velocity: proportional_recovery_velocity.unwrap_or_default().into_inner(),
            constant_recovery_velocity: constant_recovery_velocity.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpPositionConstraintMotor> for Vec<HkpPositionConstraintMotorVisitor> {
    fn from(data: &HkpPositionConstraintMotor) -> Self {
        vec![
            HkpPositionConstraintMotorVisitor::MinForce(data.min_force.into()),
            HkpPositionConstraintMotorVisitor::MaxForce(data.max_force.into()),
            HkpPositionConstraintMotorVisitor::Type(data._type.clone().into()),
            HkpPositionConstraintMotorVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpPositionConstraintMotorVisitor::ReferenceCount(data.reference_count.into()),
            HkpPositionConstraintMotorVisitor::Tau(data.tau.into()),
            HkpPositionConstraintMotorVisitor::Damping(data.damping.into()),
            HkpPositionConstraintMotorVisitor::ProportionalRecoveryVelocity(data.proportional_recovery_velocity.into()),
            HkpPositionConstraintMotorVisitor::ConstantRecoveryVelocity(data.constant_recovery_velocity.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpPositionConstraintMotor {
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
enum HkpPositionConstraintMotorVisitor {
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
    #[serde(rename = "damping")]
    Damping(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "proportionalRecoveryVelocity")]
    ProportionalRecoveryVelocity(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "constantRecoveryVelocity")]
    ConstantRecoveryVelocity(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPositionConstraintMotorVisitor, "@name",
    ("minForce" => MinForce(Primitive<f32>)),
    ("maxForce" => MaxForce(Primitive<f32>)),
    ("type" => Type(Primitive<MotorType>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("tau" => Tau(Primitive<f32>)),
    ("damping" => Damping(Primitive<f32>)),
    ("proportionalRecoveryVelocity" => ProportionalRecoveryVelocity(Primitive<f32>)),
    ("constantRecoveryVelocity" => ConstantRecoveryVelocity(Primitive<f32>)),
}
