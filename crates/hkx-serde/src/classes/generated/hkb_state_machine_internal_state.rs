//! A Rust structure that implements a serializer/deserializer corresponding to `hkbStateMachineInternalState`, a class defined in C++
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
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbStateMachineInternalState<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbStateMachineInternalState"`: Name of this class.
    #[serde(default = "HkbStateMachineInternalState::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xbd1a7502`: Unique value of this class.
    #[serde(default = "HkbStateMachineInternalState::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbStateMachineInternalStateHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbStateMachineInternalStateHkParam<'a>>
}

impl HkbStateMachineInternalState<'_> {
    /// Return `"hkbStateMachineInternalState"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbStateMachineInternalState".into()
    }

    /// Return `"0xbd1a7502"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xbd1a7502".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbStateMachineInternalStateHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"activeTransitions"`
    /// -   type: `hkArray&lt;struct hkbStateMachineActiveTransitionInfo&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "activeTransitions")]
    ActiveTransitions(Vec<HkbStateMachineActiveTransitionInfo>),
    /// # Information on fields in the original C++ class
    /// -   name:`"transitionFlags"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transitionFlags")]
    TransitionFlags(Vec<u8>),
    /// # Information on fields in the original C++ class
    /// -   name:`"wildcardTransitionFlags"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wildcardTransitionFlags")]
    WildcardTransitionFlags(Vec<u8>),
    /// # Information on fields in the original C++ class
    /// -   name:`"delayedTransitions"`
    /// -   type: `hkArray&lt;struct hkbStateMachineDelayedTransitionInfo&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "delayedTransitions")]
    DelayedTransitions(Vec<HkbStateMachineDelayedTransitionInfo>),
    /// # Information on fields in the original C++ class
    /// -   name:`"timeInState"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "timeInState")]
    TimeInState(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"lastLocalTime"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lastLocalTime")]
    LastLocalTime(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"currentStateId"`
    /// -   type: `hkInt32`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "currentStateId")]
    CurrentStateId(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"previousStateId"`
    /// -   type: `hkInt32`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "previousStateId")]
    PreviousStateId(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"nextStartStateIndexOverride"`
    /// -   type: `hkInt32`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nextStartStateIndexOverride")]
    NextStartStateIndexOverride(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"stateOrTransitionChanged"`
    /// -   type: `hkBool`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "stateOrTransitionChanged")]
    StateOrTransitionChanged(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"echoNextUpdate"`
    /// -   type: `hkBool`
    /// - offset: 77
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "echoNextUpdate")]
    EchoNextUpdate(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbStateMachineInternalStateHkParam<'de>, "@name",
    ("activeTransitions" => ActiveTransitions(Vec<HkbStateMachineActiveTransitionInfo>)),
    ("transitionFlags" => TransitionFlags(Vec<u8>)),
    ("wildcardTransitionFlags" => WildcardTransitionFlags(Vec<u8>)),
    ("delayedTransitions" => DelayedTransitions(Vec<HkbStateMachineDelayedTransitionInfo>)),
    ("timeInState" => TimeInState(f64)),
    ("lastLocalTime" => LastLocalTime(f64)),
    ("currentStateId" => CurrentStateId(i32)),
    ("previousStateId" => PreviousStateId(i32)),
    ("nextStartStateIndexOverride" => NextStartStateIndexOverride(i32)),
    ("stateOrTransitionChanged" => StateOrTransitionChanged(bool)),
    ("echoNextUpdate" => EchoNextUpdate(bool)),
}
