//! A Rust structure that implements a serializer/deserializer corresponding to `hkbGeneratorTransitionEffectInternalState`, a class defined in C++
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
pub struct HkbGeneratorTransitionEffectInternalState<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbGeneratorTransitionEffectInternalState"`: Name of this class.
    #[serde(default = "HkbGeneratorTransitionEffectInternalState::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xd6692b5d`: Unique value of this class.
    #[serde(default = "HkbGeneratorTransitionEffectInternalState::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbGeneratorTransitionEffectInternalStateHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbGeneratorTransitionEffectInternalStateHkParam<'a>>
}

impl HkbGeneratorTransitionEffectInternalState<'_> {
    /// Return `"hkbGeneratorTransitionEffectInternalState"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbGeneratorTransitionEffectInternalState".into()
    }

    /// Return `"0xd6692b5d"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xd6692b5d".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbGeneratorTransitionEffectInternalStateHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"timeInTransition"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "timeInTransition")]
    TimeInTransition(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"duration"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "duration")]
    Duration(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"effectiveBlendInDuration"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "effectiveBlendInDuration")]
    EffectiveBlendInDuration(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"effectiveBlendOutDuration"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "effectiveBlendOutDuration")]
    EffectiveBlendOutDuration(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"toGeneratorState"`
    /// -   type: `enum ToGeneratorState`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "toGeneratorState")]
    ToGeneratorState(ToGeneratorState),
    /// # Information on fields in the original C++ class
    /// -   name:`"echoTransitionGenerator"`
    /// -   type: `hkBool`
    /// - offset: 25
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "echoTransitionGenerator")]
    EchoTransitionGenerator(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"echoToGenerator"`
    /// -   type: `hkBool`
    /// - offset: 26
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "echoToGenerator")]
    EchoToGenerator(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"justActivated"`
    /// -   type: `hkBool`
    /// - offset: 27
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "justActivated")]
    JustActivated(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"updateActiveNodes"`
    /// -   type: `hkBool`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "updateActiveNodes")]
    UpdateActiveNodes(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"stage"`
    /// -   type: `enum Stage`
    /// - offset: 29
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "stage")]
    Stage(Stage),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbGeneratorTransitionEffectInternalStateHkParam<'de>, "@name",
    ("timeInTransition" => TimeInTransition(f64)),
    ("duration" => Duration(f64)),
    ("effectiveBlendInDuration" => EffectiveBlendInDuration(f64)),
    ("effectiveBlendOutDuration" => EffectiveBlendOutDuration(f64)),
    ("toGeneratorState" => ToGeneratorState(ToGeneratorState)),
    ("echoTransitionGenerator" => EchoTransitionGenerator(bool)),
    ("echoToGenerator" => EchoToGenerator(bool)),
    ("justActivated" => JustActivated(bool)),
    ("updateActiveNodes" => UpdateActiveNodes(bool)),
    ("stage" => Stage(Stage)),
}
