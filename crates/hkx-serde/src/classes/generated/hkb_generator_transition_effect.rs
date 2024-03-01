//! A Rust structure that implements a serializer/deserializer corresponding to `hkbGeneratorTransitionEffect`, a class defined in C++
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
/// -    size: 92
/// -  vtable: true
/// -  parent: hkbTransitionEffect/`945da157`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbGeneratorTransitionEffect<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbGeneratorTransitionEffect"`: Name of this class.
    #[serde(default = "HkbGeneratorTransitionEffect::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x5f771b12`: Unique value of this class.
    #[serde(default = "HkbGeneratorTransitionEffect::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbGeneratorTransitionEffectHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbGeneratorTransitionEffectHkParam<'a>>
}

impl HkbGeneratorTransitionEffect<'_> {
    /// Return `"hkbGeneratorTransitionEffect"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbGeneratorTransitionEffect".into()
    }

    /// Return `"0x5f771b12"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x5f771b12".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbGeneratorTransitionEffectHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"transitionGenerator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transitionGenerator")]
    TransitionGenerator(Box<HkbGenerator>),
    /// # Information on fields in the original C++ class
    /// -   name:`"blendInDuration"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blendInDuration")]
    BlendInDuration(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"blendOutDuration"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blendOutDuration")]
    BlendOutDuration(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"syncToGeneratorStartTime"`
    /// -   type: `hkBool`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "syncToGeneratorStartTime")]
    SyncToGeneratorStartTime(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"fromGenerator"`
    /// -   type: `void*`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "fromGenerator", skip_serializing)]
    FromGenerator(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"toGenerator"`
    /// -   type: `void*`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "toGenerator", skip_serializing)]
    ToGenerator(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"timeInTransition"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "timeInTransition", skip_serializing)]
    TimeInTransition(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"duration"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "duration", skip_serializing)]
    Duration(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"effectiveBlendInDuration"`
    /// -   type: `hkReal`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "effectiveBlendInDuration", skip_serializing)]
    EffectiveBlendInDuration(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"effectiveBlendOutDuration"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "effectiveBlendOutDuration", skip_serializing)]
    EffectiveBlendOutDuration(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"toGeneratorState"`
    /// -   type: `enum unknown`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "toGeneratorState", skip_serializing)]
    ToGeneratorState(Unknown),
    /// # Information on fields in the original C++ class
    /// -   name:`"echoTransitionGenerator"`
    /// -   type: `hkBool`
    /// - offset: 85
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "echoTransitionGenerator", skip_serializing)]
    EchoTransitionGenerator(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"echoToGenerator"`
    /// -   type: `hkBool`
    /// - offset: 86
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "echoToGenerator", skip_serializing)]
    EchoToGenerator(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"justActivated"`
    /// -   type: `hkBool`
    /// - offset: 87
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "justActivated", skip_serializing)]
    JustActivated(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"updateActiveNodes"`
    /// -   type: `hkBool`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "updateActiveNodes", skip_serializing)]
    UpdateActiveNodes(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"stage"`
    /// -   type: `enum unknown`
    /// - offset: 89
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "stage", skip_serializing)]
    Stage(Unknown),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbGeneratorTransitionEffectHkParam<'de>, "@name",
    ("transitionGenerator" => TransitionGenerator(Box<HkbGenerator>)),
    ("blendInDuration" => BlendInDuration(f64)),
    ("blendOutDuration" => BlendOutDuration(f64)),
    ("syncToGeneratorStartTime" => SyncToGeneratorStartTime(bool)),
    ("fromGenerator" => FromGenerator(())),
    ("toGenerator" => ToGenerator(())),
    ("timeInTransition" => TimeInTransition(f64)),
    ("duration" => Duration(f64)),
    ("effectiveBlendInDuration" => EffectiveBlendInDuration(f64)),
    ("effectiveBlendOutDuration" => EffectiveBlendOutDuration(f64)),
    ("toGeneratorState" => ToGeneratorState(Unknown)),
    ("echoTransitionGenerator" => EchoTransitionGenerator(bool)),
    ("echoToGenerator" => EchoToGenerator(bool)),
    ("justActivated" => JustActivated(bool)),
    ("updateActiveNodes" => UpdateActiveNodes(bool)),
    ("stage" => Stage(Unknown)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ToGeneratorState {
    #[serde(rename = "STATE_INACTIVE")]
    StateInactive = 0,
    #[serde(rename = "STATE_READY_FOR_SET_LOCAL_TIME")]
    StateReadyForSetLocalTime = 1,
    #[serde(rename = "STATE_READY_FOR_APPLY_SELF_TRANSITION_MODE")]
    StateReadyForApplySelfTransitionMode = 2,
    #[serde(rename = "STATE_ACTIVE")]
    StateActive = 3,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Stage {
    #[serde(rename = "STAGE_BLENDING_IN")]
    StageBlendingIn = 0,
    #[serde(rename = "STAGE_PLAYING_TRANSITION_GENERATOR")]
    StagePlayingTransitionGenerator = 1,
    #[serde(rename = "STAGE_BLENDING_OUT")]
    StageBlendingOut = 2,
}
