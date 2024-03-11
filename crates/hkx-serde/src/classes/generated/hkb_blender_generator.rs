//! A Rust structure that implements a serializer/deserializer corresponding to `hkbBlenderGenerator`, a class defined in C++
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
/// -    size: 116
/// -  vtable: true
/// -  parent: hkbGenerator/`d68aefc`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbBlenderGenerator<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbBlenderGenerator"`: The original C++ class name.
    #[serde(default = "HkbBlenderGenerator::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x22df7147`: Unique value of this class.
    #[serde(default = "HkbBlenderGenerator::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbBlenderGeneratorHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbBlenderGeneratorHkParam<'a>>
}

impl HkbBlenderGenerator<'_> {
    /// Return `"hkbBlenderGenerator"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbBlenderGenerator".into()
    }

    /// Return `"0x22df7147"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x22df7147".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbBlenderGeneratorHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"referencePoseWeightThreshold"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "referencePoseWeightThreshold")]
    ReferencePoseWeightThreshold(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"blendParameter"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blendParameter")]
    BlendParameter(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"minCyclicBlendParameter"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minCyclicBlendParameter")]
    MinCyclicBlendParameter(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxCyclicBlendParameter"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxCyclicBlendParameter")]
    MaxCyclicBlendParameter(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"indexOfSyncMasterChild"`
    /// -   type: `hkInt16`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indexOfSyncMasterChild")]
    IndexOfSyncMasterChild(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"flags"`
    /// -   type: `hkInt16`
    /// - offset: 58
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "flags")]
    Flags(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"subtractLastChild"`
    /// -   type: `hkBool`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "subtractLastChild")]
    SubtractLastChild(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"children"`
    /// -   type: `hkArray&lt;hkbBlenderGeneratorChild*&gt;`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "children")]
    Children(Vec<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"childrenInternalStates"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "childrenInternalStates", skip_serializing)]
    ChildrenInternalStates(Vec<()>),
    /// # Field information in the original C++ class
    /// -   name:`"sortedChildren"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "sortedChildren", skip_serializing)]
    SortedChildren(Vec<()>),
    /// # Field information in the original C++ class
    /// -   name:`"endIntervalWeight"`
    /// -   type: `hkReal`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "endIntervalWeight", skip_serializing)]
    EndIntervalWeight(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"numActiveChildren"`
    /// -   type: `hkInt32`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "numActiveChildren", skip_serializing)]
    NumActiveChildren(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"beginIntervalIndex"`
    /// -   type: `hkInt16`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "beginIntervalIndex", skip_serializing)]
    BeginIntervalIndex(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"endIntervalIndex"`
    /// -   type: `hkInt16`
    /// - offset: 110
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "endIntervalIndex", skip_serializing)]
    EndIntervalIndex(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"initSync"`
    /// -   type: `hkBool`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "initSync", skip_serializing)]
    InitSync(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"doSubtractiveBlend"`
    /// -   type: `hkBool`
    /// - offset: 113
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "doSubtractiveBlend", skip_serializing)]
    DoSubtractiveBlend(Primitive<bool>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbBlenderGeneratorHkParam<'de>, "@name",
    ("referencePoseWeightThreshold" => ReferencePoseWeightThreshold(Primitive<f32>)),
    ("blendParameter" => BlendParameter(Primitive<f32>)),
    ("minCyclicBlendParameter" => MinCyclicBlendParameter(Primitive<f32>)),
    ("maxCyclicBlendParameter" => MaxCyclicBlendParameter(Primitive<f32>)),
    ("indexOfSyncMasterChild" => IndexOfSyncMasterChild(Primitive<i16>)),
    ("flags" => Flags(Primitive<i16>)),
    ("subtractLastChild" => SubtractLastChild(Primitive<bool>)),
    ("children" => Children(Vec<Cow<'a, str>>)),
    ("childrenInternalStates" => ChildrenInternalStates(Vec<()>)),
    ("sortedChildren" => SortedChildren(Vec<()>)),
    ("endIntervalWeight" => EndIntervalWeight(Primitive<f32>)),
    ("numActiveChildren" => NumActiveChildren(Primitive<i32>)),
    ("beginIntervalIndex" => BeginIntervalIndex(Primitive<i16>)),
    ("endIntervalIndex" => EndIntervalIndex(Primitive<i16>)),
    ("initSync" => InitSync(Primitive<bool>)),
    ("doSubtractiveBlend" => DoSubtractiveBlend(Primitive<bool>)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum BlenderFlags {
    #[serde(rename = "FLAG_SYNC")]
    FlagSync = 1,
    #[serde(rename = "FLAG_SMOOTH_GENERATOR_WEIGHTS")]
    FlagSmoothGeneratorWeights = 4,
    #[serde(rename = "FLAG_DONT_DEACTIVATE_CHILDREN_WITH_ZERO_WEIGHTS")]
    FlagDontDeactivateChildrenWithZeroWeights = 8,
    #[serde(rename = "FLAG_PARAMETRIC_BLEND")]
    FlagParametricBlend = 16,
    #[serde(rename = "FLAG_IS_PARAMETRIC_BLEND_CYCLIC")]
    FlagIsParametricBlendCyclic = 32,
    #[serde(rename = "FLAG_FORCE_DENSE_POSE")]
    FlagForceDensePose = 64,
}
