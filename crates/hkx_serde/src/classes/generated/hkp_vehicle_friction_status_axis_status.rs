//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleFrictionStatusAxisStatus`
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

/// `hkpVehicleFrictionStatusAxisStatus`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 36
/// -    vtable: false
/// - signature: `0xe70e2bb4`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpVehicleFrictionStatusAxisStatus {
    /// # C++ Class Fields Info
    /// -   name:`"forward_slip_velocity"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub forward_slip_velocity: f32,
    /// # C++ Class Fields Info
    /// -   name:`"side_slip_velocity"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub side_slip_velocity: f32,
    /// # C++ Class Fields Info
    /// -   name:`"skid_energy_density"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub skid_energy_density: f32,
    /// # C++ Class Fields Info
    /// -   name:`"side_force"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub side_force: f32,
    /// # C++ Class Fields Info
    /// -   name:`"delayed_forward_impulse"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub delayed_forward_impulse: f32,
    /// # C++ Class Fields Info
    /// -   name:`"sideRhs"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub side_rhs: f32,
    /// # C++ Class Fields Info
    /// -   name:`"forwardRhs"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub forward_rhs: f32,
    /// # C++ Class Fields Info
    /// -   name:`"relativeSideForce"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub relative_side_force: f32,
    /// # C++ Class Fields Info
    /// -   name:`"relativeForwardForce"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub relative_forward_force: f32,
}

impl Serialize for HkpVehicleFrictionStatusAxisStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpVehicleFrictionStatusAxisStatusVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpVehicleFrictionStatusAxisStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpVehicleFrictionStatusAxisStatusVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpVehicleFrictionStatusAxisStatusVisitor>> for HkpVehicleFrictionStatusAxisStatus {
    fn from(_values: Vec<HkpVehicleFrictionStatusAxisStatusVisitor>) -> Self {
            let mut forward_slip_velocity = None;
            let mut side_slip_velocity = None;
            let mut skid_energy_density = None;
            let mut side_force = None;
            let mut delayed_forward_impulse = None;
            let mut side_rhs = None;
            let mut forward_rhs = None;
            let mut relative_side_force = None;
            let mut relative_forward_force = None;


        for _value in _values {
            match _value {
                HkpVehicleFrictionStatusAxisStatusVisitor::ForwardSlipVelocity(m) => forward_slip_velocity = Some(m),
                HkpVehicleFrictionStatusAxisStatusVisitor::SideSlipVelocity(m) => side_slip_velocity = Some(m),
                HkpVehicleFrictionStatusAxisStatusVisitor::SkidEnergyDensity(m) => skid_energy_density = Some(m),
                HkpVehicleFrictionStatusAxisStatusVisitor::SideForce(m) => side_force = Some(m),
                HkpVehicleFrictionStatusAxisStatusVisitor::DelayedForwardImpulse(m) => delayed_forward_impulse = Some(m),
                HkpVehicleFrictionStatusAxisStatusVisitor::SideRhs(m) => side_rhs = Some(m),
                HkpVehicleFrictionStatusAxisStatusVisitor::ForwardRhs(m) => forward_rhs = Some(m),
                HkpVehicleFrictionStatusAxisStatusVisitor::RelativeSideForce(m) => relative_side_force = Some(m),
                HkpVehicleFrictionStatusAxisStatusVisitor::RelativeForwardForce(m) => relative_forward_force = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            forward_slip_velocity: forward_slip_velocity.unwrap_or_default().into_inner(),
            side_slip_velocity: side_slip_velocity.unwrap_or_default().into_inner(),
            skid_energy_density: skid_energy_density.unwrap_or_default().into_inner(),
            side_force: side_force.unwrap_or_default().into_inner(),
            delayed_forward_impulse: delayed_forward_impulse.unwrap_or_default().into_inner(),
            side_rhs: side_rhs.unwrap_or_default().into_inner(),
            forward_rhs: forward_rhs.unwrap_or_default().into_inner(),
            relative_side_force: relative_side_force.unwrap_or_default().into_inner(),
            relative_forward_force: relative_forward_force.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpVehicleFrictionStatusAxisStatus> for Vec<HkpVehicleFrictionStatusAxisStatusVisitor> {
    fn from(data: &HkpVehicleFrictionStatusAxisStatus) -> Self {
        vec![
            HkpVehicleFrictionStatusAxisStatusVisitor::ForwardSlipVelocity(data.forward_slip_velocity.into()),
            HkpVehicleFrictionStatusAxisStatusVisitor::SideSlipVelocity(data.side_slip_velocity.into()),
            HkpVehicleFrictionStatusAxisStatusVisitor::SkidEnergyDensity(data.skid_energy_density.into()),
            HkpVehicleFrictionStatusAxisStatusVisitor::SideForce(data.side_force.into()),
            HkpVehicleFrictionStatusAxisStatusVisitor::DelayedForwardImpulse(data.delayed_forward_impulse.into()),
            HkpVehicleFrictionStatusAxisStatusVisitor::SideRhs(data.side_rhs.into()),
            HkpVehicleFrictionStatusAxisStatusVisitor::ForwardRhs(data.forward_rhs.into()),
            HkpVehicleFrictionStatusAxisStatusVisitor::RelativeSideForce(data.relative_side_force.into()),
            HkpVehicleFrictionStatusAxisStatusVisitor::RelativeForwardForce(data.relative_forward_force.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpVehicleFrictionStatusAxisStatus {
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
enum HkpVehicleFrictionStatusAxisStatusVisitor {
    /// Visitor fields
    #[serde(rename = "forward_slip_velocity")]
    ForwardSlipVelocity(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "side_slip_velocity")]
    SideSlipVelocity(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "skid_energy_density")]
    SkidEnergyDensity(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "side_force")]
    SideForce(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "delayed_forward_impulse")]
    DelayedForwardImpulse(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "sideRhs")]
    SideRhs(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "forwardRhs")]
    ForwardRhs(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "relativeSideForce")]
    RelativeSideForce(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "relativeForwardForce")]
    RelativeForwardForce(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleFrictionStatusAxisStatusVisitor, "@name",
    ("forward_slip_velocity" => ForwardSlipVelocity(Primitive<f32>)),
    ("side_slip_velocity" => SideSlipVelocity(Primitive<f32>)),
    ("skid_energy_density" => SkidEnergyDensity(Primitive<f32>)),
    ("side_force" => SideForce(Primitive<f32>)),
    ("delayed_forward_impulse" => DelayedForwardImpulse(Primitive<f32>)),
    ("sideRhs" => SideRhs(Primitive<f32>)),
    ("forwardRhs" => ForwardRhs(Primitive<f32>)),
    ("relativeSideForce" => RelativeSideForce(Primitive<f32>)),
    ("relativeForwardForce" => RelativeForwardForce(Primitive<f32>)),
}
