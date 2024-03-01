//! A Rust structure that implements a serializer/deserializer corresponding to `hkxSparselyAnimatedEnum`, a class defined in C++
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
/// -    size: 36
/// -  vtable: true
/// -  parent: hkxSparselyAnimatedInt/`ca961951`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkxSparselyAnimatedEnum<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkxSparselyAnimatedEnum"`: Name of this class.
    #[serde(default = "HkxSparselyAnimatedEnum::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x68a47b64`: Unique value of this class.
    #[serde(default = "HkxSparselyAnimatedEnum::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkxSparselyAnimatedEnumHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkxSparselyAnimatedEnumHkParam<'a>>
}

impl HkxSparselyAnimatedEnum<'_> {
    /// Return `"hkxSparselyAnimatedEnum"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkxSparselyAnimatedEnum".into()
    }

    /// Return `"0x68a47b64"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x68a47b64".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxSparselyAnimatedEnumHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"enum"`
    /// -   type: `struct hkxEnum*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enum")]
    Enum(Box<HkxEnum>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkxSparselyAnimatedEnumHkParam<'de>, "@name",
    ("enum" => Enum(Box<HkxEnum>)),
}
