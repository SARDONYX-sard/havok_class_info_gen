//! A Rust structure that implements a serializer/deserializer corresponding to `hkbCharacterControlCommand`, a class defined in C++
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
/// -    size: 24
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbCharacterControlCommand<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbCharacterControlCommand"`: The original C++ class name.
    #[serde(default = "HkbCharacterControlCommand::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x7a195d1d`: Unique value of this class.
    #[serde(default = "HkbCharacterControlCommand::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbCharacterControlCommandHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbCharacterControlCommandHkParam<'a>>
}

impl HkbCharacterControlCommand<'_> {
    /// Return `"hkbCharacterControlCommand"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbCharacterControlCommand".into()
    }

    /// Return `"0x7a195d1d"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x7a195d1d".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbCharacterControlCommandHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"characterId"`
    /// -   type: `hkUint64`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterId")]
    CharacterId(Primitive<u64>),
    /// # Field information in the original C++ class
    /// -   name:`"command"`
    /// -   type: `enum CharacterControlCommand`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "command")]
    Command(CharacterControlCommand),
    /// # Field information in the original C++ class
    /// -   name:`"padding"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "padding")]
    Padding(Primitive<i32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacterControlCommandHkParam<'de>, "@name",
    ("characterId" => CharacterId(Primitive<u64>)),
    ("command" => Command(CharacterControlCommand)),
    ("padding" => Padding(Primitive<i32>)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CharacterControlCommand {
    #[serde(rename = "COMMAND_HIDE")]
    CommandHide = 0,
    #[serde(rename = "COMMAND_SHOW")]
    CommandShow = 1,
}
