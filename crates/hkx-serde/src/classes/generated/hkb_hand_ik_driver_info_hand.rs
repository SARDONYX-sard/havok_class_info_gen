//! A Rust structure that implements a serializer/deserializer corresponding to `hkbHandIkDriverInfoHand`, a class defined in C++
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
/// -    size: 96
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbHandIkDriverInfoHand<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbHandIkDriverInfoHand"`: Name of this class.
    #[serde(default = "HkbHandIkDriverInfoHand::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x14dfe1dd`: Unique value of this class.
    #[serde(default = "HkbHandIkDriverInfoHand::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbHandIkDriverInfoHandHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbHandIkDriverInfoHandHkParam<'a>>
}

impl HkbHandIkDriverInfoHand<'_> {
    /// Return `"hkbHandIkDriverInfoHand"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbHandIkDriverInfoHand".into()
    }

    /// Return `"0x14dfe1dd"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x14dfe1dd".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbHandIkDriverInfoHandHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"elbowAxisLS"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "elbowAxisLS")]
    ElbowAxisLs(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"backHandNormalLS"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "backHandNormalLS")]
    BackHandNormalLs(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"handOffsetLS"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handOffsetLS")]
    HandOffsetLs(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"handOrienationOffsetLS"`
    /// -   type: `hkQuaternion`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handOrienationOffsetLS")]
    HandOrienationOffsetLs(cgmath::Quaternion<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxElbowAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxElbowAngleDegrees")]
    MaxElbowAngleDegrees(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"minElbowAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minElbowAngleDegrees")]
    MinElbowAngleDegrees(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"shoulderIndex"`
    /// -   type: `hkInt16`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shoulderIndex")]
    ShoulderIndex(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"shoulderSiblingIndex"`
    /// -   type: `hkInt16`
    /// - offset: 74
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shoulderSiblingIndex")]
    ShoulderSiblingIndex(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"elbowIndex"`
    /// -   type: `hkInt16`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "elbowIndex")]
    ElbowIndex(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"elbowSiblingIndex"`
    /// -   type: `hkInt16`
    /// - offset: 78
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "elbowSiblingIndex")]
    ElbowSiblingIndex(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"wristIndex"`
    /// -   type: `hkInt16`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wristIndex")]
    WristIndex(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"enforceEndPosition"`
    /// -   type: `hkBool`
    /// - offset: 82
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enforceEndPosition")]
    EnforceEndPosition(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"enforceEndRotation"`
    /// -   type: `hkBool`
    /// - offset: 83
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enforceEndRotation")]
    EnforceEndRotation(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"localFrameName"`
    /// -   type: `hkStringPtr`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "localFrameName")]
    LocalFrameName(String),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbHandIkDriverInfoHandHkParam<'de>, "@name",
    ("elbowAxisLS" => ElbowAxisLs(cgmath::Vector4<f32>)),
    ("backHandNormalLS" => BackHandNormalLs(cgmath::Vector4<f32>)),
    ("handOffsetLS" => HandOffsetLs(cgmath::Vector4<f32>)),
    ("handOrienationOffsetLS" => HandOrienationOffsetLs(cgmath::Quaternion<f32>)),
    ("maxElbowAngleDegrees" => MaxElbowAngleDegrees(f64)),
    ("minElbowAngleDegrees" => MinElbowAngleDegrees(f64)),
    ("shoulderIndex" => ShoulderIndex(i16)),
    ("shoulderSiblingIndex" => ShoulderSiblingIndex(i16)),
    ("elbowIndex" => ElbowIndex(i16)),
    ("elbowSiblingIndex" => ElbowSiblingIndex(i16)),
    ("wristIndex" => WristIndex(i16)),
    ("enforceEndPosition" => EnforceEndPosition(bool)),
    ("enforceEndRotation" => EnforceEndRotation(bool)),
    ("localFrameName" => LocalFrameName(String)),
}