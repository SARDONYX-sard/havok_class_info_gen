//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BSiStateTaggingGenerator`
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

/// `BSiStateTaggingGenerator`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkbGenerator`/`0xd68aefc`
/// - signature: `0xf0826fc1`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BSiStateTaggingGenerator<'a> {
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
    /// -   name:`"pDefaultGenerator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE|ALIGN16`
    pub p_default_generator: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"iStateToSetAs"`
    /// -   type: `hkInt32`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub i_state_to_set_as: i32,
    /// # C++ Class Fields Info
    /// -   name:`"iPriority"`
    /// -   type: `hkInt32`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub i_priority: i32,
}

impl Serialize for BSiStateTaggingGenerator<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<BSiStateTaggingGeneratorVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BSiStateTaggingGenerator<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<BSiStateTaggingGeneratorVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<BSiStateTaggingGeneratorVisitor<'a>>> for BSiStateTaggingGenerator<'a> {
    fn from(_values: Vec<BSiStateTaggingGeneratorVisitor<'a>>) -> Self {
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
            let mut p_default_generator = None;
            let mut i_state_to_set_as = None;
            let mut i_priority = None;


        for _value in _values {
            match _value {
                BSiStateTaggingGeneratorVisitor::UserData(m) => user_data = Some(m),
                BSiStateTaggingGeneratorVisitor::Name(m) => name = Some(m),
                BSiStateTaggingGeneratorVisitor::Id(m) => id = Some(m),
                BSiStateTaggingGeneratorVisitor::CloneState(m) => clone_state = Some(m),
                BSiStateTaggingGeneratorVisitor::PadNode(m) => pad_node = Some(m),
                BSiStateTaggingGeneratorVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                BSiStateTaggingGeneratorVisitor::CachedBindables(m) => cached_bindables = Some(m),
                BSiStateTaggingGeneratorVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                BSiStateTaggingGeneratorVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                BSiStateTaggingGeneratorVisitor::ReferenceCount(m) => reference_count = Some(m),
                BSiStateTaggingGeneratorVisitor::PDefaultGenerator(m) => p_default_generator = Some(m),
                BSiStateTaggingGeneratorVisitor::IStateToSetAs(m) => i_state_to_set_as = Some(m),
                BSiStateTaggingGeneratorVisitor::IPriority(m) => i_priority = Some(m),

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
            p_default_generator: p_default_generator.unwrap_or_default().into_inner(),
            i_state_to_set_as: i_state_to_set_as.unwrap_or_default().into_inner(),
            i_priority: i_priority.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&BSiStateTaggingGenerator<'a>> for Vec<BSiStateTaggingGeneratorVisitor<'a>> {
    fn from(data: &BSiStateTaggingGenerator<'a>) -> Self {
        vec![
            BSiStateTaggingGeneratorVisitor::UserData(data.user_data.into()),
            BSiStateTaggingGeneratorVisitor::Name(data.name.clone().into()),
            BSiStateTaggingGeneratorVisitor::Id(data.id.into()),
            BSiStateTaggingGeneratorVisitor::CloneState(data.clone_state.into()),
            BSiStateTaggingGeneratorVisitor::PadNode(data.pad_node.clone()),
            BSiStateTaggingGeneratorVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            BSiStateTaggingGeneratorVisitor::CachedBindables(data.cached_bindables.clone()),
            BSiStateTaggingGeneratorVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            BSiStateTaggingGeneratorVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            BSiStateTaggingGeneratorVisitor::ReferenceCount(data.reference_count.into()),
            BSiStateTaggingGeneratorVisitor::PDefaultGenerator(data.p_default_generator.clone().into()),
            BSiStateTaggingGeneratorVisitor::IStateToSetAs(data.i_state_to_set_as.into()),
            BSiStateTaggingGeneratorVisitor::IPriority(data.i_priority.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for BSiStateTaggingGenerator<'de> {
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
enum BSiStateTaggingGeneratorVisitor<'a> {
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
    #[serde(rename = "pDefaultGenerator")]
    PDefaultGenerator(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "iStateToSetAs")]
    IStateToSetAs(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "iPriority")]
    IPriority(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BSiStateTaggingGeneratorVisitor<'de>, "@name",
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
    ("pDefaultGenerator" => PDefaultGenerator(Primitive<Cow<'de, str>>)),
    ("iStateToSetAs" => IStateToSetAs(Primitive<i32>)),
    ("iPriority" => IPriority(Primitive<i32>)),
}
