//! A Rust structure that implements a serializer/deserializer corresponding to `hkbStateMachine`, a class defined in C++
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
/// -    size: 180
/// -  vtable: true
/// -  parent: hkbGenerator/`d68aefc`(Non prefix hex signature)
/// - version: 4
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbStateMachine<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbStateMachine"`: The original C++ class name.
    #[serde(default = "HkbStateMachine::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x816c1dcb`: Unique value of this class.
    #[serde(default = "HkbStateMachine::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbStateMachineHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbStateMachineHkParam<'a>>
}

impl HkbStateMachine<'_> {
    /// Return `"hkbStateMachine"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbStateMachine".into()
    }

    /// Return `"0x816c1dcb"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x816c1dcb".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbStateMachineHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"eventToSendWhenStateOrTransitionChanges"`
    /// -   type: `struct hkbEvent`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eventToSendWhenStateOrTransitionChanges")]
    EventToSendWhenStateOrTransitionChanges(HkbEvent),
    /// # Field information in the original C++ class
    /// -   name:`"startStateChooser"`
    /// -   type: `struct hkbStateChooser*`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "startStateChooser")]
    StartStateChooser(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"startStateId"`
    /// -   type: `hkInt32`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "startStateId")]
    StartStateId(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"returnToPreviousStateEventId"`
    /// -   type: `hkInt32`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "returnToPreviousStateEventId")]
    ReturnToPreviousStateEventId(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"randomTransitionEventId"`
    /// -   type: `hkInt32`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "randomTransitionEventId")]
    RandomTransitionEventId(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"transitionToNextHigherStateEventId"`
    /// -   type: `hkInt32`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transitionToNextHigherStateEventId")]
    TransitionToNextHigherStateEventId(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"transitionToNextLowerStateEventId"`
    /// -   type: `hkInt32`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transitionToNextLowerStateEventId")]
    TransitionToNextLowerStateEventId(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"syncVariableIndex"`
    /// -   type: `hkInt32`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "syncVariableIndex")]
    SyncVariableIndex(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"currentStateId"`
    /// -   type: `hkInt32`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "currentStateId", skip_serializing)]
    CurrentStateId(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"wrapAroundStateId"`
    /// -   type: `hkBool`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wrapAroundStateId")]
    WrapAroundStateId(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"maxSimultaneousTransitions"`
    /// -   type: `hkInt8`
    /// - offset: 85
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxSimultaneousTransitions")]
    MaxSimultaneousTransitions(Primitive<i8>),
    /// # Field information in the original C++ class
    /// -   name:`"startStateMode"`
    /// -   type: `enum StartStateMode`
    /// - offset: 86
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "startStateMode")]
    StartStateMode(StartStateMode),
    /// # Field information in the original C++ class
    /// -   name:`"selfTransitionMode"`
    /// -   type: `enum StateMachineSelfTransitionMode`
    /// - offset: 87
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "selfTransitionMode")]
    SelfTransitionMode(StateMachineSelfTransitionMode),
    /// # Field information in the original C++ class
    /// -   name:`"isActive"`
    /// -   type: `hkBool`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "isActive", skip_serializing)]
    IsActive(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"states"`
    /// -   type: `hkArray&lt;hkbStateMachineStateInfo*&gt;`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "states")]
    States(Vec<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"wildcardTransitions"`
    /// -   type: `struct hkbStateMachineTransitionInfoArray*`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wildcardTransitions")]
    WildcardTransitions(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"stateIdToIndexMap"`
    /// -   type: `void*`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "stateIdToIndexMap", skip_serializing)]
    StateIdToIndexMap(()),
    /// # Field information in the original C++ class
    /// -   name:`"activeTransitions"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "activeTransitions", skip_serializing)]
    ActiveTransitions(Vec<()>),
    /// # Field information in the original C++ class
    /// -   name:`"transitionFlags"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "transitionFlags", skip_serializing)]
    TransitionFlags(Vec<()>),
    /// # Field information in the original C++ class
    /// -   name:`"wildcardTransitionFlags"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "wildcardTransitionFlags", skip_serializing)]
    WildcardTransitionFlags(Vec<()>),
    /// # Field information in the original C++ class
    /// -   name:`"delayedTransitions"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "delayedTransitions", skip_serializing)]
    DelayedTransitions(Vec<()>),
    /// # Field information in the original C++ class
    /// -   name:`"timeInState"`
    /// -   type: `hkReal`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "timeInState", skip_serializing)]
    TimeInState(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"lastLocalTime"`
    /// -   type: `hkReal`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "lastLocalTime", skip_serializing)]
    LastLocalTime(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"previousStateId"`
    /// -   type: `hkInt32`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "previousStateId", skip_serializing)]
    PreviousStateId(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"nextStartStateIndexOverride"`
    /// -   type: `hkInt32`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "nextStartStateIndexOverride", skip_serializing)]
    NextStartStateIndexOverride(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"stateOrTransitionChanged"`
    /// -   type: `hkBool`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "stateOrTransitionChanged", skip_serializing)]
    StateOrTransitionChanged(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"echoNextUpdate"`
    /// -   type: `hkBool`
    /// - offset: 177
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "echoNextUpdate", skip_serializing)]
    EchoNextUpdate(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"sCurrentStateIndexAndEntered"`
    /// -   type: `hkUint16`
    /// - offset: 178
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "sCurrentStateIndexAndEntered", skip_serializing)]
    SCurrentStateIndexAndEntered(Primitive<u16>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbStateMachineHkParam<'de>, "@name",
    ("eventToSendWhenStateOrTransitionChanges" => EventToSendWhenStateOrTransitionChanges(HkbEvent)),
    ("startStateChooser" => StartStateChooser(Cow<'a, str>)),
    ("startStateId" => StartStateId(Primitive<i32>)),
    ("returnToPreviousStateEventId" => ReturnToPreviousStateEventId(Primitive<i32>)),
    ("randomTransitionEventId" => RandomTransitionEventId(Primitive<i32>)),
    ("transitionToNextHigherStateEventId" => TransitionToNextHigherStateEventId(Primitive<i32>)),
    ("transitionToNextLowerStateEventId" => TransitionToNextLowerStateEventId(Primitive<i32>)),
    ("syncVariableIndex" => SyncVariableIndex(Primitive<i32>)),
    ("currentStateId" => CurrentStateId(Primitive<i32>)),
    ("wrapAroundStateId" => WrapAroundStateId(Primitive<bool>)),
    ("maxSimultaneousTransitions" => MaxSimultaneousTransitions(Primitive<i8>)),
    ("startStateMode" => StartStateMode(StartStateMode)),
    ("selfTransitionMode" => SelfTransitionMode(StateMachineSelfTransitionMode)),
    ("isActive" => IsActive(Primitive<bool>)),
    ("states" => States(Vec<Cow<'a, str>>)),
    ("wildcardTransitions" => WildcardTransitions(Cow<'a, str>)),
    ("stateIdToIndexMap" => StateIdToIndexMap(())),
    ("activeTransitions" => ActiveTransitions(Vec<()>)),
    ("transitionFlags" => TransitionFlags(Vec<()>)),
    ("wildcardTransitionFlags" => WildcardTransitionFlags(Vec<()>)),
    ("delayedTransitions" => DelayedTransitions(Vec<()>)),
    ("timeInState" => TimeInState(Primitive<f32>)),
    ("lastLocalTime" => LastLocalTime(Primitive<f32>)),
    ("previousStateId" => PreviousStateId(Primitive<i32>)),
    ("nextStartStateIndexOverride" => NextStartStateIndexOverride(Primitive<i32>)),
    ("stateOrTransitionChanged" => StateOrTransitionChanged(Primitive<bool>)),
    ("echoNextUpdate" => EchoNextUpdate(Primitive<bool>)),
    ("sCurrentStateIndexAndEntered" => SCurrentStateIndexAndEntered(Primitive<u16>)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum StartStateMode {
    #[serde(rename = "START_STATE_MODE_DEFAULT")]
    StartStateModeDefault = 0,
    #[serde(rename = "START_STATE_MODE_SYNC")]
    StartStateModeSync = 1,
    #[serde(rename = "START_STATE_MODE_RANDOM")]
    StartStateModeRandom = 2,
    #[serde(rename = "START_STATE_MODE_CHOOSER")]
    StartStateModeChooser = 3,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum StateMachineSelfTransitionMode {
    #[serde(rename = "SELF_TRANSITION_MODE_NO_TRANSITION")]
    SelfTransitionModeNoTransition = 0,
    #[serde(rename = "SELF_TRANSITION_MODE_TRANSITION_TO_START_STATE")]
    SelfTransitionModeTransitionToStartState = 1,
    #[serde(rename = "SELF_TRANSITION_MODE_FORCE_TRANSITION_TO_START_STATE")]
    SelfTransitionModeForceTransitionToStartState = 2,
}
