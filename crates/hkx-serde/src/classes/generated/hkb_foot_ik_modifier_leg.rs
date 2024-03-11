//! A Rust structure that implements a serializer/deserializer corresponding to `hkbFootIkModifierLeg`, a class defined in C++
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
/// -    size: 160
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 2
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbFootIkModifierLeg<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbFootIkModifierLeg"`: The original C++ class name.
    #[serde(default = "HkbFootIkModifierLeg::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x9f3e3a04`: Unique value of this class.
    #[serde(default = "HkbFootIkModifierLeg::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbFootIkModifierLegHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbFootIkModifierLegHkParam<'a>>
}

impl HkbFootIkModifierLeg<'_> {
    /// Return `"hkbFootIkModifierLeg"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbFootIkModifierLeg".into()
    }

    /// Return `"0x9f3e3a04"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x9f3e3a04".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbFootIkModifierLegHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"originalAnkleTransformMS"`
    /// -   type: `hkQsTransform`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "originalAnkleTransformMS")]
    OriginalAnkleTransformMs(QsTransform<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"prevAnkleRotLS"`
    /// -   type: `hkQuaternion`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "prevAnkleRotLS", skip_serializing)]
    PrevAnkleRotLs(Quaternion<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"kneeAxisLS"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "kneeAxisLS")]
    KneeAxisLs(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"footEndLS"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "footEndLS")]
    FootEndLs(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"ungroundedEvent"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ungroundedEvent")]
    UngroundedEvent(HkbEventProperty),
    /// # Field information in the original C++ class
    /// -   name:`"footPlantedAnkleHeightMS"`
    /// -   type: `hkReal`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "footPlantedAnkleHeightMS")]
    FootPlantedAnkleHeightMs(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"footRaisedAnkleHeightMS"`
    /// -   type: `hkReal`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "footRaisedAnkleHeightMS")]
    FootRaisedAnkleHeightMs(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxAnkleHeightMS"`
    /// -   type: `hkReal`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxAnkleHeightMS")]
    MaxAnkleHeightMs(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"minAnkleHeightMS"`
    /// -   type: `hkReal`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minAnkleHeightMS")]
    MinAnkleHeightMs(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxKneeAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxKneeAngleDegrees")]
    MaxKneeAngleDegrees(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"minKneeAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minKneeAngleDegrees")]
    MinKneeAngleDegrees(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"verticalError"`
    /// -   type: `hkReal`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "verticalError")]
    VerticalError(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxAnkleAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxAnkleAngleDegrees")]
    MaxAnkleAngleDegrees(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"hipIndex"`
    /// -   type: `hkInt16`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hipIndex")]
    HipIndex(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"kneeIndex"`
    /// -   type: `hkInt16`
    /// - offset: 138
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "kneeIndex")]
    KneeIndex(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"ankleIndex"`
    /// -   type: `hkInt16`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ankleIndex")]
    AnkleIndex(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"hitSomething"`
    /// -   type: `hkBool`
    /// - offset: 142
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hitSomething")]
    HitSomething(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"isPlantedMS"`
    /// -   type: `hkBool`
    /// - offset: 143
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isPlantedMS")]
    IsPlantedMs(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"isOriginalAnkleTransformMSSet"`
    /// -   type: `hkBool`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isOriginalAnkleTransformMSSet")]
    IsOriginalAnkleTransformMsSet(Primitive<bool>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbFootIkModifierLegHkParam<'de>, "@name",
    ("originalAnkleTransformMS" => OriginalAnkleTransformMs(QsTransform<f32>)),
    ("prevAnkleRotLS" => PrevAnkleRotLs(Quaternion<f32>)),
    ("kneeAxisLS" => KneeAxisLs(Vector4<f32>)),
    ("footEndLS" => FootEndLs(Vector4<f32>)),
    ("ungroundedEvent" => UngroundedEvent(HkbEventProperty)),
    ("footPlantedAnkleHeightMS" => FootPlantedAnkleHeightMs(Primitive<f32>)),
    ("footRaisedAnkleHeightMS" => FootRaisedAnkleHeightMs(Primitive<f32>)),
    ("maxAnkleHeightMS" => MaxAnkleHeightMs(Primitive<f32>)),
    ("minAnkleHeightMS" => MinAnkleHeightMs(Primitive<f32>)),
    ("maxKneeAngleDegrees" => MaxKneeAngleDegrees(Primitive<f32>)),
    ("minKneeAngleDegrees" => MinKneeAngleDegrees(Primitive<f32>)),
    ("verticalError" => VerticalError(Primitive<f32>)),
    ("maxAnkleAngleDegrees" => MaxAnkleAngleDegrees(Primitive<f32>)),
    ("hipIndex" => HipIndex(Primitive<i16>)),
    ("kneeIndex" => KneeIndex(Primitive<i16>)),
    ("ankleIndex" => AnkleIndex(Primitive<i16>)),
    ("hitSomething" => HitSomething(Primitive<bool>)),
    ("isPlantedMS" => IsPlantedMs(Primitive<bool>)),
    ("isOriginalAnkleTransformMSSet" => IsOriginalAnkleTransformMsSet(Primitive<bool>)),
}
