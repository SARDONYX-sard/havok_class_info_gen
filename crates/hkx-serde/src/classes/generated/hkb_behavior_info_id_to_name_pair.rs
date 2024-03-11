//! A Rust structure that implements a serializer/deserializer corresponding to `hkbBehaviorInfoIdToNamePair`, a class defined in C++
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
/// -    size: 12
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbBehaviorInfoIdToNamePair<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbBehaviorInfoIdToNamePair"`: The original C++ class name.
    #[serde(default = "HkbBehaviorInfoIdToNamePair::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x35a0439a`: Unique value of this class.
    #[serde(default = "HkbBehaviorInfoIdToNamePair::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbBehaviorInfoIdToNamePairHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbBehaviorInfoIdToNamePairHkParam<'a>>
}

impl HkbBehaviorInfoIdToNamePair<'_> {
    /// Return `"hkbBehaviorInfoIdToNamePair"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbBehaviorInfoIdToNamePair".into()
    }

    /// Return `"0x35a0439a"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x35a0439a".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbBehaviorInfoIdToNamePairHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"behaviorName"`
    /// -   type: `hkStringPtr`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "behaviorName")]
    BehaviorName(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"nodeName"`
    /// -   type: `hkStringPtr`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nodeName")]
    NodeName(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"toolType"`
    /// -   type: `enum ToolNodeType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "toolType")]
    ToolType(ToolNodeType),
    /// # Field information in the original C++ class
    /// -   name:`"id"`
    /// -   type: `hkInt16`
    /// - offset: 10
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "id")]
    Id(Primitive<i16>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbBehaviorInfoIdToNamePairHkParam<'de>, "@name",
    ("behaviorName" => BehaviorName(Primitive<Cow<'a, str>>)),
    ("nodeName" => NodeName(Primitive<Cow<'a, str>>)),
    ("toolType" => ToolType(ToolNodeType)),
    ("id" => Id(Primitive<i16>)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ToolNodeType {
    #[serde(rename = "NODE_TYPE_UNKNOWN")]
    NodeTypeUnknown = 0,
    #[serde(rename = "NODE_TYPE_STATE_MACHINE")]
    NodeTypeStateMachine = 1,
    #[serde(rename = "NODE_TYPE_CLIP")]
    NodeTypeClip = 2,
    #[serde(rename = "NODE_TYPE_BLEND")]
    NodeTypeBlend = 3,
    #[serde(rename = "NODE_TYPE_MODIFIER")]
    NodeTypeModifier = 4,
    #[serde(rename = "NODE_TYPE_GENERATOR")]
    NodeTypeGenerator = 5,
    #[serde(rename = "NODE_TYPE_MODIFIER_GENERATOR")]
    NodeTypeModifierGenerator = 6,
    #[serde(rename = "NODE_TYPE_TRANSITION_EFFECT")]
    NodeTypeTransitionEffect = 7,
    #[serde(rename = "NODE_TYPE_BEHAVIOR_FILE_REFERENCE")]
    NodeTypeBehaviorFileReference = 8,
}
