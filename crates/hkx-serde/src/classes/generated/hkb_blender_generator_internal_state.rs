//! A Rust structure that implements a serializer/deserializer corresponding to `hkbBlenderGeneratorInternalState`, a class defined in C++
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
/// -    size: 48
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbBlenderGeneratorInternalState<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbBlenderGeneratorInternalState"`: The original C++ class name.
    #[serde(default = "HkbBlenderGeneratorInternalState::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x84717488`: Unique value of this class.
    #[serde(default = "HkbBlenderGeneratorInternalState::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbBlenderGeneratorInternalStateHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbBlenderGeneratorInternalStateHkParam<'a>>
}

impl HkbBlenderGeneratorInternalState<'_> {
    /// Return `"hkbBlenderGeneratorInternalState"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbBlenderGeneratorInternalState".into()
    }

    /// Return `"0x84717488"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x84717488".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbBlenderGeneratorInternalStateHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"childrenInternalStates"`
    /// -   type: `hkArray&lt;struct hkbBlenderGeneratorChildInternalState&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childrenInternalStates")]
    ChildrenInternalStates(Vec<HkbBlenderGeneratorChildInternalState>),
    /// # Field information in the original C++ class
    /// -   name:`"sortedChildren"`
    /// -   type: `hkArray&lt;hkInt16&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sortedChildren")]
    SortedChildren(Vec<Primitive<i16>>),
    /// # Field information in the original C++ class
    /// -   name:`"endIntervalWeight"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "endIntervalWeight")]
    EndIntervalWeight(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"numActiveChildren"`
    /// -   type: `hkInt32`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numActiveChildren")]
    NumActiveChildren(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"beginIntervalIndex"`
    /// -   type: `hkInt16`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "beginIntervalIndex")]
    BeginIntervalIndex(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"endIntervalIndex"`
    /// -   type: `hkInt16`
    /// - offset: 42
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "endIntervalIndex")]
    EndIntervalIndex(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"initSync"`
    /// -   type: `hkBool`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "initSync")]
    InitSync(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"doSubtractiveBlend"`
    /// -   type: `hkBool`
    /// - offset: 45
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "doSubtractiveBlend")]
    DoSubtractiveBlend(Primitive<bool>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbBlenderGeneratorInternalStateHkParam<'de>, "@name",
    ("childrenInternalStates" => ChildrenInternalStates(Vec<HkbBlenderGeneratorChildInternalState>)),
    ("sortedChildren" => SortedChildren(Vec<Primitive<i16>>)),
    ("endIntervalWeight" => EndIntervalWeight(Primitive<f32>)),
    ("numActiveChildren" => NumActiveChildren(Primitive<i32>)),
    ("beginIntervalIndex" => BeginIntervalIndex(Primitive<i16>)),
    ("endIntervalIndex" => EndIntervalIndex(Primitive<i16>)),
    ("initSync" => InitSync(Primitive<bool>)),
    ("doSubtractiveBlend" => DoSubtractiveBlend(Primitive<bool>)),
}
