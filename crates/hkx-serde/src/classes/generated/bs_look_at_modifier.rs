//! A Rust structure that implements a serializer/deserializer corresponding to `BSLookAtModifier`, a class defined in C++
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
/// - version: 4
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct BsLookAtModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"BSLookAtModifier"`: Name of this class.
    #[serde(default = "BsLookAtModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xd756fc25`: Unique value of this class.
    #[serde(default = "BsLookAtModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<BsLookAtModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<BsLookAtModifierHkParam<'a>>
}

impl BsLookAtModifier<'_> {
    /// Return `"BSLookAtModifier"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "BsLookAtModifier".into()
    }

    /// Return `"0xd756fc25"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xd756fc25".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsLookAtModifierHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"lookAtTarget"`
    /// -   type: `hkBool`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lookAtTarget")]
    LookAtTarget(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"bones"`
    /// -   type: `hkArray&lt;struct BSLookAtModifierBoneData&gt;`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bones")]
    Bones(Vec<BsLookAtModifierBoneData>),
    /// # Information on fields in the original C++ class
    /// -   name:`"eyeBones"`
    /// -   type: `hkArray&lt;struct BSLookAtModifierBoneData&gt;`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eyeBones")]
    EyeBones(Vec<BsLookAtModifierBoneData>),
    /// # Information on fields in the original C++ class
    /// -   name:`"limitAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "limitAngleDegrees")]
    LimitAngleDegrees(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"limitAngleThresholdDegrees"`
    /// -   type: `hkReal`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "limitAngleThresholdDegrees")]
    LimitAngleThresholdDegrees(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"continueLookOutsideOfLimit"`
    /// -   type: `hkBool`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "continueLookOutsideOfLimit")]
    ContinueLookOutsideOfLimit(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"onGain"`
    /// -   type: `hkReal`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "onGain")]
    OnGain(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"offGain"`
    /// -   type: `hkReal`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "offGain")]
    OffGain(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"useBoneGains"`
    /// -   type: `hkBool`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "useBoneGains")]
    UseBoneGains(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"targetLocation"`
    /// -   type: `hkVector4`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "targetLocation")]
    TargetLocation(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"targetOutsideLimits"`
    /// -   type: `hkBool`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "targetOutsideLimits")]
    TargetOutsideLimits(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"targetOutOfLimitEvent"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "targetOutOfLimitEvent")]
    TargetOutOfLimitEvent(HkbEventProperty),
    /// # Information on fields in the original C++ class
    /// -   name:`"lookAtCamera"`
    /// -   type: `hkBool`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lookAtCamera")]
    LookAtCamera(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"lookAtCameraX"`
    /// -   type: `hkReal`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lookAtCameraX")]
    LookAtCameraX(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"lookAtCameraY"`
    /// -   type: `hkReal`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lookAtCameraY")]
    LookAtCameraY(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"lookAtCameraZ"`
    /// -   type: `hkReal`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lookAtCameraZ")]
    LookAtCameraZ(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"timeStep"`
    /// -   type: `hkReal`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "timeStep", skip_serializing)]
    TimeStep(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"ballBonesValid"`
    /// -   type: `hkBool`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "ballBonesValid", skip_serializing)]
    BallBonesValid(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"pSkeletonMemory"`
    /// -   type: `void*`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "pSkeletonMemory", skip_serializing)]
    PSkeletonMemory(()),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    BsLookAtModifierHkParam<'de>, "@name",
    ("lookAtTarget" => LookAtTarget(bool)),
    ("bones" => Bones(Vec<BsLookAtModifierBoneData>)),
    ("eyeBones" => EyeBones(Vec<BsLookAtModifierBoneData>)),
    ("limitAngleDegrees" => LimitAngleDegrees(f64)),
    ("limitAngleThresholdDegrees" => LimitAngleThresholdDegrees(f64)),
    ("continueLookOutsideOfLimit" => ContinueLookOutsideOfLimit(bool)),
    ("onGain" => OnGain(f64)),
    ("offGain" => OffGain(f64)),
    ("useBoneGains" => UseBoneGains(bool)),
    ("targetLocation" => TargetLocation(cgmath::Vector4<f32>)),
    ("targetOutsideLimits" => TargetOutsideLimits(bool)),
    ("targetOutOfLimitEvent" => TargetOutOfLimitEvent(HkbEventProperty)),
    ("lookAtCamera" => LookAtCamera(bool)),
    ("lookAtCameraX" => LookAtCameraX(f64)),
    ("lookAtCameraY" => LookAtCameraY(f64)),
    ("lookAtCameraZ" => LookAtCameraZ(f64)),
    ("timeStep" => TimeStep(f64)),
    ("ballBonesValid" => BallBonesValid(bool)),
    ("pSkeletonMemory" => PSkeletonMemory(())),
}