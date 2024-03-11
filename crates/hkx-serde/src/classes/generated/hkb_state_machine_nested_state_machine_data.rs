//! A Rust structure that implements a serializer/deserializer corresponding to `hkbStateMachineNestedStateMachineData`, a class defined in C++
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
/// -    size: 8
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbStateMachineNestedStateMachineData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbStateMachineNestedStateMachineData"`: The original C++ class name.
    #[serde(default = "HkbStateMachineNestedStateMachineData::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x7358f5da`: Unique value of this class.
    #[serde(default = "HkbStateMachineNestedStateMachineData::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbStateMachineNestedStateMachineDataHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbStateMachineNestedStateMachineDataHkParam<'a>>
}

impl HkbStateMachineNestedStateMachineData<'_> {
    /// Return `"hkbStateMachineNestedStateMachineData"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbStateMachineNestedStateMachineData".into()
    }

    /// Return `"0x7358f5da"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x7358f5da".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbStateMachineNestedStateMachineDataHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"nestedStateMachine"`
    /// -   type: `void*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "nestedStateMachine", skip_serializing)]
    NestedStateMachine(()),
    /// # Field information in the original C++ class
    /// -   name:`"eventIdMap"`
    /// -   type: `void*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "eventIdMap", skip_serializing)]
    EventIdMap(()),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbStateMachineNestedStateMachineDataHkParam<'de>, "@name",
    ("nestedStateMachine" => NestedStateMachine(())),
    ("eventIdMap" => EventIdMap(())),
}
