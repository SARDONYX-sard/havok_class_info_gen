//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbPoweredRagdollControlData`
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

/// `hkbPoweredRagdollControlData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: false
/// - signature: `0xf5ba21b`
/// -   version: 3
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbPoweredRagdollControlData {
    /// # C++ Class Fields Info
    /// -   name:`"maxForce"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|ALIGN16`
    pub max_force: f32,
    /// # C++ Class Fields Info
    /// -   name:`"tau"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub tau: f32,
    /// # C++ Class Fields Info
    /// -   name:`"damping"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub damping: f32,
    /// # C++ Class Fields Info
    /// -   name:`"proportionalRecoveryVelocity"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub proportional_recovery_velocity: f32,
    /// # C++ Class Fields Info
    /// -   name:`"constantRecoveryVelocity"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub constant_recovery_velocity: f32,
}

impl Serialize for HkbPoweredRagdollControlData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbPoweredRagdollControlDataVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbPoweredRagdollControlData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbPoweredRagdollControlDataVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbPoweredRagdollControlDataVisitor>> for HkbPoweredRagdollControlData {
    fn from(_values: Vec<HkbPoweredRagdollControlDataVisitor>) -> Self {
            let mut max_force = None;
            let mut tau = None;
            let mut damping = None;
            let mut proportional_recovery_velocity = None;
            let mut constant_recovery_velocity = None;


        for _value in _values {
            match _value {
                HkbPoweredRagdollControlDataVisitor::MaxForce(m) => max_force = Some(m),
                HkbPoweredRagdollControlDataVisitor::Tau(m) => tau = Some(m),
                HkbPoweredRagdollControlDataVisitor::Damping(m) => damping = Some(m),
                HkbPoweredRagdollControlDataVisitor::ProportionalRecoveryVelocity(m) => proportional_recovery_velocity = Some(m),
                HkbPoweredRagdollControlDataVisitor::ConstantRecoveryVelocity(m) => constant_recovery_velocity = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            max_force: max_force.unwrap_or_default().into_inner(),
            tau: tau.unwrap_or_default().into_inner(),
            damping: damping.unwrap_or_default().into_inner(),
            proportional_recovery_velocity: proportional_recovery_velocity.unwrap_or_default().into_inner(),
            constant_recovery_velocity: constant_recovery_velocity.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbPoweredRagdollControlData> for Vec<HkbPoweredRagdollControlDataVisitor> {
    fn from(data: &HkbPoweredRagdollControlData) -> Self {
        vec![
            HkbPoweredRagdollControlDataVisitor::MaxForce(data.max_force.into()),
            HkbPoweredRagdollControlDataVisitor::Tau(data.tau.into()),
            HkbPoweredRagdollControlDataVisitor::Damping(data.damping.into()),
            HkbPoweredRagdollControlDataVisitor::ProportionalRecoveryVelocity(data.proportional_recovery_velocity.into()),
            HkbPoweredRagdollControlDataVisitor::ConstantRecoveryVelocity(data.constant_recovery_velocity.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbPoweredRagdollControlData {
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
enum HkbPoweredRagdollControlDataVisitor {
    /// Visitor fields
    #[serde(rename = "maxForce")]
    MaxForce(Primitive<f32>),
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
    HkbPoweredRagdollControlDataVisitor, "@name",
    ("maxForce" => MaxForce(Primitive<f32>)),
    ("tau" => Tau(Primitive<f32>)),
    ("damping" => Damping(Primitive<f32>)),
    ("proportionalRecoveryVelocity" => ProportionalRecoveryVelocity(Primitive<f32>)),
    ("constantRecoveryVelocity" => ConstantRecoveryVelocity(Primitive<f32>)),
}
