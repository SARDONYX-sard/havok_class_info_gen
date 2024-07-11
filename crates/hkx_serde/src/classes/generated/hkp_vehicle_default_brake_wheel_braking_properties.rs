//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleDefaultBrakeWheelBrakingProperties`
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

/// `hkpVehicleDefaultBrakeWheelBrakingProperties`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0x1ffad971`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpVehicleDefaultBrakeWheelBrakingProperties {
    /// # C++ Class Fields Info
    /// -   name:`"maxBreakingTorque"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub max_breaking_torque: f32,
    /// # C++ Class Fields Info
    /// -   name:`"minPedalInputToBlock"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub min_pedal_input_to_block: f32,
    /// # C++ Class Fields Info
    /// -   name:`"isConnectedToHandbrake"`
    /// -   type: `hkBool`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub is_connected_to_handbrake: bool,
}

impl Serialize for HkpVehicleDefaultBrakeWheelBrakingProperties {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpVehicleDefaultBrakeWheelBrakingPropertiesVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpVehicleDefaultBrakeWheelBrakingProperties {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpVehicleDefaultBrakeWheelBrakingPropertiesVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpVehicleDefaultBrakeWheelBrakingPropertiesVisitor>> for HkpVehicleDefaultBrakeWheelBrakingProperties {
    fn from(_values: Vec<HkpVehicleDefaultBrakeWheelBrakingPropertiesVisitor>) -> Self {
            let mut max_breaking_torque = None;
            let mut min_pedal_input_to_block = None;
            let mut is_connected_to_handbrake = None;


        for _value in _values {
            match _value {
                HkpVehicleDefaultBrakeWheelBrakingPropertiesVisitor::MaxBreakingTorque(m) => max_breaking_torque = Some(m),
                HkpVehicleDefaultBrakeWheelBrakingPropertiesVisitor::MinPedalInputToBlock(m) => min_pedal_input_to_block = Some(m),
                HkpVehicleDefaultBrakeWheelBrakingPropertiesVisitor::IsConnectedToHandbrake(m) => is_connected_to_handbrake = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            max_breaking_torque: max_breaking_torque.unwrap_or_default().into_inner(),
            min_pedal_input_to_block: min_pedal_input_to_block.unwrap_or_default().into_inner(),
            is_connected_to_handbrake: is_connected_to_handbrake.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpVehicleDefaultBrakeWheelBrakingProperties> for Vec<HkpVehicleDefaultBrakeWheelBrakingPropertiesVisitor> {
    fn from(data: &HkpVehicleDefaultBrakeWheelBrakingProperties) -> Self {
        vec![
            HkpVehicleDefaultBrakeWheelBrakingPropertiesVisitor::MaxBreakingTorque(data.max_breaking_torque.into()),
            HkpVehicleDefaultBrakeWheelBrakingPropertiesVisitor::MinPedalInputToBlock(data.min_pedal_input_to_block.into()),
            HkpVehicleDefaultBrakeWheelBrakingPropertiesVisitor::IsConnectedToHandbrake(data.is_connected_to_handbrake.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpVehicleDefaultBrakeWheelBrakingProperties {
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
enum HkpVehicleDefaultBrakeWheelBrakingPropertiesVisitor {
    /// Visitor fields
    #[serde(rename = "maxBreakingTorque")]
    MaxBreakingTorque(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "minPedalInputToBlock")]
    MinPedalInputToBlock(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "isConnectedToHandbrake")]
    IsConnectedToHandbrake(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDefaultBrakeWheelBrakingPropertiesVisitor, "@name",
    ("maxBreakingTorque" => MaxBreakingTorque(Primitive<f32>)),
    ("minPedalInputToBlock" => MinPedalInputToBlock(Primitive<f32>)),
    ("isConnectedToHandbrake" => IsConnectedToHandbrake(Primitive<bool>)),
}
