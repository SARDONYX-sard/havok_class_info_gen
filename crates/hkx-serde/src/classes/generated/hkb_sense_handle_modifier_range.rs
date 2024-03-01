//! A Rust structure that implements a serializer/deserializer corresponding to `hkbSenseHandleModifierRange`, a class defined in C++
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
/// -    size: 20
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbSenseHandleModifierRange<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbSenseHandleModifierRange"`: Name of this class.
    #[serde(default = "HkbSenseHandleModifierRange::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xfb56b692`: Unique value of this class.
    #[serde(default = "HkbSenseHandleModifierRange::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbSenseHandleModifierRangeHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbSenseHandleModifierRangeHkParam<'a>>
}

impl HkbSenseHandleModifierRange<'_> {
    /// Return `"hkbSenseHandleModifierRange"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbSenseHandleModifierRange".into()
    }

    /// Return `"0xfb56b692"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xfb56b692".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbSenseHandleModifierRangeHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"event"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "event")]
    Event(HkbEventProperty),
    /// # Information on fields in the original C++ class
    /// -   name:`"minDistance"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minDistance")]
    MinDistance(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxDistance"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxDistance")]
    MaxDistance(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"ignoreHandle"`
    /// -   type: `hkBool`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ignoreHandle")]
    IgnoreHandle(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbSenseHandleModifierRangeHkParam<'de>, "@name",
    ("event" => Event(HkbEventProperty)),
    ("minDistance" => MinDistance(f64)),
    ("maxDistance" => MaxDistance(f64)),
    ("ignoreHandle" => IgnoreHandle(bool)),
}
