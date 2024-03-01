//! A Rust structure that implements a serializer/deserializer corresponding to `hkbSequenceInternalState`, a class defined in C++
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
/// -    size: 64
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbSequenceInternalState<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbSequenceInternalState"`: Name of this class.
    #[serde(default = "HkbSequenceInternalState::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x419b9a05`: Unique value of this class.
    #[serde(default = "HkbSequenceInternalState::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbSequenceInternalStateHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbSequenceInternalStateHkParam<'a>>
}

impl HkbSequenceInternalState<'_> {
    /// Return `"hkbSequenceInternalState"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbSequenceInternalState".into()
    }

    /// Return `"0x419b9a05"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x419b9a05".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbSequenceInternalStateHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"nextSampleEvents"`
    /// -   type: `hkArray&lt;hkInt32&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nextSampleEvents")]
    NextSampleEvents(Vec<i32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"nextSampleReals"`
    /// -   type: `hkArray&lt;hkInt32&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nextSampleReals")]
    NextSampleReals(Vec<i32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"nextSampleBools"`
    /// -   type: `hkArray&lt;hkInt32&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nextSampleBools")]
    NextSampleBools(Vec<i32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"nextSampleInts"`
    /// -   type: `hkArray&lt;hkInt32&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nextSampleInts")]
    NextSampleInts(Vec<i32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"time"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "time")]
    Time(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"isEnabled"`
    /// -   type: `hkBool`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isEnabled")]
    IsEnabled(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbSequenceInternalStateHkParam<'de>, "@name",
    ("nextSampleEvents" => NextSampleEvents(Vec<i32>)),
    ("nextSampleReals" => NextSampleReals(Vec<i32>)),
    ("nextSampleBools" => NextSampleBools(Vec<i32>)),
    ("nextSampleInts" => NextSampleInts(Vec<i32>)),
    ("time" => Time(f64)),
    ("isEnabled" => IsEnabled(bool)),
}
