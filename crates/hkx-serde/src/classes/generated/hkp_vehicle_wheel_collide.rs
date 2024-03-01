//! A Rust structure that implements a serializer/deserializer corresponding to `hkpVehicleWheelCollide`, a class defined in C++
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
pub struct HkpVehicleWheelCollide<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpVehicleWheelCollide"`: Name of this class.
    #[serde(default = "HkpVehicleWheelCollide::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x4a50fcb`: Unique value of this class.
    #[serde(default = "HkpVehicleWheelCollide::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpVehicleWheelCollideHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpVehicleWheelCollideHkParam<'a>>
}

impl HkpVehicleWheelCollide<'_> {
    /// Return `"hkpVehicleWheelCollide"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpVehicleWheelCollide".into()
    }

    /// Return `"0x4a50fcb"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x4a50fcb".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleWheelCollideHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"alreadyUsed"`
    /// -   type: `hkBool`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "alreadyUsed")]
    AlreadyUsed(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"type"`
    /// -   type: `enum unknown`
    /// - offset: 9
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "type", skip_serializing)]
    Type(Unknown),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleWheelCollideHkParam<'de>, "@name",
    ("alreadyUsed" => AlreadyUsed(bool)),
    ("type" => Type(Unknown)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WheelCollideType {
    #[serde(rename = "INVALID_WHEEL_COLLIDE")]
    InvalidWheelCollide = 0,
    #[serde(rename = "RAY_CAST_WHEEL_COLLIDE")]
    RayCastWheelCollide = 1,
    #[serde(rename = "LINEAR_CAST_WHEEL_COLLIDE")]
    LinearCastWheelCollide = 2,
    #[serde(rename = "USER_WHEEL_COLLIDE1")]
    UserWheelCollide1 = 3,
    #[serde(rename = "USER_WHEEL_COLLIDE2")]
    UserWheelCollide2 = 4,
    #[serde(rename = "USER_WHEEL_COLLIDE3")]
    UserWheelCollide3 = 5,
    #[serde(rename = "USER_WHEEL_COLLIDE4")]
    UserWheelCollide4 = 6,
    #[serde(rename = "USER_WHEEL_COLLIDE5")]
    UserWheelCollide5 = 7,
}
