//! A Rust structure that implements a serializer/deserializer corresponding to `hkbAuxiliaryNodeInfo`, a class defined in C++
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
/// -    size: 28
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbAuxiliaryNodeInfo<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbAuxiliaryNodeInfo"`: Name of this class.
    #[serde(default = "HkbAuxiliaryNodeInfo::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xca0888ca`: Unique value of this class.
    #[serde(default = "HkbAuxiliaryNodeInfo::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbAuxiliaryNodeInfoHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbAuxiliaryNodeInfoHkParam<'a>>
}

impl HkbAuxiliaryNodeInfo<'_> {
    /// Return `"hkbAuxiliaryNodeInfo"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbAuxiliaryNodeInfo".into()
    }

    /// Return `"0xca0888ca"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xca0888ca".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbAuxiliaryNodeInfoHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"type"`
    /// -   type: `enum NodeType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(NodeType),
    /// # Information on fields in the original C++ class
    /// -   name:`"depth"`
    /// -   type: `hkUint8`
    /// - offset: 9
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "depth")]
    Depth(u8),
    /// # Information on fields in the original C++ class
    /// -   name:`"referenceBehaviorName"`
    /// -   type: `hkStringPtr`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "referenceBehaviorName")]
    ReferenceBehaviorName(String),
    /// # Information on fields in the original C++ class
    /// -   name:`"selfTransitionNames"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "selfTransitionNames")]
    SelfTransitionNames(Vec<String>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbAuxiliaryNodeInfoHkParam<'de>, "@name",
    ("type" => Type(NodeType)),
    ("depth" => Depth(u8)),
    ("referenceBehaviorName" => ReferenceBehaviorName(String)),
    ("selfTransitionNames" => SelfTransitionNames(Vec<String>)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum NodeType {
    #[serde(rename = "NODE_TYPE_UNKNOWN")]
    NodeTypeUnknown = 0,
    #[serde(rename = "NODE_TYPE_NODE")]
    NodeTypeNode = 1,
    #[serde(rename = "NODE_TYPE_TRANSITION")]
    NodeTypeTransition = 2,
    #[serde(rename = "NODE_TYPE_WILDCARD_TRANSITION")]
    NodeTypeWildcardTransition = 3,
    #[serde(rename = "NODE_TYPE_STATE")]
    NodeTypeState = 4,
    #[serde(rename = "NODE_TYPE_STATE_MACHINE")]
    NodeTypeStateMachine = 5,
    #[serde(rename = "NODE_TYPE_MODIFIER_GENERATOR")]
    NodeTypeModifierGenerator = 6,
    #[serde(rename = "NODE_TYPE_MODIFIER")]
    NodeTypeModifier = 7,
    #[serde(rename = "NODE_TYPE_CLIP")]
    NodeTypeClip = 8,
    #[serde(rename = "NODE_TYPE_BLEND")]
    NodeTypeBlend = 9,
    #[serde(rename = "NODE_TYPE_TRANSITION_EFFECT")]
    NodeTypeTransitionEffect = 10,
}
