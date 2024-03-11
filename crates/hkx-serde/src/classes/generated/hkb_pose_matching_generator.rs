//! A Rust structure that implements a serializer/deserializer corresponding to `hkbPoseMatchingGenerator`, a class defined in C++
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
/// -  parent: hkbBlenderGenerator/`22df7147`(Non prefix hex signature)
/// - version: 2
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbPoseMatchingGenerator<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbPoseMatchingGenerator"`: The original C++ class name.
    #[serde(default = "HkbPoseMatchingGenerator::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x29e271b4`: Unique value of this class.
    #[serde(default = "HkbPoseMatchingGenerator::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbPoseMatchingGeneratorHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbPoseMatchingGeneratorHkParam<'a>>
}

impl HkbPoseMatchingGenerator<'_> {
    /// Return `"hkbPoseMatchingGenerator"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbPoseMatchingGenerator".into()
    }

    /// Return `"0x29e271b4"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x29e271b4".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbPoseMatchingGeneratorHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"worldFromModelRotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "worldFromModelRotation")]
    WorldFromModelRotation(Quaternion<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"blendSpeed"`
    /// -   type: `hkReal`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blendSpeed")]
    BlendSpeed(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"minSpeedToSwitch"`
    /// -   type: `hkReal`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minSpeedToSwitch")]
    MinSpeedToSwitch(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"minSwitchTimeNoError"`
    /// -   type: `hkReal`
    /// - offset: 152
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minSwitchTimeNoError")]
    MinSwitchTimeNoError(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"minSwitchTimeFullError"`
    /// -   type: `hkReal`
    /// - offset: 156
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minSwitchTimeFullError")]
    MinSwitchTimeFullError(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"startPlayingEventId"`
    /// -   type: `hkInt32`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "startPlayingEventId")]
    StartPlayingEventId(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"startMatchingEventId"`
    /// -   type: `hkInt32`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "startMatchingEventId")]
    StartMatchingEventId(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"rootBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rootBoneIndex")]
    RootBoneIndex(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"otherBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 170
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "otherBoneIndex")]
    OtherBoneIndex(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"anotherBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "anotherBoneIndex")]
    AnotherBoneIndex(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"pelvisIndex"`
    /// -   type: `hkInt16`
    /// - offset: 174
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pelvisIndex")]
    PelvisIndex(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"mode"`
    /// -   type: `enum Mode`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mode")]
    Mode(Mode),
    /// # Field information in the original C++ class
    /// -   name:`"currentMatch"`
    /// -   type: `hkInt32`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "currentMatch", skip_serializing)]
    CurrentMatch(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"bestMatch"`
    /// -   type: `hkInt32`
    /// - offset: 184
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "bestMatch", skip_serializing)]
    BestMatch(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"timeSinceBetterMatch"`
    /// -   type: `hkReal`
    /// - offset: 188
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "timeSinceBetterMatch", skip_serializing)]
    TimeSinceBetterMatch(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"error"`
    /// -   type: `hkReal`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "error", skip_serializing)]
    Error(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"resetCurrentMatchLocalTime"`
    /// -   type: `hkBool`
    /// - offset: 196
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "resetCurrentMatchLocalTime", skip_serializing)]
    ResetCurrentMatchLocalTime(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"poseMatchingUtility"`
    /// -   type: `void*`
    /// - offset: 200
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "poseMatchingUtility", skip_serializing)]
    PoseMatchingUtility(()),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbPoseMatchingGeneratorHkParam<'de>, "@name",
    ("worldFromModelRotation" => WorldFromModelRotation(Quaternion<f32>)),
    ("blendSpeed" => BlendSpeed(Primitive<f32>)),
    ("minSpeedToSwitch" => MinSpeedToSwitch(Primitive<f32>)),
    ("minSwitchTimeNoError" => MinSwitchTimeNoError(Primitive<f32>)),
    ("minSwitchTimeFullError" => MinSwitchTimeFullError(Primitive<f32>)),
    ("startPlayingEventId" => StartPlayingEventId(Primitive<i32>)),
    ("startMatchingEventId" => StartMatchingEventId(Primitive<i32>)),
    ("rootBoneIndex" => RootBoneIndex(Primitive<i16>)),
    ("otherBoneIndex" => OtherBoneIndex(Primitive<i16>)),
    ("anotherBoneIndex" => AnotherBoneIndex(Primitive<i16>)),
    ("pelvisIndex" => PelvisIndex(Primitive<i16>)),
    ("mode" => Mode(Mode)),
    ("currentMatch" => CurrentMatch(Primitive<i32>)),
    ("bestMatch" => BestMatch(Primitive<i32>)),
    ("timeSinceBetterMatch" => TimeSinceBetterMatch(Primitive<f32>)),
    ("error" => Error(Primitive<f32>)),
    ("resetCurrentMatchLocalTime" => ResetCurrentMatchLocalTime(Primitive<bool>)),
    ("poseMatchingUtility" => PoseMatchingUtility(())),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "MODE_MATCH")]
    ModeMatch = 0,
    #[serde(rename = "MODE_PLAY")]
    ModePlay = 1,
}
