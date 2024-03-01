//! A Rust structure that implements a serializer/deserializer corresponding to `hkbSequence`, a class defined in C++
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
/// -    size: 168
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbSequence<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbSequence"`: Name of this class.
    #[serde(default = "HkbSequence::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x43182ca3`: Unique value of this class.
    #[serde(default = "HkbSequence::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbSequenceHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbSequenceHkParam<'a>>
}

impl HkbSequence<'_> {
    /// Return `"hkbSequence"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbSequence".into()
    }

    /// Return `"0x43182ca3"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x43182ca3".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbSequenceHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"eventSequencedData"`
    /// -   type: `hkArray&lt;hkbEventSequencedData*&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eventSequencedData")]
    EventSequencedData(Vec<Box<HkbEventSequencedData>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"realVariableSequencedData"`
    /// -   type: `hkArray&lt;hkbRealVariableSequencedData*&gt;`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "realVariableSequencedData")]
    RealVariableSequencedData(Vec<Box<HkbRealVariableSequencedData>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"boolVariableSequencedData"`
    /// -   type: `hkArray&lt;hkbBoolVariableSequencedData*&gt;`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "boolVariableSequencedData")]
    BoolVariableSequencedData(Vec<Box<HkbBoolVariableSequencedData>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"intVariableSequencedData"`
    /// -   type: `hkArray&lt;hkbIntVariableSequencedData*&gt;`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "intVariableSequencedData")]
    IntVariableSequencedData(Vec<Box<HkbIntVariableSequencedData>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"enableEventId"`
    /// -   type: `hkInt32`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enableEventId")]
    EnableEventId(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"disableEventId"`
    /// -   type: `hkInt32`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "disableEventId")]
    DisableEventId(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"stringData"`
    /// -   type: `struct hkbSequenceStringData*`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "stringData")]
    StringData(Box<HkbSequenceStringData>),
    /// # Information on fields in the original C++ class
    /// -   name:`"variableIdMap"`
    /// -   type: `void*`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "variableIdMap", skip_serializing)]
    VariableIdMap(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"eventIdMap"`
    /// -   type: `void*`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "eventIdMap", skip_serializing)]
    EventIdMap(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"nextSampleEvents"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "nextSampleEvents", skip_serializing)]
    NextSampleEvents(Vec<()>),
    /// # Information on fields in the original C++ class
    /// -   name:`"nextSampleReals"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "nextSampleReals", skip_serializing)]
    NextSampleReals(Vec<()>),
    /// # Information on fields in the original C++ class
    /// -   name:`"nextSampleBools"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "nextSampleBools", skip_serializing)]
    NextSampleBools(Vec<()>),
    /// # Information on fields in the original C++ class
    /// -   name:`"nextSampleInts"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "nextSampleInts", skip_serializing)]
    NextSampleInts(Vec<()>),
    /// # Information on fields in the original C++ class
    /// -   name:`"time"`
    /// -   type: `hkReal`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "time", skip_serializing)]
    Time(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"isEnabled"`
    /// -   type: `hkBool`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "isEnabled", skip_serializing)]
    IsEnabled(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbSequenceHkParam<'de>, "@name",
    ("eventSequencedData" => EventSequencedData(Vec<Box<HkbEventSequencedData>>)),
    ("realVariableSequencedData" => RealVariableSequencedData(Vec<Box<HkbRealVariableSequencedData>>)),
    ("boolVariableSequencedData" => BoolVariableSequencedData(Vec<Box<HkbBoolVariableSequencedData>>)),
    ("intVariableSequencedData" => IntVariableSequencedData(Vec<Box<HkbIntVariableSequencedData>>)),
    ("enableEventId" => EnableEventId(i32)),
    ("disableEventId" => DisableEventId(i32)),
    ("stringData" => StringData(Box<HkbSequenceStringData>)),
    ("variableIdMap" => VariableIdMap(())),
    ("eventIdMap" => EventIdMap(())),
    ("nextSampleEvents" => NextSampleEvents(Vec<()>)),
    ("nextSampleReals" => NextSampleReals(Vec<()>)),
    ("nextSampleBools" => NextSampleBools(Vec<()>)),
    ("nextSampleInts" => NextSampleInts(Vec<()>)),
    ("time" => Time(f64)),
    ("isEnabled" => IsEnabled(bool)),
}