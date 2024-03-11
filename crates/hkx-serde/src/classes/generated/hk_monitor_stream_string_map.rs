//! A Rust structure that implements a serializer/deserializer corresponding to `hkMonitorStreamStringMap`, a class defined in C++
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
/// -    size: 12
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkMonitorStreamStringMap<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkMonitorStreamStringMap"`: The original C++ class name.
    #[serde(default = "HkMonitorStreamStringMap::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xc4d3a8b4`: Unique value of this class.
    #[serde(default = "HkMonitorStreamStringMap::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkMonitorStreamStringMapHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkMonitorStreamStringMapHkParam<'a>>
}

impl HkMonitorStreamStringMap<'_> {
    /// Return `"hkMonitorStreamStringMap"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkMonitorStreamStringMap".into()
    }

    /// Return `"0xc4d3a8b4"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xc4d3a8b4".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMonitorStreamStringMapHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"map"`
    /// -   type: `hkArray&lt;struct hkMonitorStreamStringMapStringMap&gt;`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "map")]
    Map(Vec<HkMonitorStreamStringMapStringMap>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkMonitorStreamStringMapHkParam<'de>, "@name",
    ("map" => Map(Vec<HkMonitorStreamStringMapStringMap>)),
}
