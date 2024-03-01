//! A Rust structure that implements a serializer/deserializer corresponding to `hkbComputeRotationToTargetModifier`, a class defined in C++
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
/// -    size: 160
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbComputeRotationToTargetModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbComputeRotationToTargetModifier"`: Name of this class.
    #[serde(default = "HkbComputeRotationToTargetModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x47665f1c`: Unique value of this class.
    #[serde(default = "HkbComputeRotationToTargetModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbComputeRotationToTargetModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbComputeRotationToTargetModifierHkParam<'a>>
}

impl HkbComputeRotationToTargetModifier<'_> {
    /// Return `"hkbComputeRotationToTargetModifier"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbComputeRotationToTargetModifier".into()
    }

    /// Return `"0x47665f1c"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x47665f1c".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbComputeRotationToTargetModifierHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"rotationOut"`
    /// -   type: `hkQuaternion`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotationOut")]
    RotationOut(cgmath::Quaternion<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"targetPosition"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "targetPosition")]
    TargetPosition(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"currentPosition"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "currentPosition")]
    CurrentPosition(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"currentRotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "currentRotation")]
    CurrentRotation(cgmath::Quaternion<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"localAxisOfRotation"`
    /// -   type: `hkVector4`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "localAxisOfRotation")]
    LocalAxisOfRotation(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"localFacingDirection"`
    /// -   type: `hkVector4`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "localFacingDirection")]
    LocalFacingDirection(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"resultIsDelta"`
    /// -   type: `hkBool`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "resultIsDelta")]
    ResultIsDelta(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbComputeRotationToTargetModifierHkParam<'de>, "@name",
    ("rotationOut" => RotationOut(cgmath::Quaternion<f32>)),
    ("targetPosition" => TargetPosition(cgmath::Vector4<f32>)),
    ("currentPosition" => CurrentPosition(cgmath::Vector4<f32>)),
    ("currentRotation" => CurrentRotation(cgmath::Quaternion<f32>)),
    ("localAxisOfRotation" => LocalAxisOfRotation(cgmath::Vector4<f32>)),
    ("localFacingDirection" => LocalFacingDirection(cgmath::Vector4<f32>)),
    ("resultIsDelta" => ResultIsDelta(bool)),
}
