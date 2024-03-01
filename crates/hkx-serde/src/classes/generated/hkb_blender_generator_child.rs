//! A Rust structure that implements a serializer/deserializer corresponding to `hkbBlenderGeneratorChild`, a class defined in C++
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
/// -    size: 48
/// -  vtable: true
/// -  parent: hkbBindable/`2c1432d7`(Non prefix hex signature)
/// - version: 2
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbBlenderGeneratorChild<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbBlenderGeneratorChild"`: Name of this class.
    #[serde(default = "HkbBlenderGeneratorChild::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xe2b384b0`: Unique value of this class.
    #[serde(default = "HkbBlenderGeneratorChild::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbBlenderGeneratorChildHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbBlenderGeneratorChildHkParam<'a>>
}

impl HkbBlenderGeneratorChild<'_> {
    /// Return `"hkbBlenderGeneratorChild"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbBlenderGeneratorChild".into()
    }

    /// Return `"0xe2b384b0"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xe2b384b0".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbBlenderGeneratorChildHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"generator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "generator")]
    Generator(Box<HkbGenerator>),
    /// # Information on fields in the original C++ class
    /// -   name:`"boneWeights"`
    /// -   type: `struct hkbBoneWeightArray*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "boneWeights")]
    BoneWeights(Box<HkbBoneWeightArray>),
    /// # Information on fields in the original C++ class
    /// -   name:`"weight"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "weight")]
    Weight(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"worldFromModelWeight"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "worldFromModelWeight")]
    WorldFromModelWeight(f64),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbBlenderGeneratorChildHkParam<'de>, "@name",
    ("generator" => Generator(Box<HkbGenerator>)),
    ("boneWeights" => BoneWeights(Box<HkbBoneWeightArray>)),
    ("weight" => Weight(f64)),
    ("worldFromModelWeight" => WorldFromModelWeight(f64)),
}
