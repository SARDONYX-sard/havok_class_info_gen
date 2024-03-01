//! A Rust structure that implements a serializer/deserializer corresponding to `hkbStateMachineTransitionInfoReference`, a class defined in C++
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
/// -    size: 6
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbStateMachineTransitionInfoReference<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbStateMachineTransitionInfoReference"`: Name of this class.
    #[serde(default = "HkbStateMachineTransitionInfoReference::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x9810c2d0`: Unique value of this class.
    #[serde(default = "HkbStateMachineTransitionInfoReference::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbStateMachineTransitionInfoReferenceHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbStateMachineTransitionInfoReferenceHkParam<'a>>
}

impl HkbStateMachineTransitionInfoReference<'_> {
    /// Return `"hkbStateMachineTransitionInfoReference"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbStateMachineTransitionInfoReference".into()
    }

    /// Return `"0x9810c2d0"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x9810c2d0".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbStateMachineTransitionInfoReferenceHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"fromStateIndex"`
    /// -   type: `hkInt16`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fromStateIndex")]
    FromStateIndex(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"transitionIndex"`
    /// -   type: `hkInt16`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transitionIndex")]
    TransitionIndex(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"stateMachineId"`
    /// -   type: `hkInt16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "stateMachineId")]
    StateMachineId(i16),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbStateMachineTransitionInfoReferenceHkParam<'de>, "@name",
    ("fromStateIndex" => FromStateIndex(i16)),
    ("transitionIndex" => TransitionIndex(i16)),
    ("stateMachineId" => StateMachineId(i16)),
}
