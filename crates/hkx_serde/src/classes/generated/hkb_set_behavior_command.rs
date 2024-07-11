//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbSetBehaviorCommand`
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

/// `hkbSetBehaviorCommand`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xe18b74b9`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbSetBehaviorCommand<'a> {
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
    /// -   name:`"behavior"`
    /// -   type: `struct hkbBehaviorGraph*`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub behavior: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"rootGenerator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub root_generator: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"referencedBehaviors"`
    /// -   type: `hkArray<hkbBehaviorGraph*>`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub referenced_behaviors: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"startStateIndex"`
    /// -   type: `hkInt32`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub start_state_index: i32,
    /// # C++ Class Fields Info
    /// -   name:`"randomizeSimulation"`
    /// -   type: `hkBool`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub randomize_simulation: bool,
    /// # C++ Class Fields Info
    /// -   name:`"padding"`
    /// -   type: `hkInt32`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub padding: i32,
}

impl Serialize for HkbSetBehaviorCommand<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbSetBehaviorCommandVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbSetBehaviorCommand<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbSetBehaviorCommandVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbSetBehaviorCommandVisitor<'a>>> for HkbSetBehaviorCommand<'a> {
    fn from(_values: Vec<HkbSetBehaviorCommandVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut character_id = None;
            let mut behavior = None;
            let mut root_generator = None;
            let mut referenced_behaviors = None;
            let mut start_state_index = None;
            let mut randomize_simulation = None;
            let mut padding = None;


        for _value in _values {
            match _value {
                HkbSetBehaviorCommandVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbSetBehaviorCommandVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbSetBehaviorCommandVisitor::CharacterId(m) => character_id = Some(m),
                HkbSetBehaviorCommandVisitor::Behavior(m) => behavior = Some(m),
                HkbSetBehaviorCommandVisitor::RootGenerator(m) => root_generator = Some(m),
                HkbSetBehaviorCommandVisitor::ReferencedBehaviors(m) => referenced_behaviors = Some(m),
                HkbSetBehaviorCommandVisitor::StartStateIndex(m) => start_state_index = Some(m),
                HkbSetBehaviorCommandVisitor::RandomizeSimulation(m) => randomize_simulation = Some(m),
                HkbSetBehaviorCommandVisitor::Padding(m) => padding = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            character_id: character_id.unwrap_or_default().into_inner(),
            behavior: behavior.unwrap_or_default().into_inner(),
            root_generator: root_generator.unwrap_or_default().into_inner(),
            referenced_behaviors: referenced_behaviors.unwrap_or_default(),
            start_state_index: start_state_index.unwrap_or_default().into_inner(),
            randomize_simulation: randomize_simulation.unwrap_or_default().into_inner(),
            padding: padding.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbSetBehaviorCommand<'a>> for Vec<HkbSetBehaviorCommandVisitor<'a>> {
    fn from(data: &HkbSetBehaviorCommand<'a>) -> Self {
        vec![
            HkbSetBehaviorCommandVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbSetBehaviorCommandVisitor::ReferenceCount(data.reference_count.into()),
            HkbSetBehaviorCommandVisitor::CharacterId(data.character_id.into()),
            HkbSetBehaviorCommandVisitor::Behavior(data.behavior.clone().into()),
            HkbSetBehaviorCommandVisitor::RootGenerator(data.root_generator.clone().into()),
            HkbSetBehaviorCommandVisitor::ReferencedBehaviors(data.referenced_behaviors.clone()),
            HkbSetBehaviorCommandVisitor::StartStateIndex(data.start_state_index.into()),
            HkbSetBehaviorCommandVisitor::RandomizeSimulation(data.randomize_simulation.into()),
            HkbSetBehaviorCommandVisitor::Padding(data.padding.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbSetBehaviorCommand<'de> {
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
enum HkbSetBehaviorCommandVisitor<'a> {
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
    #[serde(rename = "behavior")]
    Behavior(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "rootGenerator")]
    RootGenerator(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "referencedBehaviors")]
    ReferencedBehaviors(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "startStateIndex")]
    StartStateIndex(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "randomizeSimulation")]
    RandomizeSimulation(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "padding")]
    Padding(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbSetBehaviorCommandVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("characterId" => CharacterId(Primitive<u64>)),
    ("behavior" => Behavior(Primitive<Cow<'de, str>>)),
    ("rootGenerator" => RootGenerator(Primitive<Cow<'de, str>>)),
    ("referencedBehaviors" => ReferencedBehaviors(HkArrayRef<Cow<'de, str>>)),
    ("startStateIndex" => StartStateIndex(Primitive<i32>)),
    ("randomizeSimulation" => RandomizeSimulation(Primitive<bool>)),
    ("padding" => Padding(Primitive<i32>)),
}
