//! A Rust structure that implements a serializer/deserializer corresponding to `hkpCallbackConstraintMotor`, a class defined in C++
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
/// -    size: 40
/// -  vtable: true
/// -  parent: hkpLimitedForceConstraintMotor/`3377b0b0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpCallbackConstraintMotor<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpCallbackConstraintMotor"`: Name of this class.
    #[serde(default = "HkpCallbackConstraintMotor::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xafcd79ad`: Unique value of this class.
    #[serde(default = "HkpCallbackConstraintMotor::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpCallbackConstraintMotorHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpCallbackConstraintMotorHkParam<'a>>
}

impl HkpCallbackConstraintMotor<'_> {
    /// Return `"hkpCallbackConstraintMotor"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpCallbackConstraintMotor".into()
    }

    /// Return `"0xafcd79ad"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xafcd79ad".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpCallbackConstraintMotorHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"callbackFunc"`
    /// -   type: `void*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "callbackFunc", skip_serializing)]
    CallbackFunc(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"callbackType"`
    /// -   type: `enum CallbackType`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "callbackType")]
    CallbackType(CallbackType),
    /// # Information on fields in the original C++ class
    /// -   name:`"userData0"`
    /// -   type: `hkUlong`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData0")]
    UserData0(u64),
    /// # Information on fields in the original C++ class
    /// -   name:`"userData1"`
    /// -   type: `hkUlong`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData1")]
    UserData1(u64),
    /// # Information on fields in the original C++ class
    /// -   name:`"userData2"`
    /// -   type: `hkUlong`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData2")]
    UserData2(u64),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpCallbackConstraintMotorHkParam<'de>, "@name",
    ("callbackFunc" => CallbackFunc(())),
    ("callbackType" => CallbackType(CallbackType)),
    ("userData0" => UserData0(u64)),
    ("userData1" => UserData1(u64)),
    ("userData2" => UserData2(u64)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CallbackType {
    #[serde(rename = "CALLBACK_MOTOR_TYPE_HAVOK_DEMO_SPRING_DAMPER")]
    CallbackMotorTypeHavokDemoSpringDamper = 0,
    #[serde(rename = "CALLBACK_MOTOR_TYPE_USER_0")]
    CallbackMotorTypeUser0 = 1,
    #[serde(rename = "CALLBACK_MOTOR_TYPE_USER_1")]
    CallbackMotorTypeUser1 = 2,
    #[serde(rename = "CALLBACK_MOTOR_TYPE_USER_2")]
    CallbackMotorTypeUser2 = 3,
    #[serde(rename = "CALLBACK_MOTOR_TYPE_USER_3")]
    CallbackMotorTypeUser3 = 4,
}
