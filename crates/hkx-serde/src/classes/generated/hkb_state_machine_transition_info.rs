//! A Rust structure that implements a serializer/deserializer corresponding to `hkbStateMachineTransitionInfo`, a class defined in C++
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
/// -    size: 60
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbStateMachineTransitionInfo<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbStateMachineTransitionInfo"`: Name of this class.
    #[serde(default = "HkbStateMachineTransitionInfo::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xcdec8025`: Unique value of this class.
    #[serde(default = "HkbStateMachineTransitionInfo::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbStateMachineTransitionInfoHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbStateMachineTransitionInfoHkParam<'a>>
}

impl HkbStateMachineTransitionInfo<'_> {
    /// Return `"hkbStateMachineTransitionInfo"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbStateMachineTransitionInfo".into()
    }

    /// Return `"0xcdec8025"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xcdec8025".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbStateMachineTransitionInfoHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"triggerInterval"`
    /// -   type: `struct hkbStateMachineTimeInterval`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "triggerInterval")]
    TriggerInterval(HkbStateMachineTimeInterval),
    /// # Information on fields in the original C++ class
    /// -   name:`"initiateInterval"`
    /// -   type: `struct hkbStateMachineTimeInterval`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "initiateInterval")]
    InitiateInterval(HkbStateMachineTimeInterval),
    /// # Information on fields in the original C++ class
    /// -   name:`"transition"`
    /// -   type: `struct hkbTransitionEffect*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transition")]
    Transition(Box<HkbTransitionEffect>),
    /// # Information on fields in the original C++ class
    /// -   name:`"condition"`
    /// -   type: `struct hkbCondition*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "condition")]
    Condition(Box<HkbCondition>),
    /// # Information on fields in the original C++ class
    /// -   name:`"eventId"`
    /// -   type: `hkInt32`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eventId")]
    EventId(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"toStateId"`
    /// -   type: `hkInt32`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "toStateId")]
    ToStateId(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"fromNestedStateId"`
    /// -   type: `hkInt32`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fromNestedStateId")]
    FromNestedStateId(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"toNestedStateId"`
    /// -   type: `hkInt32`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "toNestedStateId")]
    ToNestedStateId(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"priority"`
    /// -   type: `hkInt16`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "priority")]
    Priority(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"flags"`
    /// -   type: `flags TransitionFlags`
    /// - offset: 58
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "flags")]
    Flags(TransitionFlags),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbStateMachineTransitionInfoHkParam<'de>, "@name",
    ("triggerInterval" => TriggerInterval(HkbStateMachineTimeInterval)),
    ("initiateInterval" => InitiateInterval(HkbStateMachineTimeInterval)),
    ("transition" => Transition(Box<HkbTransitionEffect>)),
    ("condition" => Condition(Box<HkbCondition>)),
    ("eventId" => EventId(i32)),
    ("toStateId" => ToStateId(i32)),
    ("fromNestedStateId" => FromNestedStateId(i32)),
    ("toNestedStateId" => ToNestedStateId(i32)),
    ("priority" => Priority(i16)),
    ("flags" => Flags(TransitionFlags)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum TransitionFlags {
    #[serde(rename = "FLAG_USE_TRIGGER_INTERVAL")]
    FlagUseTriggerInterval = 1,
    #[serde(rename = "FLAG_USE_INITIATE_INTERVAL")]
    FlagUseInitiateInterval = 2,
    #[serde(rename = "FLAG_UNINTERRUPTIBLE_WHILE_PLAYING")]
    FlagUninterruptibleWhilePlaying = 4,
    #[serde(rename = "FLAG_UNINTERRUPTIBLE_WHILE_DELAYED")]
    FlagUninterruptibleWhileDelayed = 8,
    #[serde(rename = "FLAG_DELAY_STATE_CHANGE")]
    FlagDelayStateChange = 16,
    #[serde(rename = "FLAG_DISABLED")]
    FlagDisabled = 32,
    #[serde(rename = "FLAG_DISALLOW_RETURN_TO_PREVIOUS_STATE")]
    FlagDisallowReturnToPreviousState = 64,
    #[serde(rename = "FLAG_DISALLOW_RANDOM_TRANSITION")]
    FlagDisallowRandomTransition = 128,
    #[serde(rename = "FLAG_DISABLE_CONDITION")]
    FlagDisableCondition = 256,
    #[serde(rename = "FLAG_ALLOW_SELF_TRANSITION_BY_TRANSITION_FROM_ANY_STATE")]
    FlagAllowSelfTransitionByTransitionFromAnyState = 512,
    #[serde(rename = "FLAG_IS_GLOBAL_WILDCARD")]
    FlagIsGlobalWildcard = 1024,
    #[serde(rename = "FLAG_IS_LOCAL_WILDCARD")]
    FlagIsLocalWildcard = 2048,
    #[serde(rename = "FLAG_FROM_NESTED_STATE_ID_IS_VALID")]
    FlagFromNestedStateIdIsValid = 4096,
    #[serde(rename = "FLAG_TO_NESTED_STATE_ID_IS_VALID")]
    FlagToNestedStateIdIsValid = 8192,
    #[serde(rename = "FLAG_ABUT_AT_END_OF_FROM_GENERATOR")]
    FlagAbutAtEndOfFromGenerator = 16384,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InternalFlagBits {
    #[serde(rename = "FLAG_INTERNAL_IN_TRIGGER_INTERVAL")]
    FlagInternalInTriggerInterval = 1,
    #[serde(rename = "FLAG_INTERNAL_IN_INITIATE_INTERVAL")]
    FlagInternalInInitiateInterval = 2,
}
