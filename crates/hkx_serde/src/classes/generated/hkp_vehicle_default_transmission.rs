//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleDefaultTransmission`
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

/// `hkpVehicleDefaultTransmission`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 52
/// -    vtable: true
/// -    parent: `hkpVehicleTransmission`/`0xda8c7d7d`
/// - signature: `0x235d5d6b`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpVehicleDefaultTransmission {
    // C++ Parent class(`hkpVehicleTransmission` => parent: `hkReferencedObject`) has no fields
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
    /// -   name:`"downshiftRPM"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub downshift_rpm: f32,
    /// # C++ Class Fields Info
    /// -   name:`"upshiftRPM"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub upshift_rpm: f32,
    /// # C++ Class Fields Info
    /// -   name:`"primaryTransmissionRatio"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub primary_transmission_ratio: f32,
    /// # C++ Class Fields Info
    /// -   name:`"clutchDelayTime"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub clutch_delay_time: f32,
    /// # C++ Class Fields Info
    /// -   name:`"reverseGearRatio"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub reverse_gear_ratio: f32,
    /// # C++ Class Fields Info
    /// -   name:`"gearsRatio"`
    /// -   type: `hkArray<hkReal>`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub gears_ratio: HkArrayNum<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"wheelsTorqueRatio"`
    /// -   type: `hkArray<hkReal>`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub wheels_torque_ratio: HkArrayNum<f32>,
}

impl Serialize for HkpVehicleDefaultTransmission {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpVehicleDefaultTransmissionVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpVehicleDefaultTransmission {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpVehicleDefaultTransmissionVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpVehicleDefaultTransmissionVisitor>> for HkpVehicleDefaultTransmission {
    fn from(_values: Vec<HkpVehicleDefaultTransmissionVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut downshift_rpm = None;
            let mut upshift_rpm = None;
            let mut primary_transmission_ratio = None;
            let mut clutch_delay_time = None;
            let mut reverse_gear_ratio = None;
            let mut gears_ratio = None;
            let mut wheels_torque_ratio = None;


        for _value in _values {
            match _value {
                HkpVehicleDefaultTransmissionVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpVehicleDefaultTransmissionVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpVehicleDefaultTransmissionVisitor::DownshiftRpm(m) => downshift_rpm = Some(m),
                HkpVehicleDefaultTransmissionVisitor::UpshiftRpm(m) => upshift_rpm = Some(m),
                HkpVehicleDefaultTransmissionVisitor::PrimaryTransmissionRatio(m) => primary_transmission_ratio = Some(m),
                HkpVehicleDefaultTransmissionVisitor::ClutchDelayTime(m) => clutch_delay_time = Some(m),
                HkpVehicleDefaultTransmissionVisitor::ReverseGearRatio(m) => reverse_gear_ratio = Some(m),
                HkpVehicleDefaultTransmissionVisitor::GearsRatio(m) => gears_ratio = Some(m),
                HkpVehicleDefaultTransmissionVisitor::WheelsTorqueRatio(m) => wheels_torque_ratio = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            downshift_rpm: downshift_rpm.unwrap_or_default().into_inner(),
            upshift_rpm: upshift_rpm.unwrap_or_default().into_inner(),
            primary_transmission_ratio: primary_transmission_ratio.unwrap_or_default().into_inner(),
            clutch_delay_time: clutch_delay_time.unwrap_or_default().into_inner(),
            reverse_gear_ratio: reverse_gear_ratio.unwrap_or_default().into_inner(),
            gears_ratio: gears_ratio.unwrap_or_default(),
            wheels_torque_ratio: wheels_torque_ratio.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpVehicleDefaultTransmission> for Vec<HkpVehicleDefaultTransmissionVisitor> {
    fn from(data: &HkpVehicleDefaultTransmission) -> Self {
        vec![
            HkpVehicleDefaultTransmissionVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpVehicleDefaultTransmissionVisitor::ReferenceCount(data.reference_count.into()),
            HkpVehicleDefaultTransmissionVisitor::DownshiftRpm(data.downshift_rpm.into()),
            HkpVehicleDefaultTransmissionVisitor::UpshiftRpm(data.upshift_rpm.into()),
            HkpVehicleDefaultTransmissionVisitor::PrimaryTransmissionRatio(data.primary_transmission_ratio.into()),
            HkpVehicleDefaultTransmissionVisitor::ClutchDelayTime(data.clutch_delay_time.into()),
            HkpVehicleDefaultTransmissionVisitor::ReverseGearRatio(data.reverse_gear_ratio.into()),
            HkpVehicleDefaultTransmissionVisitor::GearsRatio(data.gears_ratio.clone()),
            HkpVehicleDefaultTransmissionVisitor::WheelsTorqueRatio(data.wheels_torque_ratio.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpVehicleDefaultTransmission {
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
enum HkpVehicleDefaultTransmissionVisitor {
    // C++ Parent class(`hkpVehicleTransmission` => parent: `hkReferencedObject`) has no fields
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
    #[serde(rename = "downshiftRPM")]
    DownshiftRpm(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "upshiftRPM")]
    UpshiftRpm(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "primaryTransmissionRatio")]
    PrimaryTransmissionRatio(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "clutchDelayTime")]
    ClutchDelayTime(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "reverseGearRatio")]
    ReverseGearRatio(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "gearsRatio")]
    GearsRatio(HkArrayNum<f32>),
    /// Visitor fields
    #[serde(rename = "wheelsTorqueRatio")]
    WheelsTorqueRatio(HkArrayNum<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDefaultTransmissionVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("downshiftRPM" => DownshiftRpm(Primitive<f32>)),
    ("upshiftRPM" => UpshiftRpm(Primitive<f32>)),
    ("primaryTransmissionRatio" => PrimaryTransmissionRatio(Primitive<f32>)),
    ("clutchDelayTime" => ClutchDelayTime(Primitive<f32>)),
    ("reverseGearRatio" => ReverseGearRatio(Primitive<f32>)),
    ("gearsRatio" => GearsRatio(HkArrayNum<f32>)),
    ("wheelsTorqueRatio" => WheelsTorqueRatio(HkArrayNum<f32>)),
}
