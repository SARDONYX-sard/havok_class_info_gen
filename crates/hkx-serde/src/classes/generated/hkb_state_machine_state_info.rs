//! A Rust structure that implements a serializer/deserializer corresponding to `hkbStateMachineStateInfo`, a class defined in C++
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
/// -    size: 72
/// -  vtable: true
/// -  parent: hkbBindable/`2c1432d7`(Non prefix hex signature)
/// - version: 4
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbStateMachineStateInfo<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbStateMachineStateInfo"`: Name of this class.
    #[serde(default = "HkbStateMachineStateInfo::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xed7f9d0`: Unique value of this class.
    #[serde(default = "HkbStateMachineStateInfo::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbStateMachineStateInfoHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbStateMachineStateInfoHkParam<'a>>
}

impl HkbStateMachineStateInfo<'_> {
    /// Return `"hkbStateMachineStateInfo"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbStateMachineStateInfo".into()
    }

    /// Return `"0xed7f9d0"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xed7f9d0".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbStateMachineStateInfoHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"listeners"`
    /// -   type: `hkArray&lt;hkbStateListener*&gt;`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "listeners")]
    Listeners(Vec<Box<HkbStateListener>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"enterNotifyEvents"`
    /// -   type: `struct hkbStateMachineEventPropertyArray*`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enterNotifyEvents")]
    EnterNotifyEvents(Box<HkbStateMachineEventPropertyArray>),
    /// # Information on fields in the original C++ class
    /// -   name:`"exitNotifyEvents"`
    /// -   type: `struct hkbStateMachineEventPropertyArray*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "exitNotifyEvents")]
    ExitNotifyEvents(Box<HkbStateMachineEventPropertyArray>),
    /// # Information on fields in the original C++ class
    /// -   name:`"transitions"`
    /// -   type: `struct hkbStateMachineTransitionInfoArray*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transitions")]
    Transitions(Box<HkbStateMachineTransitionInfoArray>),
    /// # Information on fields in the original C++ class
    /// -   name:`"generator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "generator")]
    Generator(Box<HkbGenerator>),
    /// # Information on fields in the original C++ class
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(String),
    /// # Information on fields in the original C++ class
    /// -   name:`"stateId"`
    /// -   type: `hkInt32`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "stateId")]
    StateId(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"probability"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "probability")]
    Probability(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"enable"`
    /// -   type: `hkBool`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enable")]
    Enable(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbStateMachineStateInfoHkParam<'de>, "@name",
    ("listeners" => Listeners(Vec<Box<HkbStateListener>>)),
    ("enterNotifyEvents" => EnterNotifyEvents(Box<HkbStateMachineEventPropertyArray>)),
    ("exitNotifyEvents" => ExitNotifyEvents(Box<HkbStateMachineEventPropertyArray>)),
    ("transitions" => Transitions(Box<HkbStateMachineTransitionInfoArray>)),
    ("generator" => Generator(Box<HkbGenerator>)),
    ("name" => Name(String)),
    ("stateId" => StateId(i32)),
    ("probability" => Probability(f64)),
    ("enable" => Enable(bool)),
}
