//! A Rust structure that implements a serializer/deserializer corresponding to `hkbNode`, a class defined in C++
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
/// -  parent: hkbBindable/`2c1432d7`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbNode<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbNode"`: Name of this class.
    #[serde(default = "HkbNode::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x6d26f61d`: Unique value of this class.
    #[serde(default = "HkbNode::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbNodeHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbNodeHkParam<'a>>
}

impl HkbNode<'_> {
    /// Return `"hkbNode"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbNode".into()
    }

    /// Return `"0x6d26f61d"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x6d26f61d".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbNodeHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData")]
    UserData(u64),
    /// # Information on fields in the original C++ class
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(String),
    /// # Information on fields in the original C++ class
    /// -   name:`"id"`
    /// -   type: `hkInt16`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "id", skip_serializing)]
    Id(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"cloneState"`
    /// -   type: `enum unknown`
    /// - offset: 38
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "cloneState", skip_serializing)]
    CloneState(Unknown),
    /// # Information on fields in the original C++ class
    /// -   name:`"padNode"`
    /// -   type: `hkBool[1]`
    /// - offset: 39
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "padNode", skip_serializing)]
    PadNode([bool; 1]),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbNodeHkParam<'de>, "@name",
    ("userData" => UserData(u64)),
    ("name" => Name(String)),
    ("id" => Id(i16)),
    ("cloneState" => CloneState(Unknown)),
    ("padNode" => PadNode([bool; 1])),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GetChildrenFlagBits {
    #[serde(rename = "FLAG_ACTIVE_ONLY")]
    FlagActiveOnly = 1,
    #[serde(rename = "FLAG_GENERATORS_ONLY")]
    FlagGeneratorsOnly = 2,
    #[serde(rename = "FLAG_IGNORE_REFERENCED_BEHAVIORS")]
    FlagIgnoreReferencedBehaviors = 4,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CloneState {
    #[serde(rename = "CLONE_STATE_DEFAULT")]
    CloneStateDefault = 0,
    #[serde(rename = "CLONE_STATE_TEMPLATE")]
    CloneStateTemplate = 1,
    #[serde(rename = "CLONE_STATE_CLONE")]
    CloneStateClone = 2,
    #[serde(rename = "CLONE_STATE_SHARABLE")]
    CloneStateSharable = 3,
}
