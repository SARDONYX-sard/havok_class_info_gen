//! A Rust structure that implements a serializer/deserializer corresponding to `hkbCharacterData`, a class defined in C++
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

    /// `"hkbCharacterData"`: The original C++ class name.
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
    /// Return `"hkbCharacterData"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbCharacterData".into()
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
    /// # Field information in the original C++ class
    /// -   name:`"characterControllerInfo"`
    /// -   type: `struct hkbCharacterDataCharacterControllerInfo`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterControllerInfo")]
    CharacterControllerInfo(HkbCharacterDataCharacterControllerInfo),
    /// # Field information in the original C++ class
    /// -   name:`"modelUpMS"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "modelUpMS")]
    ModelUpMs(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"modelForwardMS"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "modelForwardMS")]
    ModelForwardMs(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"modelRightMS"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "modelRightMS")]
    ModelRightMs(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"characterPropertyInfos"`
    /// -   type: `hkArray&lt;struct hkbVariableInfo&gt;`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterPropertyInfos")]
    CharacterPropertyInfos(Vec<HkbVariableInfo>),
    /// # Field information in the original C++ class
    /// -   name:`"numBonesPerLod"`
    /// -   type: `hkArray&lt;hkInt32&gt;`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numBonesPerLod")]
    NumBonesPerLod(Vec<Primitive<i32>>),
    /// # Field information in the original C++ class
    /// -   name:`"characterPropertyValues"`
    /// -   type: `struct hkbVariableValueSet*`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterPropertyValues")]
    CharacterPropertyValues(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"footIkDriverInfo"`
    /// -   type: `struct hkbFootIkDriverInfo*`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "footIkDriverInfo")]
    FootIkDriverInfo(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"handIkDriverInfo"`
    /// -   type: `struct hkbHandIkDriverInfo*`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handIkDriverInfo")]
    HandIkDriverInfo(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"stringData"`
    /// -   type: `struct hkbCharacterStringData*`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "stringData")]
    StringData(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"mirroredSkeletonInfo"`
    /// -   type: `struct hkbMirroredSkeletonInfo*`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mirroredSkeletonInfo")]
    MirroredSkeletonInfo(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"scale"`
    /// -   type: `hkReal`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "scale")]
    Scale(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"numHands"`
    /// -   type: `hkInt16`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "numHands", skip_serializing)]
    NumHands(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"numFloatSlots"`
    /// -   type: `hkInt16`
    /// - offset: 130
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "numFloatSlots", skip_serializing)]
    NumFloatSlots(Primitive<i16>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacterDataHkParam<'de>, "@name",
    ("characterControllerInfo" => CharacterControllerInfo(HkbCharacterDataCharacterControllerInfo)),
    ("modelUpMS" => ModelUpMs(Vector4<f32>)),
    ("modelForwardMS" => ModelForwardMs(Vector4<f32>)),
    ("modelRightMS" => ModelRightMs(Vector4<f32>)),
    ("characterPropertyInfos" => CharacterPropertyInfos(Vec<HkbVariableInfo>)),
    ("numBonesPerLod" => NumBonesPerLod(Vec<Primitive<i32>>)),
    ("characterPropertyValues" => CharacterPropertyValues(Cow<'a, str>)),
    ("footIkDriverInfo" => FootIkDriverInfo(Cow<'a, str>)),
    ("handIkDriverInfo" => HandIkDriverInfo(Cow<'a, str>)),
    ("stringData" => StringData(Cow<'a, str>)),
    ("mirroredSkeletonInfo" => MirroredSkeletonInfo(Cow<'a, str>)),
    ("scale" => Scale(Primitive<f32>)),
    ("numHands" => NumHands(Primitive<i16>)),
    ("numFloatSlots" => NumFloatSlots(Primitive<i16>)),
}
