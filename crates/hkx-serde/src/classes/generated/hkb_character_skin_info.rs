//! A Rust structure that implements a serializer/deserializer corresponding to `hkbCharacterSkinInfo`, a class defined in C++
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
/// -    size: 40
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 2
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbCharacterSkinInfo<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbCharacterSkinInfo"`: Name of this class.
    #[serde(default = "HkbCharacterSkinInfo::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x180d900d`: Unique value of this class.
    #[serde(default = "HkbCharacterSkinInfo::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbCharacterSkinInfoHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbCharacterSkinInfoHkParam<'a>>
}

impl HkbCharacterSkinInfo<'_> {
    /// Return `"hkbCharacterSkinInfo"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbCharacterSkinInfo".into()
    }

    /// Return `"0x180d900d"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x180d900d".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbCharacterSkinInfoHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"characterId"`
    /// -   type: `hkUint64`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterId")]
    CharacterId(u64),
    /// # Information on fields in the original C++ class
    /// -   name:`"deformableSkins"`
    /// -   type: `hkArray&lt;hkUint64&gt;`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deformableSkins")]
    DeformableSkins(Vec<u64>),
    /// # Information on fields in the original C++ class
    /// -   name:`"rigidSkins"`
    /// -   type: `hkArray&lt;hkUint64&gt;`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rigidSkins")]
    RigidSkins(Vec<u64>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacterSkinInfoHkParam<'de>, "@name",
    ("characterId" => CharacterId(u64)),
    ("deformableSkins" => DeformableSkins(Vec<u64>)),
    ("rigidSkins" => RigidSkins(Vec<u64>)),
}