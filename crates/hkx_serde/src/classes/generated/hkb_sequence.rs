//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbSequence`
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

/// `hkbSequence`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 168
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x43182ca3`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbSequence<'a> {
    /// # C++ Parent class(`hkbModifier` => parent: `hkbNode`) field Info
    /// -   name:`"enable"`
    /// -   type: `hkBool`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub enable: bool,
    /// # C++ Parent class(`hkbModifier` => parent: `hkbNode`) field Info
    /// -   name:`"padModifier"`
    /// -   type: `hkBool[3]`
    /// - offset: 41
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub pad_modifier: CStyleArray<[bool; 3]>,

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
    /// -   name:`"eventSequencedData"`
    /// -   type: `hkArray<hkbEventSequencedData*>`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub event_sequenced_data: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"realVariableSequencedData"`
    /// -   type: `hkArray<hkbRealVariableSequencedData*>`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub real_variable_sequenced_data: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"boolVariableSequencedData"`
    /// -   type: `hkArray<hkbBoolVariableSequencedData*>`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub bool_variable_sequenced_data: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"intVariableSequencedData"`
    /// -   type: `hkArray<hkbIntVariableSequencedData*>`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub int_variable_sequenced_data: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"enableEventId"`
    /// -   type: `hkInt32`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    pub enable_event_id: i32,
    /// # C++ Class Fields Info
    /// -   name:`"disableEventId"`
    /// -   type: `hkInt32`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    pub disable_event_id: i32,
    /// # C++ Class Fields Info
    /// -   name:`"stringData"`
    /// -   type: `struct hkbSequenceStringData*`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    pub string_data: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"variableIdMap"`
    /// -   type: `void*`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub variable_id_map: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"eventIdMap"`
    /// -   type: `void*`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub event_id_map: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"nextSampleEvents"`
    /// -   type: `hkArray<void>`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub next_sample_events: HkArrayRef<()>,
    /// # C++ Class Fields Info
    /// -   name:`"nextSampleReals"`
    /// -   type: `hkArray<void>`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub next_sample_reals: HkArrayRef<()>,
    /// # C++ Class Fields Info
    /// -   name:`"nextSampleBools"`
    /// -   type: `hkArray<void>`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub next_sample_bools: HkArrayRef<()>,
    /// # C++ Class Fields Info
    /// -   name:`"nextSampleInts"`
    /// -   type: `hkArray<void>`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub next_sample_ints: HkArrayRef<()>,
    /// # C++ Class Fields Info
    /// -   name:`"time"`
    /// -   type: `hkReal`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub time: f32,
    /// # C++ Class Fields Info
    /// -   name:`"isEnabled"`
    /// -   type: `hkBool`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub is_enabled: bool,
}

impl Serialize for HkbSequence<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbSequenceVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbSequence<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbSequenceVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbSequenceVisitor<'a>>> for HkbSequence<'a> {
    fn from(_values: Vec<HkbSequenceVisitor<'a>>) -> Self {
            let mut enable = None;
            let mut pad_modifier = None;
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
            let mut event_sequenced_data = None;
            let mut real_variable_sequenced_data = None;
            let mut bool_variable_sequenced_data = None;
            let mut int_variable_sequenced_data = None;
            let mut enable_event_id = None;
            let mut disable_event_id = None;
            let mut string_data = None;
            let mut variable_id_map = None;
            let mut event_id_map = None;
            let mut next_sample_events = None;
            let mut next_sample_reals = None;
            let mut next_sample_bools = None;
            let mut next_sample_ints = None;
            let mut time = None;
            let mut is_enabled = None;


        for _value in _values {
            match _value {
                HkbSequenceVisitor::Enable(m) => enable = Some(m),
                HkbSequenceVisitor::PadModifier(m) => pad_modifier = Some(m),
                HkbSequenceVisitor::UserData(m) => user_data = Some(m),
                HkbSequenceVisitor::Name(m) => name = Some(m),
                HkbSequenceVisitor::Id(m) => id = Some(m),
                HkbSequenceVisitor::CloneState(m) => clone_state = Some(m),
                HkbSequenceVisitor::PadNode(m) => pad_node = Some(m),
                HkbSequenceVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                HkbSequenceVisitor::CachedBindables(m) => cached_bindables = Some(m),
                HkbSequenceVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                HkbSequenceVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbSequenceVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbSequenceVisitor::EventSequencedData(m) => event_sequenced_data = Some(m),
                HkbSequenceVisitor::RealVariableSequencedData(m) => real_variable_sequenced_data = Some(m),
                HkbSequenceVisitor::BoolVariableSequencedData(m) => bool_variable_sequenced_data = Some(m),
                HkbSequenceVisitor::IntVariableSequencedData(m) => int_variable_sequenced_data = Some(m),
                HkbSequenceVisitor::EnableEventId(m) => enable_event_id = Some(m),
                HkbSequenceVisitor::DisableEventId(m) => disable_event_id = Some(m),
                HkbSequenceVisitor::StringData(m) => string_data = Some(m),
                HkbSequenceVisitor::VariableIdMap(m) => variable_id_map = Some(m),
                HkbSequenceVisitor::EventIdMap(m) => event_id_map = Some(m),
                HkbSequenceVisitor::NextSampleEvents(m) => next_sample_events = Some(m),
                HkbSequenceVisitor::NextSampleReals(m) => next_sample_reals = Some(m),
                HkbSequenceVisitor::NextSampleBools(m) => next_sample_bools = Some(m),
                HkbSequenceVisitor::NextSampleInts(m) => next_sample_ints = Some(m),
                HkbSequenceVisitor::Time(m) => time = Some(m),
                HkbSequenceVisitor::IsEnabled(m) => is_enabled = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            enable: enable.unwrap_or_default().into_inner(),
            pad_modifier: pad_modifier.unwrap_or_default(),
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
            event_sequenced_data: event_sequenced_data.unwrap_or_default(),
            real_variable_sequenced_data: real_variable_sequenced_data.unwrap_or_default(),
            bool_variable_sequenced_data: bool_variable_sequenced_data.unwrap_or_default(),
            int_variable_sequenced_data: int_variable_sequenced_data.unwrap_or_default(),
            enable_event_id: enable_event_id.unwrap_or_default().into_inner(),
            disable_event_id: disable_event_id.unwrap_or_default().into_inner(),
            string_data: string_data.unwrap_or_default().into_inner(),
            variable_id_map: variable_id_map.unwrap_or_default().into_inner(),
            event_id_map: event_id_map.unwrap_or_default().into_inner(),
            next_sample_events: next_sample_events.unwrap_or_default(),
            next_sample_reals: next_sample_reals.unwrap_or_default(),
            next_sample_bools: next_sample_bools.unwrap_or_default(),
            next_sample_ints: next_sample_ints.unwrap_or_default(),
            time: time.unwrap_or_default().into_inner(),
            is_enabled: is_enabled.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbSequence<'a>> for Vec<HkbSequenceVisitor<'a>> {
    fn from(data: &HkbSequence<'a>) -> Self {
        vec![
            HkbSequenceVisitor::Enable(data.enable.into()),
            HkbSequenceVisitor::PadModifier(data.pad_modifier.clone()),
            HkbSequenceVisitor::UserData(data.user_data.into()),
            HkbSequenceVisitor::Name(data.name.clone().into()),
            HkbSequenceVisitor::Id(data.id.into()),
            HkbSequenceVisitor::CloneState(data.clone_state.into()),
            HkbSequenceVisitor::PadNode(data.pad_node.clone()),
            HkbSequenceVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            HkbSequenceVisitor::CachedBindables(data.cached_bindables.clone()),
            HkbSequenceVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            HkbSequenceVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbSequenceVisitor::ReferenceCount(data.reference_count.into()),
            HkbSequenceVisitor::EventSequencedData(data.event_sequenced_data.clone()),
            HkbSequenceVisitor::RealVariableSequencedData(data.real_variable_sequenced_data.clone()),
            HkbSequenceVisitor::BoolVariableSequencedData(data.bool_variable_sequenced_data.clone()),
            HkbSequenceVisitor::IntVariableSequencedData(data.int_variable_sequenced_data.clone()),
            HkbSequenceVisitor::EnableEventId(data.enable_event_id.into()),
            HkbSequenceVisitor::DisableEventId(data.disable_event_id.into()),
            HkbSequenceVisitor::StringData(data.string_data.clone().into()),
            HkbSequenceVisitor::VariableIdMap(data.variable_id_map.clone().into()),
            HkbSequenceVisitor::EventIdMap(data.event_id_map.clone().into()),
            HkbSequenceVisitor::NextSampleEvents(data.next_sample_events.clone()),
            HkbSequenceVisitor::NextSampleReals(data.next_sample_reals.clone()),
            HkbSequenceVisitor::NextSampleBools(data.next_sample_bools.clone()),
            HkbSequenceVisitor::NextSampleInts(data.next_sample_ints.clone()),
            HkbSequenceVisitor::Time(data.time.into()),
            HkbSequenceVisitor::IsEnabled(data.is_enabled.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbSequence<'de> {
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
enum HkbSequenceVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "enable")]
    Enable(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "padModifier", skip_serializing)]
    PadModifier(CStyleArray<[bool; 3]>),

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
    #[serde(rename = "eventSequencedData")]
    EventSequencedData(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "realVariableSequencedData")]
    RealVariableSequencedData(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "boolVariableSequencedData")]
    BoolVariableSequencedData(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "intVariableSequencedData")]
    IntVariableSequencedData(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "enableEventId")]
    EnableEventId(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "disableEventId")]
    DisableEventId(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "stringData")]
    StringData(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "variableIdMap", skip_serializing)]
    VariableIdMap(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "eventIdMap", skip_serializing)]
    EventIdMap(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "nextSampleEvents", skip_serializing)]
    NextSampleEvents(HkArrayRef<()>),
    /// Visitor fields
    #[serde(rename = "nextSampleReals", skip_serializing)]
    NextSampleReals(HkArrayRef<()>),
    /// Visitor fields
    #[serde(rename = "nextSampleBools", skip_serializing)]
    NextSampleBools(HkArrayRef<()>),
    /// Visitor fields
    #[serde(rename = "nextSampleInts", skip_serializing)]
    NextSampleInts(HkArrayRef<()>),
    /// Visitor fields
    #[serde(rename = "time", skip_serializing)]
    Time(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "isEnabled", skip_serializing)]
    IsEnabled(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbSequenceVisitor<'de>, "@name",
    ("enable" => Enable(Primitive<bool>)),
    ("padModifier" => PadModifier(CStyleArray<[bool; 3]>)),
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
    ("eventSequencedData" => EventSequencedData(HkArrayRef<Cow<'de, str>>)),
    ("realVariableSequencedData" => RealVariableSequencedData(HkArrayRef<Cow<'de, str>>)),
    ("boolVariableSequencedData" => BoolVariableSequencedData(HkArrayRef<Cow<'de, str>>)),
    ("intVariableSequencedData" => IntVariableSequencedData(HkArrayRef<Cow<'de, str>>)),
    ("enableEventId" => EnableEventId(Primitive<i32>)),
    ("disableEventId" => DisableEventId(Primitive<i32>)),
    ("stringData" => StringData(Primitive<Cow<'de, str>>)),
    ("variableIdMap" => VariableIdMap(Primitive<Cow<'de, str>>)),
    ("eventIdMap" => EventIdMap(Primitive<Cow<'de, str>>)),
    ("nextSampleEvents" => NextSampleEvents(HkArrayRef<()>)),
    ("nextSampleReals" => NextSampleReals(HkArrayRef<()>)),
    ("nextSampleBools" => NextSampleBools(HkArrayRef<()>)),
    ("nextSampleInts" => NextSampleInts(HkArrayRef<()>)),
    ("time" => Time(Primitive<f32>)),
    ("isEnabled" => IsEnabled(Primitive<bool>)),
}
