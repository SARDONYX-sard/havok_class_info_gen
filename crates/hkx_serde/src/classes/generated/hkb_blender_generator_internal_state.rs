//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbBlenderGeneratorInternalState`
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

/// `hkbBlenderGeneratorInternalState`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x84717488`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbBlenderGeneratorInternalState {
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
    /// -   name:`"childrenInternalStates"`
    /// -   type: `hkArray<struct hkbBlenderGeneratorChildInternalState>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub children_internal_states: HkArrayClass<HkbBlenderGeneratorChildInternalState>,
    /// # C++ Class Fields Info
    /// -   name:`"sortedChildren"`
    /// -   type: `hkArray<hkInt16>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub sorted_children: HkArrayNum<i16>,
    /// # C++ Class Fields Info
    /// -   name:`"endIntervalWeight"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub end_interval_weight: f32,
    /// # C++ Class Fields Info
    /// -   name:`"numActiveChildren"`
    /// -   type: `hkInt32`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub num_active_children: i32,
    /// # C++ Class Fields Info
    /// -   name:`"beginIntervalIndex"`
    /// -   type: `hkInt16`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub begin_interval_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"endIntervalIndex"`
    /// -   type: `hkInt16`
    /// - offset: 42
    /// -  flags: `FLAGS_NONE`
    pub end_interval_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"initSync"`
    /// -   type: `hkBool`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub init_sync: bool,
    /// # C++ Class Fields Info
    /// -   name:`"doSubtractiveBlend"`
    /// -   type: `hkBool`
    /// - offset: 45
    /// -  flags: `FLAGS_NONE`
    pub do_subtractive_blend: bool,
}

impl Serialize for HkbBlenderGeneratorInternalState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbBlenderGeneratorInternalStateVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbBlenderGeneratorInternalState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbBlenderGeneratorInternalStateVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbBlenderGeneratorInternalStateVisitor>> for HkbBlenderGeneratorInternalState {
    fn from(_values: Vec<HkbBlenderGeneratorInternalStateVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut children_internal_states = None;
            let mut sorted_children = None;
            let mut end_interval_weight = None;
            let mut num_active_children = None;
            let mut begin_interval_index = None;
            let mut end_interval_index = None;
            let mut init_sync = None;
            let mut do_subtractive_blend = None;


        for _value in _values {
            match _value {
                HkbBlenderGeneratorInternalStateVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbBlenderGeneratorInternalStateVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbBlenderGeneratorInternalStateVisitor::ChildrenInternalStates(m) => children_internal_states = Some(m),
                HkbBlenderGeneratorInternalStateVisitor::SortedChildren(m) => sorted_children = Some(m),
                HkbBlenderGeneratorInternalStateVisitor::EndIntervalWeight(m) => end_interval_weight = Some(m),
                HkbBlenderGeneratorInternalStateVisitor::NumActiveChildren(m) => num_active_children = Some(m),
                HkbBlenderGeneratorInternalStateVisitor::BeginIntervalIndex(m) => begin_interval_index = Some(m),
                HkbBlenderGeneratorInternalStateVisitor::EndIntervalIndex(m) => end_interval_index = Some(m),
                HkbBlenderGeneratorInternalStateVisitor::InitSync(m) => init_sync = Some(m),
                HkbBlenderGeneratorInternalStateVisitor::DoSubtractiveBlend(m) => do_subtractive_blend = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            children_internal_states: children_internal_states.unwrap_or_default(),
            sorted_children: sorted_children.unwrap_or_default(),
            end_interval_weight: end_interval_weight.unwrap_or_default().into_inner(),
            num_active_children: num_active_children.unwrap_or_default().into_inner(),
            begin_interval_index: begin_interval_index.unwrap_or_default().into_inner(),
            end_interval_index: end_interval_index.unwrap_or_default().into_inner(),
            init_sync: init_sync.unwrap_or_default().into_inner(),
            do_subtractive_blend: do_subtractive_blend.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbBlenderGeneratorInternalState> for Vec<HkbBlenderGeneratorInternalStateVisitor> {
    fn from(data: &HkbBlenderGeneratorInternalState) -> Self {
        vec![
            HkbBlenderGeneratorInternalStateVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbBlenderGeneratorInternalStateVisitor::ReferenceCount(data.reference_count.into()),
            HkbBlenderGeneratorInternalStateVisitor::ChildrenInternalStates(data.children_internal_states.clone()),
            HkbBlenderGeneratorInternalStateVisitor::SortedChildren(data.sorted_children.clone()),
            HkbBlenderGeneratorInternalStateVisitor::EndIntervalWeight(data.end_interval_weight.into()),
            HkbBlenderGeneratorInternalStateVisitor::NumActiveChildren(data.num_active_children.into()),
            HkbBlenderGeneratorInternalStateVisitor::BeginIntervalIndex(data.begin_interval_index.into()),
            HkbBlenderGeneratorInternalStateVisitor::EndIntervalIndex(data.end_interval_index.into()),
            HkbBlenderGeneratorInternalStateVisitor::InitSync(data.init_sync.into()),
            HkbBlenderGeneratorInternalStateVisitor::DoSubtractiveBlend(data.do_subtractive_blend.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbBlenderGeneratorInternalState {
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
enum HkbBlenderGeneratorInternalStateVisitor {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "childrenInternalStates")]
    ChildrenInternalStates(HkArrayClass<HkbBlenderGeneratorChildInternalState>),
    /// Visitor fields
    #[serde(rename = "sortedChildren")]
    SortedChildren(HkArrayNum<i16>),
    /// Visitor fields
    #[serde(rename = "endIntervalWeight")]
    EndIntervalWeight(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "numActiveChildren")]
    NumActiveChildren(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "beginIntervalIndex")]
    BeginIntervalIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "endIntervalIndex")]
    EndIntervalIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "initSync")]
    InitSync(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "doSubtractiveBlend")]
    DoSubtractiveBlend(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbBlenderGeneratorInternalStateVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("childrenInternalStates" => ChildrenInternalStates(HkArrayClass<HkbBlenderGeneratorChildInternalState>)),
    ("sortedChildren" => SortedChildren(HkArrayNum<i16>)),
    ("endIntervalWeight" => EndIntervalWeight(Primitive<f32>)),
    ("numActiveChildren" => NumActiveChildren(Primitive<i32>)),
    ("beginIntervalIndex" => BeginIntervalIndex(Primitive<i16>)),
    ("endIntervalIndex" => EndIntervalIndex(Primitive<i16>)),
    ("initSync" => InitSync(Primitive<bool>)),
    ("doSubtractiveBlend" => DoSubtractiveBlend(Primitive<bool>)),
}
