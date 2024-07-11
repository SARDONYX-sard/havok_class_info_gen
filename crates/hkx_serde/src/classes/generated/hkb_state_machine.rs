//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbStateMachine`
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

/// `hkbStateMachine`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 180
/// -    vtable: true
/// -    parent: `hkbGenerator`/`0xd68aefc`
/// - signature: `0x816c1dcb`
/// -   version: 4
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbStateMachine<'a> {
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
    /// -   name:`"eventToSendWhenStateOrTransitionChanges"`
    /// -   type: `struct hkbEvent`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub event_to_send_when_state_or_transition_changes: SingleClass<HkbEvent<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"startStateChooser"`
    /// -   type: `struct hkbStateChooser*`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub start_state_chooser: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"startStateId"`
    /// -   type: `hkInt32`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub start_state_id: i32,
    /// # C++ Class Fields Info
    /// -   name:`"returnToPreviousStateEventId"`
    /// -   type: `hkInt32`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub return_to_previous_state_event_id: i32,
    /// # C++ Class Fields Info
    /// -   name:`"randomTransitionEventId"`
    /// -   type: `hkInt32`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub random_transition_event_id: i32,
    /// # C++ Class Fields Info
    /// -   name:`"transitionToNextHigherStateEventId"`
    /// -   type: `hkInt32`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub transition_to_next_higher_state_event_id: i32,
    /// # C++ Class Fields Info
    /// -   name:`"transitionToNextLowerStateEventId"`
    /// -   type: `hkInt32`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    pub transition_to_next_lower_state_event_id: i32,
    /// # C++ Class Fields Info
    /// -   name:`"syncVariableIndex"`
    /// -   type: `hkInt32`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    pub sync_variable_index: i32,
    /// # C++ Class Fields Info
    /// -   name:`"currentStateId"`
    /// -   type: `hkInt32`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub current_state_id: i32,
    /// # C++ Class Fields Info
    /// -   name:`"wrapAroundStateId"`
    /// -   type: `hkBool`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    pub wrap_around_state_id: bool,
    /// # C++ Class Fields Info
    /// -   name:`"maxSimultaneousTransitions"`
    /// -   type: `hkInt8`
    /// - offset: 85
    /// -  flags: `FLAGS_NONE`
    pub max_simultaneous_transitions: i8,
    /// # C++ Class Fields Info
    /// -   name:`"startStateMode"`
    /// -   type: `enum StartStateMode`
    /// - offset: 86
    /// -  flags: `FLAGS_NONE`
    pub start_state_mode: StartStateMode,
    /// # C++ Class Fields Info
    /// -   name:`"selfTransitionMode"`
    /// -   type: `enum StateMachineSelfTransitionMode`
    /// - offset: 87
    /// -  flags: `FLAGS_NONE`
    pub self_transition_mode: StateMachineSelfTransitionMode,
    /// # C++ Class Fields Info
    /// -   name:`"isActive"`
    /// -   type: `hkBool`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub is_active: bool,
    /// # C++ Class Fields Info
    /// -   name:`"states"`
    /// -   type: `hkArray<hkbStateMachineStateInfo*>`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    pub states: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"wildcardTransitions"`
    /// -   type: `struct hkbStateMachineTransitionInfoArray*`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    pub wildcard_transitions: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"stateIdToIndexMap"`
    /// -   type: `void*`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub state_id_to_index_map: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"activeTransitions"`
    /// -   type: `hkArray<void>`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub active_transitions: HkArrayRef<()>,
    /// # C++ Class Fields Info
    /// -   name:`"transitionFlags"`
    /// -   type: `hkArray<void>`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub transition_flags: HkArrayRef<()>,
    /// # C++ Class Fields Info
    /// -   name:`"wildcardTransitionFlags"`
    /// -   type: `hkArray<void>`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub wildcard_transition_flags: HkArrayRef<()>,
    /// # C++ Class Fields Info
    /// -   name:`"delayedTransitions"`
    /// -   type: `hkArray<void>`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub delayed_transitions: HkArrayRef<()>,
    /// # C++ Class Fields Info
    /// -   name:`"timeInState"`
    /// -   type: `hkReal`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub time_in_state: f32,
    /// # C++ Class Fields Info
    /// -   name:`"lastLocalTime"`
    /// -   type: `hkReal`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub last_local_time: f32,
    /// # C++ Class Fields Info
    /// -   name:`"previousStateId"`
    /// -   type: `hkInt32`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub previous_state_id: i32,
    /// # C++ Class Fields Info
    /// -   name:`"nextStartStateIndexOverride"`
    /// -   type: `hkInt32`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub next_start_state_index_override: i32,
    /// # C++ Class Fields Info
    /// -   name:`"stateOrTransitionChanged"`
    /// -   type: `hkBool`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub state_or_transition_changed: bool,
    /// # C++ Class Fields Info
    /// -   name:`"echoNextUpdate"`
    /// -   type: `hkBool`
    /// - offset: 177
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub echo_next_update: bool,
    /// # C++ Class Fields Info
    /// -   name:`"sCurrentStateIndexAndEntered"`
    /// -   type: `hkUint16`
    /// - offset: 178
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub s_current_state_index_and_entered: u16,
}

impl Serialize for HkbStateMachine<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbStateMachineVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbStateMachine<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbStateMachineVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbStateMachineVisitor<'a>>> for HkbStateMachine<'a> {
    fn from(_values: Vec<HkbStateMachineVisitor<'a>>) -> Self {
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
            let mut event_to_send_when_state_or_transition_changes = None;
            let mut start_state_chooser = None;
            let mut start_state_id = None;
            let mut return_to_previous_state_event_id = None;
            let mut random_transition_event_id = None;
            let mut transition_to_next_higher_state_event_id = None;
            let mut transition_to_next_lower_state_event_id = None;
            let mut sync_variable_index = None;
            let mut current_state_id = None;
            let mut wrap_around_state_id = None;
            let mut max_simultaneous_transitions = None;
            let mut start_state_mode = None;
            let mut self_transition_mode = None;
            let mut is_active = None;
            let mut states = None;
            let mut wildcard_transitions = None;
            let mut state_id_to_index_map = None;
            let mut active_transitions = None;
            let mut transition_flags = None;
            let mut wildcard_transition_flags = None;
            let mut delayed_transitions = None;
            let mut time_in_state = None;
            let mut last_local_time = None;
            let mut previous_state_id = None;
            let mut next_start_state_index_override = None;
            let mut state_or_transition_changed = None;
            let mut echo_next_update = None;
            let mut s_current_state_index_and_entered = None;


        for _value in _values {
            match _value {
                HkbStateMachineVisitor::UserData(m) => user_data = Some(m),
                HkbStateMachineVisitor::Name(m) => name = Some(m),
                HkbStateMachineVisitor::Id(m) => id = Some(m),
                HkbStateMachineVisitor::CloneState(m) => clone_state = Some(m),
                HkbStateMachineVisitor::PadNode(m) => pad_node = Some(m),
                HkbStateMachineVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                HkbStateMachineVisitor::CachedBindables(m) => cached_bindables = Some(m),
                HkbStateMachineVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                HkbStateMachineVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbStateMachineVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbStateMachineVisitor::EventToSendWhenStateOrTransitionChanges(m) => event_to_send_when_state_or_transition_changes = Some(m),
                HkbStateMachineVisitor::StartStateChooser(m) => start_state_chooser = Some(m),
                HkbStateMachineVisitor::StartStateId(m) => start_state_id = Some(m),
                HkbStateMachineVisitor::ReturnToPreviousStateEventId(m) => return_to_previous_state_event_id = Some(m),
                HkbStateMachineVisitor::RandomTransitionEventId(m) => random_transition_event_id = Some(m),
                HkbStateMachineVisitor::TransitionToNextHigherStateEventId(m) => transition_to_next_higher_state_event_id = Some(m),
                HkbStateMachineVisitor::TransitionToNextLowerStateEventId(m) => transition_to_next_lower_state_event_id = Some(m),
                HkbStateMachineVisitor::SyncVariableIndex(m) => sync_variable_index = Some(m),
                HkbStateMachineVisitor::CurrentStateId(m) => current_state_id = Some(m),
                HkbStateMachineVisitor::WrapAroundStateId(m) => wrap_around_state_id = Some(m),
                HkbStateMachineVisitor::MaxSimultaneousTransitions(m) => max_simultaneous_transitions = Some(m),
                HkbStateMachineVisitor::StartStateMode(m) => start_state_mode = Some(m),
                HkbStateMachineVisitor::SelfTransitionMode(m) => self_transition_mode = Some(m),
                HkbStateMachineVisitor::IsActive(m) => is_active = Some(m),
                HkbStateMachineVisitor::States(m) => states = Some(m),
                HkbStateMachineVisitor::WildcardTransitions(m) => wildcard_transitions = Some(m),
                HkbStateMachineVisitor::StateIdToIndexMap(m) => state_id_to_index_map = Some(m),
                HkbStateMachineVisitor::ActiveTransitions(m) => active_transitions = Some(m),
                HkbStateMachineVisitor::TransitionFlags(m) => transition_flags = Some(m),
                HkbStateMachineVisitor::WildcardTransitionFlags(m) => wildcard_transition_flags = Some(m),
                HkbStateMachineVisitor::DelayedTransitions(m) => delayed_transitions = Some(m),
                HkbStateMachineVisitor::TimeInState(m) => time_in_state = Some(m),
                HkbStateMachineVisitor::LastLocalTime(m) => last_local_time = Some(m),
                HkbStateMachineVisitor::PreviousStateId(m) => previous_state_id = Some(m),
                HkbStateMachineVisitor::NextStartStateIndexOverride(m) => next_start_state_index_override = Some(m),
                HkbStateMachineVisitor::StateOrTransitionChanged(m) => state_or_transition_changed = Some(m),
                HkbStateMachineVisitor::EchoNextUpdate(m) => echo_next_update = Some(m),
                HkbStateMachineVisitor::SCurrentStateIndexAndEntered(m) => s_current_state_index_and_entered = Some(m),

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
            event_to_send_when_state_or_transition_changes: event_to_send_when_state_or_transition_changes.unwrap_or_default(),
            start_state_chooser: start_state_chooser.unwrap_or_default().into_inner(),
            start_state_id: start_state_id.unwrap_or_default().into_inner(),
            return_to_previous_state_event_id: return_to_previous_state_event_id.unwrap_or_default().into_inner(),
            random_transition_event_id: random_transition_event_id.unwrap_or_default().into_inner(),
            transition_to_next_higher_state_event_id: transition_to_next_higher_state_event_id.unwrap_or_default().into_inner(),
            transition_to_next_lower_state_event_id: transition_to_next_lower_state_event_id.unwrap_or_default().into_inner(),
            sync_variable_index: sync_variable_index.unwrap_or_default().into_inner(),
            current_state_id: current_state_id.unwrap_or_default().into_inner(),
            wrap_around_state_id: wrap_around_state_id.unwrap_or_default().into_inner(),
            max_simultaneous_transitions: max_simultaneous_transitions.unwrap_or_default().into_inner(),
            start_state_mode: start_state_mode.unwrap_or_default().into_inner(),
            self_transition_mode: self_transition_mode.unwrap_or_default().into_inner(),
            is_active: is_active.unwrap_or_default().into_inner(),
            states: states.unwrap_or_default(),
            wildcard_transitions: wildcard_transitions.unwrap_or_default().into_inner(),
            state_id_to_index_map: state_id_to_index_map.unwrap_or_default().into_inner(),
            active_transitions: active_transitions.unwrap_or_default(),
            transition_flags: transition_flags.unwrap_or_default(),
            wildcard_transition_flags: wildcard_transition_flags.unwrap_or_default(),
            delayed_transitions: delayed_transitions.unwrap_or_default(),
            time_in_state: time_in_state.unwrap_or_default().into_inner(),
            last_local_time: last_local_time.unwrap_or_default().into_inner(),
            previous_state_id: previous_state_id.unwrap_or_default().into_inner(),
            next_start_state_index_override: next_start_state_index_override.unwrap_or_default().into_inner(),
            state_or_transition_changed: state_or_transition_changed.unwrap_or_default().into_inner(),
            echo_next_update: echo_next_update.unwrap_or_default().into_inner(),
            s_current_state_index_and_entered: s_current_state_index_and_entered.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbStateMachine<'a>> for Vec<HkbStateMachineVisitor<'a>> {
    fn from(data: &HkbStateMachine<'a>) -> Self {
        vec![
            HkbStateMachineVisitor::UserData(data.user_data.into()),
            HkbStateMachineVisitor::Name(data.name.clone().into()),
            HkbStateMachineVisitor::Id(data.id.into()),
            HkbStateMachineVisitor::CloneState(data.clone_state.into()),
            HkbStateMachineVisitor::PadNode(data.pad_node.clone()),
            HkbStateMachineVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            HkbStateMachineVisitor::CachedBindables(data.cached_bindables.clone()),
            HkbStateMachineVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            HkbStateMachineVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbStateMachineVisitor::ReferenceCount(data.reference_count.into()),
            HkbStateMachineVisitor::EventToSendWhenStateOrTransitionChanges(data.event_to_send_when_state_or_transition_changes.clone()),
            HkbStateMachineVisitor::StartStateChooser(data.start_state_chooser.clone().into()),
            HkbStateMachineVisitor::StartStateId(data.start_state_id.into()),
            HkbStateMachineVisitor::ReturnToPreviousStateEventId(data.return_to_previous_state_event_id.into()),
            HkbStateMachineVisitor::RandomTransitionEventId(data.random_transition_event_id.into()),
            HkbStateMachineVisitor::TransitionToNextHigherStateEventId(data.transition_to_next_higher_state_event_id.into()),
            HkbStateMachineVisitor::TransitionToNextLowerStateEventId(data.transition_to_next_lower_state_event_id.into()),
            HkbStateMachineVisitor::SyncVariableIndex(data.sync_variable_index.into()),
            HkbStateMachineVisitor::CurrentStateId(data.current_state_id.into()),
            HkbStateMachineVisitor::WrapAroundStateId(data.wrap_around_state_id.into()),
            HkbStateMachineVisitor::MaxSimultaneousTransitions(data.max_simultaneous_transitions.into()),
            HkbStateMachineVisitor::StartStateMode(data.start_state_mode.clone().into()),
            HkbStateMachineVisitor::SelfTransitionMode(data.self_transition_mode.clone().into()),
            HkbStateMachineVisitor::IsActive(data.is_active.into()),
            HkbStateMachineVisitor::States(data.states.clone()),
            HkbStateMachineVisitor::WildcardTransitions(data.wildcard_transitions.clone().into()),
            HkbStateMachineVisitor::StateIdToIndexMap(data.state_id_to_index_map.clone().into()),
            HkbStateMachineVisitor::ActiveTransitions(data.active_transitions.clone()),
            HkbStateMachineVisitor::TransitionFlags(data.transition_flags.clone()),
            HkbStateMachineVisitor::WildcardTransitionFlags(data.wildcard_transition_flags.clone()),
            HkbStateMachineVisitor::DelayedTransitions(data.delayed_transitions.clone()),
            HkbStateMachineVisitor::TimeInState(data.time_in_state.into()),
            HkbStateMachineVisitor::LastLocalTime(data.last_local_time.into()),
            HkbStateMachineVisitor::PreviousStateId(data.previous_state_id.into()),
            HkbStateMachineVisitor::NextStartStateIndexOverride(data.next_start_state_index_override.into()),
            HkbStateMachineVisitor::StateOrTransitionChanged(data.state_or_transition_changed.into()),
            HkbStateMachineVisitor::EchoNextUpdate(data.echo_next_update.into()),
            HkbStateMachineVisitor::SCurrentStateIndexAndEntered(data.s_current_state_index_and_entered.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbStateMachine<'de> {
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
enum HkbStateMachineVisitor<'a> {
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
    #[serde(rename = "eventToSendWhenStateOrTransitionChanges")]
    EventToSendWhenStateOrTransitionChanges(SingleClass<HkbEvent<'a>>),
    /// Visitor fields
    #[serde(rename = "startStateChooser")]
    StartStateChooser(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "startStateId")]
    StartStateId(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "returnToPreviousStateEventId")]
    ReturnToPreviousStateEventId(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "randomTransitionEventId")]
    RandomTransitionEventId(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "transitionToNextHigherStateEventId")]
    TransitionToNextHigherStateEventId(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "transitionToNextLowerStateEventId")]
    TransitionToNextLowerStateEventId(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "syncVariableIndex")]
    SyncVariableIndex(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "currentStateId", skip_serializing)]
    CurrentStateId(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "wrapAroundStateId")]
    WrapAroundStateId(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "maxSimultaneousTransitions")]
    MaxSimultaneousTransitions(Primitive<i8>),
    /// Visitor fields
    #[serde(rename = "startStateMode")]
    StartStateMode(Primitive<StartStateMode>),
    /// Visitor fields
    #[serde(rename = "selfTransitionMode")]
    SelfTransitionMode(Primitive<StateMachineSelfTransitionMode>),
    /// Visitor fields
    #[serde(rename = "isActive", skip_serializing)]
    IsActive(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "states")]
    States(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "wildcardTransitions")]
    WildcardTransitions(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "stateIdToIndexMap", skip_serializing)]
    StateIdToIndexMap(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "activeTransitions", skip_serializing)]
    ActiveTransitions(HkArrayRef<()>),
    /// Visitor fields
    #[serde(rename = "transitionFlags", skip_serializing)]
    TransitionFlags(HkArrayRef<()>),
    /// Visitor fields
    #[serde(rename = "wildcardTransitionFlags", skip_serializing)]
    WildcardTransitionFlags(HkArrayRef<()>),
    /// Visitor fields
    #[serde(rename = "delayedTransitions", skip_serializing)]
    DelayedTransitions(HkArrayRef<()>),
    /// Visitor fields
    #[serde(rename = "timeInState", skip_serializing)]
    TimeInState(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "lastLocalTime", skip_serializing)]
    LastLocalTime(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "previousStateId", skip_serializing)]
    PreviousStateId(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "nextStartStateIndexOverride", skip_serializing)]
    NextStartStateIndexOverride(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "stateOrTransitionChanged", skip_serializing)]
    StateOrTransitionChanged(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "echoNextUpdate", skip_serializing)]
    EchoNextUpdate(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "sCurrentStateIndexAndEntered", skip_serializing)]
    SCurrentStateIndexAndEntered(Primitive<u16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbStateMachineVisitor<'de>, "@name",
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
    ("eventToSendWhenStateOrTransitionChanges" => EventToSendWhenStateOrTransitionChanges(SingleClass<HkbEvent<'de>>)),
    ("startStateChooser" => StartStateChooser(Primitive<Cow<'de, str>>)),
    ("startStateId" => StartStateId(Primitive<i32>)),
    ("returnToPreviousStateEventId" => ReturnToPreviousStateEventId(Primitive<i32>)),
    ("randomTransitionEventId" => RandomTransitionEventId(Primitive<i32>)),
    ("transitionToNextHigherStateEventId" => TransitionToNextHigherStateEventId(Primitive<i32>)),
    ("transitionToNextLowerStateEventId" => TransitionToNextLowerStateEventId(Primitive<i32>)),
    ("syncVariableIndex" => SyncVariableIndex(Primitive<i32>)),
    ("currentStateId" => CurrentStateId(Primitive<i32>)),
    ("wrapAroundStateId" => WrapAroundStateId(Primitive<bool>)),
    ("maxSimultaneousTransitions" => MaxSimultaneousTransitions(Primitive<i8>)),
    ("startStateMode" => StartStateMode(Primitive<StartStateMode>)),
    ("selfTransitionMode" => SelfTransitionMode(Primitive<StateMachineSelfTransitionMode>)),
    ("isActive" => IsActive(Primitive<bool>)),
    ("states" => States(HkArrayRef<Cow<'de, str>>)),
    ("wildcardTransitions" => WildcardTransitions(Primitive<Cow<'de, str>>)),
    ("stateIdToIndexMap" => StateIdToIndexMap(Primitive<Cow<'de, str>>)),
    ("activeTransitions" => ActiveTransitions(HkArrayRef<()>)),
    ("transitionFlags" => TransitionFlags(HkArrayRef<()>)),
    ("wildcardTransitionFlags" => WildcardTransitionFlags(HkArrayRef<()>)),
    ("delayedTransitions" => DelayedTransitions(HkArrayRef<()>)),
    ("timeInState" => TimeInState(Primitive<f32>)),
    ("lastLocalTime" => LastLocalTime(Primitive<f32>)),
    ("previousStateId" => PreviousStateId(Primitive<i32>)),
    ("nextStartStateIndexOverride" => NextStartStateIndexOverride(Primitive<i32>)),
    ("stateOrTransitionChanged" => StateOrTransitionChanged(Primitive<bool>)),
    ("echoNextUpdate" => EchoNextUpdate(Primitive<bool>)),
    ("sCurrentStateIndexAndEntered" => SCurrentStateIndexAndEntered(Primitive<u16>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum StartStateMode {
    #[serde(rename = "START_STATE_MODE_DEFAULT")]
    #[default]
    StartStateModeDefault = 0,
    #[serde(rename = "START_STATE_MODE_SYNC")]
    StartStateModeSync = 1,
    #[serde(rename = "START_STATE_MODE_RANDOM")]
    StartStateModeRandom = 2,
    #[serde(rename = "START_STATE_MODE_CHOOSER")]
    StartStateModeChooser = 3,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum StateMachineSelfTransitionMode {
    #[serde(rename = "SELF_TRANSITION_MODE_NO_TRANSITION")]
    #[default]
    SelfTransitionModeNoTransition = 0,
    #[serde(rename = "SELF_TRANSITION_MODE_TRANSITION_TO_START_STATE")]
    SelfTransitionModeTransitionToStartState = 1,
    #[serde(rename = "SELF_TRANSITION_MODE_FORCE_TRANSITION_TO_START_STATE")]
    SelfTransitionModeForceTransitionToStartState = 2,
}
