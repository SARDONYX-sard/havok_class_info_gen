//! A Rust structure that implements a serializer/deserializer corresponding to `hkAabbUint32`, a class defined in C++
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
/// -    size: 32
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkAabbUint32<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkAabbUint32"`: The original C++ class name.
    #[serde(default = "HkAabbUint32::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x11e7c11`: Unique value of this class.
    #[serde(default = "HkAabbUint32::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkAabbUint32HkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkAabbUint32HkParam<'a>>
}

impl HkAabbUint32<'_> {
    /// Return `"hkAabbUint32"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkAabbUint32".into()
    }

    /// Return `"0x11e7c11"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x11e7c11".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkAabbUint32HkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"min"`
    /// -   type: `hkUint32[3]`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "min")]
    Min([Primitive<u32>; 3]),
    /// # Field information in the original C++ class
    /// -   name:`"expansionMin"`
    /// -   type: `hkUint8[3]`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "expansionMin")]
    ExpansionMin([Primitive<u8>; 3]),
    /// # Field information in the original C++ class
    /// -   name:`"expansionShift"`
    /// -   type: `hkUint8`
    /// - offset: 15
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "expansionShift")]
    ExpansionShift(Primitive<u8>),
    /// # Field information in the original C++ class
    /// -   name:`"max"`
    /// -   type: `hkUint32[3]`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "max")]
    Max([Primitive<u32>; 3]),
    /// # Field information in the original C++ class
    /// -   name:`"expansionMax"`
    /// -   type: `hkUint8[3]`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "expansionMax")]
    ExpansionMax([Primitive<u8>; 3]),
    /// # Field information in the original C++ class
    /// -   name:`"shapeKeyByte"`
    /// -   type: `hkUint8`
    /// - offset: 31
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shapeKeyByte")]
    ShapeKeyByte(Primitive<u8>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkAabbUint32HkParam<'de>, "@name",
    ("min" => Min([Primitive<u32>; 3])),
    ("expansionMin" => ExpansionMin([Primitive<u8>; 3])),
    ("expansionShift" => ExpansionShift(Primitive<u8>)),
    ("max" => Max([Primitive<u32>; 3])),
    ("expansionMax" => ExpansionMax([Primitive<u8>; 3])),
    ("shapeKeyByte" => ShapeKeyByte(Primitive<u8>)),
}
