//! A Rust structure that implements a serializer/deserializer corresponding to `hkpProperty`, a class defined in C++
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
/// -    size: 16
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpProperty<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpProperty"`: The original C++ class name.
    #[serde(default = "HkpProperty::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x9ce308e9`: Unique value of this class.
    #[serde(default = "HkpProperty::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpPropertyHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpPropertyHkParam<'a>>
}

impl HkpProperty<'_> {
    /// Return `"hkpProperty"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpProperty".into()
    }

    /// Return `"0x9ce308e9"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x9ce308e9".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPropertyHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"key"`
    /// -   type: `hkUint32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "key")]
    Key(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"alignmentPadding"`
    /// -   type: `hkUint32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "alignmentPadding")]
    AlignmentPadding(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"value"`
    /// -   type: `struct hkpPropertyValue`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "value")]
    Value(HkpPropertyValue),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpPropertyHkParam<'de>, "@name",
    ("key" => Key(Primitive<u32>)),
    ("alignmentPadding" => AlignmentPadding(Primitive<u32>)),
    ("value" => Value(HkpPropertyValue)),
}
