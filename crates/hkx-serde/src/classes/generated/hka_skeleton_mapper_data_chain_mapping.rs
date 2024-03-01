//! A Rust structure that implements a serializer/deserializer corresponding to `hkaSkeletonMapperDataChainMapping`, a class defined in C++
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
/// -    size: 112
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkaSkeletonMapperDataChainMapping<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkaSkeletonMapperDataChainMapping"`: Name of this class.
    #[serde(default = "HkaSkeletonMapperDataChainMapping::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xa528f7cf`: Unique value of this class.
    #[serde(default = "HkaSkeletonMapperDataChainMapping::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkaSkeletonMapperDataChainMappingHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkaSkeletonMapperDataChainMappingHkParam<'a>>
}

impl HkaSkeletonMapperDataChainMapping<'_> {
    /// Return `"hkaSkeletonMapperDataChainMapping"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkaSkeletonMapperDataChainMapping".into()
    }

    /// Return `"0xa528f7cf"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xa528f7cf".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaSkeletonMapperDataChainMappingHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"startBoneA"`
    /// -   type: `hkInt16`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "startBoneA")]
    StartBoneA(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"endBoneA"`
    /// -   type: `hkInt16`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "endBoneA")]
    EndBoneA(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"startBoneB"`
    /// -   type: `hkInt16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "startBoneB")]
    StartBoneB(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"endBoneB"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "endBoneB")]
    EndBoneB(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"startAFromBTransform"`
    /// -   type: `hkQsTransform`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "startAFromBTransform")]
    StartAFromBTransform(cgmath::Matrix4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"endAFromBTransform"`
    /// -   type: `hkQsTransform`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "endAFromBTransform")]
    EndAFromBTransform(cgmath::Matrix4<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkaSkeletonMapperDataChainMappingHkParam<'de>, "@name",
    ("startBoneA" => StartBoneA(i16)),
    ("endBoneA" => EndBoneA(i16)),
    ("startBoneB" => StartBoneB(i16)),
    ("endBoneB" => EndBoneB(i16)),
    ("startAFromBTransform" => StartAFromBTransform(cgmath::Matrix4<f32>)),
    ("endAFromBTransform" => EndAFromBTransform(cgmath::Matrix4<f32>)),
}
