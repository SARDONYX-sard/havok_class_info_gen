//! A Rust structure that implements a serializer/deserializer corresponding to `hkpVehicleDataWheelComponentParams`, a class defined in C++
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
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpVehicleDataWheelComponentParams<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpVehicleDataWheelComponentParams"`: Name of this class.
    #[serde(default = "HkpVehicleDataWheelComponentParams::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x82fe40e0`: Unique value of this class.
    #[serde(default = "HkpVehicleDataWheelComponentParams::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpVehicleDataWheelComponentParamsHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpVehicleDataWheelComponentParamsHkParam<'a>>
}

impl HkpVehicleDataWheelComponentParams<'_> {
    /// Return `"hkpVehicleDataWheelComponentParams"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpVehicleDataWheelComponentParams".into()
    }

    /// Return `"0x82fe40e0"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x82fe40e0".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleDataWheelComponentParamsHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"radius"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "radius")]
    Radius(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"mass"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mass")]
    Mass(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"width"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "width")]
    Width(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"friction"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "friction")]
    Friction(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"viscosityFriction"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "viscosityFriction")]
    ViscosityFriction(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxFriction"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxFriction")]
    MaxFriction(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"slipAngle"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "slipAngle")]
    SlipAngle(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"forceFeedbackMultiplier"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "forceFeedbackMultiplier")]
    ForceFeedbackMultiplier(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxContactBodyAcceleration"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxContactBodyAcceleration")]
    MaxContactBodyAcceleration(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"axle"`
    /// -   type: `hkInt8`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "axle")]
    Axle(i8),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDataWheelComponentParamsHkParam<'de>, "@name",
    ("radius" => Radius(f64)),
    ("mass" => Mass(f64)),
    ("width" => Width(f64)),
    ("friction" => Friction(f64)),
    ("viscosityFriction" => ViscosityFriction(f64)),
    ("maxFriction" => MaxFriction(f64)),
    ("slipAngle" => SlipAngle(f64)),
    ("forceFeedbackMultiplier" => ForceFeedbackMultiplier(f64)),
    ("maxContactBodyAcceleration" => MaxContactBodyAcceleration(f64)),
    ("axle" => Axle(i8)),
}
