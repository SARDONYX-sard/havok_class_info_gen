//! A Rust structure that implements a serializer/deserializer corresponding to `hkMonitorStreamStringMapStringMap`, a class defined in C++
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
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkMonitorStreamStringMapStringMap<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkMonitorStreamStringMapStringMap"`: The original C++ class name.
    #[serde(default = "HkMonitorStreamStringMapStringMap::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x2c76ce16`: Unique value of this class.
    #[serde(default = "HkMonitorStreamStringMapStringMap::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkMonitorStreamStringMapStringMapHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkMonitorStreamStringMapStringMapHkParam<'a>>
}

impl HkMonitorStreamStringMapStringMap<'_> {
    /// Return `"hkMonitorStreamStringMapStringMap"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkMonitorStreamStringMapStringMap".into()
    }

    /// Return `"0x2c76ce16"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x2c76ce16".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMonitorStreamStringMapStringMapHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"id"`
    /// -   type: `hkUint64`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE | ALIGN8`
    #[serde(rename = "id")]
    Id(Primitive<u64>),
    /// # Field information in the original C++ class
    /// -   name:`"string"`
    /// -   type: `hkStringPtr`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "string")]
    String(Primitive<Cow<'a, str>>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkMonitorStreamStringMapStringMapHkParam<'de>, "@name",
    ("id" => Id(Primitive<u64>)),
    ("string" => String(Primitive<Cow<'a, str>>)),
}
