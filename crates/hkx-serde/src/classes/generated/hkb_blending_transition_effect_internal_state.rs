//! A Rust structure that implements a serializer/deserializer corresponding to `hkbBlendingTransitionEffectInternalState`, a class defined in C++
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
/// -    size: 32
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbBlendingTransitionEffectInternalState<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbBlendingTransitionEffectInternalState"`: Name of this class.
    #[serde(default = "HkbBlendingTransitionEffectInternalState::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xb18c70c2`: Unique value of this class.
    #[serde(default = "HkbBlendingTransitionEffectInternalState::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbBlendingTransitionEffectInternalStateHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbBlendingTransitionEffectInternalStateHkParam<'a>>
}

impl HkbBlendingTransitionEffectInternalState<'_> {
    /// Return `"hkbBlendingTransitionEffectInternalState"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbBlendingTransitionEffectInternalState".into()
    }

    /// Return `"0xb18c70c2"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xb18c70c2".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbBlendingTransitionEffectInternalStateHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"characterPoseAtBeginningOfTransition"`
    /// -   type: `hkArray&lt;hkQsTransform&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterPoseAtBeginningOfTransition")]
    CharacterPoseAtBeginningOfTransition(Vec<cgmath::Matrix4<f32>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"timeRemaining"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "timeRemaining")]
    TimeRemaining(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"timeInTransition"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "timeInTransition")]
    TimeInTransition(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"applySelfTransition"`
    /// -   type: `hkBool`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "applySelfTransition")]
    ApplySelfTransition(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"initializeCharacterPose"`
    /// -   type: `hkBool`
    /// - offset: 29
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "initializeCharacterPose")]
    InitializeCharacterPose(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbBlendingTransitionEffectInternalStateHkParam<'de>, "@name",
    ("characterPoseAtBeginningOfTransition" => CharacterPoseAtBeginningOfTransition(Vec<cgmath::Matrix4<f32>>)),
    ("timeRemaining" => TimeRemaining(f64)),
    ("timeInTransition" => TimeInTransition(f64)),
    ("applySelfTransition" => ApplySelfTransition(bool)),
    ("initializeCharacterPose" => InitializeCharacterPose(bool)),
}
