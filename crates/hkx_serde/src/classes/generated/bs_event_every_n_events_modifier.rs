//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BSEventEveryNEventsModifier`
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

/// `BSEventEveryNEventsModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 72
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x6030970c`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BsEventEveryNEventsModifier<'a> {
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
    /// -   name:`"eventToCheckFor"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub event_to_check_for: SingleClass<HkbEventProperty<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"eventToSend"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub event_to_send: SingleClass<HkbEventProperty<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"numberOfEventsBeforeSend"`
    /// -   type: `hkInt8`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub number_of_events_before_send: i8,
    /// # C++ Class Fields Info
    /// -   name:`"minimumNumberOfEventsBeforeSend"`
    /// -   type: `hkInt8`
    /// - offset: 61
    /// -  flags: `FLAGS_NONE`
    pub minimum_number_of_events_before_send: i8,
    /// # C++ Class Fields Info
    /// -   name:`"randomizeNumberOfEvents"`
    /// -   type: `hkBool`
    /// - offset: 62
    /// -  flags: `FLAGS_NONE`
    pub randomize_number_of_events: bool,
    /// # C++ Class Fields Info
    /// -   name:`"numberOfEventsSeen"`
    /// -   type: `hkInt32`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub number_of_events_seen: i32,
    /// # C++ Class Fields Info
    /// -   name:`"calculatedNumberOfEventsBeforeSend"`
    /// -   type: `hkInt8`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub calculated_number_of_events_before_send: i8,
}

impl Serialize for BsEventEveryNEventsModifier<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<BsEventEveryNEventsModifierVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BsEventEveryNEventsModifier<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<BsEventEveryNEventsModifierVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<BsEventEveryNEventsModifierVisitor<'a>>> for BsEventEveryNEventsModifier<'a> {
    fn from(_values: Vec<BsEventEveryNEventsModifierVisitor<'a>>) -> Self {
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
            let mut event_to_check_for = None;
            let mut event_to_send = None;
            let mut number_of_events_before_send = None;
            let mut minimum_number_of_events_before_send = None;
            let mut randomize_number_of_events = None;
            let mut number_of_events_seen = None;
            let mut calculated_number_of_events_before_send = None;


        for _value in _values {
            match _value {
                BsEventEveryNEventsModifierVisitor::Enable(m) => enable = Some(m),
                BsEventEveryNEventsModifierVisitor::PadModifier(m) => pad_modifier = Some(m),
                BsEventEveryNEventsModifierVisitor::UserData(m) => user_data = Some(m),
                BsEventEveryNEventsModifierVisitor::Name(m) => name = Some(m),
                BsEventEveryNEventsModifierVisitor::Id(m) => id = Some(m),
                BsEventEveryNEventsModifierVisitor::CloneState(m) => clone_state = Some(m),
                BsEventEveryNEventsModifierVisitor::PadNode(m) => pad_node = Some(m),
                BsEventEveryNEventsModifierVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                BsEventEveryNEventsModifierVisitor::CachedBindables(m) => cached_bindables = Some(m),
                BsEventEveryNEventsModifierVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                BsEventEveryNEventsModifierVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                BsEventEveryNEventsModifierVisitor::ReferenceCount(m) => reference_count = Some(m),
                BsEventEveryNEventsModifierVisitor::EventToCheckFor(m) => event_to_check_for = Some(m),
                BsEventEveryNEventsModifierVisitor::EventToSend(m) => event_to_send = Some(m),
                BsEventEveryNEventsModifierVisitor::NumberOfEventsBeforeSend(m) => number_of_events_before_send = Some(m),
                BsEventEveryNEventsModifierVisitor::MinimumNumberOfEventsBeforeSend(m) => minimum_number_of_events_before_send = Some(m),
                BsEventEveryNEventsModifierVisitor::RandomizeNumberOfEvents(m) => randomize_number_of_events = Some(m),
                BsEventEveryNEventsModifierVisitor::NumberOfEventsSeen(m) => number_of_events_seen = Some(m),
                BsEventEveryNEventsModifierVisitor::CalculatedNumberOfEventsBeforeSend(m) => calculated_number_of_events_before_send = Some(m),

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
            event_to_check_for: event_to_check_for.unwrap_or_default(),
            event_to_send: event_to_send.unwrap_or_default(),
            number_of_events_before_send: number_of_events_before_send.unwrap_or_default().into_inner(),
            minimum_number_of_events_before_send: minimum_number_of_events_before_send.unwrap_or_default().into_inner(),
            randomize_number_of_events: randomize_number_of_events.unwrap_or_default().into_inner(),
            number_of_events_seen: number_of_events_seen.unwrap_or_default().into_inner(),
            calculated_number_of_events_before_send: calculated_number_of_events_before_send.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&BsEventEveryNEventsModifier<'a>> for Vec<BsEventEveryNEventsModifierVisitor<'a>> {
    fn from(data: &BsEventEveryNEventsModifier<'a>) -> Self {
        vec![
            BsEventEveryNEventsModifierVisitor::Enable(data.enable.into()),
            BsEventEveryNEventsModifierVisitor::PadModifier(data.pad_modifier.clone()),
            BsEventEveryNEventsModifierVisitor::UserData(data.user_data.into()),
            BsEventEveryNEventsModifierVisitor::Name(data.name.clone().into()),
            BsEventEveryNEventsModifierVisitor::Id(data.id.into()),
            BsEventEveryNEventsModifierVisitor::CloneState(data.clone_state.into()),
            BsEventEveryNEventsModifierVisitor::PadNode(data.pad_node.clone()),
            BsEventEveryNEventsModifierVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            BsEventEveryNEventsModifierVisitor::CachedBindables(data.cached_bindables.clone()),
            BsEventEveryNEventsModifierVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            BsEventEveryNEventsModifierVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            BsEventEveryNEventsModifierVisitor::ReferenceCount(data.reference_count.into()),
            BsEventEveryNEventsModifierVisitor::EventToCheckFor(data.event_to_check_for.clone()),
            BsEventEveryNEventsModifierVisitor::EventToSend(data.event_to_send.clone()),
            BsEventEveryNEventsModifierVisitor::NumberOfEventsBeforeSend(data.number_of_events_before_send.into()),
            BsEventEveryNEventsModifierVisitor::MinimumNumberOfEventsBeforeSend(data.minimum_number_of_events_before_send.into()),
            BsEventEveryNEventsModifierVisitor::RandomizeNumberOfEvents(data.randomize_number_of_events.into()),
            BsEventEveryNEventsModifierVisitor::NumberOfEventsSeen(data.number_of_events_seen.into()),
            BsEventEveryNEventsModifierVisitor::CalculatedNumberOfEventsBeforeSend(data.calculated_number_of_events_before_send.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for BsEventEveryNEventsModifier<'de> {
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
enum BsEventEveryNEventsModifierVisitor<'a> {
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
    #[serde(rename = "eventToCheckFor")]
    EventToCheckFor(SingleClass<HkbEventProperty<'a>>),
    /// Visitor fields
    #[serde(rename = "eventToSend")]
    EventToSend(SingleClass<HkbEventProperty<'a>>),
    /// Visitor fields
    #[serde(rename = "numberOfEventsBeforeSend")]
    NumberOfEventsBeforeSend(Primitive<i8>),
    /// Visitor fields
    #[serde(rename = "minimumNumberOfEventsBeforeSend")]
    MinimumNumberOfEventsBeforeSend(Primitive<i8>),
    /// Visitor fields
    #[serde(rename = "randomizeNumberOfEvents")]
    RandomizeNumberOfEvents(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "numberOfEventsSeen", skip_serializing)]
    NumberOfEventsSeen(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "calculatedNumberOfEventsBeforeSend", skip_serializing)]
    CalculatedNumberOfEventsBeforeSend(Primitive<i8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsEventEveryNEventsModifierVisitor<'de>, "@name",
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
    ("eventToCheckFor" => EventToCheckFor(SingleClass<HkbEventProperty<'de>>)),
    ("eventToSend" => EventToSend(SingleClass<HkbEventProperty<'de>>)),
    ("numberOfEventsBeforeSend" => NumberOfEventsBeforeSend(Primitive<i8>)),
    ("minimumNumberOfEventsBeforeSend" => MinimumNumberOfEventsBeforeSend(Primitive<i8>)),
    ("randomizeNumberOfEvents" => RandomizeNumberOfEvents(Primitive<bool>)),
    ("numberOfEventsSeen" => NumberOfEventsSeen(Primitive<i32>)),
    ("calculatedNumberOfEventsBeforeSend" => CalculatedNumberOfEventsBeforeSend(Primitive<i8>)),
}
