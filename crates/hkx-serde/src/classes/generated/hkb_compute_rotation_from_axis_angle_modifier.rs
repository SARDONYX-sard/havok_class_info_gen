//! A Rust structure that implements a serializer/deserializer corresponding to `hkbComputeRotationFromAxisAngleModifier`, a class defined in C++
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
/// -    size: 96
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbComputeRotationFromAxisAngleModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbComputeRotationFromAxisAngleModifier"`: The original C++ class name.
    #[serde(default = "HkbComputeRotationFromAxisAngleModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x9b3f6936`: Unique value of this class.
    #[serde(default = "HkbComputeRotationFromAxisAngleModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbComputeRotationFromAxisAngleModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbComputeRotationFromAxisAngleModifierHkParam<'a>>
}

impl HkbComputeRotationFromAxisAngleModifier<'_> {
    /// Return `"hkbComputeRotationFromAxisAngleModifier"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbComputeRotationFromAxisAngleModifier".into()
    }

    /// Return `"0x9b3f6936"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x9b3f6936".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbComputeRotationFromAxisAngleModifierHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"rotationOut"`
    /// -   type: `hkQuaternion`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotationOut")]
    RotationOut(Quaternion<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"axis"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "axis")]
    Axis(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"angleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "angleDegrees")]
    AngleDegrees(Primitive<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbComputeRotationFromAxisAngleModifierHkParam<'de>, "@name",
    ("rotationOut" => RotationOut(Quaternion<f32>)),
    ("axis" => Axis(Vector4<f32>)),
    ("angleDegrees" => AngleDegrees(Primitive<f32>)),
}
