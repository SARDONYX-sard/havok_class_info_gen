//! A Rust structure that implements a serializer/deserializer corresponding to `hkClassEnum`, a class defined in C++
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
/// -    size: 20
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkClassEnum<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkClassEnum"`: Name of this class.
    #[serde(default = "HkClassEnum::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x8a3609cf`: Unique value of this class.
    #[serde(default = "HkClassEnum::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkClassEnumHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkClassEnumHkParam<'a>>
}

impl HkClassEnum<'_> {
    /// Return `"hkClassEnum"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkClassEnum".into()
    }

    /// Return `"0x8a3609cf"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x8a3609cf".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkClassEnumHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"name"`
    /// -   type: `char*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(String),
    /// # Information on fields in the original C++ class
    /// -   name:`"items"`
    /// -   type: `hkSimpleArray&lt;struct hkClassEnumItem&gt;`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "items")]
    Items(Vec<HkClassEnumItem>),
    /// # Information on fields in the original C++ class
    /// -   name:`"attributes"`
    /// -   type: `struct hkCustomAttributes*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "attributes", skip_serializing)]
    Attributes(Box<HkCustomAttributes>),
    /// # Information on fields in the original C++ class
    /// -   name:`"flags"`
    /// -   type: `flags FlagValues`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "flags")]
    Flags(FlagValues),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkClassEnumHkParam<'de>, "@name",
    ("name" => Name(String)),
    ("items" => Items(Vec<HkClassEnumItem>)),
    ("attributes" => Attributes(Box<HkCustomAttributes>)),
    ("flags" => Flags(FlagValues)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FlagValues {
    #[serde(rename = "FLAGS_NONE")]
    FlagsNone = 0,
}
