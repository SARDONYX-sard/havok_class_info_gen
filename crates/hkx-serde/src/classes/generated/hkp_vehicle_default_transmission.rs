//! A Rust structure that implements a serializer/deserializer corresponding to `hkpVehicleDefaultTransmission`, a class defined in C++
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
/// -  parent: hkpVehicleTransmission/`da8c7d7d`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpVehicleDefaultTransmission<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpVehicleDefaultTransmission"`: The original C++ class name.
    #[serde(default = "HkpVehicleDefaultTransmission::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x235d5d6b`: Unique value of this class.
    #[serde(default = "HkpVehicleDefaultTransmission::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpVehicleDefaultTransmissionHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpVehicleDefaultTransmissionHkParam<'a>>
}

impl HkpVehicleDefaultTransmission<'_> {
    /// Return `"hkpVehicleDefaultTransmission"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpVehicleDefaultTransmission".into()
    }

    /// Return `"0x235d5d6b"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x235d5d6b".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleDefaultTransmissionHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"downshiftRPM"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "downshiftRPM")]
    DownshiftRpm(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"upshiftRPM"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "upshiftRPM")]
    UpshiftRpm(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"primaryTransmissionRatio"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "primaryTransmissionRatio")]
    PrimaryTransmissionRatio(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"clutchDelayTime"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "clutchDelayTime")]
    ClutchDelayTime(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"reverseGearRatio"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "reverseGearRatio")]
    ReverseGearRatio(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"gearsRatio"`
    /// -   type: `hkArray&lt;hkReal&gt;`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "gearsRatio")]
    GearsRatio(Vec<Primitive<f32>>),
    /// # Field information in the original C++ class
    /// -   name:`"wheelsTorqueRatio"`
    /// -   type: `hkArray&lt;hkReal&gt;`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelsTorqueRatio")]
    WheelsTorqueRatio(Vec<Primitive<f32>>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDefaultTransmissionHkParam<'de>, "@name",
    ("downshiftRPM" => DownshiftRpm(Primitive<f32>)),
    ("upshiftRPM" => UpshiftRpm(Primitive<f32>)),
    ("primaryTransmissionRatio" => PrimaryTransmissionRatio(Primitive<f32>)),
    ("clutchDelayTime" => ClutchDelayTime(Primitive<f32>)),
    ("reverseGearRatio" => ReverseGearRatio(Primitive<f32>)),
    ("gearsRatio" => GearsRatio(Vec<Primitive<f32>>)),
    ("wheelsTorqueRatio" => WheelsTorqueRatio(Vec<Primitive<f32>>)),
}
