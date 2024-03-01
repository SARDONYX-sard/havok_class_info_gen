//! A Rust structure that implements a serializer/deserializer corresponding to `hkbSetLocalTimeOfClipGeneratorCommand`, a class defined in C++
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
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbSetLocalTimeOfClipGeneratorCommand<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbSetLocalTimeOfClipGeneratorCommand"`: Name of this class.
    #[serde(default = "HkbSetLocalTimeOfClipGeneratorCommand::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xfab12b45`: Unique value of this class.
    #[serde(default = "HkbSetLocalTimeOfClipGeneratorCommand::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbSetLocalTimeOfClipGeneratorCommandHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbSetLocalTimeOfClipGeneratorCommandHkParam<'a>>
}

impl HkbSetLocalTimeOfClipGeneratorCommand<'_> {
    /// Return `"hkbSetLocalTimeOfClipGeneratorCommand"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbSetLocalTimeOfClipGeneratorCommand".into()
    }

    /// Return `"0xfab12b45"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xfab12b45".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbSetLocalTimeOfClipGeneratorCommandHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"characterId"`
    /// -   type: `hkUint64`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterId")]
    CharacterId(u64),
    /// # Information on fields in the original C++ class
    /// -   name:`"localTime"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "localTime")]
    LocalTime(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"nodeId"`
    /// -   type: `hkInt16`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nodeId")]
    NodeId(i16),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbSetLocalTimeOfClipGeneratorCommandHkParam<'de>, "@name",
    ("characterId" => CharacterId(u64)),
    ("localTime" => LocalTime(f64)),
    ("nodeId" => NodeId(i16)),
}
