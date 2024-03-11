//! A Rust structure that implements a serializer/deserializer corresponding to `hkbCharacterAddedInfo`, a class defined in C++
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
/// -    size: 96
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbCharacterAddedInfo<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbCharacterAddedInfo"`: The original C++ class name.
    #[serde(default = "HkbCharacterAddedInfo::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x3544e182`: Unique value of this class.
    #[serde(default = "HkbCharacterAddedInfo::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbCharacterAddedInfoHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbCharacterAddedInfoHkParam<'a>>
}

impl HkbCharacterAddedInfo<'_> {
    /// Return `"hkbCharacterAddedInfo"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbCharacterAddedInfo".into()
    }

    /// Return `"0x3544e182"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x3544e182".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbCharacterAddedInfoHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"characterId"`
    /// -   type: `hkUint64`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterId")]
    CharacterId(Primitive<u64>),
    /// # Field information in the original C++ class
    /// -   name:`"instanceName"`
    /// -   type: `hkStringPtr`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "instanceName")]
    InstanceName(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"templateName"`
    /// -   type: `hkStringPtr`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "templateName")]
    TemplateName(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"fullPathToProject"`
    /// -   type: `hkStringPtr`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fullPathToProject")]
    FullPathToProject(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"skeleton"`
    /// -   type: `struct hkaSkeleton*`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "skeleton")]
    Skeleton(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"worldFromModel"`
    /// -   type: `hkQsTransform`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "worldFromModel")]
    WorldFromModel(QsTransform<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"poseModelSpace"`
    /// -   type: `hkArray&lt;hkQsTransform&gt;`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "poseModelSpace")]
    PoseModelSpace(Vec<QsTransform<f32>>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacterAddedInfoHkParam<'de>, "@name",
    ("characterId" => CharacterId(Primitive<u64>)),
    ("instanceName" => InstanceName(Primitive<Cow<'a, str>>)),
    ("templateName" => TemplateName(Primitive<Cow<'a, str>>)),
    ("fullPathToProject" => FullPathToProject(Primitive<Cow<'a, str>>)),
    ("skeleton" => Skeleton(Cow<'a, str>)),
    ("worldFromModel" => WorldFromModel(QsTransform<f32>)),
    ("poseModelSpace" => PoseModelSpace(Vec<QsTransform<f32>>)),
}
