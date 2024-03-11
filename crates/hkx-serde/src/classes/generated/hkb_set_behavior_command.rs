//! A Rust structure that implements a serializer/deserializer corresponding to `hkbSetBehaviorCommand`, a class defined in C++
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
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbSetBehaviorCommand<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbSetBehaviorCommand"`: The original C++ class name.
    #[serde(default = "HkbSetBehaviorCommand::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xe18b74b9`: Unique value of this class.
    #[serde(default = "HkbSetBehaviorCommand::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbSetBehaviorCommandHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbSetBehaviorCommandHkParam<'a>>
}

impl HkbSetBehaviorCommand<'_> {
    /// Return `"hkbSetBehaviorCommand"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbSetBehaviorCommand".into()
    }

    /// Return `"0xe18b74b9"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xe18b74b9".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbSetBehaviorCommandHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"characterId"`
    /// -   type: `hkUint64`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterId")]
    CharacterId(Primitive<u64>),
    /// # Field information in the original C++ class
    /// -   name:`"behavior"`
    /// -   type: `struct hkbBehaviorGraph*`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "behavior")]
    Behavior(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"rootGenerator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rootGenerator")]
    RootGenerator(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"referencedBehaviors"`
    /// -   type: `hkArray&lt;hkbBehaviorGraph*&gt;`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "referencedBehaviors")]
    ReferencedBehaviors(Vec<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"startStateIndex"`
    /// -   type: `hkInt32`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "startStateIndex")]
    StartStateIndex(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"randomizeSimulation"`
    /// -   type: `hkBool`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "randomizeSimulation")]
    RandomizeSimulation(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"padding"`
    /// -   type: `hkInt32`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "padding")]
    Padding(Primitive<i32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbSetBehaviorCommandHkParam<'de>, "@name",
    ("characterId" => CharacterId(Primitive<u64>)),
    ("behavior" => Behavior(Cow<'a, str>)),
    ("rootGenerator" => RootGenerator(Cow<'a, str>)),
    ("referencedBehaviors" => ReferencedBehaviors(Vec<Cow<'a, str>>)),
    ("startStateIndex" => StartStateIndex(Primitive<i32>)),
    ("randomizeSimulation" => RandomizeSimulation(Primitive<bool>)),
    ("padding" => Padding(Primitive<i32>)),
}
