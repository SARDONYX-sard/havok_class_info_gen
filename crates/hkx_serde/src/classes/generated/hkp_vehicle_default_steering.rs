//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleDefaultSteering`
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

/// `hkpVehicleDefaultSteering`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 28
/// -    vtable: true
/// -    parent: `hkpVehicleSteering`/`0xda8c7d7d`
/// - signature: `0x8f0411c8`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpVehicleDefaultSteering {
    // C++ Parent class(`hkpVehicleSteering` => parent: `hkReferencedObject`) has no fields
    //
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
    /// -   name:`"maxSteeringAngle"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub max_steering_angle: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxSpeedFullSteeringAngle"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub max_speed_full_steering_angle: f32,
    /// # C++ Class Fields Info
    /// -   name:`"doesWheelSteer"`
    /// -   type: `hkArray<hkBool>`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub does_wheel_steer: HkArrayRef<bool>,
}

impl Serialize for HkpVehicleDefaultSteering {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpVehicleDefaultSteeringVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpVehicleDefaultSteering {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpVehicleDefaultSteeringVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpVehicleDefaultSteeringVisitor>> for HkpVehicleDefaultSteering {
    fn from(_values: Vec<HkpVehicleDefaultSteeringVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut max_steering_angle = None;
            let mut max_speed_full_steering_angle = None;
            let mut does_wheel_steer = None;


        for _value in _values {
            match _value {
                HkpVehicleDefaultSteeringVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpVehicleDefaultSteeringVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpVehicleDefaultSteeringVisitor::MaxSteeringAngle(m) => max_steering_angle = Some(m),
                HkpVehicleDefaultSteeringVisitor::MaxSpeedFullSteeringAngle(m) => max_speed_full_steering_angle = Some(m),
                HkpVehicleDefaultSteeringVisitor::DoesWheelSteer(m) => does_wheel_steer = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            max_steering_angle: max_steering_angle.unwrap_or_default().into_inner(),
            max_speed_full_steering_angle: max_speed_full_steering_angle.unwrap_or_default().into_inner(),
            does_wheel_steer: does_wheel_steer.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpVehicleDefaultSteering> for Vec<HkpVehicleDefaultSteeringVisitor> {
    fn from(data: &HkpVehicleDefaultSteering) -> Self {
        vec![
            HkpVehicleDefaultSteeringVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpVehicleDefaultSteeringVisitor::ReferenceCount(data.reference_count.into()),
            HkpVehicleDefaultSteeringVisitor::MaxSteeringAngle(data.max_steering_angle.into()),
            HkpVehicleDefaultSteeringVisitor::MaxSpeedFullSteeringAngle(data.max_speed_full_steering_angle.into()),
            HkpVehicleDefaultSteeringVisitor::DoesWheelSteer(data.does_wheel_steer.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpVehicleDefaultSteering {
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
enum HkpVehicleDefaultSteeringVisitor {
    // C++ Parent class(`hkpVehicleSteering` => parent: `hkReferencedObject`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "maxSteeringAngle")]
    MaxSteeringAngle(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxSpeedFullSteeringAngle")]
    MaxSpeedFullSteeringAngle(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "doesWheelSteer")]
    DoesWheelSteer(HkArrayRef<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDefaultSteeringVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("maxSteeringAngle" => MaxSteeringAngle(Primitive<f32>)),
    ("maxSpeedFullSteeringAngle" => MaxSpeedFullSteeringAngle(Primitive<f32>)),
    ("doesWheelSteer" => DoesWheelSteer(HkArrayRef<bool>)),
}
