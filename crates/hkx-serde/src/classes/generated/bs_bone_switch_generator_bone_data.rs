//! A Rust structure that implements a serializer/deserializer corresponding to `BSBoneSwitchGeneratorBoneData`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
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
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct BsBoneSwitchGeneratorBoneData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"BSBoneSwitchGeneratorBoneData"`: Name of this class.
    #[serde(default = "BsBoneSwitchGeneratorBoneData::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xc1215be6`: Unique value of this class.
    #[serde(default = "BsBoneSwitchGeneratorBoneData::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<BsBoneSwitchGeneratorBoneDataHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<BsBoneSwitchGeneratorBoneDataHkParam<'a>>
}

impl BsBoneSwitchGeneratorBoneData<'_> {
    /// Return `"BSBoneSwitchGeneratorBoneData"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "BsBoneSwitchGeneratorBoneData".into()
    }

    /// Return `"0xc1215be6"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xc1215be6".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsBoneSwitchGeneratorBoneDataHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"pGenerator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "pGenerator")]
    PGenerator(Box<HkbGenerator>),
    /// # Information on fields in the original C++ class
    /// -   name:`"spBoneWeight"`
    /// -   type: `struct hkbBoneWeightArray*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "spBoneWeight")]
    SpBoneWeight(Box<HkbBoneWeightArray>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    BsBoneSwitchGeneratorBoneDataHkParam<'de>, "@name",
    ("pGenerator" => PGenerator(Box<HkbGenerator>)),
    ("spBoneWeight" => SpBoneWeight(Box<HkbBoneWeightArray>)),
}
