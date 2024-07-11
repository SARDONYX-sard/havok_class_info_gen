//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BSEventOnFalseToTrueModifier`
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

/// `BSEventOnFalseToTrueModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 84
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x81d0777a`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BsEventOnFalseToTrueModifier<'a> {
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
    /// -   name:`"bEnableEvent1"`
    /// -   type: `hkBool`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub b_enable_event_1: bool,
    /// # C++ Class Fields Info
    /// -   name:`"bVariableToTest1"`
    /// -   type: `hkBool`
    /// - offset: 45
    /// -  flags: `FLAGS_NONE`
    pub b_variable_to_test_1: bool,
    /// # C++ Class Fields Info
    /// -   name:`"EventToSend1"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub event_to_send_1: SingleClass<HkbEventProperty<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"bEnableEvent2"`
    /// -   type: `hkBool`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub b_enable_event_2: bool,
    /// # C++ Class Fields Info
    /// -   name:`"bVariableToTest2"`
    /// -   type: `hkBool`
    /// - offset: 57
    /// -  flags: `FLAGS_NONE`
    pub b_variable_to_test_2: bool,
    /// # C++ Class Fields Info
    /// -   name:`"EventToSend2"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub event_to_send_2: SingleClass<HkbEventProperty<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"bEnableEvent3"`
    /// -   type: `hkBool`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub b_enable_event_3: bool,
    /// # C++ Class Fields Info
    /// -   name:`"bVariableToTest3"`
    /// -   type: `hkBool`
    /// - offset: 69
    /// -  flags: `FLAGS_NONE`
    pub b_variable_to_test_3: bool,
    /// # C++ Class Fields Info
    /// -   name:`"EventToSend3"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    pub event_to_send_3: SingleClass<HkbEventProperty<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"bSlot1ActivatedLastFrame"`
    /// -   type: `hkBool`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub b_slot_1_activated_last_frame: bool,
    /// # C++ Class Fields Info
    /// -   name:`"bSlot2ActivatedLastFrame"`
    /// -   type: `hkBool`
    /// - offset: 81
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub b_slot_2_activated_last_frame: bool,
    /// # C++ Class Fields Info
    /// -   name:`"bSlot3ActivatedLastFrame"`
    /// -   type: `hkBool`
    /// - offset: 82
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub b_slot_3_activated_last_frame: bool,
}

impl Serialize for BsEventOnFalseToTrueModifier<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<BsEventOnFalseToTrueModifierVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BsEventOnFalseToTrueModifier<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<BsEventOnFalseToTrueModifierVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<BsEventOnFalseToTrueModifierVisitor<'a>>> for BsEventOnFalseToTrueModifier<'a> {
    fn from(_values: Vec<BsEventOnFalseToTrueModifierVisitor<'a>>) -> Self {
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
            let mut b_enable_event_1 = None;
            let mut b_variable_to_test_1 = None;
            let mut event_to_send_1 = None;
            let mut b_enable_event_2 = None;
            let mut b_variable_to_test_2 = None;
            let mut event_to_send_2 = None;
            let mut b_enable_event_3 = None;
            let mut b_variable_to_test_3 = None;
            let mut event_to_send_3 = None;
            let mut b_slot_1_activated_last_frame = None;
            let mut b_slot_2_activated_last_frame = None;
            let mut b_slot_3_activated_last_frame = None;


        for _value in _values {
            match _value {
                BsEventOnFalseToTrueModifierVisitor::Enable(m) => enable = Some(m),
                BsEventOnFalseToTrueModifierVisitor::PadModifier(m) => pad_modifier = Some(m),
                BsEventOnFalseToTrueModifierVisitor::UserData(m) => user_data = Some(m),
                BsEventOnFalseToTrueModifierVisitor::Name(m) => name = Some(m),
                BsEventOnFalseToTrueModifierVisitor::Id(m) => id = Some(m),
                BsEventOnFalseToTrueModifierVisitor::CloneState(m) => clone_state = Some(m),
                BsEventOnFalseToTrueModifierVisitor::PadNode(m) => pad_node = Some(m),
                BsEventOnFalseToTrueModifierVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                BsEventOnFalseToTrueModifierVisitor::CachedBindables(m) => cached_bindables = Some(m),
                BsEventOnFalseToTrueModifierVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                BsEventOnFalseToTrueModifierVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                BsEventOnFalseToTrueModifierVisitor::ReferenceCount(m) => reference_count = Some(m),
                BsEventOnFalseToTrueModifierVisitor::BEnableEvent1(m) => b_enable_event_1 = Some(m),
                BsEventOnFalseToTrueModifierVisitor::BVariableToTest1(m) => b_variable_to_test_1 = Some(m),
                BsEventOnFalseToTrueModifierVisitor::EventToSend1(m) => event_to_send_1 = Some(m),
                BsEventOnFalseToTrueModifierVisitor::BEnableEvent2(m) => b_enable_event_2 = Some(m),
                BsEventOnFalseToTrueModifierVisitor::BVariableToTest2(m) => b_variable_to_test_2 = Some(m),
                BsEventOnFalseToTrueModifierVisitor::EventToSend2(m) => event_to_send_2 = Some(m),
                BsEventOnFalseToTrueModifierVisitor::BEnableEvent3(m) => b_enable_event_3 = Some(m),
                BsEventOnFalseToTrueModifierVisitor::BVariableToTest3(m) => b_variable_to_test_3 = Some(m),
                BsEventOnFalseToTrueModifierVisitor::EventToSend3(m) => event_to_send_3 = Some(m),
                BsEventOnFalseToTrueModifierVisitor::BSlot1ActivatedLastFrame(m) => b_slot_1_activated_last_frame = Some(m),
                BsEventOnFalseToTrueModifierVisitor::BSlot2ActivatedLastFrame(m) => b_slot_2_activated_last_frame = Some(m),
                BsEventOnFalseToTrueModifierVisitor::BSlot3ActivatedLastFrame(m) => b_slot_3_activated_last_frame = Some(m),

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
            b_enable_event_1: b_enable_event_1.unwrap_or_default().into_inner(),
            b_variable_to_test_1: b_variable_to_test_1.unwrap_or_default().into_inner(),
            event_to_send_1: event_to_send_1.unwrap_or_default(),
            b_enable_event_2: b_enable_event_2.unwrap_or_default().into_inner(),
            b_variable_to_test_2: b_variable_to_test_2.unwrap_or_default().into_inner(),
            event_to_send_2: event_to_send_2.unwrap_or_default(),
            b_enable_event_3: b_enable_event_3.unwrap_or_default().into_inner(),
            b_variable_to_test_3: b_variable_to_test_3.unwrap_or_default().into_inner(),
            event_to_send_3: event_to_send_3.unwrap_or_default(),
            b_slot_1_activated_last_frame: b_slot_1_activated_last_frame.unwrap_or_default().into_inner(),
            b_slot_2_activated_last_frame: b_slot_2_activated_last_frame.unwrap_or_default().into_inner(),
            b_slot_3_activated_last_frame: b_slot_3_activated_last_frame.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&BsEventOnFalseToTrueModifier<'a>> for Vec<BsEventOnFalseToTrueModifierVisitor<'a>> {
    fn from(data: &BsEventOnFalseToTrueModifier<'a>) -> Self {
        vec![
            BsEventOnFalseToTrueModifierVisitor::Enable(data.enable.into()),
            BsEventOnFalseToTrueModifierVisitor::PadModifier(data.pad_modifier.clone()),
            BsEventOnFalseToTrueModifierVisitor::UserData(data.user_data.into()),
            BsEventOnFalseToTrueModifierVisitor::Name(data.name.clone().into()),
            BsEventOnFalseToTrueModifierVisitor::Id(data.id.into()),
            BsEventOnFalseToTrueModifierVisitor::CloneState(data.clone_state.into()),
            BsEventOnFalseToTrueModifierVisitor::PadNode(data.pad_node.clone()),
            BsEventOnFalseToTrueModifierVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            BsEventOnFalseToTrueModifierVisitor::CachedBindables(data.cached_bindables.clone()),
            BsEventOnFalseToTrueModifierVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            BsEventOnFalseToTrueModifierVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            BsEventOnFalseToTrueModifierVisitor::ReferenceCount(data.reference_count.into()),
            BsEventOnFalseToTrueModifierVisitor::BEnableEvent1(data.b_enable_event_1.into()),
            BsEventOnFalseToTrueModifierVisitor::BVariableToTest1(data.b_variable_to_test_1.into()),
            BsEventOnFalseToTrueModifierVisitor::EventToSend1(data.event_to_send_1.clone()),
            BsEventOnFalseToTrueModifierVisitor::BEnableEvent2(data.b_enable_event_2.into()),
            BsEventOnFalseToTrueModifierVisitor::BVariableToTest2(data.b_variable_to_test_2.into()),
            BsEventOnFalseToTrueModifierVisitor::EventToSend2(data.event_to_send_2.clone()),
            BsEventOnFalseToTrueModifierVisitor::BEnableEvent3(data.b_enable_event_3.into()),
            BsEventOnFalseToTrueModifierVisitor::BVariableToTest3(data.b_variable_to_test_3.into()),
            BsEventOnFalseToTrueModifierVisitor::EventToSend3(data.event_to_send_3.clone()),
            BsEventOnFalseToTrueModifierVisitor::BSlot1ActivatedLastFrame(data.b_slot_1_activated_last_frame.into()),
            BsEventOnFalseToTrueModifierVisitor::BSlot2ActivatedLastFrame(data.b_slot_2_activated_last_frame.into()),
            BsEventOnFalseToTrueModifierVisitor::BSlot3ActivatedLastFrame(data.b_slot_3_activated_last_frame.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for BsEventOnFalseToTrueModifier<'de> {
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
enum BsEventOnFalseToTrueModifierVisitor<'a> {
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
    #[serde(rename = "bEnableEvent1")]
    BEnableEvent1(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "bVariableToTest1")]
    BVariableToTest1(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "EventToSend1")]
    EventToSend1(SingleClass<HkbEventProperty<'a>>),
    /// Visitor fields
    #[serde(rename = "bEnableEvent2")]
    BEnableEvent2(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "bVariableToTest2")]
    BVariableToTest2(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "EventToSend2")]
    EventToSend2(SingleClass<HkbEventProperty<'a>>),
    /// Visitor fields
    #[serde(rename = "bEnableEvent3")]
    BEnableEvent3(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "bVariableToTest3")]
    BVariableToTest3(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "EventToSend3")]
    EventToSend3(SingleClass<HkbEventProperty<'a>>),
    /// Visitor fields
    #[serde(rename = "bSlot1ActivatedLastFrame", skip_serializing)]
    BSlot1ActivatedLastFrame(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "bSlot2ActivatedLastFrame", skip_serializing)]
    BSlot2ActivatedLastFrame(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "bSlot3ActivatedLastFrame", skip_serializing)]
    BSlot3ActivatedLastFrame(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsEventOnFalseToTrueModifierVisitor<'de>, "@name",
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
    ("bEnableEvent1" => BEnableEvent1(Primitive<bool>)),
    ("bVariableToTest1" => BVariableToTest1(Primitive<bool>)),
    ("EventToSend1" => EventToSend1(SingleClass<HkbEventProperty<'de>>)),
    ("bEnableEvent2" => BEnableEvent2(Primitive<bool>)),
    ("bVariableToTest2" => BVariableToTest2(Primitive<bool>)),
    ("EventToSend2" => EventToSend2(SingleClass<HkbEventProperty<'de>>)),
    ("bEnableEvent3" => BEnableEvent3(Primitive<bool>)),
    ("bVariableToTest3" => BVariableToTest3(Primitive<bool>)),
    ("EventToSend3" => EventToSend3(SingleClass<HkbEventProperty<'de>>)),
    ("bSlot1ActivatedLastFrame" => BSlot1ActivatedLastFrame(Primitive<bool>)),
    ("bSlot2ActivatedLastFrame" => BSlot2ActivatedLastFrame(Primitive<bool>)),
    ("bSlot3ActivatedLastFrame" => BSlot3ActivatedLastFrame(Primitive<bool>)),
}
