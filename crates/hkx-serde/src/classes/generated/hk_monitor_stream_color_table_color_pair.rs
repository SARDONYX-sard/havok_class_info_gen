//! A Rust structure that implements a serializer/deserializer corresponding to `hkMonitorStreamColorTableColorPair`, a class defined in C++
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
pub struct HkMonitorStreamColorTableColorPair<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkMonitorStreamColorTableColorPair"`: Name of this class.
    #[serde(default = "HkMonitorStreamColorTableColorPair::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x738fca05`: Unique value of this class.
    #[serde(default = "HkMonitorStreamColorTableColorPair::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkMonitorStreamColorTableColorPairHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkMonitorStreamColorTableColorPairHkParam<'a>>
}

impl HkMonitorStreamColorTableColorPair<'_> {
    /// Return `"hkMonitorStreamColorTableColorPair"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkMonitorStreamColorTableColorPair".into()
    }

    /// Return `"0x738fca05"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x738fca05".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMonitorStreamColorTableColorPairHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"colorName"`
    /// -   type: `hkStringPtr`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "colorName")]
    ColorName(String),
    /// # Information on fields in the original C++ class
    /// -   name:`"color"`
    /// -   type: `enum ExtendedColors`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "color")]
    Color(ExtendedColors),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkMonitorStreamColorTableColorPairHkParam<'de>, "@name",
    ("colorName" => ColorName(String)),
    ("color" => Color(ExtendedColors)),
}
