//! A Rust structure that implements a serializer/deserializer corresponding to `hkbStateMachineProspectiveTransitionInfo`, a class defined in C++
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
/// -    size: 16
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 2
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbStateMachineProspectiveTransitionInfo<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbStateMachineProspectiveTransitionInfo"`: The original C++ class name.
    #[serde(default = "HkbStateMachineProspectiveTransitionInfo::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x3ab09a2e`: Unique value of this class.
    #[serde(default = "HkbStateMachineProspectiveTransitionInfo::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbStateMachineProspectiveTransitionInfoHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbStateMachineProspectiveTransitionInfoHkParam<'a>>
}

impl HkbStateMachineProspectiveTransitionInfo<'_> {
    /// Return `"hkbStateMachineProspectiveTransitionInfo"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbStateMachineProspectiveTransitionInfo".into()
    }

    /// Return `"0x3ab09a2e"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x3ab09a2e".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbStateMachineProspectiveTransitionInfoHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"transitionInfoReference"`
    /// -   type: `struct hkbStateMachineTransitionInfoReference`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transitionInfoReference")]
    TransitionInfoReference(HkbStateMachineTransitionInfoReference),
    /// # Field information in the original C++ class
    /// -   name:`"transitionInfoReferenceForTE"`
    /// -   type: `struct hkbStateMachineTransitionInfoReference`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transitionInfoReferenceForTE")]
    TransitionInfoReferenceForTe(HkbStateMachineTransitionInfoReference),
    /// # Field information in the original C++ class
    /// -   name:`"toStateId"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "toStateId")]
    ToStateId(Primitive<i32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbStateMachineProspectiveTransitionInfoHkParam<'de>, "@name",
    ("transitionInfoReference" => TransitionInfoReference(HkbStateMachineTransitionInfoReference)),
    ("transitionInfoReferenceForTE" => TransitionInfoReferenceForTe(HkbStateMachineTransitionInfoReference)),
    ("toStateId" => ToStateId(Primitive<i32>)),
}
