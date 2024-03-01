//! A Rust structure that implements a serializer/deserializer corresponding to `hkpGroupCollisionFilter`, a class defined in C++
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
/// -    size: 180
/// -  vtable: true
/// -  parent: hkpCollisionFilter/`60960336`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpGroupCollisionFilter<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpGroupCollisionFilter"`: Name of this class.
    #[serde(default = "HkpGroupCollisionFilter::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x5cc01561`: Unique value of this class.
    #[serde(default = "HkpGroupCollisionFilter::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpGroupCollisionFilterHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpGroupCollisionFilterHkParam<'a>>
}

impl HkpGroupCollisionFilter<'_> {
    /// Return `"hkpGroupCollisionFilter"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpGroupCollisionFilter".into()
    }

    /// Return `"0x5cc01561"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x5cc01561".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpGroupCollisionFilterHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"noGroupCollisionEnabled"`
    /// -   type: `hkBool`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "noGroupCollisionEnabled")]
    NoGroupCollisionEnabled(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"collisionGroups"`
    /// -   type: `hkUint32[32]`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collisionGroups")]
    CollisionGroups([u32; 32]),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpGroupCollisionFilterHkParam<'de>, "@name",
    ("noGroupCollisionEnabled" => NoGroupCollisionEnabled(bool)),
    ("collisionGroups" => CollisionGroups([u32; 32])),
}
