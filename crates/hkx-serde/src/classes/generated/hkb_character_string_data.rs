//! A Rust structure that implements a serializer/deserializer corresponding to `hkbCharacterStringData`, a class defined in C++
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
/// -    size: 132
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 5
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbCharacterStringData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbCharacterStringData"`: Name of this class.
    #[serde(default = "HkbCharacterStringData::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x655b42bc`: Unique value of this class.
    #[serde(default = "HkbCharacterStringData::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbCharacterStringDataHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbCharacterStringDataHkParam<'a>>
}

impl HkbCharacterStringData<'_> {
    /// Return `"hkbCharacterStringData"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbCharacterStringData".into()
    }

    /// Return `"0x655b42bc"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x655b42bc".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbCharacterStringDataHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"deformableSkinNames"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deformableSkinNames")]
    DeformableSkinNames(Vec<String>),
    /// # Information on fields in the original C++ class
    /// -   name:`"rigidSkinNames"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rigidSkinNames")]
    RigidSkinNames(Vec<String>),
    /// # Information on fields in the original C++ class
    /// -   name:`"animationNames"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "animationNames")]
    AnimationNames(Vec<String>),
    /// # Information on fields in the original C++ class
    /// -   name:`"animationFilenames"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "animationFilenames")]
    AnimationFilenames(Vec<String>),
    /// # Information on fields in the original C++ class
    /// -   name:`"characterPropertyNames"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterPropertyNames")]
    CharacterPropertyNames(Vec<String>),
    /// # Information on fields in the original C++ class
    /// -   name:`"retargetingSkeletonMapperFilenames"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "retargetingSkeletonMapperFilenames")]
    RetargetingSkeletonMapperFilenames(Vec<String>),
    /// # Information on fields in the original C++ class
    /// -   name:`"lodNames"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lodNames")]
    LodNames(Vec<String>),
    /// # Information on fields in the original C++ class
    /// -   name:`"mirroredSyncPointSubstringsA"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mirroredSyncPointSubstringsA")]
    MirroredSyncPointSubstringsA(Vec<String>),
    /// # Information on fields in the original C++ class
    /// -   name:`"mirroredSyncPointSubstringsB"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mirroredSyncPointSubstringsB")]
    MirroredSyncPointSubstringsB(Vec<String>),
    /// # Information on fields in the original C++ class
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(String),
    /// # Information on fields in the original C++ class
    /// -   name:`"rigName"`
    /// -   type: `hkStringPtr`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rigName")]
    RigName(String),
    /// # Information on fields in the original C++ class
    /// -   name:`"ragdollName"`
    /// -   type: `hkStringPtr`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ragdollName")]
    RagdollName(String),
    /// # Information on fields in the original C++ class
    /// -   name:`"behaviorFilename"`
    /// -   type: `hkStringPtr`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "behaviorFilename")]
    BehaviorFilename(String),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacterStringDataHkParam<'de>, "@name",
    ("deformableSkinNames" => DeformableSkinNames(Vec<String>)),
    ("rigidSkinNames" => RigidSkinNames(Vec<String>)),
    ("animationNames" => AnimationNames(Vec<String>)),
    ("animationFilenames" => AnimationFilenames(Vec<String>)),
    ("characterPropertyNames" => CharacterPropertyNames(Vec<String>)),
    ("retargetingSkeletonMapperFilenames" => RetargetingSkeletonMapperFilenames(Vec<String>)),
    ("lodNames" => LodNames(Vec<String>)),
    ("mirroredSyncPointSubstringsA" => MirroredSyncPointSubstringsA(Vec<String>)),
    ("mirroredSyncPointSubstringsB" => MirroredSyncPointSubstringsB(Vec<String>)),
    ("name" => Name(String)),
    ("rigName" => RigName(String)),
    ("ragdollName" => RagdollName(String)),
    ("behaviorFilename" => BehaviorFilename(String)),
}
