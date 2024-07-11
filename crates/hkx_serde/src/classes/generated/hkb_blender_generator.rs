//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbBlenderGenerator`
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

/// `hkbBlenderGenerator`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 116
/// -    vtable: true
/// -    parent: `hkbGenerator`/`0xd68aefc`
/// - signature: `0x22df7147`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbBlenderGenerator<'a> {
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
    /// -   name:`"referencePoseWeightThreshold"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub reference_pose_weight_threshold: f32,
    /// # C++ Class Fields Info
    /// -   name:`"blendParameter"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub blend_parameter: f32,
    /// # C++ Class Fields Info
    /// -   name:`"minCyclicBlendParameter"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub min_cyclic_blend_parameter: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxCyclicBlendParameter"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub max_cyclic_blend_parameter: f32,
    /// # C++ Class Fields Info
    /// -   name:`"indexOfSyncMasterChild"`
    /// -   type: `hkInt16`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub index_of_sync_master_child: i16,
    /// # C++ Class Fields Info
    /// -   name:`"flags"`
    /// -   type: `hkInt16`
    /// - offset: 58
    /// -  flags: `FLAGS_NONE`
    pub flags: i16,
    /// # C++ Class Fields Info
    /// -   name:`"subtractLastChild"`
    /// -   type: `hkBool`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub subtract_last_child: bool,
    /// # C++ Class Fields Info
    /// -   name:`"children"`
    /// -   type: `hkArray<hkbBlenderGeneratorChild*>`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub children: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"childrenInternalStates"`
    /// -   type: `hkArray<void>`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub children_internal_states: HkArrayRef<()>,
    /// # C++ Class Fields Info
    /// -   name:`"sortedChildren"`
    /// -   type: `hkArray<void>`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub sorted_children: HkArrayRef<()>,
    /// # C++ Class Fields Info
    /// -   name:`"endIntervalWeight"`
    /// -   type: `hkReal`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub end_interval_weight: f32,
    /// # C++ Class Fields Info
    /// -   name:`"numActiveChildren"`
    /// -   type: `hkInt32`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub num_active_children: i32,
    /// # C++ Class Fields Info
    /// -   name:`"beginIntervalIndex"`
    /// -   type: `hkInt16`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub begin_interval_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"endIntervalIndex"`
    /// -   type: `hkInt16`
    /// - offset: 110
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub end_interval_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"initSync"`
    /// -   type: `hkBool`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub init_sync: bool,
    /// # C++ Class Fields Info
    /// -   name:`"doSubtractiveBlend"`
    /// -   type: `hkBool`
    /// - offset: 113
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub do_subtractive_blend: bool,
}

impl Serialize for HkbBlenderGenerator<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbBlenderGeneratorVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbBlenderGenerator<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbBlenderGeneratorVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbBlenderGeneratorVisitor<'a>>> for HkbBlenderGenerator<'a> {
    fn from(_values: Vec<HkbBlenderGeneratorVisitor<'a>>) -> Self {
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


        for _value in _values {
            match _value {
                HkbBlenderGeneratorVisitor::UserData(m) => user_data = Some(m),
                HkbBlenderGeneratorVisitor::Name(m) => name = Some(m),
                HkbBlenderGeneratorVisitor::Id(m) => id = Some(m),
                HkbBlenderGeneratorVisitor::CloneState(m) => clone_state = Some(m),
                HkbBlenderGeneratorVisitor::PadNode(m) => pad_node = Some(m),
                HkbBlenderGeneratorVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                HkbBlenderGeneratorVisitor::CachedBindables(m) => cached_bindables = Some(m),
                HkbBlenderGeneratorVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                HkbBlenderGeneratorVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbBlenderGeneratorVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbBlenderGeneratorVisitor::ReferencePoseWeightThreshold(m) => reference_pose_weight_threshold = Some(m),
                HkbBlenderGeneratorVisitor::BlendParameter(m) => blend_parameter = Some(m),
                HkbBlenderGeneratorVisitor::MinCyclicBlendParameter(m) => min_cyclic_blend_parameter = Some(m),
                HkbBlenderGeneratorVisitor::MaxCyclicBlendParameter(m) => max_cyclic_blend_parameter = Some(m),
                HkbBlenderGeneratorVisitor::IndexOfSyncMasterChild(m) => index_of_sync_master_child = Some(m),
                HkbBlenderGeneratorVisitor::Flags(m) => flags = Some(m),
                HkbBlenderGeneratorVisitor::SubtractLastChild(m) => subtract_last_child = Some(m),
                HkbBlenderGeneratorVisitor::Children(m) => children = Some(m),
                HkbBlenderGeneratorVisitor::ChildrenInternalStates(m) => children_internal_states = Some(m),
                HkbBlenderGeneratorVisitor::SortedChildren(m) => sorted_children = Some(m),
                HkbBlenderGeneratorVisitor::EndIntervalWeight(m) => end_interval_weight = Some(m),
                HkbBlenderGeneratorVisitor::NumActiveChildren(m) => num_active_children = Some(m),
                HkbBlenderGeneratorVisitor::BeginIntervalIndex(m) => begin_interval_index = Some(m),
                HkbBlenderGeneratorVisitor::EndIntervalIndex(m) => end_interval_index = Some(m),
                HkbBlenderGeneratorVisitor::InitSync(m) => init_sync = Some(m),
                HkbBlenderGeneratorVisitor::DoSubtractiveBlend(m) => do_subtractive_blend = Some(m),

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

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbBlenderGenerator<'a>> for Vec<HkbBlenderGeneratorVisitor<'a>> {
    fn from(data: &HkbBlenderGenerator<'a>) -> Self {
        vec![
            HkbBlenderGeneratorVisitor::UserData(data.user_data.into()),
            HkbBlenderGeneratorVisitor::Name(data.name.clone().into()),
            HkbBlenderGeneratorVisitor::Id(data.id.into()),
            HkbBlenderGeneratorVisitor::CloneState(data.clone_state.into()),
            HkbBlenderGeneratorVisitor::PadNode(data.pad_node.clone()),
            HkbBlenderGeneratorVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            HkbBlenderGeneratorVisitor::CachedBindables(data.cached_bindables.clone()),
            HkbBlenderGeneratorVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            HkbBlenderGeneratorVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbBlenderGeneratorVisitor::ReferenceCount(data.reference_count.into()),
            HkbBlenderGeneratorVisitor::ReferencePoseWeightThreshold(data.reference_pose_weight_threshold.into()),
            HkbBlenderGeneratorVisitor::BlendParameter(data.blend_parameter.into()),
            HkbBlenderGeneratorVisitor::MinCyclicBlendParameter(data.min_cyclic_blend_parameter.into()),
            HkbBlenderGeneratorVisitor::MaxCyclicBlendParameter(data.max_cyclic_blend_parameter.into()),
            HkbBlenderGeneratorVisitor::IndexOfSyncMasterChild(data.index_of_sync_master_child.into()),
            HkbBlenderGeneratorVisitor::Flags(data.flags.into()),
            HkbBlenderGeneratorVisitor::SubtractLastChild(data.subtract_last_child.into()),
            HkbBlenderGeneratorVisitor::Children(data.children.clone()),
            HkbBlenderGeneratorVisitor::ChildrenInternalStates(data.children_internal_states.clone()),
            HkbBlenderGeneratorVisitor::SortedChildren(data.sorted_children.clone()),
            HkbBlenderGeneratorVisitor::EndIntervalWeight(data.end_interval_weight.into()),
            HkbBlenderGeneratorVisitor::NumActiveChildren(data.num_active_children.into()),
            HkbBlenderGeneratorVisitor::BeginIntervalIndex(data.begin_interval_index.into()),
            HkbBlenderGeneratorVisitor::EndIntervalIndex(data.end_interval_index.into()),
            HkbBlenderGeneratorVisitor::InitSync(data.init_sync.into()),
            HkbBlenderGeneratorVisitor::DoSubtractiveBlend(data.do_subtractive_blend.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbBlenderGenerator<'de> {
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
enum HkbBlenderGeneratorVisitor<'a> {
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
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbBlenderGeneratorVisitor<'de>, "@name",
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
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum BlenderFlags {
    #[serde(rename = "FLAG_SYNC")]
    #[default]
    FlagSync = 1,
    #[serde(rename = "FLAG_SMOOTH_GENERATOR_WEIGHTS")]
    FlagSmoothGeneratorWeights = 4,
    #[serde(rename = "FLAG_DONT_DEACTIVATE_CHILDREN_WITH_ZERO_WEIGHTS")]
    FlagDontDeactivateChildrenWithZeroWeights = 8,
    #[serde(rename = "FLAG_PARAMETRIC_BLEND")]
    FlagParametricBlend = 16,
    #[serde(rename = "FLAG_IS_PARAMETRIC_BLEND_CYCLIC")]
    FlagIsParametricBlendCyclic = 32,
    #[serde(rename = "FLAG_FORCE_DENSE_POSE")]
    FlagForceDensePose = 64,
}
