//! A Rust structure that implements a serializer/deserializer corresponding to `hkbPoweredRagdollControlsModifier`, a class defined in C++
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
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 5
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbPoweredRagdollControlsModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbPoweredRagdollControlsModifier"`: The original C++ class name.
    #[serde(default = "HkbPoweredRagdollControlsModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x7cb54065`: Unique value of this class.
    #[serde(default = "HkbPoweredRagdollControlsModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbPoweredRagdollControlsModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbPoweredRagdollControlsModifierHkParam<'a>>
}

impl HkbPoweredRagdollControlsModifier<'_> {
    /// Return `"hkbPoweredRagdollControlsModifier"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbPoweredRagdollControlsModifier".into()
    }

    /// Return `"0x7cb54065"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x7cb54065".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbPoweredRagdollControlsModifierHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"controlData"`
    /// -   type: `struct hkbPoweredRagdollControlData`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "controlData")]
    ControlData(HkbPoweredRagdollControlData),
    /// # Field information in the original C++ class
    /// -   name:`"bones"`
    /// -   type: `struct hkbBoneIndexArray*`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bones")]
    Bones(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"worldFromModelModeData"`
    /// -   type: `struct hkbWorldFromModelModeData`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "worldFromModelModeData")]
    WorldFromModelModeData(HkbWorldFromModelModeData),
    /// # Field information in the original C++ class
    /// -   name:`"boneWeights"`
    /// -   type: `struct hkbBoneWeightArray*`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "boneWeights")]
    BoneWeights(Cow<'a, str>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbPoweredRagdollControlsModifierHkParam<'de>, "@name",
    ("controlData" => ControlData(HkbPoweredRagdollControlData)),
    ("bones" => Bones(Cow<'a, str>)),
    ("worldFromModelModeData" => WorldFromModelModeData(HkbWorldFromModelModeData)),
    ("boneWeights" => BoneWeights(Cow<'a, str>)),
}
