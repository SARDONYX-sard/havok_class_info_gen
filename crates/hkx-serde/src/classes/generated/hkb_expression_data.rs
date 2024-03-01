//! A Rust structure that implements a serializer/deserializer corresponding to `hkbExpressionData`, a class defined in C++
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
/// -    size: 16
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbExpressionData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbExpressionData"`: Name of this class.
    #[serde(default = "HkbExpressionData::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x6740042a`: Unique value of this class.
    #[serde(default = "HkbExpressionData::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbExpressionDataHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbExpressionDataHkParam<'a>>
}

impl HkbExpressionData<'_> {
    /// Return `"hkbExpressionData"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbExpressionData".into()
    }

    /// Return `"0x6740042a"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x6740042a".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbExpressionDataHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"expression"`
    /// -   type: `hkStringPtr`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "expression")]
    Expression(String),
    /// # Information on fields in the original C++ class
    /// -   name:`"assignmentVariableIndex"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "assignmentVariableIndex")]
    AssignmentVariableIndex(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"assignmentEventIndex"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "assignmentEventIndex")]
    AssignmentEventIndex(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"eventMode"`
    /// -   type: `enum ExpressionEventMode`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eventMode")]
    EventMode(ExpressionEventMode),
    /// # Information on fields in the original C++ class
    /// -   name:`"raisedEvent"`
    /// -   type: `hkBool`
    /// - offset: 13
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "raisedEvent", skip_serializing)]
    RaisedEvent(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"wasTrueInPreviousFrame"`
    /// -   type: `hkBool`
    /// - offset: 14
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "wasTrueInPreviousFrame", skip_serializing)]
    WasTrueInPreviousFrame(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbExpressionDataHkParam<'de>, "@name",
    ("expression" => Expression(String)),
    ("assignmentVariableIndex" => AssignmentVariableIndex(i32)),
    ("assignmentEventIndex" => AssignmentEventIndex(i32)),
    ("eventMode" => EventMode(ExpressionEventMode)),
    ("raisedEvent" => RaisedEvent(bool)),
    ("wasTrueInPreviousFrame" => WasTrueInPreviousFrame(bool)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ExpressionEventMode {
    #[serde(rename = "EVENT_MODE_SEND_ONCE")]
    EventModeSendOnce = 0,
    #[serde(rename = "EVENT_MODE_SEND_ON_TRUE")]
    EventModeSendOnTrue = 1,
    #[serde(rename = "EVENT_MODE_SEND_ON_FALSE_TO_TRUE")]
    EventModeSendOnFalseToTrue = 2,
    #[serde(rename = "EVENT_MODE_SEND_EVERY_FRAME_ONCE_TRUE")]
    EventModeSendEveryFrameOnceTrue = 3,
}
