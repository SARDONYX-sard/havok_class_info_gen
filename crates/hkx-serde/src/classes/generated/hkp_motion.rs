//! A Rust structure that implements a serializer/deserializer corresponding to `hkpMotion`, a class defined in C++
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
/// -    size: 288
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 3
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpMotion<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpMotion"`: The original C++ class name.
    #[serde(default = "HkpMotion::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x98aadb4f`: Unique value of this class.
    #[serde(default = "HkpMotion::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpMotionHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpMotionHkParam<'a>>
}

impl HkpMotion<'_> {
    /// Return `"hkpMotion"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpMotion".into()
    }

    /// Return `"0x98aadb4f"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x98aadb4f".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpMotionHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"type"`
    /// -   type: `enum MotionType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(MotionType),
    /// # Field information in the original C++ class
    /// -   name:`"deactivationIntegrateCounter"`
    /// -   type: `hkUint8`
    /// - offset: 9
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deactivationIntegrateCounter")]
    DeactivationIntegrateCounter(Primitive<u8>),
    /// # Field information in the original C++ class
    /// -   name:`"deactivationNumInactiveFrames"`
    /// -   type: `hkUint16[2]`
    /// - offset: 10
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deactivationNumInactiveFrames")]
    DeactivationNumInactiveFrames([Primitive<u16>; 2]),
    /// # Field information in the original C++ class
    /// -   name:`"motionState"`
    /// -   type: `struct hkMotionState`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "motionState")]
    MotionState(HkMotionState),
    /// # Field information in the original C++ class
    /// -   name:`"inertiaAndMassInv"`
    /// -   type: `hkVector4`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "inertiaAndMassInv")]
    InertiaAndMassInv(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"linearVelocity"`
    /// -   type: `hkVector4`
    /// - offset: 208
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "linearVelocity")]
    LinearVelocity(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"angularVelocity"`
    /// -   type: `hkVector4`
    /// - offset: 224
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "angularVelocity")]
    AngularVelocity(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"deactivationRefPosition"`
    /// -   type: `hkVector4[2]`
    /// - offset: 240
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deactivationRefPosition")]
    DeactivationRefPosition(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"deactivationRefOrientation"`
    /// -   type: `hkUint32[2]`
    /// - offset: 272
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deactivationRefOrientation")]
    DeactivationRefOrientation([Primitive<u32>; 2]),
    /// # Field information in the original C++ class
    /// -   name:`"savedMotion"`
    /// -   type: `struct hkpMaxSizeMotion*`
    /// - offset: 280
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "savedMotion")]
    SavedMotion(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"savedQualityTypeIndex"`
    /// -   type: `hkUint16`
    /// - offset: 284
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "savedQualityTypeIndex")]
    SavedQualityTypeIndex(Primitive<u16>),
    /// # Field information in the original C++ class
    /// -   name:`"gravityFactor"`
    /// -   type: `hkHalf`
    /// - offset: 286
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "gravityFactor")]
    GravityFactor(Primitive<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpMotionHkParam<'de>, "@name",
    ("type" => Type(MotionType)),
    ("deactivationIntegrateCounter" => DeactivationIntegrateCounter(Primitive<u8>)),
    ("deactivationNumInactiveFrames" => DeactivationNumInactiveFrames([Primitive<u16>; 2])),
    ("motionState" => MotionState(HkMotionState)),
    ("inertiaAndMassInv" => InertiaAndMassInv(Vector4<f32>)),
    ("linearVelocity" => LinearVelocity(Vector4<f32>)),
    ("angularVelocity" => AngularVelocity(Vector4<f32>)),
    ("deactivationRefPosition" => DeactivationRefPosition(Vector4<f32>)),
    ("deactivationRefOrientation" => DeactivationRefOrientation([Primitive<u32>; 2])),
    ("savedMotion" => SavedMotion(Cow<'a, str>)),
    ("savedQualityTypeIndex" => SavedQualityTypeIndex(Primitive<u16>)),
    ("gravityFactor" => GravityFactor(Primitive<f32>)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MotionType {
    #[serde(rename = "MOTION_INVALID")]
    MotionInvalid = 0,
    #[serde(rename = "MOTION_DYNAMIC")]
    MotionDynamic = 1,
    #[serde(rename = "MOTION_SPHERE_INERTIA")]
    MotionSphereInertia = 2,
    #[serde(rename = "MOTION_BOX_INERTIA")]
    MotionBoxInertia = 3,
    #[serde(rename = "MOTION_KEYFRAMED")]
    MotionKeyframed = 4,
    #[serde(rename = "MOTION_FIXED")]
    MotionFixed = 5,
    #[serde(rename = "MOTION_THIN_BOX_INERTIA")]
    MotionThinBoxInertia = 6,
    #[serde(rename = "MOTION_CHARACTER")]
    MotionCharacter = 7,
    #[serde(rename = "MOTION_MAX_ID")]
    MotionMaxId = 8,
}
