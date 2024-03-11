//! A Rust structure that implements a serializer/deserializer corresponding to `hkpConstraintMotor`, a class defined in C++
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
/// -    size: 12
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpConstraintMotor<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpConstraintMotor"`: The original C++ class name.
    #[serde(default = "HkpConstraintMotor::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x6a44c317`: Unique value of this class.
    #[serde(default = "HkpConstraintMotor::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpConstraintMotorHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpConstraintMotorHkParam<'a>>
}

impl HkpConstraintMotor<'_> {
    /// Return `"hkpConstraintMotor"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpConstraintMotor".into()
    }

    /// Return `"0x6a44c317"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x6a44c317".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpConstraintMotorHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"type"`
    /// -   type: `enum MotorType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(MotorType),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpConstraintMotorHkParam<'de>, "@name",
    ("type" => Type(MotorType)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MotorType {
    #[serde(rename = "TYPE_INVALID")]
    TypeInvalid = 0,
    #[serde(rename = "TYPE_POSITION")]
    TypePosition = 1,
    #[serde(rename = "TYPE_VELOCITY")]
    TypeVelocity = 2,
    #[serde(rename = "TYPE_SPRING_DAMPER")]
    TypeSpringDamper = 3,
    #[serde(rename = "TYPE_CALLBACK")]
    TypeCallback = 4,
    #[serde(rename = "TYPE_MAX")]
    TypeMax = 5,
}
