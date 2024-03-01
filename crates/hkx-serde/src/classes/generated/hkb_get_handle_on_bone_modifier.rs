//! A Rust structure that implements a serializer/deserializer corresponding to `hkbGetHandleOnBoneModifier`, a class defined in C++
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
/// -    size: 56
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbGetHandleOnBoneModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbGetHandleOnBoneModifier"`: Name of this class.
    #[serde(default = "HkbGetHandleOnBoneModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x50c34a17`: Unique value of this class.
    #[serde(default = "HkbGetHandleOnBoneModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbGetHandleOnBoneModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbGetHandleOnBoneModifierHkParam<'a>>
}

impl HkbGetHandleOnBoneModifier<'_> {
    /// Return `"hkbGetHandleOnBoneModifier"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbGetHandleOnBoneModifier".into()
    }

    /// Return `"0x50c34a17"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x50c34a17".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbGetHandleOnBoneModifierHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"handleOut"`
    /// -   type: `struct hkbHandle*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handleOut")]
    HandleOut(Box<HkbHandle>),
    /// # Information on fields in the original C++ class
    /// -   name:`"localFrameName"`
    /// -   type: `hkStringPtr`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "localFrameName")]
    LocalFrameName(String),
    /// # Information on fields in the original C++ class
    /// -   name:`"ragdollBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ragdollBoneIndex")]
    RagdollBoneIndex(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"animationBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 54
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "animationBoneIndex")]
    AnimationBoneIndex(i16),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbGetHandleOnBoneModifierHkParam<'de>, "@name",
    ("handleOut" => HandleOut(Box<HkbHandle>)),
    ("localFrameName" => LocalFrameName(String)),
    ("ragdollBoneIndex" => RagdollBoneIndex(i16)),
    ("animationBoneIndex" => AnimationBoneIndex(i16)),
}