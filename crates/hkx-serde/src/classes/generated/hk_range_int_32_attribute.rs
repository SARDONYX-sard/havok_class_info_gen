//! A Rust structure that implements a serializer/deserializer corresponding to `hkRangeInt32Attribute`, a class defined in C++
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
pub struct HkRangeInt32Attribute<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkRangeInt32Attribute"`: Name of this class.
    #[serde(default = "HkRangeInt32Attribute::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x4846be29`: Unique value of this class.
    #[serde(default = "HkRangeInt32Attribute::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkRangeInt32AttributeHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkRangeInt32AttributeHkParam<'a>>
}

impl HkRangeInt32Attribute<'_> {
    /// Return `"hkRangeInt32Attribute"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkRangeInt32Attribute".into()
    }

    /// Return `"0x4846be29"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x4846be29".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkRangeInt32AttributeHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"absmin"`
    /// -   type: `hkInt32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "absmin")]
    Absmin(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"absmax"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "absmax")]
    Absmax(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"softmin"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "softmin")]
    Softmin(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"softmax"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "softmax")]
    Softmax(i32),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkRangeInt32AttributeHkParam<'de>, "@name",
    ("absmin" => Absmin(i32)),
    ("absmax" => Absmax(i32)),
    ("softmin" => Softmin(i32)),
    ("softmax" => Softmax(i32)),
}
