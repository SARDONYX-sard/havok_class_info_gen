//! A Rust structure that implements a serializer/deserializer corresponding to `hkbTransitionEffect`, a class defined in C++
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
/// -    size: 44
/// -  vtable: true
/// -  parent: hkbGenerator/`d68aefc`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbTransitionEffect<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbTransitionEffect"`: The original C++ class name.
    #[serde(default = "HkbTransitionEffect::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x945da157`: Unique value of this class.
    #[serde(default = "HkbTransitionEffect::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbTransitionEffectHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbTransitionEffectHkParam<'a>>
}

impl HkbTransitionEffect<'_> {
    /// Return `"hkbTransitionEffect"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbTransitionEffect".into()
    }

    /// Return `"0x945da157"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x945da157".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbTransitionEffectHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"selfTransitionMode"`
    /// -   type: `enum SelfTransitionMode`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "selfTransitionMode")]
    SelfTransitionMode(SelfTransitionMode),
    /// # Field information in the original C++ class
    /// -   name:`"eventMode"`
    /// -   type: `enum EventMode`
    /// - offset: 41
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eventMode")]
    EventMode(EventMode),
    /// # Field information in the original C++ class
    /// -   name:`"defaultEventMode"`
    /// -   type: `enum unknown`
    /// - offset: 42
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "defaultEventMode", skip_serializing)]
    DefaultEventMode(Unknown),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbTransitionEffectHkParam<'de>, "@name",
    ("selfTransitionMode" => SelfTransitionMode(SelfTransitionMode)),
    ("eventMode" => EventMode(EventMode)),
    ("defaultEventMode" => DefaultEventMode(Unknown)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SelfTransitionMode {
    #[serde(rename = "SELF_TRANSITION_MODE_CONTINUE_IF_CYCLIC_BLEND_IF_ACYCLIC")]
    SelfTransitionModeContinueIfCyclicBlendIfAcyclic = 0,
    #[serde(rename = "SELF_TRANSITION_MODE_CONTINUE")]
    SelfTransitionModeContinue = 1,
    #[serde(rename = "SELF_TRANSITION_MODE_RESET")]
    SelfTransitionModeReset = 2,
    #[serde(rename = "SELF_TRANSITION_MODE_BLEND")]
    SelfTransitionModeBlend = 3,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EventMode {
    #[serde(rename = "EVENT_MODE_DEFAULT")]
    EventModeDefault = 0,
    #[serde(rename = "EVENT_MODE_PROCESS_ALL")]
    EventModeProcessAll = 1,
    #[serde(rename = "EVENT_MODE_IGNORE_FROM_GENERATOR")]
    EventModeIgnoreFromGenerator = 2,
    #[serde(rename = "EVENT_MODE_IGNORE_TO_GENERATOR")]
    EventModeIgnoreToGenerator = 3,
}
