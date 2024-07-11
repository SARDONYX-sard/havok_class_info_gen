//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleDefaultBrake`
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

/// `hkpVehicleDefaultBrake`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: true
/// -    parent: `hkpVehicleBrake`/`0xda8c7d7d`
/// - signature: `0x4b4f8816`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpVehicleDefaultBrake {
    // C++ Parent class(`hkpVehicleBrake` => parent: `hkReferencedObject`) has no fields
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
    /// -   name:`"wheelBrakingProperties"`
    /// -   type: `hkArray<struct hkpVehicleDefaultBrakeWheelBrakingProperties>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub wheel_braking_properties: HkArrayClass<HkpVehicleDefaultBrakeWheelBrakingProperties>,
    /// # C++ Class Fields Info
    /// -   name:`"wheelsMinTimeToBlock"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub wheels_min_time_to_block: f32,
}

impl Serialize for HkpVehicleDefaultBrake {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpVehicleDefaultBrakeVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpVehicleDefaultBrake {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpVehicleDefaultBrakeVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpVehicleDefaultBrakeVisitor>> for HkpVehicleDefaultBrake {
    fn from(_values: Vec<HkpVehicleDefaultBrakeVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut wheel_braking_properties = None;
            let mut wheels_min_time_to_block = None;


        for _value in _values {
            match _value {
                HkpVehicleDefaultBrakeVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpVehicleDefaultBrakeVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpVehicleDefaultBrakeVisitor::WheelBrakingProperties(m) => wheel_braking_properties = Some(m),
                HkpVehicleDefaultBrakeVisitor::WheelsMinTimeToBlock(m) => wheels_min_time_to_block = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            wheel_braking_properties: wheel_braking_properties.unwrap_or_default(),
            wheels_min_time_to_block: wheels_min_time_to_block.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpVehicleDefaultBrake> for Vec<HkpVehicleDefaultBrakeVisitor> {
    fn from(data: &HkpVehicleDefaultBrake) -> Self {
        vec![
            HkpVehicleDefaultBrakeVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpVehicleDefaultBrakeVisitor::ReferenceCount(data.reference_count.into()),
            HkpVehicleDefaultBrakeVisitor::WheelBrakingProperties(data.wheel_braking_properties.clone()),
            HkpVehicleDefaultBrakeVisitor::WheelsMinTimeToBlock(data.wheels_min_time_to_block.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpVehicleDefaultBrake {
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
enum HkpVehicleDefaultBrakeVisitor {
    // C++ Parent class(`hkpVehicleBrake` => parent: `hkReferencedObject`) has no fields
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
    #[serde(rename = "wheelBrakingProperties")]
    WheelBrakingProperties(HkArrayClass<HkpVehicleDefaultBrakeWheelBrakingProperties>),
    /// Visitor fields
    #[serde(rename = "wheelsMinTimeToBlock")]
    WheelsMinTimeToBlock(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDefaultBrakeVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("wheelBrakingProperties" => WheelBrakingProperties(HkArrayClass<HkpVehicleDefaultBrakeWheelBrakingProperties>)),
    ("wheelsMinTimeToBlock" => WheelsMinTimeToBlock(Primitive<f32>)),
}
