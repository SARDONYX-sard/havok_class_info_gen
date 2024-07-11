//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbGeneratorTransitionEffect`
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

/// `hkbGeneratorTransitionEffect`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 92
/// -    vtable: true
/// -    parent: `hkbTransitionEffect`/`0x945da157`
/// - signature: `0x5f771b12`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbGeneratorTransitionEffect<'a> {
    /// # C++ Parent class(`hkbTransitionEffect` => parent: `hkbGenerator`) field Info
    /// -   name:`"selfTransitionMode"`
    /// -   type: `enum SelfTransitionMode`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub self_transition_mode: SelfTransitionMode,
    /// # C++ Parent class(`hkbTransitionEffect` => parent: `hkbGenerator`) field Info
    /// -   name:`"eventMode"`
    /// -   type: `enum EventMode`
    /// - offset: 41
    /// -  flags: `FLAGS_NONE`
    pub event_mode: EventMode,
    /// # C++ Parent class(`hkbTransitionEffect` => parent: `hkbGenerator`) field Info
    /// -   name:`"defaultEventMode"`
    /// -   type: `enum unknown`
    /// - offset: 42
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub default_event_mode: (),

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
    /// -   name:`"transitionGenerator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub transition_generator: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"blendInDuration"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub blend_in_duration: f32,
    /// # C++ Class Fields Info
    /// -   name:`"blendOutDuration"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub blend_out_duration: f32,
    /// # C++ Class Fields Info
    /// -   name:`"syncToGeneratorStartTime"`
    /// -   type: `hkBool`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub sync_to_generator_start_time: bool,
    /// # C++ Class Fields Info
    /// -   name:`"fromGenerator"`
    /// -   type: `void*`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub from_generator: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"toGenerator"`
    /// -   type: `void*`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub to_generator: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"timeInTransition"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub time_in_transition: f32,
    /// # C++ Class Fields Info
    /// -   name:`"duration"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub duration: f32,
    /// # C++ Class Fields Info
    /// -   name:`"effectiveBlendInDuration"`
    /// -   type: `hkReal`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub effective_blend_in_duration: f32,
    /// # C++ Class Fields Info
    /// -   name:`"effectiveBlendOutDuration"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub effective_blend_out_duration: f32,
    /// # C++ Class Fields Info
    /// -   name:`"toGeneratorState"`
    /// -   type: `enum unknown`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub to_generator_state: (),
    /// # C++ Class Fields Info
    /// -   name:`"echoTransitionGenerator"`
    /// -   type: `hkBool`
    /// - offset: 85
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub echo_transition_generator: bool,
    /// # C++ Class Fields Info
    /// -   name:`"echoToGenerator"`
    /// -   type: `hkBool`
    /// - offset: 86
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub echo_to_generator: bool,
    /// # C++ Class Fields Info
    /// -   name:`"justActivated"`
    /// -   type: `hkBool`
    /// - offset: 87
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub just_activated: bool,
    /// # C++ Class Fields Info
    /// -   name:`"updateActiveNodes"`
    /// -   type: `hkBool`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub update_active_nodes: bool,
    /// # C++ Class Fields Info
    /// -   name:`"stage"`
    /// -   type: `enum unknown`
    /// - offset: 89
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub stage: (),
}

impl Serialize for HkbGeneratorTransitionEffect<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbGeneratorTransitionEffectVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbGeneratorTransitionEffect<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbGeneratorTransitionEffectVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbGeneratorTransitionEffectVisitor<'a>>> for HkbGeneratorTransitionEffect<'a> {
    fn from(_values: Vec<HkbGeneratorTransitionEffectVisitor<'a>>) -> Self {
            let mut self_transition_mode = None;
            let mut event_mode = None;
            let mut default_event_mode = None;
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
            let mut transition_generator = None;
            let mut blend_in_duration = None;
            let mut blend_out_duration = None;
            let mut sync_to_generator_start_time = None;
            let mut from_generator = None;
            let mut to_generator = None;
            let mut time_in_transition = None;
            let mut duration = None;
            let mut effective_blend_in_duration = None;
            let mut effective_blend_out_duration = None;
            let mut to_generator_state = None;
            let mut echo_transition_generator = None;
            let mut echo_to_generator = None;
            let mut just_activated = None;
            let mut update_active_nodes = None;
            let mut stage = None;


        for _value in _values {
            match _value {
                HkbGeneratorTransitionEffectVisitor::SelfTransitionMode(m) => self_transition_mode = Some(m),
                HkbGeneratorTransitionEffectVisitor::EventMode(m) => event_mode = Some(m),
                HkbGeneratorTransitionEffectVisitor::DefaultEventMode(m) => default_event_mode = Some(m),
                HkbGeneratorTransitionEffectVisitor::UserData(m) => user_data = Some(m),
                HkbGeneratorTransitionEffectVisitor::Name(m) => name = Some(m),
                HkbGeneratorTransitionEffectVisitor::Id(m) => id = Some(m),
                HkbGeneratorTransitionEffectVisitor::CloneState(m) => clone_state = Some(m),
                HkbGeneratorTransitionEffectVisitor::PadNode(m) => pad_node = Some(m),
                HkbGeneratorTransitionEffectVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                HkbGeneratorTransitionEffectVisitor::CachedBindables(m) => cached_bindables = Some(m),
                HkbGeneratorTransitionEffectVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                HkbGeneratorTransitionEffectVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbGeneratorTransitionEffectVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbGeneratorTransitionEffectVisitor::TransitionGenerator(m) => transition_generator = Some(m),
                HkbGeneratorTransitionEffectVisitor::BlendInDuration(m) => blend_in_duration = Some(m),
                HkbGeneratorTransitionEffectVisitor::BlendOutDuration(m) => blend_out_duration = Some(m),
                HkbGeneratorTransitionEffectVisitor::SyncToGeneratorStartTime(m) => sync_to_generator_start_time = Some(m),
                HkbGeneratorTransitionEffectVisitor::FromGenerator(m) => from_generator = Some(m),
                HkbGeneratorTransitionEffectVisitor::ToGenerator(m) => to_generator = Some(m),
                HkbGeneratorTransitionEffectVisitor::TimeInTransition(m) => time_in_transition = Some(m),
                HkbGeneratorTransitionEffectVisitor::Duration(m) => duration = Some(m),
                HkbGeneratorTransitionEffectVisitor::EffectiveBlendInDuration(m) => effective_blend_in_duration = Some(m),
                HkbGeneratorTransitionEffectVisitor::EffectiveBlendOutDuration(m) => effective_blend_out_duration = Some(m),
                HkbGeneratorTransitionEffectVisitor::ToGeneratorState(m) => to_generator_state = Some(m),
                HkbGeneratorTransitionEffectVisitor::EchoTransitionGenerator(m) => echo_transition_generator = Some(m),
                HkbGeneratorTransitionEffectVisitor::EchoToGenerator(m) => echo_to_generator = Some(m),
                HkbGeneratorTransitionEffectVisitor::JustActivated(m) => just_activated = Some(m),
                HkbGeneratorTransitionEffectVisitor::UpdateActiveNodes(m) => update_active_nodes = Some(m),
                HkbGeneratorTransitionEffectVisitor::Stage(m) => stage = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            self_transition_mode: self_transition_mode.unwrap_or_default().into_inner(),
            event_mode: event_mode.unwrap_or_default().into_inner(),
            default_event_mode: default_event_mode.unwrap_or_default().into_inner(),
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
            transition_generator: transition_generator.unwrap_or_default().into_inner(),
            blend_in_duration: blend_in_duration.unwrap_or_default().into_inner(),
            blend_out_duration: blend_out_duration.unwrap_or_default().into_inner(),
            sync_to_generator_start_time: sync_to_generator_start_time.unwrap_or_default().into_inner(),
            from_generator: from_generator.unwrap_or_default().into_inner(),
            to_generator: to_generator.unwrap_or_default().into_inner(),
            time_in_transition: time_in_transition.unwrap_or_default().into_inner(),
            duration: duration.unwrap_or_default().into_inner(),
            effective_blend_in_duration: effective_blend_in_duration.unwrap_or_default().into_inner(),
            effective_blend_out_duration: effective_blend_out_duration.unwrap_or_default().into_inner(),
            to_generator_state: to_generator_state.unwrap_or_default().into_inner(),
            echo_transition_generator: echo_transition_generator.unwrap_or_default().into_inner(),
            echo_to_generator: echo_to_generator.unwrap_or_default().into_inner(),
            just_activated: just_activated.unwrap_or_default().into_inner(),
            update_active_nodes: update_active_nodes.unwrap_or_default().into_inner(),
            stage: stage.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbGeneratorTransitionEffect<'a>> for Vec<HkbGeneratorTransitionEffectVisitor<'a>> {
    fn from(data: &HkbGeneratorTransitionEffect<'a>) -> Self {
        vec![
            HkbGeneratorTransitionEffectVisitor::SelfTransitionMode(data.self_transition_mode.clone().into()),
            HkbGeneratorTransitionEffectVisitor::EventMode(data.event_mode.clone().into()),
            HkbGeneratorTransitionEffectVisitor::DefaultEventMode(data.default_event_mode.into()),
            HkbGeneratorTransitionEffectVisitor::UserData(data.user_data.into()),
            HkbGeneratorTransitionEffectVisitor::Name(data.name.clone().into()),
            HkbGeneratorTransitionEffectVisitor::Id(data.id.into()),
            HkbGeneratorTransitionEffectVisitor::CloneState(data.clone_state.into()),
            HkbGeneratorTransitionEffectVisitor::PadNode(data.pad_node.clone()),
            HkbGeneratorTransitionEffectVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            HkbGeneratorTransitionEffectVisitor::CachedBindables(data.cached_bindables.clone()),
            HkbGeneratorTransitionEffectVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            HkbGeneratorTransitionEffectVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbGeneratorTransitionEffectVisitor::ReferenceCount(data.reference_count.into()),
            HkbGeneratorTransitionEffectVisitor::TransitionGenerator(data.transition_generator.clone().into()),
            HkbGeneratorTransitionEffectVisitor::BlendInDuration(data.blend_in_duration.into()),
            HkbGeneratorTransitionEffectVisitor::BlendOutDuration(data.blend_out_duration.into()),
            HkbGeneratorTransitionEffectVisitor::SyncToGeneratorStartTime(data.sync_to_generator_start_time.into()),
            HkbGeneratorTransitionEffectVisitor::FromGenerator(data.from_generator.clone().into()),
            HkbGeneratorTransitionEffectVisitor::ToGenerator(data.to_generator.clone().into()),
            HkbGeneratorTransitionEffectVisitor::TimeInTransition(data.time_in_transition.into()),
            HkbGeneratorTransitionEffectVisitor::Duration(data.duration.into()),
            HkbGeneratorTransitionEffectVisitor::EffectiveBlendInDuration(data.effective_blend_in_duration.into()),
            HkbGeneratorTransitionEffectVisitor::EffectiveBlendOutDuration(data.effective_blend_out_duration.into()),
            HkbGeneratorTransitionEffectVisitor::ToGeneratorState(data.to_generator_state.into()),
            HkbGeneratorTransitionEffectVisitor::EchoTransitionGenerator(data.echo_transition_generator.into()),
            HkbGeneratorTransitionEffectVisitor::EchoToGenerator(data.echo_to_generator.into()),
            HkbGeneratorTransitionEffectVisitor::JustActivated(data.just_activated.into()),
            HkbGeneratorTransitionEffectVisitor::UpdateActiveNodes(data.update_active_nodes.into()),
            HkbGeneratorTransitionEffectVisitor::Stage(data.stage.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbGeneratorTransitionEffect<'de> {
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
enum HkbGeneratorTransitionEffectVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "selfTransitionMode")]
    SelfTransitionMode(Primitive<SelfTransitionMode>),
    /// Visitor fields
    #[serde(rename = "eventMode")]
    EventMode(Primitive<EventMode>),
    /// Visitor fields
    #[serde(rename = "defaultEventMode", skip_serializing)]
    DefaultEventMode(Primitive<()>),

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
    #[serde(rename = "transitionGenerator")]
    TransitionGenerator(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "blendInDuration")]
    BlendInDuration(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "blendOutDuration")]
    BlendOutDuration(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "syncToGeneratorStartTime")]
    SyncToGeneratorStartTime(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "fromGenerator", skip_serializing)]
    FromGenerator(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "toGenerator", skip_serializing)]
    ToGenerator(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "timeInTransition", skip_serializing)]
    TimeInTransition(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "duration", skip_serializing)]
    Duration(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "effectiveBlendInDuration", skip_serializing)]
    EffectiveBlendInDuration(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "effectiveBlendOutDuration", skip_serializing)]
    EffectiveBlendOutDuration(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "toGeneratorState", skip_serializing)]
    ToGeneratorState(Primitive<()>),
    /// Visitor fields
    #[serde(rename = "echoTransitionGenerator", skip_serializing)]
    EchoTransitionGenerator(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "echoToGenerator", skip_serializing)]
    EchoToGenerator(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "justActivated", skip_serializing)]
    JustActivated(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "updateActiveNodes", skip_serializing)]
    UpdateActiveNodes(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "stage", skip_serializing)]
    Stage(Primitive<()>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbGeneratorTransitionEffectVisitor<'de>, "@name",
    ("selfTransitionMode" => SelfTransitionMode(Primitive<SelfTransitionMode>)),
    ("eventMode" => EventMode(Primitive<EventMode>)),
    ("defaultEventMode" => DefaultEventMode(Primitive<()>)),
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
    ("transitionGenerator" => TransitionGenerator(Primitive<Cow<'de, str>>)),
    ("blendInDuration" => BlendInDuration(Primitive<f32>)),
    ("blendOutDuration" => BlendOutDuration(Primitive<f32>)),
    ("syncToGeneratorStartTime" => SyncToGeneratorStartTime(Primitive<bool>)),
    ("fromGenerator" => FromGenerator(Primitive<Cow<'de, str>>)),
    ("toGenerator" => ToGenerator(Primitive<Cow<'de, str>>)),
    ("timeInTransition" => TimeInTransition(Primitive<f32>)),
    ("duration" => Duration(Primitive<f32>)),
    ("effectiveBlendInDuration" => EffectiveBlendInDuration(Primitive<f32>)),
    ("effectiveBlendOutDuration" => EffectiveBlendOutDuration(Primitive<f32>)),
    ("toGeneratorState" => ToGeneratorState(Primitive<()>)),
    ("echoTransitionGenerator" => EchoTransitionGenerator(Primitive<bool>)),
    ("echoToGenerator" => EchoToGenerator(Primitive<bool>)),
    ("justActivated" => JustActivated(Primitive<bool>)),
    ("updateActiveNodes" => UpdateActiveNodes(Primitive<bool>)),
    ("stage" => Stage(Primitive<()>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum ToGeneratorState {
    #[serde(rename = "STATE_INACTIVE")]
    #[default]
    StateInactive = 0,
    #[serde(rename = "STATE_READY_FOR_SET_LOCAL_TIME")]
    StateReadyForSetLocalTime = 1,
    #[serde(rename = "STATE_READY_FOR_APPLY_SELF_TRANSITION_MODE")]
    StateReadyForApplySelfTransitionMode = 2,
    #[serde(rename = "STATE_ACTIVE")]
    StateActive = 3,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum Stage {
    #[serde(rename = "STAGE_BLENDING_IN")]
    #[default]
    StageBlendingIn = 0,
    #[serde(rename = "STAGE_PLAYING_TRANSITION_GENERATOR")]
    StagePlayingTransitionGenerator = 1,
    #[serde(rename = "STAGE_BLENDING_OUT")]
    StageBlendingOut = 2,
}
