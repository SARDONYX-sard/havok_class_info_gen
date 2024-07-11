//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleData`
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

/// `hkpVehicleData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 416
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x173feb43`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpVehicleData {
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
    /// -   name:`"numWheels"`
    /// -   type: `hkInt8`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub num_wheels: i8,
    /// # C++ Class Fields Info
    /// -   name:`"chassisOrientation"`
    /// -   type: `hkRotation`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub chassis_orientation: Rotation<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"torqueRollFactor"`
    /// -   type: `hkReal`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    pub torque_roll_factor: f32,
    /// # C++ Class Fields Info
    /// -   name:`"torquePitchFactor"`
    /// -   type: `hkReal`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    pub torque_pitch_factor: f32,
    /// # C++ Class Fields Info
    /// -   name:`"torqueYawFactor"`
    /// -   type: `hkReal`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    pub torque_yaw_factor: f32,
    /// # C++ Class Fields Info
    /// -   name:`"extraTorqueFactor"`
    /// -   type: `hkReal`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    pub extra_torque_factor: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxVelocityForPositionalFriction"`
    /// -   type: `hkReal`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    pub max_velocity_for_positional_friction: f32,
    /// # C++ Class Fields Info
    /// -   name:`"chassisUnitInertiaYaw"`
    /// -   type: `hkReal`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    pub chassis_unit_inertia_yaw: f32,
    /// # C++ Class Fields Info
    /// -   name:`"chassisUnitInertiaRoll"`
    /// -   type: `hkReal`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    pub chassis_unit_inertia_roll: f32,
    /// # C++ Class Fields Info
    /// -   name:`"chassisUnitInertiaPitch"`
    /// -   type: `hkReal`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    pub chassis_unit_inertia_pitch: f32,
    /// # C++ Class Fields Info
    /// -   name:`"frictionEqualizer"`
    /// -   type: `hkReal`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    pub friction_equalizer: f32,
    /// # C++ Class Fields Info
    /// -   name:`"normalClippingAngleCos"`
    /// -   type: `hkReal`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE`
    pub normal_clipping_angle_cos: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxFrictionSolverMassRatio"`
    /// -   type: `hkReal`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE`
    pub max_friction_solver_mass_ratio: f32,
    /// # C++ Class Fields Info
    /// -   name:`"wheelParams"`
    /// -   type: `hkArray<struct hkpVehicleDataWheelComponentParams>`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE`
    pub wheel_params: HkArrayClass<HkpVehicleDataWheelComponentParams>,
    /// # C++ Class Fields Info
    /// -   name:`"numWheelsPerAxle"`
    /// -   type: `hkArray<hkInt8>`
    /// - offset: 152
    /// -  flags: `FLAGS_NONE`
    pub num_wheels_per_axle: HkArrayNum<i8>,
    /// # C++ Class Fields Info
    /// -   name:`"frictionDescription"`
    /// -   type: `struct hkpVehicleFrictionDescription`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE`
    pub friction_description: SingleClass<HkpVehicleFrictionDescription>,
    /// # C++ Class Fields Info
    /// -   name:`"chassisFrictionInertiaInvDiag"`
    /// -   type: `hkVector4`
    /// - offset: 384
    /// -  flags: `FLAGS_NONE`
    pub chassis_friction_inertia_inv_diag: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"alreadyInitialised"`
    /// -   type: `hkBool`
    /// - offset: 400
    /// -  flags: `FLAGS_NONE`
    pub already_initialised: bool,
}

impl Serialize for HkpVehicleData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpVehicleDataVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpVehicleData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpVehicleDataVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpVehicleDataVisitor>> for HkpVehicleData {
    fn from(_values: Vec<HkpVehicleDataVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut gravity = None;
            let mut num_wheels = None;
            let mut chassis_orientation = None;
            let mut torque_roll_factor = None;
            let mut torque_pitch_factor = None;
            let mut torque_yaw_factor = None;
            let mut extra_torque_factor = None;
            let mut max_velocity_for_positional_friction = None;
            let mut chassis_unit_inertia_yaw = None;
            let mut chassis_unit_inertia_roll = None;
            let mut chassis_unit_inertia_pitch = None;
            let mut friction_equalizer = None;
            let mut normal_clipping_angle_cos = None;
            let mut max_friction_solver_mass_ratio = None;
            let mut wheel_params = None;
            let mut num_wheels_per_axle = None;
            let mut friction_description = None;
            let mut chassis_friction_inertia_inv_diag = None;
            let mut already_initialised = None;


        for _value in _values {
            match _value {
                HkpVehicleDataVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpVehicleDataVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpVehicleDataVisitor::Gravity(m) => gravity = Some(m),
                HkpVehicleDataVisitor::NumWheels(m) => num_wheels = Some(m),
                HkpVehicleDataVisitor::ChassisOrientation(m) => chassis_orientation = Some(m),
                HkpVehicleDataVisitor::TorqueRollFactor(m) => torque_roll_factor = Some(m),
                HkpVehicleDataVisitor::TorquePitchFactor(m) => torque_pitch_factor = Some(m),
                HkpVehicleDataVisitor::TorqueYawFactor(m) => torque_yaw_factor = Some(m),
                HkpVehicleDataVisitor::ExtraTorqueFactor(m) => extra_torque_factor = Some(m),
                HkpVehicleDataVisitor::MaxVelocityForPositionalFriction(m) => max_velocity_for_positional_friction = Some(m),
                HkpVehicleDataVisitor::ChassisUnitInertiaYaw(m) => chassis_unit_inertia_yaw = Some(m),
                HkpVehicleDataVisitor::ChassisUnitInertiaRoll(m) => chassis_unit_inertia_roll = Some(m),
                HkpVehicleDataVisitor::ChassisUnitInertiaPitch(m) => chassis_unit_inertia_pitch = Some(m),
                HkpVehicleDataVisitor::FrictionEqualizer(m) => friction_equalizer = Some(m),
                HkpVehicleDataVisitor::NormalClippingAngleCos(m) => normal_clipping_angle_cos = Some(m),
                HkpVehicleDataVisitor::MaxFrictionSolverMassRatio(m) => max_friction_solver_mass_ratio = Some(m),
                HkpVehicleDataVisitor::WheelParams(m) => wheel_params = Some(m),
                HkpVehicleDataVisitor::NumWheelsPerAxle(m) => num_wheels_per_axle = Some(m),
                HkpVehicleDataVisitor::FrictionDescription(m) => friction_description = Some(m),
                HkpVehicleDataVisitor::ChassisFrictionInertiaInvDiag(m) => chassis_friction_inertia_inv_diag = Some(m),
                HkpVehicleDataVisitor::AlreadyInitialised(m) => already_initialised = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            gravity: gravity.unwrap_or_default().into_inner(),
            num_wheels: num_wheels.unwrap_or_default().into_inner(),
            chassis_orientation: chassis_orientation.unwrap_or_default().into_inner(),
            torque_roll_factor: torque_roll_factor.unwrap_or_default().into_inner(),
            torque_pitch_factor: torque_pitch_factor.unwrap_or_default().into_inner(),
            torque_yaw_factor: torque_yaw_factor.unwrap_or_default().into_inner(),
            extra_torque_factor: extra_torque_factor.unwrap_or_default().into_inner(),
            max_velocity_for_positional_friction: max_velocity_for_positional_friction.unwrap_or_default().into_inner(),
            chassis_unit_inertia_yaw: chassis_unit_inertia_yaw.unwrap_or_default().into_inner(),
            chassis_unit_inertia_roll: chassis_unit_inertia_roll.unwrap_or_default().into_inner(),
            chassis_unit_inertia_pitch: chassis_unit_inertia_pitch.unwrap_or_default().into_inner(),
            friction_equalizer: friction_equalizer.unwrap_or_default().into_inner(),
            normal_clipping_angle_cos: normal_clipping_angle_cos.unwrap_or_default().into_inner(),
            max_friction_solver_mass_ratio: max_friction_solver_mass_ratio.unwrap_or_default().into_inner(),
            wheel_params: wheel_params.unwrap_or_default(),
            num_wheels_per_axle: num_wheels_per_axle.unwrap_or_default(),
            friction_description: friction_description.unwrap_or_default(),
            chassis_friction_inertia_inv_diag: chassis_friction_inertia_inv_diag.unwrap_or_default().into_inner(),
            already_initialised: already_initialised.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpVehicleData> for Vec<HkpVehicleDataVisitor> {
    fn from(data: &HkpVehicleData) -> Self {
        vec![
            HkpVehicleDataVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpVehicleDataVisitor::ReferenceCount(data.reference_count.into()),
            HkpVehicleDataVisitor::Gravity(data.gravity.into()),
            HkpVehicleDataVisitor::NumWheels(data.num_wheels.into()),
            HkpVehicleDataVisitor::ChassisOrientation(data.chassis_orientation.clone().into()),
            HkpVehicleDataVisitor::TorqueRollFactor(data.torque_roll_factor.into()),
            HkpVehicleDataVisitor::TorquePitchFactor(data.torque_pitch_factor.into()),
            HkpVehicleDataVisitor::TorqueYawFactor(data.torque_yaw_factor.into()),
            HkpVehicleDataVisitor::ExtraTorqueFactor(data.extra_torque_factor.into()),
            HkpVehicleDataVisitor::MaxVelocityForPositionalFriction(data.max_velocity_for_positional_friction.into()),
            HkpVehicleDataVisitor::ChassisUnitInertiaYaw(data.chassis_unit_inertia_yaw.into()),
            HkpVehicleDataVisitor::ChassisUnitInertiaRoll(data.chassis_unit_inertia_roll.into()),
            HkpVehicleDataVisitor::ChassisUnitInertiaPitch(data.chassis_unit_inertia_pitch.into()),
            HkpVehicleDataVisitor::FrictionEqualizer(data.friction_equalizer.into()),
            HkpVehicleDataVisitor::NormalClippingAngleCos(data.normal_clipping_angle_cos.into()),
            HkpVehicleDataVisitor::MaxFrictionSolverMassRatio(data.max_friction_solver_mass_ratio.into()),
            HkpVehicleDataVisitor::WheelParams(data.wheel_params.clone()),
            HkpVehicleDataVisitor::NumWheelsPerAxle(data.num_wheels_per_axle.clone()),
            HkpVehicleDataVisitor::FrictionDescription(data.friction_description.clone()),
            HkpVehicleDataVisitor::ChassisFrictionInertiaInvDiag(data.chassis_friction_inertia_inv_diag.into()),
            HkpVehicleDataVisitor::AlreadyInitialised(data.already_initialised.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpVehicleData {
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
enum HkpVehicleDataVisitor {
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
    #[serde(rename = "numWheels")]
    NumWheels(Primitive<i8>),
    /// Visitor fields
    #[serde(rename = "chassisOrientation")]
    ChassisOrientation(Primitive<Rotation<f32>>),
    /// Visitor fields
    #[serde(rename = "torqueRollFactor")]
    TorqueRollFactor(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "torquePitchFactor")]
    TorquePitchFactor(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "torqueYawFactor")]
    TorqueYawFactor(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "extraTorqueFactor")]
    ExtraTorqueFactor(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxVelocityForPositionalFriction")]
    MaxVelocityForPositionalFriction(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "chassisUnitInertiaYaw")]
    ChassisUnitInertiaYaw(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "chassisUnitInertiaRoll")]
    ChassisUnitInertiaRoll(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "chassisUnitInertiaPitch")]
    ChassisUnitInertiaPitch(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "frictionEqualizer")]
    FrictionEqualizer(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "normalClippingAngleCos")]
    NormalClippingAngleCos(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxFrictionSolverMassRatio")]
    MaxFrictionSolverMassRatio(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "wheelParams")]
    WheelParams(HkArrayClass<HkpVehicleDataWheelComponentParams>),
    /// Visitor fields
    #[serde(rename = "numWheelsPerAxle")]
    NumWheelsPerAxle(HkArrayNum<i8>),
    /// Visitor fields
    #[serde(rename = "frictionDescription")]
    FrictionDescription(SingleClass<HkpVehicleFrictionDescription>),
    /// Visitor fields
    #[serde(rename = "chassisFrictionInertiaInvDiag")]
    ChassisFrictionInertiaInvDiag(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "alreadyInitialised")]
    AlreadyInitialised(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDataVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("gravity" => Gravity(Primitive<Vector4<f32>>)),
    ("numWheels" => NumWheels(Primitive<i8>)),
    ("chassisOrientation" => ChassisOrientation(Primitive<Rotation<f32>>)),
    ("torqueRollFactor" => TorqueRollFactor(Primitive<f32>)),
    ("torquePitchFactor" => TorquePitchFactor(Primitive<f32>)),
    ("torqueYawFactor" => TorqueYawFactor(Primitive<f32>)),
    ("extraTorqueFactor" => ExtraTorqueFactor(Primitive<f32>)),
    ("maxVelocityForPositionalFriction" => MaxVelocityForPositionalFriction(Primitive<f32>)),
    ("chassisUnitInertiaYaw" => ChassisUnitInertiaYaw(Primitive<f32>)),
    ("chassisUnitInertiaRoll" => ChassisUnitInertiaRoll(Primitive<f32>)),
    ("chassisUnitInertiaPitch" => ChassisUnitInertiaPitch(Primitive<f32>)),
    ("frictionEqualizer" => FrictionEqualizer(Primitive<f32>)),
    ("normalClippingAngleCos" => NormalClippingAngleCos(Primitive<f32>)),
    ("maxFrictionSolverMassRatio" => MaxFrictionSolverMassRatio(Primitive<f32>)),
    ("wheelParams" => WheelParams(HkArrayClass<HkpVehicleDataWheelComponentParams>)),
    ("numWheelsPerAxle" => NumWheelsPerAxle(HkArrayNum<i8>)),
    ("frictionDescription" => FrictionDescription(SingleClass<HkpVehicleFrictionDescription>)),
    ("chassisFrictionInertiaInvDiag" => ChassisFrictionInertiaInvDiag(Primitive<Vector4<f32>>)),
    ("alreadyInitialised" => AlreadyInitialised(Primitive<bool>)),
}
