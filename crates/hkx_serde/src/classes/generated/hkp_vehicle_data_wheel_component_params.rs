//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleDataWheelComponentParams`
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

/// `hkpVehicleDataWheelComponentParams`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 40
/// -    vtable: false
/// - signature: `0x82fe40e0`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpVehicleDataWheelComponentParams {
    /// # C++ Class Fields Info
    /// -   name:`"radius"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub radius: f32,
    /// # C++ Class Fields Info
    /// -   name:`"mass"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub mass: f32,
    /// # C++ Class Fields Info
    /// -   name:`"width"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub width: f32,
    /// # C++ Class Fields Info
    /// -   name:`"friction"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub friction: f32,
    /// # C++ Class Fields Info
    /// -   name:`"viscosityFriction"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub viscosity_friction: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxFriction"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub max_friction: f32,
    /// # C++ Class Fields Info
    /// -   name:`"slipAngle"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub slip_angle: f32,
    /// # C++ Class Fields Info
    /// -   name:`"forceFeedbackMultiplier"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub force_feedback_multiplier: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxContactBodyAcceleration"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub max_contact_body_acceleration: f32,
    /// # C++ Class Fields Info
    /// -   name:`"axle"`
    /// -   type: `hkInt8`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub axle: i8,
}

impl Serialize for HkpVehicleDataWheelComponentParams {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpVehicleDataWheelComponentParamsVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpVehicleDataWheelComponentParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpVehicleDataWheelComponentParamsVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpVehicleDataWheelComponentParamsVisitor>> for HkpVehicleDataWheelComponentParams {
    fn from(_values: Vec<HkpVehicleDataWheelComponentParamsVisitor>) -> Self {
            let mut radius = None;
            let mut mass = None;
            let mut width = None;
            let mut friction = None;
            let mut viscosity_friction = None;
            let mut max_friction = None;
            let mut slip_angle = None;
            let mut force_feedback_multiplier = None;
            let mut max_contact_body_acceleration = None;
            let mut axle = None;


        for _value in _values {
            match _value {
                HkpVehicleDataWheelComponentParamsVisitor::Radius(m) => radius = Some(m),
                HkpVehicleDataWheelComponentParamsVisitor::Mass(m) => mass = Some(m),
                HkpVehicleDataWheelComponentParamsVisitor::Width(m) => width = Some(m),
                HkpVehicleDataWheelComponentParamsVisitor::Friction(m) => friction = Some(m),
                HkpVehicleDataWheelComponentParamsVisitor::ViscosityFriction(m) => viscosity_friction = Some(m),
                HkpVehicleDataWheelComponentParamsVisitor::MaxFriction(m) => max_friction = Some(m),
                HkpVehicleDataWheelComponentParamsVisitor::SlipAngle(m) => slip_angle = Some(m),
                HkpVehicleDataWheelComponentParamsVisitor::ForceFeedbackMultiplier(m) => force_feedback_multiplier = Some(m),
                HkpVehicleDataWheelComponentParamsVisitor::MaxContactBodyAcceleration(m) => max_contact_body_acceleration = Some(m),
                HkpVehicleDataWheelComponentParamsVisitor::Axle(m) => axle = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            radius: radius.unwrap_or_default().into_inner(),
            mass: mass.unwrap_or_default().into_inner(),
            width: width.unwrap_or_default().into_inner(),
            friction: friction.unwrap_or_default().into_inner(),
            viscosity_friction: viscosity_friction.unwrap_or_default().into_inner(),
            max_friction: max_friction.unwrap_or_default().into_inner(),
            slip_angle: slip_angle.unwrap_or_default().into_inner(),
            force_feedback_multiplier: force_feedback_multiplier.unwrap_or_default().into_inner(),
            max_contact_body_acceleration: max_contact_body_acceleration.unwrap_or_default().into_inner(),
            axle: axle.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpVehicleDataWheelComponentParams> for Vec<HkpVehicleDataWheelComponentParamsVisitor> {
    fn from(data: &HkpVehicleDataWheelComponentParams) -> Self {
        vec![
            HkpVehicleDataWheelComponentParamsVisitor::Radius(data.radius.into()),
            HkpVehicleDataWheelComponentParamsVisitor::Mass(data.mass.into()),
            HkpVehicleDataWheelComponentParamsVisitor::Width(data.width.into()),
            HkpVehicleDataWheelComponentParamsVisitor::Friction(data.friction.into()),
            HkpVehicleDataWheelComponentParamsVisitor::ViscosityFriction(data.viscosity_friction.into()),
            HkpVehicleDataWheelComponentParamsVisitor::MaxFriction(data.max_friction.into()),
            HkpVehicleDataWheelComponentParamsVisitor::SlipAngle(data.slip_angle.into()),
            HkpVehicleDataWheelComponentParamsVisitor::ForceFeedbackMultiplier(data.force_feedback_multiplier.into()),
            HkpVehicleDataWheelComponentParamsVisitor::MaxContactBodyAcceleration(data.max_contact_body_acceleration.into()),
            HkpVehicleDataWheelComponentParamsVisitor::Axle(data.axle.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpVehicleDataWheelComponentParams {
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
enum HkpVehicleDataWheelComponentParamsVisitor {
    /// Visitor fields
    #[serde(rename = "radius")]
    Radius(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "mass")]
    Mass(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "width")]
    Width(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "friction")]
    Friction(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "viscosityFriction")]
    ViscosityFriction(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxFriction")]
    MaxFriction(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "slipAngle")]
    SlipAngle(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "forceFeedbackMultiplier")]
    ForceFeedbackMultiplier(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxContactBodyAcceleration")]
    MaxContactBodyAcceleration(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "axle")]
    Axle(Primitive<i8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDataWheelComponentParamsVisitor, "@name",
    ("radius" => Radius(Primitive<f32>)),
    ("mass" => Mass(Primitive<f32>)),
    ("width" => Width(Primitive<f32>)),
    ("friction" => Friction(Primitive<f32>)),
    ("viscosityFriction" => ViscosityFriction(Primitive<f32>)),
    ("maxFriction" => MaxFriction(Primitive<f32>)),
    ("slipAngle" => SlipAngle(Primitive<f32>)),
    ("forceFeedbackMultiplier" => ForceFeedbackMultiplier(Primitive<f32>)),
    ("maxContactBodyAcceleration" => MaxContactBodyAcceleration(Primitive<f32>)),
    ("axle" => Axle(Primitive<i8>)),
}
