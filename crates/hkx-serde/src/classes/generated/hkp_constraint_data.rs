//! A Rust structure that implements a serializer/deserializer corresponding to `hkpConstraintData`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// In XML, it is enclosed in a `hkobject` tag
/// and the `class` attribute contains the C++ class nam
///
/// # Information on the original C++ class
/// -    size: 12
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpConstraintData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpConstraintData"`: Name of this class.
    #[serde(default = "HkpConstraintData::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x80559a4e`: Unique value of this class.
    #[serde(default = "HkpConstraintData::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpConstraintDataHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpConstraintDataHkParam<'a>>
}

impl HkpConstraintData<'_> {
    /// Return `"hkpConstraintData"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpConstraintData".into()
    }

    /// Return `"0x80559a4e"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x80559a4e".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpConstraintDataHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData")]
    UserData(u64),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpConstraintDataHkParam<'de>, "@name",
    ("userData" => UserData(u64)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
