//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbBehaviorGraphInternalStateInfo`
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

/// `hkbBehaviorGraphInternalStateInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 56
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x645f898b`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbBehaviorGraphInternalStateInfo<'a> {
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
    /// -   name:`"characterId"`
    /// -   type: `hkUint64`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub character_id: u64,
    /// # C++ Class Fields Info
    /// -   name:`"internalState"`
    /// -   type: `struct hkbBehaviorGraphInternalState*`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub internal_state: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"auxiliaryNodeInfo"`
    /// -   type: `hkArray<hkbAuxiliaryNodeInfo*>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub auxiliary_node_info: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"activeEventIds"`
    /// -   type: `hkArray<hkInt16>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub active_event_ids: HkArrayNum<i16>,
    /// # C++ Class Fields Info
    /// -   name:`"activeVariableIds"`
    /// -   type: `hkArray<hkInt16>`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub active_variable_ids: HkArrayNum<i16>,
}

impl Serialize for HkbBehaviorGraphInternalStateInfo<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbBehaviorGraphInternalStateInfoVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbBehaviorGraphInternalStateInfo<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbBehaviorGraphInternalStateInfoVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbBehaviorGraphInternalStateInfoVisitor<'a>>> for HkbBehaviorGraphInternalStateInfo<'a> {
    fn from(_values: Vec<HkbBehaviorGraphInternalStateInfoVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut character_id = None;
            let mut internal_state = None;
            let mut auxiliary_node_info = None;
            let mut active_event_ids = None;
            let mut active_variable_ids = None;


        for _value in _values {
            match _value {
                HkbBehaviorGraphInternalStateInfoVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbBehaviorGraphInternalStateInfoVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbBehaviorGraphInternalStateInfoVisitor::CharacterId(m) => character_id = Some(m),
                HkbBehaviorGraphInternalStateInfoVisitor::InternalState(m) => internal_state = Some(m),
                HkbBehaviorGraphInternalStateInfoVisitor::AuxiliaryNodeInfo(m) => auxiliary_node_info = Some(m),
                HkbBehaviorGraphInternalStateInfoVisitor::ActiveEventIds(m) => active_event_ids = Some(m),
                HkbBehaviorGraphInternalStateInfoVisitor::ActiveVariableIds(m) => active_variable_ids = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            character_id: character_id.unwrap_or_default().into_inner(),
            internal_state: internal_state.unwrap_or_default().into_inner(),
            auxiliary_node_info: auxiliary_node_info.unwrap_or_default(),
            active_event_ids: active_event_ids.unwrap_or_default(),
            active_variable_ids: active_variable_ids.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbBehaviorGraphInternalStateInfo<'a>> for Vec<HkbBehaviorGraphInternalStateInfoVisitor<'a>> {
    fn from(data: &HkbBehaviorGraphInternalStateInfo<'a>) -> Self {
        vec![
            HkbBehaviorGraphInternalStateInfoVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbBehaviorGraphInternalStateInfoVisitor::ReferenceCount(data.reference_count.into()),
            HkbBehaviorGraphInternalStateInfoVisitor::CharacterId(data.character_id.into()),
            HkbBehaviorGraphInternalStateInfoVisitor::InternalState(data.internal_state.clone().into()),
            HkbBehaviorGraphInternalStateInfoVisitor::AuxiliaryNodeInfo(data.auxiliary_node_info.clone()),
            HkbBehaviorGraphInternalStateInfoVisitor::ActiveEventIds(data.active_event_ids.clone()),
            HkbBehaviorGraphInternalStateInfoVisitor::ActiveVariableIds(data.active_variable_ids.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbBehaviorGraphInternalStateInfo<'de> {
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
enum HkbBehaviorGraphInternalStateInfoVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "characterId")]
    CharacterId(Primitive<u64>),
    /// Visitor fields
    #[serde(rename = "internalState")]
    InternalState(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "auxiliaryNodeInfo")]
    AuxiliaryNodeInfo(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "activeEventIds")]
    ActiveEventIds(HkArrayNum<i16>),
    /// Visitor fields
    #[serde(rename = "activeVariableIds")]
    ActiveVariableIds(HkArrayNum<i16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbBehaviorGraphInternalStateInfoVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("characterId" => CharacterId(Primitive<u64>)),
    ("internalState" => InternalState(Primitive<Cow<'de, str>>)),
    ("auxiliaryNodeInfo" => AuxiliaryNodeInfo(HkArrayRef<Cow<'de, str>>)),
    ("activeEventIds" => ActiveEventIds(HkArrayNum<i16>)),
    ("activeVariableIds" => ActiveVariableIds(HkArrayNum<i16>)),
}
