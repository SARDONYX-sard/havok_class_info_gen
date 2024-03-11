//! A Rust structure that implements a serializer/deserializer corresponding to `hkaSkeletonMapperDataSimpleMapping`, a class defined in C++
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
/// -    size: 64
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkaSkeletonMapperDataSimpleMapping<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkaSkeletonMapperDataSimpleMapping"`: The original C++ class name.
    #[serde(default = "HkaSkeletonMapperDataSimpleMapping::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x3405deca`: Unique value of this class.
    #[serde(default = "HkaSkeletonMapperDataSimpleMapping::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkaSkeletonMapperDataSimpleMappingHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkaSkeletonMapperDataSimpleMappingHkParam<'a>>
}

impl HkaSkeletonMapperDataSimpleMapping<'_> {
    /// Return `"hkaSkeletonMapperDataSimpleMapping"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkaSkeletonMapperDataSimpleMapping".into()
    }

    /// Return `"0x3405deca"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x3405deca".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaSkeletonMapperDataSimpleMappingHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"boneA"`
    /// -   type: `hkInt16`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "boneA")]
    BoneA(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"boneB"`
    /// -   type: `hkInt16`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "boneB")]
    BoneB(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"aFromBTransform"`
    /// -   type: `hkQsTransform`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "aFromBTransform")]
    AFromBTransform(QsTransform<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkaSkeletonMapperDataSimpleMappingHkParam<'de>, "@name",
    ("boneA" => BoneA(Primitive<i16>)),
    ("boneB" => BoneB(Primitive<i16>)),
    ("aFromBTransform" => AFromBTransform(QsTransform<f32>)),
}
