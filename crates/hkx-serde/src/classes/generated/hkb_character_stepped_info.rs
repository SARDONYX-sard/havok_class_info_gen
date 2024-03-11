//! A Rust structure that implements a serializer/deserializer corresponding to `hkbCharacterSteppedInfo`, a class defined in C++
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
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 2
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbCharacterSteppedInfo<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbCharacterSteppedInfo"`: The original C++ class name.
    #[serde(default = "HkbCharacterSteppedInfo::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x2eda84f8`: Unique value of this class.
    #[serde(default = "HkbCharacterSteppedInfo::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbCharacterSteppedInfoHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbCharacterSteppedInfoHkParam<'a>>
}

impl HkbCharacterSteppedInfo<'_> {
    /// Return `"hkbCharacterSteppedInfo"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbCharacterSteppedInfo".into()
    }

    /// Return `"0x2eda84f8"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x2eda84f8".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbCharacterSteppedInfoHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"characterId"`
    /// -   type: `hkUint64`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterId")]
    CharacterId(Primitive<u64>),
    /// # Field information in the original C++ class
    /// -   name:`"deltaTime"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deltaTime")]
    DeltaTime(Primitive<f32>),
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
    /// # Field information in the original C++ class
    /// -   name:`"rigidAttachmentTransforms"`
    /// -   type: `hkArray&lt;hkQsTransform&gt;`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rigidAttachmentTransforms")]
    RigidAttachmentTransforms(Vec<QsTransform<f32>>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacterSteppedInfoHkParam<'de>, "@name",
    ("characterId" => CharacterId(Primitive<u64>)),
    ("deltaTime" => DeltaTime(Primitive<f32>)),
    ("worldFromModel" => WorldFromModel(QsTransform<f32>)),
    ("poseModelSpace" => PoseModelSpace(Vec<QsTransform<f32>>)),
    ("rigidAttachmentTransforms" => RigidAttachmentTransforms(Vec<QsTransform<f32>>)),
}
