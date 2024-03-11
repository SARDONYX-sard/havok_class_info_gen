//! A Rust structure that implements a serializer/deserializer corresponding to `hkPackfileHeader`, a class defined in C++
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
/// -    size: 64
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkPackfileHeader<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkPackfileHeader"`: The original C++ class name.
    #[serde(default = "HkPackfileHeader::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x79f9ffda`: Unique value of this class.
    #[serde(default = "HkPackfileHeader::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkPackfileHeaderHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkPackfileHeaderHkParam<'a>>
}

impl HkPackfileHeader<'_> {
    /// Return `"hkPackfileHeader"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkPackfileHeader".into()
    }

    /// Return `"0x79f9ffda"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x79f9ffda".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkPackfileHeaderHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"magic"`
    /// -   type: `hkInt32[2]`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "magic")]
    Magic([Primitive<i32>; 2]),
    /// # Field information in the original C++ class
    /// -   name:`"userTag"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userTag")]
    UserTag(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"fileVersion"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fileVersion")]
    FileVersion(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"layoutRules"`
    /// -   type: `hkUint8[4]`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "layoutRules")]
    LayoutRules([Primitive<u8>; 4]),
    /// # Field information in the original C++ class
    /// -   name:`"numSections"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numSections")]
    NumSections(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"contentsSectionIndex"`
    /// -   type: `hkInt32`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contentsSectionIndex")]
    ContentsSectionIndex(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"contentsSectionOffset"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contentsSectionOffset")]
    ContentsSectionOffset(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"contentsClassNameSectionIndex"`
    /// -   type: `hkInt32`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contentsClassNameSectionIndex")]
    ContentsClassNameSectionIndex(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"contentsClassNameSectionOffset"`
    /// -   type: `hkInt32`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contentsClassNameSectionOffset")]
    ContentsClassNameSectionOffset(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"contentsVersion"`
    /// -   type: `hkChar[16]`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contentsVersion")]
    ContentsVersion([Primitive<char>; 16]),
    /// # Field information in the original C++ class
    /// -   name:`"flags"`
    /// -   type: `hkInt32`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "flags")]
    Flags(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"pad"`
    /// -   type: `hkInt32[1]`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pad")]
    Pad([Primitive<i32>; 1]),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkPackfileHeaderHkParam<'de>, "@name",
    ("magic" => Magic([Primitive<i32>; 2])),
    ("userTag" => UserTag(Primitive<i32>)),
    ("fileVersion" => FileVersion(Primitive<i32>)),
    ("layoutRules" => LayoutRules([Primitive<u8>; 4])),
    ("numSections" => NumSections(Primitive<i32>)),
    ("contentsSectionIndex" => ContentsSectionIndex(Primitive<i32>)),
    ("contentsSectionOffset" => ContentsSectionOffset(Primitive<i32>)),
    ("contentsClassNameSectionIndex" => ContentsClassNameSectionIndex(Primitive<i32>)),
    ("contentsClassNameSectionOffset" => ContentsClassNameSectionOffset(Primitive<i32>)),
    ("contentsVersion" => ContentsVersion([Primitive<char>; 16])),
    ("flags" => Flags(Primitive<i32>)),
    ("pad" => Pad([Primitive<i32>; 1])),
}
