//! A Rust structure that implements a serializer/deserializer corresponding to `BSEventEveryNEventsModifier`, a class defined in C++
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
/// -    size: 72
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct BsEventEveryNEventsModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"BSEventEveryNEventsModifier"`: The original C++ class name.
    #[serde(default = "BsEventEveryNEventsModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x6030970c`: Unique value of this class.
    #[serde(default = "BsEventEveryNEventsModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<BsEventEveryNEventsModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<BsEventEveryNEventsModifierHkParam<'a>>
}

impl BsEventEveryNEventsModifier<'_> {
    /// Return `"BSEventEveryNEventsModifier"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "BSEventEveryNEventsModifier".into()
    }

    /// Return `"0x6030970c"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x6030970c".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsEventEveryNEventsModifierHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"eventToCheckFor"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eventToCheckFor")]
    EventToCheckFor(HkbEventProperty),
    /// # Field information in the original C++ class
    /// -   name:`"eventToSend"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eventToSend")]
    EventToSend(HkbEventProperty),
    /// # Field information in the original C++ class
    /// -   name:`"numberOfEventsBeforeSend"`
    /// -   type: `hkInt8`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numberOfEventsBeforeSend")]
    NumberOfEventsBeforeSend(Primitive<i8>),
    /// # Field information in the original C++ class
    /// -   name:`"minimumNumberOfEventsBeforeSend"`
    /// -   type: `hkInt8`
    /// - offset: 61
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minimumNumberOfEventsBeforeSend")]
    MinimumNumberOfEventsBeforeSend(Primitive<i8>),
    /// # Field information in the original C++ class
    /// -   name:`"randomizeNumberOfEvents"`
    /// -   type: `hkBool`
    /// - offset: 62
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "randomizeNumberOfEvents")]
    RandomizeNumberOfEvents(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"numberOfEventsSeen"`
    /// -   type: `hkInt32`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "numberOfEventsSeen", skip_serializing)]
    NumberOfEventsSeen(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"calculatedNumberOfEventsBeforeSend"`
    /// -   type: `hkInt8`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "calculatedNumberOfEventsBeforeSend", skip_serializing)]
    CalculatedNumberOfEventsBeforeSend(Primitive<i8>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    BsEventEveryNEventsModifierHkParam<'de>, "@name",
    ("eventToCheckFor" => EventToCheckFor(HkbEventProperty)),
    ("eventToSend" => EventToSend(HkbEventProperty)),
    ("numberOfEventsBeforeSend" => NumberOfEventsBeforeSend(Primitive<i8>)),
    ("minimumNumberOfEventsBeforeSend" => MinimumNumberOfEventsBeforeSend(Primitive<i8>)),
    ("randomizeNumberOfEvents" => RandomizeNumberOfEvents(Primitive<bool>)),
    ("numberOfEventsSeen" => NumberOfEventsSeen(Primitive<i32>)),
    ("calculatedNumberOfEventsBeforeSend" => CalculatedNumberOfEventsBeforeSend(Primitive<i8>)),
}
