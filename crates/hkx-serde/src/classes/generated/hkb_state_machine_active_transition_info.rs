//! A Rust structure that implements a serializer/deserializer corresponding to `hkbStateMachineActiveTransitionInfo`, a class defined in C++
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
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbStateMachineActiveTransitionInfo<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbStateMachineActiveTransitionInfo"`: Name of this class.
    #[serde(default = "HkbStateMachineActiveTransitionInfo::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xbb90d54f`: Unique value of this class.
    #[serde(default = "HkbStateMachineActiveTransitionInfo::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbStateMachineActiveTransitionInfoHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbStateMachineActiveTransitionInfoHkParam<'a>>
}

impl HkbStateMachineActiveTransitionInfo<'_> {
    /// Return `"hkbStateMachineActiveTransitionInfo"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbStateMachineActiveTransitionInfo".into()
    }

    /// Return `"0xbb90d54f"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xbb90d54f".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbStateMachineActiveTransitionInfoHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"transitionEffect"`
    /// -   type: `void*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "transitionEffect", skip_serializing)]
    TransitionEffect(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"transitionEffectInternalStateInfo"`
    /// -   type: `struct hkbNodeInternalStateInfo*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transitionEffectInternalStateInfo")]
    TransitionEffectInternalStateInfo(Box<HkbNodeInternalStateInfo>),
    /// # Information on fields in the original C++ class
    /// -   name:`"transitionInfoReference"`
    /// -   type: `struct hkbStateMachineTransitionInfoReference`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transitionInfoReference")]
    TransitionInfoReference(HkbStateMachineTransitionInfoReference),
    /// # Information on fields in the original C++ class
    /// -   name:`"transitionInfoReferenceForTE"`
    /// -   type: `struct hkbStateMachineTransitionInfoReference`
    /// - offset: 14
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transitionInfoReferenceForTE")]
    TransitionInfoReferenceForTe(HkbStateMachineTransitionInfoReference),
    /// # Information on fields in the original C++ class
    /// -   name:`"fromStateId"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fromStateId")]
    FromStateId(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"toStateId"`
    /// -   type: `hkInt32`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "toStateId")]
    ToStateId(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"isReturnToPreviousState"`
    /// -   type: `hkBool`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isReturnToPreviousState")]
    IsReturnToPreviousState(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbStateMachineActiveTransitionInfoHkParam<'de>, "@name",
    ("transitionEffect" => TransitionEffect(())),
    ("transitionEffectInternalStateInfo" => TransitionEffectInternalStateInfo(Box<HkbNodeInternalStateInfo>)),
    ("transitionInfoReference" => TransitionInfoReference(HkbStateMachineTransitionInfoReference)),
    ("transitionInfoReferenceForTE" => TransitionInfoReferenceForTe(HkbStateMachineTransitionInfoReference)),
    ("fromStateId" => FromStateId(i32)),
    ("toStateId" => ToStateId(i32)),
    ("isReturnToPreviousState" => IsReturnToPreviousState(bool)),
}
