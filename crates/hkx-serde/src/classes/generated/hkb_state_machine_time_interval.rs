//! A Rust structure that implements a serializer/deserializer corresponding to `hkbStateMachineTimeInterval`, a class defined in C++
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
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbStateMachineTimeInterval<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbStateMachineTimeInterval"`: The original C++ class name.
    #[serde(default = "HkbStateMachineTimeInterval::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x60a881e5`: Unique value of this class.
    #[serde(default = "HkbStateMachineTimeInterval::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbStateMachineTimeIntervalHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbStateMachineTimeIntervalHkParam<'a>>
}

impl HkbStateMachineTimeInterval<'_> {
    /// Return `"hkbStateMachineTimeInterval"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbStateMachineTimeInterval".into()
    }

    /// Return `"0x60a881e5"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x60a881e5".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbStateMachineTimeIntervalHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"enterEventId"`
    /// -   type: `hkInt32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enterEventId")]
    EnterEventId(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"exitEventId"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "exitEventId")]
    ExitEventId(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"enterTime"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enterTime")]
    EnterTime(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"exitTime"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "exitTime")]
    ExitTime(Primitive<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbStateMachineTimeIntervalHkParam<'de>, "@name",
    ("enterEventId" => EnterEventId(Primitive<i32>)),
    ("exitEventId" => ExitEventId(Primitive<i32>)),
    ("enterTime" => EnterTime(Primitive<f32>)),
    ("exitTime" => ExitTime(Primitive<f32>)),
}
