//! A Rust structure that implements a serializer/deserializer corresponding to `hkpCollisionFilter`, a class defined in C++
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
/// -    size: 48
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpCollisionFilter<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpCollisionFilter"`: The original C++ class name.
    #[serde(default = "HkpCollisionFilter::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x60960336`: Unique value of this class.
    #[serde(default = "HkpCollisionFilter::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpCollisionFilterHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpCollisionFilterHkParam<'a>>
}

impl HkpCollisionFilter<'_> {
    /// Return `"hkpCollisionFilter"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpCollisionFilter".into()
    }

    /// Return `"0x60960336"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x60960336".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpCollisionFilterHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"prepad"`
    /// -   type: `hkUint32[2]`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "prepad")]
    Prepad([Primitive<u32>; 2]),
    /// # Field information in the original C++ class
    /// -   name:`"type"`
    /// -   type: `enum hkpFilterType`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(HkpFilterType),
    /// # Field information in the original C++ class
    /// -   name:`"postpad"`
    /// -   type: `hkUint32[3]`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "postpad")]
    Postpad([Primitive<u32>; 3]),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpCollisionFilterHkParam<'de>, "@name",
    ("prepad" => Prepad([Primitive<u32>; 2])),
    ("type" => Type(HkpFilterType)),
    ("postpad" => Postpad([Primitive<u32>; 3])),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum hkpFilterType {
    #[serde(rename = "HK_FILTER_UNKNOWN")]
    HkFilterUnknown = 0,
    #[serde(rename = "HK_FILTER_NULL")]
    HkFilterNull = 1,
    #[serde(rename = "HK_FILTER_GROUP")]
    HkFilterGroup = 2,
    #[serde(rename = "HK_FILTER_LIST")]
    HkFilterList = 3,
    #[serde(rename = "HK_FILTER_CUSTOM")]
    HkFilterCustom = 4,
    #[serde(rename = "HK_FILTER_PAIR")]
    HkFilterPair = 5,
    #[serde(rename = "HK_FILTER_CONSTRAINT")]
    HkFilterConstraint = 6,
}
