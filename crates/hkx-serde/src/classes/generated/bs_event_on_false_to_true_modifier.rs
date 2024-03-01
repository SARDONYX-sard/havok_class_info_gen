//! A Rust structure that implements a serializer/deserializer corresponding to `BSEventOnFalseToTrueModifier`, a class defined in C++
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
/// -    size: 84
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct BsEventOnFalseToTrueModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"BSEventOnFalseToTrueModifier"`: Name of this class.
    #[serde(default = "BsEventOnFalseToTrueModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x81d0777a`: Unique value of this class.
    #[serde(default = "BsEventOnFalseToTrueModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<BsEventOnFalseToTrueModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<BsEventOnFalseToTrueModifierHkParam<'a>>
}

impl BsEventOnFalseToTrueModifier<'_> {
    /// Return `"BSEventOnFalseToTrueModifier"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "BsEventOnFalseToTrueModifier".into()
    }

    /// Return `"0x81d0777a"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x81d0777a".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsEventOnFalseToTrueModifierHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"bEnableEvent1"`
    /// -   type: `hkBool`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bEnableEvent1")]
    BEnableEvent1(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"bVariableToTest1"`
    /// -   type: `hkBool`
    /// - offset: 45
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bVariableToTest1")]
    BVariableToTest1(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"EventToSend1"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "EventToSend1")]
    EventToSend1(HkbEventProperty),
    /// # Information on fields in the original C++ class
    /// -   name:`"bEnableEvent2"`
    /// -   type: `hkBool`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bEnableEvent2")]
    BEnableEvent2(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"bVariableToTest2"`
    /// -   type: `hkBool`
    /// - offset: 57
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bVariableToTest2")]
    BVariableToTest2(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"EventToSend2"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "EventToSend2")]
    EventToSend2(HkbEventProperty),
    /// # Information on fields in the original C++ class
    /// -   name:`"bEnableEvent3"`
    /// -   type: `hkBool`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bEnableEvent3")]
    BEnableEvent3(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"bVariableToTest3"`
    /// -   type: `hkBool`
    /// - offset: 69
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bVariableToTest3")]
    BVariableToTest3(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"EventToSend3"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "EventToSend3")]
    EventToSend3(HkbEventProperty),
    /// # Information on fields in the original C++ class
    /// -   name:`"bSlot1ActivatedLastFrame"`
    /// -   type: `hkBool`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "bSlot1ActivatedLastFrame", skip_serializing)]
    BSlot1ActivatedLastFrame(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"bSlot2ActivatedLastFrame"`
    /// -   type: `hkBool`
    /// - offset: 81
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "bSlot2ActivatedLastFrame", skip_serializing)]
    BSlot2ActivatedLastFrame(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"bSlot3ActivatedLastFrame"`
    /// -   type: `hkBool`
    /// - offset: 82
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "bSlot3ActivatedLastFrame", skip_serializing)]
    BSlot3ActivatedLastFrame(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    BsEventOnFalseToTrueModifierHkParam<'de>, "@name",
    ("bEnableEvent1" => BEnableEvent1(bool)),
    ("bVariableToTest1" => BVariableToTest1(bool)),
    ("EventToSend1" => EventToSend1(HkbEventProperty)),
    ("bEnableEvent2" => BEnableEvent2(bool)),
    ("bVariableToTest2" => BVariableToTest2(bool)),
    ("EventToSend2" => EventToSend2(HkbEventProperty)),
    ("bEnableEvent3" => BEnableEvent3(bool)),
    ("bVariableToTest3" => BVariableToTest3(bool)),
    ("EventToSend3" => EventToSend3(HkbEventProperty)),
    ("bSlot1ActivatedLastFrame" => BSlot1ActivatedLastFrame(bool)),
    ("bSlot2ActivatedLastFrame" => BSlot2ActivatedLastFrame(bool)),
    ("bSlot3ActivatedLastFrame" => BSlot3ActivatedLastFrame(bool)),
}
