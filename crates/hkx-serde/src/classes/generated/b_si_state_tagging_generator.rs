//! A Rust structure that implements a serializer/deserializer corresponding to `BSiStateTaggingGenerator`, a class defined in C++
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
/// -  parent: hkbGenerator/`d68aefc`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct BSiStateTaggingGenerator<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"BSiStateTaggingGenerator"`: Name of this class.
    #[serde(default = "BSiStateTaggingGenerator::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xf0826fc1`: Unique value of this class.
    #[serde(default = "BSiStateTaggingGenerator::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<BSiStateTaggingGeneratorHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<BSiStateTaggingGeneratorHkParam<'a>>
}

impl BSiStateTaggingGenerator<'_> {
    /// Return `"BSiStateTaggingGenerator"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "BSiStateTaggingGenerator".into()
    }

    /// Return `"0xf0826fc1"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xf0826fc1".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BSiStateTaggingGeneratorHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"pDefaultGenerator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "pDefaultGenerator")]
    PDefaultGenerator(Box<HkbGenerator>),
    /// # Information on fields in the original C++ class
    /// -   name:`"iStateToSetAs"`
    /// -   type: `hkInt32`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "iStateToSetAs")]
    IStateToSetAs(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"iPriority"`
    /// -   type: `hkInt32`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "iPriority")]
    IPriority(i32),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    BSiStateTaggingGeneratorHkParam<'de>, "@name",
    ("pDefaultGenerator" => PDefaultGenerator(Box<HkbGenerator>)),
    ("iStateToSetAs" => IStateToSetAs(i32)),
    ("iPriority" => IPriority(i32)),
}
