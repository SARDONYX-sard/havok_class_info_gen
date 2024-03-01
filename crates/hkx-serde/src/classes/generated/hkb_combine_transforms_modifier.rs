//! A Rust structure that implements a serializer/deserializer corresponding to `hkbCombineTransformsModifier`, a class defined in C++
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
pub struct HkbCombineTransformsModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbCombineTransformsModifier"`: Name of this class.
    #[serde(default = "HkbCombineTransformsModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xfd1f0b79`: Unique value of this class.
    #[serde(default = "HkbCombineTransformsModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbCombineTransformsModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbCombineTransformsModifierHkParam<'a>>
}

impl HkbCombineTransformsModifier<'_> {
    /// Return `"hkbCombineTransformsModifier"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbCombineTransformsModifier".into()
    }

    /// Return `"0xfd1f0b79"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xfd1f0b79".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbCombineTransformsModifierHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"translationOut"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "translationOut")]
    TranslationOut(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"rotationOut"`
    /// -   type: `hkQuaternion`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotationOut")]
    RotationOut(cgmath::Quaternion<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"leftTranslation"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "leftTranslation")]
    LeftTranslation(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"leftRotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "leftRotation")]
    LeftRotation(cgmath::Quaternion<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"rightTranslation"`
    /// -   type: `hkVector4`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rightTranslation")]
    RightTranslation(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"rightRotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rightRotation")]
    RightRotation(cgmath::Quaternion<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"invertLeftTransform"`
    /// -   type: `hkBool`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "invertLeftTransform")]
    InvertLeftTransform(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"invertRightTransform"`
    /// -   type: `hkBool`
    /// - offset: 145
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "invertRightTransform")]
    InvertRightTransform(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"invertResult"`
    /// -   type: `hkBool`
    /// - offset: 146
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "invertResult")]
    InvertResult(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbCombineTransformsModifierHkParam<'de>, "@name",
    ("translationOut" => TranslationOut(cgmath::Vector4<f32>)),
    ("rotationOut" => RotationOut(cgmath::Quaternion<f32>)),
    ("leftTranslation" => LeftTranslation(cgmath::Vector4<f32>)),
    ("leftRotation" => LeftRotation(cgmath::Quaternion<f32>)),
    ("rightTranslation" => RightTranslation(cgmath::Vector4<f32>)),
    ("rightRotation" => RightRotation(cgmath::Quaternion<f32>)),
    ("invertLeftTransform" => InvertLeftTransform(bool)),
    ("invertRightTransform" => InvertRightTransform(bool)),
    ("invertResult" => InvertResult(bool)),
}
