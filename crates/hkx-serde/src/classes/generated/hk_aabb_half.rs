//! A Rust structure that implements a serializer/deserializer corresponding to `hkAabbHalf`, a class defined in C++
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
/// -    size: 16
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkAabbHalf<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkAabbHalf"`: Name of this class.
    #[serde(default = "HkAabbHalf::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x1d716a17`: Unique value of this class.
    #[serde(default = "HkAabbHalf::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkAabbHalfHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkAabbHalfHkParam<'a>>
}

impl HkAabbHalf<'_> {
    /// Return `"hkAabbHalf"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkAabbHalf".into()
    }

    /// Return `"0x1d716a17"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x1d716a17".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkAabbHalfHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"data"`
    /// -   type: `hkUint16[6]`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data([u16; 6]),
    /// # Information on fields in the original C++ class
    /// -   name:`"extras"`
    /// -   type: `hkUint16[2]`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extras")]
    Extras([u16; 2]),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkAabbHalfHkParam<'de>, "@name",
    ("data" => Data([u16; 6])),
    ("extras" => Extras([u16; 2])),
}
