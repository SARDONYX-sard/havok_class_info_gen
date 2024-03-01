//! A Rust structure that implements a serializer/deserializer corresponding to `hkbRotateCharacterModifier`, a class defined in C++
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
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbRotateCharacterModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbRotateCharacterModifier"`: Name of this class.
    #[serde(default = "HkbRotateCharacterModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x877ebc0b`: Unique value of this class.
    #[serde(default = "HkbRotateCharacterModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbRotateCharacterModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbRotateCharacterModifierHkParam<'a>>
}

impl HkbRotateCharacterModifier<'_> {
    /// Return `"hkbRotateCharacterModifier"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbRotateCharacterModifier".into()
    }

    /// Return `"0x877ebc0b"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x877ebc0b".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbRotateCharacterModifierHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"degreesPerSecond"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "degreesPerSecond")]
    DegreesPerSecond(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"speedMultiplier"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "speedMultiplier")]
    SpeedMultiplier(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"axisOfRotation"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "axisOfRotation")]
    AxisOfRotation(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"angle"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "angle", skip_serializing)]
    Angle(f64),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbRotateCharacterModifierHkParam<'de>, "@name",
    ("degreesPerSecond" => DegreesPerSecond(f64)),
    ("speedMultiplier" => SpeedMultiplier(f64)),
    ("axisOfRotation" => AxisOfRotation(cgmath::Vector4<f32>)),
    ("angle" => Angle(f64)),
}
