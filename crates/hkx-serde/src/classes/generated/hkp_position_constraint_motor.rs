//! A Rust structure that implements a serializer/deserializer corresponding to `hkpPositionConstraintMotor`, a class defined in C++
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
/// -    size: 36
/// -  vtable: true
/// -  parent: hkpLimitedForceConstraintMotor/`3377b0b0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpPositionConstraintMotor<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpPositionConstraintMotor"`: The original C++ class name.
    #[serde(default = "HkpPositionConstraintMotor::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x748fb303`: Unique value of this class.
    #[serde(default = "HkpPositionConstraintMotor::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpPositionConstraintMotorHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpPositionConstraintMotorHkParam<'a>>
}

impl HkpPositionConstraintMotor<'_> {
    /// Return `"hkpPositionConstraintMotor"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpPositionConstraintMotor".into()
    }

    /// Return `"0x748fb303"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x748fb303".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPositionConstraintMotorHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"tau"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "tau")]
    Tau(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"damping"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "damping")]
    Damping(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"proportionalRecoveryVelocity"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "proportionalRecoveryVelocity")]
    ProportionalRecoveryVelocity(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"constantRecoveryVelocity"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "constantRecoveryVelocity")]
    ConstantRecoveryVelocity(Primitive<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpPositionConstraintMotorHkParam<'de>, "@name",
    ("tau" => Tau(Primitive<f32>)),
    ("damping" => Damping(Primitive<f32>)),
    ("proportionalRecoveryVelocity" => ProportionalRecoveryVelocity(Primitive<f32>)),
    ("constantRecoveryVelocity" => ConstantRecoveryVelocity(Primitive<f32>)),
}
