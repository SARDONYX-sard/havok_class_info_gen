//! A Rust structure that implements a serializer/deserializer corresponding to `BSModifyOnceModifier`, a class defined in C++
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
/// -    size: 80
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct BsModifyOnceModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"BSModifyOnceModifier"`: Name of this class.
    #[serde(default = "BsModifyOnceModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x1e20a97a`: Unique value of this class.
    #[serde(default = "BsModifyOnceModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<BsModifyOnceModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<BsModifyOnceModifierHkParam<'a>>
}

impl BsModifyOnceModifier<'_> {
    /// Return `"BSModifyOnceModifier"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "BsModifyOnceModifier".into()
    }

    /// Return `"0x1e20a97a"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x1e20a97a".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsModifyOnceModifierHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"pOnActivateModifier"`
    /// -   type: `struct hkbModifier*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "pOnActivateModifier")]
    POnActivateModifier(Box<HkbModifier>),
    /// # Information on fields in the original C++ class
    /// -   name:`"pOnDeactivateModifier"`
    /// -   type: `struct hkbModifier*`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "pOnDeactivateModifier")]
    POnDeactivateModifier(Box<HkbModifier>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    BsModifyOnceModifierHkParam<'de>, "@name",
    ("pOnActivateModifier" => POnActivateModifier(Box<HkbModifier>)),
    ("pOnDeactivateModifier" => POnDeactivateModifier(Box<HkbModifier>)),
}
