//! A Rust structure that implements a serializer/deserializer corresponding to `BSComputeAddBoneAnimModifier`, a class defined in C++
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
/// -    size: 112
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct BsComputeAddBoneAnimModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"BSComputeAddBoneAnimModifier"`: The original C++ class name.
    #[serde(default = "BsComputeAddBoneAnimModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xa67f8c46`: Unique value of this class.
    #[serde(default = "BsComputeAddBoneAnimModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<BsComputeAddBoneAnimModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<BsComputeAddBoneAnimModifierHkParam<'a>>
}

impl BsComputeAddBoneAnimModifier<'_> {
    /// Return `"BSComputeAddBoneAnimModifier"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "BSComputeAddBoneAnimModifier".into()
    }

    /// Return `"0xa67f8c46"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xa67f8c46".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsComputeAddBoneAnimModifierHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"boneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "boneIndex")]
    BoneIndex(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"translationLSOut"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "translationLSOut")]
    TranslationLsOut(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"rotationLSOut"`
    /// -   type: `hkQuaternion`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotationLSOut")]
    RotationLsOut(Quaternion<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"scaleLSOut"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "scaleLSOut")]
    ScaleLsOut(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"pSkeletonMemory"`
    /// -   type: `void*`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "pSkeletonMemory", skip_serializing)]
    PSkeletonMemory(()),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    BsComputeAddBoneAnimModifierHkParam<'de>, "@name",
    ("boneIndex" => BoneIndex(Primitive<i16>)),
    ("translationLSOut" => TranslationLsOut(Vector4<f32>)),
    ("rotationLSOut" => RotationLsOut(Quaternion<f32>)),
    ("scaleLSOut" => ScaleLsOut(Vector4<f32>)),
    ("pSkeletonMemory" => PSkeletonMemory(())),
}
