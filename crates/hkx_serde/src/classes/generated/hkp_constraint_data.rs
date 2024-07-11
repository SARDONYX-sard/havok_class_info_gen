//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpConstraintData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpConstraintData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x80559a4e`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpConstraintData {
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields

    /// # C++ Class Fields Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpConstraintData, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("userData" => UserData(Primitive<usize>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConstraintType {
    #[serde(rename = "CONSTRAINT_TYPE_BALLANDSOCKET")]
    ConstraintTypeBallandsocket = 0,
    #[serde(rename = "CONSTRAINT_TYPE_HINGE")]
    ConstraintTypeHinge = 1,
    #[serde(rename = "CONSTRAINT_TYPE_LIMITEDHINGE")]
    ConstraintTypeLimitedhinge = 2,
    #[serde(rename = "CONSTRAINT_TYPE_POINTTOPATH")]
    ConstraintTypePointtopath = 3,
    #[serde(rename = "CONSTRAINT_TYPE_PRISMATIC")]
    ConstraintTypePrismatic = 6,
    #[serde(rename = "CONSTRAINT_TYPE_RAGDOLL")]
    ConstraintTypeRagdoll = 7,
    #[serde(rename = "CONSTRAINT_TYPE_STIFFSPRING")]
    ConstraintTypeStiffspring = 8,
    #[serde(rename = "CONSTRAINT_TYPE_WHEEL")]
    ConstraintTypeWheel = 9,
    #[serde(rename = "CONSTRAINT_TYPE_GENERIC")]
    ConstraintTypeGeneric = 10,
    #[serde(rename = "CONSTRAINT_TYPE_CONTACT")]
    ConstraintTypeContact = 11,
    #[serde(rename = "CONSTRAINT_TYPE_BREAKABLE")]
    ConstraintTypeBreakable = 12,
    #[serde(rename = "CONSTRAINT_TYPE_MALLEABLE")]
    ConstraintTypeMalleable = 13,
    #[serde(rename = "CONSTRAINT_TYPE_POINTTOPLANE")]
    ConstraintTypePointtoplane = 14,
    #[serde(rename = "CONSTRAINT_TYPE_PULLEY")]
    ConstraintTypePulley = 15,
    #[serde(rename = "CONSTRAINT_TYPE_ROTATIONAL")]
    ConstraintTypeRotational = 16,
    #[serde(rename = "CONSTRAINT_TYPE_HINGE_LIMITS")]
    ConstraintTypeHingeLimits = 18,
    #[serde(rename = "CONSTRAINT_TYPE_RAGDOLL_LIMITS")]
    ConstraintTypeRagdollLimits = 19,
    #[serde(rename = "CONSTRAINT_TYPE_CUSTOM")]
    ConstraintTypeCustom = 20,
    #[serde(rename = "CONSTRAINT_TYPE_RACK_AND_PINION")]
    ConstraintTypeRackAndPinion = 21,
    #[serde(rename = "CONSTRAINT_TYPE_COG_WHEEL")]
    ConstraintTypeCogWheel = 22,
    #[serde(rename = "BEGIN_CONSTRAINT_CHAIN_TYPES")]
    BeginConstraintChainTypes = 100,
    #[serde(rename = "CONSTRAINT_TYPE_STIFF_SPRING_CHAIN")]
    ConstraintTypeStiffSpringChain = 100,
    #[serde(rename = "CONSTRAINT_TYPE_BALL_SOCKET_CHAIN")]
    ConstraintTypeBallSocketChain = 101,
    #[serde(rename = "CONSTRAINT_TYPE_POWERED_CHAIN")]
    ConstraintTypePoweredChain = 102,
}
