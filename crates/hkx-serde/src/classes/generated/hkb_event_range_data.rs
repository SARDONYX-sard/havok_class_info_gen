//! A Rust structure that implements a serializer/deserializer corresponding to `hkbEventRangeData`, a class defined in C++
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
pub struct HkbEventRangeData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbEventRangeData"`: The original C++ class name.
    #[serde(default = "HkbEventRangeData::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x6cb92c76`: Unique value of this class.
    #[serde(default = "HkbEventRangeData::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbEventRangeDataHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbEventRangeDataHkParam<'a>>
}

impl HkbEventRangeData<'_> {
    /// Return `"hkbEventRangeData"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbEventRangeData".into()
    }

    /// Return `"0x6cb92c76"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x6cb92c76".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbEventRangeDataHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"upperBound"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "upperBound")]
    UpperBound(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"event"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "event")]
    Event(HkbEventProperty),
    /// # Field information in the original C++ class
    /// -   name:`"eventMode"`
    /// -   type: `enum EventRangeMode`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eventMode")]
    EventMode(EventRangeMode),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbEventRangeDataHkParam<'de>, "@name",
    ("upperBound" => UpperBound(Primitive<f32>)),
    ("event" => Event(HkbEventProperty)),
    ("eventMode" => EventMode(EventRangeMode)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EventRangeMode {
    #[serde(rename = "EVENT_MODE_SEND_ON_ENTER_RANGE")]
    EventModeSendOnEnterRange = 0,
    #[serde(rename = "EVENT_MODE_SEND_WHEN_IN_RANGE")]
    EventModeSendWhenInRange = 1,
}
