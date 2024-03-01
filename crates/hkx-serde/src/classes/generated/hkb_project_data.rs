//! A Rust structure that implements a serializer/deserializer corresponding to `hkbProjectData`, a class defined in C++
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
/// -    size: 48
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 2
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbProjectData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbProjectData"`: Name of this class.
    #[serde(default = "HkbProjectData::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x13a39ba7`: Unique value of this class.
    #[serde(default = "HkbProjectData::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbProjectDataHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbProjectDataHkParam<'a>>
}

impl HkbProjectData<'_> {
    /// Return `"hkbProjectData"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbProjectData".into()
    }

    /// Return `"0x13a39ba7"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x13a39ba7".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbProjectDataHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"worldUpWS"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "worldUpWS")]
    WorldUpWs(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"stringData"`
    /// -   type: `struct hkbProjectStringData*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "stringData")]
    StringData(Box<HkbProjectStringData>),
    /// # Information on fields in the original C++ class
    /// -   name:`"defaultEventMode"`
    /// -   type: `enum EventMode`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "defaultEventMode")]
    DefaultEventMode(EventMode),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbProjectDataHkParam<'de>, "@name",
    ("worldUpWS" => WorldUpWs(cgmath::Vector4<f32>)),
    ("stringData" => StringData(Box<HkbProjectStringData>)),
    ("defaultEventMode" => DefaultEventMode(EventMode)),
}
