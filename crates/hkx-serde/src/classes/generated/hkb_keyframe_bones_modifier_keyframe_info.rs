//! A Rust structure that implements a serializer/deserializer corresponding to `hkbKeyframeBonesModifierKeyframeInfo`, a class defined in C++
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
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbKeyframeBonesModifierKeyframeInfo<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbKeyframeBonesModifierKeyframeInfo"`: Name of this class.
    #[serde(default = "HkbKeyframeBonesModifierKeyframeInfo::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x72deb7a6`: Unique value of this class.
    #[serde(default = "HkbKeyframeBonesModifierKeyframeInfo::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbKeyframeBonesModifierKeyframeInfoHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbKeyframeBonesModifierKeyframeInfoHkParam<'a>>
}

impl HkbKeyframeBonesModifierKeyframeInfo<'_> {
    /// Return `"hkbKeyframeBonesModifierKeyframeInfo"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbKeyframeBonesModifierKeyframeInfo".into()
    }

    /// Return `"0x72deb7a6"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x72deb7a6".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbKeyframeBonesModifierKeyframeInfoHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"keyframedPosition"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "keyframedPosition")]
    KeyframedPosition(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"keyframedRotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "keyframedRotation")]
    KeyframedRotation(cgmath::Quaternion<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"boneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "boneIndex")]
    BoneIndex(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"isValid"`
    /// -   type: `hkBool`
    /// - offset: 34
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isValid")]
    IsValid(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbKeyframeBonesModifierKeyframeInfoHkParam<'de>, "@name",
    ("keyframedPosition" => KeyframedPosition(cgmath::Vector4<f32>)),
    ("keyframedRotation" => KeyframedRotation(cgmath::Quaternion<f32>)),
    ("boneIndex" => BoneIndex(i16)),
    ("isValid" => IsValid(bool)),
}
