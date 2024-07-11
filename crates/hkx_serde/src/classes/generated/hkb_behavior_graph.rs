//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbBehaviorGraph`
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

/// `hkbBehaviorGraph`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 176
/// -    vtable: true
/// -    parent: `hkbGenerator`/`0xd68aefc`
/// - signature: `0xb1218f86`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbBehaviorGraph<'a> {
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
    /// -   name:`"variableMode"`
    /// -   type: `enum VariableMode`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub variable_mode: VariableMode,
    /// # C++ Class Fields Info
    /// -   name:`"uniqueIdPool"`
    /// -   type: `hkArray<void>`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub unique_id_pool: HkArrayRef<()>,
    /// # C++ Class Fields Info
    /// -   name:`"idToStateMachineTemplateMap"`
    /// -   type: `void*`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub id_to_state_machine_template_map: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"mirroredExternalIdMap"`
    /// -   type: `hkArray<void>`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub mirrored_external_id_map: HkArrayRef<()>,
    /// # C++ Class Fields Info
    /// -   name:`"pseudoRandomGenerator"`
    /// -   type: `void*`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub pseudo_random_generator: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"rootGenerator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    pub root_generator: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `struct hkbBehaviorGraphData*`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub data: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"rootGeneratorClone"`
    /// -   type: `void*`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub root_generator_clone: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"activeNodes"`
    /// -   type: `void*`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub active_nodes: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"activeNodeTemplateToIndexMap"`
    /// -   type: `void*`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub active_node_template_to_index_map: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"activeNodesChildrenIndices"`
    /// -   type: `void*`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub active_nodes_children_indices: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"globalTransitionData"`
    /// -   type: `void*`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub global_transition_data: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"eventIdMap"`
    /// -   type: `void*`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub event_id_map: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"attributeIdMap"`
    /// -   type: `void*`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub attribute_id_map: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"variableIdMap"`
    /// -   type: `void*`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub variable_id_map: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"characterPropertyIdMap"`
    /// -   type: `void*`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub character_property_id_map: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"variableValueSet"`
    /// -   type: `void*`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub variable_value_set: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"nodeTemplateToCloneMap"`
    /// -   type: `void*`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub node_template_to_clone_map: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"nodeCloneToTemplateMap"`
    /// -   type: `void*`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub node_clone_to_template_map: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"stateListenerTemplateToCloneMap"`
    /// -   type: `void*`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub state_listener_template_to_clone_map: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"nodePartitionInfo"`
    /// -   type: `void*`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub node_partition_info: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"numIntermediateOutputs"`
    /// -   type: `hkInt32`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub num_intermediate_outputs: i32,
    /// # C++ Class Fields Info
    /// -   name:`"jobs"`
    /// -   type: `hkArray<void*>`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub jobs: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"allPartitionMemory"`
    /// -   type: `hkArray<void*>`
    /// - offset: 156
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub all_partition_memory: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"numStaticNodes"`
    /// -   type: `hkInt16`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub num_static_nodes: i16,
    /// # C++ Class Fields Info
    /// -   name:`"nextUniqueId"`
    /// -   type: `hkInt16`
    /// - offset: 170
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub next_unique_id: i16,
    /// # C++ Class Fields Info
    /// -   name:`"isActive"`
    /// -   type: `hkBool`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub is_active: bool,
    /// # C++ Class Fields Info
    /// -   name:`"isLinked"`
    /// -   type: `hkBool`
    /// - offset: 173
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub is_linked: bool,
    /// # C++ Class Fields Info
    /// -   name:`"updateActiveNodes"`
    /// -   type: `hkBool`
    /// - offset: 174
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub update_active_nodes: bool,
    /// # C++ Class Fields Info
    /// -   name:`"stateOrTransitionChanged"`
    /// -   type: `hkBool`
    /// - offset: 175
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub state_or_transition_changed: bool,
}

impl Serialize for HkbBehaviorGraph<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbBehaviorGraphVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbBehaviorGraph<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbBehaviorGraphVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbBehaviorGraphVisitor<'a>>> for HkbBehaviorGraph<'a> {
    fn from(_values: Vec<HkbBehaviorGraphVisitor<'a>>) -> Self {
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
            let mut variable_mode = None;
            let mut unique_id_pool = None;
            let mut id_to_state_machine_template_map = None;
            let mut mirrored_external_id_map = None;
            let mut pseudo_random_generator = None;
            let mut root_generator = None;
            let mut data = None;
            let mut root_generator_clone = None;
            let mut active_nodes = None;
            let mut active_node_template_to_index_map = None;
            let mut active_nodes_children_indices = None;
            let mut global_transition_data = None;
            let mut event_id_map = None;
            let mut attribute_id_map = None;
            let mut variable_id_map = None;
            let mut character_property_id_map = None;
            let mut variable_value_set = None;
            let mut node_template_to_clone_map = None;
            let mut node_clone_to_template_map = None;
            let mut state_listener_template_to_clone_map = None;
            let mut node_partition_info = None;
            let mut num_intermediate_outputs = None;
            let mut jobs = None;
            let mut all_partition_memory = None;
            let mut num_static_nodes = None;
            let mut next_unique_id = None;
            let mut is_active = None;
            let mut is_linked = None;
            let mut update_active_nodes = None;
            let mut state_or_transition_changed = None;


        for _value in _values {
            match _value {
                HkbBehaviorGraphVisitor::UserData(m) => user_data = Some(m),
                HkbBehaviorGraphVisitor::Name(m) => name = Some(m),
                HkbBehaviorGraphVisitor::Id(m) => id = Some(m),
                HkbBehaviorGraphVisitor::CloneState(m) => clone_state = Some(m),
                HkbBehaviorGraphVisitor::PadNode(m) => pad_node = Some(m),
                HkbBehaviorGraphVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                HkbBehaviorGraphVisitor::CachedBindables(m) => cached_bindables = Some(m),
                HkbBehaviorGraphVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                HkbBehaviorGraphVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbBehaviorGraphVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbBehaviorGraphVisitor::VariableMode(m) => variable_mode = Some(m),
                HkbBehaviorGraphVisitor::UniqueIdPool(m) => unique_id_pool = Some(m),
                HkbBehaviorGraphVisitor::IdToStateMachineTemplateMap(m) => id_to_state_machine_template_map = Some(m),
                HkbBehaviorGraphVisitor::MirroredExternalIdMap(m) => mirrored_external_id_map = Some(m),
                HkbBehaviorGraphVisitor::PseudoRandomGenerator(m) => pseudo_random_generator = Some(m),
                HkbBehaviorGraphVisitor::RootGenerator(m) => root_generator = Some(m),
                HkbBehaviorGraphVisitor::Data(m) => data = Some(m),
                HkbBehaviorGraphVisitor::RootGeneratorClone(m) => root_generator_clone = Some(m),
                HkbBehaviorGraphVisitor::ActiveNodes(m) => active_nodes = Some(m),
                HkbBehaviorGraphVisitor::ActiveNodeTemplateToIndexMap(m) => active_node_template_to_index_map = Some(m),
                HkbBehaviorGraphVisitor::ActiveNodesChildrenIndices(m) => active_nodes_children_indices = Some(m),
                HkbBehaviorGraphVisitor::GlobalTransitionData(m) => global_transition_data = Some(m),
                HkbBehaviorGraphVisitor::EventIdMap(m) => event_id_map = Some(m),
                HkbBehaviorGraphVisitor::AttributeIdMap(m) => attribute_id_map = Some(m),
                HkbBehaviorGraphVisitor::VariableIdMap(m) => variable_id_map = Some(m),
                HkbBehaviorGraphVisitor::CharacterPropertyIdMap(m) => character_property_id_map = Some(m),
                HkbBehaviorGraphVisitor::VariableValueSet(m) => variable_value_set = Some(m),
                HkbBehaviorGraphVisitor::NodeTemplateToCloneMap(m) => node_template_to_clone_map = Some(m),
                HkbBehaviorGraphVisitor::NodeCloneToTemplateMap(m) => node_clone_to_template_map = Some(m),
                HkbBehaviorGraphVisitor::StateListenerTemplateToCloneMap(m) => state_listener_template_to_clone_map = Some(m),
                HkbBehaviorGraphVisitor::NodePartitionInfo(m) => node_partition_info = Some(m),
                HkbBehaviorGraphVisitor::NumIntermediateOutputs(m) => num_intermediate_outputs = Some(m),
                HkbBehaviorGraphVisitor::Jobs(m) => jobs = Some(m),
                HkbBehaviorGraphVisitor::AllPartitionMemory(m) => all_partition_memory = Some(m),
                HkbBehaviorGraphVisitor::NumStaticNodes(m) => num_static_nodes = Some(m),
                HkbBehaviorGraphVisitor::NextUniqueId(m) => next_unique_id = Some(m),
                HkbBehaviorGraphVisitor::IsActive(m) => is_active = Some(m),
                HkbBehaviorGraphVisitor::IsLinked(m) => is_linked = Some(m),
                HkbBehaviorGraphVisitor::UpdateActiveNodes(m) => update_active_nodes = Some(m),
                HkbBehaviorGraphVisitor::StateOrTransitionChanged(m) => state_or_transition_changed = Some(m),

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
            variable_mode: variable_mode.unwrap_or_default().into_inner(),
            unique_id_pool: unique_id_pool.unwrap_or_default(),
            id_to_state_machine_template_map: id_to_state_machine_template_map.unwrap_or_default().into_inner(),
            mirrored_external_id_map: mirrored_external_id_map.unwrap_or_default(),
            pseudo_random_generator: pseudo_random_generator.unwrap_or_default().into_inner(),
            root_generator: root_generator.unwrap_or_default().into_inner(),
            data: data.unwrap_or_default().into_inner(),
            root_generator_clone: root_generator_clone.unwrap_or_default().into_inner(),
            active_nodes: active_nodes.unwrap_or_default().into_inner(),
            active_node_template_to_index_map: active_node_template_to_index_map.unwrap_or_default().into_inner(),
            active_nodes_children_indices: active_nodes_children_indices.unwrap_or_default().into_inner(),
            global_transition_data: global_transition_data.unwrap_or_default().into_inner(),
            event_id_map: event_id_map.unwrap_or_default().into_inner(),
            attribute_id_map: attribute_id_map.unwrap_or_default().into_inner(),
            variable_id_map: variable_id_map.unwrap_or_default().into_inner(),
            character_property_id_map: character_property_id_map.unwrap_or_default().into_inner(),
            variable_value_set: variable_value_set.unwrap_or_default().into_inner(),
            node_template_to_clone_map: node_template_to_clone_map.unwrap_or_default().into_inner(),
            node_clone_to_template_map: node_clone_to_template_map.unwrap_or_default().into_inner(),
            state_listener_template_to_clone_map: state_listener_template_to_clone_map.unwrap_or_default().into_inner(),
            node_partition_info: node_partition_info.unwrap_or_default().into_inner(),
            num_intermediate_outputs: num_intermediate_outputs.unwrap_or_default().into_inner(),
            jobs: jobs.unwrap_or_default(),
            all_partition_memory: all_partition_memory.unwrap_or_default(),
            num_static_nodes: num_static_nodes.unwrap_or_default().into_inner(),
            next_unique_id: next_unique_id.unwrap_or_default().into_inner(),
            is_active: is_active.unwrap_or_default().into_inner(),
            is_linked: is_linked.unwrap_or_default().into_inner(),
            update_active_nodes: update_active_nodes.unwrap_or_default().into_inner(),
            state_or_transition_changed: state_or_transition_changed.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbBehaviorGraph<'a>> for Vec<HkbBehaviorGraphVisitor<'a>> {
    fn from(data: &HkbBehaviorGraph<'a>) -> Self {
        vec![
            HkbBehaviorGraphVisitor::UserData(data.user_data.into()),
            HkbBehaviorGraphVisitor::Name(data.name.clone().into()),
            HkbBehaviorGraphVisitor::Id(data.id.into()),
            HkbBehaviorGraphVisitor::CloneState(data.clone_state.into()),
            HkbBehaviorGraphVisitor::PadNode(data.pad_node.clone()),
            HkbBehaviorGraphVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            HkbBehaviorGraphVisitor::CachedBindables(data.cached_bindables.clone()),
            HkbBehaviorGraphVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            HkbBehaviorGraphVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbBehaviorGraphVisitor::ReferenceCount(data.reference_count.into()),
            HkbBehaviorGraphVisitor::VariableMode(data.variable_mode.clone().into()),
            HkbBehaviorGraphVisitor::UniqueIdPool(data.unique_id_pool.clone()),
            HkbBehaviorGraphVisitor::IdToStateMachineTemplateMap(data.id_to_state_machine_template_map.clone().into()),
            HkbBehaviorGraphVisitor::MirroredExternalIdMap(data.mirrored_external_id_map.clone()),
            HkbBehaviorGraphVisitor::PseudoRandomGenerator(data.pseudo_random_generator.clone().into()),
            HkbBehaviorGraphVisitor::RootGenerator(data.root_generator.clone().into()),
            HkbBehaviorGraphVisitor::Data(data.data.clone().into()),
            HkbBehaviorGraphVisitor::RootGeneratorClone(data.root_generator_clone.clone().into()),
            HkbBehaviorGraphVisitor::ActiveNodes(data.active_nodes.clone().into()),
            HkbBehaviorGraphVisitor::ActiveNodeTemplateToIndexMap(data.active_node_template_to_index_map.clone().into()),
            HkbBehaviorGraphVisitor::ActiveNodesChildrenIndices(data.active_nodes_children_indices.clone().into()),
            HkbBehaviorGraphVisitor::GlobalTransitionData(data.global_transition_data.clone().into()),
            HkbBehaviorGraphVisitor::EventIdMap(data.event_id_map.clone().into()),
            HkbBehaviorGraphVisitor::AttributeIdMap(data.attribute_id_map.clone().into()),
            HkbBehaviorGraphVisitor::VariableIdMap(data.variable_id_map.clone().into()),
            HkbBehaviorGraphVisitor::CharacterPropertyIdMap(data.character_property_id_map.clone().into()),
            HkbBehaviorGraphVisitor::VariableValueSet(data.variable_value_set.clone().into()),
            HkbBehaviorGraphVisitor::NodeTemplateToCloneMap(data.node_template_to_clone_map.clone().into()),
            HkbBehaviorGraphVisitor::NodeCloneToTemplateMap(data.node_clone_to_template_map.clone().into()),
            HkbBehaviorGraphVisitor::StateListenerTemplateToCloneMap(data.state_listener_template_to_clone_map.clone().into()),
            HkbBehaviorGraphVisitor::NodePartitionInfo(data.node_partition_info.clone().into()),
            HkbBehaviorGraphVisitor::NumIntermediateOutputs(data.num_intermediate_outputs.into()),
            HkbBehaviorGraphVisitor::Jobs(data.jobs.clone()),
            HkbBehaviorGraphVisitor::AllPartitionMemory(data.all_partition_memory.clone()),
            HkbBehaviorGraphVisitor::NumStaticNodes(data.num_static_nodes.into()),
            HkbBehaviorGraphVisitor::NextUniqueId(data.next_unique_id.into()),
            HkbBehaviorGraphVisitor::IsActive(data.is_active.into()),
            HkbBehaviorGraphVisitor::IsLinked(data.is_linked.into()),
            HkbBehaviorGraphVisitor::UpdateActiveNodes(data.update_active_nodes.into()),
            HkbBehaviorGraphVisitor::StateOrTransitionChanged(data.state_or_transition_changed.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbBehaviorGraph<'de> {
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
enum HkbBehaviorGraphVisitor<'a> {
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
    #[serde(rename = "variableMode")]
    VariableMode(Primitive<VariableMode>),
    /// Visitor fields
    #[serde(rename = "uniqueIdPool", skip_serializing)]
    UniqueIdPool(HkArrayRef<()>),
    /// Visitor fields
    #[serde(rename = "idToStateMachineTemplateMap", skip_serializing)]
    IdToStateMachineTemplateMap(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "mirroredExternalIdMap", skip_serializing)]
    MirroredExternalIdMap(HkArrayRef<()>),
    /// Visitor fields
    #[serde(rename = "pseudoRandomGenerator", skip_serializing)]
    PseudoRandomGenerator(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "rootGenerator")]
    RootGenerator(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "data")]
    Data(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "rootGeneratorClone", skip_serializing)]
    RootGeneratorClone(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "activeNodes", skip_serializing)]
    ActiveNodes(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "activeNodeTemplateToIndexMap", skip_serializing)]
    ActiveNodeTemplateToIndexMap(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "activeNodesChildrenIndices", skip_serializing)]
    ActiveNodesChildrenIndices(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "globalTransitionData", skip_serializing)]
    GlobalTransitionData(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "eventIdMap", skip_serializing)]
    EventIdMap(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "attributeIdMap", skip_serializing)]
    AttributeIdMap(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "variableIdMap", skip_serializing)]
    VariableIdMap(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "characterPropertyIdMap", skip_serializing)]
    CharacterPropertyIdMap(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "variableValueSet", skip_serializing)]
    VariableValueSet(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "nodeTemplateToCloneMap", skip_serializing)]
    NodeTemplateToCloneMap(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "nodeCloneToTemplateMap", skip_serializing)]
    NodeCloneToTemplateMap(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "stateListenerTemplateToCloneMap", skip_serializing)]
    StateListenerTemplateToCloneMap(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "nodePartitionInfo", skip_serializing)]
    NodePartitionInfo(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "numIntermediateOutputs", skip_serializing)]
    NumIntermediateOutputs(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "jobs", skip_serializing)]
    Jobs(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "allPartitionMemory", skip_serializing)]
    AllPartitionMemory(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "numStaticNodes", skip_serializing)]
    NumStaticNodes(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "nextUniqueId", skip_serializing)]
    NextUniqueId(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "isActive", skip_serializing)]
    IsActive(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "isLinked", skip_serializing)]
    IsLinked(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "updateActiveNodes", skip_serializing)]
    UpdateActiveNodes(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "stateOrTransitionChanged", skip_serializing)]
    StateOrTransitionChanged(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbBehaviorGraphVisitor<'de>, "@name",
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
    ("variableMode" => VariableMode(Primitive<VariableMode>)),
    ("uniqueIdPool" => UniqueIdPool(HkArrayRef<()>)),
    ("idToStateMachineTemplateMap" => IdToStateMachineTemplateMap(Primitive<Cow<'de, str>>)),
    ("mirroredExternalIdMap" => MirroredExternalIdMap(HkArrayRef<()>)),
    ("pseudoRandomGenerator" => PseudoRandomGenerator(Primitive<Cow<'de, str>>)),
    ("rootGenerator" => RootGenerator(Primitive<Cow<'de, str>>)),
    ("data" => Data(Primitive<Cow<'de, str>>)),
    ("rootGeneratorClone" => RootGeneratorClone(Primitive<Cow<'de, str>>)),
    ("activeNodes" => ActiveNodes(Primitive<Cow<'de, str>>)),
    ("activeNodeTemplateToIndexMap" => ActiveNodeTemplateToIndexMap(Primitive<Cow<'de, str>>)),
    ("activeNodesChildrenIndices" => ActiveNodesChildrenIndices(Primitive<Cow<'de, str>>)),
    ("globalTransitionData" => GlobalTransitionData(Primitive<Cow<'de, str>>)),
    ("eventIdMap" => EventIdMap(Primitive<Cow<'de, str>>)),
    ("attributeIdMap" => AttributeIdMap(Primitive<Cow<'de, str>>)),
    ("variableIdMap" => VariableIdMap(Primitive<Cow<'de, str>>)),
    ("characterPropertyIdMap" => CharacterPropertyIdMap(Primitive<Cow<'de, str>>)),
    ("variableValueSet" => VariableValueSet(Primitive<Cow<'de, str>>)),
    ("nodeTemplateToCloneMap" => NodeTemplateToCloneMap(Primitive<Cow<'de, str>>)),
    ("nodeCloneToTemplateMap" => NodeCloneToTemplateMap(Primitive<Cow<'de, str>>)),
    ("stateListenerTemplateToCloneMap" => StateListenerTemplateToCloneMap(Primitive<Cow<'de, str>>)),
    ("nodePartitionInfo" => NodePartitionInfo(Primitive<Cow<'de, str>>)),
    ("numIntermediateOutputs" => NumIntermediateOutputs(Primitive<i32>)),
    ("jobs" => Jobs(HkArrayRef<Cow<'de, str>>)),
    ("allPartitionMemory" => AllPartitionMemory(HkArrayRef<Cow<'de, str>>)),
    ("numStaticNodes" => NumStaticNodes(Primitive<i16>)),
    ("nextUniqueId" => NextUniqueId(Primitive<i16>)),
    ("isActive" => IsActive(Primitive<bool>)),
    ("isLinked" => IsLinked(Primitive<bool>)),
    ("updateActiveNodes" => UpdateActiveNodes(Primitive<bool>)),
    ("stateOrTransitionChanged" => StateOrTransitionChanged(Primitive<bool>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum VariableMode {
    #[serde(rename = "VARIABLE_MODE_DISCARD_WHEN_INACTIVE")]
    #[default]
    VariableModeDiscardWhenInactive = 0,
    #[serde(rename = "VARIABLE_MODE_MAINTAIN_VALUES_WHEN_INACTIVE")]
    VariableModeMaintainValuesWhenInactive = 1,
}
