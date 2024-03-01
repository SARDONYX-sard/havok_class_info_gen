//! A Rust structure that implements a serializer/deserializer corresponding to `hkbHandle`, a class defined in C++
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
/// -    size: 24
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbHandle<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbHandle"`: Name of this class.
    #[serde(default = "HkbHandle::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xd8b6401c`: Unique value of this class.
    #[serde(default = "HkbHandle::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbHandleHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbHandleHkParam<'a>>
}

impl HkbHandle<'_> {
    /// Return `"hkbHandle"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbHandle".into()
    }

    /// Return `"0xd8b6401c"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xd8b6401c".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbHandleHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"frame"`
    /// -   type: `struct hkLocalFrame*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "frame")]
    Frame(Box<HkLocalFrame>),
    /// # Information on fields in the original C++ class
    /// -   name:`"rigidBody"`
    /// -   type: `struct hkpRigidBody*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rigidBody")]
    RigidBody(Box<HkpRigidBody>),
    /// # Information on fields in the original C++ class
    /// -   name:`"character"`
    /// -   type: `struct hkbCharacter*`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "character")]
    Character(Box<HkbCharacter>),
    /// # Information on fields in the original C++ class
    /// -   name:`"animationBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "animationBoneIndex")]
    AnimationBoneIndex(i16),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbHandleHkParam<'de>, "@name",
    ("frame" => Frame(Box<HkLocalFrame>)),
    ("rigidBody" => RigidBody(Box<HkpRigidBody>)),
    ("character" => Character(Box<HkbCharacter>)),
    ("animationBoneIndex" => AnimationBoneIndex(i16)),
}
