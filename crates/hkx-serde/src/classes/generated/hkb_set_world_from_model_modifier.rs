//! A Rust structure that implements a serializer/deserializer corresponding to `hkbSetWorldFromModelModifier`, a class defined in C++
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
/// -    size: 96
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbSetWorldFromModelModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbSetWorldFromModelModifier"`: Name of this class.
    #[serde(default = "HkbSetWorldFromModelModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xafcfa211`: Unique value of this class.
    #[serde(default = "HkbSetWorldFromModelModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbSetWorldFromModelModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbSetWorldFromModelModifierHkParam<'a>>
}

impl HkbSetWorldFromModelModifier<'_> {
    /// Return `"hkbSetWorldFromModelModifier"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbSetWorldFromModelModifier".into()
    }

    /// Return `"0xafcfa211"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xafcfa211".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbSetWorldFromModelModifierHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"translation"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "translation")]
    Translation(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"rotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotation")]
    Rotation(cgmath::Quaternion<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"setTranslation"`
    /// -   type: `hkBool`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "setTranslation")]
    SetTranslation(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"setRotation"`
    /// -   type: `hkBool`
    /// - offset: 81
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "setRotation")]
    SetRotation(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbSetWorldFromModelModifierHkParam<'de>, "@name",
    ("translation" => Translation(cgmath::Vector4<f32>)),
    ("rotation" => Rotation(cgmath::Quaternion<f32>)),
    ("setTranslation" => SetTranslation(bool)),
    ("setRotation" => SetRotation(bool)),
}
