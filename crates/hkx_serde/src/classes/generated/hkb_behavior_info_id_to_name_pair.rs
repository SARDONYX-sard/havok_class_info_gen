//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbBehaviorInfoIdToNamePair`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#![allow(
  clippy::clone_on_copy,
  clippy::unit_arg
)]

#[allow(unused)]
use super::*;
#[allow(unused)]
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkbBehaviorInfoIdToNamePair`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0x35a0439a`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbBehaviorInfoIdToNamePair<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"behaviorName"`
    /// -   type: `hkStringPtr`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub behavior_name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"nodeName"`
    /// -   type: `hkStringPtr`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub node_name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"toolType"`
    /// -   type: `enum ToolNodeType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub tool_type: ToolNodeType,
    /// # C++ Class Fields Info
    /// -   name:`"id"`
    /// -   type: `hkInt16`
    /// - offset: 10
    /// -  flags: `FLAGS_NONE`
    pub id: i16,
}

impl Serialize for HkbBehaviorInfoIdToNamePair<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbBehaviorInfoIdToNamePairVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbBehaviorInfoIdToNamePair<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbBehaviorInfoIdToNamePairVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbBehaviorInfoIdToNamePairVisitor<'a>>> for HkbBehaviorInfoIdToNamePair<'a> {
    fn from(_values: Vec<HkbBehaviorInfoIdToNamePairVisitor<'a>>) -> Self {
            let mut behavior_name = None;
            let mut node_name = None;
            let mut tool_type = None;
            let mut id = None;


        for _value in _values {
            match _value {
                HkbBehaviorInfoIdToNamePairVisitor::BehaviorName(m) => behavior_name = Some(m),
                HkbBehaviorInfoIdToNamePairVisitor::NodeName(m) => node_name = Some(m),
                HkbBehaviorInfoIdToNamePairVisitor::ToolType(m) => tool_type = Some(m),
                HkbBehaviorInfoIdToNamePairVisitor::Id(m) => id = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            behavior_name: behavior_name.unwrap_or_default().into_inner(),
            node_name: node_name.unwrap_or_default().into_inner(),
            tool_type: tool_type.unwrap_or_default().into_inner(),
            id: id.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbBehaviorInfoIdToNamePair<'a>> for Vec<HkbBehaviorInfoIdToNamePairVisitor<'a>> {
    fn from(data: &HkbBehaviorInfoIdToNamePair<'a>) -> Self {
        vec![
            HkbBehaviorInfoIdToNamePairVisitor::BehaviorName(data.behavior_name.clone().into()),
            HkbBehaviorInfoIdToNamePairVisitor::NodeName(data.node_name.clone().into()),
            HkbBehaviorInfoIdToNamePairVisitor::ToolType(data.tool_type.clone().into()),
            HkbBehaviorInfoIdToNamePairVisitor::Id(data.id.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbBehaviorInfoIdToNamePair<'de> {
    fn from_bytes<B>(
        _bytes: &'bytes [u8],
        _de: &mut PackFileDeserializer,
    ) -> Result<Self>
    where
        B: ByteOrder,
        Self: Sized + 'de
    {
        todo!()
    }
}


/// # Why use Visitor pattern?
/// Since the C++ field must be deserialized from the `name` attribute name of the `hkparam` in the XML,
/// this is accomplished by having the Visitor process the internally tagged enum and convert it.
/// Leakage of field items may occur if Vec<enum> is left as it is.
///
/// struct -> (De)serialize by visitor -> struct
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
enum HkbBehaviorInfoIdToNamePairVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "behaviorName")]
    BehaviorName(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "nodeName")]
    NodeName(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "toolType")]
    ToolType(Primitive<ToolNodeType>),
    /// Visitor fields
    #[serde(rename = "id")]
    Id(Primitive<i16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbBehaviorInfoIdToNamePairVisitor<'de>, "@name",
    ("behaviorName" => BehaviorName(Primitive<Cow<'de, str>>)),
    ("nodeName" => NodeName(Primitive<Cow<'de, str>>)),
    ("toolType" => ToolType(Primitive<ToolNodeType>)),
    ("id" => Id(Primitive<i16>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum ToolNodeType {
    #[serde(rename = "NODE_TYPE_UNKNOWN")]
    #[default]
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
