//! A Rust structure that implements a serializer/deserializer corresponding to `hkbEventsFromRangeModifier`, a class defined in C++
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
/// -    size: 68
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbEventsFromRangeModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbEventsFromRangeModifier"`: The original C++ class name.
    #[serde(default = "HkbEventsFromRangeModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xbc561b6e`: Unique value of this class.
    #[serde(default = "HkbEventsFromRangeModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbEventsFromRangeModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbEventsFromRangeModifierHkParam<'a>>
}

impl HkbEventsFromRangeModifier<'_> {
    /// Return `"hkbEventsFromRangeModifier"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbEventsFromRangeModifier".into()
    }

    /// Return `"0xbc561b6e"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xbc561b6e".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbEventsFromRangeModifierHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"inputValue"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "inputValue")]
    InputValue(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"lowerBound"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lowerBound")]
    LowerBound(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"eventRanges"`
    /// -   type: `struct hkbEventRangeDataArray*`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eventRanges")]
    EventRanges(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"wasActiveInPreviousFrame"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "wasActiveInPreviousFrame", skip_serializing)]
    WasActiveInPreviousFrame(Vec<()>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbEventsFromRangeModifierHkParam<'de>, "@name",
    ("inputValue" => InputValue(Primitive<f32>)),
    ("lowerBound" => LowerBound(Primitive<f32>)),
    ("eventRanges" => EventRanges(Cow<'a, str>)),
    ("wasActiveInPreviousFrame" => WasActiveInPreviousFrame(Vec<()>)),
}
