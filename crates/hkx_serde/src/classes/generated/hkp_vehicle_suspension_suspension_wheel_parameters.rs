//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleSuspensionSuspensionWheelParameters`
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

/// `hkpVehicleSuspensionSuspensionWheelParameters`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: false
/// - signature: `0x358bfe9c`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpVehicleSuspensionSuspensionWheelParameters {
    /// # C++ Class Fields Info
    /// -   name:`"hardpointChassisSpace"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub hardpoint_chassis_space: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"directionChassisSpace"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub direction_chassis_space: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"length"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub length: f32,
}

impl Serialize for HkpVehicleSuspensionSuspensionWheelParameters {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpVehicleSuspensionSuspensionWheelParametersVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpVehicleSuspensionSuspensionWheelParameters {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpVehicleSuspensionSuspensionWheelParametersVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpVehicleSuspensionSuspensionWheelParametersVisitor>> for HkpVehicleSuspensionSuspensionWheelParameters {
    fn from(_values: Vec<HkpVehicleSuspensionSuspensionWheelParametersVisitor>) -> Self {
            let mut hardpoint_chassis_space = None;
            let mut direction_chassis_space = None;
            let mut length = None;


        for _value in _values {
            match _value {
                HkpVehicleSuspensionSuspensionWheelParametersVisitor::HardpointChassisSpace(m) => hardpoint_chassis_space = Some(m),
                HkpVehicleSuspensionSuspensionWheelParametersVisitor::DirectionChassisSpace(m) => direction_chassis_space = Some(m),
                HkpVehicleSuspensionSuspensionWheelParametersVisitor::Length(m) => length = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            hardpoint_chassis_space: hardpoint_chassis_space.unwrap_or_default().into_inner(),
            direction_chassis_space: direction_chassis_space.unwrap_or_default().into_inner(),
            length: length.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpVehicleSuspensionSuspensionWheelParameters> for Vec<HkpVehicleSuspensionSuspensionWheelParametersVisitor> {
    fn from(data: &HkpVehicleSuspensionSuspensionWheelParameters) -> Self {
        vec![
            HkpVehicleSuspensionSuspensionWheelParametersVisitor::HardpointChassisSpace(data.hardpoint_chassis_space.into()),
            HkpVehicleSuspensionSuspensionWheelParametersVisitor::DirectionChassisSpace(data.direction_chassis_space.into()),
            HkpVehicleSuspensionSuspensionWheelParametersVisitor::Length(data.length.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpVehicleSuspensionSuspensionWheelParameters {
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
enum HkpVehicleSuspensionSuspensionWheelParametersVisitor {
    /// Visitor fields
    #[serde(rename = "hardpointChassisSpace")]
    HardpointChassisSpace(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "directionChassisSpace")]
    DirectionChassisSpace(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "length")]
    Length(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleSuspensionSuspensionWheelParametersVisitor, "@name",
    ("hardpointChassisSpace" => HardpointChassisSpace(Primitive<Vector4<f32>>)),
    ("directionChassisSpace" => DirectionChassisSpace(Primitive<Vector4<f32>>)),
    ("length" => Length(Primitive<f32>)),
}
