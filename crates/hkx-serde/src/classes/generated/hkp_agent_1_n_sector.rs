//! A Rust structure that implements a serializer/deserializer corresponding to `hkpAgent1nSector`, a class defined in C++
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
/// -    size: 512
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpAgent1NSector<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpAgent1nSector"`: The original C++ class name.
    #[serde(default = "HkpAgent1NSector::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x626e55a`: Unique value of this class.
    #[serde(default = "HkpAgent1NSector::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpAgent1NSectorHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpAgent1NSectorHkParam<'a>>
}

impl HkpAgent1NSector<'_> {
    /// Return `"hkpAgent1nSector"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpAgent1nSector".into()
    }

    /// Return `"0x626e55a"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x626e55a".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpAgent1NSectorHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"bytesAllocated"`
    /// -   type: `hkUint32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bytesAllocated")]
    BytesAllocated(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"pad0"`
    /// -   type: `hkUint32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pad0")]
    Pad0(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"pad1"`
    /// -   type: `hkUint32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pad1")]
    Pad1(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"pad2"`
    /// -   type: `hkUint32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pad2")]
    Pad2(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"data"`
    /// -   type: `hkUint8[496]`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data([Primitive<u8>; 496]),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpAgent1NSectorHkParam<'de>, "@name",
    ("bytesAllocated" => BytesAllocated(Primitive<u32>)),
    ("pad0" => Pad0(Primitive<u32>)),
    ("pad1" => Pad1(Primitive<u32>)),
    ("pad2" => Pad2(Primitive<u32>)),
    ("data" => Data([Primitive<u8>; 496])),
}
