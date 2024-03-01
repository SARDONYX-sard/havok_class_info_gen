//! A Rust structure that implements a serializer/deserializer corresponding to `hkbBlendingTransitionEffect`, a class defined in C++
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
/// -    size: 88
/// -  vtable: true
/// -  parent: hkbTransitionEffect/`945da157`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbBlendingTransitionEffect<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbBlendingTransitionEffect"`: Name of this class.
    #[serde(default = "HkbBlendingTransitionEffect::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xfd8584fe`: Unique value of this class.
    #[serde(default = "HkbBlendingTransitionEffect::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbBlendingTransitionEffectHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbBlendingTransitionEffectHkParam<'a>>
}

impl HkbBlendingTransitionEffect<'_> {
    /// Return `"hkbBlendingTransitionEffect"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbBlendingTransitionEffect".into()
    }

    /// Return `"0xfd8584fe"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xfd8584fe".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbBlendingTransitionEffectHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"duration"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "duration")]
    Duration(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"toGeneratorStartTimeFraction"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "toGeneratorStartTimeFraction")]
    ToGeneratorStartTimeFraction(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"flags"`
    /// -   type: `flags FlagBits`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "flags")]
    Flags(FlagBits),
    /// # Information on fields in the original C++ class
    /// -   name:`"endMode"`
    /// -   type: `enum EndMode`
    /// - offset: 54
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "endMode")]
    EndMode(EndMode),
    /// # Information on fields in the original C++ class
    /// -   name:`"blendCurve"`
    /// -   type: `enum BlendCurve`
    /// - offset: 55
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blendCurve")]
    BlendCurve(BlendCurve),
    /// # Information on fields in the original C++ class
    /// -   name:`"fromGenerator"`
    /// -   type: `void*`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "fromGenerator", skip_serializing)]
    FromGenerator(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"toGenerator"`
    /// -   type: `void*`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "toGenerator", skip_serializing)]
    ToGenerator(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"characterPoseAtBeginningOfTransition"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "characterPoseAtBeginningOfTransition", skip_serializing)]
    CharacterPoseAtBeginningOfTransition(Vec<()>),
    /// # Information on fields in the original C++ class
    /// -   name:`"timeRemaining"`
    /// -   type: `hkReal`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "timeRemaining", skip_serializing)]
    TimeRemaining(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"timeInTransition"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "timeInTransition", skip_serializing)]
    TimeInTransition(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"applySelfTransition"`
    /// -   type: `hkBool`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "applySelfTransition", skip_serializing)]
    ApplySelfTransition(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"initializeCharacterPose"`
    /// -   type: `hkBool`
    /// - offset: 85
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "initializeCharacterPose", skip_serializing)]
    InitializeCharacterPose(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbBlendingTransitionEffectHkParam<'de>, "@name",
    ("duration" => Duration(f64)),
    ("toGeneratorStartTimeFraction" => ToGeneratorStartTimeFraction(f64)),
    ("flags" => Flags(FlagBits)),
    ("endMode" => EndMode(EndMode)),
    ("blendCurve" => BlendCurve(BlendCurve)),
    ("fromGenerator" => FromGenerator(())),
    ("toGenerator" => ToGenerator(())),
    ("characterPoseAtBeginningOfTransition" => CharacterPoseAtBeginningOfTransition(Vec<()>)),
    ("timeRemaining" => TimeRemaining(f64)),
    ("timeInTransition" => TimeInTransition(f64)),
    ("applySelfTransition" => ApplySelfTransition(bool)),
    ("initializeCharacterPose" => InitializeCharacterPose(bool)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FlagBits {
    #[serde(rename = "FLAG_NONE")]
    FlagNone = 0,
    #[serde(rename = "FLAG_IGNORE_FROM_WORLD_FROM_MODEL")]
    FlagIgnoreFromWorldFromModel = 1,
    #[serde(rename = "FLAG_SYNC")]
    FlagSync = 2,
    #[serde(rename = "FLAG_IGNORE_TO_WORLD_FROM_MODEL")]
    FlagIgnoreToWorldFromModel = 4,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EndMode {
    #[serde(rename = "END_MODE_NONE")]
    EndModeNone = 0,
    #[serde(rename = "END_MODE_TRANSITION_UNTIL_END_OF_FROM_GENERATOR")]
    EndModeTransitionUntilEndOfFromGenerator = 1,
    #[serde(rename = "END_MODE_CAP_DURATION_AT_END_OF_FROM_GENERATOR")]
    EndModeCapDurationAtEndOfFromGenerator = 2,
}
