//! A Rust structure that implements a serializer/deserializer corresponding to `hkSemanticsAttribute`, a class defined in C++
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
/// -    size: 1
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkSemanticsAttribute<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkSemanticsAttribute"`: Name of this class.
    #[serde(default = "HkSemanticsAttribute::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x837099c3`: Unique value of this class.
    #[serde(default = "HkSemanticsAttribute::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkSemanticsAttributeHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkSemanticsAttributeHkParam<'a>>
}

impl HkSemanticsAttribute<'_> {
    /// Return `"hkSemanticsAttribute"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkSemanticsAttribute".into()
    }

    /// Return `"0x837099c3"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x837099c3".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkSemanticsAttributeHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"type"`
    /// -   type: `enum Semantics`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Semantics),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkSemanticsAttributeHkParam<'de>, "@name",
    ("type" => Type(Semantics)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Semantics {
    #[serde(rename = "UNKNOWN")]
    Unknown = 0,
    #[serde(rename = "DISTANCE")]
    Distance = 1,
    #[serde(rename = "ANGLE")]
    Angle = 2,
    #[serde(rename = "NORMAL")]
    Normal = 3,
    #[serde(rename = "POSITION")]
    Position = 4,
    #[serde(rename = "COSINE_ANGLE")]
    CosineAngle = 5,
}
