//! A Rust structure that implements a serializer/deserializer corresponding to `hkpVehicleLinearCastWheelCollide`, a class defined in C++
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
/// -    size: 52
/// -  vtable: true
/// -  parent: hkpVehicleWheelCollide/`4a50fcb`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpVehicleLinearCastWheelCollide<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpVehicleLinearCastWheelCollide"`: The original C++ class name.
    #[serde(default = "HkpVehicleLinearCastWheelCollide::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xc59399d0`: Unique value of this class.
    #[serde(default = "HkpVehicleLinearCastWheelCollide::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpVehicleLinearCastWheelCollideHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpVehicleLinearCastWheelCollideHkParam<'a>>
}

impl HkpVehicleLinearCastWheelCollide<'_> {
    /// Return `"hkpVehicleLinearCastWheelCollide"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpVehicleLinearCastWheelCollide".into()
    }

    /// Return `"0xc59399d0"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xc59399d0".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleLinearCastWheelCollideHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"wheelCollisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelCollisionFilterInfo")]
    WheelCollisionFilterInfo(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"wheelStates"`
    /// -   type: `hkArray&lt;struct hkpVehicleLinearCastWheelCollideWheelState&gt;`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelStates")]
    WheelStates(Vec<HkpVehicleLinearCastWheelCollideWheelState>),
    /// # Field information in the original C++ class
    /// -   name:`"rejectChassisListener"`
    /// -   type: `struct hkpRejectChassisListener`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rejectChassisListener")]
    RejectChassisListener(HkpRejectChassisListener),
    /// # Field information in the original C++ class
    /// -   name:`"maxExtraPenetration"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxExtraPenetration")]
    MaxExtraPenetration(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"startPointTolerance"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "startPointTolerance")]
    StartPointTolerance(Primitive<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleLinearCastWheelCollideHkParam<'de>, "@name",
    ("wheelCollisionFilterInfo" => WheelCollisionFilterInfo(Primitive<u32>)),
    ("wheelStates" => WheelStates(Vec<HkpVehicleLinearCastWheelCollideWheelState>)),
    ("rejectChassisListener" => RejectChassisListener(HkpRejectChassisListener)),
    ("maxExtraPenetration" => MaxExtraPenetration(Primitive<f32>)),
    ("startPointTolerance" => StartPointTolerance(Primitive<f32>)),
}
