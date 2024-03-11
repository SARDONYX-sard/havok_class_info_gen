//! A Rust structure that implements a serializer/deserializer corresponding to `hkPackfileSectionHeader`, a class defined in C++
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
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkPackfileSectionHeader<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkPackfileSectionHeader"`: The original C++ class name.
    #[serde(default = "HkPackfileSectionHeader::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xf2a92154`: Unique value of this class.
    #[serde(default = "HkPackfileSectionHeader::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkPackfileSectionHeaderHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkPackfileSectionHeaderHkParam<'a>>
}

impl HkPackfileSectionHeader<'_> {
    /// Return `"hkPackfileSectionHeader"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkPackfileSectionHeader".into()
    }

    /// Return `"0xf2a92154"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xf2a92154".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkPackfileSectionHeaderHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"sectionTag"`
    /// -   type: `hkChar[19]`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sectionTag")]
    SectionTag([Primitive<char>; 19]),
    /// # Field information in the original C++ class
    /// -   name:`"nullByte"`
    /// -   type: `hkChar`
    /// - offset: 19
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nullByte")]
    NullByte(Primitive<char>),
    /// # Field information in the original C++ class
    /// -   name:`"absoluteDataStart"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "absoluteDataStart")]
    AbsoluteDataStart(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"localFixupsOffset"`
    /// -   type: `hkInt32`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "localFixupsOffset")]
    LocalFixupsOffset(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"globalFixupsOffset"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "globalFixupsOffset")]
    GlobalFixupsOffset(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"virtualFixupsOffset"`
    /// -   type: `hkInt32`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "virtualFixupsOffset")]
    VirtualFixupsOffset(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"exportsOffset"`
    /// -   type: `hkInt32`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "exportsOffset")]
    ExportsOffset(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"importsOffset"`
    /// -   type: `hkInt32`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "importsOffset")]
    ImportsOffset(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"endOffset"`
    /// -   type: `hkInt32`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "endOffset")]
    EndOffset(Primitive<i32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkPackfileSectionHeaderHkParam<'de>, "@name",
    ("sectionTag" => SectionTag([Primitive<char>; 19])),
    ("nullByte" => NullByte(Primitive<char>)),
    ("absoluteDataStart" => AbsoluteDataStart(Primitive<i32>)),
    ("localFixupsOffset" => LocalFixupsOffset(Primitive<i32>)),
    ("globalFixupsOffset" => GlobalFixupsOffset(Primitive<i32>)),
    ("virtualFixupsOffset" => VirtualFixupsOffset(Primitive<i32>)),
    ("exportsOffset" => ExportsOffset(Primitive<i32>)),
    ("importsOffset" => ImportsOffset(Primitive<i32>)),
    ("endOffset" => EndOffset(Primitive<i32>)),
}
