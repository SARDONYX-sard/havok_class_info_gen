//! A Rust structure that implements a serializer/deserializer corresponding to `hkaSkeletonMapperData`, a class defined in C++
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
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkaSkeletonMapperData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkaSkeletonMapperData"`: The original C++ class name.
    #[serde(default = "HkaSkeletonMapperData::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x95687ea0`: Unique value of this class.
    #[serde(default = "HkaSkeletonMapperData::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkaSkeletonMapperDataHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkaSkeletonMapperDataHkParam<'a>>
}

impl HkaSkeletonMapperData<'_> {
    /// Return `"hkaSkeletonMapperData"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkaSkeletonMapperData".into()
    }

    /// Return `"0x95687ea0"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x95687ea0".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaSkeletonMapperDataHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"skeletonA"`
    /// -   type: `struct hkaSkeleton*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "skeletonA")]
    SkeletonA(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"skeletonB"`
    /// -   type: `struct hkaSkeleton*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "skeletonB")]
    SkeletonB(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"simpleMappings"`
    /// -   type: `hkArray&lt;struct hkaSkeletonMapperDataSimpleMapping&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "simpleMappings")]
    SimpleMappings(Vec<HkaSkeletonMapperDataSimpleMapping>),
    /// # Field information in the original C++ class
    /// -   name:`"chainMappings"`
    /// -   type: `hkArray&lt;struct hkaSkeletonMapperDataChainMapping&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "chainMappings")]
    ChainMappings(Vec<HkaSkeletonMapperDataChainMapping>),
    /// # Field information in the original C++ class
    /// -   name:`"unmappedBones"`
    /// -   type: `hkArray&lt;hkInt16&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "unmappedBones")]
    UnmappedBones(Vec<Primitive<i16>>),
    /// # Field information in the original C++ class
    /// -   name:`"extractedMotionMapping"`
    /// -   type: `hkQsTransform`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extractedMotionMapping")]
    ExtractedMotionMapping(QsTransform<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"keepUnmappedLocal"`
    /// -   type: `hkBool`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "keepUnmappedLocal")]
    KeepUnmappedLocal(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"mappingType"`
    /// -   type: `enum MappingType`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mappingType")]
    MappingType(MappingType),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkaSkeletonMapperDataHkParam<'de>, "@name",
    ("skeletonA" => SkeletonA(Cow<'a, str>)),
    ("skeletonB" => SkeletonB(Cow<'a, str>)),
    ("simpleMappings" => SimpleMappings(Vec<HkaSkeletonMapperDataSimpleMapping>)),
    ("chainMappings" => ChainMappings(Vec<HkaSkeletonMapperDataChainMapping>)),
    ("unmappedBones" => UnmappedBones(Vec<Primitive<i16>>)),
    ("extractedMotionMapping" => ExtractedMotionMapping(QsTransform<f32>)),
    ("keepUnmappedLocal" => KeepUnmappedLocal(Primitive<bool>)),
    ("mappingType" => MappingType(MappingType)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MappingType {
    #[serde(rename = "HK_RAGDOLL_MAPPING")]
    HkRagdollMapping = 0,
    #[serde(rename = "HK_RETARGETING_MAPPING")]
    HkRetargetingMapping = 1,
}
