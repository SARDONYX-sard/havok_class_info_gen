//! A Rust structure that implements a serializer/deserializer corresponding to `hkaBone`, a class defined in C++
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
/// -    size: 8
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkaBone<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkaBone"`: Name of this class.
    #[serde(default = "HkaBone::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x35912f8a`: Unique value of this class.
    #[serde(default = "HkaBone::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkaBoneHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkaBoneHkParam<'a>>
}

impl HkaBone<'_> {
    /// Return `"hkaBone"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkaBone".into()
    }

    /// Return `"0x35912f8a"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x35912f8a".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaBoneHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(String),
    /// # Information on fields in the original C++ class
    /// -   name:`"lockTranslation"`
    /// -   type: `hkBool`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lockTranslation")]
    LockTranslation(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkaBoneHkParam<'de>, "@name",
    ("name" => Name(String)),
    ("lockTranslation" => LockTranslation(bool)),
}
