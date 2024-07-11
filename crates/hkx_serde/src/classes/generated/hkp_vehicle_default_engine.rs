//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleDefaultEngine`
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

/// `hkpVehicleDefaultEngine`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkpVehicleEngine`/`0xda8c7d7d`
/// - signature: `0x56f8ca24`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpVehicleDefaultEngine {
    // C++ Parent class(`hkpVehicleEngine` => parent: `hkReferencedObject`) has no fields
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
    /// -   name:`"minRPM"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub min_rpm: f32,
    /// # C++ Class Fields Info
    /// -   name:`"optRPM"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub opt_rpm: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxRPM"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub max_rpm: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxTorque"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub max_torque: f32,
    /// # C++ Class Fields Info
    /// -   name:`"torqueFactorAtMinRPM"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub torque_factor_at_min_rpm: f32,
    /// # C++ Class Fields Info
    /// -   name:`"torqueFactorAtMaxRPM"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub torque_factor_at_max_rpm: f32,
    /// # C++ Class Fields Info
    /// -   name:`"resistanceFactorAtMinRPM"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub resistance_factor_at_min_rpm: f32,
    /// # C++ Class Fields Info
    /// -   name:`"resistanceFactorAtOptRPM"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub resistance_factor_at_opt_rpm: f32,
    /// # C++ Class Fields Info
    /// -   name:`"resistanceFactorAtMaxRPM"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub resistance_factor_at_max_rpm: f32,
    /// # C++ Class Fields Info
    /// -   name:`"clutchSlipRPM"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub clutch_slip_rpm: f32,
}

impl Serialize for HkpVehicleDefaultEngine {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpVehicleDefaultEngineVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpVehicleDefaultEngine {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpVehicleDefaultEngineVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpVehicleDefaultEngineVisitor>> for HkpVehicleDefaultEngine {
    fn from(_values: Vec<HkpVehicleDefaultEngineVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut min_rpm = None;
            let mut opt_rpm = None;
            let mut max_rpm = None;
            let mut max_torque = None;
            let mut torque_factor_at_min_rpm = None;
            let mut torque_factor_at_max_rpm = None;
            let mut resistance_factor_at_min_rpm = None;
            let mut resistance_factor_at_opt_rpm = None;
            let mut resistance_factor_at_max_rpm = None;
            let mut clutch_slip_rpm = None;


        for _value in _values {
            match _value {
                HkpVehicleDefaultEngineVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpVehicleDefaultEngineVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpVehicleDefaultEngineVisitor::MinRpm(m) => min_rpm = Some(m),
                HkpVehicleDefaultEngineVisitor::OptRpm(m) => opt_rpm = Some(m),
                HkpVehicleDefaultEngineVisitor::MaxRpm(m) => max_rpm = Some(m),
                HkpVehicleDefaultEngineVisitor::MaxTorque(m) => max_torque = Some(m),
                HkpVehicleDefaultEngineVisitor::TorqueFactorAtMinRpm(m) => torque_factor_at_min_rpm = Some(m),
                HkpVehicleDefaultEngineVisitor::TorqueFactorAtMaxRpm(m) => torque_factor_at_max_rpm = Some(m),
                HkpVehicleDefaultEngineVisitor::ResistanceFactorAtMinRpm(m) => resistance_factor_at_min_rpm = Some(m),
                HkpVehicleDefaultEngineVisitor::ResistanceFactorAtOptRpm(m) => resistance_factor_at_opt_rpm = Some(m),
                HkpVehicleDefaultEngineVisitor::ResistanceFactorAtMaxRpm(m) => resistance_factor_at_max_rpm = Some(m),
                HkpVehicleDefaultEngineVisitor::ClutchSlipRpm(m) => clutch_slip_rpm = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            min_rpm: min_rpm.unwrap_or_default().into_inner(),
            opt_rpm: opt_rpm.unwrap_or_default().into_inner(),
            max_rpm: max_rpm.unwrap_or_default().into_inner(),
            max_torque: max_torque.unwrap_or_default().into_inner(),
            torque_factor_at_min_rpm: torque_factor_at_min_rpm.unwrap_or_default().into_inner(),
            torque_factor_at_max_rpm: torque_factor_at_max_rpm.unwrap_or_default().into_inner(),
            resistance_factor_at_min_rpm: resistance_factor_at_min_rpm.unwrap_or_default().into_inner(),
            resistance_factor_at_opt_rpm: resistance_factor_at_opt_rpm.unwrap_or_default().into_inner(),
            resistance_factor_at_max_rpm: resistance_factor_at_max_rpm.unwrap_or_default().into_inner(),
            clutch_slip_rpm: clutch_slip_rpm.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpVehicleDefaultEngine> for Vec<HkpVehicleDefaultEngineVisitor> {
    fn from(data: &HkpVehicleDefaultEngine) -> Self {
        vec![
            HkpVehicleDefaultEngineVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpVehicleDefaultEngineVisitor::ReferenceCount(data.reference_count.into()),
            HkpVehicleDefaultEngineVisitor::MinRpm(data.min_rpm.into()),
            HkpVehicleDefaultEngineVisitor::OptRpm(data.opt_rpm.into()),
            HkpVehicleDefaultEngineVisitor::MaxRpm(data.max_rpm.into()),
            HkpVehicleDefaultEngineVisitor::MaxTorque(data.max_torque.into()),
            HkpVehicleDefaultEngineVisitor::TorqueFactorAtMinRpm(data.torque_factor_at_min_rpm.into()),
            HkpVehicleDefaultEngineVisitor::TorqueFactorAtMaxRpm(data.torque_factor_at_max_rpm.into()),
            HkpVehicleDefaultEngineVisitor::ResistanceFactorAtMinRpm(data.resistance_factor_at_min_rpm.into()),
            HkpVehicleDefaultEngineVisitor::ResistanceFactorAtOptRpm(data.resistance_factor_at_opt_rpm.into()),
            HkpVehicleDefaultEngineVisitor::ResistanceFactorAtMaxRpm(data.resistance_factor_at_max_rpm.into()),
            HkpVehicleDefaultEngineVisitor::ClutchSlipRpm(data.clutch_slip_rpm.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpVehicleDefaultEngine {
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
enum HkpVehicleDefaultEngineVisitor {
    // C++ Parent class(`hkpVehicleEngine` => parent: `hkReferencedObject`) has no fields
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
    #[serde(rename = "minRPM")]
    MinRpm(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "optRPM")]
    OptRpm(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxRPM")]
    MaxRpm(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxTorque")]
    MaxTorque(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "torqueFactorAtMinRPM")]
    TorqueFactorAtMinRpm(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "torqueFactorAtMaxRPM")]
    TorqueFactorAtMaxRpm(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "resistanceFactorAtMinRPM")]
    ResistanceFactorAtMinRpm(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "resistanceFactorAtOptRPM")]
    ResistanceFactorAtOptRpm(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "resistanceFactorAtMaxRPM")]
    ResistanceFactorAtMaxRpm(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "clutchSlipRPM")]
    ClutchSlipRpm(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDefaultEngineVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("minRPM" => MinRpm(Primitive<f32>)),
    ("optRPM" => OptRpm(Primitive<f32>)),
    ("maxRPM" => MaxRpm(Primitive<f32>)),
    ("maxTorque" => MaxTorque(Primitive<f32>)),
    ("torqueFactorAtMinRPM" => TorqueFactorAtMinRpm(Primitive<f32>)),
    ("torqueFactorAtMaxRPM" => TorqueFactorAtMaxRpm(Primitive<f32>)),
    ("resistanceFactorAtMinRPM" => ResistanceFactorAtMinRpm(Primitive<f32>)),
    ("resistanceFactorAtOptRPM" => ResistanceFactorAtOptRpm(Primitive<f32>)),
    ("resistanceFactorAtMaxRPM" => ResistanceFactorAtMaxRpm(Primitive<f32>)),
    ("clutchSlipRPM" => ClutchSlipRpm(Primitive<f32>)),
}
