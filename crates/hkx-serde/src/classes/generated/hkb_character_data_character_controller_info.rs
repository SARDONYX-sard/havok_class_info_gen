//! A Rust structure that implements a serializer/deserializer corresponding to `hkbCharacterDataCharacterControllerInfo`, a class defined in C++
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
/// -    size: 16
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbCharacterDataCharacterControllerInfo<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbCharacterDataCharacterControllerInfo"`: Name of this class.
    #[serde(default = "HkbCharacterDataCharacterControllerInfo::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xa0f415bf`: Unique value of this class.
    #[serde(default = "HkbCharacterDataCharacterControllerInfo::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbCharacterDataCharacterControllerInfoHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbCharacterDataCharacterControllerInfoHkParam<'a>>
}

impl HkbCharacterDataCharacterControllerInfo<'_> {
    /// Return `"hkbCharacterDataCharacterControllerInfo"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbCharacterDataCharacterControllerInfo".into()
    }

    /// Return `"0xa0f415bf"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xa0f415bf".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbCharacterDataCharacterControllerInfoHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"capsuleHeight"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "capsuleHeight")]
    CapsuleHeight(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"capsuleRadius"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "capsuleRadius")]
    CapsuleRadius(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"collisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collisionFilterInfo")]
    CollisionFilterInfo(u32),
    /// # Information on fields in the original C++ class
    /// -   name:`"characterControllerCinfo"`
    /// -   type: `struct hkpCharacterControllerCinfo*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterControllerCinfo")]
    CharacterControllerCinfo(Box<HkpCharacterControllerCinfo>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacterDataCharacterControllerInfoHkParam<'de>, "@name",
    ("capsuleHeight" => CapsuleHeight(f64)),
    ("capsuleRadius" => CapsuleRadius(f64)),
    ("collisionFilterInfo" => CollisionFilterInfo(u32)),
    ("characterControllerCinfo" => CharacterControllerCinfo(Box<HkpCharacterControllerCinfo>)),
}
