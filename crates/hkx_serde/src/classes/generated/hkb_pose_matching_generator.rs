//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbPoseMatchingGenerator`
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

/// `hkbPoseMatchingGenerator`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 208
/// -    vtable: true
/// -    parent: `hkbBlenderGenerator`/`0x22df7147`
/// - signature: `0x29e271b4`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbPoseMatchingGenerator<'a> {
    /// # C++ Parent class(`hkbBlenderGenerator` => parent: `hkbGenerator`) field Info
    /// -   name:`"referencePoseWeightThreshold"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub reference_pose_weight_threshold: f32,
    /// # C++ Parent class(`hkbBlenderGenerator` => parent: `hkbGenerator`) field Info
    /// -   name:`"blendParameter"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub blend_parameter: f32,
    /// # C++ Parent class(`hkbBlenderGenerator` => parent: `hkbGenerator`) field Info
    /// -   name:`"minCyclicBlendParameter"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub min_cyclic_blend_parameter: f32,
    /// # C++ Parent class(`hkbBlenderGenerator` => parent: `hkbGenerator`) field Info
    /// -   name:`"maxCyclicBlendParameter"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub max_cyclic_blend_parameter: f32,
    /// # C++ Parent class(`hkbBlenderGenerator` => parent: `hkbGenerator`) field Info
    /// -   name:`"indexOfSyncMasterChild"`
    /// -   type: `hkInt16`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub index_of_sync_master_child: i16,
    /// # C++ Parent class(`hkbBlenderGenerator` => parent: `hkbGenerator`) field Info
    /// -   name:`"flags"`
    /// -   type: `hkInt16`
    /// - offset: 58
    /// -  flags: `FLAGS_NONE`
    pub flags: i16,
    /// # C++ Parent class(`hkbBlenderGenerator` => parent: `hkbGenerator`) field Info
    /// -   name:`"subtractLastChild"`
    /// -   type: `hkBool`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub subtract_last_child: bool,
    /// # C++ Parent class(`hkbBlenderGenerator` => parent: `hkbGenerator`) field Info
    /// -   name:`"children"`
    /// -   type: `hkArray<hkbBlenderGeneratorChild*>`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub children: HkArrayRef<Cow<'a, str>>,
    /// # C++ Parent class(`hkbBlenderGenerator` => parent: `hkbGenerator`) field Info
    /// -   name:`"childrenInternalStates"`
    /// -   type: `hkArray<void>`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub children_internal_states: HkArrayRef<()>,
    /// # C++ Parent class(`hkbBlenderGenerator` => parent: `hkbGenerator`) field Info
    /// -   name:`"sortedChildren"`
    /// -   type: `hkArray<void>`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub sorted_children: HkArrayRef<()>,
    /// # C++ Parent class(`hkbBlenderGenerator` => parent: `hkbGenerator`) field Info
    /// -   name:`"endIntervalWeight"`
    /// -   type: `hkReal`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub end_interval_weight: f32,
    /// # C++ Parent class(`hkbBlenderGenerator` => parent: `hkbGenerator`) field Info
    /// -   name:`"numActiveChildren"`
    /// -   type: `hkInt32`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub num_active_children: i32,
    /// # C++ Parent class(`hkbBlenderGenerator` => parent: `hkbGenerator`) field Info
    /// -   name:`"beginIntervalIndex"`
    /// -   type: `hkInt16`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub begin_interval_index: i16,
    /// # C++ Parent class(`hkbBlenderGenerator` => parent: `hkbGenerator`) field Info
    /// -   name:`"endIntervalIndex"`
    /// -   type: `hkInt16`
    /// - offset: 110
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub end_interval_index: i16,
    /// # C++ Parent class(`hkbBlenderGenerator` => parent: `hkbGenerator`) field Info
    /// -   name:`"initSync"`
    /// -   type: `hkBool`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub init_sync: bool,
    /// # C++ Parent class(`hkbBlenderGenerator` => parent: `hkbGenerator`) field Info
    /// -   name:`"doSubtractiveBlend"`
    /// -   type: `hkBool`
    /// - offset: 113
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub do_subtractive_blend: bool,

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
    /// -   name:`"worldFromModelRotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    pub world_from_model_rotation: Quaternion<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"blendSpeed"`
    /// -   type: `hkReal`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    pub blend_speed: f32,
    /// # C++ Class Fields Info
    /// -   name:`"minSpeedToSwitch"`
    /// -   type: `hkReal`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE`
    pub min_speed_to_switch: f32,
    /// # C++ Class Fields Info
    /// -   name:`"minSwitchTimeNoError"`
    /// -   type: `hkReal`
    /// - offset: 152
    /// -  flags: `FLAGS_NONE`
    pub min_switch_time_no_error: f32,
    /// # C++ Class Fields Info
    /// -   name:`"minSwitchTimeFullError"`
    /// -   type: `hkReal`
    /// - offset: 156
    /// -  flags: `FLAGS_NONE`
    pub min_switch_time_full_error: f32,
    /// # C++ Class Fields Info
    /// -   name:`"startPlayingEventId"`
    /// -   type: `hkInt32`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    pub start_playing_event_id: i32,
    /// # C++ Class Fields Info
    /// -   name:`"startMatchingEventId"`
    /// -   type: `hkInt32`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE`
    pub start_matching_event_id: i32,
    /// # C++ Class Fields Info
    /// -   name:`"rootBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE`
    pub root_bone_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"otherBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 170
    /// -  flags: `FLAGS_NONE`
    pub other_bone_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"anotherBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE`
    pub another_bone_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"pelvisIndex"`
    /// -   type: `hkInt16`
    /// - offset: 174
    /// -  flags: `FLAGS_NONE`
    pub pelvis_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"mode"`
    /// -   type: `enum Mode`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    pub mode: Mode,
    /// # C++ Class Fields Info
    /// -   name:`"currentMatch"`
    /// -   type: `hkInt32`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub current_match: i32,
    /// # C++ Class Fields Info
    /// -   name:`"bestMatch"`
    /// -   type: `hkInt32`
    /// - offset: 184
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub best_match: i32,
    /// # C++ Class Fields Info
    /// -   name:`"timeSinceBetterMatch"`
    /// -   type: `hkReal`
    /// - offset: 188
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub time_since_better_match: f32,
    /// # C++ Class Fields Info
    /// -   name:`"error"`
    /// -   type: `hkReal`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub error: f32,
    /// # C++ Class Fields Info
    /// -   name:`"resetCurrentMatchLocalTime"`
    /// -   type: `hkBool`
    /// - offset: 196
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub reset_current_match_local_time: bool,
    /// # C++ Class Fields Info
    /// -   name:`"poseMatchingUtility"`
    /// -   type: `void*`
    /// - offset: 200
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub pose_matching_utility: Cow<'a, str>,
}

impl Serialize for HkbPoseMatchingGenerator<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbPoseMatchingGeneratorVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbPoseMatchingGenerator<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbPoseMatchingGeneratorVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbPoseMatchingGeneratorVisitor<'a>>> for HkbPoseMatchingGenerator<'a> {
    fn from(_values: Vec<HkbPoseMatchingGeneratorVisitor<'a>>) -> Self {
            let mut reference_pose_weight_threshold = None;
            let mut blend_parameter = None;
            let mut min_cyclic_blend_parameter = None;
            let mut max_cyclic_blend_parameter = None;
            let mut index_of_sync_master_child = None;
            let mut flags = None;
            let mut subtract_last_child = None;
            let mut children = None;
            let mut children_internal_states = None;
            let mut sorted_children = None;
            let mut end_interval_weight = None;
            let mut num_active_children = None;
            let mut begin_interval_index = None;
            let mut end_interval_index = None;
            let mut init_sync = None;
            let mut do_subtractive_blend = None;
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
            let mut world_from_model_rotation = None;
            let mut blend_speed = None;
            let mut min_speed_to_switch = None;
            let mut min_switch_time_no_error = None;
            let mut min_switch_time_full_error = None;
            let mut start_playing_event_id = None;
            let mut start_matching_event_id = None;
            let mut root_bone_index = None;
            let mut other_bone_index = None;
            let mut another_bone_index = None;
            let mut pelvis_index = None;
            let mut mode = None;
            let mut current_match = None;
            let mut best_match = None;
            let mut time_since_better_match = None;
            let mut error = None;
            let mut reset_current_match_local_time = None;
            let mut pose_matching_utility = None;


        for _value in _values {
            match _value {
                HkbPoseMatchingGeneratorVisitor::ReferencePoseWeightThreshold(m) => reference_pose_weight_threshold = Some(m),
                HkbPoseMatchingGeneratorVisitor::BlendParameter(m) => blend_parameter = Some(m),
                HkbPoseMatchingGeneratorVisitor::MinCyclicBlendParameter(m) => min_cyclic_blend_parameter = Some(m),
                HkbPoseMatchingGeneratorVisitor::MaxCyclicBlendParameter(m) => max_cyclic_blend_parameter = Some(m),
                HkbPoseMatchingGeneratorVisitor::IndexOfSyncMasterChild(m) => index_of_sync_master_child = Some(m),
                HkbPoseMatchingGeneratorVisitor::Flags(m) => flags = Some(m),
                HkbPoseMatchingGeneratorVisitor::SubtractLastChild(m) => subtract_last_child = Some(m),
                HkbPoseMatchingGeneratorVisitor::Children(m) => children = Some(m),
                HkbPoseMatchingGeneratorVisitor::ChildrenInternalStates(m) => children_internal_states = Some(m),
                HkbPoseMatchingGeneratorVisitor::SortedChildren(m) => sorted_children = Some(m),
                HkbPoseMatchingGeneratorVisitor::EndIntervalWeight(m) => end_interval_weight = Some(m),
                HkbPoseMatchingGeneratorVisitor::NumActiveChildren(m) => num_active_children = Some(m),
                HkbPoseMatchingGeneratorVisitor::BeginIntervalIndex(m) => begin_interval_index = Some(m),
                HkbPoseMatchingGeneratorVisitor::EndIntervalIndex(m) => end_interval_index = Some(m),
                HkbPoseMatchingGeneratorVisitor::InitSync(m) => init_sync = Some(m),
                HkbPoseMatchingGeneratorVisitor::DoSubtractiveBlend(m) => do_subtractive_blend = Some(m),
                HkbPoseMatchingGeneratorVisitor::UserData(m) => user_data = Some(m),
                HkbPoseMatchingGeneratorVisitor::Name(m) => name = Some(m),
                HkbPoseMatchingGeneratorVisitor::Id(m) => id = Some(m),
                HkbPoseMatchingGeneratorVisitor::CloneState(m) => clone_state = Some(m),
                HkbPoseMatchingGeneratorVisitor::PadNode(m) => pad_node = Some(m),
                HkbPoseMatchingGeneratorVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                HkbPoseMatchingGeneratorVisitor::CachedBindables(m) => cached_bindables = Some(m),
                HkbPoseMatchingGeneratorVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                HkbPoseMatchingGeneratorVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbPoseMatchingGeneratorVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbPoseMatchingGeneratorVisitor::WorldFromModelRotation(m) => world_from_model_rotation = Some(m),
                HkbPoseMatchingGeneratorVisitor::BlendSpeed(m) => blend_speed = Some(m),
                HkbPoseMatchingGeneratorVisitor::MinSpeedToSwitch(m) => min_speed_to_switch = Some(m),
                HkbPoseMatchingGeneratorVisitor::MinSwitchTimeNoError(m) => min_switch_time_no_error = Some(m),
                HkbPoseMatchingGeneratorVisitor::MinSwitchTimeFullError(m) => min_switch_time_full_error = Some(m),
                HkbPoseMatchingGeneratorVisitor::StartPlayingEventId(m) => start_playing_event_id = Some(m),
                HkbPoseMatchingGeneratorVisitor::StartMatchingEventId(m) => start_matching_event_id = Some(m),
                HkbPoseMatchingGeneratorVisitor::RootBoneIndex(m) => root_bone_index = Some(m),
                HkbPoseMatchingGeneratorVisitor::OtherBoneIndex(m) => other_bone_index = Some(m),
                HkbPoseMatchingGeneratorVisitor::AnotherBoneIndex(m) => another_bone_index = Some(m),
                HkbPoseMatchingGeneratorVisitor::PelvisIndex(m) => pelvis_index = Some(m),
                HkbPoseMatchingGeneratorVisitor::Mode(m) => mode = Some(m),
                HkbPoseMatchingGeneratorVisitor::CurrentMatch(m) => current_match = Some(m),
                HkbPoseMatchingGeneratorVisitor::BestMatch(m) => best_match = Some(m),
                HkbPoseMatchingGeneratorVisitor::TimeSinceBetterMatch(m) => time_since_better_match = Some(m),
                HkbPoseMatchingGeneratorVisitor::Error(m) => error = Some(m),
                HkbPoseMatchingGeneratorVisitor::ResetCurrentMatchLocalTime(m) => reset_current_match_local_time = Some(m),
                HkbPoseMatchingGeneratorVisitor::PoseMatchingUtility(m) => pose_matching_utility = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            reference_pose_weight_threshold: reference_pose_weight_threshold.unwrap_or_default().into_inner(),
            blend_parameter: blend_parameter.unwrap_or_default().into_inner(),
            min_cyclic_blend_parameter: min_cyclic_blend_parameter.unwrap_or_default().into_inner(),
            max_cyclic_blend_parameter: max_cyclic_blend_parameter.unwrap_or_default().into_inner(),
            index_of_sync_master_child: index_of_sync_master_child.unwrap_or_default().into_inner(),
            flags: flags.unwrap_or_default().into_inner(),
            subtract_last_child: subtract_last_child.unwrap_or_default().into_inner(),
            children: children.unwrap_or_default(),
            children_internal_states: children_internal_states.unwrap_or_default(),
            sorted_children: sorted_children.unwrap_or_default(),
            end_interval_weight: end_interval_weight.unwrap_or_default().into_inner(),
            num_active_children: num_active_children.unwrap_or_default().into_inner(),
            begin_interval_index: begin_interval_index.unwrap_or_default().into_inner(),
            end_interval_index: end_interval_index.unwrap_or_default().into_inner(),
            init_sync: init_sync.unwrap_or_default().into_inner(),
            do_subtractive_blend: do_subtractive_blend.unwrap_or_default().into_inner(),
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
            world_from_model_rotation: world_from_model_rotation.unwrap_or_default().into_inner(),
            blend_speed: blend_speed.unwrap_or_default().into_inner(),
            min_speed_to_switch: min_speed_to_switch.unwrap_or_default().into_inner(),
            min_switch_time_no_error: min_switch_time_no_error.unwrap_or_default().into_inner(),
            min_switch_time_full_error: min_switch_time_full_error.unwrap_or_default().into_inner(),
            start_playing_event_id: start_playing_event_id.unwrap_or_default().into_inner(),
            start_matching_event_id: start_matching_event_id.unwrap_or_default().into_inner(),
            root_bone_index: root_bone_index.unwrap_or_default().into_inner(),
            other_bone_index: other_bone_index.unwrap_or_default().into_inner(),
            another_bone_index: another_bone_index.unwrap_or_default().into_inner(),
            pelvis_index: pelvis_index.unwrap_or_default().into_inner(),
            mode: mode.unwrap_or_default().into_inner(),
            current_match: current_match.unwrap_or_default().into_inner(),
            best_match: best_match.unwrap_or_default().into_inner(),
            time_since_better_match: time_since_better_match.unwrap_or_default().into_inner(),
            error: error.unwrap_or_default().into_inner(),
            reset_current_match_local_time: reset_current_match_local_time.unwrap_or_default().into_inner(),
            pose_matching_utility: pose_matching_utility.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbPoseMatchingGenerator<'a>> for Vec<HkbPoseMatchingGeneratorVisitor<'a>> {
    fn from(data: &HkbPoseMatchingGenerator<'a>) -> Self {
        vec![
            HkbPoseMatchingGeneratorVisitor::ReferencePoseWeightThreshold(data.reference_pose_weight_threshold.into()),
            HkbPoseMatchingGeneratorVisitor::BlendParameter(data.blend_parameter.into()),
            HkbPoseMatchingGeneratorVisitor::MinCyclicBlendParameter(data.min_cyclic_blend_parameter.into()),
            HkbPoseMatchingGeneratorVisitor::MaxCyclicBlendParameter(data.max_cyclic_blend_parameter.into()),
            HkbPoseMatchingGeneratorVisitor::IndexOfSyncMasterChild(data.index_of_sync_master_child.into()),
            HkbPoseMatchingGeneratorVisitor::Flags(data.flags.into()),
            HkbPoseMatchingGeneratorVisitor::SubtractLastChild(data.subtract_last_child.into()),
            HkbPoseMatchingGeneratorVisitor::Children(data.children.clone()),
            HkbPoseMatchingGeneratorVisitor::ChildrenInternalStates(data.children_internal_states.clone()),
            HkbPoseMatchingGeneratorVisitor::SortedChildren(data.sorted_children.clone()),
            HkbPoseMatchingGeneratorVisitor::EndIntervalWeight(data.end_interval_weight.into()),
            HkbPoseMatchingGeneratorVisitor::NumActiveChildren(data.num_active_children.into()),
            HkbPoseMatchingGeneratorVisitor::BeginIntervalIndex(data.begin_interval_index.into()),
            HkbPoseMatchingGeneratorVisitor::EndIntervalIndex(data.end_interval_index.into()),
            HkbPoseMatchingGeneratorVisitor::InitSync(data.init_sync.into()),
            HkbPoseMatchingGeneratorVisitor::DoSubtractiveBlend(data.do_subtractive_blend.into()),
            HkbPoseMatchingGeneratorVisitor::UserData(data.user_data.into()),
            HkbPoseMatchingGeneratorVisitor::Name(data.name.clone().into()),
            HkbPoseMatchingGeneratorVisitor::Id(data.id.into()),
            HkbPoseMatchingGeneratorVisitor::CloneState(data.clone_state.into()),
            HkbPoseMatchingGeneratorVisitor::PadNode(data.pad_node.clone()),
            HkbPoseMatchingGeneratorVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            HkbPoseMatchingGeneratorVisitor::CachedBindables(data.cached_bindables.clone()),
            HkbPoseMatchingGeneratorVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            HkbPoseMatchingGeneratorVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbPoseMatchingGeneratorVisitor::ReferenceCount(data.reference_count.into()),
            HkbPoseMatchingGeneratorVisitor::WorldFromModelRotation(data.world_from_model_rotation.clone().into()),
            HkbPoseMatchingGeneratorVisitor::BlendSpeed(data.blend_speed.into()),
            HkbPoseMatchingGeneratorVisitor::MinSpeedToSwitch(data.min_speed_to_switch.into()),
            HkbPoseMatchingGeneratorVisitor::MinSwitchTimeNoError(data.min_switch_time_no_error.into()),
            HkbPoseMatchingGeneratorVisitor::MinSwitchTimeFullError(data.min_switch_time_full_error.into()),
            HkbPoseMatchingGeneratorVisitor::StartPlayingEventId(data.start_playing_event_id.into()),
            HkbPoseMatchingGeneratorVisitor::StartMatchingEventId(data.start_matching_event_id.into()),
            HkbPoseMatchingGeneratorVisitor::RootBoneIndex(data.root_bone_index.into()),
            HkbPoseMatchingGeneratorVisitor::OtherBoneIndex(data.other_bone_index.into()),
            HkbPoseMatchingGeneratorVisitor::AnotherBoneIndex(data.another_bone_index.into()),
            HkbPoseMatchingGeneratorVisitor::PelvisIndex(data.pelvis_index.into()),
            HkbPoseMatchingGeneratorVisitor::Mode(data.mode.clone().into()),
            HkbPoseMatchingGeneratorVisitor::CurrentMatch(data.current_match.into()),
            HkbPoseMatchingGeneratorVisitor::BestMatch(data.best_match.into()),
            HkbPoseMatchingGeneratorVisitor::TimeSinceBetterMatch(data.time_since_better_match.into()),
            HkbPoseMatchingGeneratorVisitor::Error(data.error.into()),
            HkbPoseMatchingGeneratorVisitor::ResetCurrentMatchLocalTime(data.reset_current_match_local_time.into()),
            HkbPoseMatchingGeneratorVisitor::PoseMatchingUtility(data.pose_matching_utility.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbPoseMatchingGenerator<'de> {
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
enum HkbPoseMatchingGeneratorVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "referencePoseWeightThreshold")]
    ReferencePoseWeightThreshold(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "blendParameter")]
    BlendParameter(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "minCyclicBlendParameter")]
    MinCyclicBlendParameter(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxCyclicBlendParameter")]
    MaxCyclicBlendParameter(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "indexOfSyncMasterChild")]
    IndexOfSyncMasterChild(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "flags")]
    Flags(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "subtractLastChild")]
    SubtractLastChild(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "children")]
    Children(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "childrenInternalStates", skip_serializing)]
    ChildrenInternalStates(HkArrayRef<()>),
    /// Visitor fields
    #[serde(rename = "sortedChildren", skip_serializing)]
    SortedChildren(HkArrayRef<()>),
    /// Visitor fields
    #[serde(rename = "endIntervalWeight", skip_serializing)]
    EndIntervalWeight(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "numActiveChildren", skip_serializing)]
    NumActiveChildren(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "beginIntervalIndex", skip_serializing)]
    BeginIntervalIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "endIntervalIndex", skip_serializing)]
    EndIntervalIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "initSync", skip_serializing)]
    InitSync(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "doSubtractiveBlend", skip_serializing)]
    DoSubtractiveBlend(Primitive<bool>),

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
    #[serde(rename = "worldFromModelRotation")]
    WorldFromModelRotation(Primitive<Quaternion<f32>>),
    /// Visitor fields
    #[serde(rename = "blendSpeed")]
    BlendSpeed(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "minSpeedToSwitch")]
    MinSpeedToSwitch(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "minSwitchTimeNoError")]
    MinSwitchTimeNoError(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "minSwitchTimeFullError")]
    MinSwitchTimeFullError(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "startPlayingEventId")]
    StartPlayingEventId(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "startMatchingEventId")]
    StartMatchingEventId(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "rootBoneIndex")]
    RootBoneIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "otherBoneIndex")]
    OtherBoneIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "anotherBoneIndex")]
    AnotherBoneIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "pelvisIndex")]
    PelvisIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "mode")]
    Mode(Primitive<Mode>),
    /// Visitor fields
    #[serde(rename = "currentMatch", skip_serializing)]
    CurrentMatch(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "bestMatch", skip_serializing)]
    BestMatch(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "timeSinceBetterMatch", skip_serializing)]
    TimeSinceBetterMatch(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "error", skip_serializing)]
    Error(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "resetCurrentMatchLocalTime", skip_serializing)]
    ResetCurrentMatchLocalTime(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "poseMatchingUtility", skip_serializing)]
    PoseMatchingUtility(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbPoseMatchingGeneratorVisitor<'de>, "@name",
    ("referencePoseWeightThreshold" => ReferencePoseWeightThreshold(Primitive<f32>)),
    ("blendParameter" => BlendParameter(Primitive<f32>)),
    ("minCyclicBlendParameter" => MinCyclicBlendParameter(Primitive<f32>)),
    ("maxCyclicBlendParameter" => MaxCyclicBlendParameter(Primitive<f32>)),
    ("indexOfSyncMasterChild" => IndexOfSyncMasterChild(Primitive<i16>)),
    ("flags" => Flags(Primitive<i16>)),
    ("subtractLastChild" => SubtractLastChild(Primitive<bool>)),
    ("children" => Children(HkArrayRef<Cow<'de, str>>)),
    ("childrenInternalStates" => ChildrenInternalStates(HkArrayRef<()>)),
    ("sortedChildren" => SortedChildren(HkArrayRef<()>)),
    ("endIntervalWeight" => EndIntervalWeight(Primitive<f32>)),
    ("numActiveChildren" => NumActiveChildren(Primitive<i32>)),
    ("beginIntervalIndex" => BeginIntervalIndex(Primitive<i16>)),
    ("endIntervalIndex" => EndIntervalIndex(Primitive<i16>)),
    ("initSync" => InitSync(Primitive<bool>)),
    ("doSubtractiveBlend" => DoSubtractiveBlend(Primitive<bool>)),
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
    ("worldFromModelRotation" => WorldFromModelRotation(Primitive<Quaternion<f32>>)),
    ("blendSpeed" => BlendSpeed(Primitive<f32>)),
    ("minSpeedToSwitch" => MinSpeedToSwitch(Primitive<f32>)),
    ("minSwitchTimeNoError" => MinSwitchTimeNoError(Primitive<f32>)),
    ("minSwitchTimeFullError" => MinSwitchTimeFullError(Primitive<f32>)),
    ("startPlayingEventId" => StartPlayingEventId(Primitive<i32>)),
    ("startMatchingEventId" => StartMatchingEventId(Primitive<i32>)),
    ("rootBoneIndex" => RootBoneIndex(Primitive<i16>)),
    ("otherBoneIndex" => OtherBoneIndex(Primitive<i16>)),
    ("anotherBoneIndex" => AnotherBoneIndex(Primitive<i16>)),
    ("pelvisIndex" => PelvisIndex(Primitive<i16>)),
    ("mode" => Mode(Primitive<Mode>)),
    ("currentMatch" => CurrentMatch(Primitive<i32>)),
    ("bestMatch" => BestMatch(Primitive<i32>)),
    ("timeSinceBetterMatch" => TimeSinceBetterMatch(Primitive<f32>)),
    ("error" => Error(Primitive<f32>)),
    ("resetCurrentMatchLocalTime" => ResetCurrentMatchLocalTime(Primitive<bool>)),
    ("poseMatchingUtility" => PoseMatchingUtility(Primitive<Cow<'de, str>>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum Mode {
    #[serde(rename = "MODE_MATCH")]
    #[default]
    ModeMatch = 0,
    #[serde(rename = "MODE_PLAY")]
    ModePlay = 1,
}
