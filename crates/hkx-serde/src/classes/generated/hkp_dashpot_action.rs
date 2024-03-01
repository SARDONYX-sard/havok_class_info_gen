//! A Rust structure that implements a serializer/deserializer corresponding to `hkpDashpotAction`, a class defined in C++
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
/// -  parent: hkpBinaryAction/`c00f3403`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpDashpotAction<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpDashpotAction"`: Name of this class.
    #[serde(default = "HkpDashpotAction::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x50746c6e`: Unique value of this class.
    #[serde(default = "HkpDashpotAction::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpDashpotActionHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpDashpotActionHkParam<'a>>
}

impl HkpDashpotAction<'_> {
    /// Return `"hkpDashpotAction"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpDashpotAction".into()
    }

    /// Return `"0x50746c6e"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x50746c6e".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpDashpotActionHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"point"`
    /// -   type: `hkVector4[2]`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "point")]
    Point([cgmath::Vector4<f32>; 2]),
    /// # Information on fields in the original C++ class
    /// -   name:`"strength"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "strength")]
    Strength(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"damping"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "damping")]
    Damping(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"impulse"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "impulse")]
    Impulse(cgmath::Vector4<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpDashpotActionHkParam<'de>, "@name",
    ("point" => Point([cgmath::Vector4<f32>; 2])),
    ("strength" => Strength(f64)),
    ("damping" => Damping(f64)),
    ("impulse" => Impulse(cgmath::Vector4<f32>)),
}
