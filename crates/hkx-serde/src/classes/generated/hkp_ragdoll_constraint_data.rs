//! A Rust structure that implements a serializer/deserializer corresponding to `hkpRagdollConstraintData`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::hk_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// In XML, it is enclosed in a `hkobject` tag
/// and the `class` attribute contains the C++ class nam
///
/// # Information on the original C++ class
/// -    size: 352
/// -  vtable: true
/// -  parent: hkpConstraintData/`80559a4e`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpRagdollConstraintData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpRagdollConstraintData"`: The original C++ class name.
    #[serde(default = "HkpRagdollConstraintData::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x8fb5dd29`: Unique value of this class.
    #[serde(default = "HkpRagdollConstraintData::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpRagdollConstraintDataHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpRagdollConstraintDataHkParam<'a>>
}

impl HkpRagdollConstraintData<'_> {
    /// Return `"hkpRagdollConstraintData"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpRagdollConstraintData".into()
    }

    /// Return `"0x8fb5dd29"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x8fb5dd29".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpRagdollConstraintDataHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"atoms"`
    /// -   type: `struct hkpRagdollConstraintDataAtoms`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "atoms")]
    Atoms(HkpRagdollConstraintDataAtoms),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpRagdollConstraintDataHkParam<'de>, "@name",
    ("atoms" => Atoms(HkpRagdollConstraintDataAtoms)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MotorIndex {
    #[serde(rename = "MOTOR_TWIST")]
    MotorTwist = 0,
    #[serde(rename = "MOTOR_PLANE")]
    MotorPlane = 1,
    #[serde(rename = "MOTOR_CONE")]
    MotorCone = 2,
}
