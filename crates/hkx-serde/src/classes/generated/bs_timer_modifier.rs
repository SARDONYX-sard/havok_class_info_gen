//! A Rust structure that implements a serializer/deserializer corresponding to `BSTimerModifier`, a class defined in C++
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
/// -    size: 64
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct BsTimerModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"BSTimerModifier"`: The original C++ class name.
    #[serde(default = "BsTimerModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x531f3292`: Unique value of this class.
    #[serde(default = "BsTimerModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<BsTimerModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<BsTimerModifierHkParam<'a>>
}

impl BsTimerModifier<'_> {
    /// Return `"BSTimerModifier"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "BSTimerModifier".into()
    }

    /// Return `"0x531f3292"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x531f3292".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsTimerModifierHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"alarmTimeSeconds"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "alarmTimeSeconds")]
    AlarmTimeSeconds(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"alarmEvent"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "alarmEvent")]
    AlarmEvent(HkbEventProperty),
    /// # Field information in the original C++ class
    /// -   name:`"resetAlarm"`
    /// -   type: `hkBool`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "resetAlarm")]
    ResetAlarm(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"secondsElapsed"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "secondsElapsed", skip_serializing)]
    SecondsElapsed(Primitive<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    BsTimerModifierHkParam<'de>, "@name",
    ("alarmTimeSeconds" => AlarmTimeSeconds(Primitive<f32>)),
    ("alarmEvent" => AlarmEvent(HkbEventProperty)),
    ("resetAlarm" => ResetAlarm(Primitive<bool>)),
    ("secondsElapsed" => SecondsElapsed(Primitive<f32>)),
}
