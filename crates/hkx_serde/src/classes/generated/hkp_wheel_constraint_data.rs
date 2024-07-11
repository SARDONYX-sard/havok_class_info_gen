//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpWheelConstraintData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpWheelConstraintData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 352
/// -    vtable: true
/// -    parent: `hkpConstraintData`/`0x80559a4e`
/// - signature: `0xb4c46671`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpWheelConstraintData {
    /// # C++ Class Fields Info
    /// -   name:`"atoms"`
    /// -   type: `struct hkpWheelConstraintDataAtoms`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "atoms")]
    Atoms(HkpWheelConstraintDataAtoms),
    /// # C++ Class Fields Info
    /// -   name:`"initialAxleInB"`
    /// -   type: `hkVector4`
    /// - offset: 320
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "initialAxleInB")]
    InitialAxleInB(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"initialSteeringAxisInB"`
    /// -   type: `hkVector4`
    /// - offset: 336
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "initialSteeringAxisInB")]
    InitialSteeringAxisInB(Vector4<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpWheelConstraintData, "@name",
    ("atoms" => Atoms(HkpWheelConstraintDataAtoms)),
    ("initialAxleInB" => InitialAxleInB(Vector4<f32>)),
    ("initialSteeringAxisInB" => InitialSteeringAxisInB(Vector4<f32>)),
}
