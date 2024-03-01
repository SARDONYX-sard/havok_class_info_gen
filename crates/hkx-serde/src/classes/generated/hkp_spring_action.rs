//! A Rust structure that implements a serializer/deserializer corresponding to `hkpSpringAction`, a class defined in C++
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
pub struct HkpSpringAction<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpSpringAction"`: Name of this class.
    #[serde(default = "HkpSpringAction::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x88fc09fa`: Unique value of this class.
    #[serde(default = "HkpSpringAction::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpSpringActionHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpSpringActionHkParam<'a>>
}

impl HkpSpringAction<'_> {
    /// Return `"hkpSpringAction"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpSpringAction".into()
    }

    /// Return `"0x88fc09fa"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x88fc09fa".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpSpringActionHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"lastForce"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lastForce")]
    LastForce(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"positionAinA"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "positionAinA")]
    PositionAinA(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"positionBinB"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "positionBinB")]
    PositionBinB(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"restLength"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "restLength")]
    RestLength(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"strength"`
    /// -   type: `hkReal`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "strength")]
    Strength(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"damping"`
    /// -   type: `hkReal`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "damping")]
    Damping(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"onCompression"`
    /// -   type: `hkBool`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "onCompression")]
    OnCompression(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"onExtension"`
    /// -   type: `hkBool`
    /// - offset: 93
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "onExtension")]
    OnExtension(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpSpringActionHkParam<'de>, "@name",
    ("lastForce" => LastForce(cgmath::Vector4<f32>)),
    ("positionAinA" => PositionAinA(cgmath::Vector4<f32>)),
    ("positionBinB" => PositionBinB(cgmath::Vector4<f32>)),
    ("restLength" => RestLength(f64)),
    ("strength" => Strength(f64)),
    ("damping" => Damping(f64)),
    ("onCompression" => OnCompression(bool)),
    ("onExtension" => OnExtension(bool)),
}
