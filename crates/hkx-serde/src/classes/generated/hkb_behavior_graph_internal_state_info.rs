//! A Rust structure that implements a serializer/deserializer corresponding to `hkbBehaviorGraphInternalStateInfo`, a class defined in C++
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
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbBehaviorGraphInternalStateInfo<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbBehaviorGraphInternalStateInfo"`: Name of this class.
    #[serde(default = "HkbBehaviorGraphInternalStateInfo::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x645f898b`: Unique value of this class.
    #[serde(default = "HkbBehaviorGraphInternalStateInfo::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbBehaviorGraphInternalStateInfoHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbBehaviorGraphInternalStateInfoHkParam<'a>>
}

impl HkbBehaviorGraphInternalStateInfo<'_> {
    /// Return `"hkbBehaviorGraphInternalStateInfo"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbBehaviorGraphInternalStateInfo".into()
    }

    /// Return `"0x645f898b"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x645f898b".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbBehaviorGraphInternalStateInfoHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"characterId"`
    /// -   type: `hkUint64`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterId")]
    CharacterId(u64),
    /// # Information on fields in the original C++ class
    /// -   name:`"internalState"`
    /// -   type: `struct hkbBehaviorGraphInternalState*`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "internalState")]
    InternalState(Box<HkbBehaviorGraphInternalState>),
    /// # Information on fields in the original C++ class
    /// -   name:`"auxiliaryNodeInfo"`
    /// -   type: `hkArray&lt;hkbAuxiliaryNodeInfo*&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "auxiliaryNodeInfo")]
    AuxiliaryNodeInfo(Vec<Box<HkbAuxiliaryNodeInfo>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"activeEventIds"`
    /// -   type: `hkArray&lt;hkInt16&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "activeEventIds")]
    ActiveEventIds(Vec<i16>),
    /// # Information on fields in the original C++ class
    /// -   name:`"activeVariableIds"`
    /// -   type: `hkArray&lt;hkInt16&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "activeVariableIds")]
    ActiveVariableIds(Vec<i16>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbBehaviorGraphInternalStateInfoHkParam<'de>, "@name",
    ("characterId" => CharacterId(u64)),
    ("internalState" => InternalState(Box<HkbBehaviorGraphInternalState>)),
    ("auxiliaryNodeInfo" => AuxiliaryNodeInfo(Vec<Box<HkbAuxiliaryNodeInfo>>)),
    ("activeEventIds" => ActiveEventIds(Vec<i16>)),
    ("activeVariableIds" => ActiveVariableIds(Vec<i16>)),
}
