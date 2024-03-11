//! A Rust structure that implements a serializer/deserializer corresponding to `hkpVelocityConstraintMotor`, a class defined in C++
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
/// -    size: 32
/// -  vtable: true
/// -  parent: hkpLimitedForceConstraintMotor/`3377b0b0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpVelocityConstraintMotor<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpVelocityConstraintMotor"`: The original C++ class name.
    #[serde(default = "HkpVelocityConstraintMotor::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xfca2fcc3`: Unique value of this class.
    #[serde(default = "HkpVelocityConstraintMotor::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpVelocityConstraintMotorHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpVelocityConstraintMotorHkParam<'a>>
}

impl HkpVelocityConstraintMotor<'_> {
    /// Return `"hkpVelocityConstraintMotor"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpVelocityConstraintMotor".into()
    }

    /// Return `"0xfca2fcc3"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xfca2fcc3".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVelocityConstraintMotorHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"tau"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "tau")]
    Tau(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"velocityTarget"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "velocityTarget")]
    VelocityTarget(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"useVelocityTargetFromConstraintTargets"`
    /// -   type: `hkBool`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "useVelocityTargetFromConstraintTargets")]
    UseVelocityTargetFromConstraintTargets(Primitive<bool>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpVelocityConstraintMotorHkParam<'de>, "@name",
    ("tau" => Tau(Primitive<f32>)),
    ("velocityTarget" => VelocityTarget(Primitive<f32>)),
    ("useVelocityTargetFromConstraintTargets" => UseVelocityTargetFromConstraintTargets(Primitive<bool>)),
}
