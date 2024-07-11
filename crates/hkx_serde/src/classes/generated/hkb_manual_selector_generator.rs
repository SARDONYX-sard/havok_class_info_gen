//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbManualSelectorGenerator`
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

/// `hkbManualSelectorGenerator`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 56
/// -    vtable: true
/// -    parent: `hkbGenerator`/`0xd68aefc`
/// - signature: `0xd932fab8`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbManualSelectorGenerator<'a> {
    // C++ Parent class(`hkbGenerator` => parent: `hkbNode`) has no fields
    //
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub user_data: usize,
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub name: Cow<'a, str>,
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"id"`
    /// -   type: `hkInt16`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub id: i16,
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"cloneState"`
    /// -   type: `enum unknown`
    /// - offset: 38
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub clone_state: (),
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"padNode"`
    /// -   type: `hkBool[1]`
    /// - offset: 39
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub pad_node: CStyleArray<[bool; 1]>,

    /// # C++ Parent class(`hkbBindable` => parent: `hkReferencedObject`) field Info
    /// -   name:`"variableBindingSet"`
    /// -   type: `struct hkbVariableBindingSet*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub variable_binding_set: Cow<'a, str>,
    /// # C++ Parent class(`hkbBindable` => parent: `hkReferencedObject`) field Info
    /// -   name:`"cachedBindables"`
    /// -   type: `hkArray<void>`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub cached_bindables: HkArrayRef<()>,
    /// # C++ Parent class(`hkbBindable` => parent: `hkReferencedObject`) field Info
    /// -   name:`"areBindablesCached"`
    /// -   type: `hkBool`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub are_bindables_cached: bool,

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
    /// -   name:`"generators"`
    /// -   type: `hkArray<hkbGenerator*>`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub generators: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"selectedGeneratorIndex"`
    /// -   type: `hkInt8`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub selected_generator_index: i8,
    /// # C++ Class Fields Info
    /// -   name:`"currentGeneratorIndex"`
    /// -   type: `hkInt8`
    /// - offset: 53
    /// -  flags: `FLAGS_NONE`
    pub current_generator_index: i8,
}

impl Serialize for HkbManualSelectorGenerator<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbManualSelectorGeneratorVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbManualSelectorGenerator<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbManualSelectorGeneratorVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbManualSelectorGeneratorVisitor<'a>>> for HkbManualSelectorGenerator<'a> {
    fn from(_values: Vec<HkbManualSelectorGeneratorVisitor<'a>>) -> Self {
            let mut user_data = None;
            let mut name = None;
            let mut id = None;
            let mut clone_state = None;
            let mut pad_node = None;
            let mut variable_binding_set = None;
            let mut cached_bindables = None;
            let mut are_bindables_cached = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut generators = None;
            let mut selected_generator_index = None;
            let mut current_generator_index = None;


        for _value in _values {
            match _value {
                HkbManualSelectorGeneratorVisitor::UserData(m) => user_data = Some(m),
                HkbManualSelectorGeneratorVisitor::Name(m) => name = Some(m),
                HkbManualSelectorGeneratorVisitor::Id(m) => id = Some(m),
                HkbManualSelectorGeneratorVisitor::CloneState(m) => clone_state = Some(m),
                HkbManualSelectorGeneratorVisitor::PadNode(m) => pad_node = Some(m),
                HkbManualSelectorGeneratorVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                HkbManualSelectorGeneratorVisitor::CachedBindables(m) => cached_bindables = Some(m),
                HkbManualSelectorGeneratorVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                HkbManualSelectorGeneratorVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbManualSelectorGeneratorVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbManualSelectorGeneratorVisitor::Generators(m) => generators = Some(m),
                HkbManualSelectorGeneratorVisitor::SelectedGeneratorIndex(m) => selected_generator_index = Some(m),
                HkbManualSelectorGeneratorVisitor::CurrentGeneratorIndex(m) => current_generator_index = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            user_data: user_data.unwrap_or_default().into_inner(),
            name: name.unwrap_or_default().into_inner(),
            id: id.unwrap_or_default().into_inner(),
            clone_state: clone_state.unwrap_or_default().into_inner(),
            pad_node: pad_node.unwrap_or_default(),
            variable_binding_set: variable_binding_set.unwrap_or_default().into_inner(),
            cached_bindables: cached_bindables.unwrap_or_default(),
            are_bindables_cached: are_bindables_cached.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            generators: generators.unwrap_or_default(),
            selected_generator_index: selected_generator_index.unwrap_or_default().into_inner(),
            current_generator_index: current_generator_index.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbManualSelectorGenerator<'a>> for Vec<HkbManualSelectorGeneratorVisitor<'a>> {
    fn from(data: &HkbManualSelectorGenerator<'a>) -> Self {
        vec![
            HkbManualSelectorGeneratorVisitor::UserData(data.user_data.into()),
            HkbManualSelectorGeneratorVisitor::Name(data.name.clone().into()),
            HkbManualSelectorGeneratorVisitor::Id(data.id.into()),
            HkbManualSelectorGeneratorVisitor::CloneState(data.clone_state.into()),
            HkbManualSelectorGeneratorVisitor::PadNode(data.pad_node.clone()),
            HkbManualSelectorGeneratorVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            HkbManualSelectorGeneratorVisitor::CachedBindables(data.cached_bindables.clone()),
            HkbManualSelectorGeneratorVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            HkbManualSelectorGeneratorVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbManualSelectorGeneratorVisitor::ReferenceCount(data.reference_count.into()),
            HkbManualSelectorGeneratorVisitor::Generators(data.generators.clone()),
            HkbManualSelectorGeneratorVisitor::SelectedGeneratorIndex(data.selected_generator_index.into()),
            HkbManualSelectorGeneratorVisitor::CurrentGeneratorIndex(data.current_generator_index.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbManualSelectorGenerator<'de> {
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
enum HkbManualSelectorGeneratorVisitor<'a> {
    // C++ Parent class(`hkbGenerator` => parent: `hkbNode`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// Visitor fields
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "id", skip_serializing)]
    Id(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "cloneState", skip_serializing)]
    CloneState(Primitive<()>),
    /// Visitor fields
    #[serde(rename = "padNode", skip_serializing)]
    PadNode(CStyleArray<[bool; 1]>),

    /// Visitor fields
    #[serde(rename = "variableBindingSet")]
    VariableBindingSet(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "cachedBindables", skip_serializing)]
    CachedBindables(HkArrayRef<()>),
    /// Visitor fields
    #[serde(rename = "areBindablesCached", skip_serializing)]
    AreBindablesCached(Primitive<bool>),

    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "generators")]
    Generators(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "selectedGeneratorIndex")]
    SelectedGeneratorIndex(Primitive<i8>),
    /// Visitor fields
    #[serde(rename = "currentGeneratorIndex")]
    CurrentGeneratorIndex(Primitive<i8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbManualSelectorGeneratorVisitor<'de>, "@name",
    ("userData" => UserData(Primitive<usize>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("id" => Id(Primitive<i16>)),
    ("cloneState" => CloneState(Primitive<()>)),
    ("padNode" => PadNode(CStyleArray<[bool; 1]>)),
    ("variableBindingSet" => VariableBindingSet(Primitive<Cow<'de, str>>)),
    ("cachedBindables" => CachedBindables(HkArrayRef<()>)),
    ("areBindablesCached" => AreBindablesCached(Primitive<bool>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("generators" => Generators(HkArrayRef<Cow<'de, str>>)),
    ("selectedGeneratorIndex" => SelectedGeneratorIndex(Primitive<i8>)),
    ("currentGeneratorIndex" => CurrentGeneratorIndex(Primitive<i8>)),
}
