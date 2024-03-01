//! A Rust structure that implements a serializer/deserializer corresponding to `BSDirectAtModifier`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
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
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct BsDirectAtModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"BSDirectAtModifier"`: Name of this class.
    #[serde(default = "BsDirectAtModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x19a005c0`: Unique value of this class.
    #[serde(default = "BsDirectAtModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<BsDirectAtModifierHkParam>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<BsDirectAtModifierHkParam>,
}

impl BsDirectAtModifier<'_> {
    /// Return `"BSDirectAtModifier"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "BsDirectAtModifier".into()
    }

    /// Return `"0x19a005c0"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x19a005c0".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsDirectAtModifierHkParam {
    /// # Information on fields in the original C++ class
    /// -   name:`"directAtTarget"`
    /// -   type: `hkBool`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "directAtTarget")]
    DirectAtTarget(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"sourceBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 46
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sourceBoneIndex")]
    SourceBoneIndex(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"startBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "startBoneIndex")]
    StartBoneIndex(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"endBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 50
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "endBoneIndex")]
    EndBoneIndex(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"limitHeadingDegrees"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "limitHeadingDegrees")]
    LimitHeadingDegrees(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"limitPitchDegrees"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "limitPitchDegrees")]
    LimitPitchDegrees(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"offsetHeadingDegrees"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "offsetHeadingDegrees")]
    OffsetHeadingDegrees(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"offsetPitchDegrees"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "offsetPitchDegrees")]
    OffsetPitchDegrees(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"onGain"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "onGain")]
    OnGain(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"offGain"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "offGain")]
    OffGain(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"targetLocation"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "targetLocation")]
    TargetLocation(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"userInfo"`
    /// -   type: `hkUint32`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userInfo")]
    UserInfo(u32),
    /// # Information on fields in the original C++ class
    /// -   name:`"directAtCamera"`
    /// -   type: `hkBool`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "directAtCamera")]
    DirectAtCamera(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"directAtCameraX"`
    /// -   type: `hkReal`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "directAtCameraX")]
    DirectAtCameraX(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"directAtCameraY"`
    /// -   type: `hkReal`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "directAtCameraY")]
    DirectAtCameraY(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"directAtCameraZ"`
    /// -   type: `hkReal`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "directAtCameraZ")]
    DirectAtCameraZ(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"active"`
    /// -   type: `hkBool`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "active")]
    Active(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"currentHeadingOffset"`
    /// -   type: `hkReal`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "currentHeadingOffset")]
    CurrentHeadingOffset(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"currentPitchOffset"`
    /// -   type: `hkReal`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "currentPitchOffset")]
    CurrentPitchOffset(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"timeStep"`
    /// -   type: `hkReal`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "timeStep", skip_serializing)]
    TimeStep(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"pSkeletonMemory"`
    /// -   type: `void*`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "pSkeletonMemory", skip_serializing)]
    PSkeletonMemory(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"hasTarget"`
    /// -   type: `hkBool`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "hasTarget", skip_serializing)]
    HasTarget(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"directAtTargetLocation"`
    /// -   type: `hkVector4`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "directAtTargetLocation", skip_serializing)]
    DirectAtTargetLocation(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"boneChainIndices"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "boneChainIndices", skip_serializing)]
    BoneChainIndices(Vec<()>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    BsDirectAtModifierHkParam, "@name",
    ("directAtTarget" => DirectAtTarget(bool)),
    ("sourceBoneIndex" => SourceBoneIndex(i16)),
    ("startBoneIndex" => StartBoneIndex(i16)),
    ("endBoneIndex" => EndBoneIndex(i16)),
    ("limitHeadingDegrees" => LimitHeadingDegrees(f64)),
    ("limitPitchDegrees" => LimitPitchDegrees(f64)),
    ("offsetHeadingDegrees" => OffsetHeadingDegrees(f64)),
    ("offsetPitchDegrees" => OffsetPitchDegrees(f64)),
    ("onGain" => OnGain(f64)),
    ("offGain" => OffGain(f64)),
    ("targetLocation" => TargetLocation(cgmath::Vector4<f32>)),
    ("userInfo" => UserInfo(u32)),
    ("directAtCamera" => DirectAtCamera(bool)),
    ("directAtCameraX" => DirectAtCameraX(f64)),
    ("directAtCameraY" => DirectAtCameraY(f64)),
    ("directAtCameraZ" => DirectAtCameraZ(f64)),
    ("active" => Active(bool)),
    ("currentHeadingOffset" => CurrentHeadingOffset(f64)),
    ("currentPitchOffset" => CurrentPitchOffset(f64)),
    ("timeStep" => TimeStep(f64)),
    ("pSkeletonMemory" => PSkeletonMemory(())),
    ("hasTarget" => HasTarget(bool)),
    ("directAtTargetLocation" => DirectAtTargetLocation(cgmath::Vector4<f32>)),
    ("boneChainIndices" => BoneChainIndices(Vec<()>)),
}
