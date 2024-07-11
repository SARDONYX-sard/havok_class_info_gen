//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbTransitionEffect`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#![allow(clippy::clone_on_copy, clippy::unit_arg)]

#[allow(unused)]
use super::*;
#[allow(unused)]
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkbTransitionEffect`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 44
/// -    vtable: true
/// -    parent: `hkbGenerator`/`0xd68aefc`
/// - signature: `0x945da157`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbTransitionEffect<'a> {
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
    /// -   name:`"selfTransitionMode"`
    /// -   type: `enum SelfTransitionMode`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub self_transition_mode: SelfTransitionMode,
    /// # C++ Class Fields Info
    /// -   name:`"eventMode"`
    /// -   type: `enum EventMode`
    /// - offset: 41
    /// -  flags: `FLAGS_NONE`
    pub event_mode: EventMode,
    /// # C++ Class Fields Info
    /// -   name:`"defaultEventMode"`
    /// -   type: `enum unknown`
    /// - offset: 42
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub default_event_mode: (),
}

impl Serialize for HkbTransitionEffect<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbTransitionEffectVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbTransitionEffect<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbTransitionEffectVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbTransitionEffectVisitor<'a>>> for HkbTransitionEffect<'a> {
    fn from(_values: Vec<HkbTransitionEffectVisitor<'a>>) -> Self {
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
        let mut self_transition_mode = None;
        let mut event_mode = None;
        let mut default_event_mode = None;

        for _value in _values {
            match _value {
                HkbTransitionEffectVisitor::UserData(m) => user_data = Some(m),
                HkbTransitionEffectVisitor::Name(m) => name = Some(m),
                HkbTransitionEffectVisitor::Id(m) => id = Some(m),
                HkbTransitionEffectVisitor::CloneState(m) => clone_state = Some(m),
                HkbTransitionEffectVisitor::PadNode(m) => pad_node = Some(m),
                HkbTransitionEffectVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                HkbTransitionEffectVisitor::CachedBindables(m) => cached_bindables = Some(m),
                HkbTransitionEffectVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                HkbTransitionEffectVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbTransitionEffectVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbTransitionEffectVisitor::SelfTransitionMode(m) => self_transition_mode = Some(m),
                HkbTransitionEffectVisitor::EventMode(m) => event_mode = Some(m),
                HkbTransitionEffectVisitor::DefaultEventMode(m) => default_event_mode = Some(m),
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
            self_transition_mode: self_transition_mode.unwrap_or_default().into_inner(),
            event_mode: event_mode.unwrap_or_default().into_inner(),
            default_event_mode: default_event_mode.unwrap_or_default().into_inner(),
        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbTransitionEffect<'a>> for Vec<HkbTransitionEffectVisitor<'a>> {
    fn from(data: &HkbTransitionEffect<'a>) -> Self {
        vec![
            HkbTransitionEffectVisitor::UserData(data.user_data.into()),
            HkbTransitionEffectVisitor::Name(data.name.clone().into()),
            HkbTransitionEffectVisitor::Id(data.id.into()),
            HkbTransitionEffectVisitor::CloneState(data.clone_state.into()),
            HkbTransitionEffectVisitor::PadNode(data.pad_node.clone()),
            HkbTransitionEffectVisitor::VariableBindingSet(
                data.variable_binding_set.clone().into(),
            ),
            HkbTransitionEffectVisitor::CachedBindables(data.cached_bindables.clone()),
            HkbTransitionEffectVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            HkbTransitionEffectVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbTransitionEffectVisitor::ReferenceCount(data.reference_count.into()),
            HkbTransitionEffectVisitor::SelfTransitionMode(
                data.self_transition_mode.clone().into(),
            ),
            HkbTransitionEffectVisitor::EventMode(data.event_mode.clone().into()),
            HkbTransitionEffectVisitor::DefaultEventMode(data.default_event_mode.into()),
        ]
    }
}

impl<'de> ByteDeSerialize<'de> for HkbTransitionEffect<'de> {
    fn from_bytes<D>(deserializer: &D, position: &mut u32) -> Result<Self>
    where
        D: ByteDeserializer,
        Self: Sized + 'de,
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
enum HkbTransitionEffectVisitor<'a> {
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
    #[serde(rename = "selfTransitionMode")]
    SelfTransitionMode(Primitive<SelfTransitionMode>),
    /// Visitor fields
    #[serde(rename = "eventMode")]
    EventMode(Primitive<EventMode>),
    /// Visitor fields
    #[serde(rename = "defaultEventMode", skip_serializing)]
    DefaultEventMode(Primitive<()>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbTransitionEffectVisitor<'de>, "@name",
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
    ("selfTransitionMode" => SelfTransitionMode(Primitive<SelfTransitionMode>)),
    ("eventMode" => EventMode(Primitive<EventMode>)),
    ("defaultEventMode" => DefaultEventMode(Primitive<()>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum SelfTransitionMode {
    #[serde(rename = "SELF_TRANSITION_MODE_CONTINUE_IF_CYCLIC_BLEND_IF_ACYCLIC")]
    #[default]
    SelfTransitionModeContinueIfCyclicBlendIfAcyclic = 0,
    #[serde(rename = "SELF_TRANSITION_MODE_CONTINUE")]
    SelfTransitionModeContinue = 1,
    #[serde(rename = "SELF_TRANSITION_MODE_RESET")]
    SelfTransitionModeReset = 2,
    #[serde(rename = "SELF_TRANSITION_MODE_BLEND")]
    SelfTransitionModeBlend = 3,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum EventMode {
    #[serde(rename = "EVENT_MODE_DEFAULT")]
    #[default]
    EventModeDefault = 0,
    #[serde(rename = "EVENT_MODE_PROCESS_ALL")]
    EventModeProcessAll = 1,
    #[serde(rename = "EVENT_MODE_IGNORE_FROM_GENERATOR")]
    EventModeIgnoreFromGenerator = 2,
    #[serde(rename = "EVENT_MODE_IGNORE_TO_GENERATOR")]
    EventModeIgnoreToGenerator = 3,
}
