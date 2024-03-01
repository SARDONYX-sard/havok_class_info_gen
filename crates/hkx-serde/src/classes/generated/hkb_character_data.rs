//! A Rust structure that implements a serializer/deserializer corresponding to `hkbCharacterData`, a class defined in C++
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
/// -    size: 144
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 7
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbCharacterData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbCharacterData"`: Name of this class.
    #[serde(default = "HkbCharacterData::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x300d6808`: Unique value of this class.
    #[serde(default = "HkbCharacterData::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbCharacterDataHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbCharacterDataHkParam<'a>>
}

impl HkbCharacterData<'_> {
    /// Return `"hkbCharacterData"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbCharacterData".into()
    }

    /// Return `"0x300d6808"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x300d6808".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbCharacterDataHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"characterControllerInfo"`
    /// -   type: `struct hkbCharacterDataCharacterControllerInfo`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterControllerInfo")]
    CharacterControllerInfo(HkbCharacterDataCharacterControllerInfo),
    /// # Information on fields in the original C++ class
    /// -   name:`"modelUpMS"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "modelUpMS")]
    ModelUpMs(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"modelForwardMS"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "modelForwardMS")]
    ModelForwardMs(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"modelRightMS"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "modelRightMS")]
    ModelRightMs(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"characterPropertyInfos"`
    /// -   type: `hkArray&lt;struct hkbVariableInfo&gt;`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterPropertyInfos")]
    CharacterPropertyInfos(Vec<HkbVariableInfo>),
    /// # Information on fields in the original C++ class
    /// -   name:`"numBonesPerLod"`
    /// -   type: `hkArray&lt;hkInt32&gt;`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numBonesPerLod")]
    NumBonesPerLod(Vec<i32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"characterPropertyValues"`
    /// -   type: `struct hkbVariableValueSet*`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterPropertyValues")]
    CharacterPropertyValues(Box<HkbVariableValueSet>),
    /// # Information on fields in the original C++ class
    /// -   name:`"footIkDriverInfo"`
    /// -   type: `struct hkbFootIkDriverInfo*`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "footIkDriverInfo")]
    FootIkDriverInfo(Box<HkbFootIkDriverInfo>),
    /// # Information on fields in the original C++ class
    /// -   name:`"handIkDriverInfo"`
    /// -   type: `struct hkbHandIkDriverInfo*`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handIkDriverInfo")]
    HandIkDriverInfo(Box<HkbHandIkDriverInfo>),
    /// # Information on fields in the original C++ class
    /// -   name:`"stringData"`
    /// -   type: `struct hkbCharacterStringData*`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "stringData")]
    StringData(Box<HkbCharacterStringData>),
    /// # Information on fields in the original C++ class
    /// -   name:`"mirroredSkeletonInfo"`
    /// -   type: `struct hkbMirroredSkeletonInfo*`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mirroredSkeletonInfo")]
    MirroredSkeletonInfo(Box<HkbMirroredSkeletonInfo>),
    /// # Information on fields in the original C++ class
    /// -   name:`"scale"`
    /// -   type: `hkReal`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "scale")]
    Scale(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"numHands"`
    /// -   type: `hkInt16`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "numHands", skip_serializing)]
    NumHands(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"numFloatSlots"`
    /// -   type: `hkInt16`
    /// - offset: 130
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "numFloatSlots", skip_serializing)]
    NumFloatSlots(i16),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacterDataHkParam<'de>, "@name",
    ("characterControllerInfo" => CharacterControllerInfo(HkbCharacterDataCharacterControllerInfo)),
    ("modelUpMS" => ModelUpMs(cgmath::Vector4<f32>)),
    ("modelForwardMS" => ModelForwardMs(cgmath::Vector4<f32>)),
    ("modelRightMS" => ModelRightMs(cgmath::Vector4<f32>)),
    ("characterPropertyInfos" => CharacterPropertyInfos(Vec<HkbVariableInfo>)),
    ("numBonesPerLod" => NumBonesPerLod(Vec<i32>)),
    ("characterPropertyValues" => CharacterPropertyValues(Box<HkbVariableValueSet>)),
    ("footIkDriverInfo" => FootIkDriverInfo(Box<HkbFootIkDriverInfo>)),
    ("handIkDriverInfo" => HandIkDriverInfo(Box<HkbHandIkDriverInfo>)),
    ("stringData" => StringData(Box<HkbCharacterStringData>)),
    ("mirroredSkeletonInfo" => MirroredSkeletonInfo(Box<HkbMirroredSkeletonInfo>)),
    ("scale" => Scale(f64)),
    ("numHands" => NumHands(i16)),
    ("numFloatSlots" => NumFloatSlots(i16)),
}
