//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleInstance`
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

/// `hkpVehicleInstance`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 212
/// -    vtable: true
/// -    parent: `hkpUnaryAction`/`0x895532c0`
/// - signature: `0x877bb579`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpVehicleInstance<'a> {
    /// # C++ Parent class(`hkpUnaryAction` => parent: `hkpAction`) field Info
    /// -   name:`"entity"`
    /// -   type: `struct hkpEntity*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub entity: Cow<'a, str>,

    /// # C++ Parent class(`hkpAction` => parent: `hkReferencedObject`) field Info
    /// -   name:`"world"`
    /// -   type: `void*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub world: Cow<'a, str>,
    /// # C++ Parent class(`hkpAction` => parent: `hkReferencedObject`) field Info
    /// -   name:`"island"`
    /// -   type: `void*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub island: Cow<'a, str>,
    /// # C++ Parent class(`hkpAction` => parent: `hkReferencedObject`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub user_data: usize,
    /// # C++ Parent class(`hkpAction` => parent: `hkReferencedObject`) field Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub name: Cow<'a, str>,

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
    /// -   name:`"data"`
    /// -   type: `struct hkpVehicleData*`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub data: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"driverInput"`
    /// -   type: `struct hkpVehicleDriverInput*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub driver_input: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"steering"`
    /// -   type: `struct hkpVehicleSteering*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub steering: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"engine"`
    /// -   type: `struct hkpVehicleEngine*`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub engine: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"transmission"`
    /// -   type: `struct hkpVehicleTransmission*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub transmission: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"brake"`
    /// -   type: `struct hkpVehicleBrake*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub brake: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"suspension"`
    /// -   type: `struct hkpVehicleSuspension*`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub suspension: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"aerodynamics"`
    /// -   type: `struct hkpVehicleAerodynamics*`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub aerodynamics: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"wheelCollide"`
    /// -   type: `struct hkpVehicleWheelCollide*`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub wheel_collide: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"tyreMarks"`
    /// -   type: `struct hkpTyremarksInfo*`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub tyre_marks: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"velocityDamper"`
    /// -   type: `struct hkpVehicleVelocityDamper*`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub velocity_damper: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"wheelsInfo"`
    /// -   type: `hkArray<struct hkpVehicleInstanceWheelInfo>`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    pub wheels_info: HkArrayClass<HkpVehicleInstanceWheelInfo<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"frictionStatus"`
    /// -   type: `struct hkpVehicleFrictionStatus`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    pub friction_status: SingleClass<HkpVehicleFrictionStatus>,
    /// # C++ Class Fields Info
    /// -   name:`"deviceStatus"`
    /// -   type: `struct hkpVehicleDriverInputStatus*`
    /// - offset: 156
    /// -  flags: `FLAGS_NONE`
    pub device_status: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"isFixed"`
    /// -   type: `hkArray<hkBool>`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    pub is_fixed: HkArrayRef<bool>,
    /// # C++ Class Fields Info
    /// -   name:`"wheelsTimeSinceMaxPedalInput"`
    /// -   type: `hkReal`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE`
    pub wheels_time_since_max_pedal_input: f32,
    /// # C++ Class Fields Info
    /// -   name:`"tryingToReverse"`
    /// -   type: `hkBool`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    pub trying_to_reverse: bool,
    /// # C++ Class Fields Info
    /// -   name:`"torque"`
    /// -   type: `hkReal`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE`
    pub torque: f32,
    /// # C++ Class Fields Info
    /// -   name:`"rpm"`
    /// -   type: `hkReal`
    /// - offset: 184
    /// -  flags: `FLAGS_NONE`
    pub rpm: f32,
    /// # C++ Class Fields Info
    /// -   name:`"mainSteeringAngle"`
    /// -   type: `hkReal`
    /// - offset: 188
    /// -  flags: `FLAGS_NONE`
    pub main_steering_angle: f32,
    /// # C++ Class Fields Info
    /// -   name:`"wheelsSteeringAngle"`
    /// -   type: `hkArray<hkReal>`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE`
    pub wheels_steering_angle: HkArrayNum<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"isReversing"`
    /// -   type: `hkBool`
    /// - offset: 204
    /// -  flags: `FLAGS_NONE`
    pub is_reversing: bool,
    /// # C++ Class Fields Info
    /// -   name:`"currentGear"`
    /// -   type: `hkInt8`
    /// - offset: 205
    /// -  flags: `FLAGS_NONE`
    pub current_gear: i8,
    /// # C++ Class Fields Info
    /// -   name:`"delayed"`
    /// -   type: `hkBool`
    /// - offset: 206
    /// -  flags: `FLAGS_NONE`
    pub delayed: bool,
    /// # C++ Class Fields Info
    /// -   name:`"clutchDelayCountdown"`
    /// -   type: `hkReal`
    /// - offset: 208
    /// -  flags: `FLAGS_NONE`
    pub clutch_delay_countdown: f32,
}

impl Serialize for HkpVehicleInstance<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpVehicleInstanceVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpVehicleInstance<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpVehicleInstanceVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpVehicleInstanceVisitor<'a>>> for HkpVehicleInstance<'a> {
    fn from(_values: Vec<HkpVehicleInstanceVisitor<'a>>) -> Self {
            let mut entity = None;
            let mut world = None;
            let mut island = None;
            let mut user_data = None;
            let mut name = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut data = None;
            let mut driver_input = None;
            let mut steering = None;
            let mut engine = None;
            let mut transmission = None;
            let mut brake = None;
            let mut suspension = None;
            let mut aerodynamics = None;
            let mut wheel_collide = None;
            let mut tyre_marks = None;
            let mut velocity_damper = None;
            let mut wheels_info = None;
            let mut friction_status = None;
            let mut device_status = None;
            let mut is_fixed = None;
            let mut wheels_time_since_max_pedal_input = None;
            let mut trying_to_reverse = None;
            let mut torque = None;
            let mut rpm = None;
            let mut main_steering_angle = None;
            let mut wheels_steering_angle = None;
            let mut is_reversing = None;
            let mut current_gear = None;
            let mut delayed = None;
            let mut clutch_delay_countdown = None;


        for _value in _values {
            match _value {
                HkpVehicleInstanceVisitor::Entity(m) => entity = Some(m),
                HkpVehicleInstanceVisitor::World(m) => world = Some(m),
                HkpVehicleInstanceVisitor::Island(m) => island = Some(m),
                HkpVehicleInstanceVisitor::UserData(m) => user_data = Some(m),
                HkpVehicleInstanceVisitor::Name(m) => name = Some(m),
                HkpVehicleInstanceVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpVehicleInstanceVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpVehicleInstanceVisitor::Data(m) => data = Some(m),
                HkpVehicleInstanceVisitor::DriverInput(m) => driver_input = Some(m),
                HkpVehicleInstanceVisitor::Steering(m) => steering = Some(m),
                HkpVehicleInstanceVisitor::Engine(m) => engine = Some(m),
                HkpVehicleInstanceVisitor::Transmission(m) => transmission = Some(m),
                HkpVehicleInstanceVisitor::Brake(m) => brake = Some(m),
                HkpVehicleInstanceVisitor::Suspension(m) => suspension = Some(m),
                HkpVehicleInstanceVisitor::Aerodynamics(m) => aerodynamics = Some(m),
                HkpVehicleInstanceVisitor::WheelCollide(m) => wheel_collide = Some(m),
                HkpVehicleInstanceVisitor::TyreMarks(m) => tyre_marks = Some(m),
                HkpVehicleInstanceVisitor::VelocityDamper(m) => velocity_damper = Some(m),
                HkpVehicleInstanceVisitor::WheelsInfo(m) => wheels_info = Some(m),
                HkpVehicleInstanceVisitor::FrictionStatus(m) => friction_status = Some(m),
                HkpVehicleInstanceVisitor::DeviceStatus(m) => device_status = Some(m),
                HkpVehicleInstanceVisitor::IsFixed(m) => is_fixed = Some(m),
                HkpVehicleInstanceVisitor::WheelsTimeSinceMaxPedalInput(m) => wheels_time_since_max_pedal_input = Some(m),
                HkpVehicleInstanceVisitor::TryingToReverse(m) => trying_to_reverse = Some(m),
                HkpVehicleInstanceVisitor::Torque(m) => torque = Some(m),
                HkpVehicleInstanceVisitor::Rpm(m) => rpm = Some(m),
                HkpVehicleInstanceVisitor::MainSteeringAngle(m) => main_steering_angle = Some(m),
                HkpVehicleInstanceVisitor::WheelsSteeringAngle(m) => wheels_steering_angle = Some(m),
                HkpVehicleInstanceVisitor::IsReversing(m) => is_reversing = Some(m),
                HkpVehicleInstanceVisitor::CurrentGear(m) => current_gear = Some(m),
                HkpVehicleInstanceVisitor::Delayed(m) => delayed = Some(m),
                HkpVehicleInstanceVisitor::ClutchDelayCountdown(m) => clutch_delay_countdown = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            entity: entity.unwrap_or_default().into_inner(),
            world: world.unwrap_or_default().into_inner(),
            island: island.unwrap_or_default().into_inner(),
            user_data: user_data.unwrap_or_default().into_inner(),
            name: name.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            data: data.unwrap_or_default().into_inner(),
            driver_input: driver_input.unwrap_or_default().into_inner(),
            steering: steering.unwrap_or_default().into_inner(),
            engine: engine.unwrap_or_default().into_inner(),
            transmission: transmission.unwrap_or_default().into_inner(),
            brake: brake.unwrap_or_default().into_inner(),
            suspension: suspension.unwrap_or_default().into_inner(),
            aerodynamics: aerodynamics.unwrap_or_default().into_inner(),
            wheel_collide: wheel_collide.unwrap_or_default().into_inner(),
            tyre_marks: tyre_marks.unwrap_or_default().into_inner(),
            velocity_damper: velocity_damper.unwrap_or_default().into_inner(),
            wheels_info: wheels_info.unwrap_or_default(),
            friction_status: friction_status.unwrap_or_default(),
            device_status: device_status.unwrap_or_default().into_inner(),
            is_fixed: is_fixed.unwrap_or_default(),
            wheels_time_since_max_pedal_input: wheels_time_since_max_pedal_input.unwrap_or_default().into_inner(),
            trying_to_reverse: trying_to_reverse.unwrap_or_default().into_inner(),
            torque: torque.unwrap_or_default().into_inner(),
            rpm: rpm.unwrap_or_default().into_inner(),
            main_steering_angle: main_steering_angle.unwrap_or_default().into_inner(),
            wheels_steering_angle: wheels_steering_angle.unwrap_or_default(),
            is_reversing: is_reversing.unwrap_or_default().into_inner(),
            current_gear: current_gear.unwrap_or_default().into_inner(),
            delayed: delayed.unwrap_or_default().into_inner(),
            clutch_delay_countdown: clutch_delay_countdown.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpVehicleInstance<'a>> for Vec<HkpVehicleInstanceVisitor<'a>> {
    fn from(data: &HkpVehicleInstance<'a>) -> Self {
        vec![
            HkpVehicleInstanceVisitor::Entity(data.entity.clone().into()),
            HkpVehicleInstanceVisitor::World(data.world.clone().into()),
            HkpVehicleInstanceVisitor::Island(data.island.clone().into()),
            HkpVehicleInstanceVisitor::UserData(data.user_data.into()),
            HkpVehicleInstanceVisitor::Name(data.name.clone().into()),
            HkpVehicleInstanceVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpVehicleInstanceVisitor::ReferenceCount(data.reference_count.into()),
            HkpVehicleInstanceVisitor::Data(data.data.clone().into()),
            HkpVehicleInstanceVisitor::DriverInput(data.driver_input.clone().into()),
            HkpVehicleInstanceVisitor::Steering(data.steering.clone().into()),
            HkpVehicleInstanceVisitor::Engine(data.engine.clone().into()),
            HkpVehicleInstanceVisitor::Transmission(data.transmission.clone().into()),
            HkpVehicleInstanceVisitor::Brake(data.brake.clone().into()),
            HkpVehicleInstanceVisitor::Suspension(data.suspension.clone().into()),
            HkpVehicleInstanceVisitor::Aerodynamics(data.aerodynamics.clone().into()),
            HkpVehicleInstanceVisitor::WheelCollide(data.wheel_collide.clone().into()),
            HkpVehicleInstanceVisitor::TyreMarks(data.tyre_marks.clone().into()),
            HkpVehicleInstanceVisitor::VelocityDamper(data.velocity_damper.clone().into()),
            HkpVehicleInstanceVisitor::WheelsInfo(data.wheels_info.clone()),
            HkpVehicleInstanceVisitor::FrictionStatus(data.friction_status.clone()),
            HkpVehicleInstanceVisitor::DeviceStatus(data.device_status.clone().into()),
            HkpVehicleInstanceVisitor::IsFixed(data.is_fixed.clone()),
            HkpVehicleInstanceVisitor::WheelsTimeSinceMaxPedalInput(data.wheels_time_since_max_pedal_input.into()),
            HkpVehicleInstanceVisitor::TryingToReverse(data.trying_to_reverse.into()),
            HkpVehicleInstanceVisitor::Torque(data.torque.into()),
            HkpVehicleInstanceVisitor::Rpm(data.rpm.into()),
            HkpVehicleInstanceVisitor::MainSteeringAngle(data.main_steering_angle.into()),
            HkpVehicleInstanceVisitor::WheelsSteeringAngle(data.wheels_steering_angle.clone()),
            HkpVehicleInstanceVisitor::IsReversing(data.is_reversing.into()),
            HkpVehicleInstanceVisitor::CurrentGear(data.current_gear.into()),
            HkpVehicleInstanceVisitor::Delayed(data.delayed.into()),
            HkpVehicleInstanceVisitor::ClutchDelayCountdown(data.clutch_delay_countdown.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpVehicleInstance<'de> {
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
enum HkpVehicleInstanceVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "entity")]
    Entity(Primitive<Cow<'a, str>>),

    /// Visitor fields
    #[serde(rename = "world", skip_serializing)]
    World(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "island", skip_serializing)]
    Island(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// Visitor fields
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),

    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "data")]
    Data(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "driverInput")]
    DriverInput(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "steering")]
    Steering(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "engine")]
    Engine(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "transmission")]
    Transmission(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "brake")]
    Brake(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "suspension")]
    Suspension(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "aerodynamics")]
    Aerodynamics(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "wheelCollide")]
    WheelCollide(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "tyreMarks")]
    TyreMarks(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "velocityDamper")]
    VelocityDamper(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "wheelsInfo")]
    WheelsInfo(HkArrayClass<HkpVehicleInstanceWheelInfo<'a>>),
    /// Visitor fields
    #[serde(rename = "frictionStatus")]
    FrictionStatus(SingleClass<HkpVehicleFrictionStatus>),
    /// Visitor fields
    #[serde(rename = "deviceStatus")]
    DeviceStatus(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "isFixed")]
    IsFixed(HkArrayRef<bool>),
    /// Visitor fields
    #[serde(rename = "wheelsTimeSinceMaxPedalInput")]
    WheelsTimeSinceMaxPedalInput(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "tryingToReverse")]
    TryingToReverse(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "torque")]
    Torque(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "rpm")]
    Rpm(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "mainSteeringAngle")]
    MainSteeringAngle(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "wheelsSteeringAngle")]
    WheelsSteeringAngle(HkArrayNum<f32>),
    /// Visitor fields
    #[serde(rename = "isReversing")]
    IsReversing(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "currentGear")]
    CurrentGear(Primitive<i8>),
    /// Visitor fields
    #[serde(rename = "delayed")]
    Delayed(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "clutchDelayCountdown")]
    ClutchDelayCountdown(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleInstanceVisitor<'de>, "@name",
    ("entity" => Entity(Primitive<Cow<'de, str>>)),
    ("world" => World(Primitive<Cow<'de, str>>)),
    ("island" => Island(Primitive<Cow<'de, str>>)),
    ("userData" => UserData(Primitive<usize>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("data" => Data(Primitive<Cow<'de, str>>)),
    ("driverInput" => DriverInput(Primitive<Cow<'de, str>>)),
    ("steering" => Steering(Primitive<Cow<'de, str>>)),
    ("engine" => Engine(Primitive<Cow<'de, str>>)),
    ("transmission" => Transmission(Primitive<Cow<'de, str>>)),
    ("brake" => Brake(Primitive<Cow<'de, str>>)),
    ("suspension" => Suspension(Primitive<Cow<'de, str>>)),
    ("aerodynamics" => Aerodynamics(Primitive<Cow<'de, str>>)),
    ("wheelCollide" => WheelCollide(Primitive<Cow<'de, str>>)),
    ("tyreMarks" => TyreMarks(Primitive<Cow<'de, str>>)),
    ("velocityDamper" => VelocityDamper(Primitive<Cow<'de, str>>)),
    ("wheelsInfo" => WheelsInfo(HkArrayClass<HkpVehicleInstanceWheelInfo<'de>>)),
    ("frictionStatus" => FrictionStatus(SingleClass<HkpVehicleFrictionStatus>)),
    ("deviceStatus" => DeviceStatus(Primitive<Cow<'de, str>>)),
    ("isFixed" => IsFixed(HkArrayRef<bool>)),
    ("wheelsTimeSinceMaxPedalInput" => WheelsTimeSinceMaxPedalInput(Primitive<f32>)),
    ("tryingToReverse" => TryingToReverse(Primitive<bool>)),
    ("torque" => Torque(Primitive<f32>)),
    ("rpm" => Rpm(Primitive<f32>)),
    ("mainSteeringAngle" => MainSteeringAngle(Primitive<f32>)),
    ("wheelsSteeringAngle" => WheelsSteeringAngle(HkArrayNum<f32>)),
    ("isReversing" => IsReversing(Primitive<bool>)),
    ("currentGear" => CurrentGear(Primitive<i8>)),
    ("delayed" => Delayed(Primitive<bool>)),
    ("clutchDelayCountdown" => ClutchDelayCountdown(Primitive<f32>)),
}
