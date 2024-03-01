//! A Rust structure that implements a serializer/deserializer corresponding to `hkMonitorStreamFrameInfo`, a class defined in C++
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
/// -    size: 36
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkMonitorStreamFrameInfo<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkMonitorStreamFrameInfo"`: Name of this class.
    #[serde(default = "HkMonitorStreamFrameInfo::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x7798b7db`: Unique value of this class.
    #[serde(default = "HkMonitorStreamFrameInfo::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkMonitorStreamFrameInfoHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkMonitorStreamFrameInfoHkParam<'a>>
}

impl HkMonitorStreamFrameInfo<'_> {
    /// Return `"hkMonitorStreamFrameInfo"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkMonitorStreamFrameInfo".into()
    }

    /// Return `"0x7798b7db"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x7798b7db".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMonitorStreamFrameInfoHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"heading"`
    /// -   type: `hkStringPtr`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "heading")]
    Heading(String),
    /// # Information on fields in the original C++ class
    /// -   name:`"indexOfTimer0"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indexOfTimer0")]
    IndexOfTimer0(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"indexOfTimer1"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indexOfTimer1")]
    IndexOfTimer1(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"absoluteTimeCounter"`
    /// -   type: `enum AbsoluteTimeCounter`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "absoluteTimeCounter")]
    AbsoluteTimeCounter(AbsoluteTimeCounter),
    /// # Information on fields in the original C++ class
    /// -   name:`"timerFactor0"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "timerFactor0")]
    TimerFactor0(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"timerFactor1"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "timerFactor1")]
    TimerFactor1(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"threadId"`
    /// -   type: `hkInt32`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "threadId")]
    ThreadId(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"frameStreamStart"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "frameStreamStart")]
    FrameStreamStart(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"frameStreamEnd"`
    /// -   type: `hkInt32`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "frameStreamEnd")]
    FrameStreamEnd(i32),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkMonitorStreamFrameInfoHkParam<'de>, "@name",
    ("heading" => Heading(String)),
    ("indexOfTimer0" => IndexOfTimer0(i32)),
    ("indexOfTimer1" => IndexOfTimer1(i32)),
    ("absoluteTimeCounter" => AbsoluteTimeCounter(AbsoluteTimeCounter)),
    ("timerFactor0" => TimerFactor0(f64)),
    ("timerFactor1" => TimerFactor1(f64)),
    ("threadId" => ThreadId(i32)),
    ("frameStreamStart" => FrameStreamStart(i32)),
    ("frameStreamEnd" => FrameStreamEnd(i32)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AbsoluteTimeCounter {
    #[serde(rename = "ABSOLUTE_TIME_TIMER_0")]
    AbsoluteTimeTimer0 = 0,
    #[serde(rename = "ABSOLUTE_TIME_TIMER_1")]
    AbsoluteTimeTimer1 = 1,
    #[serde(rename = "ABSOLUTE_TIME_NOT_TIMED")]
    AbsoluteTimeNotTimed = -1,
}
