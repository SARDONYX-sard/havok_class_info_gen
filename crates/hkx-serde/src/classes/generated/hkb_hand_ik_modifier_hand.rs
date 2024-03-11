//! A Rust structure that implements a serializer/deserializer corresponding to `hkbHandIkModifierHand`, a class defined in C++
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
/// -    size: 96
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 3
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbHandIkModifierHand<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbHandIkModifierHand"`: The original C++ class name.
    #[serde(default = "HkbHandIkModifierHand::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x14dfe1dd`: Unique value of this class.
    #[serde(default = "HkbHandIkModifierHand::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbHandIkModifierHandHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbHandIkModifierHandHkParam<'a>>
}

impl HkbHandIkModifierHand<'_> {
    /// Return `"hkbHandIkModifierHand"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbHandIkModifierHand".into()
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
pub enum HkbHandIkModifierHandHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"elbowAxisLS"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "elbowAxisLS")]
    ElbowAxisLs(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"backHandNormalLS"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "backHandNormalLS")]
    BackHandNormalLs(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"handOffsetLS"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handOffsetLS")]
    HandOffsetLs(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"handOrienationOffsetLS"`
    /// -   type: `hkQuaternion`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handOrienationOffsetLS")]
    HandOrienationOffsetLs(Quaternion<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxElbowAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxElbowAngleDegrees")]
    MaxElbowAngleDegrees(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"minElbowAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minElbowAngleDegrees")]
    MinElbowAngleDegrees(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"shoulderIndex"`
    /// -   type: `hkInt16`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shoulderIndex")]
    ShoulderIndex(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"shoulderSiblingIndex"`
    /// -   type: `hkInt16`
    /// - offset: 74
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shoulderSiblingIndex")]
    ShoulderSiblingIndex(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"elbowIndex"`
    /// -   type: `hkInt16`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "elbowIndex")]
    ElbowIndex(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"elbowSiblingIndex"`
    /// -   type: `hkInt16`
    /// - offset: 78
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "elbowSiblingIndex")]
    ElbowSiblingIndex(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"wristIndex"`
    /// -   type: `hkInt16`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wristIndex")]
    WristIndex(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"enforceEndPosition"`
    /// -   type: `hkBool`
    /// - offset: 82
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enforceEndPosition")]
    EnforceEndPosition(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"enforceEndRotation"`
    /// -   type: `hkBool`
    /// - offset: 83
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enforceEndRotation")]
    EnforceEndRotation(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"localFrameName"`
    /// -   type: `hkStringPtr`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "localFrameName")]
    LocalFrameName(Primitive<Cow<'a, str>>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbHandIkModifierHandHkParam<'de>, "@name",
    ("elbowAxisLS" => ElbowAxisLs(Vector4<f32>)),
    ("backHandNormalLS" => BackHandNormalLs(Vector4<f32>)),
    ("handOffsetLS" => HandOffsetLs(Vector4<f32>)),
    ("handOrienationOffsetLS" => HandOrienationOffsetLs(Quaternion<f32>)),
    ("maxElbowAngleDegrees" => MaxElbowAngleDegrees(Primitive<f32>)),
    ("minElbowAngleDegrees" => MinElbowAngleDegrees(Primitive<f32>)),
    ("shoulderIndex" => ShoulderIndex(Primitive<i16>)),
    ("shoulderSiblingIndex" => ShoulderSiblingIndex(Primitive<i16>)),
    ("elbowIndex" => ElbowIndex(Primitive<i16>)),
    ("elbowSiblingIndex" => ElbowSiblingIndex(Primitive<i16>)),
    ("wristIndex" => WristIndex(Primitive<i16>)),
    ("enforceEndPosition" => EnforceEndPosition(Primitive<bool>)),
    ("enforceEndRotation" => EnforceEndRotation(Primitive<bool>)),
    ("localFrameName" => LocalFrameName(Primitive<Cow<'a, str>>)),
}
