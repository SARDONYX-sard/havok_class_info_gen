//! A Rust structure that implements a serializer/deserializer corresponding to `hkpPairCollisionFilter`, a class defined in C++
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
/// -  parent: hkpCollisionFilter/`60960336`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpPairCollisionFilter<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpPairCollisionFilter"`: Name of this class.
    #[serde(default = "HkpPairCollisionFilter::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x4abc140e`: Unique value of this class.
    #[serde(default = "HkpPairCollisionFilter::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpPairCollisionFilterHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpPairCollisionFilterHkParam<'a>>
}

impl HkpPairCollisionFilter<'_> {
    /// Return `"hkpPairCollisionFilter"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpPairCollisionFilter".into()
    }

    /// Return `"0x4abc140e"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x4abc140e".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPairCollisionFilterHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"disabledPairs"`
    /// -   type: `struct hkpPairCollisionFilterMapPairFilterKeyOverrideType`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "disabledPairs", skip_serializing)]
    DisabledPairs(HkpPairCollisionFilterMapPairFilterKeyOverrideType),
    /// # Information on fields in the original C++ class
    /// -   name:`"childFilter"`
    /// -   type: `struct hkpCollisionFilter*`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childFilter")]
    ChildFilter(Box<HkpCollisionFilter>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpPairCollisionFilterHkParam<'de>, "@name",
    ("disabledPairs" => DisabledPairs(HkpPairCollisionFilterMapPairFilterKeyOverrideType)),
    ("childFilter" => ChildFilter(Box<HkpCollisionFilter>)),
}
