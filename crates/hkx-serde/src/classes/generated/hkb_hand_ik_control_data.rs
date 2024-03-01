//! A Rust structure that implements a serializer/deserializer corresponding to `hkbHandIkControlData`, a class defined in C++
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
/// -    size: 80
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 2
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbHandIkControlData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbHandIkControlData"`: Name of this class.
    #[serde(default = "HkbHandIkControlData::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xd72b8d17`: Unique value of this class.
    #[serde(default = "HkbHandIkControlData::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbHandIkControlDataHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbHandIkControlDataHkParam<'a>>
}

impl HkbHandIkControlData<'_> {
    /// Return `"hkbHandIkControlData"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbHandIkControlData".into()
    }

    /// Return `"0xd72b8d17"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xd72b8d17".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbHandIkControlDataHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"targetPosition"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "targetPosition")]
    TargetPosition(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"targetRotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "targetRotation")]
    TargetRotation(cgmath::Quaternion<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"targetNormal"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "targetNormal")]
    TargetNormal(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"targetHandle"`
    /// -   type: `struct hkbHandle*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "targetHandle")]
    TargetHandle(Box<HkbHandle>),
    /// # Information on fields in the original C++ class
    /// -   name:`"transformOnFraction"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformOnFraction")]
    TransformOnFraction(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"normalOnFraction"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "normalOnFraction")]
    NormalOnFraction(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"fadeInDuration"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fadeInDuration")]
    FadeInDuration(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"fadeOutDuration"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fadeOutDuration")]
    FadeOutDuration(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"extrapolationTimeStep"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extrapolationTimeStep")]
    ExtrapolationTimeStep(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"handleChangeSpeed"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handleChangeSpeed")]
    HandleChangeSpeed(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"handleChangeMode"`
    /// -   type: `enum HandleChangeMode`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handleChangeMode")]
    HandleChangeMode(HandleChangeMode),
    /// # Information on fields in the original C++ class
    /// -   name:`"fixUp"`
    /// -   type: `hkBool`
    /// - offset: 77
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fixUp")]
    FixUp(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbHandIkControlDataHkParam<'de>, "@name",
    ("targetPosition" => TargetPosition(cgmath::Vector4<f32>)),
    ("targetRotation" => TargetRotation(cgmath::Quaternion<f32>)),
    ("targetNormal" => TargetNormal(cgmath::Vector4<f32>)),
    ("targetHandle" => TargetHandle(Box<HkbHandle>)),
    ("transformOnFraction" => TransformOnFraction(f64)),
    ("normalOnFraction" => NormalOnFraction(f64)),
    ("fadeInDuration" => FadeInDuration(f64)),
    ("fadeOutDuration" => FadeOutDuration(f64)),
    ("extrapolationTimeStep" => ExtrapolationTimeStep(f64)),
    ("handleChangeSpeed" => HandleChangeSpeed(f64)),
    ("handleChangeMode" => HandleChangeMode(HandleChangeMode)),
    ("fixUp" => FixUp(bool)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum HandleChangeMode {
    #[serde(rename = "HANDLE_CHANGE_MODE_ABRUPT")]
    HandleChangeModeAbrupt = 0,
    #[serde(rename = "HANDLE_CHANGE_MODE_CONSTANT_VELOCITY")]
    HandleChangeModeConstantVelocity = 1,
}
