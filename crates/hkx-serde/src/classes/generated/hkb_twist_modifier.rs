//! A Rust structure that implements a serializer/deserializer corresponding to `hkbTwistModifier`, a class defined in C++
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
pub struct HkbTwistModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbTwistModifier"`: The original C++ class name.
    #[serde(default = "HkbTwistModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xb6b76b32`: Unique value of this class.
    #[serde(default = "HkbTwistModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbTwistModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbTwistModifierHkParam<'a>>
}

impl HkbTwistModifier<'_> {
    /// Return `"hkbTwistModifier"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbTwistModifier".into()
    }

    /// Return `"0xb6b76b32"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xb6b76b32".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbTwistModifierHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"axisOfRotation"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "axisOfRotation")]
    AxisOfRotation(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"twistAngle"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "twistAngle")]
    TwistAngle(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"startBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "startBoneIndex")]
    StartBoneIndex(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"endBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 70
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "endBoneIndex")]
    EndBoneIndex(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"setAngleMethod"`
    /// -   type: `enum SetAngleMethod`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "setAngleMethod")]
    SetAngleMethod(SetAngleMethod),
    /// # Field information in the original C++ class
    /// -   name:`"rotationAxisCoordinates"`
    /// -   type: `enum RotationAxisCoordinates`
    /// - offset: 73
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotationAxisCoordinates")]
    RotationAxisCoordinates(RotationAxisCoordinates),
    /// # Field information in the original C++ class
    /// -   name:`"isAdditive"`
    /// -   type: `hkBool`
    /// - offset: 74
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isAdditive")]
    IsAdditive(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"boneChainIndices"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "boneChainIndices", skip_serializing)]
    BoneChainIndices(Vec<()>),
    /// # Field information in the original C++ class
    /// -   name:`"parentBoneIndices"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "parentBoneIndices", skip_serializing)]
    ParentBoneIndices(Vec<()>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbTwistModifierHkParam<'de>, "@name",
    ("axisOfRotation" => AxisOfRotation(Vector4<f32>)),
    ("twistAngle" => TwistAngle(Primitive<f32>)),
    ("startBoneIndex" => StartBoneIndex(Primitive<i16>)),
    ("endBoneIndex" => EndBoneIndex(Primitive<i16>)),
    ("setAngleMethod" => SetAngleMethod(SetAngleMethod)),
    ("rotationAxisCoordinates" => RotationAxisCoordinates(RotationAxisCoordinates)),
    ("isAdditive" => IsAdditive(Primitive<bool>)),
    ("boneChainIndices" => BoneChainIndices(Vec<()>)),
    ("parentBoneIndices" => ParentBoneIndices(Vec<()>)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SetAngleMethod {
    #[serde(rename = "LINEAR")]
    Linear = 0,
    #[serde(rename = "RAMPED")]
    Ramped = 1,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RotationAxisCoordinates {
    #[serde(rename = "ROTATION_AXIS_IN_MODEL_COORDINATES")]
    RotationAxisInModelCoordinates = 0,
    #[serde(rename = "ROTATION_AXIS_IN_LOCAL_COORDINATES")]
    RotationAxisInLocalCoordinates = 1,
}
