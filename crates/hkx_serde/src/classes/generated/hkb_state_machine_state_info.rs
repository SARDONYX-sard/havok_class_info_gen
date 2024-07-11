//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbStateMachineStateInfo`
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

/// `hkbStateMachineStateInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 72
/// -    vtable: true
/// -    parent: `hkbBindable`/`0x2c1432d7`
/// - signature: `0xed7f9d0`
/// -   version: 4
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbStateMachineStateInfo<'a> {
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
    /// -   name:`"listeners"`
    /// -   type: `hkArray<hkbStateListener*>`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub listeners: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"enterNotifyEvents"`
    /// -   type: `struct hkbStateMachineEventPropertyArray*`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub enter_notify_events: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"exitNotifyEvents"`
    /// -   type: `struct hkbStateMachineEventPropertyArray*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub exit_notify_events: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"transitions"`
    /// -   type: `struct hkbStateMachineTransitionInfoArray*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub transitions: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"generator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub generator: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"stateId"`
    /// -   type: `hkInt32`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub state_id: i32,
    /// # C++ Class Fields Info
    /// -   name:`"probability"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub probability: f32,
    /// # C++ Class Fields Info
    /// -   name:`"enable"`
    /// -   type: `hkBool`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub enable: bool,
}

impl Serialize for HkbStateMachineStateInfo<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbStateMachineStateInfoVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbStateMachineStateInfo<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbStateMachineStateInfoVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbStateMachineStateInfoVisitor<'a>>> for HkbStateMachineStateInfo<'a> {
    fn from(_values: Vec<HkbStateMachineStateInfoVisitor<'a>>) -> Self {
            let mut variable_binding_set = None;
            let mut cached_bindables = None;
            let mut are_bindables_cached = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut listeners = None;
            let mut enter_notify_events = None;
            let mut exit_notify_events = None;
            let mut transitions = None;
            let mut generator = None;
            let mut name = None;
            let mut state_id = None;
            let mut probability = None;
            let mut enable = None;


        for _value in _values {
            match _value {
                HkbStateMachineStateInfoVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                HkbStateMachineStateInfoVisitor::CachedBindables(m) => cached_bindables = Some(m),
                HkbStateMachineStateInfoVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                HkbStateMachineStateInfoVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbStateMachineStateInfoVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbStateMachineStateInfoVisitor::Listeners(m) => listeners = Some(m),
                HkbStateMachineStateInfoVisitor::EnterNotifyEvents(m) => enter_notify_events = Some(m),
                HkbStateMachineStateInfoVisitor::ExitNotifyEvents(m) => exit_notify_events = Some(m),
                HkbStateMachineStateInfoVisitor::Transitions(m) => transitions = Some(m),
                HkbStateMachineStateInfoVisitor::Generator(m) => generator = Some(m),
                HkbStateMachineStateInfoVisitor::Name(m) => name = Some(m),
                HkbStateMachineStateInfoVisitor::StateId(m) => state_id = Some(m),
                HkbStateMachineStateInfoVisitor::Probability(m) => probability = Some(m),
                HkbStateMachineStateInfoVisitor::Enable(m) => enable = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            variable_binding_set: variable_binding_set.unwrap_or_default().into_inner(),
            cached_bindables: cached_bindables.unwrap_or_default(),
            are_bindables_cached: are_bindables_cached.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            listeners: listeners.unwrap_or_default(),
            enter_notify_events: enter_notify_events.unwrap_or_default().into_inner(),
            exit_notify_events: exit_notify_events.unwrap_or_default().into_inner(),
            transitions: transitions.unwrap_or_default().into_inner(),
            generator: generator.unwrap_or_default().into_inner(),
            name: name.unwrap_or_default().into_inner(),
            state_id: state_id.unwrap_or_default().into_inner(),
            probability: probability.unwrap_or_default().into_inner(),
            enable: enable.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbStateMachineStateInfo<'a>> for Vec<HkbStateMachineStateInfoVisitor<'a>> {
    fn from(data: &HkbStateMachineStateInfo<'a>) -> Self {
        vec![
            HkbStateMachineStateInfoVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            HkbStateMachineStateInfoVisitor::CachedBindables(data.cached_bindables.clone()),
            HkbStateMachineStateInfoVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            HkbStateMachineStateInfoVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbStateMachineStateInfoVisitor::ReferenceCount(data.reference_count.into()),
            HkbStateMachineStateInfoVisitor::Listeners(data.listeners.clone()),
            HkbStateMachineStateInfoVisitor::EnterNotifyEvents(data.enter_notify_events.clone().into()),
            HkbStateMachineStateInfoVisitor::ExitNotifyEvents(data.exit_notify_events.clone().into()),
            HkbStateMachineStateInfoVisitor::Transitions(data.transitions.clone().into()),
            HkbStateMachineStateInfoVisitor::Generator(data.generator.clone().into()),
            HkbStateMachineStateInfoVisitor::Name(data.name.clone().into()),
            HkbStateMachineStateInfoVisitor::StateId(data.state_id.into()),
            HkbStateMachineStateInfoVisitor::Probability(data.probability.into()),
            HkbStateMachineStateInfoVisitor::Enable(data.enable.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbStateMachineStateInfo<'de> {
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
enum HkbStateMachineStateInfoVisitor<'a> {
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
    #[serde(rename = "listeners")]
    Listeners(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "enterNotifyEvents")]
    EnterNotifyEvents(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "exitNotifyEvents")]
    ExitNotifyEvents(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "transitions")]
    Transitions(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "generator")]
    Generator(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "stateId")]
    StateId(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "probability")]
    Probability(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "enable")]
    Enable(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbStateMachineStateInfoVisitor<'de>, "@name",
    ("variableBindingSet" => VariableBindingSet(Primitive<Cow<'de, str>>)),
    ("cachedBindables" => CachedBindables(HkArrayRef<()>)),
    ("areBindablesCached" => AreBindablesCached(Primitive<bool>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("listeners" => Listeners(HkArrayRef<Cow<'de, str>>)),
    ("enterNotifyEvents" => EnterNotifyEvents(Primitive<Cow<'de, str>>)),
    ("exitNotifyEvents" => ExitNotifyEvents(Primitive<Cow<'de, str>>)),
    ("transitions" => Transitions(Primitive<Cow<'de, str>>)),
    ("generator" => Generator(Primitive<Cow<'de, str>>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("stateId" => StateId(Primitive<i32>)),
    ("probability" => Probability(Primitive<f32>)),
    ("enable" => Enable(Primitive<bool>)),
}
