//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbAuxiliaryNodeInfo`
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

/// `hkbAuxiliaryNodeInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 28
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xca0888ca`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbAuxiliaryNodeInfo<'a> {
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub mem_size_and_flags: u16,
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub reference_count: i16,

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum NodeType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub _type: NodeType,
    /// # C++ Class Fields Info
    /// -   name:`"depth"`
    /// -   type: `hkUint8`
    /// - offset: 9
    /// -  flags: `FLAGS_NONE`
    pub depth: u8,
    /// # C++ Class Fields Info
    /// -   name:`"referenceBehaviorName"`
    /// -   type: `hkStringPtr`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub reference_behavior_name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"selfTransitionNames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub self_transition_names: HkArrayStringPtr<'a>,
}

impl Serialize for HkbAuxiliaryNodeInfo<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbAuxiliaryNodeInfoVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbAuxiliaryNodeInfo<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbAuxiliaryNodeInfoVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbAuxiliaryNodeInfoVisitor<'a>>> for HkbAuxiliaryNodeInfo<'a> {
    fn from(_values: Vec<HkbAuxiliaryNodeInfoVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut _type = None;
            let mut depth = None;
            let mut reference_behavior_name = None;
            let mut self_transition_names = None;


        for _value in _values {
            match _value {
                HkbAuxiliaryNodeInfoVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbAuxiliaryNodeInfoVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbAuxiliaryNodeInfoVisitor::Type(m) => _type = Some(m),
                HkbAuxiliaryNodeInfoVisitor::Depth(m) => depth = Some(m),
                HkbAuxiliaryNodeInfoVisitor::ReferenceBehaviorName(m) => reference_behavior_name = Some(m),
                HkbAuxiliaryNodeInfoVisitor::SelfTransitionNames(m) => self_transition_names = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            _type: _type.unwrap_or_default().into_inner(),
            depth: depth.unwrap_or_default().into_inner(),
            reference_behavior_name: reference_behavior_name.unwrap_or_default().into_inner(),
            self_transition_names: self_transition_names.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbAuxiliaryNodeInfo<'a>> for Vec<HkbAuxiliaryNodeInfoVisitor<'a>> {
    fn from(data: &HkbAuxiliaryNodeInfo<'a>) -> Self {
        vec![
            HkbAuxiliaryNodeInfoVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbAuxiliaryNodeInfoVisitor::ReferenceCount(data.reference_count.into()),
            HkbAuxiliaryNodeInfoVisitor::Type(data._type.clone().into()),
            HkbAuxiliaryNodeInfoVisitor::Depth(data.depth.into()),
            HkbAuxiliaryNodeInfoVisitor::ReferenceBehaviorName(data.reference_behavior_name.clone().into()),
            HkbAuxiliaryNodeInfoVisitor::SelfTransitionNames(data.self_transition_names.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbAuxiliaryNodeInfo<'de> {
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
enum HkbAuxiliaryNodeInfoVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<NodeType>),
    /// Visitor fields
    #[serde(rename = "depth")]
    Depth(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "referenceBehaviorName")]
    ReferenceBehaviorName(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "selfTransitionNames")]
    SelfTransitionNames(HkArrayStringPtr<'a>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbAuxiliaryNodeInfoVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("type" => Type(Primitive<NodeType>)),
    ("depth" => Depth(Primitive<u8>)),
    ("referenceBehaviorName" => ReferenceBehaviorName(Primitive<Cow<'de, str>>)),
    ("selfTransitionNames" => SelfTransitionNames(HkArrayStringPtr<'de>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum NodeType {
    #[serde(rename = "NODE_TYPE_UNKNOWN")]
    #[default]
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
