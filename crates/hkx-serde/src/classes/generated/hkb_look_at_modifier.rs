//! A Rust structure that implements a serializer/deserializer corresponding to `hkbLookAtModifier`, a class defined in C++
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
/// -    size: 208
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 3
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbLookAtModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbLookAtModifier"`: The original C++ class name.
    #[serde(default = "HkbLookAtModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x3d28e066`: Unique value of this class.
    #[serde(default = "HkbLookAtModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbLookAtModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbLookAtModifierHkParam<'a>>
}

impl HkbLookAtModifier<'_> {
    /// Return `"hkbLookAtModifier"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbLookAtModifier".into()
    }

    /// Return `"0x3d28e066"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x3d28e066".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbLookAtModifierHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"targetWS"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "targetWS")]
    TargetWs(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"headForwardLS"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "headForwardLS")]
    HeadForwardLs(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"neckForwardLS"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "neckForwardLS")]
    NeckForwardLs(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"neckRightLS"`
    /// -   type: `hkVector4`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "neckRightLS")]
    NeckRightLs(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"eyePositionHS"`
    /// -   type: `hkVector4`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eyePositionHS")]
    EyePositionHs(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"newTargetGain"`
    /// -   type: `hkReal`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "newTargetGain")]
    NewTargetGain(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"onGain"`
    /// -   type: `hkReal`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "onGain")]
    OnGain(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"offGain"`
    /// -   type: `hkReal`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "offGain")]
    OffGain(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"limitAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "limitAngleDegrees")]
    LimitAngleDegrees(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"limitAngleLeft"`
    /// -   type: `hkReal`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "limitAngleLeft")]
    LimitAngleLeft(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"limitAngleRight"`
    /// -   type: `hkReal`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "limitAngleRight")]
    LimitAngleRight(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"limitAngleUp"`
    /// -   type: `hkReal`
    /// - offset: 152
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "limitAngleUp")]
    LimitAngleUp(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"limitAngleDown"`
    /// -   type: `hkReal`
    /// - offset: 156
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "limitAngleDown")]
    LimitAngleDown(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"headIndex"`
    /// -   type: `hkInt16`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "headIndex")]
    HeadIndex(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"neckIndex"`
    /// -   type: `hkInt16`
    /// - offset: 162
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "neckIndex")]
    NeckIndex(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"isOn"`
    /// -   type: `hkBool`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isOn")]
    IsOn(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"individualLimitsOn"`
    /// -   type: `hkBool`
    /// - offset: 165
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "individualLimitsOn")]
    IndividualLimitsOn(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"isTargetInsideLimitCone"`
    /// -   type: `hkBool`
    /// - offset: 166
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isTargetInsideLimitCone")]
    IsTargetInsideLimitCone(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"lookAtLastTargetWS"`
    /// -   type: `hkVector4`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "lookAtLastTargetWS", skip_serializing)]
    LookAtLastTargetWs(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"lookAtWeight"`
    /// -   type: `hkReal`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "lookAtWeight", skip_serializing)]
    LookAtWeight(Primitive<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbLookAtModifierHkParam<'de>, "@name",
    ("targetWS" => TargetWs(Vector4<f32>)),
    ("headForwardLS" => HeadForwardLs(Vector4<f32>)),
    ("neckForwardLS" => NeckForwardLs(Vector4<f32>)),
    ("neckRightLS" => NeckRightLs(Vector4<f32>)),
    ("eyePositionHS" => EyePositionHs(Vector4<f32>)),
    ("newTargetGain" => NewTargetGain(Primitive<f32>)),
    ("onGain" => OnGain(Primitive<f32>)),
    ("offGain" => OffGain(Primitive<f32>)),
    ("limitAngleDegrees" => LimitAngleDegrees(Primitive<f32>)),
    ("limitAngleLeft" => LimitAngleLeft(Primitive<f32>)),
    ("limitAngleRight" => LimitAngleRight(Primitive<f32>)),
    ("limitAngleUp" => LimitAngleUp(Primitive<f32>)),
    ("limitAngleDown" => LimitAngleDown(Primitive<f32>)),
    ("headIndex" => HeadIndex(Primitive<i16>)),
    ("neckIndex" => NeckIndex(Primitive<i16>)),
    ("isOn" => IsOn(Primitive<bool>)),
    ("individualLimitsOn" => IndividualLimitsOn(Primitive<bool>)),
    ("isTargetInsideLimitCone" => IsTargetInsideLimitCone(Primitive<bool>)),
    ("lookAtLastTargetWS" => LookAtLastTargetWs(Vector4<f32>)),
    ("lookAtWeight" => LookAtWeight(Primitive<f32>)),
}
