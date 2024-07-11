//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpWheelConstraintDataAtoms`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpWheelConstraintDataAtoms`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 304
/// -    vtable: false
/// - signature: `0x1188cbe1`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpWheelConstraintDataAtoms {
    /// # C++ Class Fields Info
    /// -   name:`"suspensionBase"`
    /// -   type: `struct hkpSetLocalTransformsConstraintAtom`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "suspensionBase")]
    SuspensionBase(HkpSetLocalTransformsConstraintAtom),
    /// # C++ Class Fields Info
    /// -   name:`"lin0Limit"`
    /// -   type: `struct hkpLinLimitConstraintAtom`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lin0Limit")]
    Lin0Limit(HkpLinLimitConstraintAtom),
    /// # C++ Class Fields Info
    /// -   name:`"lin0Soft"`
    /// -   type: `struct hkpLinSoftConstraintAtom`
    /// - offset: 156
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lin0Soft")]
    Lin0Soft(HkpLinSoftConstraintAtom),
    /// # C++ Class Fields Info
    /// -   name:`"lin1"`
    /// -   type: `struct hkpLinConstraintAtom`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lin1")]
    Lin1(HkpLinConstraintAtom),
    /// # C++ Class Fields Info
    /// -   name:`"lin2"`
    /// -   type: `struct hkpLinConstraintAtom`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lin2")]
    Lin2(HkpLinConstraintAtom),
    /// # C++ Class Fields Info
    /// -   name:`"steeringBase"`
    /// -   type: `struct hkpSetLocalRotationsConstraintAtom`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "steeringBase")]
    SteeringBase(HkpSetLocalRotationsConstraintAtom),
    /// # C++ Class Fields Info
    /// -   name:`"2dAng"`
    /// -   type: `struct hkp2dAngConstraintAtom`
    /// - offset: 288
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "2dAng")]
    _2DAng(Hkp2DAngConstraintAtom),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpWheelConstraintDataAtoms, "@name",
    ("suspensionBase" => SuspensionBase(HkpSetLocalTransformsConstraintAtom)),
    ("lin0Limit" => Lin0Limit(HkpLinLimitConstraintAtom)),
    ("lin0Soft" => Lin0Soft(HkpLinSoftConstraintAtom)),
    ("lin1" => Lin1(HkpLinConstraintAtom)),
    ("lin2" => Lin2(HkpLinConstraintAtom)),
    ("steeringBase" => SteeringBase(HkpSetLocalRotationsConstraintAtom)),
    ("2dAng" => _2DAng(Hkp2DAngConstraintAtom)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Axis {
    #[serde(rename = "AXIS_SUSPENSION")]
    AxisSuspension = 0,
    #[serde(rename = "AXIS_PERP_SUSPENSION")]
    AxisPerpSuspension = 1,
    #[serde(rename = "AXIS_AXLE")]
    AxisAxle = 0,
    #[serde(rename = "AXIS_STEERING")]
    AxisSteering = 1,
}
