//! A Rust structure that implements a serializer/deserializer corresponding to `BSLimbIKModifier`, a class defined in C++
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
/// -    size: 76
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct BsLimbIkModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"BSLimbIKModifier"`: The original C++ class name.
    #[serde(default = "BsLimbIkModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x8ea971e5`: Unique value of this class.
    #[serde(default = "BsLimbIkModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<BsLimbIkModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<BsLimbIkModifierHkParam<'a>>
}

impl BsLimbIkModifier<'_> {
    /// Return `"BSLimbIKModifier"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "BSLimbIKModifier".into()
    }

    /// Return `"0x8ea971e5"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x8ea971e5".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsLimbIkModifierHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"limitAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "limitAngleDegrees")]
    LimitAngleDegrees(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"currentAngle"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "currentAngle", skip_serializing)]
    CurrentAngle(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"startBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "startBoneIndex")]
    StartBoneIndex(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"endBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 54
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "endBoneIndex")]
    EndBoneIndex(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"gain"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "gain")]
    Gain(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"boneRadius"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "boneRadius")]
    BoneRadius(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"castOffset"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "castOffset")]
    CastOffset(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"timeStep"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "timeStep", skip_serializing)]
    TimeStep(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"pSkeletonMemory"`
    /// -   type: `void*`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "pSkeletonMemory", skip_serializing)]
    PSkeletonMemory(()),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    BsLimbIkModifierHkParam<'de>, "@name",
    ("limitAngleDegrees" => LimitAngleDegrees(Primitive<f32>)),
    ("currentAngle" => CurrentAngle(Primitive<f32>)),
    ("startBoneIndex" => StartBoneIndex(Primitive<i16>)),
    ("endBoneIndex" => EndBoneIndex(Primitive<i16>)),
    ("gain" => Gain(Primitive<f32>)),
    ("boneRadius" => BoneRadius(Primitive<f32>)),
    ("castOffset" => CastOffset(Primitive<f32>)),
    ("timeStep" => TimeStep(Primitive<f32>)),
    ("pSkeletonMemory" => PSkeletonMemory(())),
}
