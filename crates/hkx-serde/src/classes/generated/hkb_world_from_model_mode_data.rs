//! A Rust structure that implements a serializer/deserializer corresponding to `hkbWorldFromModelModeData`, a class defined in C++
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
/// -    size: 8
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbWorldFromModelModeData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbWorldFromModelModeData"`: Name of this class.
    #[serde(default = "HkbWorldFromModelModeData::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xa3af8783`: Unique value of this class.
    #[serde(default = "HkbWorldFromModelModeData::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbWorldFromModelModeDataHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbWorldFromModelModeDataHkParam<'a>>
}

impl HkbWorldFromModelModeData<'_> {
    /// Return `"hkbWorldFromModelModeData"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbWorldFromModelModeData".into()
    }

    /// Return `"0xa3af8783"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xa3af8783".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbWorldFromModelModeDataHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"poseMatchingBone0"`
    /// -   type: `hkInt16`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "poseMatchingBone0")]
    PoseMatchingBone0(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"poseMatchingBone1"`
    /// -   type: `hkInt16`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "poseMatchingBone1")]
    PoseMatchingBone1(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"poseMatchingBone2"`
    /// -   type: `hkInt16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "poseMatchingBone2")]
    PoseMatchingBone2(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"mode"`
    /// -   type: `enum WorldFromModelMode`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mode")]
    Mode(WorldFromModelMode),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbWorldFromModelModeDataHkParam<'de>, "@name",
    ("poseMatchingBone0" => PoseMatchingBone0(i16)),
    ("poseMatchingBone1" => PoseMatchingBone1(i16)),
    ("poseMatchingBone2" => PoseMatchingBone2(i16)),
    ("mode" => Mode(WorldFromModelMode)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WorldFromModelMode {
    #[serde(rename = "WORLD_FROM_MODEL_MODE_USE_OLD")]
    WorldFromModelModeUseOld = 0,
    #[serde(rename = "WORLD_FROM_MODEL_MODE_USE_INPUT")]
    WorldFromModelModeUseInput = 1,
    #[serde(rename = "WORLD_FROM_MODEL_MODE_COMPUTE")]
    WorldFromModelModeCompute = 2,
    #[serde(rename = "WORLD_FROM_MODEL_MODE_NONE")]
    WorldFromModelModeNone = 3,
    #[serde(rename = "WORLD_FROM_MODEL_MODE_RAGDOLL")]
    WorldFromModelModeRagdoll = 4,
}
