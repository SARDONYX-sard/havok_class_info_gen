//! A Rust structure that implements a serializer/deserializer corresponding to `hkbManualSelectorGenerator`, a class defined in C++
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
/// -    size: 56
/// -  vtable: true
/// -  parent: hkbGenerator/`d68aefc`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbManualSelectorGenerator<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbManualSelectorGenerator"`: The original C++ class name.
    #[serde(default = "HkbManualSelectorGenerator::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xd932fab8`: Unique value of this class.
    #[serde(default = "HkbManualSelectorGenerator::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbManualSelectorGeneratorHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbManualSelectorGeneratorHkParam<'a>>
}

impl HkbManualSelectorGenerator<'_> {
    /// Return `"hkbManualSelectorGenerator"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbManualSelectorGenerator".into()
    }

    /// Return `"0xd932fab8"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xd932fab8".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbManualSelectorGeneratorHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"generators"`
    /// -   type: `hkArray&lt;hkbGenerator*&gt;`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "generators")]
    Generators(Vec<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"selectedGeneratorIndex"`
    /// -   type: `hkInt8`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "selectedGeneratorIndex")]
    SelectedGeneratorIndex(Primitive<i8>),
    /// # Field information in the original C++ class
    /// -   name:`"currentGeneratorIndex"`
    /// -   type: `hkInt8`
    /// - offset: 53
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "currentGeneratorIndex")]
    CurrentGeneratorIndex(Primitive<i8>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbManualSelectorGeneratorHkParam<'de>, "@name",
    ("generators" => Generators(Vec<Cow<'a, str>>)),
    ("selectedGeneratorIndex" => SelectedGeneratorIndex(Primitive<i8>)),
    ("currentGeneratorIndex" => CurrentGeneratorIndex(Primitive<i8>)),
}
