//! A Rust structure that implements a serializer/deserializer corresponding to `hkbClipTrigger`, a class defined in C++
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
/// -    size: 16
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbClipTrigger<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbClipTrigger"`: The original C++ class name.
    #[serde(default = "HkbClipTrigger::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x7eb45cea`: Unique value of this class.
    #[serde(default = "HkbClipTrigger::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbClipTriggerHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbClipTriggerHkParam<'a>>
}

impl HkbClipTrigger<'_> {
    /// Return `"hkbClipTrigger"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbClipTrigger".into()
    }

    /// Return `"0x7eb45cea"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x7eb45cea".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbClipTriggerHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"localTime"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "localTime")]
    LocalTime(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"event"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "event")]
    Event(HkbEventProperty),
    /// # Field information in the original C++ class
    /// -   name:`"relativeToEndOfClip"`
    /// -   type: `hkBool`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "relativeToEndOfClip")]
    RelativeToEndOfClip(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"acyclic"`
    /// -   type: `hkBool`
    /// - offset: 13
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "acyclic")]
    Acyclic(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"isAnnotation"`
    /// -   type: `hkBool`
    /// - offset: 14
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isAnnotation")]
    IsAnnotation(Primitive<bool>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbClipTriggerHkParam<'de>, "@name",
    ("localTime" => LocalTime(Primitive<f32>)),
    ("event" => Event(HkbEventProperty)),
    ("relativeToEndOfClip" => RelativeToEndOfClip(Primitive<bool>)),
    ("acyclic" => Acyclic(Primitive<bool>)),
    ("isAnnotation" => IsAnnotation(Primitive<bool>)),
}
