//! A Rust structure that implements a serializer/deserializer corresponding to `hkpVehicleRayCastWheelCollide`, a class defined in C++
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
/// -    size: 36
/// -  vtable: true
/// -  parent: hkpVehicleWheelCollide/`4a50fcb`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpVehicleRayCastWheelCollide<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpVehicleRayCastWheelCollide"`: Name of this class.
    #[serde(default = "HkpVehicleRayCastWheelCollide::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x41efd9e3`: Unique value of this class.
    #[serde(default = "HkpVehicleRayCastWheelCollide::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpVehicleRayCastWheelCollideHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpVehicleRayCastWheelCollideHkParam<'a>>
}

impl HkpVehicleRayCastWheelCollide<'_> {
    /// Return `"hkpVehicleRayCastWheelCollide"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpVehicleRayCastWheelCollide".into()
    }

    /// Return `"0x41efd9e3"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x41efd9e3".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleRayCastWheelCollideHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"wheelCollisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelCollisionFilterInfo")]
    WheelCollisionFilterInfo(u32),
    /// # Information on fields in the original C++ class
    /// -   name:`"phantom"`
    /// -   type: `struct hkpAabbPhantom*`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "phantom")]
    Phantom(Box<HkpAabbPhantom>),
    /// # Information on fields in the original C++ class
    /// -   name:`"rejectRayChassisListener"`
    /// -   type: `struct hkpRejectChassisListener`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rejectRayChassisListener")]
    RejectRayChassisListener(HkpRejectChassisListener),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleRayCastWheelCollideHkParam<'de>, "@name",
    ("wheelCollisionFilterInfo" => WheelCollisionFilterInfo(u32)),
    ("phantom" => Phantom(Box<HkpAabbPhantom>)),
    ("rejectRayChassisListener" => RejectRayChassisListener(HkpRejectChassisListener)),
}
