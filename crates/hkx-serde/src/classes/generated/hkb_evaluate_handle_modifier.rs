//! A Rust structure that implements a serializer/deserializer corresponding to `hkbEvaluateHandleModifier`, a class defined in C++
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
/// -    size: 176
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 2
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbEvaluateHandleModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbEvaluateHandleModifier"`: Name of this class.
    #[serde(default = "HkbEvaluateHandleModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x79757102`: Unique value of this class.
    #[serde(default = "HkbEvaluateHandleModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbEvaluateHandleModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbEvaluateHandleModifierHkParam<'a>>
}

impl HkbEvaluateHandleModifier<'_> {
    /// Return `"hkbEvaluateHandleModifier"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbEvaluateHandleModifier".into()
    }

    /// Return `"0x79757102"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x79757102".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbEvaluateHandleModifierHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"handle"`
    /// -   type: `struct hkbHandle*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handle")]
    Handle(Box<HkbHandle>),
    /// # Information on fields in the original C++ class
    /// -   name:`"handlePositionOut"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handlePositionOut")]
    HandlePositionOut(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"handleRotationOut"`
    /// -   type: `hkQuaternion`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handleRotationOut")]
    HandleRotationOut(cgmath::Quaternion<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"isValidOut"`
    /// -   type: `hkBool`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isValidOut")]
    IsValidOut(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"extrapolationTimeStep"`
    /// -   type: `hkReal`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extrapolationTimeStep")]
    ExtrapolationTimeStep(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"handleChangeSpeed"`
    /// -   type: `hkReal`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handleChangeSpeed")]
    HandleChangeSpeed(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"handleChangeMode"`
    /// -   type: `enum HandleChangeMode`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handleChangeMode")]
    HandleChangeMode(HandleChangeMode),
    /// # Information on fields in the original C++ class
    /// -   name:`"oldHandle"`
    /// -   type: `struct hkbHandle`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "oldHandle", skip_serializing)]
    OldHandle(HkbHandle),
    /// # Information on fields in the original C++ class
    /// -   name:`"oldHandlePosition"`
    /// -   type: `hkVector4`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "oldHandlePosition", skip_serializing)]
    OldHandlePosition(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"oldHandleRotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "oldHandleRotation", skip_serializing)]
    OldHandleRotation(cgmath::Quaternion<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"timeSinceLastModify"`
    /// -   type: `hkReal`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "timeSinceLastModify", skip_serializing)]
    TimeSinceLastModify(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"smoothlyChangingHandles"`
    /// -   type: `hkBool`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "smoothlyChangingHandles", skip_serializing)]
    SmoothlyChangingHandles(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbEvaluateHandleModifierHkParam<'de>, "@name",
    ("handle" => Handle(Box<HkbHandle>)),
    ("handlePositionOut" => HandlePositionOut(cgmath::Vector4<f32>)),
    ("handleRotationOut" => HandleRotationOut(cgmath::Quaternion<f32>)),
    ("isValidOut" => IsValidOut(bool)),
    ("extrapolationTimeStep" => ExtrapolationTimeStep(f64)),
    ("handleChangeSpeed" => HandleChangeSpeed(f64)),
    ("handleChangeMode" => HandleChangeMode(HandleChangeMode)),
    ("oldHandle" => OldHandle(HkbHandle)),
    ("oldHandlePosition" => OldHandlePosition(cgmath::Vector4<f32>)),
    ("oldHandleRotation" => OldHandleRotation(cgmath::Quaternion<f32>)),
    ("timeSinceLastModify" => TimeSinceLastModify(f64)),
    ("smoothlyChangingHandles" => SmoothlyChangingHandles(bool)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum HandleChangeMode {
    #[serde(rename = "HANDLE_CHANGE_MODE_ABRUPT")]
    HandleChangeModeAbrupt = 0,
    #[serde(rename = "HANDLE_CHANGE_MODE_CONSTANT_VELOCITY")]
    HandleChangeModeConstantVelocity = 1,
}
